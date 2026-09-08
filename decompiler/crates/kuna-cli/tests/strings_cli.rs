//! Gate for `kuna strings` — the string inventory five testers on four crackmes
//! asked for and fell back to `strings(1)` for.
//!
//! Two layers, because the command lands before the integrator wires its
//! dispatch arm in `main.rs` (the `disassemble_cli.rs` precedent):
//!
//! * **In-process** — the module is pulled in by `#[path]` (with the crate
//!   modules it uses) and driven through its own `parse_args` + `query`, so the
//!   whole command *except* the dispatch arm is under test from the day the file
//!   lands. `query` returns the rendered text instead of writing it, which is
//!   also what lets these tests assert on exact columns.
//! * **End to end** — the same invocations through the built `kuna` binary.
//!   Until `main.rs` routes `"strings"`, those are a visible skip, never a false
//!   green.
//!
//! Two cases are load-bearing.
//!
//! [`the_acceptance_probe`] is the promoted probe: `kuna strings <binary>` used
//! to exit 2 with `unknown subcommand "strings"`.
//!
//! [`a_wide_string_is_a_one_character_string_at_byte_width`] is the recorded
//! defect `--encoding` exists for: a UTF-16LE literal read at 1-byte width stops
//! at the NUL after its first character, which is why the decompiler renders
//! `LoadLibraryW("n")` for `L"ntdll.dll"`. It runs on a synthetic ELF built
//! in-process, so it needs no vendored fixture.
//!
//! [`a_utf8_prompt_keeps_its_start_address`] is the other one: a literal whose
//! first characters are multi-byte is reported from the byte AFTER its last
//! sequence, which is not an address anything in the image refers to — so the
//! row also arrives with no references and no owning function.
//!
//! ## `.sla` precondition
//!
//! The reference walk bootstraps the architecture, which needs the built `x86`
//! `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! command cannot load; the test prints that and returns early — a specs-less CI
//! is a visible skip. The scan itself needs no `.sla`, which is what the
//! `--no-xrefs` cases run without.

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
#[path = "../src/assertdecl.rs"]
mod assertdecl;

#[path = "../src/funcdecl.rs"]
mod funcdecl;
#[allow(dead_code)]
#[path = "../src/optname.rs"]
mod optname;
#[allow(dead_code)]
#[path = "../src/decompile.rs"]
mod decompile;
#[allow(dead_code)]
#[path = "../src/jobs.rs"]
mod jobs;
#[allow(dead_code)]
#[path = "../src/decompile_all.rs"]
mod decompile_all;
#[allow(dead_code)]
#[path = "../src/strings.rs"]
mod strings;

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

/// The vendored non-stripped `fauxware`: `.rodata` prompts referenced from
/// `main`, which is exactly the string→its-user hop this command exists for.
fn fauxware() -> String {
    fixture("fauxware")
}

/// The vendored `nametable_x86_64`: a `.rodata` holding a length-prefixed name
/// table (no NUL anywhere) plus one ordinary NUL-terminated literal, which is the
/// pair `--termination` separates.
fn nametable() -> String {
    fixture("nametable_x86_64")
}

/// The vendored `utf8prompt_x86_64`: `prompt_user` loads a prompt at `0x101000`
/// whose first characters are multi-byte UTF-8, then an ASCII-only control
/// literal at `0x101038`.
fn utf8prompt() -> String {
    fixture("utf8prompt_x86_64")
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
    let args = strings::parse_args(&argv).unwrap_or_else(|e| panic!("{argv:?}: {e}"));
    match strings::query(&args) {
        Ok(text) => Some(text),
        Err(e) if is_specs_skip(&e) => {
            eprintln!("skipping: {e}");
            None
        }
        Err(e) => panic!("kuna strings {argv:?} failed: {e}"),
    }
}

/// The error a rejected invocation reports, whichever half rejected it: the
/// parse (a `2` on the command line) or the query (a `1` on the work).
enum Refusal {
    Usage(String),
    Query(String),
}

