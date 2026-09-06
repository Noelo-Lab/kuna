//! GH-428: the V850 indirect-branch predicate must use the translator-neutral
//! register lookup in ghidra mode.
//!
//! The option is intentionally absent from Ghidra's upstream `<optionslist>`
//! vocabulary, so this test enables it through the process test seam after
//! `registerProgram`. Before the fix, the first walked op downcasts the live
//! [`GhidraTranslate`](kuna_ghidra::translate::GhidraTranslate) to standalone
//! SLEIGH and panics. A successful `getRegisterName` callback proves the same
//! path now stays behind the engine-neutral translator interface.

mod ghidra_sim;

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::marshal::PackedEncode;
use kuna_ghidra::ids::ELEM_COMMAND_GETREGISTERNAME;
use kuna_ghidra::process::GhidraProcess;

use ghidra_sim::oracle::{generate_tspec, repo_root, SimOracle};
use ghidra_sim::{
    cmd_decompile_at, cmd_deregister_program, cmd_register_program, cmd_set_action, trace_session,
    MockReader, MockState, MockWriter,
};

#[test]
fn v850_predicate_queries_register_names_without_a_sleigh_downcast() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_arm");
    let Some(oracle) = SimOracle::bootstrap(&binary) else {
        return;
    };
    let tspec = generate_tspec(&oracle.manager, oracle.big_endian, oracle.unique_base);
    let lang = repo_root().join("specs/Ghidra/Processors/ARM/data/languages");
    let pspec = std::fs::read(lang.join("ARMt.pspec")).expect("vendored pspec");
    let cspec = std::fs::read(lang.join("ARM.cspec")).expect("vendored cspec");
    let addr = oracle
        .prog
        .find_entry_by_name("main")
        .expect("fmt_arm main")
        .addr;
    let mut packed_addr = Vec::new();
    {
        let mut encoder = PackedEncode::new(&mut packed_addr);
        addr.encode(&mut encoder).expect("entry addr encodes");
    }

    let mut commands = Vec::new();
    cmd_register_program(
        &mut commands,
        &pspec,
        &cspec,
        &tspec,
        ghidra_sim::DEFAULT_CORETYPES_XML,
    );
    cmd_set_action(&mut commands, "0", "decompile", "c");
    cmd_decompile_at(&mut commands, "0", &packed_addr);
    cmd_deregister_program(&mut commands, "0");

    let shared = Rc::new(RefCell::new(MockState::new(commands, oracle)));
    let reader = MockReader {
        shared: Rc::clone(&shared),
    };
    let writer = MockWriter {
        shared: Rc::clone(&shared),
    };
    let mut process = GhidraProcess::new(reader, writer);

    assert_eq!(process.read_command().expect("registerProgram"), 0);
    process
        .set_kuna_option_for_test(0, "v850indirectbranch", "on")
        .expect("enable v850 predicate");
    assert_eq!(process.read_command().expect("setAction"), 0);
    assert_eq!(process.read_command().expect("decompileAt"), 0);
    assert_eq!(process.read_command().expect("deregisterProgram"), 1);

    let _ = process.into_inner();
    let state = Rc::try_unwrap(shared)
        .unwrap_or_else(|_| panic!("mock state still shared"))
        .into_inner();
    let trace = trace_session(&state.from_process);
    let name_queries = state
        .source
        .log
        .counts
        .get(&ELEM_COMMAND_GETREGISTERNAME.get_id())
        .copied()
        .unwrap_or(0);

    assert!(
        name_queries > 0,
        "the option-gated register lookup was not reached"
    );
    assert!(
        trace.responses[2]
            .payload
            .as_deref()
            .is_some_and(|payload| !payload.is_empty()),
        "decompileAt returned no function: {}",
        trace.responses[2].warnings.trim()
    );
    assert!(
        trace.responses[2].warnings.trim().is_empty(),
        "decompileAt warnings: {}",
        trace.responses[2].warnings
    );
}
