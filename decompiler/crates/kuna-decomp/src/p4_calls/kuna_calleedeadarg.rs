//! (kuna) `calleedeadarg` — drop a recovered call argument the callee's own body
//! proves it never reads.
//!
//! # The symptom
//!
//! One Mach-O arm64 crackme, decompiled by kuna, contradicts itself inside a
//! single file:
//!
//! ```text
//! int _secret_function(void);          // the header kuna itself emits
//! ...
//!   v3 = scanf("%d",&v2);
//!   if (v2 == 0x539)
//!     _secret_function(v3);            // called with an argument anyway
//! ```
//!
//! That is not C: the declaration and the call cannot both be right, so the
//! export does not recompile and an agent reading the output cannot tell whether
//! the callee consumes the passed value. The disassembly settles it — the callee
//! is `stp x29,x30,[sp,#-0x10]!; mov x29,sp; adrp x0,…; add x0,x0,#0xeec; bl
//! printf; …`, which OVERWRITES `x0` before it ever reads it.
//!
//! # Why the call site invents the argument
//!
//! `ActionActiveParam` recovers an unlocked callee's argument list from the
//! CALLER's data flow alone (`FuncCallSpecs::checkInputTrialUse`, `fspec.cc:5592`):
//! a trial is *active* when the ABI's argument register holds a value the caller
//! wrote and does not otherwise use. Here `w0` holds `scanf`'s return value,
//! nothing else consumes it, and `AncestorRealistic` returns `pop_solid` for a
//! Varnode defined by a CALL — so the trial is admitted. Every ingredient of that
//! decision is on the caller's side of the call, and on the caller's side the
//! evidence really is ambiguous: a live argument register at a call to an
//! unprototyped callee is exactly what a real argument looks like. The
//! `entrymainproto` record already names this as kuna's standing behaviour
//! ("at any unprototyped callee reached with a live argument register").
//!
//! `calleearity`/`calleearityfwd` cannot help: they reconcile a call against a
//! SIBLING call to the same callee in the same function, they are only ever
//! additive, and here there is exactly one call site.
//!
//! # The evidence this pass adds
//!
//! The callee's own body, read directly. [`probe_callee_entry_dead`] decodes the
//! callee starting at its entry and answers one question per register range:
//! *does every path from the entry WRITE these bytes before reading them?* If it
//! does, the register is dead on entry and cannot be carrying a parameter.
//!
//! That evidence alone is not enough to act on, and the second half is what keeps
//! this pass off real argument lists. The veto also requires the value in the
//! register to be an earlier call's **leftover result** — not something the
//! caller loaded, computed, or received as its own parameter
//! ([`is_leftover_call_result`]). Two reasons. It is the shape the symptom is
//! actually made of, since only a value the caller never placed can be an
//! argument by coincidence. And a caller that DID place a value there is passing
//! an argument the callee happens to ignore (`int dm_init(bool of_live)` built
//! with `of_live` compiled out is a real one): dropping it would be a judgement
//! about the source, and — worse — dropping a LEADING one punches a hole in the
//! register argument list that `ParamListStandard::fillinMap`'s positional rules
//! read as the end of the list, taking every later argument with it. Measured on
//! u-boot ARM, requiring only the callee evidence emptied the argument lists of
//! `do_bootm(ctx->cmdtp,0,v2,bootm_argv)` and
//! `ubifs_scan_a_node(a0,v9,v11,a1,v1,1)` exactly that way.
//!
//! With both conditions met the trial is scored `no-use` like any other
//! definitely-unused trial — the CALL input is freed and the argument
//! disappears.
//!
//! This is the same shape of evidence as the callee-body probe `rustabi` takes
//! for the call OUTPUT seam ([`crate::kuna_rustabi::probe_callee_return_writes`]),
//! and it is taken the same way: from the driver, right after the flow build,
//! because the per-function `ArchContext` the pipeline runs against carries the
//! load image but no translator. Results are cached per callee entry on the
//! `Architecture`, so each distinct body is decoded once per run.
//!
//! # What the walk can prove, and what it refuses to
//!
//! The claim is one-sided: `dead` means *no counter-example was found on a walk
//! that covered every path*, and everything the walk cannot see makes it decline.
//!
//! * Each path carries the set of register bytes already written on it. A read of
//!   a byte not in that set is a **read-before-write** and vetoes those bytes for
//!   the whole callee.
//! * Every path ENDS somewhere — at a `RETURN`, at a nested
//!   `CALL`/`CALLIND`/`CALLOTHER`, at an unresolved `BRANCHIND`, at a `LOAD`/
//!   `STORE` naming the register space (an indexed register file), or at an
//!   undecodable instruction — and the register must already be WRITTEN when it
//!   does. Past that point the walk is not reading the code that runs, so only
//!   bytes already written stay provable. That is what lets a body which
//!   overwrites `x0` and then calls `printf` still prove `x0` dead, while a body
//!   whose first act is a call proves nothing.
//! * Requiring the write is not the same as requiring the absence of a read, and
//!   the difference is load-bearing. A callee whose entire body is `ret` reads
//!   nothing at all, so a "never read" rule would declare EVERY register dead
//!   there and delete the arguments of every stub, thunk and placeholder in the
//!   image — which is exactly what the `stackreturn` datatest (three callees
//!   that are one `c3` byte each) catches. The claim this pass makes is the
//!   positive one: the callee demonstrably CLOBBERS the register, so the value
//!   the caller left there cannot be reaching it.
//! * An instruction whose p-code contains an internal (constant-space) branch is
//!   scored against the set it was ENTERED with and credits none of its writes:
//!   a conditionally-executed write must not hide a later read.
//! * A walk that records NO terminator at all proves nothing either, and this
//!   one is easy to read the wrong way round: the "written before every
//!   terminator" test is a conjunction, so over an empty list it holds for every
//!   register at once. Every path closing back onto an already-visited address
//!   is what a body that is one endless loop does — and what a PE import's IAT
//!   slot does when its pointer bytes are decoded as instructions, which is
//!   where the argument lists of `CloseHandle` and `Process32NextW` went.
//! * The instruction budget, a too-large written set or a too-large cut list
//!   abandon the whole summary, which then proves nothing at all.
//!
//! Only the `register` space is answered. A `ram`-space (global) trial would need
//! the walk to model memory, which it does not, so those keep today's answer.
//!
//! Default-**on**: this only ever REMOVES an argument, and only against a decoded
//! body that contradicts it. Flip `off` to restore the pre-option rendering.

use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleedeadarg>` (kuna 4000+ range; 4139 was the previous
/// high-water mark).
pub const ELEM_CALLEEDEADARG: ElementId = ElementId::new("calleedeadarg", 4140);

/// (kuna) Drop a provably-unread call argument: `calleedeadarg on|off`.
pub struct OptionCalleeDeadArg;

impl OptionCalleeDeadArg {
    /// The option name.
    pub const NAME: &'static str = "calleedeadarg";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_dead_arg`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-body dead-argument veto turned {prop}")))
    }
}

/// How many machine instructions one callee probe decodes before abandoning the
/// summary. Every path normally ends at the callee's first call or return, so
/// this bound is reached only by a branchy argument-free prologue — a body whose
/// answer would be "may read" almost surely.
const MAX_PROBE_INSTRUCTIONS: u32 = 192;

/// How many distinct register bytes one path may accumulate before the summary
/// declares itself incomplete.
const MAX_WRITTEN_BYTES: usize = 4096;

/// How many cuts one summary keeps before declaring itself incomplete.
const MAX_CUTS: usize = 64;

/// A set of individual register-space bytes, keyed by `(space index, offset)`.
type ByteSet = BTreeSet<(int4, u64)>;

