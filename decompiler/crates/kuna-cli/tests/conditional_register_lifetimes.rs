//! Compile and execute unchanged C from authored ARM conditional-register flows.
mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

fn fixture() -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    // Assembled from docs/features/conditional-register-lifetimes/fixture.S.
    let words: &[u32] = &[
        0xe92d4010, 0xe1a04000, 0xeb000008, 0xe3500000, 0x15941004, 0x15d10000, 0x13500000,
        0x0a000001, 0xe5910004, 0xe8bd8010, 0xe3a00000, 0xe8bd8010, 0xe3500000, 0x15900000,
        0xe12fff1e, 0xe92d41f0, 0xe1a04000, 0xe1a05001, 0xe3a06000, 0xe3550000, 0x0a00000b,
        0xe4947004, 0xe1a00007, 0xebfffff3, 0xe3500000, 0x15971004, 0x15d10000, 0x13500000,
        0x0a000001, 0xe5910004, 0xe0866000, 0xe2555001, 0x1afffff3, 0xe1a00006, 0xe8bd81f0,
        0xe5901000, 0xe5910004, 0xe12fff1e, 0xe3500000, 0x13a01013, 0xe1a00001, 0xe12fff1e,
        0xe3500000, 0x15913000, 0x05923000, 0xe5930004, 0xe12fff1e, 0xe3500000, 0x012fff1e,
        0xe5902000, 0xe3520000, 0x03a00000, 0x012fff1e, 0xe5901004, 0xe5d12000, 0xe3520000,
        0x03a00000, 0x012fff1e, 0xe5912004, 0xe352000a, 0x85910008, 0x93a00000, 0xe12fff1e,
        0xe92d4010, 0xe1a04000, 0xebffffc9, 0xe3500000, 0x13a01013, 0xe1a00001, 0xe8bd8010,
        0xe92d41f0, 0xe1a04000, 0xe1a05001, 0xe1a08002, 0xe3550000, 0x0a000009, 0xe4947004,
        0xe1a00007, 0xebffffbc, 0xe3500000, 0x15971004, 0x15d10000, 0x13500000, 0x11a08001,
        0xe2555001, 0x1afffff5, 0xe5980004, 0xe8bd81f0, 0xe92d41f0, 0xe1a04000, 0xe1a05001,
        0xe3a08000, 0xe3550000, 0x0a000005, 0xe4940004, 0xebffffab, 0xe3500000, 0x11a08001,
        0xe2555001, 0x1afffff9, 0xe1a00008, 0xe8bd81f0,
    ];
    let bytes: Vec<_> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    obj.append_section_data(text, &bytes, 4);
    for (name, value, size) in [
        ("candidate", 0, 48),
        ("valid", 48, 12),
        ("scan", 60, 80),
        ("unconditional", 140, 12),
        ("retained", 152, 16),
        ("distinct", 168, 20),
        ("nested", 188, 64),
        ("unknown_used", 252, 28),
        ("remember", 280, 72),
        ("unknown_loop", 352, 56),
    ] {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    obj.write().unwrap()
}

