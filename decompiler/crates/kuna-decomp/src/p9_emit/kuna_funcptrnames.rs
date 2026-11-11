//! Namespace qualification for exact standalone function-address expressions.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::kuna_namespaces::plain_scope;
use crate::printlanguage::NamespaceStrategy;

/// Resolve the same whole function in both symbol views before applying C++
/// lookup rules. A renamed high, conflicting scope, or remote provider declines.
pub fn prefix(
    fd: &Funcdata,
    arch: &Architecture,
    target: &Address,
    usepoint: &Address,
    name: &str,
    strategy: NamespaceStrategy,
) -> Option<Vec<String>> {
    if arch.remote_scope.is_some() {
        return None;
    }
    if matches!(strategy, NamespaceStrategy::NoNamespaces) {
        return Some(Vec::new());
    }
    let (mapped_name, offset, ty, mapped_scope) = fd
        .get_arch()
        .name_for_global_varnode_scoped(target, 1, usepoint)?;
    if offset != 0 || mapped_name != name || ty?.get_metatype() != type_metatype::TYPE_CODE {
        return None;
    }
    let db = &arch.symboltab;
    let (symbol, scope) = db.find_function_across_scopes(target)?;
    if db.symbol(symbol).get_display_name() != name {
        return None;
    }
    let (_, use_scope) = db.find_function_across_scopes(fd.get_address())?;
    let symbol_ids = db.get_scope_path(scope);
    let symbol_path: Vec<_> = symbol_ids
        .iter()
        .skip(1)
        .rev()
        .map(|&scope| db.scope(scope).get_display_name().to_string())
        .collect();
    if mapped_scope != symbol_path || !symbol_path.iter().all(|scope| plain_scope(scope)) {
        return None;
    }
    let use_path: Vec<_> = db
        .get_scope_path(use_scope)
        .iter()
        .skip(1)
        .map(|&scope| db.scope(scope).get_display_name().to_string())
        .collect();
    let used = |candidate: &str, levels: usize| {
        if fd
            .get_scope_local()
            .is_some_and(|local| local.local_name_used(candidate))
        {
            return true;
        }
        let stop = symbol_ids.get(levels).copied();
        Some(use_scope) != stop && db.is_name_used(use_scope, candidate, stop)
    };
    let prefix =
        crate::kuna_namespaces::scope_prefix(strategy, name, &symbol_path, &use_path, &used);
    if matches!(strategy, NamespaceStrategy::AllNamespaces) {
        let required = crate::kuna_namespaces::scope_prefix(
            NamespaceStrategy::MinimalNamespaces,
            name,
            &symbol_path,
            &use_path,
            &used,
        );
        if required.len() > prefix.len() {
            return Some(required);
        }
    }
    Some(prefix)
}
