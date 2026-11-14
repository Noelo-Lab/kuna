//! (kuna `mzdgroup`) The data segment an MS-DOS program's startup code loads
//! into DS.
//!
//! DOS starts a program with DS holding its PSP. The startup code of a compiled
//! program loads the segment its near data pointers address (the linker's
//! DGROUP) into DS before it calls into the program, and the program keeps it
//! there: a function that changes DS restores it. [`startup_ds`] reads that
//! segment off the entry code.
//!
//! The walk decodes forward from the entry `CS:IP` and tracks the 16-bit
//! registers that hold a segment of the module: an immediate whose word the
//! relocation table names (`mov dx, DGROUP`), CS at the entry, and a relocated
//! word read through `CS:` (`mov ds, cs:[6]`). Register moves, `mov sreg, reg`,
//! `mov reg, sreg` and a `push` the next instruction pops carry them. Any other
//! write drops a register (CS only changes by a far transfer, which ends the
//! walk, though a `CS:` override's p-code restates it), and so does an
//! interrupt for each register its service can return in: INT 21h by the
//! function in AH, any other interrupt every register.
//!
//! The walk decodes straight through. After an instruction that does not fall
//! through it resumes at the next forward branch target, and at a target a
//! register keeps its segment only when every path into the target brings the
//! same one. A call or DOS exit that a pending forward branch can skip ends its
//! path the same way, and DS there is a vote. DGROUP is the segment DS holds at
//! the first call or DOS exit no pending branch can skip, when every vote
//! agrees with it (MSC's DOS 1 error exit, which runs before DS is set, votes
//! for nothing); a walk that ends before such a call, at a return, an indirect
//! or far jump or [`MAX_INSNS`] instructions, takes the segment every vote
//! names (Lattice C's startup calls into the program with an error branch
//! still pending). An undecodable byte or a branch into an instruction states
//! none, and DGROUP must start inside the module.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::listing::classify::classify;
use crate::listing::model::RawOp;
use crate::loadimage_mz::MzImage;

/// The instructions the walk decodes before it gives up.
pub const MAX_INSNS: usize = 512;

const AX: usize = 0;
const CX: usize = 1;
const DX: usize = 2;
const BX: usize = 3;
const BP: usize = 5;
const SI: usize = 6;
const DI: usize = 7;
const ES: usize = 8;
const CS: usize = 9;
const DS: usize = 11;
const REGS: usize = 12;

/// The tracked registers: the 16-bit general registers in ModR/M order, then
/// the segment registers in `sreg` order.
const NAMES: [&[u8]; REGS] = [b"AX", b"CX", b"DX", b"BX", b"SP", b"BP", b"SI", b"DI", b"ES", b"CS", b"SS", b"DS"];

const fn bits(regs: &[usize]) -> u16 {
    let mut mask = 0u16;
    let mut i = 0;
    while i < regs.len() {
        mask |= 1 << regs[i];
        i += 1;
    }
    mask
}

const SERVICE_ANY: u16 = bits(&[AX, BX, CX, DX, SI, DI, BP, ES]);
const INTERRUPT_ANY: u16 = SERVICE_ANY | bits(&[DS]);

/// One decoded instruction as the walk reads it.
pub struct Step {
    /// Byte length.
    pub len: u32,
    /// Bit `r` is set when the instruction writes register `r` or part of it.
    pub writes: u16,
    /// A `CALL`, direct or indirect.
    pub call: bool,
    /// Whether the next instruction can run after this one.
    pub falls_through: bool,
    /// The direct branch targets.
    pub targets: Vec<u64>,
}

/// The registers an `INT n` writes, with `ah` the function number when the
/// walk knows it, or `None` when the interrupt ends the program.
fn interrupt_writes(n: u8, ah: Option<u8>) -> Option<u16> {
    match (n, ah) {
        (0x20 | 0x27, _) | (0x21, Some(0x00 | 0x31 | 0x4c)) => None,
        (0x21, Some(0x1a | 0x25)) => Some(0),
        (0x21, Some(0x02 | 0x06 | 0x09)) => Some(bits(&[AX])),
        (0x21, Some(0x30)) => Some(bits(&[AX, BX, CX])),
        (0x21, Some(0x2f | 0x35)) => Some(bits(&[BX, ES])),
        (0x21, Some(0x44)) => Some(bits(&[AX, DX])),
        (0x21, Some(0x48 | 0x4a)) => Some(bits(&[AX, BX])),
        (0x21, Some(0x51 | 0x62)) => Some(bits(&[BX])),
        (0x21, _) => Some(SERVICE_ANY),
        _ => Some(INTERRUPT_ANY),
    }
}

