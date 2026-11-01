//! Caller-local PPC32 PIC import identities, proved before SSA memory folding.
//!
//! All reachable paths must close at proved no-return imports (or converge in a
//! loop). A reachable RETURN is a refusal: PPC link-register dispatch is not
//! represented faithfully enough by ordinary return flow to certify it here.
//! Facts never escape the selected Funcdata entry or become stub symbols.

use std::collections::{BTreeMap, VecDeque};
use kuna_base::{address::Address, error::KunaResult};
use kuna_num::opcodes::OpCode;
use crate::{architecture::Architecture, context::{OpId, VarnodeId}, funcdata::Funcdata};

/// Loader evidence for one validated R_PPC_JMP_SLOT relocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportSlot {
    pub address: u64,
    pub name: String,
    /// A name from the ELF no-return list; its use also requires noreturn_known.
    pub no_return: bool,
}

/// A call identity scoped to one function entry and one instruction address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallBinding {
    pub site: u64,
    /// Physical direct destination, if the original instruction was a call.
    pub target: Option<u64>,
    pub tail: bool,
    pub import: ImportSlot,
}

/// Complete short/long r30-relative stub, including the short form's padding.
pub fn stub_delta(words: &[u32]) -> Option<u32> {
    if words.len() != 4 { return None; }
    if words[0] & 0xffff_0000 == 0x817e_0000
        && words[1..] == [0x7d69_03a6, 0x4e80_0420, 0x6000_0000]
    {
        Some(words[0] as u16 as i16 as u32)
    } else if words[0] & 0xffff_0000 == 0x3d7e_0000
        && words[1] & 0xffff_0000 == 0x816b_0000
        && words[2..] == [0x7d69_03a6, 0x4e80_0420]
    {
        Some((words[0] << 16).wrapping_add(words[1] as u16 as i16 as u32))
    } else { None }
}

type Key = (i32, u64, i32);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value { Constant(u64), Slot(u64), Stack(u32) }
#[derive(Clone, Default, PartialEq, Eq)]
struct State { values: BTreeMap<Key, Value>, memory_clobbered: bool }
fn key(data: &Funcdata, vn: VarnodeId) -> Key {
    let v = data.vbank().get(vn).expect("live raw varnode");
    (v.get_space().get_index(), v.get_offset(), v.get_size())
}
fn value(data: &Funcdata, vn: VarnodeId, state: &State) -> Option<Value> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() { Some(Value::Constant(v.get_offset())) }
    else { state.values.get(&key(data, vn)).copied() }
}
fn scalar(v: Option<Value>) -> Option<u64> {
    if let Some(Value::Constant(c)) = v { Some(c) } else { None }
}
fn write(state: &mut State, key: Key, value: Option<Value>) {
    state.values.retain(|&(s, off, size), _| s != key.0
        || off.saturating_add(size as u64) <= key.1
        || key.1.saturating_add(key.2 as u64) <= off);
    if let Some(v) = value { state.values.insert(key, v); }
}
fn register(engine: &Architecture, name: &str) -> Option<Key> {
    let v = engine.translate().get_register_data(name).ok()?;
    Some((v.space?.get_index(), v.offset, v.size as i32))
}
fn load(data: &Funcdata, engine: &Architecture, op: OpId, state: &State) -> Option<Value> {
    if state.memory_clobbered { return None; }
    let o = data.obank().get(op)?;
    let space = scalar(value(data, o.get_in(0)?, state))? as i32;
    let offset = scalar(value(data, o.get_in(1)?, state))?;
    if space != data.get_address().get_space()?.get_index()
        || data.vbank().get(o.get_out()?)?.get_size() != 4 { return None; }
    engine.ppc32_pic_imports.contains_key(&offset).then_some(Value::Slot(offset))
}
fn pure(data: &Funcdata, op: OpId, state: &State) -> Option<Value> {
    let o = data.obank().get(op)?;
    let out = data.vbank().get(o.get_out()?)?;
    let values: Vec<_> = (0..o.num_input()).map(|n| o.get_in(n).and_then(|v| value(data, v, state))).collect();
    if o.code() == OpCode::CPUI_COPY { return values.first().copied().flatten(); }
    if out.get_size() == 4 && matches!(o.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB) {
        if let [Some(Value::Stack(s)), Some(Value::Constant(c))] = values.as_slice() {
            return Some(Value::Stack(if o.code() == OpCode::CPUI_INT_ADD {
                s.wrapping_add(*c as u32)
            } else { s.wrapping_sub(*c as u32) }));
        }
    }
    let values: Option<Vec<_>> = values.into_iter().map(scalar).collect();
    let values = values?;
    let insize = data.vbank().get(o.get_in(0)?)?.get_size();
    let behavior = data.get_arch().op_behavior(o.code())?;
    let c = match values.as_slice() {
        [a] => behavior.evaluate_unary(out.get_size(), insize, *a).ok(),
        [a,b] => behavior.evaluate_binary(out.get_size(), insize, *a, *b).ok(),
        [a,b,c] => behavior.evaluate_ternary(out.get_size(), insize, *a, *b, *c).ok(),
        _ => None,
    }?;
    Some(Value::Constant(c))
}
fn direct_slot(data: &Funcdata, engine: &Architecture, target: &Address, state: &State) -> Option<u64> {
    if state.memory_clobbered { return None; }
    if target.get_offset() & 3 != 0 { return None; }
    let mut bytes = [0u8; 16];
    data.get_arch().loader_fill(&mut bytes, target).ok()?;
    let little = !target.get_space()?.is_big_endian();
    let words: Vec<_> = bytes.chunks_exact(4).map(|b| if little {
        u32::from_le_bytes(b.try_into().unwrap())
    } else { u32::from_be_bytes(b.try_into().unwrap()) }).collect();
    let slot = if let Some(delta) = stub_delta(&words) {
        let base = scalar(state.values.get(&register(engine, "r30")?).copied())?;
        (base as u32).wrapping_add(delta)
    } else if words[0] & 0xffff_0000 == 0x3d60_0000
        && words[1] & 0xffff_0000 == 0x816b_0000
        && words[2..] == [0x7d69_03a6, 0x4e80_0420] {
        (words[0] << 16).wrapping_add(words[1] as u16 as i16 as u32)
    } else { return None; };
    engine.ppc32_pic_imports.contains_key(&u64::from(slot)).then_some(u64::from(slot))
}

