//! Gate for `kuna disassemble` — the instruction listing an RE agent falls back
//! to when decompilation gives it nothing to read.
//!
//! Two layers, because the command lands before the integrator wires its
//! dispatch arm in `main.rs`:
//!
//! * **In-process** — the module is pulled in by `#[path]` (with the three
//!   crate modules it uses) and driven through its own `parse_args` + `render`,
//!   so the whole command *except* the dispatch arm is under test from the day
//!   the file lands. `render` returns the listing instead of writing it, which
//!   is also what lets these tests assert on exact bytes and columns.
//! * **End to end** — the same invocations through the built `kuna` binary.
//!   Until `main.rs` routes `"disassemble"`, those are a visible skip (the
//!   `docs_cli.rs` precedent), never a false green.
//!
//! The ground truth is `objdump -d` on the vendored non-stripped `fauxware`:
//! `main` @ `0x40071d` opens `55` / `4889e5` / `4883ec40` and calls `puts@plt`
//! at `0x400739`. Those literals are in here on purpose — a listing that agrees
//! with itself but not with the machine is the failure mode this command exists
//! to rule out.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). When it is absent no architecture can be built; the tests
//! print that and return early.

use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
#[path = "../src/jsonfmt.rs"]
mod jsonfmt;
#[allow(dead_code)]
#[path = "../src/output.rs"]
mod output;
#[allow(dead_code)]
#[path = "../src/paths.rs"]
mod paths;
#[allow(dead_code)]
#[path = "../src/decompile.rs"]
mod decompile;
#[allow(dead_code)]
#[path = "../src/decompile_all.rs"]
mod decompile_all;
#[allow(dead_code)]
#[path = "../src/disassemble.rs"]
mod disassemble;

use jsonfmt::Json;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

/// The vendored non-stripped x86-64 `fauxware`: named functions and a `.rodata`
/// the `strings` pass names, so both the name path and the raw-address path have
/// ground truth.
fn fauxware() -> String {
    fixture("fauxware")
}

/// The checked-in stripped x86-64 openssh `ssh-sk-helper` (`tests/hang-repro/`):
/// the only fixture in the tree with functions big enough to probe the
/// derived-length cap against.
fn hang_repro() -> String {
    repo_root().join("tests/hang-repro/ssh-sk-helper").to_str().unwrap().to_string()
}

/// `true` when a failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(message: &str) -> bool {
    message.contains("could not build an architecture")
        || message.contains("SLEIGH")
        || message.contains("Could not discover")
}

/// Drive the command in-process, exactly as `main.rs` will: parse the argv, then
/// render. `None` is the missing-`.sla` skip.
fn listing(argv: &[&str]) -> Option<String> {
    let argv: Vec<String> = argv.iter().map(|s| (*s).to_string()).collect();
    let args = disassemble::parse_args(&argv).unwrap_or_else(|e| panic!("{argv:?}: {e}"));
    match disassemble::render(&args) {
        Ok(text) => Some(text),
        Err(e) if is_specs_skip(&e) => {
            eprintln!("skipping: {e}");
            None
        }
        Err(e) => panic!("kuna disassemble {argv:?} failed: {e}"),
    }
}

/// The data rows of the human surface (everything past the `#` header).
fn rows(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).collect()
}

/// The three columns of one human row: address, raw bytes, instruction text.
fn columns(row: &str) -> (&str, &str, String) {
    let mut it = row.split_whitespace();
    let addr = it.next().expect("an address column");
    let bytes = it.next().expect("a bytes column");
    (addr, bytes, it.collect::<Vec<_>>().join(" "))
}

fn parse_doc(doc: &str) -> Vec<(String, Json)> {
    match jsonfmt::parse(doc).expect("--json must parse as JSON") {
        Json::Object(pairs) => pairs,
        other => panic!("--json must emit an object, got {other:?}"),
    }
}

fn field<'a>(pairs: &'a [(String, Json)], key: &str) -> &'a Json {
    pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v).unwrap_or_else(|| {
        panic!("no {key:?} in the document (keys: {:?})", pairs.iter().map(|(k, _)| k).collect::<Vec<_>>())
    })
}

fn as_str(v: &Json) -> &str {
    match v {
        Json::Str(s) => s,
        other => panic!("expected a string, got {other:?}"),
    }
}

