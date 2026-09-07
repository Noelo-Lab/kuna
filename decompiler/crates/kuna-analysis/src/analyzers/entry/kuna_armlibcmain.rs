//! (kuna) `armlibcmain` — recover `main` from the **non-PIE** ARM crt1, where
//! `_start` hands it to `__libc_start_main` through a PC-relative literal pool
//! word rather than through a relocated GOT slot (P1 program prep).
//!
//! A stripped, non-PIE ARM32 executable answers `kuna functions` with the PLT
//! imports, the `DT_INIT`/`INIT_ARRAY` entries, and `_start` — and nothing that
//! covers the program's own body. The consequence is not a short inventory but a
//! blind spot: the gap between the last discovered entry and the end of `.text`
//! is attributed to whichever entry precedes it, so the walk that follows that
//! entry's control flow stops at its real epilogue and never decodes another
//! instruction. Every literal the missing code loads therefore has no reader.
//! On the witness (a stripped ARM crackme) `kuna strings` reported the program's
//! own input prompt with `xrefs_count 0` and an empty `functions` list although
//! `main` loads it from a pool word four instructions before its `printf` call.
//!
//! Entry-discovery oracle 4 already recovers `main` from this idiom on x86-64,
//! AArch64, RISC-V and **PIE** ARM. The PIE ARM path is written around a
//! relocation: crt1 loads `main` from a GOT slot, and the slot is identified by
//! the `R_ARM_RELATIVE` that a position-independent image must carry to relocate
//! it. A non-PIE image carries no such relocation — the linker knows the final
//! address and stores it — so that cross-check finds nothing and the oracle
//! returns `None`. This pass is the arm of oracle 4 that reads the value out of
//! the image instead of out of the relocation table.
//!
//! ## What it decodes
//!
//! Both shapes glibc's ARM `crt1.o` has shipped, distinguished by the
//! instruction that follows the pool load, and both anchored on the same
//! landmark — the `bl` to the PLT stub the import table names
//! `__libc_start_main`:
//!
//! - **literal pool** (the classic `crt1.o`): `ldr r0,[pc,#imm]` where the pool
//!   word IS `main`'s address.
//! - **GOT-indexed** (what a modern `-no-pie` link supplies):
//!   `ldr r0,[pc,#imm] ; ldr r0,[rN,r0]`, where the pool word is a `.got`-
//!   relative offset and the slot at `.got + off` holds `main`'s address,
//!   written by the linker with no dynamic relocation to find it by.
//!
//! The `bl __libc_start_main` is what makes the recovered word `main` rather
//! than a plausible code address: the ARM procedure call standard puts the first
//! argument in `r0`, and the last write to `r0` before that call is by
//! definition the value the C runtime is being handed as `main`.
//!
//! ## What it refuses
//!
//! - any image that is not a 32-bit ARM ELF, and any Thumb `_start` (`e_entry`
//!   with bit 0 set) — glibc's ARM crt1 is A32 and the shapes above are A32
//!   encodings;
//! - an image with no PLT stub named `__libc_start_main` — a static link, or an
//!   entry point that is not a C runtime, leaves the landmark absent and nothing
//!   is claimed;
//! - a `bl __libc_start_main` with no `ldr r0,[pc,#imm]` before it inside the
//!   `_start` window;
//! - a recovered address outside every executable section, or one that already
//!   carries a function symbol (whatever named it has the better name).
//!
//! Like every other analysis pass the facts are computed at LOAD and COMMITTED
//! only when the gate is on, so `--option armlibcmain off` restores the previous
//! inventory exactly.

use object::read::{Object, ObjectSection};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{executable_sections, existing_function_addrs, in_executable_section};

/// The name the `__libc_start_main` call site licenses — its first argument is
/// `main` by the C runtime's own contract.
const MAIN: &str = "main";

/// The import whose call site anchors the decode.
const LIBC_START_MAIN: &[u8] = b"__libc_start_main";

/// How far past `e_entry` the crt1 window runs. glibc's ARM `_start` is under
/// 0x40 bytes of code plus its pool; 0x80 covers both shapes with room and keeps
/// the scan away from whatever follows.
const WINDOW: u64 = 0x80;

/// (kuna) The non-PIE ARM `_start`→`main` pass (`armlibcmain`).
pub struct ArmLibcMainPass;

