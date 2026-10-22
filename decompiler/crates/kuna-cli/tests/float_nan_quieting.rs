//! Format conversion round trips quiet signaling NaNs without changing payloads;
//! `nanignore none` keeps that effect, the default `compare` policy drops it.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn float_format_roundtrips_preserve_native_nan_quieting() {
    let compilers: Vec<_> = ["clang", "gcc"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        return;
    }
    let dir = common::scratch_file("float-nan-quieting", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let original = dir.join("original.c");
    std::fs::write(
        &original,
        r#"
__attribute__((naked)) unsigned quiet32(unsigned bits) {
 __asm__("movl %edi,-4(%rsp);flds -4(%rsp);fstps -4(%rsp);movl -4(%rsp),%eax;retq");
}
__attribute__((naked)) unsigned long quiet64(unsigned long bits) {
 __asm__("movq %rdi,-8(%rsp);fldl -8(%rsp);fstpl -8(%rsp);movq -8(%rsp),%rax;retq");
}
__attribute__((naked)) unsigned quiet_sse32(unsigned bits) {
 __asm__("movd %edi,%xmm0;cvtss2sd %xmm0,%xmm0;cvtsd2ss %xmm0,%xmm0;movd %xmm0,%eax;retq");
}
__attribute__((naked)) unsigned quiet_merge(unsigned a, unsigned b, int c) {
 __asm__("pushq %rbp;movq %rsp,%rbp;testl %edx,%edx;je 1f;movd %edi,%xmm0;cvtss2sd %xmm0,%xmm0;"
   "movsd %xmm0,-8(%rbp);jmp 2f;1:movd %esi,%xmm0;cvtss2sd %xmm0,%xmm0;movsd %xmm0,-8(%rbp);"
   "2:pxor %xmm0,%xmm0;cvtsd2ss -8(%rbp),%xmm0;movd %xmm0,%eax;popq %rbp;retq");
}
"#,
    )
    .unwrap();
    let native = dir.join("sample.so");
    process::required_output(
        Command::new(compilers[0])
            .args(["-shared", "-fPIC", "-O0"])
            .arg(&original)
            .arg("-o")
            .arg(&native),
    );
    let driver = dir.join("driver.c");
    std::fs::write(&driver, r#"
#include <fenv.h>
#include <stdint.h>
#include <stdio.h>
unsigned quiet32(unsigned),printed_quiet32(unsigned);
unsigned quiet_sse32(unsigned),printed_quiet_sse32(unsigned);
unsigned quiet_merge(unsigned,unsigned,int),printed_quiet_merge(unsigned,unsigned,int);
unsigned long quiet64(unsigned long),printed_quiet64(unsigned long);
int main(void) {
 const unsigned values32[]={0,0x80000000u,1,0x80000001u,0x3f800000u,0xbf800000u,
   0x7f800000u,0xff800000u,0x7f800001u,0xff800001u,0x7fc00001u,0xffc00001u};
 const uint64_t values64[]={0,0x8000000000000000ull,1,0x8000000000000001ull,
   0x3ff0000000000000ull,0xbff0000000000000ull,0x7ff0000000000000ull,
   0xfff0000000000000ull,0x7ff0000000000001ull,0xfff0000000000001ull,
   0x7ff8000000000001ull,0xfff8000000000001ull};
 const int rounds[]={FE_TONEAREST,FE_DOWNWARD,FE_UPWARD,FE_TOWARDZERO};
 for(unsigned mode=0;mode<4;mode++) {
  if(fesetround(rounds[mode]))return 2;
  for(unsigned i=0;i<4108;i++) {
   unsigned x=i<12?values32[i]:(0x7f800001u|((i*2654435761u)&0x803fffffu));
   uint64_t y=i<12?values64[i]:(0x7ff0000000000001ull|((i*11400714819323198485ull)&0x8007ffffffffffffull));
   unsigned a=quiet32(x),b=printed_quiet32(x);
   uint64_t c=quiet64(y),d=printed_quiet64(y);
   unsigned e=quiet_sse32(x),f=printed_quiet_sse32(x);
   unsigned g=quiet_merge(x,~x,i&1),h=printed_quiet_merge(x,~x,i&1);
   if(a!=b||c!=d||e!=f||g!=h) {
    fprintf(stderr,"round=%u case=%u float=%08x/%08x double=%016llx/%016llx sse=%08x/%08x merge=%08x/%08x\n",
      mode,i,a,b,(unsigned long long)c,(unsigned long long)d,e,f,g,h);return 1;
   }
  }
 }
 return 0;
}
"#).unwrap();
    let (default, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        native.to_str().unwrap(),
        "--functions",
        "quiet32,quiet64,quiet_sse32,quiet_merge",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{default}");
    assert!(!default.contains("volatile"), "the default nanignore compare drops quieting\n{default}");
    for policy in ["none"] {
        let (printed, stderr, rc) = common::run_kuna(&[
            "decompile-all",
            native.to_str().unwrap(),
            "--functions",
            "quiet32,quiet64,quiet_sse32,quiet_merge",
            "--option",
            "nanignore",
            policy,
            "--assert",
            "prototype quiet32 unsigned int quiet32(unsigned int bits)",
            "--assert",
            "prototype quiet64 unsigned long quiet64(unsigned long bits)",
            "--assert",
            "prototype quiet_sse32 unsigned int quiet_sse32(unsigned int bits)",
            "--assert",
            "prototype quiet_merge unsigned int quiet_merge(unsigned int a,unsigned int b,int c)",
            "--assert-strict",
        ]);
        assert_eq!(rc, 0, "{stderr}\n{printed}");
        let regenerated = dir.join(format!("regenerated-{policy}.c"));
        std::fs::write(&regenerated,format!("typedef float float4;typedef double float8;typedef long double float10;\n#define quiet32 printed_quiet32\n#define quiet64 printed_quiet64\n#define quiet_sse32 printed_quiet_sse32\n#define quiet_merge printed_quiet_merge\n{printed}")).unwrap();
        for cc in &compilers {
            for opt in ["-O0", "-O2"] {
                let exe = dir.join(format!("check-{policy}-{cc}-{opt}"));
                process::required_output(
                    Command::new(cc)
                        .args([
                            opt,
                            "-fno-fast-math",
                            "-frounding-math",
                            "-ffp-contract=off",
                        ])
                        .arg(&regenerated)
                        .arg(&driver)
                        .arg(&native)
                        .arg("-lm")
                        .arg(format!("-Wl,-rpath,{}", dir.display()))
                        .arg("-o")
                        .arg(&exe),
                );
                process::required_output(&mut Command::new(exe));
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
