//! Bind Android's public JNI declarations to the loaded ELF and the program type factory.

use kuna_analysis::loader::format::elf_userland::is_android_image;
use kuna_analysis::pass::{AnalysisOutput, SymFact, SymKind};
use kuna_base::error::KunaResult;
use object::{Object, ObjectSymbol};

use crate::grammar::{parse_c, parse_protopieces, DataOrg};
use kuna_decomp::architecture::Architecture;

const TYPES: &str = include_str!("../../kuna-analysis/data/android-jni/jni-types.h");

pub(crate) fn load(arch: &Architecture, bytes: &[u8]) -> KunaResult<Option<AnalysisOutput>> {
    if !is_android_image(bytes) {
        return Ok(None);
    }
    let Ok(file) = kuna_analysis::loadimage_object::parse_object(bytes) else {
        return Ok(None);
    };
    let abi = match file.architecture() {
        object::Architecture::Aarch64 if arch.archid.starts_with("AARCH64:LE:64:") => {
            include_str!("../../kuna-analysis/data/android-jni/va-arm64.h")
        }
        object::Architecture::Arm if arch.archid.starts_with("ARM:LE:32:") => {
            include_str!("../../kuna-analysis/data/android-jni/va-arm.h")
        }
        object::Architecture::I386 if arch.archid.starts_with("x86:LE:32:") => {
            include_str!("../../kuna-analysis/data/android-jni/va-x86.h")
        }
        object::Architecture::X86_64 if arch.archid.starts_with("x86:LE:64:") => {
            include_str!("../../kuna-analysis/data/android-jni/va-x86_64.h")
        }
        _ => return Ok(None),
    };
    let mut out = AnalysisOutput::default();
    let (addr_size, word_size) = arch.data_org();
    if addr_size != if file.is_64() { 8 } else { 4 } {
        return Ok(None);
    }
    let org = DataOrg {
        addr_size,
        word_size,
    };
    let mut fixed = std::collections::HashMap::new();
    for declaration in abi.split("\n\n").chain(TYPES.split("\n\n")).map(str::trim) {
        if declaration.is_empty() || declaration.starts_with("/*") {
            continue;
        }
        parse_c(declaration, arch.types(), org, &[], |pieces, _| {
            fixed.insert(pieces.name.clone(), pieces);
            Ok(())
        })?;
    }
    let mut imports_jni = false;
    for symbol in file.dynamic_symbols() {
        let Some(pieces) = symbol.name().ok().and_then(|n| fixed.get(n)) else {
            continue;
        };
        if symbol.is_undefined() {
            imports_jni = true;
        } else if symbol.is_definition()
            && !symbol.is_local()
            && symbol.kind() == object::SymbolKind::Text
        {
            let address = if file.architecture() == object::Architecture::Arm {
                symbol.address() & !1
            } else {
                symbol.address()
            };
            out.prototypes_at.push((address, pieces.clone()));
        }
    }
    if imports_jni {
        for import in kuna_analysis::loader::format::resolve_imports(&file, bytes) {
            if let Some(pieces) = std::str::from_utf8(&import.name)
                .ok()
                .and_then(|n| fixed.get(n))
            {
                out.prototypes_at.push((import.addr, pieces.clone()));
            }
        }
    }
    Ok(Some(out))
}

pub(crate) fn collect(arch: &Architecture, bytes: &[u8], out: &mut AnalysisOutput) {
    let Ok(file) = kuna_analysis::loadimage_object::parse_object(bytes) else {
        return;
    };
    let (addr_size, word_size) = arch.data_org();
    let org = DataOrg {
        addr_size,
        word_size,
    };
    for method in
        kuna_analysis::android_jni::registrations(&file, bytes, arch, arch.analysis_jniregister)
    {
        out.symbols.push(SymFact {
            addr: method.addr,
            name: method.name,
            kind: SymKind::Function,
        });
        if let Some(decl) = method.declaration {
            if let Ok(mut pieces) = parse_protopieces(&decl, arch.types(), org) {
                if !method.return_known {
                    pieces.outtype = None;
                }
                out.prototypes_at.push((method.addr, pieces));
            }
        }
    }
}
