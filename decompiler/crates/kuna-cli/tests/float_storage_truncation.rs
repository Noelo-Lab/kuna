//! A floating argument's integer storage bits survive a low-word extraction.
use crate::common;
use std::process::Command;

fn decoded(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn emitted_float_bit_truncation_matches_sparc_shifts() {
    let mut image = vec![0; 148];
    let shifts = decoded("9de3bf90b12e2010b13e201081c7e00881e80000");
    let numeric = decoded("9de3bf90f027bffcc307bffc87a01a21c727bffcf007bffc81c7e00881e80000");
    image[..shifts.len()].copy_from_slice(&shifts);
    image[64..64 + numeric.len()].copy_from_slice(&numeric);
    image[128..128 + shifts.len()].copy_from_slice(&shifts);
    let path = common::scratch_file("float-storage-truncation", "bin");
    std::fs::write(&path, image).unwrap();
    let mut source = String::new();
    for (policy, option) in [("default", None), ("off", Some("off")), ("on", Some("on"))] {
        for (address, name, signature) in [
            ("0x1000", "low_bits", "short low_bits(float value)"),
            ("0x1040", "numeric", "int numeric(float value)"),
            ("0x1080", "integer", "short integer(int value)"),
        ] {
            let full_name = format!("{name}_{policy}");
            let signature = signature.replace(name, &full_name);
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command
                .args([
                    "decompile",
                    path.to_str().unwrap(),
                    address,
                    "--raw-image",
                    "--base",
                    "0x1000",
                    "--target",
                    "sparc:BE:32:default",
                    "--define-function",
                ])
                .arg(format!("{address}={full_name}"))
                .args([
                    "--assert",
                    &format!("prototype {full_name} {signature}"),
                    "--assert-strict",
                    "--json",
                ]);
            if let Some(option) = option {
                command.args(["--option", "castnarrow", option]);
            }
            let output = command.output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{json}");
            source.push_str(function["code"].as_str().unwrap());
            source.push('\n');
        }
    }
    assert!(!source.contains("SUB"), "{source}");
    source.push_str(r#"
#include <stdint.h>
#include <string.h>
int main(void) {
    static const uint32_t samples[] = {511,8183,16350,0x7fff,0x8000,0xffff,0xffffdfff,0x3f812345,0xffc1abcd};
    for (unsigned i=0;i<sizeof samples/sizeof samples[0];++i) {
        uint32_t bits=samples[i]; float value; memcpy(&value,&bits,4);
        int expected=(int)(bits&0xffff); if (expected>=32768) expected-=65536;
        if (low_bits_default(value)!=expected || low_bits_off(value)!=expected || low_bits_on(value)!=expected || integer_default((int)bits)!=expected || integer_off((int)bits)!=expected || integer_on((int)bits)!=expected) return 1;
    }
    static const float values[]={-12.75f,-0.25f,0.0f,1.75f,32767.5f};
    static const int expected[]={-12,0,0,1,32767};
    for (unsigned i=0;i<sizeof values/sizeof values[0];++i)
        if(numeric_default(values[i])!=expected[i] || numeric_off(values[i])!=expected[i] || numeric_on(values[i])!=expected[i]) return 2;
    return 0;
}
"#);
    let c_path = common::scratch_file("float-storage-truncation", "c");
    std::fs::write(&c_path, source).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("float-storage-truncation", "exe");
            let build = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Wextra", "-Werror", optimization])
                .arg(&c_path)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{} {}: {}",
                compiler,
                optimization,
                String::from_utf8_lossy(&build.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{compiler} {optimization}: {run:?}");
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(c_path).unwrap();
}

#[test]
fn logical_integer_views_preserve_escaped_storage_and_call_effects() {
    let samples = [
        ("effect", "0x1100", "9de3bf80db064000db27bff89007bff8400003bc0100000090100018400004790100000095a0002097a000219ba01a4adb27bff8d007bff8912a2010913a2010400003f001000000db0640009ba3492ddb27bff89007bff8400003aa01000000d007bff8400004270100000081c7e00881e80000"),
        ("clobber", "0x1200", "9de3bf80db064000db27bff89007bff84000037c0100000090100018400004390100000095a0002097a000219ba01a4adb27bff89007bff84000047201000000d007bff8912a2010913a2010400003ad01000000db0640009ba3492ddb27bff89007bff84000036701000000d007bff8400003e40100000081c7e00881e80000"),
    ];
    let mut image = vec![0; 384];
    for (_, address, hex) in samples {
        let start = usize::from_str_radix(address.trim_start_matches("0x"), 16).unwrap() - 0x1100;
        let bytes = decoded(hex);
        image[start..start + bytes.len()].copy_from_slice(&bytes);
    }
    let path = common::scratch_file("logical-integer-views", "bin");
    std::fs::write(&path, image).unwrap();
    let mut source = String::from("#include <stdint.h>\n#include <string.h>\nvoid touch(float *);\nvoid sink_word(short);\nvoid sink_float(float);\ndouble sample(double *);\nvoid alter(float *);\n");
    let mut calls = Vec::new();
    for (kind, address, _) in samples {
        for feature in ["off", "on"] {
            for policy in ["off", "on"] {
                let name = format!("transport_{kind}_{feature}_{policy}");
                let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args([
                        "decompile",
                        path.to_str().unwrap(),
                        address,
                        "--raw-image",
                        "--base",
                        "0x1100",
                        "--target",
                        "sparc:BE:32:default",
                    ])
                    .args([
                        "--define-function",
                        "0x2000=touch",
                        "--define-function",
                        "0x2100=sink_word",
                        "--define-function",
                        "0x2200=sink_float",
                        "--define-function",
                        "0x2300=sample",
                        "--define-function",
                        "0x2400=alter",
                    ])
                    .args(["--define-function", &format!("{address}={name}")])
                    .args([
                        "--assert",
                        "prototype touch void touch(float *)",
                        "--assert",
                        "prototype sink_word void sink_word(short)",
                        "--assert",
                        "prototype sink_float void sink_float(float)",
                        "--assert",
                        "prototype sample double sample(double *)",
                        "--assert",
                        "prototype alter void alter(float *)",
                    ])
                    .args([
                        "--assert",
                        &format!("prototype {name} void {name}(double *,float *)"),
                        "--assert-strict",
                        "--option",
                        "fputruncviews",
                        feature,
                        "--option",
                        "castnarrow",
                        policy,
                        "--json",
                    ])
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                let function = &json["functions"][0];
                assert!(function["error"].is_null(), "{json}");
                let code = function["code"].as_str().unwrap();
                assert_eq!(code.matches("sample(").count(), 1, "{code}");
                if kind == "effect" && feature == "on" {
                    assert!(!code.contains("float from; unsigned int to;"), "{code}");
                    assert!(code.contains("(int)sample("), "{code}");
                }
                source.push_str(code);
                source.push('\n');
                calls.push(format!(
                    "run({name},{})",
                    if kind == "clobber" { 1 } else { 0 }
                ));
            }
        }
    }
    source.push_str(r#"
static float *escaped;
static float input_float;
static double last_sample;
static int sample_calls, touch_calls, word_calls, float_calls, clobbered, failed;
void touch(float *p) { escaped=p; ++touch_calls; }
double sample(double *p) { ++sample_calls; last_sample=*p; *p+=100; return last_sample; }
void alter(float *p) { uint32_t bits=0xffffffc7u; memcpy(p,&bits,4); clobbered=1; }
void sink_word(short value) {
    uint32_t bits; memcpy(&bits,escaped,4);
    uint32_t expected=clobbered?0xffffffc7u:(uint32_t)(int)last_sample;
    if(bits!=expected || value!=(short)(int)expected) failed=1;
    ++word_calls;
}
void sink_float(float value) { if(value!=input_float*input_float) failed=1; ++float_calls; escaped=0; }
static int run(void (*fn)(double *,float *),int modifies) {
    static const double values[]={-12345.75,-7.5,0.25,8183.75,65537.25};
    for(unsigned i=0;i<sizeof values/sizeof values[0];++i) {
        double value=values[i]; input_float=2.5f;
        sample_calls=touch_calls=word_calls=float_calls=clobbered=failed=0;
        fn(&value,&input_float);
        if(failed || sample_calls!=1 || touch_calls!=2 || word_calls!=1 || float_calls!=1 || clobbered!=modifies || value!=values[i]+100) return 1;
    }
    return 0;
}
int main(void) {
"#);
    for call in calls {
        source.push_str(&format!("if({call}) return 1;\n"));
    }
    source.push_str("return 0;\n}\n");
    let c_path = common::scratch_file("logical-integer-views", "c");
    std::fs::write(&c_path, source).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("logical-integer-views", "exe");
            let build = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Wextra", "-Werror", optimization])
                .arg(&c_path)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{compiler} {optimization}: {}",
                String::from_utf8_lossy(&build.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{compiler} {optimization}: {run:?}");
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(c_path).unwrap();
}

#[test]
fn mixed_phi_slot_orders_preserve_bits_before_alias_clobber() {
    let bodies = [
        ("0x1300", "9de3bf80db064000db27bff89007bff84000033c0100000080a6a0001280000601000000d51e00009ba01a4a1080000301000000db064000db27bff8f007bff8400004700100000081c7e00881e80000"),
        ("0x1400", "9de3bf80db064000db27bff89007bff8400002fc0100000080a6a0000280000501000000db0640001080000401000000d51e00009ba01a4adb27bff8f007bff8400004300100000081c7e00881e80000"),
    ];
    let mut image = vec![0; 336];
    for (address, hex) in bodies {
        let start = usize::from_str_radix(address.trim_start_matches("0x"), 16).unwrap() - 0x1300;
        let bytes = decoded(hex);
        image[start..start + bytes.len()].copy_from_slice(&bytes);
    }
    let path = common::scratch_file("mixed-phi-storage", "bin");
    std::fs::write(&path, image).unwrap();
    let mut source = String::from(
        "#include <stdint.h>\n#include <string.h>\nvoid touch(float *);void release(void);\n",
    );
    let mut names = Vec::new();
    for (index, (address, _)) in bodies.into_iter().enumerate() {
        for option in ["off", "on"] {
            let name = format!("phi_{index}_{option}");
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile",
                    path.to_str().unwrap(),
                    address,
                    "--raw-image",
                    "--base",
                    "0x1300",
                    "--target",
                    "sparc:BE:32:default",
                ])
                .args([
                    "--define-function",
                    "0x2000=touch",
                    "--define-function",
                    "0x2500=release",
                    "--define-function",
                    &format!("{address}={name}"),
                ])
                .args([
                    "--assert",
                    "prototype touch void touch(float *)",
                    "--assert",
                    "prototype release void release(void)",
                    "--assert",
                    &format!("prototype {name} unsigned int {name}(double *,float *,int)"),
                    "--assert-strict",
                    "--option",
                    "fputruncviews",
                    option,
                    "--json",
                ])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{json}");
            source.push_str(function["code"].as_str().unwrap());
            source.push('\n');
            names.push(name);
        }
    }
    source.push_str(
        r#"
static float *escaped;
static uint32_t expected;
static int failed;
void touch(float *p) { escaped=p; }
void release(void) {
    uint32_t bits; memcpy(&bits,escaped,4);
    if(bits!=expected) failed=1;
    bits=0; memcpy(escaped,&bits,4); escaped=0;
}
int main(void) {
    static const double numbers[]={-2000000000.75,-7.5,0.25,8183.75,65537.25};
    static const uint32_t bits[]={0,0x80000000u,0x3f812345u,0xff812345u};
    for(unsigned n=0;n<sizeof numbers/sizeof numbers[0];++n) {
        for(unsigned b=0;b<sizeof bits/sizeof bits[0];++b) {
            for(int branch=0;branch<2;++branch) {
                double value=numbers[n]; float floating; memcpy(&floating,&bits[b],4);
                expected=branch?bits[b]:(uint32_t)(int)value; failed=0;
"#,
    );
    for name in names {
        source.push_str(&format!(
            "if({name}(&value,&floating,branch)!=expected||failed) return 1;\n"
        ));
    }
    source.push_str("            }\n        }\n    }\n    return 0;\n}\n");
    let c_path = common::scratch_file("mixed-phi-storage", "c");
    std::fs::write(&c_path, source).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("mixed-phi-storage", "exe");
            let build = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Wextra", "-Werror", optimization])
                .arg(&c_path)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{compiler} {optimization}: {}",
                String::from_utf8_lossy(&build.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{compiler} {optimization}: {run:?}");
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(c_path).unwrap();
}

#[test]
fn an_integer_register_argument_reads_the_logical_integer() {
    let mut image = vec![0; 0x104];
    let body = decoded("00482de9048b2dedc09abdee608ab0ee100a19ee390000eb080a29ee048bbdec0088bde8");
    image[..body.len()].copy_from_slice(&body);
    image[0x100..].copy_from_slice(&decoded("1eff2fe1"));
    let path = common::scratch_file("integer-register-argument", "bin");
    std::fs::write(&path, image).unwrap();
    let mut source = String::from("#include <stdint.h>\n#include <string.h>\nvoid sinki(int);\n");
    for option in ["on", "off"] {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile",
                path.to_str().unwrap(),
                "0x1000",
                "--raw-image",
                "--base",
                "0x1000",
                "--target",
                "ARM:LE:32:v8",
                "--isa",
                "arm",
                "--define-function",
                &format!("0x1000=scale_{option}"),
                "--define-function",
                "0x1100=sinki",
                "--option",
                "fputruncviews",
                option,
                "--json",
            ])
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let code = json["functions"][0]["code"].as_str().unwrap();
        let call = code.lines().find(|line| line.contains("sinki(")).unwrap();
        assert!(!call.contains("union"), "{code}");
        if option == "on" {
            assert!(code.contains("unsigned int from; float to;"), "{code}");
        }
        source.push_str(code);
        source.push('\n');
    }
    source.push_str(
        r#"
static int seen;
void sinki(int value) { seen = value; }
static int check(float (*fn)(float, float)) {
    static const float values[] = {12345.75f, 3.5f, 0.25f, 7.0f, 100000.5f};
    for (unsigned i = 0; i < sizeof values / sizeof values[0]; ++i) {
        int bits = (int)values[i];
        float view, product;
        memcpy(&view, &bits, 4);
        product = view * 0.5f;
        seen = 0;
        float result = fn(values[i], 0.5f);
        if (seen != bits || memcmp(&result, &product, 4)) return 1;
    }
    return 0;
}
int main(void) { return check(scale_on) || check(scale_off); }
"#,
    );
    let c_path = common::scratch_file("integer-register-argument", "c");
    std::fs::write(&c_path, source).unwrap();
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file("integer-register-argument", "exe");
            let build = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Wextra", "-Werror", optimization])
                .arg(&c_path)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                build.status.success(),
                "{compiler} {optimization}: {}",
                String::from_utf8_lossy(&build.stderr)
            );
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{compiler} {optimization}: {run:?}");
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(c_path).unwrap();
}
