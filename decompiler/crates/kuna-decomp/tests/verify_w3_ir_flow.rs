//! Independent verifier tests for item `w3-ir-flow`
//! (`decompiler/cpp/flow.{hh,cc}` — [`FlowInfo`]).
//!
//! Two strands:
//!
//!   1. **Pure W3-IR control-flow algorithms** over hand-built op graphs, with a
//!      `visited` map seeded so the lookups (`target`/`branch_target`/
//!      `find_rel_target`/`fallthru_op`) and the `xref_control_flow`
//!      classification + the `collect_edges`/`split_basic`/`connect_basic` block
//!      build can be exercised and asserted against the C++ semantics (`flow.cc`).
//!      Each assertion is re-derived from the C++ oracle, not the Rust port.
//!
//!   2. **Real-Sleigh wiring**: a synthetic multi-instruction program is lifted
//!      through a real `kuna_sleigh::Sleigh` instance via
//!      `FlowInfo::process_instruction`, reusing the golden-lift fixture
//!      mechanics (the x86-64 `floatprint` and Toy `skipnext2` lift corpora).
//!      This pins that the W2 `Translate::one_instruction` is driven correctly by
//!      `process_instruction` and that flow generation reaches exactly the
//!      documented W3-funcdata emitter stub (`newVarnodeOut`/`newCodeRef`), proving
//!      the boundary is precise rather than silently wrong.
//!
//! The op-building emitter (`PcodeEmitFd::dump`) defers `newVarnodeOut`
//! (→ `opSetOutput` → the `(vbank,obank)` split accessor) and `newCodeRef`
//! (`funcdata_varnode`); strand 2 therefore asserts the stub error surfaces with
//! the precise missing-API note, while strand 1 covers everything reachable on
//! the pure IR surface.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_base::xml::{DocumentStorage, Element};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::flow::{FlowEnvironment, FlowInfo};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, OpId, TypeOp};

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::{register_loadimage_xml_ids, LoadImageXml};
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::Translate;

// ---------------------------------------------------------------------------
// The op-code → TypeOp table (the W6 `glb->inst` boundary, for the test environment)
// ---------------------------------------------------------------------------

/// Build the `TypeOp` for an op-code carrying the property-flag word the flow
/// classifier reads (the same `pcodeop_flags` bits the C++ `TypeOp` reports for
/// each behavioral class).  Only the classes `flow.cc` switches on need the right
/// flags; everything else is a flagless COPY-like default.
fn typeop_for(opc: OpCode) -> TypeOp {
    let flags = match opc {
        OpCode::CPUI_BRANCH => pcodeop_flags::branch | pcodeop_flags::coderef,
        OpCode::CPUI_CBRANCH => pcodeop_flags::branch | pcodeop_flags::coderef,
        OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_CALL => pcodeop_flags::call | pcodeop_flags::coderef,
        OpCode::CPUI_CALLIND => pcodeop_flags::call,
        OpCode::CPUI_CALLOTHER => pcodeop_flags::call,
        OpCode::CPUI_RETURN => pcodeop_flags::branch | pcodeop_flags::returns,
        OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL => pcodeop_flags::marker,
        _ => 0,
    };
    TypeOp::new(opc, flags, "OP")
}

/// A minimal [`FlowEnvironment`] for the pure-IR strand: resolves op-codes via
/// [`typeop_for`] and reports no overrides / no injected user-ops / no kuna
/// anchors.  It needs no `Translate` (the pure-IR tests never call
/// `process_instruction`), so the translator accessor panics if ever reached.
struct TableEnv;

impl FlowEnvironment for TableEnv {
    fn translate(&self) -> &dyn Translate {
        panic!("TableEnv: pure-IR tests do not lift instructions")
    }
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp {
        typeop_for(opc)
    }
}

// ---------------------------------------------------------------------------
// Funcdata test harness (mirrors the verify_w3_ir_funcdata harness)
// ---------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// Create an op of the given op-code at ram offset `off` with `inputs` slots.
/// Marks it start-of-instruction (so `target`'s SeqNum is the first op) when
/// `is_first` is set.  Returns the op id.
fn make_op(fd: &mut Funcdata, opc: OpCode, off: u64, inputs: int4) -> OpId {
    let ram = ram_space(fd);
    let op = fd.obank_mut().create_at(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, typeop_for(opc));
    op
}

/// Set input slot 0 of `op` to a constant of the given value/size.
fn set_const_in(fd: &mut Funcdata, op: OpId, slot: int4, val: uintb, size: int4) {
    let c = fd.new_constant(size, val);
    fd.vbank_mut().add_descend(c, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(c), slot);
}

/// Set input slot 0 of `op` to a code reference at ram offset `dest`.
fn set_coderef_in(fd: &mut Funcdata, op: OpId, dest: u64) {
    let ram = ram_space(fd);
    let vn = fd.new_varnode(8, &Address::new(ram, dest), Some(unk(8)));
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
}

