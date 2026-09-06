//! (kuna, rustc) Keep the two-register value a Rust function returns, and connect
//! it at the call.
//!
//! # The symptom
//!
//! A `Result<u32,u32>` producer and the `match` that consumes it, `rustc -C
//! opt-level=2`, x86-64:
//!
//! ```text
//! prod:  xor %eax,%eax; cmp $0xb,%edi; setb %al; lea 0x7(%rdi),%edx; cmovae %ecx,%edx; ret
//! cons:  call prod; test $0x1,%al; lea 0x1(%rdx),%esi; lea 0x64(%rdx),%ecx; cmove %esi,%ecx
//! ```
//!
//! recovered as
//!
//! ```text
//! bool prod(uint4 a0) { return a0 < 0xb; }        // the EDX payload is GONE
//!
//! int4 cons(void) {
//!   uint8 v2; // rax
//!   int4 v3;  // edx                              // declared, read twice, NEVER ASSIGNED
//!   prod();
//!   v1 = v3 + 100;
//!   if (!(v2 & 1)) v1 = v3 + 1;
//!   return v1;
//! }
//! ```
//!
//! The payload the whole function exists to produce is not merely mis-typed: on
//! the producer side it is deleted, and on the consumer side it is a variable
//! with no definition anywhere in the program. Every `Result`/`Option` guard
//! downstream reads a phantom.
//!
//! # What was actually missing
//!
//! Not the ABI decision. rustc's `ScalarPair` return -- discriminant in the first
//! return register, payload in the second -- lands on exactly the storage the
//! x86-64 cspec's `<join_dual_class/>` output rule already describes, and kuna
//! already recovers it: two active output trials in `RAX`/`RDX`, the rule matches,
//! `buildReturnOutput` builds the `join` concatenation. Two *later* seams throw it
//! away, and both of them are invisible to a C corpus because a C function whose
//! first returned register holds a one-bit value is rare and a Rust `Result` is
//! nothing else.
//!
//! **Producer.** `SubvariableFlow::tryReturnPull` truncates a RETURN to the
//! logical width of the value being traced. rustc materializes the discriminant
//! with `xor %eax,%eax; setb %al`, so `RAX` is a one-bit logical value and the
//! subvariable rules truncate the whole RETURN to it -- which silently discards
//! the *other register* in the concatenation. The recovered return goes from a
//! 16-byte pair to `bool`.
//!
//! **Consumer.** `FuncCallSpecs::buildOutputFromTrials` handles one used output
//! trial and returns early on two or more ("STUB(W4 translate-on-handle)"), so a
//! call whose model asks for a register pair gets **no output at all**. The
//! INDIRECT creations that stood for "the callee wrote something here" survive,
//! and every read of the payload register after the call renders as a local the
//! function never assigns. The stub's stated blocker -- no `constructJoinAddress`
//! on the merged arch handle -- is stale: the sibling
//! `ActionReturnRecovery::buildReturnOutput` calls it today.
//!
//! # Two seams, two classifications -- and what each one can prove
//!
//! The two seams ask the same *question* -- is this register pair one logical
//! value? -- but they are not looking at the same thing, so they cannot share an
//! answer. Saying they do would be a false claim about the evidence.
//!
//! ## The producer: [`classify_return_pair`]
//!
//! Here the concatenation's halves are values *this* function computed, so their
//! shape answers the question. Asked of the observed writes at the return sites,
//! never of a size -- rustc's choice is a function of the variant layout, and
//! size does not predict it (`Result<u32,u32>` is 8 bytes and a pair;
//! `Result<Box<u64>,u32>` is 16 bytes and is returned through memory). Three
//! verdicts:
//!
//! * **`ScalarPair`** -- the least-significant half is *discriminant-shaped*: its
//!   value at the RETURN is confined to a byte (a small constant, a `setCC`, a
//!   masked flag, or a phi over those). That is the discriminant, and the other
//!   half is its payload. The recognition is deliberately on the value's known
//!   non-zero bits rather than on "a constant per path", because rustc emits both
//!   forms for the same source -- `mov $0/$1` on the branchy shape and `setb %al`
//!   on the branchless one, and the branchless shape is the common one at
//!   `-C opt-level=2`.
//! * **`Memory`** -- the half traces back to the function's own incoming pointer
//!   argument. That is the `sret` epilogue (`mov %rbx,%rax` where `rbx` came from
//!   `rdi`), where `RAX` carries the hidden return pointer and is not a
//!   discriminant. A veto, not an action: the pair must not form.
//! * **`Scalar`** -- anything else. Today's answer, unchanged.
//!
//! ## The consumer: [`classify_call_output_pair`]
//!
//! At a call there are **no callee values in the IR at all**. Both halves are
//! INDIRECT creations standing for "the callee may have written this", so their
//! shape says nothing and `classify_return_pair` has nothing to read. Running it
//! here would be reading the caller's tea leaves and calling it a classification.
//!
//! What the seam has instead is three pieces of real evidence, and
//! [`classify_call_output_pair`] is the whole list:
//!
//! 1. **The prototype model.** The `join_dual_class` output rule already matched
//!    a justified, consecutive, first-in-class register pair -- that is what put
//!    two used trials here rather than one.
//! 2. **The caller's reads.** Both halves are read out of the call (an unread
//!    trial is not active), they are distinct non-overlapping registers, and the
//!    payload half has a descendant.
//! 3. **The callee's body.** [`probe_callee_return_writes`] decodes the direct
//!    callee, bounded, and reports the processor-space writes it can *prove*. If
//!    that proof covers every path to a `RETURN` and never touches the payload
//!    register, the caller's read is a clobber and **the pair is vetoed**.
//!
//! The third is the only one that looks at the callee, and it is one-sided: it
//! can refute a pair, never confirm one. A callee containing any call, indirect
//! branch, undecodable byte, or more instructions than the budget yields no
//! proof, and the seam falls back on 1 and 2.
//!
//! **So `ScalarPair` at the consumer means "no counter-example", not "the callee
//! returns a pair".** That positive fact is not derivable at this point in the
//! pipeline: the recovered prototype of a local callee is never written back to
//! the symbol table, so a caller has no recovered callee signature to consult,
//! and the residual evidence is the same evidence upstream Ghidra ships this
//! branch on unguarded. The honest reading of the consumer half is *complete the
//! stubbed multi-trial branch, and refuse it where the callee refutes it*.
//!
//! # What this does NOT do
//!
//! It does not name anything `Result` or `Option`, does not synthesize a union,
//! struct or enum type, and does not touch emission. Its entire deliverable is
//! that the payload **exists as a variable and is connected to its producer**;
//! spelling that value as a Rust enum is a later, separate decision that cannot
//! be made until the value survives to be spelled.
//!
//! # The gate
//!
//! `option rustabi off|auto|always`. `auto` acts only when the loader's
//! source-language detection reported rustc (`Compiler::Rustc`, from the
//! `.comment` `rustc version` record, a `.rodata` signature, or a Rust-mangled
//! symbol), which the XML datatest bootstrap never runs -- so the C corpus is
//! untouched by construction, not by luck. `always` drops the language test and
//! is what the stage testcase uses, since a `<bytechunk>` carries no
//! `.comment`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// How far back the discriminant/sret walks chase move-only operations. The
/// shapes that matter are one or two copies deep; the bound only stops a cycle.
const MAX_DEPTH: u32 = 16;

