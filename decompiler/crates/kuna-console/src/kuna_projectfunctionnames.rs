//! Project C definitions use the existing global symbol spelling of their callers.
use kuna_decomp::{architecture::Architecture, funcdata::Funcdata};

fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && !matches!(
            name,
            "asm"
                | "typeof"
                | "__asm__"
                | "__typeof__"
                | "auto"
                | "break"
                | "case"
                | "char"
                | "const"
                | "continue"
                | "default"
                | "do"
                | "double"
                | "else"
                | "enum"
                | "extern"
                | "float"
                | "for"
                | "goto"
                | "if"
                | "inline"
                | "int"
                | "long"
                | "register"
                | "restrict"
                | "return"
                | "short"
                | "signed"
                | "sizeof"
                | "static"
                | "struct"
                | "switch"
                | "typedef"
                | "union"
                | "unsigned"
                | "void"
                | "volatile"
                | "while"
                | "_Alignas"
                | "_Alignof"
                | "_Atomic"
                | "_Bool"
                | "_Complex"
                | "_Generic"
                | "_Imaginary"
                | "_Noreturn"
                | "_Static_assert"
                | "_Thread_local"
                | "bool"
                | "true"
                | "false"
        )
}

pub(crate) fn display(arch: &Architecture, fd: &Funcdata) -> Option<String> {
    display_at(arch, fd.get_address(), fd.get_display_name())
}