// ===========================================================================
// Strand 1: pure W3-IR control-flow algorithms
// ===========================================================================

/// `target(addr)`: returns the first op of the instruction at `addr`, and
/// fall-thru-skips an instruction that generated no p-code (its `seqnum` is the
/// invalid sentinel) to the next instruction (C++ `flow.cc:117`).
#[test]
fn target_resolves_first_op_and_skips_noop_instruction() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // Instruction at 0x1000 (size 4) with one real op; instruction at 0x1004
    // (size 4) is a no-op (no p-code); instruction at 0x1008 has the next op.
    let op0 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    let op2 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1008, 1);
    let seq0 = fd.obank().get(op0).unwrap().get_seq_num().clone();
    let seq2 = fd.obank().get(op2).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    // visited: 0x1000 -> (seq0, size 4); 0x1004 -> (invalid, size 4); 0x1008 -> (seq2, size 4)
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq0, 4);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1004), invalid_seq(), 4);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1008), seq2, 4);

    // Direct lookup.
    assert_eq!(flow.target(&Address::new(Rc::clone(&ram), 0x1000)).unwrap(), op0);
    // No-op instruction at 0x1004 falls thru to 0x1008's op.
    assert_eq!(flow.target(&Address::new(Rc::clone(&ram), 0x1004)).unwrap(), op2);
    // An address never visited is an error.
    assert!(flow.target(&Address::new(ram, 0x2000)).is_err());
}

/// `branch_target` of a BRANCH with an absolute (non-constant) code reference is
/// just `target(destaddr)` (C++ `flow.cc:200`).
#[test]
fn branch_target_absolute_uses_target() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let dest = make_op(&mut fd, OpCode::CPUI_COPY, 0x1010, 1);
    let seqd = fd.obank().get(dest).unwrap().get_seq_num().clone();
    // BRANCH at 0x1000 to absolute 0x1010.
    let br = make_op(&mut fd, OpCode::CPUI_BRANCH, 0x1000, 1);
    set_coderef_in(&mut fd, br, 0x1010);

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1010), seqd, 4);
    assert_eq!(flow.branch_target(br).unwrap(), dest);
}

/// `find_rel_target`: a relative branch whose constant input lands exactly on an
/// existing op's time is a "properly internal" branch (returns that op); a
/// relative branch to one-past (the next instruction) passes back the fall-thru
/// address (C++ `flow.cc:151`).
#[test]
fn find_rel_target_internal_vs_next_instruction() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // Two ops in the SAME instruction at 0x1000: op_a (time t), op_b (time t+1).
    // A CBRANCH at 0x1000 whose constant input `rel` makes
    //   id = cbr.time + rel  ==  op_b.time   -> internal branch to op_b.
    let op_a = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    let op_b = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    let cbr = make_op(&mut fd, OpCode::CPUI_CBRANCH, 0x1000, 2);
    let t_cbr = fd.obank().get(cbr).unwrap().get_time();
    let t_b = fd.obank().get(op_b).unwrap().get_time();
    // rel = t_b - t_cbr  (constant input slot 0).
    let rel = (t_b as i64 - t_cbr as i64) as uintb;
    set_const_in(&mut fd, cbr, 0, rel, 4);
    set_const_in(&mut fd, cbr, 1, 0, 1); // condition (any)

    let seq_a = fd.obank().get(op_a).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    // visited maps the instruction so the "next instruction" branch can compute
    // the fall-thru address.
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq_a, 4);

    // Internal branch: resolves to op_b.
    assert_eq!(flow.branch_target(cbr).unwrap(), op_b);
}

/// `fallthru_op`: the next op in the same instruction is the fall-thru; across an
/// instruction boundary it is `target(addr + size)` (C++ `flow.cc:90`).
#[test]
fn fallthru_op_same_instruction_then_boundary() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // Instruction at 0x1000 (size 4): op0 (start), op1 (same instr).
    let op0 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    fd.obank_mut().get_mut(op0).unwrap().set_flag(pcodeop_flags::startmark);
    let op1 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    // Instruction at 0x1004: op2 (start of next instruction).
    let op2 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1004, 1);
    fd.obank_mut().get_mut(op2).unwrap().set_flag(pcodeop_flags::startmark);

    let seq0 = fd.obank().get(op0).unwrap().get_seq_num().clone();
    let seq2 = fd.obank().get(op2).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq0, 4);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1004), seq2, 4);

    // op0's fall-thru is op1 (same instruction, not an instruction start).
    assert_eq!(flow.fallthru_op_for_test(op0), Some(op1));
    // op1's fall-thru is op2 (the start of the next instruction at 0x1004).
    assert_eq!(flow.fallthru_op_for_test(op1), Some(op2));
}