/// What a bounded decode of a callee body proves about its entry liveness.
///
/// Consumed through [`Self::proves_dead`], which answers `false` for everything
/// the walk could not cover, so an incomplete summary never vetoes an argument.
#[derive(Clone, Debug, Default)]
pub struct CalleeEntryDead {
    /// The `register` space's manager index — the only space this summary
    /// answers for. `-1` when the image has no register space.
    reg_idx: int4,
    /// Register byte ranges some path READS before writing — `(space, offset, size)`.
    reads: Vec<(int4, u64, int4)>,
    /// The same, minus the reads whose value provably cannot matter: `xor
    /// ecx,ecx`, `and edx,0` and `or rdx,-1` all write a CONSTANT, but their
    /// p-code says `ECX = ECX ^ ECX`, so a walk that counts every read calls a
    /// scratch register an input.  `reads` keeps them, because dropping a read
    /// can only make [`Self::proves_dead`] answer `true` more often and that
    /// direction deletes arguments; [`Self::proves_input`] reads this one
    /// instead.
    reads_live: Vec<(int4, u64, int4)>,
    /// The individual register bytes the reads in [`Self::reads_live`] found
    /// unwritten. A read is recorded whole when ANY byte of it is unwritten, so
    /// `mov 0x8(%rbx),%cl; lea -0x4(%rcx),%edx` records all of `rcx` although
    /// only the bytes above `cl` were never written. Two more reads are left out
    /// because they consume no value: a register compared with itself
    /// (`sbb %ecx,%ecx` sets its flags from `ecx < ecx`), and a register stored
    /// by an instruction that stores several (`push {r0, r1, r4, lr}`, which gcc
    /// emits to reserve stack with dead registers). Read by
    /// [`Self::reads_low_byte`].
    live_read_bytes: ByteSet,
    /// The register bytes left out of [`Self::live_read_bytes`] because the
    /// read only stored them with a single store. A variadic callee whose
    /// register save gcc bounds to the `va_arg`s it can reach stores just those
    /// (`str x2, [sp, #24]`), and only where a caller marks its variadic calls
    /// (SysV's `xor %eax,%eax`) is a store alone evidence. Read by
    /// [`Self::reads_low_byte`].
    stored_read_bytes: ByteSet,
    /// Did some path reach an instruction the walk could not decode? A body
    /// decoded in the wrong instruction set reads registers at random before it
    /// breaks. Read by [`Self::reads_low_byte`].
    undecodable: bool,
    /// For every path terminator — a `RETURN` as much as a nested call or an
    /// unresolved branch — the register bytes already written on the way to it.
    /// A range is only dead if it is fully written before EVERY one of them, and
    /// an EMPTY list is no evidence at all rather than every range dead (see
    /// [`Self::proves_dead`]).
    cuts: Vec<ByteSet>,
    /// The subset of [`Self::cuts`] taken where control left for code the walk
    /// could not NAME: an indirect call or tail call, a `CALLOTHER`, an indexed
    /// register-file access, an undecodable instruction. Nothing a caller left
    /// in a register survives one of these provably. Read by
    /// [`resolve_forward_transfer`].
    opaque_cuts: Vec<ByteSet>,
    /// The subset of [`Self::cuts`] taken at a DIRECT call, with the target's
    /// entry address. Naming the target is not the same as accounting for what
    /// is forwarded to it — the callee's own recovery only accounted for a
    /// target whose prototype it knew — so each of these is resolved against
    /// that target by [`resolve_forward_transfer`], recursively.
    named_cuts: Vec<(Address, ByteSet)>,
    /// How many of [`Self::opaque_cuts`] are a user op inside an instruction
    /// that goes on to return unconditionally, such as ARM's `bx lr` switching
    /// the instruction set on its way back. Read by [`Self::returns_untouched`].
    returning_opaque: usize,
    /// The register reads such an instruction makes after its user op, which
    /// [`Self::reads`] does not see.
    return_reads: Vec<(int4, u64, int4)>,
    /// Register bytes some instruction writes only after a branch inside it,
    /// a conditionally executed write (ARM `vmovgt`, a Thumb IT block) that
    /// the walk credits on both paths, where nothing on that path wrote them
    /// before. Read by [`Self::proves_dead_firmly`].
    cond_written: ByteSet,
    /// Did a path end inside an instruction that had already branched, such as
    /// ARM's conditional `bxlt lr`, so the path skipping it was never walked?
    lost: bool,
    /// The paths that skip a conditional return, walked on their own from the
    /// bytes written before its branch when the probe was asked to follow them.
    /// Read only by [`Self::returns_untouched`].
    skipped: Option<Box<CalleeEntryDead>>,
    /// The bytes written where a path reached a system-call user op (`svc`),
    /// whose kernel reads registers no p-code input names. Kept when the walk
    /// is abandoned, since each one is positive evidence. Read by
    /// [`Self::traps_unwritten`].
    trap_cuts: Vec<ByteSet>,
    /// Did the walk cover every path with nothing abandoned?
    complete: bool,
}

impl CalleeEntryDead {
    /// Does this summary *prove* the callee never reads any byte of
    /// `[addr, addr+size)` before writing it?
    pub fn proves_dead(&self, addr: &Address, size: int4) -> bool {
        if !self.complete || size <= 0 {
            return false;
        }
        // No path terminator was recorded, so no path was seen ENDING with the
        // register written and the conjunction over `cuts` below would hold
        // vacuously — for every register at once.  A walk reaches here when the
        // revisit rule closes every path back onto an already-visited address,
        // which is what a body that is one endless loop does, and what data
        // decoded as instructions usually does.
        if self.cuts.is_empty() {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        if self.reg_idx < 0 || sp.get_index() != self.reg_idx {
            return false;
        }
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size as u64);
        if end < off {
            return false;
        }
        if self
            .reads
            .iter()
            .any(|&(ridx, roff, rsz)| ridx == idx && roff < end && off < roff + rsz as u64)
        {
            return false;
        }
        self.cuts.iter().all(|c| (off..end).all(|b| c.contains(&(idx, b))))
    }

    /// Does this summary show some path READING any byte of `[addr, addr+size)`
    /// before writing it — the callee consuming the register as an input?
    ///
    /// The mirror of [`Self::proves_dead`] and the evidence
    /// [`crate::p4_calls::kuna_calleearitylive`] extends an argument list on.
    /// An incomplete walk clears `reads`, so it answers `false` there too.
    pub fn proves_read(&self, addr: &Address, size: int4) -> bool {
        if !self.complete || size <= 0 {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        if self.reg_idx < 0 || sp.get_index() != self.reg_idx {
            return false;
        }
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size as u64);
        if end < off {
            return false;
        }
        self.reads
            .iter()
            .any(|&(ridx, roff, rsz)| ridx == idx && roff < end && off < roff + rsz as u64)
    }

    /// Does this summary show some path reading any byte of `[addr, addr+size)`
    /// before writing it, for a value that can actually reach something?
    ///
    /// [`Self::proves_read`] with the register-zeroing idiom excluded, and the
    /// evidence [`crate::p4_calls::kuna_calleearitybody`] recovers an argument
    /// list on.  The distinction matters there and only there: that rule takes
    /// a read as proof of an INPUT with no sibling call to check it against, and
    /// an API-resolution stub whose prologue is `xor ecx,ecx; lea rdx,[..]; call`
    /// would otherwise be read as taking a first argument it discards, and a
    /// body reached through `or rdx,-1` as taking a second.
    pub fn proves_input(&self, addr: &Address, size: int4) -> bool {
        if !self.complete || size <= 0 {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        if self.reg_idx < 0 || sp.get_index() != self.reg_idx {
            return false;
        }
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size as u64);
        if end < off {
            return false;
        }
        self.reads_live
            .iter()
            .any(|&(ridx, roff, rsz)| ridx == idx && roff < end && off < roff + rsz as u64)
    }

