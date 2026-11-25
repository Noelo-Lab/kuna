//! A function whose callers read its result returns it in `decompile-all` when
//! the callers agree only on the low register: one reads the `rdx` it finds
//! beside `rax` and hands both on, as openssh's `auth_restrict_session` does with
//! `sshauthopt_new`. And an ARM function that forwards its own `r0` to a tail
//! call returns what the tail call returns once its callers read `r0`: the
//! tail callee, recovered `void`, is forced in turn, as u-boot's `bootm_run`
//! over `boot_run`. Before, the first stayed `void` and the second chain was
//! never forced, while each reader printed a use of the result.
use crate::common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

/// A direct call or branch to patch: the function it is in, the offset of the
/// instruction there, and the function it reaches.
type Patch = (&'static str, u64, &'static str);

/// A relocatable object of `funcs`, laid out in order in `.text`, with each
/// branch of `patches` pointed at its target by `encode(word_at, from, to)`.
fn object_file(
    arch: Architecture,
    funcs: &[(&str, Vec<u8>)],
    patches: &[Patch],
    encode: impl Fn(&mut [u8], u64, u64),
) -> Vec<u8> {
    let mut text = Vec::new();
    let mut at = Vec::new();
    for (_, code) in funcs {
        at.push(text.len() as u64);
        text.extend_from_slice(code);
    }
    let start = |name: &str| at[funcs.iter().position(|(n, _)| *n == name).unwrap()];
    for &(func, off, target) in patches {
        let from = start(func) + off;
        encode(&mut text[from as usize..], from, start(target));
    }
    let mut object = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    let section = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(section, &text, 16);
    for ((name, code), &value) in funcs.iter().zip(&at) {
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

fn words(code: &[u32]) -> Vec<u8> {
    code.iter().flat_map(|w| w.to_le_bytes()).collect()
}

fn decompile_all(path: &std::path::Path) -> Vec<(String, String)> {
    let binary = std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(Command::new(binary).args(["decompile-all", path.to_str().unwrap(), "--json"]));
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

/// `new_opt` calls `alloc_opt`, stores through the pointer when it is not null
/// and returns it. `restrict_session` passes the result and the `rdx` it finds
/// beside it to `fatal2`, so it reads `rdx:rax`; `first_field` loads through
/// the result, so it reads `rax`.
#[test]
fn callers_that_read_a_leftover_high_register_agree_on_the_low_one() {
    let funcs = [
        // lea 8(%rdi),%rax; ret
        ("alloc_opt", vec![0x48, 0x8d, 0x47, 0x08, 0xc3]),
        // sub $8,%rsp; call alloc_opt; test %rax,%rax; je +7; movl $-1,0x30(%rax); add $8,%rsp; ret
        (
            "new_opt",
            vec![
                0x48, 0x83, 0xec, 0x08, 0xe8, 0, 0, 0, 0, 0x48, 0x85, 0xc0, 0x74, 0x07, 0xc7, 0x40, 0x30, 0xff, 0xff,
                0xff, 0xff, 0x48, 0x83, 0xc4, 0x08, 0xc3,
            ],
        ),
        // lea (%rdi,%rsi),%rax; ret
        ("fatal2", vec![0x48, 0x8d, 0x04, 0x37, 0xc3]),
        // sub $8,%rsp; call new_opt; mov %rax,%rdi; mov %rdx,%rsi; call fatal2; add $8,%rsp; ret
        (
            "restrict_session",
            vec![
                0x48, 0x83, 0xec, 0x08, 0xe8, 0, 0, 0, 0, 0x48, 0x89, 0xc7, 0x48, 0x89, 0xd6, 0xe8, 0, 0, 0, 0, 0x48,
                0x83, 0xc4, 0x08, 0xc3,
            ],
        ),
        // sub $8,%rsp; call new_opt; mov 0xc(%rax),%eax; add $8,%rsp; ret
        ("first_field", vec![0x48, 0x83, 0xec, 0x08, 0xe8, 0, 0, 0, 0, 0x8b, 0x40, 0x0c, 0x48, 0x83, 0xc4, 0x08, 0xc3]),
    ];
    let patches: [Patch; 4] = [
        ("new_opt", 4, "alloc_opt"),
        ("restrict_session", 4, "new_opt"),
        ("restrict_session", 15, "fatal2"),
        ("first_field", 4, "new_opt"),
    ];
    let bytes = object_file(Architecture::X86_64, &funcs, &patches, |at, from, to| {
        let rel = (to as i64 - (from as i64 + 5)) as i32;
        at[1..5].copy_from_slice(&rel.to_le_bytes());
    });
    let path = common::scratch_file("forced-return-low-register", "o");
    std::fs::write(&path, bytes).unwrap();
    let functions = decompile_all(&path);
    let new_opt = code(&functions, "new_opt");
    assert!(!new_opt.contains("void new_opt("), "{new_opt}");
    assert!(new_opt.contains("return v1;"), "{new_opt}");
    let reader = code(&functions, "first_field");
    assert!(reader.contains("new_opt(a0)"), "{reader}");
}

/// `run_states` returns a value; `boot_run` stores through its `r0` and tail
/// calls it; `bootm_run` forwards its `r0` untouched in a tail call to
/// `boot_run`; `reader` adds one to `bootm_run`'s result.
#[test]
fn a_tail_call_that_forwards_the_return_register_forces_its_callee() {
    let funcs = [
        // str r1,[r0,#20]; and r0,r1,#3; bx lr
        ("run_states", words(&[0xe5801014, 0xe2010003, 0xe12fff1e])),
        // str r1,[r0,#20]; orr r1,r2,#0x1700; b run_states
        ("boot_run", words(&[0xe5801014, 0xe3821c17, 0xea000000])),
        // mov r1,#0; movw r2,#0x80f; b boot_run
        ("bootm_run", words(&[0xe3a01000, 0xe300280f, 0xea000000])),
        // push {r4,lr}; bl bootm_run; add r0,r0,#1; pop {r4,pc}
        ("reader", words(&[0xe92d4010, 0xeb000000, 0xe2800001, 0xe8bd8010])),
    ];
    let patches: [Patch; 3] = [("boot_run", 8, "run_states"), ("bootm_run", 8, "boot_run"), ("reader", 4, "bootm_run")];
    let bytes = object_file(Architecture::Arm, &funcs, &patches, |at, from, to| {
        let word = u32::from_le_bytes(at[..4].try_into().unwrap());
        let off = ((to as i64 - (from as i64 + 8)) >> 2) as u32 & 0x00ff_ffff;
        at[..4].copy_from_slice(&((word & 0xff00_0000) | off).to_le_bytes());
    });
    let path = common::scratch_file("forced-return-tail-callee", "o");
    std::fs::write(&path, bytes).unwrap();
    let functions = decompile_all(&path);
    let boot_run = code(&functions, "boot_run");
    assert!(boot_run.contains("return run_states("), "{boot_run}");
    let bootm_run = code(&functions, "bootm_run");
    assert!(bootm_run.contains("return boot_run(a0,0,0x80f);"), "{bootm_run}");
    assert!(code(&functions, "reader").contains("return bootm_run(a0) + 1;"), "{functions:?}");
}
