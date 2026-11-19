//! The short returning leaf body admitted by automatic Go ABI0 binding.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

const MAX_BODY_BYTES: usize = 256;

/// File-backed evidence whose instructions must be checked after loader attachment.
#[derive(Clone, Debug)]
pub struct LeafStackInputs {
    pub(super) byte_limit: u32,
    body: Box<[u8]>,
}

impl LeafStackInputs {
    pub(super) fn new(byte_limit: u32, body: &[u8]) -> Option<Self> {
        if body.is_empty() || body.len() > MAX_BODY_BYTES {
            return None;
        }
        Some(Self {
            byte_limit,
            body: body.into(),
        })
    }

    /// The stack limit is usable only if the attached image proves the leaf body.
    pub fn proven_limit(&self, arch: &Architecture, start: u64) -> Option<u32> {
        prove(arch, start, &self.body)?;
        Some(self.byte_limit)
    }
}

struct Instruction<'a> {
    rsp: &'a VarnodeData,
    returns: bool,
    unsupported: bool,
    writes_stack_pointer: bool,
}

impl PcodeEmit for Instruction<'_> {
    fn dump(
        &mut self,
        _address: &Address,
        opcode: OpCode,
        output: Option<&VarnodeData>,
        _inputs: &[VarnodeData],
    ) {
        self.unsupported |= self.returns
            || matches!(
                opcode,
                OpCode::CPUI_BRANCH
                    | OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BRANCHIND
                    | OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_CALLOTHER
            );
        self.returns |= opcode == OpCode::CPUI_RETURN;
        self.writes_stack_pointer |= output.is_some_and(|value| {
            value
                .space
                .as_ref()
                .zip(self.rsp.space.as_ref())
                .is_some_and(|(out, stack)| {
                    out.get_index() == stack.get_index()
                        && value.offset < self.rsp.offset.saturating_add(u64::from(self.rsp.size))
                        && self.rsp.offset < value.offset.saturating_add(u64::from(value.size))
                })
        });
    }
}

fn prove(arch: &Architecture, start: u64, body: &[u8]) -> Option<()> {
    let code = arch.manage().get_default_code_space()?;
    let rsp = arch.get_register_varnode(b"RSP").ok()?;
    let translate = arch.translate();
    let image = translate.loader_rc().try_borrow().ok()?.shared_bytes()?;
    if !image.mapped_covers(start, start.checked_add(body.len() as u64)?) {
        return None;
    }
    let mut mapped = [0; MAX_BODY_BYTES];
    let mapped = &mut mapped[..body.len()];
    if image.fill_span(mapped, start) != 0 || &*mapped != body {
        return None;
    }
    let _scope = translate.decode_context_scope()?;
    let mut offset = 0;
    while offset < body.len() {
        let mut decoded = Instruction {
            rsp: &rsp,
            returns: false,
            unsupported: false,
            writes_stack_pointer: false,
        };
        let pc = Address::new(Rc::clone(code), start.checked_add(offset as u64)?);
        let length = usize::try_from(
            translate
                .one_instruction_checked(&mut decoded, &pc, image.as_ref())
                .ok()?,
        )
        .ok()?;
        let end = offset.checked_add(length)?;
        let instruction = body.get(offset..end)?;
        if length == 0 || decoded.unsupported {
            return None;
        }
        if decoded.returns {
            return (end == body.len() && instruction == [0xc3]).then_some(());
        }
        if decoded.writes_stack_pointer {
            return None;
        }
        offset = end;
    }
    None
}
