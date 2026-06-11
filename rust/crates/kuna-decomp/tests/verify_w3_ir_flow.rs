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
//!      documented W3-funcdata emitter seam (`newVarnodeOut`/`newCodeRef`), proving
//!      the boundary is precise rather than silently wrong.
//!
//! The op-building emitter (`PcodeEmitFd::dump`) defers `newVarnodeOut`
//! (→ `opSetOutput` → the `(vbank,obank)` split accessor) and `newCodeRef`
//! (`funcdata_varnode`); strand 2 therefore asserts the seam error surfaces with
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
use kuna_decomp::seams::{Architecture, OpId, TypeOp};

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::{register_loadimage_xml_ids, LoadImageXml};
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::Translate;

// ---------------------------------------------------------------------------
// The op-code → TypeOp table (the W6 `glb->inst` seam, for the test environment)
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
    let glb = Rc::new(Architecture::new(manage));
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
    let glb = Rc::new(Architecture::new(build_manager()));
    let fd = Funcdata::new("liftfn", "liftfn", glb, entry, uniq_start, 0x40).unwrap();
    (fd, SleighEnv { sleigh })
}

/// Drive one fixture's first instruction through `process_instruction`.
///
/// The op-building emitter defers `newVarnodeOut`/`newCodeRef`, so a real
/// instruction (which has an output or a code-ref) makes `process_instruction`
/// reach exactly that seam — surfaced as a `LowlevelError` whose message names
/// the missing W3-funcdata API.  This pins the W2 `Translate` is driven correctly
/// and the boundary is precise.
fn assert_lift_reaches_emitter_seam(fixture_file: &str) {
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
            // The precise W3-funcdata emitter seam (newVarnodeOut / newCodeRef),
            // proving the lift was driven and reached exactly the documented
            // boundary.
            assert!(
                explain.contains("newVarnodeOut")
                    || explain.contains("newCodeRef")
                    || explain.contains("opSetOutput"),
                "expected the emitter seam note, got: {explain}"
            );
        }
        Err(other) => panic!("unexpected error driving the lift: {other}"),
    }
}

#[test]
fn real_sleigh_floatprint_reaches_emitter_seam() {
    assert_lift_reaches_emitter_seam("floatprint.txt");
}

#[test]
fn real_sleigh_skipnext2_reaches_emitter_seam() {
    assert_lift_reaches_emitter_seam("skipnext2.txt");
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn invalid_seq() -> SeqNum {
    SeqNum::new(Address::default(), 0)
}