/// The widest non-zero mask a *discriminant* half may carry. A tag is an index
/// into a variant list, so one byte is already generous; a payload register that
/// happens to be byte-wide is indistinguishable from a tag and is left alone.
const DISCRIMINANT_MASK: u64 = 0xff;

/// The three values of `option rustabi`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustAbiMode {
    /// Never act (the shipped default): byte-identical to the pre-fix engine.
    Off,
    /// Act only when the loader detected a rustc-produced image.
    Auto,
    /// Act regardless of the detected source language.
    Always,
}

impl RustAbiMode {
    /// The `option rustabi <p1>` token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            RustAbiMode::Off => "off",
            RustAbiMode::Auto => "auto",
            RustAbiMode::Always => "always",
        }
    }

    /// The wire encoding carried on `Architecture`/`ArchContext` (0/1/2).
    pub fn as_u8(self) -> u8 {
        match self {
            RustAbiMode::Off => 0,
            RustAbiMode::Auto => 1,
            RustAbiMode::Always => 2,
        }
    }

    /// Decode the wire encoding; anything unrecognized is [`RustAbiMode::Off`].
    pub fn from_u8(v: u8) -> RustAbiMode {
        match v {
            1 => RustAbiMode::Auto,
            2 => RustAbiMode::Always,
            _ => RustAbiMode::Off,
        }
    }
}

