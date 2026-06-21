//! Tests for the W8 `prettyprint` port.
//!
//! The golden outputs in [`engine`] were produced by a standalone harness that
//! compiles the **real** Ghidra `EmitPrettyPrint`/`EmitNoMarkup`/`TokenSplit`/
//! `circularqueue` code (engine-only extraction, no-markup back-end) and runs
//! the same token streams these tests drive.  They pin the byte-for-byte line
//! breaking / indentation that `print C` produces and the datatests match.

use super::*;
use std::rc::Rc;

use kuna_base::space::ConstantSpace;

/// Drive a no-markup [`EmitPrettyPrint`] at a given width and return the text.
fn run(maxline: int4, build: impl FnOnce(&mut EmitPrettyPrint)) -> String {
    let mut pp = EmitPrettyPrint::new();
    pp.set_max_line_size_pretty(maxline).expect("max line size");
    build(&mut pp);
    pp.try_flush().expect("flush");
    pp.output().to_string()
}

// ===========================================================================
// circularqueue
// ===========================================================================

mod circularqueue {
    use super::*;

    #[test]
    fn empty_on_construct() {
        let q: CircularQueue<int4> = CircularQueue::new(8);
        assert!(q.empty());
        assert_eq!(q.get_max(), 8);
    }

    #[test]
    fn push_pop_stack() {
        // Acts as a stack: push() / pop().
        let mut q: CircularQueue<int4> = CircularQueue::new(8);
        let s0 = q.push();
        *q.get_ref_mut(s0) = 10;
        let s1 = q.push();
        *q.get_ref_mut(s1) = 20;
        assert!(!q.empty());
        assert_eq!(*q.top(), 20);
        let popped = q.pop();
        assert_eq!(*q.get_ref(popped), 20);
        assert_eq!(*q.top(), 10);
        q.pop();
        assert!(q.empty());
    }

    #[test]
    fn push_popbottom_queue() {
        // Acts as a queue: push() / popbottom().
        let mut q: CircularQueue<int4> = CircularQueue::new(8);
        for v in [1, 2, 3] {
            let s = q.push();
            *q.get_ref_mut(s) = v;
        }
        assert_eq!(*q.bottom(), 1);
        let b = q.popbottom();
        assert_eq!(*q.get_ref(b), 1);
        assert_eq!(*q.bottom(), 2);
        let b = q.popbottom();
        assert_eq!(*q.get_ref(b), 2);
        assert_eq!(*q.bottom(), 3);
        q.popbottom();
        assert!(q.empty());
    }

    #[test]
    fn expand_preserves_and_zeroes_left() {
        // After expand the leftmost element is at reference 0 (C++ contract).
        let mut q: CircularQueue<int4> = CircularQueue::new(4);
        // Fill so that left wraps off zero: push 3, pop bottom 1, push more.
        for v in [1, 2, 3] {
            let s = q.push();
            *q.get_ref_mut(s) = v;
        }
        q.popbottom(); // drop 1, left advances
        let s = q.push();
        *q.get_ref_mut(s) = 4; // queue now holds 2,3,4 wrapped
        let before_bottom = *q.bottom();
        q.expand(4);
        assert_eq!(q.get_max(), 8);
        assert_eq!(q.bottomref(), 0, "leftmost ends at reference 0");
        assert_eq!(*q.bottom(), before_bottom);
        // Contents preserved in order 2,3,4.
        assert_eq!(*q.get_ref(0), 2);
        assert_eq!(*q.get_ref(1), 3);
        assert_eq!(*q.get_ref(2), 4);
    }

    #[test]
    fn set_max_empties() {
        let mut q: CircularQueue<int4> = CircularQueue::new(4);
        let s = q.push();
        *q.get_ref_mut(s) = 9;
        q.set_max(16);
        assert_eq!(q.get_max(), 16);
        assert!(q.empty());
    }
}

// ===========================================================================
// TokenSplit
// ===========================================================================

mod tokensplit {
    use super::*;

    #[test]
    fn content_token_size_is_byte_len() {
        let mut t = TokenSplit::default();
        t.tag_variable("hello", SyntaxHighlight::VarColor, &MarkupRef::none());
        assert_eq!(t.get_size(), 5);
        assert_eq!(t.get_class(), PrintClass::TokenString);
        assert_eq!(t.get_tag(), TagType::VariT);
    }

    #[test]
    fn open_paren_size_is_one() {
        let mut t = TokenSplit::default();
        t.open_paren("(", 7);
        assert_eq!(t.get_size(), 1);
        assert_eq!(t.get_class(), PrintClass::TokenString);
        assert_eq!(t.get_tag(), TagType::OparT);
        assert_eq!(t.get_count(), 7);
    }

