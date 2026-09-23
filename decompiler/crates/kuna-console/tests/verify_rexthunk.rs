//! End-to-end gate for `rexthunk`: on an x86-64 PE, the `FF 25` one byte into a
//! REX-prefixed `48 FF 25` jump through an import slot is not an import thunk,
//! while a real thunk after a byte in the same range keeps its name.
//!
//! `pe_rexthunk_x86_64.exe` (generator `pe_rexthunk_x86_64.py`) holds the MSVC
//! `__scrt_initialize_type_info` shape at 0x140001030 (`lea rcx,[list]; jmp qword
//! ptr [__imp_InitializeSListHead]`, REX.W-prefixed), a bare linker thunk through
//! the same slot at 0x140001040, and a called `ExitProcess` thunk at 0x140001057
//! directly after a function ending in `jmp qword ptr [rax+0x48]` (`48 FF 60 48`).
//! The import-thunk scan matched the `FF 25` at 0x140001038, one byte into the
//! tail jump, and registered it as a second `InitializeSListHead` function.
//! `pe_rexthunk_far_x86_64.exe` is a contiguous link.exe-style thunk table more
//! than 1 GiB below its IAT, where the byte before each later thunk is `40`, the
//! previous displacement's high byte, and no code calls any of them.
//! `pe_rexthunk_reach_x86_64.exe` puts each real thunk directly after a function
//! ending in `48 FF 60 48` and reaches it only through a pointer, an `imm64`, a
//! function table, a `.pdata` record, an export or an `E9` stub.
//!
//! The gate is a process-global env var (import names are resolved inside `load
//! file`, upstream of every per-function `option`), so each test holds one mutex
//! for its whole run.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). When absent the bootstrap fails, and the test prints that and
//! returns early.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_rexthunk::REXTHUNK_ENV;

static GATE: Mutex<()> = Mutex::new(());

const ENTRY: u64 = 0x140001000;
const WRAPPER: u64 = 0x140001030;
const MID_INSTRUCTION: u64 = 0x140001038;
const THUNK: u64 = 0x140001040;
const EXIT_THUNK: u64 = 0x140001057;
const SLOT: u64 = 0x140002050;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Take the gate for the whole test, so no other test flips the env var while this
/// one loads or decompiles.
fn lock() -> MutexGuard<'static, ()> {
    GATE.lock().unwrap_or_else(|e| e.into_inner())
}

