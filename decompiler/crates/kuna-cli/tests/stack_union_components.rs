//! Interior scalar declarations must coexist with a declared union array view.
#![cfg(all(target_os = "linux", target_pointer_width = "64"))]
use crate::common;
use std::io::Write;
use std::process::{Command, Stdio};

fn checked(command: &mut Command) {
    let output = command.output().expect("run compiler or native comparison");
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn union_array_interior_keeps_later_stack_references_declared() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| {
            Command::new(cc)
                .arg("--version")
                .output()
                .is_ok_and(|o| o.status.success())
        })
        .collect();
    assert!(!compilers.is_empty(), "a native C compiler is required");
    let dir = common::scratch_file("stack-union-components", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let assembly = dir.join("sample.s");
    std::fs::write(&assembly, include_str!("fixtures/stack_union_components.s")).unwrap();
    let original = dir.join("original.so");
    checked(
        Command::new(compilers[0])
            .args(["-shared", "-fPIC", "-Wl,-Bsymbolic-functions"])
            .arg(&assembly)
            .arg("-o")
            .arg(&original),
    );
    let engine = std::env::var_os("KUNA_DECOMP_DBG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_BIN_EXE_kuna"))
                .parent()
                .unwrap()
                .join("decomp_dbg")
        });
    let specs = std::env::var_os("SLEIGHHOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"));
    let mut child = Command::new(engine)
        .arg("-s")
        .arg(specs)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run decompiler console");
    let script = format!(
        "load file \"{}\"\noption stackviews off\n\
         parse line typedef union PairBacking {{unsigned long long bits;float elements[2];}} PairStorage;\n\
         map prototype sample void sample(float *input,float *output)\n\
         map prototype combine float combine(float *left,float *right)\n\
         load function sample\n\
         map addr s0xffffffffffffffc8 PairStorage storage\n\
         map addr s0xffffffffffffffcc float neighbor\n\
         decompile\nprint C\nquit\n", original.display());
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let transcript = String::from_utf8(output.stdout).unwrap();
    assert!(!transcript.contains("Execution error:"), "{transcript}");
    let code = transcript
        .split_once("\nvoid sample(")
        .expect("sample declaration")
        .1;
    let code = format!("void sample({}", code.split("\n[decomp]>").next().unwrap());
    assert!(!code.contains("Stack"), "{code}");
    let generated = dir.join("generated.c");
    std::fs::write(
        &generated,
        format!(
            "typedef float float4; typedef long double float10;\n\
         extern float combine(float *,float *);\n{code}\n"
        ),
    )
    .unwrap();
    let driver = dir.join("driver.c");
    std::fs::write(
        &driver,
        r#"
#include <dlfcn.h>
#include <stdint.h>
#include <string.h>
typedef void (*sample_fn)(float *,float *);
float combine(float *a,float *b) {return *a+*b;}
int main(int argc,char **argv) {
    if(argc!=3)return 2;
    void *a=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL),*b=dlopen(argv[2],RTLD_NOW|RTLD_LOCAL);
    if(!a||!b)return 3;
    sample_fn reference=(sample_fn)dlsym(a,"sample"),rebuilt=(sample_fn)dlsym(b,"sample");
    if(!reference||!rebuilt)return 4;
    for(int i=-2048;i<2048;i++) {
        float input=(float)i/16.0f,x=0,y=0;
        reference(&input,&x);rebuilt(&input,&y);
        uint32_t bx,by;memcpy(&bx,&x,4);memcpy(&by,&y,4);
        if(bx!=by)return 5;
    }
    return 0;
}
"#,
    )
    .unwrap();
    for cc in compilers {
        let executable = dir.join(format!("driver-{cc}"));
        checked(
            Command::new(cc)
                .args(["-std=gnu11", "-O2", "-rdynamic"])
                .arg(&driver)
                .arg("-ldl")
                .arg("-o")
                .arg(&executable),
        );
        for optimization in ["-O0", "-O2"] {
            let rebuilt = dir.join(format!("rebuilt-{cc}{optimization}.so"));
            checked(
                Command::new(cc)
                    .args(["-std=gnu11", "-shared", "-fPIC", optimization])
                    .arg(&generated)
                    .arg("-o")
                    .arg(&rebuilt),
            );
            checked(Command::new(&executable).arg(&original).arg(&rebuilt));
        }
    }
}
