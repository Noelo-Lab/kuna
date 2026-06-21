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
//! The kuna stage model is pure static data ([`kuna_decomp::kuna_stages`], the
//! W4 registry), so the **registry-only** commands port in full, byte-for-byte:
//!
//! - [`IfcKunaStageList`] — `stage list`: the stages, Band B membership, and the
//!   sub-stage decision catalog (with `LATENT` markers).
//! - [`IfcKunaStageMap`] — `stage map [<name>]`: resolve an action/rule group,
//!   console surface, or sub-stage to its stage (or dump the full tables).
//! - [`IfcKunaStageCatalog`] — `stage catalog [<option>]`: emit the
//!   LLM-settable assertion catalog as JSON, calling the **W4 byte-compatible
//!   emitter** ([`kuna_decomp::kuna_stages::emit_catalog_json`]). With a program
//!   loaded the live `current` field is joined per option from the
//!   [`Architecture`] flags ([`kuna_live_value`], the port of
//!   `kuna_console.cc kunaLiveValue`). `tests/stages/kuna-catalog.xml`
//!   string-matches this output.
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
//! (`Architecture::print`, for `stage status`'s `arraynotation` line), the
//! restart side-table (`Architecture`-owned `RestartLog`, `SEAM(W5)`), the
//! reduced-pipeline decompile drive (`allacts.getCurrent()->reset/perform`,
//! `clearAnalysis`), and `FlowBlock::nextFlowAfter` (for `BlockGoto::gotoPrints`
//! in the `quality` metric). Each of these commands ports faithfully every part
//! that *is* expressible — its `dcp->conf`/`dcp->fd` null guards with exact text
//! — then routes the engine read through [`engine_unavailable`]:
//!
//! - [`IfcKunaStageStatus`] — `stage status`.
//! - [`IfcKunaRestarts`] — `restarts`.
//! - [`IfcKunaPipeline`] — `pipeline [list|<variant>]` (the `list` sub-form is
//!   fully expressible and ported; running a variant needs the drive).
//! - [`IfcKunaQuality`] — `quality`.
//!
//! The S7 region commands (`region tree`/`blocks`/`walk`, C++ `IfcKunaRegion*`
//! from `kuna_regionid.cc`) register through this capability in C++; their
//! `buildFromBlockGraph` block-graph adapter is explicitly `SEAM(W7)` in the
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
use kuna_decomp::kuna_stages::{
    emit_catalog_json, emit_catalog_json_one, kuna_group_by_index, kuna_num_groups,
    kuna_num_substages, kuna_num_surfaces, kuna_substage_by_index, kuna_surface_by_index,
    lookup_group, lookup_settable, lookup_substage, lookup_surface, KunaStage, KunaStrength,
};
use std::cell::RefCell;

/// The named pipeline variants built by `ActionDatabase::buildDefaultGroups`
/// (C++ `PIPELINE_VARIANTS`).
const PIPELINE_VARIANTS: [&str; 6] =
    ["decompile", "jumptable", "normalize", "paramid", "register", "firstpass"];

/// The stages in registry order P0,S1..S9 — the `for(i=0;i<=9;++i)(KunaStage)i`
/// loop of `IfcKunaStageList::execute`.
const STAGES_IN_ORDER: [KunaStage; 10] = [
    KunaStage::P0,
    KunaStage::S1,
    KunaStage::S2,
    KunaStage::S3,
    KunaStage::S4,
    KunaStage::S5,
    KunaStage::S6,
    KunaStage::S7,
    KunaStage::S8,
    KunaStage::S9,
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
         decompile drive and SEAM(W5/W7/W8) side-tables are a later W-item)"
    ))
}