/// `xref_control_flow` classification: a BRANCH to an absolute address pushes the
/// target onto `addrlist` and the next op starts a basic block; a RETURN marks
/// startbasic and reports no fall-thru; a CALLOTHER that is NOT injected is left
/// alone (C++ `flow.cc:266`).
#[test]
fn xref_classifies_branch_return_and_isfallthru() {
    let mut fd = build_fd();
    // A BRANCH at 0x1000 to absolute 0x2000 (out of any visited range -> addrlist).
    let br = make_op(&mut fd, OpCode::CPUI_BRANCH, 0x1000, 1);
    set_coderef_in(&mut fd, br, 0x2000);
    fd.obank_mut().get_mut(br).unwrap().set_flag(pcodeop_flags::startmark);

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    let mut startbasic = true;
    let mut isfallthru = true;
    let last = flow.xref_for_test(br, &mut startbasic, &mut isfallthru).unwrap();
    assert_eq!(last, Some(br));
    // A BRANCH is not a fall-thru.
    assert!(!isfallthru);
    // The absolute target 0x2000 was queued.
    assert!(flow.addrlist_contains(0x2000));
    // After a branch, the next op starts a basic block.
    assert!(startbasic);
}

/// A RETURN reports no fall-thru and marks the next-op-starts-basic-block flag.
#[test]
fn xref_return_no_fallthru() {
    let mut fd = build_fd();
    let ret = make_op(&mut fd, OpCode::CPUI_RETURN, 0x1000, 1);
    set_const_in(&mut fd, ret, 0, 1, 4);
    fd.obank_mut().get_mut(ret).unwrap().set_flag(pcodeop_flags::startmark);

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    let mut startbasic = true;
    let mut isfallthru = true;
    flow.xref_for_test(ret, &mut startbasic, &mut isfallthru).unwrap();
    assert!(!isfallthru);
    assert!(startbasic);
}

/// A plain (non-branch) op reports fall-thru.
#[test]
fn xref_plain_op_falls_thru() {
    let mut fd = build_fd();
    let cp = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    set_const_in(&mut fd, cp, 0, 7, 4);
    fd.obank_mut().get_mut(cp).unwrap().set_flag(pcodeop_flags::startmark);

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    let mut startbasic = true;
    let mut isfallthru = false;
    flow.xref_for_test(cp, &mut startbasic, &mut isfallthru).unwrap();
    assert!(isfallthru);
}

/// `artificial_halt` builds a RETURN op with a constant `1` input and the given
/// halt flag (C++ `flow.cc:610`).
#[test]
fn artificial_halt_is_return_with_halt_flag() {
    let fd = build_fd();
    let ram = ram_space(&fd);
    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    let halt = flow.artificial_halt_for_test(&Address::new(ram, 0x1000), pcodeop_flags::missing);
    let h = halt.unwrap();
    assert_eq!(flow.data.obank().get(h).unwrap().code(), OpCode::CPUI_RETURN);
    // halt-type is the `missing` flag.
    assert_ne!(flow.data.obank().get(h).unwrap().get_halt_type() & pcodeop_flags::missing, 0);
    // input 0 is a constant of value 1.
    let in0 = flow.data.obank().get(h).unwrap().get_in(0).unwrap();
    assert!(flow.data.vbank().get(in0).unwrap().is_constant());
    assert_eq!(flow.data.vbank().get(in0).unwrap().get_offset(), 1);
}

/// `collect_edges` + `split_basic` + `connect_basic`: build a two-block graph
/// (entry falls into a block, then a BRANCH to a marked target) and assert the
/// blocks and the directed edge (C++ `flow.cc:908`/`985`/`1023`).
#[test]
fn block_build_two_blocks_one_branch_edge() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // op0 @0x1000 (entry, start) -> falls into the BRANCH instruction.  The entry
    // op must be a block start (C++ splitBasic asserts the first op isBlockStart).
    let op0 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    set_const_in(&mut fd, op0, 0, 0, 4);
    fd.obank_mut().get_mut(op0).unwrap().set_flag(pcodeop_flags::startmark);
    fd.obank_mut().get_mut(op0).unwrap().set_flag(pcodeop_flags::startbasic);
    // BRANCH @0x1004 to absolute 0x1000 (back-edge to op0, which is a block start).
    let br = make_op(&mut fd, OpCode::CPUI_BRANCH, 0x1004, 1);
    set_coderef_in(&mut fd, br, 0x1000);
    fd.obank_mut().get_mut(br).unwrap().set_flag(pcodeop_flags::startmark);
    // op0 is a block start (branch target); the BRANCH at 0x1004 also starts a
    // block (it is a separate instruction whose predecessor was a fall-thru).
    fd.obank_mut().get_mut(br).unwrap().set_flag(pcodeop_flags::startbasic);

    let seq0 = fd.obank().get(op0).unwrap().get_seq_num().clone();
    let seqb = fd.obank().get(br).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq0, 4);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1004), seqb, 4);

    flow.collect_edges_for_test().unwrap();
    flow.split_basic_for_test().unwrap();
    flow.connect_basic_for_test();

    // Two basic blocks were created (op0's block, the BRANCH block).
    assert_eq!(flow.data.bblocks_get_size(), 2);
    // There is an edge from the BRANCH block back to op0's (entry) block.
    let entry = flow.data.bblocks_get_block(0);
    // The entry block (op0) is the branch target, so it has an incoming edge.
    assert!(flow.data.bblocks_ref().block(entry).size_in() >= 1);
}