impl AnalysisPass for ArmLibcMainPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "armlibcmain"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let Some(vma) = main_vma(ctx.file, ctx.bytes) else {
            return out;
        };
        out.entries.push(vma);
        out.entry_names.push((vma, MAIN.to_string()));
        out
    }
}

/// The VMA the non-PIE crt1 hands to `__libc_start_main`, once every refusal in
/// the module header has been applied. `None` means this pass contributes
/// nothing and the commit is byte-identical to before.
fn main_vma(file: &object::File, bytes: &[u8]) -> Option<u64> {
    if file.architecture() != object::Architecture::Arm {
        return None;
    }
    let entry = file.entry();
    // A Thumb `_start` (the mode bit in `e_entry`) is not the shape below.
    if entry == 0 || entry & 1 != 0 {
        return None;
    }
    let stub = libc_start_main_stub(file)?;
    let (sec_addr, data) = super::section_bytes_containing(file, entry)?;
    let start = (entry - sec_addr) as usize;
    let end = (start + WINDOW as usize).min(data.len());
    let vma = decode_window(file, entry, data.get(start..end)?, stub)? & !1;
    if vma == entry || !in_executable_section(&executable_sections(file), vma) {
        return None;
    }
    if existing_function_addrs(file, bytes).binary_search(&vma).is_ok() {
        return None;
    }
    Some(vma)
}

/// Walk the `_start` window as A32 instructions, remembering the most recent
/// `ldr r0,[pc,#imm]` and the instruction after it, and resolve that pair the
/// moment the `bl` to `stub` is reached.
fn decode_window(file: &object::File, entry: u64, window: &[u8], stub: u64) -> Option<u64> {
    let le = file.is_little_endian();
    let mut pool: Option<(u64, u32)> = None; // (pool word vma, next instruction)
    let mut off = 0usize;
    while off + 4 <= window.len() {
        let addr = entry + off as u64;
        let insn = super::read_u32(&window[off..], le);
        if let Some(pool_vma) = ldr_literal(insn, addr, 0) {
            let next =
                window.get(off + 4..off + 8).map(|b| super::read_u32(b, le)).unwrap_or(0);
            pool = Some((pool_vma, next));
        } else if bl_target(insn, addr) == Some(stub) {
            let (pool_vma, next) = pool?;
            let word = read_word_at(file, pool_vma, le)?;
            return if is_ldr_reg_offset_r0(next) {
                // GOT-indexed: the pool word is a `.got`-relative offset and the
                // slot the linker filled holds `main`.
                let got = file.section_by_name(".got")?.address();
                read_word_at(file, got.checked_add(word as u64)?, le).map(u64::from)
            } else {
                Some(u64::from(word))
            };
        }
        off += 4;
    }
    None
}

/// The address of the PLT stub the import table names `__libc_start_main`.
fn libc_start_main_stub(file: &object::File) -> Option<u64> {
    crate::loader::elf_plt::resolve_plt_imports(file)
        .into_iter()
        .find(|s| s.name == LIBC_START_MAIN)
        .map(|s| s.addr)
}

/// `ldr <rt>,[pc,#±imm12]` (A32): `cond 010 P U 0 W 1 1111 rt imm12` with
/// `P=1, U=?, B=0, W=0, L=1, Rn=15`. Returns the literal's VMA — the ARM PC is
/// two instructions ahead of the one being executed.
fn ldr_literal(insn: u32, addr: u64, rt: u32) -> Option<u64> {
    let masked = insn & 0x0FFF_F000;
    let up = match masked {
        m if m == 0x059F_0000 | (rt << 12) => true,
        m if m == 0x051F_0000 | (rt << 12) => false,
        _ => return None,
    };
    let imm = (insn & 0xFFF) as u64;
    let pc = addr.wrapping_add(8);
    Some(if up { pc.wrapping_add(imm) } else { pc.wrapping_sub(imm) })
}

/// `ldr r0,[rN,rM]` (A32 register-offset load into `r0`): `cond 011 P U 0 W 1
/// rn 0000 shift rm`. Only the destination and the register form matter — the
/// base is whatever the crt1 computed the GOT into.
fn is_ldr_reg_offset_r0(insn: u32) -> bool {
    (insn & 0x0E50_F000) == 0x0610_0000 && (insn & 0x10) == 0
}

