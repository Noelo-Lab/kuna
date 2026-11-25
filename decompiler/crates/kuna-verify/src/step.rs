//! diffverify Tier L, kuna's side: one instruction's p-code, run from a
//! random state of the named registers, with what it read from and wrote to
//! memory and where it goes next. A driver runs the same instruction from the
//! same state under Unicorn (`scripts/diffverify/tier_l.py`) and compares.
//!
//! The instruction's p-code and each op's semantics are the ones Tier C's
//! machine side runs (`machine::lift`, `value::eval`), so a difference here is
//! a difference in what the check believes the machine does.
//!
//! Instructions are picked per form: the mnemonic and its operands with every
//! number replaced, a few of each, from a linear sweep of each function.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_console::engine::ConsoleProgram;
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::VarnodeData;
use serde_json::{json, Value};

use crate::machine::lift;
use crate::state::Env;
use crate::value::{eval, mask, Inputs};

/// A register the driver sets and reads back: its name and storage.
pub struct Reg {
    pub name: String,
    pub offset: u64,
    pub size: u32,
    /// A one-byte flag, given 0 or 1.
    pub flag: bool,
}

pub struct Options {
    pub seed: u64,
    pub states: u32,
    pub per_form: usize,
    pub limit: usize,
    pub sweep: usize,
    /// Keep 8-byte registers below 2^47, so a pointer among them is one the
    /// 48-bit virtual address space of x86-64 and AArch64 can hold.
    pub canonical: bool,
}

fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// The registers named in `names`, flags in `flags`.
pub fn registers(prog: &ConsoleProgram, names: &[String], flags: &[String]) -> Result<Vec<Reg>, String> {
    let mut out = Vec::new();
    for (n, flag) in names.iter().map(|n| (n, false)).chain(flags.iter().map(|n| (n, true))) {
        let vn = prog.arch().translate().get_register_data(n).map_err(|_| format!("no register {n}"))?;
        out.push(Reg { name: n.clone(), offset: vn.offset, size: vn.size, flag });
    }
    Ok(out)
}

struct Machine<'a> {
    env: &'a Env<'a>,
    register: i32,
    regs: HashMap<u64, u8>,
    unique: HashMap<u64, u8>,
    mem: BTreeMap<u64, u8>,
    reads: BTreeMap<u64, u8>,
    writes: BTreeMap<u64, u8>,
    unlisted: BTreeSet<String>,
    seed: u64,
}

