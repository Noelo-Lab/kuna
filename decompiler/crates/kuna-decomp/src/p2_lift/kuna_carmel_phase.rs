//! Bind Carmel's D1 address admission to a proved older issue clock before SSA.

use std::{collections::{BTreeSet, VecDeque}, rc::Rc};

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
}

impl<'a> PhaseDecode<'a> {
    pub(crate) fn enter(arch: &'a Architecture) -> Option<Self> {
        if !arch.archid.starts_with("Carmel:") {
            return None;
        }
        let tick = arch.userops.get_op_by_name(b"carmel_record_admission")?.get_index() as u64;
        let admission = arch.userops.get_op_by_name(b"carmel_read_address_admission")?.get_index() as u64;
        let (word, bits) = arch.with_context_db(|db| {
            let field = db.get_variable(b"carmel_phase_emit").ok()?;
            Some((field.get_word() as usize, field.get_mask() << field.get_shift()))
        })?;
        let translate = arch.translate();
        let saved = translate.set_context_read_override(word, 0, 0);
        translate.set_context_read_override(word, saved.0 | bits, saved.1 | bits);
        Some(Self { translate, word, saved, tick, admission })
    }

    pub(crate) fn lower(&self, fd: &mut Funcdata, visits: &VisitedMap) -> KunaResult<()> {
        let mut clocks = Vec::new();
        let mut reads = false;
        for (_, op_id) in fd.obank().iter_all() {
            let op = fd.obank().get(op_id).unwrap();
            if op.get_parent().is_none() { continue; }
            let Some(index) = userop(fd, op) else { continue };
            if index == self.tick {
                clocks.push(op_id);
            } else if index == self.admission {
                reads = true;
            }
        }
        if !reads {
            for clock in clocks {
                fd.op_destroy(clock);
            }
            return Ok(());
        }
        let known: BTreeSet<_> = clocks.iter().map(|&id| fd.obank().get(id).unwrap().get_addr().get_offset()).collect();
        // structure_reset has assigned dense block numbers in this order.
        let blocks: Vec<_> = (0..fd.bblocks_get_size()).map(|n| fd.bblocks_get_block(n)).collect();
        let traces: Vec<_> = blocks.iter().map(|&block| self.trace(fd, visits, &known, block)).collect();
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
        Ok(())
    }

    fn trace(&self, fd: &Funcdata, visits: &VisitedMap, known: &BTreeSet<u64>, block: BlockId) -> Trace {
        let mut trace = Trace::default();
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
            }
            match userop(fd, op) {
                Some(index) if index == self.tick => trace.events.push(Event::Tick(id)),
                Some(index) if index == self.admission => trace.events.push(Event::Read(id)),
                _ if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN
                    | OpCode::CPUI_LOAD | OpCode::CPUI_STORE) => {
                    // Bus effects and opaque transfers can change the spacing
                    // of the next sample. Prove its age again after this op.
                    trace.events.push(Event::Barrier);
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
                let predecessor = graph.block(block.get_in(edge)).get_index() as usize;
                match traces[predecessor].last.as_ref().zip(trace.first.as_ref()) {
                    Some((from, to)) if contiguous(visits, from, to) => exits[predecessor],
                    _ => 0,
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
