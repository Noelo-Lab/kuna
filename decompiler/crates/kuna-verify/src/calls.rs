//! What kuna says about each call the function makes, shared by both sides so
//! a call has one effect whichever side makes it.
//!
//! A call kills the registers its prototype model lists as killed-by-call and
//! every register an output can occupy, pops `extrapop` bytes of stack, and
//! may write the image's writable data and memory near the pointers passed to
//! it (`State::call`). Where kuna has no call at an instruction that calls code
//! in the image, kuna inlined, injected or dropped it: the machine runs the
//! callee for real, so a deliberate removal agrees and a wrong one shows in
//! what the callee does. A call kuna lacks to code outside the image gets a
//! stub with the function's own model, and the comparison reports it.

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::fspec::{effect_type, FuncProto, EXTRAPOP_UNKNOWN};
use kuna_decomp::context::VarnodeId;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

use crate::state::{Effect, Env, Writes};
use crate::value::Halt;

/// Calls one run may make before it is given up as `UNKNOWN(budget)`: each
/// call keeps a snapshot of the memory its callee could read.
pub const MAX_CALLS: usize = 256;

/// Keep a call's record, or end the run when there have been too many.
pub fn record(records: &mut Vec<Record>, r: Record) -> Result<(), Halt> {
    if records.len() >= MAX_CALLS {
        return Err(Halt::Budget);
    }
    records.push(r);
    Ok(())
}
use crate::storage::Storage;

/// Where an argument is passed: the callee prototype's parameter storage. A
/// stack location is relative to the stack pointer the callee starts with. An
/// argument past the prototype's parameters (a variadic one) has no known
/// storage, and the input varnode's own storage is where the value was
/// computed, not where it is passed, so it is not compared.
#[derive(Clone, Debug)]
pub struct Arg {
    pub storage: Storage,
    pub callee_frame: bool,
}

#[derive(Clone, Debug)]
pub struct Site {
    /// The callee kuna names, when it is a known address.
    pub target: Option<u64>,
    /// Bytes of stack the call removes, the return address included; `None`
    /// when kuna does not know.
    pub extrapop: Option<i32>,
    pub noreturn: bool,
    /// Register ranges the call kills.
    pub killed: Vec<(u64, u32)>,
    /// Where each of kuna's arguments is passed, by input slot; `None` where
    /// it is not machine state.
    pub args: Vec<Option<Arg>>,
    /// The callee's parameters are declared, not recovered: an argument that
    /// differs is a difference, not a doubtful prototype.
    pub locked: bool,
    /// Where the callee's prototype returns its value.
    pub output: Option<Storage>,
}

pub struct Calls {
    /// By the address of the call instruction.
    pub by_addr: BTreeMap<u64, Site>,
    pub default: Site,
    /// Every instruction kuna kept an op of, and where its blocks start: a
    /// branch there stays in the function.
    pub inside: BTreeSet<u64>,
    /// Instructions kuna made an indirect jump: a CALLIND the machine lifts
    /// there is executed as one (an option reclassified it).
    pub jumps: BTreeSet<u64>,
    /// Instructions kuna ends in a branch and not a return: a RETURN the
    /// machine lifts there is followed (kuna resolved where it goes).
    pub branches: BTreeSet<u64>,
}

/// One call as a side made it.
#[derive(Clone, Debug)]
pub struct Record {
    /// The address of the call instruction.
    pub at: u64,
    pub target: Option<u64>,
    /// The target was computed (CALLIND, an indirect tail jump) rather than
    /// encoded in the instruction.
    pub indirect: bool,
    pub args: Vec<u128>,
    /// Memory the callee can read: what this side wrote since its last call.
    pub writes: Writes,
    /// What the callee did when the machine ran it for real.
    pub effect: Option<Effect>,
}

impl Calls {
    pub fn site(&self, at: u64) -> &Site {
        self.by_addr.get(&at).unwrap_or(&self.default)
    }

    pub fn known(&self, at: u64) -> bool {
        self.by_addr.contains_key(&at)
    }

    /// A branch at `at` to `target` that kuna turned into a call.
    pub fn tail(&self, at: u64, target: u64) -> bool {
        self.known(at) && !self.inside.contains(&target)
    }
}