fn refusal(argv: &[&str]) -> Refusal {
    let argv: Vec<String> = argv.iter().map(|s| (*s).to_string()).collect();
    match strings::parse_args(&argv) {
        Err(e) => Refusal::Usage(e),
        Ok(args) => match strings::query(&args) {
            Err(e) => Refusal::Query(e),
            Ok(text) => panic!("{argv:?} was expected to be refused, got:\n{text}"),
        },
    }
}

/// The data rows of the text surface (everything after the `#` header).
fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| l.split('\t').map(str::to_string).collect())
        .collect()
}

/// The row at `address_hex`, by its first column.
fn row_at<'a>(rows: &'a [Vec<String>], addr: &str) -> Option<&'a Vec<String>> {
    rows.iter().find(|r| r[0] == addr)
}

// --- the inventory -----------------------------------------------------------

/// Every row the analyzer marked up is reported, at its address, with its
/// section — the columns `strings(1)` cannot produce.
#[test]
fn the_inventory_carries_addresses_and_sections() {
    let Some(out) = listing(&[&fauxware(), "--no-xrefs"]) else { return };
    let rows = rows(&out);
    let prompt = row_at(&rows, "0x400915").expect("\"Username: \" @ 0x400915");
    assert_eq!(prompt[1], "ascii");
    assert_eq!(prompt[2], "10", "ten visible characters");
    assert_eq!(prompt[3], ".rodata");
    assert_eq!(prompt[6], "Username: ");
    assert!(row_at(&rows, "0x4008d0").is_some(), "the backdoor password must be listed");
    assert!(out.starts_with("# 14 strings in "), "a header naming the query:\n{out}");
    assert!(out.contains("termination any"), "and the ending policy it ran under:\n{out}");
}

/// The reason to ask kuna rather than `strings(1)`: the row names the function
/// that uses the string.
#[test]
fn a_row_names_the_functions_that_reference_it() {
    let Some(out) = listing(&[&fauxware(), "--filter", "^Username: $"]) else { return };
    let rows = rows(&out);
    let prompt = row_at(&rows, "0x400915").expect("the prompt survives its own filter");
    assert_eq!(prompt[4], "1", "referenced once");
    assert_eq!(prompt[5], "main", "and the reference is from main");
}

/// The `--json` document carries every documented field, on every row, with each
/// address spelled both ways.
#[test]
fn the_json_document_has_the_house_shape() {
    let Some(out) = listing(&[&fauxware(), "--json", "--filter", "^Username: $"]) else { return };
    let parsed = jsonfmt::parse(&out).expect("the document parses as JSON");
    let jsonfmt::Json::Object(root) = &parsed else { panic!("the document is an object") };
    let key = |k: &str| root.iter().find(|(name, _)| name == k).map(|(_, v)| v);
    for k in [
        "binary",
        "encoding",
        "min_length",
        "filter",
        "section",
        "termination",
        "scanned",
        "xrefs",
        "count",
        "strings",
    ] {
        assert!(key(k).is_some(), "the document must carry {k:?}:\n{out}");
    }
    let Some(jsonfmt::Json::Array(items)) = key("strings") else { panic!("strings is an array") };
    assert_eq!(items.len(), 1, "one match:\n{out}");
    let jsonfmt::Json::Object(row) = &items[0] else { panic!("a row is an object") };
    let field = |k: &str| row.iter().find(|(name, _)| name == k).map(|(_, v)| v);
    for k in [
        "address",
        "address_hex",
        "text",
        "length",
        "byte_length",
        "nul_terminated",
        "encoding",
        "section",
        "xrefs_count",
        "functions",
    ] {
        assert!(field(k).is_some(), "every row must carry {k:?}:\n{out}");
    }
    assert_eq!(field("address"), Some(&jsonfmt::Json::Number("4196629".into())));
    assert_eq!(field("address_hex"), Some(&jsonfmt::Json::Str("0x400915".into())));
    assert_eq!(field("text"), Some(&jsonfmt::Json::Str("Username: ".into())));
    let Some(jsonfmt::Json::Array(functions)) = field("functions") else {
        panic!("functions is an array")
    };
    assert_eq!(functions.len(), 1, "one referencing function:\n{out}");
    let jsonfmt::Json::Object(f) = &functions[0] else { panic!("a function is an object") };
    assert!(
        f.contains(&("name".to_string(), jsonfmt::Json::Str("main".into()))),
        "named, with both address forms:\n{out}"
    );
    assert!(f.iter().any(|(k, _)| k == "address") && f.iter().any(|(k, _)| k == "address_hex"));
}