/// The bytes of an instruction past its prefixes, and whether a `CS:` override
/// or an operand- or address-size prefix is among them.
fn strip_prefixes(bytes: &[u8]) -> (&[u8], bool, bool) {
    let mut cs = false;
    let mut wide = false;
    let mut at = 0;
    while let Some(&byte) = bytes.get(at) {
        match byte {
            0x2e => cs = true,
            0x66 | 0x67 => wide = true,
            0x26 | 0x36 | 0x3e | 0x64 | 0x65 | 0xf0 | 0xf2 | 0xf3 => {}
            _ => break,
        }
        at += 1;
    }
    (&bytes[at..], cs, wide)
}

/// What the walk knows at one point: each register's segment, and AH.
#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    vals: [Option<u16>; REGS],
    ah: Option<u8>,
}

impl State {
    fn kill(&mut self, mask: u16) {
        for (reg, val) in self.vals.iter_mut().enumerate() {
            if mask & (1 << reg) != 0 {
                *val = None;
            }
        }
        if mask & (1 << AX) != 0 {
            self.ah = None;
        }
    }

    /// The facts both `self` and `other` hold.
    fn meet(mut self, other: &State) -> State {
        for (val, theirs) in self.vals.iter_mut().zip(other.vals) {
            if *val != theirs {
                *val = None;
            }
        }
        if self.ah != other.ah {
            self.ah = None;
        }
        self
    }

    /// Record the value the instruction `op` gives a register, returning the
    /// register it assigned.
    fn assign(
        &mut self,
        op: &[u8],
        cs_override: bool,
        imm_at: u64,
        image: &MzImage,
        popped: Option<Option<u16>>,
    ) -> Option<usize> {
        let modrm = op.get(1).copied().unwrap_or(0);
        let direct = modrm >> 6 == 3;
        let reg = usize::from((modrm >> 3) & 7);
        let rm = usize::from(modrm & 7);
        let (dst, val) = match op {
            [opc @ 0xb8..=0xbf, lo, hi, ..] => {
                let dst = usize::from(opc - 0xb8);
                if dst == AX {
                    self.ah = Some(*hi);
                }
                let relocated = image.relocated_words.binary_search(&imm_at).is_ok();
                (dst, relocated.then(|| u16::from_le_bytes([*lo, *hi])))
            }
            [0xb4, imm, ..] => {
                self.ah = Some(*imm);
                return None;
            }
            [0x8e, ..] if matches!(reg, 0 | 2 | 3) && direct => (ES + reg, self.vals[rm]),
            [0x8e, rest @ ..] if matches!(reg, 0 | 2 | 3) && modrm & 0xc7 == 0x06 && cs_override => {
                let (Some(&lo), Some(&hi), Some(cs)) = (rest.get(1), rest.get(2), self.vals[CS]) else {
                    return None;
                };
                let word = u64::from(cs) * 16 + u64::from(u16::from_le_bytes([lo, hi]));
                let relocated = image.relocated_words.binary_search(&word).is_ok();
                (ES + reg, relocated.then(|| image_word(image, word)).flatten())
            }
            [0x8c, ..] if direct && reg < 4 => (rm, self.vals[ES + reg]),
            [0x8b, ..] if direct => (reg, self.vals[rm]),
            [0x89, ..] if direct => (rm, self.vals[reg]),
            [opc @ (0x58..=0x5f | 0x07 | 0x17 | 0x1f), ..] => {
                let dst = match opc {
                    0x58..=0x5f => usize::from(opc - 0x58),
                    _ => ES + usize::from(opc >> 3),
                };
                (dst, popped?)
            }
            _ => return None,
        };
        self.vals[dst] = val;
        Some(dst)
    }

    /// The value a `push` instruction `op` puts on the stack.
    fn pushed(&self, op: &[u8]) -> Option<Option<u16>> {
        let src = match op.first()? {
            opc @ 0x50..=0x57 => usize::from(opc - 0x50),
            opc @ (0x06 | 0x0e | 0x16 | 0x1e) => ES + usize::from(opc >> 3),
            _ => return None,
        };
        Some(self.vals[src])
    }
}