/// Parse `option rustabi off|auto|always`, returning the mode and the
/// confirmation message.
pub fn parse_rust_abi_mode(p1: &str) -> KunaResult<(RustAbiMode, String)> {
    let mode = match p1.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => RustAbiMode::Off,
        "auto" | "on" | "1" | "true" => RustAbiMode::Auto,
        "always" => RustAbiMode::Always,
        _ => return Err(KunaError::parse("Must specify off, auto or always")),
    };
    Ok((mode, format!("Rust return-ABI pair recovery set to {} form", mode.as_str())))
}

/// Is the Rust return-ABI rule live for this function?
pub fn live(data: &Funcdata) -> bool {
    match RustAbiMode::from_u8(data.get_arch().rust_abi) {
        RustAbiMode::Off => false,
        RustAbiMode::Auto => data.get_arch().source_is_rust,
        RustAbiMode::Always => true,
    }
}

/// What one recovered register pair represents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnRepr {
    /// Discriminant + payload: two registers holding one logical value.
    ScalarPair,
    /// A hidden return pointer handed back in the first return register.
    Memory,
    /// Neither shape applies; keep today's answer.
    Scalar,
}

/// Classify a recovered two-register return concatenation from its halves.
///
/// `lo` is the least-significant register (the first return register, where
/// rustc puts the tag) and `hi` the second. Three conditions, in the order they
/// can be decided: the low half is not the sret pointer, the low half is a tag,
/// and the high half is a value this function put there rather than leftover.
pub fn classify_return_pair(data: &Funcdata, hi: VarnodeId, lo: VarnodeId) -> ReturnRepr {
    if traces_to_incoming_pointer(data, lo, 0) {
        return ReturnRepr::Memory;
    }
    if is_discriminant_shaped(data, lo) && carries_a_payload(data, hi) {
        return ReturnRepr::ScalarPair;
    }
    ReturnRepr::Scalar
}

/// Is `vn` a value the function put in the payload register?
///
/// The narrow half alone does not make a pair: the same shape appears when a
/// function returns a boolean and the second return register merely still holds
/// whatever it held. Two terminals say leftover — a Varnode the function never
/// wrote, and a callee's INDIRECT creation — and they are the same two
/// [`crate::kuna_returnuncomputed`] exists to drop. Asking here as well keeps
/// this rule from holding a pair alive for the phantom-killer to have to undo.
fn carries_a_payload(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    let Some(def) = v.get_def() else { return false };
    match data.obank().get(def) {
        Some(op) => !(op.code() == OpCode::CPUI_INDIRECT && op.is_indirect_creation()),
        None => false,
    }
}

