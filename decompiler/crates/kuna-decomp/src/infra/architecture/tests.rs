//! Porter in-module tests for `architecture.rs`.
//!
//! Two strands:
//!   1. **Defaults + flags + naming**: a minimal `Architecture` over a fresh
//!      (uninitialized) Sleigh — exercises the verbatim `resetDefaultsInternal`
//!      transcription, the kuna anchor flags, `nameFunction`, and the warning
//!      sink without needing a built `.sla`.
//!   2. **Real-`.sla` bootstrap**: builds an `Architecture` over a Sleigh with a
//!      decoded `.sla` (skips gracefully when specs are unbuilt), then lifts a
//!      `Funcdata` through it (the W3 boot seam).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{Translate, UniqueLayout};

use super::*;
use crate::flow::flow_flags;
use crate::options::split_datatype;

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// A bare (uninitialized) Sleigh — enough to construct an `Architecture` and
/// read its defaults; it has no spaces (no `.sla`), so it cannot lift.
fn bare_sleigh() -> Sleigh {
    Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

// --- Strand 1: defaults + flags + naming -----------------------------------

#[test]
fn reset_defaults_internal_matches_cpp() {
    let arch = Architecture::new("test:LE:32", bare_sleigh());
    // Verbatim from architecture.cc:1420 (kuna DIV-2/DIV-3 defaults).
    assert_eq!(arch.trim_recurse_max, 5);
    assert_eq!(arch.max_implied_ref, 2);
    assert_eq!(arch.max_term_duplication, 2);
    assert_eq!(arch.max_basetype_size, 10);
    assert_eq!(arch.flowoptions, flow_flags::error_toomanyinstructions);
    assert_eq!(arch.max_instructions, 100000);
    assert_eq!(arch.max_jumptable_size, 1024);
    assert_eq!(arch.alias_block_level, 2);
    assert_eq!(
        arch.split_datatype_config,
        split_datatype::OPTION_STRUCT | split_datatype::OPTION_ARRAY | split_datatype::OPTION_POINTER
    );
    // ctor one-offs.
    assert_eq!(arch.min_funcsymbol_size, 1);
    assert!(!arch.aggressive_ext_trim);
    assert_eq!(arch.funcptr_align, 0);
    // plain bools.
    assert!(arch.infer_pointers);
    assert!(!arch.readonlypropagate);
    assert!(arch.analyze_for_loops);
    assert!(!arch.nan_ignore_all);
    assert!(arch.nan_ignore_compare);
    assert!(!arch.loadersymbols_parsed);
}

#[test]
fn kuna_anchor_flags_default_to_div_values() {
    let arch = Architecture::new("test:LE:32", bare_sleigh());
    // DIV-2/DIV-3 default-on (architecture.cc:1430-1451).
    assert!(arch.infer_funcentry);
    assert!(arch.memset_recover);
    assert!(arch.ov_less_simplify);
    assert!(arch.fold_boolean_mask);
    assert!(arch.input_varnode_adjust);
    assert!(arch.dynamic_hash_maxdup_high);
    assert!(arch.fold_flag_compare);
    assert!(arch.recover_array_stride);
    assert!(arch.recover_lowered_switch);
    assert!(arch.name_style_angr);
    assert!(arch.condexe_block_placement);
    assert!(arch.add_carry_chain);
    assert!(arch.model_stack_probe_loop);
    assert!(arch.present_lessequal);
    assert!(arch.preserve_thumb_funcptr);
    // default-off (upstream byte-identical).
    assert!(!arch.return_single);
    assert!(!arch.v850_indirect_branch);
    assert!(!arch.sparc_struct_return);
    assert!(!arch.switch_modulo_bound);
    assert!(!arch.switch_multi_pred);
    assert!(!arch.unrolled_guard);
    assert!(!arch.stack_alias_deadstore);
    assert!(!arch.strip_stack_guard);
}

#[test]
fn reset_defaults_is_idempotent_after_mutation() {
    let mut arch = Architecture::new("t", bare_sleigh());
    arch.infer_funcentry = false;
    arch.max_instructions = 7;
    arch.reset_defaults();
    assert!(arch.infer_funcentry);
    assert_eq!(arch.max_instructions, 100000);
}

#[test]
fn name_function_angr_vs_upstream() {
    let mut arch = Architecture::new("t", bare_sleigh());
    // Need a space to make an address; use a constant-space address (any space
    // suffices for the textual default name).
    let ram = kuna_base::space::AddrSpace::new(
        kuna_base::space::spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        7,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    );
    let addr = Address::new(Rc::new(ram), 0x401000);

    // angr-style on by default: sub_<addr> (kuna_function_name).
    assert!(arch.name_style_angr);
    let n_angr = arch.name_function(&addr);
    assert!(n_angr.starts_with("sub_"), "angr name was {n_angr}");

    // upstream style: func_<raw-addr>, where raw-addr is C++ Address::printRaw
    // (AddrSpace::printRaw): zero-padded to the address width (4 bytes -> 8 hex
    // digits), no space-name prefix, word-size division (wordsize 1 here).
    arch.name_style_angr = false;
    let n_up = arch.name_function(&addr);
    assert_eq!(n_up, "func_0x00401000", "upstream name was {n_up}");
}

#[test]
fn comment_database_dedups_matching_text() {
    let mut db = CommentDatabase::new();
    let ram = Rc::new(kuna_base::space::AddrSpace::new(
        kuna_base::space::spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        7,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let fad = Address::new(Rc::clone(&ram), 0x1000);
    let ad = Address::new(Rc::clone(&ram), 0x1004);
    assert!(db.add_comment_no_duplicate(comment_type::warning, &fad, &ad, "WARNING: x"));
    // Identical (fad,ad,text) is dropped (returns false).
    assert!(!db.add_comment_no_duplicate(comment_type::warning, &fad, &ad, "WARNING: x"));
    // Different text at the same address is stored.
    assert!(db.add_comment_no_duplicate(comment_type::warning, &fad, &ad, "WARNING: y"));
    assert_eq!(db.comments().len(), 2);
}

#[test]
fn version_constants_via_capability() {
    // The architecture's capability version constants are the decompiler
    // version (architecture.cc:36-37).
    assert_eq!(crate::capability::get_major_version(), 6);
    assert_eq!(crate::capability::get_minor_version(), 1);
}

#[test]
fn inject_tempbase_comes_from_unique_start() {
    // The injection library's tempbase is translate->getUniqueStart(INJECT)
    // (PcodeInjectLibrarySleigh ctor).  With a fresh engine the unique base is
    // 0, so getUniqueStart(INJECT) == 0x300 (the INJECT offset over base 0).
    let sleigh = bare_sleigh();
    let expected = sleigh.get_unique_start(UniqueLayout::INJECT);
    let arch = Architecture::new("t", sleigh);
    assert_eq!(arch.pcodeinjectlib.base.tempbase, expected);
}

// --- Strand 2: real-`.sla` bootstrap + Funcdata lift -----------------------

fn build_arch_from_sla(sla_rel: &str) -> Option<Architecture> {
    let root = repo_root();
    let sla_path = root.join("specs").join(sla_rel);
    if !sla_path.is_file() {
        eprintln!("SKIP: {} not built (run `make specs`)", sla_path.display());
        return None;
    }
    let sla = std::fs::read(&sla_path).expect("read .sla");
    let mut sleigh = Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&sla).expect("initialize from sla");
    Some(Architecture::new("ARM:LE:32:v7", sleigh))
}

#[test]
fn real_sla_bootstraps_subsystems_and_global_scope() {
    let Some(arch) = build_arch_from_sla("Ghidra/Processors/ARM/data/languages/ARM7_le.sla") else {
        return;
    };
    // The manager has real spaces from the .sla (ram, register, unique, ...).
    assert!(arch.manage().num_spaces() > 2, "decoded .sla yields multiple spaces");
    assert!(arch.manage().get_constant_space().is_some());
    assert!(arch.manage().get_unique_space().is_some());
    // The symbol table has a global scope attached (buildDatabase).
    assert!(arch.symboltab.get_global_scope().is_some());
}

#[test]
fn real_sla_creates_funcdata_at_entry() {
    let Some(mut arch) = build_arch_from_sla("Ghidra/Processors/ARM/data/languages/ARM7_le.sla")
    else {
        return;
    };
    // C++ `restoreFromSpec` inserts the fspec/iop/join spaces into the single
    // engine manager (LOSS-132); `init_post_engine` is that step in the Rust
    // port.  The funcdata's `glb` then shares that one manager, so it carries
    // const/unique/iop/fspec exactly as the lift+analyze path sees them.
    arch.init_post_engine().expect("init_post_engine");
    // Pick the default code space for the function entry.
    let code = Rc::clone(arch.manage().get_default_code_space().expect("default code space"));
    let entry = Address::new(code, 0x8000);
    let fd = arch.new_funcdata("main", entry.clone(), 0x40).expect("create funcdata");
    assert_eq!(fd.get_name(), "main");
    assert_eq!(fd.get_address(), &entry);
    assert_eq!(fd.get_size(), 0x40);
    // The funcdata's IR-boundary glb carries const/unique/iop/fspec spaces.
    assert!(fd.get_arch().manage().get_constant_space().is_some());
    assert!(fd.get_arch().manage().get_iop_space().is_some());
    assert!(fd.get_arch().manage().get_fspec_space().is_some());
    // uniq_start was the analysis unique-start (an absolute high offset).
    let analysis_start = arch.translate().get_unique_start(UniqueLayout::ANALYSIS);
    assert!(analysis_start >= 0x1000_0000);
}

// ===========================================================================
// w9x-arch-engine-glue: owned-subsystem accessors + ArchOptionContext.
// ===========================================================================

use crate::options::{ArchOptionContext, BraceCategory, BraceStyle, NamespaceStrategy};

#[test]
fn owned_subsystems_are_constructed_and_accessible() {
    let mut arch = Architecture::new("t", bare_sleigh());
    // The type factory + printer exist immediately (empty, as the C++ ctor
    // leaves them null until init; the Rust port default-constructs them).
    assert_eq!(arch.print().get_name(), "c-language");
    // The proto registry starts empty (no cspec parsed).
    assert!(arch.default_fp().is_none());
    assert!(!arch.has_model("unknown"));
    // build_default_proto seeds an `unknown` default model.
    arch.build_default_proto();
    assert!(arch.has_model("unknown"));
    assert!(arch.default_fp().is_some());
    assert_eq!(arch.get_model("unknown").unwrap().get_name(), "unknown");
    // The full type-factory finish (`finish_typegrp` -> `get_default_size`)
    // needs a decoded `.sla`'s default code space; the e2e gate exercises that
    // real path.  Here (a bare Sleigh, no spaces) the factory is the empty
    // container `build_typegrp` produces.
    arch.build_typegrp();
}

#[test]
fn build_instructions_populates_the_inst_table() {
    use kuna_num::opcodes::OpCode;
    use crate::op::pcodeop_flags;
    let mut arch = Architecture::new("t", bare_sleigh());
    arch.build_instructions();
    // A BRANCH resolves to a TypeOp carrying the branch + coderef flags.
    let to = arch.resolve_typeop(OpCode::CPUI_BRANCH);
    assert_eq!(to.get_opcode(), OpCode::CPUI_BRANCH);
    assert!(to.get_flags() & pcodeop_flags::branch != 0);
    assert!(to.get_flags() & pcodeop_flags::coderef != 0);
}

#[test]
fn arch_option_context_mutates_config_fields() {
    let mut arch = Architecture::new("t", bare_sleigh());
    // Plain config fields round-trip through the trait.
    arch.set_readonly_propagate(true);
    assert!(arch.readonlypropagate);
    arch.set_infer_pointers(false);
    assert!(!arch.infer_pointers);
    arch.set_max_jumptable_size(64);
    assert_eq!(arch.max_jumptable_size, 64);
    arch.set_alias_block_level(1);
    assert_eq!(ArchOptionContext::alias_block_level(&arch), 1);
    arch.set_nan_ignore_all(true);
    assert!(arch.nan_ignore_all());
}

#[test]
fn arch_option_context_drives_the_printer() {
    let mut arch = Architecture::new("t", bare_sleigh());
    assert!(arch.print_is_c_language());
    assert!(!arch.print().options.null);
    arch.set_null_printing(true);
    assert!(arch.print().options.null);
    arch.set_convention_printing(false);
    assert!(!arch.print().options.convention);
    arch.set_brace_format(BraceCategory::Function, BraceStyle::SameLine);
    assert_eq!(arch.print().options.brace_func, BraceStyle::SameLine);
    arch.set_namespace_strategy(NamespaceStrategy::None);
    arch.set_integer_format("hex");
    // Switching the print language flips print_is_c_language.
    arch.set_print_language("not-c");
    assert!(!arch.print_is_c_language());
}

#[test]
fn arch_option_context_proto_model_registry() {
    let mut arch = Architecture::new("t", bare_sleigh());
    arch.build_default_proto();
    // set_default_extra_pop mutates the shared default model.
    arch.set_default_extra_pop(8);
    assert_eq!(arch.default_fp().unwrap().get_extra_pop(), 8);
    // set_default_model resolves a registered model and errors on unknown.
    assert!(arch.set_default_model("unknown").is_ok());
    let err = arch.set_default_model("no-such-model").unwrap_err();
    assert!(format!("{err}").contains("Unknown prototype model"));
    // Unknown function (no symbol table function) errors faithfully.
    let err = arch.set_function_extra_pop("nofunc", 4).unwrap_err();
    assert!(format!("{err}").contains("Unknown function name"));
}
