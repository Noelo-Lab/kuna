//! Bind Carmel's address-admission, operand-fetch and issue-slot samples before SSA.

use std::{collections::{BTreeMap, BTreeSet, VecDeque}, rc::Rc};

use kuna_base::{address::Address, error::{KunaError, KunaResult}};
use kuna_num::opcodes::OpCode;

use crate::{
    architecture::Architecture, context::{BlockId, OpId, VarnodeId},
    engine_translate::EngineTranslate, flow::VisitedMap, funcdata::Funcdata,
    op::PcodeOp, typeop::type_op_for,
};

use super::kuna_carmel_reset::{AddressAdmission, Execute, IssueEffects, Overlap, lower_control};

const CLOCKS: usize = 5;
const MATURE_ENTRY: u8 = 1 << CLOCKS;
const ANY_ENTRY_AGE: u8 = (MATURE_ENTRY << 1) - 1;
const FF_UNKNOWN: u8 = 1;
const FF_LIVE: u8 = 2;
const FF_OLDER: u8 = 4;
const WB_WINDOW: u8 = 0b111;
const OLDER_WB: u8 = 0b110;
const ISSUE_CELLS: u16 = 0x3bff;

#[derive(Clone, Copy)]
struct FfOps {
    capture: u64,
    fetch: u64,
    execute: u64,
    late_tick: u64,
    visibility: u64,
}

#[derive(Clone, Copy)]
struct DataOps {
    reserve: u64,
    commit: u64,
}

#[derive(Clone, Copy)]
struct CellOps {
    read: u64,
    write: u64,
    tick: u64,
}

pub(crate) struct PhaseDecode<'a> {
    translate: &'a dyn EngineTranslate,
    word: usize,
    saved: (u32, u32),
    tick: u64,
    admission: u64,
    entry_admission: Option<u64>,
    redirect: Option<u64>,
    delay_tick: Option<u64>,
    delay_mask: Option<u64>,
    fixed_read_tick: Option<u64>,
    fixed_load: Option<u64>,
    fixed_store: Option<u64>,
    end_unmodeled: Option<u64>,
    omitted: Option<u64>,
    idle_tick: Option<u64>,
    reset: Option<u64>,
    register_ex: Option<u64>,
    data_ex: Option<u64>,
    address_ex: Option<u64>,
    operand_fetch: Option<u64>,
    reset_ex_end: Option<u64>,
    staged_data: Option<u64>,
    repeat_r3: Option<u64>,
    repeat_d2_write: Option<u64>,
    repeat_of: Option<u64>,
    issue_slot: Option<u64>,
    ff: Option<FfOps>,
    data: Option<DataOps>,
    cells: Option<CellOps>,
}

impl<'a> PhaseDecode<'a> {
    pub(crate) fn enter(arch: &'a Architecture) -> Option<Self> {
        if !arch.archid.starts_with("Carmel:") {
            return None;
        }
        let index = |name: &[u8]| arch.userops.get_op_by_name(name).map(|op| op.get_index() as u64);
        let tick = index(b"carmel_record_admission")?;
        let admission = index(b"carmel_read_address_admission")?;
        let entry_admission = index(b"carmel_read_entry_admission");
        let redirect = index(b"carmel_known_redirect");
        let delay_tick = index(b"carmel_delay_slot_admission");
        let delay_mask = index(b"carmel_modeled_delay_mask");
        let fixed_read_tick = index(b"carmel_fixed_read_admission");
        let fixed_load = index(b"carmel_single_data_load");
        let fixed_store = index(b"carmel_single_data_store");
        let end_unmodeled = index(b"carmel_end_unmodeled_issue");
        let omitted = index(b"carmel_omitted_sequential_slot");
        let idle_tick = index(b"carmel_record_idle_admission");
        let reset = index(b"carmel_d2_reset");
        let register_ex = index(b"carmel_register_only_ex");
        let data_ex = index(b"carmel_data_only_ex");
        let address_ex = index(b"carmel_address_only_ex");
        let operand_fetch = index(b"carmel_operand_fetch_survives");
        let reset_ex_end = index(b"carmel_d2_end_ex_controls");
        let staged_data = index(b"carmel_staged_wb_data");
        let repeat_r3 = index(b"carmel_prove_repeat_r3_sample");
        let repeat_d2_write = index(b"carmel_repeat_d2_write");
        let repeat_of = index(b"carmel_repeat_register_at_of");
        let issue_slot = index(b"carmel_sample_issue_slot");
        let ff = (|| Some(FfOps {
            capture: index(b"carmel_capture_ff_before_ex")?,
            fetch: index(b"carmel_sample_ff_at_of")?,
            execute: index(b"carmel_sample_ff_at_ex")?,
            late_tick: index(b"carmel_record_ff_late_admission")?,
            visibility: index(b"carmel_record_ff_visibility")?,
        }))();
        let data = (|| Some(DataOps {
            reserve: index(b"carmel_of_reserve_word")?,
            commit: index(b"carmel_wb_data_commit")?,
        }))();
        let cells = (|| Some(CellOps {
            read: index(b"carmel_read_issue_cell")?,
            write: index(b"carmel_write_issue_cell")?,
            tick: index(b"carmel_record_cell_admission")?,
        }))();
        let (word, bits) = arch.with_context_db(|db| {
            let field = db.get_variable(b"carmel_phase_emit").ok()?;
            Some((field.get_word() as usize, field.get_mask() << field.get_shift()))
        })?;
        let translate = arch.translate();
        let saved = translate.set_context_read_override(word, 0, 0);
        translate.set_context_read_override(word, saved.0 | bits, saved.1 | bits);
        Some(Self { translate, word, saved, tick, admission, entry_admission, redirect, delay_tick, delay_mask,
            fixed_read_tick, fixed_load, fixed_store, end_unmodeled, omitted, idle_tick, reset, register_ex, data_ex, address_ex,
            operand_fetch, reset_ex_end,
            staged_data, repeat_r3, repeat_d2_write, repeat_of, issue_slot, ff, data, cells })
    }

