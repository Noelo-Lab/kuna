//! (kuna `objcarc`) Hide ARC bookkeeping: the Objective-C runtime's
//! retain/release/autorelease calls become the copies and stores they stand
//! for, through `<callfixup>` payloads registered beside the compiler specs'
//! (the `kuna_msvcftol` pattern).
//!
//! An ARC-compiled method is strewn with `objc_retain(x)`, `objc_release(x)`,
//! `v = objc_retainAutoreleasedReturnValue(v)` and `objc_storeStrong(&slot, x)`.
//! ARC's contract makes them data-flow neutral, so each is replaced in p-code
//! by what it stands for — a pass-through
//! helper's body is `x0 = x0` marked an incidental copy (its result keeps the
//! argument's variable), a release has no effect, and `objc_storeStrong` is the
//! store `*slot = value` (the old value's release is bookkeeping). Clang's
//! outlined `objc_retain_x<N>` / `objc_release_x<N>` (arm64, the object in
//! `x<N>`, a retain's result in `x0`) get the same treatment per register.
//!
//! x86-64 `CALL` lifts as a push of the return address, so every x86-64 body
//! starts with `RSP = RSP + 8` (the `alloca_probe` correction); arm64 `bl` only
//! writes `x30`.

use kuna_base::error::KunaResult;

/// Every fixup this module registers starts with this; the installer
/// (`kuna_analysis::analyzers::callfixup`) drops them all under `option objcarc off`.
pub const FIXUP_PREFIX: &str = "kuna_objcarc_";

/// Runtime entry points whose result is their argument.
pub const PASS_THROUGH: [&str; 9] = [
    "objc_retain",
    "objc_autorelease",
    "objc_retainAutorelease",
    "objc_retainAutoreleaseReturnValue",
    "objc_autoreleaseReturnValue",
    "objc_retainAutoreleasedReturnValue",
    "objc_unsafeClaimAutoreleasedReturnValue",
    "objc_claimAutoreleasedReturnValue",
    "objc_retainBlock",
];

/// Runtime entry points with no value: a release.
pub const DROPPED: [&str; 1] = ["objc_release"];

/// `objc_storeStrong(id *slot, id value)`: the store.
pub const STORE: &str = "objc_storeStrong";

/// The highest `x<N>` clang outlines an ARC call on (`x29` is the frame pointer).
pub const MAX_OUTLINED_REG: u32 = 28;

fn fixup(name: &str, targets: &[String], body: &str) -> String {
    let targets: String = targets.iter().map(|t| format!("  <target name=\"{t}\"/>\n")).collect();
    format!(
        "<callfixup name=\"{FIXUP_PREFIX}{name}\">\n{targets}  <pcode incidentalcopy=\"true\">\n    <body><![CDATA[\n      {body}\n    ]]></body>\n  </pcode>\n</callfixup>\n"
    )
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

/// The arm64 payloads: pass-through, release, store, and one per outlined register.
pub fn payload_xml_aarch64() -> String {
    let mut xml = String::new();
    xml.push_str(&fixup("pass", &names(&PASS_THROUGH), "x0 = x0;"));
    xml.push_str(&fixup("drop", &names(&DROPPED), "x0 = x0;"));
    xml.push_str(&fixup("store", &[STORE.to_string()], "*:8 x0 = x1;"));
    for n in 0..=MAX_OUTLINED_REG {
        xml.push_str(&fixup(&format!("retain_x{n}"), &[format!("objc_retain_x{n}")], &format!("x0 = x{n};")));
        xml.push_str(&fixup(&format!("release_x{n}"), &[format!("objc_release_x{n}")], &format!("x{n} = x{n};")));
    }
    xml
}

/// The x86-64 payloads.
pub fn payload_xml_x86_64() -> String {
    let mut xml = String::new();
    xml.push_str(&fixup("pass", &names(&PASS_THROUGH), "RSP = RSP + 8; RAX = RDI;"));
    xml.push_str(&fixup("drop", &names(&DROPPED), "RSP = RSP + 8;"));
    xml.push_str(&fixup("store", &[STORE.to_string()], "RSP = RSP + 8; *:8 RDI = RSI;"));
    xml
}

/// The payload for this language: arm64 when `x0`..`x28` resolve at an 8-byte
/// code space, x86-64 when `RAX`/`RDI`/`RSI`/`RSP` do; `None` elsewhere.
pub fn payload_xml(resolve_register: impl Fn(&[u8]) -> bool, code_space_addr_size: i32) -> Option<String> {
    if code_space_addr_size != 8 {
        return None;
    }
    if resolve_register(b"x0") && resolve_register(b"x28") && resolve_register(b"x30") {
        return Some(payload_xml_aarch64());
    }
    if ["RAX", "RDI", "RSI", "RSP"].iter().all(|r| resolve_register(r.as_bytes())) {
        return Some(payload_xml_x86_64());
    }
    None
}

/// Parse `xml` — one `<callfixup>` after another — and hand each root to `register`.
pub fn decode_payload<F>(xml: &str, mut register: F) -> KunaResult<()>
where
    F: FnMut(&std::rc::Rc<kuna_base::xml::Element>) -> KunaResult<()>,
{
    use kuna_base::xml::DocumentStorage;
    let wrapped = format!("<kuna_objcarc>\n{xml}</kuna_objcarc>");
    let mut store = DocumentStorage::new();
    let root = store.parse_document(wrapped.as_bytes())?.get_root().clone();
    for child in root.get_children() {
        register(child)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm64_payload_covers_every_helper_and_register() {
        let xml = payload_xml_aarch64();
        for t in PASS_THROUGH.iter().chain(DROPPED.iter()) {
            assert!(xml.contains(&format!("<target name=\"{t}\"/>")), "{t}");
        }
        assert!(xml.contains("*:8 x0 = x1;"));
        assert!(xml.contains("<target name=\"objc_retain_x28\"/>"));
        assert!(xml.contains("<target name=\"objc_release_x19\"/>"));
        assert!(!xml.contains("objc_retain_x29"));
        assert_eq!(xml.matches("<callfixup name=").count(), 3 + 2 * (MAX_OUTLINED_REG as usize + 1));
    }

    #[test]
    fn language_guard_picks_the_payload() {
        let arm = |r: &[u8]| r.starts_with(b"x");
        assert!(payload_xml(arm, 8).unwrap().contains("x0 = x0;"));
        assert!(payload_xml(arm, 4).is_none());
        let x64 = |r: &[u8]| matches!(r, b"RAX" | b"RDI" | b"RSI" | b"RSP");
        assert!(payload_xml(x64, 8).unwrap().contains("RSP = RSP + 8; RAX = RDI;"));
        assert!(payload_xml(|_| false, 8).is_none());
    }

    #[test]
    fn payload_parses_as_one_element_per_fixup() {
        let mut n = 0;
        decode_payload(&payload_xml_x86_64(), |root| {
            assert_eq!(root.get_name(), "callfixup");
            n += 1;
            Ok(())
        })
        .unwrap();
        assert_eq!(n, 3);
    }
}