fn as_u64(v: &Json) -> u64 {
    match v {
        Json::Number(n) => n.parse().expect("an integer"),
        other => panic!("expected a number, got {other:?}"),
    }
}

fn instructions(doc: &str) -> Vec<Vec<(String, Json)>> {
    match field(&parse_doc(doc), "instructions") {
        Json::Array(items) => items
            .iter()
            .map(|i| match i {
                Json::Object(pairs) => pairs.clone(),
                other => panic!("expected an instruction object, got {other:?}"),
            })
            .collect(),
        other => panic!("`instructions` must be an array, got {other:?}"),
    }
}

// --- the acceptance probes ---------------------------------------------------

/// **The acceptance probe**: `kuna disassemble <bin> main` lists real
/// instructions. Before this command there was no way to see one at all — three
/// testers hit `unknown subcommand "disassemble"` after decompilation had
/// already failed them.
#[test]
fn the_acceptance_probe() {
    let Some(text) = listing(&[&fauxware(), "main"]) else {
        return;
    };
    let rows = rows(&text);
    assert!(rows.len() > 10, "a two-line stub is not a listing:\n{text}");
    // objdump -d: 40071d push %rbp / 40071e mov %rsp,%rbp / 400721 sub $0x40,%rsp.
    assert_eq!(columns(rows[0]), ("0x40071d", "55", "PUSH RBP".to_string()));
    assert_eq!(columns(rows[1]), ("0x40071e", "4889e5", "MOV RBP,RSP".to_string()));
    assert_eq!(columns(rows[2]), ("0x400721", "4883ec40", "SUB RSP,0x40".to_string()));
    // ...and the call objdump renders as `call 400510 <puts@plt>`.
    assert!(text.contains("CALL 0x400510"), "the call to puts@plt is missing:\n{text}");
}

/// The second acceptance probe: `--json` is valid JSON carrying `address_hex`
/// and `mnemonic` per row, plus the `bytes`/`operands`/`text` an agent
/// cross-references against its own tooling.
#[test]
fn the_json_document_is_valid_and_carries_every_documented_field() {
    let Some(doc) = listing(&[&fauxware(), "main", "--json"]) else {
        return;
    };
    let pairs = parse_doc(&doc);
    for key in
        ["binary", "target", "start", "start_hex", "end", "end_hex", "count", "bytes", "truncated", "instructions"]
    {
        let _ = field(&pairs, key);
    }
    assert_eq!(as_u64(field(&pairs, "start")), 0x40071d);
    assert_eq!(as_str(field(&pairs, "start_hex")), "0x40071d");
    let Json::Object(target) = field(&pairs, "target") else { panic!("target must be an object") };
    assert_eq!(as_str(field(target, "name")), "main");
    assert_eq!(as_str(field(target, "address_hex")), "0x40071d");

    let insns = instructions(&doc);
    assert_eq!(as_u64(field(&pairs, "count")) as usize, insns.len());
    let first = &insns[0];
    assert_eq!(as_str(field(first, "address_hex")), "0x40071d");
    assert_eq!(as_str(field(first, "mnemonic")), "PUSH");
    assert_eq!(as_str(field(first, "operands")), "RBP");
    assert_eq!(as_str(field(first, "bytes")), "55");
    assert_eq!(as_str(field(first, "text")), "PUSH RBP");
    assert_eq!(as_u64(field(first, "size")), 1);
}

/// The third acceptance probe: a raw `0x` address with `--addr`, which is the
/// only door open when the binary is stripped or the region is not a function
/// the inventory found.
#[test]
fn a_raw_address_lists_from_exactly_there() {
    let Some(doc) = listing(&[&fauxware(), "0x400739", "--addr", "--count", "2", "--json"]) else {
        return;
    };
    let insns = instructions(&doc);
    assert_eq!(insns.len(), 2, "--count 2 must list two instructions:\n{doc}");
    // 0x400739 is mid-`main`, not an entry: the listing starts on the byte asked
    // for, and objdump reads it as `call 400510 <puts@plt>`.
    assert_eq!(as_str(field(&insns[0], "address_hex")), "0x400739");
    assert_eq!(as_str(field(&insns[0], "text")), "CALL 0x400510");
    assert_eq!(as_str(field(&insns[0], "bytes")), "e8d2fdffff");
    assert_eq!(as_str(field(&insns[1], "address_hex")), "0x40073e");
}

