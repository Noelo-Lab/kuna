//! The no-return contract shared by flow construction and callee write probes.

use crate::architecture::Architecture;
use kuna_base::address::Address;

pub(crate) fn is_no_return(arch: &Architecture, entry: &Address) -> bool {
    if arch
        .remote_scope
        .as_ref()
        .and_then(|remote| remote.function_at(entry))
        .is_some_and(|facts| facts.no_return)
        || arch.symboltab.function_is_no_return_across_scopes(entry)
    {
        return true;
    }
    let name = arch.symboltab.function_display_name_across_scopes(entry);
    if let Some(name) = name.as_deref() {
        if arch.noreturn_extern_match
            && crate::kuna_noreturn_externmatch::is_known_noreturn_name(name)
        {
            return true;
        }
        if arch.strip_security_check && crate::kuna_securitycheck::is_security_check_name(name) {
            return true;
        }
    }
    if arch.noreturn_extern_calls {
        let name = arch
            .remote_scope
            .as_ref()
            .and_then(|remote| remote.function_at(entry))
            .map(|facts| facts.display_name)
            .or(name);
        return name
            .as_deref()
            .is_some_and(crate::kuna_noreturnextern::matches_noreturn_extern_name);
    }
    false
}
