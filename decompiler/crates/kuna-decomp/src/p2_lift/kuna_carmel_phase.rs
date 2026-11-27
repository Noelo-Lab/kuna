//! Bind Carmel's D1 address admission to a proved older issue clock before SSA.

use std::{collections::{BTreeMap, BTreeSet, VecDeque}, rc::Rc};

use kuna_base::{address::Address, error::{KunaError, KunaResult}};
use kuna_num::opcodes::OpCode;

use crate::{
    architecture::Architecture, context::{BlockId, OpId, VarnodeId},
    engine_translate::EngineTranslate, flow::VisitedMap, funcdata::Funcdata,
    op::PcodeOp, typeop::type_op_for,
};

const CLOCKS: usize = 5;

pub(crate) struct PhaseDecode<'a> {
    translate: &'a dyn EngineTranslate,
    word: usize,
    saved: (u32, u32),
    tick: u64,
    admission: u64,
    redirect: Option<u64>,
    delay_tick: Option<u64>,
    delay_mask: Option<u64>,
    fixed_read_tick: Option<u64>,
    end_unmodeled: Option<u64>,
}

impl<'a> PhaseDecode<'a> {
    pub(crate) fn enter(arch: &'a Architecture) -> Option<Self> {
        if !arch.archid.starts_with("Carmel:") {
            return None;
        }
        let tick = arch.userops.get_op_by_name(b"carmel_record_admission")?.get_index() as u64;
        let admission = arch.userops.get_op_by_name(b"carmel_read_address_admission")?.get_index() as u64;
        let redirect = arch.userops.get_op_by_name(b"carmel_known_redirect").map(|op| op.get_index() as u64);
        let delay_tick = arch.userops.get_op_by_name(b"carmel_delay_slot_admission").map(|op| op.get_index() as u64);
        let delay_mask = arch.userops.get_op_by_name(b"carmel_modeled_delay_mask").map(|op| op.get_index() as u64);
        let fixed_read_tick = arch.userops.get_op_by_name(b"carmel_fixed_read_admission").map(|op| op.get_index() as u64);
        let end_unmodeled = arch.userops.get_op_by_name(b"carmel_end_unmodeled_issue").map(|op| op.get_index() as u64);
        let (word, bits) = arch.with_context_db(|db| {
            let field = db.get_variable(b"carmel_phase_emit").ok()?;
            Some((field.get_word() as usize, field.get_mask() << field.get_shift()))
        })?;
        let translate = arch.translate();
        let saved = translate.set_context_read_override(word, 0, 0);
        translate.set_context_read_override(word, saved.0 | bits, saved.1 | bits);
        Some(Self { translate, word, saved, tick, admission, redirect, delay_tick, delay_mask, fixed_read_tick, end_unmodeled })
    }

