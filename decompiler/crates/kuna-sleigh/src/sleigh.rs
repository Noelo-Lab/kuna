//! Port of `decompiler/cpp/sleigh.{hh,cc}` + the parser pieces of
//! `context.{hh,cc}` (W2, item `w2-sleigh-core`): the main SLEIGH decode
//! engine.
//!
//! What lives here:
//!
//! - [`ConstructState`] / [`ContextSet`] / [`ParserContext`] / [`ParserWalker`]
//!   / [`ParserWalkerChange`] from `context.{hh,cc}` (context.rs deferred them
//!   here — see that module's docs — because they are built around the symbol
//!   table and the engine).  The C++ pointer tree becomes an index-based arena
//!   (ADR 0001): `ConstructState` nodes live in `ParserContext::state`, child
//!   links are `Option<usize>` indices, the `ParserWalker` carries a node
//!   index plus the breadcrumb path.
//! - [`PcodeCacher`] / [`DisassemblyCache`] / [`SleighBuilder`] / [`Sleigh`]
//!   from `sleigh.{hh,cc}`.
//!
//! The walker implements the `SymbolWalker`/`SymbolWalkerChange`/
//! `PatternExpressionContext` hooks (slghsymbol.rs / slghpatexpress.rs):
//! constructor resolution returns a `ConstructorRef`, and the walker borrows
//! the [`SymbolTable`] so it can navigate constructors/operands during a walk.
//!
//! Interior mutability: the C++ `Translate::oneInstruction`/`instructionLength`
//! are `const` but mutate caches and read the load image through pointers; the
//! Rust [`Sleigh`] keeps the load image, context database, context cache and
//! disassembly/p-code caches behind `RefCell` so the trait methods stay `&self`
//! (ADR 0004 reserves panics for invariant violations — borrow conflicts here
//! are invariant violations, since the engine is single-threaded and never
//! re-enters a cache mid-borrow).

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, RegisterLookup, VarnodeStorage};
use kuna_base::xml::DocumentStorage;

use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::FixedHandle;
use crate::globalcontext::{ContextCache, ContextDatabase};
use crate::loadimage::LoadImage;
use crate::semantics::{ConstructTpl, OpTpl, PcodeBuilder, VField, VarnodeTpl};
use crate::sleighbase::SleighBase;
use crate::slghpatexpress::{PatternExpression, PatternExpressionContext};
use crate::slghpattern::DisjointPattern;
use crate::slghsymbol::{
    ConstructorRef, SymbolKind, SymbolTable, SymbolType, SymbolWalker, SymbolWalkerChange,
};
use crate::translate::{
    storage_from_varnode_data, AssemblyEmit, PcodeEmit, Translate, TranslateBase, UniqueLayout,
};

// ---------------------------------------------------------------------------
// ConstructState / ContextSet (context.hh/.cc)
// ---------------------------------------------------------------------------

/// C++ `ParserContext::MAX_DEPTH`.
const MAX_DEPTH: i32 = 32;
/// C++ `ParserContext::MAX_OPERAND`.
const MAX_OPERAND: i32 = 20;
/// C++ `ParserContext::MAX_INSTRUCTION_LEN`.
const MAX_INSTRUCTION_LEN: i32 = 16;
/// C++ `ParserContext::INITIAL_STATE_NUM`.
const INITIAL_STATE_NUM: i32 = 64;
/// C++ `ParserContext::STATE_GROWTH`.
const STATE_GROWTH: i32 = 64;

/// C++ `ConstructState`: a node in the subconstructor tree.  Pointers become
/// arena indices into [`ParserContext::state`] (ADR 0001).
#[derive(Debug, Clone)]
struct ConstructState {
    /// The matched constructor (C++ `Constructor *ct`, null until resolved).
    ct: Option<ConstructorRef>,
    /// Resolved Varnode associated with the constructor (C++ `hand`).
    hand: FixedHandle,
    /// Child node indices (C++ `ConstructState **resolve`; `None` = null).
    resolve: Vec<Option<usize>>,
    /// Parent node index (C++ `parent`).
    parent: Option<usize>,
    /// Length of this instantiation (C++ `length`).
    length: i32,
    /// Absolute offset from start of instruction (C++ `offset`).
    offset: u32,
    /// kuna-only (not in C++): the specific `DisjointPattern` leaf whose
    /// `is_match` succeeded when this node's constructor was chosen during
    /// decode.  Captured at the resolution point, where the per-node context
    /// (the multi-phase parser context — REX/prefix `instrPhase` etc.) is the
    /// one that actually selected the constructor.  Read back by
    /// [`Sleigh::instruction_mask`] to compute the fixed-bit mask without
    /// re-walking the decision tree post-decode (the post-decode re-walk reads
    /// the *final* context and so misresolves multi-phase encodings — the FID
    /// PR1 bug this captures around).  Purely additive: it retains a pattern
    /// the decode already computed and never affects which constructor is
    /// chosen, the length, the handles, or any p-code.
    matched_pattern: Option<DisjointPattern>,
}

impl ConstructState {
    /// C++ `ConstructState(int4 numOperands)`.
    fn with_operands(num_operands: usize) -> ConstructState {
        ConstructState {
            ct: None,
            hand: FixedHandle::default(),
            resolve: vec![None; num_operands],
            parent: None,
            length: 0,
            offset: 0,
            matched_pattern: None,
        }
    }
}

/// C++ `ContextSet`: a command for globally setting a SLEIGH context value.
#[derive(Debug, Clone)]
struct ContextSet {
    /// Symbol resolving to address where setting takes effect (symbol id).
    sym: u32,
    /// Index of the node at which context set was made (C++ `point`).
    point: usize,
    /// Index of the specific context word affected (C++ `num`).
    num: i32,
    /// Bits within word affected (C++ `mask`).
    mask: u32,
    /// New setting for bits (C++ `value`).
    value: u32,
    /// Does the new context flow from its set point (C++ `flow`).
    flow: bool,
}

// ---------------------------------------------------------------------------
// ParserContext (context.hh/.cc)
// ---------------------------------------------------------------------------

/// C++ `ParserContext::parse_state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseState {
    /// Instruction has not been parsed at all.
    Uninitialized = 0,
    /// Instruction is parsed in preparation for disassembly.
    Disassembly = 1,
    /// Instruction is parsed in preparation for generating p-code.
    Pcode = 2,
}

/// C++ `ParserContext`: context maintained while parsing a single instruction.
pub struct ParserContext {
    /// Overall state of the parse (C++ `parsestate`).
    parsestate: ParseState,
    /// Address space for constants (C++ `const_space`).
    const_space: Option<Rc<AddrSpace>>,
    /// Buffer of instruction bytes (C++ `buf[MAX_INSTRUCTION_LEN]`).
    buf: [u8; MAX_INSTRUCTION_LEN as usize],
    /// Local context words (C++ `context`/`contextsize`).
    context: Vec<u32>,
    /// Changes to SLEIGH context slated by this instruction (C++
    /// `contextcommit`).
    contextcommit: Vec<ContextSet>,
    /// Address of start of instruction (C++ `addr`).
    addr: Address,
    /// Address of next instruction (C++ `naddr`).
    naddr: Address,
    /// Address of instruction after next (C++ `n2addr`, mutable/lazy).
    n2addr: Address,
    /// For injections, address of the call being overridden (C++ `calladdr`).
    calladdr: Address,
    /// Available nodes for the constructor tree (C++ `state`).
    state: Vec<ConstructState>,
    /// Root node index of the constructor tree (C++ `base_state`).
    base_state: usize,
    /// Number of unallocated `ConstructState` nodes remaining (C++ `alloc`).
    alloc: i32,
    /// Delayslot depth (C++ `delayslot`).
    delayslot: i32,
}

impl ParserContext {
    /// C++ `ParserContext(ContextCache *ccache,Translate *trans)`.  The
    /// context size is taken from the engine's database; the const space and
    /// state arena are filled by `initialize`.
    fn new(contextsize: i32) -> ParserContext {
        ParserContext {
            parsestate: ParseState::Uninitialized,
            const_space: None,
            buf: [0u8; MAX_INSTRUCTION_LEN as usize],
            context: vec![0u32; contextsize.max(0) as usize],
            contextcommit: Vec::new(),
            addr: Address::new_invalid(),
            naddr: Address::new_invalid(),
            n2addr: Address::new_invalid(),
            calladdr: Address::new_invalid(),
            state: Vec::new(),
            base_state: 0,
            alloc: 0,
            delayslot: 0,
        }
    }

    /// C++ `ParserContext::initialize`.
    fn initialize(&mut self, spc: Rc<AddrSpace>, maxstate: i32) {
        self.const_space = Some(spc);
        let n = maxstate as usize; // maxstate is a positive constant
        self.state = (0..n)
            .map(|_| ConstructState::with_operands(MAX_OPERAND as usize))
            .collect();
        self.base_state = n - 1;
    }

    /// C++ `getBuffer` write target.
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }

    /// C++ `getParserState`.
    pub fn get_parser_state(&self) -> ParseState {
        self.parsestate
    }

    /// C++ `setParserState`.
    fn set_parser_state(&mut self, st: ParseState) {
        self.parsestate = st;
    }

    /// C++ `setAddr` (resets the lazy `n2addr`).
    fn set_addr(&mut self, ad: Address) {
        self.addr = ad;
        self.n2addr = Address::new_invalid();
    }

    /// C++ `setNaddr`.
    fn set_naddr(&mut self, ad: Address) {
        self.naddr = ad;
    }

    /// C++ `getAddr`.
    pub fn get_addr(&self) -> &Address {
        &self.addr
    }

    /// C++ `getNaddr`.
    fn get_naddr(&self) -> &Address {
        &self.naddr
    }

    /// C++ `getCurSpace`.
    fn get_cur_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.addr.get_space().expect("instruction address has a space"))
    }

    /// C++ `getConstSpace`.
    fn get_const_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.const_space.as_ref().expect("ParserContext const space initialized"))
    }

    /// C++ `getLength`: `base_state->length`.
    pub fn get_length(&self) -> i32 {
        self.state[self.base_state].length
    }

    /// C++ `setDelaySlot`.
    fn set_delay_slot(&mut self, val: i32) {
        self.delayslot = val;
    }

    /// C++ `getDelaySlot`.
    pub fn get_delay_slot(&self) -> i32 {
        self.delayslot
    }

    /// C++ `clearCommits`.
    fn clear_commits(&mut self) {
        self.contextcommit.clear();
    }

    /// C++ `setContextWord(int4 i,uintm val,uintm mask)`.
    fn set_context_word(&mut self, i: i32, val: u32, mask: u32) {
        let idx = i as usize; // i is a valid word index
        // C++: context[i] = (context[i]&(~mask))|(mask&val)
        self.context[idx] = (self.context[idx] & !mask) | (mask & val);
    }

    /// C++ `getInstructionBytes(int4 bytestart,int4 size,uint4 off)`.
    fn get_instruction_bytes(&self, bytestart: i32, size: i32, off: u32) -> KunaResult<u32> {
        // off += bytestart
        let off = off.wrapping_add(bytestart as u32); // C++ uint4 += int4
        // C++ `if (off >= MAX_INSTRUCTION_LEN)`: uint4 vs int4 (off >= 0)
        if off >= MAX_INSTRUCTION_LEN as u32 {
            return Err(KunaError::bad_data(format!(
                "Instruction is using more than {MAX_INSTRUCTION_LEN} bytes"
            )));
        }
        let base = off as usize;
        let mut res: u32 = 0;
        for i in 0..size {
            // C++ reads ptr[i] beyond the buffer if off+size > 16 (UB);
            // the guard above only checks the start byte, matching C++.
            let b = self.buf.get(base + i as usize).copied().ok_or_else(|| {
                KunaError::bad_data("Instruction byte read past buffer (C++ UB)")
            })?;
            res <<= 8; // uintm << 8
            res |= u32::from(b);
        }
        Ok(res)
    }

    /// C++ `getInstructionBits(int4 startbit,int4 size,uint4 off)`.
    fn get_instruction_bits(&self, startbit: i32, size: i32, off: u32) -> KunaResult<u32> {
        // off += (startbit/8)
        let off = off.wrapping_add((startbit / 8) as u32);
        if off >= MAX_INSTRUCTION_LEN as u32 {
            return Err(KunaError::bad_data(format!(
                "Instruction is using more than {MAX_INSTRUCTION_LEN} bytes"
            )));
        }
        let base = off as usize;
        let startbit = startbit % 8;
        let bytesize = (startbit + size - 1) / 8 + 1;
        let mut res: u32 = 0;
        for i in 0..bytesize {
            let b = self.buf.get(base + i as usize).copied().ok_or_else(|| {
                KunaError::bad_data("Instruction bit read past buffer (C++ UB)")
            })?;
            res <<= 8;
            res |= u32::from(b);
        }
        // res <<= 8*(sizeof(uintm)-bytesize)+startbit
        // res >>= 8*sizeof(uintm)-size
        // sizeof(uintm)==4.  Out-of-range shifts are C++ UB and resolve
        // x86-masked (mod 32) via wrapping_shl/shr (ADR 0003).
        let lshift = (8 * (4 - bytesize) + startbit) as u32;
        let rshift = (8 * 4 - size) as u32;
        res = res.wrapping_shl(lshift);
        res = res.wrapping_shr(rshift);
        Ok(res)
    }

    /// C++ `getContextBytes(int4 bytestart,int4 size)`.
    fn get_context_bytes(&self, bytestart: i32, size: i32) -> KunaResult<u32> {
        // sizeof(uintm) == 4
        let mut intstart = (bytestart / 4) as usize;
        let mut res = *self
            .context
            .get(intstart)
            .ok_or_else(|| KunaError::bad_data("context word out of range"))?;
        let byte_offset = bytestart % 4;
        let unused_bytes = 4 - size;
        // res <<= byteOffset*8; res >>= unusedBytes*8 (counts in [0,31] for
        // valid sizes; UB for size>4 -> x86 mask, ADR 0003)
        res = res.wrapping_shl((byte_offset * 8) as u32);
        res = res.wrapping_shr((unused_bytes * 8) as u32);
        let remaining = size - 4 + byte_offset;
        intstart += 1;
        if remaining > 0 && intstart < self.context.len() {
            let mut res2 = self.context[intstart];
            let unused = 4 - remaining;
            res2 = res2.wrapping_shr((unused * 8) as u32);
            res |= res2;
        }
        Ok(res)
    }

    /// C++ `getContextBits(int4 startbit,int4 size)`.
    fn get_context_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
        let mut intstart = (startbit / 32) as usize;
        let mut res = *self
            .context
            .get(intstart)
            .ok_or_else(|| KunaError::bad_data("context word out of range"))?;
        let bit_offset = startbit % 32;
        let unused_bits = 32 - size;
        res = res.wrapping_shl(bit_offset as u32);
        res = res.wrapping_shr(unused_bits as u32);
        let remaining = size - 32 + bit_offset;
        intstart += 1;
        if remaining > 0 && intstart < self.context.len() {
            let mut res2 = self.context[intstart];
            let unused = 32 - remaining;
            res2 = res2.wrapping_shr(unused as u32);
            res |= res2;
        }
        Ok(res)
    }

    /// C++ `addCommit`.
    fn add_commit(&mut self, sym: u32, num: i32, mask: u32, flow: bool, point: usize) {
        let value = self.context[num as usize] & mask;
        self.contextcommit.push(ContextSet { sym, point, num, mask, value, flow });
    }

    /// C++ `expandState`: prepend `amount` new nodes.
    fn expand_state(&mut self, amount: i32) {
        let amount = amount.max(0) as usize;
        let mut newnodes: Vec<ConstructState> = (0..amount)
            .map(|_| ConstructState::with_operands(MAX_OPERAND as usize))
            .collect();
        // C++ `state.insert(state.begin(),amount,...)` shifts every existing
        // node up by `amount`; the parent/resolve/base_state indices must be
        // rebased.
        newnodes.append(&mut self.state);
        self.state = newnodes;
        for node in &mut self.state {
            if let Some(p) = node.parent {
                node.parent = Some(p + amount);
            }
            for child in &mut node.resolve {
                if let Some(c) = *child {
                    *child = Some(c + amount);
                }
            }
        }
        self.base_state += amount;
        self.alloc += amount as i32;
    }
}

