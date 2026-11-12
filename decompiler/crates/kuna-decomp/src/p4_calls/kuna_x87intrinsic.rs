//! (kuna) The prototype model of the MSVC x86-32 one-operand x87 math intrinsics.
//!
//! MSVC compiles `sin(x)`, `cos(x)`, `sqrt(x)` and their siblings on x86-32 into
//! `fld <x>; call __CIsin`: the operand arrives in the x87 stack top `ST0` and the
//! result comes back there, with no stack argument. No model in `x86win.cspec`
//! has an `ST0` input, so the call recovers no argument, nothing reads `ST0`, and
//! the whole expression that computed the operand is dead-code eliminated.
//! Adding an `ST0` input to a shared model would invent a phantom argument on
//! every other callee (see [`crate::p2_lift::kuna_msvcftol`]), so these helpers
//! get a model of their own, bound to them by address by the analysis tier.
//!
//! The model, [`MODEL_NAME`], takes one floating-point input in `ST0` and returns
//! one in `ST0`, pops nothing from the stack (`extrapop` 4), and preserves the
//! registers `__cdecl` preserves. The operand stays where the result goes, so the
//! x87 stack depth is the same before and after the call, which is what a model
//! can express. The two-operand helpers (`__CIpow`, `__CIatan2`, `__CIfmod`) pop
//! one slot and are left alone.

use kuna_base::error::KunaResult;

/// The model's name, as bound by the analysis tier.
pub const MODEL_NAME: &str = "__kuna_x87unary";

/// The one-operand intrinsics of the MSVC CRT that follow the model.
pub const INTRINSICS: [&str; 10] = [
    "__CIsqrt", "__CIsin", "__CIcos", "__CItan", "__CIexp", "__CIlog", "__CIlog10", "__CIasin",
    "__CIacos", "__CIatan",
];

/// Whether `name` is one of [`INTRINSICS`], spelled with or without leading underscores.
pub fn is_intrinsic(name: &str) -> bool {
    let bare = name.trim_start_matches('_');
    INTRINSICS.iter().any(|i| i.trim_start_matches('_') == bare)
}

/// The synthesized `<prototype>` element.
pub fn prototype_xml() -> String {
    format!(
        r#"<prototype name="{MODEL_NAME}" extrapop="4" stackshift="4">
  <input>
    <pentry minsize="4" maxsize="10" metatype="float" extension="float">
      <register name="ST0"/>
    </pentry>
  </input>
  <output killedbycall="true">
    <pentry minsize="4" maxsize="10" metatype="float" extension="float">
      <register name="ST0"/>
    </pentry>
  </output>
  <unaffected>
    <register name="ESP"/>
    <register name="EBP"/>
    <register name="ESI"/>
    <register name="EDI"/>
    <register name="EBX"/>
    <register name="DF"/>
  </unaffected>
  <killedbycall>
    <register name="EAX"/>
    <register name="ECX"/>
    <register name="EDX"/>
    <register name="ST0"/>
  </killedbycall>
</prototype>"#
    )
}

/// Parse [`prototype_xml`] and hand the element to `decode`.
pub fn decode_prototype<F>(mut decode: F) -> KunaResult<()>
where
    F: FnMut(&std::rc::Rc<kuna_base::xml::Element>) -> KunaResult<()>,
{
    use kuna_base::xml::DocumentStorage;
    let xml = prototype_xml();
    let mut store = DocumentStorage::new();
    let root = store.parse_document(xml.as_bytes())?.get_root().clone();
    decode(&root)
}

#[cfg(test)]
mod tests;
