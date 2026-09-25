use super::*;

fn raw(line: usize, col: usize, text: &str, kind: TokenKind) -> EmitToken {
    EmitToken {
        line,
        col,
        text: text.to_string(),
        kind,
        color: SyntaxHighlight::NoColor,
        opref: None,
        varref: None,
        at: None,
        type_name: None,
        in_var_decl: false,
        in_return_type: false,
        in_proto: false,
    }
}

fn token(line: usize, col: usize, text: &str) -> CodeToken {
    CodeToken {
        line,
        col,
        len: text.encode_utf16().count(),
        kind: TokenKind::Syntax,
        color: SyntaxHighlight::NoColor,
        text: text.to_string(),
        address: None,
        callee: None,
        var: None,
        decl: None,
        type_text: None,
    }
}

#[test]
fn verify_accepts_an_exact_cover_with_utf16_columns() {
    let code = "int f(void)\n{\n  s = \"\u{1f600}\"; x = 1;\n}";
    let tokens = vec![
        token(1, 0, "int"),
        token(1, 4, "f"),
        token(1, 5, "("),
        token(1, 6, "void"),
        token(1, 10, ")"),
        token(2, 0, "{"),
        token(3, 2, "s"),
        token(3, 4, "="),
        token(3, 6, "\"\u{1f600}\";"),
        token(3, 12, "x"),
        token(3, 14, "="),
        token(3, 16, "1;"),
        token(4, 0, "}"),
    ];
    assert_eq!(verify(&tokens, code), Ok(()));
}

#[test]
fn verify_rejects_drift_gaps_overlap_and_order() {
    let code = "a = b;";
    let good = vec![token(1, 0, "a"), token(1, 2, "="), token(1, 4, "b;")];
    assert_eq!(verify(&good, code), Ok(()));
    let drift = vec![token(1, 0, "a"), token(1, 3, "="), token(1, 4, "b;")];
    assert!(verify(&drift, code).is_err());
    let uncovered = vec![token(1, 0, "a"), token(1, 2, "=")];
    assert!(verify(&uncovered, code).unwrap_err().contains("not covered"));
    let overlap = vec![token(1, 0, "a ="), token(1, 2, "="), token(1, 4, "b;")];
    assert!(verify(&overlap, code).is_err());
    let order = vec![token(1, 2, "="), token(1, 0, "a"), token(1, 4, "b;")];
    assert!(verify(&order, code).is_err());
    let out_of_range = vec![token(2, 0, "a")];
    assert!(verify(&out_of_range, code).is_err());
}

#[test]
fn trimmed_leading_breaks_shift_lines() {
    assert_eq!(leading_breaks("\nint f"), 1);
    assert_eq!(leading_breaks("\n\n\nint f"), 3);
    assert_eq!(code_line(1, 1), Some(1));
    assert_eq!(code_line(3, 3), Some(1));
    assert_eq!(code_line(0, 1), None);
    assert_eq!(code_line(0, 0), Some(1));
}

#[test]
fn a_cookie_rewrite_moves_the_literal_onto_the_return_line() {
    let mut tokens = vec![
        raw(4, 2, "v1", TokenKind::Variable),
        raw(4, 5, "=", TokenKind::Op),
        EmitToken { opref: Some(9), color: SyntaxHighlight::ConstColor, ..raw(4, 7, "0", TokenKind::Value) },
        raw(4, 8, ";", TokenKind::Syntax),
        raw(5, 2, "check", TokenKind::FuncName),
        raw(5, 7, "(", TokenKind::Syntax),
        raw(5, 8, ");", TokenKind::Syntax),
        raw(6, 2, "return", TokenKind::Syntax),
        raw(6, 9, "v1", TokenKind::Variable),
        raw(6, 11, ";", TokenKind::Syntax),
        raw(7, 0, "}", TokenKind::Syntax),
    ];
    apply_cookie_rewrites(
        &mut tokens,
        &[CookieRewrite { assignment_line: 4, return_line: 6, indent: 2, literal: '0' }],
    );
    let placed: Vec<(usize, usize, &str)> =
        tokens.iter().map(|t| (t.line, t.col, t.text.as_str())).collect();
    assert_eq!(
        placed,
        vec![
            (5, 2, "check"),
            (5, 7, "("),
            (5, 8, ");"),
            (6, 2, "return"),
            (6, 9, "0"),
            (6, 10, ";"),
            (7, 0, "}"),
        ]
    );
    let literal = &tokens[4];
    assert_eq!((literal.kind, literal.opref), (TokenKind::Value, Some(9)));
}

/// The text rule rewrites only a return line reading exactly `return <v>;`, so
/// a rewrite record whose return line carries anything else leaves every token
/// where it was rather than deleting what it cannot rebuild.
#[test]
fn a_cookie_rewrite_leaves_a_return_line_it_does_not_recognise() {
    let before = vec![
        raw(4, 2, "v1", TokenKind::Variable),
        raw(4, 5, "=", TokenKind::Op),
        raw(4, 7, "0", TokenKind::Value),
        raw(4, 8, ";", TokenKind::Syntax),
        raw(6, 2, "return", TokenKind::Syntax),
        raw(6, 9, "v1", TokenKind::Variable),
        raw(6, 11, ";", TokenKind::Syntax),
        raw(6, 13, "// rax", TokenKind::Comment),
    ];
    let mut tokens = before.clone();
    apply_cookie_rewrites(
        &mut tokens,
        &[CookieRewrite { assignment_line: 4, return_line: 6, indent: 2, literal: '0' }],
    );
    assert_eq!(tokens, before);
}
