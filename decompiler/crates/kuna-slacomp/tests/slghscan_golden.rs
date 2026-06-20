//! WS1 golden-token tests for the SLEIGH lexer (`slghscan.rs`).
//!
//! Each `tests/golden/<name>.txt` is a golden token stream produced by an
//! instrumented copy of the C++ flex scanner (`slghscan.l`) running over
//! `tests/golden/snips/<name>.slaspec` (the harness lives in a throwaway /tmp
//! copy of `decompiler/cpp/**`; the vendored tree stays byte-untouched -- see
//! `docs/rust-port/sleigh-compiler/ws1-lexer.md`).  This test runs the Rust
//! [`SleighScanner`] over the SAME snippet and asserts a token-for-token match.
//!
//! The golden format is one token per line:
//!   `NAME`                      -- a keyword/operator/symbol token (no value)
//!   `NAME str=<bytes>`          -- STRING / SYMBOLSTRING (carried identifier)
//!   `NAME i=<u64>`              -- INTEGER
//!   `NAME big=<i64>`            -- INTB
//!   `CHAR ch=<u8>`              -- a print-state CHAR
//!   `CHAR1 <code>`             -- a single-character literal token (its ASCII)
//!
//! The symbol table the C++ harness ran against was empty, so every identifier
//! resolved through `find_symbol` to STRING; the Rust host stub mirrors that by
//! returning `None` from `find_symbol_kind`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use kuna_slacomp::slghscan::{
    ScannerHost, SleighScanner, SymbolTokenKind, Token, TokenKind, TokenValue,
};

/// A test `ScannerHost`: an empty symbol table (so identifiers -> STRING),
/// preprocessor values in a map, and `@include` resolved from a base directory.
struct TestHost {
    preproc: BTreeMap<Vec<u8>, Vec<u8>>,
    base_dir: PathBuf,
}

impl TestHost {
    fn new(base_dir: PathBuf) -> TestHost {
        TestHost {
            preproc: BTreeMap::new(),
            base_dir,
        }
    }
}

impl ScannerHost for TestHost {
    fn next_line(&mut self) {}
    fn calc_context_layout(&mut self) {}
    fn read_include(&mut self, fname: &[u8]) -> Option<Vec<u8>> {
        let name = std::str::from_utf8(fname).ok()?;
        std::fs::read(self.base_dir.join(name)).ok()
    }
    fn parse_file_finished(&mut self) {}
    fn parse_preproc_macro(&mut self) {}
    fn get_preproc_value(&self, name: &[u8]) -> Option<Vec<u8>> {
        self.preproc.get(name).cloned()
    }
    fn set_preproc_value(&mut self, name: &[u8], value: &[u8]) {
        self.preproc.insert(name.to_vec(), value.to_vec());
    }
    fn undefine_preproc_value(&mut self, name: &[u8]) -> bool {
        self.preproc.remove(name).is_some()
    }
    fn find_symbol_kind(&self, _name: &[u8]) -> Option<SymbolTokenKind> {
        None // empty symbol table -> every identifier becomes STRING
    }
}

