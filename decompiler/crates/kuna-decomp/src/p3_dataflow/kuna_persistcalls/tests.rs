//! Unit tests for the sparse call guards of a persistent global: which calls
//! [`keep`](super::keep) keeps, and when [`guarded`](super::guarded) lets a
//! read cross them.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

use super::{guarded, keep, MemoryOps};

const X: u64 = 0x4000;

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 2, addrspace_flags::hasphysical, 1, 1,
    )))
    .unwrap();
    m.insert_space(Rc::new(IopSpace::new(3))).unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let entry = Address::new(Rc::clone(glb.manage().get_space_by_name("ram").unwrap()), 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x10000000, 0x40).unwrap()
}

fn addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

fn blocks(fd: &mut Funcdata, n: usize, edges: &[(usize, usize)]) -> Vec<BlockId> {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let g = fd.bblocks_mut();
    let blocks: Vec<BlockId> = (0..n).map(|_| g.new_block_basic(root)).collect();
    for (i, &id) in blocks.iter().enumerate() {
        g.block_mut(id).set_index(i as int4);
    }
    for &(a, b) in edges {
        g.add_edge(blocks[a], blocks[b]);
    }
    g.calc_forward_dominator(root, &[blocks[0]]);
    blocks
}

fn call(fd: &mut Funcdata, bl: BlockId) -> OpId {
    let op = fd.new_op(1, addr(fd, 0x1000));
    fd.op_set_opcode_code(op, OpCode::CPUI_CALL);
    fd.op_insert_end(op, bl);
    op
}

/// A COPY reading `X` into a temporary; returns the op and its free read.
fn read(fd: &mut Funcdata, bl: BlockId) -> (OpId, VarnodeId) {
    let op = fd.new_op(1, addr(fd, 0x1000));
    fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
    let x = addr(fd, X);
    let vn = fd.new_varnode(8, &x, None);
    fd.op_set_input(op, vn, 0).unwrap();
    let _ = fd.new_unique_out(8, op).unwrap();
    fd.op_insert_end(op, bl);
    (op, vn)
}

/// A COPY of a constant into `X`.
fn write(fd: &mut Funcdata, bl: BlockId) -> OpId {
    let op = fd.new_op(1, addr(fd, 0x1000));
    fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
    let c = fd.new_constant(8, 7);
    fd.op_set_input(op, c, 0).unwrap();
    let x = addr(fd, X);
    let _ = fd.new_varnode_out(8, &x, op).unwrap();
    fd.op_insert_end(op, bl);
    op
}

fn ret(fd: &mut Funcdata, bl: BlockId) -> OpId {
    let op = fd.new_op(1, addr(fd, 0x1000));
    fd.op_set_opcode_code(op, OpCode::CPUI_RETURN);
    let c = fd.new_constant(8, 0);
    fd.op_set_input(op, c, 0).unwrap();
    fd.op_insert_end(op, bl);
    op
}

fn space(fd: &Funcdata) -> i32 {
    fd.get_arch().manage().get_space_by_name("ram").unwrap().get_index()
}

#[test]
fn a_call_keeps_its_guard_only_next_to_a_read_or_a_write() {
    let mut fd = build_fd();
    let b = blocks(&mut fd, 1, &[]);
    let c1 = call(&mut fd, b[0]);
    let (_, r) = read(&mut fd, b[0]);
    let c2 = call(&mut fd, b[0]);
    let c3 = call(&mut fd, b[0]);
    let w = write(&mut fd, b[0]);
    let c4 = call(&mut fd, b[0]);
    let c5 = call(&mut fd, b[0]);
    ret(&mut fd, b[0]);
    let calls = [c1, c2, c3, c4, c5];
    let (kept, read_kept) = keep(&fd, &mut MemoryOps::default(), space(&fd), &calls, &[r], &[w], &[]);
    assert!(kept.contains(&c1), "the read sees the call before it");
    assert!(kept.contains(&c3), "the last call before the write is the read value's last reader");
    assert!(kept.contains(&c4), "the call after the write reads it");
    assert_eq!(kept.len(), 3, "c2 and c5 only pass a value along to a reader");
    assert!(read_kept.contains(&c1) && !read_kept.contains(&c3) && !read_kept.contains(&c4));
}

