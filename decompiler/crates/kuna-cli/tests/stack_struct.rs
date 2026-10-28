//! An address-taken stack record prints as a declared structure, and no partial
//! access prints Ghidra's `name._<off>_<size>_`, under the aggressive preset.
#![cfg(all(target_os = "linux", target_endian = "little"))]

use crate::common;
use common::process;
use std::process::Command;

/// `(base, memset)` of the four builds of `fixtures/stack_struct.c` in
/// `tests/stages/kuna-stackstruct.xml`: gcc -O0, gcc -O2, clang -O0, clang -O2.
const BUILDS: [(u64, Option<u64>); 4] =
    [(0x401000, None), (0x501000, None), (0x601000, Some(0x60117f)), (0x701000, None)];

fn image() -> std::path::PathBuf {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-stackstruct.xml"
    ));
    let chunk =
        regex::Regex::new(r#"<bytechunk[^>]*offset="(0x[0-9a-f]+)"[^>]*>([0-9a-f]+)</bytechunk>"#)
            .unwrap();
    let mut bytes = Vec::new();
    for captures in chunk.captures_iter(xml) {
        let offset = usize::from_str_radix(&captures[1][2..], 16).unwrap() - 0x401000;
        let hex = &captures[2];
        bytes.resize(offset, 0);
        bytes.extend(
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()),
        );
    }
    let path = common::scratch_file("stack-struct", "bin");
    std::fs::write(&path, bytes).unwrap();
    path
}

/// The helpers' entries in one build, read off the stage file's symbols.
fn helpers(base: u64) -> Vec<(String, u64)> {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-stackstruct.xml"
    ));
    let symbol = regex::Regex::new(r#"offset="0x([0-9a-f]+)" name="(fetch|setup|report)_"#).unwrap();
    symbol
        .captures_iter(xml)
        .map(|c| (c[2].to_owned(), u64::from_str_radix(&c[1], 16).unwrap()))
        .filter(|(_, a)| (a & !0xfffff) == (base & !0xfffff))
        .collect()
}

fn decompile(image: &std::path::Path, base: u64, memset: Option<u64>, mode: &str) -> String {
    decompile_function(image, base, memset, mode, "parse_header", &[])
}

fn decompile_function(
    image: &std::path::Path, base: u64, memset: Option<u64>, mode: &str,
    name: &str, options: &[(&str, &str)],
) -> String {
    let mut asserts = vec![format!("function 0x{base:x}={name}")];
    asserts.extend(helpers(base).into_iter().map(|(n, a)| format!("function 0x{a:x}={n}")));
    asserts.extend(memset.map(|a| format!("function 0x{a:x}=memset")));
    let entry = format!("0x{base:x}");
    let mut args = vec![
        "decompile",
        image.to_str().unwrap(),
        &entry,
        "--raw-image",
        "--base",
        "0x401000",
        "--target",
        "x86:LE:64:default:gcc",
        "--mode",
        mode,
        "--option",
        "structdefs",
        "on",
        "--assert-strict",
    ];
    for a in &asserts {
        args.extend(["--assert", a.as_str()]);
    }
    for &(option, value) in options {
        args.extend(["--option", option, value]);
    }
    let (stdout, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{stderr}");
    stdout
}

fn compilers() -> Vec<&'static str> {
    let found: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!found.is_empty(), "a C compiler is required");
    found
}

const DECLARATIONS: &str = "#include <stddef.h>\n\
void *memset(void *p, int c, unsigned long n);\n\
void fetch(void *dst, unsigned long n);\n\
void setup(void *h);\n\
int report(unsigned int w, unsigned int h, unsigned long sum);\n";

#[test]
fn every_build_declares_the_record_and_prints_c() {
    let image = image();
    let partial = regex::Regex::new(r"\._[0-9]+_[0-9]+_").unwrap();
    let record = regex::Regex::new(r"(?m)^  struct_[0-9]+ v[0-9]+; //").unwrap();
    for (base, memset) in BUILDS {
        let code = decompile(&image, base, memset, "aggressive");
        assert!(!partial.is_match(&code), "0x{base:x}: {code}");
        assert!(record.is_match(&code), "0x{base:x}: {code}");
        assert!(code.contains("typedef struct struct_"), "0x{base:x}: {code}");
        assert!(code.contains(".field_0x4"), "0x{base:x}: {code}");
        let source = common::scratch_file("stack-struct", "c");
        std::fs::write(&source, format!("{DECLARATIONS}{code}")).unwrap();
        for cc in compilers() {
            let compiled = Command::new(cc)
                .args(["-std=gnu11", "-fsyntax-only", "-fno-strict-aliasing"])
                .args(common::CC_GCC15_DEMOTE)
                .arg(&source)
                .output()
                .unwrap();
            assert!(
                compiled.status.success(),
                "{cc} 0x{base:x}: {}\n{code}",
                String::from_utf8_lossy(&compiled.stderr)
            );
        }
    }
}

