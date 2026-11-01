//! Integration tests for `kuna disassemble` against the built executable.
//! Processor specs and fixtures are required; load failures fail the tests.

use serde_json::{Map, Value as Json};

use crate::common;
use common::{fixture, repo_root, run_kuna};

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
    repo_root()
        .join("tests/hang-repro/ssh-sk-helper")
        .to_str()
        .unwrap()
        .to_string()
}

fn listing(argv: &[&str]) -> String {
    rendered(argv).0
}

fn rendered(argv: &[&str]) -> (String, Vec<String>) {
    let (stdout, stderr, code) = run_kuna(&[&["disassemble"][..], argv].concat());
    assert_eq!(code, 0, "kuna disassemble {argv:?}: {stderr}");
    let notes = stderr
        .lines()
        .map(|line| {
            line.strip_prefix("note: ")
                .unwrap_or_else(|| panic!("unexpected stderr: {line}"))
                .to_owned()
        })
        .collect();
    (stdout, notes)
}

/// The data rows of the human surface (everything past the `#` header).
fn rows(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .collect()
}

/// The three columns of one human row: address, raw bytes, instruction text.
fn columns(row: &str) -> (&str, &str, String) {
    let mut it = row.split_whitespace();
    let addr = it.next().expect("an address column");
    let bytes = it.next().expect("a bytes column");
    (addr, bytes, it.collect::<Vec<_>>().join(" "))
}

fn parse_doc(doc: &str) -> Map<String, Json> {
    match serde_json::from_str::<Json>(doc).expect("--json must parse as JSON") {
        Json::Object(pairs) => pairs,
        other => panic!("--json must emit an object, got {other:?}"),
    }
}

fn field<'a>(pairs: &'a Map<String, Json>, key: &str) -> &'a Json {
    pairs
        .get(key)
        .unwrap_or_else(|| panic!("no {key:?} in the document (keys: {:?})", pairs.keys()))
}

fn as_str(v: &Json) -> &str {
    match v {
        Json::String(s) => s,
        other => panic!("expected a string, got {other:?}"),
    }
}

fn as_u64(v: &Json) -> u64 {
    match v {
        Json::Number(n) => n.as_u64().expect("an integer"),
        other => panic!("expected a number, got {other:?}"),
    }
}

/// The `rows` array of a `--as data` document (the byte view's `instructions`).
fn listed_rows(doc: &str) -> Vec<Map<String, Json>> {
    let pairs = parse_doc(doc);
    let Json::Array(items) = field(&pairs, "rows") else {
        panic!("rows must be an array")
    };
    items
        .iter()
        .map(|i| match i {
            Json::Object(o) => o.clone(),
            other => panic!("a row must be an object, got {other:?}"),
        })
        .collect()
}

fn instructions(doc: &str) -> Vec<Map<String, Json>> {
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
    let text = listing(&[&fauxware(), "main"]);
    let rows = rows(&text);
    assert!(rows.len() > 10, "a two-line stub is not a listing:\n{text}");
    // objdump -d: 40071d push %rbp / 40071e mov %rsp,%rbp / 400721 sub $0x40,%rsp.
    assert_eq!(columns(rows[0]), ("0x40071d", "55", "PUSH RBP".to_string()));
    assert_eq!(
        columns(rows[1]),
        ("0x40071e", "4889e5", "MOV RBP,RSP".to_string())
    );
    assert_eq!(
        columns(rows[2]),
        ("0x400721", "4883ec40", "SUB RSP,0x40".to_string())
    );
    // ...and the call objdump renders as `call 400510 <puts@plt>`.
    assert!(
        text.contains("CALL 0x400510"),
        "the call to puts@plt is missing:\n{text}"
    );
}

/// The third acceptance probe: a word the listed code READS is the constant it
/// holds, not the instruction its bytes happen to spell.
///
/// `sub_8000140` in `cortexm_poolentry_le32` is the shape in the wild
/// (`1337ARM`'s `main` ends the same way): four Thumb instructions, then the
/// literal `0x20001000` the `ldr` two rows above loaded. Decoded, those four
/// bytes read `asrs r0,r0,#0x20` / `movs r0,#0x0` — two instructions nothing
/// executes, in place of the constant the program is about.
#[test]
fn a_literal_pool_word_lists_as_the_constant_it_holds() {
    let bin = fixture("cortexm_poolentry_le32");
    let (text, notes) = rendered(&[&bin, "0x8000140", "--addr"]);
    let rows = rows(&text);
    assert_eq!(
        columns(rows[1]),
        ("0x8000142", "0148", "ldr r0,[0x8000148]".to_string()),
        "the load that proves 0x8000148 is data:\n{text}"
    );
    assert_eq!(
        columns(rows[4]),
        ("0x8000148", "00100020", ".word 0x20001000".to_string()),
        "the pool word, and the two Thumb rows it was decoded as folded into one:\n{text}"
    );
    assert_eq!(
        rows.len(),
        5,
        "the extent is unchanged, one row shorter:\n{text}"
    );
    assert!(!text.contains("asrs"), "the misdecode is gone:\n{text}");
    assert!(
        notes.iter().any(|n| n.contains("literal pool")),
        "a `.word` row must say why it is not an instruction: {notes:?}"
    );

    // The evidence has to be IN the listing, which is also the escape hatch: ask
    // for the word on its own and there is no load to prove anything, so it
    // decodes exactly as it always did.
    let alone = listing(&[&bin, "0x8000148-0x800014c"]);
    assert!(
        alone.contains("asrs r0,r0,#0x20"),
        "the raw decode is still reachable:\n{alone}"
    );
}

/// The same row through `--json`, which is the surface an agent reads: the fold
/// is one row of `size` 4 carrying the pool word's own bytes.
#[test]
fn a_pool_word_is_one_json_row_carrying_its_bytes() {
    let bin = fixture("cortexm_poolentry_le32");
    let doc = listing(&[&bin, "0x8000148", "--addr", "--bytes", "20", "--json"]);
    // Listed from the pool word itself there is no load in range, so nothing
    // folds -- the same self-limiting rule the human surface has.
    let raw = instructions(&doc);
    assert_eq!(as_str(field(&raw[0], "mnemonic")), "asrs");

    let doc = listing(&[&bin, "0x8000140", "--addr", "--json"]);
    let insns = instructions(&doc);
    let word = insns
        .iter()
        .find(|i| as_str(field(i, "address_hex")) == "0x8000148")
        .unwrap();
    assert_eq!(as_str(field(word, "mnemonic")), ".word");
    assert_eq!(as_str(field(word, "operands")), "0x20001000");
    assert_eq!(as_str(field(word, "text")), ".word 0x20001000");
    assert_eq!(as_str(field(word, "bytes")), "00100020");
    assert_eq!(as_u64(field(word, "size")), 4);
    // Folding two rows into one must not move the extent the header reports.
    assert_eq!(as_u64(field(&parse_doc(&doc), "end")), 0x800014c);
    assert_eq!(as_u64(field(&parse_doc(&doc), "bytes")), 12);
}

/// The refusals, on the A32 fixture built for the reference walk's own pool
/// following: a pointer-sized read folds, and a HALFWORD read of the same kind
/// of slot does not — the width has to come from the access, and two bytes do
/// not tile a four-byte row.
#[test]
fn a_narrow_read_does_not_fold_the_word_around_it() {
    let bin = fixture("poolref_arm_le32");
    let wide = listing(&[&bin, "0x10040", "--addr"]);
    let wide = rows(&wide);
    assert_eq!(
        columns(wide[0]),
        ("0x10040", "00009fe5", "ldr r0,[0x10048]".to_string())
    );
    assert_eq!(
        columns(wide[2]),
        ("0x10048", "2a000000", ".word 0x0000002a".to_string())
    );

    let narrow = listing(&[&bin, "0x10034", "--addr"]);
    let narrow = rows(&narrow);
    assert_eq!(
        columns(narrow[0]),
        ("0x10034", "b000dfe1", "ldrh r0,[0x1003c]".to_string())
    );
    let (addr, _, insn) = columns(narrow[2]);
    assert_eq!(addr, "0x1003c");
    assert!(
        !insn.starts_with('.'),
        "a halfword read must not fold a word: {insn}"
    );
}