#[test]
fn a_full_write_hides_a_later_read_from_earlier_calls() {
    let mut fd = build_fd();
    let b = blocks(&mut fd, 1, &[]);
    let c1 = call(&mut fd, b[0]);
    let w = write(&mut fd, b[0]);
    let (_, r) = read(&mut fd, b[0]);
    let (kept, read_kept) = keep(&fd, &mut MemoryOps::default(), space(&fd), &[c1], &[r], &[w], &[]);
    assert!(read_kept.is_empty(), "the read sees the write, not the call");
    assert!(kept.contains(&c1), "the call is still the entry value's last reader");
}

#[test]
fn guards_are_kept_across_joins_and_loops() {
    let mut fd = build_fd();
    // 0 -> 1 -> 3, 0 -> 2 -> 3, 3 -> 1 (loop back), 3 -> 4
    let b = blocks(&mut fd, 5, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 1), (3, 4)]);
    let w = write(&mut fd, b[0]);
    let c1 = call(&mut fd, b[1]);
    let c2 = call(&mut fd, b[2]);
    let (_, r) = read(&mut fd, b[3]);
    let c4 = call(&mut fd, b[4]);
    ret(&mut fd, b[4]);
    let (kept, read_kept) = keep(&fd, &mut MemoryOps::default(), space(&fd), &[c1, c2, c4], &[r], &[w], &[]);
    assert!(kept.contains(&c1) && kept.contains(&c2), "both arms reach the read and see the write");
    assert!(read_kept.contains(&c1) && read_kept.contains(&c2));
    assert!(!kept.contains(&c4), "only the return follows the last call");
}

#[test]
fn a_read_crosses_only_guarded_calls() {
    let mut fd = build_fd();
    let b = blocks(&mut fd, 2, &[(0, 1)]);
    let c1 = call(&mut fd, b[0]);
    let c2 = call(&mut fd, b[1]);
    let (r_op, _) = read(&mut fd, b[1]);
    let x = addr(&fd, X);
    assert!(!guarded(&fd, r_op, &x, 8), "c2 has no guard of the global");
    let (mut pending, mut iops) = (Vec::new(), Default::default());
    let g2 = crate::p3_dataflow::kuna_renamereads::new_indirect(&mut fd, c2, &x, 8, 0, &mut pending, &mut iops);
    assert!(guarded(&fd, r_op, &x, 8), "c2's guard is what the read sees");
    fd.op_destroy(g2);
    let w = write(&mut fd, b[1]);
    fd.op_uninsert(w);
    fd.op_insert_before(w, r_op);
    assert!(guarded(&fd, r_op, &x, 8), "a write between the calls and the read defines it");
    let c3 = call(&mut fd, b[1]);
    assert!(!guarded(&fd, r_op, &x, 8), "the value read is live past c3, which has no guard");
    crate::p3_dataflow::kuna_renamereads::new_indirect(&mut fd, c3, &x, 8, 0, &mut pending, &mut iops);
    assert!(guarded(&fd, r_op, &x, 8), "c3's guard keeps the value read apart from the global");
    let _ = c1;
    crate::p3_dataflow::kuna_renamereads::finish(&mut fd, &mut pending);
}

#[test]
fn a_restart_turns_the_sparse_guards_off_once() {
    let mut fd = build_fd();
    assert!(!fd.get_override().persist_calls_off());
    super::restart_full(&mut fd);
    assert!(fd.get_override().persist_calls_off());
    assert!(fd.has_restart_pending());
    fd.set_restart_pending(false);
    super::restart_full(&mut fd);
    assert!(!fd.has_restart_pending(), "a re-flow that already guards every call asks for no second restart");
}
