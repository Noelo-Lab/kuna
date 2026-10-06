//! `mixedtailret`: a function that returns a value it also compares on one
//! path and tail-calls a value-returning function on the other returns that
//! value on both paths in `decompile-all`, with no caller reading it. The x86-64
//! fixture holds gcc and clang -O2 builds; the printed C is compiled against it
//! and compared with it. A function whose tail callee returns nothing, and one
//! that leaves the return register unwritten on its other path, stay `void`;
//! so does an AArch64 function whose tail callee takes its value in `w0`.
mod common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

const ASM: &str = include_str!("fixtures/mixed_tail_returns.S");

const RETURNING: &[(&str, &str)] =
    &[("keep_gcc", "int"), ("keep_clang", "int"), ("fwd_clang", "int"), ("keepl_gcc", "long")];

const DRIVER: &str = r#"
extern int g;
extern long gl;
int keep_gcc(int *);
int keep_clang(int *);
int fwd_clang(int *);
long keepl_gcc(long *);
int emitted_keep_gcc(int *);
int emitted_keep_clang(int *);
int emitted_fwd_clang(int *);
long emitted_keepl_gcc(long *);
int main(void) {
    for (int a = -20; a < 20; a++) {
        int x = a;
        long y = a * 1000000007L;
        g = a + 3;
        gl = a - 7;
        if (keep_gcc(&x) != emitted_keep_gcc(&x)) return 1;
        if (keep_clang(&x) != emitted_keep_clang(&x)) return 2;
        if (fwd_clang(&x) != emitted_fwd_clang(&x)) return 3;
        if (keepl_gcc(&y) != emitted_keepl_gcc(&y)) return 4;
    }
    return 0;
}
"#;

fn decompile_all(path: &std::path::Path, mixed: bool) -> Vec<(String, String)> {
    let binary = std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut command = Command::new(binary);
    command.args(["decompile-all", path.to_str().unwrap(), "--json"]);
    if !mixed {
        command.args(["--option", "mixedtailret", "off"]);
    }
    let output = process::required_output(&mut command);
    let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            assert!(f["error"].is_null(), "{f}");
            (f["name"].as_str().unwrap().to_owned(), f["code"].as_str().unwrap().to_owned())
        })
        .collect()
}

fn code(functions: &[(String, String)], name: &str) -> String {
    functions
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, c)| c.clone())
        .unwrap_or_else(|| panic!("no {name} in {functions:?}"))
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn a_value_returned_beside_a_tail_call_is_returned() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let asm = common::scratch_file("mixed-tail-returns", "S");
    let elf = asm.with_extension("elf");
    std::fs::write(&asm, ASM).unwrap();
    process::required_output(
        Command::new("clang")
            .args(["-nostdlib", "-no-pie", "-Wl,--build-id=none", "-Wl,-e,getv"])
            .arg(&asm)
            .arg("-o")
            .arg(&elf),
    );
    let functions = decompile_all(&elf, true);
    let mut emitted = String::from("extern int g;\nextern long gl;\nint getv(void);\nint getk(int);\nlong getl(void);\n");
    for (name, ret) in RETURNING {
        let text = code(&functions, name);
        assert!(text.contains(&format!("{ret} {name}(")), "{text}");
        assert!(!text.contains("void") && text.matches("return ").count() == 2, "{text}");
        emitted.push_str(&text.replace(&format!(" {name}("), &format!(" emitted_{name}(")));
        emitted.push('\n');
    }
    for name in ["vkeep_gcc", "vin"] {
        let text = code(&functions, name);
        assert!(text.contains(&format!("void {name}(")), "{text}");
    }
    let off = decompile_all(&elf, false);
    for (name, _) in RETURNING {
        let text = code(&off, name);
        assert!(text.contains(&format!("void {name}(")), "{text}");
    }

    let source = asm.with_extension("emitted.c");
    std::fs::write(&source, format!("{emitted}\n{DRIVER}")).unwrap();
    for level in ["-O0", "-O2"] {
        let exe = asm.with_extension("emitted");
        process::required_output(
            Command::new("clang").arg(level).arg("-no-pie").arg(&source).arg(&asm).arg("-o").arg(&exe),
        );
        process::required_output(&mut Command::new(&exe));
    }
}

fn words(code: &[u32]) -> Vec<u8> {
    code.iter().flat_map(|w| w.to_le_bytes()).collect()
}

/// `getv`, `getk`, and `keep`/`fwd` as clang -O2 builds them for AArch64, each
/// ending `b <callee>` at offset 16.
fn aarch64_object() -> Vec<u8> {
    let funcs: [(&str, Vec<u8>, Option<&str>); 4] = [
        // adrp x8,0; ldr w8,[x8]; add w0,w8,w8,lsl #1; ret
        ("getv", words(&[0x90000008, 0xb9400108, 0x0b080500, 0xd65f03c0]), None),
        // add w0,w0,w0,lsl #1; ret
        ("getk", words(&[0x0b000400, 0xd65f03c0]), None),
        // ldr w0,[x0]; cmp w0,#5; b.le +8; ret; b getv
        ("keep", words(&[0xb9400000, 0x7100141f, 0x5400004d, 0xd65f03c0, 0x14000000]), Some("getv")),
        // ldr w0,[x0]; cmp w0,#5; b.le +8; ret; b getk
        ("fwd", words(&[0xb9400000, 0x7100141f, 0x5400004d, 0xd65f03c0, 0x14000000]), Some("getk")),
    ];
    let mut text = Vec::new();
    let mut at = Vec::new();
    for (_, code, _) in &funcs {
        at.push(text.len() as u64);
        text.extend_from_slice(code);
    }
    for (k, (_, _, target)) in funcs.iter().enumerate() {
        let Some(target) = target else { continue };
        let to = at[funcs.iter().position(|(n, _, _)| n == target).unwrap()];
        let from = at[k] + 16;
        let word = 0x14000000u32 | (((to as i64 - from as i64) >> 2) as u32 & 0x3ffffff);
        text[from as usize..from as usize + 4].copy_from_slice(&word.to_le_bytes());
    }
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Aarch64, Endianness::Little);
    let section = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(section, &text, 16);
    for ((name, code, _), &value) in funcs.iter().zip(&at) {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size: code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}

#[test]
fn an_aarch64_value_returned_beside_a_tail_call_is_returned_unless_the_callee_reads_it() {
    let path = common::scratch_file("mixed-tail-returns-a64", "o");
    std::fs::write(&path, aarch64_object()).unwrap();
    let functions = decompile_all(&path, true);
    let keep = code(&functions, "keep");
    assert!(keep.contains("int keep(") && keep.contains("return getv();"), "{keep}");
    assert!(keep.matches("return ").count() == 2, "{keep}");
    let fwd = code(&functions, "fwd");
    assert!(fwd.contains("void fwd("), "{fwd}");
    let off = decompile_all(&path, false);
    assert!(code(&off, "keep").contains("void keep("), "{off:?}");
}
