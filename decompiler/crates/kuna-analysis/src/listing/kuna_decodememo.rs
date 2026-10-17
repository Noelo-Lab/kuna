//! (kuna `armframes`) Share decodes across the passes and rebuilds of one frame
//! recovery.
//!
//! Frame validation, the AIF gap walks, prefix reconciliation and every Listing
//! rebuild decode the same image again, almost all of it exactly as before. A
//! decode is kept only when SLEIGH certifies it context-free, together with the
//! effective decode context it read; it is reused only while the address still
//! reads that same context, so a reused record is the record a fresh decode
//! makes. A context-free decode commits no context, so a reused one reports no
//! commits either. Records are fixed-size slots, one per halfword of executable
//! code, with their text and flow targets in shared arenas, because a whole
//! image's worth of them is alive at once.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use super::classify::{classify, Classified};
use super::decode::{decode_one_listing, DecodedListing};
use super::kuna_decodereuse::ReusableDecode;
use super::model::{FlowKind, FlowType, Mnemonic};
use super::walk::StepCtx;

/// The decodes one frame recovery has certified, in one slot per halfword of
/// each executable range.
pub(crate) struct DecodeMemo {
    ranges: Vec<(u64, u64)>,
    slots: RefCell<Vec<Vec<Memo>>>,
    text: RefCell<String>,
    flows: RefCell<Vec<u64>>,
    contexts: RefCell<Contexts>,
    reused: Cell<bool>,
}

/// The distinct decode contexts, which a whole image shares a handful of.
#[derive(Default)]
struct Contexts {
    ids: HashMap<Box<[u32]>, u8>,
    reuse: Vec<Rc<ReusableDecode>>,
}

/// A record; `len` 0 marks an empty slot.
#[derive(Clone, Copy, Default)]
struct Memo {
    text: u32,
    flows: u32,
    flow: u16,
    context: u8,
    mnemonic: u8,
    operands: u8,
    len: u8,
    nflows: u8,
}

/// One certified decode, borrowed from the memo.
pub(crate) struct Recalled<'a> {
    pub(crate) len: u32,
    pub(crate) flow: FlowType,
    pub(crate) fall_through: Option<u64>,
    pub(crate) flows: &'a [u64],
    pub(crate) mnemonic: &'a str,
    pub(crate) operands: &'a str,
    pub(crate) reusable: &'a Rc<ReusableDecode>,
}

const KINDS: [FlowKind; 9] = [
    FlowKind::Fallthrough,
    FlowKind::UnconditionalBranch,
    FlowKind::ConditionalBranch,
    FlowKind::ComputedJump,
    FlowKind::Call,
    FlowKind::ComputedCall,
    FlowKind::CallOther,
    FlowKind::Return,
    FlowKind::Unimplemented,
];
const FALLS: u16 = 1 << 15;

fn pack(flow: FlowType, falls: bool) -> u16 {
    let kind = KINDS.iter().position(|&k| k == flow.kind).unwrap_or_default() as u16;
    let bits = [flow.is_call, flow.is_jump, flow.is_terminal, flow.is_computed, flow.is_indirect,
        flow.is_conditional, flow.has_fallthrough];
    bits.iter().enumerate().fold(kind, |acc, (i, &on)| acc | (u16::from(on) << (4 + i)))
        | if falls { FALLS } else { 0 }
}

fn unpack(bits: u16) -> (FlowType, bool) {
    let on = |i: u16| bits & (1 << (4 + i)) != 0;
    let flow = FlowType {
        kind: KINDS[usize::from(bits & 0xf)],
        is_call: on(0),
        is_jump: on(1),
        is_terminal: on(2),
        is_computed: on(3),
        is_indirect: on(4),
        is_conditional: on(5),
        has_fallthrough: on(6),
    };
    (flow, bits & FALLS != 0)
}

impl DecodeMemo {
    /// An empty memo over the executable `ranges`; slots are allocated per range
    /// on first use.
    pub(crate) fn new(ranges: &[(u64, u64)]) -> Self {
        DecodeMemo {
            ranges: ranges.to_vec(),
            slots: RefCell::new(vec![Vec::new(); ranges.len()]),
            text: RefCell::default(),
            flows: RefCell::default(),
            contexts: RefCell::default(),
            reused: Cell::new(false),
        }
    }

    /// Did the last walk [`decode`] reuse a record instead of decoding?
    pub(super) fn reused(&self) -> bool {
        self.reused.get()
    }

    fn slot(&self, vma: u64) -> Option<(usize, usize)> {
        if vma % 2 != 0 {
            return None;
        }
        let range = self.ranges.iter().position(|&(lo, hi)| lo <= vma && vma < hi)?;
        Some((range, ((vma - self.ranges[range].0) / 2) as usize))
    }

