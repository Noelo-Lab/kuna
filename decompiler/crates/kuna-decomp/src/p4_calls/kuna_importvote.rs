//! (kuna) `importvote` — an import with no declaration takes the arguments its
//! other call sites pass (P4 call prototypes).
//!
//! A tail jump to an import (`add esi,1; jmp ext3@plt`) forwards every argument
//! register the function leaves untouched, and nothing in the function names
//! them: without a prototype for the import the forwarded `rdx` is no argument,
//! so `int g(int a,int b,int c){ return ext3(a, b+1, c); }` printed
//! `void g(a0,a1) { ext3(a0,a1 + 1); }`. The import has no body to read, but the
//! image calls it elsewhere: `h` sets `esi` and `edx` before `call ext3@plt` and
//! reads `eax` after it.
//!
//! The analysis tier records every direct call site of each PLT stub. When a
//! function tail-calls (a CALL with an artificial RETURN at its own address) an
//! import that no declaration and no statement reached, [`seed`] decodes a
//! window around each of the import's call sites: the instructions since the
//! previous call or branch (at most 16 back, found by decoding forward from a
//! start that lands exactly on the call) and up to 8 after it. A site's arity
//! runs through the last SysV argument register it writes (an earlier one it
//! leaves alone is its own parameter passed through), and it reads the result
//! when the return register is read before it is written after the call. The
//! import's vote is the largest site arity and whether any site reads the
//! result. It is stated the way `protoorder` states a decompiled callee, marked
//! [`RecoveredTypes::callers_vote`], so `passthrough` takes its registers and
//! the tail call returns the result.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::p4_calls::kuna_protoorder::RecoveredTypes;

const ARG_REGS: [&str; 6] = ["RDI", "RSI", "RDX", "RCX", "R8", "R9"];
const LOOK_BACK: usize = 16;
const LOOK_AHEAD: usize = 8;
const WINDOW: u64 = 64;

type Reg = (int4, u64, u32);

/// What the call sites of one import say about it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImportVote {
    /// The SysV argument registers through the last one some site writes.
    pub arity: u8,
    /// Some site reads the return register after the call.
    pub output: bool,
}

#[derive(Default)]
struct Emit {
    call: bool,
    branches: bool,
    writes: Vec<Reg>,
    reads: Vec<Reg>,
}

fn overlaps(a: Reg, b: Reg) -> bool {
    a.0 == b.0 && a.1 < b.1 + u64::from(b.2) && b.1 < a.1 + u64::from(a.2)
}

fn processor(v: &VarnodeData) -> Option<Reg> {
    let sp = v.space.as_ref()?;
    (sp.get_type() == spacetype::IPTR_PROCESSOR).then(|| (sp.get_index(), v.offset, v.size))
}

impl kuna_sleigh::translate::PcodeEmit for Emit {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        for r in vars.iter().filter_map(processor) {
            if !self.writes.iter().any(|w| overlaps(*w, r)) {
                self.reads.push(r);
            }
        }
        if let Some(r) = outvar.and_then(processor) {
            self.writes.push(r);
        }
        match opc {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => self.call = true,
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => {
                let internal = vars
                    .first()
                    .and_then(|v| v.space.as_ref())
                    .is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT);
                self.branches |= !internal;
            }
            _ => {}
        }
    }
}

fn decode(tr: &dyn kuna_sleigh::translate::Translate, space: &Rc<kuna_base::space::AddrSpace>, at: u64) -> Option<(Emit, u64)> {
    let mut emit = Emit::default();
    let len = tr.one_instruction(&mut emit, &Address::new(Rc::clone(space), at)).ok().filter(|&n| n > 0)?;
    Some((emit, len as u64))
}

/// The instructions before `site`, decoded from the earliest start in the
/// window whose instruction boundaries land exactly on `site`.
fn before(tr: &dyn kuna_sleigh::translate::Translate, space: &Rc<kuna_base::space::AddrSpace>, site: u64) -> Vec<Emit> {
    for back in (1..=WINDOW).rev() {
        let Some(mut at) = site.checked_sub(back) else { continue };
        let mut insns = Vec::new();
        while at < site {
            let Some((e, len)) = decode(tr, space, at) else { break };
            insns.push(e);
            at += len;
        }
        if at == site {
            return insns;
        }
    }
    Vec::new()
}