// (continued in subsequent edits)

// ---------------------------------------------------------------------------
// ParserWalker / ParserWalkerChange (context.hh/.cc)
// ---------------------------------------------------------------------------

/// Mutable walk state (C++ `ParserWalker::point/depth/breadcrumb`).  Split out
/// so the immutable [`ParserWalker`] and the mutating [`ParserWalkerChange`]
/// share one cursor representation.
#[derive(Debug, Clone)]
struct WalkCursor {
    /// Current node index (C++ `point`, `None` = null/end of walk).
    point: Option<usize>,
    /// Depth of the current node (C++ `depth`).
    depth: i32,
    /// Path of operands from root (C++ `breadcrumb[MAX_DEPTH]`).
    breadcrumb: [i32; (MAX_DEPTH + 1) as usize],
}

impl WalkCursor {
    fn new() -> WalkCursor {
        WalkCursor { point: None, depth: 0, breadcrumb: [0; (MAX_DEPTH + 1) as usize] }
    }
}

/// C++ `ParserWalker`: a read-only walk over the constructor tree, plus the
/// `SymbolWalker`/`PatternExpressionContext` boundary implementations.  Borrows
/// the [`ParserContext`], the [`SymbolTable`], and the [`Sleigh`] engine (the
/// last only for `inst_next2` length computation).
struct ParserWalker<'a> {
    ctx: &'a ParserContext,
    cross: Option<&'a ParserContext>,
    table: &'a SymbolTable,
    engine: &'a Sleigh,
    cur: WalkCursor,
}

impl<'a> ParserWalker<'a> {
    /// C++ `ParserWalker(const ParserContext *c)`.
    fn new(ctx: &'a ParserContext, table: &'a SymbolTable, engine: &'a Sleigh) -> ParserWalker<'a> {
        ParserWalker { ctx, cross: None, table, engine, cur: WalkCursor::new() }
    }

    /// C++ `baseState`.
    fn base_state(&mut self) {
        self.cur.point = Some(self.ctx.base_state);
        self.cur.depth = 0;
        self.cur.breadcrumb[0] = 0;
    }

    /// Current point index (panics on null — C++ would deref a null pointer).
    fn point(&self) -> usize {
        self.cur.point.expect("ParserWalker: point is null (C++ UB)")
    }

    /// C++ `pushOperand(int4 i)`.
    fn push_operand_inner(&mut self, i: i32) -> KunaResult<()> {
        if self.cur.depth > MAX_DEPTH - 2 {
            return Err(KunaError::lowlevel("SLEIGH exceeded maximum parse depth"));
        }
        self.cur.breadcrumb[self.cur.depth as usize] = i + 1;
        self.cur.depth += 1;
        let node = &self.ctx.state[self.point()];
        self.cur.point = node.resolve[i as usize];
        self.cur.breadcrumb[self.cur.depth as usize] = 0;
        Ok(())
    }

    /// C++ `popOperand`.
    fn pop_operand_inner(&mut self) {
        let node = &self.ctx.state[self.point()];
        self.cur.point = node.parent;
        self.cur.depth -= 1;
    }

    /// C++ `getConstructor`: identify the current node's constructor.
    fn get_constructor_inner(&self) -> KunaResult<ConstructorRef> {
        self.ctx.state[self.point()]
            .ct
            .ok_or_else(|| KunaError::sleigh("ParserWalker: no constructor at current node"))
    }

    /// C++ `getFixedHandle(int4 i)`: handle of child operand `i`.
    fn get_fixed_handle_inner(&self, i: i32) -> KunaResult<FixedHandle> {
        let pt = &self.ctx.state[self.point()];
        let child = pt.resolve[i as usize]
            .ok_or_else(|| KunaError::sleigh("ParserWalker: operand not resolved"))?;
        Ok(self.ctx.state[child].hand.clone())
    }

    /// The active context for the address-fetching accessors (C++ uses the
    /// cross context if present, else the main one).
    fn addr_ctx(&self) -> &ParserContext {
        self.cross.unwrap_or(self.ctx)
    }

    /// C++ `setOutOfBandState`: simulate a single-node tree so a TokenField
    /// behaves as if just parsed (used by the `operand_value` hook).  Returns
    /// the temp node placed into a scratch slot of the context-local arena —
    /// here, since the walker borrows the context immutably, the temp node is
    /// returned by value and stored on `self` via [`OobWalker`].
    fn out_of_band(
        &self,
        ct: ConstructorRef,
        index: i32,
    ) -> KunaResult<OobState> {
        // Walk back from the current point to the node whose ct == ct.
        let mut pt = self.point();
        let mut curdepth = self.cur.depth;
        while self.ctx.state[pt].ct != Some(ct) {
            if curdepth <= 0 {
                // C++ returns with point unchanged (a degenerate walk).
                return Ok(OobState { offset: 0, ct, length: 0, valid: false });
            }
            curdepth -= 1;
            pt = self.ctx.state[pt]
                .parent
                .ok_or_else(|| KunaError::sleigh("out-of-band: null parent"))?;
        }
        let sym = self.table.get_constructor(ct)?.get_operand(index)?;
        let opsym = self.table.find_symbol_by_id(sym).ok_or_else(|| {
            KunaError::sleigh("out-of-band: operand symbol undefined")
        })?;
        let SymbolKind::Operand(op) = opsym.kind() else {
            return Err(KunaError::sleigh("out-of-band: not an operand symbol"));
        };
        let i = op.get_offset_base();
        let offset = if i < 0 {
            // offset is constructor-relative; build it explicitly.
            self.ctx.state[pt].offset.wrapping_add(op.get_relative_offset())
        } else {
            let child = self.ctx.state[pt].resolve[index as usize]
                .ok_or_else(|| KunaError::sleigh("out-of-band: operand not resolved"))?;
            self.ctx.state[child].offset
        };
        Ok(OobState { offset, ct, length: self.ctx.state[pt].length, valid: true })
    }
}

/// The simulated single-node tree state produced by [`ParserWalker::out_of_band`]
/// (C++ `setOutOfBandState`'s `tempstate`).  `ct`/`length` are recorded
/// faithfully (the C++ tempstate sets `tempstate->ct`/`tempstate->length`); the
/// pattern leaves evaluated against an out-of-band walker — TokenField,
/// ContextField — read only the synthetic offset, so those two fields are
/// carried but not consulted here.
#[allow(dead_code)] // ct/length mirror the C++ tempstate (see doc comment)
#[derive(Debug, Clone)]
struct OobState {
    offset: u32,
    ct: ConstructorRef,
    length: i32,
    valid: bool,
}


impl PatternExpressionContext for ParserWalker<'_> {
    fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        let off = self.ctx.state[self.point()].offset;
        self.ctx.get_instruction_bytes(byteoff, numbytes, off)
    }
    fn get_context_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        self.ctx.get_context_bytes(byteoff, numbytes)
    }
    fn get_addr(&self) -> Address {
        self.addr_ctx().get_addr().clone()
    }
    fn get_naddr(&self) -> Address {
        self.addr_ctx().get_naddr().clone()
    }
    fn get_n2addr(&self) -> KunaResult<Address> {
        self.engine.compute_n2addr(self.addr_ctx())
    }
    fn operand_value(&self, index: i32, table_id: u32, ct_id: u32) -> KunaResult<i64> {
        // C++ OperandValue::getValue (via setOutOfBandState):
        let ct = ConstructorRef { table_id, ct_id };
        let sym_id = self.table.get_constructor(ct)?.get_operand(index)?;
        let opsym = self
            .table
            .find_symbol_by_id(sym_id)
            .ok_or_else(|| KunaError::sleigh("operand_value: operand symbol undefined"))?;
        let SymbolKind::Operand(op) = opsym.kind() else {
            return Err(KunaError::sleigh("operand_value: not an operand symbol"));
        };
        // patexp = sym->getDefiningExpression(); if null, the defining
        // symbol's pattern expression; if still null, return 0.
        let patexp: PatternExpression = match op.get_defining_expression() {
            Some(pe) => pe.clone(),
            None => match op.get_defining_symbol() {
                Some(defid) => {
                    let defsym = self.table.find_symbol_by_id(defid).ok_or_else(|| {
                        KunaError::sleigh("operand_value: defining symbol undefined")
                    })?;
                    match defsym.get_pattern_expression()? {
                        Some(pe) => pe,
                        None => return Ok(0),
                    }
                }
                None => return Ok(0),
            },
        };
        let oob = self.out_of_band(ct, index)?;
        if !oob.valid {
            // C++ leaves point unchanged; evaluating then reads the original
            // point's offset.  Fall back to the current node.
            let fallback = OobWalker {
                ctx: self.ctx,
                cross: self.cross,
                engine: self.engine,
                state: OobState {
                    offset: self.ctx.state[self.point()].offset,
                    ct,
                    length: self.ctx.state[self.point()].length,
                    valid: true,
                },
            };
            return patexp.get_value(&fallback);
        }
        let oobwalker = OobWalker {
            ctx: self.ctx,
            cross: self.cross,
            engine: self.engine,
            state: oob,
        };
        patexp.get_value(&oobwalker)
    }
}

