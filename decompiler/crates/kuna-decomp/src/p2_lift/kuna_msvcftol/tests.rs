//! Unit tests for the MSVC `__ftol` call-fixup payloads: the synthesized XML
//! parses into the shape `decode_inject` consumes, splits the CRT targets by
//! result width, and the architecture guard admits x86-32 alone.

use super::*;

/// Both payloads parse as `<callfixup>` elements named [`FIXUP_NAME`] and
/// [`FIXUP_NAME_SSE`].
#[test]
fn payloads_parse_as_named_callfixups() {
    let mut seen = Vec::new();
    decode_payload(|root| {
        seen.push((
            root.get_name().to_string(),
            String::from_utf8_lossy(root.get_attribute_value("name")?).into_owned(),
        ));
        Ok(())
    })
    .expect("payloads must parse");
    assert_eq!(
        seen,
        vec![
            ("callfixup".to_string(), FIXUP_NAME.to_string()),
            ("callfixup".to_string(), FIXUP_NAME_SSE.to_string()),
        ]
    );
}

/// The 64-bit payload targets `__ftol`/`__ftol2`, the 32-bit one the SSE2
/// entries. The installer probes a function name bare / `_`-prefixed /
/// `__`-prefixed, so the `__`-spelled targets also cover `ftol` / `_ftol` etc.
#[test]
fn payloads_split_the_crt_targets_by_result_width() {
    let mut targets: Vec<Vec<String>> = Vec::new();
    decode_payload(|root| {
        let mut t = Vec::new();
        for child in root.get_children().iter() {
            if child.get_name() == "target" {
                t.push(String::from_utf8_lossy(child.get_attribute_value("name")?).into_owned());
            }
        }
        targets.push(t);
        Ok(())
    })
    .expect("payloads must parse");
    assert_eq!(
        targets,
        vec![
            vec!["__ftol", "__ftol2"],
            vec!["__ftol2_sse", "__ftol2_pentium4", "__ftol2_sse_excpt"],
        ]
    );
}

/// The SSE2 entries return only `EAX` and leave `EDX` alone.
#[test]
fn sse_body_converts_into_eax_only() {
    let xml = payload_sse_xml();
    assert!(xml.contains("ESP = ESP + 4;"));
    assert!(xml.contains("EAX = trunc(ST0);"));
    assert!(!xml.contains("EDX"), "the SSE2 path never writes EDX");
    assert!(xml.contains("ST6 = ST7;"));
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