    /// Does some path read the LOW byte of `[addr, addr+size)` before writing
    /// it, for a value that can reach something?
    ///
    /// [`Self::proves_input`] narrowed to the byte every integer or pointer
    /// argument occupies, so a wide read of a register whose low byte the
    /// callee wrote first (`mov 0x8(%rbx),%cl; lea -0x4(%rcx),%edx`) is not an
    /// input, and without the reads [`Self::live_read_bytes`] leaves out. A
    /// read that only stores the register counts with `stores`. A walk that met
    /// an undecodable instruction answers `false`. Read by
    /// [`crate::p4_calls::kuna_calleereadarg`].
    pub fn reads_low_byte(&self, addr: &Address, size: int4, stores: bool) -> bool {
        if !self.complete || self.undecodable || size <= 0 {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        if self.reg_idx < 0 || sp.get_index() != self.reg_idx {
            return false;
        }
        let low = if addr.is_big_endian() {
            addr.get_offset().wrapping_add(size as u64 - 1)
        } else {
            addr.get_offset()
        };
        self.live_read_bytes.contains(&(self.reg_idx, low))
            || (stores && self.stored_read_bytes.contains(&(self.reg_idx, low)))
    }

    /// Mark byte `b` of the register file written before every read of it, so
    /// a test can stand up a partial write followed by a wide read.
    #[cfg(test)]
    pub(crate) fn with_written_byte(mut self, b: u64) -> Self {
        self.live_read_bytes.remove(&(self.reg_idx, b));
        self
    }

    /// How many low bytes of `[addr, addr+size)` the reads [`Self::proves_input`]
    /// counts consume: the widest such read, measured from `addr`.
    ///
    /// `None` when no counted read overlaps the range, when the walk is
    /// incomplete, or when an overlapping read starts above `addr` (on a
    /// big-endian register file the low bytes are not at `addr`).
    pub fn live_input_width(&self, addr: &Address, size: int4) -> Option<int4> {
        if !self.proves_input(addr, size) {
            return None;
        }
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size as u64);
        let mut width = 0u64;
        for &(ridx, roff, rsz) in &self.reads_live {
            if ridx != idx || roff >= end || roff + rsz as u64 <= off {
                continue;
            }
            if roff != off {
                return None;
            }
            width = width.max(rsz as u64);
        }
        (width > 0).then(|| width.min(size as u64) as int4)
    }

    /// The written sets recorded where control left for a target the walk could
    /// not name, read by [`resolve_forward_transfer`].
    fn opaque_cuts(&self) -> &[ByteSet] {
        &self.opaque_cuts
    }

    /// The direct-call terminators: target entry, and the bytes written on the
    /// way to it. Read by [`resolve_forward_transfer`].
    fn named_cuts(&self) -> &[(Address, ByteSet)] {
        &self.named_cuts
    }

    /// Every register byte some path reads before writing, ignoring the
    /// register-zeroing idiom ([`Self::reads_live`]).
    fn read_bytes(&self) -> ByteSet {
        let mut out = ByteSet::new();
        for &(idx, off, sz) in &self.reads_live {
            for b in off..off + sz.max(0) as u64 {
                out.insert((idx, b));
            }
        }
        out
    }

    /// Did the walk complete? (Diagnostics and tests.)
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// [`Self::proves_dead`] without leaning on a conditionally executed write:
    /// every path writes the range by instructions that always run.
    pub fn proves_dead_firmly(&self, addr: &Address, size: int4) -> bool {
        let (idx, off) = (self.reg_idx, addr.get_offset());
        self.proves_dead(addr, size)
            && !(off..off.wrapping_add(size as u64)).any(|b| self.cond_written.contains(&(idx, b)))
    }

    /// Did this walk itself follow every path, including the one past a
    /// conditional return or call?
    pub fn walked_every_path(&self) -> bool {
        self.complete && !self.lost && self.skipped.is_none()
    }

    /// Does this body call nothing and only return, leaving
    /// `[addr, addr+size)` unread on the way? A user op that leads straight to
    /// an unconditional return hands nothing on.
    pub fn returns_untouched(&self, addr: &Address, size: int4) -> bool {
        let Some(sp) = addr.get_space() else { return false };
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size.max(0) as u64);
        size > 0
            && end > off
            && !self.cuts.is_empty()
            && sp.get_index() == idx
            && self.leaves_untouched(idx, off, end)
    }

    /// Does no path read `[addr, addr+size)` before writing it, or leave for
    /// code the walk did not read with any of those bytes still unwritten? A
    /// RETURN hands the register back unread, so it needs no write first.
    pub fn never_takes(&self, addr: &Address, size: int4) -> bool {
        let Some(sp) = addr.get_space() else { return false };
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size.max(0) as u64);
        size > 0
            && end > off
            && !self.cuts.is_empty()
            && sp.get_index() == idx
            && self.takes_none(idx, off, end)
    }

    /// Does some path reach a system call with a byte of `[addr, addr+size)`
    /// still holding the caller's value, so the kernel may read it?
    pub fn traps_unwritten(&self, addr: &Address, size: int4) -> bool {
        let Some(sp) = addr.get_space() else { return false };
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size.max(0) as u64);
        sp.get_index() == idx
            && self.trap_cuts.iter().any(|c| (off..end).any(|b| !c.contains(&(idx, b))))
    }

    /// [`Self::never_takes`] over register bytes `[off, end)`, including the
    /// paths past a conditional return handed to [`Self::skipped`].
    fn takes_none(&self, idx: int4, off: u64, end: u64) -> bool {
        let overlaps = |reads: &[(int4, u64, int4)]| {
            reads
                .iter()
                .any(|&(ridx, roff, rsz)| ridx == idx && roff < end && off < roff + rsz as u64)
        };
        let written = |c: &ByteSet| (off..end).all(|b| c.contains(&(idx, b)));
        self.complete
            && !self.lost
            && !overlaps(&self.reads)
            && !overlaps(&self.return_reads)
            && !(off..end).any(|b| self.cond_written.contains(&(idx, b)))
            && self.opaque_cuts.iter().all(written)
            && self.named_cuts.iter().all(|(_, c)| written(c))
            && self.skipped.as_deref().map_or(true, |s| s.takes_none(idx, off, end))
    }

    /// Does every path this walk covered, and every path past a conditional
    /// return it handed to [`Self::skipped`], end at a return without reading
    /// or conditionally writing register bytes `[off, end)`?
    fn leaves_untouched(&self, idx: int4, off: u64, end: u64) -> bool {
        let overlaps = |reads: &[(int4, u64, int4)]| {
            reads
                .iter()
                .any(|&(ridx, roff, rsz)| ridx == idx && roff < end && off < roff + rsz as u64)
        };
        self.complete
            && !self.lost
            && self.named_cuts.is_empty()
            && self.opaque_cuts.len() == self.returning_opaque
            && !overlaps(&self.reads)
            && !overlaps(&self.return_reads)
            && !(off..end).any(|b| self.cond_written.contains(&(idx, b)))
            && self.skipped.as_deref().map_or(true, |s| s.leaves_untouched(idx, off, end))
    }

    /// Record one unnameable-transfer terminator, so a consumer of this
    /// evidence can unit-test the forwarding case without a translator.
    #[cfg(test)]
    pub(crate) fn with_opaque_cut(mut self, written: Vec<(int4, u64)>) -> Self {
        self.opaque_cuts.push(written.into_iter().collect());
        self
    }

    /// Record one system-call terminator, for the same reason.
    #[cfg(test)]
    pub(crate) fn with_trap_cut(mut self, written: Vec<(int4, u64)>) -> Self {
        let cut: ByteSet = written.into_iter().collect();
        self.opaque_cuts.push(cut.clone());
        self.cuts.push(cut.clone());
        self.trap_cuts.push(cut);
        self
    }

    /// Record one direct-call terminator, for the same reason.
    #[cfg(test)]
    pub(crate) fn with_named_cut(mut self, target: Address, written: Vec<(int4, u64)>) -> Self {
        self.named_cuts.push((target, written.into_iter().collect()));
        self
    }

    /// Build a summary by hand, so a module that only CONSUMES this evidence can
    /// unit-test its own decisions without standing up a translator.
    #[cfg(test)]
    pub(crate) fn from_parts(
        reg_idx: int4,
        reads: Vec<(int4, u64, int4)>,
        cuts: Vec<Vec<(int4, u64)>>,
        complete: bool,
    ) -> Self {
        CalleeEntryDead {
            reg_idx,
            reads_live: reads.clone(),
            live_read_bytes: reads
                .iter()
                .flat_map(|&(idx, off, sz)| (off..off + sz.max(0) as u64).map(move |b| (idx, b)))
                .collect(),
            reads,
            cuts: cuts.into_iter().map(|c| c.into_iter().collect()).collect(),
            opaque_cuts: Vec::new(),
            named_cuts: Vec::new(),
            returning_opaque: 0,
            return_reads: Vec::new(),
            cond_written: ByteSet::new(),
            lost: false,
            skipped: None,
            trap_cuts: Vec::new(),
            stored_read_bytes: ByteSet::new(),
            undecodable: false,
            complete,
        }
    }

    /// Turn the read of byte `b` into one that only stored it.
    #[cfg(test)]
    pub(crate) fn with_stored_byte(mut self, b: u64) -> Self {
        if self.live_read_bytes.remove(&(self.reg_idx, b)) {
            self.stored_read_bytes.insert((self.reg_idx, b));
        }
        self
    }

    /// Record that some path met an undecodable instruction, so a consumer
    /// can unit-test its refusal without a translator.
    #[cfg(test)]
    pub(crate) fn with_undecodable(mut self) -> Self {
        self.undecodable = true;
        self
    }
}