/// Is `vn` confined to the bits a variant tag occupies?
///
/// A constant that fits in a byte, or a value whose dataflow-computed non-zero
/// mask fits in a byte. The second arm is what catches `xor %eax,%eax; setb %al`
/// and `and $0x1,%eax`, which is how rustc materializes a two-variant tag at
/// `-C opt-level=2`; the first catches the branchy `mov $0` / `mov $1` form.
fn is_discriminant_shaped(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return v.get_offset() <= DISCRIMINANT_MASK;
    }
    // A zero non-zero-mask means "provably zero", which is a tag value too, but
    // it is also what an unanalyzed Varnode reports before dataflow has run; the
    // rule only ever runs after heritage, where zero is a real answer.
    v.get_nz_mask() <= DISCRIMINANT_MASK
}

/// Does `vn` trace back, through move-only operations, to an incoming pointer
/// argument of this function?
///
/// The `sret` shape: the callee is handed the result buffer in the first
/// argument register and returns that same pointer, so the first return register
/// carries an address, not a tag.
fn traces_to_incoming_pointer(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth >= MAX_DEPTH {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    let Some(def) = v.get_def() else {
        // An unwritten Varnode in input-parameter storage, pointer-sized.
        if !v.is_input() {
            return false;
        }
        let (addr, size) = (v.get_addr().clone(), v.get_size());
        if Some(size) != pointer_size(data) {
            return false;
        }
        return data.get_func_proto().possible_input_param(&addr, size);
    };
    let Some(op) = data.obank().get(def) else { return false };
    let inputs: Vec<VarnodeId> = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => op.get_in(0).into_iter().collect(),
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).filter_map(|i| op.get_in(i)).collect(),
        _ => return false,
    };
    inputs.into_iter().any(|i| traces_to_incoming_pointer(data, i, depth + 1))
}