// --- the filters -------------------------------------------------------------

/// `--min-length` is the analyzer's own `minStringLength`, so lowering it admits
/// shorter runs and raising it drops them.
#[test]
fn min_length_moves_the_analyzer_threshold() {
    let Some(loose) = listing(&[&fauxware(), "--no-xrefs", "--min-length", "3"]) else { return };
    let Some(tight) = listing(&[&fauxware(), "--no-xrefs", "--min-length", "20"]) else { return };
    assert!(rows(&loose).len() > rows(&tight).len(), "a lower minimum must admit more");
    assert!(
        rows(&tight).iter().all(|r| r[2].parse::<usize>().unwrap() >= 20),
        "no row shorter than the minimum survives"
    );
}

/// The recorded defect `--termination` exists for: a length-prefixed name table
/// holds no NUL, so an inventory that takes only NUL-ended runs answers zero for
/// a region `strings(1)` reads in full.
#[test]
fn a_length_prefixed_name_table_is_reported_by_default() {
    let bin = nametable();
    let mut argv = vec![&bin[..], "--no-xrefs", "--section", ".rodata", "--min-length", "4"];
    let Some(out) = listing(&argv) else { return };
    let texts: Vec<String> = rows(&out).iter().map(|r| r[6].clone()).collect();
    assert_eq!(
        texts,
        vec!["out.js", "_0x8ec6b3", "_0x3f5c60", "Correct serial!"],
        "the whole table, plus the ordinary literal:\n{out}"
    );

    // The pass-faithful policy is the behaviour this replaced: the table is gone
    // and only the C literal survives.
    argv.extend_from_slice(&["--termination", "nul"]);
    let Some(out) = listing(&argv) else { return };
    let strict = rows(&out);
    assert_eq!(strict.len(), 1, "only the NUL-ended literal:\n{out}");
    assert_eq!(strict[0][6], "Correct serial!");
}

/// An unterminated row is reported as one: it says so, and its extent stops at
/// its last visible byte rather than claiming a terminator it does not have.
#[test]
fn an_unterminated_row_declares_itself() {
    let Some(out) = listing(&[
        &nametable(),
        "--no-xrefs",
        "--json",
        "--min-length",
        "4",
        "--filter",
        "^out.js$",
    ]) else {
        return;
    };
    let parsed = jsonfmt::parse(&out).expect("the document parses as JSON");
    let jsonfmt::Json::Object(root) = &parsed else { panic!("the document is an object") };
    let key = |k: &str| root.iter().find(|(name, _)| name == k).map(|(_, v)| v);
    assert_eq!(key("termination"), Some(&jsonfmt::Json::Str("any".into())), "echoed:\n{out}");
    let Some(jsonfmt::Json::Array(items)) = key("strings") else { panic!("strings is an array") };
    assert_eq!(items.len(), 1, "one match:\n{out}");
    let jsonfmt::Json::Object(row) = &items[0] else { panic!("a row is an object") };
    let field = |k: &str| row.iter().find(|(name, _)| name == k).map(|(_, v)| v);
    assert_eq!(field("nul_terminated"), Some(&jsonfmt::Json::Bool(false)));
    assert_eq!(field("length"), Some(&jsonfmt::Json::Number("6".into())));
    assert_eq!(
        field("byte_length"),
        Some(&jsonfmt::Json::Number("6".into())),
        "no terminator to count:\n{out}"
    );
}

