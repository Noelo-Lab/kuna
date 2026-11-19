//! A function reached only through a pointer table is referenced by its slot.
#![cfg(target_os = "linux")]

use crate::common;
use common::process;
use object::{Object, ObjectSymbol};
use std::process::Command;

const SOURCE: &str = "static int h0(int x) { return x + 1; }\n\
static int h1(int x) { return x * 3; }\n\
static int h2(int x) { return x - 7; }\n\
int (*table[])(int) = { h0, h1, h2 };\n\
int run_all(int x) { int s = 0; for (int i = 0; i < 3; i++) s += table[i](x); return s; }\n\
void _start(void) { for (;;) run_all(5); }\n";

#[test]
fn table_slots_are_data_references_under_addrtable() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("addrtable-xrefs", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample");
    std::fs::write(&source, SOURCE).unwrap();
    process::required_output(
        Command::new("clang")
            .args(["-O1", "-fno-inline", "-ffreestanding", "-fno-pic", "-nostdlib", "-no-pie"])
            .args(["-fuse-ld=lld", "-Wl,--no-dynamic-linker"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let address = |name: &str| {
        image.symbols().find(|s| s.name() == Ok(name)).unwrap().address()
    };
    let (table, h2) = (address("table"), address("h2"));
    let path = binary.to_str().unwrap();

    let (off, err, status) = common::run_kuna(&["xrefs", path, "--to", "h2"]);
    assert_eq!(status, 0, "{err}");
    assert!(off.starts_with("# 0 references"), "{off}");

    let (on, err, status) =
        common::run_kuna(&["xrefs", path, "--to", "h2", "--option", "addrtable", "on"]);
    assert_eq!(status, 0, "{err}");
    let row = format!("0x{:x}\tdata\t0x{:x}\taddr 0x{h2:x}", table + 16, table + 16);
    assert!(on.lines().any(|line| line == row), "want {row:?}:\n{on}");
    std::fs::remove_dir_all(root).unwrap();
}