    /// Hand `f` the record for `vma`, when its address still reads the context
    /// it was decoded in.
    pub(crate) fn recall<R>(
        &self,
        translate: &dyn Translate,
        space: &Rc<AddrSpace>,
        vma: u64,
        f: impl FnOnce(Recalled<'_>) -> R,
    ) -> Option<R> {
        let (range, at) = self.slot(vma)?;
        let memo = *self.slots.borrow()[range].get(at).filter(|memo| memo.len != 0)?;
        let contexts = self.contexts.borrow();
        let reusable = &contexts.reuse[usize::from(memo.context)];
        if !reusable.matches(translate, &Address::new(Rc::clone(space), vma)) {
            return None;
        }
        let (text, flows) = (self.text.borrow(), self.flows.borrow());
        let (flow, falls) = unpack(memo.flow);
        let first = memo.text as usize;
        let ops = first + usize::from(memo.mnemonic);
        let targets = memo.flows as usize;
        Some(f(Recalled {
            len: u32::from(memo.len),
            flow,
            fall_through: falls.then(|| vma.wrapping_add(u64::from(memo.len))),
            flows: &flows[targets..targets + usize::from(memo.nflows)],
            mnemonic: &text[first..ops],
            operands: &text[ops..ops + usize::from(memo.operands)],
            reusable,
        }))
    }

    /// Keep a decode certified under `context`, answering with the shared
    /// context-only reuse record for it. A decode a record cannot hold is not
    /// kept.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn keep(
        &self,
        vma: u64,
        context: &[u32],
        len: u32,
        flow: FlowType,
        fall_through: Option<u64>,
        flows: &[u64],
        mnemonic: &str,
        operands: &str,
    ) -> Option<Rc<ReusableDecode>> {
        let (range, at) = self.slot(vma)?;
        let (Ok(len8), Ok(mnemonic8), Ok(operands8), Ok(nflows)) = (
            u8::try_from(len),
            u8::try_from(mnemonic.len()),
            u8::try_from(operands.len()),
            u8::try_from(flows.len()),
        ) else {
            return None;
        };
        if len8 == 0 || fall_through.is_some_and(|next| next != vma.wrapping_add(u64::from(len))) {
            return None;
        }
        let context = self.context_id(context)?;
        let mut text = self.text.borrow_mut();
        let mut targets = self.flows.borrow_mut();
        let memo = Memo {
            text: u32::try_from(text.len()).ok()?,
            flows: u32::try_from(targets.len()).ok()?,
            flow: pack(flow, fall_through.is_some()),
            context,
            mnemonic: mnemonic8,
            operands: operands8,
            len: len8,
            nflows,
        };
        text.push_str(mnemonic);
        text.push_str(operands);
        targets.extend_from_slice(flows);
        let mut slots = self.slots.borrow_mut();
        let (lo, hi) = self.ranges[range];
        let slots = &mut slots[range];
        if slots.is_empty() {
            slots.resize(hi.saturating_sub(lo).div_ceil(2) as usize, Memo::default());
        }
        slots[at] = memo;
        Some(Rc::clone(&self.contexts.borrow().reuse[usize::from(context)]))
    }

    fn context_id(&self, context: &[u32]) -> Option<u8> {
        let mut contexts = self.contexts.borrow_mut();
        if let Some(&id) = contexts.ids.get(context) {
            return Some(id);
        }
        let id = u8::try_from(contexts.reuse.len()).ok()?;
        contexts.reuse.push(Rc::new(ReusableDecode::context_only(context.to_vec())));
        contexts.ids.insert(context.into(), id);
        Some(id)
    }
}

/// Decode `vma` for the walk and classify it, through the memo when `ctx`
/// carries one. `None` stops the path: undecodable or zero-length.
pub(super) fn decode(
    ctx: &StepCtx<'_>,
    vma: u64,
    intern: &mut impl FnMut(&str) -> Mnemonic,
) -> Option<(DecodedListing, Classified)> {
    let memo = ctx.memo.filter(|_| ctx.detail.assembly && !ctx.policy.stack_callbacks);
    let Some(memo) = memo else {
        let decoded = decode_one_listing(
            ctx.translate, vma, ctx.code_space, ctx.detail.assembly, ctx.policy.stack_callbacks, intern, None,
        )
        .ok()
        .filter(|d| d.len != 0)?;
        let c = classify(&decoded.ops, vma, decoded.len);
        return Some((decoded, c));
    };
    memo.reused.set(false);
    let recalled = memo.recall(ctx.translate, ctx.code_space, vma, |m| {
        let decoded = DecodedListing {
            len: m.len,
            ops: Vec::new(),
            mnemonic: intern(m.mnemonic),
            operands: m.operands.to_string(),
            stored_scalar_values: Vec::new(),
        };
        (decoded, Classified { flow: m.flow, flows: m.flows.to_vec(), fall_through: m.fall_through })
    });
    if recalled.is_some() {
        memo.reused.set(true);
        return recalled;
    }
    let mut key = None;
    let decoded =
        decode_one_listing(ctx.translate, vma, ctx.code_space, true, false, intern, Some(&mut key))
            .ok()
            .filter(|d| d.len != 0)?;
    let c = classify(&decoded.ops, vma, decoded.len);
    if let Some(context) = key {
        memo.keep(vma, &context, decoded.len, c.flow, c.fall_through, &c.flows, &decoded.mnemonic, &decoded.operands);
    }
    Some((decoded, c))
}