impl SymbolWalker for ParserWalker<'_> {
    fn push_operand(&mut self, i: i32) -> KunaResult<()> {
        self.push_operand_inner(i)
    }
    fn pop_operand(&mut self) -> KunaResult<()> {
        self.pop_operand_inner();
        Ok(())
    }
    fn get_constructor(&self) -> KunaResult<ConstructorRef> {
        self.get_constructor_inner()
    }
    fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle> {
        self.get_fixed_handle_inner(i)
    }
    fn get_const_space(&self) -> Rc<AddrSpace> {
        self.ctx.get_const_space()
    }
    fn get_cur_space(&self) -> Rc<AddrSpace> {
        self.addr_ctx().get_cur_space()
    }
    fn get_dest_addr(&self) -> KunaResult<Address> {
        Ok(self.addr_ctx().calladdr.clone())
    }
    fn get_ref_addr(&self) -> KunaResult<Address> {
        Ok(self.addr_ctx().calladdr.clone())
    }
    fn get_instruction_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
        let off = self.ctx.state[self.point()].offset;
        self.ctx.get_instruction_bits(startbit, size, off)
    }
    fn get_context_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
        self.ctx.get_context_bits(startbit, size)
    }
}

/// C++ `ParserWalkerChange`: a [`ParserWalker`] that can modify the tree as the
/// instruction is parsed (`Sleigh::resolve`).  Holds the [`ParserContext`]
/// mutably; reads share the immutable `ParserWalker` body via [`as_reader`].
struct ParserWalkerChange<'a> {
    ctx: &'a mut ParserContext,
    table: &'a SymbolTable,
    engine: &'a Sleigh,
    cur: WalkCursor,
}

impl<'a> ParserWalkerChange<'a> {
    fn new(
        ctx: &'a mut ParserContext,
        table: &'a SymbolTable,
        engine: &'a Sleigh,
    ) -> ParserWalkerChange<'a> {
        ParserWalkerChange { ctx, table, engine, cur: WalkCursor::new() }
    }

    /// Build a read-only [`ParserWalker`] view sharing this cursor (for the
    /// boundary methods that only read — `applyContext`/`resolve` evaluation).
    fn as_reader(&self) -> ParserWalker<'_> {
        ParserWalker {
            ctx: self.ctx,
            cross: None,
            table: self.table,
            engine: self.engine,
            cur: self.cur.clone(),
        }
    }

    fn point(&self) -> usize {
        self.cur.point.expect("ParserWalkerChange: null point (C++ UB)")
    }

    /// C++ `baseState`.
    fn base_state(&mut self) {
        self.cur.point = Some(self.ctx.base_state);
        self.cur.depth = 0;
        self.cur.breadcrumb[0] = 0;
    }

    fn is_state(&self) -> bool {
        self.cur.point.is_some()
    }

    /// C++ `getConstructor`.
    fn get_constructor(&self) -> KunaResult<ConstructorRef> {
        self.ctx.state[self.point()]
            .ct
            .ok_or_else(|| KunaError::sleigh("no constructor at current node"))
    }

    /// C++ `getOperand`.
    fn get_operand(&self) -> i32 {
        self.cur.breadcrumb[self.cur.depth as usize]
    }

    /// C++ `ParserWalker::getOffset(int4 i)`.
    fn get_offset(&self, i: i32) -> u32 {
        let pt = &self.ctx.state[self.point()];
        if i < 0 {
            pt.offset
        } else {
            let op_idx = pt.resolve[i as usize].expect("operand resolved (C++ deref)");
            let op = &self.ctx.state[op_idx];
            op.offset.wrapping_add(op.length as u32)
        }
    }

    /// C++ `ParserWalkerChange::setOffset`.
    fn set_offset(&mut self, off: u32) {
        let p = self.point();
        self.ctx.state[p].offset = off;
    }

    /// C++ `ParserWalkerChange::setConstructor`.
    fn set_constructor(&mut self, c: ConstructorRef) {
        let p = self.point();
        self.ctx.state[p].ct = Some(c);
    }

    /// kuna-only: stash the matched `DisjointPattern` leaf for the current
    /// node, captured at decode time (correct per-node context).  Read back by
    /// [`Sleigh::instruction_mask`].  Additive — no decode effect.
    fn set_matched_pattern(&mut self, pat: DisjointPattern) {
        let p = self.point();
        self.ctx.state[p].matched_pattern = Some(pat);
    }

    /// C++ `ParserWalkerChange::setCurrentLength`.
    fn set_current_length(&mut self, len: i32) {
        let p = self.point();
        self.ctx.state[p].length = len;
    }

    /// C++ `pushOperand`.
    fn push_operand(&mut self, i: i32) -> KunaResult<()> {
        if self.cur.depth > MAX_DEPTH - 2 {
            return Err(KunaError::lowlevel("SLEIGH exceeded maximum parse depth"));
        }
        let parent = self.point(); // point->resolve[i] read with the OLD point
        let child = self.ctx.state[parent].resolve[i as usize];
        self.cur.breadcrumb[self.cur.depth as usize] = i + 1;
        self.cur.depth += 1;
        self.cur.point = child;
        self.cur.breadcrumb[self.cur.depth as usize] = 0;
        Ok(())
    }

    /// C++ `popOperand`.
    fn pop_operand(&mut self) {
        let parent = self.ctx.state[self.point()].parent;
        self.cur.point = parent;
        self.cur.depth -= 1;
    }

    /// C++ `ParserContext::deallocateState`.
    fn deallocate_state(&mut self) {
        // alloc = state.size() - 2
        self.ctx.alloc = self.ctx.state.len() as i32 - 2;
        self.base_state();
    }

    /// C++ `ParserContext::allocateOperand`.
    fn allocate_operand(&mut self, i: i32) -> KunaResult<()> {
        if i >= MAX_OPERAND {
            return Err(KunaError::lowlevel("SLEIGH parser out of state space"));
        }
        if self.ctx.alloc < 0 {
            // C++ `ParserContext::expandState` does `state.insert(state.begin(),
            // amount, ...)`, front-inserting `amount` nodes. In C++ the walker's
            // `point`/`parent`/`resolve`/`base_state` are raw pointers into the
            // heap `ConstructState` objects, so this vector reshuffle never
            // invalidates them. kuna models those references as Vec indices, so
            // `expand_state` already rebases the ones it owns (parent/resolve/
            // base_state/alloc) by `amount`. The live walker cursor `self.cur.
            // point` is the one index it can't reach — rebase it here so the
            // in-progress parse (deep operand trees, e.g. ARM NEON `{d16-d31}`
            // reg lists) keeps pointing at the same node after the growth.
            let before = self.ctx.state.len();
            self.ctx.expand_state(STATE_GROWTH);
            let amount = self.ctx.state.len() - before;
            if let Some(p) = self.cur.point {
                self.cur.point = Some(p + amount);
            }
        }
        let opstate = self.ctx.alloc as usize;
        self.ctx.alloc -= 1;
        let parent = self.point();
        self.ctx.state[opstate].parent = Some(parent);
        self.ctx.state[opstate].ct = None;
        self.ctx.state[parent].resolve[i as usize] = Some(opstate);
        if self.cur.depth > MAX_DEPTH - 2 {
            return Err(KunaError::lowlevel("SLEIGH exceeded maximum parse depth"));
        }
        self.cur.breadcrumb[self.cur.depth as usize] += 1;
        self.cur.depth += 1;
        self.cur.point = Some(opstate);
        self.cur.breadcrumb[self.cur.depth as usize] = 0;
        Ok(())
    }

    /// C++ `ParserWalkerChange::calcCurrentLength`.
    fn calc_current_length(&mut self, length: i32, numopers: i32) {
        let p = self.point();
        let mut length = length + self.ctx.state[p].offset as i32; // C++ length += point->offset
        for i in 0..numopers {
            let sub = self.ctx.state[p].resolve[i as usize].expect("operand resolved");
            let sublength = self.ctx.state[sub].length + self.ctx.state[sub].offset as i32;
            if sublength > length {
                length = sublength;
            }
        }
        self.ctx.state[p].length = length - self.ctx.state[p].offset as i32;
    }
}


/// The out-of-band single-node walker built by [`ParserWalker::out_of_band`]
/// (C++ `setOutOfBandState`'s `tempstate` + a fresh `ParserWalker`).  Only the
/// `PatternExpressionContext` surface is needed: a `TokenField`/`ContextField`
/// reads instruction/context bytes against the synthetic node's offset.
struct OobWalker<'a> {
    ctx: &'a ParserContext,
    cross: Option<&'a ParserContext>,
    engine: &'a Sleigh,
    state: OobState,
}

impl PatternExpressionContext for OobWalker<'_> {
    fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        self.ctx.get_instruction_bytes(byteoff, numbytes, self.state.offset)
    }
    fn get_context_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        self.ctx.get_context_bytes(byteoff, numbytes)
    }
    fn get_addr(&self) -> Address {
        self.cross.unwrap_or(self.ctx).get_addr().clone()
    }
    fn get_naddr(&self) -> Address {
        self.cross.unwrap_or(self.ctx).get_naddr().clone()
    }
    fn get_n2addr(&self) -> KunaResult<Address> {
        self.engine.compute_n2addr(self.cross.unwrap_or(self.ctx))
    }
    fn operand_value(&self, index: i32, table_id: u32, ct_id: u32) -> KunaResult<i64> {
        // Nested out-of-band operand evaluation: build a ParserWalker over the
        // same context positioned at the simulated node, and recurse.  Since
        // setOutOfBandState resets to a single node tree, a nested operand
        // reference would re-derive from the same point; faithful enough for
        // the (rare) chained case, otherwise unreachable for token/context
        // patterns.
        let _ = (index, table_id, ct_id);
        Err(KunaError::sleigh("nested out-of-band operand value not supported"))
    }
}

// ---------------------------------------------------------------------------
// PcodeCacher (sleigh.hh/.cc)
// ---------------------------------------------------------------------------

/// C++ `RelativeRecord`: a varnode holding a relative branch offset plus the
/// absolute index of the instruction containing it.  The varnode pointer is an
/// index into [`PcodeCacher::pool`].
#[derive(Debug, Clone)]
struct RelativeRecord {
    /// Pool index of the varnode holding the relative offset (C++ `dataptr`).
    dataptr: usize,
    /// Absolute index of the next instruction (C++ `calling_index`).
    calling_index: usize,
}

/// C++ `PcodeData`: raw data for one emitted p-code op.  The C++ `VarnodeData *`
/// pointers become indices into [`PcodeCacher::pool`].
#[derive(Debug, Clone)]
struct PcodeData {
    /// Output varnode pool index (C++ `outvar`, `None` = null).
    outvar: Option<usize>,
    /// Input varnode array start pool index (C++ `invar`, `None` = null).
    invar: Option<usize>,
    /// Op code (C++ `opc`).
    opc: OpCode,
    /// Number of input varnodes (C++ `isize`).
    isize: i32,
}

/// C++ `PcodeCacher`: a pool of `VarnodeData` plus the issued p-code ops for
/// one instruction.  The C++ raw-pointer pool becomes a `Vec<VarnodeData>`
/// (pool) indexed by op input/output (ADR 0001); growth never invalidates
/// indices.
struct PcodeCacher {
    /// The pool of VarnodeData objects (C++ `poolstart..endpool`).
    pool: Vec<VarnodeData>,
    /// Issued p-code ops (C++ `issued`).
    issued: Vec<PcodeData>,
    /// References to labels (C++ `label_refs`).
    label_refs: Vec<RelativeRecord>,
    /// Locations of labels (C++ `labels`).
    labels: Vec<u64>,
}

impl PcodeCacher {
    /// C++ `PcodeCacher()`.
    fn new() -> PcodeCacher {
        PcodeCacher { pool: Vec::new(), issued: Vec::new(), label_refs: Vec::new(), labels: Vec::new() }
    }

    /// C++ `allocateVarnodes(uint4 size)`: returns the pool index of the first
    /// of `size` freshly allocated VarnodeData (indices are stable).
    fn allocate_varnodes(&mut self, size: u32) -> usize {
        let start = self.pool.len();
        for _ in 0..size {
            self.pool.push(VarnodeData::default());
        }
        start
    }

    /// C++ `allocateInstruction`: append a new (cleared) PcodeData; returns its
    /// index in `issued`.
    fn allocate_instruction(&mut self) -> usize {
        self.issued.push(PcodeData {
            outvar: None,
            invar: None,
            opc: OpCode::CPUI_COPY,
            isize: 0,
        });
        self.issued.len() - 1
    }

    /// C++ `addLabelRef`.
    fn add_label_ref(&mut self, ptr: usize) {
        self.label_refs.push(RelativeRecord { dataptr: ptr, calling_index: self.issued.len() });
    }

    /// C++ `addLabel(uint4 id)`.
    fn add_label(&mut self, id: u32) {
        while self.labels.len() as u64 <= u64::from(id) {
            self.labels.push(0x0badbeef);
        }
        self.labels[id as usize] = self.issued.len() as u64;
    }