fn image_word(image: &MzImage, linear: u64) -> Option<u16> {
    let at = usize::try_from(linear.checked_sub(image.base)?).ok()?;
    let bytes = image.module.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// The meet of `state` (the fall-through's, when it reaches `at`) and every
/// state a pending forward branch carries to `at`, which leaves `pending`.
fn arrive(state: Option<State>, pending: &mut Vec<(u64, State)>, at: u64) -> Option<State> {
    let mut joined = state;
    pending.retain(|(target, carried)| {
        if *target != at {
            return true;
        }
        joined = Some(joined.map_or(*carried, |state| state.meet(carried)));
        false
    });
    joined
}

/// The segment DS holds when the startup code that starts at `image.entry`
/// with code segment `entry_cs` makes the first call every path reaches,
/// decoding each instruction with `decode`.
///
/// A backward branch is not followed; the registers assigned between its
/// target and itself are dropped, since another trip through that code can
/// assign them again from values the trip changed. A branch back to code
/// before the entry leaves the walk, as a call or an exit would.
pub fn walk(image: &MzImage, entry_cs: u16, mut decode: impl FnMut(u64) -> Option<Step>) -> Option<u16> {
    let mut start = State { vals: [None; REGS], ah: None };
    start.vals[CS] = Some(entry_cs);
    let mut reaching = Some(start);
    let mut pending: Vec<(u64, State)> = Vec::new();
    let mut assigned: Vec<(u64, usize)> = Vec::new();
    let mut popped: Option<Option<u16>> = None;
    let mut votes = Votes::default();
    let mut vma = image.entry;
    for _ in 0..MAX_INSNS {
        if pending.iter().any(|&(target, _)| target == vma) {
            popped = None;
        }
        let mut state = arrive(reaching, &mut pending, vma)?;
        let step = decode(vma)?;
        let end = vma + u64::from(step.len);
        if step.len == 0 || pending.iter().any(|&(target, _)| target > vma && target < end) {
            return None;
        }
        let at = usize::try_from(vma.checked_sub(image.base)?).ok()?;
        let bytes = image.module.get(at..at.checked_add(step.len as usize)?)?;
        let (op, cs_override, wide) = strip_prefixes(bytes);
        let imm_at = vma + (bytes.len() - op.len()) as u64 + 1;
        let push = state.pushed(op);
        let mut falls_through = step.falls_through;
        let mut exits = false;
        match op {
            [0xcd, n, ..] => match interrupt_writes(*n, state.ah) {
                Some(writes) => {
                    state.kill(writes);
                    falls_through = true;
                }
                None => {
                    exits = true;
                    falls_through = false;
                }
            },
            [0xcc | 0xce, ..] => {
                state.kill(INTERRUPT_ANY);
                falls_through = true;
            }
            _ if step.call => {
                exits = true;
                falls_through = false;
            }
            _ => {
                state.kill(step.writes & !(1 << CS));
                if !wide {
                    if let Some(reg) = state.assign(op, cs_override, imm_at, image, popped) {
                        assigned.push((vma, reg));
                    }
                }
                for &target in step.targets.iter().filter(|&&target| target <= vma && target >= image.entry) {
                    for &(_, reg) in assigned.iter().filter(|&&(site, _)| site >= target) {
                        state.vals[reg] = None;
                    }
                }
                pending.extend(step.targets.iter().filter(|&&target| target > vma).map(|&target| (target, state)));
            }
        }
        popped = push;
        if falls_through {
            reaching = Some(state);
            vma = end;
            continue;
        }
        reaching = None;
        match (pending.iter().map(|&(target, _)| target).min(), exits) {
            (Some(next), _) => {
                if exits {
                    votes.add(state.vals[DS]);
                }
                vma = next;
            }
            (None, true) => return votes.verdict(state.vals[DS]),
            (None, false) => return votes.unanimous(),
        }
    }
    votes.unanimous()
}

/// The segments DS holds at the calls and exits the walk leaves because a
/// pending forward branch can skip them.
#[derive(Default)]
struct Votes {
    seen: Option<u16>,
    split: bool,
}

impl Votes {
    fn add(&mut self, ds: Option<u16>) {
        let Some(ds) = ds else { return };
        match self.seen {
            None => self.seen = Some(ds),
            Some(seen) if seen != ds => self.split = true,
            Some(_) => {}
        }
    }

    /// The segment every vote names, when they agree.
    fn unanimous(&self) -> Option<u16> {
        self.seen.filter(|_| !self.split)
    }

    /// `ds`, the segment at a call or exit every path reaches, when every vote
    /// agrees with it.
    fn verdict(&self, ds: Option<u16>) -> Option<u16> {
        let ds = ds?;
        (!self.split && self.seen.is_none_or(|seen| seen == ds)).then_some(ds)
    }
}

#[derive(Default)]
struct Capture {
    ops: Vec<(OpCode, Option<VarnodeData>, Option<VarnodeData>)>,
}

impl PcodeEmit for Capture {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.cloned(), vars.first().cloned()));
    }
}

