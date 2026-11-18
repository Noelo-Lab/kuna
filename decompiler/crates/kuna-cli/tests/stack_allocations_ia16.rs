//! Real-mode (ia16) runtime stack reservations under `stackalloc on`.
//!
//! The load modules are gcc-ia16 6.3.0 output (`-ffreestanding -fno-builtin`,
//! small model, linked flat at offset 0) for
//!
//! ```c
//! __attribute__((noinline)) unsigned observe(unsigned char *p, unsigned n) { return p[0] + n; }
//! __attribute__((noinline)) unsigned sample_one(unsigned n) {
//!     unsigned char *p = __builtin_alloca(n); p[0] = 7; return observe(p, n);
//! }
//! __attribute__((noinline)) unsigned sample_vla(unsigned n) {
//!     unsigned r; { unsigned char d[n + 1]; d[0] = 7; r = observe(d, n + 1); } return r + 1;
//! }
//! ```
//!
//! Embedded so the test needs no 16-bit toolchain. The recovered C of all four
//! functions, recompiled with gcc-ia16 and run beside the native code in a
//! 16-bit emulator, returned the same values for n in 0..=1000 (368 cases).
//!
//! The prototypes are asserted as `__cdecl16near`. The x86-16 default model
//! has unknown extrapop: without the assertion even main drops `observe`'s
//! arguments, and `stackalloc` declines, since it cannot bound the callee's
//! reach into the frame.

use std::process::Command;

const O0: &str = "5589e58b5e048a0788c030e489c28b460601d089c089c05d161fc35589e583ec028b46044040d1e8d1e029c489e040d1e8d1e08946fe8b5efe36c60707ff7604ff76fe161fe8b8ff83c40489c089c089c089ec5d161fc3575589e583ec2289e089c78b460689c04089c189c8488946fe89cabb000089d0d1e08946f289d8d1d08946f48b46f2d1e08946ee8b46f4d1d08946f08b46eed1e08946ea8b46f0d1d08946ec8b46ea8b56ec894ef6c746f800008b46f6d1e08946e68b46f8d1d08946e88b46e6d1e08946e28b46e8d1d08946e48b46e2d1e08946de8b46e4d1d08946e08b46de8b56e089c84089c0d1e8d1e029c489e080c4008946fc8b5efc36c607078b460689c04089c28b46fc5250161fe8edfe83c40489c08946fa89fc8b46fa4089c089c089ec5d5f161fc35589e5b8030050161fe8e3fe83c402b8040050161fe813ff83c402ebfe";
const O2: &str = "5589e58b5e048a0730e40346065dc35589e58b560489d083c00283e0fe29c489e3c607075253e8d7ff89ec5dc35589e58b46044089c24283e2fe29d489e289e3c607075052e8b8ff4089ec5dc3b8030050e8bbffb8040050e8d2ff83c404ebfe";

fn hex(text: &str) -> Vec<u8> {
    (0..text.len()).step_by(2).map(|at| u8::from_str_radix(&text[at..at + 2], 16).unwrap()).collect()
}