fn killed(env: &Env, proto: &FuncProto) -> Vec<(u64, u32)> {
    let reg = env.register.get_index();
    let model = proto.model();
    let mut out: Vec<(u64, u32)> = model
        .effect_list()
        .iter()
        .filter(|e| e.get_type() == effect_type::KILLEDBYCALL)
        .filter_map(|e| {
            let a = e.get_address();
            (a.get_space()?.get_index() == reg).then(|| (a.get_offset(), e.get_size() as u32))
        })
        .collect();
    out.extend(
        model
            .output()
            .get_entry()
            .iter()
            .filter(|e| e.get_space().get_index() == reg)
            .map(|e| (e.get_base(), e.get_size() as u32)),
    );
    out.sort_unstable();
    out.dedup();
    out
}

/// The register a call's value is copied or cast into, for a call whose
/// prototype has no output yet keeps its value in a temporary.
fn copied_to_register(fd: &Funcdata, env: &Env, out: Option<VarnodeId>) -> Option<Storage> {
    let v = fd.vbank().get(out?)?;
    v.descend_iter().filter_map(|d| fd.obank().get(d)).find_map(|d| {
        if !matches!(d.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return None;
        }
        let w = fd.vbank().get(d.get_out()?)?;
        let st = Storage::at(w.get_addr(), w.get_size() as u32)?;
        (st.space.get_index() == env.register.get_index()).then_some(st)
    })
}

fn extrapop(pop: i32) -> Option<i32> {
    (pop != EXTRAPOP_UNKNOWN).then_some(pop)
}

/// Every call site of `fd`, keyed by its instruction address.
pub fn sites(fd: &Funcdata, env: &Env) -> Calls {
    let mut by_addr = BTreeMap::new();
    for i in 0..fd.num_calls() {
        let fc = fd.get_call_specs(i);
        let Some(op) = fd.obank().get(fc.get_op()) else { continue };
        let proto = fc.proto();
        let pop = extrapop(fc.get_effective_extra_pop()).or_else(|| extrapop(fc.get_extra_pop()));
        let sp = Storage::new(&env.register, env.sp.0, env.sp.1);
        let args = (1..op.num_input())
            .map(|k| {
                let p = proto.get_param(k - 1)?;
                let storage = Storage::at(&p.get_address(), p.get_size() as u32)?;
                (!storage.overlaps(&sp)).then_some(Arg { storage, callee_frame: true })
            })
            .collect();
        let entry = fc.get_entry_address();
        let target = (!entry.is_invalid()).then(|| entry.get_offset());
        by_addr.insert(
            op.get_addr().get_offset(),
            Site {
                target,
                extrapop: pop,
                noreturn: proto.is_no_return(),
                killed: killed(env, proto),
                args,
                locked: proto.is_input_locked(),
                output: (proto.get_output().get_size() > 0)
                    .then(|| Storage::at(&proto.get_output().get_address(), proto.get_output().get_size() as u32))
                    .flatten()
                    .or_else(|| copied_to_register(fd, env, op.get_out())),
            },
        );
    }
    let own = fd.get_func_proto();
    let default = Site {
        target: None,
        extrapop: extrapop(own.model().get_extra_pop()),
        noreturn: false,
        killed: killed(env, own),
        args: Vec::new(),
        locked: false,
        output: None,
    };
    let graph = fd.bblocks_ref();
    let blocks: Vec<_> = (0..fd.bblocks_get_size()).map(|i| fd.bblocks_get_block(i)).collect();
    let inside = blocks
        .iter()
        .map(|&b| kuna_decomp::block::block_get_start(&graph.arena, b).get_offset())
        .chain(blocks.iter().flat_map(|&b| fd.bb_ops(b)).filter_map(|op| fd.obank().get(op).map(|o| o.get_addr().get_offset())))
        .collect();
    use kuna_num::opcodes::OpCode;
    let ops: Vec<_> = blocks.iter().flat_map(|&b| fd.bb_ops(b)).filter_map(|op| fd.obank().get(op)).collect();
    let at = |codes: &[OpCode]| -> BTreeSet<u64> {
        ops.iter().filter(|o| codes.contains(&o.code())).map(|o| o.get_addr().get_offset()).collect()
    };
    let jumps = at(&[OpCode::CPUI_BRANCHIND]);
    let returns = at(&[OpCode::CPUI_RETURN]);
    let branches = at(&[OpCode::CPUI_BRANCH, OpCode::CPUI_CBRANCH, OpCode::CPUI_BRANCHIND])
        .into_iter()
        .filter(|a| !returns.contains(a))
        .collect();
    Calls { by_addr, default, inside, jumps, branches }
}
