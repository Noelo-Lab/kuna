//! Port of `decompiler/cpp/kuna_console.{cc,hh}` (W9) — the kuna console
//! capability and the stage-registry console commands.
//!
//! This file is **not** part of upstream Ghidra. In C++ it registers all kuna
//! console commands through the upstream `IfaceCapability` discovery mechanism
//! (a static `IfaceKunaCapability` singleton collected by
//! `CapabilityPoint::initializeAll`). The kuna Rust console has no static
//! capability registry; instead [`register_kuna_commands`] is called from the
//! console binaries' command-registration step (mirroring how
//! `register_decomp_commands` is invoked), which is the faithful equivalent of
//! `IfaceKunaCapability::registerCommands`.
//!
//! # The commands and what is expressible today
//!
//! The kuna stage model is pure static data ([`kuna_decomp::kuna_phases`], the
//! W4 registry), so the **registry-only** commands port in full, byte-for-byte:
//!
//! - [`IfcKunaPhaseList`] — `phase list`: the stages, Band B membership, and the
//!   sub-stage decision catalog (with `LATENT` markers).
//! - [`IfcKunaPhaseMap`] — `phase map [<name>]`: resolve an action/rule group,
//!   console surface, or sub-stage to its stage (or dump the full tables).
//! - [`IfcKunaPhaseCatalog`] — `phase catalog [<option>]`: emit the
//!   LLM-settable assertion catalog as JSON, calling the **W4 byte-compatible
//!   emitter** ([`kuna_decomp::kuna_phases::emit_catalog_json`]). With a program
//!   loaded the live `current` field is joined per option from the
//!   [`Architecture`] flags ([`kuna_live_value`], the port of
//!   `kuna_console.cc kunaLiveValue`). `tests/stages/kuna-catalog.xml`
//!   string-matches this output.
//! - [`IfcKunaFunctions`] — `functions`: emit the canonical executable
//!   function inventory used by whole-binary decompilation.
//! - [`IfcKunaAssert`] — `kassert ...`: validate a typed assertion against the
//!   registry and route it to a store. The **validation + routing decision**
//!   core (parse-error precedence, `kassert list`, the LATENT/unroutable
//!   branches) ports in full via [`kuna_decomp::kuna_assert`]; the store
//!   mutation for a *routable* assertion needs the unported engine integration
//!   layer (Override / FuncProto locks / retype / `OptionDatabase::set`) and is
//!   routed through [`engine_unavailable`].
//!
//! The remaining kuna commands read live decompiler state through accessors the
//! merged `rust-port` tree does not yet expose at the `Architecture` level (the
//! same integration gap `ifacedecomp.rs` documents): the print language
//! (`Architecture::print`, for `phase status`'s `arraynotation` line), the
//! restart side-table (`Architecture`-owned `RestartLog`, `STUB(W5)`), the
//! reduced-pipeline decompile drive (`allacts.getCurrent()->reset/perform`,
//! `clearAnalysis`), and `FlowBlock::nextFlowAfter` (for `BlockGoto::gotoPrints`
//! in the `quality` metric). Each of these commands ports faithfully every part
//! that *is* expressible — its `dcp->conf`/`dcp->fd` null guards with exact text
//! — then routes the engine read through [`engine_unavailable`]:
//!
//! - [`IfcKunaPhaseStatus`] — `phase status`.
//! - [`IfcKunaRestarts`] — `restarts`.
//! - [`IfcKunaPipeline`] — `pipeline [list|<variant>]` (the `list` sub-form is
//!   fully expressible and ported; running a variant needs the drive).
//! - [`IfcKunaQuality`] — `quality`.
//!
//! The S7 region commands (`region tree`/`blocks`/`walk`, C++ `IfcKunaRegion*`
//! from `kuna_regionid.cc`) register through this capability in C++; their
//! `buildFromBlockGraph` block-graph adapter is explicitly `STUB(W7)` in the
//! ported `kuna_regionid` module, so they are registered with a placeholder that
//! routes through [`engine_unavailable`] (see [`register_kuna_commands`]).
//!
//! Where C++ writes the *bulk* stream (`*status->fileoptr`) the port writes
//! [`IfaceStatus::file_out`] (honoring an open `openfile` redirect, which the
//! datatests use to capture the JSON); where C++ writes `*status->optr` it
//! writes [`IfaceStatus::out`]. The exact text in both streams is what the
//! Python harness and the datatest `<stringmatch>` assertions parse.

use crate::ifacedecomp::{IfaceDecompData, DECOMPILE_MODULE};
use crate::interface::{
    CommandStream, IfaceCommandAction, IfaceError, IfaceResult, IfaceStatus,
};
use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_assert::{validate_assertion, AssertLog, Dispatch, KunaAssertion};
use kuna_decomp::kuna_regionid::KunaRegionIdentifier;
use kuna_decomp::kuna_phases::{
    emit_catalog_json, emit_catalog_json_one, kuna_group_by_index, kuna_num_groups,
    kuna_num_subphases, kuna_num_surfaces, kuna_subphase_by_index, kuna_surface_by_index,
    lookup_group, lookup_settable, lookup_subphase, lookup_surface, KunaPhase, KunaStrength,
};
use std::borrow::Cow;
use std::cell::RefCell;

/// The named pipeline variants built by `ActionDatabase::buildDefaultGroups`
/// (C++ `PIPELINE_VARIANTS`).
const PIPELINE_VARIANTS: [&str; 6] =
    ["decompile", "jumptable", "normalize", "paramid", "register", "firstpass"];

/// The stages in registry order P0,S1..S9 — the `for(i=0;i<=9;++i)(KunaPhase)i`
/// loop of `IfcKunaPhaseList::execute`.
const STAGES_IN_ORDER: [KunaPhase; 10] = [
    KunaPhase::P0,
    KunaPhase::P1,
    KunaPhase::P2,
    KunaPhase::P3,
    KunaPhase::P4,
    KunaPhase::P5,
    KunaPhase::P6,
    KunaPhase::P7,
    KunaPhase::P8,
    KunaPhase::P9,
];

// ---------------------------------------------------------------------------
// dcp access — every kuna command shares the "decompile" module data, exactly
// like the upstream IfaceDecompCommand family (kuna_console.cc reads dcp->...).
// ---------------------------------------------------------------------------

/// Reach the shared [`IfaceDecompData`] from a kuna command.
///
/// The kuna commands share the `"decompile"` module's [`IfaceDecompData`] (in
/// C++ they are `IfaceDecompCommand` subclasses, so `getModule()` is
/// `"decompile"` and `dcp` is the same object the decompiler commands read).
/// A missing entry is an internal wiring bug (never user-reachable) surfaced as
/// a base [`IfaceError`], matching [`crate::ifacedecomp`]'s `dcp_mut`.
fn dcp_mut(status: &mut IfaceStatus) -> IfaceResult<&mut IfaceDecompData> {
    match status.get_data_mut(DECOMPILE_MODULE) {
        Some(d) => match d.as_any_mut().downcast_mut::<IfaceDecompData>() {
            Some(dcp) => Ok(dcp),
            None => Err(IfaceError::base("decompile module data has wrong type")),
        },
        None => Err(IfaceError::base("decompile module data not registered")),
    }
}