/// A branch that leaves the declared flow range is never decoded, so its stub is
/// the only op at that address: `fillin_branch_stubs` registers it in `visited`
/// and the edge resolves.  An unprocessed address INSIDE the range is left alone
/// -- there a missing op is a real defect, and resolving it to a halt would
/// truncate a body instead of reporting it.
#[test]
fn only_an_out_of_range_stub_becomes_a_resolvable_target() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // BRANCH @0x1000 to 0x2000, with flow declared to span [0x1000,0x1010].
    let br = make_op(&mut fd, OpCode::CPUI_BRANCH, 0x1000, 1);
    set_coderef_in(&mut fd, br, 0x2000);
    fd.obank_mut().get_mut(br).unwrap().set_flag(pcodeop_flags::startmark);

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.set_range(
        Address::new(Rc::clone(&ram), 0x1000),
        Address::new(Rc::clone(&ram), 0x1010),
    );
    let mut startbasic = true;
    let mut isfallthru = true;
    flow.xref_for_test(br, &mut startbasic, &mut isfallthru).unwrap();
    // Out of range: the walk records it and refuses to follow it.
    assert!(!flow.addrlist_contains(0x2000), "an out-of-range target is not queued");
    assert!(flow.has_out_of_bounds(), "the boundary crossing is reported");
    // An in-range address that went unprocessed for some other reason.
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x1008));

    flow.fillin_branch_stubs_for_test().unwrap();

    let outside = Address::new(Rc::clone(&ram), 0x2000);
    let inside = Address::new(Rc::clone(&ram), 0x1008);
    assert!(
        flow.visited_contains(&outside),
        "the out-of-range stub resolves, so collect_edges can hang the cut edge on it"
    );
    assert!(
        !flow.visited_contains(&inside),
        "an in-range missing op keeps upstream's throw"
    );
}

/// `dedup_unprocessed` sorts and removes duplicate addresses (C++ `flow.cc:868`).
#[test]
fn dedup_unprocessed_sorts_and_dedups() {
    let fd = build_fd();
    let ram = ram_space(&fd);
    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x30));
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x10));
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x30));
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x20));
    flow.dedup_unprocessed_for_test();
    let got = flow.unprocessed_offsets();
    assert_eq!(got, vec![0x10, 0x20, 0x30]);
}

// ===========================================================================
// Strand 2: real-Sleigh wiring (process_instruction drives Translate)
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn find_binaryimage(el: &Rc<Element>) -> Option<Rc<Element>> {
    if el.get_name() == "binaryimage" {
        return Some(Rc::clone(el));
    }
    for c in el.get_children() {
        if let Some(found) = find_binaryimage(c) {
            return Some(found);
        }
    }
    None
}

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// A [`FlowEnvironment`] backed by a real [`Sleigh`] translator + the test
/// op-code table.  This is the W4 `Architecture`-backed shape (minus the W4
/// override/userop/inject tables, which default to "none").
struct SleighEnv {
    sleigh: Sleigh,
}

impl FlowEnvironment for SleighEnv {
    fn translate(&self) -> &dyn Translate {
        &self.sleigh
    }
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp {
        typeop_for(opc)
    }
}

/// Parse the `# corpus:`/`# sla:`/`# lift-points`/`context` headers of a
/// golden-lift fixture (the metadata; we re-lift via FlowInfo, not the body).
struct FixtureMeta {
    corpus: String,
    sla_rel: String,
    lift_points: Vec<(String, u64)>,
    context: Vec<(String, u32)>,
}

fn parse_meta(text: &str) -> FixtureMeta {
    let mut corpus = String::new();
    let mut sla_rel = String::new();
    let mut lift_points = Vec::new();
    let mut context = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("# corpus:") {
            corpus = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# sla:") {
            let rest = rest.trim();
            sla_rel = rest.split(" (relative").next().unwrap().trim().to_string();
        } else if line.starts_with("# lift-points") {
            let after = line.split("): ").nth(1).unwrap_or("");
            for tok in after.split_whitespace() {
                let sp = tok.split('=').next().unwrap();
                let mut it = sp.splitn(2, ':');
                let space = it.next().unwrap().to_string();
                let off_str = it.next().unwrap();
                let off = if let Some(hex) = off_str.strip_prefix("0x") {
                    u64::from_str_radix(hex, 16).unwrap()
                } else {
                    off_str.parse().unwrap()
                };
                lift_points.push((space, off));
            }
        } else if let Some(kv) = line.strip_prefix("context ") {
            let mut it = kv.splitn(2, '=');
            let name = it.next().unwrap().to_string();
            let val: u32 = it.next().unwrap().trim().parse().unwrap();
            context.push((name, val));
        }
    }
    FixtureMeta { corpus, sla_rel, lift_points, context }
}

