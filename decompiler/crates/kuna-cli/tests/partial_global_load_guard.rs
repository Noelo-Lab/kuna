//! Partial writes must reach a pointer read before a later whole-object write.
mod common;
use common::process;
use std::process::Command;

const FUNCTIONS: [&str; 11] = [
    "halves",
    "high",
    "low",
    "mixed",
    "across_call",
    "conditional",
    "two_reads",
    "volatile_halves",
    "affine",
    "agreeing_phi",
    "differing_phi",
];
const PROTOTYPES: [&str; 11] = [
    "prototype halves unsigned long halves(unsigned long *, unsigned, unsigned)",
    "prototype high unsigned high(unsigned *, unsigned)",
    "prototype low unsigned low(unsigned *, unsigned)",
    "prototype mixed unsigned long mixed(unsigned long *, unsigned short, unsigned char)",
    "prototype across_call unsigned long across_call(unsigned long *, unsigned, unsigned)",
    "prototype conditional unsigned long conditional(unsigned long *, unsigned, unsigned, int)",
    "prototype two_reads unsigned long two_reads(unsigned long *, unsigned, unsigned)",
    "prototype volatile_halves unsigned long volatile_halves(unsigned long *, unsigned, unsigned)",
    "prototype affine unsigned long affine(unsigned long *, unsigned, unsigned, unsigned)",
    "prototype agreeing_phi unsigned long agreeing_phi(unsigned long *, unsigned, unsigned, int)",
    "prototype differing_phi unsigned long differing_phi(unsigned long *, unsigned long *, unsigned, unsigned, int)",
];

#[test]
#[cfg(target_arch = "x86_64")]
fn partial_global_writes_reach_aliasing_loads_before_whole_writes() {
    let source = common::fixture("partial_global_load_x86_64.c");
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "the round trip requires a C compiler"
    );
    let binary = common::scratch_file("partial-global-native", "exe");
    let emitted = common::scratch_file("partial-global-emitted", "c");
    let rebuilt = common::scratch_file("partial-global-rebuilt", "exe");
    let parts = regex::Regex::new(r"(\w+)\._(\d+)_(\d+)_").unwrap();
    for cc in &compilers {
        for opt in ["-O0", "-O2"] {
            let build = Command::new(cc)
                .args([opt, "-fno-pie", "-no-pie", "-fno-strict-aliasing"])
                .arg(&source)
                .arg("-o")
                .arg(&binary)
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{cc} {opt}: {}",
                String::from_utf8_lossy(&build.stderr)
            );
            let expected = process::required_output(&mut Command::new(&binary)).stdout;
            let selectors = FUNCTIONS.join(",");
            let mut args = vec![
                "decompile-all",
                binary.to_str().unwrap(),
                "--functions",
                &selectors,
                "--assert-strict",
            ];
            for p in &PROTOTYPES {
                args.extend(["--assert", p]);
            }
            let (printed, stderr, status) = common::run_kuna(&args);
            assert_eq!(status, 0, "{cc} {opt}: {stderr}");
            let volatile_body = printed
                .split("// Function: volatile_halves @")
                .nth(1)
                .unwrap();
            assert_eq!(
                volatile_body
                    .lines()
                    .filter(|line| line.trim_start().starts_with("gv") && line.contains(" = "))
                    .count(),
                3,
                "volatile stores changed width or multiplicity:\n{volatile_body}"
            );
            let c = parts.replace_all(&printed, |m: &regex::Captures| {
                let ty = match &m[3] {
                    "1" => "unsigned char",
                    "2" => "unsigned short",
                    "4" => "unsigned int",
                    "8" => "unsigned long",
                    size => panic!("unexpected partial width {size}: {printed}"),
                };
                let qualifier = if &m[1] == "gv" { "volatile " } else { "" };
                format!("(*({qualifier}{ty} *)((char *)&{} + {}))", &m[1], &m[2])
            });
            std::fs::write(&emitted, format!("extern unsigned long gp, gb, seen;\nextern volatile unsigned long gv;\nvoid touch(void);\n#define CONCAT44(a,b) (((unsigned long)(unsigned)(a) << 32) | (unsigned)(b))\n#define SUB84(a,b) ((unsigned)((unsigned long)(a) >> (8 * (b))))\n{c}")).unwrap();
            for out_cc in &compilers {
                for out_opt in ["-O0", "-O2"] {
                    let build = Command::new(out_cc)
                        .args([
                            "-std=gnu11",
                            "-w",
                            out_opt,
                            "-fno-strict-aliasing",
                            "-DPARTIAL_GLOBAL_HARNESS",
                        ])
                        .arg(&emitted)
                        .arg(&source)
                        .arg("-o")
                        .arg(&rebuilt)
                        .output()
                        .unwrap();
                    assert!(
                        build.status.success(),
                        "{cc} {opt} printed C rejected by {out_cc} {out_opt}:\n{}\n{printed}",
                        String::from_utf8_lossy(&build.stderr)
                    );
                    let actual = process::required_output(&mut Command::new(&rebuilt)).stdout;
                    assert_eq!(String::from_utf8_lossy(&actual), String::from_utf8_lossy(&expected), "{cc} {opt} printed C built by {out_cc} {out_opt} changes return values or memory:\n{printed}");
                }
            }
        }
    }
    for path in [binary, emitted, rebuilt] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
#[cfg(target_arch = "x86_64")]
fn memory_and_call_roots_keep_the_existing_partial_store_behavior() {
    let source = common::fixture("partial_global_excluded_x86_64.c");
    let callback = common::scratch_file("partial-global-excluded-callback", "c");
    let binary = common::scratch_file("partial-global-excluded", "exe");
    std::fs::write(
        &callback,
        "unsigned long *excluded_give(void) { return 0; }\n",
    )
    .unwrap();
    let stores = regex::Regex::new(r"(?m)^\s*excluded_g(?:\._\d+_\d+_)? = [^\n]+;").unwrap();
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the exclusions require a C compiler");
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            let build = Command::new(cc)
                .args([opt, "-fno-pie", "-no-pie", "-fno-strict-aliasing"])
                .arg(&source)
                .arg(&callback)
                .arg("-o")
                .arg(&binary)
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{cc} {opt}: {}",
                String::from_utf8_lossy(&build.stderr)
            );
            for function in ["memory_root", "call_root", "memory_cycle"] {
                let args = ["decompile", binary.to_str().unwrap(), function];
                let (printed, stderr, status) = common::run_kuna(&args);
                assert_eq!(status, 0, "{cc} {opt} {function}: {stderr}");
                let (existing, stderr, status) = common::run_kuna(&[
                    "decompile",
                    binary.to_str().unwrap(),
                    function,
                    "--option",
                    "indexaliasguard",
                    "off",
                ]);
                assert_eq!(status, 0, "{cc} {opt} {function}: {stderr}");
                let writes: Vec<_> = stores
                    .find_iter(&printed)
                    .map(|m| m.as_str().trim())
                    .collect();
                let before: Vec<_> = stores
                    .find_iter(&existing)
                    .map(|m| m.as_str().trim())
                    .collect();
                assert_eq!(
                    writes, before,
                    "{cc} {opt} {function}: excluded stores changed:\n{printed}"
                );
                assert_eq!(
                    writes,
                    ["excluded_g = 0;"],
                    "{cc} {opt} {function}: provenance broadened beyond inputs:\n{printed}"
                );
            }
        }
    }
    for path in [callback, binary] {
        std::fs::remove_file(path).unwrap();
    }
}
