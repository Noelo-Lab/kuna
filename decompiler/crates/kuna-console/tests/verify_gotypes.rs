//! End-to-end gate for the Go runtime-type harvest (`gotypes`,
//! `docs/multilang.md` W3, the Go provider, phase 1).
//!
//! The proof, on a REAL stripped Go build (no DWARF, no symtab — the binary's
//! own runtime metadata is the only possible source):
//! - default (on): the type factory holds `main.Point` as a struct with the
//!   program's own fields at their true offsets, the definition renders as C,
//!   and the `go.string` header shell carries its text facet;
//! - `KUNA_GOTYPES=off`: a fresh load interns nothing — `main.Point` is
//!   unknown to the factory.
//!
//! Own `[[test]]` target because the off arm mutates process env (the
//! `verify_goreg`/`verify_goabi` pattern, RAII-guarded). The metadata parsers
//! themselves are pinned hermetically by the unit tests in
//! `kuna-analysis/src/analyzers/gotypes/tests.rs` (no Go toolchain needed).
//! TOOLCHAIN-DRIFT RULE: every assertion here is on the program's OWN type
//! (`main.Point`), never on a runtime-internal whose shape shifts release to
//! release; verified under go1.22 and go1.25.

use std::path::PathBuf;
use std::process::{Command, Stdio};

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::kuna_langfacet::LangFacet;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

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

/// A program whose own named struct reaches the typelinks: `fmt.Println(p)`
/// converts `main.Point` to an interface, which is exactly what registers a
/// type there. Field layout under the 64-bit ABI: `X @0`, `Y @8`,
/// `Tag @16` (a 16-byte string header) — total size 32.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

type Point struct {
	X   int
	Y   int
	Tag string
}

//go:noinline
func mk(i int) Point {
	return Point{X: i, Y: i * 3, Tag: "t"}
}

func main() {
	p := mk(len(os.Args))
	fmt.Println(p)
}
"#;

/// Build [`MAIN_GO`] stripped (`-ldflags=-s -w`) for linux/amd64, hermetically.
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
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("GOCACHE", dir.join(".gocache"))
        .env("GOPATH", dir.join(".gopath"))
        .env("CGO_ENABLED", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Bootstrap + commit, returning the live program.
fn load(bin: &str) -> ConsoleProgram {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

struct EnvGuard(&'static str, Option<std::ffi::OsString>);
impl EnvGuard {
    fn set(name: &'static str, value: &str) -> EnvGuard {
        let prev = std::env::var_os(name);
        std::env::set_var(name, value);
        EnvGuard(name, prev)
    }
}
impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.1 {
            Some(v) => std::env::set_var(self.0, v),
            None => std::env::remove_var(self.0),
        }
    }
}

#[test]
fn stripped_go_binary_yields_its_own_named_struct() {
    if !go_available() {
        eprintln!("verify_gotypes: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_gotypes_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    // Default: the program's own struct, from its own metadata.
    let prog = load(bin);
    let tf = prog.arch().types();
    let pt = tf
        .find_by_name("main.Point")
        .expect("factory lookup succeeds")
        .expect("main.Point interned from the typelinks");
    assert_eq!(pt.get_metatype(), type_metatype::TYPE_STRUCT, "a struct shell");
    assert_eq!(pt.get_size(), 32, "X(8) + Y(8) + Tag(string header, 16)");

    // The header shells carry their facets (the W3 plane fills).
    assert_eq!(tf.kuna_lang_facet("go.string"), Some(LangFacet::GoString));
    assert!(matches!(tf.kuna_lang_facet("go.slice"), Some(LangFacet::GoSlice { .. })));

    // The definition renders as C for the type surfaces — the rendering is
    // also how the field names and layout are asserted (the field payload is
    // private to the factory).
    let text =
        kuna_decomp::kuna_structdefs::definition_text(&pt, kuna_decomp::kuna_langtypes::SpellCtx::OFF);
    // The C renderer sanitizes the dotted tag (`struct main_Point`) and spells
    // the string header field through its shell.
    for needle in ["main_Point", "int8 X", "int8 Y", "go.string Tag"] {
        assert!(text.contains(needle), "definition renders {needle}; got:\n{text}");
    }

    // Off: a fresh load interns nothing.
    {
        let _guard = EnvGuard::set("KUNA_GOTYPES", "off");
        let prog_off = load(bin);
        assert!(
            prog_off.arch().types().find_by_name("main.Point").expect("lookup runs").is_none(),
            "with gotypes off the factory must not hold main.Point"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