/// Does `vn` hold a recovered register pair that must survive intact?
///
/// The predicate `SubvariableFlow::tryReturnPull` consults before truncating a
/// RETURN. `vn` is the RETURN's value; the pair shape is the `join`-space
/// concatenation that `ActionReturnRecovery::buildReturnOutput` builds, and
/// nothing else in the engine produces one. Truncating it would keep one half's
/// logical bits and drop the other register entirely -- which is not a narrower
/// rendering of the same value, it is a different value.
///
/// The later uncomputed-half repair ([`crate::kuna_returnuncomputed`]) still runs
/// on the pair this keeps alive, so a half that is genuine leftover is still
/// dropped -- just by the rule that can tell, instead of by a width heuristic.
pub fn holds_scalar_pair(data: &Funcdata, vn: VarnodeId) -> bool {
    if !live(data) {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.get_addr().is_join() {
        return false;
    }
    let Some((hi, lo)) = pair_pieces(data, vn, 0) else { return false };
    classify_return_pair(data, hi, lo) == ReturnRepr::ScalarPair
}

/// The two halves of the concatenation behind `vn`, looking through the
/// width-reshaping the rule pool applies to it.
///
/// `buildReturnOutput` leaves a bare `PIECE(hi, lo)`, but the pool rewrites that
/// shape as soon as a half is itself an extension: `RuleConcatZext` turns
/// `PIECE(ZEXT(V), W)` into `ZEXT(PIECE(V, W))`, which is exactly what happens
/// when the payload register is written 32-bit (`lea 0x7(%rdi),%edx`) -- the
/// overwhelmingly common rustc case. Matching only the bare PIECE would miss it.
fn pair_pieces(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<(VarnodeId, VarnodeId)> {
    if depth >= MAX_DEPTH {
        return None;
    }
    let def = data.vbank().get(vn)?.get_def()?;
    let op = data.obank().get(def)?;
    match op.code() {
        OpCode::CPUI_PIECE if op.num_input() == 2 => Some((op.get_in(0)?, op.get_in(1)?)),
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            pair_pieces(data, op.get_in(0)?, depth + 1)
        }
        _ => None,
    }
}

// =============================================================================
// The consumer seam: what a CALL site can and cannot establish
// =============================================================================

/// The register writes a bounded decode of one callee body proves.
///
/// Completeness is the load-bearing property: it says the walk
/// reached a `RETURN` on every path it followed, inside the instruction budget,
/// without meeting anything that could write an arbitrary register (a nested
/// call, an unresolved indirect branch, an undecodable byte). Only then does an
/// *absent* write mean the callee never performs it; otherwise the summary
/// proves nothing and every query answers "may write".
#[derive(Clone, Debug, Default)]
pub struct CalleeReturnWrites {
    /// Processor-space ranges the decoded body writes, as `(space index,
    /// offset, size)`.
    writes: Vec<(int4, u64, int4)>,
    /// Space indices the decoded body STOREs into. A STORE's address is a
    /// runtime value, so it stands for "any location in that space" -- which
    /// matters only on a processor that puts its registers behind an indexed
    /// register file, and is free to record either way.
    store_spaces: Vec<int4>,
    /// Did the walk cover every path to a `RETURN` with nothing unresolved?
    complete: bool,
}

impl CalleeReturnWrites {
    /// Does this summary *prove* the callee never writes any byte of
    /// `[addr, addr+size)`?
    ///
    /// Answers `false` for every incomplete summary, so a callee the probe
    /// could not fully cover never vetoes anything.
    pub fn proves_untouched(&self, addr: &Address, size: int4) -> bool {
        if !self.complete {
            return false;
        }
        let Some(sp) = addr.get_space() else { return false };
        let (idx, off, end) = (sp.get_index(), addr.get_offset(), addr.get_offset() + size as u64);
        if self.store_spaces.contains(&idx) {
            return false;
        }
        !self
            .writes
            .iter()
            .any(|&(widx, woff, wsz)| widx == idx && woff < end && off < woff + wsz as u64)
    }

    /// The processor-space ranges the walk recorded, as `(space index, offset,
    /// size)`.  Empty for an incomplete summary, which records nothing.
    pub fn written_ranges(&self) -> &[(int4, u64, int4)] {
        &self.writes
    }

    /// Did the walk complete? (Diagnostics and tests.)
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Assemble a summary from its parts, for tests that pin a seam's reading of
    /// one rather than the walk that produced it.
    #[cfg(test)]
    pub fn from_parts(
        writes: Vec<(int4, u64, int4)>,
        store_spaces: Vec<int4>,
        complete: bool,
    ) -> Self {
        CalleeReturnWrites { writes, store_spaces, complete }
    }
}

/// How many machine instructions the callee probe decodes before giving up.
///
/// The walk exists to recognize a *leaf* callee that provably leaves a return
/// register alone; anything longer than this is a function whose answer would be
/// "may write" almost surely, and the bound is what keeps the probe off the
/// critical path.
const MAX_PROBE_INSTRUCTIONS: u32 = 192;

/// How many write records one summary keeps before it declares itself
/// incomplete. A body that writes this many distinct locations is not the leaf
/// shape the probe is looking for.
const MAX_PROBE_WRITES: usize = 512;

/// Recording sink for [`probe_callee_return_writes`] (the probe's `PcodeEmit`).
#[derive(Default)]
struct ProbeEmit {
    writes: Vec<(int4, u64, int4)>,
    store_spaces: Vec<int4>,
    targets: Vec<Address>,
    ends_flow: bool,
    unresolved: bool,
}

impl kuna_sleigh::translate::PcodeEmit for ProbeEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&kuna_num::pcoderaw::VarnodeData>,
        vars: &[kuna_num::pcoderaw::VarnodeData],
    ) {
        if let Some(o) = outvar {
            if let Some(sp) = &o.space {
                if sp.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR {
                    self.writes.push((sp.get_index(), o.offset, o.size as int4));
                }
            }
        }
        match opc {
            // Anything that transfers control to code the walk is not reading
            // can write any register at all.
            OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_BRANCHIND => self.unresolved = true,
            OpCode::CPUI_RETURN => self.ends_flow = true,
            // The `<spaceid>` operand's offset IS the space-manager index
            // (`Varnode::getSpaceFromConst`).
            OpCode::CPUI_STORE => {
                if let Some(v) = vars.first() {
                    self.store_spaces.push(v.offset as int4);
                }
            }
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                match vars.first().and_then(|v| v.space.as_ref()) {
                    // A p-code-relative branch stays inside this instruction.
                    // The walk reads every op of the instruction regardless, so
                    // ignoring it over-approximates the writes -- the safe
                    // direction -- and must NOT end the machine-level flow.
                    Some(sp) if sp.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT => {}
                    Some(sp) => {
                        self.targets.push(Address::new(Rc::clone(sp), vars[0].offset));
                        if opc == OpCode::CPUI_BRANCH {
                            self.ends_flow = true;
                        }
                    }
                    None => self.unresolved = true,
                }
            }
            _ => {}
        }
    }
}

