//! (kuna `mzsweep`) The raw-image call sweep over an MS-DOS executable keeps
//! only the calls a DOS program can make.
//!
//! [`super::kuna_rawdiscover`] decodes the whole mapped image straight through
//! and takes the target of every direct call it sees as a function. That suits
//! a firmware image, which is mostly code. A DOS load module is mostly data:
//! the far data segments a large-model program declares (bitmaps, fonts,
//! tables) and its DGROUP sit in the same module as its code, and their bytes
//! decode as `call`s too. Each such call mints a function that is a run of
//! data, most often zero fill decoded as `add [bx+si], al` over and over. The
//! image says enough to refuse those calls:
//!
//!  1. A far `CALL`/`JMP ptr16:16` into the load module names its segment in a
//!     word the relocation table lists, because the linker cannot know where
//!     DOS will put the module. A far branch whose segment word is not
//!     relocated names an absolute segment, so it does not land in the module.
//!  2. Zero fill is data. A call decoded in a straight-line run that also
//!     decoded zero fill (or an undecodable byte) since the last control
//!     transfer is a decode of data, not a call some instruction makes.
//!  3. A function opens with code. A target whose straight-line run up to its
//!     first control transfer decodes zero fill or an undecodable byte, or ends
//!     in a far branch rule 1 refuses, is not one. The far-branch arm needs a
//!     relocation table: a module whose header lists no relocation (EXEPACK's,
//!     or one an unpacker stripped) applies its own fixups, so every far call
//!     its code makes is unrelocated and says nothing about whether the bytes
//!     before it are code.
//!
//! The rules act on the sweep only. The recursive descent still walks every
//! target a kept call reaches, so a function a refused call names is found
//! again when code the descent reaches calls it.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;
use crate::listing::FlowType;

/// The instructions [`opens_as_code`] decodes before it accepts a target
/// whose run has not reached a control transfer.
const OPENING_INSNS: usize = 32;

/// What an MS-DOS executable states about its load module.
pub struct MzEvidence {
    base: u64,
    module: Vec<u8>,
    relocated_words: Vec<u64>,
}

impl MzEvidence {
    /// The relocated `module` at linear `base`, and the linear address of every
    /// word its relocation table adjusted.
    pub fn new(base: u64, module: Vec<u8>, mut relocated_words: Vec<u64>) -> MzEvidence {
        relocated_words.sort_unstable();
        relocated_words.dedup();
        MzEvidence { base, module, relocated_words }
    }

    fn bytes(&self, vma: u64, len: u32) -> Option<&[u8]> {
        let start = usize::try_from(vma.checked_sub(self.base)?).ok()?;
        self.module.get(start..start.checked_add(len as usize)?)
    }

    /// Whether the instruction of `len` bytes at `vma` is all zero bytes.
    pub(super) fn is_zero_fill(&self, vma: u64, len: u32) -> bool {
        self.bytes(vma, len).is_some_and(|bytes| bytes.iter().all(|&byte| byte == 0))
    }

    /// The instruction's bytes past its prefixes.
    fn opcode(&self, vma: u64, len: u32) -> &[u8] {
        let bytes = self.bytes(vma, len).unwrap_or_default();
        let prefixes = bytes
            .iter()
            .take_while(|byte| matches!(byte, 0x26 | 0x2e | 0x36 | 0x3e | 0x64 | 0x65 | 0x66 | 0x67 | 0xf0 | 0xf2 | 0xf3))
            .count();
        &bytes[prefixes..]
    }

    /// Whether the instruction of `len` bytes at `vma` is `HLT`. SLEIGH lifts
    /// it as a branch to itself, but it resumes at the next instruction once an
    /// interrupt arrives, so it does not end a straight-line run.
    fn is_halt(&self, vma: u64, len: u32) -> bool {
        self.opcode(vma, len) == [0xf4]
    }

    /// Whether the instruction of `len` bytes at `vma` is a far `CALL`/`JMP
    /// ptr16:16` whose segment word no relocation names.
    pub(super) fn is_unrelocated_far_branch(&self, vma: u64, len: u32) -> bool {
        let opcode = self.opcode(vma, len);
        opcode.len() == 5
            && matches!(opcode[0], 0x9a | 0xea)
            && self.relocated_words.binary_search(&(vma + u64::from(len) - 2)).is_err()
    }

    /// Whether the far branch of `len` bytes at `vma` that ends a target's
    /// opening run refuses the target: its segment word is unrelocated in a
    /// module whose relocation table names any word.
    fn refuses_opening(&self, vma: u64, len: u32) -> bool {
        !self.relocated_words.is_empty() && self.is_unrelocated_far_branch(vma, len)
    }

    /// Whether `flow`, the flow of the instruction of `len` bytes at `vma`,
    /// ends a straight-line run.
    fn ends_run(&self, vma: u64, len: u32, flow: &FlowType) -> bool {
        (flow.is_call || flow.is_jump || flow.is_terminal) && !self.is_halt(vma, len)
    }
}