/// An MZ image whose load module is `module`, entry `0000:0000`, no relocations.
fn dos_image(module: &[u8]) -> Vec<u8> {
    let mut image = vec![0u8; 0x20];
    image[..2].copy_from_slice(b"MZ");
    let total = image.len() + module.len();
    for (at, value) in [(0x02, (total % 512) as u16), (0x04, total.div_ceil(512) as u16), (0x08, 2), (0x10, 0x100), (0x18, 0x1c)] {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    image.extend_from_slice(module);
    image
}

/// (`-O` level, module, sample_one, sample_vla) load-module offsets.
const CASES: [(&str, &str, u32, u32); 2] = [("O0", O0, 0x1b, 0x57), ("O2", O2, 0x0f, 0x2d)];

#[test]
fn real_mode_reservations_lower_to_c_allocations_with_their_arguments() {
    for (level, module, one, vla) in CASES {
        let path = crate::common::scratch_file(&format!("ia16-stackalloc-{level}"), "exe");
        std::fs::write(&path, dos_image(&hex(module))).unwrap();
        for (name, offset, argument) in [("sample_one", one, "n"), ("sample_vla", vla, "n + 1")] {
            let address = format!("{:#x}", 0x10000 + offset);
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .env("KUNA_SPECS", crate::common::repo_root().join("specs"))
                .arg("decompile").arg(&path).arg(&address)
                .args(["--option", "stackalloc", "on",
                    "--define-function", "0x10000=observe",
                    "--define-function", &format!("{address}={name}"),
                    "--assert", "prototype observe unsigned short __cdecl16near observe(unsigned short p,unsigned short n)",
                    "--assert", &format!("prototype {name} unsigned short __cdecl16near {name}(unsigned short n)")])
                .output().unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            let code = String::from_utf8(output.stdout).unwrap();
            assert!(code.contains("__builtin_alloca(") && !code.contains("(&Stack"),
                "{level} {name}: the reservation was not recovered\n{code}");
            // The call reads its stack arguments out of the lowered backing.
            assert!(code.contains("observe(p,n_00)") && code.contains(&format!("= {argument};")),
                "{level} {name}: observe lost its arguments\n{code}");
        }
    }
}

/// Turbo C 2.01 (1988) has neither `alloca` nor VLAs; period code reserved
/// stack through the `_SP` pseudo-register. `ia16_turboc_sp.c` built with
/// `tcc -ms` (`ia16_turboc_sp.exe`) and `tcc -ms -O -G -Z`
/// (`ia16_turboc_sp_opt.exe`); `sample_sp` saves SP in a frame slot, lowers it
/// by `(n + 1) & ~1`, calls `observe(p, n)` and restores SP from the slot.
/// The recovered C, recompiled and run beside the native code in a 16-bit
/// emulator, returned the same values for n in 0..=1000 (296 cases).
#[test]
fn turbo_c_sp_pseudo_register_reservations_lower_to_c_allocations() {
    for (fixture, sample) in [("ia16_turboc_sp.exe", "0x1020b"), ("ia16_turboc_sp_opt.exe", "0x10209")] {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", crate::common::repo_root().join("specs"))
            .arg("decompile").arg(crate::common::fixture(fixture)).arg(sample)
            .args(["--option", "stackalloc", "on",
                "--define-function", "0x101fa=observe",
                "--define-function", &format!("{sample}=sample_sp"),
                "--assert", "prototype observe unsigned short __cdecl16near observe(unsigned short p,unsigned short n)",
                "--assert", "prototype sample_sp unsigned short __cdecl16near sample_sp(unsigned short n)"])
            .output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        assert!(code.contains("__builtin_alloca(") && !code.contains("(&Stack"),
            "{fixture}: the reservation was not recovered\n{code}");
        assert!(code.contains("return observe(p,n_00) + 1;") && code.contains("= n;"),
            "{fixture}: observe lost its arguments\n{code}");
    }
}

#[test]
fn near_call_pop_before_reservation_preserves_the_saved_word() {
    // Read n from the near-call frame, save a known DX across a balanced
    // near CALL/RET, then reserve n bytes and write that saved word at +4.
    // The return IP is 0x000d, not the saved value 0x1234.
    let module = hex("5589e58b4e04ba341252e80c005a29cc89e389570489ec5dc3c3");
    let path = crate::common::scratch_file("ia16-stackalloc-call-pop", "exe");
    std::fs::write(&path, dos_image(&module)).unwrap();
    for declared_callee in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.env("KUNA_SPECS", crate::common::repo_root().join("specs"))
            .arg("decompile").arg(&path).arg("0x10000")
            .args(["--option", "stackalloc", "on",
                "--define-function", "0x10000=sample_call_pop",
                "--define-function", "0x10019=balanced",
                "--assert", "prototype sample_call_pop void __cdecl16near sample_call_pop(unsigned short n)"]);
        if declared_callee {
            command.args(["--assert", "prototype balanced void __cdecl16near balanced(void)"]);
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        let Some(allocation) = code.find("__builtin_alloca(") else {
            panic!("declared_callee={declared_callee}: the runtime reservation was not recovered\n{code}");
        };
        assert!(code[..allocation].contains("balanced("),
            "the saved-word control must call before reserving storage\n{code}");
        // Check the one native word write, not the earlier PUSH. A byte-typed
        // backing spells the same little-endian 0x1234 as two byte writes.
        let word_write = code[allocation..].contains("= 0x1234;");
        let byte_write = code.lines().filter_map(|line| line.trim().split_once(" = "))
            .find_map(|(lhs, rhs)| {
                (rhs.contains("__builtin_alloca(") && rhs.ends_with(")[4];")).then_some(lhs)
            }).is_some_and(|target| {
                [("[0]", "'4';"), ("[1]", "'\\x12';")].into_iter().all(|(index, value)| {
                    code[allocation..].lines().filter_map(|line| line.trim().split_once(" = "))
                        .any(|(lhs, rhs)| lhs.strip_suffix(index) == Some(target) && rhs == value)
                })
            });
        assert!(word_write || byte_write,
            "declared_callee={declared_callee}: the allocation-backed write lost DX across CALL/RET\n{code}");
    }
}

#[test]
fn scalar_sp_consumers_keep_the_runtime_amount_when_recovery_declines() {
    // Reserve a runtime prefix, then twice subtract [BX+2d], POP ES, and
    // XOR the resulting SP word into memory. The loop has no object lifetime
    // to recover, but its scalar observation still depends on each amount.
    let module = hex("5589e5a1020329c4be02002b672d07312600034e75f589ec5dc3");
    let path = crate::common::scratch_file("ia16-stackalloc-scalar-sp", "exe");
    std::fs::write(&path, dos_image(&module)).unwrap();
    let mut outputs = [String::new(), String::new()];
    for (index, mode) in ["off", "on"].into_iter().enumerate() {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", crate::common::repo_root().join("specs"))
            .arg("decompile").arg(&path).arg("0x10000")
            .args(["--option", "stackalloc", mode,
                "--define-function", "0x10000=scalar_sp",
                "--assert", "prototype scalar_sp void __cdecl16near scalar_sp(void)"])
            .output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        outputs[index] = String::from_utf8(output.stdout).unwrap();
    }
    let [off, code] = outputs;
    // A refusal may add a header diagnostic, never change the native consumer.
    assert_eq!(off.split_once('{').unwrap().1, code.split_once('{').unwrap().1,
        "allocation refusal changed the native scalar-SP consumer");
    assert!(!code.contains("__builtin_alloca("),
        "the scalar-only loop acquired an allocation\n{code}");
    let Some(consumer) = code.lines().find_map(|line| {
        line.contains("0x300 =").then(|| line.split_once(" ^ ")).flatten()
    }) else {
        panic!("the native scalar-SP observation disappeared\n{code}");
    };
    let Some(value) = consumer.1.trim_end_matches(';').strip_prefix("(unsigned short)") else {
        panic!("the native SP word was not observed\n{code}");
    };
    let Some(expression) = code.lines().filter_map(|line| line.trim().split_once(" = "))
        .find_map(|(lhs, rhs)| (lhs == value).then_some(rhs)) else {
        panic!("the observed SP word has no producer\n{code}");
    };
    assert!(expression.contains("0x2d") && expression.contains(" + 2"),
        "the SP observation lost the runtime subtraction or POP increment\n{code}");
}