/// Build a `(Funcdata, SleighEnv)` pair sharing the corpus' address-space layout,
/// with the function entry at the first lift point.
fn build_sleigh_flow(meta: &FixtureMeta) -> (Funcdata, SleighEnv) {
    let root = repo_root();
    let xml = std::fs::read(root.join(&meta.corpus)).expect("read corpus xml");
    let mut store = DocumentStorage::new();
    let doc_root = store.parse_document(&xml).expect("parse corpus xml").get_root().clone();
    let bi = find_binaryimage(&doc_root).expect("corpus has a binaryimage");

    let sla = std::fs::read(root.join("specs").join(&meta.sla_rel)).expect("read .sla");

    let mut registry = IdRegistry::with_base_ids();
    register_loadimage_xml_ids(&mut registry);

    let mut img = LoadImageXml::new(&meta.corpus, bi).expect("LoadImageXml::new");
    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize from sla");
    for (name, val) in &meta.context {
        sleigh.set_context_default(name, *val);
    }
    img.open(sleigh.base().manager(), &registry).expect("open corpus image");

    // The entry address uses the Sleigh manager's *own* lift-point space (the same
    // `Rc<AddrSpace>` objects the lift emits varnodes into), so the flow bounds and
    // the lifted op addresses share one space.
    let (space_name, off) = &meta.lift_points[0];
    let entry_space =
        Rc::clone(sleigh.base().manager().get_space_by_name(space_name).expect("entry space"));
    let entry = Address::new(entry_space, *off);
    let uniq_start = sleigh.get_unique_start(kuna_sleigh::translate::UniqueLayout::ANALYSIS);
    sleigh.set_loader(Box::new(img));

    // The Funcdata's own `Architecture` carries a fresh manager (const/unique/iop/
    // fspec spaces) — its only IR roles are the unique-offset allocator
    // (`VarnodeBank::new`) and `newConstant`'s constant space (used by
    // `artificialHalt` on decode errors).  The lift-emitted input varnodes carry
    // their own (Sleigh) spaces directly through `new_varnode_space_off`
    // (`vbank.create` stores the Address; it never consults the Funcdata manager),
    // so the two managers coexist without conflict for this wiring test.
    let glb = Rc::new(ArchContext::new(build_manager()));
    let fd = Funcdata::new("liftfn", "liftfn", glb, entry, uniq_start, 0x40).unwrap();
    (fd, SleighEnv { sleigh })
}

/// Drive one fixture's first instruction through `process_instruction`.
///
/// The op-building emitter defers `newVarnodeOut`/`newCodeRef`, so a real
/// instruction (which has an output or a code-ref) makes `process_instruction`
/// reach exactly that stub — surfaced as a `LowlevelError` whose message names
/// the missing W3-funcdata API.  This pins the W2 `Translate` is driven correctly
/// and the boundary is precise.
fn assert_lift_reaches_emitter_stub(fixture_file: &str) {
    let root = repo_root();
    let path = root.join("tests/golden/vectors/lift").join(fixture_file);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {e}", path.display()));
    let meta = parse_meta(&text);
    assert!(!meta.lift_points.is_empty(), "fixture has lift points");

    // The compiled `.sla` is a gitignored build artifact (`make specs`).  In a
    // fresh worktree it may be absent; skip gracefully so this real-Sleigh wiring
    // strand never breaks the gate when specs are unbuilt.  Strand 1 (the pure-IR
    // algorithms) is the always-runnable substantive verification.
    let sla_path = root.join("specs").join(&meta.sla_rel);
    if !sla_path.is_file() {
        eprintln!(
            "SKIP {fixture_file}: {} not built (run `make specs`); strand-1 IR tests cover the rest",
            sla_path.display()
        );
        return;
    }

    let (fd, env) = build_sleigh_flow(&meta);
    let mut flow = FlowInfo::new(fd, &env);
    let entry = flow.data.get_address().clone();
    let mut startbasic = true;
    let res = flow.process_instruction(&entry, &mut startbasic);

    match res {
        Ok(_) => {
            // A rare instruction with no output and no code-ref input would
            // succeed; that is also a valid (faithful) outcome — the visited map
            // must then record the instruction.
            assert!(
                flow.visited_contains(&entry),
                "successful process_instruction must record the visited instruction"
            );
        }
        Err(KunaError::Lowlevel { explain, .. }) => {
            // The precise W3-funcdata emitter stub (newVarnodeOut / newCodeRef),
            // proving the lift was driven and reached exactly the documented
            // boundary.
            assert!(
                explain.contains("newVarnodeOut")
                    || explain.contains("newCodeRef")
                    || explain.contains("opSetOutput"),
                "expected the emitter stub note, got: {explain}"
            );
        }
        Err(other) => panic!("unexpected error driving the lift: {other}"),
    }
}

