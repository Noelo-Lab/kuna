//! (kuna) `retpolinethunk` — a retpoline thunk recognized by its body.
//!
//! `-mretpoline` (clang) and `-mindirect-branch=thunk` (gcc) turn every indirect
//! call into a direct call to a per-register thunk:
//!
//! ```text
//!   call 1f           ; e8 rel32
//! 2: pause            ; f3 90
//!   lfence            ; 0f ae e8
//!   jmp 2b            ; eb f9
//! 1: mov %reg,(%rsp)  ; [rex] 89 /r 24
//!   ret               ; c3
//! ```
//!
//! The thunk returns into the target held in the register, so a call to it is
//! `call *%reg`. The cspecs carry that body as the `x86_indirect_thunk_<reg>`
//! call-fixups, which the call-fixup installer applies by the name gcc gives
//! the thunk. clang's thunks are named `__llvm_retpoline_<reg>`, and a stripped
//! image names neither, so the call rendered as an argument-taking call of the
//! thunk itself. This reads the callee's first bytes for the shape above and
//! hands the matching fixup's inject id to the flow builder.

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::architecture::Architecture;

const RET_SPIN: [u8; 7] = [0xf3, 0x90, 0x0f, 0xae, 0xe8, 0xeb, 0xf9];
const REG64: [&str; 16] = [
    "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
];
const REG32: [&str; 8] = ["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];

/// The register a retpoline thunk at `entry` jumps through, read from its body.
pub fn thunk_register(read: impl Fn(u64) -> Option<u8>) -> Option<&'static str> {
    let b = |i: u64| read(i);
    if b(0)? != 0xe8 {
        return None;
    }
    let rel = i32::from_le_bytes([b(1)?, b(2)?, b(3)?, b(4)?]);
    if (0..RET_SPIN.len() as u64).any(|i| b(5 + i) != Some(RET_SPIN[i as usize])) {
        return None;
    }
    let t = 5i64 + i64::from(rel);
    if !(12..=64).contains(&t) {
        return None;
    }
    let mut at = t as u64;
    let rex = match b(at)? {
        r @ (0x48 | 0x4c) => {
            at += 1;
            Some(r)
        }
        _ => None,
    };
    let modrm = b(at + 1)?;
    if b(at)? != 0x89 || modrm & 0xc7 != 0x04 || b(at + 2)? != 0x24 || b(at + 3)? != 0xc3 {
        return None;
    }
    let reg = usize::from((modrm >> 3) & 7);
    match rex {
        Some(r) => Some(REG64[reg + if r == 0x4c { 8 } else { 0 }]),
        None => Some(REG32[reg]),
    }
}

/// The inject id of the `x86_indirect_thunk_<reg>` call-fixup for a retpoline
/// thunk at `entry`, or -1.
pub fn inject_id(arch: &Architecture, entry: &Address) -> int4 {
    if !arch.retpoline_thunk || !arch.get_description().starts_with("x86:") {
        return -1;
    }
    let read = |i: u64| arch.read_loadimage_value(&(entry + i as i64), 1).ok().map(|v| v as u8);
    let Some(reg) = thunk_register(read) else { return -1 };
    let name = format!("x86_indirect_thunk_{reg}");
    arch.pcodeinjectlib.base.get_payload_id(crate::pcodeinject::CALLFIXUP_TYPE, name.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reader(bytes: &[u8]) -> impl Fn(u64) -> Option<u8> + '_ {
        move |i| bytes.get(i as usize).copied()
    }

    #[test]
    fn clang_and_gcc_thunks_name_their_register() {
        let clang = [
            0xe8, 0x0b, 0, 0, 0, 0xf3, 0x90, 0x0f, 0xae, 0xe8, 0xeb, 0xf9, 0x0f, 0x1f, 0x40, 0x00, 0x4c, 0x89, 0x1c,
            0x24, 0xc3,
        ];
        assert_eq!(thunk_register(reader(&clang)), Some("r11"));
        let gcc = [0xe8, 0x07, 0, 0, 0, 0xf3, 0x90, 0x0f, 0xae, 0xe8, 0xeb, 0xf9, 0x48, 0x89, 0x04, 0x24, 0xc3];
        assert_eq!(thunk_register(reader(&gcc)), Some("rax"));
        let i386 = [0xe8, 0x07, 0, 0, 0, 0xf3, 0x90, 0x0f, 0xae, 0xe8, 0xeb, 0xf9, 0x89, 0x0c, 0x24, 0xc3];
        assert_eq!(thunk_register(reader(&i386)), Some("ecx"));
        let plain_call = [0xe8, 0x07, 0, 0, 0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x48, 0x89, 0x04, 0x24, 0xc3];
        assert_eq!(thunk_register(reader(&plain_call)), None);
    }
}