fn overlaps(a: &VarnodeData, b: &VarnodeData) -> bool {
    let same_space = match (&a.space, &b.space) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        _ => false,
    };
    same_space && a.offset < b.offset + u64::from(b.size) && b.offset < a.offset + u64::from(a.size)
}

fn sleigh_step(translate: &dyn Translate, code_space: &Rc<AddrSpace>, regs: &[VarnodeData], vma: u64) -> Option<Step> {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut cap = Capture::default();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| translate.one_instruction(&mut cap, &addr)));
    let len = match decoded {
        Ok(Ok(len)) if len > 0 => len as u32,
        _ => return None,
    };
    let raw: Vec<RawOp> = cap.ops.iter().map(|(opcode, _, in0)| RawOp { opcode: *opcode, in0: in0.clone() }).collect();
    let classified = classify(&raw, vma, len);
    let mut writes = 0u16;
    for out in cap.ops.iter().filter_map(|(_, out, _)| out.as_ref()) {
        for (reg, loc) in regs.iter().enumerate() {
            if overlaps(out, loc) {
                writes |= 1 << reg;
            }
        }
    }
    let flow = &classified.flow;
    Some(Step {
        len,
        writes,
        call: flow.is_call,
        falls_through: classified.fall_through.is_some() && !flow.is_computed,
        targets: classified.flows,
    })
}

