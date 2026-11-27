//! Objective-C methods the metadata does not list: a direct method's `self`.
//!
//! A method declared `__attribute__((objc_direct))` (or in an
//! `objc_direct_members` class) is called as a C function: it has no entry in
//! any `method_list_t`, takes no `_cmd`, and only its symbol, `-[Class sel]`,
//! says what it is. On AArch64 its receiver is always in `x0`, so each such
//! symbol hints `x0` as `self`, of the class's type under `objcivars`: an
//! unlocked hint ([`kuna_decomp::kuna_inputhints`]), since the encoding that
//! would give the other parameters does not exist.

use object::Architecture;

use super::encoding::TypeCtx;
use super::kuna_objcivars::Structs;
use super::sections::MachoImage;
use crate::pass::{AnalysisOutput, InputHintFact};

/// The register `self` always arrives in. AArch64 passes a result buffer in
/// `x8`, so `x0` is the receiver; x86-64 passes one in `RDI` and moves `self`
/// to `RSI`, and nothing in the symbol tells the two shapes apart.
fn self_register(architecture: Architecture) -> Option<&'static str> {
    match architecture {
        Architecture::Aarch64 => Some("x0"),
        _ => None,
    }
}

/// The class a method symbol names: `-[Greeter(Extra) greet:]` is `Greeter`'s.
fn owner(symbol: &str) -> Option<&str> {
    let inner = symbol.get(1..)?.strip_prefix('[')?.strip_suffix(']')?;
    let (class, _) = inner.split_once(' ')?;
    let class = class.split_once('(').map_or(class, |(class, _)| class);
    (!class.is_empty()).then_some(class)
}

/// Hint `self` on every method symbol whose address no method list gave (the
/// listed ones are `out.fid_names` by now). `structs` types an instance
/// method's `self`; a class method's is only named.
pub(super) fn emit(
    img: &MachoImage,
    architecture: Architecture,
    tctx: &TypeCtx,
    structs: Option<&Structs>,
    out: &mut AnalysisOutput,
) {
    let Some(reg) = self_register(architecture) else { return };
    let listed: std::collections::HashSet<u64> = out.fid_names.iter().map(|m| m.addr).collect();
    let mut direct: Vec<(u64, &str)> = img.method_symbols().filter(|(addr, _)| !listed.contains(addr)).collect();
    direct.sort_unstable();
    for (func_addr, symbol) in direct {
        let Some(class) = owner(symbol) else { continue };
        let type_ = structs.filter(|_| symbol.starts_with('-')).and_then(|s| s.self_type(tctx, class));
        out.objc_self_hints.push(InputHintFact { func_addr, reg: reg.into(), name: "self".to_string(), type_ });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_aarch64_has_a_fixed_receiver_register() {
        assert_eq!(self_register(Architecture::Aarch64), Some("x0"));
        assert_eq!(self_register(Architecture::X86_64), None, "a struct return moves self to RSI");
    }

    #[test]
    fn owner_is_the_class_under_any_category() {
        assert_eq!(owner("-[Greeter greet:]"), Some("Greeter"));
        assert_eq!(owner("+[Greeter(Extra) make]"), Some("Greeter"));
        assert_eq!(owner("-[ greet]"), None);
        assert_eq!(owner("-[Greeter"), None);
        assert_eq!(owner("-"), None);
    }
}