fn generated() -> (String, Vec<String>) {
    let image = common::scratch_file("conditional-registers", "o");
    std::fs::write(&image, fixture()).unwrap();
    let wrapper = std::env::var_os("KUNA_CLI_WRAPPER");
    let mut command = Command::new(
        wrapper
            .as_deref()
            .unwrap_or(std::ffi::OsStr::new(env!("CARGO_BIN_EXE_kuna"))),
    );
    command
        .env("KUNA_BINARY", env!("CARGO_BIN_EXE_kuna"))
        .env("KUNA_SPECS", common::repo_root().join("specs"))
        .env("KUNA_DECODE_JOBS", "1")
        .arg("decompile-all")
        .arg(&image)
        .args([
            "--mode",
            "reliable",
            "--language",
            "c",
            "--jobs",
            "1",
            "--json",
            "--target",
            "ARM:LE:32:v5t:default",
            "--isa",
            "arm",
            "--assert-strict",
            "--no-vars",
        ]);
    for prototype in [
        "unsigned int candidate(unsigned int *entity)",
        "unsigned int valid(unsigned int *entity)",
        "unsigned int scan(unsigned int **entities,unsigned int count)",
        "unsigned int unconditional(unsigned int **payload)",
        "unsigned int retained(unsigned int flag,unsigned int previous)",
        "unsigned int distinct(unsigned int flag,unsigned int **left,unsigned int **right)",
        "unsigned int nested(unsigned int *entity)",
        "unsigned int unknown_used(unsigned int *entity)",
        "unsigned int unknown_loop(unsigned int **entities,unsigned int count)",
        "unsigned int remember(unsigned int **entities,unsigned int count,unsigned int *previous)",
    ] {
        let name = prototype
            .split('(')
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();
        command.args(["--assert", &format!("prototype {name} {prototype}")]);
    }
    let output = common::process::required_output(&mut command);
    let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(doc["functions"].as_array().unwrap().len(), 10);
    let functions = doc["functions"].as_array().unwrap();
    let defined = functions
        .iter()
        .filter(|f| !f["name"].as_str().unwrap().starts_with("unknown_"))
        .map(|f| f["code"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let unknown: Vec<_> = functions
        .iter()
        .filter(|f| f["name"].as_str().unwrap().starts_with("unknown_"))
        .map(|f| f["code"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(unknown.len(), 2);
    (defined, unknown)
}

const MODEL: &str = r#"
typedef unsigned char uint1; typedef unsigned short uint2; typedef unsigned int uint4;
typedef signed char int1; typedef short int2; typedef int int4;
typedef unsigned char xunknown1; typedef unsigned int xunknown4;
typedef _Bool bool;
_Static_assert(sizeof(void *) == 4, "ARM pointer width");
unsigned int valid(unsigned int *);
unsigned int candidate(unsigned int *);
unsigned int scan(unsigned int **,unsigned int);
unsigned int remember(unsigned int **,unsigned int,unsigned int *);
unsigned int unconditional(unsigned int **);
unsigned int retained(unsigned int,unsigned int);
unsigned int distinct(unsigned int,unsigned int **,unsigned int **);
unsigned int nested(unsigned int *);
static unsigned int model(unsigned int *p) {
    if (!p || !p[0]) return 0;
    unsigned int *payload = (unsigned int *)p[1];
    return *(unsigned char *)payload ? payload[1] : 0;
}
int main(void) {
    unsigned int off[3] = {0,13,29}, on[3] = {1,17,31}, small[3] = {1,7,37};
    unsigned int dead[2] = {0,0xdead0000}, inactive[2] = {1,(unsigned int)off};
    unsigned int active[2] = {3,(unsigned int)on}, low[2] = {1,(unsigned int)small};
    unsigned int *choices[5] = {0,dead,inactive,active,low};
    for (unsigned int i=0;i<5;i++) {
        if (candidate(choices[i]) != model(choices[i])) return 1;
        unsigned int expected = i==3 ? 31 : 0;
        if (nested(choices[i]) != expected) return 2;
    }
    for (unsigned int byte=0;byte<256;byte++) {
        on[0]=byte;
        if (candidate(active)!=(byte ? 17 : 0)) return 7;
        if (nested(active)!=(byte ? 31 : 0)) return 8;
    }
    on[0]=1;
    for (unsigned int count=0;count<=4;count++) {
        unsigned int limit=1;
        for (unsigned int k=0;k<count;k++) limit*=5;
        for (unsigned int sequence=0;sequence<limit;sequence++) {
            unsigned int *entities[4], number=sequence, expected=0, remembered=13;
            for (unsigned int k=0;k<count;k++) {
                entities[k]=choices[number%5]; number/=5;
                unsigned int value=model(entities[k]);
                expected+=value;
                if (value) remembered=value;
            }
            if (scan(entities,count)!=expected) return 3;
            if (remember(entities,count,off)!=remembered) return 9;
        }
    }
    unsigned int *left=on,*right=small;
    if (unconditional(&left)!=17) return 4;
    for (unsigned int flag=0;flag<3;flag++) {
        if (distinct(flag,&left,&right)!=(flag ? 17 : 7)) return 5;
        for (unsigned int previous=0;previous<32;previous++)
            if (retained(flag,previous)!=(flag ? 19 : previous)) return 6;
    }
    return 0;
}
#if defined(__i386__)
void _start(void) {
    int result = main();
    __asm__ volatile("int $0x80" : : "a"(1), "b"(result) : "memory");
    __builtin_unreachable();
}
#endif
"#;

#[test]
fn guarded_registers_compile_and_match_the_model_at_o0_and_o2() {
    let source = common::scratch_file("conditional-registers", "c");
    let (defined, unknown) = generated();
    std::fs::write(&source, format!("{MODEL}\n{defined}")).unwrap();
    let gcc = if cfg!(all(
        target_os = "linux",
        any(target_arch = "x86", target_arch = "x86_64")
    )) {
        common::process::optional_output(Command::new("gcc").arg("--version"))
    } else {
        None
    };
    let arm = common::process::optional_output(Command::new("arm-none-eabi-gcc").arg("--version"));
    for (index, unknown) in unknown.iter().enumerate().filter(|_| gcc.is_some()) {
        let uncertain = source.with_extension(format!("unknown{index}.c"));
        std::fs::write(&uncertain, format!("{MODEL}\n{unknown}")).unwrap();
        let output = Command::new("gcc")
            .args([
                "-std=c11",
                "-m32",
                "-O0",
                "-Werror=uninitialized",
                "-Werror=maybe-uninitialized",
                "-c",
            ])
            .arg(&uncertain)
            .arg("-o")
            .arg(uncertain.with_extension("o"))
            .output()
            .unwrap();
        assert!(
            !output.status.success(),
            "observable undefined path was initialized: {unknown}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("uninitialized"),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    for level in ["-O0", "-O2"] {
        if gcc.is_some() {
            let binary = source.with_extension(&level[1..]);
            common::process::required_output(
                Command::new("gcc")
                    .args([
                        "-std=c11",
                        "-m32",
                        "-nostdlib",
                        "-fno-pie",
                        "-no-pie",
                        "-fno-builtin",
                        "-fno-stack-protector",
                        "-Werror=uninitialized",
                        "-Werror=maybe-uninitialized",
                        level,
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
        if arm.is_some() {
            common::process::required_output(
                Command::new("arm-none-eabi-gcc")
                    .args([
                        "-std=c11",
                        "-marm",
                        "-march=armv5te",
                        "-Werror=uninitialized",
                        "-Werror=maybe-uninitialized",
                        level,
                        "-c",
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(source.with_extension(format!("arm{}o", &level[1..]))),
            );
        }
    }
}
