//! `RuleMultiCollapse` keeps the path its last failed walk stopped on and answers
//! "no change" while that path still ends at a non-matching branch. A rule that
//! keeps its witnesses must answer exactly as a fresh rule (which always walks)
//! does, on random `MULTIEQUAL` webs edited between applications.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_base::types::int4;

use kuna_decomp::action::Rule;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_3::RuleMultiCollapse;
use kuna_decomp::varnode::DefOpInfo;
use kuna_num::opcodes::OpCode;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

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
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt() -> Rc<Datatype> {
    Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN))
}

/// A web: `regs` unwritten inputs and `phis` MULTIEQUALs, one per block.
struct Web {
    fd: Funcdata,
    regs: Vec<VarnodeId>,
    phis: Vec<OpId>,
}

impl Web {
    fn out(&self, phi: OpId) -> VarnodeId {
        self.fd.obank().get(phi).unwrap().get_out().unwrap()
    }
    fn alive(&self, phi: OpId) -> bool {
        self.fd.obank().get(phi).is_some_and(|o| !o.is_dead() && o.code() == OpCode::CPUI_MULTIEQUAL)
    }
    /// A Varnode named by its place in the web, comparable across twins.
    fn label(&self, vn: VarnodeId) -> i64 {
        if let Some(r) = self.regs.iter().position(|&r| r == vn) {
            return r as i64;
        }
        match self.phis.iter().position(|&p| self.alive(p) && self.out(p) == vn) {
            Some(p) => -1 - p as i64,
            None => i64::MIN,
        }
    }
    /// Every live MULTIEQUAL's inputs, by label.
    fn shape(&self) -> Vec<(usize, Vec<i64>)> {
        (0..self.phis.len())
            .filter(|&i| self.alive(self.phis[i]))
            .map(|i| {
                let op = self.fd.obank().get(self.phis[i]).unwrap();
                (i, (0..op.num_input()).map(|s| self.label(op.get_in(s).unwrap())).collect())
            })
            .collect()
    }
}

/// A random web from `seed`: building it twice gives twins.
fn build_web(seed: u64, nphis: usize, nregs: usize) -> Web {
    let mut rng = Rng(seed);
    let mut fd = build_fd();
    let r = ram(&fd);
    let regs: Vec<VarnodeId> = (0..nregs)
        .map(|i| {
            let vn = fd.vbank_mut().create(4, Address::new(Rc::clone(&r), 0x40 + 8 * i as u64), dt());
            fd.vbank_mut().set_input(vn, &mut |_, _, _| Ok(())).unwrap()
        })
        .collect();
    let root = fd.bblocks_ref().root.unwrap();
    let mut phis = Vec::new();
    let mut fanin = Vec::new();
    for i in 0..nphis {
        let n = 2 + rng.below(4);
        let bl = fd.bblocks_mut().new_block_basic(root);
        let op = fd.new_op(n as int4, Address::new(Rc::clone(&r), 0x100 + 0x10 * i as u64));
        fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_MULTIEQUAL, 0, "MULTIEQUAL"));
        fd.op_insert(op, bl, None);
        let seqnum = fd.obank().get(op).unwrap().get_seq_num().clone();
        let vn = fd
            .vbank_mut()
            .create_def_unique(4, dt(), DefOpInfo { id: op, seqnum }, &mut |_, _, _| Ok(()))
            .unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
        fd.vbank_mut().get_mut(vn).unwrap().set_flags_pub(kuna_decomp::varnode::varnode_flags::insert);
        phis.push(op);
        fanin.push(n);
    }
    // Mostly phi outputs and one register, so some webs collapse.
    let common = rng.below(nregs);
    let mut choices = Vec::new();
    for &n in &fanin {
        let mut row = Vec::new();
        for _ in 0..n {
            let pick = match rng.below(10) {
                0..=5 => rng.below(nphis),
                6..=8 => nphis + common,
                _ => nphis + rng.below(nregs),
            };
            row.push(pick);
        }
        choices.push(row);
    }
    let mut web = Web { fd, regs, phis };
    for (i, row) in choices.iter().enumerate() {
        for (slot, &pick) in row.iter().enumerate() {
            let vn = if pick < nphis { web.out(web.phis[pick]) } else { web.regs[pick - nphis] };
            web.fd.op_set_input(web.phis[i], vn, slot as int4).unwrap();
        }
    }
    web
}

#[test]
fn a_rule_keeping_witnesses_answers_as_a_fresh_rule_does() {
    let mut fast_fails = 0usize;
    let mut collapses = 0usize;
    for seed in 1..=1500u64 {
        let nphis = 2 + (seed as usize % 11);
        let nregs = 1 + (seed as usize % 3);
        let mut kept = build_web(seed * 7919, nphis, nregs);
        let mut fresh = build_web(seed * 7919, nphis, nregs);
        let mut rule = RuleMultiCollapse::default();
        let mut rng = Rng(seed * 104729 + 1);
        for _ in 0..60 {
            let live: Vec<usize> = (0..nphis).filter(|&i| kept.alive(kept.phis[i])).collect();
            assert_eq!(live, (0..nphis).filter(|&i| fresh.alive(fresh.phis[i])).collect::<Vec<_>>());
            if live.is_empty() {
                break;
            }
            let i = live[rng.below(live.len())];
            if rng.below(4) == 0 {
                let n = kept.fd.obank().get(kept.phis[i]).unwrap().num_input();
                let slot = rng.below(n as usize) as int4;
                let pick = rng.below(live.len() + nregs);
                for web in [&mut kept, &mut fresh] {
                    let vn = if pick < live.len() { web.out(web.phis[live[pick]]) } else { web.regs[pick - live.len()] };
                    web.fd.op_set_input(web.phis[i], vn, slot).unwrap();
                }
                continue;
            }
            let own = kept.out(kept.phis[i]);
            let op = kept.fd.obank().get(kept.phis[i]).unwrap();
            if (0..op.num_input()).all(|s| op.get_in(s) == Some(own)) {
                continue; // no defining branch: not a heritaged shape
            }
            let witnessed = rule.has_witness(kept.phis[i]);
            let got = rule.apply_op(kept.phis[i], &mut kept.fd);
            let want = RuleMultiCollapse::default().apply_op(fresh.phis[i], &mut fresh.fd);
            assert_eq!(got, want, "seed {seed}: phi {i} answered {got}, a fresh walk {want}");
            if witnessed && want == 0 {
                fast_fails += 1;
            }
            collapses += want as usize;
            assert_eq!(kept.shape(), fresh.shape(), "seed {seed}: the webs diverged");
        }
    }
    assert!(collapses > 100, "only {collapses} collapses: the webs are too hard");
    assert!(fast_fails > 1000, "only {fast_fails} answers came from a witness");
}
