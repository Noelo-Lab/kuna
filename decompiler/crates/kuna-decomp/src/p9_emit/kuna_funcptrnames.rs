//! Namespace qualification for exact standalone function-address expressions.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::database::ScopeId;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::kuna_namespaces::plain_scope;
use crate::printlanguage::NamespaceStrategy;

pub(crate) struct ProvenScopes {
    pub(crate) target_inner: Vec<String>,
    pub(crate) caller: ScopeId,
    target_ids: Vec<ScopeId>,
}

/// Require the live and detached symbol views to name the same whole function.
pub(crate) fn proven_scopes(
    fd: &Funcdata,
    arch: &Architecture,
    target: &Address,
    usepoint: &Address,
    name: &str,
) -> Option<ProvenScopes> {
    if arch.remote_scope.is_some() {
        return None;
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
    let (_, caller) = db.find_function_across_scopes(fd.get_address())?;
    let target_ids = db.get_scope_path(scope);
    let target_inner: Vec<_> = target_ids
        .iter()
        .skip(1)
        .rev()
        .map(|&scope| db.scope(scope).get_display_name().to_string())
        .collect();
    if mapped_scope != target_inner {
        return None;
    }
    Some(ProvenScopes { target_inner, caller, target_ids })
}

/// Apply C++ lookup rules after proving the function identity in both views.
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
    let scopes = proven_scopes(fd, arch, target, usepoint, name)?;
    if !scopes.target_inner.iter().all(|scope| plain_scope(scope)) {
        return None;
    }
    let db = &arch.symboltab;
    let use_path: Vec<_> = db
        .get_scope_path(scopes.caller)
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
        let stop = scopes.target_ids.get(levels).copied();
        Some(scopes.caller) != stop && db.is_name_used(scopes.caller, candidate, stop)
    };
    let prefix = crate::kuna_namespaces::scope_prefix(
        strategy, name, &scopes.target_inner, &use_path, &used,
    );
    if matches!(strategy, NamespaceStrategy::AllNamespaces) {
        let required = crate::kuna_namespaces::scope_prefix(
            NamespaceStrategy::MinimalNamespaces,
            name,
            &scopes.target_inner,
            &use_path,
            &used,
        );
        if required.len() > prefix.len() {
            return Some(required);
        }
    }
    Some(prefix)
}