/// A byte the translator refuses must not take the rest of the listing off the
/// instruction grid.
///
/// `armpoolgrid_le32` is the shape a stripped ARM crackme's `main` has: four
/// PC-relative loads, then the four constants they name — and the first of those,
/// `0xfffffeb8`, is a bit pattern the ARM translator will not decode. Recovering
/// from that one byte at a time restarts the walk at `0x10021`, which no ARM
/// instruction can begin at, and the three pool words after it are then not row
/// starts, so none of the four folds.
#[test]
fn a_refused_byte_resumes_the_listing_on_the_instruction_grid() {
    let bin = fixture("armpoolgrid_le32");
    let (text, notes) = rendered(&[&bin, "0x10000", "--addr"]);
    let listed = rows(&text);
    assert_eq!(
        listed.len(),
        12,
        "one row per instruction and one per pool word:\n{text}"
    );
    for (i, (addr, bytes, insn)) in [
        (8, ("0x10020", "b8feffff", ".word 0xfffffeb8")),
        (9, ("0x10024", "89feffff", ".word 0xfffffe89")),
        (10, ("0x10028", "84fdffff", ".word 0xfffffd84")),
        (11, ("0x1002c", "3ffeffff", ".word 0xfffffe3f")),
    ] {
        assert_eq!(
            columns(listed[i]),
            (addr, bytes, insn.to_string()),
            "row {i}:\n{text}"
        );
    }
    for off in ["0x10021", "0x10025", "0x10029", "0x1002d"] {
        assert!(
            !text.contains(off),
            "{off} is not an ARM instruction address:\n{text}"
        );
    }
    assert!(
        notes.iter().any(|n| n.contains("literal pool")),
        "all four words are proved by the range's own loads: {notes:?}"
    );

    // The extent the header reports is the function's, not one the off-grid
    // walk overshot: 0x10000..0x10030, not ..0x10031.
    let doc = listing(&[&bin, "0x10000", "--addr", "--json"]);
    assert_eq!(as_u64(field(&parse_doc(&doc), "end")), 0x10030);
    assert_eq!(as_u64(field(&parse_doc(&doc), "bytes")), 48);
}

/// The grid is inferred from the listing's own rows, so a walk that has decoded
/// nothing yet has none and keeps the byte-at-a-time recovery: asked for the
/// refused word on its own, the command still shows the byte it would not decode
/// rather than inventing a four-byte row around it.
#[test]
fn a_walk_that_has_decoded_nothing_yet_still_recovers_one_byte_at_a_time() {
    let bin = fixture("armpoolgrid_le32");
    let alone = listing(&[&bin, "0x10020", "--addr", "--count", "1"]);
    let listed = rows(&alone);
    assert_eq!(
        columns(listed[0]),
        ("0x10020", "b8", ".byte 0xb8".to_string()),
        "{alone}"
    );
}

/// x86-64 parks its constants in immediates, so nothing folds there and the
/// listing an agent already knew is byte-identical.
#[test]
fn a_listing_with_no_literal_pool_is_untouched() {
    let (text, notes) = rendered(&[&fauxware(), "main"]);
    assert!(
        !text.contains(".word"),
        "nothing to fold in an x86-64 body:\n{text}"
    );
    assert!(notes.is_empty(), "and nothing to say about it: {notes:?}");
}

/// The second acceptance probe: `--json` is valid JSON carrying `address_hex`
/// and `mnemonic` per row, plus the `bytes`/`operands`/`text` an agent
/// cross-references against its own tooling.
#[test]
fn the_json_document_is_valid_and_carries_every_documented_field() {
    let doc = listing(&[&fauxware(), "main", "--json"]);
    let pairs = parse_doc(&doc);
    for key in [
        "binary",
        "kind",
        "target",
        "start",
        "start_hex",
        "end",
        "end_hex",
        "count",
        "bytes",
        "truncated",
        "notes",
        "instructions",
    ] {
        let _ = field(&pairs, key);
    }
    assert_eq!(as_str(field(&pairs, "kind")), "code");
    assert_eq!(
        field(&pairs, "notes"),
        &Json::Array(Vec::new()),
        "a code listing has nothing to say"
    );
    assert_eq!(as_u64(field(&pairs, "start")), 0x40071d);
    assert_eq!(as_str(field(&pairs, "start_hex")), "0x40071d");
    let Json::Object(target) = field(&pairs, "target") else {
        panic!("target must be an object")
    };
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
    let doc = listing(&[&fauxware(), "0x400739", "--addr", "--count", "2", "--json"]);
    let insns = instructions(&doc);
    assert_eq!(
        insns.len(),
        2,
        "--count 2 must list two instructions:\n{doc}"
    );
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
    let doc = listing(&[&fauxware(), "0x400664-0x400674", "--json"]);
    let pairs = parse_doc(&doc);
    assert_eq!(as_u64(field(&pairs, "start")), 0x400664);
    // The walk stops at the first instruction that starts at or past the end, so
    // the listed extent covers the request and overshoots by at most one
    // instruction.
    let end = as_u64(field(&pairs, "end"));
    assert!(
        (0x400674..0x400684).contains(&end),
        "the range was not honored: end 0x{end:x}\n{doc}"
    );
    for insn in instructions(&doc) {
        let a = as_u64(field(&insn, "address"));
        assert!(
            (0x400664..0x400674).contains(&a),
            "0x{a:x} is outside the requested range"
        );
    }
    // `..` spells the same range.
    let dotted = listing(&[&fauxware(), "0x400664..0x400674", "--json"]);
    assert_eq!(dotted, doc, "`-` and `..` must spell the same range");
}

/// `--bytes N` is the other way to bound a region — the one an agent reaches for
/// with a size in hand (a 197-byte decrypted payload, a 0x28-byte blob). On a
/// data address it bounds the byte view, which is the one `auto` picks there.
#[test]
fn bytes_bounds_the_listing_and_works_on_data() {
    let doc = listing(&[&fauxware(), "0x400915", "--addr", "--bytes", "16", "--json"]);
    let pairs = parse_doc(&doc);
    assert_eq!(as_u64(field(&pairs, "start")), 0x400915);
    assert_eq!(
        as_u64(field(&pairs, "end")),
        0x400925,
        "--bytes 16 is exactly 16 bytes:\n{doc}"
    );
    assert_eq!(
        as_str(field(&pairs, "kind")),
        "data",
        ".rodata is not an instruction stream"
    );
    assert!(
        !listed_rows(&doc).is_empty(),
        "a data address produced no rows:\n{doc}"
    );

    // Forced back to code, the same request is the instruction walk it always was.
    let code = listing(&[
        &fauxware(),
        "0x400915",
        "--addr",
        "--bytes",
        "16",
        "--as",
        "code",
        "--json",
    ]);
    assert_eq!(as_str(field(&parse_doc(&code), "kind")), "code");
    assert!(
        !instructions(&code).is_empty(),
        "--as code produced no instructions:\n{code}"
    );
}

/// A named function lists its whole extent by default — the same clip
/// `kuna functions` reports as `size` — and stops before the next entry.
#[test]
fn a_named_function_defaults_to_its_extent() {
    let doc = listing(&[&fauxware(), "main", "--json"]);
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
        let doc = listing(&[&fauxware(), "main", "--count", &n.to_string(), "--json"]);
        assert_eq!(
            instructions(&doc).len(),
            n,
            "--count {n} listed the wrong number:\n{doc}"
        );
    }
}