impl Machine<'_> {
    fn byte(&mut self, space: &Rc<AddrSpace>, off: u64) -> u8 {
        let idx = space.get_index();
        if idx == self.register {
            if let Some(&b) = self.regs.get(&off) {
                return b;
            }
            let name = self.env.prog.arch().translate().as_sleigh().map(|s| s.base().get_register_name(space, off, 1));
            self.unlisted.insert(name.map(|n| String::from_utf8_lossy(&n).into_owned()).unwrap_or_else(|| format!("{off:#x}")));
            return 0;
        }
        if space.get_type() == spacetype::IPTR_INTERNAL {
            return self.unique.get(&off).copied().unwrap_or(0);
        }
        if let Some(&b) = self.mem.get(&off) {
            return b;
        }
        let image = self.env.prog.read_bytes_at(&Address::new(Rc::clone(space), off), 1).map(|v| v[0]);
        let b = image.unwrap_or_else(|| splitmix(self.seed ^ off.wrapping_mul(0xc2b2_ae3d_27d4_eb4f)) as u8);
        self.mem.insert(off, b);
        self.reads.insert(off, b);
        b
    }

    fn put(&mut self, space: &Rc<AddrSpace>, off: u64, b: u8) {
        let idx = space.get_index();
        if idx == self.register {
            self.regs.insert(off, b);
        } else if space.get_type() == spacetype::IPTR_INTERNAL {
            self.unique.insert(off, b);
        } else {
            self.mem.insert(off, b);
            self.writes.insert(off, b);
        }
    }

    fn read(&mut self, vn: &VarnodeData) -> Result<u128, String> {
        let spc = vn.space.as_ref().ok_or("varnode without a space")?;
        if spc.get_type() == spacetype::IPTR_CONSTANT {
            return Ok(u128::from(vn.offset) & mask(vn.size));
        }
        Ok(self.load(&Rc::clone(spc), vn.offset, vn.size))
    }

    fn load(&mut self, spc: &Rc<AddrSpace>, off: u64, size: u32) -> u128 {
        let mut v = 0u128;
        for i in 0..u64::from(size) {
            let b = u128::from(self.byte(spc, spc.wrap_offset(off.wrapping_add(i))));
            v = if spc.is_big_endian() { (v << 8) | b } else { v | (b << (8 * i)) };
        }
        v
    }

    fn store(&mut self, spc: &Rc<AddrSpace>, off: u64, size: u32, v: u128) {
        for i in 0..u64::from(size) {
            let shift = if spc.is_big_endian() { u64::from(size) - 1 - i } else { i };
            self.put(spc, spc.wrap_offset(off.wrapping_add(i)), (v >> (shift * 8)) as u8);
        }
    }

    fn write(&mut self, vn: &VarnodeData, v: u128) -> Result<(), String> {
        let spc = Rc::clone(vn.space.as_ref().ok_or("varnode without a space")?);
        self.store(&spc, vn.offset, vn.size, v);
        Ok(())
    }

    fn mem_space(&self, vn: &VarnodeData) -> Result<Rc<AddrSpace>, String> {
        vn.get_space_from_const(self.env.prog.arch().manage()).ok_or_else(|| "bad LOAD/STORE space id".into())
    }

    /// Run the instruction at `addr`; the address it goes to next.
    fn run(&mut self, addr: u64) -> Result<u64, String> {
        let insn = lift(self.env, addr).map_err(|h| h.render())?;
        let mut i = 0usize;
        let mut steps = 0;
        while i < insn.ops.len() {
            steps += 1;
            if steps > 100_000 {
                return Err("the instruction's p-code does not finish".into());
            }
            let op = &insn.ops[i];
            let arg = |k: usize| op.ins.get(k).ok_or_else(|| format!("{} lacks input {k}", get_opname(op.opc)));
            match op.opc {
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    let taken = op.opc == OpCode::CPUI_BRANCH || self.read(arg(1)?)? != 0;
                    if !taken {
                        i += 1;
                        continue;
                    }
                    let t = arg(0)?;
                    if t.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT) {
                        i = (t.offset as u32).wrapping_add(i as u32) as i32 as usize;
                        continue;
                    }
                    return Ok(t.offset);
                }
                OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => return Ok(self.read(arg(0)?)? as u64),
                OpCode::CPUI_CALL => return Ok(arg(0)?.offset),
                OpCode::CPUI_CALLOTHER => {
                    let name = crate::program::userop_name(self.env.prog, arg(0)?.offset);
                    let quiet = crate::machine::EXIT_ONLY_USEROPS.contains(&name.as_str())
                        || crate::machine::NO_EFFECT_USEROPS.contains(&name.as_str());
                    if !(quiet && op.out.is_none()) {
                        return Err(format!("CALLOTHER {name}"));
                    }
                }
                OpCode::CPUI_LOAD => {
                    let spc = self.mem_space(arg(0)?)?;
                    let ptr = self.read(arg(1)?)? as u64;
                    let out = op.out.as_ref().ok_or("LOAD without output")?;
                    let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                    let v = self.load(&spc, at, out.size);
                    self.write(out, v)?;
                }
                OpCode::CPUI_STORE => {
                    let spc = self.mem_space(arg(0)?)?;
                    let ptr = self.read(arg(1)?)? as u64;
                    let val = arg(2)?;
                    let v = self.read(val)?;
                    let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                    self.store(&spc, at, val.size, v);
                }
                opc => {
                    let out = op.out.as_ref().ok_or_else(|| format!("{} without output", get_opname(opc)))?;
                    let mut ins = Inputs::default();
                    for vn in &op.ins {
                        ins.push((self.read(vn)?, vn.size));
                    }
                    let v = eval(opc, out.size, ins.as_slice()).map_err(|h| h.render())?;
                    self.write(out, v)?;
                }
            }
            i += 1;
        }
        Ok(addr.wrapping_add(insn.len))
    }
}

fn hex(v: u128) -> String {
    format!("{v:#x}")
}

/// The form an instruction is picked by: its mnemonic and operands with every
/// number replaced by `#`.
fn form(mnemonic: &str, body: &str) -> String {
    let mut out = format!("{} ", mnemonic.to_lowercase());
    let mut in_number = false;
    for c in body.chars() {
        let digit = c.is_ascii_digit() || (in_number && (c.is_ascii_hexdigit() || c == 'x'));
        if digit {
            if !in_number {
                out.push('#');
            }
            in_number = true;
        } else {
            in_number = false;
            out.push(c);
        }
    }
    out
}