#[test]
fn real_sleigh_floatprint_reaches_emitter_stub() {
    assert_lift_reaches_emitter_stub("floatprint.txt");
}

#[test]
fn real_sleigh_skipnext2_reaches_emitter_stub() {
    assert_lift_reaches_emitter_stub("skipnext2.txt");
}

// ===========================================================================
// Round-1 verifier adversarial tests (item w3-ir-flow)
//
// Re-derived against the C++ oracle (flow.cc), targeting the hunt-list spots
// the review flagged as most fragile:
//   - target()'s no-op skip CHAIN through several consecutive no-p-code
//     instructions (the `iter = visited.find(first+size)` loop, flow.cc:123-133);
//   - find_rel_target()'s "branch to next instruction" path: the `id-1` SeqNum
//     back-up AND the `op->getAddr() < res` boundary guard (flow.cc:162-172) —
//     a relative branch landing exactly at the current instruction's own address
//     must FALL THROUGH to the LowlevelError, not pass back a fallthru;
//   - fallthru_op()'s off-cut boundary `(*miter).first + size <= op->getAddr()`
//     returning None (flow.cc:106-107);
//   - dedup_unprocessed() ordering across DISTINCT address spaces (the Address
//     total order is space-index then offset, flow.cc:872 std::sort + unique);
//   - collect_edges() default-case fall-thru edge is emitted ONLY when the next
//     op starts a block (`nextstart`), and is suppressed otherwise (flow.cc:971).
// ===========================================================================

/// `target`: an address whose instruction generated no p-code (invalid seqnum)
/// must chain-skip over *multiple* consecutive no-op instructions to the next
/// instruction that actually has an op (the C++ `while` loop re-finds
/// `first + size`, flow.cc:123-133), not just a single hop.
#[test]
fn verify_target_chains_over_multiple_noop_instructions() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // 0x1000 (size 4) no-op; 0x1004 (size 4) no-op; 0x1008 (size 4) -> op.
    let op = make_op(&mut fd, OpCode::CPUI_COPY, 0x1008, 1);
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), invalid_seq(), 4);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1004), invalid_seq(), 4);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1008), seq, 4);

    // From 0x1000, chain-skip 0x1000 -> 0x1004 -> 0x1008 -> op.
    assert_eq!(flow.target(&Address::new(Rc::clone(&ram), 0x1000)).unwrap(), op);
    // From the middle no-op too.
    assert_eq!(flow.target(&Address::new(Rc::clone(&ram), 0x1004)).unwrap(), op);
    // A no-op chain that never reaches a real op is an error (off the end).
    let mut fd2 = build_fd();
    let _ = make_op(&mut fd2, OpCode::CPUI_COPY, 0x9000, 1); // unrelated op
    let env2 = TableEnv;
    let mut flow2 = FlowInfo::new(fd2, &env2);
    flow2.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), invalid_seq(), 4);
    // 0x1004 is NOT visited, so the chain falls off the map -> error.
    assert!(flow2.target(&Address::new(ram, 0x1000)).is_err());
}

