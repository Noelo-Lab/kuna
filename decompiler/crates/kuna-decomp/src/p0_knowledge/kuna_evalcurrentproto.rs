//! (kuna) Evaluate a function's **own** unknown prototype
//! with the model the compiler spec nominates, so register-passed parameters are
//! recovered instead of surfacing as reads of an undefined local.
//!
//! # The gap
//!
//! A compiler spec can declare a *merged* model — a `<resolveprototype>` union of
//! candidate conventions — and then nominate it for evaluating the current
//! function's prototype:
//!
//! ```xml
//! <resolveprototype name="__fastcall/__thiscall/__stdcall">
//!   <model name="__stdcall"/>    <!-- The default case -->
//!   <model name="__fastcall"/>
//!   <model name="__thiscall"/>
//! </resolveprototype>
//! <eval_current_prototype name="__fastcall/__thiscall/__stdcall"/>
//! ```
//!
//! kuna decoded the `<resolveprototype>` union and registered it by name, and
//! every consumer of the choice was already wired — `ActionPrototypeTypes` sets
//! the function's model from `glb->evalfp_current` (`coreaction.cc:4843`),
//! `ActionInputPrototype` calls `FuncProto::resolveModel` on it
//! (`fspec.cc:3772`), and `ProtoModelMerged::selectModel` scores the trial set
//! against each constituent — but the `<eval_current_prototype>` element itself
//! was never read, so `evalfp_current` stayed null and the fallback was the
//! `<default_proto>` model. On x86 Windows that default is `__stdcall`: inputs
//! are stack-only, so a `__fastcall`/`__thiscall` function's `ECX`/`EDX`
//! arguments are not *possible* parameters, and the reads of them at entry are
//! left as undefined locals:
//!
//! ```text
//!   uint4 sub_6ae390(void)             |   uint4 sub_6ae390(int4 a0,int4 a1)
//!   {                                  |   {
//!     int4 v4; // ecx                   |
//!     ...                               |     ...
//!     v3 = *(uint1 *)(v4 + 0xf5);       |     v3 = *(uint1 *)(a0 + 0xf5);
//! ```
//!
//! The left column is not merely less informative — `v4` is read before it is
//! ever written, which is not a thing a C function can do. Ghidra and IDA both
//! report this function as taking two arguments.
//!
//! # What the rule does
//!
//! The spec's `<eval_current_prototype>` model is decoded at
//! `<default_proto>` time and handed to each function as the model its unlocked
//! input recovery is evaluated with; the merged model then *resolves* per
//! function against the trials that are actually active, so a function that
//! touches neither `ECX` nor `EDX` still comes out `__stdcall`. An explicit
//! `option protoeval <model>` still wins — it is the same `evalfp_current` slot,
//! set by hand.
//!
//! Six vendored specs nominate a model: `x86win` (`__fastcall/__thiscall/
//! __stdcall`), `x86borland` (`__stdcall/__fastcall`), `x86gcc`
//! (`__cdecl/__regparm`), `CR16` (`__ptrcall/__ptrcall2/__stdcall`) and
//! `HCS12`/`HCS12X` (`__asmA_longcall/__asmA`). Every other language has no such
//! element and is untouched.
//!
//! `<eval_called_prototype>` — the sibling element that nominates the model for
//! *callees* — is deliberately not decoded here: no vendored spec declares one,
//! so the code would be unreachable and unmeasurable.

/// The model name a compiler spec's `<eval_current_prototype name="..."/>`
/// nominates for evaluating the current function's prototype (C++
/// `Architecture::parseCompilerConfig`'s `ELEM_EVAL_CURRENT_PROTOTYPE` arm,
/// `architecture.cc:1321`). `None` when the spec declares no such element (most
/// languages) or it carries no `name`.
pub fn eval_current_model_name(root: &std::rc::Rc<kuna_base::xml::Element>) -> Option<String> {
    root.get_children()
        .iter()
        .find(|c| c.get_name() == "eval_current_prototype")
        .and_then(|el| {
            el.get_attribute_value("name").ok().map(|v| String::from_utf8_lossy(v).into_owned())
        })
        .filter(|n: &String| !n.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(xml: &str) -> std::rc::Rc<kuna_base::xml::Element> {
        let mut store = kuna_base::xml::DocumentStorage::new();
        store.parse_document(xml.as_bytes()).unwrap().get_root().clone()
    }

    /// The nominated model name is read off the element; a spec without one (the
    /// common case) yields `None`, which is what leaves the language untouched.
    #[test]
    fn reads_the_nominated_model_name() {
        let with = parse(
            r#"<compiler_spec>
                 <eval_current_prototype name="__fastcall/__thiscall/__stdcall"/>
               </compiler_spec>"#,
        );
        assert_eq!(
            eval_current_model_name(&with).as_deref(),
            Some("__fastcall/__thiscall/__stdcall")
        );

        let without = parse(r#"<compiler_spec><default_proto/></compiler_spec>"#);
        assert_eq!(eval_current_model_name(&without), None);

        let empty = parse(r#"<compiler_spec><eval_current_prototype name=""/></compiler_spec>"#);
        assert_eq!(eval_current_model_name(&empty), None, "an empty name nominates nothing");
    }
}