fn site_vote(tr: &dyn kuna_sleigh::translate::Translate, space: &Rc<kuna_base::space::AddrSpace>, site: u64, regs: &[Reg], rax: Reg) -> Option<ImportVote> {
    let (call, len) = decode(tr, space, site)?;
    if !call.call {
        return None;
    }
    let mut written = vec![false; regs.len()];
    for prev in before(tr, space, site).iter().rev().take(LOOK_BACK) {
        if prev.call || prev.branches {
            break;
        }
        for (i, r) in regs.iter().enumerate() {
            written[i] |= prev.writes.iter().any(|w| overlaps(*w, *r));
        }
    }
    let arity = written.iter().rposition(|&w| w).map_or(0, |i| i + 1) as u8;
    let mut output = false;
    let mut at = site + len;
    for _ in 0..LOOK_AHEAD {
        let Some((e, len)) = decode(tr, space, at) else { break };
        if e.reads.iter().any(|r| overlaps(*r, rax)) {
            output = true;
            break;
        }
        if e.writes.iter().any(|w| overlaps(*w, rax)) || e.call || e.branches {
            break;
        }
        at += len;
    }
    Some(ImportVote { arity, output })
}

fn register(arch: &Architecture, name: &str) -> Option<(Address, int4)> {
    let v = arch.translate().probe_register_varnode(name.as_bytes())?;
    Some((Address::new(Rc::clone(v.space.as_ref()?), v.offset), v.size as int4))
}

/// The vote of the import at `stub` over its recorded call sites.
pub fn vote(arch: &Architecture, stub: &Address) -> Option<ImportVote> {
    let space = stub.get_space()?;
    let sites = arch.kuna_import_sites.get(&(space.get_index(), stub.get_offset()))?;
    let reg = |name: &str| register(arch, name).and_then(|(a, s)| Some((a.get_space()?.get_index(), a.get_offset(), s as u32)));
    let regs: Vec<Reg> = ARG_REGS.iter().filter_map(|r| reg(r)).collect();
    let rax = reg("RAX")?;
    if regs.len() != ARG_REGS.len() {
        return None;
    }
    let tr = arch.translate();
    let mut out = ImportVote::default();
    for &site in sites {
        if let Some(v) = site_vote(tr, space, site, &regs, rax) {
            out.arity = out.arity.max(v.arity);
            out.output |= v.output;
        }
    }
    Some(out)
}

/// State the vote of every undeclared import `data` tail-calls, before the
/// drive seeds the `protoorder` statements. A no-op unless `importvote` is on
/// and the analysis tier recorded call sites.
pub fn seed(arch: &mut Architecture, data: &Funcdata) {
    if !arch.analysis_importvote || arch.kuna_import_sites.is_empty() {
        return;
    }
    let returns: Vec<Address> = data
        .obank()
        .iter_alive()
        .filter_map(|op| data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_RETURN).map(|o| o.get_addr().clone()))
        .collect();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let Some(at) = data.obank().get(fc.get_op()).map(|o| o.get_addr().clone()) else { continue };
        if !returns.contains(&at) {
            continue;
        }
        let stub = fc.get_entry_address().clone();
        let Some(key) = crate::p4_calls::kuna_protoorder::stated_key(&stub) else { continue };
        if arch.kuna_protoorder_types.contains_key(&key) || arch.symboltab.function_proto_pieces_across_scopes(&stub).is_some() {
            continue;
        }
        let Some(v) = vote(arch, &stub).filter(|v| v.arity > 0 || v.output) else { continue };
        if let Some(stated) = statement(arch, v) {
            arch.kuna_protoorder_types.insert(key, Rc::new(stated));
        }
    }
}

fn statement(arch: &Architecture, v: ImportVote) -> Option<RecoveredTypes> {
    let ct = crate::dtype::TypeFactory::get_base(arch.types(), 8, type_metatype::TYPE_UNKNOWN).ok()?;
    let inputs: Vec<(Address, int4, Rc<crate::dtype::Datatype>)> = ARG_REGS
        .iter()
        .take(v.arity as usize)
        .filter_map(|r| register(arch, r))
        .map(|(a, s)| (a, s, Rc::clone(&ct)))
        .collect();
    if inputs.len() != v.arity as usize {
        return None;
    }
    let output = if v.output { register(arch, "RAX").map(|(a, s)| (a, s, Rc::clone(&ct))) } else { None };
    let reads = inputs.iter().map(|(a, s, _)| (a.clone(), *s)).collect();
    let result = output.as_ref().map(|(a, s, _)| (a.clone(), *s));
    Some(RecoveredTypes {
        arity_sound: !inputs.is_empty(),
        inputs,
        output,
        result,
        vararg_tail: Vec::new(),
        reads,
        passes: Vec::new(),
        callers_vote: true,
    })
}
