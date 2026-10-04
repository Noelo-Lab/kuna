//! A stored integer still reaches a variadic call; format evidence must not
//! claim registers outside its argument list or apply to unknown formats.
mod common;
use common::process;
use std::process::Command;

const NAMES: [&str; 18] = [
    "caller_integer",
    "caller_stored_integer",
    "caller_computed_integer",
    "caller_stored_extra",
    "caller_stored_unused",
    "caller_stored_malformed",
    "caller_stored_float_format",
    "caller_stored_writable",
    "caller_stored_dynamic",
    "caller_undeclared_stored",
    "caller_clobbered",
    "caller_narrow_stored",
    "caller_scanf_control",
    "caller_partial_unknown",
    "caller_moved_narrow",
    "caller_gap",
    "caller_gap_clobbered",
    "caller_stored_two",
];

fn function<'a>(text: &'a str, name: &str) -> &'a str {
    text.split(&format!("// Function: {name} @"))
        .nth(1)
        .unwrap_or_else(|| panic!("no {name}:\n{text}"))
        .split("// Function:")
        .next()
        .unwrap()
        .split_once('\n')
        .unwrap()
        .1
}

fn decompile(binary: &std::path::Path, enabled: bool) -> String {
    let mut cmd = Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    );
    cmd.env("KUNA_SPECS", common::repo_root().join("specs"));
    cmd.arg("decompile-all").arg(binary).args([
        "--mode",
        "reliable",
        "--assert-strict",
        "--option",
        "realtypes",
        "on",
        "--functions",
        &NAMES.join(","),
        "--option",
        "varargforward",
        if enabled { "on" } else { "off" },
        "--assert",
        "prototype render_value unsigned long long MSABI render_value(const char *format,...)",
    ]);
    cmd.args(["--assert", "prototype clobber void MSABI clobber(void)"]);
    cmd.args([
        "--assert",
        "prototype scanflike unsigned long long MSABI scanflike(const char *format,...)",
    ]);
    cmd.args([
        "--assert",
        "prototype render_two unsigned long long MSABI render_two(const char *format,...)",
    ]);
    cmd.args(["--assert", "prototype render_with_flag unsigned long long MSABI render_with_flag(int flag,const char *format,...)"]);
    for name in NAMES {
        if name == "caller_undeclared_stored" {
            continue;
        }
        let params = if name == "caller_moved_narrow" {
            "char *value,unsigned char item,int unused"
        } else if name == "caller_narrow_stored" {
            "char *value,unsigned char item"
        } else if name == "caller_integer" {
            "int value"
        } else if name == "caller_stored_dynamic" {
            "const char *format,int *value"
        } else {
            "int *value"
        };
        cmd.args([
            "--assert",
            &format!("prototype {name} unsigned long long MSABI {name}({params})"),
        ]);
    }
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn a_stored_variadic_integer_round_trips_without_claiming_unused_registers() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        eprintln!("skipping native x86-64 fixture: gcc unavailable");
        return;
    }
    let fixture = common::repo_root().join("tests/cli/fixtures/variadic-secondary-reader");
    let binary = common::scratch_file("variadic-secondary-native", "exe");
    let output = Command::new("gcc")
        .args(["-O0", "-no-pie"])
        .arg(fixture.join("probe.S"))
        .arg(fixture.join("native.c"))
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(Command::new(&binary).status().unwrap().success());
    let off = decompile(&binary, false);
    let on = decompile(&binary, true);
    assert!(
        function(&off, "caller_stored_integer").contains("render_value(\"%i\")"),
        "{off}"
    );
    for name in [
        "caller_integer",
        "caller_stored_integer",
        "caller_computed_integer",
        "caller_stored_extra",
    ] {
        assert!(
            function(&on, name).contains("render_value(\"%i\","),
            "{name}:\n{on}"
        );
    }
    assert!(
        function(&on, "caller_stored_two").contains("render_two(\"%i %i\","),
        "{on}"
    );
    for name in ["caller_integer", "caller_computed_integer"] {
        assert_eq!(function(&on, name), function(&off, name), "{name}");
    }
    for name in [
        "caller_stored_unused",
        "caller_stored_malformed",
        "caller_stored_float_format",
        "caller_stored_writable",
        "caller_stored_dynamic",
        "caller_undeclared_stored",
        "caller_clobbered",
        "caller_narrow_stored",
        "caller_scanf_control",
        "caller_partial_unknown",
        "caller_moved_narrow",
        "caller_gap",
        "caller_gap_clobbered",
    ] {
        assert_eq!(
            function(&on, name),
            function(&off, name),
            "{name}:\n{on}\n{off}"
        );
    }
    let extra = function(&on, "caller_stored_extra");
    let args = extra
        .split("render_value(")
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap();
    assert_eq!(args.matches(',').count(), 1, "{extra}");

    let printed: String = [
        "caller_integer",
        "caller_stored_integer",
        "caller_computed_integer",
        "caller_stored_extra",
        "caller_stored_two",
    ]
    .iter()
    .map(|name| function(&on, name))
    .collect();
    let src = common::scratch_file("variadic-secondary-emitted", "c");
    let exe = common::scratch_file("variadic-secondary-emitted", "exe");
    std::fs::write(&src, format!(r#"
#include <stdarg.h>
#include <limits.h>
#include <stdint.h>
typedef int32_t int4;
typedef uint32_t uint4;
typedef uint64_t uint8;
unsigned long long render_value(const char *format,...) {{
    va_list ap;
    va_start(ap,format);
    unsigned int value = va_arg(ap,int);
    va_end(ap);
    return value;
}}
unsigned long long render_two(const char *format,...) {{
    va_list ap;
    va_start(ap,format);
    unsigned int first = va_arg(ap,int);
    unsigned int second = va_arg(ap,int);
    va_end(ap);
    return ((uint64_t)second << 32) | first;
}}
{printed}
int main(void) {{
    const int values[] = {{INT_MIN,-123,-1,0,1,99,INT_MAX-1}};
    for (unsigned i = 0; i < sizeof(values)/sizeof(values[0]); ++i) {{
        int x = values[i];
        if (caller_integer(x) != (unsigned)x) return 1;
        int stored = x;
        if (caller_stored_integer(&stored) != (unsigned)(x+1) || stored != x+1) return 2;
        if (caller_computed_integer(&x) != (unsigned)(x+1) || x != values[i]) return 3;
        int pair[2] = {{x,7}};
        if (caller_stored_extra(pair) != (unsigned)(x+1) || pair[0] != x+1 || pair[1] != 9) return 4;
        int two[2] = {{x,-123}};
        uint64_t packed = ((uint64_t)(uint32_t)-121 << 32) | (uint32_t)(x+1);
        if (caller_stored_two(two) != packed || two[0] != x+1 || two[1] != -121) return 5;
    }}
    return 0;
}}
"#)).unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-std=gnu11", level, "-o"])
                .arg(&exe)
                .arg(&src)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            assert!(
                Command::new(&exe).status().unwrap().success(),
                "{cc} {level}:\n{printed}"
            );
        }
    }
    for path in [binary, src, exe] {
        std::fs::remove_file(path).unwrap();
    }
}