// --- ranges, budgets, and the addresses no function owns ---------------------

/// The case the RE loop recorded as "runtime-decrypted stack code remains an
/// opaque indirect call": bytes at an address that is not a known function, and
/// might not be code at all. An explicit range lists exactly that span.
#[test]
fn an_explicit_range_lists_exactly_that_span() {
    let Some(doc) = listing(&[&fauxware(), "0x400664-0x400674", "--json"]) else {
        return;
    };
    let pairs = parse_doc(&doc);
    assert_eq!(as_u64(field(&pairs, "start")), 0x400664);
    // The walk stops at the first instruction that starts at or past the end, so
    // the listed extent covers the request and overshoots by at most one
    // instruction.
    let end = as_u64(field(&pairs, "end"));
    assert!((0x400674..0x400684).contains(&end), "the range was not honored: end 0x{end:x}\n{doc}");
    for insn in instructions(&doc) {
        let a = as_u64(field(&insn, "address"));
        assert!((0x400664..0x400674).contains(&a), "0x{a:x} is outside the requested range");
    }
    // `..` spells the same range.
    let Some(dotted) = listing(&[&fauxware(), "0x400664..0x400674", "--json"]) else {
        return;
    };
    assert_eq!(dotted, doc, "`-` and `..` must spell the same range");
}

/// `--bytes N` is the other way to bound a region — the one an agent reaches for
/// with a size in hand (a 197-byte decrypted payload, a 0x28-byte blob).
#[test]
fn bytes_bounds_the_listing_and_works_on_data() {
    let Some(doc) = listing(&[&fauxware(), "0x400915", "--addr", "--bytes", "16", "--json"]) else {
        return;
    };
    let pairs = parse_doc(&doc);
    assert_eq!(as_u64(field(&pairs, "start")), 0x400915);
    let end = as_u64(field(&pairs, "end"));
    assert!(end >= 0x400925, "fewer than 16 bytes listed (end 0x{end:x}):\n{doc}");
    assert!(!instructions(&doc).is_empty(), "a data address produced no rows:\n{doc}");
}

/// A named function lists its whole extent by default — the same clip
/// `kuna functions` reports as `size` — and stops before the next entry.
#[test]
fn a_named_function_defaults_to_its_extent() {
    let Some(doc) = listing(&[&fauxware(), "main", "--json"]) else {
        return;
    };
    let pairs = parse_doc(&doc);
    // The next canonical entry after main @ 0x40071d is __libc_csu_init @ 0x4007e0.
    let end = as_u64(field(&pairs, "end"));
    assert!(
        (0x400780..=0x4007e0).contains(&end),
        "main's listing ended at 0x{end:x}, not inside its extent:\n{}",
        &doc[..doc.len().min(400)]
    );
    assert_eq!(field(&pairs, "truncated"), &Json::Bool(false));
}

/// `--count N` lists N instructions and nothing else, whatever the extent says.
#[test]
fn count_bounds_the_listing() {
    for n in [1usize, 3, 7] {
        let Some(doc) = listing(&[&fauxware(), "main", "--count", &n.to_string(), "--json"]) else {
            return;
        };
        assert_eq!(instructions(&doc).len(), n, "--count {n} listed the wrong number:\n{doc}");
    }
}

/// Every row's `bytes` really is that instruction's bytes: the hex is exactly
/// `2 * size` characters, and consecutive rows are contiguous.
#[test]
fn each_row_carries_its_own_bytes_contiguously() {
    let Some(doc) = listing(&[&fauxware(), "main", "--json"]) else {
        return;
    };
    let insns = instructions(&doc);
    let mut expected = 0x40071d;
    for insn in &insns {
        let addr = as_u64(field(insn, "address"));
        let size = as_u64(field(insn, "size"));
        assert_eq!(addr, expected, "a gap or overlap in the listing at 0x{addr:x}");
        assert_eq!(
            as_str(field(insn, "bytes")).len() as u64,
            size * 2,
            "row at 0x{addr:x} reports {size} bytes but prints {:?}",
            as_str(field(insn, "bytes"))
        );
        let text = as_str(field(insn, "text"));
        assert!(text.starts_with(as_str(field(insn, "mnemonic"))), "text/mnemonic disagree: {text}");
        expected = addr + size;
    }
}

