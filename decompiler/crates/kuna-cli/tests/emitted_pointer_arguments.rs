//! A scalar address matches its character-pointer callee in a buffered batch,
//! independently of inference, address order, or worker assignment.
use crate::common;
use common::process;

use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::path::PathBuf;
use std::process::Command;

/// One function: its name, its bytes, and the offset of the `call rel32`
/// opcode it makes to another function, if any.
type Func<'a> = (&'a str, Vec<u8>, Option<(usize, &'a str)>);

/// An x86-64 relocatable object with `funcs` laid out in order and every call
/// resolved to its callee.
fn object(funcs: &[Func]) -> PathBuf {
    let mut at = Vec::new();
    let mut code = Vec::new();
    for (_, bytes, _) in funcs {
        at.push(code.len());
        code.extend_from_slice(bytes);
    }
    for (i, (_, _, call)) in funcs.iter().enumerate() {
        if let Some((off, callee)) = call {
            let site = at[i] + off;
            let target = at[funcs.iter().position(|f| f.0 == *callee).unwrap()];
            let rel = target as i64 - (site + 5) as i64;
            code[site + 1..site + 5].copy_from_slice(&(rel as i32).to_le_bytes());
        }
    }
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, &code, 1);
    for (i, (name, bytes, _)) in funcs.iter().enumerate() {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: at[i] as u64,
            size: bytes.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let path = common::scratch_file("pointer-call", "o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    path
}

fn decompile(input: &PathBuf, extra: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", input.to_str().unwrap()])
        .args(extra)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn compilers() -> Vec<&'static str> {
    let found: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!found.is_empty(), "the round trip requires a C compiler");
    found
}

/// Compile the complete emitted bodies and support preambles against their
/// own accepted declarations, not fixture-imposed prototypes. These scalar
/// fixtures have single-line signatures immediately before an opening brace.
fn round_trip(printed: &str, main: &str, include: Option<&std::path::Path>) {
    let declarations = printed.lines().collect::<Vec<_>>().windows(2)
        .filter_map(|lines| {
            let signature = lines[0].split("//").next().unwrap().trim_end();
            (lines[1] == "{" && signature.ends_with(')'))
                .then(|| format!("{signature};\n"))
        })
        .collect::<String>();
    let instrumentation = r#"
static void *observed_callee;
static unsigned callee_visits;
__attribute__((no_instrument_function))
void __cyg_profile_func_enter(void *callee, void *caller) {
    (void)caller;
    if (callee == observed_callee) ++callee_visits;
}
__attribute__((no_instrument_function))
void __cyg_profile_func_exit(void *callee, void *caller) {
    (void)callee; (void)caller;
}
"#;
    let src = common::scratch_file("pointer-call", "c");
    let exe = common::scratch_file("pointer-call", "exe");
    std::fs::write(&src, format!("{declarations}\n{printed}\n{instrumentation}\n{main}")).unwrap();
    for cc in compilers() {
        for level in ["-O0", "-O2"] {
            let mut command = Command::new(cc);
            command.args(["-std=c11", "-Werror=incompatible-pointer-types",
                "-Werror=implicit-function-declaration", "-finstrument-functions", level]);
            if let Some(dir) = include {
                command.arg("-I").arg(dir);
            }
            let compile = command.arg(&src).arg("-o").arg(&exe).output().unwrap();
            assert!(
                compile.status.success(),
                "{cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            assert!(
                Command::new(&exe).status().unwrap().success(),
                "{cc} {level}: {printed}"
            );
        }
    }
    for file in [src, exe] {
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn a_byte_writing_callee_casts_its_callers_full_width_local_in_either_address_order() {
    // sink: xor byte [rdi],1; ret.  change: push rdi; mov rdi,rsp; call sink;
    // pop rax; ret.  Only the low byte of the pushed word changes.
    let sink: Func = ("sink", vec![0x80, 0x37, 1, 0xc3], None);
    let change: Func = (
        "change",
        vec![0x57, 0x48, 0x89, 0xe7, 0xe8, 0, 0, 0, 0, 0x58, 0xc3],
        Some((4, "sink")),
    );
    let main = r#"
int main(void) {
    unsigned long values[] = {0, 1, 0x123456789abcdef0UL, ~0UL, 0x8000000000000000UL};
    observed_callee = (void *)sink;
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i) {
        callee_visits = 0;
        if (change(values[i]) != (values[i] ^ 1)) return 1;
        if (callee_visits != 1) return 2;
    }
    return 0;
}
"#;
    for layout in [
        [sink.clone(), change.clone()],
        [change.clone(), sink.clone()],
    ] {
        let input = object(&layout);
        let printed = decompile(&input, &[]);
        round_trip(&printed, main, None);
        let serial = decompile(&input, &["--option", "protoorder", "off"]);
        round_trip(&serial, main, None);
        for extra in [
            vec!["--functions", "change,sink"],
            vec!["--functions", "change,sink", "--option", "protoorder", "types"],
            vec!["--option", "protoorder", "off", "--jobs", "2", "--jobs-chunk", "1"],
        ] {
            let args = extra;
            let printed = decompile(&input, &args);
            round_trip(&printed, main, None);
        }
        for jobs in ["1", "2"] {
            let dir = common::scratch_file("pointer-project", "dir");
            let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args(["decompile-project", input.to_str().unwrap(), "-o", dir.to_str().unwrap(),
                    "--functions", "change,sink", "--option", "protoorder", "off", "--jobs", jobs,
                    "--jobs-chunk", "1"])
                .output().unwrap();
            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
            let files: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
            let c = files.iter().find(|p| p.extension().is_some_and(|e| e == "c")).unwrap();
            let printed = std::fs::read_to_string(c).unwrap();
            round_trip(&printed, main, Some(&dir));
            std::fs::remove_dir_all(dir).unwrap();
        }
        std::fs::remove_file(input).unwrap();
    }
}

/// Buffered text, graph JSON and project exports all compile against the
/// accepted callee interface and preserve the byte write exactly once.
#[test]
fn every_buffered_document_prints_the_same_call() {
    let sink: Func = ("sink", vec![0x80, 0x37, 1, 0xc3], None);
    let change: Func = (
        "change",
        vec![0x57, 0x48, 0x89, 0xe7, 0xe8, 0, 0, 0, 0, 0x58, 0xc3],
        Some((4, "sink")),
    );
    let input = object(&[change, sink]);
    let all = decompile(&input, &["--option", "protoorder", "off"]);
    let main = r#"
int main(void) {
    unsigned long values[] = {0, 1, 0x123456789abcdef0UL, ~0UL, 0x8000000000000000UL};
    observed_callee = (void *)sink;
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i) {
        callee_visits = 0;
        if (change(values[i]) != (values[i] ^ 1)) return 1;
        if (callee_visits != 1) return 2;
    }
    return 0;
}
"#;
    round_trip(&all, main, None);
    for jobs in ["1", "2"] {
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile-graph", input.to_str().unwrap(), "--jobs", jobs, "--jobs-chunk", "1"])
            .output()
            .unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let graph: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        let code = graph["functions"].as_array().unwrap().iter()
            .map(|function| function["codeC"].as_str().expect("fixture function decompiled"))
            .collect::<Vec<_>>().join("\n");
        round_trip(&code, main, None);
    }
    std::fs::remove_file(input).unwrap();
}

#[test]
fn only_a_byte_of_another_signedness_or_width_decides_the_cast() {
    // The same byte may need a signedness conversion at the call boundary;
    // the word needs a character pointer without changing the local's width.
    let byte_sink: Func = (
        "byte_sink",
        vec![
            0x55, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xec, 0x10, 0x40, 0x88, 0x7d, 0xff, 0x48, 0x8d,
            0x7d, 0xff, 0xe8, 0, 0, 0, 0, 0x0f, 0xbe, 0x45, 0xff, 0xc9, 0xc3,
        ],
        Some((16, "sink")),
    );
    let word_wr: Func = (
        "word_wr",
        vec![
            0x55, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xec, 0x10, 0x89, 0x7d, 0xfc, 0x48, 0x8d, 0x7d,
            0xfc, 0xe8, 0, 0, 0, 0, 0x8b, 0x45, 0xfc, 0xc9, 0xc3,
        ],
        Some((15, "wr")),
    );
    let sink: Func = ("sink", vec![0x80, 0x37, 1, 0xc3], None);
    let wr: Func = ("wr", vec![0xc6, 0x07, 7, 0xc3], None);
    let input = object(&[byte_sink, word_wr, sink, wr]);
    let printed = decompile(&input, &[]);
    round_trip(
        &printed,
        r#"
int main(void) {
    unsigned values[] = {0, 1, 0x7f, 0x80, 0xfe, 0xff, 0x12345678, ~0u};
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i) {
        observed_callee = (void *)sink;
        callee_visits = 0;
        if (byte_sink((char)values[i]) != (char)((unsigned char)values[i] ^ 1)) return 1;
        if (callee_visits != 1) return 3;
        observed_callee = (void *)wr;
        callee_visits = 0;
        if (word_wr(values[i]) != ((values[i] & ~0xffu) | 7)) return 2;
        if (callee_visits != 1) return 4;
    }
    return 0;
}
"#,
        None,
    );
    std::fs::remove_file(input).unwrap();
}