/// The segment an MS-DOS program's startup code loads into DS before its first
/// call, when the walk above can tell and that segment starts inside the load
/// module.
pub fn startup_ds(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    image: &MzImage,
) -> Option<u16> {
    let regs: Vec<VarnodeData> =
        NAMES.iter().map(|name| arch.get_register_varnode(name).ok()).collect::<Option<_>>()?;
    let entry_cs = image.code_segments.iter().find(|&&(linear, _)| linear == image.entry).map(|&(_, cs)| cs)?;
    let ds = crate::loader::kuna_realmodecs::without_commits(arch, translate, || {
        walk(image, entry_cs, |vma| sleigh_step(translate, code_space, &regs, vma))
    })?;
    let start = u64::from(ds) * 16;
    (start >= image.base && start < image.base + image.module.len() as u64).then_some(ds)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A decoder for the handful of 8086 forms the tests use: register writes
    /// by opcode, `CALL`, `RET`, `JMP`/`Jcc` rel8 and `INT`.
    fn step(image: &MzImage, vma: u64) -> Option<Step> {
        let at = (vma - image.base) as usize;
        let b = &image.module[at..];
        let gpr = |r: u8| 1u16 << r;
        let rel8 = |len: u64| (vma + len).wrapping_add(i64::from(b[1] as i8) as u64);
        let (len, writes, call, falls_through, targets) = match b[0] {
            0xb8..=0xbf => (3, gpr(b[0] - 0xb8), false, true, vec![]),
            0xb4 => (2, gpr(0), false, true, vec![]),
            0x8e => (2, 1 << (8 + ((b[1] >> 3) & 7)), false, true, vec![]),
            0x8b | 0x8c => (
                2,
                gpr((b[1] >> 3) & 7) * u16::from(b[0] == 0x8b) | gpr(b[1] & 7) * u16::from(b[0] == 0x8c),
                false,
                true,
                vec![],
            ),
            0x50..=0x57 | 0x06 | 0x0e | 0x16 | 0x1e => (1, 1 << 4, false, true, vec![]),
            0x58..=0x5f => (1, 1 << 4 | gpr(b[0] - 0x58), false, true, vec![]),
            0x07 | 0x17 | 0x1f => (1, 1 << 4 | 1 << (8 + (b[0] >> 3)), false, true, vec![]),
            0xcd => (2, 1 << 4, true, true, vec![]),
            0xe8 => (3, 1 << 4, true, true, vec![]),
            0xc3 => (1, 1 << 4, false, false, vec![]),
            0xeb => (2, 0, false, false, vec![rel8(2)]),
            0x70..=0x7f => (2, 0, false, true, vec![rel8(2)]),
            0x90 => (1, 0, false, true, vec![]),
            _ => return None,
        };
        Some(Step { len, writes, call, falls_through, targets })
    }

    fn image(code: &[u8], relocated: &[usize]) -> MzImage {
        let mut module = vec![0x90u8; 0x200];
        module[..code.len()].copy_from_slice(code);
        MzImage {
            module,
            base: 0x10000,
            entry: 0x10000,
            stack: 0,
            relocations: relocated.len(),
            relocated_words: relocated.iter().map(|&at| 0x10000 + at as u64).collect(),
            skipped_relocations: 0,
            packer: None,
            code_segments: vec![(0x10000, 0x1000)],
        }
    }

    fn ds(code: &[u8], relocated: &[usize]) -> Option<u16> {
        let image = image(code, relocated);
        walk(&image, 0x1000, |vma| step(&image, vma))
    }

    #[test]
    fn a_relocated_segment_moved_into_ds_before_the_first_call_is_dgroup() {
        let code = [0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[1]), Some(0x1010));
        assert_eq!(ds(&code, &[]), None, "an unrelocated immediate is no segment of the module");
    }

    #[test]
    fn dos_version_keeps_dx_and_the_date_overwrites_it() {
        let turbo_c = [0xba, 0x10, 0x10, 0xb4, 0x30, 0xcd, 0x21, 0x8e, 0xda, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&turbo_c, &[1]), Some(0x1010));
        let mut date = turbo_c;
        date[4] = 0x2a;
        assert_eq!(ds(&date, &[1]), None);
    }

    #[test]
    fn a_segment_set_on_one_path_only_is_dropped_at_the_join() {
        let code = [0x73, 0x02, 0x0e, 0x1f, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[]), None);
        let both = [0x0e, 0x1f, 0x73, 0x01, 0x90, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&both, &[]), Some(0x1000), "push cs; pop ds before the branch holds on both paths");
    }

    #[test]
    fn a_dead_fall_through_resumes_at_the_branch_target() {
        let code = [0xb8, 0x10, 0x10, 0x73, 0x03, 0xb4, 0x4c, 0xc3, 0x50, 0x1f, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[1]), Some(0x1010), "the branch carries AX past the fall-through's write");
        let exit = [0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xb4, 0x4c, 0xcd, 0x21];
        assert_eq!(ds(&exit, &[1]), Some(0x1010), "a DOS exit with no branch target ahead decides");
        assert_eq!(ds(&[0x0e, 0x1f, 0xc3], &[]), None, "a return decides nothing");
    }

    #[test]
    fn a_call_a_branch_can_skip_is_a_side_path() {
        let msc = [0x73, 0x04, 0x50, 0xe8, 0x00, 0x00, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&msc, &[7]), Some(0x1010), "the error call before DS is set decides nothing");
        let psp = [0x73, 0x05, 0x0e, 0x1f, 0xe8, 0x00, 0x00, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&psp, &[]), None, "a DS the skipped path loads is not the joined path's");
    }

    #[test]
    fn a_call_with_an_error_branch_pending_votes() {
        let lattice = [0xba, 0x10, 0x10, 0x72, 0x08, 0x8e, 0xda, 0xe8, 0x00, 0x00, 0x90, 0x90, 0x90, 0xeb, 0xf1];
        assert_eq!(ds(&lattice, &[1]), Some(0x1010), "the startup's only call names DGROUP");
        let split = [0x0e, 0x1f, 0x73, 0x03, 0xe8, 0x00, 0x00, 0xb8, 0x10, 0x10, 0x8e, 0xd8, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&split, &[8]), None, "a side path's DS that the joined path replaces is a conflict");
    }

    #[test]
    fn an_interrupt_other_than_dos_drops_ds() {
        let code = [0x0e, 0x1f, 0xcd, 0x2f, 0xe8, 0x00, 0x00];
        assert_eq!(ds(&code, &[]), None);
    }

    #[test]
    fn interrupt_writes_follow_the_dos_function() {
        assert_eq!(interrupt_writes(0x21, Some(0x4c)), None);
        assert_eq!(interrupt_writes(0x20, None), None);
        assert_eq!(interrupt_writes(0x21, Some(0x30)), Some(bits(&[AX, BX, CX])));
        assert_eq!(interrupt_writes(0x21, None), Some(SERVICE_ANY));
        assert_eq!(interrupt_writes(0x10, Some(0x30)), Some(INTERRUPT_ANY));
    }
}