// The caller's frame and its two linkage words are private storage. An affine
// stack value alone is insufficient: a large offset could alias a GOT slot.
fn private_store(data: &Funcdata, engine: &Architecture, op: OpId, state: &State) -> Option<()> {
    let o = data.obank().get(op)?;
    let space = scalar(value(data, o.get_in(0)?, state))? as i32;
    if space != data.get_address().get_space()?.get_index() { return None; }
    let Value::Stack(offset) = value(data, o.get_in(1)?, state)? else { return None; };
    let Value::Stack(sp) = *state.values.get(&register(engine, "r1")?)? else { return None; };
    let mut lower = i64::from(sp as i32);
    let offset = i64::from(offset as i32);
    if !(-65536..=0).contains(&lower) { return None; }
    if offset < lower {
        // stwu r1,-frame(r1) writes the backchain before updating r1 in P-code.
        let mut bytes = [0; 4];
        data.get_arch().loader_fill(&mut bytes, o.get_addr()).ok()?;
        let word = if o.get_addr().get_space()?.is_big_endian() {
            u32::from_be_bytes(bytes)
        } else { u32::from_le_bytes(bytes) };
        let delta = i64::from(word as u16 as i16);
        if word & 0xffff_0000 != 0x9421_0000 || delta >= 0 || delta % 16 != 0
            || lower + delta != offset { return None; }
        lower = offset;
    }
    let size = i64::from(data.vbank().get(o.get_in(2)?)?.get_size());
    (lower >= -65536 && offset >= lower && size > 0 && offset + size <= 8).then_some(())
}

