//! DWARF type, member and enumerator names must not restructure emitted C.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = r#"
struct SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS { int FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; };
enum EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE { NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN = 5, other = 9 };
typedef unsigned TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT;
__attribute__((noinline)) int helper(struct SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS *p,
                                     enum EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE e,
                                     TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT t) {
    if (e == NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN) return p->FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF + (int)t;
    return p->FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF - (int)t;
}
int main(int c, char **v) {
    struct SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS s = { c };
    return helper(&s, (enum EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE)c, (TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT)c);
}
"#;

const PATCHES: [(u8, &[u8]); 4] = [
    (b'S', b"S; */ system(\"id\"); /*\n\x1b[2J"),
    (b'F', b"f\n// note: \x1b[0m\x07'"),
    (b'N', b"n*/ }\r\nint x = 1; /*\\"),
    (b'T', b"t//\t\x7f/*"),
];

fn patched(image: &[u8]) -> Vec<u8> {
    let mut out = image.to_vec();
    for (letter, payload) in PATCHES {
        let placeholder = [letter; 40];
        let mut replacement = payload.to_vec();
        replacement.resize(40, b'_');
        let mut hits = 0;
        let mut at = 0;
        while at + 40 <= out.len() {
            if out[at..at + 40] == placeholder {
                out[at..at + 40].copy_from_slice(&replacement);
                hits += 1;
                at += 40;
            } else {
                at += 1;
            }
        }
        assert!(hits > 0, "placeholder {:?} is not in the image", letter as char);
    }
    out
}

fn decompile(binary: &std::path::Path, mode: &str) -> String {
    let (body, err, status) =
        common::run_kuna(&["decompile-all", binary.to_str().unwrap(), "--option", "symbolnamechars", mode]);
    assert_eq!(status, 0, "{mode}: {err}");
    body
}

/// The lines that spell the patched names: the two function bodies.
fn bodies(text: &str) -> String {
    let mut out = String::new();
    for name in ["helper", "main"] {
        let start = text.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("{name} missing:\n{text}"));
        let end = text[start..].find("\n}\n").map_or(text.len(), |n| start + n + 3);
        out.push_str(&text[start..end]);
    }
    out
}

#[test]
#[cfg(target_os = "linux")]
fn dwarf_names_cannot_restructure_the_document() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let input = common::scratch_file("hostile-dwarf-names", "c");
    let plain = input.with_extension("plain");
    let hostile = input.with_extension("hostile");
    std::fs::write(&input, SOURCE).unwrap();
    process::required_output(
        Command::new("gcc").args(["-g", "-O1", "-no-pie"]).arg(&input).arg("-o").arg(&plain),
    );
    std::fs::write(&hostile, patched(&std::fs::read(&plain).unwrap())).unwrap();

    let off = bodies(&decompile(&hostile, "off"));
    for raw in ["S; */ system(\"id\"); /*\n\x1b[2J", "f\n// note: \x1b[0m\x07'"] {
        assert!(off.contains(raw), "off must stay verbatim ({raw:?}):\n{off}");
    }

    for mode in ["safe", "ident"] {
        let text = bodies(&decompile(&hostile, mode));
        for escaped in [
            "S; _x2a_x2f system(_x22id_x22); _x2f_x2a_x0a_x1b[2J",
            "f_x0a_x2f_x2f note: _x1b[0m_x07_x27",
        ] {
            assert!(text.contains(escaped), "{mode}: missing {escaped:?}:\n{text}");
        }
        assert!(
            !text.bytes().any(|b| b == 0x1b || b == 0x07 || b == b'\r' || b == 0x7f),
            "{mode}: a control byte survived:\n{text}"
        );
        assert!(!text.contains("*/") && !text.contains("/*"), "{mode}: a comment delimiter survived:\n{text}");
        let lines = text.lines().count();
        assert_eq!(lines, bodies(&decompile(&plain, mode)).lines().count(), "{mode}: a name split a line:\n{text}");
    }

    assert_eq!(decompile(&plain, "safe"), decompile(&plain, "off"), "ordinary names must not change");
    let _ = (std::fs::remove_file(input), std::fs::remove_file(plain), std::fs::remove_file(hostile));
}
