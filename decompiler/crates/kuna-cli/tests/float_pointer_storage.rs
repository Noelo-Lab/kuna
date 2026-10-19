//! Same-width float/pointer storage transfers must preserve the representation.
#![cfg(all(target_os = "linux", target_pointer_width = "64"))]
use crate::common;
use std::process::Command;

fn checked(command: &mut Command) {
    let output = command
        .output()
        .expect("start compiler or differential driver");
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn float_pointer_storage_round_trips() {
    round_trip(false);
}

#[test]
fn float_pointer_storage_keeps_required_casts() {
    round_trip(true);
}

fn round_trip(nocasts: bool) {
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
    let root = common::scratch_file("float-pointer-storage", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    std::fs::write(
        &source,
        r#"
union bits { double value; void *pointer; };
void *to_pointer(double value) {
    union bits temporary; temporary.value=value; return temporary.pointer;
}
double from_pointer(void *pointer) {
    union bits temporary; temporary.pointer=pointer; return temporary.value;
}
void *sum_pointer(double a,double b) {
    union bits temporary; temporary.value=a+b; return temporary.pointer;
}
"#,
    )
    .unwrap();
    let original = root.join("sample.so");
    checked(
        Command::new(compilers[0])
            .args(["-O0", "-g", "-shared", "-fPIC"])
            .arg(&source)
            .arg("-o")
            .arg(&original),
    );
    let project = root.join("project");
    let specs = common::repo_root().join("specs");
    let mut args = vec![
        "decompile-project",
        original.to_str().unwrap(),
        "--functions",
        "to_pointer,from_pointer,sum_pointer",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--assert",
        "prototype to_pointer void *to_pointer(double value)",
        "--assert",
        "prototype from_pointer double from_pointer(void *pointer)",
        "--assert",
        "prototype sum_pointer void *sum_pointer(double a,double b)",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ];
    if nocasts {
        args.extend(["--option", "nocastprinting", "on"]);
    }
    let (_, error, status) = common::run_kuna(&args);
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
_Static_assert(sizeof(void *) == sizeof(double), "equal storage widths");
typedef void *(*to_fn)(double);
typedef double (*from_fn)(void *);
typedef void *(*sum_fn)(double,double);
static uint64_t bits(double value) { uint64_t result; memcpy(&result,&value,8); return result; }
static double real(uint64_t value) { double result; memcpy(&result,&value,8); return result; }
int main(int argc,char **argv) {
    if(argc != 3) return 2;
    void *a=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL), *b=dlopen(argv[2],RTLD_NOW|RTLD_LOCAL);
    if(!a || !b) { fprintf(stderr,"%s\n",dlerror()); return 3; }
    to_fn oa=(to_fn)dlsym(a,"to_pointer"), na=(to_fn)dlsym(b,"to_pointer");
    from_fn ob=(from_fn)dlsym(a,"from_pointer"), nb=(from_fn)dlsym(b,"from_pointer");
    sum_fn oc=(sum_fn)dlsym(a,"sum_pointer"), nc=(sum_fn)dlsym(b,"sum_pointer");
    if(!oa || !na || !ob || !nb || !oc || !nc) return 4;
    const uint64_t edges[]={0,UINT64_C(0x8000000000000000),1,UINT64_MAX,
        UINT64_C(0x7ff0000000000000),UINT64_C(0xfff0000000000000),
        UINT64_C(0x7ff8000012345678),UINT64_C(0x7ff0000000000001),
        UINT64_C(0x3ff0000000000000),UINT64_C(0xbff0000000000000)};
    uint64_t state=UINT64_C(0xabcde1234567890);
    unsigned count=0;
    for(unsigned i=0;i<1034;i++) {
        state=state*UINT64_C(6364136223846793005)+1;
        uint64_t word=i<10?edges[i]:state;
        double value=real(word);
        if((uintptr_t)oa(value)!=(uintptr_t)na(value)) return 5;
        if(bits(ob((void *)(uintptr_t)word))!=bits(nb((void *)(uintptr_t)word))) return 6;
        /* Finite arithmetic checks expression grouping without NaN payload selection. */
        double x=(double)(int32_t)state/16.0, y=(double)(int32_t)(state>>32)/32.0;
        if((uintptr_t)oc(x,y)!=(uintptr_t)nc(x,y)) return 7;
        count+=3;
    }
    printf("%u storage comparisons passed\n",count);
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
                    .args(["-std=gnu11", opt, "-shared", "-fPIC", "-fno-fast-math"])
                    .arg(&generated)
                    .arg("-o")
                    .arg(&rebuilt),
            );
            let executable = root.join(format!("driver-{cc}-{opt}"));
            checked(
                Command::new(cc)
                    .args(["-std=gnu11", opt, "-fno-fast-math"])
                    .arg(&driver)
                    .args(["-ldl", "-o"])
                    .arg(&executable),
            );
            checked(Command::new(&executable).arg(&original).arg(&rebuilt));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