/// Load a fixture with the gate forced on/off, or left unset (`None`) for the
/// shipped default. Call under [`lock`]. `None` back means the `.sla` is absent.
fn boot_fixture(name: &str, gate: Option<bool>) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    match gate {
        Some(on) => std::env::set_var(REXTHUNK_ENV, if on { "on" } else { "off" }),
        None => std::env::remove_var(REXTHUNK_ENV),
    }
    let mut prog = match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_rexthunk: skipping (bootstrap failed; build `.sla` with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn boot(gate: Option<bool>) -> Option<ConsoleProgram> {
    boot_fixture("pe_rexthunk_x86_64.exe", gate)
}

fn decompile_at(prog: ConsoleProgram, vma: u64) -> String {
    let cmds: Vec<String> = [format!("load addr 0x{vma:x}"), "decompile".into(), "print C".into()]
        .into_iter()
        .collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

fn executable_entries(prog: &ConsoleProgram) -> Vec<u64> {
    prog.function_entries_executable().iter().map(|e| e.addr.get_offset()).collect()
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    prog.find_entry_at(vma).unwrap_or_else(|| panic!("no entry at {vma:#x}")).name.clone()
}

/// The shipped default: no entry one byte into the wrapper's tail jump, while both
/// real thunks and the slot keep their import names. The wrapper itself is a call
/// target, which the console bootstrap leaves to the CLI's discovery walk.
#[test]
fn rex_tail_is_not_a_thunk_and_real_thunks_keep_their_names() {
    let _gate = lock();
    let Some(prog) = boot(None) else { return };
    assert_eq!(executable_entries(&prog), vec![ENTRY, THUNK, EXIT_THUNK]);
    assert!(prog.find_entry_at(MID_INSTRUCTION).is_none());
    assert_eq!(name_at(&prog, THUNK), "InitializeSListHead");
    assert_eq!(name_at(&prog, EXIT_THUNK), "ExitProcess");
    assert_eq!(name_at(&prog, SLOT), "InitializeSListHead");
}

/// The wrapper's tail jump still names the import through the slot.
#[test]
fn wrapper_tail_jump_still_names_the_import() {
    let _gate = lock();
    let Some(prog) = boot(None) else { return };
    let out = decompile_at(prog, WRAPPER);
    assert!(out.contains("InitializeSListHead(0x140002100);"), "got:\n{out}");
}

/// The caller reaches the wrapper by its own entry, the import through the bare
/// thunk, and `ExitProcess` through the thunk whose preceding byte is `48`, with
/// its no-return fact intact.
#[test]
fn caller_names_both_thunks_and_keeps_the_wrapper() {
    let _gate = lock();
    let Some(prog) = boot(None) else { return };
    let out = decompile_at(prog, ENTRY);
    assert!(out.contains("sub_140001030();"), "got:\n{out}");
    // `globalref` (default on) names the list head `&dat_140002100`; either
    // spelling types the argument.
    assert!(
        out.contains("InitializeSListHead((void *)0x140002100);")
            || out.contains("InitializeSListHead(&dat_140002100);"),
        "got:\n{out}"
    );
    assert!(out.contains("ExitProcess(0); // no-return"), "got:\n{out}");
}

/// Gate off restores the previous inventory, mid-instruction entry included.
#[test]
fn gate_off_restores_the_mid_instruction_thunk() {
    let _gate = lock();
    let Some(prog) = boot(Some(false)) else { return };
    assert_eq!(executable_entries(&prog), vec![ENTRY, MID_INSTRUCTION, THUNK, EXIT_THUNK]);
    assert_eq!(name_at(&prog, MID_INSTRUCTION), "InitializeSListHead");
    assert_eq!(name_at(&prog, EXIT_THUNK), "ExitProcess");
}

/// Gate on explicitly matches the unset default.
#[test]
fn gate_on_matches_the_default() {
    let _gate = lock();
    let Some(prog) = boot(Some(true)) else { return };
    assert_eq!(executable_entries(&prog), vec![ENTRY, THUNK, EXIT_THUNK]);
}

/// Every thunk of a far contiguous table keeps its name although the byte before
/// each later one is `40` and nothing calls them: the six bytes before it are the
/// previous thunk.
#[test]
fn far_contiguous_table_keeps_every_thunk() {
    let _gate = lock();
    let Some(prog) = boot_fixture("pe_rexthunk_far_x86_64.exe", None) else { return };
    assert_eq!(name_at(&prog, 0x140001020), "GetTickCount");
    assert_eq!(name_at(&prog, 0x140001026), "Sleep");
    assert_eq!(name_at(&prog, 0x14000102c), "ExitProcess");
}

/// Every real thunk after a `48` byte keeps its import name when the image reaches
/// it through an absolute pointer, an `imm64`, a table, `.pdata`, an export or an
/// `E9` stub, as does the control thunk after `CC`.
#[test]
fn thunks_reached_without_rel32_keep_their_names() {
    let _gate = lock();
    let Some(prog) = boot_fixture("pe_rexthunk_reach_x86_64.exe", None) else { return };
    for (vma, name) in [
        (0x140001207, "ExitProcess"),
        (0x140001217, "ExitThread"),
        (0x140001227, "GetTickCount"),
        (0x140001237, "Sleep"),
        (0x140001247, "GetLastError"),
        (0x140001257, "Beep"),
        (0x140001267, "GetCurrentProcessId"),
        (0x140001297, "GetVersion"),
        (0x1400012a7, "RaiseFailFastException"),
        (0x1400012b4, "FatalExit"),
    ] {
        assert_eq!(name_at(&prog, vma), name, "thunk at {vma:#x}");
    }
}

/// A no-return call through an `imm64` or a pointer in `.text` as a function's last
/// instruction stays a call of `ExitProcess`, and the next function's body stays out.
#[test]
fn noreturn_calls_through_pointers_keep_the_import() {
    let _gate = lock();
    for (vma, next_body) in [(0x140001170u64, "return 7;"), (0x140001150, "0x140002100")] {
        let Some(prog) = boot_fixture("pe_rexthunk_reach_x86_64.exe", None) else { return };
        let out = decompile_at(prog, vma);
        assert!(out.contains("ExitProcess(0); // no-return"), "{vma:#x} got:\n{out}");
        assert!(!out.contains(next_body), "{vma:#x} absorbed the next function:\n{out}");
    }
}