    pub(crate) fn lower(&self, fd: &mut Funcdata, visits: &VisitedMap) -> KunaResult<()> {
        let mut markers = Vec::new();
        let mut delay_queries = Vec::new();
        let mut slot_clocks = BTreeMap::<u64, usize>::new();
        let mut dual_reads = BTreeSet::new();
        let mut ff_fetches = false;
        let mut ff_late_visibility = false;
        let mut reads = false;
        let mut entry_reads = false;
        let mut slot_fetches = false;
        let mut scheduled_data = false;
        let mut issue_cells = false;
        let mut resets = Vec::new();
        let mut address_ex = Vec::new();
        let mut repeat_samples = Vec::new();
        let mut repeat_writes = Vec::new();
        let mut repeat_operand_fetches = Vec::new();
        for (_, op_id) in fd.obank().iter_all() {
            let op = fd.obank().get(op_id).unwrap();
            if op.get_parent().is_none() { continue; }
            let Some(index) = userop(fd, op) else { continue };
            let offset = op.get_addr().get_offset();
            if index == self.tick || Some(index) == self.fixed_read_tick
                || self.cells.is_some_and(|cells| index == cells.tick) {
                markers.push((op_id, offset, Mark::Tick));
            } else if Some(index) == self.idle_tick {
                markers.push((op_id, offset, Mark::Idle));
            } else if Some(index) == self.reset {
                resets.push(op_id);
            } else if Some(index) == self.register_ex {
                markers.push((op_id, offset, Mark::RegisterEx));
            } else if Some(index) == self.data_ex {
                markers.push((op_id, offset, Mark::DataEx));
            } else if Some(index) == self.address_ex {
                address_ex.push((op_id, offset));
            } else if Some(index) == self.operand_fetch {
                markers.push((op_id, offset, Mark::OperandFetch));
            } else if Some(index) == self.reset_ex_end {
                markers.push((op_id, offset, Mark::ResetExEnd));
            } else if Some(index) == self.staged_data {
                markers.push((op_id, offset, Mark::StagedData));
            } else if self.ff.is_some_and(|ff| index == ff.late_tick) {
                dual_reads.insert(offset);
                markers.push((op_id, offset, Mark::Tick));
            } else if self.ff.is_some_and(|ff| index == ff.visibility) {
                ff_late_visibility |= ff_is_late(fd, op);
                markers.push((op_id, offset, Mark::FfVisibility));
            } else if self.ff.is_some_and(|ff| index == ff.fetch) {
                ff_fetches = true;
            } else if self.ff.is_some_and(|ff| index == ff.capture || index == ff.execute) {
                markers.push((op_id, offset, Mark::Copy));
            } else if Some(index) == self.redirect {
                markers.push((op_id, offset, Mark::Redirect));
            } else if Some(index) == self.delay_tick {
                *slot_clocks.entry(offset).or_default() += 1;
                markers.push((op_id, offset, Mark::Slot));
            } else if Some(index) == self.end_unmodeled {
                markers.push((op_id, offset, Mark::EndUnmodeled));
            } else if Some(index) == self.omitted {
                markers.push((op_id, offset, Mark::Omitted));
            } else if Some(index) == self.fixed_load || Some(index) == self.fixed_store {
                markers.push((op_id, offset, Mark::FixedAccess));
            } else if Some(index) == self.delay_mask {
                delay_queries.push((op_id, offset));
            } else if Some(index) == self.repeat_r3 {
                repeat_samples.push(op_id);
            } else if Some(index) == self.repeat_d2_write {
                repeat_writes.push(op_id);
            } else if Some(index) == self.repeat_of {
                repeat_operand_fetches.push(op_id);
            } else if Some(index) == self.issue_slot {
                slot_fetches = true;
            } else if self.data.is_some_and(|data| index == data.reserve || index == data.commit) {
                scheduled_data = true;
                markers.push((op_id, offset, Mark::DataSchedule));
            } else if self.cells.is_some_and(|cells| index == cells.read || index == cells.write) {
                issue_cells = true;
            } else if index == self.admission {
                reads = true;
            } else if Some(index) == self.entry_admission {
                entry_reads = true;
            }
        }
        reads |= entry_reads;
        if !repeat_samples.is_empty() {
            prove_repeat_samples(fd, visits, &repeat_samples, self.repeat_d2_write)?;
        }
        if !repeat_operand_fetches.is_empty() {
            bind_repeat_operands(fd, visits, &repeat_operand_fetches, self)?;
        }
        for write in repeat_writes {
            lower_copy(fd, write, 1);
        }
        if let Some(&(_, offset, _)) = markers.iter().find(|(_, _, kind)| *kind == Mark::Omitted) {
            prove_omitted_prefix(fd, visits, &markers, offset)?;
        }
        let overlap = if resets.is_empty() {
            Overlap::default()
        } else {
            let mut effects = BTreeMap::<u64, IssueEffects>::new();
            for &(id, offset, kind) in &markers {
                match kind {
                    Mark::Idle => effects.entry(offset).or_default().idle = true,
                    Mark::RegisterEx => effects.entry(offset).or_default().execute = Some(Execute::Register(id)),
                    Mark::DataEx => effects.entry(offset).or_default().execute = Some(Execute::Data(id)),
                    Mark::OperandFetch => effects.entry(offset).or_default().operand_fetch = Some(id),
                    Mark::ResetExEnd => effects.entry(offset).or_default().reset_ex_end = Some(id),
                    Mark::StagedData => effects.entry(offset).or_default().staged_data = Some(id),
                    _ => {}
                }
            }
            for &(id, offset) in &address_ex {
                effects.entry(offset).or_default().execute = Some(Execute::AddressOnly(id));
            }
            lower_control(fd, visits, &resets, &effects)?
        };
        for (mark, _) in address_ex {
            match overlap.admission.get(&mark) {
                None => lower_copy(fd, mark, 1),
                Some(AddressAdmission::Entry) => lower_copy(fd, mark, 2),
                Some(AddressAdmission::History) => {
                    let userop = fd.new_constant(4, self.admission);
                    let ce1 = fd.new_constant(1, 1);
                    fd.op_remove_input(mark, 2);
                    fd.op_set_input(mark, userop, 0)?;
                    fd.op_set_input(mark, ce1, 1)?;
                    reads = true;
                }
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
        if !reads && !ff_fetches && !ff_late_visibility && !slot_fetches
            && !scheduled_data && !issue_cells && dual_reads.is_empty() {
            for (marker, _, kind) in markers {
                match kind {
                    Mark::Copy => lower_copy(fd, marker, 1),
                    Mark::OperandFetch => lower_operand_fetch(fd, marker, overlap.canceled.contains(&marker))?,
                    _ => fd.op_destroy(marker),
                }
            }
            return Ok(());
        }
        let mut known = BTreeMap::<u64, Boundary>::new();
        let mut cleanup = Vec::new();
        let mut copies = Vec::new();
        for (marker, offset, kind) in markers {
            let modeled = delayed.get(&offset).copied();
            if kind == Mark::OperandFetch {
                lower_operand_fetch(fd, marker, overlap.canceled.contains(&marker))?;
            } else if matches!(kind, Mark::RegisterEx | Mark::DataEx | Mark::ResetExEnd | Mark::StagedData)
                || modeled == Some(false) || (kind == Mark::Slot && modeled.is_none()) {
                fd.op_destroy(marker);
            } else if kind == Mark::Copy {
                copies.push(marker);
            } else if matches!(kind, Mark::Redirect | Mark::DataSchedule) {
                cleanup.push(marker);
            } else if kind == Mark::FfVisibility {
                known.entry(offset).or_default().ff = true;
                cleanup.push(marker);
            } else {
                known.entry(offset).or_default().address = true;
                if matches!(kind, Mark::EndUnmodeled | Mark::FixedAccess | Mark::Omitted) {
                    cleanup.push(marker);
                }
            }
        }
        // structure_reset has assigned dense block numbers in this order.
        let blocks: Vec<_> = (0..fd.bblocks_get_size()).map(|n| fd.bblocks_get_block(n)).collect();
        let ff_regs = if dual_reads.is_empty() && !ff_late_visibility {
            None
        } else {
            let reg_addr = |name: &[u8]| {
                let reg = self.translate.get_register_varnode(name)
                    .map_err(|_| KunaError::lowlevel("Carmel FF register storage is missing"))?;
                Ok::<_, KunaError>(Address::new(reg.space.unwrap(), reg.offset))
            };
            Some([reg_addr(b"FF1")?, reg_addr(b"FF2")?])
        };
        let traces: Vec<_> = blocks.iter().map(|&block| {
            let ports = ff_regs.as_ref().map_or(&[][..], |ports| &ports[..]);
            self.trace(fd, visits, &known, &delayed, &dual_reads, ports, scheduled_data, block)
        }).collect();
        let inputs = incoming_proofs(fd, visits, &blocks, &traces, issue_cells);
        for (trace, mut proof) in traces.iter().zip(inputs.iter().copied()) {
            for &event in &trace.events {
                match event {
                    Event::Read(op, false) if proof.age != CLOCKS => {
                        let addr = fd.obank().get(op).unwrap().get_addr();
                        return Err(KunaError::lowlevel(format!(
                            "Unproved Carmel address admission at CODE {:#x}: {} of {CLOCKS} clocks",
                            addr.get_offset() / 3, proof.age,
                        )));
                    }
                    Event::Read(op, true) if proof.age != CLOCKS
                        && !(proof.entry_ages.is_power_of_two() && proof.entry_ages < MATURE_ENTRY) => {
                        return Err(sample_error(fd, op, "incoming address-admission history"));
                    }
                    Event::FfFetch(op) if !matches!(proof.ff, FF_LIVE | FF_OLDER) => {
                        return Err(sample_error(fd, op, "FF operand fetch"));
                    }
                    Event::RawFfRead(op) if proof.ff & FF_OLDER != 0 => {
                        return Err(sample_error(fd, op, "FF input stage after late write"));
                    }
                    Event::IssueSlot(op) if !proof.slot_offsets.is_power_of_two() => {
                        return Err(sample_error(fd, op, "issue slot"));
                    }
                    Event::OfWord(op) if proof.wb & OLDER_WB != 0 => {
                        return Err(sample_error(fd, op, "DATA operand fetch with older writeback"));
                    }
                    Event::RawLoad(op) if proof.typed_wb & OLDER_WB != 0 => {
                        return Err(sample_error(fd, op, "DATA read with older writeback"));
                    }
                    Event::OpaqueTransfer(op) if proof.typed_wb != 0 => {
                        return Err(sample_error(fd, op, "DATA writeback before opaque control transfer"));
                    }
                    Event::Cell(op, cell, read) if proof.cell_bit(cell)
                        .is_none_or(|bit| read && proof.cells & bit == 0) => {
                        return Err(sample_error(fd, op, "issue cell"));
                    }
                    _ => {}
                }
                proof = event.advance(proof);
            }
        }
        let slots = if reads {
            let scratch = self.translate.get_register_varnode(b"__CE_ADDRESS_HISTORY")
                .map_err(|_| KunaError::lowlevel("Carmel phase history storage is missing"))?;
            let space = scratch.space.as_ref().unwrap();
            Some(std::array::from_fn::<_, CLOCKS, _>(|i| Address::new(Rc::clone(space), scratch.offset + i as u64)))
        } else {
            None
        };
        let entry_history = if entry_reads {
            let vn = self.translate.get_register_varnode(b"CE_ADMISSION_BEFORE_0")
                .map_err(|_| KunaError::lowlevel("Carmel entry admission history storage is missing"))?;
            Some(Address::new(vn.space.unwrap(), vn.offset))
        } else {
            None
        };
        let cells_base = if issue_cells {
            let vn = self.translate.get_register_varnode(b"MAC1_MIDDLE_0")
                .map_err(|_| KunaError::lowlevel("Carmel issue cell storage is missing"))?;
            Some(Address::new(vn.space.unwrap(), vn.offset))
        } else {
            None
        };
        for (trace, mut proof) in traces.into_iter().zip(inputs) {
            for event in trace.events {
                match event {
                    Event::Tick(mark, ..) => {
                        if let Some(slots) = &slots {
                            for i in (1..CLOCKS).rev() {
                                let previous = fd.new_varnode(1, &slots[i - 1], None);
                                copy_before(fd, mark, previous, &slots[i])?;
                            }
                            lower_copy(fd, mark, 1);
                            fd.new_varnode_out(1, &slots[0], mark)?;
                        } else {
                            fd.op_destroy(mark);
                        }
                    }
                    Event::Read(read, allow_entry) => {
                        let address = if allow_entry && proof.age < CLOCKS {
                            entry_history.as_ref().unwrap() + i64::from(proof.entry_ages.trailing_zeros())
                        } else {
                            slots.as_ref().unwrap()[CLOCKS - 1].clone()
                        };
                        let sample = fd.new_varnode(1, &address, None);
                        fd.op_set_opcode(read, type_op_for(OpCode::CPUI_INT_AND));
                        fd.op_set_input(read, sample, 0)?;
                    }
                    Event::FfFetch(read) => lower_copy(fd, read, if proof.ff == FF_OLDER { 2 } else { 1 }),
                    Event::IssueSlot(read) => {
                        let offset = proof.slot_offsets.trailing_zeros();
                        if offset == 0 {
                            lower_copy(fd, read, 1);
                        } else {
                            let offset = fd.new_constant(1, u64::from(offset));
                            fd.op_set_opcode(read, type_op_for(OpCode::CPUI_INT_ADD));
                            fd.op_set_input(read, offset, 0)?;
                        }
                    }
                    Event::Cell(id, cell, read) => {
                        let slot = proof.slot_offsets.trailing_zeros();
                        let address = cells_base.as_ref().unwrap() + i64::from(cell * 64 + slot * 8);
                        if read {
                            let source = fd.new_varnode(8, &address, None);
                            fd.op_set_input(id, source, 1)?;
                            lower_copy(fd, id, 1);
                        } else {
                            lower_copy(fd, id, 2);
                            fd.new_varnode_out(8, &address, id)?;
                        }
                    }
                    _ => {}
                }
                proof = event.advance(proof);
            }
        }
        for marker in copies {
            lower_copy(fd, marker, 1);
        }
        for marker in cleanup {
            fd.op_destroy(marker);
        }
        Ok(())
    }

    fn trace(
        &self, fd: &Funcdata, visits: &VisitedMap, known: &BTreeMap<u64, Boundary>, delayed: &BTreeMap<u64, bool>,
        dual_reads: &BTreeSet<u64>, ff_regs: &[Address], scheduled_data: bool, block: BlockId,
    ) -> Trace {
        let mut trace = Trace::default();
        let mut fixed_bus = None;
        let mut data_access = None;
        let mut current = fd.bb_op_head(block);
        while let Some(id) = current {
            let op = fd.obank().get(id).unwrap();
            let addr = op.get_addr();
            if trace.last.as_ref() != Some(addr) {
                let boundary = known.get(&addr.get_offset()).copied().unwrap_or_default();
                let discontinuity = trace.last.as_ref().is_some_and(|previous| !contiguous(visits, previous, addr));
                if !boundary.address || discontinuity {
                    trace.events.push(Event::Barrier(boundary.ff && !discontinuity));
                }
                trace.first.get_or_insert_with(|| addr.clone());
                trace.last = Some(addr.clone());
                trace.redirect_edges = 0;
                fixed_bus = None;
                data_access = None;
            }
            let marker = userop(fd, op);
            if scheduled_data {
                match op.code() {
                    OpCode::CPUI_LOAD if data_access != Some(OpCode::CPUI_LOAD) => trace.events.push(Event::RawLoad(id)),
                    OpCode::CPUI_STORE if data_access != Some(OpCode::CPUI_STORE) => trace.events.push(Event::DataStore(false)),
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN =>
                        trace.events.push(Event::OpaqueTransfer(id)),
                    _ => {}
                }
                if data_access == Some(op.code()) {
                    data_access = None;
                }
            }
            match marker {
                Some(index) if index == self.tick || Some(index) == self.delay_tick
                    || Some(index) == self.idle_tick || self.cells.is_some_and(|cells| index == cells.tick) => {
                    let known_cells = if Some(index) == self.idle_tick {
                        u16::MAX
                    } else if self.cells.is_some_and(|cells| index == cells.tick) {
                        let mask = fd.vbank().get(op.get_in(2).unwrap()).unwrap();
                        debug_assert!(mask.is_constant() && mask.get_size() == 2);
                        mask.get_offset() as u16
                    } else {
                        0
                    };
                    trace.events.push(Event::Tick(id, false, known_cells));
                }
                Some(index) if Some(index) == self.fixed_read_tick => {
                    trace.events.push(Event::Tick(id, false, 0));
                    fixed_bus = Some(OpCode::CPUI_LOAD);
                }
                Some(index) if Some(index) == self.fixed_load => {
                    fixed_bus = Some(OpCode::CPUI_LOAD);
                }
                Some(index) if Some(index) == self.fixed_store => {
                    fixed_bus = Some(OpCode::CPUI_STORE);
                }
                Some(index) if self.ff.is_some_and(|ff| index == ff.late_tick) => {
                    trace.events.push(Event::Tick(id, true, 0));
                }
                Some(index) if self.ff.is_some_and(|ff| index == ff.visibility) => {
                    trace.events.push(Event::FfVisibility(ff_is_late(fd, op)));
                }
                Some(index) if self.ff.is_some_and(|ff| index == ff.fetch) => trace.events.push(Event::FfFetch(id)),
                Some(index) if index == self.admission || Some(index) == self.entry_admission => {
                    trace.events.push(Event::Read(id, index != self.admission));
                }
                Some(index) if Some(index) == self.issue_slot => trace.events.push(Event::IssueSlot(id)),
                Some(index) if self.cells.is_some_and(|cells| index == cells.read || index == cells.write) => {
                    let cell = fd.vbank().get(op.get_in(1).unwrap()).unwrap();
                    debug_assert!(cell.is_constant() && cell.get_size() == 1);
                    trace.events.push(Event::Cell(id, cell.get_offset() as u32, self.cells.unwrap().read == index));
                }
                Some(index) if self.data.is_some_and(|data| index == data.reserve) => {
                    trace.events.push(Event::OfWord(id));
                    fixed_bus = Some(OpCode::CPUI_LOAD);
                    data_access = fixed_bus;
                }
                Some(index) if self.data.is_some_and(|data| index == data.commit) => {
                    trace.events.push(Event::DataStore(true));
                    fixed_bus = Some(OpCode::CPUI_STORE);
                    data_access = fixed_bus;
                }
                Some(index) if Some(index) == self.end_unmodeled => trace.events.push(Event::Barrier(false)),
                Some(index) if Some(index) == self.redirect => {
                    trace.redirect_edges = if delayed.contains_key(&addr.get_offset()) { 2 } else { 1 };
                }
                _ if fixed_bus == Some(op.code()) => fixed_bus = None,
                _ if op.code() == OpCode::CPUI_LOAD && dual_reads.contains(&addr.get_offset()) => {}
                _ if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN
                    | OpCode::CPUI_LOAD | OpCode::CPUI_STORE) => {
                    // Bus effects and opaque transfers can change the spacing
                    // of the next sample. Prove its age again after this op.
                    trace.events.push(Event::Barrier(false));
                    fixed_bus = None;
                }
                _ => {}
            }
            let typed_ff = self.ff.zip(marker)
                .is_some_and(|(ff, index)| index == ff.capture || index == ff.fetch || index == ff.execute);
            if !typed_ff && op_reads_ff(fd, op, ff_regs) {
                trace.events.push(Event::RawFfRead(id));
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

fn ff_is_late(fd: &Funcdata, op: &PcodeOp) -> bool {
    let mode = fd.vbank().get(op.get_in(1).unwrap()).unwrap();
    debug_assert!(mode.is_constant() && mode.get_offset() <= 1);
    mode.get_offset() == 1
}

fn op_reads_ff(fd: &Funcdata, op: &PcodeOp, ff: &[Address]) -> bool {
    !ff.is_empty() && (0..op.num_input()).any(|i| op.get_in(i)
        .and_then(|id| fd.vbank().get(id))
        .is_some_and(|vn| ff.iter().any(|addr| vn.overlap_range(addr, 2) >= 0)))
}

fn lower_copy(fd: &mut Funcdata, op: OpId, input: i32) {
    let n = fd.obank().get(op).unwrap().num_input();
    for i in (0..n).rev() {
        if i != input {
            fd.op_remove_input(op, i);
        }
    }
    fd.op_set_opcode(op, type_op_for(OpCode::CPUI_COPY));
}

fn lower_operand_fetch(fd: &mut Funcdata, op: OpId, canceled: bool) -> KunaResult<()> {
    if canceled {
        let veto = fd.new_constant(1, 0);
        fd.op_set_input(op, veto, 1)?;
    }
    lower_copy(fd, op, 1);
    Ok(())
}

fn sample_error(fd: &Funcdata, op: OpId, sample: &str) -> KunaError {
    let addr = fd.obank().get(op).unwrap().get_addr();
    KunaError::lowlevel(format!("Unproved Carmel {sample} at CODE {:#x}", addr.get_offset() / 3))
}

// The count sees the first successor's address phase, not its entire issue.
// Only writes explicitly classified as D2 may reach the OF sample.
fn prove_repeat_samples(
    fd: &mut Funcdata, visits: &VisitedMap, samples: &[OpId], d2_write: Option<u64>,
) -> KunaResult<()> {
    let mut successors = BTreeMap::new();
    for &sample in samples {
        let marker = fd.obank().get(sample).unwrap();
        let address = marker.get_addr();
        let following = fd.vbank().get(marker.get_in(1).unwrap()).unwrap().get_offset();
        let Some(visit) = visits.get(address) else {
            return Err(sample_error(fd, sample, "R3 repeat count"));
        };
        let next = address + i64::from(visit.size);
        if next.get_offset() != following || !visits.contains_key(&next) {
            return Err(sample_error(fd, sample, "R3 repeat count"));
        }
        successors.insert(following, sample);
    }
    for (_, id) in fd.obank().iter_all() {
        let op = fd.obank().get(id).unwrap();
        let Some(&sample) = successors.get(&op.get_addr().get_offset()) else { continue };
        if op.get_parent().is_none() { continue; }
        let marker = fd.obank().get(sample).unwrap();
        let register = fd.vbank().get(marker.get_in(2).unwrap()).unwrap();
        let writes = op.get_out().and_then(|out| fd.vbank().get(out))
            .is_some_and(|out| out.overlap_range(register.get_addr(), register.get_size()) >= 0);
        let transfer = exits_issue(fd, op);
        let early = d2_write.is_some_and(|index| userop(fd, op) == Some(index));
        if transfer || (writes && !early) {
            return Err(sample_error(fd, id, "R3 repeat count"));
        }
    }
    for &sample in samples {
        fd.op_destroy(sample);
    }
    Ok(())
}

fn exits_issue(fd: &Funcdata, op: &PcodeOp) -> bool {
    match op.code() {
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
            !fd.vbank().get(op.get_in(0).unwrap()).unwrap().is_constant()
        }
        OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => true,
        _ => false,
    }
}

// OF sees the first younger D2 write, but not the immediately older issue's
// EX write. Prove the neighboring issue phases before binding the count.
fn bind_repeat_operands(
    fd: &mut Funcdata, visits: &VisitedMap, samples: &[OpId], phases: &PhaseDecode<'_>,
) -> KunaResult<()> {
    let mut neighbors = BTreeMap::<Address, Vec<OpId>>::new();
    let mut windows = Vec::with_capacity(samples.len());
    for &sample in samples {
        let op = fd.obank().get(sample).unwrap();
        let at = op.get_addr();
        let next = at + i64::from(visits.get(at).ok_or_else(|| sample_error(fd, sample, "repeat OF count"))?.size);
        if !visits.contains_key(&next) {
            return Err(sample_error(fd, sample, "repeat OF successor"));
        }
        let previous = visits.range(..at.clone()).next_back().map(|(a, _)| a)
            .filter(|a| contiguous(visits, a, at));
        let block = op.get_parent().unwrap();
        let incoming = fd.bblocks_ref().block(block);
        if fd.bb_op_head(block) == Some(sample) {
            let direct = incoming.size_in() != 0 && (0..incoming.size_in()).all(|i| {
                fd.bb_op_tail(incoming.get_in(i)).is_some_and(|end| {
                    Some(fd.obank().get(end).unwrap().get_addr()) == previous
                })
            });
            if !direct && (at != fd.get_address() || incoming.size_in() != 0) {
                return Err(sample_error(fd, sample, "repeat OF predecessor"));
            }
        }
        if previous.is_none() && at != fd.get_address() {
            return Err(sample_error(fd, sample, "repeat OF predecessor"));
        }
        if let Some(previous) = previous {
            neighbors.entry(previous.clone()).or_default();
        }
        neighbors.entry(next.clone()).or_default();
        windows.push((sample, previous.cloned(), next));
    }
    for (_, id) in fd.obank().iter_all() {
        let op = fd.obank().get(id).unwrap();
        if op.get_parent().is_some() {
            if let Some(ops) = neighbors.get_mut(op.get_addr()) { ops.push(id); }
        }
    }
    for (sample, previous, next) in windows {
        let live = fd.obank().get(sample).unwrap().get_in(1).unwrap();
        let operand = fd.vbank().get(live).unwrap();
        let mut old_ex = None;
        let mut young_d2 = None;
        for (younger, address) in previous.iter().map(|a| (false, a)).chain([(true, &next)]) {
            let ops = &neighbors[address];
            let ex = ops.iter().copied().find(|&id| phases.register_ex.is_some_and(|index| {
                userop(fd, fd.obank().get(id).unwrap()) == Some(index)
            }));
            for &id in ops {
                let op = fd.obank().get(id).unwrap();
                let marker = userop(fd, op);
                if exits_issue(fd, op) || matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
                    || marker.is_some_and(|index| Some(index) == phases.reset || Some(index) == phases.end_unmodeled)
                {
                    return Err(sample_error(fd, sample, "repeat OF issue boundary"));
                }
                let writes = op.get_out().and_then(|v| fd.vbank().get(v))
                    .is_some_and(|out| out.overlap_range(operand.get_addr(), operand.get_size()) >= 0);
                if !writes { continue; }
                if phases.repeat_d2_write.is_some_and(|index| marker == Some(index)) {
                    if younger {
                        let value = fd.vbank().get(op.get_in(1).unwrap()).unwrap();
                        if !value.is_constant() || young_d2.replace(value.get_offset()).is_some() {
                            return Err(sample_error(fd, sample, "repeat OF younger D2 value"));
                        }
                    }
                } else if let Some(ex) = ex {
                    if !younger { old_ex = Some(ex); }
                } else {
                    return Err(sample_error(fd, sample, "repeat OF register write"));
                }
            }
        }
        let size = operand.get_size();
        if let Some(value) = young_d2 {
            let value = fd.new_constant(size, value);
            fd.op_set_input(sample, value, 1)?;
        } else if let Some(before) = old_ex {
            let address = fd.obank().get(before).unwrap().get_addr().clone();
            let register = operand.get_addr().clone();
            let copy = fd.new_op(1, address);
            fd.op_set_opcode(copy, type_op_for(OpCode::CPUI_COPY));
            let input = fd.new_varnode(size, &register, None);
            fd.op_set_input(copy, input, 0)?;
            let value = fd.new_unique_out(size, copy)?;
            fd.op_insert_before(copy, before);
            fd.op_set_input(sample, value, 1)?;
        }
        lower_copy(fd, sample, 1);
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mark {
    Tick,
    Idle,
    RegisterEx,
    DataEx,
    OperandFetch,
    ResetExEnd,
    StagedData,
    Slot,
    Redirect,
    EndUnmodeled,
    FixedAccess,
    FfVisibility,
    Omitted,
    DataSchedule,
    Copy,
}

fn prove_omitted_prefix(fd: &Funcdata, visits: &VisitedMap, markers: &[(OpId, u64, Mark)],
                        omitted: u64) -> KunaResult<()> {
    let unproved = || KunaError::lowlevel(format!(
        "Unproved Carmel control flow after omitted sequential slot at CODE {:#x}", omitted / 3,
    ));
    let entry = fd.get_address().get_offset();
    if omitted <= entry || markers.iter().any(|&(_, _, kind)| matches!(kind, Mark::Redirect | Mark::Slot)) {
        return Err(unproved());
    }
    let modeled: BTreeSet<_> = markers.iter().filter_map(|&(_, offset, kind)|
        matches!(kind, Mark::Tick | Mark::Idle | Mark::Omitted).then_some(offset)).collect();
    let mut returned = None;
    let mut unknown = None;
    let mut first = u64::MAX;
    let mut last = 0;
    let graph = fd.bblocks_ref();
    for n in 0..fd.bblocks_get_size() {
        let block = fd.bblocks_get_block(n);
        let edges = graph.block(block);
        let mut previous = None;
        let mut current = fd.bb_op_head(block);
        while let Some(id) = current {
            let op = fd.obank().get(id).unwrap();
            let addr = op.get_addr();
            let offset = addr.get_offset();
            if op.get_halt_type() != 0 || matches!(op.code(),
                OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                || previous.is_some_and(|from| !contiguous(visits, from, addr))
                || (previous.is_none() && edges.size_in() == 0 && offset != entry)
            {
                return Err(unproved());
            }
            if op.code() == OpCode::CPUI_RETURN && returned.replace(offset).is_some() {
                return Err(unproved());
            }
            if !modeled.contains(&offset) && unknown.replace(offset).is_some_and(|old| old != offset) {
                return Err(unproved());
            }
            first = first.min(offset);
            last = last.max(offset);
            previous = Some(addr);
            current = fd.bb_op_next(id);
        }
        let Some(from) = previous else { return Err(unproved()) };
        for edge in 0..edges.size_out() {
            let Some(head) = fd.bb_op_head(edges.get_out(edge)) else { return Err(unproved()) };
            let to = fd.obank().get(head).unwrap().get_addr();
            if !contiguous(visits, from, to) { return Err(unproved()) }
        }
    }
    if first != entry || returned != Some(last) || unknown.is_some_and(|offset| offset != last) {
        return Err(unproved());
    }
    Ok(())
}

#[derive(Clone, Copy, Default)]
struct Boundary {
    address: bool,
    ff: bool,
}

#[derive(Clone, Copy)]
enum Event {
    Barrier(bool),
    Tick(OpId, bool, u16),
    Read(OpId, bool),
    IssueSlot(OpId),
    FfFetch(OpId),
    FfVisibility(bool),
    RawFfRead(OpId),
    OfWord(OpId),
    RawLoad(OpId),
    DataStore(bool),
    OpaqueTransfer(OpId),
    Cell(OpId, u32, bool),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Proof {
    age: usize,
    entry_ages: u8,
    ff: u8,
    slot_offsets: u8,
    wb: u8,
    typed_wb: u8,
    cells: u128,
}

impl Proof {
    const ENTRY: Self = Self { age: 0, entry_ages: ANY_ENTRY_AGE, ff: FF_UNKNOWN,
        slot_offsets: u8::MAX, wb: OLDER_WB, typed_wb: 0, cells: 0 };
    const TOP: Self = Self { age: CLOCKS, entry_ages: 0, ff: 0,
        slot_offsets: 0, wb: 0, typed_wb: 0, cells: u128::MAX };

    fn join(self, other: Self) -> Self {
        Self { age: self.age.min(other.age), entry_ages: self.entry_ages | other.entry_ages, ff: self.ff | other.ff,
            slot_offsets: self.slot_offsets | other.slot_offsets,
            wb: self.wb | other.wb, typed_wb: self.typed_wb | other.typed_wb,
            cells: self.cells & other.cells }
    }

    fn cell_bit(self, cell: u32) -> Option<u128> {
        let mask = 1u16.checked_shl(cell)?;
        (ISSUE_CELLS & mask != 0 && self.slot_offsets.is_power_of_two())
            .then(|| u128::from(mask) << (self.slot_offsets.trailing_zeros() * 16))
    }

    fn invalidate_current_cells(self, known: u16) -> u128 {
        let unknown = ISSUE_CELLS & !known;
        let mut valid = self.cells;
        if unknown != 0 && valid != 0 {
            let mut slots = self.slot_offsets;
            while slots != 0 {
                valid &= !(u128::from(unknown) << (16 * slots.trailing_zeros()));
                slots &= slots - 1;
            }
        }
        valid
    }
}

impl Event {
    fn advance(self, proof: Proof) -> Proof {
        match self {
            Self::Barrier(keep_ff) => Proof {
                age: 0,
                entry_ages: ANY_ENTRY_AGE,
                ff: if keep_ff { proof.ff } else { FF_UNKNOWN | (proof.ff & FF_OLDER) },
                slot_offsets: u8::MAX,
                wb: WB_WINDOW,
                typed_wb: proof.typed_wb,
                cells: 0,
            },
            Self::FfVisibility(late) => Proof { ff: if late { FF_OLDER } else { FF_LIVE }, ..proof },
            Self::Tick(_, late, known_cells) => Proof {
                age: (proof.age + 1).min(CLOCKS),
                entry_ages: ((proof.entry_ages << 1) & ANY_ENTRY_AGE) | (proof.entry_ages & MATURE_ENTRY),
                ff: if late { FF_OLDER } else { FF_LIVE },
                slot_offsets: proof.slot_offsets.rotate_left(1),
                wb: (proof.wb << 1) & WB_WINDOW,
                typed_wb: (proof.typed_wb << 1) & WB_WINDOW,
                cells: proof.invalidate_current_cells(known_cells),
            },
            Self::DataStore(typed) => Proof {
                wb: proof.wb | 1,
                typed_wb: proof.typed_wb | u8::from(typed),
                ..proof
            },
            Self::Cell(_, cell, false) => Proof {
                cells: proof.cells | proof.cell_bit(cell).unwrap_or(0),
                ..proof
            },
            _ => proof,
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

fn incoming_proofs(
    fd: &Funcdata, visits: &VisitedMap, blocks: &[BlockId], traces: &[Trace], issue_cells: bool,
) -> Vec<Proof> {
    let graph = fd.bblocks_ref();
    let mut inputs = vec![Proof::TOP; blocks.len()];
    let mut exits = inputs.clone();
    let mut queue: VecDeque<_> = (0..blocks.len()).collect();
    let mut queued = vec![true; blocks.len()];
    while let Some(i) = queue.pop_front() {
        queued[i] = false;
        let block = graph.block(blocks[i]);
        let trace = &traces[i];
        let proof = if i == 0 && block.size_in() == 0 && trace.first.as_ref() == Some(fd.get_address()) {
            // A real function entry observes its supplied register values,
            // unlike an unproved intra-function redirect or loop backedge.
            Proof { entry_ages: 1, ff: FF_LIVE, slot_offsets: 1,
                cells: if issue_cells { u128::MAX } else { 0 }, ..Proof::ENTRY }
        } else if i == 0 || block.size_in() == 0 {
            Proof::ENTRY
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
                        _ => Proof { typed_wb: exits[predecessor].typed_wb, ..Proof::ENTRY },
                    }
                }
            }).fold(Proof::TOP, Proof::join)
        };
        inputs[i] = proof;
        let exit = trace.events.iter().fold(proof, |p, event| event.advance(p));
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
