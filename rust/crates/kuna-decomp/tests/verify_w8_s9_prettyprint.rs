//! Independent VERIFIER adversarial tests for item `w8-s9-prettyprint`
//! (`decompiler/cpp/prettyprint.{cc,hh}`).
//!
//! These target the hunt-list spots flagged as most fragile for this item.
//! Every expected value is re-derived from a **C++ oracle**, NOT from the Rust
//! port: a standalone harness that compiles the verbatim Ghidra
//! `EmitPrettyPrint`/`EmitNoMarkup`/`TokenSplit`/`circularqueue` engine
//! (no-markup back-end) and runs the same token streams.  The harness was
//! first validated against the porter's own golden cases (`case_b@20`,
//! `case_b@25`, `case_c@20`, `case_d@100`) which it reproduces byte-for-byte.
//!
//! Fragile spots covered:
//!   - `EmitPrettyPrint::overflow()` (prettyprint.cc:609) — the half-line
//!     indentstack rewrite + forced `tagLine` when a *single* token is wider
//!     than the whole line, and when an oversized token appears mid-line.
//!   - the `< 10` short-break heuristic (prettyprint.cc:705-710) — `(tok
//!     numspaces <= spaceremain) && (val - spaceremain < 10)` suppresses a
//!     break; the signed `val - spaceremain` comparison is the off-by-one /
//!     signedness hunt.
//!   - `EmitPrettyPrint::expand()` (prettyprint.cc:589) + the scanqueue
//!     reference adjustment `(ref(i)+max-left)%max` — driven by a single group
//!     far larger than the `3*maxlinesize` token queue, forcing one or more
//!     buffer expansions; the iteration-order / index-provenance hunt.
//!   - nested `startIndent`/`stopIndent` interacting with `overflow()`
//!     mutating `indentstack.back()` (the begin_indent `val = back() - bump`
//!     arithmetic).
//!   - `circularqueue::expand` contract: leftmost element ends at reference 0
//!     even when `left` has wrapped past `right` in the ring (prettyprint.hh
//!     :1032).
//!   - `flush()` rejecting an unbalanced (negative-size) group
//!     (prettyprint.cc:1233).

use std::rc::Rc;

use kuna_base::space::ConstantSpace;
use kuna_base::types::int4;
use kuna_decomp::prettyprint::{
    CircularQueue, Emit, EmitPrettyPrint, MarkupRef, SyntaxHighlight,
};

/// Drive a no-markup `EmitPrettyPrint` at `maxline` and return the text.
fn run(maxline: int4, build: impl FnOnce(&mut EmitPrettyPrint)) -> String {
    let mut pp = EmitPrettyPrint::new();
    pp.set_max_line_size_pretty(maxline).expect("max line size");
    build(&mut pp);
    pp.try_flush().expect("flush");
    pp.output().to_string()
}

const NONE: MarkupRef = MarkupRef {
    opref: None,
    varref: None,
    blockref: None,
    symref: None,
    return_varref: None,
    type_id: None,
    type_name: None,
};