    /// C++ `clear`: reset the cache so all objects are unallocated.  The
    /// engine allocates a fresh [`PcodeCacher`] per instruction (the C++
    /// single instance is `clear`ed between instructions instead); kept for
    /// API parity.
    #[allow(dead_code)] // C++ PcodeCacher::clear (the port allocates fresh per insn)
    fn clear(&mut self) {
        self.pool.clear();
        self.issued.clear();
        self.label_refs.clear();
        self.labels.clear();
    }

    /// C++ `resolveRelatives`.
    fn resolve_relatives(&mut self) -> KunaResult<()> {
        for rec in &self.label_refs {
            let ptr = rec.dataptr;
            let id = self.pool[ptr].offset; // uint4 id = ptr->offset
            // C++ `(id >= labels.size())||(labels[id] == 0xbadbeef)`
            if id >= self.labels.len() as u64 || self.labels[id as usize] == 0x0badbeef {
                return Err(KunaError::lowlevel("Reference to non-existant sleigh label"));
            }
            // res = labels[id] - calling_index  (uintb)
            let res = self.labels[id as usize].wrapping_sub(rec.calling_index as u64);
            let res = res & calc_mask(self.pool[ptr].size as i32); // C++ &= calc_mask(ptr->size)
            self.pool[ptr].offset = res;
        }
        Ok(())
    }

    /// C++ `emit(const Address&,PcodeEmit*)`.
    fn emit(&self, addr: &Address, emt: &mut dyn PcodeEmit, manager: &AddrSpaceManager) {
        for op in &self.issued {
            let outvar = op.outvar.map(|i| &self.pool[i]);
            let invars: Vec<VarnodeData> = (0..op.isize)
                .map(|k| {
                    let base = op.invar.expect("issued op with inputs has invar");
                    self.pool[base + k as usize].clone()
                })
                .collect();
            // The spaceid pointer constant (input 0 of LOAD/STORE) is stored
            // as the space's manager index (LOSS-015); the emitter renders the
            // space name, so the value passed through is the raw stored index.
            let _ = manager;
            emt.dump(addr, op.opc, outvar, &invars);
        }
    }
}


// ---------------------------------------------------------------------------
// SleighBuilder (sleigh.hh/.cc)
// ---------------------------------------------------------------------------

/// A parser context fully resolved to `pcode` state, paired with its address.
/// The engine resolves the main instruction plus any delay-slot / crossbuild
/// targets into a flat set before p-code emission, so the [`SleighBuilder`]
/// can walk them by index without re-entering the (RefCell) caches.
struct ResolvedCtx {
    addr: Address,
    ctx: ParserContext,
}

/// C++ `SleighBuilder : PcodeBuilder`: walks the parse tree and prepares data
/// for final p-code emission (through the [`PcodeCacher`]).
struct SleighBuilder<'a> {
    /// C++ `PcodeBuilder::labelbase`.
    labelbase: u32,
    /// C++ `PcodeBuilder::labelcount`.
    labelcount: u32,
    /// The walk cursor (C++ `ParserWalker *walker` position).
    cur: WalkCursor,
    /// Index into `contexts` of the context the walker currently points into.
    cur_ctx: usize,
    /// Index of the "cross" context for a crossbuild walk (`None` normally).
    cross_ctx: Option<usize>,
    /// All resolved contexts (index 0 is the main instruction).
    contexts: &'a [ResolvedCtx],
    /// Symbol table (read-only navigation).
    table: &'a SymbolTable,
    /// ConstructTpl store (template lookups during build).
    templates: &'a [ConstructTpl],
    /// The engine (n2addr length lookups).
    engine: &'a Sleigh,
    const_space: Rc<AddrSpace>,
    uniq_space: Rc<AddrSpace>,
    uniquemask: u32,
    uniqueoffset: u64,
    cache: &'a mut PcodeCacher,
}

impl<'a> SleighBuilder<'a> {
    /// A read-only walker positioned at the builder's current cursor.
    fn walker(&self) -> ParserWalker<'_> {
        ParserWalker {
            ctx: &self.contexts[self.cur_ctx].ctx,
            cross: self.cross_ctx.map(|i| &self.contexts[i].ctx),
            table: self.table,
            engine: self.engine,
            cur: self.cur.clone(),
        }
    }

    /// The constructor template referenced by a handle (C++ `getTempl`).
    fn templ(&self, handle: usize) -> &ConstructTpl {
        &self.templates[handle]
    }

    /// C++ `SleighBuilder::setUniqueOffset`.
    fn set_unique_offset(&mut self, addr: &Address) {
        self.uniqueoffset = (addr.get_offset() & u64::from(self.uniquemask)) << 8;
    }

    /// C++ `SleighBuilder::generateLocation`.
    fn generate_location(&self, vntpl: &VarnodeTpl, vn: &mut VarnodeData) -> KunaResult<()> {
        let walker = self.walker();
        let space = vntpl
            .get_space()
            .fix_space(&walker)?
            .ok_or_else(|| KunaError::sleigh("generateLocation: null space"))?;
        vn.size = vntpl.get_size().fix(&walker)? as u32; // C++ uintb -> uint4
        if Rc::ptr_eq(&space, &self.const_space) {
            vn.offset = vntpl.get_offset().fix(&walker)? & calc_mask(vn.size as i32);
        } else if Rc::ptr_eq(&space, &self.uniq_space) {
            vn.offset = vntpl.get_offset().fix(&walker)?;
            vn.offset |= self.uniqueoffset;
        } else {
            vn.offset = space.wrap_offset(vntpl.get_offset().fix(&walker)?);
        }
        vn.space = Some(space);
        Ok(())
    }

    /// C++ `SleighBuilder::generatePointer`.
    fn generate_pointer(
        &self,
        vntpl: &VarnodeTpl,
        vn: &mut VarnodeData,
    ) -> KunaResult<Rc<AddrSpace>> {
        let walker = self.walker();
        let hand = walker.get_fixed_handle(vntpl.get_offset().get_handle_index())?;
        let space = hand
            .offset_space
            .clone()
            .ok_or_else(|| KunaError::sleigh("generatePointer: null offset space"))?;
        vn.size = hand.offset_size;
        if Rc::ptr_eq(&space, &self.const_space) {
            vn.offset = hand.offset_offset & calc_mask(vn.size as i32);
        } else if Rc::ptr_eq(&space, &self.uniq_space) {
            vn.offset = hand.offset_offset | self.uniqueoffset;
        } else {
            vn.offset = space.wrap_offset(hand.offset_offset);
        }
        vn.space = Some(space);
        hand.space.ok_or_else(|| KunaError::sleigh("generatePointer: null pointed-to space"))
    }

    /// C++ `SleighBuilder::generatePointerAdd`.
    fn generate_pointer_add(&mut self, op_idx: usize, vntpl: &VarnodeTpl) -> KunaResult<()> {
        let offset_plus = vntpl.get_offset().get_real() & 0xffff;
        if offset_plus == 0 {
            return Ok(());
        }
        let nextop_idx = self.cache.allocate_instruction();
        self.cache.issued[nextop_idx].opc = self.cache.issued[op_idx].opc;
        self.cache.issued[nextop_idx].invar = self.cache.issued[op_idx].invar;
        self.cache.issued[nextop_idx].isize = self.cache.issued[op_idx].isize;
        self.cache.issued[nextop_idx].outvar = self.cache.issued[op_idx].outvar;
        self.cache.issued[op_idx].isize = 2;
        self.cache.issued[op_idx].opc = OpCode::CPUI_INT_ADD;
        let newparams = self.cache.allocate_varnodes(2);
        self.cache.issued[op_idx].invar = Some(newparams);
        let nextop_invar = self.cache.issued[nextop_idx]
            .invar
            .expect("nextop has inputs (copied from op)");
        self.cache.pool[newparams] = self.cache.pool[nextop_invar + 1].clone();
        let p0_size = self.cache.pool[newparams].size;
        self.cache.pool[newparams + 1].space = Some(Rc::clone(&self.const_space));
        self.cache.pool[newparams + 1].offset = offset_plus;
        self.cache.pool[newparams + 1].size = p0_size;
        // outvar becomes the original op's input slot 1
        self.cache.issued[op_idx].outvar = Some(nextop_invar + 1);
        let ea = self.engine.get_unique_start(UniqueLayout::RUNTIME_BITRANGE_EA);
        self.cache.pool[nextop_invar + 1].space = Some(Rc::clone(&self.uniq_space));
        self.cache.pool[nextop_invar + 1].offset = u64::from(ea);
        Ok(())
    }

    /// C++ `SleighBuilder::buildEmpty`.
    fn build_empty(&mut self, ct: ConstructorRef, secnum: i32) -> KunaResult<()> {
        let numops = self.table.get_constructor(ct)?.get_num_operands();
        for i in 0..numops {
            let opid = self.table.get_constructor(ct)?.get_operand(i)?;
            let opsym = self
                .table
                .find_symbol_by_id(opid)
                .ok_or_else(|| KunaError::sleigh("buildEmpty: operand undefined"))?;
            let SymbolKind::Operand(op) = opsym.kind() else { continue };
            let defsym = match op.get_defining_symbol() {
                Some(d) => d,
                None => continue,
            };
            let defsym_obj = self
                .table
                .find_symbol_by_id(defsym)
                .ok_or_else(|| KunaError::sleigh("buildEmpty: defining symbol undefined"))?;
            if defsym_obj.get_type() != SymbolType::Subtable {
                continue;
            }
            self.push_operand(i)?;
            let cur_ct = self.walker().get_constructor_inner()?;
            let construct = self.table.get_constructor(cur_ct)?.get_named_templ(secnum);
            match construct {
                None => self.build_empty(cur_ct, secnum)?,
                Some(handle) => self.build(Some(self.templ(handle).clone()).as_ref(), secnum)?,
            }
            self.pop_operand();
        }
        Ok(())
    }

    /// C++ `pushOperand` on the builder's walker.
    fn push_operand(&mut self, i: i32) -> KunaResult<()> {
        if self.cur.depth > MAX_DEPTH - 2 {
            return Err(KunaError::lowlevel("SLEIGH exceeded maximum parse depth"));
        }
        let parent = self.cur.point.expect("push: null point");
        let child = self.contexts[self.cur_ctx].ctx.state[parent].resolve[i as usize];
        self.cur.breadcrumb[self.cur.depth as usize] = i + 1;
        self.cur.depth += 1;
        self.cur.point = child;
        self.cur.breadcrumb[self.cur.depth as usize] = 0;
        Ok(())
    }

    /// C++ `popOperand` on the builder's walker.
    fn pop_operand(&mut self) {
        let p = self.cur.point.expect("pop: null point");
        self.cur.point = self.contexts[self.cur_ctx].ctx.state[p].parent;
        self.cur.depth -= 1;
    }
}


