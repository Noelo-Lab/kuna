//! Undefined untyped call targets receive known library prototypes.
use crate::common;
use kuna_console::engine::{bootstrap_from_object, EntrySelector};
use kuna_console::project::decompile_targets;
use object::{
    write, Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind,
    SymbolScope,
};

struct Scratch(std::path::PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn notype_memset_call_types_pointer_parameter_and_return() {
    let sparc_words: [u32; 8] = [
        0x9de3bfa0, 0x90100018, 0x92102000, 0x941020a8, 0x40000000, 0x01000000, 0x81c7e008,
        0x91e80008,
    ];
    let sparc: Vec<_> = sparc_words.into_iter().flat_map(u32::to_be_bytes).collect();
    let x86 = vec![
        0x48, 0x83, 0xec, 8, 0x31, 0xf6, 0xba, 0xa8, 0, 0, 0, 0xe8, 0, 0, 0, 0, 0x48, 0x89, 0x04,
        0x24, 0x48, 0x8b, 0x04, 0x24, 0x48, 0x83, 0xc4, 8, 0xc3,
    ];
    for (architecture, endian, bytes, offset, addend, r_type) in [
        (
            Architecture::Sparc,
            Endianness::Big,
            sparc,
            16,
            0,
            object::elf::R_SPARC_WDISP30,
        ),
        (
            Architecture::X86_64,
            Endianness::Little,
            x86,
            12,
            -4,
            object::elf::R_X86_64_PLT32,
        ),
    ] {
        let mut object = write::Object::new(BinaryFormat::Elf, architecture, endian);
        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(text, &bytes, 4);
        object.add_symbol(write::Symbol {
            name: b"clear".to_vec(),
            value: 0,
            size: bytes.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Dynamic,
            weak: false,
            section: write::SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
        let target = object.add_symbol(write::Symbol {
            name: b"memset".to_vec(),
            value: 0,
            size: 0,
            kind: SymbolKind::Unknown,
            scope: SymbolScope::Dynamic,
            weak: false,
            section: write::SymbolSection::Undefined,
            flags: SymbolFlags::None,
        });
        object
            .add_relocation(
                text,
                write::Relocation {
                    offset,
                    symbol: target,
                    addend,
                    flags: RelocationFlags::Elf { r_type },
                },
            )
            .expect("call relocation");
        let path = Scratch(common::scratch_file("notype-memset", "o"));
        std::fs::write(&path.0, object.write().expect("synthetic ELF")).expect("write fixture");
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let specs = vec![root.join("specs").to_str().unwrap().to_string()];
        let mut program =
            bootstrap_from_object(path.0.to_str().unwrap(), "", &specs).expect("bootstrap");
        program.commit_pending_analysis().expect("commit analysis");
        let entry = program
            .resolve_entry(&EntrySelector::Name("clear".into()))
            .expect("clear entry");
        let functions = decompile_targets(&mut program, vec![entry], false, false, false);
        let code = functions[0].code.as_ref().expect("decompiled code");
        let signature = code
            .lines()
            .find(|line| line.contains("clear("))
            .expect("signature");
        assert!(
            regex::Regex::new(r"^void\s*\*\s*clear\(")
                .unwrap()
                .is_match(signature),
            "return type {architecture:?}: {code}"
        );
        assert!(
            regex::Regex::new(r"clear\(void\s*\*\s*\w+\)")
                .unwrap()
                .is_match(signature),
            "parameter type {architecture:?}: {code}"
        );
        assert!(
            regex::Regex::new(r"return\s+memset\([^,]+,\s*0,\s*0xa8\);")
                .unwrap()
                .is_match(code),
            "return call result {architecture:?}: {code}"
        );
    }
}