/// The bison symbolic name for a `TokenKind` (matches the C++ harness's
/// `tokname()`), or `None` for `Char1`/`Eof` which the harness prints specially.
fn token_name(kind: TokenKind) -> Option<&'static str> {
    use TokenKind::*;
    Some(match kind {
        OpBoolOr => "OP_BOOL_OR",
        OpBoolAnd => "OP_BOOL_AND",
        OpBoolXor => "OP_BOOL_XOR",
        OpOr => "OP_OR",
        OpXor => "OP_XOR",
        OpAnd => "OP_AND",
        OpEqual => "OP_EQUAL",
        OpNotEqual => "OP_NOTEQUAL",
        OpFEqual => "OP_FEQUAL",
        OpFNotEqual => "OP_FNOTEQUAL",
        OpGreatEqual => "OP_GREATEQUAL",
        OpLessEqual => "OP_LESSEQUAL",
        OpSless => "OP_SLESS",
        OpSgreatEqual => "OP_SGREATEQUAL",
        OpSlessEqual => "OP_SLESSEQUAL",
        OpSgreat => "OP_SGREAT",
        OpFless => "OP_FLESS",
        OpFgreat => "OP_FGREAT",
        OpFlessEqual => "OP_FLESSEQUAL",
        OpFgreatEqual => "OP_FGREATEQUAL",
        OpLeft => "OP_LEFT",
        OpRight => "OP_RIGHT",
        OpSright => "OP_SRIGHT",
        OpFadd => "OP_FADD",
        OpFsub => "OP_FSUB",
        OpSdiv => "OP_SDIV",
        OpSrem => "OP_SREM",
        OpFmult => "OP_FMULT",
        OpFdiv => "OP_FDIV",
        OpZext => "OP_ZEXT",
        OpCarry => "OP_CARRY",
        OpBorrow => "OP_BORROW",
        OpSext => "OP_SEXT",
        OpScarry => "OP_SCARRY",
        OpSborrow => "OP_SBORROW",
        OpNan => "OP_NAN",
        OpAbs => "OP_ABS",
        OpSqrt => "OP_SQRT",
        OpCeil => "OP_CEIL",
        OpFloor => "OP_FLOOR",
        OpRound => "OP_ROUND",
        OpInt2Float => "OP_INT2FLOAT",
        OpFloat2Float => "OP_FLOAT2FLOAT",
        OpTrunc => "OP_TRUNC",
        OpCpoolref => "OP_CPOOLREF",
        OpNew => "OP_NEW",
        OpPopcount => "OP_POPCOUNT",
        OpLzcount => "OP_LZCOUNT",
        OpUnimpl => "OP_UNIMPL",
        BadInteger => "BADINTEGER",
        GotoKey => "GOTO_KEY",
        CallKey => "CALL_KEY",
        ReturnKey => "RETURN_KEY",
        IfKey => "IF_KEY",
        DefineKey => "DEFINE_KEY",
        AttachKey => "ATTACH_KEY",
        MacroKey => "MACRO_KEY",
        SpaceKey => "SPACE_KEY",
        TypeKey => "TYPE_KEY",
        RamKey => "RAM_KEY",
        DefaultKey => "DEFAULT_KEY",
        RegisterKey => "REGISTER_KEY",
        EndianKey => "ENDIAN_KEY",
        WithKey => "WITH_KEY",
        AlignKey => "ALIGN_KEY",
        TokenKey => "TOKEN_KEY",
        SignedKey => "SIGNED_KEY",
        NoflowKey => "NOFLOW_KEY",
        HexKey => "HEX_KEY",
        DecKey => "DEC_KEY",
        BigKey => "BIG_KEY",
        LittleKey => "LITTLE_KEY",
        SizeKey => "SIZE_KEY",
        WordsizeKey => "WORDSIZE_KEY",
        OffsetKey => "OFFSET_KEY",
        NamesKey => "NAMES_KEY",
        ValuesKey => "VALUES_KEY",
        VariablesKey => "VARIABLES_KEY",
        PcodeopKey => "PCODEOP_KEY",
        IsKey => "IS_KEY",
        LocalKey => "LOCAL_KEY",
        DelayslotKey => "DELAYSLOT_KEY",
        CrossbuildKey => "CROSSBUILD_KEY",
        ExportKey => "EXPORT_KEY",
        BuildKey => "BUILD_KEY",
        ContextKey => "CONTEXT_KEY",
        EllipsisKey => "ELLIPSIS_KEY",
        GlobalsetKey => "GLOBALSET_KEY",
        BitrangeKey => "BITRANGE_KEY",
        Char => "CHAR",
        Integer => "INTEGER",
        Intb => "INTB",
        String => "STRING",
        SymbolString => "SYMBOLSTRING",
        SpaceSym => "SPACESYM",
        SectionSym => "SECTIONSYM",
        TokenSym => "TOKENSYM",
        UseropSym => "USEROPSYM",
        ValueSym => "VALUESYM",
        ValueMapSym => "VALUEMAPSYM",
        ContextSym => "CONTEXTSYM",
        NameSym => "NAMESYM",
        VarSym => "VARSYM",
        BitSym => "BITSYM",
        SpecSym => "SPECSYM",
        VarListSym => "VARLISTSYM",
        OperandSym => "OPERANDSYM",
        JumpSym => "JUMPSYM",
        MacroSym => "MACROSYM",
        LabelSym => "LABELSYM",
        SubtableSym => "SUBTABLESYM",
        Char1(_) | Eof => return None,
    })
}