fn display_at(
    arch: &Architecture,
    address: &kuna_base::address::Address,
    original: &str,
) -> Option<String> {
    if arch.print().out_lang() != kuna_decomp::kuna_lang::OutLang::C || !identifier(original) {
        return None;
    }
    let db = &arch.symboltab;
    let global = db.get_global_scope()?;
    let symbol = db.find_function(global, address)?;
    let name = db.symbol(symbol).get_display_name();
    identifier(name).then(|| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::identifier;
    #[test]
    fn only_unqualified_c_identifiers_are_candidates() {
        for name in ["helper_linkage", "_worker", "sample2"] {
            assert!(identifier(name));
        }
        for name in [
            "",
            "2sample",
            "ns::sample",
            "operator+",
            "for",
            "_Atomic",
            "helper.0",
            "asm",
            "typeof",
            "__asm__",
            "__typeof__",
        ] {
            assert!(!identifier(name));
        }
    }
    #[test]
    fn exact_global_lookup_respects_renames_and_declines_namespace_owners() {
        use kuna_base::address::Address;
        use std::rc::Rc;
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let mut program = crate::engine::bootstrap_from_file(
            root.join("tests/datatests/boolless.xml").to_str().unwrap(),
            "",
            &[root.join("specs").to_string_lossy().into_owned()],
        )
        .unwrap();
        let space = Rc::clone(program.arch().manage().get_default_code_space().unwrap());
        let entry = Address::new(Rc::clone(&space), 0x6000);
        program
            .declare_function(entry.clone(), Some("helper_linkage"), 4)
            .unwrap();
        assert_eq!(
            super::display_at(program.arch(), &entry, "helper").as_deref(),
            Some("helper_linkage")
        );
        program
            .declare_function(entry.clone(), Some("caller_named"), 4)
            .unwrap();
        assert_eq!(
            super::display_at(program.arch(), &entry, "helper").as_deref(),
            Some("caller_named")
        );
        let method = Address::new(space, 0x7000);
        program
            .declare_function(method.clone(), Some("ns::method"), 4)
            .unwrap();
        assert_eq!(super::display_at(program.arch(), &method, "method"), None);
        assert_eq!(
            super::display_at(program.arch(), &entry, "ns::helper"),
            None
        );
    }
}

/// Address-only reports of conflicting C ownership; aliases and storage are not guessed.
pub(crate) fn conflicts(
    results: &[crate::project::FuncResult],
    typedefs: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    use std::collections::{BTreeMap, BTreeSet};
    let mut owners: BTreeMap<&str, BTreeSet<u64>> = BTreeMap::new();
    for r in results
        .iter()
        .filter(|r| r.error.is_none() && r.proto.is_some())
    {
        if let Some(name) = r.emitted_c_name.as_deref() {
            owners.entry(name).or_default().insert(r.byte_address);
        }
    }
    let globals: BTreeSet<&str> = results
        .iter()
        .flat_map(|r| r.globals.iter())
        .map(|g| g.name.as_str())
        .collect();
    let mut errors = Vec::new();
    for (name, entries) in owners {
        let first = entries.iter().next().unwrap();
        if entries.len() > 1 {
            errors.push(format!(
                "C function name has multiple owning entries, including 0x{first:x}"
            ));
        }
        if globals.contains(name) || typedefs.contains(name) {
            errors.push(format!(
                "C function name at 0x{first:x} conflicts with a data or type declaration"
            ));
        }
    }
    errors
}

pub(crate) fn reserve(arch: &Architecture, fd: &mut Funcdata) -> kuna_base::error::KunaResult<()> {
    use std::collections::{BTreeMap, BTreeSet};
    let Some(global) = arch.symboltab.get_global_scope() else {
        return Ok(());
    };
    let Some(local) = fd.get_scope_local() else {
        return Ok(());
    };
    let symbols = local.database().scope_symbol_storage(local.scope_id());
    let mut used: BTreeSet<_> = symbols.iter().map(|(_, _, _, name)| name.clone()).collect();
    if arch.print().out_lang() != kuna_decomp::kuna_lang::OutLang::C {
        return Ok(());
    }
    used.extend(
        fd.high_bank()
            .iter()
            .filter_map(|(_, h)| h.kuna_name().map(str::to_string)),
    );
    let mut replacements = BTreeMap::new();
    let mut seen = Vec::new();
    let mut plan = Vec::new();
    for (id, _, _, name) in symbols {
        if seen.contains(&id) {
            continue;
        }
        seen.push(id);
        if !identifier(&name)
            || arch
                .symboltab
                .query_function_by_name(global, &name)
                .is_none()
        {
            continue;
        }
        let replacement = if let Some(name) = replacements.get(&name) {
            String::clone(name)
        } else {
            let mut number = 0;
            let replacement = loop {
                let candidate = if number == 0 {
                    format!("{name}_local")
                } else {
                    format!("{name}_local{number}")
                };
                number += 1;
                if !used.contains(&candidate)
                    && arch.symboltab.query_by_name(global, &candidate).is_empty()
                {
                    break candidate;
                }
            };
            used.insert(replacement.clone());
            replacements.insert(name.clone(), replacement.clone());
            replacement
        };
        let symbol = local.database().symbol(id);
        let parameter = if symbol.get_category() == 0 {
            let index = i32::from(symbol.get_category_index());
            fd.get_func_proto().get_param(index).map(|p| {
                (
                    index,
                    kuna_decomp::fspec::ParameterPieces {
                        addr: p.get_address(),
                        type_: p.get_type().cloned(),
                        flags: p.get_flags(),
                    },
                )
            })
        } else {
            None
        };
        plan.push((id, replacement, parameter));
    }
    let high_names: Vec<_> = fd
        .high_bank()
        .iter()
        .filter_map(|(id, high)| {
            let replacement = replacements.get(high.kuna_name()?)?;
            let instances: Vec<_> = (0..high.num_instances())
                .filter_map(|i| fd.vbank().get(high.get_instance(i)))
                .collect();
            if instances.iter().any(|v| v.is_persist())
                || !instances
                    .iter()
                    .any(|v| !v.is_constant() && !v.is_annotation())
            {
                return None;
            }
            Some((id, replacement.clone()))
        })
        .collect();
    for (id, name) in high_names {
        fd.high_bank_mut().get_mut(id).unwrap().set_kuna_name(name);
    }
    for (id, name, parameter) in plan {
        fd.get_scope_local_mut().unwrap().rename_symbol(id, &name)?;
        if let Some((index, piece)) = parameter {
            fd.get_func_proto_mut().set_param(index, &name, &piece);
        }
    }
    Ok(())
}