impl PcodeBuilder for SleighBuilder<'_> {
    fn get_label_base(&self) -> u32 {
        self.labelbase
    }
    fn set_label_base(&mut self, val: u32) {
        self.labelbase = val;
    }
    fn get_label_count(&self) -> u32 {
        self.labelcount
    }
    fn set_label_count(&mut self, val: u32) {
        self.labelcount = val;
    }

    /// C++ `SleighBuilder::dump(OpTpl *op)`.
    fn dump(&mut self, op: &OpTpl) -> KunaResult<()> {
        let isize = op.num_input();
        // invars = allocateVarnodes(isize)
        let invars = self.cache.allocate_varnodes(isize as u32);
        for i in 0..isize {
            let vn = op.get_in(i);
            let dynamic = vn.is_dynamic(&self.walker())?;
            if dynamic {
                // input is really temporary storage
                let mut tmp = VarnodeData::default();
                self.generate_location(vn, &mut tmp)?;
                self.cache.pool[invars + i as usize] = tmp;
                let load_op = self.cache.allocate_instruction();
                self.cache.issued[load_op].opc = OpCode::CPUI_LOAD;
                self.cache.issued[load_op].outvar = Some(invars + i as usize);
                self.cache.issued[load_op].isize = 2;
                let loadvars = self.cache.allocate_varnodes(2);
                self.cache.issued[load_op].invar = Some(loadvars);
                let mut ptrvn = VarnodeData::default();
                let spc = self.generate_pointer(vn, &mut ptrvn)?;
                self.cache.pool[loadvars + 1] = ptrvn;
                // loadvars[0] = const_space spaceid (LOSS-015: store index)
                self.cache.pool[loadvars].space = Some(Rc::clone(&self.const_space));
                self.cache.pool[loadvars].offset = spaceid_const(&spc);
                self.cache.pool[loadvars].size = SIZEOF_SPACE;
                if vn.get_offset().get_select() == VField::VOffsetPlus {
                    self.generate_pointer_add(load_op, vn)?;
                }
            } else {
                let mut tmp = VarnodeData::default();
                self.generate_location(vn, &mut tmp)?;
                self.cache.pool[invars + i as usize] = tmp;
            }
        }
        if isize > 0 && op.get_in(0).is_relative() {
            self.cache.pool[invars].offset =
                self.cache.pool[invars].offset.wrapping_add(u64::from(self.get_label_base()));
            self.cache.add_label_ref(invars);
        }
        let thisop = self.cache.allocate_instruction();
        self.cache.issued[thisop].opc = op.get_opcode();
        self.cache.issued[thisop].invar = if isize > 0 { Some(invars) } else { None };
        self.cache.issued[thisop].isize = isize;
        if let Some(outvn) = op.get_out() {
            let dynamic = outvn.is_dynamic(&self.walker())?;
            if dynamic {
                let storevars = self.cache.allocate_varnodes(3);
                let mut tmp = VarnodeData::default();
                self.generate_location(outvn, &mut tmp)?;
                self.cache.pool[storevars + 2] = tmp;
                self.cache.issued[thisop].outvar = Some(storevars + 2);
                let store_op = self.cache.allocate_instruction();
                self.cache.issued[store_op].opc = OpCode::CPUI_STORE;
                self.cache.issued[store_op].isize = 3;
                self.cache.issued[store_op].invar = Some(storevars);
                let mut ptrvn = VarnodeData::default();
                let spc = self.generate_pointer(outvn, &mut ptrvn)?;
                self.cache.pool[storevars + 1] = ptrvn;
                self.cache.pool[storevars].space = Some(Rc::clone(&self.const_space));
                self.cache.pool[storevars].offset = spaceid_const(&spc);
                self.cache.pool[storevars].size = SIZEOF_SPACE;
                if outvn.get_offset().get_select() == VField::VOffsetPlus {
                    self.generate_pointer_add(store_op, outvn)?;
                }
            } else {
                let out = self.cache.allocate_varnodes(1);
                let mut tmp = VarnodeData::default();
                self.generate_location(outvn, &mut tmp)?;
                self.cache.pool[out] = tmp;
                self.cache.issued[thisop].outvar = Some(out);
            }
        }
        Ok(())
    }

    /// C++ `SleighBuilder::appendBuild`.
    fn append_build(&mut self, bld: &OpTpl, secnum: i32) -> KunaResult<()> {
        let index = bld.get_in(0).get_offset().get_real() as i32; // C++ uintb -> int4
        let ct = self.walker().get_constructor_inner()?;
        let opid = self.table.get_constructor(ct)?.get_operand(index)?;
        let opsym = self
            .table
            .find_symbol_by_id(opid)
            .ok_or_else(|| KunaError::sleigh("appendBuild: operand undefined"))?;
        let SymbolKind::Operand(op) = opsym.kind() else { return Ok(()) };
        let sym = match op.get_defining_symbol() {
            Some(d) => d,
            None => return Ok(()),
        };
        let defsym = self
            .table
            .find_symbol_by_id(sym)
            .ok_or_else(|| KunaError::sleigh("appendBuild: defining symbol undefined"))?;
        if defsym.get_type() != SymbolType::Subtable {
            return Ok(());
        }
        self.push_operand(index)?;
        let cur_ct = self.walker().get_constructor_inner()?;
        if secnum >= 0 {
            let construct = self.table.get_constructor(cur_ct)?.get_named_templ(secnum);
            match construct {
                None => self.build_empty(cur_ct, secnum)?,
                Some(handle) => self.build(Some(self.templ(handle).clone()).as_ref(), secnum)?,
            }
        } else {
            let handle = self.table.get_constructor(cur_ct)?.get_templ();
            let tpl = handle.map(|h| self.templ(h).clone());
            self.build(tpl.as_ref(), -1)?;
        }
        self.pop_operand();
        Ok(())
    }

    /// C++ `SleighBuilder::delaySlot`.
    fn delay_slot(&mut self, _op: &OpTpl) -> KunaResult<()> {
        let saved_cur = self.cur.clone();
        let saved_ctx = self.cur_ctx;
        let saved_cross = self.cross_ctx;
        let old_uniqueoffset = self.uniqueoffset;

        let baseaddr = self.contexts[self.cur_ctx].addr.clone();
        let fall_base = self.contexts[self.cur_ctx].ctx.get_length();
        let delay_byte_cnt = self.contexts[self.cur_ctx].ctx.get_delay_slot();
        let mut fall_offset = fall_base;
        let mut bytecount = 0i32;
        loop {
            let newaddr = &baseaddr + i64::from(fall_offset);
            self.set_unique_offset(&newaddr);
            // Locate the resolved delay-slot context (must already be pcode).
            let idx = self.find_context(&newaddr).ok_or_else(|| {
                KunaError::lowlevel("Could not obtain cached delay slot instruction")
            })?;
            if self.contexts[idx].ctx.get_parser_state() != ParseState::Pcode {
                return Err(KunaError::lowlevel(
                    "Could not obtain cached delay slot instruction",
                ));
            }
            let len = self.contexts[idx].ctx.get_length();
            // Walk the whole delay slot from its base state.
            self.cur_ctx = idx;
            self.cross_ctx = None;
            self.cur = WalkCursor::new();
            self.cur.point = Some(self.contexts[idx].ctx.base_state);
            let ct = self.walker().get_constructor_inner()?;
            let handle = self.table.get_constructor(ct)?.get_templ();
            let tpl = handle.map(|h| self.templ(h).clone());
            self.build(tpl.as_ref(), -1)?;
            fall_offset += len;
            bytecount += len;
            if bytecount >= delay_byte_cnt {
                break;
            }
        }
        self.cur = saved_cur;
        self.cur_ctx = saved_ctx;
        self.cross_ctx = saved_cross;
        self.uniqueoffset = old_uniqueoffset;
        Ok(())
    }

    /// C++ `SleighBuilder::setLabel`.
    fn set_label(&mut self, op: &OpTpl) -> KunaResult<()> {
        let id = op.get_in(0).get_offset().get_real() as u32; // C++ uintb -> uint4
        self.cache.add_label(id.wrapping_add(self.get_label_base()));
        Ok(())
    }

    /// C++ `SleighBuilder::appendCrossBuild`.
    fn append_cross_build(&mut self, bld: &OpTpl, secnum: i32) -> KunaResult<()> {
        if secnum >= 0 {
            return Err(KunaError::lowlevel("CROSSBUILD directive within a named section"));
        }
        let secnum = bld.get_in(1).get_offset().get_real() as i32; // C++ uintb -> int4
        let vn = bld.get_in(0);
        let spc = vn
            .get_space()
            .fix_space(&self.walker())?
            .ok_or_else(|| KunaError::sleigh("crossbuild: null space"))?;
        let addr = spc.wrap_offset(vn.get_offset().fix(&self.walker())?);

        let saved_cur = self.cur.clone();
        let saved_ctx = self.cur_ctx;
        let saved_cross = self.cross_ctx;
        let old_uniqueoffset = self.uniqueoffset;

        let newaddr = Address::new(Rc::clone(&spc), addr);
        self.set_unique_offset(&newaddr);
        let idx = self.find_context(&newaddr).ok_or_else(|| {
            KunaError::lowlevel("Could not obtain cached crossbuild instruction")
        })?;
        if self.contexts[idx].ctx.get_parser_state() != ParseState::Pcode {
            return Err(KunaError::lowlevel("Could not obtain cached crossbuild instruction"));
        }
        self.cur_ctx = idx;
        self.cross_ctx = Some(saved_ctx);
        self.cur = WalkCursor::new();
        self.cur.point = Some(self.contexts[idx].ctx.base_state);
        let ct = self.walker().get_constructor_inner()?;
        let construct = self.table.get_constructor(ct)?.get_named_templ(secnum);
        match construct {
            None => self.build_empty(ct, secnum)?,
            Some(handle) => self.build(Some(self.templ(handle).clone()).as_ref(), secnum)?,
        }
        self.cur = saved_cur;
        self.cur_ctx = saved_ctx;
        self.cross_ctx = saved_cross;
        self.uniqueoffset = old_uniqueoffset;
        Ok(())
    }
}

impl SleighBuilder<'_> {
    /// Find a resolved context by address (C++ `discache->getParserContext`,
    /// but over the pre-resolved set).
    fn find_context(&self, addr: &Address) -> Option<usize> {
        self.contexts.iter().position(|c| &c.addr == addr)
    }
}

/// LOSS-015: a LOAD/STORE space-pointer constant stores the space's manager
/// index in place of the C++ heap pointer `(uintb)(uintp)spc`.
fn spaceid_const(spc: &Rc<AddrSpace>) -> u64 {
    spc.get_index() as u64 // cast: manager index, small and non-negative
}

/// C++ `sizeof(spc)` (a `void *`): the size stored on a LOAD/STORE space
/// pointer constant.  The golden fixtures were generated on a 64-bit host.
const SIZEOF_SPACE: u32 = 8;


// ---------------------------------------------------------------------------
// Sleigh engine (sleigh.hh/.cc)
// ---------------------------------------------------------------------------

/// C++ `Sleigh : SleighBase`: a full SLEIGH engine.
///
/// Each `oneInstruction`/`instructionLength` resolves a fresh
/// [`ParserContext`] (the C++ `DisassemblyCache` is an optimization, not a
/// correctness requirement — resolution is a pure function of the bytes and
/// the painted context).  The load image and context database/cache sit behind
/// `RefCell` so the `const` C++ trait methods stay `&self` (see module docs).
pub struct Sleigh {
    /// The SLEIGH spec core (spaces, symbol table, templates, register map).
    base: SleighBase,
    /// The mapped bytes of the program (C++ `LoadImage *loader`).
    ///
    /// Wrapped in an [`Rc`] so the IR-boundary `glb` skeleton (the
    /// `kuna_decomp::context::ArchContext` the Funcdata holds) can share read access for
    /// jump-table LOAD emulation (`EmulateFunction::executeLoad`); the C++
    /// `Architecture::loader` is a long-lived `LoadImage *` reached identically
    /// from both the engine and the emulator.
    loader: Rc<RefCell<Box<dyn LoadImage>>>,
    /// Database of context values steering disassembly (C++ `context_db`).
    context_db: RefCell<Box<dyn ContextDatabase>>,
    /// Cache of recently used context values (C++ `cache`).
    cache: RefCell<ContextCache>,
}

// ---------------------------------------------------------------------------
// Instruction-mask accessor (FID / AIF fingerprinting prerequisite)
// ---------------------------------------------------------------------------

/// A decoded instruction's *fixed-bit mask*: which encoding bits SLEIGH pins to
/// a constant (opcode / addressing-mode bits) versus which carry operand values
/// (immediates, displacements, register selectors).  Produced by
/// [`Sleigh::instruction_mask`]; consumed by FID-family operand-independent
/// fingerprinting.
///
/// Invariant: `fixed_mask.len() == bytes.len() == length as usize`.  The FID
/// "full mask" of byte `i` is `bytes[i] & fixed_mask[i]`; the operand (variable)
/// mask is `!fixed_mask[i]`.  This is purely an *accessor* — it does not alter
/// the decode in any way.
#[derive(Debug, Clone)]
pub struct InsnMask {
    /// The raw instruction bytes (length `length`).
    pub bytes: Vec<u8>,
    /// `fixed_mask[i]` has a 1 wherever SLEIGH pins a fixed encoding bit, 0
    /// where the bit belongs to an operand value (`operand_mask = !fixed_mask`).
    pub fixed_mask: Vec<u8>,
    /// The instruction length in bytes (C++ `instructionLength`).
    pub length: i32,
    /// Per-operand views: the value-bit range and the classified op-objects.
    pub operands: Vec<OperandView>,
}

/// One operand of a decoded instruction, as seen by the mask accessor.
#[derive(Debug, Clone)]
pub struct OperandView {
    /// Byte mask (length = instruction length) selecting this operand's value
    /// bits — the bits SLEIGH did *not* pin within the operand's byte span.
    pub value_mask: Vec<u8>,
    /// The classified objects the operand resolves to (a register, a scalar
    /// immediate, or a code/data address).
    pub objects: Vec<OpObject>,
}

/// A classified operand object (the FID `OperandValue`/`Varnode` analog): what
/// the operand's resolved [`FixedHandle`] turned out to denote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpObject {
    /// A scalar/immediate (constant-space handle).  `signed` is the resolved
    /// value; `is_address` marks a scalar SLEIGH flagged as a code address.
    Scalar { signed: i64, whole_operand: bool, is_address: bool },
    /// A register (register-space handle) at the given register-space offset.
    Register { offset: u64 },
    /// A code/data address (any other resolved space).
    Address,
}

