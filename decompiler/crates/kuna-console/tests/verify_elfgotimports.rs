//! `importdata` on ELF: a data read through an `R_*_GLOB_DAT` GOT slot of an
//! undefined symbol renders as the import, the ELF analog of a Mach-O `__got`
//! slot (pass `elfgotimports`, committed under `importdata`).
//!
//! Fixtures: `tests/fixtures/elf_gotdata_x86_64.so` and `elf_gotdata_aarch64.so`,
//! built from `elf_gotdata.c` (`clang -fPIC -shared -nostdlib`, see the fixtures
//! README). Each reads `imported_counter`, `environ` and `stdout` through GOT slots
//! the object leaves undefined, and `exported_value`, which it defines itself.
//!
//! - **`importdata` off** (default): the three reads are `*dat_<slot>`.
//! - **`importdata` on**: they are `imported_counter`, `environ`, `stdout`.
//! - **Both**: `exported_value` / `&exported_value` are unchanged. That slot names
//!   a defined symbol, which `dynrelocs` fills in and folds; it is not painted.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

/// One fixture: its file, then `(function VMA, expected off, expected on)`.
struct Fixture {
    bin: &'static str,
    reads: [(u64, &'static str, &'static str); 5],
}

const X86_64: Fixture = Fixture {
    bin: "elf_gotdata_x86_64.so",
    reads: [
        (0x1580, "return *dat_2680;", "return imported_counter;"),
        (0x1590, "return *dat_2688;", "return environ;"),
        (0x15a0, "return *dat_2690;", "return stdout;"),
        (0x15b0, "return exported_value;", "return exported_value;"),
        (0x15c0, "return &exported_value;", "return &exported_value;"),
    ],
};

const AARCH64: Fixture = Fixture {
    bin: "elf_gotdata_aarch64.so",
    reads: [
        (0x104e4, "return *dat_205c0;", "return imported_counter;"),
        (0x104f4, "return *dat_205c8;", "return environ;"),
        (0x10504, "return *dat_205d0;", "return stdout;"),
        (0x10514, "return exported_value;", "return exported_value;"),
        (0x10524, "return &exported_value;", "return &exported_value;"),
    ],
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
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

/// Bootstrap `bin`, set `importdata`, commit, then decompile the function at `vma`.
fn render(bin: &str, vma: u64, on: bool) -> String {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(bin);
    assert!(path.exists(), "missing fixture {path:?}");

    let mut prog = bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
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

fn assert_fixture(fx: &Fixture) {
    for &(vma, off_line, on_line) in &fx.reads {
        let off = render(fx.bin, vma, false);
        let on = render(fx.bin, vma, true);
        assert!(off.contains(off_line), "[{}] 0x{vma:x} importdata off must read `{off_line}`:\n{off}", fx.bin);
        assert!(on.contains(on_line), "[{}] 0x{vma:x} importdata on must read `{on_line}`:\n{on}", fx.bin);
    }
}

#[test]
fn x86_64_got_data_imports_render_the_import_only_when_on() {
    assert_fixture(&X86_64);
}

#[test]
fn aarch64_got_data_imports_render_the_import_only_when_on() {
    assert_fixture(&AARCH64);
}