/// Prove a closed caller without memory-value propagation or interprocedural
/// guesses. Refusal returns no bindings and leaves the original flow untouched.
pub(crate) fn prove(data: &Funcdata, engine: &Architecture) -> BTreeMap<u64, CallBinding> {
    prove_inner(data, engine).unwrap_or_default()
}
fn prove_inner(data: &Funcdata, engine: &Architecture) -> Option<BTreeMap<u64, CallBinding>> {
    if !engine.analysis_ppc32picimports.scoped() || engine.ppc32_pic_imports.is_empty()
        || !engine.archid.starts_with("PowerPC:") || !engine.archid.contains(":32:") { return None; }
    let n = data.bblocks_get_size() as usize;
    if n == 0 || n > 4096 { return None; }
    let blocks: Vec<_> = (0..n).map(|i| data.bblocks_get_block(i as i32)).collect();
    let indices: BTreeMap<_, _> = blocks.iter().enumerate().map(|(i,b)| (*b,i)).collect();
    let mut inputs = vec![None; n];
    let mut initial = State::default();
    initial.values.insert(register(engine, "r1")?, Value::Stack(0));
    inputs[0] = Some(initial);
    let mut queue = VecDeque::from([0usize]);
    let mut queued = vec![false; n];
    queued[0] = true;
    let mut result = BTreeMap::<u64, CallBinding>::new();
    let mut steps = 0;
    let mut operations = 0;
    let preserved: Vec<Key> = [1,2,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31]
        .iter().filter_map(|r| register(engine, &format!("r{r}"))).collect();
    while let Some(i) = queue.pop_front() {
        queued[i] = false;
        steps += 1;
        if steps > n * 64 { return None; }
        let mut state = inputs[i].clone()?;
        let mut terminal = false;
        for op in data.bb_ops(blocks[i]) {
            operations += 1;
            if operations > 1_000_000 || state.values.len() > 4096 { return None; }
            let o = data.obank().get(op)?;
            let site = o.get_addr().get_offset();
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCHIND => {
                    // Explicit user semantics take precedence over inference.
                    if data.get_override().find_proto_override(o.get_addr()).is_some()
                        || data.get_override().find_indirect_override(o.get_addr()).is_some() { return None; }
                    let target = if o.code() == OpCode::CPUI_CALL {
                        Some(data.get_call_specs(data.get_call_specs_index(op)?).get_entry_address().clone())
                    } else { None };
                    let slot = if let Some(target) = &target {
                        direct_slot(data, engine, target, &state)?
                    } else if let Some(Value::Slot(s)) = value(data, o.get_in(0)?, &state) { s }
                    else { return None; };
                    let mut import = engine.ppc32_pic_imports.get(&slot)?.clone();
                    import.no_return &= engine.analysis_noreturn_known;
                    let tail = o.code() == OpCode::CPUI_BRANCHIND;
                    if tail && !import.no_return { return None; }
                    let binding = CallBinding { site, target: target.map(|t| t.get_offset()), tail, import };
                    if result.get(&site).is_some_and(|old| old != &binding) { return None; }
                    terminal = binding.import.no_return;
                    result.insert(site, binding);
                    if terminal { break; }
                    // The callee's writes are unproved, including writes through globals.
                    // Already loaded targets in preserved registers remain values, not reads.
                    state.memory_clobbered = true;
                    state.values.retain(|k,_| preserved.contains(k));
                }
                OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN => return None,
                OpCode::CPUI_STORE => {
                    // Stack writes cannot change the relocation slots or code.
                    // Other writes require an alias proof this pass does not own.
                    private_store(data, engine, op, &state)?;
                }
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {},
                _ => if let Some(vn) = o.get_out() {
                    let v = if o.code() == OpCode::CPUI_LOAD { load(data, engine, op, &state) }
                        else { pure(data, op, &state) };
                    write(&mut state, key(data, vn), v);
                },
            }
        }
        if terminal { continue; }
        let block = data.bblocks_ref().block(blocks[i]);
        if block.size_out() == 0 { return None; }
        for j in 0..block.size_out() {
            let next = *indices.get(&block.get_out(j))?;
            let changed = if let Some(prior) = &mut inputs[next] {
                let old_len = prior.values.len();
                let old_clobber = prior.memory_clobbered;
                prior.memory_clobbered |= state.memory_clobbered;
                prior.values.retain(|k,v| state.values.get(k) == Some(v));
                prior.values.len() != old_len || prior.memory_clobbered != old_clobber
            } else { inputs[next] = Some(state.clone()); true };
            if changed && !queued[next] { queued[next] = true; queue.push_back(next); }
        }
    }
    Some(result)
}

/// Apply one proof while re-lifting the same entry. The relocation address is
/// used only in this call spec; neither it nor the stub gets a FunctionSymbol.
pub(crate) fn bind(data: &mut Funcdata, op: OpId, entry: &Address, indirect: bool) -> KunaResult<bool> {
    let Some(o) = data.obank().get(op) else { return Ok(false); };
    let Some(binding) = data.ppc32_pic_calls.get(&o.get_addr().get_offset()).cloned() else { return Ok(false); };
    if binding.target != (!indirect).then(|| entry.get_offset()) { return Ok(false); }
    let target = Address::new(data.get_address().get_space().unwrap().clone(), binding.import.address);
    let mut fc = crate::fspec::FuncCallSpecs::new(op, target.clone());
    fc.set_funcdata(target, &binding.import.name)?;
    if let (Some(model), Some(types)) = (data.get_arch().eval_fp_called().or_else(|| data.get_arch().default_fp()), data.get_arch().types()) {
        fc.proto_mut().set_internal(model.clone(), types.get_type_void()?);
    }
    fc.proto_mut().set_no_return(binding.import.no_return);
    let handle = crate::flow::next_fspec_handle();
    fc.register_in_fspec_space(handle, data.get_arch().kuna_name_style());
    data.push_call_specs(fc);
    let vn = data.new_varnode_call_specs(handle);
    data.op_set_input(op, vn, 0)?;
    if indirect { data.op_set_opcode_code(op, OpCode::CPUI_CALL); }
    Ok(true)
}

/// Distinct text lets a later flow remove only this pass's generated comment.
pub(crate) const WARNING: &str = "Subroutine does not return (PPC32 PIC call)";

pub(crate) fn clear_stale_warnings(db: &mut crate::architecture::CommentDatabase, entry: &Address) {
    db.retain_comments(|comment| &comment.func_addr != entry
        || comment.tp != crate::architecture::comment_type::warning
        || !comment.text.ends_with(WARNING));
}