impl Sleigh {
    /// C++ `Sleigh(LoadImage *ld,ContextDatabase *c_db)`.
    pub fn new(loader: Box<dyn LoadImage>, context_db: Box<dyn ContextDatabase>) -> Sleigh {
        Sleigh {
            base: SleighBase::new(),
            loader: Rc::new(RefCell::new(loader)),
            context_db: RefCell::new(context_db),
            cache: RefCell::new(ContextCache::new()),
        }
    }

    /// Share the program load image (C++ `Architecture::loader`) so the
    /// IR-boundary `glb` skeleton can read read-only memory (jump-table
    /// emulation).  The returned `Rc` aliases the engine's loader; `set_loader`
    /// replaces the inner `Box` in place, so a shared handle stays current.
    pub fn loader_rc(&self) -> Rc<RefCell<Box<dyn LoadImage>>> {
        Rc::clone(&self.loader)
    }

    /// Borrow the SLEIGH spec core.
    pub fn base(&self) -> &SleighBase {
        &self.base
    }

    /// Mutably borrow the SLEIGH spec core (the `Architecture` reaches its
    /// `manager_mut` through this to insert the fspec/iop/join spaces during
    /// `restoreFromSpec`, LOSS-132).
    pub fn base_mut(&mut self) -> &mut SleighBase {
        &mut self.base
    }

    /// Share the single address-space manager (LOSS-132): the `Rc` the lift
    /// populated, handed to the `Architecture` / `Funcdata::glb` so every
    /// subsystem keys state by the same space identities/indices.
    pub fn manager_rc(&self) -> Rc<AddrSpaceManager> {
        self.base.manager_rc()
    }

    /// Replace the load image (C++ `Sleigh::reset` keeps the rest of the
    /// engine; the lift gate installs the corpus image after the `.sla` is
    /// decoded so the image can be opened against the engine's manager).
    pub fn set_loader(&mut self, loader: Box<dyn LoadImage>) {
        *self.loader.borrow_mut() = loader;
    }

    /// Read `sz` (<=8) bytes of a Varnode's worth of value from the load image
    /// (C++ `EmulatePcodeOp::getLoadImageValue` / `EmulateSnippet::getLoadImageValue`,
    /// `emulateutil.cc:30`/`150`): `loadFill` a full `uintb` worth of bytes at
    /// the address, then byte-swap to host order if the space endianness differs
    /// from the host, and mask/shift down to `sz`.  Used by jump-table emulation
    /// (`EmulateFunction::executeLoad`) to read read-only switch tables.
    pub fn read_loadimage_value(&self, addr: &Address, sz: i32) -> KunaResult<u64> {
        use kuna_base::address::byte_swap;
        use kuna_base::types::HOST_ENDIAN;
        let mut buf = [0u8; 8]; // sizeof(uintb)
        self.loader.borrow_mut().load_fill(&mut buf, addr)?;
        let big = addr.is_big_endian();
        // C++ reads the raw uintb in host byte order off the filled buffer.
        let mut res = if HOST_ENDIAN == 1 {
            u64::from_be_bytes(buf)
        } else {
            u64::from_le_bytes(buf)
        };
        if (HOST_ENDIAN == 1) != big {
            res = byte_swap(res, 8); // byte_swap(res,sizeof(uintb))
        }
        if big && (sz as usize) < 8 {
            res >>= (8 - sz as u32) * 8;
        } else {
            res &= kuna_base::address::calc_mask(sz);
        }
        Ok(res)
    }

    /// C++ `Sleigh::allowContextSet`.
    pub fn allow_context_set(&self, val: bool) {
        self.cache.borrow_mut().allow_set(val);
    }

    /// Install a [`RegisterLookup`] on the engine's address-space manager (the
    /// kuna stand-in for the C++ `AddrSpace::trans` back-pointer that
    /// `getDefaultCodeSpace()->getTrans()` reaches).  The lookup is a standalone
    /// snapshot of the engine's register cross-reference, so there is no `Rc`
    /// cycle back into `self`.
    ///
    /// This must run after the `.sla` decode populated the register table and
    /// while the engine is still the sole `Rc` owner of the manager (before the
    /// `glb`/`open_image` share it) — i.e. in the bootstrap, right before the
    /// spec decode that resolves register names (`<context_data>` tracked sets,
    /// `<pentry>` `<addr name=…>`, etc.).
    pub fn install_register_lookup(&mut self) -> KunaResult<()> {
        let lookup: Rc<dyn RegisterLookup> =
            Rc::new(crate::sleighbase::SnapshotRegisterLookup::from_base(&self.base));
        self.base.manager_mut().set_register_lookup(lookup);
        Ok(())
    }

    /// Run a closure with mutable access to the engine's [`ContextDatabase`] (C++
    /// `glb->context`, the `ContextDatabase*` the translator holds).  The console
    /// `set context`/`set track` commands paint context/tracked values through
    /// this; the borrow is scoped to the closure so it never overlaps a decode.
    pub fn with_context_db_mut<R>(&self, f: impl FnOnce(&mut dyn ContextDatabase) -> R) -> R {
        let mut db = self.context_db.borrow_mut();
        f(&mut **db)
    }

    /// Resolve a register by name to its [`VarnodeData`] storage (C++
    /// `Translate::getRegister(name)`).  Used by `set track` to record the tracked
    /// register's location.
    pub fn get_register_varnode(
        &self,
        nm: &[u8],
    ) -> KunaResult<kuna_num::pcoderaw::VarnodeData> {
        self.base.get_register(nm)
    }

    /// C++ `Sleigh::initialize(DocumentStorage&)`: load the `.sla` and prepare
    /// the engine.  The `.sla` bytes are supplied directly (the C++ resolves
    /// the file name from the `<sleigh>` tag; the Rust caller passes the raw
    /// compressed file contents).
    pub fn initialize_from_sla(&mut self, sla_bytes: &[u8]) -> KunaResult<()> {
        if !self.base.is_initialized() {
            // The C++ `FormatDecode decoder(this)` aliases the
            // `AddrSpaceManager` that `SleighBase::decode` simultaneously
            // mutates via `insertSpace` (it IS the manager, by inheritance):
            // a raw-pointer back-reference (LOSS-020).  The reads
            // (`read_space`) only ever happen AFTER the relevant inserts, and
            // never while the decoder holds a borrow across an insert, so the
            // aliasing is sound; Rust cannot prove the temporal disjointness,
            // so we mirror the C++ raw pointer with a contained `unsafe`
            // re-borrow.
            //
            // SAFETY: `self.base.manager` outlives `decoder` (both live for
            // this call); `SleighBase::decode` mutates the manager only via
            // `insert_space`/`set_default_code_space` in `decode_sla_spaces`,
            // strictly before any `read_space` the symbol-table decode issues,
            // so no manager read and mutation overlap in time.
            let manager_ptr: *const AddrSpaceManager = &*self.base.manager;
            let mut decoder = crate::slaformat::FormatDecode::new(unsafe { &*manager_ptr });
            decoder.ingest_stream(sla_bytes)?;
            let db_cell = &self.context_db;
            let register_context = |nm: &[u8], sb: i32, eb: i32| -> KunaResult<()> {
                db_cell.borrow_mut().register_variable(nm, sb, eb)
            };
            self.base.decode(&mut decoder, register_context)?;
            return Ok(());
        }
        // Re-registration path (engine reused with a new program): re-register
        // the context variables with the (new) database.
        let db = &mut **self.context_db.borrow_mut();
        self.base.reregister_context(|nm, sb, eb| db.register_variable(nm, sb, eb))?;
        Ok(())
    }

