//! (kuna `jumpassist`) The `JumpAssisted` jump-table model (`jumptable.cc`).
//!
//! A processor spec can describe a switch whose table the SLEIGH semantics
//! cannot express by itself: the instruction computes its destination through a
//! user-op (Dalvik's `switchAssist`, the JVM's `lookupswitch`), and a
//! `<jumpassist>` element gives the p-code scripts that turn a table index into
//! a case label (`<case_pcode>`), a destination (`<addr_pcode>`), the default
//! destination (`<default_pcode>`) and the table size (`<size_pcode>`).
//!
//! [`JumpAssists`] holds those scripts, compiled once per architecture; each is
//! evaluated by emitting its p-code with the inputs in temporary registers and
//! running it over the load image (`ExecutablePcode::evaluate`).
//! [`JumpModelAssisted`] is the model: it matches a `BRANCHIND` fed by such a
//! user-op whose remaining arguments are constants, builds the table from the
//! scripts, and replaces the user-op with its switch variable.

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::{ArchContext, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::inject_sleigh::{PcodeInjectLibrarySleigh, SleighInjectEngine, SleighPayload};
use crate::jumptable::{JumpModel, JumpTable, LoadTable, NO_LABEL};
use crate::kuna_callmechanisms::Operation;
use crate::pcodeinject::InjectContext;
use crate::userop::UserOpManage;

/// One compiled `<..._pcode>` script.
#[derive(Debug)]
pub struct Script {
    payload: SleighPayload,
    tpl: kuna_sleigh::semantics::ConstructTpl,
}

/// The scripts of one `<jumpassist>` user-op.
#[derive(Debug)]
pub struct Assist {
    pub name: Vec<u8>,
    case: Option<Script>,
    addr: Script,
    default: Script,
    size: Option<Script>,
}

/// Every `<jumpassist>` of an architecture, by CALLOTHER index.
#[derive(Debug, Default)]
pub struct JumpAssists {
    ops: BTreeMap<uint4, Rc<Assist>>,
}

impl JumpAssists {
    pub fn from_library(userops: &UserOpManage, lib: &PcodeInjectLibrarySleigh) -> JumpAssists {
        let script = |id: int4| -> Option<Script> {
            let tpl = lib.get_tpl(id)?.clone();
            let payload = lib.injection.get(usize::try_from(id).ok()?)?.clone();
            Some(Script { payload, tpl })
        };
        let mut ops = BTreeMap::new();
        for index in 0..userops.num_ops() {
            let Some(op) = userops.get_op(index) else { continue };
            let Some(assist) = op.as_jump_assist() else { continue };
            let (Some(addr), Some(default)) = (script(assist.index2addr), script(assist.defaultaddr)) else {
                continue;
            };
            let case = (assist.index2case >= 0).then(|| script(assist.index2case)).flatten();
            let size = (assist.calcsize >= 0).then(|| script(assist.calcsize)).flatten();
            if (assist.index2case >= 0 && case.is_none()) || (assist.calcsize >= 0 && size.is_none()) {
                continue;
            }
            ops.insert(index, Rc::new(Assist { name: op.get_name().to_vec(), case, addr, default, size }));
        }
        JumpAssists { ops }
    }

    pub fn get(&self, index: uint4) -> Option<Rc<Assist>> {
        self.ops.get(&index).cloned()
    }
}

impl Script {
    fn num_inputs(&self) -> usize {
        self.payload.as_payload().core().size_input() as usize
    }

    /// Run the script on `inputs` and return its output
    /// (`ExecutablePcode::evaluate`).
    pub fn evaluate(&self, glb: &ArchContext, inputs: &[uintb]) -> KunaResult<uintb> {
        let manage = glb.manage();
        let space = |s: Option<&Rc<AddrSpace>>| s.cloned().ok_or_else(|| KunaError::lowlevel("jumpassist: missing address space"));
        let unique = space(manage.get_unique_space())?;
        let core = self.payload.as_payload().core();
        if inputs.len() != core.size_input() as usize {
            return Err(KunaError::lowlevel("jumpassist: script has the wrong number of parameters"));
        }
        let mut context = InjectContext::default();
        let mut slot: u64 = 0x10;
        for i in 0..core.size_input() {
            context.inputlist.push(VarnodeData { space: Some(Rc::clone(&unique)), offset: slot, size: core.get_input(i).get_size() as u32 });
            slot += 0x20;
        }
        for i in 0..core.size_output() {
            context.output.push(VarnodeData { space: Some(Rc::clone(&unique)), offset: slot, size: core.get_output(i).get_size() as u32 });
            slot += 0x20;
        }
        let code = space(manage.get_default_code_space())?;
        context.baseaddr = Some(Address::new(Rc::clone(&code), 0));
        let engine = SleighInjectEngine::new(space(manage.get_constant_space())?, Rc::clone(&unique), code);
        let mut ops = Ops::default();
        engine.emit_payload(self.payload.as_payload(), &self.tpl, &mut context, &mut ops)?;

        let mut state: HashMap<(int4, u64), u64> = HashMap::new();
        for (vn, value) in context.inputlist.iter().zip(inputs) {
            state.insert(key(vn), value & calc_mask(vn.size as int4));
        }
        let data = space(manage.get_default_data_space())?;
        for op in &ops.0 {
            let read = |vn: &VarnodeData| -> u64 {
                match vn.space.as_ref() {
                    Some(s) if s.get_type() == spacetype::IPTR_CONSTANT => vn.offset,
                    _ => state.get(&key(vn)).copied().unwrap_or(0),
                }
            };
            let Some(out) = op.output.as_ref() else {
                return Err(KunaError::lowlevel("jumpassist: script op without an output"));
            };
            let value = match op.opcode {
                OpCode::CPUI_COPY => read(&op.inputs[0]),
                OpCode::CPUI_LOAD => glb.get_load_image_value(&Address::new(Rc::clone(&data), read(&op.inputs[1])), out.size as int4)?,
                code => {
                    let behave = glb
                        .opbehaviors
                        .get(code as usize)
                        .and_then(Option::as_ref)
                        .ok_or_else(|| KunaError::lowlevel("jumpassist: script op cannot be evaluated"))?;
                    let insize = op.inputs[0].size as int4;
                    if op.inputs.len() == 1 {
                        behave.evaluate_unary(out.size as int4, insize, read(&op.inputs[0]))?
                    } else {
                        behave.evaluate_binary(out.size as int4, insize, read(&op.inputs[0]), read(&op.inputs[1]))?
                    }
                }
            };
            state.insert(key(out), value & calc_mask(out.size as int4));
        }
        let out = context.output.first().ok_or_else(|| KunaError::lowlevel("jumpassist: script has no output"))?;
        Ok(state.get(&key(out)).copied().unwrap_or(0))
    }
}

fn key(vn: &VarnodeData) -> (int4, u64) {
    (vn.space.as_ref().map_or(-1, |s| s.get_index()), vn.offset)
}

#[derive(Default)]
struct Ops(Vec<Operation>);

impl kuna_sleigh::translate::PcodeEmit for Ops {
    fn dump(&mut self, address: &Address, opcode: OpCode, output: Option<&VarnodeData>, inputs: &[VarnodeData]) {
        self.0.push(Operation { address: address.clone(), opcode, output: output.cloned(), inputs: inputs.to_vec() });
    }
}

/// The `JumpAssisted` model.
#[derive(Debug, Clone, Default)]
pub struct JumpModelAssisted {
    assist: Option<Rc<Assist>>,
    assist_op: Option<OpId>,
    switchvn: Option<VarnodeId>,
    size_indices: uint4,
    inputs: Vec<uintb>,
}

impl JumpModelAssisted {
    pub fn new() -> JumpModelAssisted {
        JumpModelAssisted::default()
    }

    fn assist(&self) -> KunaResult<&Rc<Assist>> {
        self.assist.as_ref().ok_or_else(|| KunaError::lowlevel("JumpAssisted: model not recovered"))
    }

    fn run(&self, fd: &Funcdata, script: &Script, index: uintb) -> KunaResult<uintb> {
        if script.num_inputs() != self.inputs.len() {
            return Err(KunaError::lowlevel(format!(
                "{}: script has wrong number of parameters",
                String::from_utf8_lossy(&self.assist()?.name)
            )));
        }
        let mut inputs = self.inputs.clone();
        inputs[0] = index;
        script.evaluate(fd.get_arch(), &inputs)
    }
}

impl JumpModel for JumpModelAssisted {
    fn is_override(&self) -> bool {
        false
    }

    fn get_table_size(&self) -> int4 {
        self.size_indices as int4 + 1
    }

    fn recover_model(&mut self, fd: &mut Funcdata, indop: OpId, matchsize: uint4, maxtablesize: uint4) -> KunaResult<bool> {
        let Some(addr_vn) = fd.obank().get(indop).and_then(|o| o.get_in(0)) else { return Ok(false) };
        let Some(assist_op) = fd.vbank().get(addr_vn).filter(|v| v.is_written()).and_then(|v| v.get_def()) else {
            return Ok(false);
        };
        let Some(o) = fd.obank().get(assist_op) else { return Ok(false) };
        if o.code() != OpCode::CPUI_CALLOTHER || o.num_input() < 3 {
            return Ok(false);
        }
        let offset = |slot: int4| o.get_in(slot).and_then(|v| fd.vbank().get(v)).map(|v| (v.get_offset(), v.is_constant()));
        let Some((index, _)) = offset(0) else { return Ok(false) };
        let Some(assist) = fd.get_arch().jump_assists.get(index as uint4) else { return Ok(false) };
        let mut inputs = Vec::new();
        for slot in 1..o.num_input() {
            let Some((value, constant)) = offset(slot) else { return Ok(false) };
            if slot >= 2 && !constant {
                return Ok(false);
            }
            inputs.push(value);
        }
        self.switchvn = o.get_in(1);
        self.assist_op = Some(assist_op);
        self.inputs = inputs;
        self.assist = Some(Rc::clone(&assist));
        self.size_indices = match assist.size.as_ref() {
            None => self.inputs[1] as uint4,
            Some(script) => {
                if script.num_inputs() != self.inputs.len() {
                    return Err(KunaError::lowlevel(format!(
                        "{}: <size_pcode> has wrong number of parameters",
                        String::from_utf8_lossy(&assist.name)
                    )));
                }
                script.evaluate(fd.get_arch(), &self.inputs)? as uint4
            }
        };
        if matchsize != 0 && matchsize - 1 != self.size_indices {
            return Ok(false);
        }
        Ok(self.size_indices <= maxtablesize)
    }

    fn build_addresses(
        &self,
        fd: &Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        _loadpoints: Option<&mut Vec<LoadTable>>,
        _loadcounts: Option<&mut Vec<int4>>,
    ) -> KunaResult<()> {
        let assist = self.assist()?;
        let spc = fd
            .obank()
            .get(indop)
            .and_then(|o| o.get_addr().get_space().cloned())
            .ok_or_else(|| KunaError::lowlevel("JumpAssisted: BRANCHIND has no space"))?;
        let mut mask: uintb = !0;
        let bit = fd.get_arch().funcptr_align;
        if bit != 0 {
            mask = (mask >> bit) << bit;
        }
        addresstable.clear();
        for index in 0..self.size_indices {
            let out = self.run(fd, &assist.addr, uintb::from(index))? & mask;
            addresstable.push(Address::new(Rc::clone(&spc), out));
        }
        let default = self.run(fd, &assist.default, 0)?;
        addresstable.push(Address::new(spc, default));
        Ok(())
    }

    fn find_unnormalized(&mut self, _fd: &mut Funcdata, _maxaddsub: uint4, _maxleftright: uint4, _maxext: uint4) -> KunaResult<()> {
        Ok(())
    }

    fn build_labels(&self, fd: &mut Funcdata, _addresstable: &mut [Address], label: &mut Vec<uintb>, orig: &dyn JumpModel) -> KunaResult<()> {
        let same = orig.as_any().downcast_ref::<JumpModelAssisted>().is_some_and(|o| o.size_indices == self.size_indices);
        if !same {
            return Err(KunaError::lowlevel("JumpAssisted table size changed during recovery"));
        }
        match self.assist()?.case.as_ref() {
            None => label.extend((0..self.size_indices).map(uintb::from)),
            Some(script) => {
                for index in 0..self.size_indices {
                    label.push(self.run(fd, script, uintb::from(index))?);
                }
            }
        }
        label.push(NO_LABEL);
        Ok(())
    }

    fn fold_in_normalization(&mut self, fd: &mut Funcdata, _indop: OpId) -> KunaResult<Option<VarnodeId>> {
        let (Some(assist_op), Some(switchvn)) = (self.assist_op, self.switchvn) else {
            return Err(KunaError::lowlevel("JumpAssisted: model not recovered"));
        };
        if let Some(out) = fd.obank().get(assist_op).and_then(|o| o.get_out()) {
            for reader in fd.descend_snapshot(out) {
                let slots: Vec<int4> = fd
                    .obank()
                    .get(reader)
                    .map(|o| (0..o.num_input()).filter(|&i| o.get_in(i) == Some(out)).collect())
                    .unwrap_or_default();
                for slot in slots {
                    fd.op_set_input(reader, switchvn, slot)?;
                }
            }
        }
        fd.op_destroy(assist_op);
        Ok(Some(switchvn))
    }

    fn fold_in_guards(&mut self, _fd: &mut Funcdata, jump: &mut JumpTable) -> KunaResult<bool> {
        let original = jump.get_default_block();
        jump.set_last_as_default();
        Ok(original != jump.get_default_block())
    }

    fn sanity_check(
        &mut self,
        _fd: &mut Funcdata,
        _indop: OpId,
        _addresstable: &mut Vec<Address>,
        _loadpoints: &mut Vec<LoadTable>,
        _loadcounts: Option<&Vec<int4>>,
    ) -> KunaResult<bool> {
        Ok(true)
    }

    fn clone_box(&self) -> Box<dyn JumpModel> {
        Box::new(JumpModelAssisted {
            assist: self.assist.clone(),
            size_indices: self.size_indices,
            inputs: self.inputs.clone(),
            ..JumpModelAssisted::default()
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clear(&mut self) {
        self.assist_op = None;
        self.switchvn = None;
    }
}
