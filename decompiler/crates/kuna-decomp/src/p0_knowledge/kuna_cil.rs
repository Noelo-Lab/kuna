//! (kuna) The `cil` option: decode a managed .NET assembly from its CIL method
//! bodies instead of from the native stub of its PE.
//!
//! A managed PE holds one native instruction, the jump to `_CorExeMain`; the
//! program is ECMA-335 metadata and CIL. With `cil on` (the default) an input
//! with a CLI header and at least one method body loads as the
//! `CIL:LE:64:default` language through
//! `kuna_analysis::loadimage_cil`; every other input is untouched. `off` loads
//! the PE as native code, as an explicit `--target` does.
//!
//! The choice is made inside `load file`, before any per-function `option`
//! command, so it is a load-time option ([`crate::kuna_loadoptions`]) that the
//! CLI carries in [`CIL_ENV`].

use kuna_base::error::KunaResult;

/// Absent or any value other than the off-tokens means enabled.
pub const CIL_ENV: &str = "KUNA_CIL";

/// Whether a managed assembly loads as CIL for the load in progress.
pub fn cil_enabled() -> bool {
    crate::kuna_loadoptions::scoped(|o| o.cil).unwrap_or_else(crate::kuna_loadoptions::env::cil)
}

pub fn set_cil_env(on: bool) {
    std::env::set_var(CIL_ENV, if on { "on" } else { "off" });
}

/// Apply `option cil VALUE`.
pub fn set_cil(value: &str) -> KunaResult<String> {
    crate::options::on_or_off(value).map(|on| {
        set_cil_env(on);
        format!("managed .NET assemblies load as {}", if on { "CIL" } else { "native code" })
    })
}
