//! (kuna) A table of precomputed p-code served in place of decoding bytes.
//!
//! The Pushan devirtualizer builds the protected function's recovered control
//! flow as synthetic instructions at addresses no section maps; the flow
//! follower lifts them through the ordinary [`Translate`](crate::translate::Translate)
//! calls, so a translator consults its overlay before touching the image.  An
//! entry is one synthetic instruction: its p-code, its length (the distance
//! to the next synthetic address) and a rendering for listings.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::translate::{AssemblyEmit, PcodeEmit};

#[derive(Clone, Debug)]
pub struct OverlayOp {
    pub opc: OpCode,
    pub out: Option<VarnodeData>,
    pub ins: Vec<VarnodeData>,
}

#[derive(Clone, Debug)]
pub struct OverlayInsn {
    pub ops: Vec<OverlayOp>,
    pub length: i32,
    pub mnemonic: String,
    pub body: String,
}

#[derive(Default, Debug)]
pub struct PcodeOverlay {
    /// Index of the address space the synthetic instructions live in.
    pub space_index: i32,
    pub insns: BTreeMap<u64, OverlayInsn>,
}

impl PcodeOverlay {
    pub fn new(space_index: i32) -> Self {
        PcodeOverlay { space_index, insns: BTreeMap::new() }
    }

    pub fn get(&self, addr: &Address) -> Option<&OverlayInsn> {
        let sp = addr.get_space()?;
        if sp.get_index() != self.space_index {
            return None;
        }
        self.insns.get(&addr.get_offset())
    }

    pub fn contains(&self, addr: &Address) -> bool {
        self.get(addr).is_some()
    }

    pub fn len(&self) -> usize {
        self.insns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.insns.is_empty()
    }

    pub fn emit(&self, insn: &OverlayInsn, emit: &mut dyn PcodeEmit, addr: &Address) {
        for op in &insn.ops {
            emit.dump(addr, op.opc, op.out.as_ref(), &op.ins);
        }
    }

    pub fn emit_assembly(&self, insn: &OverlayInsn, emit: &mut dyn AssemblyEmit, addr: &Address) {
        emit.dump(addr, &insn.mnemonic, &insn.body);
    }
}