/// What a caller's value in a register can reach once the callee has it.
///
/// [`CalleeEntryDead`] answers for ONE body; this answers for the bodies that
/// body goes on to call, which is what a recovered parameter list is short
/// about. It is a conjunction of byte sets: a register range is *transfer free*
/// when it is written before every terminator that leads somewhere unaccounted
/// for, and is not read anywhere the resolution walked.
#[derive(Clone, Debug)]
pub struct ForwardTransfer {
    /// The register-space manager index this answers for.
    reg_idx: int4,
    /// Written sets, one per unaccounted-for terminator in the closure: a range
    /// is transfer free only if every one of them already contains it.
    cut_sets: Vec<ByteSet>,
    /// Bytes read before being written somewhere in the closure, on a path that
    /// carried the caller's value there.
    forbidden: ByteSet,
}

impl ForwardTransfer {
    /// The answer that proves nothing: a single empty cut set leaves no range
    /// transfer free.
    fn denied(reg_idx: int4) -> Self {
        ForwardTransfer { reg_idx, cut_sets: vec![ByteSet::new()], forbidden: ByteSet::new() }
    }

    /// Can a value the caller left in `[addr, addr+size)` reach code that reads
    /// it without the callee's own recovery having accounted for the read?
    ///
    /// `true` says it cannot.  A summary that is incomplete, one that answers
    /// for another space, and one the resolution declined all answer `false`.
    pub fn transfer_free(&self, addr: &Address, size: int4) -> bool {
        if size <= 0 {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        if self.reg_idx < 0 || sp.get_index() != self.reg_idx {
            return false;
        }
        let (idx, off) = (self.reg_idx, addr.get_offset());
        let end = off.wrapping_add(size as u64);
        if end < off {
            return false;
        }
        if (off..end).any(|b| self.forbidden.contains(&(idx, b))) {
            return false;
        }
        self.cut_sets.iter().all(|c| (off..end).all(|b| c.contains(&(idx, b))))
    }
}

/// How many callee bodies one resolution walks before giving up.
const MAX_FORWARD_NODES: usize = 32;

/// Resolve what `entry`'s body can forward, following its direct calls.
///
/// [`CalleeEntryDead`] ends every path at the callee's first call, so on its own
/// it says nothing about what happens past one. Naming the target is not an
/// answer either — the callee's recovered parameter list is short exactly when
/// the recovery could not see what the target reads. So the question is asked of
/// the target, and only three things let the register through:
///
/// * the target carries a **declared, non-variadic** prototype (a library
///   signature, DWARF, a console declaration), which is authoritative about what
///   it reads and is there for a target with no body to read;
/// * the target's own body answers it, recursively — it neither reads the
///   register nor forwards it anywhere unaccounted for;
/// * the register is already written when control reaches the call, so what is
///   forwarded is the callee's own value.
///
/// Everything else is a hole: an import whose PLT stub jumps through its GOT
/// slot, an undecodable body, a recursive component, an indirect transfer, a
/// probe that ran out of budget. A `protoorder lock`-parked prototype is parked
/// with `first_var_arg_slot` set, so it reads as variadic here and is never
/// mistaken for a declaration.
pub fn resolve_forward_transfer(
    arch: &mut crate::architecture::Architecture,
    entry: &Address,
    reg_idx: int4,
) -> ForwardTransfer {
    let mut visiting: Vec<(int4, u64)> = Vec::new();
    let mut budget = MAX_FORWARD_NODES;
    resolve_node(arch, entry, reg_idx, &mut visiting, &mut budget)
}

/// What one direct-call target answers about the register it is handed.
pub enum TargetAnswer {
    /// Its prototype accounts for what it reads: a source declaration.
    Accounted,
    /// Nothing accounts for it: an import with no signature, a body this run
    /// never decompiled, a recursive component.
    Unaccounted,
    /// A prototype this run recovered, with the same question answered of it.
    Through(ForwardTransfer),
}

/// Fold one body's terminators, with each direct-call target already answered,
/// into what the body can forward.
///
/// Split out from [`resolve_node`] so the fold can be read — and tested —
/// without an `Architecture` behind it.
fn fold_node(
    dead: &CalleeEntryDead,
    reg_idx: int4,
    mut answer: impl FnMut(&Address) -> TargetAnswer,
) -> ForwardTransfer {
    if !dead.is_complete() || dead.reg_idx != reg_idx {
        return ForwardTransfer::denied(reg_idx);
    }
    let mut out = ForwardTransfer {
        reg_idx,
        cut_sets: dead.opaque_cuts().to_vec(),
        forbidden: dead.read_bytes(),
    };
    for (target, written) in dead.named_cuts() {
        match answer(target) {
            TargetAnswer::Accounted => {}
            TargetAnswer::Unaccounted => out.cut_sets.push(written.clone()),
            TargetAnswer::Through(sub) => {
                for c in &sub.cut_sets {
                    out.cut_sets.push(c.union(written).cloned().collect());
                }
                out.forbidden.extend(sub.forbidden.difference(written).cloned());
            }
        }
    }
    out
}

/// One node of [`resolve_forward_transfer`]'s walk.
fn resolve_node(
    arch: &mut crate::architecture::Architecture,
    entry: &Address,
    reg_idx: int4,
    visiting: &mut Vec<(int4, u64)>,
    budget: &mut usize,
) -> ForwardTransfer {
    let Some(sp) = entry.get_space() else { return ForwardTransfer::denied(reg_idx) };
    let key = (sp.get_index(), entry.get_offset());
    // A cycle in the closure is a recursive component: the recovery of every
    // member of it is short about the others, so nothing is proven.
    if visiting.contains(&key) || *budget == 0 {
        return ForwardTransfer::denied(reg_idx);
    }
    *budget -= 1;
    let Some(dead) = probe_cached(arch, entry, reg_idx) else {
        return ForwardTransfer::denied(reg_idx);
    };
    visiting.push(key);
    let out = fold_node(&dead, reg_idx, |target| {
        if target.get_space().is_none() {
            return TargetAnswer::Unaccounted;
        }
        if declared_accounts_for_its_reads(arch, target) {
            return TargetAnswer::Accounted;
        }
        TargetAnswer::Through(resolve_node(arch, target, reg_idx, visiting, budget))
    });
    visiting.pop();
    out
}

/// Does a source declaration state what the function at `entry` reads?
///
/// True only for a non-variadic prototype parked on its function symbol. A
/// variadic one says nothing about the registers past its named parameters, and
/// `protoorder lock` parks its recovered lists with the variadic slot set at the
/// end of the recovered list for exactly that reason — so a recovered list
/// parked there is never mistaken for a declaration here.
fn declared_accounts_for_its_reads(
    arch: &crate::architecture::Architecture,
    entry: &Address,
) -> bool {
    match arch.symboltab.function_proto_pieces_across_scopes(entry) {
        Some(pieces) => pieces.first_var_arg_slot < 0,
        None => false,
    }
}

/// How many levels of direct calls [`reads_through_calls`] follows.
const MAX_THROUGH_DEPTH: u32 = 3;

/// One register read, `(space index, offset, size)`.
type RegRead = (int4, u64, int4);

/// [`reads_through_calls`]'s memo for one caller: what each target, asked at
/// each remaining depth, is shown to take.
type ThroughMemo = HashMap<(int4, u64, u32), Rc<Vec<RegRead>>>;

/// (kuna `passthrough`) The probe of `entry`'s body, with what each of its
/// direct calls hands a target shown to take it.
///
/// A forwarder that calls its target (`call f; ret`, or a tail call the spec
/// models as a call) reads nothing itself, yet what its target takes is what it
/// was handed. A target's read counts for the register bytes nothing on the
/// forwarder's path wrote before that call ([`add_reads_through`]), and only
/// where the target's own `protoorder` statement takes that register on the
/// terms `passthrough` asks of a callee ([`taken_reads`]): a variadic's
/// register-save prologue reads every argument register, and a forwarder into
/// one would otherwise be read as taking them all. Calls are followed
/// [`MAX_THROUGH_DEPTH`] deep, so a cycle of calls ends. Only
/// [`CalleeEntryDead::reads_live`] grows, so `proves_dead` and `proves_read`
/// answer exactly as the plain probe does. Statements accumulate over a run, so
/// only the probes are cached for the run and the fold is memoized per caller.
pub fn reads_through_calls(
    arch: &mut crate::architecture::Architecture,
    entry: &Address,
    reg_idx: int4,
    memo: &mut ThroughMemo,
) -> Option<Rc<CalleeEntryDead>> {
    let dead = probe_cached(arch, entry, reg_idx)?;
    let reads = add_reads_through(&dead, |t| taken_reads(arch, t, reg_idx, MAX_THROUGH_DEPTH - 1, memo));
    Some(match reads {
        Some(reads_live) => Rc::new(CalleeEntryDead { reads_live, ..(*dead).clone() }),
        None => dead,
    })
}

/// The live reads of the function at `entry`, its own direct calls followed
/// `depth` more levels, kept where its statement takes a parameter: an
/// arity-sound list naming the register, not as a variadic tail. Empty for a
/// function that stated nothing.
fn taken_reads(
    arch: &mut crate::architecture::Architecture,
    entry: &Address,
    reg_idx: int4,
    depth: u32,
    memo: &mut ThroughMemo,
) -> Option<Rc<Vec<RegRead>>> {
    let sp = entry.get_space()?;
    let key = (sp.get_index(), entry.get_offset());
    if let Some(r) = memo.get(&(key.0, key.1, depth)) {
        return Some(Rc::clone(r));
    }
    let taken = match arch.kuna_protoorder_types.get(&key).cloned() {
        Some(stated) if stated.arity_sound => {
            let dead = probe_cached(arch, entry, reg_idx)?;
            let through = if depth == 0 {
                None
            } else {
                add_reads_through(&dead, |t| taken_reads(arch, t, reg_idx, depth - 1, memo))
            };
            let reads = through.unwrap_or_else(|| dead.reads_live.clone());
            reads.into_iter().filter(|r| takes(&stated, r)).collect()
        }
        _ => Vec::new(),
    };
    let out = Rc::new(taken);
    memo.insert((key.0, key.1, depth), Rc::clone(&out));
    Some(out)
}

/// Does `stated` take a parameter that `read` overlaps, other than a variadic tail?
fn takes(stated: &crate::kuna_protoorder::RecoveredTypes, &(ridx, off, sz): &RegRead) -> bool {
    let end = off + sz.max(0) as u64;
    stated.inputs.iter().any(|(a, size, _)| {
        a.get_space().is_some_and(|s| s.get_index() == ridx)
            && a.get_offset() < end
            && off < a.get_offset() + (*size).max(0) as u64
            && !stated.vararg_tail.contains(a)
    })
}

/// The entry-liveness probe of `entry`, from the run's cache or taken now.
pub(crate) fn probe_cached(
    arch: &mut crate::architecture::Architecture,
    entry: &Address,
    reg_idx: int4,
) -> Option<Rc<CalleeEntryDead>> {
    let sp = entry.get_space()?;
    let key = (sp.get_index(), entry.get_offset());
    if !arch.kuna_callee_dead_cache.contains_key(&key) {
        let follow = crate::kuna_armfloatargs::applies(arch);
        let traps = crate::kuna_syscallregs::userop_ids(
            &arch.userops,
            crate::kuna_syscallregs::SyscallFamily::from_archid(&arch.archid),
        );
        let probed = probe_entry_with(arch.translate(), entry, reg_idx, follow, &traps);
        arch.kuna_callee_dead_cache.insert(key, Rc::new(probed));
    }
    arch.kuna_callee_dead_cache.get(&key).cloned()
}

/// `dead`'s live reads plus what each direct call's target takes of the bytes
/// still unwritten at that call, or `None` when that adds nothing.
///
/// A read that overlaps a byte written before the call is dropped whole, and so
/// is one that overlaps a byte `dead` reads itself: that register is already
/// proven an input, and a deeper read could only change how wide it is taken
/// (openssh `channel_by_id` pushes all of `rsi` for a variadic `%d`, which would
/// make `channel_send_open(ssh, int id)`'s `id` 64 bits). An incomplete summary
/// adds nothing, and neither does an indirect transfer, which names no target.
fn add_reads_through(
    dead: &CalleeEntryDead,
    mut target: impl FnMut(&Address) -> Option<Rc<Vec<RegRead>>>,
) -> Option<Vec<RegRead>> {
    if !dead.complete || dead.named_cuts.is_empty() {
        return None;
    }
    let idx = dead.reg_idx;
    let own = dead.read_bytes();
    let mut reads = dead.reads_live.clone();
    let before = reads.len();
    for (t, written) in &dead.named_cuts {
        let Some(inner) = target(t) else { continue };
        for &(ridx, off, sz) in inner.iter() {
            let covered = |b: u64| written.contains(&(idx, b)) || own.contains(&(idx, b));
            if ridx != idx || (off..off + sz.max(0) as u64).any(covered) {
                continue;
            }
            if !reads.contains(&(ridx, off, sz)) {
                reads.push((ridx, off, sz));
            }
        }
    }
    (reads.len() > before).then_some(reads)
}

/// One decoded p-code op, kept in emission order.
struct RawOp {
    opc: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

/// Recording sink for [`probe_callee_entry_dead`]: the raw p-code of ONE machine
/// instruction, in order, plus whether that instruction branches inside itself.
#[derive(Default)]
struct EntryEmit {
    ops: Vec<RawOp>,
    internal_flow: bool,
}

impl kuna_sleigh::translate::PcodeEmit for EntryEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        if matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) {
            if let Some(sp) = vars.first().and_then(|v| v.space.as_ref()) {
                if sp.get_type() == spacetype::IPTR_CONSTANT {
                    self.internal_flow = true;
                }
            }
        }
        self.ops.push(RawOp { opc, out: outvar.cloned(), ins: vars.to_vec() });
    }
}

