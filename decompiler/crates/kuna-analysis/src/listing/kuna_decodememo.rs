//! (kuna `armframes`) Share decodes across the passes and rebuilds of one frame
//! recovery.
//!
//! Frame validation, the AIF gap walks, prefix reconciliation and every Listing
//! rebuild decode the same image again, almost all of it exactly as before. A
//! decode is kept only when SLEIGH certifies it context-free, together with the
//! effective decode context it read; it is reused only while the address still
//! reads that same context, so a reused record is the record a fresh decode
//! makes. A context-free decode commits no context, so a reused one reports no
//! commits either.

use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use super::classify::{classify, Classified};
use super::decode::{decode_one_listing, DecodedListing};
use super::kuna_decodereuse::ReusableDecode;
use super::model::{FlowType, Mnemonic};
use super::walk::StepCtx;

/// The decodes one frame recovery has certified, by address.
#[derive(Default)]
pub(crate) struct DecodeMemo {
    entries: RefCell<HashMap<u64, Memo>>,
    contexts: RefCell<Contexts>,
    reused: Cell<bool>,
}

/// The distinct decode contexts, which a whole image shares a handful of.
#[derive(Default)]
struct Contexts {
    ids: HashMap<Box<[u32]>, u32>,
    reuse: Vec<Rc<ReusableDecode>>,
}

/// One certified decode.
pub(crate) struct Memo {
    context: u32,
    pub(crate) len: u32,
    pub(crate) flow: FlowType,
    pub(crate) fall_through: Option<u64>,
    pub(crate) flows: Box<[u64]>,
    pub(crate) mnemonic: Arc<str>,
    pub(crate) operands: Box<str>,
}

impl DecodeMemo {
    /// Did the last walk [`decode`] reuse a record instead of decoding?
    pub(super) fn reused(&self) -> bool {
        self.reused.get()
    }

    /// The record for `vma`, when its address still reads the context it was
    /// decoded in.
    pub(crate) fn get(&self, translate: &dyn Translate, space: &Rc<AddrSpace>, vma: u64) -> Option<Ref<'_, Memo>> {
        let memo = Ref::filter_map(self.entries.borrow(), |entries| entries.get(&vma)).ok()?;
        let reuse = Rc::clone(&self.contexts.borrow().reuse[memo.context as usize]);
        reuse.matches(translate, &Address::new(Rc::clone(space), vma)).then_some(memo)
    }

    /// The context-only reuse record `memo` was certified under.
    pub(crate) fn reusable(&self, memo: &Memo) -> Rc<ReusableDecode> {
        Rc::clone(&self.contexts.borrow().reuse[memo.context as usize])
    }

    /// Keep a decode certified under `context`, answering with the shared
    /// context-only reuse record for it.
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
    ) -> Rc<ReusableDecode> {
        let context = self.context_id(context);
        let reuse = Rc::clone(&self.contexts.borrow().reuse[context as usize]);
        self.entries.borrow_mut().insert(
            vma,
            Memo {
                context,
                len,
                flow,
                fall_through,
                flows: flows.into(),
                mnemonic: mnemonic.into(),
                operands: operands.into(),
            },
        );
        reuse
    }

    fn context_id(&self, context: &[u32]) -> u32 {
        let mut contexts = self.contexts.borrow_mut();
        if let Some(&id) = contexts.ids.get(context) {
            return id;
        }
        let id = contexts.reuse.len() as u32;
        contexts.reuse.push(Rc::new(ReusableDecode::context_only(context.to_vec())));
        contexts.ids.insert(context.into(), id);
        id
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
    if let Some(m) = memo.get(ctx.translate, ctx.code_space, vma) {
        memo.reused.set(true);
        let decoded = DecodedListing {
            len: m.len,
            ops: Vec::new(),
            mnemonic: intern(&m.mnemonic),
            operands: m.operands.to_string(),
            stored_scalar_values: Vec::new(),
        };
        let c = Classified { flow: m.flow, flows: m.flows.to_vec(), fall_through: m.fall_through };
        return Some((decoded, c));
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