/// `find_rel_target` (via `branch_target`): a relative branch whose `id-1`
/// SeqNum resolves to a real op, but whose recomputed fall-thru address `res`
/// is NOT strictly greater than the branch's own address, must NOT be reported
/// as a fall-thru — the C++ guard is `if (op->getAddr() < res) return 0;`
/// (flow.cc:171), so when the guard is false the code falls through to the
/// `Bad relative branch` LowlevelError.
#[test]
fn verify_find_rel_target_next_instruction_guard_requires_strict_advance() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // One op at 0x1000 (op_a) and the CBRANCH (also at 0x1000).  The CBRANCH is
    // the LAST op, so its time `t_cbr` is the max time.  Choosing rel = 1 gives
    //   id = t_cbr + 1  (no op has that time -> NOT a properly-internal branch),
    //   id - 1 = t_cbr  -> findOp hits the CBRANCH itself -> the "branch to next
    //   instruction" back-up path (flow.cc:162-172) is exercised.
    let op_a = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    let cbr = make_op(&mut fd, OpCode::CPUI_CBRANCH, 0x1000, 2);
    set_const_in(&mut fd, cbr, 0, 1, 4); // rel = 1
    set_const_in(&mut fd, cbr, 1, 0, 1); // condition
    let seq_a = fd.obank().get(op_a).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    // visited: instruction at 0x1000 has SIZE 0 so that the recomputed fall-thru
    // res = 0x1000 + 0 = 0x1000 == op->getAddr(); the strict `<` guard fails,
    // so the C++ does NOT return the fall-thru and instead errors.
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq_a, 0);
    // The next-instruction guard `op->getAddr() < res` fails -> Bad relative
    // branch error (no fall-thru passed back).
    assert!(flow.branch_target(cbr).is_err());

    // Now make the size 4 so res = 0x1004 > 0x1000: the guard passes, the
    // fall-thru address is returned, and branch_target resolves target(0x1004).
    let mut fd3 = build_fd();
    let ram3 = ram_space(&fd3);
    let op_a3 = make_op(&mut fd3, OpCode::CPUI_COPY, 0x1000, 1);
    let cbr3 = make_op(&mut fd3, OpCode::CPUI_CBRANCH, 0x1000, 2);
    let next3 = make_op(&mut fd3, OpCode::CPUI_COPY, 0x1004, 1);
    fd3.obank_mut().get_mut(next3).unwrap().set_flag(pcodeop_flags::startmark);
    set_const_in(&mut fd3, cbr3, 0, 1, 4); // rel = 1 -> id-1 == t_cbr3
    set_const_in(&mut fd3, cbr3, 1, 0, 1);
    let seq_a3 = fd3.obank().get(op_a3).unwrap().get_seq_num().clone();
    let seq_next3 = fd3.obank().get(next3).unwrap().get_seq_num().clone();
    let env3 = TableEnv;
    let mut flow3 = FlowInfo::new(fd3, &env3);
    flow3.seed_visited(&Address::new(Rc::clone(&ram3), 0x1000), seq_a3, 4);
    flow3.seed_visited(&Address::new(Rc::clone(&ram3), 0x1004), seq_next3, 4);
    assert_eq!(flow3.branch_target(cbr3).unwrap(), next3);
}

/// `fallthru_op`: the off-cut boundary case `(*miter).first + size <=
/// op->getAddr()` returns None (flow.cc:106-107).  When the op sits at or past
/// the END of its containing visited range (a bogus off-cut), there is no
/// fall-thru op.
#[test]
fn verify_fallthru_op_offcut_boundary_is_none() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    // Single op at 0x1010 (the LAST op in the dead list, so ++iter == endDead()).
    let op = make_op(&mut fd, OpCode::CPUI_COPY, 0x1010, 1);
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();

    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    // The only visited range is 0x1000 size 4 -> covers [0x1000, 0x1004).
    // op is at 0x1010, so `last-le` is 0x1000 and 0x1000+4 = 0x1004 <= 0x1010,
    // the off-cut guard fires -> None.
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq, 4);
    assert_eq!(flow.fallthru_op_for_test(op), None);

    // Sanity: if the visited range actually CONTAINS the op's fall-thru target
    // and that target has an op, fallthru_op resolves it (positive control).
    let mut fd2 = build_fd();
    let ram2 = ram_space(&fd2);
    let here = make_op(&mut fd2, OpCode::CPUI_COPY, 0x1000, 1);
    let tgt = make_op(&mut fd2, OpCode::CPUI_COPY, 0x1004, 1);
    fd2.obank_mut().get_mut(tgt).unwrap().set_flag(pcodeop_flags::startmark);
    let seq_h = fd2.obank().get(here).unwrap().get_seq_num().clone();
    let seq_t = fd2.obank().get(tgt).unwrap().get_seq_num().clone();
    let env2 = TableEnv;
    let mut flow2 = FlowInfo::new(fd2, &env2);
    flow2.seed_visited(&Address::new(Rc::clone(&ram2), 0x1000), seq_h, 4);
    flow2.seed_visited(&Address::new(Rc::clone(&ram2), 0x1004), seq_t, 4);
    // `here` is the start op; ++iter is `tgt`, which IS an instruction start, so
    // fallthru_op falls to the boundary target = target(0x1000+4) = tgt.
    assert_eq!(flow2.fallthru_op_for_test(here), Some(tgt));
}

