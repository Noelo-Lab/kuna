//! Summarize the verified AMD64 Rust stack probe before SSA and call setup.
//! `rustprobestack` abstracts page-touch faults, but preserves the return slot,
//! saved RBP, restored SP, and the helper's observable R11 residual. The next
//! instruction must overwrite the arithmetic flags with `sub rsp,rax`.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_x64syscall::overlaps;

const PROBE: [u8; 57] = [
    0x55, 0x48, 0x89, 0xe5, 0x49, 0x89, 0xc3, 0x49, 0x81, 0xfb, 0x00, 0x10, 0x00, 0x00, 0x76, 0x1c,
    0x48, 0x81, 0xec, 0x00, 0x10, 0x00, 0x00, 0x48, 0x85, 0x64, 0x24, 0x08, 0x49, 0x81, 0xeb, 0x00,
    0x10, 0x00, 0x00, 0x49, 0x81, 0xfb, 0x00, 0x10, 0x00, 0x00, 0x77, 0xe4, 0x4c, 0x29, 0xdc, 0x48,
    0x85, 0x64, 0x24, 0x08, 0x48, 0x01, 0xc4, 0xc9, 0xc3,
];
const MAX_FRAME: u64 = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RustProbestackMode {
    #[default]
    Off,
    Auto,
    Always,
}

impl RustProbestackMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Auto => "auto",
            Self::Always => "always",
        }
    }
}

pub fn parse_mode(value: &str) -> Result<(RustProbestackMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => RustProbestackMode::Off,
        "auto" | "" => RustProbestackMode::Auto,
        "always" | "on" | "1" | "true" => RustProbestackMode::Always,
        other => {
            return Err(format!(
                "unknown rustprobestack value {other:?} (expected off, auto or always)"
            ))
        }
    };
    Ok((
        mode,
        format!("Rust stack-probe summary set to {}", mode.as_str()),
    ))
}

struct Registers {
    rax: Address,
    rsp: Address,
    rbp: Address,
    r11: Address,
}

fn registers(data: &Funcdata) -> Option<Registers> {
    let manage = data.get_arch().manage();
    let code = manage.get_default_code_space()?;
    if code.get_addr_size() != 8 || code.is_big_endian() || code.get_word_size() != 1 {
        return None;
    }
    let lookup = manage.register_lookup()?;
    let reg = |name, offset| {
        let storage = lookup.probe_register(name)?;
        let space = storage.space?;
        if storage.size != 8
            || storage.offset != offset
            || space.get_name() != "register"
            || space.is_big_endian()
        {
            return None;
        }
        Some(Address::new(space, storage.offset))
    };
    let regs = Registers {
        rax: reg("RAX", 0)?,
        rsp: reg("RSP", 0x20)?,
        rbp: reg("RBP", 0x28)?,
        r11: reg("R11", 0x98)?,
    };
    let stack = manage.get_stack_space()?;
    let base = stack.get_spacebase(0).ok()?;
    if stack.get_addr_size() != 8
        || stack.num_spacebase() != 1
        || !stack.stack_grows_negative()
        || base.size != 8
        || Address::new(base.space.clone()?, base.offset) != regs.rsp
    {
        return None;
    }
    Some(regs)
}

fn reg_is(data: &Funcdata, vn: Option<VarnodeId>, addr: &Address) -> bool {
    vn.and_then(|v| data.vbank().get(v))
        .is_some_and(|v| v.get_size() == 8 && v.get_addr() == addr)
}

fn constant(data: &Funcdata, vn: Option<VarnodeId>) -> Option<u64> {
    let v = data.vbank().get(vn?)?;
    (v.is_constant() && v.get_size() == 8).then_some(v.get_offset())
}

