//! (kuna) MSVC `__ftol` family call-fixup — lower the x86-32 floating-point to
//! 64-bit-integer CRT helper to p-code so its x87 argument survives.
//!
//! # The defect
//!
//! MSVC compiles a floating-point to integer conversion on x86-32 into
//! `fld <src>; call __ftol`: the argument arrives in the x87 stack top `ST0` and
//! the `__int64` result comes back in `EDX:EAX`. No prototype model in any
//! vendored x86 compiler spec has an `<input>` `<pentry>` naming an x87 register
//! — `ST0` appears only as an `<output>` pentry and in `<killedbycall>` — so a
//! call to `__ftol` characterizes `ST0` as `NoContainment` and `Heritage`'s
//! call guard never appends the argument. With nothing reading `ST0`, the whole
//! feeding `FLD` chain is dead-code eliminated: the call renders as a bare
//! `__ftol()` and every value that fed it disappears, which in turn kills the
//! registers those loads were based on (on a `__thiscall` method, that is the
//! `ECX` `this` pointer).
//!
//! Adding an `ST0` input pentry to the shared prototype model is NOT the fix: it
//! invents a phantom `float10` first argument on every unrelated stack-passing
//! callee.
//!
//! # The fix
//!
//! Register a `<callfixup>` — the same mechanism upstream Ghidra uses for
//! `alloca_probe` / `mcount` / `__security_check_cookie` — that *replaces* the
//! CALL with the helper's semantics in p-code. The argument is then an ordinary
//! `ST0` read, the result an ordinary `EDX:EAX` write, and the conversion prints
//! as a plain C cast. This is what Hex-Rays does with its own compiler-helper
//! table (`ftol`, `ftol2`, `ftol2_sse`, …).
//!
//! The payload is synthesized here rather than added to
//! `specs/Ghidra/Processors/x86/data/languages/x86win.cspec` so the vendored
//! spec tree stays byte-identical to upstream (see the sync procedure in
//! `docs/history.md`). It is registered through the same
//! `PcodeInjectLibrary::decode_inject` path the cspec `<callfixup>` elements use,
//! immediately after them, so `parse_inject_all` compiles it with the rest.
//!
//! # Body
//!
//! ```text
//! ESP = ESP + 4;                  // the CALL's pushed return address
//! ftol_res:8 = trunc(ST0);        // __int64 truncation toward zero
//! EAX = ftol_res:4;               // low half
//! EDX = ftol_res(4);              // high half
//! ST0 = ST1; … ST6 = ST7;         // the x87 pop (`fpop()`, ia.sinc:2283)
//! ```
//!
//! `ESP = ESP + 4` is mandatory: x86 `CALL rel32` lifts as
//! `push44(&:4 inst_next); call rel32;` (`ia.sinc:2962`), so a fixup that
//! replaces the CALL leaves the pushed return address on the stack. Without the
//! pop those addresses surface as bogus arguments to the *next* call. Upstream's
//! own `alloca_probe` fixup carries the same correction (`ESP = ESP + 4 - EAX`).
//!
//! The result is modelled at its true 64-bit width rather than as a
//! sign-extended 32-bit value. A caller that uses only `EAX` (the `(int)d` case,
//! which dominates) therefore reads `(int)(long long)x`; modelling `EDX` as
//! `EAX s>> 31` would render that case one cast shorter but would silently
//! produce the wrong high word for a `(__int64)d` conversion.
//!
//! # Targets
//!
//! `__ftol`, `__ftol2`, `__ftol2_sse` — the three MSVC CRT spellings of the same
//! contract (`_ftol` is pre-VS2002, `_ftol2` VS2002+, `_ftol2_sse` the SSE2 path;
//! all take `ST0` and return `EDX:EAX`). The installer probes a function name
//! bare, then `_`-prefixed, then `__`-prefixed
//! (`CallFixupAnalyzer.getCallFixupNameForFunction`), so each `__`-spelled target
//! also covers the `_`- and bare-spelled function names.

use kuna_base::error::KunaResult;

/// The `<callfixup name=…>` this module registers. The installer
/// (`kuna_analysis::analyzers::callfixup`) suppresses this one fixup — and only
/// this one — when `option msvcftol off`.
pub const FIXUP_NAME: &str = "kuna_msvcftol";

/// The registers the body reads or writes. Presence of all of them (at a 32-bit
/// default code space) is the architecture guard: `ST0` exists only on x86, and
/// `ESP`/`EAX`/`EDX` narrow that to the 32-bit variant, where alone this helper
/// exists.
const REQUIRED_REGISTERS: [&str; 11] =
    ["ST0", "ST1", "ST2", "ST3", "ST4", "ST5", "ST6", "ST7", "EAX", "EDX", "ESP"];

/// The synthesized `<callfixup>` element, in the shape
/// `Architecture::decode_call_fixups` hands to `decode_inject`.
pub fn payload_xml() -> String {
    format!(
        r#"<callfixup name="{FIXUP_NAME}">
  <target name="__ftol"/>
  <target name="__ftol2"/>
  <target name="__ftol2_sse"/>
  <pcode>
    <body><![CDATA[
      ESP = ESP + 4;
      ftol_res:8 = trunc(ST0);
      EAX = ftol_res:4;
      EDX = ftol_res(4);
      ST0 = ST1; ST1 = ST2; ST2 = ST3; ST3 = ST4; ST4 = ST5; ST5 = ST6; ST6 = ST7;
    ]]></body>
  </pcode>
</callfixup>"#
    )
}

/// Whether this language is the x86-32 target the `__ftol` family belongs to:
/// every register the body touches resolves, and the default code space is 4
/// bytes wide. x86-64 also resolves `EAX`/`EDX`/`ESP`/`ST0` as sub-registers, so
/// the width test is what excludes it — MSVC x64 has no `_ftol`.
pub fn language_is_x86_32(
    resolve_register: impl Fn(&[u8]) -> bool,
    code_space_addr_size: i32,
) -> bool {
    code_space_addr_size == 4 && REQUIRED_REGISTERS.iter().all(|r| resolve_register(r.as_bytes()))
}

/// Parse [`payload_xml`] into the decoder shape `decode_inject` consumes, and
/// hand it to `register`. Split from the architecture so the XML and the guard
/// can be unit-tested without a bootstrapped engine.
pub fn decode_payload<F>(mut register: F) -> KunaResult<()>
where
    F: FnMut(&std::rc::Rc<kuna_base::xml::Element>) -> KunaResult<()>,
{
    use kuna_base::xml::DocumentStorage;
    let xml = payload_xml();
    let mut store = DocumentStorage::new();
    let root = store.parse_document(xml.as_bytes())?.get_root().clone();
    register(&root)
}

#[cfg(test)]
mod tests;
