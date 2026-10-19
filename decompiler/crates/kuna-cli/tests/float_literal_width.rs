//! Printed C literals retain the precision used by floating-point operations.
#![cfg(target_os = "linux")]
use crate::common;
use object::{Object, ObjectSection};
use std::process::Command;

fn checked(command: &mut Command) {
    let output = command
        .output()
        .expect("start compiler or native comparison");
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn float_literals_recompile_at_the_original_precision() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| {
            Command::new(cc)
                .arg("--version")
                .output()
                .is_ok_and(|output| output.status.success())
        })
        .collect();
    assert!(!compilers.is_empty(), "a native C compiler is required");
    let root = common::scratch_file("float-literal-width", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(
        &source,
        r#"
int equal_tenth(float value) { return value == 0.1f; }
int greater_tenth(float value) { return value > 0.1f; }
float product(float value) { return value * 0.1f; }
float expression(float value,float scale) { return (value + 0.5f) * scale; }
double widened(float value) { return (double)value + 0.1; }
"#,
    )
    .unwrap();
    let original = root.join("sample.so");
    checked(
        Command::new(compilers[0])
            .args(["-O0", "-g", "-shared", "-fPIC", "-ffp-contract=off"])
            .arg(&source)
            .arg("-o")
            .arg(&original),
    );
    let bytes = std::fs::read(&original).unwrap();
    let object = object::File::parse(&*bytes).unwrap();
    let project = root.join("project");
    let specs = common::repo_root().join("specs");
    let mut arguments = vec![
        "decompile-project".to_string(),
        original.to_str().unwrap().to_string(),
        "--functions".into(),
        "equal_tenth,greater_tenth,product,expression,widened".into(),
        "--sleighpath".into(),
        specs.to_str().unwrap().into(),
        "--assert-strict".into(),
        "-o".into(),
        project.to_str().unwrap().into(),
    ];
    let mut ranges = 0;
    for section in object.sections() {
        if section.name().is_ok_and(|name| name.starts_with(".rodata")) && section.size() != 0 {
            arguments.extend([
                "--assert".into(),
                format!("readonly {:#x}+{}", section.address(), section.size()),
            ]);
            ranges += 1;
        }
    }
    assert!(
        ranges > 0,
        "the fixture must contain its floating constants"
    );
    let borrowed: Vec<_> = arguments.iter().map(String::as_str).collect();
    let (_, error, status) = common::run_kuna(&borrowed);
    assert_eq!(status, 0, "{error}");
    let generated = project.join("sample.so.c");
    let driver = root.join("driver.c");
    std::fs::write(
        &driver,
        r#"
#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
typedef int (*predicate)(float);
typedef float (*unary)(float);
typedef float (*binary)(float,float);
typedef double (*wide)(float);
static float real(uint32_t value) { float result;memcpy(&result,&value,4);return result; }
static uint32_t bits(float value) { uint32_t result;memcpy(&result,&value,4);return result; }
static uint64_t widebits(double value) { uint64_t result;memcpy(&result,&value,8);return result; }
int main(int argc,char **argv) {
    if(argc != 3)return 2;
    void *a=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL),*b=dlopen(argv[2],RTLD_NOW|RTLD_LOCAL);
    if(!a||!b) { fprintf(stderr,"%s\n",dlerror());return 3; }
    predicate oe=(predicate)dlsym(a,"equal_tenth"),ne=(predicate)dlsym(b,"equal_tenth");
    predicate og=(predicate)dlsym(a,"greater_tenth"),ng=(predicate)dlsym(b,"greater_tenth");
    unary op=(unary)dlsym(a,"product"),np=(unary)dlsym(b,"product");
    binary ox=(binary)dlsym(a,"expression"),nx=(binary)dlsym(b,"expression");
    wide ow=(wide)dlsym(a,"widened"),nw=(wide)dlsym(b,"widened");
    if(!oe||!ne||!og||!ng||!op||!np||!ox||!nx||!ow||!nw)return 4;
    const uint32_t edges[]={0,0x80000000u,1,0x007fffffu,0x00800000u,
        0x3dcccccdu,0x3dccccccu,0x3dccccceu,0xbdcccccdu,
        0x7f7fffffu,0xff7fffffu,0x3f000000u,0xbf000000u,
        0x7f800000u,0xff800000u,0x7fc12345u,0x7f812345u};
    uint32_t state=0x1234abcd;
    unsigned count=0;
    for(unsigned i=0;i<1041;i++) {
        state=state*1664525u+1013904223u;
        float value=real(i<17?edges[i]:state);
        if(oe(value)!=ne(value) || og(value)!=ng(value))return 5;
        if(bits(op(value))!=bits(np(value)))return 6;
        if(bits(ox(value,1.25f))!=bits(nx(value,1.25f)))return 7;
        if(widebits(ow(value))!=widebits(nw(value)))return 8;
        count+=5;
    }
    printf("%u comparisons passed\n",count);
    return 0;
}
"#,
    )
    .unwrap();
    for cc in compilers {
        for opt in ["-O0", "-O2"] {
            let rebuilt = root.join(format!("rebuilt-{cc}-{opt}.so"));
            checked(
                Command::new(cc)
                    .args([
                        "-std=gnu11",
                        opt,
                        "-shared",
                        "-fPIC",
                        "-fno-fast-math",
                        "-ffp-contract=off",
                    ])
                    .arg(&generated)
                    .arg("-o")
                    .arg(&rebuilt),
            );
            let executable = root.join(format!("driver-{cc}-{opt}"));
            checked(
                Command::new(cc)
                    .args(["-std=gnu11", opt, "-fno-fast-math", "-ffp-contract=off"])
                    .arg(&driver)
                    .args(["-ldl", "-o"])
                    .arg(&executable),
            );
            checked(Command::new(&executable).arg(&original).arg(&rebuilt));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
