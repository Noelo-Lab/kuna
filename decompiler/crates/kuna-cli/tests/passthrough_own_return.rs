//! `passthrough` hands a call-then-return function its callee's result only
//! when the function returns nothing of its own: a float it computes after the
//! call (ARM `s0`, x86-64 `xmm0`, AArch64 `s0`, MIPS `$f0`) stays its return
//! value, and the callee's integer result left in `r0`, `rax`, `x0` or `$v0` is
//! not returned. A zero is not such a value: `-fzero-call-used-regs` writes one
//! into every call-used register a forwarder does not return in.
mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

/// One function: its bytes, and where in them the call to `provider` goes.
struct Func {
    name: &'static str,
    code: Vec<u8>,
    call_at: Option<usize>,
}

fn words(code: &[u32], big: bool) -> Vec<u8> {
    code.iter()
        .flat_map(|w| {
            if big {
                w.to_be_bytes()
            } else {
                w.to_le_bytes()
            }
        })
        .collect()
}

/// A relocatable object whose `.text` is `funcs` back to back from offset 0,
/// each call encoded by `call(from, to)` against those offsets.
fn object(
    arch: Architecture,
    endian: Endianness,
    e_flags: u32,
    funcs: &[Func],
    call: impl Fn(u64, u64) -> Vec<u8>,
) -> Vec<u8> {
    let mut text = Vec::new();
    let mut at = Vec::new();
    for f in funcs {
        at.push(text.len() as u64);
        text.extend_from_slice(&f.code);
    }
    let provider = at[funcs.iter().position(|f| f.name == "provider").unwrap()];
    for (f, &start) in funcs.iter().zip(&at) {
        if let Some(off) = f.call_at {
            let from = start + off as u64;
            let bytes = call(from, provider);
            text[from as usize..from as usize + bytes.len()].copy_from_slice(&bytes);
        }
    }
    let mut object = Object::new(BinaryFormat::Elf, arch, endian);
    object.flags = FileFlags::Elf {
        os_abi: 0,
        abi_version: 0,
        e_flags,
    };
    let section = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(section, &text, 16);
    for (f, &value) in funcs.iter().zip(&at) {
        object.add_symbol(Symbol {
            name: f.name.as_bytes().to_vec(),
            value,
            size: f.code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}

fn arm_image() -> Vec<u8> {
    let f = |name, code: &[u32], call: usize| Func {
        name,
        code: words(code, false),
        call_at: Some(call * 4),
    };
    let funcs = [
        Func {
            name: "provider",
            code: words(&[0xe5900000, 0xe12fff1e], false),
            call_at: None,
        },
        // push {r11,lr}; vpush {d8}; vmov.f32 s16,s0; bl provider;
        // vadd.f32 s0,s16,s16; vpop {d8}; pop {r11,pc}
        f(
            "f_after",
            &[
                0xe92d4800, 0xed2d8b02, 0xeeb08a40, 0, 0xee380a08, 0xecbd8b02, 0xe8bd8800,
            ],
            3,
        ),
        // push {r11,lr}; bl provider; vmov.f32 s0,#1.5; pop {r11,pc}
        f("f_const", &[0xe92d4800, 0, 0xeeb70a08, 0xe8bd8800], 1),
        // push {r4,lr}; bl provider; pop {r4,pc}
        f("wrapper", &[0xe92d4010, 0, 0xe8bd8010], 1),
        // push {r11,lr}; bl provider; mov r1,#0; pop {r11,pc}
        f("w_high", &[0xe92d4800, 0, 0xe3a01000, 0xe8bd8800], 1),
    ];
    object(
        Architecture::Arm,
        Endianness::Little,
        0x0500_0400,
        &funcs,
        |from, to| {
            (0xeb000000u32 | ((to as i64 - from as i64 - 8) >> 2) as u32 & 0xffffff)
                .to_le_bytes()
                .to_vec()
        },
    )
}

fn x86_image() -> Vec<u8> {
    let call = [0xe8, 0, 0, 0, 0];
    let funcs = [
        Func {
            name: "provider",
            code: vec![0x8b, 0x07, 0xc3],
            call_at: None,
        },
        // sub $8,%rsp; movss %xmm0,4(%rsp); call provider; movss 4(%rsp),%xmm0;
        // add $8,%rsp; addss %xmm0,%xmm0; ret
        Func {
            name: "f_after",
            code: [
                &[0x48, 0x83, 0xec, 0x08, 0xf3, 0x0f, 0x11, 0x44, 0x24, 0x04][..],
                &call,
                &[0xf3, 0x0f, 0x10, 0x44, 0x24, 0x04, 0x48, 0x83, 0xc4, 0x08],
                &[0xf3, 0x0f, 0x58, 0xc0, 0xc3],
            ]
            .concat(),
            call_at: Some(10),
        },
        // sub $8,%rsp; movsd %xmm0,(%rsp); call provider; movsd (%rsp),%xmm0;
        // add $8,%rsp; addsd %xmm0,%xmm0; ret
        Func {
            name: "d_after",
            code: [
                &[0x48, 0x83, 0xec, 0x08, 0xf2, 0x0f, 0x11, 0x04, 0x24][..],
                &call,
                &[0xf2, 0x0f, 0x10, 0x04, 0x24, 0x48, 0x83, 0xc4, 0x08],
                &[0xf2, 0x0f, 0x58, 0xc0, 0xc3],
            ]
            .concat(),
            call_at: Some(9),
        },
        // sub $8,%rsp; call provider; add $8,%rsp; ret
        Func {
            name: "wrapper",
            code: [
                &[0x48, 0x83, 0xec, 0x08][..],
                &call,
                &[0x48, 0x83, 0xc4, 0x08, 0xc3],
            ]
            .concat(),
            call_at: Some(4),
        },
        // the same, ending in the -fzero-call-used-regs scrub: xor %edx,%edx;
        // xor %ecx,%ecx; xor %esi,%esi; xor %edi,%edi; pxor %xmm0,%xmm0;
        // pxor %xmm1,%xmm1; ret
        Func {
            name: "scrubbed",
            code: [
                &[0x48, 0x83, 0xec, 0x08][..],
                &call,
                &[
                    0x48, 0x83, 0xc4, 0x08, 0x31, 0xd2, 0x31, 0xc9, 0x31, 0xf6, 0x31, 0xff,
                ],
                &[0x66, 0x0f, 0xef, 0xc0, 0x66, 0x0f, 0xef, 0xc9, 0xc3],
            ]
            .concat(),
            call_at: Some(4),
        },
    ];
    object(
        Architecture::X86_64,
        Endianness::Little,
        0,
        &funcs,
        |from, to| {
            let mut bytes = vec![0xe8];
            bytes.extend_from_slice(&((to as i64 - from as i64 - 5) as i32).to_le_bytes());
            bytes
        },
    )
}

fn a64_image() -> Vec<u8> {
    let f = |name, code: &[u32], call: Option<usize>| Func {
        name,
        code: words(code, false),
        call_at: call.map(|c| c * 4),
    };
    let funcs = [
        // add w0,w0,w0,lsl #1; ret
        f("provider", &[0x0b000400, 0xd65f03c0], None),
        // str d8,[sp,#-32]!; stp x29,x30,[sp,#16]; add x29,sp,#16; fmov s8,s0;
        // bl provider; ldp x29,x30,[sp,#16]; fadd s0,s8,s8; ldr d8,[sp],#32; ret
        f(
            "f_after",
            &[
                0xfc1e0fe8, 0xa9017bfd, 0x910043fd, 0x1e204008, 0, 0xa9417bfd, 0x1e282900,
                0xfc4207e8, 0xd65f03c0,
            ],
            Some(4),
        ),
        // stp x29,x30,[sp,#-16]!; mov x29,sp; bl provider; ldp x29,x30,[sp],#16; ret
        f(
            "wrapper",
            &[0xa9bf7bfd, 0x910003fd, 0, 0xa8c17bfd, 0xd65f03c0],
            Some(2),
        ),
    ];
    object(
        Architecture::Aarch64,
        Endianness::Little,
        0,
        &funcs,
        |from, to| {
            (0x94000000u32 | ((to as i64 - from as i64) >> 2) as u32 & 0x3ffffff)
                .to_le_bytes()
                .to_vec()
        },
    )
}

/// `jal` is absolute: the loader lays a relocatable object's `.text` out at
/// 0x400000.
fn mips_image(big: bool) -> Vec<u8> {
    let f = |name, code: &[u32], call: Option<usize>| Func {
        name,
        code: words(code, big),
        call_at: call.map(|c| c * 4),
    };
    let funcs = [
        f("provider", &[0x8c820000, 0x03e00008, 0], None),
        // addiu sp,-24; sw ra,20(sp); sw s0,16(sp); move s0,a1; jal provider; nop;
        // mtc1 s0,$f0; add.s $f0,$f0,$f0; lw s0,16(sp); lw ra,20(sp); jr ra; addiu sp,24
        f(
            "f_after",
            &[
                0x27bdffe8, 0xafbf0014, 0xafb00010, 0x00a08025, 0, 0, 0x44900000, 0x46000000,
                0x8fb00010, 0x8fbf0014, 0x03e00008, 0x27bd0018,
            ],
            Some(4),
        ),
        f(
            "wrapper",
            &[
                0x27bdffe8, 0xafbf0014, 0, 0, 0x8fbf0014, 0x03e00008, 0x27bd0018,
            ],
            Some(2),
        ),
    ];
    let endian = if big {
        Endianness::Big
    } else {
        Endianness::Little
    };
    object(Architecture::Mips, endian, 0x5000_1000, &funcs, |_, to| {
        words(
            &[0x0c000000 | ((0x400000 + to) >> 2) as u32 & 0x3ffffff],
            big,
        )
    })
}

fn decompile(bytes: &[u8]) -> String {
    decompile_with(bytes, &[])
}

fn decompile_with(bytes: &[u8], options: &[&str]) -> String {
    let path = common::scratch_file("passthrough-own-return", "o");
    std::fs::write(&path, bytes).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-all",
            path.to_str().unwrap(),
            "--mode",
            "aggressive",
        ])
        .args(options)
        .output()
        .unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    text
}

fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let marker = format!("// Function: {name} @");
    text.split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{text}"))
        .split("// Function:")
        .next()
        .unwrap()
}

