//! ARM/Thumb interworking: a `blx` selects Thumb for its own target only, a `bl`
//! keeps its caller's mode, and neither leaks into the other helper (GH-780,
//! option `flowmode`, off by default). Under `flowmode on` nothing after an
//! unconditional call or a user-defined operation is proven; `flowmode
//! aftercall` continues past a call whose callee returns. Images outside the
//! option's scope keep what the walk alone gives them.

#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;
use crate::common;

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

fn run(args: &[&str], extra: &[&str]) -> String {
    kuna(&[args, extra].concat())
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

fn decompile_with(path: &PathBuf, addr: u64, extra: &[&str]) -> String {
    let addr = format!("{addr:#x}");
    let mut args = vec!["decompile", path.to_str().unwrap(), &addr, "--json"];
    args.extend_from_slice(extra);
    kuna(&args)
}

fn decompile(path: &PathBuf, addr: u64, mode: &str) -> String {
    decompile_with(path, addr, &["--mode", mode])
}

fn decompile_off(path: &PathBuf, addr: u64) -> String {
    decompile_with(path, addr, &["--option", "flowmode", "off"])
}

const ON: [&str; 3] = ["--option", "flowmode", "on"];
const AFTERCALL: [&str; 3] = ["--option", "flowmode", "aftercall"];
const VALUES: [&[&str]; 3] = [&[], &ON, &AFTERCALL];

/// The body `decompile-all` printed for `name`.
fn body(all: &str, name: &str) -> String {
    let start = all
        .find(&format!("// Function: {name}"))
        .unwrap_or_else(|| panic!("no {name} in\n{all}"));
    let rest = &all[start + 1..];
    rest[..rest.find("// Function:").unwrap_or(rest.len())].to_string()
}

/// A32 `b .`, a function that never returns.
const SPIN: u32 = 0xEAFF_FFFE;

fn assert_helpers(path: &PathBuf, extra: &[&str]) {
    let arm = decompile_with(path, ARM_HELPER, extra);
    assert!(
        arm.contains("return a0 + 1;"),
        "{extra:?}: the A32 helper lost its mode: {arm}"
    );
    assert!(!arm.contains("halt_"), "{extra:?}: {arm}");
    let thumb = decompile_with(path, THUMB_HELPER, extra);
    assert!(
        thumb.contains("return 7;"),
        "{extra:?}: the Thumb helper lost its mode: {thumb}"
    );
}

/// `entry: bl arm_helper; blx thumb_helper; pop {pc}`: the `bl` is the
/// entry's first call, so its target is proven A32 under `flowmode on`.
#[test]
fn a_blx_target_mode_does_not_reach_the_arm_callee_of_an_earlier_bl() {
    let path = stripped("bl-blx", &bl_then_blx());
    for mode in ["reliable", "auto"] {
        assert_helpers(&path, &["--mode", mode, "--option", "flowmode", "on"]);
    }
    let off = decompile_off(&path, ARM_HELPER);
    assert!(off.contains("halt_missing"), "flowmode off: {off}");
    assert_eq!(decompile_with(&path, ARM_HELPER, &[]), off);
    std::fs::remove_file(path).unwrap();
}

/// `entry: blx thumb_helper; bl arm_helper; pop {pc}` (GH-780): the `bl`
/// follows a call, so under `on` nothing proves it runs and every function
/// prints what the walk alone gives it; `aftercall` proves it once the Thumb
/// helper is proven to return.
#[test]
fn a_bl_after_a_returning_blx_is_proven_only_under_aftercall() {
    let path = stripped("blx-bl", &blx_then_bl());
    for addr in [0x10000, THUMB_HELPER, ARM_HELPER] {
        assert_eq!(decompile_with(&path, addr, &ON), decompile_off(&path, addr));
    }
    assert!(decompile_off(&path, ARM_HELPER).contains("halt_missing"));
    assert_helpers(&path, &AFTERCALL);
    std::fs::remove_file(path).unwrap();
}

/// `entry: bl f1; bl arm_helper; pop {pc}` where `f1: push {lr}; blx
/// thumb_helper; pop {pc}`: under `aftercall` the second call is proven only
/// once `f1` is proven to return, which needs `thumb_helper` proven to return
/// first.
#[test]
fn a_call_proven_to_return_carries_the_mode_on_to_the_next_call() {
    let mut code = text(&[
        PUSH_LR,
        bl(0x10004, 0x10020),
        bl(0x10008, ARM_HELPER),
        POP_PC,
    ]);
    for (i, word) in [PUSH_LR, blx(0x10024, THUMB_HELPER), POP_PC]
        .iter()
        .enumerate()
    {
        code[0x20 + i * 4..0x24 + i * 4].copy_from_slice(&word.to_le_bytes());
    }
    let path = stripped("returns-chain", &code);
    assert_helpers(&path, &AFTERCALL);
    let off = decompile_off(&path, ARM_HELPER);
    assert!(off.contains("halt_missing"), "flowmode off: {off}");
    assert_eq!(decompile_with(&path, ARM_HELPER, &ON), off);
    std::fs::remove_file(path).unwrap();
}

/// `entry: blx thumb_helper; bl arm_last; pop {pc}`, where the A32
/// `arm_last` at 0x10050, inside the Thumb the `blx` writes from 0x10040,
/// ends in a call to `spin` (`b .`) that never returns, and the Thumb
/// `movs r0,#9; bx lr` that follows it at 0x10058 is reached by nothing the
/// walk follows. The bytes after the call are not proven in either mode, so
/// they keep the walk's Thumb; nor is `arm_last`, whose call is never shown
/// to return, so it prints what the walk gives it.
#[test]
fn the_instruction_after_a_call_that_never_returns_is_not_proven() {
    let mut code = text(&[
        PUSH_LR,
        blx(0x10004, THUMB_HELPER),
        bl(0x10008, 0x10050),
        POP_PC,
    ]);
    code[0x10..0x14].copy_from_slice(&SPIN.to_le_bytes());
    for (i, word) in [PUSH_LR, bl(0x10054, 0x10010)].iter().enumerate() {
        code[0x50 + i * 4..0x54 + i * 4].copy_from_slice(&word.to_le_bytes());
    }
    code[0x58..0x5c].copy_from_slice(&[0x09, 0x20, 0x70, 0x47]);
    let path = stripped("noreturn", &code);
    let after = decompile_off(&path, 0x10058);
    assert!(after.contains("return 9;"), "{after}");
    for extra in [&ON[..], &AFTERCALL[..]] {
        assert_eq!(decompile_with(&path, 0x10058, extra), after, "{extra:?}");
        assert_eq!(
            decompile_with(&path, 0x10050, extra),
            decompile_off(&path, 0x10050),
            "{extra:?}"
        );
    }
    std::fs::remove_file(path).unwrap();
}

/// `entry: blx helper; bl helper; pop {pc}`: under `aftercall` two proofs
/// give one address different modes, so nothing is painted and every
/// function prints what the walk alone gives it, as it does under `on`.
#[test]
fn two_proven_modes_for_one_address_paint_nothing() {
    let code = text(&[
        PUSH_LR,
        blx(0x10004, THUMB_HELPER),
        bl(0x10008, THUMB_HELPER),
        bl(0x1000c, ARM_HELPER),
        POP_PC,
    ]);
    let path = stripped("conflict", &code);
    for addr in [0x10000, THUMB_HELPER, ARM_HELPER] {
        let off = decompile_with(
            &path,
            addr,
            &["--mode", "auto", "--option", "flowmode", "off"],
        );
        for extra in [
            &["--mode", "auto", "--option", "flowmode", "on"][..],
            &["--mode", "auto", "--option", "flowmode", "aftercall"],
        ] {
            assert_eq!(decompile_with(&path, addr, extra), off, "{extra:?}");
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn whole_binary_decompile_keeps_each_callee_in_its_own_mode() {
    for (stem, code, extra) in [
        ("bl-blx-all", bl_then_blx(), &ON[..]),
        ("blx-bl-all", blx_then_bl(), &AFTERCALL[..]),
    ] {
        let path = stripped(stem, &code);
        let mut args = vec!["decompile-all", path.to_str().unwrap()];
        args.extend_from_slice(extra);
        let all = kuna(&args);
        assert!(
            body(&all, "sub_10040").contains("return 7;"),
            "{stem}: {all}"
        );
        assert!(
            body(&all, "sub_10080").contains("return a0 + 1;"),
            "{stem}: {all}"
        );
        let entry = body(&all, "sub_10000");
        assert!(
            entry.contains("sub_10040()") && entry.contains("sub_10080("),
            "{stem}: {all}"
        );
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn mapping_symbols_still_select_each_mode() {
    let path = marked("blx-bl-marked", &blx_then_bl());
    assert_helpers(&path, &["--mode", "reliable"]);
    let entry = decompile(&path, 0x10000, "reliable");
    assert!(entry.contains("arm_helper(thumb_helper())"), "{entry}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn a_register_interworking_call_leaves_the_arm_callee_alone() {
    let path = stripped("blx-r3", &register_blx_then_bl());
    let arm = decompile(&path, ARM_HELPER, "reliable");
    assert!(arm.contains("return a0 + 1;"), "{arm}");
    std::fs::remove_file(path).unwrap();

    let path = marked("blx-r3-marked", &register_blx_then_bl());
    assert_helpers(&path, &["--mode", "reliable"]);
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
    assert_helpers(&path, &["--mode", "reliable"]);
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
    assert!(body(&all, "thumb_tail").contains("sub_10020("), "{all}");
    assert!(body(&all, "sub_10020").contains("return 9;"), "{all}");
    std::fs::remove_file(path).unwrap();
}

fn fixture(name: &str) -> PathBuf {
    common::repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
}

/// Stripped armel shared objects (`arm_interwork_stubs_le32.c`) whose every
/// export has an even address (the Thumb ones are A32 linker stubs), so no
/// symbol paints Thumb: the A32 `arm_export`, which calls three functions, is
/// proven by its own symbol only under `aftercall`, where it keeps A32 over
/// the Thumb a `blx` below it writes, and the Thumb `t_ptr_only`, reached
/// only through a pointer table, keeps the walk's Thumb.
#[test]
fn arm_exports_are_proven_and_pointer_only_thumb_keeps_its_mode() {
    let path = fixture("arm_interwork_stubs_o0_le32");
    let lib = path.to_str().unwrap();
    let arm = kuna(&[
        "decompile",
        lib,
        "arm_export",
        "--option",
        "flowmode",
        "aftercall",
    ]);
    assert!(arm.contains("return v1 + v2 + sub_294(a0);"), "{arm}");
    let off = kuna(&[
        "decompile",
        lib,
        "arm_export",
        "--option",
        "flowmode",
        "off",
    ]);
    assert!(off.contains("halt_missing"), "flowmode off: {off}");
    assert_eq!(kuna(&["decompile", lib, "arm_export"]), off);
    let on = kuna(&["decompile", lib, "arm_export", "--option", "flowmode", "on"]);
    assert_eq!(on, off);
    for (name, pointer_only, ternary) in [
        (
            "arm_interwork_stubs_o0_le32",
            "0x398",
            "(0xb <= a0) ? a0 + -10 : a0 << 1",
        ),
        ("arm_interwork_stubs_o2_le32", "0x304", "v1 = a0 + -10;"),
    ] {
        let path = fixture(name);
        let pointer = kuna(&["decompile", path.to_str().unwrap(), pointer_only, "--addr"]);
        assert!(pointer.contains(ternary), "{name}: {pointer}");
    }
}

/// Stripped armel shared objects (`arm_static_thumb_so_le32.c`): a Thumb
/// export, an A32 export, then static Thumb functions reached only through a
/// pointer table. The Thumb export's symbol paints the image, so `flowmode`
/// leaves it alone and the static code keeps the Thumb that paint gives it.
#[test]
fn static_thumb_functions_after_an_arm_export_keep_their_mode() {
    for (name, ternary, popcount) in [
        (
            "arm_static_thumb_so_o0_le32",
            "0x280",
            ("0x2a2", "v2 = (v1 & 1) + v2;"),
        ),
        (
            "arm_static_thumb_so_o2_le32",
            "0x234",
            ("0x240", "v1 += v2;"),
        ),
    ] {
        let path = fixture(name);
        let lib = path.to_str().unwrap();
        for extra in VALUES {
            let first = run(&["decompile", lib, ternary, "--addr"], extra);
            assert!(
                first.contains("a0 = (0xb <= a0) ? a0 + -10 : a0 << 1;"),
                "{name} {extra:?}: {first}"
            );
            let second = run(&["decompile", lib, popcount.0, "--addr"], extra);
            assert!(second.contains(popcount.1), "{name} {extra:?}: {second}");
            let all = run(&["decompile-all", lib], extra);
            for entry in [ternary, popcount.0] {
                let header = format!("@ {entry}");
                assert!(all.contains(&header), "{name} {extra:?}: no {entry} in\n{all}");
            }
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
    for extra in VALUES {
        let t_ptr = run(&["decompile", exe, "0x10172", "--addr"], extra);
        assert!(t_ptr.contains("v2 += sub_10158(v1);"), "{extra:?}: {t_ptr}");
        let t0 = run(&["decompile", exe, "0x1013c", "--addr"], extra);
        assert!(t0.contains("return a0 * 7 + 1;"), "{extra:?}: {t0}");
    }
}

/// Stripped armel shared objects (`arm_noreturn_fallthrough_le32.c`) where a
/// function ends in a call to the `noreturn` import `fatal`, and the next
/// function, in the other mode, is reached only through a `blx`: the Thumb
/// `thumb_last` before the A32 `arm_hidden` at 0x4bc, and the A32 `arm_last`
/// before the Thumb `thumb_hidden` at 0x4a4. Both loops decompile, alone and
/// in the whole-binary run.
#[test]
fn a_call_that_never_returns_leaves_the_next_function_in_its_own_mode() {
    for (name, addr) in [
        ("arm_noreturn_fallthrough_thumb_le32", "0x4bc"),
        ("arm_noreturn_fallthrough_arm_le32", "0x4a4"),
    ] {
        let path = fixture(name);
        let lib = path.to_str().unwrap();
        for extra in VALUES {
            let alone = run(&["decompile", lib, addr, "--addr"], extra);
            assert!(alone.contains("v1 += v2 ^ 5;"), "{name} {extra:?}: {alone}");
            let all = run(&["decompile-all", lib], extra);
            let named = format!("sub_{}", &addr[2..]);
            assert!(
                body(&all, &named).contains("v1 += v2 ^ 5;"),
                "{name} {extra:?}: {all}"
            );
        }
    }
}

/// A stripped armel shared object (`arm_unsized_export_le32.c`): the Thumb
/// export `thumb_first`, the A32 assembly export `asm_nosz` with no `.size`,
/// then the static Thumb `tsum` and `tpop` at 0x410 and 0x430. Both statics
/// decompile in the whole-binary run.
#[test]
fn static_thumb_functions_after_an_unsized_arm_export_keep_their_mode() {
    let path = fixture("arm_unsized_export_le32");
    for extra in VALUES {
        let all = run(&["decompile-all", path.to_str().unwrap()], extra);
        assert!(body(&all, "sub_410").contains("v1 += v2 ^ 5;"), "{extra:?}: {all}");
        assert!(body(&all, "sub_430").contains("a0 >>= 1"), "{extra:?}: {all}");
        assert!(!body(&all, "sub_430").contains("halt_"), "{extra:?}: {all}");
    }
}

/// A stripped armel shared object (`arm_noreturn_site_le32.c`) where the A32
/// `a_last` calls `report_bad`, which returns, and then
/// `__builtin_unreachable()`, so nothing follows the call, and the Thumb `u`
/// at 0x524 comes next. `u` keeps its Thumb decode, alone and in the
/// whole-binary run.
#[test]
fn a_call_that_returns_elsewhere_but_not_at_its_site_leaves_the_next_function_alone() {
    let path = fixture("arm_noreturn_site_le32");
    let lib = path.to_str().unwrap();
    let expected = "return (a0 * 8 - ((int)a1 >> 1)) + (a0 & a1);";
    for extra in [&[][..], &ON[..]] {
        let alone = run(&["decompile", lib, "0x524", "--addr"], extra);
        assert!(alone.contains(expected), "{extra:?}: {alone}");
        let all = run(&["decompile-all", lib], extra);
        assert!(body(&all, "sub_524").contains(expected), "{extra:?}: {all}");
    }
}

/// A stripped armel shared object (`arm_exception_end_le32.c`): the A32
/// export `my_exit` ends in `svc #0` and `my_trap` in `bkpt #1`, each then
/// `__builtin_unreachable()`, so the static Thumb `u` at 0x510 and `v` at
/// 0x56c, which the A32 `api2` calls with `blx`, come right after them.
/// Neither exception is proven to come back, so `u` and `v` keep their Thumb
/// decode under every value, alone and in the whole-binary run.
#[test]
fn an_exception_that_does_not_come_back_leaves_the_next_function_alone() {
    let path = fixture("arm_exception_end_le32");
    let lib = path.to_str().unwrap();
    let u = "return (a1 & a0) + (a0 * 8 - ((int)a1 >> 1));";
    let v = "a0 = (a0 <= a1) ? a1 * 3 + 1 : a0 - a1;";
    for extra in VALUES {
        for (addr, expected) in [("0x510", u), ("0x56c", v)] {
            let alone = run(&["decompile", lib, addr, "--addr"], extra);
            assert!(alone.contains(expected), "{addr} {extra:?}: {alone}");
        }
        let all = run(&["decompile-all", lib], extra);
        assert!(body(&all, "sub_510").contains(u), "{extra:?}: {all}");
        assert!(body(&all, "sub_56c").contains(v), "{extra:?}: {all}");
    }
}