/// Render a token in the golden line format the C++ harness emits.
///
/// The `print`-state `^` rule returns the literal char token `'^'` (CHAR1 94)
/// in C++ (`return '^'`), but flex's `^` rule for `<print>` returns it as the
/// single-character token, so the harness prints `CHAR1 94`.  We map our
/// `Char1(b'^')` (with a Char value) the same way.
fn render(tok: &Token) -> String {
    match tok.kind {
        TokenKind::Char1(c) => format!("CHAR1 {}", c),
        TokenKind::Eof => "EOF".to_string(),
        _ => {
            let name = token_name(tok.kind).expect("named token");
            match (&tok.kind, &tok.value) {
                (TokenKind::Char, TokenValue::Char(c)) => format!("{name} ch={}", *c as i32),
                (TokenKind::Integer, TokenValue::Integer(i)) => format!("{name} i={i}"),
                (TokenKind::Intb, TokenValue::Intb(b)) => format!("{name} big={b}"),
                (TokenKind::String | TokenKind::SymbolString, TokenValue::Str(s)) => {
                    format!("{name} str={}", String::from_utf8_lossy(s))
                }
                // *SYM tokens carry SymbolName, but with an empty table they
                // never occur here; render their name only (no harness value).
                _ => name.to_string(),
            }
        }
    }
}

/// Run the Rust lexer over `snip` (a path to a `.slaspec`) and produce the
/// golden-format token stream (one rendered token per line, EOF excluded).
fn lex_to_golden(snip: &Path) -> String {
    let contents = std::fs::read(snip).expect("read snippet");
    let base_dir = snip.parent().unwrap().to_path_buf();
    let mut host = TestHost::new(base_dir);
    let mut scanner = SleighScanner::new();
    scanner.open(contents);
    let mut out = String::new();
    loop {
        let tok = scanner.next_token(&mut host).expect("lex");
        if tok.kind == TokenKind::Eof {
            break;
        }
        out.push_str(&render(&tok));
        out.push('\n');
    }
    out
}

/// Assert the Rust lexer reproduces the golden stream for `name`
/// (`tests/golden/<name>.txt` vs `tests/golden/snips/<name>.slaspec`).
fn check_golden(name: &str) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden");
    let snip = dir.join("snips").join(format!("{name}.slaspec"));
    let gold_path = dir.join(format!("{name}.txt"));
    let gold = std::fs::read_to_string(&gold_path)
        .unwrap_or_else(|_| panic!("missing golden {}", gold_path.display()));
    let got = lex_to_golden(&snip);
    // Compare line-by-line for a readable first-divergence message.
    let gold_lines: Vec<&str> = gold.lines().collect();
    let got_lines: Vec<&str> = got.lines().collect();
    for (i, (g, r)) in gold_lines.iter().zip(got_lines.iter()).enumerate() {
        assert_eq!(
            g, r,
            "[{name}] token {i} mismatch:\n  golden: {g}\n  rust:   {r}"
        );
    }
    assert_eq!(
        gold_lines.len(),
        got_lines.len(),
        "[{name}] token count mismatch: golden={} rust={}",
        gold_lines.len(),
        got_lines.len()
    );
}

// --- the simplest real spec, end-to-end token-for-token (WS7 first target) ---

#[test]
fn golden_data_le_64() {
    check_golden("data_le_64");
}

#[test]
fn golden_data_be_64() {
    check_golden("data_be_64");
}

// --- targeted snippets: one per start-condition / keyword / literal form ---

#[test]
fn golden_macroblock() {
    check_golden("macro");
}

#[test]
fn golden_attach_and_with() {
    check_golden("attach_with");
}

#[test]
fn golden_print_state() {
    check_golden("printstate");
}

#[test]
fn golden_numbers() {
    check_golden("numbers");
}

#[test]
fn golden_pattern_operators() {
    check_golden("pattern_ops");
}

#[test]
fn golden_preprocessor() {
    check_golden("preproc");
}

#[test]
fn golden_misc_keywords() {
    check_golden("misc");
}

#[test]
fn golden_sem_operators() {
    check_golden("sem_ops");
}

// --- longest-match edge cases (the flex subtleties) ---

#[test]
fn golden_badinteger_partial_hex() {
    // `0xZZ` -> INTEGER 0 then identifier `xZZ` (hex rule needs a hex digit).
    check_golden("badint");
}

#[test]
fn golden_overflow_is_badinteger() {
    // an oversized hex literal -> BADINTEGER (scan_number's stoul throws).
    check_golden("overflow");
}

#[test]
fn golden_empty_hex_prefix() {
    // `0x;` -> INTEGER 0 then `x` identifier.
    check_golden("emptyhex");
}

#[test]
fn golden_at_sign_midline() {
    // an `@` not at column 0 is NOT a preprocessor directive (the `^` anchor).
    check_golden("atmid");
}