/// Relaxing the ending only adds rows: every NUL-terminated literal keeps its
/// address, its text and its terminator-inclusive extent.
#[test]
fn the_strict_policy_is_a_subset_of_the_default() {
    let Some(relaxed) = listing(&[&fauxware(), "--no-xrefs", "--json"]) else { return };
    let Some(strict) = listing(&[&fauxware(), "--no-xrefs", "--json", "--termination", "nul"])
    else {
        return;
    };
    let rows = |doc: &str| -> Vec<jsonfmt::Json> {
        let jsonfmt::Json::Object(root) = jsonfmt::parse(doc).expect("JSON") else {
            panic!("object")
        };
        let Some((_, jsonfmt::Json::Array(items))) =
            root.into_iter().find(|(name, _)| name == "strings")
        else {
            panic!("strings is an array")
        };
        items
    };
    let (relaxed, strict) = (rows(&relaxed), rows(&strict));
    assert_eq!(strict.len(), 13, "the analyzer's own inventory of fauxware");
    assert_eq!(relaxed.len(), 14, "plus the one unterminated printable run");
    for row in &strict {
        assert!(relaxed.contains(row), "the relaxed ending dropped a row: {row:?}");
    }
}

/// `--section` narrows the scan to one section, by name, with the leading dot
/// optional.
#[test]
fn section_narrows_the_scan() {
    let Some(out) = listing(&[&fauxware(), "--no-xrefs", "--section", "rodata"]) else { return };
    let rows = rows(&out);
    assert!(!rows.is_empty(), "fauxware has .rodata strings");
    assert!(rows.iter().all(|r| r[3] == ".rodata"), "only .rodata rows:\n{out}");
    assert!(row_at(&rows, "0x400238").is_none(), "the .interp string is out of scope");
}

/// An unknown `--section` is a question that cannot be answered: a failed query
/// naming the sections that exist, not a silent empty answer.
#[test]
fn an_unknown_section_reports_what_is_there() {
    match refusal(&[&fauxware(), "--section", "nope"]) {
        Refusal::Query(e) => {
            assert!(e.contains("no section named"), "{e}");
            assert!(e.contains(".rodata"), "the reachable sections are named: {e}");
        }
        Refusal::Usage(e) => panic!("an unknown section is a failed query, not a usage error: {e}"),
    }
}

/// `--filter` is a regex over the text, not a substring test.
#[test]
fn filter_is_a_regex() {
    let cases: [(&str, usize); 4] = [
        ("^Go away!$", 1),
        ("Username|Password", 2),
        ("GLIBC_[0-9.]+$", 1),
        ("zzz-no-such-string", 0),
    ];
    for (pattern, want) in cases {
        let Some(out) = listing(&[&fauxware(), "--no-xrefs", "--filter", pattern]) else { return };
        assert_eq!(rows(&out).len(), want, "--filter {pattern:?}:\n{out}");
    }
}

/// A pattern the matcher does not implement is refused on the command line, not
/// reinterpreted into a different one.
#[test]
fn a_malformed_filter_is_a_usage_error() {
    for bad in ["(unclosed", "[a-", "*leading", "a\\b", "(?=x)"] {
        match refusal(&[&fauxware(), "--filter", bad]) {
            Refusal::Usage(e) => assert!(e.contains("--filter"), "{bad:?}: {e}"),
            Refusal::Query(e) => panic!("{bad:?} must be refused at parse time, not run: {e}"),
        }
    }
}

// --- the UTF-16 width --------------------------------------------------------

