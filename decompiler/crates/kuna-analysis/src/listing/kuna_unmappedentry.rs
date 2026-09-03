//! (kuna) `unmappedentry` — the recursive-descent walk never mints a function
//! at a CALL target it is not allowed to decode (P1 code/data partition).
//!
//! [`super::walk`] runs a two-level worklist: an inner instruction worklist and
//! an outer function worklist that a direct CALL target feeds. The two levels
//! disagreed about what counts as code. The inner loop gates every address on
//! the executable-range universe (`in_exec`, the `flow.rs:891` out-of-bounds
//! analog) and silently drops anything outside it; the outer level took the CALL
//! target unconditionally, so a target the walk was about to refuse still became
//! a `DiscoveredFunction` — an entry with no instructions, no extent and no
//! bytes behind it, which `fast_funcdisc` / `funcdisc_recursive` then commit as
//! a real function.
//!
//! Such an entry is not a discovery that turned out empty; it is one the walk
//! had already decided it could never decode. Nothing downstream can recover
//! from it: the address is outside every executable section, so `kuna decompile`
//! reports no body, `kuna functions` prints `size: 0`, and the inventory an agent
//! is supposed to trust carries a function the image does not contain.
//!
//! ## Where the phantom targets come from
//!
//! A CALL whose target is unmapped is not a decode failure — the CALL itself
//! decoded correctly. It is what anti-disassembly bytes look like once you follow
//! them. `crackmes.one/66fd63059b533b4c22bd0b91` (32-bit ELF, "Overlapping /
//! misaligned instructions") ends its decrypt loop with
//!
//! ```text
//!   4509: 31 c0           xor eax,eax
//!   450b: 74 01           je  450e          ; ZF is 1 -- always taken
//!   450d: e8 8d 85 6d fe  call 0xfe6dca9f   ; never executed; 450e is `lea eax,...`
//! ```
//!
//! The `e8` is the first byte of the real instruction at `450e` read one byte
//! early. The walk follows the `je`'s fall-through (it does not evaluate flags),
//! decodes the overlapping `call`, and hands `0xfe6dca9f` to the function
//! worklist — an address 4 GB above a 25 KB image. The same shape appears in any
//! image with junk after an always-taken branch, in a switch's default arm read
//! at the wrong alignment, and in a padding gap decoded as code.
//!
//! ## What the gate does NOT do
//!
//! The Call **reference** is still filed in both directions. The instruction
//! genuinely encodes a call to that address and `kuna xrefs` should say so; what
//! changes is only that the target stops being claimed as a *function*. Nor does
//! this touch a call into a mapped-but-undecodable region: if the address is
//! inside an executable section the entry is created exactly as before, even when
//! the decode there fails, because that case is a real gap in the walk rather
//! than a fabricated one.
//!
//! ## Measured
//!
//! Over 234 crackmes images (ELF/PE/Mach-O, x86/x86-64/ARM/MIPS) the gate removes
//! 150 entries on 19 images and adds none. Every one of the 150 is `size: 0` and
//! outside every executable section; two are at VMA 0 and the rest are wild
//! addresses far above the image. Emitted C over the 18 of those 19 that finish a
//! whole-binary decompile — 6,085 functions — changes in exactly one function,
//! and there it improves: `sub_440b5c` in a PE that carried six phantoms had two
//! parameters typed `code *` (a phantom function lived at the address they
//! pointed to), and they come back as the `uint4 *` / `uint1 *` they are, taking
//! `*(uint4 *)((int4)a0 + 0x20)` back to `a0[8]`.
//!
//! Default-**on** (`--option unmappedentry off` restores the previous, phantom-
//! producing set exactly). It only ever removes entries the walk itself refused
//! to decode, so it can never remove a function that had a body.

use kuna_decomp::architecture::Architecture;

use super::walk::in_exec;

/// Should a direct CALL to `target` create a function entry?
///
/// The predicate is deliberately the SAME one the instruction worklist applies
/// before it decodes ([`in_exec`]): the walk may only claim a function where it
/// is willing to disassemble. With the gate off this is unconditionally `true`,
/// which is the pre-fix behaviour byte for byte.
pub(super) fn admits_call_entry(
    arch: &Architecture,
    exec_ranges: &[(u64, u64)],
    target: u64,
) -> bool {
    !arch.analysis_unmappedentry || in_exec(exec_ranges, target)
}