    /// C++ `Translate::getConstantSpace` via the manager.
    fn const_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.base.manager.get_constant_space().expect("constant space registered"))
    }

    /// C++ `Translate::getUniqueSpace` via the manager.
    fn unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.base.manager.get_unique_space().expect("unique space registered"))
    }

    /// Load the instruction bytes for `addr` into `pos.buf` (C++
    /// `loader->loadFill(pos.getBuffer(),16,pos.getAddr())`).
    fn load_fill_context(&self, pos: &mut ParserContext) -> KunaResult<()> {
        let addr = pos.addr.clone();
        let buf = pos.get_buffer_mut();
        self.loader.borrow_mut().load_fill(buf, &addr)
    }

    /// Build a fresh, uninitialized parser context positioned at `addr`.
    fn fresh_context(&self, addr: &Address) -> ParserContext {
        let contextsize = self.context_db.borrow().get_context_size();
        let mut pos = ParserContext::new(contextsize);
        pos.initialize(self.const_space(), INITIAL_STATE_NUM);
        pos.set_addr(addr.clone());
        pos
    }

    /// C++ `Sleigh::resolve`: build the constructor tree (disassembly state).
    fn resolve(&self, pos: &mut ParserContext) -> KunaResult<()> {
        self.load_fill_context(pos)?;
        // loadContext: pull context words for the current address.
        {
            let db = self.context_db.borrow();
            let mut cache = self.cache.borrow_mut();
            let mut buf = vec![0u32; pos.context.len()];
            cache.get_context(&**db, &pos.addr, &mut buf);
            pos.context = buf;
        }
        pos.set_delay_slot(0);
        pos.clear_commits();

        let table = &self.base.symtab;
        let root = self
            .base
            .root
            .ok_or_else(|| KunaError::sleigh("SLEIGH not initialized (no root)"))?;
        let mut walker = ParserWalkerChange::new(pos, table, self);
        walker.deallocate_state();
        walker.set_offset(0);
        // ct = root->resolve(walker).  We use the `resolve_matched` variant
        // (identical decision walk) so we can ALSO capture the matched
        // DisjointPattern leaf here, where the per-node context is the one that
        // selected the constructor — and stash it on this node for the FID
        // instruction-mask accessor.  The chosen constructor is unchanged.
        let subtable = subtable_ref(table, root)?;
        let (root_ct_id, root_pat) = {
            let reader = walker.as_reader();
            let (pat, ct) = subtable.resolve_matched(&reader)?;
            (*ct, pat.clone())
        };
        let root_ref = ConstructorRef { table_id: root, ct_id: root_ct_id };
        walker.set_constructor(root_ref);
        walker.set_matched_pattern(root_pat);
        apply_context(table, root_ref, &mut walker)?;

        while walker.is_state() {
            let ct = walker.get_constructor()?;
            let mut oper = walker.get_operand();
            let numoper = table.get_constructor(ct)?.get_num_operands();
            while oper < numoper {
                let opid = table.get_constructor(ct)?.get_operand(oper)?;
                let opsym = table
                    .find_symbol_by_id(opid)
                    .ok_or_else(|| KunaError::sleigh("resolve: operand undefined"))?;
                let SymbolKind::Operand(op) = opsym.kind() else {
                    return Err(KunaError::sleigh("resolve: not an operand symbol"));
                };
                let offset_base = op.get_offset_base();
                let rel = op.get_relative_offset();
                let off = walker.get_offset(offset_base).wrapping_add(rel);
                walker.allocate_operand(oper)?;
                walker.set_offset(off);
                let triple = op.get_defining_symbol();
                let mut descended = false;
                if let Some(tsym) = triple {
                    // `resolve_triple_matched` is the `resolve_triple` walk plus
                    // the matched DisjointPattern leaf (None for non-subtable
                    // triples).  Captured here under the correct per-node
                    // context; stashed on the child node we just allocated, for
                    // the FID instruction-mask accessor.  Same constructor.
                    let (subct, subpat) = {
                        let reader = walker.as_reader();
                        resolve_triple_matched(table, tsym, &reader)?
                    };
                    if let Some(subct_id) = subct {
                        let subct_ref = ConstructorRef { table_id: tsym, ct_id: subct_id };
                        walker.set_constructor(subct_ref);
                        if let Some(pat) = subpat {
                            walker.set_matched_pattern(pat);
                        }
                        apply_context(table, subct_ref, &mut walker)?;
                        descended = true;
                    }
                }
                if descended {
                    break;
                }
                let minlen = op.get_minimum_length();
                walker.set_current_length(minlen);
                walker.pop_operand();
                oper += 1;
            }
            if oper >= numoper {
                let ct_minlen = table.get_constructor(ct)?.get_minimum_length();
                walker.calc_current_length(ct_minlen, numoper);
                walker.pop_operand();
                // delay slot
                let handle = table.get_constructor(ct)?.get_templ();
                if let Some(h) = handle {
                    let ds = self.base.templates[h].delay_slot();
                    if ds > 0 {
                        walker.ctx.set_delay_slot(ds as i32); // C++ uint4 -> int4
                    }
                }
            }
        }
        // naddr = addr + length
        let naddr = &pos.addr + i64::from(pos.get_length());
        pos.set_naddr(naddr);
        pos.set_parser_state(ParseState::Disassembly);
        Ok(())
    }

    /// C++ `Sleigh::resolveHandles`.
    fn resolve_handles(&self, pos: &mut ParserContext) -> KunaResult<()> {
        let table = &self.base.symtab;
        let mut walker = ParserWalkerChange::new(pos, table, self);
        walker.base_state();
        while walker.is_state() {
            let ct = walker.get_constructor()?;
            let mut oper = walker.get_operand();
            let numoper = table.get_constructor(ct)?.get_num_operands();
            while oper < numoper {
                let opid = table.get_constructor(ct)?.get_operand(oper)?;
                let opsym = table
                    .find_symbol_by_id(opid)
                    .ok_or_else(|| KunaError::sleigh("resolveHandles: operand undefined"))?;
                let SymbolKind::Operand(op) = opsym.kind() else {
                    return Err(KunaError::sleigh("resolveHandles: not an operand"));
                };
                walker.push_operand(oper)?;
                let triple = op.get_defining_symbol();
                if let Some(tid) = triple {
                    let tsym = table
                        .find_symbol_by_id(tid)
                        .ok_or_else(|| KunaError::sleigh("resolveHandles: triple undefined"))?;
                    if tsym.get_type() == SymbolType::Subtable {
                        break;
                    } else {
                        let mut hand = FixedHandle::default();
                        {
                            let reader = walker.as_reader();
                            tsym.get_fixed_handle(&mut hand, &reader, table)?;
                        }
                        let parent = walker.point();
                        walker.ctx.state[parent].hand = hand;
                    }
                } else {
                    // expression operand: result is a constant
                    let patexp = op
                        .get_defining_expression()
                        .ok_or_else(|| KunaError::sleigh("resolveHandles: no defining expr"))?
                        .clone();
                    let res = {
                        let reader = walker.as_reader();
                        patexp.get_value(&reader)?
                    };
                    let const_space = pos_const_space(walker.ctx);
                    let parent = walker.point();
                    let h = &mut walker.ctx.state[parent].hand;
                    h.space = Some(const_space);
                    h.offset_space = None;
                    h.offset_offset = res as u64; // C++ (uintb)res
                    h.size = 0;
                }
                walker.pop_operand();
                oper += 1;
            }
            if oper >= numoper {
                let handle = table.get_constructor(ct)?.get_templ();
                if let Some(h) = handle {
                    if let Some(res) = self.base.templates[h].get_result().cloned() {
                        let mut hand = FixedHandle::default();
                        {
                            let reader = walker.as_reader();
                            res.fix(&mut hand, &reader)?;
                        }
                        let parent = walker.point();
                        walker.ctx.state[parent].hand = hand;
                    }
                }
                walker.pop_operand();
            }
        }
        pos.set_parser_state(ParseState::Pcode);
        Ok(())
    }

    /// C++ `Sleigh::obtainContext`: resolve a fresh context up to `state`.
    fn obtain_context(&self, addr: &Address, state: ParseState) -> KunaResult<ParserContext> {
        let mut pos = self.fresh_context(addr);
        self.resolve(&mut pos)?;
        if state == ParseState::Disassembly {
            return Ok(pos);
        }
        self.resolve_handles(&mut pos)?;
        Ok(pos)
    }

    /// C++ `ParserContext::applyCommits`.
    fn apply_commits(&self, pos: &ParserContext) -> KunaResult<()> {
        if pos.contextcommit.is_empty() {
            return Ok(());
        }
        let table = &self.base.symtab;
        let mut walker = ParserWalker::new(pos, table, self);
        walker.base_state();
        for set in &pos.contextcommit {
            let sym = table
                .find_symbol_by_id(set.sym)
                .ok_or_else(|| KunaError::sleigh("applyCommits: symbol undefined"))?;
            let commitaddr = if sym.get_type() == SymbolType::Operand {
                let SymbolKind::Operand(op) = sym.kind() else {
                    return Err(KunaError::sleigh("applyCommits: not operand"));
                };
                let i = op.get_index();
                let child = pos.state[set.point].resolve[i as usize]
                    .ok_or_else(|| KunaError::sleigh("applyCommits: operand not resolved"))?;
                let h = &pos.state[child].hand;
                Address::new(
                    h.space.clone().ok_or_else(|| KunaError::sleigh("applyCommits: null space"))?,
                    h.offset_offset,
                )
            } else {
                let mut hand = FixedHandle::default();
                sym.get_fixed_handle(&mut hand, &walker, table)?;
                Address::new(
                    hand.space.ok_or_else(|| KunaError::sleigh("applyCommits: null space"))?,
                    hand.offset_offset,
                )
            };
            let commitaddr = if commitaddr.is_constant() {
                // Convert the constant-space offset into the current space.
                let ws = pos.addr.get_space().expect("addr space").get_word_size();
                let newoff = AddrSpace::address_to_byte(commitaddr.get_offset(), ws);
                Address::new(Rc::clone(pos.addr.get_space().expect("addr space")), newoff)
            } else {
                commitaddr
            };
            let db = &mut **self.context_db.borrow_mut();
            let mut cache = self.cache.borrow_mut();
            if set.flow {
                cache.set_context(db, &commitaddr, set.num, set.mask, set.value);
            } else {
                let nextaddr = &commitaddr + 1;
                if nextaddr.get_offset() < commitaddr.get_offset() {
                    cache.set_context(db, &commitaddr, set.num, set.mask, set.value);
                } else {
                    cache.set_context_region(
                        db, &commitaddr, &nextaddr, set.num, set.mask, set.value,
                    );
                }
            }
        }
        Ok(())
    }

    /// C++ `ParserContext::getN2addr`: address of the instruction after next.
    fn compute_n2addr(&self, pos: &ParserContext) -> KunaResult<Address> {
        if pos.parsestate == ParseState::Uninitialized {
            return Err(KunaError::lowlevel("inst_next2 not available in this context"));
        }
        let length = self.instruction_length(&pos.naddr)?;
        Ok(&pos.naddr + i64::from(length))
    }
}

/// Classify a resolved operand [`FixedHandle`] (Pcode state) into an
/// [`OpObject`] for the instruction-mask accessor.  A constant-space handle is
/// a scalar immediate; a register-space handle is a register at its offset;
/// anything else is a code/data address.  `is_code_address` carries SLEIGH's
/// own marking for scalars that denote a code address.
fn classify_handle(hand: &FixedHandle, is_code_address: bool) -> OpObject {
    match hand.space.as_ref() {
        Some(space) if space.get_type() == spacetype::IPTR_CONSTANT => OpObject::Scalar {
            // The handle offset holds the resolved constant; reinterpret as
            // signed (FID's operand sub-hash truncates to i32 downstream).
            signed: hand.offset_offset as i64,
            whole_operand: true,
            is_address: is_code_address,
        },
        Some(space) if space.get_name() == "register" => {
            OpObject::Register { offset: hand.offset_offset }
        }
        Some(_) => OpObject::Address,
        // No resolved space (dynamic/unresolved handle): treat as an address —
        // FID hashes it as a non-scalar placeholder.
        None => OpObject::Address,
    }
}

/// Helper: borrow a [`SubtableSymbol`] by symbol id (C++ blind cast).
fn subtable_ref(table: &SymbolTable, id: u32) -> KunaResult<&crate::slghsymbol::SubtableSymbol> {
    let sym = table
        .find_symbol_by_id(id)
        .ok_or_else(|| KunaError::sleigh("subtable id undefined"))?;
    match sym.kind() {
        SymbolKind::Subtable(s) => Ok(s),
        _ => Err(KunaError::sleigh("symbol is not a subtable")),
    }
}

/// C++ `TripleSymbol::resolve`, plus (kuna-only) the matched `DisjointPattern`
/// leaf alongside the constructor id: `(Some(ct_id), Some(pattern))` for a
/// subtable triple, `(None, None)` for the table-driven validators / base
/// no-ops.  Identical decision walk to upstream `TripleSymbol::resolve`; the
/// pattern is captured for the FID instruction-mask accessor and changes no
/// decode behavior.
fn resolve_triple_matched(
    table: &SymbolTable,
    id: u32,
    walker: &ParserWalker<'_>,
) -> KunaResult<(Option<u32>, Option<DisjointPattern>)> {
    let sym = table
        .find_symbol_by_id(id)
        .ok_or_else(|| KunaError::sleigh("triple symbol undefined"))?;
    sym.resolve_matched(walker)
}

/// C++ `Constructor::applyContext` via the symbol table + mutating walker.
fn apply_context(
    table: &SymbolTable,
    ct: ConstructorRef,
    walker: &mut ParserWalkerChange<'_>,
) -> KunaResult<()> {
    // The context commands are owned by the constructor; cloning them keeps
    // the symbol-table borrow disjoint from the &mut walker.
    let changes = table.get_constructor(ct)?.get_context_changes().to_vec();
    for change in &changes {
        change.apply(walker)?;
    }
    Ok(())
}

/// The constant space of a parser context.
fn pos_const_space(pos: &ParserContext) -> Rc<AddrSpace> {
    pos.get_const_space()
}


// ParserWalkerChange implements the full walker boundary (reads delegate to a
// transient ParserWalker view; mutations go straight to the context).
impl PatternExpressionContext for ParserWalkerChange<'_> {
    fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        let off = self.ctx.state[self.point()].offset;
        self.ctx.get_instruction_bytes(byteoff, numbytes, off)
    }
    fn get_context_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        self.ctx.get_context_bytes(byteoff, numbytes)
    }
    fn get_addr(&self) -> Address {
        self.ctx.get_addr().clone()
    }
    fn get_naddr(&self) -> Address {
        self.ctx.get_naddr().clone()
    }
    fn get_n2addr(&self) -> KunaResult<Address> {
        self.engine.compute_n2addr(self.ctx)
    }
    fn operand_value(&self, index: i32, table_id: u32, ct_id: u32) -> KunaResult<i64> {
        self.as_reader().operand_value(index, table_id, ct_id)
    }
}

impl SymbolWalker for ParserWalkerChange<'_> {
    fn push_operand(&mut self, i: i32) -> KunaResult<()> {
        ParserWalkerChange::push_operand(self, i)
    }
    fn pop_operand(&mut self) -> KunaResult<()> {
        ParserWalkerChange::pop_operand(self);
        Ok(())
    }
    fn get_constructor(&self) -> KunaResult<ConstructorRef> {
        ParserWalkerChange::get_constructor(self)
    }
    fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle> {
        let pt = &self.ctx.state[self.point()];
        let child = pt.resolve[i as usize]
            .ok_or_else(|| KunaError::sleigh("getFixedHandle: operand not resolved"))?;
        Ok(self.ctx.state[child].hand.clone())
    }
    fn get_const_space(&self) -> Rc<AddrSpace> {
        self.ctx.get_const_space()
    }
    fn get_cur_space(&self) -> Rc<AddrSpace> {
        self.ctx.get_cur_space()
    }
    fn get_dest_addr(&self) -> KunaResult<Address> {
        Ok(self.ctx.calladdr.clone())
    }
    fn get_ref_addr(&self) -> KunaResult<Address> {
        Ok(self.ctx.calladdr.clone())
    }
    fn get_instruction_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
        let off = self.ctx.state[self.point()].offset;
        self.ctx.get_instruction_bits(startbit, size, off)
    }
    fn get_context_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
        self.ctx.get_context_bits(startbit, size)
    }
}

impl SymbolWalkerChange for ParserWalkerChange<'_> {
    fn set_context_word(&mut self, i: i32, val: u32, mask: u32) {
        self.ctx.set_context_word(i, val, mask);
    }
    fn add_commit(&mut self, symbol_id: u32, num: i32, mask: u32, flow: bool) {
        let point = self.point();
        self.ctx.add_commit(symbol_id, num, mask, flow, point);
    }
}

// ---------------------------------------------------------------------------
// Translate / RegisterLookup impls
// ---------------------------------------------------------------------------

impl Sleigh {
    /// C++ `Translate::instructionLength`.
    pub fn instruction_length(&self, baseaddr: &Address) -> KunaResult<i32> {
        let pos = self.obtain_context(baseaddr, ParseState::Disassembly)?;
        Ok(pos.get_length())
    }