    #[test]
    fn tagline_is_huge_break() {
        let mut t = TokenSplit::default();
        t.tag_line();
        assert_eq!(t.get_class(), PrintClass::TokenBreak);
        assert_eq!(t.get_num_spaces(), 999999);
        assert_eq!(t.get_indent_bump(), 0);
        assert_eq!(t.get_tag(), TagType::BumpT);
    }

    #[test]
    fn tagline_indent_carries_bump() {
        let mut t = TokenSplit::default();
        t.tag_line_indent(4);
        assert_eq!(t.get_class(), PrintClass::TokenBreak);
        assert_eq!(t.get_num_spaces(), 999999);
        assert_eq!(t.get_indent_bump(), 4);
        assert_eq!(t.get_tag(), TagType::LineT);
    }

    #[test]
    fn spaces_token_classes_as_break() {
        let mut t = TokenSplit::default();
        t.spaces(3, 2);
        assert_eq!(t.get_class(), PrintClass::TokenBreak);
        assert_eq!(t.get_num_spaces(), 3);
        assert_eq!(t.get_indent_bump(), 2);
        assert_eq!(t.get_tag(), TagType::SpacT);
    }

    #[test]
    fn begin_end_pairs_have_distinct_ids() {
        let mut a = TokenSplit::default();
        let mut b = TokenSplit::default();
        let ia = a.begin_document();
        let ib = b.begin_function();
        assert_ne!(ia, ib, "countbase is monotonic");
        assert_eq!(a.get_class(), PrintClass::Begin);
        assert_eq!(b.get_class(), PrintClass::Begin);
    }

    #[test]
    fn indent_tokens_classify_correctly() {
        let mut s = TokenSplit::default();
        let id = s.start_indent(2);
        assert_eq!(s.get_class(), PrintClass::BeginIndent);
        let mut e = TokenSplit::default();
        e.stop_indent(id);
        assert_eq!(e.get_class(), PrintClass::EndIndent);
    }

    #[test]
    fn comment_block_delimiters() {
        let mut s = TokenSplit::default();
        s.start_comment();
        assert_eq!(s.get_class(), PrintClass::BeginComment);
        let mut e = TokenSplit::default();
        e.stop_comment(0);
        assert_eq!(e.get_class(), PrintClass::EndComment);
    }

    #[test]
    fn line_and_bump_throw_on_print() {
        // line_t / bump_t must never reach a low-level emitter.
        let mut e = EmitNoMarkup::new();
        let mut t = TokenSplit::default();
        t.tag_line();
        let err = t.print_to(&mut e).unwrap_err();
        assert!(err.is_lowlevel());
    }
}

// ===========================================================================
// EmitNoMarkup (low-level back-end, the byte-exact path)
// ===========================================================================

mod nomarkup {
    use super::*;

    #[test]
    fn dumps_tokens_verbatim() {
        let mut e = EmitNoMarkup::new();
        e.tag_variable("a", SyntaxHighlight::VarColor, &MarkupRef::none());
        e.print(" = ", SyntaxHighlight::NoColor);
        e.tag_variable("b", SyntaxHighlight::VarColor, &MarkupRef::none());
        e.print(";", SyntaxHighlight::NoColor);
        assert_eq!(e.output(), "a = b;");
    }

    #[test]
    fn tag_line_emits_newline_and_indent() {
        let mut e = EmitNoMarkup::new();
        let id = e.start_indent(); // indentincrement default 2
        let _ = id;
        let id2 = e.start_indent(); // now 4
        let _ = id2;
        e.tag_line();
        assert_eq!(e.output(), "\n    ");
    }

    #[test]
    fn tag_line_indent_overrides() {
        let mut e = EmitNoMarkup::new();
        e.tag_line_indent(3);
        assert_eq!(e.output(), "\n   ");
    }

    #[test]
    fn paren_tracks_level() {
        let mut e = EmitNoMarkup::new();
        assert_eq!(e.get_paren_level(), 0);
        e.open_paren("(", 0);
        assert_eq!(e.get_paren_level(), 1);
        e.close_paren(")", 0);
        assert_eq!(e.get_paren_level(), 0);
        assert_eq!(e.output(), "()");
    }

    #[test]
    fn spaces_helper_uses_table_and_overflow() {
        let mut e = EmitNoMarkup::new();
        e.spaces(3, 0);
        assert_eq!(e.output(), "   ");
        let mut e2 = EmitNoMarkup::new();
        e2.spaces(12, 0); // > 10 path
        assert_eq!(e2.output(), "            ");
    }

