use crate::common;
use common::process;
use std::process::Command;

#[test]
fn an_impure_getter_observes_the_global_before_the_later_store() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let image = common::scratch_file("global-load-call-order", "bin");
    let mut bytes = vec![0u8; 0x2110];
    let caller: [u8; 64] = [
        0x9d, 0xe3, 0xbf, 0xa0, 0x21, 0x00, 0x00, 0x08, 0xd1, 0x04, 0x20, 0x00, 0xd1, 0x27, 0xbf,
        0xfc, 0x40, 0x00, 0x00, 0x3c, 0x01, 0x00, 0x00, 0x00, 0xd1, 0x07, 0xbf, 0xfc, 0x95, 0xa2,
        0x09, 0xa0, 0x25, 0x00, 0x00, 0x0c, 0xd5, 0x24, 0xa0, 0x00, 0x27, 0x00, 0x00, 0x0c, 0xd1,
        0x24, 0xe0, 0x04, 0x40, 0x00, 0x00, 0x74, 0x01, 0x00, 0x00, 0x00, 0x81, 0xc7, 0xe0, 0x08,
        0x81, 0xe8, 0x00, 0x00,
    ];
    bytes[..64].copy_from_slice(&caller);
    let mut computed = caller.to_vec();
    computed.splice(12..12, [0x91, 0xa2, 0x08, 0x28]);
    computed[20..24].copy_from_slice(&[0x7f, 0xff, 0xfe, 0x3b]);
    computed[52..56].copy_from_slice(&[0x7f, 0xff, 0xfe, 0x73]);
    bytes[0x800..0x800 + computed.len()].copy_from_slice(&computed);
    let loop_clear: [u8; 92] = [
        0x9d, 0xe3, 0xbf, 0xa0, 0x21, 0x00, 0x00, 0x08, 0xd1, 0x04, 0x20, 0x00, 0xd1, 0x27, 0xbf,
        0xfc, 0x7f, 0xff, 0xff, 0x3c, 0x01, 0x00, 0x00, 0x00, 0xd1, 0x07, 0xbf, 0xfc, 0x95, 0xa2,
        0x09, 0xa0, 0x25, 0x00, 0x00, 0x0c, 0xd5, 0x24, 0xa0, 0x00, 0x27, 0x00, 0x00, 0x0c, 0xd1,
        0x24, 0xe0, 0x04, 0xa8, 0x10, 0x00, 0x00, 0x2b, 0x00, 0x00, 0x0c, 0xaa, 0x15, 0x61, 0x00,
        0xad, 0x2d, 0x20, 0x02, 0xc0, 0x25, 0x40, 0x16, 0xa8, 0x05, 0x20, 0x01, 0x80, 0xa5, 0x20,
        0x03, 0x04, 0xbf, 0xff, 0xfc, 0x01, 0x00, 0x00, 0x00, 0x81, 0xc7, 0xe0, 0x08, 0x81, 0xe8,
        0x00, 0x00,
    ];
    bytes[0x400..0x400 + 92].copy_from_slice(&loop_clear);
    for offset in [0x100, 0x200] {
        bytes[offset..offset + 8].copy_from_slice(&[0x81, 0xc3, 0xe0, 0x08, 0x01, 0, 0, 0]);
    }
    bytes[0x1000..0x1004].copy_from_slice(&0x3f800000u32.to_be_bytes());
    std::fs::write(&image, bytes).unwrap();
    let specs = common::repo_root().join("specs");
    for (entry, name, expected_result, expected_after) in [
        ("0x1000", "caller", "0.5f", "1.0f"),
        ("0x1400", "loop_clear", "0.5f", "1.0f"),
        ("0x1800", "computed", "1.0f", "2.0f"),
    ] {
        let (code, stderr, status) = common::run_kuna(&[
            "decompile",
            image.to_str().unwrap(),
            entry,
            "--raw-image",
            "--base",
            "0x1000",
            "--target",
            "sparc:BE:32:default",
            "--mode",
            "reliable",
            "--option",
            "globalarrayanchor",
            "on",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--define-function",
            "0x1000=caller",
            "--define-function",
            "0x1400=loop_clear",
            "--define-function",
            "0x1800=computed",
            "--define-function",
            "0x1100=get",
            "--define-function",
            "0x1200=finish",
            "--assert",
            "prototype 0x1000 void caller(void)",
            "--assert",
            "prototype 0x1400 void loop_clear(void)",
            "--assert",
            "prototype 0x1800 void computed(void)",
            "--assert",
            "prototype get float get(void)",
            "--assert",
            "prototype finish void finish(void)",
            "--assert",
            "data 0x2000 float input",
            "--assert",
            "data 0x3000 float result",
            "--assert",
            "data 0x3004 float after",
            "--assert",
            "data 0x3100 unsigned int scratch[4]",
            "--assert-strict",
        ]);
        assert_eq!(status, 0, "{stderr}");
        let source = common::scratch_file("global-load-call-order", "c");
        let check_scratch = if name == "loop_clear" {
            "for (unsigned i=0;i<4;i++) if(scratch[i]!=0) return 1;"
        } else {
            ""
        };
        std::fs::write(&source, format!("typedef float float4;\ntypedef int int4;\nconst float input=1.0f;\nfloat result, after, observed;\nunsigned int scratch[4];\nvolatile unsigned calls;\nfloat get(void) {{ calls++; observed=after; return 2.0f; }}\nvoid finish(void) {{}}\n{code}\nint main(void) {{ for (unsigned i=0;i<4;i++) scratch[i]=7; {name}(); {check_scratch} return calls!=1 || observed!=0.0f || result!={expected_result} || after!={expected_after}; }}\n")).unwrap();
        for &cc in &compilers {
            for opt in ["-O0", "-O2"] {
                let exe = common::scratch_file("global-load-call-order", "exe");
                process::required_output(
                    Command::new(cc)
                        .args([opt, "-x", "c"])
                        .arg(&source)
                        .arg("-o")
                        .arg(&exe),
                );
                let result = Command::new(&exe).output().unwrap();
                assert!(result.status.success(), "{cc} {opt}: {code}");
            }
        }
    }
}

