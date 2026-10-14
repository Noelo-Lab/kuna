use crate::common;
use common::process;
use std::process::Command;

fn check_guarded_stack_read(target: &str, code: &str, qualifier: &str, preloaded: bool) {
    let image = common::scratch_file("guarded-local-read", "bin");
    let mut bytes = vec![0u8; 0x301];
    for (i, pair) in code.as_bytes().chunks_exact(2).enumerate() {
        bytes[i] = u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
    }
    if target.starts_with("sparc") {
        bytes.resize(0x308, 0);
    }
    for offset in [0x100, 0x200, 0x300] {
        if target.starts_with("sparc") {
            bytes[offset..offset + 8].copy_from_slice(&[0x81, 0xc3, 0xe0, 0x08, 0x01, 0, 0, 0]);
        } else {
            bytes[offset] = 0xc3;
        }
    }
    if preloaded {
        if target.starts_with("sparc") {
            bytes[12..28].copy_from_slice(&[
                0xf0, 0x07, 0xbf, 0xfc, 0x80, 0xa2, 0x20, 0, 0x02, 0x48, 0, 0x08, 0x01, 0, 0, 0,
            ]);
        } else {
            bytes[15..24].copy_from_slice(&[0x8b, 0x5c, 0x24, 0x0c, 0x66, 0x85, 0xc0, 0x74, 0x19]);
        }
    }
    std::fs::write(&image, bytes).unwrap();
    let specs = common::repo_root().join("specs");
    let acquire_proto = format!("prototype acquire short acquire({qualifier}int *)");
    let mutate_proto = format!("prototype mutate void mutate({qualifier}int *)");
    let (decompiled, stderr, status) = common::run_kuna(&[
        "decompile",
        image.to_str().unwrap(),
        "0x1000",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        target,
        "--sleighpath",
        specs.to_str().unwrap(),
        "--define-function",
        "0x1000=guarded",
        "--define-function",
        "0x1100=acquire",
        "--define-function",
        "0x1200=mutate",
        "--define-function",
        "0x1300=consume",
        "--assert",
        "prototype guarded int guarded(void)",
        "--assert",
        &acquire_proto,
        "--assert",
        &mutate_proto,
        "--assert",
        "prototype consume void consume(int)",
        "--assert-strict",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let source = common::scratch_file("guarded-local-read", "c");
    std::fs::write(&source, format!("int success, consumed, calls, bad;\nshort acquire({qualifier}int *p) {{ if(success) *p=42; return success; }}\nvoid mutate({qualifier}int *p) {{ calls++; if(*p!=23 && *p!=7) bad=1; }}\nvoid consume(int value) {{ consumed=value; }}\n{decompiled}\nint main(void) {{ for(success=0;success<=1;success++) {{ consumed=0;calls=0; if(guarded()!=7 || consumed!=(success?42:0) || calls!=(success?2:1) || bad) return 1; }} return 0; }}\n")).unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let exe = common::scratch_file("guarded-local-read", "exe");
            process::required_output(
                Command::new(cc)
                    .args([opt, "-x", "c"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe),
            );
            assert!(
                Command::new(&exe).status().unwrap().success(),
                "{cc} {opt}: {decompiled}"
            );
        }
    }
}

#[test]
fn guarded_x86_stack_capture_preserves_call_effects() {
    for qualifier in ["", "volatile "] {
        for preloaded in [false, true] {
            check_guarded_stack_read("x86:LE:64:default:gcc", "534883ec10488d7c240ce8f10000006685c0741d8b5c240cc744240c17000000488d7c240ce8d601000089dfe8cf020000c744240c07000000488d7c240ce8bd0100008b44240c4883c4105bc3", qualifier, preloaded);
        }
    }
}

#[test]
fn guarded_sparc_stack_capture_preserves_call_effects() {
    for qualifier in ["", "volatile "] {
        for preloaded in [false, true] {
            check_guarded_stack_read("sparc:BE:32:default", "9de3bfa04000003f9007bffc80a220000248000901000000f007bffcb2102017f227bffc400000779007bffc400000b590100018b0102007f027bffc400000719007bffcf007bffc81c7e00881e80000", qualifier, preloaded);
        }
    }
}