/// Reach the shared [`IfaceDecompData`] immutably (the read path of the
/// commands that only inspect `dcp->conf`/`dcp->fd`).
fn dcp_ref(status: &IfaceStatus) -> Option<&IfaceDecompData> {
    status
        .get_data(DECOMPILE_MODULE)
        .and_then(|d| d.as_any().downcast_ref::<IfaceDecompData>())
}

/// The error returned where a kuna command's engine read depends on the
/// unported decompiler integration layer (mirrors `ifacedecomp.rs`'s private
/// `engine_unavailable`, kept local because that one is module-private).
///
/// `entry` names the exact missing C++ entry point so the gap is self-describing
/// in the console; it is an `IfaceExecutionError` (the kind a started-but-failed
/// command throws), which the console driver renders under `"Execution error: "`.
fn engine_unavailable(entry: &str) -> IfaceError {
    IfaceError::execution(format!(
        "engine integration not yet ported: {entry} (Architecture print/loader/context + the \
         decompile drive and STUB(W5/W7/W8) side-tables are a later W-item)"
    ))
}

/// Read the live value of a kuna settable from the loaded [`Architecture`], or
/// `None` if it cannot be determined — port of `kuna_console.cc kunaLiveValue`.
///
/// The C++ `arraynotation` reader dereferences `conf->print` (the owned
/// `PrintC`); that accessor is `STUB(W8)` in the merged tree (no `print` field
/// on [`Architecture`]), so `arraynotation` returns `None` here — exactly the
/// `kunaLiveValue` `""` path that suppresses the `current` field. Every other
/// option reads a flag/string that [`Architecture`] does expose, so the
/// `current` field is joined for them, matching C++.
pub fn kuna_live_value(conf: &Architecture, option: &str) -> Option<Cow<'static, str>> {
    // C++ returns "" for an unknown option (suppressing the current field).
    let on_off = |b: bool| if b { "on" } else { "off" };
    // (kuna `symbolnamebound`) The one VALUED live reader whose value is not a
    // fixed token, so it owns its string; every other arm borrows and the
    // emitted catalog bytes are unchanged.
    if option == "symbolnamebound" {
        return Some(match conf.analysis_symbolnamebound {
            None => Cow::Borrowed("off"),
            Some(n) => Cow::Owned(n.to_string()),
        });
    }
    Some(Cow::Borrowed(match option {
        "compareform" => {
            if conf.present_lessequal {
                "original"
            } else {
                "canonical"
            }
        }
        // C++ `arraynotation` reads `conf->print` (PrintC::getArrayNotation), now
        // exposed via `Architecture::print()` (the owned `PrintC`).
        "arraynotation" => on_off(conf.print().options.array_notation()),
        "thumbfuncptr" => on_off(conf.preserve_thumb_funcptr),
        "inferfuncentry" => on_off(conf.infer_funcentry),
        "booleanmask" => on_off(conf.fold_boolean_mask),
        "ovlesssimplify" => on_off(conf.ov_less_simplify),
        "flagcompare" => on_off(conf.fold_flag_compare),
        "addcarrychain" => on_off(conf.add_carry_chain),
        "memsetrecover" => on_off(conf.memset_recover),
        "rodatastring" => on_off(conf.rodata_string),
        "returnpair" => {
            if conf.return_single {
                "single"
            } else {
                "pair"
            }
        }
        "simdlane" => on_off(conf.simd_lane_fold),
        "retsplitglobal" => on_off(conf.ret_split_global),
        "inputvarnodeadjust" => on_off(conf.input_varnode_adjust),
        "retinputhalf" => on_off(conf.ret_input_half),
        "inputparamgap" => on_off(conf.input_param_gap),
        // (kuna `rustabi`) Three-valued, so it reports its own token.
        "rustabi" => kuna_decomp::kuna_rustabi::RustAbiMode::from_u8(conf.rust_abi).as_str(),
        "condexeplace" => on_off(conf.condexe_block_placement),
        "sparcstructret" => on_off(conf.sparc_struct_return),
        "arraystride" => on_off(conf.recover_array_stride),
        "stackalias" => on_off(conf.stack_alias_deadstore),
        "dynamichashmax" => on_off(conf.dynamic_hash_maxdup_high),
        "stackprobeloop" => on_off(conf.model_stack_probe_loop),
        "v850indirectbranch" => on_off(conf.v850_indirect_branch),
        "switchmodbound" => on_off(conf.switch_modulo_bound),
        "switchsharedcase" => on_off(conf.switch_shared_case),
        "realtypes" => on_off(conf.realtypes),
        // (kuna `calloverlap`) Three-valued, so it reports its own token.
        "calloverlap" => match conf.call_overlap {
            0 => "off",
            1 => "in",
            _ => "full",
        },
        // (kuna) Analysis-pass gates: the live `current` field reflects each pass's
        // per-run enable flag (set by `--option <id> on|off`). Real-ELF path only;
        // with no program loaded these never reach (kuna_live_value's caller passes
        // None), so the no-program catalog byte-compat fixture is untouched.
        "noreturn_known" => on_off(conf.analysis_noreturn_known),
        "libproto" => on_off(conf.analysis_libproto),
        "libcsigs" => on_off(conf.analysis_libcsigs),
        "strings" => on_off(conf.analysis_strings),
        "widestrings" => on_off(conf.analysis_widestrings),
        "entry_disc" => on_off(conf.analysis_entry_disc),
        "eh_frame_full" => on_off(conf.analysis_eh_frame_full),
        "arm_markers" => on_off(conf.analysis_arm_markers),
        "mips_gp" => on_off(conf.analysis_mips_gp),
        "i386_pie_plt" => on_off(conf.analysis_i386_pie_plt),
        "ifuncfpret" => on_off(conf.analysis_ifuncfpret),
        "relocrebase" => on_off(conf.analysis_relocrebase),
        "dynrelocs" => on_off(conf.analysis_dynrelocs),
        "pdatachained" => on_off(conf.analysis_pdatachained),
        "symbolnamerepair" => on_off(conf.analysis_symbolnamerepair),
        // (kuna `symbolnamechars`) Three-valued, so it reports its own token.
        "symbolnamechars" => conf.analysis_symbolnamechars.as_str(),
        "msvcfpconst" => on_off(conf.analysis_msvcfpconst),
        "mips_isa" => on_off(conf.analysis_mips_isa),
        "dwarf" => on_off(conf.analysis_dwarf),
        // (kuna) ELF data-symbol naming (DIV-76): committed at `read symbols`
        // like the analysis-pass gates, so the live flag is authoritative.
        "datasyms" => on_off(conf.analysis_datasyms),
        "typedepth" => on_off(conf.analysis_typedepth),
        "dwarfstructs" => on_off(conf.analysis_dwarfstructs),
        "dwarfvariants" => on_off(conf.analysis_dwarfvariants),
        // (kuna `cppsig`) Three-valued, so it reports its own token rather than
        // on/off.
        "cppsig" => conf.analysis_cppsig.as_str(),
        "callfixup" => on_off(conf.analysis_callfixup),
        "addrtable" => on_off(conf.analysis_addrtable),
        "listing" => on_off(conf.analysis_listing),
        "unmappedentry" => on_off(conf.analysis_unmappedentry),
        "ppclocalentry" => on_off(conf.analysis_ppclocalentry),
        "picbase" => on_off(conf.analysis_picbase),
        "entrymainproto" => on_off(conf.analysis_entrymainproto),
        "machomain" => on_off(conf.analysis_machomain),
        "fast_funcdisc" => on_off(conf.analysis_fast_funcdisc),
        "gopclntab" => on_off(conf.analysis_gopclntab),
        // (PR-8) Mach-O arm64e spec selection: reflects the recorded requested
        // state (the live spec-selection gate is the load-time env var, but the
        // catalog `current` mirrors the `option macho-arm64e on|off` request).
        "macho-arm64e" => on_off(conf.macho_arm64e),
        // No current field (C++ returns "").
        _ => return None,
    }))
}

