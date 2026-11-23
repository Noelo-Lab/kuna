//! Calls and memory (slice C2) on x86-64, from clang -O2 -fno-pic:
//!
//! ```c
//! int counter = 5;
//! extern void tick(void);
//! int twice(void) { int a = counter; tick(); return a + counter; }
//! void bump(int *p) { *p += 1; tick(); *p += 2; }
//! ```
//!
//! `tick` is outside the image, so both sides run its stub: it may write
//! `counter` and what `p` points to. kuna reloads both after the call, so both
//! functions agree; a kuna that kept `counter` across the call would differ.

use std::path::PathBuf;

use kuna_verify::{check_binary, Options, Verdict};
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};

const TEXT: &str = "538b1d00000000e800000000031d0000000089d85bc3662e0f1f840000000000534889fbff07e8000000008303025bc3";

fn object() -> PathBuf {
    let text: Vec<u8> = (0..TEXT.len()).step_by(2).map(|i| u8::from_str_radix(&TEXT[i..i + 2], 16).unwrap()).collect();
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let code = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(code, &text, 16);
    let data = obj.add_section(Vec::new(), b".data".to_vec(), SectionKind::Data);
    obj.append_section_data(data, &5u32.to_le_bytes(), 4);
    let sym = |obj: &mut Object, name: &str, value, size, kind, section| {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind,
            scope: SymbolScope::Linkage,
            weak: false,
            section,
            flags: SymbolFlags::None,
        })
    };
    sym(&mut obj, "twice", 0, 22, SymbolKind::Text, SymbolSection::Section(code));
    sym(&mut obj, "bump", 0x20, 16, SymbolKind::Text, SymbolSection::Section(code));
    let counter = sym(&mut obj, "counter", 0, 4, SymbolKind::Data, SymbolSection::Section(data));
    let tick = sym(&mut obj, "tick", 0, 0, SymbolKind::Text, SymbolSection::Undefined);
    for (offset, symbol, r_type) in [(3, counter, 2), (8, tick, 4), (0xe, counter, 2), (0x27, tick, 4)] {
        obj.add_relocation(code, Relocation { offset, symbol, addend: -4, flags: RelocationFlags::Elf { r_type } })
            .unwrap();
    }
    let dir = std::env::temp_dir().join(format!("kuna-verify-calls-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("callmem.o");
    crate::zext_pair::write_whole(&path, &obj.write().unwrap());
    path
}

#[test]
fn a_global_and_a_pointee_are_reloaded_after_a_call() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs").canonicalize().unwrap();
    let functions = vec!["twice".to_string(), "bump".to_string()];
    let reports = check_binary(
        &object().to_string_lossy(),
        &functions,
        &[],
        &[specs.to_string_lossy().into_owned()],
        None,
        &Options { runs: 64, ..Options::default() },
    )
    .unwrap();
    for r in &reports {
        assert_eq!(r.verdict, Verdict::Agrees(64), "{}", r.render());
    }
}
