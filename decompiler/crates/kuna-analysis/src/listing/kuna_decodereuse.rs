//! Reuse frame-probe p-code only when translation certifies no context effects.

use super::{
    decode::Decoded,
    model::RawOp,
    xrefs::{FullCapture, FullOp},
};
use kuna_base::{address::Address, error::KunaResult, space::AddrSpace};
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate};
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub(crate) struct ReusableDecode {
    context: Vec<u32>,
    ops: Vec<FullOp>,
}

impl ReusableDecode {
    pub fn matches(&self, translate: &dyn Translate, addr: &Address) -> bool {
        translate.matches_decode_context(addr, &self.context)
    }

    pub fn replay(
        &self,
        at: u64,
        translate: &dyn Translate,
        space: &Rc<AddrSpace>,
        capture: &mut FullCapture,
    ) -> bool {
        let addr = Address::new(Rc::clone(space), at);
        if !self.matches(translate, &addr) {
            return false;
        }
        capture.begin();
        for op in &self.ops {
            capture.dump(&addr, op.opcode, op.out.as_ref(), &op.ins);
        }
        true
    }
}

pub(crate) fn capture(
    translate: &dyn Translate,
    at: u64,
    space: &Rc<AddrSpace>,
    decoded: &mut Decoded,
    full: &mut FullCapture,
) -> KunaResult<Option<Rc<ReusableDecode>>> {
    let addr = Address::new(Rc::clone(space), at);
    decoded.mnemonic.clear();
    decoded.operands.clear();
    full.begin();
    let mut context = None;
    let mut text = Text(&mut decoded.mnemonic, &mut decoded.operands);
    let len = translate.one_instruction_reusable(full, &mut text, &addr, &mut context)?;
    decoded.len = len.max(0) as u32;
    decoded.ops.clear();
    decoded.ops.extend(full.ops().iter().map(|op| RawOp {
        opcode: op.opcode,
        in0: op.ins.first().cloned(),
    }));
    Ok(context.map(|context| {
        Rc::new(ReusableDecode {
            context,
            ops: full.ops().to_vec(),
        })
    }))
}

struct Text<'a>(&'a mut String, &'a mut String);
impl AssemblyEmit for Text<'_> {
    fn dump(&mut self, _: &Address, mnemonic: &str, operands: &str) {
        self.0.clear();
        self.0.push_str(mnemonic);
        self.1.clear();
        self.1.push_str(operands);
    }
}
