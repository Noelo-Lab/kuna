//! Freeing a stack-register tag must not invent a floating payload read.
#![cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn ffree_preserves_each_register_identity_without_numeric_inputs() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("x87-free-operands", "s");
    let binary = source.with_extension("dll");
    let assembly: String = (0..8).map(|i| format!(
        ".text\n.globl _sample{i}\n.def _sample{i}; .scl 2; .type 32; .endef\n_sample{i}:\n ffree %st({i})\n mov $7,%eax\n ret\n"
    )).collect();
    std::fs::write(&source, assembly).unwrap();
    let output = Command::new("clang")
        .args([
            "--target=i686-w64-windows-gnu",
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut recovered = String::new();
    for i in 0..8 {
        let name = format!("_sample{i}");
        let prototype = format!("prototype {name} unsigned int {name}(void)");
        let (body, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            &name,
            "--assert",
            &prototype,
        ]);
        assert_eq!(status, 0, "{err}");
        assert!(body.contains(&format!("ffree(ST{i})")), "{body}");
        assert!(!body.contains("undefined10"), "{body}");
        assert!(body.contains("return 7;"), "{body}");
        recovered.push_str(&body);
    }
    let mut header = String::from("#include <string.h>\n#define ffree(reg) free_ ## reg()\n");
    for i in 0..8 {
        header.push_str(&format!(
            "static inline void free_ST{i}(void) {{ __asm__ volatile(\"ffree %%st({i})\" ::: \"memory\"); }}\nstatic unsigned reference{i}(void) {{ __asm__ volatile(\"ffree %%st({i})\" ::: \"memory\"); return 7; }}\n"
        ));
    }
    let native = source.with_extension("c");
    let executable = source.with_extension("native");
    let actual = (0..8)
        .map(|i| format!("_sample{i}"))
        .collect::<Vec<_>>()
        .join(",");
    let expected = (0..8)
        .map(|i| format!("reference{i}"))
        .collect::<Vec<_>>()
        .join(",");
    let harness = r#"
static const long double payload = 1.25L;
static void initialize(unsigned count) {
  __asm__ volatile("fninit" ::: "memory");
  for(unsigned i=0;i<count;i++)
    __asm__ volatile("fldt %0" :: "m"(payload) : "memory");
}
struct snapshot { unsigned char bytes[108]; };
static void capture(struct snapshot *s) {
  __asm__ volatile("fnsave %0" : "=m"(*s) :: "memory");
}
static unsigned word(const unsigned char *p) { return p[0] | ((unsigned)p[1]<<8); }
int main(void) {
  unsigned (*actual[8])(void) = { ACTUAL_FUNCTIONS };
  unsigned (*expected[8])(void) = { EXPECTED_FUNCTIONS };
  for(unsigned count=0;count<=8;count++) for(unsigned i=0;i<8;i++) {
    struct snapshot a,b;
    initialize(count); unsigned ra=actual[i](); capture(&a);
    initialize(count); unsigned rb=expected[i](); capture(&b);
    if(ra!=rb || word(a.bytes+8)!=word(b.bytes+8) ||
       ((word(a.bytes+4)^word(b.bytes+4)) & 0x3800) ||
       memcmp(a.bytes+28,b.bytes+28,80)) return 1;
  }
  return 0;
}
"#
    .replace("ACTUAL_FUNCTIONS", &actual)
    .replace("EXPECTED_FUNCTIONS", &expected);
    std::fs::write(&native, format!("{header}{recovered}{harness}")).unwrap();
    let mut ran = false;
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let output = Command::new(cc)
                .args(["-std=gnu11", "-Wall", "-Werror", optimization])
                .arg(&native)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(Command::new(&executable).status().unwrap().success());
            ran = true;
        }
    }
    assert!(ran, "a host C compiler is required");
    std::fs::remove_file(native).unwrap();
    std::fs::remove_file(executable).unwrap();
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(binary).unwrap();
}
