//! End-to-end gate for the **uncomputed-return-value repair**: a function that
//! was modelled as returning a 16-byte `RAX:RDX` pair, whose high half is
//! leftover, returns a single register instead — and a function that genuinely
//! returns a pair keeps it.
//!
//! The symptom, on `fmt/main` with no usable prototype:
//!
//! ```text
//! undefined16 main(uint4 a0,void *a1)
//!   char v5 [16];
//!   v5._0_8_ = v16 ^ 1;   // the real result
//!   v5._8_8_ = v31;       // an uninitialized stack slot
//!   return v5;
//! ```
//!
//! That output is not merely unreadable — it reads memory the function never
//! wrote. Return recovery registers a trial per output register the model
//! characterizes (x86-64 SysV: `RAX` *and* `RDX`), ancestor realism accepts the
//! epilogue's callee-saved restore as a value that could legitimately reach the
//! RETURN, and the compiler spec's `join_dual_class` output rule then joins the
//! consecutive pair into one 16-byte return.
//!
//! The repair runs late, once heritage has resolved the restore into a bare
//! unwritten Varnode, and rewrites the RETURN to the half that carries a value.
//!
//! Fixtures: `regglobal_fmt_x86_64` with DWARF disabled (the prototype-less path;
//! with DWARF the locked `int` return never enters recovery at all) and
//! `structreturn_x86_64`, the negative control.