// ---------------------------------------------------------------------------
// `phase list` — IfcKunaPhaseList
// ---------------------------------------------------------------------------

/// (kuna) `phase list`: print the stage model and sub-stage catalog
/// (C++ `IfcKunaPhaseList`). Pure static data; requires no program.
pub struct IfcKunaPhaseList;

impl IfaceCommandAction for IfcKunaPhaseList {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut os = String::new();
        os.push_str("Phases (kuna phase model, docs/phases.md):\n");
        for stage in STAGES_IN_ORDER {
            os.push_str("  ");
            os.push_str(stage.code());
            os.push_str("  ");
            os.push_str(stage.name());
            if stage == KunaPhase::P0 {
                os.push_str("  [orthogonal plane]");
            } else if stage.in_band_b() {
                os.push_str("  [Band B]");
            }
            os.push('\n');
            os.push_str("        artifact: ");
            os.push_str(stage.artifact());
            os.push('\n');
        }
        os.push('\n');
        os.push_str("Sub-phases (named decision points; LATENT = no override surface today):\n");
        for i in 0..kuna_num_subphases() {
            let sub = kuna_subphase_by_index(i);
            os.push_str("  [");
            os.push_str(sub.phase.code());
            os.push_str("] ");
            os.push_str(sub.name);
            if sub.latent {
                os.push_str("  (LATENT)");
            }
            os.push('\n');
            os.push_str("        decision: ");
            os.push_str(sub.decision);
            os.push('\n');
            os.push_str("        assertion: ");
            os.push_str(sub.assertion);
            match sub.strength {
                KunaStrength::Hard => os.push_str(" (HARD)"),
                KunaStrength::Hint => os.push_str(" (HINT)"),
                KunaStrength::None => {}
            }
            os.push_str("   rewind: ");
            os.push_str(sub.rewind.code());
            os.push('\n');
            os.push_str("        exposure: ");
            os.push_str(sub.exposure);
            os.push('\n');
        }
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `phase map [<name>]` — IfcKunaPhaseMap
// ---------------------------------------------------------------------------

/// (kuna) `phase map [<name>]`: resolve a group/surface/sub-stage to its stage
/// (C++ `IfcKunaPhaseMap`). With no argument, dump the full tables.
pub struct IfcKunaPhaseMap;

impl IfaceCommandAction for IfcKunaPhaseMap {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++ builds a space-joined token from the rest of the line.
        let mut token = String::new();
        s.skip_ws();
        while !s.eof() {
            let word = s.read_token();
            s.skip_ws();
            if word.is_empty() {
                break;
            }
            if !token.is_empty() {
                token.push(' ');
            }
            token.push_str(&word);
        }

        let mut os = String::new();
        if token.is_empty() {
            // Dump everything.
            os.push_str("Action/rule groups -> phase (dominant artifact; see docs/history/stage-model.md s15 for straddlers):\n");
            for i in 0..kuna_num_groups() {
                let entry = kuna_group_by_index(i);
                os.push_str("  ");
                os.push_str(entry.phase.code());
                os.push_str("  ");
                os.push_str(entry.group);
                if !entry.subphase.is_empty() {
                    os.push_str("  (");
                    os.push_str(entry.subphase);
                    os.push(')');
                }
                os.push('\n');
                if !entry.note.is_empty() {
                    os.push_str("        ");
                    os.push_str(entry.note);
                    os.push('\n');
                }
            }
            os.push('\n');
            os.push_str("Console surfaces -> phase:\n");
            for i in 0..kuna_num_surfaces() {
                let entry = kuna_surface_by_index(i);
                os.push_str("  ");
                os.push_str(entry.phase.code());
                os.push_str("  ");
                os.push_str(entry.surface);
                if !entry.subphase.is_empty() {
                    os.push_str("  (");
                    os.push_str(entry.subphase);
                    os.push(')');
                }
                if !entry.note.is_empty() {
                    os.push_str("  -- ");
                    os.push_str(entry.note);
                }
                os.push('\n');
            }
            status.file_out(&os);
            return Ok(());
        }

