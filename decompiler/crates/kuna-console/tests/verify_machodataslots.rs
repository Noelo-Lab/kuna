//! Mach-O symbol-pointer slots whose import is a data object (pass
//! `machodataslots`, committed under `importdata`): a `__got` slot holding an
//! Objective-C class object, a Swift type's metadata or a C++ static data member
//! is not a function-inventory entry, and a read through it renders the import.
//!
//! Fixture: `tests/fixtures/macho_dataslots_arm64`, built from
//! `macho_dataslots.c` (see the fixtures README). Four readers each load one
//! `__got` slot and read through it; `main` calls `imported_call` through its stub.
//!
//! - **Inventory**: the three data-import slots are not entries; the plain C
//!   `imported_counter` slot, which carries no mark, still is, as is the stub.
//! - **Lookup**: a data-import slot's name selects the slot as a data object.
//! - **`importdata` off**: the four reads are `*dat_<slot>`.
//! - **`importdata` on** (default): they are the imports.
//! - **Addresses** (`macho_dataslotaddr_arm64`, from `macho_dataslotaddr.c`): a
//!   slot's own value passed on is `&<import>` when on, `dat_<slot>` when off.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

const BIN: &str = "macho_dataslots_arm64";

/// `(function VMA, slot VMA, expected off, expected on)`.
const READS: [(u64, u64, &str, &str); 4] = [
    (0x100000510, 0x100004000, "return *dat_100004000;", "return OBJC_CLASS_$_NSObject;"),
    (0x100000520, 0x100004008, "return *dat_100004008;", "return Swift.String.type_metadata;"),
    (0x100000530, 0x100004010, "return *dat_100004010;", "return id;"),
    (0x100000540, 0x100004018, "return *dat_100004018;", "return imported_counter;"),
];
const IMPORTED_CALL_STUB: u64 = 0x100000554;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn boot() -> ConsoleProgram {
    boot_fixture(BIN)
}

fn boot_fixture(bin: &str) -> ConsoleProgram {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(bin);
    assert!(path.exists(), "missing fixture {path:?}");
    bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs")
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap, set `importdata`, commit, then decompile the function at `vma`.
fn render(vma: u64, on: bool) -> String {
    render_in(BIN, vma, on)
}

fn render_in(bin: &str, vma: u64, on: bool) -> String {
    let mut prog = boot_fixture(bin);
    prog.arch_mut()
        .set_kuna_option("importdata", if on { "on" } else { "off" })
        .expect("importdata flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let name = name_at(&prog, vma);
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let fd = {
        let arch = prog.arch_mut();
        build_and_follow_flow(arch, &name, Address::new(code_space, vma), kuna_console::engine::UNBOUNDED_SIZE)
            .expect("the funcdata builds")
    };
    let cmds: Vec<String> = ["decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
        dcp.fd = Some(fd);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn data_import_slots_are_not_function_entries() {
    let mut prog = boot();
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let entries: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    for &(_, slot, _, _) in &READS[..3] {
        assert!(!entries.contains(&slot), "data-import slot 0x{slot:x} must not be an entry: {entries:x?}");
    }
    let (_, counter_slot, _, _) = READS[3];
    assert!(entries.contains(&counter_slot), "the plain C import slot stays an entry: {entries:x?}");
    assert!(entries.contains(&IMPORTED_CALL_STUB), "the stub stays an entry: {entries:x?}");
}

#[test]
fn data_import_slot_names_select_the_slot_as_data() {
    let mut prog = boot();
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    assert_eq!(prog.lookup_data_alias("OBJC_CLASS_$_NSObject"), Some(0x100004000));
    assert_eq!(prog.lookup_data_alias("Swift.String.type_metadata"), Some(0x100004008));
}

#[test]
fn data_import_slot_reads_render_the_import_only_when_on() {
    for &(vma, _, off_line, on_line) in &READS {
        let off = render(vma, false);
        let on = render(vma, true);
        assert!(off.contains(off_line), "0x{vma:x} importdata off must read `{off_line}`:\n{off}");
        assert!(on.contains(on_line), "0x{vma:x} importdata on must read `{on_line}`:\n{on}");
    }
}

#[test]
fn data_import_slot_values_render_the_import_address_only_when_on() {
    const PASS_ADDRESSES: u64 = 0x100000510;
    let off = render_in("macho_dataslotaddr_arm64", PASS_ADDRESSES, false);
    let on = render_in("macho_dataslotaddr_arm64", PASS_ADDRESSES, true);
    assert!(off.contains("use_addresses(dat_100004008,dat_100004000);"), "importdata off:\n{off}");
    assert!(
        on.contains("use_addresses(&OBJC_CLASS_$_NSObject,&<SwiftUI.Color:SwiftUI.View>.protocol_witness_table);"),
        "importdata on:\n{on}"
    );
}
