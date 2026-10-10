//! A global read back after a store through a pointer that may point at it is
//! printed as a read of the global, a value read before such a store keeps its
//! own variable, a store to the global stays ahead of the pointer store, a load
//! through the pointer stays ahead of a store to the global, also one on a
//! branch or in a loop, and a store to the global stays ahead of a pointer load,
//! in a loop, before a call, before a shared return block, and when the global
//! is stored again; a function with more pointer stores than heritage guards
//! keeps the output of `indexaliasguard load`.
use crate::common;
use common::process;
use std::process::Command;

const FUNCS: &str = "s_param,s_const,s_once,s_plain,s_short,s_byte,s_long,s_narrow,s_branch,s_loop,s_cmp,\
                     l_keep,l_keepc,l_reload,l_twice,w_order,w_order2,r_order,w_inc,r_cond,r_two,r_loop,\
                     e_after,e_call,e_reg,w_epi,w_big,w_big2";

/// At -O0 the first store of `w_dead` and `w_dead2` is lost (#825), so that
/// build's harness compiles the source's own copies of them.
const DEAD: &str = "w_dead,w_dead2";

const DECLS: &str = "extern int gi;\nextern int gj;\nextern short gs;\nextern unsigned char gb;\nextern long gl;\n\
                     extern int gz0, gz1, gz2, gz3, gz4, gz5, gz6, gz7, gz8, gz9, gz10, gz11;\nextern int gk;\n\
                     extern int rbuf[];\nvoid touch(void);\nvoid touch2(int);\n";

const WANT: &str = "s_param -4\ns_const 10\ns_once -5\ns_plain 4\ns_short -27\ns_byte 201\ns_long -33\n\
                    s_narrow 4727\ns_branch 16 6\ns_loop 9 0\ns_cmp 11\nl_keep 4 8\nl_keepc 30 36\n\
                    l_reload 64\nl_twice 367\nw_order 9 1\nw_order2 4 8 2\nr_order 3 9\nw_inc 6\n\
                    r_cond 5 9 9\nr_two -2 1 3 3\nr_loop 4 0 100\ne_after 24 10\ne_call 15 5\n\
                    e_reg -1 6 -1\nw_dead 5 5\nw_dead2 -1 -1\nw_epi 3 0 3\nw_big 507 7 7\nw_big2 507 7 2\n";

/// `storealias_x86_64.c` stores to a global and then through a pointer, reads
/// a global and then stores through a pointer, in straight lines, on a branch
/// and in a loop, loads through a pointer before storing to the global, and
/// stores to the global before loading through a pointer.  Its own `main` aims
/// every pointer at the global, so each build's printed functions, compiled by
/// gcc and clang at -O0 and -O2 with the globals declared as in the source,
/// must print what the binary prints.
#[test]
fn a_global_is_read_where_the_binary_reads_it_around_a_pointer_store() {
    let harness = common::fixture("storealias_x86_64.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    let all = format!("{FUNCS},{DEAD}");
    for (fixture, funcs, keep) in [
        ("storealias_gcc_O0_x86_64", FUNCS, Some("-DSTOREALIAS_KEEP_DEAD")),
        ("storealias_gcc_O2_x86_64", all.as_str(), None),
        ("storealias_clang_O2_x86_64", all.as_str(), None),
    ] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            funcs,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{printed}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-fno-strict-aliasing", "-DSTOREALIAS_HARNESS"])
                    .args(common::CC_GCC15_DEMOTE)
                    .args(keep)
                    .arg("-o")
                    .arg(&exe)
                    .arg(&src)
                    .arg(&harness)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C ({fixture}):\n{}\n{printed}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{fixture} built by {cc} {level} computes a different value than the binary:\n{printed}"
                );
            }
        }
        let _ = std::fs::remove_file(src);
        let _ = std::fs::remove_file(exe);
    }
}