/// A pending path: where to resume and what is already written on the way there.
struct Frame {
    at: Address,
    written: ByteSet,
}

/// Decode a callee body and report which register bytes it is *proven* never to
/// read before writing (C++ has no analogue).
///
/// The walk starts at `entry`, follows fall-through and resolved machine branch
/// targets, and ends a path at a `RETURN`, a nested call, an unresolved indirect
/// branch, a register-file `LOAD`/`STORE`, or an undecodable instruction —
/// recording, at each of those, the bytes already written on the way there. Only
/// bytes written before EVERY such terminator stay provable. It abandons the
/// whole summary — proving nothing — on the instruction budget or a runaway
/// written/cut set.
pub fn probe_callee_entry_dead<T: kuna_sleigh::translate::Translate + ?Sized>(
    tr: &T,
    entry: &Address,
    reg_idx: int4,
) -> CalleeEntryDead {
    probe_entry(tr, entry, reg_idx, false)
}

/// [`probe_callee_entry_dead`], with `follow` also walking the paths that skip
/// a conditional return into [`CalleeEntryDead::skipped`].
pub fn probe_entry<T: kuna_sleigh::translate::Translate + ?Sized>(
    tr: &T,
    entry: &Address,
    reg_idx: int4,
    follow: bool,
) -> CalleeEntryDead {
    probe_entry_with(tr, entry, reg_idx, follow, &[])
}

