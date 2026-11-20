//! Tier C, slice C1: run both sides of one function and compare what it returns.
//!
//! The scope is register-only leaf functions. A static scan of the decompiled
//! function, and every machine run, refuse calls and memory beyond the frame and
//! read-only image data as `SKIPPED`. The return value is compared under the
//! physical ABI (the bytes kuna's claimed return occupies) and, when a typed
//! contract is given, at the contract's width with kuna's value converted to the
//! contract type by C's rules.

use std::rc::Rc;
use std::time::{Duration, Instant};

use kuna_base::space::{spacetype, AddrSpace};
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::fspec::{ParameterPieces, PrototypePieces};
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

use crate::contract::{CSizes, CType, Contract};
use crate::divergence::{self, ReturnMismatch};
use crate::state::{Env, State};
use crate::value::Halt;
use crate::{graph, machine};

#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub runs: u32,
    pub seed: u64,
    /// Ops per side per run.
    pub budget: u64,
}

impl Default for Options {
    fn default() -> Self {
        Options { runs: 256, seed: 0x6b75_6e61, budget: 200_000 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Agrees(u32),
    Differs { run: u32, of: u32 },
    Diverges(&'static str),
    Unknown(String),
    Skipped(String),
}

impl Verdict {
    pub fn label(&self) -> String {
        match self {
            Verdict::Agrees(n) => format!("AGREES({n})"),
            Verdict::Differs { .. } => "DIFFERS".to_string(),
            Verdict::Diverges(id) => format!("DIVERGES({id})"),
            Verdict::Unknown(s) | Verdict::Skipped(s) => s.clone(),
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Verdict::Agrees(_) => "AGREES",
            Verdict::Differs { .. } => "DIFFERS",
            Verdict::Diverges(_) => "DIVERGES",
            Verdict::Unknown(_) => "UNKNOWN",
            Verdict::Skipped(_) => "SKIPPED",
        }
    }

    fn from_halt(h: &Halt) -> Verdict {
        if h.is_skip() {
            Verdict::Skipped(h.render())
        } else {
            Verdict::Unknown(h.render())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub function: String,
    pub verdict: Verdict,
    pub lines: Vec<String>,
    pub suspects: Vec<String>,
    pub elapsed: Duration,
}

impl Report {
    pub fn render(&self) -> String {
        let mut s = match self.verdict {
            Verdict::Differs { run, of } => format!("{}  DIFFERS  (run {} of {})\n", self.function, run + 1, of),
            _ => format!("{}  {}\n", self.function, self.verdict.label()),
        };
        for l in &self.lines {
            s.push_str("  ");
            s.push_str(l);
            s.push('\n');
        }
        for p in &self.suspects {
            s.push_str(&format!("  PROTOTYPE_SUSPECT({p})\n"));
        }
        s
    }
}

#[derive(Clone)]
struct Storage {
    space: Rc<AddrSpace>,
    off: u64,
    size: u32,
}

fn hex(v: u128, size: u32) -> String {
    format!("0x{:0width$x}", v, width = (size.max(1) * 2) as usize)
}

fn mask128(size: u32) -> u128 {
    if size >= 16 {
        u128::MAX
    } else {
        (1u128 << (size * 8)) - 1
    }
}

/// C's conversion of kuna's claimed return to the contract type: extend by the
/// claimed type's signedness, or truncate.
fn convert(v: u128, from: u32, signed: bool, to: u32) -> u128 {
    let v = v & mask128(from);
    let negative = signed && from > 0 && from < 16 && (v >> (from * 8 - 1)) & 1 == 1;
    let ext = if negative { v | !mask128(from) } else { v };
    ext & mask128(to)
}

fn pieces(prog: &ConsoleProgram, st: &Storage) -> Vec<Storage> {
    if st.space.get_type() != spacetype::IPTR_JOIN {
        return vec![st.clone()];
    }
    match prog.arch().manage().find_join(st.off) {
        Ok(rec) => (0..rec.num_pieces())
            .filter_map(|i| {
                let p = rec.get_piece(i);
                p.space.as_ref().map(|s| Storage { space: Rc::clone(s), off: p.offset, size: p.size })
            })
            .collect(),
        Err(_) => vec![st.clone()],
    }
}

fn storage_name(prog: &ConsoleProgram, st: &Storage) -> String {
    let parts: Vec<String> = pieces(prog, st)
        .iter()
        .map(|p| {
            if p.space.get_type() == spacetype::IPTR_SPACEBASE {
                return format!("stack[{:#x}]", p.off);
            }
            let name = prog
                .arch()
                .translate()
                .as_sleigh()
                .map(|s| s.base().get_register_name(&p.space, p.off, p.size as i32))
                .unwrap_or_default();
            if name.is_empty() {
                format!("{}:{:#x}", p.space.get_name(), p.off)
            } else {
                String::from_utf8_lossy(&name).into_owned()
            }
        })
        .collect();
    parts.join(":")
}

fn overlaps(a: &Storage, b: &Storage) -> bool {
    a.space.get_index() == b.space.get_index()
        && a.off < b.off.wrapping_add(u64::from(b.size))
        && b.off < a.off.wrapping_add(u64::from(a.size))
}

fn covers(outer: &Storage, inner: &Storage) -> bool {
    outer.space.get_index() == inner.space.get_index()
        && inner.off >= outer.off
        && inner.off + u64::from(inner.size) <= outer.off + u64::from(outer.size)
}

/// The C integer sizes the contract reader needs, from the program's types.
pub fn c_sizes(prog: &ConsoleProgram) -> CSizes {
    let t = prog.arch().types();
    CSizes {
        int: t.get_size_of_int() as u32,
        long: t.get_size_of_long() as u32,
        long_long: t.get_size_of_long_long() as u32,
        pointer: t.get_size_of_pointer() as u32,
    }
}

/// Where the function's prototype model returns an integer of this type.
fn return_storage(prog: &ConsoleProgram, fd: &Funcdata, ty: &CType) -> Result<Storage, String> {
    let arch = prog.arch();
    let meta = if ty.signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
    let dt = arch
        .types()
        .get_base(ty.size as i32, meta)
        .map_err(|e| format!("no {}-byte integer type: {}", ty.size, e.explain()))?;
    let pieces = PrototypePieces { outtype: Some(dt), first_var_arg_slot: -1, ..Default::default() };
    let mut res: Vec<ParameterPieces> = Vec::new();
    fd.get_func_proto()
        .model()
        .assign_parameter_storage(&pieces, &mut res, false, arch.types(), arch.manage())
        .map_err(|e| format!("model assigns no storage to `{}`: {}", ty.spelling, e.explain()))?;
    let p = res.first().ok_or("model assigned no output")?;
    let space = p.addr.get_space().cloned().ok_or("model output has no space")?;
    Ok(Storage { space, off: p.addr.get_offset(), size: ty.size })
}

struct Claim {
    storage: Option<Storage>,
    signed: bool,
    text: String,
    params: Vec<(String, Storage)>,
}

/// The extensions the calling convention promises for the claimed parameters:
/// a 32-bit `int` in a 64-bit RISC-V or MIPS register arrives sign-extended.
/// Inputs that break the promise are not inputs a conforming caller can pass,
/// so both sides start from inputs that keep it. The rule is the one the ABI
/// documents (`narrowext abi`), whatever the run's own `narrowext` value, so a
/// run with the option off is checked against the real convention.
fn extensions(fd: &Funcdata, env: &Env, prog: &ConsoleProgram) -> Vec<crate::state::Extension> {
    use kuna_decomp::kuna_narrowext::{or_model, rules_for, NarrowExtMode};
    let proto = fd.get_func_proto();
    let rule = rules_for(prog.arch(), NarrowExtMode::Abi).input;
    let list = proto.model().input();
    let mut out = Vec::new();
    for i in 0..proto.num_params() {
        let Some(p) = proto.get_param(i) else { continue };
        let addr = p.get_address();
        let Some(space) = addr.get_space() else { continue };
        if space.get_index() != env.register.get_index() {
            continue;
        }
        let size = p.get_size();
        let mut res = kuna_num::pcoderaw::VarnodeData::default();
        let (ext, _) = or_model(rule, list, &addr, size, p.get_type().map(|t| t.as_ref()), &mut res);
        let signed = match ext {
            OpCode::CPUI_INT_SEXT => true,
            OpCode::CPUI_INT_ZEXT => false,
            OpCode::CPUI_PIECE => p.get_type().is_some_and(|t| {
                matches!(t.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT)
            }),
            _ => continue,
        };
        let same_space = res.space.as_ref().is_some_and(|s| s.get_index() == space.get_index());
        if same_space && res.size > size as u32 {
            out.push(crate::state::Extension {
                space: space.get_index(),
                container: (res.offset, res.size),
                param: (addr.get_offset(), size as u32),
                signed,
            });
        }
    }
    out
}

fn claim(fd: &Funcdata) -> Claim {
    let proto = fd.get_func_proto();
    let out = proto.get_output();
    let out_type = proto.get_output_type();
    let meta = out_type.map(|t| t.get_metatype()).unwrap_or(type_metatype::TYPE_VOID);
    let addr = out.get_address();
    let storage = match (meta, addr.get_space()) {
        (type_metatype::TYPE_VOID, _) => None,
        (_, Some(space)) if !addr.is_invalid() && out.get_size() > 0 => {
            Some(Storage { space: Rc::clone(space), off: addr.get_offset(), size: out.get_size() as u32 })
        }
        _ => None,
    };
    let type_name = |t: Option<&Rc<kuna_decomp::dtype::Datatype>>| {
        t.map(|t| t.get_display_name().to_string()).filter(|n| !n.is_empty()).unwrap_or_else(|| "?".into())
    };
    let mut params = Vec::new();
    let mut decl = Vec::new();
    for i in 0..proto.num_params() {
        let Some(p) = proto.get_param(i) else { continue };
        let pname = if p.get_name().is_empty() { format!("a{i}") } else { p.get_name().to_string() };
        decl.push(format!("{} {}", type_name(p.get_type()), pname));
        let a = p.get_address();
        if let Some(space) = a.get_space() {
            params.push((
                pname,
                Storage { space: Rc::clone(space), off: a.get_offset(), size: p.get_size() as u32 },
            ));
        }
    }
    let ret = if storage.is_none() { "void".to_string() } else { type_name(out_type) };
    Claim {
        storage,
        signed: matches!(meta, type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT),
        text: format!("{} {}({})", ret, fd.get_name(), decl.join(", ")),
        params,
    }
}

/// C1's static scope: no call and no store anywhere in the decompiled function.
fn static_scope(fd: &Funcdata) -> Option<Verdict> {
    for i in 0..fd.bblocks_get_size() {
        for op in fd.bb_ops(fd.bblocks_get_block(i)) {
            let Some(o) = fd.obank().get(op) else { continue };
            let at = o.get_addr().get_offset();
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    return Some(Verdict::Skipped(format!("SKIPPED(not-leaf: call at {at:#x})")));
                }
                OpCode::CPUI_STORE => return Some(Verdict::Skipped(format!("SKIPPED(memory: store at {at:#x})"))),
                OpCode::CPUI_CALLOTHER => {
                    let builtin = o
                        .get_in(0)
                        .and_then(|v| fd.vbank().get(v))
                        .filter(|v| v.is_constant())
                        .and_then(|v| graph::memory_builtin(v.get_offset()));
                    if let Some(name) = builtin {
                        return Some(Verdict::Skipped(format!("SKIPPED(memory: {name} at {at:#x})")));
                    }
                }
                _ => {}
            }
        }
    }
    None
}

/// Input registers the body reads that no declared parameter covers (#875's
/// shape). Reported, never a verdict: a custom ABI reads them legitimately.
fn unclaimed_inputs(prog: &ConsoleProgram, env: &Env, fd: &Funcdata, claim: &Claim) -> Vec<String> {
    let params: Vec<Storage> = claim.params.iter().flat_map(|(_, s)| pieces(prog, s)).collect();
    let sp = Storage { space: Rc::clone(&env.register), off: env.sp.0, size: env.sp.1 };
    let mut found: Vec<Storage> = Vec::new();
    for i in 0..fd.bblocks_get_size() {
        for op in fd.bb_ops(fd.bblocks_get_block(i)) {
            let Some(o) = fd.obank().get(op) else { continue };
            for k in 0..o.num_input() {
                let Some(v) = o.get_in(k).and_then(|id| fd.vbank().get(id)) else { continue };
                if !v.is_input() {
                    continue;
                }
                let Some(space) = v.get_addr().get_space() else { continue };
                if space.get_index() != env.register.get_index() {
                    continue;
                }
                let st = Storage { space: Rc::clone(space), off: v.get_offset(), size: v.get_size() as u32 };
                if overlaps(&st, &sp) || params.iter().any(|p| covers(p, &st)) || found.iter().any(|f| covers(f, &st)) {
                    continue;
                }
                found.push(st);
            }
        }
    }
    found
        .iter()
        .map(|s| format!("reads {}, which no declared parameter covers", storage_name(prog, s)))
        .collect()
}

struct UpperWatch {
    extra: Vec<Storage>,
    value: Option<Vec<u128>>,
    every_run: bool,
}

/// Check one decompiled function. `contract`, when given, is the function's
/// real type.
pub fn check_function(prog: &ConsoleProgram, fd: &Funcdata, contract: Option<&Contract>, opts: &Options) -> Report {
    let started = Instant::now();
    let name = fd.get_name().to_string();
    let finish = |verdict: Verdict, lines: Vec<String>, suspects: Vec<String>| Report {
        function: name.clone(),
        verdict,
        lines,
        suspects,
        elapsed: started.elapsed(),
    };
    if let Some(v) = static_scope(fd) {
        return finish(v, Vec::new(), Vec::new());
    }
    let mut env = match Env::new(prog, opts.seed) {
        Ok(e) => e,
        Err(h) => return finish(Verdict::from_halt(&h), Vec::new(), Vec::new()),
    };
    let claim = claim(fd);
    if let Some(out) = claim.storage.as_ref().filter(|o| o.space.get_type() == spacetype::IPTR_INTERNAL) {
        let at = format!("{}:{:#x}", out.space.get_name(), out.off);
        return finish(Verdict::Unknown(format!("UNKNOWN(return storage {at} is not machine state)")), Vec::new(), Vec::new());
    }
    env.extensions = extensions(fd, &env, prog);
    let contract_storage = match contract.filter(|c| !c.ret.is_void()) {
        Some(c) => match return_storage(prog, fd, &c.ret) {
            Ok(s) => Some(s),
            Err(e) => return finish(Verdict::Unknown(format!("UNKNOWN(contract: {e})")), Vec::new(), Vec::new()),
        },
        None => None,
    };
    let mut upper = claim.storage.as_ref().and_then(|out| {
        let wide = return_storage(prog, fd, &CType { spelling: String::new(), size: out.size * 2, signed: false }).ok()?;
        let extra: Vec<Storage> = pieces(prog, &wide).into_iter().filter(|p| !overlaps(p, out)).collect();
        (!extra.is_empty() && pieces(prog, &wide).iter().any(|p| covers(p, out)))
            .then_some(UpperWatch { extra, value: None, every_run: true })
    });
    let entry = fd.get_address().get_offset();
    let starts = graph::block_starts(fd);
    let left_cfg = |mo: &machine::Outcome| mo.indirect.iter().any(|t| !starts.contains(t));
    let mut agreed = 0u32;
    let mut unknown: Option<Halt> = None;
    let mut diverged: Option<&'static str> = None;
    for run in 0..opts.runs {
        let m = machine::run(&env, run, entry, opts.budget);
        let g = graph::run(&env, fd, run, opts.budget);
        let (ms, gv) = match (m, g) {
            (Err(h), _) | (_, Err(h)) if h.is_skip() => {
                return finish(Verdict::from_halt(&h), Vec::new(), Vec::new());
            }
            (Err(Halt::Trap(_)), Err(Halt::Trap(_))) => {
                agreed += 1;
                continue;
            }
            (Ok(mo), Err(Halt::NoRow(_))) if left_cfg(&mo) => {
                diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                continue;
            }
            (Err(h), _) | (_, Err(h)) => {
                unknown.get_or_insert(h);
                continue;
            }
            (Ok(mo), Ok(go)) => {
                if go.unmatched_switch && left_cfg(&mo) {
                    diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                    continue;
                }
                (mo.state, go.ret)
            }
        };
        let read = |st: &Storage| ms.read_storage(&st.space, st.off, st.size);
        let mut lines = Vec::new();
        let mut mismatch = false;
        if let Some(out) = &claim.storage {
            let Some(k) = gv else {
                unknown.get_or_insert(Halt::Harness("RETURN without the prototype's value".into()));
                continue;
            };
            match read(out) {
                Ok(mv) if mv != k & mask128(out.size) => {
                    mismatch = true;
                    lines.push(format!("machine  {} = {}", storage_name(prog, out), hex(mv, out.size)));
                    lines.push(format!("kuna     {} = {}", claim.text, hex(k, out.size)));
                }
                Ok(_) => {}
                Err(h) => {
                    unknown.get_or_insert(h);
                    continue;
                }
            }
        }
        if let (Some(c), Some(cs)) = (contract, &contract_storage) {
            let mv = match read(cs) {
                Ok(v) => v,
                Err(h) => {
                    unknown.get_or_insert(h);
                    continue;
                }
            };
            let kuna = match (&claim.storage, gv) {
                (Some(out), Some(k)) => Some((out.size, convert(k, out.size, claim.signed, cs.size))),
                _ => None,
            };
            if kuna.is_none_or(|(_, kv)| kv != mv) && !mismatch {
                let declared = kuna.and_then(|(ks, kv)| {
                    divergence::find(&ReturnMismatch {
                        arch: &prog.arch().archid,
                        contract_size: cs.size,
                        kuna_size: ks,
                        machine: mv,
                        kuna: kv,
                    })
                });
                if let Some(d) = declared {
                    diverged.get_or_insert(d.id);
                } else {
                    mismatch = true;
                    lines.push(format!("contract {}  ({})", c.declaration, c.source));
                    lines.push(format!("machine  {} = {}", storage_name(prog, cs), hex(mv, cs.size)));
                    match (&claim.storage, gv, kuna) {
                        (Some(out), Some(k), Some((_, kv))) => lines.push(format!(
                            "kuna     {} = {}; as {} {}",
                            claim.text,
                            hex(k, out.size),
                            c.ret.spelling,
                            hex(kv, cs.size)
                        )),
                        _ => lines.push(format!("kuna     {} returns no value", claim.text)),
                    }
                }
            }
        }
        if mismatch {
            let init = State::new(&env, run);
            let inputs: Vec<String> = claim
                .params
                .iter()
                .map(|(n, st)| match init.read_storage(&st.space, st.off, st.size) {
                    Ok(v) => format!("{n}={} ({})", hex(v, st.size), storage_name(prog, st)),
                    Err(_) => format!("{n}=? ({})", storage_name(prog, st)),
                })
                .collect();
            let input = if inputs.is_empty() { "no declared parameters".to_string() } else { inputs.join("  ") };
            lines.insert(usize::from(lines.first().is_some_and(|l| l.starts_with("contract"))), format!("input    {input}"));
            return finish(Verdict::Differs { run, of: opts.runs }, lines, Vec::new());
        }
        if let Some(w) = upper.as_mut() {
            let vals: Option<Vec<u128>> = w
                .extra
                .iter()
                .map(|p| (ms.wrote(&p.space, p.off, p.size)).then(|| read(p).ok()).flatten())
                .collect();
            match (&w.value, vals) {
                (_, None) => w.every_run = false,
                (None, Some(v)) => w.value = Some(v),
                (Some(prev), Some(v)) if *prev != v => w.every_run = false,
                _ => {}
            }
        }
        agreed += 1;
    }
    let mut suspects = unclaimed_inputs(prog, &env, fd, &claim);
    if let Some(w) = upper.filter(|w| w.every_run && agreed == opts.runs) {
        if let Some(vals) = w.value {
            for (p, v) in w.extra.iter().zip(vals) {
                suspects.push(format!(
                    "the machine sets {} to {} on every run; the claimed return leaves it out",
                    storage_name(prog, p),
                    hex(v, p.size)
                ));
            }
        }
    }
    let verdict = match (unknown, diverged) {
        (Some(h), _) => Verdict::from_halt(&h),
        (None, Some(d)) => Verdict::Diverges(d),
        (None, None) => Verdict::Agrees(agreed),
    };
    let lines = match contract {
        Some(c) if matches!(verdict, Verdict::Agrees(_)) => vec![format!("contract {}  ({})", c.declaration, c.source)],
        _ => Vec::new(),
    };
    finish(verdict, lines, suspects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_follows_the_claimed_signedness() {
        assert_eq!(convert(0x8000_0000, 4, true, 8), 0xffff_ffff_8000_0000);
        assert_eq!(convert(0x8000_0000, 4, false, 8), 0x8000_0000);
        assert_eq!(convert(0x1_2345_6789, 8, true, 4), 0x2345_6789);
    }
}
