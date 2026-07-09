//! Core data model for the Listing/xref tier (design doc `docs/listing-tier-design.md` §2).
//!
//! Three sub-models behind one [`super::Listing`] facade: the instruction model
//! ([`Insn`]/[`FlowType`]/[`FlowKind`]/[`RawOp`]), the cross-reference model
//! ([`Reference`]/[`RefKind`]), and the discovered-function model
//! ([`DiscoveredFunction`]). [`CodeUnit`] is the program-partition projection.
//!
//! These are owned, flat structs produced by the recursive-descent walk
//! ([`super::walk`]); they never borrow the engine. The `FlowType` predicate set
//! is the faithful projection of Ghidra's `FlowType` (`typeop.rs`/`op.rs` flags),
//! derived once from the emitted p-code by [`super::classify::classify`], the
//! lifted copy of `p2_lift/flow.rs::xref_control_flow`.

use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

/// One decoded machine instruction. Owned; produced by the walker.
///
/// `len` is the fall-through byte length the SLEIGH decoder returned from
/// `one_instruction` — it already folds in delay slots (SPARC/MIPS), so
/// `addr + len` is the correct fall-through VMA (design §4.3 gotcha 3).
#[derive(Debug, Clone)]
pub struct Insn {
    /// `getMinAddress` — the instruction's VMA.
    pub addr: u64,
    /// Bytes consumed incl. delay slots (the `one_instruction` return value).
    pub len: u32,
    /// `None` iff `!flow.has_fallthrough` (a terminal instruction).
    pub fall_through: Option<u64>,
    /// Flow class projected from the emitted p-code (NOT a separate API result).
    pub flow: FlowType,
    /// Static control targets (`getFlows`): branch/call destination VMAs.
    pub flows: Vec<u64>,
    /// The decoded mnemonic (from a capturing `AssemblyEmit`).
    pub mnemonic: String,
    /// The decoded operand body — the `body` half of `AssemblyEmit::dump`
    /// (`MOV EDI,0x2` ⇒ mnemonic `MOV`, operands `EDI,0x2`). Populated on the
    /// real-ELF Listing path alongside `mnemonic`; the `noreturn_error` consumer
    /// reads it to recognise a `call error(status,…)` argument as a constant
    /// (edi/rdi = a nonzero literal ⇒ `error` never returns). Empty when the
    /// disassembly emit produced no operands.
    pub operands: String,
    /// Lazy: only `skipNOPS` / `isUsedForCalculation` need the raw ops, so the
    /// common case keeps this `None` (design §2.1 / §8).
    pub pcode: Option<Vec<RawOp>>,
}

/// Faithful projection of Ghidra's `FlowType` predicate set.
///
/// The predicates are derived once from the [`FlowKind`] (which itself is the
/// OpCode → flow-class projection), so this is the same bit-logic Ghidra uses
/// (`op.rs` `pcodeop_flags`), not a re-classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FlowType {
    /// The flow class of the instruction.
    pub kind: FlowKind,
    /// CALL / CALLIND.
    pub is_call: bool,
    /// BRANCH / CBRANCH / BRANCHIND (any branch).
    pub is_jump: bool,
    /// RETURN (no fall-through, no static target).
    pub is_terminal: bool,
    /// Indirect/computed target (BRANCHIND / CALLIND).
    pub is_computed: bool,
    /// Indirect target (BRANCHIND / CALLIND).
    pub is_indirect: bool,
    /// CBRANCH (conditional — also falls through).
    pub is_conditional: bool,
    /// `true` unless the LAST op is BRANCH / BRANCHIND / RETURN (design §4.3).
    pub has_fallthrough: bool,
}

/// OpCode → flow-class projection (mirrors `typeop.rs` / `op.rs` flow flags).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlowKind {
    /// Emits ops; last op is not a flow break. Falls through.
    #[default]
    Fallthrough,
    /// Last op `CPUI_BRANCH` with a non-constant `in0`. No fall-through.
    UnconditionalBranch,
    /// `CPUI_CBRANCH` — also falls through.
    ConditionalBranch,
    /// `CPUI_BRANCHIND` — indirect, target unresolved. No fall-through.
    ComputedJump,
    /// `CPUI_CALL` — falls through.
    Call,
    /// `CPUI_CALLIND` — indirect, falls through.
    ComputedCall,
    /// `CPUI_CALLOTHER` — syscall/userop, falls through.
    CallOther,
    /// `CPUI_RETURN` — terminal.
    Return,
    /// Decode produced no flow effect at all (rare; treated as fall-through).
    Unimplemented,
}

/// Minimal captured p-code op (materialized only when [`Insn::pcode`] is set).
///
/// Carries just enough for a future `skipNOPS` / `isUsedForCalculation`
/// consumer: the opcode and the first input varnode.
#[derive(Debug, Clone)]
pub struct RawOp {
    /// The op's opcode.
    pub opcode: OpCode,
    /// The first input varnode, if any (`in0`).
    pub in0: Option<VarnodeData>,
}

/// The kind of a cross-reference edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefKind {
    /// A CALL/CALLIND edge (callsite → callee entry).
    Call,
    /// A branch/jump/fall-through edge (intra-function control flow).
    Code,
    /// A data reference (reserved; not populated by the keystone — design §2.2).
    Data,
    /// A read of a data location (reserved).
    Read,
    /// A write to a data location (reserved).
    Write,
}

/// A cross-reference edge `from -> to`.
///
/// The keystone populates only [`RefKind::Call`] and [`RefKind::Code`] (control
/// flow); the data kinds and `op_index` are present for faithfulness (so the
/// type never changes if operand markup is ever revisited) — design §2.2.
#[derive(Debug, Clone)]
pub struct Reference {
    /// Source VMA (the referencing instruction).
    pub from: u64,
    /// Target VMA (the referenced location).
    pub to: u64,
    /// The edge kind.
    pub kind: RefKind,
    /// Operand index (reserved for operand markup; mostly `None`).
    pub op_index: Option<u8>,
}

/// A function entry discovered (or seeded) by the walk.
#[derive(Debug, Clone)]
pub struct DiscoveredFunction {
    /// The function entry VMA.
    pub entry: u64,
    /// Name from a funcsym / entry-name overlay; `None` ⇒ `sub_<addr>`.
    pub name: Option<String>,
    /// `true` iff seeded by a real funcsym (vs discovered via a CALL target).
    pub from_symbol: bool,
    /// Seeded from the no-return Known-list (refined by a future consumer).
    pub has_no_return: bool,
    /// From `callfixup` facts (skip-modeled-callees); `None` for the keystone.
    pub call_fixup: Option<String>,
}

/// The code/data partition class of a VMA (design §2.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeUnit {
    /// The start of a decoded instruction (carries the instruction's VMA).
    Instruction(u64),
    /// A typed data record (carries `(addr, len)`).
    Data(u64, u32),
    /// Neither code nor typed data inside an executable range.
    Undefined,
}