#[test]
fn reliable_keeps_ghidra_notation() {
    let code = decompile(&image(), 0x401000, None, "reliable");
    assert!(code.contains("._4_2_ != 3"), "{code}");
    assert!(!code.contains("struct_"), "{code}");
}

/// Keep both the full-record and scalar-piece consumers executable. The latter
/// uses a no-op setup callback, so it never depends on unrelated stack locals
/// being contiguous in the generated C.
fn check_header_reads(code: &str, whole_header: bool, tag: &str) {
    let full = u8::from(whole_header);
    let program = format!(
        r#"
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#define FULL_HEADER {full}
struct header {{ unsigned int magic; unsigned short version; unsigned char w, h; unsigned int count; unsigned char *pixels; }};
static unsigned char bytes[8],pixels[65536];
static unsigned fetches,setups,reports,bad;
static void *address;
void fetch(void *dst,unsigned long n) {{
    ++fetches; address=dst;
    if(n!=sizeof(bytes)){{bad=1;return;}}
    memcpy(dst,bytes,sizeof(bytes));
}}
void setup(void *p) {{
    ++setups;
    if(p!=address)bad=1;
    if(FULL_HEADER) {{
        struct header *h=p;
        if(h->count!=0 || h->pixels!=NULL)bad=1;
        h->count=h->w*h->h;
        for(unsigned i=0;i<h->count;++i)pixels[i]=(unsigned char)(i*37);
        h->pixels=pixels;
    }}
}}
int report(unsigned int w,unsigned int h,unsigned long sum) {{
    ++reports;
    if(w!=bytes[6] || h!=bytes[7])bad=1;
    return (int)(w*1000+h*100+sum);
}}
{code}
int main(void) {{
    const uint32_t magics[]={{UINT32_C(0x52444845),0,UINT32_C(0xffffffff)}};
    const uint16_t versions[]={{0,3,0x103,0x8003,0xffff}};
    const unsigned char widths[]={{0,1,127,128,200,255}},heights[]={{0,1,2,255}};
    for(unsigned m=0;m<sizeof(magics)/sizeof(*magics);++m)
    for(unsigned v=0;v<sizeof(versions)/sizeof(*versions);++v)
    for(unsigned w=0;w<sizeof(widths);++w)
    for(unsigned h=0;h<sizeof(heights);++h) {{
        memcpy(bytes,&magics[m],4);
        bytes[4]=(unsigned char)versions[v]; bytes[5]=(unsigned char)(versions[v]>>8);
        bytes[6]=widths[w]; bytes[7]=heights[h];
        fetches=setups=reports=bad=0; address=NULL;
        int got=(int)parse_header(),expected;
        unsigned valid=magics[m]==UINT32_C(0x52444845) && versions[v]==3;
        if(magics[m]!=UINT32_C(0x52444845))expected=-1;
        else if(versions[v]!=3)expected=-2;
        else {{
            unsigned long sum=0;
            unsigned count=FULL_HEADER ? (unsigned)widths[w]*heights[h] : 0;
            for(unsigned i=0;i<count;++i)sum+=(unsigned char)(i*37);
            expected=(int)((unsigned)widths[w]*1000+(unsigned)heights[h]*100+sum);
        }}
        if(got!=expected || bad || fetches!=1 || setups!=valid || reports!=valid)return 1;
    }}
    return 0;
}}
"#
    );
    run_native_program(tag, code, &program);
}