    /// Decode the instruction at `baseaddr` and return its *fixed-bit mask*
    /// (the FID / AIF fingerprinting prerequisite).  This re-uses the proven
    /// decode (`obtain_context`) verbatim — it adds no decode behavior and
    /// changes none — and then *reads* the resolved constructor tree to compute
    /// which encoding bits SLEIGH pinned to a constant (opcode/addressing-mode)
    /// versus which carry operand values.
    ///
    /// `fixed_mask[i]` has a 1 wherever a matched constructor pattern fixes a
    /// bit; `operand_mask = !fixed_mask`.  The operand views carry, per operand,
    /// the operand's value-bit mask and the classified [`OpObject`]s (register /
    /// scalar / address) from the resolved [`FixedHandle`]s.
    ///
    /// Must live on `Sleigh` because `obtain_context`/`ParserContext`/
    /// `ConstructState`/`ParserWalker` are all private to this module.
    pub fn instruction_mask(&self, baseaddr: &Address) -> KunaResult<InsnMask> {
        // Pcode state so operand handles are resolved (classification reads
        // them); the fixed-mask tree walk works at either parse state.
        let pos = self.obtain_context(baseaddr, ParseState::Pcode)?;
        let len = pos.get_length();
        if len <= 0 || len > MAX_INSTRUCTION_LEN {
            return Err(KunaError::bad_data(format!(
                "instruction_mask: out-of-range length {len} (cap {MAX_INSTRUCTION_LEN})"
            )));
        }
        let ulen = len as usize;
        // The raw bytes are already in the context buffer (loadFill'd by
        // `resolve`); copy from there rather than re-reading the loadimage.
        let bytes: Vec<u8> = pos.buf[..ulen].to_vec();

        let table = &self.base.symtab;
        let mut fixed = vec![0u8; ulen];

        // --- Tree walk: OR each matched constructor's fixed bits into the mask.
        // Iterate the resolved ConstructState arena reachable from base_state
        // (each populated node holds the constructor that matched, its absolute
        // byte `offset`, and — captured DURING decode under the correct per-node
        // multi-phase context — its matched `DisjointPattern` leaf).  For every
        // such node we read the stashed pattern and OR its fixed bits.  Context
        // bits are NEVER folded in (`context=false`), since the context stream
        // has no instruction-byte position.
        //
        // Reading the decode-time pattern (rather than re-walking the decision
        // tree post-decode) is the FID PR1 fix: the post-decode re-walk reads
        // the *final* parser context (`instrPhase` already advanced past the
        // x86 REX / prefix phases) and misresolves multi-phase encodings; the
        // stashed pattern is the leaf decode actually matched.
        let mut stack = vec![pos.base_state];
        let mut operands: Vec<OperandView> = Vec::new();
        while let Some(node_idx) = stack.pop() {
            let node = &pos.state[node_idx];
            let Some(ctref) = node.ct else { continue };
            let node_off = node.offset as usize;

            let dp = node.matched_pattern.as_ref().ok_or_else(|| {
                KunaError::sleigh("instruction_mask: node has no captured pattern")
            })?;
            // Pattern length is relative to the node start (already offset-
            // resolved by decode); OR byte-by-byte, capping at the instruction
            // length (multi-word patterns return <=32 bits per get_mask call).
            let patlen = dp.get_length(false); // false == instruction stream only
            let mut b = 0i32;
            while b < patlen {
                let abs = node_off + b as usize;
                if abs >= ulen {
                    break; // never write past the decoded instruction
                }
                // get_mask(startbit,size,context): startbit is bit position
                // relative to the node start; context=false reads the
                // instruction-byte mask. Take 8 bits per byte.
                let m = dp.get_mask(b * 8, 8, false) as u8;
                fixed[abs] |= m;
                b += 1;
            }

            // --- Operand classification for this constructor's operands.
            let ct = table.get_constructor(ctref)?;
            let numoper = ct.get_num_operands();
            for oper in 0..numoper {
                let opid = ct.get_operand(oper)?;
                let opsym = table
                    .find_symbol_by_id(opid)
                    .ok_or_else(|| KunaError::sleigh("instruction_mask: operand undefined"))?;
                let SymbolKind::Operand(op) = opsym.kind() else {
                    return Err(KunaError::sleigh("instruction_mask: not an operand symbol"));
                };

                // Compute the operand's byte span within the instruction.  An
                // operand defined by a subtable lives in its own child node (we
                // visit that child separately for the fixed bits); the operand
                // *value* span is [child.offset, child.offset+child.length) when
                // resolved, else the constructor-relative [node_off+rel, +min).
                let child_idx = node.resolve.get(oper as usize).copied().flatten();
                let (span_off, span_len) = match child_idx {
                    Some(ci) => {
                        let c = &pos.state[ci];
                        (c.offset as usize, c.length.max(0) as usize)
                    }
                    None => {
                        // A local/expression operand (no resolved child node):
                        // approximate its span as constructor-relative
                        // [node_off + reloffset, +minimumLength).  This feeds
                        // FID's operand sub-hash; the fixed_mask (above) is the
                        // load-bearing output.
                        let off = node_off + op.get_relative_offset() as usize;
                        (off, op.get_minimum_length().max(0) as usize)
                    }
                };

                // The operand's value mask = bits in its span NOT pinned fixed.
                // (We defer the actual fixed bits of sub-constructor operands to
                // their own node visit; here we expose the complement over the
                // span, which is what FID's operand sub-hash consumes.)
                let mut value_mask = vec![0u8; ulen];
                for i in 0..span_len {
                    let abs = span_off + i;
                    if abs < ulen {
                        value_mask[abs] = !fixed[abs];
                    }
                }

                // Classify the resolved handle (Pcode state) into an OpObject.
                let mut objects: Vec<OpObject> = Vec::new();
                if let Some(ci) = child_idx {
                    let hand = &pos.state[ci].hand;
                    objects.push(classify_handle(hand, op.is_code_address()));
                }
                operands.push(OperandView { value_mask, objects });
            }

            // Descend into resolved children (subtable operands).
            for child in node.resolve.iter().flatten() {
                stack.push(*child);
            }
        }

        Ok(InsnMask { bytes, fixed_mask: fixed, length: len, operands })
    }

    /// C++ `Translate::printAssembly`.
    pub fn print_assembly(
        &self,
        emit: &mut dyn AssemblyEmit,
        baseaddr: &Address,
    ) -> KunaResult<i32> {
        let pos = self.obtain_context(baseaddr, ParseState::Disassembly)?;
        let table = &self.base.symtab;
        let mut walker = ParserWalker::new(&pos, table, self);
        walker.base_state();
        let ct = walker.get_constructor_inner()?;
        let mut mons = String::new();
        table.get_constructor(ct)?.print_mnemonic(&mut mons, &mut walker, table)?;
        let mut body = String::new();
        table.get_constructor(ct)?.print_body(&mut body, &mut walker, table)?;
        emit.dump(baseaddr, &mons, &body);
        Ok(pos.get_length())
    }

    /// C++ `Translate::oneInstruction`.
    pub fn one_instruction(
        &self,
        emit: &mut dyn PcodeEmit,
        baseaddr: &Address,
    ) -> KunaResult<i32> {
        let alignment = self.base.base.get_alignment();
        // C++ `(baseaddr.getOffset() % alignment) != 0`; clippy prefers the
        // is_multiple_of phrasing (alignment is a small positive int4).
        if alignment != 1 && !baseaddr.get_offset().is_multiple_of(alignment as u64) {
            let mut s = String::new();
            baseaddr.print_raw(&mut s)?;
            return Err(KunaError::unimpl(format!("Instruction address not aligned: {s}"), 0));
        }
        let mut pos = self.obtain_context(baseaddr, ParseState::Pcode)?;
        self.apply_commits(&pos)?;
        let mut fall_offset = pos.get_length();

        // Resolve all delay-slot contexts up front (the C++ caches them).
        let mut contexts: Vec<ResolvedCtx> = Vec::new();
        if pos.get_delay_slot() > 0 {
            let mut bytecount = 0i32;
            loop {
                let delayaddr = &pos.addr + i64::from(fall_offset);
                let mut delaypos = self.obtain_context(&delayaddr, ParseState::Pcode)?;
                self.apply_commits(&delaypos)?;
                let len = delaypos.get_length();
                delaypos.parsestate = ParseState::Pcode;
                contexts.push(ResolvedCtx { addr: delayaddr, ctx: delaypos });
                fall_offset += len;
                bytecount += len;
                if bytecount >= pos.get_delay_slot() {
                    break;
                }
            }
            let naddr = &pos.addr + i64::from(fall_offset);
            pos.set_naddr(naddr);
        }
        let main_addr = pos.addr.clone();
        // Index 0 is the main instruction.
        contexts.insert(0, ResolvedCtx { addr: main_addr, ctx: pos });

        let table = &self.base.symtab;
        let const_space = self.const_space();
        let uniq_space = self.unique_space();
        let umask = self.base.unique_allocatemask;
        let mut cache = PcodeCacher::new();
        // walker over the main context, baseState
        let mut builder = SleighBuilder {
            labelbase: 0,
            labelcount: 0,
            cur: WalkCursor::new(),
            cur_ctx: 0,
            cross_ctx: None,
            contexts: &contexts,
            table,
            templates: &self.base.templates,
            engine: self,
            const_space: Rc::clone(&const_space),
            uniq_space,
            uniquemask: umask,
            uniqueoffset: (contexts[0].addr.get_offset() & u64::from(umask)) << 8,
            cache: &mut cache,
        };
        builder.cur.point = Some(contexts[0].ctx.base_state);
        let main_ct = builder.walker().get_constructor_inner()?;
        let handle = table.get_constructor(main_ct)?.get_templ();
        let tpl = handle.map(|h| self.base.templates[h].clone());
        let build_res = (|| -> KunaResult<()> {
            builder.build(tpl.as_ref(), -1)?;
            Ok(())
        })();
        match build_res {
            Ok(()) => {}
            Err(KunaError::Unimpl { .. }) => {
                // C++ rethrows with a descriptive message + instruction_length.
                let mut s = String::from("Instruction not implemented in pcode:\n ");
                // baseState the current walker, print the constructor.
                let cw = builder.walker();
                // reset to base for the message
                let mut basewalker = ParserWalker::new(&contexts[builder.cur_ctx].ctx, table, self);
                basewalker.base_state();
                let bct = basewalker.get_constructor_inner()?;
                let mut a = String::new();
                contexts[builder.cur_ctx].addr.print_raw(&mut a)?;
                s.push_str(&a);
                s.push_str(": ");
                let mut mon = String::new();
                table.get_constructor(bct)?.print_mnemonic(&mut mon, &mut basewalker, table)?;
                s.push_str(&mon);
                s.push_str("  ");
                let mut bod = String::new();
                table.get_constructor(bct)?.print_body(&mut bod, &mut basewalker, table)?;
                s.push_str(&bod);
                let _ = cw;
                return Err(KunaError::unimpl(s, fall_offset));
            }
            Err(e) => return Err(e),
        }
        cache.resolve_relatives()?;
        cache.emit(baseaddr, emit, &self.base.manager);
        Ok(fall_offset)
    }
}

impl RegisterLookup for Sleigh {
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
        let vd = self.base.get_register(nm.as_bytes())?;
        Ok(storage_from_varnode_data(&vd))
    }
    fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        String::from_utf8_lossy(&self.base.get_register_name(base, off, size)).into_owned()
    }
    fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        String::from_utf8_lossy(&self.base.get_exact_register_name(base, off, size)).into_owned()
    }
}

impl Translate for Sleigh {
    fn translate_base(&self) -> &TranslateBase {
        &self.base.base
    }
    fn translate_base_mut(&mut self) -> &mut TranslateBase {
        &mut self.base.base
    }
    fn initialize(&mut self, _store: &mut DocumentStorage) -> KunaResult<()> {
        // The .sla file is supplied via initialize_from_sla; the DocumentStorage
        // path is the C++ console wiring (not needed for the lift gate).
        Err(KunaError::lowlevel("use Sleigh::initialize_from_sla"))
    }
    fn register_context(&mut self, name: &str, sbit: i32, ebit: i32) {
        let _ = self
            .context_db
            .borrow_mut()
            .register_variable(name.as_bytes(), sbit, ebit);
    }
    fn set_context_default(&mut self, name: &str, val: u32) {
        let _ = self.context_db.borrow_mut().set_variable_default(name.as_bytes(), val);
    }
    fn allow_context_set(&self, val: bool) {
        self.cache.borrow_mut().allow_set(val);
    }
    #[allow(clippy::mutable_key_type)]
    fn get_all_registers(&self, reglist: &mut std::collections::BTreeMap<VarnodeData, String>) {
        for (vs, name) in self.base.get_all_registers() {
            reglist.insert(
                crate::translate::varnode_data_from_storage(vs),
                String::from_utf8_lossy(name).into_owned(),
            );
        }
    }
    fn get_user_op_names(&self, res: &mut Vec<String>) {
        for nm in self.base.get_user_op_names() {
            res.push(String::from_utf8_lossy(nm).into_owned());
        }
    }
    fn instruction_length(&self, baseaddr: &Address) -> KunaResult<i32> {
        Sleigh::instruction_length(self, baseaddr)
    }
    fn one_instruction(&self, emit: &mut dyn PcodeEmit, baseaddr: &Address) -> KunaResult<i32> {
        Sleigh::one_instruction(self, emit, baseaddr)
    }
    fn print_assembly(&self, emit: &mut dyn AssemblyEmit, baseaddr: &Address) -> KunaResult<i32> {
        Sleigh::print_assembly(self, emit, baseaddr)
    }
}