/// ADV1 — a single content token wider than the entire line.
///
/// C++ oracle (`./harness`, width 20):
///   `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n;`
/// The oversized variable triggers `overflow()`, which rewrites the (single)
/// indentstack entry to `maxlinesize/2` and forces a `tagLine`.  The trailing
/// `;` lands on its own line.
#[test]
fn adv1_oversized_token_triggers_overflow() {
    let big = "a".repeat(30); // 30 > 20
    let out = run(20, |e| {
        e.begin_document();
        e.begin_statement(&NONE);
        e.tag_variable(&big, SyntaxHighlight::VarColor, &NONE);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    });
    assert_eq!(out, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n;");
}

/// ADV2 — a normal word, a space, then an oversized token mid-line.
///
/// C++ oracle (width 20): `foo \nbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n;`
/// Note the **trailing space** after `foo` before the break: the space token
/// is emitted (fits), then the oversized token overflows.  Reproducing the
/// exact trailing whitespace is the byte-exact contract the datatests match.
#[test]
fn adv2_overflow_mid_line_keeps_trailing_space() {
    let big = "b".repeat(28);
    let out = run(20, |e| {
        e.begin_document();
        e.begin_statement(&NONE);
        e.tag_variable("foo", SyntaxHighlight::VarColor, &NONE);
        e.spaces(1, 0);
        e.tag_variable(&big, SyntaxHighlight::VarColor, &NONE);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    });
    assert_eq!(out, "foo \nbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n;");
}

/// ADV3 — many short words; the `< 10` short-break heuristic keeps them all on
/// one line up to the width, then breaks.
///
/// C++ oracle (width 20): `aa bb cc dd ee ff gg\n;`
/// Seven 2-char words + six single spaces = 20 chars exactly fit; the `;`
/// breaks.  This exercises the `(numspaces <= spaceremain) && (val-spaceremain
/// < 10)` suppression on the tokenbreaks that *don't* break.
#[test]
fn adv3_short_break_heuristic_fills_to_width() {
    let out = run(20, |e| {
        e.begin_document();
        e.begin_statement(&NONE);
        for (i, w) in ["aa", "bb", "cc", "dd", "ee", "ff", "gg"].iter().enumerate() {
            if i != 0 {
                e.spaces(1, 0);
            }
            e.tag_variable(w, SyntaxHighlight::VarColor, &NONE);
        }
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    });
    assert_eq!(out, "aa bb cc dd ee ff gg\n;");
}

/// ADV4 — nested indent level with a long statement that overflows; `overflow`
/// rewrites `indentstack.back()` so subsequent breaks land at the half-line
/// indent (2 spaces here, since maxlinesize/2 = 12 but the indent of 2 is
/// already < 12 so it is bumped to 12... see oracle).
///
/// C++ oracle (width 24):
///   `{\n  verylongname =\n  anotherlongname +\n  third;\n}`
/// The indent inside the block is 2 (one `startIndent`).  Each over-long
/// sub-expression breaks but the relative-indent arithmetic keeps the
/// continuation at the 2-space block indent.
#[test]
fn adv4_nested_indent_with_overflow() {
    let out = run(24, |e| {
        e.begin_document();
        e.begin_block(0);
        e.print("{", SyntaxHighlight::NoColor);
        let ind = e.start_indent();
        e.tag_line();
        e.begin_statement(&NONE);
        e.tag_variable("verylongname", SyntaxHighlight::VarColor, &NONE);
        e.spaces(1, 0);
        e.tag_op("=", SyntaxHighlight::NoColor, &NONE);
        e.spaces(1, 0);
        e.tag_variable("anotherlongname", SyntaxHighlight::VarColor, &NONE);
        e.spaces(1, 0);
        e.tag_op("+", SyntaxHighlight::NoColor, &NONE);
        e.spaces(1, 0);
        e.tag_variable("third", SyntaxHighlight::VarColor, &NONE);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.stop_indent(ind);
        e.tag_line();
        e.print("}", SyntaxHighlight::NoColor);
        e.end_block(0);
        e.end_document(0);
    });
    assert_eq!(out, "{\n  verylongname =\n  anotherlongname +\n  third;\n}");
}

/// ADV5 — a single group with 120 one-char tokens at width 20: the token queue
/// (`3*20 = 60` slots) is far too small, forcing `EmitPrettyPrint::expand()`
/// (and the scanqueue reference adjustment) at least twice.
///
/// C++ oracle (width 20): six lines of 20 `x` (= 120) then `\n;`.
/// This is the queue-expansion / index-provenance stress test — if the
/// `(ref(i)+max-left)%max` reference fix-up were wrong the breaks would land
/// in the wrong places or panic.
#[test]
fn adv5_queue_expansion_long_group() {
    let out = run(20, |e| {
        e.begin_document();
        e.begin_statement(&NONE);
        let g = e.open_group();
        for _ in 0..120 {
            e.tag_variable("x", SyntaxHighlight::VarColor, &NONE);
        }
        e.close_group(g);
        e.print(";", SyntaxHighlight::NoColor);
        e.end_statement(0);
        e.end_document(0);
    });
    let line = "x".repeat(20);
    let expected = format!("{line}\n{line}\n{line}\n{line}\n{line}\n{line}\n;");
    assert_eq!(out, expected);
}

/// ADV6 — `circularqueue::expand` with a `left` index that has wrapped past
/// `right` in the ring.  C++ contract (prettyprint.hh:1032-1053): after expand
/// the leftmost element is at reference 0 and the contents are preserved in
/// queue order.  The Rust port copies `i=left; while i!=right { ...; i=(i+1)%max
/// }` then the rightmost — a wrap-aware copy that must visit the same elements.
#[test]
fn adv6_circularqueue_expand_wrapped_left() {
    // Build a size-4 queue whose live window wraps the ring boundary.
    let mut q: CircularQueue<int4> = CircularQueue::new(4);
    // Push 1,2,3 (right walks 1,2,3), pop bottom (left -> 2), push 4 (right
    // wraps to 0): live window is indices {2,3,0} holding 2,3,4 with left=2 >
    // right=0 — the wrapped case.
    for v in [1, 2, 3] {
        let s = q.push();
        *q.get_ref_mut(s) = v;
    }
    q.popbottom();
    let s = q.push();
    *q.get_ref_mut(s) = 4;
    assert_eq!(*q.bottom(), 2, "bottom is 2 before expand");
    q.expand(4);
    assert_eq!(q.get_max(), 8);
    assert_eq!(q.bottomref(), 0, "leftmost lands at reference 0");
    assert_eq!(*q.get_ref(0), 2);
    assert_eq!(*q.get_ref(1), 3);
    assert_eq!(*q.get_ref(2), 4);
    assert_eq!(*q.bottom(), 2, "contents preserved in order");
}

/// ADV7 — `flush()` on an unbalanced begin (no matching end) must reject: the
/// begin token keeps a negative size and `flush` (prettyprint.cc:1233) throws
/// "Cannot flush pretty printer. Missing group end".
#[test]
fn adv7_flush_missing_group_end_rejects() {
    let mut pp = EmitPrettyPrint::new();
    pp.set_max_line_size_pretty(100).unwrap();
    pp.try_begin_document().unwrap();
    pp.try_begin_statement(&NONE).unwrap(); // opened, never ended
    let err = pp.try_flush().unwrap_err();
    assert!(err.is_lowlevel(), "unbalanced group => LowlevelError");
}

/// ADV8 — comment fill on a forced break.  With `setCommentFill("// ")` and a
/// comment block, a forced break inserts the fill prefix.  C++ oracle (width
/// 20, `overflow`/`tokenbreak` comment-fill arms, prettyprint.cc:630-633,
/// 715-718): the second line is prefixed with the fill.
#[test]
fn adv8_comment_fill_on_forced_break() {
    let spc = Rc::new(ConstantSpace::new());
    let mut pp = EmitPrettyPrint::new();
    pp.set_max_line_size_pretty(20).unwrap();
    pp.set_comment_fill("// ");
    pp.begin_document();
    pp.begin_statement(&NONE);
    pp.start_comment();
    pp.tag_comment("aaaa", SyntaxHighlight::CommentColor, &spc, 0);
    pp.spaces(1, 0);
    pp.tag_comment("bbbb", SyntaxHighlight::CommentColor, &spc, 0);
    pp.spaces(1, 0);
    pp.tag_comment("cccc", SyntaxHighlight::CommentColor, &spc, 0);
    pp.spaces(1, 0);
    pp.tag_comment("dddd", SyntaxHighlight::CommentColor, &spc, 0);
    pp.spaces(1, 0);
    pp.tag_comment("eeee", SyntaxHighlight::CommentColor, &spc, 0);
    pp.stop_comment(0);
    pp.end_statement(0);
    pp.end_document(0);
    pp.try_flush().unwrap();
    // Compared against the C++ oracle output captured for this exact stream.
    let out = pp.output().to_string();
    assert_eq!(out, COMMENT_FILL_ORACLE);
}

/// Oracle output for ADV8, captured from the verbatim C++ engine (see the
/// module header).  Pinned as a constant so an accidental edit is visible.
const COMMENT_FILL_ORACLE: &str = "aaaa bbbb cccc dddd\n// eeee";