fn assert_own_return(text: &str, name: &str, ty: &str) {
    let body = function(text, name);
    assert!(body.contains(&format!("{ty} {name}(")), "{text}");
    assert!(body.contains("provider(a"), "{text}");
    assert!(!body.contains("return provider"), "{text}");
}

#[test]
fn a_float_computed_after_the_call_is_the_return_value() {
    for armfloatreturn in ["off", "on"] {
        let text = decompile_with(
            &arm_image(),
            &["--option", "armfloatreturn", armfloatreturn],
        );
        assert_own_return(&text, "f_after", "float");
        let body = function(&text, "f_const");
        assert!(body.contains("provider(a0);"), "{text}");
        assert!(!body.contains("return provider"), "{text}");
    }
    let text = decompile(&x86_image());
    assert_own_return(&text, "f_after", "float");
    assert_own_return(&text, "d_after", "double");
    let text = decompile(&a64_image());
    assert_own_return(&text, "f_after", "float");
    for big in [false, true] {
        let text = decompile(&mips_image(big));
        assert_own_return(&text, "f_after", "float");
    }
}

#[test]
fn a_forwarder_still_hands_back_its_callee_result() {
    for text in [
        decompile(&arm_image()),
        decompile(&x86_image()),
        decompile(&a64_image()),
        decompile(&mips_image(false)),
        decompile(&mips_image(true)),
    ] {
        assert!(
            function(&text, "wrapper").contains("return provider(a0);"),
            "{text}"
        );
    }
    let text = decompile(&arm_image());
    assert!(
        function(&text, "w_high").contains("return provider(a0);"),
        "{text}"
    );
    let text = decompile(&x86_image());
    assert!(
        function(&text, "scrubbed").contains("return provider("),
        "{text}"
    );
}
