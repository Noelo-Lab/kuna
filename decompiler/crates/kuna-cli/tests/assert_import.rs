use std::path::PathBuf;
use std::process::{Command, Output};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/assert_import_x86_64.elf")
}

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg(args[0])
        .arg(fixture())
        .args(&args[1..])
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

const IMPORTS: [&str; 4] = ["--assert", "import 0x3f8 puts", "--assert", "import 0x400 atoi"];

#[test]
fn a_call_through_an_unbound_slot_is_anonymous() {
    let out = kuna(&["decompile", "greet"]);
    let stdout = text(&out.stdout);
    assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
    assert!(stdout.contains("(*dat_3f8)("), "{stdout}");
    assert!(!stdout.contains("puts"), "{stdout}");
}

#[test]
fn a_declared_import_slot_names_the_call_through_it() {
    for extra in [&[][..], &["--json"][..]] {
        let mut args = vec!["decompile", "greet"];
        args.extend_from_slice(&IMPORTS);
        args.extend_from_slice(extra);
        let out = kuna(&args);
        let stdout = text(&out.stdout);
        assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
        assert!(stdout.contains(r#"puts(\"hello\");"#) || stdout.contains(r#"puts("hello");"#), "{stdout}");
        assert!(stdout.contains("return atoi(a0) + 1;"), "{stdout}");
        assert!(stdout.contains("greet(char *a0)"), "{stdout}");
        assert!(!stdout.contains("dat_3f8"), "{stdout}");
    }
}

#[test]
fn the_slot_stays_a_pointer_word() {
    let mut args = vec!["decompile-all"];
    args.extend_from_slice(&IMPORTS);
    let out = kuna(&args);
    let stdout = text(&out.stdout);
    assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
    assert!(stdout.contains("dat_3f8 = resolve("), "{stdout}");
    assert!(!stdout.contains("// Function: puts"), "{stdout}");
    assert!(stdout.contains("return atoi(a0) + 1;"), "{stdout}");
}

#[test]
fn the_directive_is_reported_on_the_environment_plane() {
    let mut args = vec!["decompile", "greet", "--json"];
    args.extend_from_slice(&IMPORTS[..2]);
    let out = kuna(&args);
    let stdout = text(&out.stdout);
    assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let row = &doc["assertions"][0];
    assert_eq!(row["kind"], "import", "{stdout}");
    assert_eq!(row["phase"], "P1", "{stdout}");
    assert_eq!(row["subphase"], "environment-binding", "{stdout}");
    assert_eq!(row["status"], "applied", "{stdout}");
}
