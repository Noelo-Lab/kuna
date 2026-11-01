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
//! Layout checks use application structs and the public Linux Iovec ABI,
//! rather than runtime-internal types whose fields change between releases.

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
	"syscall"
)

type Point struct {
	X   int
	Y   int
	Tag string
}

type Envelope struct {
	Lead byte
	*Point
	Tail uint16
}

type Z struct { X int }
type A struct { Z; Tail int }
type Pointer *Pointer
type Left *Right
type Right *Left
type Recursive struct { Direct Pointer; Mutual Left; Tail int }
type Node struct { Next *Node; Value int }

//go:noinline
func mk(i int) Point {
	return Point{X: i, Y: i * 3, Tag: "t"}
}

func main() {
	p := mk(len(os.Args))
	fmt.Println(p)
	fmt.Println(Envelope{Lead: 1, Point: &p, Tail: 2})
	fmt.Println(A{Z{3}, 4})
	fmt.Println(syscall.Iovec{})
	fmt.Println(Recursive{Tail: 7})
	fmt.Println(Node{Value: 9})
}
"#;

/// Build [`MAIN_GO`] stripped (`-ldflags=-s -w`) for a Linux target.
fn build_go(dir: &std::path::Path, out_bin: &std::path::Path, arch: &str) -> bool {
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
        .env("GOARCH", arch)
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
    for arch in ["amd64", "arm64", "386", "arm"] {
        check_named_struct(arch);
    }
}

fn check_named_struct(arch: &str) {
    let width = if matches!(arch, "386" | "arm") { 4 } else { 8 };
    let dir = std::env::temp_dir().join(format!("kuna_gotypes_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin, arch) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/{arch} fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    // Default: the program's own struct, from its own metadata.
    let prog = load(bin);
    let tf = prog.arch().types();
    if width == 4 {
        let bytes = std::fs::read(&out_bin).unwrap();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let version = kuna_analysis::loader::gobuildinfo::buildinfo_blob(&file)
            .and_then(kuna_analysis::loader::gobuildinfo::version_from_blob);
        if !matches!(version, Some((1, 18..=25))) {
            assert!(tf.find_by_name("main.Point").unwrap().is_none(), "unsupported 32-bit layout");
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }
    }
    let pt = tf
        .find_by_name("main.Point")
        .expect("factory lookup succeeds")
        .expect("main.Point interned from the typelinks");
    assert_eq!(pt.get_metatype(), type_metatype::TYPE_STRUCT, "a struct shell");
    assert_eq!(pt.get_size(), 4 * width, "two integers and a string header");
    for (i, offset) in [0, width, 2 * width].into_iter().enumerate() {
        assert_eq!(pt.get_field(i as i32).unwrap().offset, offset);
    }
    let envelope = tf.find_by_name("main.Envelope").unwrap().unwrap();
    assert_eq!(envelope.get_size(), 3 * width);
    for (i, (name, offset)) in [("Lead", 0), ("Point", width), ("Tail", 2 * width)].into_iter().enumerate() {
        let field = envelope.get_field(i as i32).unwrap();
        assert_eq!((field.name.as_str(), field.offset), (name, offset));
    }
    let parent = tf.find_by_name("main.A").unwrap().unwrap();
    let child = parent.get_field(0).expect("embedded value field survives");
    assert_eq!((child.name.as_str(), child.offset), ("Z", 0));
    assert_eq!(child.field_type.get_size(), width);
    assert_eq!(child.field_type.get_field(0).unwrap().name, "X");
    assert_eq!(parent.get_field(1).unwrap().offset, width);
    let iovec = tf.find_by_name("syscall.Iovec").unwrap().unwrap();
    assert_eq!(iovec.get_size(), 2 * width);
    assert_eq!(iovec.get_field(0).unwrap().name, "Base");
    assert_eq!(iovec.get_field(1).unwrap().name, "Len");
    assert_eq!(iovec.get_field(1).unwrap().offset, width);

    let recursive = tf.find_by_name("main.Recursive").unwrap().unwrap();
    assert_eq!(recursive.get_size(), 3 * width);
    for (i, expected_depth) in [(0, 1), (1, 2)] {
        let field = recursive.get_field(i).unwrap();
        assert_eq!(field.offset, i * width);
        let mut ty = field.field_type.clone();
        for _ in 0..expected_depth {
            assert_eq!(ty.get_size(), width);
            ty = ty.get_ptr_to().expect("recursive alias retains a pointer");
        }
        assert_eq!(ty.get_metatype(), type_metatype::TYPE_VOID);
    }
    assert_eq!(recursive.get_field(2).unwrap().offset, 2 * width);
    let node = tf.find_by_name("main.Node").unwrap().unwrap();
    let next = node.get_field(0).unwrap().field_type.get_ptr_to().unwrap();
    assert_eq!(next.get_name(), "main.Node");
    assert_eq!(node.get_size(), 2 * width);

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
    for needle in ["main_Point".to_string(), format!("int{width} X"), format!("int{width} Y"), "go.string Tag".to_string()] {
        assert!(text.contains(&needle), "definition renders {needle}; got:\n{text}");
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