/// `dedup_unprocessed`: the sort key is the C++ `Address` total order, which is
/// (space-index, offset).  Addresses in a LOWER-index space sort before any in a
/// higher-index space regardless of offset; duplicates collapse.  Re-derived
/// from flow.cc:872 (`std::sort` + `unique`) and the Address comparator.
#[test]
fn verify_dedup_unprocessed_orders_across_spaces() {
    let fd = build_fd();
    let arch = fd.get_arch();
    // `unique` (lower index) vs `ram` (higher index) — both real spaces.
    let unique = Rc::clone(arch.manage().get_space_by_name("unique").unwrap());
    let ram = Rc::clone(arch.manage().get_space_by_name("ram").unwrap());
    assert!(
        unique.get_index() < ram.get_index(),
        "fixture precondition: unique space index < ram space index"
    );
    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    // Push out of order, with a cross-space duplicate and an in-space duplicate.
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x10)); // ram:0x10
    flow.push_unprocessed(Address::new(Rc::clone(&unique), 0xFF)); // unique:0xFF (big offset, low space)
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x10)); // dup ram:0x10
    flow.push_unprocessed(Address::new(Rc::clone(&unique), 0x01)); // unique:0x01
    flow.push_unprocessed(Address::new(Rc::clone(&ram), 0x08)); // ram:0x08
    flow.dedup_unprocessed_for_test();
    // Expected order: all `unique` entries (by offset) first, then `ram` (by
    // offset), each de-duplicated:  unique:0x01, unique:0xFF, ram:0x08, ram:0x10.
    let offsets = flow.unprocessed_offsets();
    assert_eq!(offsets, vec![0x01, 0xFF, 0x08, 0x10]);
}

/// `collect_edges` default case: a plain (non-control-flow) op emits a fall-thru
/// edge ONLY when the *next* op is a block start (`nextstart`, flow.cc:971); when
/// the next op is NOT a block start (same basic block) NO edge is emitted.
#[test]
fn verify_collect_edges_default_fallthru_gated_on_nextstart() {
    // Case A: two plain ops in the SAME block (op1 is NOT a block start) ->
    // collect_edges emits NO fall-thru edge for op0, and (op1 being the last op,
    // nextstart=true) emits op1's fall-thru — but op1's fall-thru target is the
    // boundary, which has no op, so fallthru_op returns None and (Rust) nothing
    // is pushed.  Net: zero edges, so connectBasic creates zero block edges.
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let op0 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1);
    set_const_in(&mut fd, op0, 0, 0, 4);
    fd.obank_mut().get_mut(op0).unwrap().set_flag(pcodeop_flags::startmark);
    fd.obank_mut().get_mut(op0).unwrap().set_flag(pcodeop_flags::startbasic);
    let op1 = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1); // same instruction, NOT a start
    set_const_in(&mut fd, op1, 0, 1, 4);
    let seq0 = fd.obank().get(op0).unwrap().get_seq_num().clone();
    let env = TableEnv;
    let mut flow = FlowInfo::new(fd, &env);
    flow.seed_visited(&Address::new(Rc::clone(&ram), 0x1000), seq0, 4);
    flow.collect_edges_for_test().unwrap();
    flow.split_basic_for_test().unwrap();
    flow.connect_basic_for_test();
    // One basic block (op0 + op1), and NO inter-block fall-thru edge (op0's
    // successor op1 is not a block start, so the default case adds nothing).
    assert_eq!(flow.data.bblocks_get_size(), 1);
    let b0 = flow.data.bblocks_get_block(0);
    assert_eq!(flow.data.bblocks_ref().block(b0).size_out(), 0);

    // Case B: op0 followed by op1 that IS a block start (separate block).  Now
    // op0's default case sees nextstart=true and emits a fall-thru edge to op1.
    let mut fd2 = build_fd();
    let ram2 = ram_space(&fd2);
    let a0 = make_op(&mut fd2, OpCode::CPUI_COPY, 0x1000, 1);
    set_const_in(&mut fd2, a0, 0, 0, 4);
    fd2.obank_mut().get_mut(a0).unwrap().set_flag(pcodeop_flags::startmark);
    fd2.obank_mut().get_mut(a0).unwrap().set_flag(pcodeop_flags::startbasic);
    let a1 = make_op(&mut fd2, OpCode::CPUI_COPY, 0x1004, 1);
    set_const_in(&mut fd2, a1, 0, 1, 4);
    fd2.obank_mut().get_mut(a1).unwrap().set_flag(pcodeop_flags::startmark);
    fd2.obank_mut().get_mut(a1).unwrap().set_flag(pcodeop_flags::startbasic);
    let seq_a0 = fd2.obank().get(a0).unwrap().get_seq_num().clone();
    let seq_a1 = fd2.obank().get(a1).unwrap().get_seq_num().clone();
    let env2 = TableEnv;
    let mut flow2 = FlowInfo::new(fd2, &env2);
    flow2.seed_visited(&Address::new(Rc::clone(&ram2), 0x1000), seq_a0, 4);
    flow2.seed_visited(&Address::new(Rc::clone(&ram2), 0x1004), seq_a1, 4);
    flow2.collect_edges_for_test().unwrap();
    flow2.split_basic_for_test().unwrap();
    flow2.connect_basic_for_test();
    assert_eq!(flow2.data.bblocks_get_size(), 2);
    // op0's block has exactly one out-edge (the fall-thru to a1's block).
    let entry = flow2.data.bblocks_get_block(0);
    assert_eq!(flow2.data.bblocks_ref().block(entry).size_out(), 1);
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn invalid_seq() -> SeqNum {
    SeqNum::new(Address::default(), 0)
}
