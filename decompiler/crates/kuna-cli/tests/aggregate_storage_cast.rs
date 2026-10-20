//! Recovered aggregate arguments retain the bits of their integer carriers.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn integer_carrier_to_record_preserves_both_words() {
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("aggregate-storage-cast", "c");
    let binary = source.with_extension("so");
    std::fs::write(
        &source,
        r#"
struct words { unsigned int low, high; };
__attribute__((noinline)) static struct words consume(struct words x) { x.low += 1; return x; }
struct words pack(unsigned int x) { return consume((struct words){0,x}); }
unsigned long unpack(struct words x) { unsigned long value; __builtin_memcpy(&value,&x,sizeof value); return value; }
struct box { unsigned long tag; struct words payload; };
unsigned long payload_bits(struct box *x) { unsigned long value; __builtin_memcpy(&value,&x->payload,sizeof value); return value; }
struct pair { float real, imag; };
unsigned long unpack_float(struct pair x) { unsigned long value; __builtin_memcpy(&value,&x,sizeof value); return value; }
struct pair pack_float(unsigned long x) { struct pair value; __builtin_memcpy(&value,&x,sizeof value); return value; }
__attribute__((noinline)) static unsigned long fetch(void) { static unsigned long count; return ++count; }
struct pair pack_count(void) { unsigned long x=fetch(); struct pair value; __builtin_memcpy(&value,&x,sizeof value); return value; }
struct words mix_double(double x, int c) { struct words value; __builtin_memcpy(&value,&x,sizeof value); if (c) value.low ^= value.high; return value; }
struct pair swap_double(double x, int c) { struct pair value; __builtin_memcpy(&value,&x,sizeof value); if (c) value.real = value.imag; return value; }
"#,
    )
    .unwrap();
    let result = Command::new("gcc")
        .args([
            "-g",
            "-O2",
            "-fno-ipa-sra",
            "-fno-ipa-cp",
            "-shared",
            "-fPIC",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let mut bodies = String::new();
    for name in [
        "pack",
        "unpack",
        "pack_float",
        "unpack_float",
        "pack_count",
        "payload_bits",
        "mix_double",
        "swap_double",
    ] {
        let (body, err, status) = common::run_kuna(&["decompile", binary.to_str().unwrap(), name]);
        assert_eq!(status, 0, "{err}");
        bodies.push_str(&body.replace(&format!("{name}("), &format!("recovered_{name}(")));
    }
    let code = format!(
        r#"
#include <stdint.h>
#include <string.h>
typedef struct words {{ unsigned int low, high; }} words;
typedef struct pair {{ float real, imag; }} pair;
typedef struct box {{ unsigned long tag; words payload; }} box;
static unsigned long calls;
static unsigned long fetch(void) {{ return ++calls; }}
static words consume(words x) {{ x.low += 1; return x; }}
extern words pack(unsigned int);
extern unsigned long unpack(words);
extern unsigned long unpack_float(pair);
extern pair pack_float(unsigned long);
extern pair pack_count(void);
extern unsigned long payload_bits(box *);
extern words mix_double(double, int);
extern pair swap_double(double, int);
{}
int main(void) {{
    unsigned int values[] = {{0,1,0x80000000u,0xffffffffu,0x13579bdfu,0x2468ace0u}};
    for (unsigned i=0; i<sizeof values/sizeof *values; ++i) {{
        words actual=recovered_pack(values[i]), expected=pack(values[i]);
        if (memcmp(&actual,&expected,sizeof actual)) return 1;
        for (unsigned j=0; j<sizeof values/sizeof *values; ++j) {{
            words input={{values[i],values[j]}};
            if (recovered_unpack(input) != unpack(input)) return 2;
            box object={{0xa5a5a5a5a5a5a5a5UL,input}};
            if (recovered_payload_bits(&object) != payload_bits(&object)) return 6;
        }}
    }}
    unsigned int patterns[]={{0,0x80000000u,1,0x80000001u,0x3f800000u,0x7f800000u,0xff800000u,0x7fc12345u}};
    for (unsigned i=0;i<8;i++) for (unsigned j=0;j<8;j++) {{
        unsigned long bits=(unsigned long)patterns[i] | (unsigned long)patterns[j]<<32;
        pair input; memcpy(&input,&bits,8);
        if (recovered_unpack_float(input)!=unpack_float(input)) return 3;
        pair actual=recovered_pack_float(bits), expected=pack_float(bits);
        if (memcmp(&actual,&expected,8)) return 4;
        double wide; memcpy(&wide,&bits,8);
        for (int c=0; c<2; c++) {{
            words mixed=recovered_mix_double(wide,c), mixed_expected=mix_double(wide,c);
            if (memcmp(&mixed,&mixed_expected,8)) return 7;
            pair swapped=recovered_swap_double(wide,c), swapped_expected=swap_double(wide,c);
            if (memcmp(&swapped,&swapped_expected,8)) return 8;
        }}
    }}
    pair actual=recovered_pack_count(), expected=pack_count();
    if (calls!=1 || memcmp(&actual,&expected,8)) return 5;
    return 0;
}}
"#,
        bodies
    );
    let recovered = source.with_extension("roundtrip.c");
    let executable = source.with_extension("exe");
    std::fs::write(&recovered, code).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let result = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Wno-parentheses", "-Werror", level])
                .arg(&recovered)
                .arg(&binary)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{compiler} {level}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            let result = Command::new(&executable).output().unwrap();
            assert!(result.status.success(), "{compiler} {level}: {result:?}");
        }
    }
    for path in [source, binary, recovered, executable] {
        std::fs::remove_file(path).unwrap();
    }
}
