//! ARM/Thumb interworking: a `blx` selects Thumb for its own target only, a `bl`
//! keeps its caller's mode, and neither leaks into the other helper (GH-780).

#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;
mod common;

use std::path::PathBuf;
use std::process::Command;

const THUMB_HELPER: u64 = 0x10040;
const ARM_HELPER: u64 = 0x10080;

fn bl(src: u64, dst: u64) -> u32 {
    0xEB00_0000 | ((dst.wrapping_sub(src + 8) >> 2) as u32 & 0x00FF_FFFF)
}

fn blx(src: u64, dst: u64) -> u32 {
    let offset = dst.wrapping_sub(src + 8);
    0xFA00_0000 | (((offset >> 1) & 1) as u32) << 24 | ((offset >> 2) as u32 & 0x00FF_FFFF)
}

const PUSH_LR: u32 = 0xE92D_4000;
const POP_PC: u32 = 0xE8BD_8000;

/// `.text` at 0x10000: the A32 entry `words`, a Thumb `movs r0,#7; bx lr` at
/// 0x10040 and an A32 `add r0,r0,#1; bx lr` at 0x10080.
fn text(words: &[u32]) -> Vec<u8> {
    let mut code = vec![0u8; 0x88];
    for (i, word) in words.iter().enumerate() {
        code[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    code[0x40..0x44].copy_from_slice(&[0x07, 0x20, 0x70, 0x47]);
    code[0x80..0x88].copy_from_slice(&[0x01, 0x00, 0x80, 0xE2, 0x1E, 0xFF, 0x2F, 0xE1]);
    code
}

fn blx_then_bl() -> Vec<u8> {
    text(&[
        PUSH_LR,
        blx(0x10004, THUMB_HELPER),
        bl(0x10008, ARM_HELPER),
        POP_PC,
    ])
}

fn bl_then_blx() -> Vec<u8> {
    text(&[
        PUSH_LR,
        bl(0x10004, ARM_HELPER),
        blx(0x10008, THUMB_HELPER),
        POP_PC,
    ])
}

/// `ldr r3,=thumb_helper|1; blx r3; bl arm_helper; pop {pc}`.
fn register_blx_then_bl() -> Vec<u8> {
    let ldr_r3 = 0xE59F_3008; // the pool word at 0x10014
    let blx_r3 = 0xE12F_FF33;
    text(&[
        PUSH_LR,
        ldr_r3,
        blx_r3,
        bl(0x1000c, ARM_HELPER),
        POP_PC,
        0x10041,
    ])
}

/// A Thumb entry: `push {lr}; blx arm_helper; bl thumb_helper; pop {pc}`.
fn thumb_entry() -> Vec<u8> {
    let mut code = text(&[]);
    code[..12].copy_from_slice(&[
        0x00, 0xB5, 0x00, 0xF0, 0x3E, 0xE8, 0x00, 0xF0, 0x1B, 0xF8, 0x00, 0xBD,
    ]);
    code
}

fn stripped(stem: &str, code: &[u8]) -> PathBuf {
    let path = common::scratch_file(stem, "elf");
    std::fs::write(&path, arm_images::elf(code, &[], &[])).unwrap();
    path
}

const FUNCTIONS: [(u64, &str, u64); 3] = [
    (0, "entry", 0x18),
    (0x41, "thumb_helper", 4),
    (0x80, "arm_helper", 8),
];

fn marked(stem: &str, code: &[u8]) -> PathBuf {
    let bytes = arm_images::elf(
        code,
        &[(0, "$a"), (0x14, "$d"), (0x40, "$t"), (0x80, "$a")],
        &FUNCTIONS,
    );
    let path = common::scratch_file(stem, "elf");
    std::fs::write(&path, bytes).unwrap();
    path
}

fn kuna(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .args(["--sleighpath"])
        .arg(common::repo_root().join("specs"))
        .output()
        .expect("run kuna");
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "kuna {args:?}: {text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    text
}

fn decompile(path: &PathBuf, addr: u64, mode: &str) -> String {
    let addr = format!("{addr:#x}");
    kuna(&[
        "decompile",
        path.to_str().unwrap(),
        &addr,
        "--mode",
        mode,
        "--json",
    ])
}

fn assert_helpers(path: &PathBuf, mode: &str) {
    let arm = decompile(path, ARM_HELPER, mode);
    assert!(
        arm.contains("return a0 + 1;"),
        "{mode}: the A32 helper lost its mode: {arm}"
    );
    assert!(!arm.contains("halt_"), "{mode}: {arm}");
    let thumb = decompile(path, THUMB_HELPER, mode);
    assert!(
        thumb.contains("return 7;"),
        "{mode}: the Thumb helper lost its mode: {thumb}"
    );
}

#[test]
fn a_blx_target_mode_does_not_reach_the_arm_callee_after_it() {
    for (stem, code) in [("blx-bl", blx_then_bl()), ("bl-blx", bl_then_blx())] {
        let path = stripped(stem, &code);
        for mode in ["reliable", "auto"] {
            assert_helpers(&path, mode);
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn whole_binary_decompile_keeps_each_callee_in_its_own_mode() {
    let path = stripped("blx-bl-all", &blx_then_bl());
    let all = kuna(&["decompile-all", path.to_str().unwrap()]);
    let body = |name: &str| {
        let start = all
            .find(&format!("// Function: {name}"))
            .unwrap_or_else(|| panic!("{all}"));
        let rest = &all[start + 1..];
        rest[..rest.find("// Function:").unwrap_or(rest.len())].to_string()
    };
    assert!(body("sub_10040").contains("return 7;"), "{all}");
    assert!(body("sub_10080").contains("return a0 + 1;"), "{all}");
    let entry = body("sub_10000");
    assert!(
        entry.contains("sub_10040()") && entry.contains("sub_10080("),
        "{all}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mapping_symbols_still_select_each_mode() {
    let path = marked("blx-bl-marked", &blx_then_bl());
    assert_helpers(&path, "reliable");
    let entry = decompile(&path, 0x10000, "reliable");
    assert!(entry.contains("arm_helper(thumb_helper())"), "{entry}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn function_symbols_alone_keep_each_helper_in_its_mode() {
    for (stem, code) in [
        ("blx-bl-funcs", blx_then_bl()),
        ("bl-blx-funcs", bl_then_blx()),
    ] {
        let path = common::scratch_file(stem, "elf");
        std::fs::write(&path, arm_images::elf(&code, &[], &FUNCTIONS)).unwrap();
        assert_helpers(&path, "reliable");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn a_register_interworking_call_leaves_the_arm_callee_alone() {
    let path = stripped("blx-r3", &register_blx_then_bl());
    let arm = decompile(&path, ARM_HELPER, "reliable");
    assert!(arm.contains("return a0 + 1;"), "{arm}");
    std::fs::remove_file(path).unwrap();

    let path = marked("blx-r3-marked", &register_blx_then_bl());
    assert_helpers(&path, "reliable");
    std::fs::remove_file(path).unwrap();
}

/// An ELF of `code` whose `e_entry` is the Thumb address 0x10001, with the
/// given function symbols.
fn thumb_entry_elf(stem: &str, code: &[u8], functions: &[(u64, &str, u64)]) -> PathBuf {
    let mut bytes = arm_images::elf(code, &[], functions);
    bytes[24..28].copy_from_slice(&0x10001u32.to_le_bytes());
    let path = common::scratch_file(stem, "elf");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn a_named_thumb_entry_keeps_its_blx_target_arm() {
    let path = thumb_entry_elf("thumb-entry", &thumb_entry(), &[(1, "entry", 12)]);
    assert_helpers(&path, "reliable");
    let entry = decompile(&path, 0x10000, "reliable");
    assert!(entry.contains("sub_10080("), "{entry}");
    assert!(entry.contains("sub_10040()"), "{entry}");
    std::fs::remove_file(path).unwrap();
}

/// A stripped image whose odd `e_entry` is its only Thumb evidence: the entry
/// `push {lr}; ldr r0,=0x10080; blx r0; pop {pc}` reaches the A32 helper above
/// it only through the pointer it loads, so nothing says where the Thumb code
/// ends and the helper keeps its A32 decode.
#[test]
fn a_stripped_thumb_entry_leaves_an_arm_function_it_points_at_alone() {
    let mut code = text(&[]);
    code[..16].copy_from_slice(&[
        0x00, 0xB5, 0x02, 0x48, 0x80, 0x47, 0x00, 0xBD, 0x00, 0xBF, 0x00, 0xBF, 0x80, 0x00, 0x01,
        0x00,
    ]);
    let path = thumb_entry_elf("thumb-entry-pointer", &code, &[]);
    let arm = decompile(&path, ARM_HELPER, "reliable");
    assert!(arm.contains("return a0 + 1;"), "{arm}");
    assert!(!arm.contains("halt_"), "{arm}");
    std::fs::remove_file(path).unwrap();
}

/// `caller` (the entry, A32) tail-branches to `arm_head`, an A32 function that
/// ends in `bl arm_helper` with the Thumb function symbol `thumb_tail` right
/// after it. The walk cannot tell whether that call returns, so it falls
/// through into `thumb_tail` in A32. The odd symbol keeps `thumb_tail` Thumb,
/// so its own `bl` still finds the unnamed Thumb leaf at 0x10020 (`movs r0,#9;
/// bx lr`).
#[test]
fn a_fall_through_does_not_redecode_the_next_function_symbol() {
    let mut code = text(&[PUSH_LR, bl(0x10004, ARM_HELPER)]);
    code[8..18].copy_from_slice(&[0x00, 0xB5, 0x01, 0x20, 0x00, 0xF0, 0x08, 0xF8, 0x00, 0xBD]);
    code[0x20..0x24].copy_from_slice(&[0x09, 0x20, 0x70, 0x47]);
    let b_arm_head = bl(0x10040, 0x10000) & !0x0100_0000;
    code[0x40..0x44].copy_from_slice(&b_arm_head.to_le_bytes());
    let functions = [
        (0, "arm_head", 8),
        (9, "thumb_tail", 10),
        (0x40, "caller", 4),
        (0x80, "arm_helper", 8),
    ];
    let mut bytes = arm_images::elf(&code, &[], &functions);
    bytes[24..28].copy_from_slice(&0x10040u32.to_le_bytes());
    let path = common::scratch_file("fall-through", "elf");
    std::fs::write(&path, bytes).unwrap();
    let all = kuna(&["decompile-all", path.to_str().unwrap()]);
    let body = |name: &str| {
        let start = all
            .find(&format!("// Function: {name}"))
            .unwrap_or_else(|| panic!("{all}"));
        let rest = &all[start + 1..];
        rest[..rest.find("// Function:").unwrap_or(rest.len())].to_string()
    };
    assert!(body("thumb_tail").contains("sub_10020("), "{all}");
    assert!(body("sub_10020").contains("return 9;"), "{all}");
    std::fs::remove_file(path).unwrap();
}

fn fixture(name: &str) -> PathBuf {
    common::repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
}

/// Stripped armel shared objects (`arm_interwork_stubs_le32.c`) whose exported
/// Thumb functions are reached only through the linker's A32 stubs, and whose
/// `t_ptr_only` only through a pointer table: no symbol and no `blx` says
/// they are Thumb.
#[test]
fn thumb_functions_behind_stubs_and_pointers_keep_their_mode() {
    for (name, called, pointer_only, ternary, indirect) in [
        (
            "arm_interwork_stubs_o0_le32",
            "return v1 - sub_360(a0 + 1);",
            "0x398",
            "(0xb <= a0) ? a0 + -10 : a0 << 1",
            "return (*v1)(a1);",
        ),
        (
            "arm_interwork_stubs_o2_le32",
            "return v1 - sub_2e8(a0 + 1);",
            "0x304",
            "v1 = a0 + -10;",
            "(**(void **)((a0 & 3) * 4 + dat_2010))(a1);",
        ),
    ] {
        let path = fixture(name);
        let lib = path.to_str().unwrap();
        let thumb_export = kuna(&["decompile", lib, "thumb_export"]);
        assert!(thumb_export.contains(called), "{name}: {thumb_export}");
        let dispatch = kuna(&["decompile", lib, "dispatch"]);
        assert!(dispatch.contains(indirect), "{name}: {dispatch}");
        let pointer = kuna(&["decompile", lib, pointer_only, "--addr"]);
        assert!(pointer.contains(ternary), "{name}: {pointer}");
    }
}

/// Stripped armel shared objects (`arm_static_thumb_so_le32.c`): a Thumb
/// export, an A32 export, then static Thumb functions reached only through a
/// pointer table. The A32 export's own symbol says nothing about the static
/// code after it, which keeps the mode the Thumb export's paint gives it.
#[test]
fn static_thumb_functions_after_an_arm_export_keep_their_mode() {
    for (name, ternary, popcount, arm_body) in [
        (
            "arm_static_thumb_so_o0_le32",
            "0x280",
            ("0x2a2", "v2 = (v1 & 1) + v2;"),
            "v2 += v1 ^ 0x33;",
        ),
        (
            "arm_static_thumb_so_o2_le32",
            "0x234",
            ("0x240", "v1 += v2;"),
            "v2 = v3 ^ 0x33;",
        ),
    ] {
        let path = fixture(name);
        let lib = path.to_str().unwrap();
        let first = kuna(&["decompile", lib, ternary, "--addr"]);
        assert!(
            first.contains("a0 = (0xb <= a0) ? a0 + -10 : a0 << 1;"),
            "{name}: {first}"
        );
        let second = kuna(&["decompile", lib, popcount.0, "--addr"]);
        assert!(second.contains(popcount.1), "{name}: {second}");
        let arm = kuna(&["decompile", lib, "arm_export"]);
        assert!(arm.contains(arm_body), "{name}: {arm}");
        let all = kuna(&["decompile-all", lib]);
        for entry in [ternary, popcount.0] {
            let header = format!("@ {entry}");
            assert!(all.contains(&header), "{name}: no {entry} in\n{all}");
        }
    }
}

/// A stripped A32-entry executable (`arm_pointer_thumb_exe_le32.c`) whose
/// `_start` calls a Thumb `t0` through `blx` and reaches `t_ptr`, which sits
/// after `t0`, only through a pointer table: no evidence says what `t_ptr`
/// is, so it keeps the mode the database gives it, which is the Thumb the
/// `blx` wrote from `t0` onwards.
#[test]
fn a_pointer_only_thumb_function_keeps_the_mode_a_blx_before_it_gives() {
    let path = fixture("arm_pointer_thumb_exe_le32");
    let exe = path.to_str().unwrap();
    let t_ptr = kuna(&["decompile", exe, "0x10172", "--addr"]);
    assert!(t_ptr.contains("v2 += sub_10158(v1);"), "{t_ptr}");
    let t0 = kuna(&["decompile", exe, "0x1013c", "--addr"]);
    assert!(t0.contains("return a0 * 7 + 1;"), "{t0}");
}

/// An ARMv4T image (`arm_thumb_veneer_le32.c`, symbols kept, no mapping
/// symbols) whose Thumb `thumb_caller` reaches the A32 `arm_target` through
/// the linker's `bx pc; b.n; b arm_target` veneer at 0x10120: the `bx pc`
/// continues in A32, so the veneer is a tail call, not undecodable Thumb.
#[test]
fn a_thumb_bx_pc_veneer_continues_in_arm() {
    let path = fixture("arm_thumb_veneer_le32");
    let exe = path.to_str().unwrap();
    let veneer = kuna(&["decompile", exe, "0x10120", "--addr"]);
    assert!(veneer.contains("arm_target(a0); // tail-call"), "{veneer}");
    assert!(!veneer.contains("halt_baddata"), "{veneer}");
}
