//! Rust paths for independently verified whole-function address expressions.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;
use crate::printlanguage::NamespaceStrategy;

pub(crate) fn prefix(
    fd: &Funcdata,
    arch: &Architecture,
    target: &Address,
    usepoint: &Address,
    name: &str,
    strategy: NamespaceStrategy,
) -> Option<Vec<String>> {
    if matches!(strategy, NamespaceStrategy::NoNamespaces) {
        return Some(Vec::new());
    }
    if !plain_module(name) || crate::kuna_rustuserop::raw_keyword(name) {
        return None;
    }
    let scopes = crate::kuna_funcptrnames::proven_scopes(fd, arch, target, usepoint, name)?;
    let target: Vec<_> = scopes.target_inner.iter().rev().cloned().collect();
    let caller: Vec<_> = arch.symboltab.get_scope_path(scopes.caller).iter().skip(1)
        .map(|&scope| arch.symboltab.scope(scope).get_display_name().to_string()).collect();
    if !target.iter().chain(&caller).all(|scope| plain_module(scope)) {
        return None;
    }
    let used = |candidate: &str| fd.get_scope_local()
        .is_some_and(|local| local.local_name_used(candidate));
    let mut path = if matches!(strategy, NamespaceStrategy::MinimalNamespaces)
        && target.starts_with(&caller)
    {
        let relative = &target[caller.len()..];
        if !used(relative.first().map(String::as_str).unwrap_or(name)) {
            relative.to_vec()
        } else {
            std::iter::once("crate".into()).chain(target).collect()
        }
    } else if caller.is_empty() && !used(target.first().map(String::as_str).unwrap_or(name)) {
        target
    } else {
        std::iter::once("crate".into()).chain(target).collect()
    };
    for scope in &mut path {
        if crate::kuna_rustuserop::raw_keyword(scope) {
            *scope = format!("r#{scope}");
        }
    }
    Some(path)
}

fn plain_module(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes.next().is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && !matches!(name, "crate" | "self" | "Self" | "super" | "_"
            | "bool" | "char" | "str"
            | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
            | "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
            | "f16" | "f32" | "f64" | "f128")
}