use std::path::PathBuf;
use std::process::Command;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::{print_c, print_c_recompile_prelude, print_c_types};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `bin`, apply `options` (before the analysis commit, as the CLI does),
/// decompile `func`, and return the captured C.
fn decompile_with_types(bin: &str, func: &str, options: &[&str]) -> (String, String, String) {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str().expect("UTF-8 fixture path").to_string();
    let mut prog = bootstrap_from_object(&path, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");

    let mut cmds: Vec<String> = options.iter().map(|o| o.to_string()).collect();
    cmds.push(format!("load function {func}"));
    cmds.push("decompile".into());
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        // The `option` lines above must land before the analysis commit; do the
        // commit here, after they are queued but before any of them runs, exactly
        // as `read symbols` does in the CLI script.
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    let fd = dcp.fd.as_ref().expect("decompiled fixture function");
    let prog = dcp.conf.as_mut().unwrap();
    let code = print_c(prog.arch_mut(), fd);
    let prelude = print_c_recompile_prelude(prog.arch());
    let types = print_c_types(prog.arch_mut());
    (code, prelude, types)
}

fn decompile(bin: &str, func: &str, options: &[&str]) -> String {
    decompile_with_types(bin, func, options).0
}

/// With no prototype to lock the return, `fmt/main` used to join `RAX:RDX`. The
/// `listing` flag is what surfaces it — the no-return analyses it drives add the
/// return sites where the leftover shows up.
#[test]
fn fmt_main_without_a_prototype_returns_one_register() {
    let code = decompile("regglobal_fmt_x86_64", "main", &["option dwarf off", "option listing on"]);

    assert!(
        !code.contains("undefined16"),
        "the bogus RAX:RDX return join must be gone; got:\n{code}",
    );
    assert!(
        !code.contains("[16]"),
        "the phantom 16-byte return buffer must be gone; got:\n{code}",
    );
    // Both spellings: an 8-byte write at offset 8 of the container renders
    // `._8_8_` since arraysubfield, and `[8]` before it -- neither may appear,
    // or this arm silently stops testing anything.
    assert!(
        !code.contains("[8] =") && !code.contains("._8_8_ ="),
        "nothing should write the phantom high half; got:\n{code}",
    );
}

/// NEGATIVE CONTROL: `make` builds both halves of a real `struct pair` return
/// (`x * 3`, `y + 7`). Arithmetic is not a move, so the walk stops there and the
/// pair is kept.
#[test]
fn a_computed_struct_pair_return_is_kept() {
    let code = decompile("structreturn_x86_64", "make", &[]);

    assert!(
        code.contains("undefined16"),
        "a genuine 16-byte struct return must survive the repair; got:\n{code}",
    );
    // Each half is an 8-byte write into the `undefined1[16]` container, so it
    // renders as the sized sub-field, not a size-blind element subscript
    // (arraysubfield; `v1[8] = ...` claimed a one-byte store).
    assert!(
        code.contains("._8_8_ =") && code.contains("._0_8_ ="),
        "both halves of a real pair are written; got:\n{code}",
    );
}

/// NEGATIVE CONTROL: `passthru` returns a struct it read through a pointer — the
/// shape closest to leftover, since both halves arrive by load rather than
/// arithmetic. A LOAD is a real value, so the pair is kept.
#[test]
fn a_loaded_struct_pair_return_is_kept() {
    // Select the CLI's compilable partial-access rendering, not Ghidra's
    // synthetic `._offset_size_` notation. Recovery and return inference are
    // unchanged; only the storage expression printer differs.
    let (code, prelude, types) = decompile_with_types(
        "structreturn_x86_64", "passthru", &["option partialaccess on"],
    );

    assert!(
        code.contains("undefined16"),
        "a pair loaded through a pointer must survive the repair; got:\n{code}",
    );
    // The recovered pointee can be an array or a synthesized record. Its spelling
    // is not the contract: both words must survive the generated function's ABI.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    check_loaded_pair_consumer(&code, &prelude, &types);
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    let _ = (prelude, types);
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn check_loaded_pair_consumer(code: &str, prelude: &str, types: &str) {
    let directory = crate::common::scratch_file("loaded-pair-return", "dir");
    std::fs::create_dir(&directory).unwrap();
    // The generic project header's byte-array-struct undefined16 is a separate,
    // preexisting incompatibility with scalar wide returns. State the native
    // RAX:RDX scalar contract explicitly, keeping every recovered record verbatim.
    let prelude = prelude.replace(
        "typedef struct { unsigned char b[16]; } undefined16;",
        "typedef unsigned __int128 undefined16;",
    );
    let source = directory.join("consumer.c");
    std::fs::write(&source, format!(r#"
#include <stdint.h>
{prelude}
{types}
{code}
extern unsigned __int128 native_passthru(const void *);
_Static_assert(sizeof(unsigned long) == 8, "native fixture uses 64-bit words");
int main(void) {{
    const uint64_t words[] = {{0,1,UINT64_MAX,UINT64_C(0x8000000000000000),
        UINT64_C(0x0123456789abcdef),UINT64_C(0xfedcba9876543210)}};
    for(unsigned i=0;i<sizeof(words)/sizeof(*words);++i) {{
        for(unsigned j=0;j<sizeof(words)/sizeof(*words);++j) {{
            _Alignas(16) uint64_t input[2]={{words[i],words[j]}};
            unsigned __int128 expected=((unsigned __int128)input[1]<<64)|input[0];
            unsigned __int128 native=native_passthru(input);
            unsigned __int128 generated=passthru((void *)input);
            if(native!=expected || generated!=native)return 1;
        }}
    }}
    return 0;
}}
"#)).unwrap();
    for compiler in ["gcc", "clang"] {
        match Command::new(compiler).arg("--version").output() {
            Ok(output) => assert!(output.status.success(), "{compiler} --version failed"),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => panic!("cannot run {compiler}: {error}"),
        }
        for optimization in ["-O0", "-O2"] {
            let native = directory.join(format!("{compiler}{optimization}.o"));
            let output = Command::new(compiler)
                .args(["-std=gnu11", optimization, "-Dpassthru=native_passthru",
                    "-Dmake=native_make", "-Dmain=native_main", "-c"])
                .arg(fixture("structreturn_x86_64.c")).arg("-o").arg(&native).output().unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            let executable = directory.join(format!("{compiler}{optimization}"));
            let output = Command::new(compiler)
                .args(["-std=gnu11", optimization, "-Werror"])
                .arg(&source).arg(&native).arg("-o").arg(&executable).output().unwrap();
            assert!(output.status.success(), "{}\n{code}\n{types}", String::from_utf8_lossy(&output.stderr));
            assert!(Command::new(&executable).status().unwrap().success(), "{compiler} {optimization}\n{code}");
        }
    }
    std::fs::remove_dir_all(directory).unwrap();
}
