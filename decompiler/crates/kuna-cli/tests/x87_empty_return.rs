//! FFREE invalidates only the actual raw ST0 return slot, on every path.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn freed_x87_results_are_not_inferred_after_merges_or_loops() {
    let cc = "i686-w64-mingw32-gcc";
    if process::optional_output(Command::new(cc).arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("x87-empty-return", "s");
    let binary = source.with_extension("dll");
    std::fs::write(
        &source,
        r#".text
.macro function name
.globl _\name
.def _\name; .scl 2; .type 32; .endef
_\name:
.endm
function free_top
 mov 4(%esp),%ecx
 flds (%ecx)
 fmul %st(0),%st(0)
 ffree %st(0)
 ret
function valid_top
 mov 4(%esp),%ecx
 flds (%ecx)
 fmul %st(0),%st(0)
 ret
function free_other_equal
 mov 4(%esp),%ecx
 flds (%ecx)
 fld %st(0)
 ffree %st(1)
 ret
function refill_top
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 fld1
 ret
function pop_to_valid
 mov 4(%esp),%ecx
 flds (%ecx)
 fld1
 ffreep %st(0)
 ret
function mixed_path
 mov 4(%esp),%ecx
 flds (%ecx)
 cmpl $0,8(%esp)
 je 1f
 ffree %st(0)
1: ret
function all_paths_free
 mov 4(%esp),%ecx
 flds (%ecx)
 cmpl $0,8(%esp)
 je 1f
 fmul %st(0),%st(0)
 ffree %st(0)
 jmp 2f
1: ffree %st(0)
2: ret
function free_before_loop
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 mov 8(%esp),%ecx
 test %ecx,%ecx
 jz 2f
1: dec %ecx
 jnz 1b
2: ret
function refill_in_loop
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 mov 8(%esp),%ecx
 test %ecx,%ecx
 jz 2f
1: fld1
 dec %ecx
 jnz 1b
2: ret
function call_after_free
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 call _constant_result
 fadd %st(0),%st(0)
 ret
function constant_result
 fld1
 ret

function environment_after_free
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 mov 8(%esp),%edx
 fldenv (%edx)
 ret

function mmx_read_after_free
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 movd %mm1,%ecx
 ret

function mmx_after_free
 mov 4(%esp),%ecx
 flds (%ecx)
 ffree %st(0)
 pxor %mm1,%mm1
 ret
"#,
    )
    .unwrap();
    let built = Command::new(cc)
        .args([
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    for (name, empty) in [
        ("_free_top", true),
        ("_all_paths_free", true),
        ("_free_before_loop", true),
        ("_valid_top", false),
        ("_free_other_equal", false),
        ("_refill_top", false),
        ("_pop_to_valid", false),
        ("_mixed_path", false),
        ("_refill_in_loop", false),
        ("_mmx_after_free", false),
        ("_mmx_read_after_free", false),
        ("_environment_after_free", false),
        ("_call_after_free", false),
    ] {
        let (code, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            name,
            "--assert",
            "return _constant_result::ST0 float10",
        ]);
        assert_eq!(status, 0, "{err}");
        assert_eq!(
            code.starts_with(&format!("void {name}(")),
            empty,
            "{name}: {code}"
        );
        if empty {
            assert!(!code.contains("return "), "{code}");
        } else {
            assert!(code.starts_with(&format!("long double {name}(")), "{code}");
        }
    }
    let (code, err, status) = common::run_kuna(&[
        "decompile",
        binary.to_str().unwrap(),
        "_free_top",
        "--assert",
        "return _free_top::ST0 float10",
    ]);
    assert_eq!(status, 0, "{err}");
    assert!(code.starts_with("long double _free_top("), "{code}");
    for path in [source, binary] {
        std::fs::remove_file(path).unwrap();
    }
}