/// Bytes the translator will not decode are listed in place as `.byte` rows
/// rather than ending the listing or being silently skipped — the difference
/// between "there is data here" and "kuna stopped".
///
/// `fauxware`'s `.rodata` string at `0x400915` is named `s_400915` by the
/// `strings` pass, so this also covers the string-to-its-bytes hop: the symbol
/// resolves as a target, and `0x61` (`popad`, invalid in 64-bit mode) is the
/// byte that will not decode.
#[test]
fn undecodable_bytes_are_listed_in_place_and_a_string_symbol_resolves() {
    let Some(doc) = listing(&[&fauxware(), "s_400915", "--bytes", "24", "--json"]) else {
        return;
    };
    let pairs = parse_doc(&doc);
    assert_eq!(as_u64(field(&pairs, "start")), 0x400915, "the string symbol did not resolve");
    let insns = instructions(&doc);
    let bad: Vec<_> = insns.iter().filter(|i| as_str(field(i, "mnemonic")) == ".byte").collect();
    assert!(!bad.is_empty(), "no .byte row over undecodable data:\n{doc}");
    for i in &bad {
        assert_eq!(as_u64(field(i, "size")), 1, "a .byte row must cover one byte");
        assert!(as_str(field(i, "text")).starts_with(".byte 0x"), "{:?}", as_str(field(i, "text")));
    }
}

/// The derived-length cap bounds a listing nobody sized — and only that one. An
/// explicit `--count` above the cap is honored verbatim, because a caller that
/// named a number has already decided what it can afford.
#[test]
fn the_derived_cap_never_bounds_an_explicit_request() {
    let Some(doc) = listing(&[&hang_repro(), "0x5020", "--addr", "--json"]) else {
        return;
    };
    let pairs = parse_doc(&doc);
    let derived = as_u64(field(&pairs, "count"));
    assert!(derived <= 1024, "a derived listing ran past the cap: {derived}");
    assert_eq!(field(&pairs, "truncated"), &Json::Bool(false), "nothing was cut, so nothing is flagged");

    let Some(doc) = listing(&[&hang_repro(), "0x5020", "--addr", "--count", "1200", "--json"]) else {
        return;
    };
    assert_eq!(instructions(&doc).len(), 1200, "--count above the cap was clipped");
}

// --- the human surface -------------------------------------------------------

/// The human surface is a `#` header plus aligned rows — never the JSON
/// document, and never a mnemonic glued to its operands.
#[test]
fn the_human_surface_is_a_header_plus_aligned_rows() {
    let Some(text) = listing(&[&fauxware(), "main"]) else {
        return;
    };
    let header = text.lines().next().expect("a header line");
    assert!(
        header.starts_with("# ") && header.contains("instructions at main @ 0x40071d"),
        "{header:?}"
    );
    assert!(!text.contains('{'), "the human surface emitted JSON:\n{text}");
    for row in rows(&text) {
        let (addr, bytes, text_col) = columns(row);
        assert!(addr.starts_with("0x"), "{row:?} does not lead with an address");
        assert!(bytes.chars().all(|c| c.is_ascii_hexdigit()), "{row:?} has no bytes column");
        assert!(!text_col.is_empty(), "{row:?} has no instruction text");
    }
}

// --- failure modes -----------------------------------------------------------

/// An address with nothing behind it is a failed query (exit 1) that says why —
/// distinct from a usage error, and never an empty success a caller has to
/// second-guess.
#[test]
fn an_unmapped_address_fails_with_a_reason() {
    let argv: Vec<String> =
        [fauxware(), "0xdeadbeef000".into(), "--addr".into()].into_iter().collect();
    let args = disassemble::parse_args(&argv).expect("a well-formed command line");
    match disassemble::render(&args) {
        Ok(text) => panic!("an unmapped address must not produce a listing:\n{text}"),
        Err(e) if is_specs_skip(&e) => eprintln!("skipping: {e}"),
        Err(e) => {
            assert!(e.contains("no bytes mapped"), "{e}");
            assert!(e.contains("kuna unpack"), "the failure does not name the move that fixes it: {e}");
        }
    }
}