/// Every row's `bytes` really is that instruction's bytes: the hex is exactly
/// `2 * size` characters, and consecutive rows are contiguous.
#[test]
fn each_row_carries_its_own_bytes_contiguously() {
    let doc = listing(&[&fauxware(), "main", "--json"]);
    let insns = instructions(&doc);
    let mut expected = 0x40071d;
    for insn in &insns {
        let addr = as_u64(field(insn, "address"));
        let size = as_u64(field(insn, "size"));
        assert_eq!(
            addr, expected,
            "a gap or overlap in the listing at 0x{addr:x}"
        );
        assert_eq!(
            as_str(field(insn, "bytes")).len() as u64,
            size * 2,
            "row at 0x{addr:x} reports {size} bytes but prints {:?}",
            as_str(field(insn, "bytes"))
        );
        let text = as_str(field(insn, "text"));
        assert!(
            text.starts_with(as_str(field(insn, "mnemonic"))),
            "text/mnemonic disagree: {text}"
        );
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
    let doc = listing(&[
        &fauxware(),
        "s_400915",
        "--bytes",
        "24",
        "--as",
        "code",
        "--json",
    ]);
    let pairs = parse_doc(&doc);
    assert_eq!(
        as_u64(field(&pairs, "start")),
        0x400915,
        "the string symbol did not resolve"
    );
    let insns = instructions(&doc);
    let bad: Vec<_> = insns
        .iter()
        .filter(|i| as_str(field(i, "mnemonic")) == ".byte")
        .collect();
    assert!(
        !bad.is_empty(),
        "no .byte row over undecodable data:\n{doc}"
    );
    for i in &bad {
        assert_eq!(
            as_u64(field(i, "size")),
            1,
            "a .byte row must cover one byte"
        );
        assert!(
            as_str(field(i, "text")).starts_with(".byte 0x"),
            "{:?}",
            as_str(field(i, "text"))
        );
    }
}

/// The derived-length cap bounds a listing nobody sized — and only that one. An
/// explicit `--count` above the cap is honored verbatim, because a caller that
/// named a number has already decided what it can afford.
#[test]
fn the_derived_cap_never_bounds_an_explicit_request() {
    let doc = listing(&[&hang_repro(), "0x5020", "--addr", "--json"]);
    let pairs = parse_doc(&doc);
    let derived = as_u64(field(&pairs, "count"));
    assert!(
        derived <= 1024,
        "a derived listing ran past the cap: {derived}"
    );
    assert_eq!(
        field(&pairs, "truncated"),
        &Json::Bool(false),
        "nothing was cut, so nothing is flagged"
    );

    let doc = listing(&[
        &hang_repro(),
        "0x5020",
        "--addr",
        "--count",
        "1200",
        "--json",
    ]);
    assert_eq!(
        instructions(&doc).len(),
        1200,
        "--count above the cap was clipped"
    );
}

// --- the byte view -----------------------------------------------------------

/// The need this view closes: an agent inspecting the encoded globals at a data
/// address got instructions back and left kuna for `xxd`
/// (`docs/re-needs/cli-mode-read-raw.md`). Now the span comes back as one
/// contiguous hex string, per-row bytes, and an ASCII gutter — and `auto` picks
/// it without being asked, because `.rodata` is not an instruction stream.
#[test]
fn a_data_range_answers_with_its_bytes() {
    let (doc, notes) = rendered(&[&fauxware(), "0x400915-0x400925", "--json"]);
    let pairs = parse_doc(&doc);
    assert_eq!(as_str(field(&pairs, "kind")), "data");
    assert_eq!(as_u64(field(&pairs, "start")), 0x400915);
    assert_eq!(
        as_u64(field(&pairs, "end")),
        0x400925,
        "a byte view honors the end exactly"
    );
    assert_eq!(as_u64(field(&pairs, "bytes")), 16);
    // objdump -s -j .rodata: "Username: " then a NUL then "Passw".
    assert_eq!(
        as_str(field(&pairs, "hex")),
        "557365726e616d653a20005061737377"
    );
    // ...and the note that explains the view is on the record, not just on stderr.
    assert_eq!(notes.len(), 1, "the inferred view must say so: {notes:?}");
    assert!(notes[0].contains("non-executable"), "{notes:?}");
    let Json::Array(carried) = field(&pairs, "notes") else {
        panic!("notes must be an array")
    };
    assert_eq!(
        carried.len(),
        1,
        "the JSON carries the same note the stderr does"
    );

    let rows = listed_rows(&doc);
    assert_eq!(rows.len(), 1, "16 bytes is one row");
    assert_eq!(as_str(field(&rows[0], "address_hex")), "0x400915");
    assert_eq!(as_u64(field(&rows[0], "size")), 16);
    assert_eq!(
        as_str(field(&rows[0], "ascii")),
        "Username: .Passw",
        "the gutter is printable-only"
    );
}

/// `hex` is the whole span in one piece, and the rows are exactly that string
/// cut into sixteens — a caller may use either and never both.
#[test]
fn the_span_hex_is_the_rows_joined() {
    let doc = listing(&[&fauxware(), "0x4008c8", "--addr", "--bytes", "40", "--json"]);
    let pairs = parse_doc(&doc);
    let rows = listed_rows(&doc);
    assert_eq!(rows.len(), 3, "40 bytes is two full rows and a short one");
    let joined: String = rows
        .iter()
        .map(|r| as_str(field(r, "bytes")).to_string())
        .collect();
    assert_eq!(as_str(field(&pairs, "hex")), joined);
    assert_eq!(joined.len(), 80, "40 bytes is 80 hex characters");
    for r in &rows {
        assert_eq!(
            as_str(field(r, "bytes")).len() as u64,
            as_u64(field(r, "size")) * 2
        );
    }
    assert_eq!(
        as_u64(field(&rows[2], "size")),
        8,
        "the last row is the remainder"
    );
}

/// A code address stays code under `auto`, and `--as data` reads its bytes on
/// demand — a packer puts real code in `.data` and the caller outranks the flags.
#[test]
fn the_view_is_the_callers_to_override() {
    let (auto, notes) = rendered(&[&fauxware(), "main", "--count", "1", "--json"]);
    assert_eq!(as_str(field(&parse_doc(&auto), "kind")), "code");
    assert!(
        notes.is_empty(),
        "nothing was inferred, so nothing is said: {notes:?}"
    );

    let (forced, notes) = rendered(&[
        &fauxware(),
        "main",
        "--bytes",
        "4",
        "--as",
        "data",
        "--json",
    ]);
    let pairs = parse_doc(&forced);
    assert_eq!(as_str(field(&pairs, "kind")), "data");
    // objdump -d: main opens 55 48 89 e5.
    assert_eq!(as_str(field(&pairs, "hex")), "554889e5");
    assert!(
        notes.is_empty(),
        "an explicit --as is not explained back at the caller: {notes:?}"
    );
}

/// The human byte surface is `xxd -g1` with kuna's address column.
#[test]
fn the_human_byte_surface_is_a_hexdump() {
    let text = listing(&[&fauxware(), "0x400915", "--addr", "--bytes", "16"]);
    let mut lines = text.lines();
    assert_eq!(
        lines.next().unwrap(),
        "# 16 bytes at s_400915 @ 0x400915 (0x400915..0x400925)"
    );
    assert_eq!(
        lines.next().unwrap(),
        "0x400915      55 73 65 72 6e 61 6d 65 3a 20 00 50 61 73 73 77  |Username: .Passw|"
    );
    assert!(lines.next().is_none(), "16 bytes is one row:\n{text}");
    assert!(
        !text.contains('{'),
        "the human surface emitted JSON:\n{text}"
    );
}

// --- the human surface -------------------------------------------------------

/// The human surface is a `#` header plus aligned rows — never the JSON
/// document, and never a mnemonic glued to its operands.
#[test]
fn the_human_surface_is_a_header_plus_aligned_rows() {
    let text = listing(&[&fauxware(), "main"]);
    let header = text.lines().next().expect("a header line");
    assert!(
        header.starts_with("# ") && header.contains("instructions at main @ 0x40071d"),
        "{header:?}"
    );
    assert!(
        !text.contains('{'),
        "the human surface emitted JSON:\n{text}"
    );
    for row in rows(&text) {
        let (addr, bytes, text_col) = columns(row);
        assert!(
            addr.starts_with("0x"),
            "{row:?} does not lead with an address"
        );
        assert!(
            bytes.chars().all(|c| c.is_ascii_hexdigit()),
            "{row:?} has no bytes column"
        );
        assert!(!text_col.is_empty(), "{row:?} has no instruction text");
    }
}

// --- failure modes -----------------------------------------------------------

/// An address with nothing behind it is a failed query (exit 1) that says why —
/// distinct from a usage error, and never an empty success a caller has to
/// second-guess.
#[test]
fn an_unmapped_address_fails_with_a_reason() {
    let (_, error, code) = run_kuna(&["disassemble", &fauxware(), "0xdeadbeef000", "--addr"]);
    assert_eq!(code, 1, "{error}");
    assert!(error.contains("no bytes mapped"), "{error}");
    assert!(error.contains("kuna unpack"), "{error}");
}

/// A name nothing carries is the same kind of failure, and points at `--addr`.
#[test]
fn an_unresolvable_name_fails_with_a_reason() {
    let (_, error, code) = run_kuna(&["disassemble", &fauxware(), "no_such_symbol_here"]);
    assert_eq!(code, 1, "{error}");
    assert!(error.contains("no symbol named"), "{error}");
    assert!(error.contains("--addr"), "{error}");
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
        let (_, error, code) = run_kuna(&[&["disassemble"][..], &argv].concat());
        assert_eq!(code, 2, "{argv:?} should be a usage error: {error}");
    }
}