/// The straight-line state of the sweep's decode stream since its last control
/// transfer.
#[derive(Default)]
pub(super) struct SweepRun {
    dirty: bool,
}

impl SweepRun {
    /// Note a byte the sweep could not decode.
    pub(super) fn undecodable(&mut self) {
        self.dirty = true;
    }

    /// Note the instruction of `len` bytes at `vma` whose flow is `flow`;
    /// returns whether the call it makes counts.
    pub(super) fn step(&mut self, mz: &MzEvidence, vma: u64, len: u32, flow: &FlowType) -> bool {
        if mz.is_zero_fill(vma, len) {
            self.dirty = true;
        }
        let counts = flow.is_call && !self.dirty && !mz.is_unrelocated_far_branch(vma, len);
        if mz.ends_run(vma, len, flow) {
            self.dirty = false;
        }
        counts
    }
}

/// Whether the straight-line run from `target` up to its first control
/// transfer decodes as code: no zero fill, no undecodable byte, no step out of
/// `exec_ranges`, and no far branch [`MzEvidence::refuses_opening`] refuses.
pub(super) fn opens_as_code(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    mz: &MzEvidence,
    target: u64,
) -> bool {
    let mut vma = target;
    for _ in 0..OPENING_INSNS {
        if !exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi) {
            return false;
        }
        let Ok(decoded) = decode_one(translate, vma, code_space, false, false) else {
            return false;
        };
        if decoded.len == 0 || mz.is_zero_fill(vma, decoded.len) {
            return false;
        }
        if mz.ends_run(vma, decoded.len, &classify(&decoded.ops, vma, decoded.len).flow) {
            return !mz.refuses_opening(vma, decoded.len);
        }
        vma = vma.saturating_add(u64::from(decoded.len));
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence() -> MzEvidence {
        let mut module = vec![0x90u8; 0x40];
        module[0x00..0x05].copy_from_slice(&[0x9a, 0x10, 0x00, 0x00, 0x10]);
        module[0x05..0x0a].copy_from_slice(&[0x9a, 0x10, 0x00, 0x00, 0x10]);
        module[0x0a..0x0c].copy_from_slice(&[0x00, 0x00]);
        module[0x0c..0x12].copy_from_slice(&[0x2e, 0xea, 0x00, 0x00, 0x00, 0x10]);
        module[0x12..0x15].copy_from_slice(&[0xf4, 0xb0, 0xf4]);
        MzEvidence::new(0x10000, module, vec![0x10003])
    }

    fn call() -> FlowType {
        FlowType { is_call: true, ..FlowType::default() }
    }

    #[test]
    fn a_far_branch_counts_only_through_a_relocated_segment_word() {
        let mz = evidence();
        assert!(!mz.is_unrelocated_far_branch(0x10000, 5));
        assert!(mz.is_unrelocated_far_branch(0x10005, 5));
        assert!(mz.is_unrelocated_far_branch(0x1000c, 6), "a prefixed far jump");
        assert!(!mz.is_unrelocated_far_branch(0x10012, 1), "not a far branch");
    }

    #[test]
    fn zero_fill_dirties_the_run_until_the_next_transfer() {
        let mz = evidence();
        let mut run = SweepRun::default();
        assert!(run.step(&mz, 0x10000, 5, &call()));
        assert!(!run.step(&mz, 0x10005, 5, &call()), "an unrelocated far call does not count");
        assert!(!run.step(&mz, 0x1000a, 2, &FlowType::default()));
        assert!(!run.step(&mz, 0x10000, 5, &call()), "a call after zero fill does not count");
        assert!(run.step(&mz, 0x10000, 5, &call()), "the transfer ended the dirty run");
        run.undecodable();
        assert!(!run.step(&mz, 0x10000, 5, &call()));
        assert!(mz.is_zero_fill(0x1000a, 2) && !mz.is_zero_fill(0x10012, 1) && !mz.is_zero_fill(0x10040, 2));
    }

    #[test]
    fn an_opening_far_branch_refuses_a_target_only_where_relocations_exist() {
        let mz = evidence();
        assert!(!mz.refuses_opening(0x10000, 5), "a relocated far call");
        assert!(mz.refuses_opening(0x10005, 5));
        let unrelocated = MzEvidence::new(0x10000, mz.module.clone(), Vec::new());
        assert!(unrelocated.is_unrelocated_far_branch(0x10000, 5));
        assert!(!unrelocated.refuses_opening(0x10000, 5), "a module with no relocation table fixes its own far calls");
    }

    #[test]
    fn halt_does_not_end_a_run() {
        let mz = evidence();
        let spin = FlowType { is_jump: true, ..FlowType::default() };
        assert!(!mz.ends_run(0x10012, 1, &spin), "HLT resumes after an interrupt");
        assert!(mz.ends_run(0x10013, 2, &spin), "an instruction whose last byte is f4 is not HLT");
        let mut run = SweepRun::default();
        run.undecodable();
        run.step(&mz, 0x10012, 1, &spin);
        assert!(!run.step(&mz, 0x10000, 5, &call()));
    }
}