/// A name nothing carries is the same kind of failure, and points at `--addr`.
#[test]
fn an_unresolvable_name_fails_with_a_reason() {
    let argv: Vec<String> = [fauxware(), "no_such_symbol_here".into()].into_iter().collect();
    let args = disassemble::parse_args(&argv).expect("a well-formed command line");
    match disassemble::render(&args) {
        Ok(text) => panic!("an unresolvable name must not produce a listing:\n{text}"),
        Err(e) if is_specs_skip(&e) => eprintln!("skipping: {e}"),
        Err(e) => {
            assert!(e.contains("no symbol named"), "{e}");
            assert!(e.contains("--addr"), "the failure does not point at --addr: {e}");
        }
    }
}

/// Malformed command lines are rejected at parse time, before anything is
/// loaded — the `main.rs` dispatch turns each of these into exit 2.
#[test]
fn malformed_command_lines_are_usage_errors() {
    for argv in [
        vec![],
        vec!["a.out"],
        vec!["a.out", "main", "extra"],
        vec!["a.out", "main", "--count"],
        vec!["a.out", "main", "--count", "zero"],
        vec!["a.out", "main", "--bytes", "0"],
        vec!["a.out", "main", "--sideways"],
    ] {
        let owned: Vec<String> = argv.iter().map(|s| (*s).to_string()).collect();
        assert!(disassemble::parse_args(&owned).is_err(), "{argv:?} should be a usage error");
    }
}

/// An empty range is a question with no answer, not a zero-row listing.
#[test]
fn an_inverted_range_is_rejected() {
    let argv: Vec<String> = [fauxware(), "0x400680-0x400664".into()].into_iter().collect();
    let args = disassemble::parse_args(&argv).expect("a well-formed command line");
    match disassemble::render(&args) {
        Ok(text) => panic!("an inverted range must not produce a listing:\n{text}"),
        Err(e) if is_specs_skip(&e) => eprintln!("skipping: {e}"),
        Err(e) => assert!(e.contains("empty range"), "{e}"),
    }
}

// --- end to end, through the built binary ------------------------------------

fn run_kuna(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// `disassemble.rs` is dispatch-free until `main.rs` routes `"disassemble"` to
/// it. Until then the end-to-end tests are a visible skip rather than a false
/// green; every test above still covers the command itself.
fn dispatch_wired() -> bool {
    let (_, stderr, _) = run_kuna(&["disassemble"]);
    let wired = !stderr.contains("unknown subcommand");
    if !wired {
        eprintln!("disassemble_cli: skipping (main.rs does not dispatch `disassemble` yet)");
    }
    wired
}

/// The probe as the RE loop will run it: through the binary, exit 0, real
/// instructions on stdout.
#[test]
fn the_cli_exits_zero_and_prints_instructions() {
    if !dispatch_wired() {
        return;
    }
    let (stdout, stderr, code) = run_kuna(&["disassemble", &fauxware(), "main"]);
    if is_specs_skip(&stderr) {
        eprintln!("skipping: {stderr}");
        return;
    }
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("PUSH RBP"), "{stdout}");
    assert!(stdout.contains("CALL 0x400510"), "{stdout}");
}

/// Usage errors are exit 2 with the usage block, never a silent empty listing.
#[test]
fn the_cli_reports_usage_errors_as_exit_two() {
    if !dispatch_wired() {
        return;
    }
    for args in [
        vec!["disassemble"],
        vec!["disassemble", "/nonexistent"],
        vec!["disassemble", "/nonexistent", "main", "--count", "0"],
        vec!["disassemble", "/nonexistent", "main", "--sideways"],
    ] {
        let (_, stderr, code) = run_kuna(&args);
        assert_eq!(code, 2, "{args:?} should be a usage error, got {code}: {stderr}");
        assert!(stderr.contains("usage: kuna disassemble"), "{args:?}: {stderr}");
    }
}

/// A binary that is not there is a load failure (exit 1), not a usage error.
#[test]
fn the_cli_reports_a_missing_binary_as_exit_one() {
    if !dispatch_wired() {
        return;
    }
    let (_, stderr, code) = run_kuna(&["disassemble", "/nonexistent/binary", "main"]);
    assert_eq!(code, 1, "{stderr}");
    // Assert what an agent needs -- the path that failed and why -- not the exact
    // wording. The message comes from the shared load path, so pinning its phrasing
    // here would make an unrelated reword of that path fail this command's test.
    assert!(stderr.contains("/nonexistent/binary"), "{stderr}");
    assert!(
        stderr.contains("No such file or directory"),
        "the reason must be legible, got: {stderr}"
    );
}