fn frame_before(data: &Funcdata, call: OpId, regs: &Registers) -> Option<u64> {
    let block = data.obank().get(call)?.get_parent()?;
    let mut cur = call;
    for _ in 0..48 {
        cur = data.op_previous_op(cur)?;
        let op = data.obank().get(cur)?;
        if op.get_parent() != Some(block) {
            return None;
        }
        match op.code() {
            OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_RETURN => return None,
            _ => {}
        }
        if let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) {
            if overlaps(out.get_addr(), out.get_size(), &regs.rax, 8) {
                if op.code() != OpCode::CPUI_COPY
                    || op.num_input() != 1
                    || !reg_is(data, op.get_out(), &regs.rax)
                {
                    return None;
                }
                return constant(data, op.get_in(0)).filter(|v| *v <= MAX_FRAME);
            }
        }
    }
    None
}

fn read_bytes(data: &Funcdata, address: &Address, bytes: &mut [u8]) -> Option<()> {
    let arch = data.get_arch();
    if address.get_space()?.get_index() != arch.manage().get_default_code_space()?.get_index() {
        return None;
    }
    let image = arch.loader.as_ref()?.borrow().shared_bytes()?;
    let start = address.get_offset();
    if !image.mapped_covers(start, start.checked_add(bytes.len() as u64)?) {
        return None;
    }
    (image.fill_span(bytes, start) == 0).then_some(())
}

fn matches_call(data: &Funcdata, call: OpId, target: &Address, regs: &Registers) -> Option<u64> {
    let op = data.obank().get(call)?;
    let pc = op.get_addr();
    let block = op.get_parent()?;
    if op.code() != OpCode::CPUI_CALL
        || op.num_input() != 1
        || op.get_out().is_some()
        || op.is_dead()
        || pc.get_space()?.get_index() != target.get_space()?.get_index()
    {
        return None;
    }
    let frame = frame_before(data, call, regs)?;
    let mut site = [0; 8];
    read_bytes(data, pc, &mut site)?;
    if site[0] != 0xe8 || site[5..] != [0x48, 0x29, 0xc4] {
        return None;
    }
    let next = pc.get_offset().checked_add(5)?;
    let displacement = i32::from_le_bytes(site[1..5].try_into().ok()?) as i64;
    if next.wrapping_add(displacement as u64) != target.get_offset() {
        return None;
    }
    let store_id = data.op_previous_op(call)?;
    let store = data.obank().get(store_id)?;
    if store.code() != OpCode::CPUI_STORE
        || store.num_input() != 3
        || store.get_out().is_some()
        || store.get_addr() != pc
        || store.get_parent() != Some(block)
        || !reg_is(data, store.get_in(1), &regs.rsp)
        || constant(data, store.get_in(2)) != Some(next)
    {
        return None;
    }
    let space = data.vbank().get(store.get_in(0)?)?;
    if !space.is_constant() || space.get_offset() != pc.get_space()?.get_index() as u64 {
        return None;
    }
    let push = data.obank().get(data.op_previous_op(store_id)?)?;
    if push.code() != OpCode::CPUI_INT_SUB
        || push.num_input() != 2
        || push.get_addr() != pc
        || push.get_parent() != Some(block)
        || !reg_is(data, push.get_out(), &regs.rsp)
        || !reg_is(data, push.get_in(0), &regs.rsp)
        || constant(data, push.get_in(1)) != Some(8)
    {
        return None;
    }
    let mut cur = call;
    for opcode in [
        OpCode::CPUI_INT_LESS,
        OpCode::CPUI_INT_SBORROW,
        OpCode::CPUI_INT_SUB,
    ] {
        cur = data.op_next_op(cur)?;
        let part = data.obank().get(cur)?;
        if part.code() != opcode
            || part.num_input() != 2
            || part.get_parent() != Some(block)
            || part.get_addr().get_offset() != next
            || part.get_addr().get_space()?.get_index() != pc.get_space()?.get_index()
            || !reg_is(data, part.get_in(0), &regs.rsp)
            || !reg_is(data, part.get_in(1), &regs.rax)
        {
            return None;
        }
        if opcode == OpCode::CPUI_INT_SUB && !reg_is(data, part.get_out(), &regs.rsp) {
            return None;
        }
    }
    let mut body = [0; PROBE.len()];
    read_bytes(data, target, &mut body)?;
    (body == PROBE).then_some(frame)
}

