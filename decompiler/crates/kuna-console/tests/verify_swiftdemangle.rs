//! End-to-end gate for Swift symbol demangling (`swiftdemangle`,
//! `docs/multilang.md` — the Swift pack's first prerequisite).
//!
//! No Swift toolchain exists on this host or CI, so the image is SYNTHESIZED:
//! an ELF object whose `.text` carries two functions under genuine
//! Swift-mangled names (verified against `swift-demangle`, like the
//! `kuna_swift` unit-test corpus). What this proves is the PIPELINE:
//! loader → `demangle_name` → the canonical entry inventory renders the
//! demangled path, and `KUNA_SWIFTDEMANGLE=off` restores the mangled spelling.
//! What it cannot prove is real-swiftc codegen; the Swift pack proper adds a
//! vendored binary fixture (devcontainer-built) when one exists.
//!
//! Own `[[test]]` target: the off arm mutates process env (`EnvGuard`), the
//! repo rule for env-mutating tests.

use std::path::PathBuf;

use object::write;
use object::{Architecture, BinaryFormat, Endianness, SymbolFlags, SymbolKind, SymbolScope};

use kuna_console::engine::bootstrap_from_object;

struct EnvGuard(&'static str, Option<std::ffi::OsString>);
impl EnvGuard {
    fn set(name: &'static str, value: &str) -> EnvGuard {
        let prev = std::env::var_os(name);
        std::env::set_var(name, value);
        EnvGuard(name, prev)
    }
}
impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.1 {
            Some(v) => std::env::set_var(self.0, v),
            None => std::env::remove_var(self.0),
        }
    }
}

/// Build the synthetic ELF object: `$s4main5helloyyF` (demangles to
/// `main.hello()`) and `$s4main5helloyyFTJfSSUpSr` (an autodiff derivative,
/// outside the demangler's coverage: must pass through unchanged).
fn build_fixture(dir: &std::path::Path) -> PathBuf {
    std::fs::create_dir_all(dir).expect("fixture dir");
    let mut obj = write::Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = obj.section_id(write::StandardSection::Text);
    // Two `ret` bodies at distinct offsets.
    obj.append_section_data(text, &[0xc3, 0x90, 0x90, 0x90, 0xc3], 4);
    for (name, value) in [("$s4main5helloyyF", 0u64), ("$s4main5helloyyFTJfSSUpSr", 4u64)] {
        obj.add_symbol(write::Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size: 1,
            kind: SymbolKind::Text,
            scope: SymbolScope::Dynamic,
            weak: false,
            section: write::SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let path = dir.join("swift_syms.o");
    std::fs::write(&path, obj.write().expect("write object")).expect("write fixture");
    path
}

fn entry_names(bin: &str) -> Vec<String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = vec![root.join("specs").to_str().unwrap().to_string()];
    let mut program = bootstrap_from_object(bin, "", &specs).expect("bootstrap synthetic object");
    program.commit_pending_analysis().expect("commit analysis");
    program
        .function_entries_canonical()
        .into_iter()
        .map(|e| e.name)
        .collect()
}

/// Default: the covered name renders demangled, the autodiff derivative
/// passes through; `off` restores both mangled spellings.
#[test]
fn swift_symbols_demangle_through_the_name_surfaces() {
    let dir = std::env::temp_dir().join(format!("kuna_swiftdemangle_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let fixture = build_fixture(&dir);
    let bin = fixture.to_str().expect("utf8 path");

    let on = entry_names(bin);
    assert!(
        on.iter().any(|n| n == "main.hello()"),
        "the covered name demangles; inventory: {on:?}"
    );
    assert!(
        on.iter().any(|n| n == "$s4main5helloyyFTJfSSUpSr"),
        "the uncovered derivative passes through unchanged; inventory: {on:?}"
    );
    assert!(
        !on.iter().any(|n| n == "$s4main5helloyyF"),
        "the mangled spelling is replaced, not duplicated; inventory: {on:?}"
    );

    {
        let _guard = EnvGuard::set("KUNA_SWIFTDEMANGLE", "off");
        let off = entry_names(bin);
        assert!(
            off.iter().any(|n| n == "$s4main5helloyyF"),
            "off restores the mangled spelling; inventory: {off:?}"
        );
        assert!(
            !off.iter().any(|n| n == "main.hello()"),
            "off demangles nothing; inventory: {off:?}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