/// An empty range is a question with no answer, not a zero-row listing.
#[test]
fn an_inverted_range_is_rejected() {
    let (_, error, code) = run_kuna(&["disassemble", &fauxware(), "0x400680-0x400664"]);
    assert_eq!(code, 1, "{error}");
    assert!(error.contains("empty range"), "{error}");
}

// --- end to end, through the built binary ------------------------------------

/// The probe as the RE loop will run it: through the binary, exit 0, real
/// instructions on stdout.
#[test]
fn the_cli_exits_zero_and_prints_instructions() {
    let (stdout, stderr, code) = run_kuna(&["disassemble", &fauxware(), "main"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("PUSH RBP"), "{stdout}");
    assert!(stdout.contains("CALL 0x400510"), "{stdout}");
}

/// `kuna read` is the same command with the byte view as its default, so an
/// agent that wants the bytes at an address does not have to know they are
/// spelled `disassemble`. Through the binary, because the alias IS the dispatch.
#[test]
fn the_read_alias_prints_bytes_and_explains_nothing() {
    let (stdout, stderr, code) = run_kuna(&["read", &fauxware(), "main", "--bytes", "4"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("55 48 89 e5"), "{stdout}");
    assert!(
        !stdout.contains("PUSH RBP"),
        "`read` must not disassemble:\n{stdout}"
    );
    // The view was asked for, not inferred, so there is nothing to explain.
    assert!(!stderr.contains("note:"), "{stderr}");

    // ...and the caller can still ask for instructions through it.
    let (stdout, _, code) =
        run_kuna(&["read", &fauxware(), "main", "--count", "1", "--as", "code"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("PUSH RBP"),
        "--as code must win over the alias default:\n{stdout}"
    );
}

/// The inferred view puts its reason on **stderr**, so `--json` stdout stays a
/// document a caller can pipe straight into a parser.
#[test]
fn an_inferred_byte_view_explains_itself_on_stderr() {
    let (stdout, stderr, code) =
        run_kuna(&["disassemble", &fauxware(), "0x400915-0x400925", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stderr.contains("non-executable data section"), "{stderr}");
    assert!(
        stdout.trim_start().starts_with('{'),
        "stdout must be the document alone:\n{stdout}"
    );
    assert!(
        stdout.contains("557365726e616d653a20005061737377"),
        "{stdout}"
    );
}

/// Usage errors are exit 2 with the usage block, never a silent empty listing.
#[test]
fn the_cli_reports_usage_errors_as_exit_two() {
    for args in [
        vec!["disassemble"],
        vec!["disassemble", "/nonexistent"],
        vec!["disassemble", "/nonexistent", "main", "--count", "0"],
        vec!["disassemble", "/nonexistent", "main", "--sideways"],
    ] {
        let (_, stderr, code) = run_kuna(&args);
        assert_eq!(
            code, 2,
            "{args:?} should be a usage error, got {code}: {stderr}"
        );
        assert!(
            stderr.contains("usage: kuna disassemble"),
            "{args:?}: {stderr}"
        );
    }
}

/// A binary that is not there is a load failure (exit 1), not a usage error.
#[test]
fn the_cli_reports_a_missing_binary_as_exit_one() {
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

// --- the windowed load --------------------------------------------------------
//
// A caller-bounded listing is answered without the program-wide discovery walk,
// and falls back to it when the walk is the only thing that can answer. These
// pin both halves: the fallback still resolves a name only discovery invents,
// and the fast half agrees with the walk it skipped.

/// The stripped x86-64 fixture whose `sub_1190` exists ONLY because the Listing
/// tier's discovery walk found it: `kuna functions` lists 20 entries with the
/// walk on and 19 with it off, and `sub_1190` is the difference.
fn stripped_dynamic() -> String {
    fixture("stripped_dynamic_x86_64")
}

/// The name the discovery walk invents still resolves, by either spelling.
///
/// This is the regression the windowed load could have shipped: skip the walk
/// for a bounded window and `kuna disassemble sub_1190 --count 4` answers "no
/// symbol named" for a name `kuna functions` prints (RE-need
/// `analysis-generated-function-name`).
#[test]
fn a_name_only_the_discovery_walk_invents_still_resolves() {
    let by_name = listing(&[&stripped_dynamic(), "sub_1190", "--count", "4"]);
    assert_eq!(rows(&by_name).len(), 4, "{by_name}");
    assert!(by_name.contains("sub_1190 @ 0x1190"), "{by_name}");

    // The address spelling of the same target: the windowed load has no name for
    // it either, so it must fall back too rather than print a nameless header.
    let by_addr = listing(&[&stripped_dynamic(), "0x1190", "--addr", "--count", "4"]);
    assert_eq!(rows(&by_addr), rows(&by_name), "{by_addr}");
    assert!(by_addr.contains("sub_1190 @ 0x1190"), "{by_addr}");
}

/// The bounded listing is the same one the discovery walk would have produced.
///
/// The right-hand side names both walk options, which the windowed load leaves
/// alone (the caller's word is the last one), so it is the full-inventory answer
/// — and the left-hand side is the windowed one.
#[test]
fn a_bounded_listing_agrees_with_the_walk_it_skipped() {
    let walk = [
        "--option",
        "listing",
        "on",
        "--option",
        "fast_funcdisc",
        "on",
    ];
    for (binary, target, bound) in [
        (fauxware(), "main", ["--count", "6"]),
        (fauxware(), "main", ["--bytes", "24"]),
        (fauxware(), "0x40071d", ["--addr", "--count"]),
        (stripped_dynamic(), "sub_1020", ["--count", "6"]),
        (stripped_dynamic(), "0x1020-0x1040", ["--count", "6"]),
    ] {
        // `0x40071d --addr` needs its count spelled after the flag pair above.
        let bound: Vec<&str> = if bound[1] == "--count" {
            vec![bound[0], bound[1], "6"]
        } else {
            bound.to_vec()
        };
        let mut plain = vec![binary.as_str(), target];
        plain.extend(bound.iter().copied());
        let windowed = listing(&plain);
        let mut full = plain.clone();
        full.extend(walk.iter().copied());
        let full = listing(&full);
        assert_eq!(windowed, full, "{plain:?}");
    }
}

/// A listing that reaches the end of mapped memory stops there.
///
/// The load image answers a read that STARTS on mapped memory for its whole
/// length, zero-filling whatever the segments do not cover (the upstream BFD
/// `loadFill` contract), so only the start address was ever checked and the
/// listing decoded the fill. `segmentgap_i386` is the witness's own layout: one
/// `R E` `PT_LOAD` ending mid-page at 0x8048014, an unmapped hole, then a `RW`
/// `PT_LOAD` at 0x804a000. Unbounded, this walked to 0x804803f and reported
/// twenty-two `ADD byte ptr [EAX],AL` rows the file does not contain.
#[test]
fn a_listing_stops_at_the_end_of_mapped_memory() {
    let bin = fixture("segmentgap_i386");
    let (text, notes) = rendered(&[&bin, "0x8048000", "--addr", "--count", "30"]);
    let listed = rows(&text);
    assert_eq!(
        listed.len(),
        9,
        "the segment holds nine rows, not thirty:\n{text}"
    );
    assert!(
        listed.iter().all(|r| !r.starts_with("0x8048014")),
        "0x8048014 is the first unmapped byte and can hold no row:\n{text}"
    );

    // The last mapped byte is a lone 0x00. The two-byte `add [eax],al` the
    // translator reads there straddles the boundary, so it is not an
    // instruction: the one byte the image really holds is listed instead.
    assert_eq!(
        columns(listed[8]),
        ("0x8048013", "00", ".byte 0x00".to_string()),
        "{text}"
    );
    assert!(
        notes
            .iter()
            .any(|n| n.contains("0x8048014") && n.contains("not in the image")),
        "the image, not the ask, ended the listing and must say so: {notes:?}"
    );
    assert!(
        notes.iter().any(|n| n.contains("0x804a000")),
        "the next mapped address is where a caller resumes: {notes:?}"
    );

    let doc = listing(&[&bin, "0x8048000", "--addr", "--count", "30", "--json"]);
    let doc = parse_doc(&doc);
    assert_eq!(as_u64(field(&doc, "end")), 0x8048014);
    assert_eq!(as_u64(field(&doc, "count")), 9);
    assert_eq!(as_u64(field(&doc, "bytes")), 20);

    // The byte view walked the same fill, and stops on the same bound.
    let bytes = listing(&[&bin, "0x8048008", "--addr", "--as", "data", "--count", "4"]);
    assert_eq!(
        rows(&bytes).len(),
        1,
        "twelve mapped bytes, not four rows of sixteen:\n{bytes}"
    );
    assert!(bytes.contains("0x8048008..0x8048014"), "{bytes}");
}

/// The bound is the end of mapped memory, not the end of a segment: a listing
/// that asks for less than the image holds is untouched, and one that asks for
/// exactly the mapped bytes says nothing about it.
#[test]
fn a_listing_inside_mapped_memory_is_untouched_by_the_bound() {
    let bin = fixture("segmentgap_i386");
    let (text, notes) = rendered(&[&bin, "0x8048000", "--addr", "--count", "4"]);
    assert_eq!(rows(&text).len(), 4, "{text}");
    assert!(notes.is_empty(), "the ask ended this one: {notes:?}");

    // Exactly the mapped extent, spelled as a range: all of it, and no note.
    let (all, notes) = rendered(&[&bin, "0x8048000-0x8048014", "--addr"]);
    assert_eq!(rows(&all).len(), 9, "{all}");
    assert!(notes.is_empty(), "nothing was cut short: {notes:?}");

    // `fauxware`'s `main` is nowhere near a segment edge, so its listing is the
    // one an agent already knew, byte for byte.
    let (before, _) = rendered(&[&fauxware(), "main"]);
    assert!(before.contains("PUSH RBP"), "{before}");
    assert!(!before.contains("not in the image"), "{before}");
}

// --- decoy bytes and the branch targets a straight line walks over -----------

/// (kuna, `linear-disassembly-silently-skips`) The straight line says so when it
/// decodes across an address the range's own branches name.
///
/// `jumpoverdecoy_i386` is the round-12 witness's own 60 bytes rebased to
/// `0x10000` — a decryption loop entered through `EB 01` / `EB 04` jumps over one
/// junk byte each. What a plain listing of it gets wrong is not the missing row:
/// it prints a `JMP` outside the image and a `CALL` to a negative displacement,
/// and neither is in the bytes.
#[test]
fn a_straight_line_listing_names_the_branch_targets_it_decoded_across() {
    let bin = fixture("jumpoverdecoy_i386");
    let (text, notes) = rendered(&[&bin, "0x10000", "--addr", "--count", "70"]);
    let note = notes
        .iter()
        .find(|n| n.contains("ran across"))
        .unwrap_or_else(|| panic!("no skipped-target note in {notes:?}\n{text}"));
    for target in ["0x10008", "0x10024", "0x10037"] {
        assert!(note.contains(target), "{target} is not named in {note:?}");
        assert!(
            !rows(&text)
                .iter()
                .any(|r| r.starts_with(&format!("0x{}", &target[2..]))),
            "{target} starts a row, so it was not walked over:\n{text}"
        );
    }
    assert!(
        note.contains("--follow"),
        "the note must name the move that fixes it: {note:?}"
    );
    // The rows that ARE there, and are wrong — the harm the note is about.
    assert!(
        text.contains("JMP 0x496c8e8"),
        "the invented out-of-image jump:\n{text}"
    );
    assert!(
        text.contains("CALL -0x1d546d91"),
        "the invented call:\n{text}"
    );
}

/// The same range under `--follow`: every branch target starts a row, the
/// invented instructions are gone, and the loop the bytes really spell — LODSB,
/// STOSB, and a `LOOP` back edge — is readable.
#[test]
fn follow_decodes_from_the_targets_the_straight_line_walked_over() {
    let bin = fixture("jumpoverdecoy_i386");
    let (text, notes) = rendered(&[&bin, "0x10000", "--addr", "--count", "70", "--follow"]);
    assert!(
        !notes.iter().any(|n| n.contains("ran across")),
        "nothing was walked over this time: {notes:?}\n{text}"
    );
    assert!(
        notes.iter().any(|n| n.contains("--follow decoded from")),
        "--follow must say what it re-anchored: {notes:?}"
    );
    for (addr, insn) in [
        ("0x10008", "LODSB"),
        ("0x10024", "ROR AL,0x95"),
        ("0x10037", "XOR AL,0x92"),
    ] {
        let row = rows(&text)
            .into_iter()
            .find(|r| r.starts_with(addr))
            .unwrap_or_else(|| panic!("no row at {addr}:\n{text}"));
        assert!(row.contains(insn), "{row:?} is not {insn}");
    }
    assert!(text.contains("STOSB"), "{text}");
    assert!(
        text.contains("LOOP 0x10008"),
        "the back edge that closes the loop:\n{text}"
    );
    assert!(
        !text.contains("JMP 0x496c8e8"),
        "the invented jump survived:\n{text}"
    );
    assert!(
        !text.contains("CALL -0x1d546d91"),
        "the invented call survived:\n{text}"
    );
    // A byte no flow reaches is still listed, spelled as the byte it is.
    assert!(
        text.contains(".byte 0xc2"),
        "the decoy byte is not in the listing:\n{text}"
    );
}

/// `--follow` never lists less than the straight line: it covers the same span,
/// and on a function with no decoys in it the two listings are the same bytes.
#[test]
fn follow_leaves_an_ordinary_listing_alone() {
    let (plain, notes) = rendered(&[&fauxware(), "main"]);
    assert!(
        notes.is_empty(),
        "an ordinary main has nothing to report: {notes:?}"
    );
    let (followed, notes) = rendered(&[&fauxware(), "main", "--follow"]);
    assert!(notes.is_empty(), "{notes:?}");
    assert_eq!(
        plain, followed,
        "--follow moved a row in a function with no decoy in it"
    );
}

/// The two views cover the same addresses: `--follow` re-anchors rows, it does
/// not drop the bytes between them.
#[test]
fn follow_covers_the_same_bytes_as_the_straight_line() {
    let bin = fixture("jumpoverdecoy_i386");
    let span = |argv: &[&str]| {
        instructions(&listing(argv))
            .into_iter()
            .fold(None, |acc: Option<(u64, u64)>, row| {
                let addr = as_u64(field(&row, "address"));
                let end = addr + as_u64(field(&row, "size"));
                Some(acc.map_or((addr, end), |(lo, hi)| (lo.min(addr), hi.max(end))))
            })
    };
    let plain = span(&[&bin, "0x10000", "--addr", "--count", "70", "--json"])
        .expect("nonempty straight-line listing");
    let followed = span(&[
        &bin, "0x10000", "--addr", "--count", "70", "--follow", "--json",
    ]).expect("nonempty followed listing");
    assert_eq!(
        plain, followed,
        "the two listings must cover the same bytes"
    );
    assert_eq!(plain, (0x10000, 0x1003d), "the whole mapped run");
}

#[test]
fn raw_pcode_exposes_instruction_ops_and_reports_limits() {
    let binary = fauxware();
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "main", "--count", "1", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    let instructions = doc["instructions"].as_array().unwrap();
    assert_eq!(instructions.len(), 1);
    assert_eq!(doc["truncated"], false, "a --count the walk met is not truncation, as in disassemble");
    let ops = instructions[0]["ops"].as_array().unwrap();
    assert_eq!(instructions[0]["mnemonic"], "PUSH");
    assert_eq!(instructions[0]["bytes"], "55");
    assert_eq!(doc["start_hex"], instructions[0]["address_hex"]);
    assert_eq!(doc["end_hex"], format!("0x{:x}", doc["end"].as_u64().unwrap()));
    assert_eq!(ops.iter().map(|op| op["opcode"].as_str().unwrap()).collect::<Vec<_>>(), ["COPY", "INT_SUB", "STORE"]);
    assert_eq!(ops[1]["inputs"][1]["space"], "const");
    assert_eq!(ops[1]["inputs"][1]["offset"], 8);
    assert_eq!(ops[2]["output"], Json::Null);
    for op in ops {
        assert!(op["opcode"].as_str().is_some());
        for vn in op["inputs"].as_array().unwrap() {
            assert!(vn["space"].as_str().is_some());
            assert!(vn["offset"].as_u64().is_some());
            assert!(vn["size"].as_u64().is_some());
        }
    }
    assert_eq!(doc["kind"], "pcode");
    assert_eq!(doc["target"]["name"], "main");
    assert_eq!(doc["count"], 1);
    let (text, stderr, code) = run_kuna(&["raw-pcode", &binary, "main", "--count", "1"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(text.starts_with("# 1 instruction at main @ 0x40071d (0x40071d..0x40071e, 1 bytes)\n"), "{text}");
    assert!(text.contains("55  PUSH RBP"));
    assert!(text.contains(ops[0]["opcode"].as_str().unwrap()));
    assert!(!text.contains("# truncated at"), "{text}");
    let next = format!("0x{:x}", doc["end"].as_u64().unwrap());
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, &next, "--bytes", "1"]);
    assert_eq!(code, 1);
    assert!(stderr.contains("error: instruction at 0x40071e crosses the requested or mapped boundary"), "{stderr}");
    assert!(stdout.contains("# stopped at 0x40071e: instruction at 0x40071e crosses"), "{stdout}");
    let (partial, stderr, code) = run_kuna(&["raw-pcode", &binary, "main", "--bytes", "2", "--json"]);
    assert_eq!(code, 1, "{stderr}");
    let partial: Json = serde_json::from_str(&partial).unwrap();
    assert_eq!(partial["stop_reason"], "decode_error");
    assert_eq!(partial["instructions"].as_array().unwrap().len(), 1);
    assert_eq!(partial["end"], doc["end"]);
    assert_eq!(partial["error"]["address"], doc["end"]);
    assert_eq!(doc["stop_reason"], "instruction_limit");
}

#[test]
fn raw_pcode_rejects_unmapped_addresses_and_unsupported_walks() {
    let binary = fauxware();
    for args in [vec!["raw-pcode", &binary, "0xffffffffffffffff"], vec!["raw-pcode", &binary, "main", "--as", "data"]] {
        let (_, stderr, code) = run_kuna(&args);
        assert_ne!(code, 0);
        assert!(stderr.contains("error:"));
    }
}

#[test]
fn raw_pcode_arm_interworking_selects_the_right_mode() {
    let binary = fixture("arm_interwork_le32");
    let raw = |target: &str| {
        let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, target, "--count", "1", "--json"]);
        assert_eq!(code, 0, "{stderr}");
        serde_json::from_str::<Json>(&stdout).unwrap()
    };
    let call = raw("0x2000004");
    let instruction = &call["instructions"][0];
    assert_eq!(instruction["bytes"], "0d0000fa");
    assert_eq!(instruction["mnemonic"], "blx");
    assert!(instruction["ops"].as_array().unwrap().iter().any(|op|
        op["opcode"] == "CALL" && op["inputs"][0]["offset"] == 0x2000040));
    let thumb = raw("0x2000040");
    let odd_thumb = raw("0x2000041");
    assert_eq!(thumb["instructions"], odd_thumb["instructions"]);
    assert_eq!(thumb["instructions"][0]["bytes"], "0720");
    assert_eq!(thumb["instructions"][0]["size"], 2);
    assert_eq!(thumb["instructions"][0]["mnemonic"], "movs");
    let arm = raw("0x2000080");
    assert_eq!(arm["instructions"][0]["bytes"], "010080e2");
    assert_eq!(arm["instructions"][0]["size"], 4);
    assert_eq!(arm["instructions"][0]["mnemonic"], "add");
}

#[test]
fn raw_pcode_mips_return_includes_delay_slot() {
    let binary = fixture("bejoin_order_mips32_O2");
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "widen", "--count", "2", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    let ret = &doc["instructions"][1];
    assert_eq!(ret["address_hex"], "0x400134");
    assert_eq!(ret["bytes"], "03e0000800801825");
    assert_eq!(ret["size"], 8);
    assert_eq!(ret["mnemonic"], "jr");
    let ops = ret["ops"].as_array().unwrap();
    let delay = ops.iter().position(|op| op["opcode"] == "INT_OR").unwrap();
    let return_op = ops.iter().position(|op| op["opcode"] == "RETURN").unwrap();
    assert!(delay < return_op);
}


fn raw_flow_fixture(bytes: &[u8]) -> String {
    raw_flow_fixture_for(object::Architecture::X86_64, bytes)
}

fn raw_flow_fixture_for(arch: object::Architecture, bytes: &[u8]) -> String {
    use object::write::{Object, Symbol, SymbolSection};
    use object::{BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
    let mut obj = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, bytes, 16);
    obj.add_symbol(Symbol { name: b"probe".to_vec(), value: 0, size: bytes.len() as u64,
        kind: SymbolKind::Text, scope: SymbolScope::Linkage, weak: false,
        section: SymbolSection::Section(section), flags: SymbolFlags::None });
    let path = common::scratch_file("raw-flow", "o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    path.to_str().unwrap().to_string()
}

#[test]
fn raw_pcode_follow_skips_inline_data_and_follows_both_paths() {
    // jmp +2; ud2 (unreachable); je +1; ret; ret
    let binary = raw_flow_fixture(&[0xeb, 0x02, 0x0f, 0x0b, 0x74, 0x01, 0xc3, 0xc3]);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--follow", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["complete"], true, "{doc}");
    let start = doc["start"].as_u64().unwrap();
    let blocks = doc["blocks"].as_array().unwrap();
    let addresses: Vec<u64> = blocks.iter().flat_map(|block| block["instructions"].as_array().unwrap())
        .map(|row| row["address"].as_u64().unwrap()).collect();
    assert_eq!(addresses, [start, start + 4, start + 6, start + 7]);
    let conditional = blocks.iter().find(|b| b["start"] == start + 4).unwrap();
    let successors = conditional["successors"].as_array().unwrap();
    assert!(successors.iter().any(|edge| edge["target"] == start + 6));
    assert!(successors.iter().any(|edge| edge["target"] == start + 7));
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--follow", "--count", "1", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let limited: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(limited["complete"], false);
    assert!(limited["issues"].as_array().unwrap().iter().any(|issue| issue["reason"] == "instruction_limit"));
}

#[test]
fn raw_pcode_truncated_means_the_derived_cap_as_in_disassemble() {
    let binary = raw_flow_fixture(&[0x90; 1100]);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["count"], 1024);
    assert_eq!(doc["stop_reason"], "instruction_limit");
    assert_eq!(doc["truncated"], true);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--count", "1024", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["stop_reason"], "instruction_limit");
    assert_eq!(doc["truncated"], false);
}

#[test]
fn raw_pcode_follow_reports_each_undecodable_target_once() {
    // je, jne and jb all target the same truncated instruction at the window's end.
    let binary = raw_flow_fixture(&[0x74, 0x04, 0x75, 0x02, 0x72, 0x00, 0x0f]);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--follow", "--json"]);
    assert_eq!(code, 1, "{stderr}");
    assert!(stderr.contains("error: 1 instruction(s) in the walk did not lift"), "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    let issues = doc["issues"].as_array().unwrap();
    assert_eq!(issues.len(), 1, "{doc}");
    assert_eq!(issues[0]["reason"], "decode_error");
}

#[test]
fn raw_pcode_follow_reports_unresolved_indirect_jump() {
    let binary = raw_flow_fixture(&[0xff, 0xe0]); // jmp rax
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--follow", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["complete"], false);
    assert!(doc["issues"].as_array().unwrap().iter().any(|issue| issue["reason"] == "indirect_branch"));
}

#[test]
fn raw_pcode_follow_preserves_arm_modes_and_mips_delay_slots() {
    for (name, target, expected, bytes, count) in [
        ("arm_interwork_le32", "0x2000041", "0x2000040", "0720", 2),
        ("bejoin_order_mips32_O2", "widen", "0x400130", "000417c3", 2),
    ] {
        let binary = fixture(name);
        let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, target, "--follow", "--json"]);
        assert_eq!(code, 0, "{stderr}");
        let doc: Json = serde_json::from_str(&stdout).unwrap();
        assert_eq!(doc["complete"], true, "{doc}");
        assert_eq!(doc["instruction_count"], count);
        assert_eq!(doc["start_hex"], expected);
        assert_eq!(doc["blocks"][0]["instructions"][0]["bytes"], bytes);
        if name.starts_with("bejoin") {
            assert_eq!(doc["blocks"][0]["instructions"][1]["bytes"], "03e0000800801825");
        }
    }
}

#[test]
fn raw_pcode_follow_preserves_out_of_range_edges_and_partial_errors() {
    let external = raw_flow_fixture(&[0xeb, 0x7f]);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &external, "probe", "--follow", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["complete"], false);
    assert!(doc["issues"].as_array().unwrap().iter().any(|issue| issue["reason"] == "out_of_range" && issue["target"].as_u64().is_some()));
    assert!(doc["blocks"][0]["successors"].as_array().unwrap().iter().any(|edge| edge["target"].as_u64().is_some()));

    let boundary = raw_flow_fixture(&[0x90, 0x0f]);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &boundary, "probe", "--follow", "--json"]);
    assert_eq!(code, 1, "{stderr} {stdout}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["complete"], false);
    assert_eq!(doc["instruction_count"], 1);
    assert!(doc["issues"].as_array().unwrap().iter().any(|issue| issue["reason"] == "decode_error"));
}