        let mut found = false;
        if let Some(grp) = lookup_group(&token) {
            found = true;
            os.push_str("group ");
            os.push_str(grp.group);
            os.push_str(" -> ");
            os.push_str(grp.phase.code());
            os.push_str(" (");
            os.push_str(grp.phase.name());
            os.push(')');
            if !grp.subphase.is_empty() {
                os.push_str(" sub-phase ");
                os.push_str(grp.subphase);
            }
            os.push('\n');
            if !grp.note.is_empty() {
                os.push_str("  ");
                os.push_str(grp.note);
                os.push('\n');
            }
        }
        if let Some(surf) = lookup_surface(&token) {
            found = true;
            os.push_str("surface \"");
            os.push_str(surf.surface);
            os.push_str("\" -> ");
            os.push_str(surf.phase.code());
            os.push_str(" (");
            os.push_str(surf.phase.name());
            os.push(')');
            if !surf.subphase.is_empty() {
                os.push_str(" sub-phase ");
                os.push_str(surf.subphase);
            }
            os.push('\n');
            if !surf.note.is_empty() {
                os.push_str("  ");
                os.push_str(surf.note);
                os.push('\n');
            }
        }
        if let Some(sub) = lookup_subphase(&token) {
            found = true;
            os.push_str("sub-phase ");
            os.push_str(sub.name);
            os.push_str(" -> ");
            os.push_str(sub.phase.code());
            os.push_str(" (");
            os.push_str(sub.phase.name());
            os.push(')');
            if sub.latent {
                os.push_str("  LATENT");
            }
            os.push('\n');
            os.push_str("  decision: ");
            os.push_str(sub.decision);
            os.push('\n');
            os.push_str("  assertion: ");
            os.push_str(sub.assertion);
            os.push_str("   rewind: ");
            os.push_str(sub.rewind.code());
            os.push('\n');
            os.push_str("  exposure: ");
            os.push_str(sub.exposure);
            os.push('\n');
        }
        if !found {
            return Err(IfaceError::execution(format!(
                "Unknown group/surface/sub-phase: {token}"
            )));
        }
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `phase status` — IfcKunaPhaseStatus
// ---------------------------------------------------------------------------

/// (kuna) `phase status`: report the active pipeline variant and the state of
/// kuna sub-stage options (C++ `IfcKunaPhaseStatus`).
///
/// The first two lines (pipeline variant, compareform) read accessors the
/// merged tree exposes; the third (`arraynotation`) dereferences `conf->print`
/// (`STUB(W8)`), which the architecture does not expose. Because the exact
/// three-line output cannot be reproduced byte-for-byte today, the engine read
/// is routed through [`engine_unavailable`] after the `No load image present`
/// guard, exactly as the merged `ifacedecomp` commands do for unported `print`
/// accessors.
pub struct IfcKunaPhaseStatus;

impl IfaceCommandAction for IfcKunaPhaseStatus {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        let conf = match dcp.conf.as_ref() {
            Some(c) => c.arch(),
            None => return Err(IfaceError::execution("No load image present")),
        };
        // C++ IfcKunaPhaseStatus::execute.
        let mut os = String::new();
        os.push_str("pipeline variant: ");
        os.push_str(conf.allacts.get_current_name());
        os.push('\n');
        os.push_str("compareform: ");
        os.push_str(if conf.present_lessequal { "original" } else { "canonical" });
        os.push('\n');
        os.push_str("arraynotation: ");
        os.push_str(if conf.print().options.array_notation() { "on" } else { "off" });
        os.push('\n');
        // C++ writes the bulk stream (`*status->fileoptr`); the datatest harness
        // sets `fileoptr` to the matched bulk buffer (the interactive console
        // falls back to stdout when no `openfile` redirect is active).
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `phase catalog [<option>]` — IfcKunaPhaseCatalog
// ---------------------------------------------------------------------------

/// (kuna) `phase catalog [<option>]`: emit the LLM-settable assertion catalog as
/// JSON (C++ `IfcKunaPhaseCatalog`).
///
/// Calls the W4 byte-compatible emitter
/// ([`kuna_decomp::kuna_phases::emit_catalog_json`] /
/// [`emit_catalog_json_one`]); the live `current` field is joined per option
/// from the loaded [`Architecture`] via [`kuna_live_value`] (C++
/// `kunaLiveValue`). Works with no program loaded (the static doc form).
pub struct IfcKunaPhaseCatalog;

impl IfaceCommandAction for IfcKunaPhaseCatalog {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++: s >> ws >> option;
        s.skip_ws();
        let option = s.read_token();

        let dcp = dcp_mut(status)?;
        let conf = dcp.conf.as_ref();

        let out = if !option.is_empty() {
            // Single-option form: `phase catalog <option>`.
            if lookup_settable(&option).is_none() {
                return Err(IfaceError::execution(format!(
                    "Unknown settable option: {option} (try `phase catalog`)"
                )));
            }
            let live = conf.and_then(|c| kuna_live_value(c.arch(), &option));
            let live = live.as_deref();
            // lookup_settable just succeeded, so this is Some.
            emit_catalog_json_one(&option, live).unwrap_or_default()
        } else {
            // Full form: `phase catalog`. The emitter calls the closure per
            // option (in registry order), matching the C++ per-row kunaLiveValue
            // join. The closure returns &'static str (the registry value table).
            match conf {
                Some(c) => emit_catalog_json(|opt| kuna_live_value(c.arch(), opt)),
                None => emit_catalog_json(|_| None),
            }
        };
        status.file_out(&out);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `kassert ...` — IfcKunaAssert (class lives in kuna_assert.cc; registered here)
// ---------------------------------------------------------------------------

/// (kuna) `kassert <phase> <subphase> <args...> [hard|hint]` / `kassert list`:
/// write a typed assertion through an existing store (C++ `IfcKunaAssert`,
/// `kuna_assert.cc`; registered by the kuna capability in `kuna_console.cc`).
///
/// The **validation + routing decision** core ports in full (the parse-error
/// precedence, `kassert list`, the LATENT/unroutable error arms) on top of
/// [`kuna_decomp::kuna_assert`]. The store mutation for a *routable* assertion
/// (Override / FuncProto locks / retype / rename / `OptionDatabase::set`) needs
/// the unported engine integration and is routed through [`engine_unavailable`];
/// because the success path cannot complete the store write, the `assertLog`
/// records nothing for a routable assertion (the `kassert applied` line and
/// `assertLog.push_back` follow the store write in C++).
///
/// `assert_log` is the per-instance equivalent of the C++ file-static
/// `assertLog`: one [`IfcKunaAssert`] is registered, so its [`RefCell`] holds
/// the session log for the console's lifetime, matching the C++ semantics.
pub struct IfcKunaAssert {
    /// Session log (C++ file-static `assertLog`).
    assert_log: RefCell<AssertLog>,
}

impl IfcKunaAssert {
    /// A fresh `kassert` command with an empty session log.
    pub fn new() -> IfcKunaAssert {
        IfcKunaAssert { assert_log: RefCell::new(AssertLog::new()) }
    }
}

impl Default for IfcKunaAssert {
    fn default() -> Self {
        IfcKunaAssert::new()
    }
}

impl IfaceCommandAction for IfcKunaAssert {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }

        s.skip_ws();
        let stagecode = s.read_token();
        if stagecode == "list" {
            // listAssertions writes the bulk stream.
            let text = self.assert_log.borrow().render_list();
            status.file_out(&text);
            return Ok(());
        }

        s.skip_ws();
        let subname = s.read_token();

        // Collect the rest of the line (C++ tokenizeRest).
        let mut tokens: Vec<String> = Vec::new();
        s.skip_ws();
        while !s.eof() {
            let word = s.read_token();
            s.skip_ws();
            if word.is_empty() {
                break;
            }
            tokens.push(word);
        }