/// [`probe_entry`], recording where a path reaches one of the system-call user
/// ops `traps` ([`CalleeEntryDead::traps_unwritten`]).
pub fn probe_entry_with<T: kuna_sleigh::translate::Translate + ?Sized>(
    tr: &T,
    entry: &Address,
    reg_idx: int4,
    follow: bool,
    traps: &[uint4],
) -> CalleeEntryDead {
    let mut res = CalleeEntryDead { reg_idx, complete: true, ..CalleeEntryDead::default() };
    let Some(entry_space) = entry.get_space() else {
        res.complete = false;
        return res;
    };
    if entry_space.get_type() != spacetype::IPTR_PROCESSOR || reg_idx < 0 {
        res.complete = false;
        return res;
    }
    let start = vec![Frame { at: entry.clone(), written: ByteSet::new() }];
    let mut skips = Vec::new();
    walk(tr, &mut res, start, if follow { Skips::Aside(&mut skips) } else { Skips::Dropped }, traps);
    if res.complete && !skips.is_empty() {
        let mut late = CalleeEntryDead { reg_idx, complete: true, ..CalleeEntryDead::default() };
        walk(tr, &mut late, skips, Skips::Walked, traps);
        res.skipped = Some(Box::new(late));
    }
    if !res.complete {
        res.reads.clear();
        res.reads_live.clear();
        res.live_read_bytes.clear();
        res.stored_read_bytes.clear();
        res.cuts.clear();
        res.opaque_cuts.clear();
        res.named_cuts.clear();
        res.returning_opaque = 0;
        res.return_reads.clear();
        res.cond_written.clear();
        res.skipped = None;
    }
    res
}

/// Where a walk sends the paths that skip a conditional return.
enum Skips<'a> {
    /// Not walked: the path ends with the return, and [`CalleeEntryDead::lost`] records it.
    Dropped,
    /// Collected for a separate walk.
    Aside(&'a mut Vec<Frame>),
    /// Walked here, as any other path.
    Walked,
}

/// Walk every path from `todo` into `res`.
fn walk<T: kuna_sleigh::translate::Translate + ?Sized>(
    tr: &T,
    res: &mut CalleeEntryDead,
    mut todo: Vec<Frame>,
    mut skips: Skips<'_>,
    traps: &[uint4],
) {
    let mut visited: HashMap<(int4, u64), ByteSet> = HashMap::new();
    let mut budget = MAX_PROBE_INSTRUCTIONS;
    let follow = !matches!(skips, Skips::Dropped);
    let mut found = Vec::new();
    while let Some(frame) = todo.pop() {
        let Some(sp) = frame.at.get_space() else {
            res.complete = false;
            break;
        };
        let key = (sp.get_index(), frame.at.get_offset());
        // A revisit whose incoming written set is a SUPERSET of one already
        // processed adds nothing: the earlier, more conservative pass recorded
        // at least every read-before-write this one would.
        if let Some(prev) = visited.get(&key) {
            if prev.is_subset(&frame.written) {
                continue;
            }
        }
        visited.insert(key, frame.written.clone());
        if budget == 0 || res.cuts.len() > MAX_CUTS || frame.written.len() > MAX_WRITTEN_BYTES {
            res.complete = false;
            break;
        }
        budget -= 1;
        let mut emit = EntryEmit::default();
        let len = match tr.one_instruction(&mut emit, &frame.at) {
            Ok(n) if n > 0 => n,
            // Undecodable: the path ends where the walk cannot see.
            _ => {
                res.undecodable = true;
                res.opaque_cuts.push(frame.written.clone());
                res.cuts.push(frame.written);
                continue;
            }
        };
        let skip = if follow { Some(&mut found) } else { None };
        match step_instruction(res, &emit, frame.written, &frame.at, len, skip, traps) {
            Some(next) => todo.extend(next),
            None => break,
        }
        match &mut skips {
            Skips::Dropped => {}
            Skips::Aside(out) => out.append(&mut found),
            Skips::Walked => todo.append(&mut found),
        }
    }
}

/// Count a user op whose instruction then only computes and returns, and keep
/// the register reads it makes on the way. Answers whether it was one.
fn note_returning_user_op(res: &mut CalleeEntryDead, rest: &[RawOp]) -> bool {
    let returns = rest.last().is_some_and(|o| o.opc == OpCode::CPUI_RETURN);
    let flows = rest[..rest.len().saturating_sub(1)].iter().any(|o| {
        matches!(
            o.opc,
            OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_LOAD
                | OpCode::CPUI_STORE
        )
    });
    if !returns || flows {
        return false;
    }
    res.returning_opaque += 1;
    for o in rest {
        for v in &o.ins {
            if v.space.as_ref().is_some_and(|sp| sp.get_index() == res.reg_idx) {
                res.return_reads.push((res.reg_idx, v.offset, v.size as int4));
            }
        }
    }
    true
}

/// End a path at a return inside an instruction that may already have branched
/// past it. With `skip`, each such branch's path resumes there from the bytes
/// written before it; without, the path is lost.
fn end_at_return(
    res: &mut CalleeEntryDead,
    targets: &[Address],
    before: &[ByteSet],
    skip: Option<&mut Vec<Frame>>,
) -> Option<Vec<Frame>> {
    match skip {
        Some(out) if targets.len() == before.len() => out.extend(
            targets.iter().zip(before).map(|(t, w)| Frame { at: t.clone(), written: w.clone() }),
        ),
        _ => res.lost |= !targets.is_empty(),
    }
    Some(Vec::new())
}

/// Is this op's result a CONSTANT whatever the register held — a compiler's way
/// of writing a fixed value into a register it is about to reuse?
///
/// `xor ecx,ecx` and `sub eax,eax` are how every compiler writes zero, `and
/// edx,0` the same, and `or rdx,-1` is how it writes all-ones; each of them
/// reads the register it is about to clobber, and the p-code says so. The read
/// is formal — the result does not depend on it — and counting it would make a
/// scratch register look like an incoming argument.
fn is_value_erasing(op: &RawOp) -> bool {
    if op.ins.len() != 2 {
        return false;
    }
    let (a, b) = (&op.ins[0], &op.ins[1]);
    let same_varnode = a.size == b.size
        && a.offset == b.offset
        && match (a.space.as_ref(), b.space.as_ref()) {
            (Some(x), Some(y)) => x.get_index() == y.get_index(),
            _ => false,
        };
    let constant = |v: &VarnodeData, want: u64| {
        v.space.as_ref().map(|s| s.get_type() == spacetype::IPTR_CONSTANT).unwrap_or(false)
            && v.offset == want
    };
    let all_ones = |v: &VarnodeData| {
        let mask = if v.size >= 8 { u64::MAX } else { (1u64 << (v.size * 8)) - 1 };
        constant(v, mask)
    };
    match op.opc {
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB => same_varnode,
        OpCode::CPUI_INT_AND => constant(a, 0) || constant(b, 0),
        OpCode::CPUI_INT_OR => all_ones(a) || all_ones(b),
        _ => false,
    }
}