    pub(crate) fn lower(&self, fd: &mut Funcdata, visits: &VisitedMap) -> KunaResult<()> {
        let mut markers = Vec::new();
        let mut delay_queries = Vec::new();
        let mut slot_clocks = BTreeMap::<u64, usize>::new();
        let mut reads = false;
        for (_, op_id) in fd.obank().iter_all() {
            let op = fd.obank().get(op_id).unwrap();
            if op.get_parent().is_none() { continue; }
            let Some(index) = userop(fd, op) else { continue };
            let offset = op.get_addr().get_offset();
            if index == self.tick || Some(index) == self.fixed_read_tick {
                markers.push((op_id, offset, Mark::Tick));
            } else if Some(index) == self.redirect {
                markers.push((op_id, offset, Mark::Redirect));
            } else if Some(index) == self.delay_tick {
                *slot_clocks.entry(offset).or_default() += 1;
                markers.push((op_id, offset, Mark::Slot));
            } else if Some(index) == self.end_unmodeled {
                markers.push((op_id, offset, Mark::EndUnmodeled));
            } else if Some(index) == self.delay_mask {
                delay_queries.push((op_id, offset));
            } else if index == self.admission {
                reads = true;
            }
        }
        let mut delayed = BTreeMap::new();
        for (query, offset) in delay_queries {
            let modeled = slot_clocks.get(&offset) == Some(&1);
            delayed.insert(offset, modeled);
            let mask = fd.new_constant(2, if modeled { 0xffff } else { 0 });
            fd.op_set_opcode(query, type_op_for(OpCode::CPUI_COPY));
            fd.op_set_input(query, mask, 0)?;
        }
        if !reads {
            for (marker, _, _) in markers {
                fd.op_destroy(marker);
            }
            return Ok(());
        }
        let mut known = BTreeSet::new();
        let mut cleanup = Vec::new();
        for (marker, offset, kind) in markers {
            let modeled = delayed.get(&offset).copied();
            if modeled == Some(false) || (kind == Mark::Slot && modeled.is_none()) {
                fd.op_destroy(marker);
            } else if kind == Mark::Redirect {
                cleanup.push(marker);
            } else {
                known.insert(offset);
                if kind == Mark::EndUnmodeled {
                    cleanup.push(marker);
                }
            }
        }
        // structure_reset has assigned dense block numbers in this order.
        let blocks: Vec<_> = (0..fd.bblocks_get_size()).map(|n| fd.bblocks_get_block(n)).collect();
        let traces: Vec<_> = blocks.iter().map(|&block| self.trace(fd, visits, &known, &delayed, block)).collect();
        let inputs = incoming_ages(fd, visits, &blocks, &traces);
        for (trace, mut age) in traces.iter().zip(inputs) {
            for event in &trace.events {
                if let Event::Read(op) = *event {
                    if age != CLOCKS {
                        let addr = fd.obank().get(op).unwrap().get_addr();
                        return Err(KunaError::lowlevel(format!(
                            "Unproved Carmel address admission at CODE {:#x}: {age} of {CLOCKS} clocks",
                            addr.get_offset() / 3,
                        )));
                    }
                }
                age = event.advance(age);
            }
        }
        let scratch = self.translate.get_register_varnode(b"__CE_ADDRESS_HISTORY")
            .map_err(|_| KunaError::lowlevel("Carmel phase history storage is missing"))?;
        let space = scratch.space.as_ref().unwrap();
        let slots: [Address; CLOCKS] = std::array::from_fn(|i| Address::new(Rc::clone(space), scratch.offset + i as u64));
        for trace in traces {
            for event in trace.events {
                match event {
                    Event::Tick(mark) => {
                        for i in (1..CLOCKS).rev() {
                            let previous = fd.new_varnode(1, &slots[i - 1], None);
                            copy_before(fd, mark, previous, &slots[i])?;
                        }
                        // Keep the marker's existing pre-SSA input: a free
                        // varnode may only feed one operation.
                        fd.op_remove_input(mark, 0);
                        fd.op_set_opcode(mark, type_op_for(OpCode::CPUI_COPY));
                        fd.new_varnode_out(1, &slots[0], mark)?;
                    }
                    Event::Read(read) => {
                        let sample = fd.new_varnode(1, &slots[CLOCKS - 1], None);
                        fd.op_set_opcode(read, type_op_for(OpCode::CPUI_INT_AND));
                        fd.op_set_input(read, sample, 0)?;
                    }
                    Event::Barrier => {}
                }
            }
        }
        for marker in cleanup {
            fd.op_destroy(marker);
        }
        Ok(())
    }

    fn trace(
        &self, fd: &Funcdata, visits: &VisitedMap, known: &BTreeSet<u64>, delayed: &BTreeMap<u64, bool>,
        block: BlockId,
    ) -> Trace {
        let mut trace = Trace::default();
        let mut fixed_read = false;
        let mut current = fd.bb_op_head(block);
        while let Some(id) = current {
            let op = fd.obank().get(id).unwrap();
            let addr = op.get_addr();
            if trace.last.as_ref() != Some(addr) {
                if !known.contains(&addr.get_offset())
                    || trace.last.as_ref().is_some_and(|previous| !contiguous(visits, previous, addr))
                {
                    trace.events.push(Event::Barrier);
                }
                trace.first.get_or_insert_with(|| addr.clone());
                trace.last = Some(addr.clone());
                trace.redirect_edges = 0;
                fixed_read = false;
            }
            match userop(fd, op) {
                Some(index) if index == self.tick || Some(index) == self.delay_tick => {
                    trace.events.push(Event::Tick(id));
                }
                Some(index) if Some(index) == self.fixed_read_tick => {
                    trace.events.push(Event::Tick(id));
                    fixed_read = true;
                }
                Some(index) if index == self.admission => trace.events.push(Event::Read(id)),
                Some(index) if Some(index) == self.end_unmodeled => trace.events.push(Event::Barrier),
                Some(index) if Some(index) == self.redirect => {
                    trace.redirect_edges = if delayed.contains_key(&addr.get_offset()) { 2 } else { 1 };
                }
                _ if op.code() == OpCode::CPUI_LOAD && fixed_read => fixed_read = false,
                _ if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN
                    | OpCode::CPUI_LOAD | OpCode::CPUI_STORE) => {
                    // Bus effects and opaque transfers can change the spacing
                    // of the next sample. Prove its age again after this op.
                    trace.events.push(Event::Barrier);
                    fixed_read = false;
                }
                _ => {}
            }
            current = fd.bb_op_next(id);
        }
        trace
    }
}