/// Pick instructions from `prog` and run each from `opts.states` states.
pub fn run(prog: &ConsoleProgram, regs: &[Reg], opts: &Options, mut emit: impl FnMut(Value)) -> Result<(), String> {
    let canonical = opts.canonical;
    let env = Env::new(prog, opts.seed).map_err(|h| h.render())?;
    let register = env.register.get_index();
    let sleigh = prog.arch().translate().as_sleigh().ok_or("not a SLEIGH translator")?;
    let mut entries: Vec<u64> = prog
        .function_entries_executable()
        .into_iter()
        .filter(|e| prog.entry_bytes_mapped(&e.addr))
        .map(|e| e.addr.get_offset())
        .collect();
    entries.sort_unstable();
    entries.dedup();
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut picked = 0usize;
    let (mut mnemonic, mut body) = (String::new(), String::new());
    for (k, &entry) in entries.iter().enumerate() {
        let end = entries.get(k + 1).copied().unwrap_or(u64::MAX);
        let mut addr = entry;
        for _ in 0..opts.sweep {
            if addr >= end || picked >= opts.limit {
                break;
            }
            let at = Address::new(Rc::clone(&env.code), addr);
            let Ok(len) = sleigh.print_assembly_into(&at, &mut mnemonic, &mut body) else { break };
            let key = form(&mnemonic, &body);
            let count = seen.entry(key.clone()).or_default();
            if *count < opts.per_form {
                *count += 1;
                picked += 1;
                let bytes = prog.read_bytes_at(&at, len.max(1) as usize).unwrap_or_default();
                for s in 0..opts.states {
                    let seed = splitmix(opts.seed ^ addr.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ u64::from(s));
                    emit(one(&env, register, regs, addr, &bytes, &mnemonic, &body, &key, seed, canonical));
                }
            }
            addr = addr.wrapping_add(len.max(1) as u64);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn one(env: &Env, register: i32, regs: &[Reg], addr: u64, bytes: &[u8], mnemonic: &str, body: &str, key: &str, seed: u64, canonical: bool) -> Value {
    let mut m = Machine {
        env,
        register,
        regs: HashMap::new(),
        unique: HashMap::new(),
        mem: BTreeMap::new(),
        reads: BTreeMap::new(),
        writes: BTreeMap::new(),
        unlisted: BTreeSet::new(),
        seed,
    };
    let mut input = serde_json::Map::new();
    for (k, r) in regs.iter().enumerate() {
        let h = splitmix(seed ^ (k as u64 + 1).wrapping_mul(0xd6e8_feb8_6659_fd93));
        let v = if r.flag {
            u128::from(h & 1)
        } else if canonical && r.size == 8 {
            u128::from(h & 0x7fff_ffff_ffff)
        } else {
            (u128::from(h) | (u128::from(splitmix(h)) << 64)) & mask(r.size)
        };
        for i in 0..u64::from(r.size) {
            m.regs.insert(r.offset + i, (v >> (8 * i)) as u8);
        }
        input.insert(r.name.clone(), json!(hex(v)));
    }
    let next = m.run(addr);
    let mut output = serde_json::Map::new();
    for r in regs {
        let mut v = 0u128;
        for i in 0..u64::from(r.size) {
            v |= u128::from(m.regs.get(&(r.offset + i)).copied().unwrap_or(0)) << (8 * i);
        }
        output.insert(r.name.clone(), json!(hex(v)));
    }
    let bytes_of = |map: &BTreeMap<u64, u8>| map.iter().map(|(a, b)| (format!("{a:#x}"), json!(b))).collect::<serde_json::Map<_, _>>();
    json!({
        "addr": hex(u128::from(addr)),
        "bytes": bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(),
        "asm": format!("{mnemonic} {body}").trim().to_string(),
        "form": key,
        "in": input,
        "out": output,
        "reads": bytes_of(&m.reads),
        "writes": bytes_of(&m.writes),
        "next": next.as_ref().map(|n| hex(u128::from(*n))).ok(),
        "error": next.err(),
        "unlisted": m.unlisted.into_iter().collect::<Vec<_>>(),
    })
}