        // C++: optional trailing hard|hint pops off the token list and sets the
        // requested strength override (default to the catalog strength).
        let mut strength_override: Option<KunaStrength> = None;
        if let Some(last) = tokens.last() {
            if last == "hard" {
                strength_override = Some(KunaStrength::Hard);
                tokens.pop();
            } else if last == "hint" {
                strength_override = Some(KunaStrength::Hint);
                tokens.pop();
            }
        }

        // Validate stage/sub-stage and compute strength + minimal rewind + the
        // routing decision (the console-independent core). C++ throws
        // IfaceParseError in this precedence: bad stage code -> missing sub-stage
        // -> unknown sub-stage -> sub-stage/stage mismatch. validate_assertion
        // returns KunaError::Parse for all four; surface them as IfaceParseError.
        let validated = validate_assertion(&stagecode, &subname, strength_override)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;

        // Assemble the would-be record (matches the C++ `rec` assembly). The
        // fields are computed here so the store-mutation wire site (each routable
        // arm) is the single place to push it on success when the engine lands.
        let func_name = match &dcp_mut(status)?.fd {
            Some(fd) => fd.get_name().to_string(),
            None => "(global)".to_string(),
        };
        let record = KunaAssertion {
            func_name,
            phase: validated.phase,
            subphase: validated.subphase.clone(),
            args: join_tokens(&tokens),
            strength: validated.requested,
            applied: validated.applied,
            rewind: validated.rewind,
        };

        // C++ `kassert applied:` confirmation line (written to `*optr`), emitted
        // after a routable store mutation succeeds.
        let applied_line = {
            let mut s = String::new();
            s.push_str("kassert applied: [");
            s.push_str(record.phase.code());
            s.push_str("] ");
            s.push_str(&record.subphase);
            if !record.args.is_empty() {
                s.push(' ');
                s.push_str(&record.args);
            }
            s.push_str("  rewind->");
            s.push_str(record.rewind.code());
            s.push_str(" (Ghidra-actual: whole-function)\n");
            s
        };