impl Drop for PhaseDecode<'_> {
    fn drop(&mut self) {
        self.translate.set_context_read_override(self.word, self.saved.0, self.saved.1);
    }
}

fn userop(fd: &Funcdata, op: &PcodeOp) -> Option<u64> {
    if op.code() != OpCode::CPUI_CALLOTHER { return None }
    let index = fd.vbank().get(op.get_in(0)?)?;
    index.is_constant().then(|| index.get_offset())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mark {
    Tick,
    Slot,
    Redirect,
    EndUnmodeled,
}

#[derive(Clone, Copy)]
enum Event {
    Barrier,
    Tick(OpId),
    Read(OpId),
}

impl Event {
    fn advance(self, age: usize) -> usize {
        match self {
            Self::Barrier => 0,
            Self::Tick(_) => (age + 1).min(CLOCKS),
            Self::Read(_) => age,
        }
    }
}

#[derive(Default)]
struct Trace {
    first: Option<Address>,
    last: Option<Address>,
    events: Vec<Event>,
    redirect_edges: i32,
}

fn contiguous(visits: &VisitedMap, from: &Address, to: &Address) -> bool {
    from == to || visits.get(from).is_some_and(|visit| from + i64::from(visit.size) == *to)
}

fn incoming_ages(fd: &Funcdata, visits: &VisitedMap, blocks: &[BlockId], traces: &[Trace]) -> Vec<usize> {
    let graph = fd.bblocks_ref();
    let mut inputs = vec![CLOCKS; blocks.len()];
    let mut exits = inputs.clone();
    let mut queue: VecDeque<_> = (0..blocks.len()).collect();
    let mut queued = vec![true; blocks.len()];
    while let Some(i) = queue.pop_front() {
        queued[i] = false;
        let block = graph.block(blocks[i]);
        let trace = &traces[i];
        let age = if i == 0 || block.size_in() == 0 {
            0
        } else {
            (0..block.size_in()).map(|edge| {
                let source = graph.block(block.get_in(edge));
                let predecessor = source.get_index() as usize;
                let prior = &traces[predecessor];
                if source.size_out() <= prior.redirect_edges {
                    exits[predecessor]
                } else {
                    match prior.last.as_ref().zip(trace.first.as_ref()) {
                        Some((from, to)) if contiguous(visits, from, to) => exits[predecessor],
                        _ => 0,
                    }
                }
            }).min().unwrap()
        };
        inputs[i] = age;
        let exit = trace.events.iter().fold(age, |age, event| event.advance(age));
        if exit != exits[i] {
            exits[i] = exit;
            for edge in 0..block.size_out() {
                let successor = graph.block(block.get_out(edge)).get_index() as usize;
                if !queued[successor] {
                    queued[successor] = true;
                    queue.push_back(successor);
                }
            }
        }
    }
    inputs
}

fn copy_before(fd: &mut Funcdata, marker: OpId, source: VarnodeId, destination: &Address) -> KunaResult<()> {
    let addr = fd.obank().get(marker).unwrap().get_addr().clone();
    let op = fd.new_op(1, addr);
    fd.op_set_opcode(op, type_op_for(OpCode::CPUI_COPY));
    fd.op_set_input(op, source, 0)?;
    fd.new_varnode_out(1, destination, op)?;
    fd.op_insert_before(op, marker);
    Ok(())
}
