//! A stored integer still reaches a variadic call; format evidence must not
//! claim registers outside its argument list or apply to unknown formats.
use crate::common;
use common::process;
use std::process::Command;

const NAMES: [&str; 25] = [
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
    "caller_clamped_integer",
    "caller_clamped_call",
    "caller_stored_comparison",
    "caller_stored_boolean",
    "caller_unknown_comparison",
    "caller_unknown_phi",
    "caller_unknown_upper_shift",
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
    cmd.args([
        "--assert",
        "prototype tag_value int MSABI tag_value(int *sender)",
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
        let params = if name == "caller_clamped_call" {
            "int *value,int *sender"
        } else if matches!(
            name,
            "caller_clamped_integer"
                | "caller_stored_comparison"
                | "caller_stored_boolean"
                | "caller_unknown_phi"
                | "caller_unknown_upper_shift"
        ) {
            "int *value,int tag"
        } else if name == "caller_moved_narrow" {
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

fn decompile_imported(binary: &std::path::Path, enabled: bool) -> String {
    decompile_imported_kind(binary, enabled, false)
}

fn decompile_imported_kind(binary: &std::path::Path, enabled: bool, floating: bool) -> String {
    let (name, end, params, format_width) = if floating {
        (
            "caller_imported_float",
            "0x140001022",
            "void *a,void *b,float value",
            6,
        )
    } else {
        ("caller_imported_integer", "0x14000101d", "int *value", 3)
    };
    let output = Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    )
    .env("KUNA_SPECS", common::repo_root().join("specs"))
    .arg("decompile-all")
    .arg(binary)
    .args([
        "--addr",
        "0x140001000",
        "--mode",
        "reliable",
        "--assert-strict",
        "--option",
        "realtypes",
        "on",
        "--option",
        "varargforward",
        if enabled { "on" } else { "off" },
        "--assert",
        &format!("function 0x140001000-{end}={name}"),
        "--assert",
        &format!("prototype {name} unsigned long long {name}({params})"),
        "--assert",
        "prototype 0x140002050 unsigned long long render_value(const char *format,...)",
        "--assert",
        &format!("data 0x1400020a0 char integer_format[{format_width}]"),
    ])
    .output()
    .unwrap();
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
    let object = common::scratch_file("variadic-secondary-selected", "o");
    let compiled = Command::new("gcc")
        // Pin the baseline ISA: GCC 15 (Ubuntu 26.04) defaults to a higher
        // -march and vectorizes the clamp with AVX pminsd/pmaxsd, which leaks
        // the stored argument into the varargforward-off reading. Baseline
        // x86-64 keeps the clamp scalar, as on the GCC the fixture was authored.
        .args(["-O2", "-fno-optimize-sibling-calls", "-march=x86-64", "-c"])
        .arg(fixture.join("selected.c"))
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let output = Command::new("gcc")
        .args(["-O0", "-no-pie"])
        .arg(&object)
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
    for name in [
        "caller_stored_integer",
        "caller_clamped_integer",
        "caller_clamped_call",
        "caller_stored_comparison",
    ] {
        assert!(
            function(&off, name).contains("render_value(\"%i\")"),
            "{name}:\n{off}"
        );
    }
    for name in [
        "caller_integer",
        "caller_stored_integer",
        "caller_computed_integer",
        "caller_stored_extra",
        "caller_clamped_integer",
        "caller_clamped_call",
        "caller_stored_comparison",
        "caller_stored_boolean",
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
        "caller_unknown_comparison",
        "caller_unknown_phi",
        "caller_unknown_upper_shift",
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

    let imported = common::scratch_file("variadic-secondary-imported", "exe");
    let generated = Command::new("python3")
        .arg(fixture.join("imported.py"))
        .arg(&imported)
        .output()
        .unwrap();
    assert!(
        generated.status.success(),
        "{}",
        String::from_utf8_lossy(&generated.stderr)
    );
    let imported_off = decompile_imported(&imported, false);
    let imported_on = decompile_imported(&imported, true);
    assert!(
        imported_off.contains("render_value(\"%i\")"),
        "{imported_off}"
    );
    assert!(
        imported_on.contains("render_value(\"%i\","),
        "{imported_on}"
    );
    let image = std::fs::read(&imported).unwrap();
    let mut writable = image.clone();
    writable[0x1b0 + 36..0x1b0 + 40].copy_from_slice(&0xc0000040u32.to_le_bytes());
    std::fs::write(&imported, writable).unwrap();
    assert_eq!(
        decompile_imported(&imported, false),
        decompile_imported(&imported, true)
    );
    let mut unsupported = image.clone();
    unsupported[0x4a1] = b'f';
    std::fs::write(&imported, unsupported).unwrap();
    assert_eq!(
        decompile_imported(&imported, false),
        decompile_imported(&imported, true)
    );
    let mut unknown = image.clone();
    unknown[0x200 + 4..0x200 + 9].fill(0x90);
    std::fs::write(&imported, unknown).unwrap();
    assert_eq!(
        decompile_imported(&imported, false),
        decompile_imported(&imported, true)
    );
    let mut healthy = image;
    healthy[0x200 + 9..0x200 + 11].copy_from_slice(&[0x90, 0x90]);
    std::fs::write(&imported, healthy).unwrap();
    let healthy_off = decompile_imported(&imported, false);
    assert!(
        healthy_off.contains("render_value(\"%i\","),
        "{healthy_off}"
    );
    assert_eq!(healthy_off, decompile_imported(&imported, true));
    let generated = Command::new("python3")
        .arg(fixture.join("imported.py"))
        .arg(&imported)
        .arg("--float")
        .output()
        .unwrap();
    assert!(
        generated.status.success(),
        "{}",
        String::from_utf8_lossy(&generated.stderr)
    );
    let floating_off = decompile_imported_kind(&imported, false, true);
    let floating_on = decompile_imported_kind(&imported, true, true);
    assert!(
        floating_off.contains("render_value(\"%.02f\",(float8)value)"),
        "{floating_off}"
    );
    assert_eq!(floating_off, floating_on);

    let mut printed: String = [
        "caller_integer",
        "caller_stored_integer",
        "caller_computed_integer",
        "caller_stored_extra",
        "caller_stored_two",
        "caller_clamped_integer",
        "caller_clamped_call",
        "caller_stored_comparison",
        "caller_stored_boolean",
    ]
    .iter()
    .map(|name| function(&on, name))
    .collect();
    printed.push_str(function(&imported_on, "caller_imported_integer"));
    printed.push_str(function(&floating_on, "caller_imported_float"));
    let src = common::scratch_file("variadic-secondary-emitted", "c");
    let exe = common::scratch_file("variadic-secondary-emitted", "exe");
    std::fs::write(&src, format!(r#"
#include <stdarg.h>
#include <limits.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
typedef int32_t int4;
typedef uint32_t uint4;
typedef uint64_t uint8;
typedef float float4;
typedef double float8;
unsigned long long render_value(const char *format,...) {{
    va_list ap;
    va_start(ap,format);
    if (format[1] == '.') {{
        double value = va_arg(ap,double);
        uint64_t bits;
        memcpy(&bits,&value,sizeof(bits));
        va_end(ap);
        return bits;
    }}
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
int tag_value(int *sender) {{ return *sender; }}
{printed}
int main(void) {{
    const int values[] = {{INT_MIN,-123,-1,0,1,99,INT_MAX-1}};
    for (unsigned i = 0; i < sizeof(values)/sizeof(values[0]); ++i) {{
        int x = values[i];
        if (caller_integer(x) != (unsigned)x) return 1;
        int stored = x;
        if (caller_stored_integer(&stored) != (unsigned)(x+1) || stored != x+1) return 2;
        stored = x;
        if (caller_imported_integer(&stored) != (unsigned)(x+1) || stored != x+1) return 10;
        if (caller_computed_integer(&x) != (unsigned)(x+1) || x != values[i]) return 3;
        int pair[2] = {{x,7}};
        if (caller_stored_extra(pair) != (unsigned)(x+1) || pair[0] != x+1 || pair[1] != 9) return 4;
        int two[2] = {{x,-123}};
        uint64_t packed = ((uint64_t)(uint32_t)-121 << 32) | (uint32_t)(x+1);
        if (caller_stored_two(two) != packed || two[0] != x+1 || two[1] != -121) return 5;
    }}
    const int selected[] = {{INT_MIN+1,-123,-1,0,1,5,6,7,11,12,13,99,INT_MAX-1}};
    const float floats[] = {{0.0f,-0.0f,1.25f,-2.5f,123456.0f}};
    for (unsigned i = 0; i < sizeof(floats)/sizeof(floats[0]); ++i) {{
        double expected = (double)floats[i];
        uint64_t bits;
        memcpy(&bits,&expected,sizeof(bits));
        if (caller_imported_float(0,0,floats[i]) != bits) return 11;
    }}
    const int tags[] = {{INT_MIN,-1,0,1,2,INT_MAX}};
    for (unsigned i = 0; i < sizeof(selected)/sizeof(selected[0]); ++i) {{
        for (unsigned j = 0; j < sizeof(tags)/sizeof(tags[0]); ++j) {{
            int value = selected[i], tag = tags[j];
            int expected = value + (tag == 1 ? 1 : -1);
            if (expected < 6) expected = 6;
            if (expected > 12) expected = 12;
            if (caller_clamped_integer(&value,tag) != (uint32_t)expected || value != expected) return 6;
            value = selected[i];
            if (caller_clamped_call(&value,&tag) != (uint32_t)expected || value != expected || tag != tags[j]) return 7;
            value = selected[i]; expected = value + (tag == 1);
            if (caller_stored_comparison(&value,tag) != (uint32_t)expected || value != expected) return 8;
            value = selected[i]; expected = value + ((tag < 0) != (tag == 1));
            if (caller_stored_boolean(&value,tag) != (uint32_t)expected || value != expected) return 9;
        }}
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
    for path in [binary, object, imported, src, exe] {
        std::fs::remove_file(path).unwrap();
    }
}