        // Dispatch: the store mutation. The LATENT/unroutable arms touch no store
        // and emit their exact text now; every routable arm performs a store
        // mutation through the unported engine layer (Override::insert*,
        // FuncProto locks, retype/rename, OptionDatabase::set, with
        // parse_machaddr/parse_type/readSymbol).
        match validated.dispatch {
            Dispatch::Latent => {
                // C++: throw IfaceExecutionError("Sub-stage <name> is LATENT: ...")
                Err(IfaceError::execution(format!(
                    "Sub-phase {subname} is LATENT: no override surface exists yet (kuna roadmap)"
                )))
            }
            Dispatch::Unroutable => {
                // C++: throw IfaceExecutionError("Sub-stage <name> is not yet
                //   routable through kassert; use its native surface: <exposure>")
                let exposure = lookup_subphase(&subname).map(|sub| sub.exposure).unwrap_or("");
                Err(IfaceError::execution(format!(
                    "Sub-phase {subname} is not yet routable through kassert; use its native surface: {exposure}"
                )))
            }
            Dispatch::ForceGoto => Err(engine_unavailable(
                "Override::insertForceGoto (parse_machaddr) for kassert edge-virtualization",
            )),
            Dispatch::DeadcodeDelay => Err(engine_unavailable(
                "Override::insertDeadcodeDelay (Architecture::getSpaceByName) for kassert dead-definition-gate",
            )),
            Dispatch::MultistageJump => Err(engine_unavailable(
                "Override::insertMultistageJump (parse_machaddr) for kassert switch-model",
            )),
            Dispatch::FlowOverride => Err(engine_unavailable(
                "Override::insertFlowOverride (parse_machaddr) for kassert flow-classification",
            )),
            Dispatch::ProtoLock => Err(engine_unavailable(
                "FuncProto::setInputLock/setOutputLock for kassert prototype-source",
            )),
            Dispatch::Retype => Err(engine_unavailable(
                "parse_type + Scope::retypeSymbol (IfaceDecompData::readSymbol) for kassert type-propagation",
            )),
            Dispatch::Isolate => Err(engine_unavailable(
                "Symbol::setIsolated (IfaceDecompData::readSymbol) for kassert merge-aggressiveness",
            )),
            Dispatch::Rename => {
                // C++ kassert naming-policy -> Scope::renameSymbol + namelock,
                // exactly like IfcRename (ifacedecomp.rs IfcRename::execute).
                use kuna_decomp::varnode::varnode_flags;
                if tokens.len() < 2 {
                    return Err(IfaceError::parse(
                        "naming-policy assertion needs <oldname> <newname>",
                    ));
                }
                let oldname = tokens[0].clone();
                let newname = tokens[1].clone();
                let dcp = dcp_mut(status)?;
                let sym_list = dcp.read_symbol(&oldname)?;
                if sym_list.is_empty() {
                    return Err(IfaceError::execution(format!("No symbol named: {oldname}")));
                }
                if sym_list.len() > 1 {
                    return Err(IfaceError::execution(format!(
                        "More than one symbol named: {oldname}"
                    )));
                }
                let sym = sym_list[0];
                let fd = dcp.fd.as_mut().expect("read_symbol succeeded => fd present");
                let lm = fd
                    .get_scope_local_mut()
                    .ok_or_else(|| IfaceError::execution("Function has no local scope"))?;
                lm.rename_symbol(sym, &newname)
                    .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
                lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
                status.out(&applied_line);
                self.assert_log.borrow_mut().push(record);
                Ok(())
            }
            Dispatch::Option(name) => {
                // C++ kassert option -> OptionDatabase::set (the kuna options are
                // not in the OptionDatabase; `set_kuna_option` dispatches them into
                // the live architecture flags — e.g. compareform original sets
                // present_lessequal). Then emit the option's confirmation message
                // and the `kassert applied:` line, and record the assertion.
                if tokens.is_empty() {
                    return Err(IfaceError::parse(format!(
                        "option assertion for {name} needs a value"
                    )));
                }
                let dcp = dcp_mut(status)?;
                let conf = dcp
                    .conf
                    .as_mut()
                    .expect("conf checked non-None at command entry");
                let msg = conf
                    .arch_mut()
                    .set_kuna_option(name, &tokens[0])
                    .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
                let mut out = msg;
                out.push('\n');
                out.push_str(&applied_line);
                status.out(&out);
                self.assert_log.borrow_mut().push(record);
                Ok(())
            }
        }
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `restarts` — IfcKunaRestarts
// ---------------------------------------------------------------------------

/// (kuna) `restarts`: dump recorded restart-trigger events for the current
/// function (C++ `IfcKunaRestarts`).
///
/// C++ calls `kunaDumpRestarts(*status->fileoptr, *dcp->fd)` over the global
/// restart side table; the ported [`kuna_decomp::kuna_restartlog::RestartLog`]
/// is `Architecture`-owned, but the architecture does not yet hold one
/// (`STUB(W5)`: the log and its trigger sites land together), so the dump is
/// routed through [`engine_unavailable`] after the `No function selected`
/// guard.
pub struct IfcKunaRestarts;

impl IfaceCommandAction for IfcKunaRestarts {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        let fd = match dcp.fd.as_ref() {
            Some(fd) => fd,
            None => return Err(IfaceError::execution("No function selected")),
        };
        // C++ kunaDumpRestarts over the Architecture-owned RestartLog.
        let conf = match dcp.conf.as_ref() {
            Some(c) => c.arch(),
            None => return Err(IfaceError::execution("No load image present")),
        };
        let text = conf.restart_log().render(fd);
        status.file_out(&text);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `pipeline [list|<variant>]` — IfcKunaPipeline
// ---------------------------------------------------------------------------

/// (kuna) `pipeline [list|<variant>]`: run a named reduced pipeline on the
/// current function as a sub-query (C++ `IfcKunaPipeline`).
///
/// The `list` sub-form (and the `(current)` marker on the active variant) ports
/// in full. Running a variant mirrors `Funcdata::stageJumpTable`'s
/// save/switch/restore of the root action, but the decompile drive
/// (`allacts.getCurrent()->reset/perform`, `clearAnalysis`) is not assembled at
/// the `Architecture` level in the merged tree, so a valid variant is routed
/// through [`engine_unavailable`] after the guards and the "Unknown pipeline
/// variant" parse error.
pub struct IfcKunaPipeline;

impl IfcKunaPipeline {
    /// Print the named pipeline variants: `pipeline list` (C++
    /// `IfcKunaPipeline::listPipelines`).
    fn list_pipelines(&self, status: &mut IfaceStatus) {
        // C++: if (conf != 0 && allacts.getCurrentName() == variant) "  (current)"
        let current =
            dcp_ref(status).and_then(|dcp| dcp.conf.as_ref()).map(|conf| {
                conf.arch().allacts.get_current_name().to_string()
            });
        let mut os = String::new();
        os.push_str("Named pipeline variants (group filters over the universal action; P0 pipeline-variant sub-phase):\n");
        for v in PIPELINE_VARIANTS {
            os.push_str("  ");
            os.push_str(v);
            if current.as_deref() == Some(v) {
                os.push_str("  (current)");
            }
            os.push('\n');
        }
        status.file_out(&os);
    }
}

impl IfaceCommandAction for IfcKunaPipeline {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        s.skip_ws();
        let name = s.read_token();
        if name.is_empty() || name == "list" {
            self.list_pipelines(status);
            return Ok(());
        }
        // C++: validate against PIPELINE_VARIANTS, throw IfaceParseError if unknown.
        if !PIPELINE_VARIANTS.contains(&name.as_str()) {
            return Err(IfaceError::parse(format!(
                "Unknown pipeline variant: {name} (try `pipeline list`)"
            )));
        }
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        // hasNoCode IS expressible; the message uses the command stream (optr).
        let fd = dcp.fd.as_ref().expect("fd checked Some above");
        if fd.has_no_code() {
            let line = format!("No code for {}\n", fd.get_name());
            status.out(&line);
            return Ok(());
        }
        // C++ reduced-pipeline sub-query: rebuild the IR, run the named action
        // group as the current root, restore `decompile` (clearAnalysis +
        // allacts.setCurrent/reset/perform/restore).  Mirror IfcDecompile's
        // take-program pattern so the engine work borrows neither status nor dcp.
        let (fname, has_no_code, proc_started, entry, size, mut prog) = {
            let dcp = dcp_mut(status)?;
            let fd = dcp.fd.as_ref().expect("fd checked Some above");
            let info = (
                fd.get_name().to_string(),
                fd.has_no_code(),
                fd.is_proc_started(),
                fd.get_address().clone(),
                fd.get_size(),
            );
            let prog = dcp.conf.take().expect("conf checked Some above");
            (info.0, info.1, info.2, info.3, info.4, prog)
        };
        if has_no_code {
            dcp_mut(status)?.conf = Some(prog);
            status.out(&format!("No code for {fname}\n"));
            return Ok(());
        }
        // C++: clearAnalysis notice if a prior decompilation exists.
        if proc_started {
            status.out("Clearing old decompilation\n");
        }
        status.out(&format!(
            "Processing {fname} under reduced pipeline `{name}`\n"
        ));
        let result = kuna_decomp::decompile_drive::run_named_pipeline_variant(
            prog.arch_mut(),
            &fname,
            entry,
            size,
            &name,
        );
        // Restore the program (and the reduced-pipeline Funcdata on success).
        let dcp = dcp_mut(status)?;
        dcp.conf = Some(prog);
        match result {
            Ok(fd) => {
                dcp.fd = Some(fd);
                status.out(
                    "Sub-query complete (root action restored to `decompile`)\n",
                );
                Ok(())
            }
            Err(e) => Err(IfaceError::execution(e.explain().to_string())),
        }
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `mode <name>` — IfcKunaMode
// ---------------------------------------------------------------------------

/// (kuna) `mode <reliable|aggressive|fast>`: apply a decompiler mode preset — a batch
/// of option overrides fanned out through `Architecture::apply_mode` (see
/// `kuna_decomp::modes`).  Mirrors `IfcOption`'s dcp/conf access; issue it
/// before `read symbols` so an analysis-tier override (`listing`/`aif`/…) is
/// committed, exactly like an `option` command.  A later `option NAME VALUE`
/// overrides the mode (last-write).
pub struct IfcKunaMode;

impl IfaceCommandAction for IfcKunaMode {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        s.skip_ws();
        let name = s.read_token();
        s.skip_ws();
        if name.is_empty() {
            return Err(IfaceError::parse(
                "Missing mode name (try `mode reliable`, `mode aggressive`, or `mode fast`)",
            ));
        }
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let res = prog
            .arch_mut()
            .apply_mode(&name)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        status.out(&format!("{res}\n"));
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `quality` — IfcKunaQuality
// ---------------------------------------------------------------------------

/// (kuna) `quality`: goto-count structure-quality metric over the structured
/// result (C++ `IfcKunaQuality`).
///
/// The goto-count walk (`kunaCountGotos`) operates on the ported `BlockGraph` /
/// `FlowBlock` arena, but the `goto nodes: N (printed: M)` line needs
/// `BlockGoto::gotoPrints`, which depends on `FlowBlock::nextFlowAfter`
/// (`STUB(W7)`, unported). Because that exact line cannot be reproduced today,
/// the metric is routed through [`engine_unavailable`] after the `No function
/// selected` and `hasNoStructBlocks` guards (both fully expressible).
pub struct IfcKunaQuality;

impl IfaceCommandAction for IfcKunaQuality {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        let fd = dcp.fd.as_ref().expect("fd checked Some above");
        if fd.has_no_struct_blocks() {
            let line = format!(
                "No structured blocks for {} (decompile first)\n",
                fd.get_name()
            );
            status.file_out(&line);
            return Ok(());
        }
        // C++ IfcKunaQuality: count gotos/multi-gotos/if-gotos over the
        // structured tree (kunaCountGotos), report the goto-count quality signal.
        let name = fd.get_name().to_string();
        let basic_blocks = fd.bblocks_get_size();
        let counts = fd.kuna_quality_counts();
        let unstructured = counts.printed_gotos + counts.multigoto_edges + counts.ifgoto_edges;
        let mut os = String::new();
        os.push_str(&format!("Structure quality for {name}:\n"));
        os.push_str(&format!("  basic blocks: {basic_blocks}\n"));
        os.push_str(&format!(
            "  goto nodes: {} (printed: {})\n",
            counts.goto_nodes, counts.printed_gotos
        ));
        os.push_str(&format!("  multi-goto edges: {}\n", counts.multigoto_edges));
        os.push_str(&format!("  if-goto edges: {}\n", counts.ifgoto_edges));
        os.push_str(&format!("  unstructured total: {unstructured}\n"));
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// Region commands (IfcKunaRegionTree/Blocks/Walk, from kuna_regionid.cc)
// ---------------------------------------------------------------------------

/// (kuna) `region tree` / `region blocks` / `region walk`: inspect the S7
/// region hierarchy (C++ `IfcKunaRegion*` from `kuna_regionid.cc`; registered by
/// the kuna capability in `kuna_console.cc`).
///
/// The region identifier is ported (`kuna_decomp::kuna_regionid`); its
/// `buildFromBlockGraph` block-graph adapter (the W7 boundary) is now closed
/// ([`KunaRegionIdentifier::build_from_block_graph`]), so these three commands
/// drive it over the real decompiled `bblocks` CFG (block start addresses, the
/// CFG out-edges, and the per-block `endsWithBranchindOrCbranch` `lastOp` probe).
///
/// Build a [`KunaRegionIdentifier`] from the current function's basic-block
/// graph (the C++ `IfcKunaRegion*` `buildFromBlockGraph` adapter): one `k_block`
/// node per basic block (keyed on its start address, carrying the real block)
/// and one edge per CFG out-edge; the entry address is block 0's start.  Returns
/// the computed identifier ready for tree/blocks/walk rendering.
fn build_region_identifier(
    fd: &kuna_decomp::funcdata::Funcdata,
) -> IfaceResult<KunaRegionIdentifier> {
    let mut ri = KunaRegionIdentifier::new();
    ri.build_from_block_graph(fd)
        .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
    ri.compute().map_err(|e| IfaceError::execution(e.explain().to_string()))?;
    Ok(ri)
}

/// (kuna) `region tree`: render the nested region hierarchy (C++
/// `IfcKunaRegionTree`).
struct IfcKunaRegionTree;

impl IfaceCommandAction for IfcKunaRegionTree {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        let fd = match dcp.fd.as_ref() {
            Some(fd) => fd,
            None => return Err(IfaceError::execution("No function selected")),
        };
        let fname = fd.get_name().to_string();
        let ri = build_region_identifier(fd)?;
        let mut os = String::new();
        os.push_str("Region tree for ");
        os.push_str(&fname);
        os.push_str(":\n");
        os.push_str(&ri.render_tree());
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

/// (kuna) `region blocks`: list each region's flat block-address set (C++
/// `IfcKunaRegionBlocks`).
struct IfcKunaRegionBlocks;

impl IfaceCommandAction for IfcKunaRegionBlocks {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        let fd = match dcp.fd.as_ref() {
            Some(fd) => fd,
            None => return Err(IfaceError::execution("No function selected")),
        };
        let fname = fd.get_name().to_string();
        let ri = build_region_identifier(fd)?;
        let lists = ri.get_regions_by_block_addrs();
        let mut os = String::new();
        os.push_str("Regions for ");
        os.push_str(&fname);
        os.push_str(": ");
        os.push_str(&lists.len().to_string());
        os.push('\n');
        for list in lists {
            os.push('[');
            for (i, a) in list.iter().enumerate() {
                if i != 0 {
                    os.push_str(", ");
                }
                os.push_str(&format!("0x{a:x}"));
            }
            os.push_str("]\n");
        }
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

/// Visitor for `region walk`: pushes a `walk 0x<addr>` line per leaf block.
struct RegionWalkVisitor {
    out: String,
}

impl kuna_decomp::kuna_regionid::KunaRegionVisitor for RegionWalkVisitor {
    fn visit_block(&mut self, _block: Option<kuna_decomp::context::BlockId>, addr: u64) {
        self.out.push_str(&format!("walk 0x{addr:x}\n"));
    }
}

/// (kuna) `region walk`: visit the region tree's leaf blocks in walk order (C++
/// `IfcKunaRegionWalk`).
struct IfcKunaRegionWalk;

impl IfaceCommandAction for IfcKunaRegionWalk {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        let fd = match dcp.fd.as_ref() {
            Some(fd) => fd,
            None => return Err(IfaceError::execution("No function selected")),
        };
        let fname = fd.get_name().to_string();
        let ri = build_region_identifier(fd)?;
        let mut visitor = RegionWalkVisitor { out: String::new() };
        ri.walk_blocks(&mut visitor)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let mut os = String::new();
        os.push_str("Region walk for ");
        os.push_str(&fname);
        os.push_str(":\n");
        os.push_str(&visitor.out);
        status.file_out(&os);
        Ok(())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

/// (kuna) `functions`: print the canonical executable function inventory.
pub struct IfcKunaFunctions;

impl IfaceCommandAction for IfcKunaFunctions {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let entries = {
            let dcp = dcp_mut(status)?;
            let prog = dcp
                .conf
                .as_mut()
                .ok_or_else(|| IfaceError::execution("No load image present"))?;
            prog.commit_pending_analysis()
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            prog.function_entries_executable()
        };
        let mut out = String::new();
        for entry in entries {
            out.push_str(&format!("{:#x} {}\n", entry.addr.get_offset(), entry.name));
        }
        status.file_out(&out);
        Ok(())
    }

    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

/// (kuna) `function bounds <start> [<end>] [as <name>]`: declare where a
/// function starts and ends.
///
/// The one console primitive for "function F spans `[start,end)`", which kuna
/// otherwise has nowhere to say: every extent is DERIVED (the address-contiguous
/// clip in `funcextent`, and an unbounded flow follow), so on an obfuscated or
/// packed image — where discovery is exactly what fails — the caller had no way
/// to correct it (`docs/re-needs/no-cli-function-boundary-override.md`).
///
/// `start` registers the entry the way `map function` does, so it enumerates,
/// resolves by name and names its call sites.  `end` is EXCLUSIVE and records
/// the entry's declared extent, which bounds every later flow follow of it
/// (`ConsoleProgram::declared_extent` -> `Funcdata::size` -> `FlowInfo::setRange`)
/// and replaces the clip the inventory reports.  Both are plain integers in the
/// default code space — no `parse_machaddr` size grammar, whose `[ram,a,n]` size
/// is indistinguishable from the address width for a small `n` — and the name is
/// keyed by `as` so a declaration that gives a name but no extent cannot have
/// its name read as the end address.
pub struct IfcKunaFunctionBounds;

impl IfaceCommandAction for IfcKunaFunctionBounds {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let start = read_vma(&s.read_token())
            .ok_or_else(|| IfaceError::parse("Missing function start address"))?;
        s.skip_ws();
        let mut tok = s.read_token();
        let mut end = None;
        if !tok.is_empty() && tok != "as" {
            end = Some(
                read_vma(&tok)
                    .ok_or_else(|| IfaceError::parse(format!("Bad end address {tok:?}")))?,
            );
            s.skip_ws();
            tok = s.read_token();
        }
        let name = if tok == "as" {
            s.skip_ws();
            let name = s.read_token();
            if name.is_empty() {
                return Err(IfaceError::parse("Missing name after 'as'"));
            }
            Some(name)
        } else if tok.is_empty() {
            None
        } else {
            return Err(IfaceError::parse(format!("Unexpected token {tok:?} (expected 'as')")));
        };
        let size = match end {
            None => 0,
            Some(end) if end > start => (end - start) as kuna_base::types::int4,
            Some(end) => {
                return Err(IfaceError::parse(format!(
                    "function bounds: end {end:#x} must be above start {start:#x}"
                )))
            }
        };
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_mut()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        let space = prog
            .arch()
            .manage()
            .get_default_code_space()
            .cloned()
            .ok_or_else(|| IfaceError::execution("No default code space"))?;
        let addr = kuna_base::address::Address::new(space, start);
        let name = prog
            .declare_function(addr, name.as_deref(), size)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        status.out(&format!("Declared {name} at {start:#x} spanning {size} bytes\n"));
        Ok(())
    }

    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

/// (kuna) `map prototype <func> <C declaration>`: bind a declared signature to
/// the function NAMED by `<func>`.
///
/// `parse line extern <decl>` binds by the name inside the declaration, so it
/// can only confirm a signature for a function that is already called what the
/// declaration calls it.  The RE case is the other one: an agent that has worked
/// out that `sub_1400055e0` hashes a buffer writes
/// `void *sha256(void *out,void *input)`, and every such declaration landed on a
/// fresh unrelated symbol while the selected function kept its recovered `void
/// sub_1400055e0(uint4 *,uint8 *)` (`docs/re-needs/text-output-silently-ignores.md`).
/// `<func>` is authoritative over the declaration's own name; the declaration
/// supplies the types and the parameter names.
pub struct IfcKunaMapPrototype;

impl IfaceCommandAction for IfcKunaMapPrototype {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        s.skip_ws();
        let func = s.read_token();
        if func.is_empty() {
            return Err(IfaceError::parse("Missing function name"));
        }
        s.skip_ws();
        let decl = s.rest();
        if decl.trim().is_empty() {
            return Err(IfaceError::parse("Missing C declaration"));
        }
        crate::ifacedecomp::bind_prototype(status, &func, &decl)
    }

    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

/// Read a `0x`-prefixed-or-bare hexadecimal VMA token.  `function bounds` takes
/// plain numbers rather than the console address grammar precisely so its size
/// argument cannot be confused with an address width.
fn read_vma(tok: &str) -> Option<u64> {
    let body = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")).unwrap_or(tok);
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(body, 16).ok()
}

// ---------------------------------------------------------------------------
// Registration — IfaceKunaCapability::registerCommands
// ---------------------------------------------------------------------------

/// C++ `IfaceKunaCapability::registerCommands(IfaceStatus *status)`
/// (`kuna_console.cc:22`): register every kuna stage-model console command.
///
/// In C++ the capability is a static singleton discovered by
/// `CapabilityPoint::initializeAll`; the kuna Rust console invokes this directly
/// from its command-registration step (alongside `register_decomp_commands`),
/// the faithful equivalent. The token sequences are byte-identical to the C++
/// `registerCom` calls, so the prefix-expansion the datatests drive
/// (`stage cat` -> `phase catalog`, etc.) is preserved.
pub fn register_kuna_commands(status: &mut IfaceStatus) {
    status.register_com(Box::new(IfcKunaPhaseList), &["phase", "list"]);
    status.register_com(Box::new(IfcKunaPhaseMap), &["phase", "map"]);
    status.register_com(Box::new(IfcKunaPhaseStatus), &["phase", "status"]);
    status.register_com(Box::new(IfcKunaPhaseCatalog), &["phase", "catalog"]);
    // Deprecated aliases: the pre-rename `stage ...` spellings keep working.
    status.register_com(Box::new(IfcKunaPhaseList), &["stage", "list"]);
    status.register_com(Box::new(IfcKunaPhaseMap), &["stage", "map"]);
    status.register_com(Box::new(IfcKunaPhaseStatus), &["stage", "status"]);
    status.register_com(Box::new(IfcKunaPhaseCatalog), &["stage", "catalog"]);
    status.register_com(Box::new(IfcKunaAssert::new()), &["kassert"]);
    status.register_com(Box::new(IfcKunaRestarts), &["restarts"]);
    status.register_com(Box::new(IfcKunaPipeline), &["pipeline"]);
    status.register_com(Box::new(IfcKunaMode), &["mode"]);
    status.register_com(Box::new(IfcKunaQuality), &["quality"]);
    status.register_com(Box::new(IfcKunaFunctions), &["functions"]);
    status.register_com(Box::new(IfcKunaRegionTree), &["region", "tree"]);
    status.register_com(Box::new(IfcKunaRegionBlocks), &["region", "blocks"]);
    status.register_com(Box::new(IfcKunaRegionWalk), &["region", "walk"]);
    // (kuna) Not a C++ command: the function-boundary override the `kuna` binary
    // exposes as `--define-function`.
    status.register_com(Box::new(IfcKunaFunctionBounds), &["function", "bounds"]);
    // (kuna) Not a C++ command either: `parse line extern` binds a prototype by
    // the name in the declaration, which is the wrong key for an override that
    // exists because the function has no name worth keeping.
    status.register_com(Box::new(IfcKunaMapPrototype), &["map", "prototype"]);
}

/// Join tokens with single spaces — C++ `joinTokens(tokens,0,tokens.size())`.
fn join_tokens(tokens: &[String]) -> String {
    tokens.join(" ")
}

#[cfg(test)]
mod tests;
