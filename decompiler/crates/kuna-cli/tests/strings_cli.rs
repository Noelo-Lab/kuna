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
    assert!(out.starts_with("# 13 strings in "), "a header naming the query:\n{out}");
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