/// Does this op compare a value with itself, so its result is the same
/// whatever the value is? x86 `sbb %ecx,%ecx` computes its carry and overflow
/// from `ecx < ecx` and `sborrow(ecx,ecx)` and its result from `ecx - ecx`.
fn compares_with_itself(op: &RawOp) -> bool {
    let [a, b] = op.ins.as_slice() else { return false };
    let same = a.size == b.size
        && a.offset == b.offset
        && matches!((a.space.as_ref(), b.space.as_ref()), (Some(x), Some(y)) if x.get_index() == y.get_index());
    same && matches!(
        op.opc,
        OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SBORROW
    )
}

/// Run one decoded instruction's p-code against the incoming written set.
///
/// Records every read-before-write into `res`, and returns the frames the walk
/// should continue with (empty when the path ended). `None` means the summary
/// was abandoned. With `skip`, the paths that branch past a return inside this
/// instruction are pushed there.
fn step_instruction(
    res: &mut CalleeEntryDead,
    emit: &EntryEmit,
    written: ByteSet,
    at: &Address,
    len: i32,
    skip: Option<&mut Vec<Frame>>,
    traps: &[uint4],
) -> Option<Vec<Frame>> {
    let incoming = written.clone();
    let mut cur = written;
    let mut targets: Vec<Address> = Vec::new();
    let mut before: Vec<ByteSet> = Vec::new();
    let mut ends_flow = false;
    let multi_store = emit
        .ops
        .iter()
        .filter(|o| {
            o.opc == OpCode::CPUI_STORE
                && o.ins.get(2).and_then(|v| v.space.as_ref()).is_some_and(|s| s.get_index() == res.reg_idx)
        })
        .count()
        >= 2;
    for (k, op) in emit.ops.iter().enumerate() {
        // Reads. An instruction that branches inside itself is scored against
        // the set it was entered with, since a conditionally-executed write
        // earlier in the same instruction may not have run.
        let base = if emit.internal_flow { &incoming } else { &cur };
        let value_erasing = is_value_erasing(op);
        let formal = value_erasing || compares_with_itself(op);
        for (i, v) in op.ins.iter().enumerate() {
            if skip_input(op.opc, i) {
                continue;
            }
            let Some(sp) = v.space.as_ref() else { continue };
            if sp.get_index() != res.reg_idx {
                continue;
            }
            let (idx, off, sz) = (res.reg_idx, v.offset, v.size as int4);
            if (off..off + v.size as u64).any(|b| !base.contains(&(idx, b))) {
                res.reads.push((idx, off, sz));
                if !value_erasing {
                    res.reads_live.push((idx, off, sz));
                }
                let stored = op.opc == OpCode::CPUI_STORE && i == 2;
                if !formal && !(stored && multi_store) {
                    let into = if stored { &mut res.stored_read_bytes } else { &mut res.live_read_bytes };
                    for b in off..off + v.size as u64 {
                        if !base.contains(&(idx, b)) {
                            into.insert((idx, b));
                        }
                    }
                }
            }
        }
        match op.opc {
            // Control transfer into code this walk is not reading. A direct
            // CALL is recorded with its target, so the forwarding question can
            // be asked of that target in turn; the indirect forms name nothing
            // to ask, and are recorded as opaque.
            OpCode::CPUI_CALL => {
                match op.ins.first().and_then(|v| v.space.as_ref()) {
                    Some(sp) if sp.get_type() == spacetype::IPTR_PROCESSOR => {
                        let t = Address::new(Rc::clone(sp), op.ins[0].offset);
                        res.named_cuts.push((t, cur.clone()));
                    }
                    _ => res.opaque_cuts.push(cur.clone()),
                }
                res.cuts.push(cur);
                res.lost |= !targets.is_empty();
                return Some(Vec::new());
            }
            OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCHIND => {
                if op.opc == OpCode::CPUI_CALLOTHER
                    && op.ins.first().is_some_and(|v| traps.contains(&(v.offset as uint4)))
                {
                    res.trap_cuts.push(cur.clone());
                }
                let returns = op.opc == OpCode::CPUI_CALLOTHER
                    && !emit.internal_flow
                    && note_returning_user_op(res, &emit.ops[k + 1..]);
                res.opaque_cuts.push(cur.clone());
                res.cuts.push(cur);
                if returns {
                    return end_at_return(res, &targets, &before, skip);
                }
                res.lost |= !targets.is_empty();
                return Some(Vec::new());
            }
            // A RETURN is a path terminator like any other: the register has to
            // be written BEFORE it, or the body has shown nothing.  A callee
            // whose whole body is `ret` reads nothing, and treating that as
            // proof would delete the arguments of every stub and thunk.
            OpCode::CPUI_RETURN => {
                res.cuts.push(cur);
                return end_at_return(res, &targets, &before, skip);
            }
            // The `<spaceid>` operand's offset IS the space-manager index. An
            // access to the register space through LOAD/STORE is an indexed
            // register file: the walk cannot say which register it names.
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                if op.ins.first().map(|v| v.offset as int4) == Some(res.reg_idx) {
                    res.opaque_cuts.push(cur.clone());
                    res.cuts.push(cur);
                    res.lost |= !targets.is_empty();
                    return Some(Vec::new());
                }
            }
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                match op.ins.first().and_then(|v| v.space.as_ref()) {
                    // p-code-relative: stays inside this instruction, and the
                    // whole instruction is being read anyway.
                    Some(sp) if sp.get_type() == spacetype::IPTR_CONSTANT => {}
                    Some(sp) => {
                        targets.push(Address::new(Rc::clone(sp), op.ins[0].offset));
                        if skip.is_some() {
                            before.push(cur.clone());
                        }
                        if op.opc == OpCode::CPUI_BRANCH {
                            ends_flow = true;
                        }
                    }
                    None => {
                        res.opaque_cuts.push(cur.clone());
                        res.cuts.push(cur);
                        res.lost |= !targets.is_empty();
                        return Some(Vec::new());
                    }
                }
            }
            _ => {}
        }
        // Writes, credited only for an instruction with no internal branching.
        if !emit.internal_flow {
            if let Some(o) = &op.out {
                if let Some(sp) = o.space.as_ref() {
                    if sp.get_index() == res.reg_idx {
                        let idx = res.reg_idx;
                        for b in o.offset..o.offset + o.size as u64 {
                            if cur.insert((idx, b)) && !targets.is_empty() {
                                res.cond_written.insert((idx, b));
                            }
                        }
                        if cur.len() > MAX_WRITTEN_BYTES {
                            res.complete = false;
                            return None;
                        }
                    }
                }
            }
        }
    }
    let mut next: Vec<Frame> =
        targets.into_iter().map(|t| Frame { at: t, written: cur.clone() }).collect();
    if !ends_flow {
        next.push(Frame { at: at + len as i64, written: cur });
    }
    Some(next)
}

/// Is input `i` of `opc` an address/annotation operand rather than a data read?
fn skip_input(opc: OpCode, i: usize) -> bool {
    match opc {
        // Slot 0 is the branch/call destination.
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND
        | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => i == 0,
        // Slot 0 is the userop id; slot 0 of LOAD/STORE is the space id.
        OpCode::CPUI_CALLOTHER | OpCode::CPUI_LOAD | OpCode::CPUI_STORE => i == 0,
        _ => false,
    }
}