#[test]
fn raw_pcode_thumb_interior_pointer_uses_thumb_without_changing_other_isas() {
    let binary = fixture("arm_interwork_le32");
    let read = |args: &[&str]| {
        let (stdout, stderr, status) = run_kuna(args);
        assert_eq!(status, 0, "{stderr}\n{stdout}");
        serde_json::from_str::<Json>(&stdout).unwrap()
    };
    let odd = read(&["raw-pcode", &binary, "0x2000043", "--count", "1", "--json"]);
    let explicit = read(&["raw-pcode", &binary, "0x2000042", "--isa", "thumb", "--count", "1", "--json"]);
    assert_eq!(odd["start_hex"], "0x2000042");
    assert_eq!(odd["instructions"], explicit["instructions"]);
    assert_eq!(odd["instructions"][0]["bytes"], "7047");
    let graph = read(&["raw-pcode", &binary, "0x2000043", "--follow", "--json"]);
    assert_eq!(graph["start_hex"], "0x2000042");
    assert_eq!(graph["blocks"][0]["instructions"][0], odd["instructions"][0]);

    // The pointer's low bit supplies the mode even when inventory says ARM.
    let forced = read(&["raw-pcode", &binary, "0x2000001", "--count", "1", "--json"]);
    let explicit = read(&["raw-pcode", &binary, "0x2000000", "--isa", "thumb", "--count", "1", "--json"]);
    assert_eq!(forced["instructions"], explicit["instructions"]);
    assert_eq!(forced["instructions"][0]["bytes"], "0040");

    let (_, stderr, code) = run_kuna(&["raw-pcode", &binary, "0x2000043", "--isa", "arm", "--count", "1"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("conflicting with --isa arm"), "{stderr}");
    let x86 = fauxware();
    let doc = read(&["raw-pcode", &x86, "0x40071d", "--count", "1", "--json"]);
    assert_eq!(doc["start_hex"], "0x40071d");
    assert_eq!(doc["instructions"][0]["bytes"], "55");
}

#[test]
fn raw_pcode_follow_falls_through_predicated_instructions() {
    // cmp edi,esi; cmovg eax,edi; ret -- CMOVG skips its own body with a CBRANCH
    // to the next address, which is no reason to stop the walk.
    let binary = raw_flow_fixture(&[0x39, 0xf7, 0x0f, 0x4f, 0xc7, 0xc3]);
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &binary, "probe", "--follow", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: Json = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["kind"], "flow");
    assert_eq!(doc["complete"], true, "{doc}");
    assert_eq!(doc["instruction_count"], 3, "{doc}");
    let start = doc["start"].as_u64().unwrap();
    let cmov = doc["blocks"][0]["successors"].as_array().unwrap();
    let edges: Vec<_> = cmov.iter().map(|e| (e["kind"].as_str().unwrap(), e["target"].as_u64())).collect();
    assert_eq!(edges, [("branch", Some(start + 5)), ("fallthrough", Some(start + 5))]);
    assert_eq!(doc["blocks"][1]["instructions"][0]["mnemonic"], "RET");

    // cmp r0,r1; movle r0,r1; bx lr
    let arm = raw_flow_fixture_for(
        object::Architecture::Arm,
        &[0x01, 0x00, 0x50, 0xe1, 0x01, 0x00, 0xa0, 0xd1, 0x1e, 0xff, 0x2f, 0xe1],
    );
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", &arm, "probe", "--follow"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.starts_with("# 3 instructions reached from probe @ "), "{stdout}");
    assert!(stdout.contains("movle r0,r1"), "{stdout}");
    assert!(stdout.contains("bx lr"), "{stdout}");
    assert!(!stdout.contains("# incomplete"), "{stdout}");
}