/// Decode a callee body and report the processor-space writes it is *proven* to
/// make (C++ has no analogue; this is the evidence the call-output seam needs
/// and the IR at that seam cannot supply).
///
/// The walk starts at `entry`, follows fall-through and resolved machine branch
/// targets, and stops a path at a `RETURN`. It declares itself **incomplete**
/// -- proving nothing -- on a nested call, an unresolved indirect branch, an
/// undecodable instruction, or the instruction budget, because past any of those
/// the callee could write any register.
pub fn probe_callee_return_writes<T: kuna_sleigh::translate::Translate + ?Sized>(
    tr: &T,
    entry: &Address,
) -> CalleeReturnWrites {
    let mut res =
        CalleeReturnWrites { writes: Vec::new(), store_spaces: Vec::new(), complete: true };
    let Some(entry_space) = entry.get_space() else {
        res.complete = false;
        return res;
    };
    if entry_space.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR {
        res.complete = false;
        return res;
    }
    let mut seen: std::collections::HashSet<(int4, u64)> = std::collections::HashSet::new();
    let mut todo: Vec<Address> = vec![entry.clone()];
    let mut budget = MAX_PROBE_INSTRUCTIONS;
    while let Some(at) = todo.pop() {
        let Some(sp) = at.get_space() else {
            res.complete = false;
            break;
        };
        if !seen.insert((sp.get_index(), at.get_offset())) {
            continue;
        }
        if budget == 0 || res.writes.len() > MAX_PROBE_WRITES {
            res.complete = false;
            break;
        }
        budget -= 1;
        let mut emit = ProbeEmit::default();
        let len = match tr.one_instruction(&mut emit, &at) {
            Ok(n) if n > 0 => n,
            _ => {
                res.complete = false;
                break;
            }
        };
        res.writes.append(&mut emit.writes);
        for sp in emit.store_spaces.drain(..) {
            if !res.store_spaces.contains(&sp) {
                res.store_spaces.push(sp);
            }
        }
        if emit.unresolved {
            res.complete = false;
            break;
        }
        todo.append(&mut emit.targets);
        if !emit.ends_flow {
            todo.push(&at + len as i64);
        }
    }
    if !res.complete {
        res.writes.clear();
        res.store_spaces.clear();
    }
    res
}

/// What a CALL site can conclude about a two-register call output.
///
/// This is the consumer seam's classifier, and it is deliberately *not*
/// [`classify_return_pair`]: the two seams see different things. At the producer
/// the concatenation's halves are values the function computed, so their shape
/// answers the question. At a call there are no callee values in the IR at all
/// -- both halves are INDIRECT creations standing for "the callee may have
/// written this" -- so the shape of the halves says nothing, and the evidence
/// has to come from the prototype model, from the caller's reads, and from the
/// callee body itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallPairRepr {
    /// The model's output rule matched a justified two-register pair, the caller
    /// reads both halves out of the call, and nothing known about the callee
    /// contradicts it.
    ScalarPair,
    /// A bounded decode of the callee proves it never writes the payload half,
    /// so the caller's read of that register is a clobber, not a return value.
    CalleeScalar,
    /// Not a plain two-register call output; keep today's answer.
    Scalar,
}