fn run_native_program(tag: &str, code: &str, program: &str) {
    let source = common::scratch_file(tag, "c");
    std::fs::write(&source, program).unwrap();
    for cc in compilers() {
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file(tag, "exe");
            let compiled = Command::new(cc)
                .args(["-std=gnu11", optimization, "-fno-strict-aliasing"])
                .args(common::CC_GCC15_DEMOTE)
                .arg(&source).arg("-o").arg(&executable).output().unwrap();
            assert!(compiled.status.success(), "{cc} {optimization}: {}\n{code}",
                String::from_utf8_lossy(&compiled.stderr));
            let run = Command::new(&executable).output().unwrap();
            assert!(run.status.success(), "{cc} {optimization}: {:?}\n{code}", run.status);
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(source).unwrap();
}

#[test]
fn emitted_record_reads_the_fields_the_source_wrote() {
    let code = decompile(&image(), 0x601000, Some(0x60117f), "aggressive");
    check_header_reads(&code, true, "stack-record-run");
}

#[test]
fn partial_scalar_reads_keep_the_short_and_unsigned_byte_values() {
    let code = decompile_function(&image(), 0x401000, None, "reliable", "parse_header",
        &[("ctypes", "on"), ("partialaccess", "on")]);
    check_header_reads(&code, false, "stack-partial-run");
}

#[test]
fn in_place_text_reads_both_bytes_after_the_callback() {
    let code = decompile_function(&image(), 0x5010f0, None, "aggressive", "text_len", &[]);
    let program = format!(r#"
#include <stdint.h>
#include <string.h>
static unsigned char replacement[2];
static unsigned fetches,reports,bad;
void fetch(void *dst,unsigned long n) {{
    ++fetches;
    if(n!=0 || memcmp(dst,"hello wo",8)!=0)bad=1;
    memcpy(dst,replacement,sizeof(replacement));
}}
int report(unsigned int w,unsigned int h,unsigned long sum) {{
    ++reports;
    if(w!=(unsigned int)(int)(int8_t)replacement[0] ||
       h!=(unsigned int)(int)(int8_t)replacement[1] || sum!=0)bad=1;
    return 17;
}}
{code}
int main(void) {{
    const unsigned char values[]={{0,1,127,128,255}};
    for(unsigned i=0;i<sizeof(values);++i)for(unsigned j=0;j<sizeof(values);++j) {{
        replacement[0]=values[i]; replacement[1]=values[j];
        fetches=reports=bad=0;
        if((int)text_len()!=17 || bad || fetches!=1 || reports!=1)return 1;
    }}
    return 0;
}}
"#);
    run_native_program("stack-text-run", &code, &program);
}

/// One 8-byte store zeroes two adjacent locals and one 8-byte load reloads
/// them, but each address reaches a call at its own start: two objects.
#[test]
fn two_escaped_addresses_stay_two_objects() {
    let fixture = common::fixture("stack_struct_adjacent_a64.o");
    let (code, stderr, status) = common::run_kuna(&[
        "decompile",
        &fixture,
        "adjacent",
        "--mode",
        "aggressive",
        "--option",
        "structdefs",
        "on",
    ]);
    assert_eq!(status, 0, "{stderr}");
    assert!(!code.contains("struct_"), "{code}");
    assert!(!regex::Regex::new(r"\._[0-9]+_[0-9]+_").unwrap().is_match(&code), "{code}");
}

/// A 16-byte piece reads through `unsigned __int128`, a plain 16-byte store or
/// copy is `memset`/`memcpy`, and nothing prints an array-typed value.
#[test]
fn wide_pieces_print_as_c() {
    let vm = common::fixture("arraycoverwidth_x86_64");
    let (code, stderr, status) = common::run_kuna(&["decompile", &vm, "vm"]);
    assert_eq!(status, 0, "{stderr}");
    for want in ["memset(&v2,0,0x10);", "memcpy(&v2,&v3[0],0x10);", "memcpy(&v1,&v2,0x10);"] {
        assert!(code.contains(want), "missing `{want}`:\n{code}");
    }
    let pair = common::fixture("zerofillfwd_a64.o");
    let (pair_code, stderr, status) = common::run_kuna(&["decompile-all", &pair]);
    assert_eq!(status, 0, "{stderr}");
    assert!(pair_code.contains("return *(unsigned __int128 *)v1;"), "{pair_code}");
    for text in [&code, &pair_code] {
        assert!(!text.contains("(*)["), "{text}");
        assert!(!regex::Regex::new(r"\._[0-9]+_[0-9]+_").unwrap().is_match(text), "{text}");
    }
}
