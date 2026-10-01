use super::*;
use crate::prettyprint::BraceStyle;
use crate::printc::PrintC;

fn render(body: impl FnOnce(&mut PrintEmit)) -> String {
    let mut print = PrintC::new();
    print.set_output_stream();
    let e = print.emit_mut();
    let id = e.open_brace_indent("{", BraceStyle::SameLine);
    body(e);
    e.close_brace_indent("}", id);
    e.output_str().to_string()
}

fn label(e: &mut PrintEmit, name: &str) {
    e.tag_line();
    e.print(name, SyntaxHighlight::NoColor);
    e.print(":", SyntaxHighlight::NoColor);
    e.note_label();
}

fn statement(e: &mut PrintEmit, text: &str) {
    e.tag_line();
    let id = e.begin_statement(&MarkupRef::none());
    e.print(text, SyntaxHighlight::NoColor);
    e.end_statement(id);
}

#[test]
fn a_label_before_a_closing_brace_gets_a_null_statement() {
    assert_eq!(render(|e| label(e, "done")), " {\n  done:\n  ;\n}");
}

#[test]
fn a_comment_after_the_label_does_not_count_as_its_statement() {
    let out = render(|e| {
        label(e, "done");
        e.tag_line();
        e.print("/* note */", SyntaxHighlight::CommentColor);
    });
    assert_eq!(out, " {\n  done:\n  /* note */\n  ;\n}");
}

#[test]
fn a_labeled_statement_or_block_is_left_alone() {
    let out = render(|e| {
        label(e, "top");
        statement(e, "x = 1;");
    });
    assert_eq!(out, " {\n  top:\n  x = 1;\n}");
    let out = render(|e| {
        label(e, "top");
        e.tag_line();
        e.print("do", SyntaxHighlight::KeywordColor);
        let id = e.open_brace_indent("{", BraceStyle::SameLine);
        e.close_brace_indent("}", id);
        e.print(" while (x);", SyntaxHighlight::NoColor);
    });
    assert_eq!(out, " {\n  top:\n  do {\n  } while (x);\n}");
}

#[test]
fn consecutive_labels_share_one_statement() {
    let out = render(|e| {
        label(e, "case 1");
        label(e, "case 2");
    });
    assert_eq!(out, " {\n  case 1:\n  case 2:\n  ;\n}");
}

#[test]
fn a_new_function_forgets_a_label_left_by_an_aborted_one() {
    let mut print = PrintC::new();
    print.set_output_stream();
    let e = print.emit_mut();
    e.note_label();
    let id = e.begin_function();
    e.end_function(id);
    assert!(!e.label_dangling());
}
