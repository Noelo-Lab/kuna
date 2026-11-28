//! Issue #1824: a computed jump whose jump table the reference walk cannot read
//! leaves its case bodies undecoded, and `kuna xrefs` says so in
//! `coverage.notes` instead of answering an empty `--to` in silence. A jump
//! through a pointer, an import slot or a returned value, and a switch whose
//! table the walk reads, add no note (`unreadtable_x86_64`).

use crate::common;
use serde_json::Value;

fn xrefs(args: &[&str]) -> Value {
    let binary = common::fixture("unreadtable_x86_64");
    let mut argv = vec!["xrefs", binary.as_str()];
    argv.extend_from_slice(args);
    argv.push("--json");
    let (stdout, stderr, rc) = common::run_kuna(&argv);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    serde_json::from_str(&stdout).unwrap()
}

fn notes(doc: &Value) -> Vec<String> {
    doc["coverage"]["notes"].as_array().unwrap().iter().map(|n| n.as_str().unwrap().to_owned()).collect()
}

/// The dispatch addresses a note lists, in order.
fn sites(note: &str) -> Vec<String> {
    note.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| w.starts_with("0x")).map(str::to_owned).collect()
}

#[test]
fn an_empty_answer_names_the_jump_tables_the_walk_could_not_read() {
    let doc = xrefs(&["--to", "t_c_0"]);
    assert_eq!(doc["count"], 0);
    let notes = notes(&doc);
    assert_eq!(notes.len(), 1, "{notes:?}");
    let note = &notes[0];
    assert!(note.starts_with("2 computed jumps dispatch through jump tables the walk could not read"), "{note}");
    assert!(note.contains(" in caller_saved, ") && note.contains(" in unguarded:"), "{note}");
    assert_eq!(sites(note).len(), 2, "{note}");
}

#[test]
fn a_function_with_an_unread_table_says_so_and_its_dispatch_does_too() {
    for function in ["caller_saved", "unguarded"] {
        let notes = notes(&xrefs(&["--from", function]));
        assert_eq!(notes.len(), 1, "{function}: {notes:?}");
        let note = &notes[0];
        assert!(note.starts_with(&format!("{function} dispatches through a jump table the walk could not read")), "{note}");
        let [dispatch] = sites(note).try_into().unwrap();
        let notes = self::notes(&xrefs(&["--from", &dispatch]));
        assert_eq!(notes.len(), 1, "{dispatch}: {notes:?}");
        assert!(notes[0].starts_with(&format!("{dispatch} is a computed jump through a jump table")), "{notes:?}");
    }
}

#[test]
fn a_read_table_a_tail_call_through_a_pointer_and_a_found_reference_add_no_note() {
    for function in ["resolved", "via_field", "via_argument", "via_slot", "via_loaded_slot", "via_vtable", "via_return"] {
        let notes = notes(&xrefs(&["--from", function]));
        assert!(notes.is_empty(), "{function}: {notes:?}");
    }
    let doc = xrefs(&["--to", "t_r_0"]);
    assert_eq!(doc["count"], 1);
    assert!(notes(&doc).is_empty(), "{:?}", notes(&doc));
}