/// Build a minimal ELF64 holding one `SHF_ALLOC` `.rodata` section, so the scan
/// has a real image to walk without a vendored fixture. `rodata` is mapped at
/// `vma`; the file needs no program headers because nothing here is executed.
fn synthetic_elf(vma: u64, rodata: &[u8]) -> Vec<u8> {
    const EHDR: usize = 64;
    const SHDR: usize = 64;
    let shstrtab = b"\0.rodata\0.shstrtab\0";
    let rodata_off = EHDR;
    let shstrtab_off = rodata_off + rodata.len();
    let shoff = shstrtab_off + shstrtab.len();

    let mut out = Vec::new();
    out.extend_from_slice(&[0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    out.extend_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
    out.extend_from_slice(&62u16.to_le_bytes()); // e_machine = x86-64
    out.extend_from_slice(&1u32.to_le_bytes()); // e_version
    out.extend_from_slice(&0u64.to_le_bytes()); // e_entry
    out.extend_from_slice(&0u64.to_le_bytes()); // e_phoff
    out.extend_from_slice(&(shoff as u64).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // e_flags
    out.extend_from_slice(&(EHDR as u16).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // e_phentsize
    out.extend_from_slice(&0u16.to_le_bytes()); // e_phnum
    out.extend_from_slice(&(SHDR as u16).to_le_bytes());
    out.extend_from_slice(&3u16.to_le_bytes()); // e_shnum
    out.extend_from_slice(&2u16.to_le_bytes()); // e_shstrndx
    assert_eq!(out.len(), EHDR);
    out.extend_from_slice(rodata);
    out.extend_from_slice(shstrtab);

    let mut shdr = |name: u32, kind: u32, flags: u64, addr: u64, off: usize, size: usize| {
        out.extend_from_slice(&name.to_le_bytes());
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&addr.to_le_bytes());
        out.extend_from_slice(&(off as u64).to_le_bytes());
        out.extend_from_slice(&(size as u64).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes()); // sh_link
        out.extend_from_slice(&0u32.to_le_bytes()); // sh_info
        out.extend_from_slice(&1u64.to_le_bytes()); // sh_addralign
        out.extend_from_slice(&0u64.to_le_bytes()); // sh_entsize
    };
    shdr(0, 0, 0, 0, 0, 0); // SHT_NULL
    shdr(1, 1, 2, vma, rodata_off, rodata.len()); // .rodata: PROGBITS | SHF_ALLOC
    shdr(9, 3, 0, 0, shstrtab_off, shstrtab.len()); // .shstrtab: STRTAB
    out
}

/// The UTF-16LE encoding of `text`, NUL-terminated.
fn wide(text: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for ch in text.chars() {
        out.push(ch as u8);
        out.push(0);
    }
    out.extend_from_slice(&[0, 0]);
    out
}

/// Write `bytes` under a per-test temp path and return it.
fn temp_binary(tag: &str, bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!("kuna-strings-{}-{tag}.elf", std::process::id()));
    std::fs::write(&path, bytes).expect("write the synthetic image");
    path
}

/// The recorded defect, at the inventory level: `L"ntdll.dll"` is a
/// one-character string at 1-byte width and the whole literal at 2-byte width.
#[test]
fn a_wide_string_is_a_one_character_string_at_byte_width() {
    let vma = 0x400000u64;
    let path = temp_binary("wide", &synthetic_elf(vma, &wide("ntdll.dll")));
    let path = path.to_str().unwrap();

    // 1-byte width, the analyzer's own minimum: the literal is invisible.
    let Some(ascii) = listing(&[path, "--no-xrefs", "--encoding", "ascii"]) else { return };
    assert!(rows(&ascii).is_empty(), "a wide literal has no 5-char ASCII run:\n{ascii}");

    // 1-byte width, minimum 1: exactly the `LoadLibraryW("n")` rendering.
    let Some(truncated) =
        listing(&[path, "--no-xrefs", "--encoding", "ascii", "--min-length", "1"])
    else {
        return;
    };
    let byte_rows = rows(&truncated);
    let first = row_at(&byte_rows, "0x400000").expect("a row at the literal's address");
    assert_eq!(first[6], "n", "1-byte width stops at the NUL after the first character");
    assert_eq!(first[2], "1");

    // 2-byte width: the whole literal.
    let Some(text) = listing(&[path, "--no-xrefs", "--encoding", "utf16"]) else { return };
    let wide_rows = rows(&text);
    let row = row_at(&wide_rows, "0x400000").expect("a row at the literal's address");
    assert_eq!(row[1], "utf16");
    assert_eq!(row[2], "9", "nine code units");
    assert_eq!(row[3], ".rodata");
    assert_eq!(row[6], "ntdll.dll");

    let _ = std::fs::remove_file(path);
}

/// `--encoding all` reports both widths, each labelled, from one scan.
#[test]
fn encoding_all_reports_both_widths() {
    // A trailing pad keeps the wide literal on an even address.
    let mut rodata = b"Username: \0\0".to_vec();
    let wide_at = rodata.len() as u64;
    rodata.extend_from_slice(&wide("ntdll.dll"));
    let vma = 0x400000u64;
    let path = temp_binary("both", &synthetic_elf(vma, &rodata));
    let path = path.to_str().unwrap();

    let Some(out) = listing(&[path, "--no-xrefs", "--encoding", "all"]) else { return };
    let rows = rows(&out);
    assert_eq!(row_at(&rows, "0x400000").expect("the ASCII row")[1], "ascii");
    let wide = row_at(&rows, &format!("0x{:x}", vma + wide_at)).expect("the UTF-16 row");
    assert_eq!(wide[1], "utf16");
    assert_eq!(wide[6], "ntdll.dll");

    let _ = std::fs::remove_file(path);
}

/// The recorded defect: the ASCII recognizer ends a run at every byte `>= 0x80`,
/// so a prompt opening with a kaomoji is reported from the byte after its LAST
/// multi-byte sequence — and that address, unlike the literal's own, is not one
/// the image refers to, which is why the row also arrives ownerless.
#[test]
fn a_utf8_prompt_keeps_its_start_address() {
    // 1-byte width: the row starts twelve bytes into the literal, holds 43 of
    // its 50 characters, and nothing references it.
    let Some(ascii) = listing(&[&utf8prompt(), "--filter", "magical"]) else { return };
    let ascii_rows = rows(&ascii);
    let truncated = row_at(&ascii_rows, "0x10100c").expect("the ASCII reading loses the prefix");
    assert_eq!(truncated[1], "ascii");
    assert_eq!(truncated[2], "43");
    assert_eq!(truncated[4], "0", "no reference lands on a mid-literal address");
    assert_eq!(truncated[5], "-", "so no function owns it");

    // UTF-8: the whole literal, at the address `prompt_user` loads.
    let Some(utf8) = listing(&[&utf8prompt(), "--encoding", "utf8", "--filter", "magical"])
    else {
        return;
    };
    let utf8_rows = rows(&utf8);
    assert!(row_at(&utf8_rows, "0x10100c").is_none(), "the truncated row is gone:\n{utf8}");
    let whole =
        row_at(&utf8_rows, "0x101000").expect("the literal is reported at its own address");
    assert_eq!(whole[1], "utf8");
    assert_eq!(whole[2], "50", "fifty characters in fifty-five bytes");
    assert_eq!(whole[4], "1", "the LEA the reference walk already found");
    assert_eq!(whole[5], "prompt_user");
    assert!(whole[6].starts_with('\u{ff3f}'), "the fullwidth low line leads it: {:?}", whole[6]);
}

/// A row with no multi-byte content reads identically under both readings of the
/// 1-byte width — same address, same width label, same text — so turning UTF-8
/// on costs an ASCII-only image nothing.
#[test]
fn an_ascii_only_row_is_unmoved_by_the_utf8_reading() {
    let Some(ascii) = listing(&[&utf8prompt(), "--no-xrefs", "--filter", "control literal"])
    else {
        return;
    };
    let Some(utf8) = listing(&[
        &utf8prompt(),
        "--no-xrefs",
        "--encoding",
        "utf8",
        "--filter",
        "control literal",
    ]) else {
        return;
    };
    assert_eq!(rows(&ascii), rows(&utf8), "the control literal must not move");
    let ascii_rows = rows(&ascii);
    let control = row_at(&ascii_rows, "0x101038").expect("the ASCII-only control literal");
    assert_eq!(control[1], "ascii");
    assert_eq!(control[6], "plain ascii control literal");

    // And the whole fauxware inventory, which holds no multi-byte sequence at
    // all, is byte-identical under `ascii` and `utf8`.
    let Some(a) = listing(&[&fauxware(), "--no-xrefs"]) else { return };
    let Some(u) = listing(&[&fauxware(), "--no-xrefs", "--encoding", "utf8"]) else { return };
    assert_eq!(rows(&a), rows(&u), "a pure-ASCII image reads the same either way");
}

/// An image with no usable section table — a UPX-packed ELF keeps its program
/// headers and nothing else — is scanned by segment rather than answered empty.
#[test]
fn a_section_less_image_falls_back_to_its_segments() {
    let mut image = synthetic_elf(0x400000, b"Username: \0");
    // Erase the section header table: e_shoff/e_shnum/e_shstrndx to zero.
    image[0x28..0x30].copy_from_slice(&0u64.to_le_bytes());
    image[0x3c..0x3e].copy_from_slice(&0u16.to_le_bytes());
    image[0x3e..0x40].copy_from_slice(&0u16.to_le_bytes());
    let path = temp_binary("segments", &image);
    let path = path.to_str().unwrap();

    let Some(out) = listing(&[path, "--no-xrefs"]) else { return };
    assert!(out.contains("scanned by segments"), "the header says which set was walked:\n{out}");
    let _ = std::fs::remove_file(path);
}

// --- the command line --------------------------------------------------------

/// The command line is refused where it is wrong, never quietly reinterpreted.
#[test]
fn the_command_line_contract() {
    for bad in [
        vec![],
        vec![&fauxware()[..], "--encoding", "ebcdic"],
        vec![&fauxware()[..], "--termination", "maybe"],
        vec![&fauxware()[..], "--min-length", "0"],
        vec![&fauxware()[..], "--min-length", "many"],
        vec![&fauxware()[..], "--nonsense"],
        vec![&fauxware()[..], "extra-operand"],
    ] {
        let argv: Vec<&str> = bad.iter().map(|s| &s[..]).collect();
        assert!(
            matches!(refusal(&argv), Refusal::Usage(_)),
            "{argv:?} must be a command-line error"
        );
    }
    // An unreadable binary is a failed query, not a malformed command line.
    assert!(matches!(refusal(&["/no/such/binary"]), Refusal::Query(_)));
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

/// `strings.rs` is dispatch-free until `main.rs` routes `"strings"` to it. Until
/// then the end-to-end tests are a visible skip rather than a false green; every
/// test above still covers the command itself.
fn dispatch_wired() -> bool {
    let (_, stderr, _) = run_kuna(&["strings"]);
    let wired = !stderr.contains("unknown subcommand");
    if !wired {
        eprintln!("strings_cli: skipping (main.rs does not dispatch `strings` yet)");
    }
    wired
}

/// The promoted acceptance probe, as the RE loop runs it: through the binary,
/// exit 0, the literals on stdout.
#[test]
fn the_acceptance_probe() {
    if !dispatch_wired() {
        return;
    }
    let (stdout, stderr, code) = run_kuna(&["strings", &fauxware()]);
    if is_specs_skip(&stderr) {
        eprintln!("skipping: {stderr}");
        return;
    }
    assert_eq!(code, 0, "kuna strings must exit 0, not {code}: {stderr}");
    assert!(stdout.contains("Username: "), "the .rodata prompts must be listed:\n{stdout}");
    assert!(stdout.contains("main"), "and the function that uses them:\n{stdout}");
}

/// `--json` on stdout is a whole JSON document, which is what the probe asserts.
#[test]
fn the_cli_json_is_a_document() {
    if !dispatch_wired() {
        return;
    }
    let (stdout, stderr, code) = run_kuna(&["strings", &fauxware(), "--json", "--no-xrefs"]);
    if is_specs_skip(&stderr) {
        eprintln!("skipping: {stderr}");
        return;
    }
    assert_eq!(code, 0, "{stderr}");
    assert!(jsonfmt::parse(&stdout).is_some(), "stdout must parse as JSON:\n{stdout}");
}

/// The exit codes the reference documents: `2` for a malformed command line,
/// `1` for a query that cannot be answered.
#[test]
fn the_cli_exit_codes() {
    if !dispatch_wired() {
        return;
    }
    let (_, stderr, code) = run_kuna(&["strings"]);
    assert_eq!(code, 2, "no binary is a usage error");
    assert!(stderr.contains("usage: kuna strings"), "{stderr}");

    let (_, _, code) = run_kuna(&["strings", &fauxware(), "--encoding", "ebcdic"]);
    assert_eq!(code, 2, "an unknown encoding is a usage error");

    let (_, stderr, code) = run_kuna(&["strings", "/no/such/binary"]);
    assert_eq!(code, 1, "an unreadable binary is a failed query, not a usage error");
    assert!(stderr.starts_with("error: "), "{stderr}");
}

// --- a fat / universal Mach-O ------------------------------------------------

/// The recorded defect: `strings` re-parses the image itself, and that raw parse
/// used to skip the Mach-O fat-header peel the engine load applies — so a
/// UNIVERSAL binary answered `could not parse <bin>: Unsupported file format`
/// while `kuna functions` on the same file returned an inventory
/// (crackmes.one 5ab77f5633c5d40ad448c29b). The scan needs no `.sla`, so this
/// runs with `--no-xrefs` and is never a skip.
#[test]
fn a_universal_macho_is_scanned_not_rejected() {
    let out = listing(&[&fixture("macho_fat"), "--no-xrefs", "--min-length", "2"])
        .expect("the scan needs no .sla");
    let rows = rows(&out);
    let cstring = row_at(&rows, "0x1000005ee").expect("the x86-64 slice's format string");
    assert_eq!(cstring[3], "__cstring");
    assert!(cstring[6].starts_with("%d"), "the literal itself: {cstring:?}");
    assert_eq!(rows.len(), 8, "the whole x86-64 slice, no more:\n{out}");
    assert!(
        rows.iter().all(|r| r[0].starts_with("0x1000")),
        "every row is in the x86-64 slice, none from the arm64 one:\n{out}"
    );
}

/// `--slice` is LIVE on this path, not merely non-fatal: it was parsed and then
/// threaded only into the reference walk, never into the scan's own parse, so
/// `--slice <arch>` failed with the identical message. The two slices carry the
/// same literal at different addresses, which is what proves the override chose.
#[test]
fn the_slice_override_picks_which_slice_is_scanned() {
    let scan = |slice: &str| {
        let out = listing(&[
            &fixture("macho_fat"),
            "--no-xrefs",
            "--min-length",
            "2",
            "--slice",
            slice,
        ])
        .expect("the scan needs no .sla");
        rows(&out)
    };
    let x86 = scan("x86_64");
    let arm = scan("arm64");
    assert!(row_at(&x86, "0x1000005ee").is_some(), "the x86-64 literal: {x86:?}");
    assert!(row_at(&arm, "0x1000005d0").is_some(), "the arm64 literal: {arm:?}");
    assert!(row_at(&arm, "0x1000005ee").is_none(), "the arm64 scan is not the x86-64 one: {arm:?}");
    assert_ne!(x86.len(), arm.len(), "two slices, two inventories");
}
