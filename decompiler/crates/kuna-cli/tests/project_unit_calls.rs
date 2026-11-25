//! A project export declares the p-code helpers (`SCARRY4`, `SUB168`),
//! user-defined operations (`invalidInstructionException`) and decode-failure
//! halts (`halt_baddata`) its C calls for code that defines
//! `KUNA_DECLARE_HELPERS`, and its `undefined16` takes arithmetic, so the
//! export compiles as C11 and as C2x, where `T f();` stops meaning "any
//! arguments". `checked_arith_x86_64`'s `scale` checks its add with SCARRY4 and
//! traps with `ud1`; `argclobber_pair_x86_64`'s `mk` returns a 16-byte rax:rdx
//! pair, which its caller splits with SUB168; `pseudo_names_x86_64`'s
//! `undecodable` holds a byte x86-64 does not decode.
use crate::common;
use common::process::{self, Capped};
use std::path::PathBuf;
use std::process::Command;

fn export(fixture: &str) -> PathBuf {
    let dir = common::scratch_file(fixture, "kuna");
    let specs = common::repo_root().join("specs");
    let (_stdout, stderr, code) = common::run_kuna(&[
        "decompile-project",
        &common::fixture(fixture),
        "-o",
        dir.to_str().unwrap(),
        "--sleighpath",
        specs.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "kuna decompile-project {fixture}: {stderr}");
    dir
}

fn declares(header: &str, name: &str) -> bool {
    header.lines().any(|line| {
        line.starts_with("extern ")
            && line.split(|c: char| !c.is_ascii_alphanumeric() && c != '_').any(|word| word == name)
    })
}

#[test]
fn an_export_that_calls_helpers_and_operations_compiles() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("an_export_that_calls_helpers_and_operations_compiles: skipping (no `cc`)");
        return;
    }
    for (fixture, names) in [
        ("checked_arith_x86_64", &["SCARRY4", "invalidInstructionException"][..]),
        ("argclobber_pair_x86_64", &["SUB168"][..]),
        ("pseudo_names_x86_64", &["halt_baddata"][..]),
    ] {
        let dir = export(fixture);
        let c = dir.join(format!("{fixture}.c"));
        let header = std::fs::read_to_string(dir.join(format!("{fixture}.h"))).unwrap();
        let body = std::fs::read_to_string(&c).unwrap();
        for name in names {
            assert!(body.contains(&format!("{name}(")), "{fixture} calls no {name}:\n{body}");
            assert!(declares(&header, name), "{fixture}'s header does not declare {name}:\n{header}");
        }
        for standard in ["-std=c11", "-std=c2x"] {
            let out = Command::new("cc")
                .args([standard, "-fsyntax-only"])
                .args(common::CC_GCC15_DEMOTE)
                .args(["-Werror=implicit-function-declaration", "-Werror=undef", "-DKUNA_DECLARE_HELPERS"])
                .arg(&c)
                .capped_output()
                .expect("spawn cc");
            assert!(
                out.status.success(),
                "{fixture}'s export failed cc {standard}:\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
}

/// A jump stub forwards whatever its caller passes, so an export gives one kuna
/// recovered `(void)` for the parameter list `(KUNA_ANY_ARGS)`, in its prototype
/// and its definition, and declares the GOT slot a PLT stub jumps through, but
/// not the code address a jump names. `plt_import_x86_64`'s `main` calls
/// `strfry` through its PLT stub, which jumps through the GOT slot 0x10024f8;
/// `jump_stubs_x86_64`'s `main` passes two arguments to each of
/// `endbr64; jmp *slot`, `endbr64; bnd jmp *slot` and `jmp *slot`. Each `_start`
/// ends in `jmp .`, at 0x10013ce and 0x10012fe.
#[test]
fn an_export_that_calls_through_jump_stubs_compiles() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("an_export_that_calls_through_jump_stubs_compiles: skipping (no `cc`)");
        return;
    }
    for (fixture, stubs, slots, branch_target) in [
        ("plt_import_x86_64", &["strfry"][..], &["dat_10024f8"][..], "dat_10013ce"),
        ("jump_stubs_x86_64", &["ibt_stub", "bnd_stub", "plain_stub"][..], &[][..], "dat_10012fe"),
    ] {
        let dir = export(fixture);
        let c = dir.join(format!("{fixture}.c"));
        let header = std::fs::read_to_string(dir.join(format!("{fixture}.h"))).unwrap();
        let body = std::fs::read_to_string(&c).unwrap();
        for stub in stubs {
            let open = format!("{stub}(KUNA_ANY_ARGS)");
            assert!(
                header.lines().any(|line| line.ends_with(&format!(" {open};")) || line.ends_with(&format!("*{open};"))),
                "{fixture}'s header does not declare {open}:\n{header}"
            );
            let lines: Vec<&str> = body.lines().collect();
            assert!(
                lines.windows(2).any(|pair| pair[0].contains(&open) && !pair[0].ends_with(';') && pair[1] == "{"),
                "{fixture} does not define {open}:\n{body}"
            );
            let void = format!("{stub}(void)");
            assert!(!header.contains(&void) && !body.contains(&void), "{fixture} keeps {void}");
        }
        for slot in slots {
            assert!(declares(&header, slot), "{fixture}'s header does not declare {slot}:\n{header}");
        }
        assert!(!declares(&header, branch_target), "{fixture}'s header declares {branch_target}:\n{header}");
        for standard in ["-std=c11", "-std=c2x"] {
            if !defines_any_args(&dir, fixture, standard) {
                eprintln!("{fixture}: skipping cc {standard}, which cannot define a function taking any arguments");
                continue;
            }
            let out = Command::new("cc")
                .args([standard, "-fsyntax-only"])
                .args(common::CC_GCC15_DEMOTE)
                .args(["-Werror=implicit-function-declaration", "-Werror=undef", "-DKUNA_DECLARE_HELPERS"])
                .arg(&c)
                .capped_output()
                .expect("spawn cc");
            assert!(
                out.status.success(),
                "{fixture}'s export failed cc {standard}:\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
}

/// Can `cc` at `standard` define a function as taking `(KUNA_ANY_ARGS)` and
/// then call it with arguments? gcc before 13 cannot at `-std=c2x`: it rejects
/// `f(...)` and reads the definition `f()` as taking nothing.
fn defines_any_args(dir: &std::path::Path, fixture: &str, standard: &str) -> bool {
    let probe = dir.join("defines_any_args.c");
    let text = format!(
        "#include \"{fixture}.h\"\nint kuna_probe(KUNA_ANY_ARGS) {{ return 0; }}\n\
         int kuna_probe_call(void) {{ return kuna_probe(1); }}\n"
    );
    std::fs::write(&probe, text).unwrap();
    let out = Command::new("cc").args([standard, "-fsyntax-only"]).arg(&probe).capped_output().expect("spawn cc");
    out.status.success()
}

/// A jump stub whose prototype a library declaration gave keeps it: its callers
/// pass what that prototype says. `noreturn_propagate_x86_64`'s `abort` is GNU
/// ld's `endbr64; bnd jmp *slot` `.plt.sec` stub, and `abort` takes nothing.
#[test]
fn a_jump_stub_with_a_declared_prototype_keeps_it() {
    let fixture = "noreturn_propagate_x86_64";
    let dir = export(fixture);
    let header = std::fs::read_to_string(dir.join(format!("{fixture}.h"))).unwrap();
    let body = std::fs::read_to_string(dir.join(format!("{fixture}.c"))).unwrap();
    assert!(header.lines().any(|line| line == "void abort(void);"), "{fixture}'s header:\n{header}");
    assert!(body.lines().any(|line| line == "void abort(void)"), "{fixture}:\n{body}");
    assert!(!header.contains("abort(KUNA_ANY_ARGS)") && !body.contains("abort(KUNA_ANY_ARGS)"));
}

/// A call through an import's GOT slot prints as a call by the import's name,
/// so an export declares an import no prototype table lists as taking any
/// arguments, at the type of its call's output, whether or not the including
/// code defines `KUNA_DECLARE_HELPERS`. `got_import_x86_64`'s `_start` hands
/// `main` to `__libc_start_main` through its slot, as glibc's crt1.o does, and
/// `main` returns 1 more than `got_value` returns.
#[test]
fn an_export_that_calls_imports_through_their_got_slots_compiles() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("an_export_that_calls_imports_through_their_got_slots_compiles: skipping (no `cc`)");
        return;
    }
    let fixture = "got_import_x86_64";
    let dir = export(fixture);
    let c = dir.join(format!("{fixture}.c"));
    let header = std::fs::read_to_string(dir.join(format!("{fixture}.h"))).unwrap();
    let body = std::fs::read_to_string(&c).unwrap();
    for (name, declaration) in [
        ("__libc_start_main", "extern void __libc_start_main(KUNA_ANY_ARGS);"),
        ("got_value", "extern undefined4 got_value(KUNA_ANY_ARGS);"),
    ] {
        assert!(body.contains(&format!("{name}(")), "{fixture} calls no {name}:\n{body}");
        assert!(header.lines().any(|line| line == declaration), "{fixture}'s header lacks {declaration}:\n{header}");
    }
    for standard in ["-std=c11", "-std=c2x"] {
        let out = Command::new("cc")
            .args([standard, "-fsyntax-only"])
            .args(common::CC_GCC15_DEMOTE)
            .args(["-Werror=implicit-function-declaration", "-Werror=undef"])
            .arg(&c)
            .capped_output()
            .expect("spawn cc");
        assert!(
            out.status.success(),
            "{fixture}'s export failed cc {standard}:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}
