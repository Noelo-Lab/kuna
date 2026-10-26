//! End-to-end gate for the Rust `&str`-record string segmentation
//! (`fatstrrecords`, the analysis-side half of `option fatstr` —
//! `docs/multilang.md` W4).
//!
//! The proof, on a REAL rustc PIE built at test time: rustc packs its
//! unterminated literals back to back in `.rodata`, so the NUL-run scan reads
//! the format pieces of a `println!` as ONE glued string. The `&str` records
//! in `.data.rel.ro` state the exact lengths, and with `fatstr` on (the
//! default; the image detects as rustc) the committed data symbol at the
//! literal's address spans exactly the literal — with `--option fatstr off`
//! the glued NUL-run symbol is what remains.
//!
//! The hermetic record-matcher logic is pinned by the unit tests in
//! `kuna-analysis/src/analyzers/strings/kuna_strrecords.rs` (no rustc
//! needed); this gate is the detection-driven path over a genuine PIE's
//! dynamic relocations. Guarded on `rustc` being on PATH; an installed
//! compiler must build the fixture successfully.

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use object::read::{Object, ObjectSection};

/// The first entry of the fixture's runtime-indexed table — 21 bytes, no NUL.
const WITNESS: &[u8] = b"kuna fatstr witness, ";

/// A RUNTIME-INDEXED `&str` table, not a `println!` piece: rustc 1.99 lowered
/// the format machinery so a literal piece no longer materializes a
/// `(ptr, len)` record anywhere in the image (no relocation, no pointer word —
/// measured on 1.99.1 vs 1.90.0), and a bare `static &str` is const-folded
/// into code at `-O`. An index only the arguments decide forces the table —
/// three records in `.data.rel.ro` under every rustc this test has met — which
/// is also the real-world shape the record pass most often feeds on.
const MAIN_RS: &str = r#"static TABLE: [&str; 3] =
    ["kuna fatstr witness, ", "kuna second piece", "kuna third piece"];
fn main() {
    let i = std::env::args().len() % 3;
    println!("{}!", TABLE[i]);
}
"#;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `true` if a runnable `rustc` is on PATH.
fn rustc_available() -> bool {
    Command::new("rustc")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Build [`MAIN_RS`] optimized (the default PIE layout) into `out_bin`.
fn build_rust(dir: &std::path::Path, out_bin: &std::path::Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    if std::fs::write(dir.join("main.rs"), MAIN_RS).is_err() {
        return false;
    }
    Command::new("rustc")
        .args(["-O", "-o"])
        .arg(out_bin)
        .arg("main.rs")
        .current_dir(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// The Go twin of [`MAIN_RS`]: the same runtime-indexed table, whose headers
/// the Go linker emits as absolute `(ptr, len)` pairs in the data image.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

var table = [3]string{"kuna fatstr witness, ", "kuna second piece", "kuna third piece"}

func main() {
	i := len(os.Args) % 3
	fmt.Println(table[i] + "!")
}
"#;

/// `true` if a runnable `go` toolchain is on PATH.
fn go_available() -> bool {
    Command::new("go")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Build [`MAIN_GO`] stripped (`-ldflags=-s -w`) for the host, hermetically.
fn build_go(dir: &std::path::Path, out_bin: &std::path::Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    if std::fs::write(dir.join("main.go"), MAIN_GO).is_err() {
        return false;
    }
    Command::new("go")
        .args(["build", "-ldflags=-s -w", "-o"])
        .arg(out_bin)
        .arg("main.go")
        .current_dir(dir)
        .env("GOCACHE", dir.join(".gocache"))
        .env("GOPATH", dir.join(".gopath"))
        .env("CGO_ENABLED", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// The virtual address of [`WITNESS`] inside the built binary's `.rodata`.
fn witness_vma(bin: &std::path::Path) -> Option<u64> {
    let bytes = std::fs::read(bin).ok()?;
    let file = object::File::parse(bytes.as_slice()).ok()?;
    for sec in file.sections() {
        if !matches!(
            sec.kind(),
            object::SectionKind::ReadOnlyData | object::SectionKind::ReadOnlyString
        ) {
            continue;
        }
        let data = sec.data().ok()?;
        if let Some(pos) = data.windows(WITNESS.len()).position(|w| w == WITNESS) {
            return Some(sec.address() + pos as u64);
        }
    }
    None
}

/// Bootstrap, optionally flip `fatstr`, commit, and report the size of the
/// data symbol containing the witness address (1 byte probed).
fn committed_extent(bin: &str, vma: u64, option: Option<&str>) -> Option<i32> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap rust fixture with built processor specs");
    if let Some(v) = option {
        prog.arch_mut().set_kuna_option("fatstr", v).expect("fatstr option applies");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .cloned()
        .expect("a default code space");
    let addr = Address::new(Rc::clone(&space), vma);
    let arch = prog.arch();
    let global = arch.symboltab.get_global_scope()?;
    let eref = arch.symboltab.find_container(global, &addr, 1, &Address::new_invalid())?;
    Some(arch.symboltab.entry(global, eref).get_size())
}

/// The record states 21 bytes; the NUL run cannot stop there (the next piece
/// is printable), so the two options commit visibly different extents.
#[test]
fn rust_str_records_segment_the_glued_nul_run() {
    if !rustc_available() {
        eprintln!("verify_fatstr_records: skipping (`rustc` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_fatstr_records_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("rust_fatstr");
    if !build_rust(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed rustc failed to build the fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");
    let vma = witness_vma(&out_bin).expect("the witness literal is in .rodata");

    let bounded = committed_extent(bin, vma, None)
        .expect("fatstr auto commits a bounded symbol at the literal");
    assert_eq!(
        bounded,
        WITNESS.len() as i32,
        "the record-stated extent is the literal's own 21 bytes"
    );

    match committed_extent(bin, vma, Some("off")) {
        // The glued NUL-run symbol: it cannot end at the record boundary,
        // because the next piece's bytes are printable.
        Some(glued) => assert!(
            glued > WITNESS.len() as i32,
            "with fatstr off the NUL scan glues past the literal (got {glued})"
        ),
        // Or no symbol at all, if the scan rejected the run for other reasons —
        // either way, nothing states the literal's true extent.
        None => {}
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// The Go arm: the same record shape, slots in the Go data image, under the
/// same option. A stripped Go build's table headers state the literal's exact
/// 21 bytes; `off` leaves the blob to the NUL scan, which cannot.
#[test]
fn go_string_records_bound_the_blob_literal() {
    if !go_available() {
        eprintln!("verify_fatstr_records: skipping Go arm (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_fatstr_go_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_fatstr");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");
    let vma = witness_vma(&out_bin).expect("the witness literal is in .rodata");

    let bounded = committed_extent(bin, vma, None)
        .expect("fatstr auto commits a bounded symbol at the literal");
    assert_eq!(
        bounded,
        WITNESS.len() as i32,
        "the Go header-stated extent is the literal's own 21 bytes"
    );

    match committed_extent(bin, vma, Some("off")) {
        Some(glued) => assert!(
            glued != WITNESS.len() as i32,
            "with fatstr off nothing states the literal's exact extent"
        ),
        None => {}
    }

    let _ = std::fs::remove_dir_all(&dir);
}
