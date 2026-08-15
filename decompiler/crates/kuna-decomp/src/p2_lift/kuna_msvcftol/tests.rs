//! Unit tests for the MSVC `__ftol` call-fixup payload: the synthesized XML
//! parses into the shape `decode_inject` consumes, carries the three CRT target
//! spellings, and the architecture guard admits x86-32 alone.

use super::*;

/// The payload parses as a `<callfixup>` whose `name` is [`FIXUP_NAME`] — the
/// name the installer suppression keys on.
#[test]
fn payload_parses_as_a_named_callfixup() {
    let mut seen = None;
    decode_payload(|root| {
        seen = Some((
            root.get_name().to_string(),
            String::from_utf8_lossy(root.get_attribute_value("name")?).into_owned(),
        ));
        Ok(())
    })
    .expect("payload_xml must parse");
    assert_eq!(seen, Some(("callfixup".to_string(), FIXUP_NAME.to_string())));
}

/// All three CRT spellings are targets. The installer probes a function name
/// bare / `_`-prefixed / `__`-prefixed, so the `__`-spelled targets also cover
/// `ftol` / `_ftol` etc.; spelling them `__`-first is what makes that work.
#[test]
fn payload_targets_the_three_crt_spellings() {
    let mut targets: Vec<String> = Vec::new();
    decode_payload(|root| {
        for child in root.get_children().iter() {
            if child.get_name() == "target" {
                targets.push(
                    String::from_utf8_lossy(child.get_attribute_value("name")?).into_owned(),
                );
            }
        }
        Ok(())
    })
    .expect("payload_xml must parse");
    assert_eq!(targets, vec!["__ftol", "__ftol2", "__ftol2_sse"]);
}

/// The body pops the CALL's pushed return address, converts at the full 64-bit
/// width, splits the halves into `EDX:EAX`, and pops the x87 stack. Omitting the
/// `ESP` correction leaks the pushed `inst_next` into the next call's arguments,
/// so it is asserted explicitly rather than left to review.
#[test]
fn body_pops_the_return_address_and_converts_at_64_bits() {
    let xml = payload_xml();
    assert!(xml.contains("ESP = ESP + 4;"), "the CALL's pushed return address must be popped");
    assert!(xml.contains("ftol_res:8 = trunc(ST0);"), "conversion is a 64-bit truncation");
    assert!(xml.contains("EAX = ftol_res:4;"), "low half returns in EAX");
    assert!(xml.contains("EDX = ftol_res(4);"), "high half returns in EDX");
    assert!(xml.contains("ST6 = ST7;"), "the x87 stack must be popped (`fpop()`)");
}

/// x86-32 admits; x86-64 does not (it resolves the same sub-registers but its
/// code space is 8 bytes, and MSVC x64 ships no `_ftol`); a language with no x87
/// does not.
#[test]
fn guard_admits_x86_32_only() {
    let x86 = |_: &[u8]| true;
    let no_x87 = |n: &[u8]| !n.starts_with(b"ST");
    assert!(language_is_x86_32(x86, 4), "x86-32 is the target");
    assert!(!language_is_x86_32(x86, 8), "x86-64 has no _ftol helper");
    assert!(!language_is_x86_32(no_x87, 4), "a language without x87 cannot run this body");
}