/// Take the callee-body entry-liveness probe for every direct call in `data`,
/// before the action pipeline reaches the trial-scoring seam that needs it.
///
/// Called from the driver for the same reason
/// [`crate::kuna_rustabi::seed_callee_return_writes`] is: the per-function
/// `ArchContext` the pipeline runs against carries the load image but no
/// translator, so the probe cannot be taken lazily at the seam. Results are
/// cached on the `Architecture`, so each distinct callee body is decoded once per
/// run. A no-op with the option off.
pub fn seed_callee_entry_dead(
    arch: &mut crate::architecture::Architecture,
    data: &mut Funcdata,
) {
    if crate::kuna_armfloatargs::applies(arch) && !data.get_address().is_invalid() {
        let reg_idx =
            arch.manage().get_space_by_name("register").map(|s| s.get_index()).unwrap_or(-1);
        let own = data.get_address().clone();
        if let Some(d) = (reg_idx >= 0).then(|| probe_cached(arch, &own, reg_idx)).flatten() {
            data.kuna_set_own_entry_dead(d);
        }
    }
    let body_arity = arch.callee_arity && arch.callee_arity_body;
    // `argclobber` reads this probe only as a veto on a drop its recovered-
    // prototype clause already admitted, so with nothing parked it can never
    // consult it and the decode is pure cost.
    let clobber_veto = arch.arg_clobber && !arch.kuna_protoorder_types.is_empty();
    let pass_through = arch.pass_through && !arch.kuna_protoorder_types.is_empty();
    let reg_idx =
        arch.manage().get_space_by_name("register").map(|s| s.get_index()).unwrap_or(-1);
    if reg_idx < 0 {
        return;
    }
    let hidden_ret = arch.hidden_ret_arg && {
        let proto = data.get_func_proto();
        proto.has_model()
            && crate::p4_calls::kuna_hiddenretarg::has_register_hidden_return(proto.model(), reg_idx)
    };
    let read_arg = arch.callee_read_arg;
    if !arch.callee_dead_arg
        && !clobber_veto
        && !(arch.callee_arity && arch.callee_arity_live)
        && !body_arity
        && !pass_through
        && !hidden_ret
        && !read_arg
    {
        return;
    }
    // The veto needs an EARLIER call to have left the value in the register, so
    // a function with fewer than two calls can never produce one — and probing
    // its callees would be pure cost.  This matters most in ghidra mode, where a
    // decode is a round trip to the host.  `calleearitybody` is the exception:
    // its whole subject is the callee that is called ONCE, and so is a model
    // with a hidden-return register, whose veto needs no earlier call.  So is
    // `calleereadarg`, which reads the probe at any one call.
    if data.num_calls() < 2 && !body_arity && !pass_through && !hidden_ret && !read_arg {
        return;
    }
    let mut through_memo = ThroughMemo::new();
    let mut entries: Vec<Address> = Vec::new();
    // `passthrough` asks this walk of the function's OWN body too: a register it
    // writes before reading is not a parameter it can be carrying, which is what
    // says a claim across that slot would materialize one
    // ([`crate::p4_calls::kuna_passthrough`]'s `no_hole_before`).
    // `calleereadarg` asks it too, of a function with a call to extend: one
    // that reads its last argument register on entry may be variadic, and is
    // left alone.
    if (pass_through || (read_arg && data.num_calls() > 0)) && !data.get_address().is_invalid() {
        entries.push(data.get_address().clone());
    }
    for i in 0..data.num_calls() {
        let e = data.get_call_specs(i).get_entry_address().clone();
        if e.is_invalid() {
            continue;
        }
        entries.push(e);
    }
    for e in entries {
        let Some(sp) = e.get_space() else { continue };
        let key = (sp.get_index(), e.get_offset());
        if let Some(d) = probe_cached(arch, &e, reg_idx) {
            data.kuna_set_callee_entry_dead(&e, d);
        }
        if pass_through && arch.kuna_protoorder_types.get(&key).is_some_and(|s| !s.inputs.is_empty()) {
            if let Some(t) = reads_through_calls(arch, &e, reg_idx, &mut through_memo) {
                data.kuna_set_callee_entry_through(&e, t);
            }
        }
        // `argclobber` also needs what this callee can FORWARD, which is a walk
        // over the bodies it calls in turn and so cannot be answered at the
        // seam either.
        if clobber_veto {
            if !arch.kuna_callee_forward_cache.contains_key(&key) {
                let fwd = resolve_forward_transfer(arch, &e, reg_idx);
                arch.kuna_callee_forward_cache.insert(key, Rc::new(fwd));
            }
            if let Some(f) = arch.kuna_callee_forward_cache.get(&key) {
                data.kuna_set_callee_forward(&e, Rc::clone(f));
            }
        }
    }
}

/// How deep the leftover-return-value test walks through value-preserving ops
/// before giving up. The chain between the two calls is short but not direct:
/// on the AArch64 witness it is `INDIRECT -> INDIRECT -> INT_ZEXT -> SUBPIECE ->
/// PIECE -> INDIRECT(creation)`, six links of register-width bookkeeping around
/// the call that produced the value.
const MAX_LEFTOVER_DEPTH: u32 = 12;

/// Is `vn` a value the caller never computed *for this call* — the leftover
/// output of an EARLIER call still sitting in the register?
///
/// True only when EVERY chain back from `vn` bottoms out at a call: a `CALL`/
/// `CALLIND` output, or the INDIRECT creation `guard_calls` plants for each
/// register a call may clobber. Value-preserving and width-adjusting links
/// (`COPY`, `SUBPIECE`, `PIECE`, `INT_ZEXT`, `INT_SEXT`, a non-creation
/// `INDIRECT` standing for "survived across a call") are followed through, and
/// a `PIECE` or `MULTIEQUAL` has to have all of its inputs qualify. Anything
/// else — a constant, a `LOAD`, arithmetic, or a Varnode that is a function
/// input — answers `false`, because the caller put it there on purpose.
pub(crate) fn is_leftover_call_result(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth >= MAX_LEFTOVER_DEPTH {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.is_written() {
        return false;
    }
    let Some(def) = v.get_def() else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    let all_inputs = |n: int4| -> bool {
        (0..n).all(|i| match data.obank().get(def).and_then(|o| o.get_in(i)) {
            Some(x) => is_leftover_call_result(data, x, depth + 1),
            None => false,
        })
    };
    match op.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => true,
        OpCode::CPUI_INDIRECT if op.is_indirect_creation() => true,
        OpCode::CPUI_COPY
        | OpCode::CPUI_SUBPIECE
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_INDIRECT => match op.get_in(0) {
            Some(next) => is_leftover_call_result(data, next, depth + 1),
            None => false,
        },
        OpCode::CPUI_PIECE => all_inputs(2),
        OpCode::CPUI_MULTIEQUAL => all_inputs(op.num_input()),
        _ => false,
    }
}

/// Does the callee's own body prove this register trial is not an argument?
///
/// The `checkInputTrialUse` register arm asks this before scoring a trial. Two
/// independent things have to hold, and the pass is only as safe as their
/// conjunction:
///
/// 1. the value in the register is an earlier call's leftover result, not
///    something the caller placed there ([`is_leftover_call_result`]); and
/// 2. the callee overwrites that register on every path from its entry, before
///    ever reading it ([`CalleeEntryDead::proves_dead`]; for a VFP register
///    with `armfloatargs` on, by instructions that always run).
///
/// Answers `false` with the option off, for a non-register trial, for an
/// indirect call, and for every callee the probe could not fully cover.
pub fn trial_is_dead_in_callee(
    data: &Funcdata,
    call_idx: int4,
    slot: int4,
    trial_addr: &Address,
    trial_size: int4,
) -> bool {
    if !data.get_arch().callee_dead_arg {
        return false;
    }
    let entry = data.get_call_specs(call_idx).get_entry_address();
    if entry.is_invalid() {
        return false;
    }
    let call = data.get_call_specs(call_idx);
    let dead = match data.kuna_callee_entry_dead(entry) {
        Some(d) if crate::kuna_armfloatargs::vfp_slot(data, call, trial_addr, trial_size) => {
            d.proves_dead_firmly(trial_addr, trial_size)
        }
        Some(d) => d.proves_dead(trial_addr, trial_size),
        None => false,
    };
    if !dead {
        return false;
    }
    let op = data.get_call_specs(call_idx).get_op();
    match data.obank().get(op).and_then(|o| o.get_in(slot)) {
        Some(vn) => is_leftover_call_result(data, vn, 0),
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_calleedeadarg/tests.rs"]
mod tests;