#[test]
fn a_vector_copy_captures_both_halves_before_an_aliasing_global_store() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let image = common::scratch_file("global-vector-load-order", "bin");
    let mut bytes = vec![0u8; 0x2040];
    let copy: [u8; 30] = [
        0xf3, 0x0f, 0x6f, 0x07, 0x66, 0x0f, 0x7f, 0x05, 0x04, 0x20, 0x00, 0x00, 0xf3, 0x0f, 0x6f,
        0x47, 0x10, 0x66, 0x0f, 0x7f, 0x05, 0x07, 0x20, 0x00, 0x00, 0xe9, 0xe2, 0x00, 0x00, 0x00,
    ];
    bytes[..30].copy_from_slice(&copy);
    bytes[0x100] = 0xc3;
    std::fs::write(&image, bytes).unwrap();
    let specs = common::repo_root().join("specs");
    let (code, stderr, status) = common::run_kuna(&[
        "decompile",
        image.to_str().unwrap(),
        "0x1000",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        "x86:LE:64:default:gcc",
        "--mode",
        "reliable",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--define-function",
        "0x1000=copy",
        "--define-function",
        "0x1100=finish",
        "--assert",
        "prototype copy void copy(unsigned long long *p)",
        "--assert",
        "prototype finish void finish(void)",
        "--assert",
        "data 0x3000 unsigned long long state[8]",
        "--assert-strict",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let source = common::scratch_file("global-vector-load-order", "c");
    std::fs::write(
        &source,
        format!(
            "typedef unsigned long long uint8;\nuint8 state[8];\nvoid finish(void) {{}}\n{code}\n\
         int main(void) {{\n\
           const uint8 initial[8]={{10,20,30,40,50,60,70,80}};\n\
           const uint8 expected[8]={{10,20,20,30,30,50,70,80}};\n\
           for (unsigned i=0;i<8;i++) state[i]=initial[i];\n\
           copy(state+1);\n\
           for (unsigned i=0;i<8;i++) if(state[i]!=expected[i]) return 1;\n\
           return 0;\n}}\n"
        ),
    )
    .unwrap();
    for &cc in &compilers {
        for opt in ["-O0", "-O2"] {
            let exe = common::scratch_file("global-vector-load-order", "exe");
            process::required_output(
                Command::new(cc)
                    .args([opt, "-x", "c"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe),
            );
            let result = Command::new(&exe).output().unwrap();
            assert!(result.status.success(), "{cc} {opt}: {code}");
        }
    }
}
