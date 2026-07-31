//! (kuna) The warning-style presentation sub-stage — no upstream Ghidra
//! equivalent (DIV-38).
//!
//! Upstream Ghidra renders every analysis warning as a full banner line at the
//! comment indent (`/* WARNING: Subroutine does not return */`) above the
//! statement it describes.  kuna parameterizes the rendering:
//! `option warnstyle inline` (the default) maps each warning to a terse slug
//! and appends it as one `// slug, slug` comment at the END of the line it
//! describes — the call statement (`usage(1); // no-return`), the
//! `if (cond) {` header for condition-attached warnings (`// branch-flip`),
//! the function prototype for header warnings (`// early-return x3`);
//! `option warnstyle banner` restores the upstream banner lines.
//!
//! The rendering lives in [`crate::printc`]: `emit_comment_group` and
//! `emit_comment_func_header` collect [`crate::printc::warning_slug`] slugs
//! under [`crate::printc::PrintCOptions::warn_inline`], and
//! `flush_eol_warnings` appends them at the owning line's last token.  Only
//! comments of type WARNING/WARNINGHEADER restyle; user and `dwarf_lines`
//! comments keep their banner-line form.
//!
//! Following the kuna-option idiom ([`crate::kuna_arraynotation::OptionArrayNotation`]),
//! this module owns only the option struct; the caller
//! (`Architecture::set_kuna_option`) writes the live flag.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;

/// Marshaling element `<warnstyle>` (kuna 4000+ range; 4116 = braceelide).
pub const ELEM_WARNSTYLE: ElementId = ElementId::new("warnstyle", 4117);

/// (kuna) Choose the warning rendering: `warnstyle inline|banner` (DIV-38).
///
/// "banner" keeps the upstream full-line `/* WARNING: ... */` form; "inline"
/// (the kuna default) renders terse `// slug` end-of-line comments.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionWarnStyle;

impl OptionWarnStyle {
    /// The option name.
    pub const NAME: &'static str = "warnstyle";

    /// Parse + validate the `inline`/`banner` value; the caller performs the
    /// printer write ([`crate::printc::PrintCOptions::set_warn_inline`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = match p1 {
            "inline" => true,
            "banner" => false,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown warnstyle value: {other} (expected inline|banner)"
                )))
            }
        };
        Ok((val, format!("Warning style set to {p1}")))
    }
}

#[cfg(test)]
#[path = "kuna_warnstyle/tests.rs"]
mod tests;