    #[test]
    fn does_not_emit_markup() {
        assert!(!EmitNoMarkup::new().emits_markup());
    }
}

// ===========================================================================
// EmitPrettyPrint — the line-breaking engine (golden vs. real Ghidra)
// ===========================================================================

mod engine {
    use super::*;

    // Case A: a short statement that fits — no breaks inserted.
    fn case_a(e: &mut EmitPrettyPrint) {
        let none = MarkupRef::none();
        e.begin_document();
        e.begin_statement(&none);
        e.tag_variable("a", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("=", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("b", SyntaxHighlight::VarColor, &none);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    }

    // Case B: a long sum, broken differently by width.
    fn case_b(e: &mut EmitPrettyPrint) {
        let none = MarkupRef::none();
        e.begin_document();
        e.begin_statement(&none);
        e.tag_variable("result", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("=", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("alpha", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("+", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("beta", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("+", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("gamma", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("+", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("delta", SyntaxHighlight::VarColor, &none);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    }

    // Case C: an if-block with parentheses group and a nested indent level.
    fn case_c(e: &mut EmitPrettyPrint) {
        let none = MarkupRef::none();
        e.begin_document();
        e.begin_block(0);
        e.print("if", SyntaxHighlight::NoColor);
        e.spaces(1, 0);
        let id = e.open_paren("(", 0);
        e.tag_variable("x", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("<", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("y", SyntaxHighlight::VarColor, &none);
        e.close_paren(")", id);
        e.spaces(1, 0);
        e.print("{", SyntaxHighlight::NoColor);
        let ind = e.start_indent();
        e.tag_line();
        e.begin_statement(&none);
        e.tag_variable("z", SyntaxHighlight::VarColor, &none);
        e.spaces(1, 0);
        e.tag_op("=", SyntaxHighlight::NoColor, &none);
        e.spaces(1, 0);
        e.tag_variable("w", SyntaxHighlight::VarColor, &none);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.stop_indent(ind);
        e.tag_line();
        e.print("}", SyntaxHighlight::NoColor);
        e.end_block(0);
        e.end_document(0);
    }

    // Case D: an absolute-indent line break (`tagLine(indent)`).
    fn case_d(e: &mut EmitPrettyPrint) {
        let none = MarkupRef::none();
        e.begin_document();
        e.begin_statement(&none);
        e.tag_variable("foo", SyntaxHighlight::VarColor, &none);
        e.tag_line_indent(4);
        e.tag_variable("bar", SyntaxHighlight::VarColor, &none);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    }

    #[test]
    fn case_a_fits_at_100() {
        assert_eq!(run(100, case_a), "a = b;");
    }

    #[test]
    fn case_b_breaks_at_20() {
        // Golden from real Ghidra engine.
        assert_eq!(run(20, case_b), "result = alpha +\nbeta + gamma + delta\n;");
    }

    #[test]
    fn case_b_breaks_once_at_25() {
        // Golden from real Ghidra engine: a single break after the second '+'.
        assert_eq!(run(25, case_b), "result = alpha + beta +\ngamma + delta;");
    }

    #[test]
    fn case_b_fits_at_100() {
        assert_eq!(run(100, case_b), "result = alpha + beta + gamma + delta;");
    }

    #[test]
    fn case_c_indent_block_at_20() {
        assert_eq!(run(20, case_c), "if (x < y) {\n  z = w;\n}");
    }

    #[test]
    fn case_c_indent_block_at_30() {
        assert_eq!(run(30, case_c), "if (x < y) {\n  z = w;\n}");
    }

    #[test]
    fn case_c_indent_block_at_100() {
        assert_eq!(run(100, case_c), "if (x < y) {\n  z = w;\n}");
    }

    #[test]
    fn case_d_absolute_indent_break() {
        assert_eq!(run(100, case_d), "foo\n    bar;");
    }

    #[test]
    fn flush_missing_group_end_errors() {
        // An unbalanced begin (no matching end) leaves a negative-size group
        // at the bottom of the queue; flush must reject it.
        let mut pp = EmitPrettyPrint::new();
        pp.set_max_line_size_pretty(100).unwrap();
        pp.try_begin_document().unwrap();
        // No end_document: the begin token's size stays negative.
        let err = pp.try_flush().unwrap_err();
        assert!(err.is_lowlevel());
    }

    #[test]
    fn bad_max_line_size_rejected() {
        let mut pp = EmitPrettyPrint::new();
        assert!(pp.set_max_line_size_pretty(19).is_err());
        assert!(pp.set_max_line_size_pretty(10001).is_err());
        assert!(pp.set_max_line_size_pretty(20).is_ok());
        assert!(pp.set_max_line_size_pretty(10000).is_ok());
    }

    #[test]
    fn default_max_line_size_is_100() {
        let pp = EmitPrettyPrint::new();
        assert_eq!(pp.get_max_line_size(), 100);
    }

    #[test]
    fn clear_resets_engine_state_not_the_stream() {
        // C++ `clear()` resets the pretty-printer's queues/indent/paren state
        // but does NOT erase bytes already sent to the bound stream (the stream
        // is external).  So after clear the prior output persists and fresh
        // emission appends; a caller wanting a clean buffer rebinds the stream.
        let mut pp = EmitPrettyPrint::new();
        pp.set_max_line_size_pretty(100).unwrap();
        case_a(&mut pp);
        pp.try_flush().unwrap();
        assert_eq!(pp.output(), "a = b;");
        assert_eq!(pp.get_paren_level(), 0, "balanced parens after flush");

        pp.clear_pretty();
        // Engine state is clean and the emitter is reusable; the buffer retains
        // the earlier output (faithful to the external-stream model).
        case_a(&mut pp);
        pp.try_flush().unwrap();
        assert_eq!(pp.output(), "a = b;a = b;");
    }

    #[test]
    fn comment_token_emits_text_no_markup() {
        // tagComment through the no-markup back-end emits just the text; the
        // address space is ignored (as in EmitNoMarkup::tagComment).
        let spc = Rc::new(ConstantSpace::new());
        let out = run(100, |e| {
            let none = MarkupRef::none();
            e.begin_document();
            e.begin_statement(&none);
            e.tag_comment("// note", SyntaxHighlight::CommentColor, &spc, 0x1000);
            e.end_statement(0);
            e.end_document(0);
        });
        assert_eq!(out, "// note");
    }

    #[test]
    fn emits_markup_follows_lowlevel() {
        let mut pp = EmitPrettyPrint::new();
        assert!(!pp.emits_markup());
        pp.set_markup_pretty(true);
        assert!(pp.emits_markup());
        pp.set_markup_pretty(false);
        assert!(!pp.emits_markup());
    }
}

// ===========================================================================
// EmitMarkup — the packed-encoding back-end
// ===========================================================================

mod markup {
    use super::*;

    /// Packed format header markers (from kuna-base `marshal`).
    const HEADER_MASK: u8 = 0xc0;
    const ELEMENT_START: u8 = 0x40;
    const ELEMENT_END: u8 = 0x80;

    #[test]
    fn emits_markup() {
        assert!(EmitMarkup::new().emits_markup());
    }

    #[test]
    fn variable_token_writes_packed_element_with_content() {
        // EmitMarkup produces a packed <variable> element (an ELEMENT_START
        // header) carrying the content bytes — distinct from the raw text the
        // no-markup back-end produces.
        let mut e = EmitMarkup::new();
        let markup = MarkupRef { varref: Some(42), ..MarkupRef::none() };
        e.tag_variable("v", SyntaxHighlight::VarColor, &markup);
        let bytes = e.take_output();
        assert!(!bytes.is_empty());
        assert_eq!(
            bytes[0] & HEADER_MASK,
            ELEMENT_START,
            "leading byte is an element-start header"
        );
        // The string content "v" must appear verbatim in the byte stream.
        assert!(bytes.windows(1).any(|w| w == b"v"));
    }

    #[test]
    fn begin_end_block_bracket_with_start_then_end_markers() {
        // A begin/end pair appends an ELEMENT_START then a later ELEMENT_END
        // marker — the stateless-PackedEncode property the port relies on.
        let mut e = EmitMarkup::new();
        e.begin_block(7);
        let after_begin = e.output().len();
        e.end_block(0);
        let bytes = e.take_output();
        assert!(after_begin > 0);
        assert_eq!(bytes[0] & HEADER_MASK, ELEMENT_START);
        // The first byte of the end_block contribution is an element-end header.
        assert_eq!(bytes[after_begin] & HEADER_MASK, ELEMENT_END);
    }

    #[test]
    fn no_color_omits_color_attribute() {
        // With no_color the color attribute is not written; the element is just
        // start + content + end.  A colored token writes strictly more bytes.
        let mut plain = EmitMarkup::new();
        plain.tag_op("+", SyntaxHighlight::NoColor, &MarkupRef::none());
        let plain_len = plain.take_output().len();

        let mut colored = EmitMarkup::new();
        colored.tag_op("+", SyntaxHighlight::KeywordColor, &MarkupRef::none());
        let colored_len = colored.take_output().len();

        assert!(colored_len > plain_len, "color attribute adds bytes");
    }
}