/// Read the live value of a kuna settable from the loaded [`Architecture`], or
/// `None` if it cannot be determined — port of `kuna_console.cc kunaLiveValue`.
///
/// The C++ `arraynotation` reader dereferences `conf->print` (the owned
/// `PrintC`); that accessor is `SEAM(W8)` in the merged tree (no `print` field
/// on [`Architecture`]), so `arraynotation` returns `None` here — exactly the
/// `kunaLiveValue` `""` path that suppresses the `current` field. Every other
/// option reads a flag/string that [`Architecture`] does expose, so the
/// `current` field is joined for them, matching C++.
pub fn kuna_live_value(conf: &Architecture, option: &str) -> Option<&'static str> {
    // C++ returns "" for an unknown option (suppressing the current field).
    let on_off = |b: bool| if b { "on" } else { "off" };
    Some(match option {
        "compareform" => {
            if conf.present_lessequal {
                "original"
            } else {
                "canonical"
            }
        }
        // SEAM(W8): conf->print (PrintC::getArrayNotation) is not exposed; the
        // C++ "" path -> no current field.
        "arraynotation" => return None,
        "thumbfuncptr" => on_off(conf.preserve_thumb_funcptr),
        "inferfuncentry" => on_off(conf.infer_funcentry),
        "booleanmask" => on_off(conf.fold_boolean_mask),
        "ovlesssimplify" => on_off(conf.ov_less_simplify),
        "flagcompare" => on_off(conf.fold_flag_compare),
        "addcarrychain" => on_off(conf.add_carry_chain),
        "memsetrecover" => on_off(conf.memset_recover),
        "returnpair" => {
            if conf.return_single {
                "single"
            } else {
                "pair"
            }
        }
        "inputvarnodeadjust" => on_off(conf.input_varnode_adjust),
        "condexeplace" => on_off(conf.condexe_block_placement),
        "sparcstructret" => on_off(conf.sparc_struct_return),
        "arraystride" => on_off(conf.recover_array_stride),
        "stackalias" => on_off(conf.stack_alias_deadstore),
        "dynamichashmax" => on_off(conf.dynamic_hash_maxdup_high),
        "stackprobeloop" => on_off(conf.model_stack_probe_loop),
        "v850indirectbranch" => on_off(conf.v850_indirect_branch),
        "switchmodbound" => on_off(conf.switch_modulo_bound),
        "realtypes" => on_off(conf.realtypes),
        // C++: return ""; -> no current field.
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// `stage list` — IfcKunaStageList
// ---------------------------------------------------------------------------

/// (kuna) `stage list`: print the stage model and sub-stage catalog
/// (C++ `IfcKunaStageList`). Pure static data; requires no program.
pub struct IfcKunaStageList;

impl IfaceCommandAction for IfcKunaStageList {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut os = String::new();
        os.push_str("Stages (kuna stage model, docs/stages.md / docs/stage-model.md):\n");
        for stage in STAGES_IN_ORDER {
            os.push_str("  ");
            os.push_str(stage.code());
            os.push_str("  ");
            os.push_str(stage.name());
            if stage == KunaStage::P0 {
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
        os.push_str("Sub-stages (named decision points; LATENT = no override surface today):\n");
        for i in 0..kuna_num_substages() {
            let sub = kuna_substage_by_index(i);
            os.push_str("  [");
            os.push_str(sub.stage.code());
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
// `stage map [<name>]` — IfcKunaStageMap
// ---------------------------------------------------------------------------

/// (kuna) `stage map [<name>]`: resolve a group/surface/sub-stage to its stage
/// (C++ `IfcKunaStageMap`). With no argument, dump the full tables.
pub struct IfcKunaStageMap;

impl IfaceCommandAction for IfcKunaStageMap {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++ builds a space-joined token from the rest of the line:
        //   while(!s.eof()) { s >> word >> ws; if(word.empty()) break;
        //     if(!token.empty()) token += ' '; token += word; }
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
            os.push_str("Action/rule groups -> stage (dominant artifact; see stage-model.md s15 for straddlers):\n");
            for i in 0..kuna_num_groups() {
                let entry = kuna_group_by_index(i);
                os.push_str("  ");
                os.push_str(entry.stage.code());
                os.push_str("  ");
                os.push_str(entry.group);
                if !entry.substage.is_empty() {
                    os.push_str("  (");
                    os.push_str(entry.substage);
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
            os.push_str("Console surfaces -> stage:\n");
            for i in 0..kuna_num_surfaces() {
                let entry = kuna_surface_by_index(i);
                os.push_str("  ");
                os.push_str(entry.stage.code());
                os.push_str("  ");
                os.push_str(entry.surface);
                if !entry.substage.is_empty() {
                    os.push_str("  (");
                    os.push_str(entry.substage);
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
            os.push_str(grp.stage.code());
            os.push_str(" (");
            os.push_str(grp.stage.name());
            os.push(')');
            if !grp.substage.is_empty() {
                os.push_str(" sub-stage ");
                os.push_str(grp.substage);
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
            os.push_str(surf.stage.code());
            os.push_str(" (");
            os.push_str(surf.stage.name());
            os.push(')');
            if !surf.substage.is_empty() {
                os.push_str(" sub-stage ");
                os.push_str(surf.substage);
            }
            os.push('\n');
            if !surf.note.is_empty() {
                os.push_str("  ");
                os.push_str(surf.note);
                os.push('\n');
            }
        }
        if let Some(sub) = lookup_substage(&token) {
            found = true;
            os.push_str("sub-stage ");
            os.push_str(sub.name);
            os.push_str(" -> ");
            os.push_str(sub.stage.code());
            os.push_str(" (");
            os.push_str(sub.stage.name());
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
                "Unknown group/surface/sub-stage: {token}"
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
// `stage status` — IfcKunaStageStatus
// ---------------------------------------------------------------------------

/// (kuna) `stage status`: report the active pipeline variant and the state of
/// kuna sub-stage options (C++ `IfcKunaStageStatus`).
///
/// The first two lines (pipeline variant, compareform) read accessors the
/// merged tree exposes; the third (`arraynotation`) dereferences `conf->print`
/// (`SEAM(W8)`), which the architecture does not expose. Because the exact
/// three-line output cannot be reproduced byte-for-byte today, the engine read
/// is routed through [`engine_unavailable`] after the `No load image present`
/// guard, exactly as the merged `ifacedecomp` commands do for unported `print`
/// accessors.
pub struct IfcKunaStageStatus;

impl IfaceCommandAction for IfcKunaStageStatus {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        // C++: allacts.getCurrentName() + present_lessequal are expressible, but
        // the arraynotation line needs conf->print (PrintC::getArrayNotation),
        // which is SEAM(W8).
        Err(engine_unavailable(
            "Architecture::print (PrintC::getArrayNotation) for the arraynotation status line",
        ))
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

// ---------------------------------------------------------------------------
// `stage catalog [<option>]` — IfcKunaStageCatalog
// ---------------------------------------------------------------------------

/// (kuna) `stage catalog [<option>]`: emit the LLM-settable assertion catalog as
/// JSON (C++ `IfcKunaStageCatalog`).
///
/// Calls the W4 byte-compatible emitter
/// ([`kuna_decomp::kuna_stages::emit_catalog_json`] /
/// [`emit_catalog_json_one`]); the live `current` field is joined per option
/// from the loaded [`Architecture`] via [`kuna_live_value`] (C++
/// `kunaLiveValue`). Works with no program loaded (the static doc form).
pub struct IfcKunaStageCatalog;

impl IfaceCommandAction for IfcKunaStageCatalog {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++: s >> ws >> option;
        s.skip_ws();
        let option = s.read_token();

        let dcp = dcp_mut(status)?;
        let conf = dcp.conf.as_ref();

        let out = if !option.is_empty() {
            // Single-option form: `stage catalog <option>`.
            if lookup_settable(&option).is_none() {
                return Err(IfaceError::execution(format!(
                    "Unknown settable option: {option} (try `stage catalog`)"
                )));
            }
            let live = conf.and_then(|c| kuna_live_value(c.arch(), &option));
            // lookup_settable just succeeded, so this is Some.
            emit_catalog_json_one(&option, live).unwrap_or_default()
        } else {
            // Full form: `stage catalog`. The emitter calls the closure per
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

/// (kuna) `kassert <stage> <substage> <args...> [hard|hint]` / `kassert list`:
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
        // C++: if (dcp->conf == 0) throw IfaceExecutionError("No load image present");
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }

        // C++: s >> ws >> stagecode;  if (stagecode == "list") { listAssertions(); return; }
        s.skip_ws();
        let stagecode = s.read_token();
        if stagecode == "list" {
            // listAssertions writes the bulk stream.
            let text = self.assert_log.borrow().render_list();
            status.file_out(&text);
            return Ok(());
        }

        // C++: s >> ws >> subname;
        s.skip_ws();
        let subname = s.read_token();

        // C++ tokenizeRest(s,tokens): collect the rest of the line.
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
        let _record = KunaAssertion {
            func_name,
            stage: validated.stage,
            substage: validated.substage.clone(),
            args: join_tokens(&tokens),
            strength: validated.requested,
            applied: validated.applied,
            rewind: validated.rewind,
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
                    "Sub-stage {subname} is LATENT: no override surface exists yet (kuna roadmap)"
                )))
            }
            Dispatch::Unroutable => {
                // C++: throw IfaceExecutionError("Sub-stage <name> is not yet
                //   routable through kassert; use its native surface: <exposure>")
                let exposure = lookup_substage(&subname).map(|sub| sub.exposure).unwrap_or("");
                Err(IfaceError::execution(format!(
                    "Sub-stage {subname} is not yet routable through kassert; use its native surface: {exposure}"
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
            Dispatch::Rename => Err(engine_unavailable(
                "Scope::renameSymbol (IfaceDecompData::readSymbol) for kassert naming-policy",
            )),
            Dispatch::Option(name) => Err(engine_unavailable(&format!(
                "OptionDatabase::set (option {name}, Architecture: ArchOptionContext) for kassert"
            ))),
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
/// (`SEAM(W5)`: the log and its trigger sites land together), so the dump is
/// routed through [`engine_unavailable`] after the `No function selected`
/// guard.
pub struct IfcKunaRestarts;

impl IfaceCommandAction for IfcKunaRestarts {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable(
            "kunaDumpRestarts (Architecture-owned RestartLog)",
        ))
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
        os.push_str("Named pipeline variants (group filters over the universal action; P0 pipeline-variant sub-stage):\n");
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
        // C++: s >> ws >> name; if (name.empty() || name=="list") { listPipelines(); return; }
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
        // C++: if (fd->hasNoCode()) { *optr << "No code for " << name << endl; return; }
        // hasNoCode IS expressible; the message uses the command stream (optr).
        let fd = dcp.fd.as_ref().expect("fd checked Some above");
        if fd.has_no_code() {
            let line = format!("No code for {}\n", fd.get_name());
            status.out(&line);
            return Ok(());
        }
        // The remainder (clearAnalysis, allacts.setCurrent/reset/perform/restore)
        // is the reduced-pipeline decompile drive, not assembled at the
        // Architecture level in the merged tree.
        Err(engine_unavailable(
            "ActionDatabase reduced-pipeline drive (clearAnalysis + getCurrent()->reset/perform)",
        ))
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
/// (`SEAM(W7)`, unported). Because that exact line cannot be reproduced today,
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
        // C++: if (fd->hasNoStructBlocks()) { os << "No structured blocks for "
        //   << name << " (decompile first)" << endl; return; }
        if fd.has_no_struct_blocks() {
            let line = format!(
                "No structured blocks for {} (decompile first)\n",
                fd.get_name()
            );
            status.file_out(&line);
            return Ok(());
        }
        // The goto walk needs BlockGoto::gotoPrints (FlowBlock::nextFlowAfter,
        // SEAM(W7)) for the `(printed: M)` count.
        Err(engine_unavailable(
            "BlockGoto::gotoPrints (FlowBlock::nextFlowAfter) for the goto-count metric",
        ))
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
/// The region identifier itself is ported (`kuna_decomp::kuna_regionid`), but
/// its `buildFromBlockGraph` block-graph adapter — which these console commands
/// drive — is explicitly `SEAM(W7)` (it needs `FlowBlock::getStart()`/`lastOp()`
/// over a decompiled function). So the three commands register under their token
/// sequences (the prefix surface the datatests rely on) and route through
/// [`engine_unavailable`] after the `No function selected` guard.
///
/// `entry` names the exact unported region adapter for the self-describing
/// console error.
struct IfcKunaRegionStub {
    /// The unported entry point named in the `engine_unavailable` message.
    entry: &'static str,
}

impl IfaceCommandAction for IfcKunaRegionStub {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable(self.entry))
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
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
/// (`stage cat` -> `stage catalog`, etc.) is preserved.
pub fn register_kuna_commands(status: &mut IfaceStatus) {
    status.register_com(Box::new(IfcKunaStageList), &["stage", "list"]);
    status.register_com(Box::new(IfcKunaStageMap), &["stage", "map"]);
    status.register_com(Box::new(IfcKunaStageStatus), &["stage", "status"]);
    status.register_com(Box::new(IfcKunaStageCatalog), &["stage", "catalog"]);
    status.register_com(Box::new(IfcKunaAssert::new()), &["kassert"]);
    status.register_com(Box::new(IfcKunaRestarts), &["restarts"]);
    status.register_com(Box::new(IfcKunaPipeline), &["pipeline"]);
    status.register_com(Box::new(IfcKunaQuality), &["quality"]);
    status.register_com(
        Box::new(IfcKunaRegionStub {
            entry: "KunaRegionIdentifier::buildFromBlockGraph (region tree, SEAM W7)",
        }),
        &["region", "tree"],
    );
    status.register_com(
        Box::new(IfcKunaRegionStub {
            entry: "KunaRegionIdentifier::buildFromBlockGraph (region blocks, SEAM W7)",
        }),
        &["region", "blocks"],
    );
    status.register_com(
        Box::new(IfcKunaRegionStub {
            entry: "KunaRegionIdentifier::buildFromBlockGraph (region walk, SEAM W7)",
        }),
        &["region", "walk"],
    );
}

/// Join tokens with single spaces — C++ `joinTokens(tokens,0,tokens.size())`.
fn join_tokens(tokens: &[String]) -> String {
    tokens.join(" ")
}

#[cfg(test)]
mod tests;