/// Classify the CALL output the model's `join_dual_class` rule asked for.
///
/// `finalvn` are the used output trials in trial order, least-significant first.
/// `callee_entry` is the direct-call target when the flow build resolved one.
///
/// **What this can prove, and what it cannot.** The vetoes below are the whole
/// list, and they are one-sided: `ScalarPair` is the *absence* of a
/// counter-example, not a positive finding that the callee returns a pair. That
/// positive finding is not available at this seam — see the pass's module
/// header.
pub fn classify_call_output_pair(
    data: &Funcdata,
    finalvn: &[VarnodeId],
    callee_entry: Option<&Address>,
) -> CallPairRepr {
    if finalvn.len() != 2 {
        return CallPairRepr::Scalar;
    }
    let (Some((lo_addr, lo_size)), Some((hi_addr, hi_size))) =
        (register_piece(data, finalvn[0]), register_piece(data, finalvn[1]))
    else {
        return CallPairRepr::Scalar;
    };
    // Two halves of one value are two DIFFERENT, non-overlapping registers.
    let same_space = match (lo_addr.get_space(), hi_addr.get_space()) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        _ => false,
    };
    if same_space
        && lo_addr.get_offset() < hi_addr.get_offset() + hi_size as u64
        && hi_addr.get_offset() < lo_addr.get_offset() + lo_size as u64
    {
        return CallPairRepr::Scalar;
    }
    // The payload half has to be read; a pair nothing consumes is not worth
    // forming and its INDIRECT creation would simply be replaced by a dead one.
    if data.vbank().get(finalvn[1]).map(|v| v.num_descend()).unwrap_or(0) == 0 {
        return CallPairRepr::Scalar;
    }
    // The one piece of callee evidence this seam can obtain: a bounded decode of
    // the callee body that proves the payload register is never written.
    if let Some(entry) = callee_entry {
        if let Some(w) = data.kuna_callee_ret_writes(entry) {
            if w.proves_untouched(&hi_addr, hi_size) {
                return CallPairRepr::CalleeScalar;
            }
        }
    }
    CallPairRepr::ScalarPair
}

/// Take the callee-body probe for every direct call in `data`, before the action
/// pipeline reaches the call-output seam that needs it.
///
/// Called from the driver, which is the last point where the disassembly engine
/// is still reachable: the per-function `ArchContext` handle the pipeline runs
/// against carries the load image but no translator, so the probe cannot be
/// taken lazily at the seam itself. Results are cached on the `Architecture`, so
/// each distinct callee body is decoded once per run rather than once per
/// caller. A no-op unless the rule is live for this image.
pub fn seed_callee_return_writes(
    arch: &mut crate::architecture::Architecture,
    data: &mut Funcdata,
) {
    let mode = RustAbiMode::from_u8(arch.rust_abi);
    let on = match mode {
        RustAbiMode::Off => false,
        RustAbiMode::Auto => arch.source_is_rust,
        RustAbiMode::Always => true,
    };
    if !on {
        return;
    }
    seed_callee_write_probe(arch, data);
}

/// Take (or reuse) the callee-body write probe for every direct call in `data`,
/// without asking whether any particular rule wants it.
///
/// The gate belongs to the caller: this is shared by
/// [`seed_callee_return_writes`] and
/// [`crate::p4_calls::kuna_calleepreserves::seed_callee_preserves`], and the
/// per-image cache means whichever runs first pays for the decode.
pub fn seed_callee_write_probe(
    arch: &mut crate::architecture::Architecture,
    data: &mut Funcdata,
) {
    let mut entries: Vec<Address> = Vec::new();
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
        if !arch.kuna_callee_write_cache.contains_key(&key) {
            let probed = probe_callee_return_writes(arch.translate(), &e);
            arch.kuna_callee_write_cache.insert(key, Rc::new(probed));
        }
        if let Some(w) = arch.kuna_callee_write_cache.get(&key) {
            data.kuna_set_callee_ret_writes(&e, Rc::clone(w));
        }
    }
}