#[test]
fn raw_pcode_help_and_data_targets_say_what_they_are() {
    let (stdout, stderr, code) = run_kuna(&["raw-pcode", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.is_empty(), "help belongs on stderr like every other block: {stdout}");
    assert!(stderr.starts_with("usage: kuna raw-pcode"), "{stderr}");
    let (_, stderr, code) = run_kuna(&["raw-pcode"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("error: raw-pcode requires <binary>"), "{stderr}");
    let (_, stderr, _) = run_kuna(&["raw-pcode", &fauxware(), "0x601040", "--bytes", "2", "--json"]);
    assert!(stderr.contains("note: 0x601040 is in a non-executable data section"), "{stderr}");
}

// --- two reachable readings of the same bytes (GH #1363) ----------------------

/// `followoverlap_x86_64` (generator beside it): one function per way two
/// reachable paths can read the same bytes as different instructions.
fn follow_overlap() -> String {
    fixture("followoverlap_x86_64")
}

/// The address and text of every row of a `--json` listing, in listing order.
fn listed(doc: &str) -> Vec<(u64, String)> {
    instructions(doc)
        .iter()
        .map(|r| (as_u64(field(r, "address")), as_str(field(r, "text")).to_owned()))
        .collect()
}

/// The reporter's bytes: a `je` into the fall-through `MOV`, whose own decode
/// then runs past the next row that path decodes. Both readings are listed, the
/// rows overlap, and the notes say the target was decoded from without also
/// saying it could not be.
#[test]
fn follow_lists_both_readings_of_a_branch_into_the_next_instruction() {
    let bin = follow_overlap();
    let doc = listing(&[&bin, "audit_overlap", "--follow", "--json"]);
    let rows = listed(&doc);
    for (addr, text) in [
        (0x401004, "MOV EAX,0x2ab8"),
        (0x401005, "MOV EAX,0x2a"),
        (0x401009, "ADD BL,AL"),
        (0x40100a, "RET"),
        (0x40100b, "RET"),
    ] {
        assert!(
            rows.contains(&(addr, text.to_owned())),
            "no {text} at 0x{addr:x}: {rows:?}"
        );
    }
    assert!(rows.windows(2).all(|w| w[0].0 < w[1].0), "rows out of address order: {rows:?}");
    let pairs = parse_doc(&doc);
    assert_eq!(field(&pairs, "truncated"), &Json::Bool(false));
    assert_eq!(as_str(field(&pairs, "end_hex")), "0x401010");
    let Json::Array(notes) = field(&pairs, "notes") else { panic!("notes must be an array") };
    let notes: Vec<&str> = notes.iter().map(as_str).collect();
    assert!(
        notes
            .iter()
            .any(|n| n.starts_with("--follow decoded from one address") && n.contains("0x401005")),
        "{notes:?}"
    );
    assert!(notes.iter().any(|n| n.contains("overlap") && n.contains("0x401005")), "{notes:?}");
    assert!(
        !notes.iter().any(|n| n.contains("no row starts") || n.contains("ran across")),
        "0x401005 has a row; nothing may say it does not: {notes:?}"
    );
}

/// The same admission one hop further (a target named by a row that itself
/// starts inside another), and backwards (a jump into an instruction already
/// listed, whose decode swallows the jump).
#[test]
fn follow_admits_a_chain_of_overlaps_and_a_jump_back_into_an_instruction() {
    let bin = follow_overlap();
    let chain = listed(&listing(&[&bin, "audit_chain", "--follow", "--json"]));
    for (addr, text) in [
        (0x401012, "MOV EAX,0x274a2a1"),
        (0x401015, "JZ 0x401019"),
        (0x401017, "MOV ECX,0x2ab8c3"),
        (0x401019, "MOV EAX,0x2a"),
    ] {
        assert!(chain.contains(&(addr, text.to_owned())), "no {text} at 0x{addr:x}: {chain:?}");
    }
    let back = listed(&listing(&[&bin, "audit_backward", "--follow", "--json"]));
    for (addr, text) in [
        (0x401020, "MOV EAX,0x90c30590"),
        (0x401022, "ADD EAX,0xfbeb90c3"),
        (0x401025, "JMP 0x401022"),
        (0x401027, "RET"),
    ] {
        assert!(back.contains(&(addr, text.to_owned())), "no {text} at 0x{addr:x}: {back:?}");
    }
}

/// `overlapbranch_i386`'s `75 01` hop over a junk `e8`: the re-anchored run used
/// to stop, with nothing said, the first time it crossed a row of the junk
/// reading. The real stream is listed to its `RET`, beside the junk one.
#[test]
fn a_followed_run_is_not_cut_short_by_another_reading_of_its_bytes() {
    let bin = fixture("overlapbranch_i386");
    let rows = listed(&listing(&[&bin, "0x804881c", "--addr", "--follow", "--json"]));
    for (addr, text) in [
        (0x8048824, "CALL 0xb0c48ac"),
        (0x8048825, "ADD EAX,0x7"),
        (0x8048828, "ADD EAX,dword ptr [EAX]"),
        (0x804882a, "ADD EAX,0xd98ddd00"),
        (0x804882f, "RET"),
    ] {
        assert!(rows.contains(&(addr, text.to_owned())), "no {text} at 0x{addr:x}: {rows:?}");
    }
}

/// A target `--follow` still cannot start a row at is named with the reason
/// that is actually true, and the move that fixes that reason.
#[test]
fn follow_says_why_a_target_still_has_no_row() {
    let bin = follow_overlap();
    let note_for = |argv: &[&str], target: &str| {
        let (_, notes) = rendered(&[&[bin.as_str()][..], argv, &["--follow"]].concat());
        assert!(
            !notes.iter().any(|n| n.contains("decoded from") && n.contains(target)),
            "{target} has no row, so nothing may say it was decoded from: {notes:?}"
        );
        notes
            .into_iter()
            .find(|n| n.contains("no row starts") && n.contains(target))
            .unwrap_or_else(|| panic!("no note names {target}"))
    };
    let undecodable = note_for(&["audit_undecodable"], "0x401033");
    assert!(undecodable.contains("do not decode"), "{undecodable}");
    let unreached = note_for(&["audit_unreached"], "0x401044");
    assert!(unreached.contains("no flow from 0x401040 reaches"), "{unreached}");
    let window = note_for(&["0x401000", "--addr", "--bytes", "5"], "0x401005");
    assert!(window.contains("window ends at 0x401005"), "{window}");
    assert!(window.contains("--bytes"), "{window}");
    let budget = note_for(&["0x401000", "--addr", "--count", "3"], "0x401005");
    assert!(budget.contains("--count 3"), "{budget}");
}

/// `--count` spends its budget on the lowest addresses, however late flow
/// reaches them: the fall-through path decodes four rows before the branch
/// target is taken from the worklist, and the target is still the fourth row.
/// An explicit `--count` that ran out is the length asked for, not a truncation,
/// with or without `--follow`; and `end` is the furthest any row reaches.
#[test]
fn a_counted_follow_lists_its_lowest_rows_and_is_not_truncated() {
    let bin = follow_overlap();
    let doc = listing(&[&bin, "0x401000", "--addr", "--count", "4", "--follow", "--json"]);
    let rows: Vec<u64> = listed(&doc).into_iter().map(|r| r.0).collect();
    assert_eq!(rows, vec![0x401000, 0x401002, 0x401004, 0x401005]);
    let pairs = parse_doc(&doc);
    assert_eq!(as_str(field(&pairs, "end_hex")), "0x40100a", "0x401005 is 5 bytes long");
    assert_eq!(as_u64(field(&pairs, "bytes")), 10);
    for follow in [&[][..], &["--follow"][..]] {
        let fauxware = fauxware();
        let argv = [&[fauxware.as_str(), "main", "--count", "9", "--json"][..], follow].concat();
        let doc = listing(&argv);
        assert_eq!(field(&parse_doc(&doc), "truncated"), &Json::Bool(false), "{follow:?}");
    }
}

/// `audit_staircase`: forty levels, each branching to the one below it, entered
/// from the top. Every level flow reaches displaces the rows of the one above it
/// from the lowest `--count` rows, so keeping those rows exactly would cost a
/// decode per level, and a stair as long as the window would make a short
/// listing decode all of it. The walk spends a few decodes per row it may
/// print: a small `--count` stops short of the bottom and says the target there
/// is past that budget, not unreached, and a `--count` the staircase fits in
/// lists it.
#[test]
fn a_counted_follow_bounds_its_decodes_and_says_so() {
    let bin = follow_overlap();
    let (_, notes) = rendered(&[&bin, "0x401050", "--addr", "--count", "4", "--follow"]);
    let budget = notes
        .iter()
        .find(|n| n.contains("no row starts") && n.contains("0x401058"))
        .unwrap_or_else(|| panic!("no note names 0x401058: {notes:?}"));
    assert!(budget.contains("--count 4 budget"), "{budget}");
    let doc = listing(&[&bin, "0x401050", "--addr", "--count", "64", "--follow", "--json"]);
    let rows = listed(&doc);
    assert!(rows.contains(&(0x401058, "MOV EAX,0x2a".to_owned())), "{rows:?}");
}
