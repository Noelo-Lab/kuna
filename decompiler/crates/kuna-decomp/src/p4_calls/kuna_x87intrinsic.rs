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
//! [`MODEL_NAME`] takes one floating-point input in `ST0` and returns one in
//! `ST0`, pops nothing from the stack (`extrapop` 4), and preserves the registers
//! `__cdecl` preserves. [`BINARY_MODEL_NAME`] serves the two-operand helpers
//! (`__CIpow`, `__CIatan2`, `__CIfmod`): the first operand in `ST1`, the second
//! in `ST0` (the order the source loads them, so `atan2(y, x)` reads `y`
//! first), the result in `ST0`, and `ST1` killed. Those helpers pop one x87
//! slot, which a model cannot state; MSVC calls them with only their two
//! operands on the x87 stack, so nothing the caller keeps there is displaced.

use kuna_base::error::KunaResult;

/// The one-operand model's name, as bound by the analysis tier.
pub const MODEL_NAME: &str = "__kuna_x87unary";

/// The two-operand model's name.
pub const BINARY_MODEL_NAME: &str = "__kuna_x87binary";

/// The one-operand intrinsics of the MSVC CRT.
pub const INTRINSICS: [&str; 10] = [
    "__CIsqrt", "__CIsin", "__CIcos", "__CItan", "__CIexp", "__CIlog", "__CIlog10", "__CIasin",
    "__CIacos", "__CIatan",
];

/// The two-operand intrinsics of the MSVC CRT, with their operand names in
/// `ST1, ST0` order.
pub const BINARY_INTRINSICS: [(&str, [&str; 2]); 3] =
    [("__CIpow", ["x", "y"]), ("__CIatan2", ["y", "x"]), ("__CIfmod", ["x", "y"])];

fn bare(name: &str) -> &str {
    name.trim_start_matches('_')
}

/// The model of the intrinsic `name`, spelled with or without leading
/// underscores, with its operand names; `None` for any other name.
pub fn model_for(name: &str) -> Option<(&'static str, &'static [&'static str])> {
    if INTRINSICS.iter().any(|i| bare(i) == bare(name)) {
        return Some((MODEL_NAME, &["x"]));
    }
    BINARY_INTRINSICS
        .iter()
        .find(|(i, _)| bare(i) == bare(name))
        .map(|(_, names)| (BINARY_MODEL_NAME, &names[..]))
}

/// Whether `name` is one of the intrinsics.
pub fn is_intrinsic(name: &str) -> bool {
    model_for(name).is_some()
}

const EFFECTS: &str = r#"  <unaffected>
    <register name="ESP"/>
    <register name="EBP"/>
    <register name="ESI"/>
    <register name="EDI"/>
    <register name="EBX"/>
    <register name="DF"/>
  </unaffected>"#;

const ST: &str = r#"<pentry minsize="4" maxsize="10" metatype="float" extension="float">"#;

/// The synthesized one-operand `<prototype>` element.
pub fn prototype_xml() -> String {
    format!(
        r#"<prototype name="{MODEL_NAME}" extrapop="4" stackshift="4">
  <input>
    {ST}<register name="ST0"/></pentry>
  </input>
  <output killedbycall="true">
    {ST}<register name="ST0"/></pentry>
  </output>
{EFFECTS}
  <killedbycall>
    <register name="EAX"/>
    <register name="ECX"/>
    <register name="EDX"/>
    <register name="ST0"/>
  </killedbycall>
</prototype>"#
    )
}

/// The synthesized two-operand `<prototype>` element.
pub fn binary_prototype_xml() -> String {
    format!(
        r#"<prototype name="{BINARY_MODEL_NAME}" extrapop="4" stackshift="4">
  <input>
    {ST}<register name="ST1"/></pentry>
    {ST}<register name="ST0"/></pentry>
  </input>
  <output killedbycall="true">
    {ST}<register name="ST0"/></pentry>
  </output>
{EFFECTS}
  <killedbycall>
    <register name="EAX"/>
    <register name="ECX"/>
    <register name="EDX"/>
    <register name="ST0"/>
    <register name="ST1"/>
  </killedbycall>
</prototype>"#
    )
}

/// Parse both prototypes and hand each element to `decode`.
pub fn decode_prototype<F>(mut decode: F) -> KunaResult<()>
where
    F: FnMut(&std::rc::Rc<kuna_base::xml::Element>) -> KunaResult<()>,
{
    use kuna_base::xml::DocumentStorage;
    for xml in [prototype_xml(), binary_prototype_xml()] {
        let mut store = DocumentStorage::new();
        let root = store.parse_document(xml.as_bytes())?.get_root().clone();
        decode(&root)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
