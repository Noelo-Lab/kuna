//! Unit tests for the no-use memo of `ancestor_op_use`: it answers as the
//! upstream walk over every path does, trial flags included, and visits each
//! Varnode a bounded number of times through MULTIEQUALs with many inputs.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};
use crate::fspec::ParamTrial;
use crate::funcdata::Funcdata;

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.set_default_code_space(2).unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("f", "f", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

struct Ir {
    fd: Funcdata,
    bl: BlockId,
    next: u64,
}

impl Ir {
    fn new() -> Ir {
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        Ir { fd, bl, next: 0x100 }
    }

    fn op(&mut self, opc: OpCode, ninputs: int4) -> OpId {
        self.next += 4;
        let o = self.fd.new_op(ninputs, ram(&self.fd, self.next));
        self.fd.op_set_opcode(o, TypeOp::new(opc, 0, format!("{opc:?}")));
        self.fd.op_insert_end(o, self.bl);
        o
    }

    fn out(&mut self, o: OpId, off: u64, size: int4) -> VarnodeId {
        let at = ram(&self.fd, off);
        self.fd.new_varnode_out(size, &at, o).unwrap()
    }

    fn set(&mut self, o: OpId, ins: &[VarnodeId]) {
        for (i, &v) in ins.iter().enumerate() {
            self.fd.op_set_input(o, v, i as int4).unwrap();
        }
    }

    /// A value written by an `INT_ADD`, read by a `LOAD` when `bad` (so it is not
    /// only a parameter).
    fn leaf(&mut self, off: u64, size: int4, bad: bool) -> VarnodeId {
        let (a, b) = (self.fd.new_constant(size, 1), self.fd.new_constant(size, 2));
        let add = self.op(OpCode::CPUI_INT_ADD, 2);
        self.set(add, &[a, b]);
        let v = self.out(add, off, size);
        if bad {
            self.load(v);
        }
        v
    }

    /// The low half of an `INT_REM` result, which marks the trial `rem_formed`.
    fn rem_leaf(&mut self, off: u64, bad: bool) -> VarnodeId {
        let (a, b) = (self.fd.new_constant(8, 9), self.fd.new_constant(8, 4));
        let rem = self.op(OpCode::CPUI_INT_REM, 2);
        self.set(rem, &[a, b]);
        let wide = self.out(rem, off, 8);
        if bad {
            self.load(wide);
        }
        let zero = self.fd.new_constant(4, 0);
        let sub = self.op(OpCode::CPUI_SUBPIECE, 2);
        self.set(sub, &[wide, zero]);
        self.out(sub, off, 4)
    }

    fn load(&mut self, v: VarnodeId) {
        let space = self.fd.new_constant(8, 2);
        let ld = self.op(OpCode::CPUI_LOAD, 2);
        self.set(ld, &[space, v]);
    }

    /// `call 0x4000(vn)`, the op whose slot-1 trial is scored.
    fn call(&mut self, vn: VarnodeId) -> OpId {
        let target = self.fd.new_code_ref(&ram(&self.fd, 0x4000));
        let call = self.op(OpCode::CPUI_CALL, 2);
        self.set(call, &[target, vn]);
        call
    }

    fn score(&mut self, every_path: bool, maxlevel: int4, vn: VarnodeId, call: OpId) -> (bool, bool, u64) {
        let mut trial = ParamTrial::new(ram(&self.fd, 0x8), 4, 1);
        let (used, steps) = self.fd.ancestor_op_use_counted(every_path, maxlevel, vn, call, &mut trial, 0, 0);
        (used, trial.is_rem_formed(), steps)
    }
}

struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

/// A random graph of MULTIEQUALs, 8-byte PIECEs, 4-byte SUBPIECEs of 8-byte values
/// at both halves, and leaves (some read by a `LOAD`, some `INT_REM` low
/// halves), with cycles, scored from a random value at every level.
fn random_graph(rng: &mut Rng) -> (Ir, Vec<OpId>, VarnodeId, OpId) {
    let mut ir = Ir::new();
    let mut wide = Vec::new();
    let mut narrow = Vec::new();
    let mut phis = Vec::new();
    let mut pieces = Vec::new();
    let mut at = 0x10_000u64;
    let mut place = || {
        at += 0x10;
        at
    };
    for _ in 0..3 + rng.below(6) {
        let fan = 1 + rng.below(4);
        let o = ir.op(OpCode::CPUI_MULTIEQUAL, fan as int4);
        wide.push(ir.out(o, place(), 8));
        phis.push((o, fan));
    }
    for _ in 0..rng.below(3) {
        let o = ir.op(OpCode::CPUI_PIECE, 2);
        wide.push(ir.out(o, place(), 8));
        pieces.push(o);
    }
    for _ in 0..1 + rng.below(3) {
        let bad = rng.below(3) != 0;
        wide.push(ir.leaf(place(), 8, bad));
    }
    for _ in 0..rng.below(4) {
        let whole = wide[rng.below(wide.len())];
        let half = 4 * rng.below(2) as u64;
        let base = ir.fd.vbank().get(whole).unwrap().get_offset();
        let k = ir.fd.new_constant(4, half);
        let o = ir.op(OpCode::CPUI_SUBPIECE, 2);
        ir.set(o, &[whole, k]);
        narrow.push(ir.out(o, base + half, 4));
    }
    for _ in 0..1 + rng.below(4) {
        let bad = rng.below(3) != 0;
        let v = if rng.below(3) == 0 { ir.rem_leaf(place(), bad) } else { ir.leaf(place(), 4, bad) };
        narrow.push(v);
    }
    let all: Vec<VarnodeId> = wide.iter().chain(narrow.iter()).copied().collect();
    for &(o, fan) in &phis {
        let ins: Vec<VarnodeId> = (0..fan).map(|_| all[rng.below(all.len())]).collect();
        ir.set(o, &ins);
    }
    for &o in &pieces {
        let ins = [narrow[rng.below(narrow.len())], narrow[rng.below(narrow.len())]];
        ir.set(o, &ins);
    }
    let root = all[rng.below(all.len())];
    let call = ir.call(root);
    (ir, phis.iter().map(|&(o, _)| o).collect(), root, call)
}

#[test]
fn the_memo_answers_as_the_walk_over_every_path_does() {
    let mut differ = Vec::new();
    let (mut saved, mut used, mut rem) = (0, 0, 0);
    for seed in 1..=3000u64 {
        let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        let (mut ir, phis, root, call) = random_graph(&mut rng);
        for maxlevel in 1..=7 {
            let walk = ir.score(true, maxlevel, root, call);
            let memo = ir.score(false, maxlevel, root, call);
            if (walk.0, walk.1) != (memo.0, memo.1) {
                differ.push((seed, maxlevel, walk.0, walk.1, memo.0, memo.1));
            }
            saved += usize::from(memo.2 < walk.2);
            used += usize::from(walk.0);
            rem += usize::from(walk.1);
        }
        for o in phis {
            assert!(!ir.fd.obank().get(o).unwrap().is_mark(), "seed {seed}: a MULTIEQUAL stayed marked");
        }
    }
    assert!(differ.is_empty(), "memo differs from the full walk (seed, level, walk, memo): {differ:?}");
    assert!(saved > 1000 && used > 1000 && rem > 100, "saved {saved}, used {used}, rem {rem}");
}

/// Four layers of 40 MULTIEQUALs, each reading every output of the layer
/// under it: the full walk tries 40^4 paths to the leaves, the memo each
/// Varnode once.
fn lattice(good_leaf: Option<usize>) -> (Ir, VarnodeId, OpId) {
    const WIDTH: usize = 40;
    let mut ir = Ir::new();
    let mut below: Vec<VarnodeId> =
        (0..WIDTH).map(|i| ir.leaf(0x20_000 + 0x10 * i as u64, 4, Some(i) != good_leaf)).collect();
    for layer in 0..3u64 {
        below = (0..WIDTH)
            .map(|i| {
                let o = ir.op(OpCode::CPUI_MULTIEQUAL, WIDTH as int4);
                let v = ir.out(o, 0x10_000 * (layer + 3) + 0x10 * i as u64, 4);
                ir.set(o, &below.clone());
                v
            })
            .collect();
    }
    let top = ir.op(OpCode::CPUI_MULTIEQUAL, WIDTH as int4);
    let root = ir.out(top, 0x100_000, 4);
    ir.set(top, &below);
    let call = ir.call(root);
    (ir, root, call)
}

#[test]
fn a_lattice_of_wide_multiequals_is_walked_once_per_varnode() {
    let (mut ir, root, call) = lattice(None);
    let (used, _, steps) = ir.score(false, 5, root, call);
    assert!(!used);
    assert!(steps <= 4 * 40 + 1, "{steps} steps");
    let (mut ir, root, call) = lattice(Some(39));
    let (used, _, steps) = ir.score(false, 5, root, call);
    assert!(used, "the one leaf only the call reads is found");
    assert!(steps <= 4 * 40 + 1, "{steps} steps");
    let (used, _, _) = ir.score(false, 4, root, call);
    assert!(!used, "the leaf is past the level bound");
}

/// `r = phi(z.hi, c)`, `z = phi(piece(c, x))`, `c = y.lo`, `y = phi(z)`, with
/// `x` read only by the call. Through `z.hi` the walk reaches `c` with `z`
/// marked and finds nothing, since `z` is then read at offset 4 (`c`, not
/// `x`); reached straight from `r`, `c` leads back to `z` at offset 0 and on to
/// `x`. The first answer is cut by `z`, so it must not answer the second.
#[test]
fn a_no_use_cut_by_a_marked_multiequal_is_not_reused_once_it_is_unmarked() {
    let mut ir = Ir::new();
    let x = ir.leaf(0x20_000, 4, false);
    let z = ir.op(OpCode::CPUI_MULTIEQUAL, 1);
    let zv = ir.out(z, 0x10_000, 8);
    let y = ir.op(OpCode::CPUI_MULTIEQUAL, 1);
    let yv = ir.out(y, 0x11_000, 8);
    ir.set(y, &[zv]);
    let zero = ir.fd.new_constant(4, 0);
    let lo = ir.op(OpCode::CPUI_SUBPIECE, 2);
    ir.set(lo, &[yv, zero]);
    let c = ir.out(lo, 0x11_000, 4);
    let piece = ir.op(OpCode::CPUI_PIECE, 2);
    ir.set(piece, &[c, x]);
    let pv = ir.out(piece, 0x12_000, 8);
    ir.set(z, &[pv]);
    let four = ir.fd.new_constant(4, 4);
    let hi = ir.op(OpCode::CPUI_SUBPIECE, 2);
    ir.set(hi, &[zv, four]);
    let zhi = ir.out(hi, 0x10_004, 4);
    let r = ir.op(OpCode::CPUI_MULTIEQUAL, 2);
    let rv = ir.out(r, 0x13_000, 4);
    ir.set(r, &[zhi, c]);
    let call = ir.call(rv);
    for maxlevel in 1..=8 {
        let walk = ir.score(true, maxlevel, rv, call);
        let memo = ir.score(false, maxlevel, rv, call);
        assert_eq!(walk.0, maxlevel >= 6, "level {maxlevel}");
        assert_eq!(memo.0, walk.0, "level {maxlevel}");
    }
}