/// Build the call's two-piece output the model asked for (C++
/// `FuncCallSpecs::buildOutputFromTrials`, the `numTrials > 1` arm kuna shipped
/// as a stub).
///
/// `finalvn` are the trial Varnodes in trial order -- least-significant first --
/// each currently the output of an INDIRECT creation sitting just before the
/// CALL; `callee_entry` is the direct-call target the flow build resolved, when
/// there is one. The verdict comes from [`classify_call_output_pair`]. On
/// `ScalarPair` the CALL gains a `join`-space output covering both registers,
/// each half becomes a SUBPIECE of it inserted after the CALL, and the INDIRECT
/// creations are destroyed. Returns `false` (changing nothing) when the gate is
/// off, the classification declines, or the join address cannot be constructed.
pub fn build_call_output_pair(
    callop: OpId,
    data: &mut Funcdata,
    finalvn: &[VarnodeId],
    callee_entry: Option<&Address>,
) -> bool {
    if !live(data) {
        return false;
    }
    if classify_call_output_pair(data, finalvn, callee_entry) != CallPairRepr::ScalarPair {
        return false;
    }
    let (lo, hi) = (finalvn[0], finalvn[1]);
    let Some((lo_addr, lo_size)) = register_piece(data, lo) else { return false };
    let Some((hi_addr, hi_size)) = register_piece(data, hi) else { return false };
    let Some(call_addr) = data.obank().get(callop).map(|o| o.get_addr().clone()) else {
        return false;
    };
    let manage = data.get_arch().manage.clone();
    let Some(joinaddr) = manage.register_lookup().and_then(|rl| {
        manage
            .construct_join_address(rl.as_ref(), &hi_addr, hi_size, &lo_addr, lo_size)
            .ok()
    }) else {
        return false;
    };
    let Ok(whole) = data.new_varnode_out(hi_size + lo_size, &joinaddr, callop) else {
        return false;
    };
    // The concatenation is a rendering of storage the CALL already wrote; it must
    // not become a heritage root of its own (same reason `buildReturnOutput` masks
    // the whole it builds).
    if let Some(v) = data.vbank_mut().get_mut(whole) {
        v.set_write_mask();
    }
    for (piece, off) in [(lo, 0), (hi, lo_size)] {
        let indop = data.vbank().get(piece).and_then(|v| v.get_def());
        let sub = data.new_op(2, call_addr.clone());
        data.op_set_opcode_code(sub, OpCode::CPUI_SUBPIECE);
        let c = data.new_constant(4, off as u64);
        let _ = data.op_set_input(sub, whole, 0);
        let _ = data.op_set_input(sub, c, 1);
        // Steals `piece` from the INDIRECT creation that defined it.
        let _ = data.op_set_output(sub, piece);
        data.op_insert_after(sub, callop);
        if let Some(indop) = indop {
            data.op_destroy(indop);
        }
    }
    true
}

/// The pointer width of this architecture (the default code space's address
/// size), or `None` when the manager has no default code space.
fn pointer_size(data: &Funcdata) -> Option<int4> {
    data.get_arch().manage.get_default_code_space().map(|s| s.get_addr_size() as int4)
}

/// The (address, size) of a Varnode that is a plain register piece defined by an
/// INDIRECT creation at a call, or `None` for any other shape.
fn register_piece(data: &Funcdata, vn: VarnodeId) -> Option<(Address, int4)> {
    let v = data.vbank().get(vn)?;
    if v.get_addr().get_space()?.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR {
        return None;
    }
    let def = v.get_def()?;
    let op = data.obank().get(def)?;
    if op.code() != OpCode::CPUI_INDIRECT || !op.is_indirect_creation() {
        return None;
    }
    Some((v.get_addr().clone(), v.get_size()))
}

#[cfg(test)]
#[path = "kuna_rustabi/tests.rs"]
mod tests;