/// `bl <target>` (A32): `cond 1011 imm24`, the target `pc + (sign_extend(imm24)
/// << 2)` with the ARM PC two instructions ahead. `blx` (`cond == 0xF`) is not
/// accepted — a PLT stub is entered in A32 state.
fn bl_target(insn: u32, addr: u64) -> Option<u64> {
    if (insn >> 28) == 0xF || (insn >> 24) & 0xF != 0xB {
        return None;
    }
    let imm = ((insn & 0x00FF_FFFF) << 8) as i32 >> 6; // sign-extend imm24, <<2
    Some(addr.wrapping_add(8).wrapping_add(imm as i64 as u64))
}

/// The 4-byte word the image stores at `vma`, or `None` when no section holds it.
fn read_word_at(file: &object::File, vma: u64, le: bool) -> Option<u32> {
    let (sec_addr, data) = super::section_bytes_containing(file, vma)?;
    let off = (vma - sec_addr) as usize;
    let b = data.get(off..off + 4)?;
    Some(super::read_u32(b, le))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    fn claim(name: &str) -> Option<u64> {
        let bytes = fixture(name);
        let file = object::File::parse(bytes.as_slice()).unwrap_or_else(|e| panic!("{name}: {e}"));
        main_vma(&file, bytes.as_slice())
    }

    /// The headline shape: the classic `crt1.o` hands `main` over in a literal
    /// pool word, and nothing in the image relocates it.
    #[test]
    fn claims_main_from_the_literal_pool_of_a_non_pie_start() {
        assert_eq!(claim("armlibcmain_le32"), Some(0x103dc));
    }

    /// The shape a modern `-no-pie` link supplies: the pool word is a `.got`
    /// offset and the slot the linker filled holds `main`.
    #[test]
    fn claims_main_from_a_statically_filled_got_slot() {
        assert_eq!(claim("armlibcmain_got_le32"), Some(0x10518));
    }

    /// The refusal that keeps this off the PIE ARM images oracle 4 already
    /// resolves through their `R_ARM_RELATIVE` table, and off a static link with
    /// no `__libc_start_main` stub to anchor the decode.
    #[test]
    fn defers_to_the_pie_and_static_arm_shapes() {
        assert_eq!(claim("entrymain_arm"), None, "PIE crt1 is oracle 4's own");
        assert_eq!(claim("poolref_arm_le32"), None, "no PLT stub to anchor on");
        assert_eq!(claim("arm_thumb_linked_le32"), None, "static, and already named");
    }

    /// Structurally inert off 32-bit ARM ELF, which is why no parity assertion
    /// on any other target can move.
    #[test]
    fn contributes_nothing_off_arm32_elf() {
        assert_eq!(claim("cet_pie_x86_64"), None, "x86-64 ELF");
        assert_eq!(claim("entrymain_aarch64"), None, "AArch64 ELF");
        assert_eq!(claim("macho_imports_arm64"), None, "Mach-O");
        assert_eq!(claim("armv4t_thumb_pe.exe"), None, "ARM PE");
    }

    /// The three A32 decoders, on the exact words the witness `_start` carries
    /// (`arm_kgme1` @0x8380/0x838c) and the GOT-indexed fixture (@0x10450).
    #[test]
    fn decodes_the_a32_words_the_idiom_is_built_from() {
        // ldr r0,[pc,#0x10] at 0x8380 -> the pool word at 0x8398.
        assert_eq!(ldr_literal(0xe59f_0010, 0x8380, 0), Some(0x8398));
        // The same encoding with U=0 subtracts.
        assert_eq!(ldr_literal(0xe51f_0010, 0x8380, 0), Some(0x8378));
        // r12, not r0.
        assert_eq!(ldr_literal(0xe59f_c024, 0x8368, 0), None);
        // bl 0x8320 at 0x838c (a backwards branch).
        assert_eq!(bl_target(0xebff_ffe3, 0x838c), Some(0x8320));
        // blx is not a PLT-stub entry.
        assert_eq!(bl_target(0xfbff_ffe3, 0x838c), None);
        // ldr r0,[r10,r0] — the GOT index.
        assert!(is_ldr_reg_offset_r0(0xe79a_0000));
        // ldr r3,[r10,r3] writes r3, not r0.
        assert!(!is_ldr_reg_offset_r0(0xe79a_3003));
    }
}