fn rewrite(data: &mut Funcdata, call: OpId, frame: u64, regs: &Registers) -> KunaResult<()> {
    let pc = data
        .obank()
        .get(call)
        .expect("matched call")
        .get_addr()
        .clone();
    let address = data.new_op(2, pc.clone());
    data.op_set_opcode(address, crate::typeop::type_op_for(OpCode::CPUI_INT_SUB));
    let sp = data.new_varnode(8, &regs.rsp, None);
    let eight = data.new_constant(8, 8);
    data.op_set_input(address, sp, 0)?;
    data.op_set_input(address, eight, 1)?;
    let saved = data.new_unique_out(8, address)?;
    data.op_insert_before(address, call);
    let store = data.new_op(3, pc.clone());
    data.op_set_opcode(store, crate::typeop::type_op_for(OpCode::CPUI_STORE));
    let ram = data.new_varnode_space(pc.get_space().expect("matched code space"));
    let rbp = data.new_varnode(8, &regs.rbp, None);
    data.op_set_input(store, ram, 0)?;
    data.op_set_input(store, saved, 1)?;
    data.op_set_input(store, rbp, 2)?;
    data.obank_mut()
        .get_mut(store)
        .expect("probe spill")
        .set_additional_flag(crate::op::pcodeop_addlflags::kuna_rustprobe_spill);
    data.op_insert_before(store, call);
    let pop = data.new_op(2, pc.clone());
    data.op_set_opcode(pop, crate::typeop::type_op_for(OpCode::CPUI_INT_ADD));
    let sp = data.new_varnode(8, &regs.rsp, None);
    let eight = data.new_constant(8, 8);
    data.op_set_input(pop, sp, 0)?;
    data.op_set_input(pop, eight, 1)?;
    data.new_varnode_out(8, &regs.rsp, pop)?;
    data.op_insert_before(pop, call);
    let result = data.new_op(1, pc);
    data.op_set_opcode(result, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    let residual = if frame == 0 {
        0
    } else {
        (frame - 1) % 4096 + 1
    };
    let value = data.new_constant(8, residual);
    data.op_set_input(result, value, 0)?;
    data.new_varnode_out(8, &regs.r11, result)?;
    data.op_insert_before(result, call);
    data.delete_call_specs(call);
    data.op_destroy(call);
    Ok(())
}

pub struct ActionRustProbestack {
    base: ActionBase,
}

impl ActionRustProbestack {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(
                crate::action::ruleflags::rule_onceperfunc,
                "rustprobestack",
                group,
            ),
        })
    }
}

impl Action for ActionRustProbestack {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(|| {
            Box::new(Self {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        match data.get_arch().rust_probestack {
            RustProbestackMode::Off => return 0,
            RustProbestackMode::Auto if !data.get_arch().source_lang.is_rust() => return 0,
            _ => {}
        }
        if data.get_heritage_pass() != 0
            || data.num_calls() == 0
            || data.get_arch().stack_views
            || data.get_arch().stack_alias_deadstore
        {
            return 0;
        }
        let Some(regs) = registers(data) else {
            return 0;
        };
        let candidates: Vec<_> = data
            .call_specs()
            .iter()
            .filter_map(|fc| {
                let proto = fc.proto();
                if proto.is_inline()
                    || proto.is_no_return()
                    || proto.get_inject_id() >= 0
                    || (proto.has_model()
                        && (proto.get_inject_upon_entry() >= 0
                            || proto.get_inject_upon_return() >= 0))
                {
                    return None;
                }
                matches_call(data, fc.get_op(), fc.get_entry_address(), &regs)
                    .map(|frame| (fc.get_op(), frame))
            })
            .collect();
        for (call, frame) in &candidates {
            rewrite(data, *call, *frame, &regs).expect("verified Rust stack-probe summary");
            self.base.count += 1;
        }
        i32::from(!candidates.is_empty())
    }
}
