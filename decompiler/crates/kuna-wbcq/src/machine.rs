//! One step of a step function `result step(ctx *, input)`, run on its own
//! machine code: the instructions lifted by the program's SLEIGH translator
//! (`kuna_verify::machine::lift`) and executed op by op with kuna-verify's op
//! semantics (`kuna_verify::value::eval`).
//!
//! Each byte carries its concrete value, an expression over the input's bytes
//! when it depends on the input (`kuna_verify::concolic`), and the sources it
//! depends on: each context byte, and the input. A step that leaves the model
//! stops with the reason: a call, a user op with an effect, memory outside the
//! context, the frame and the image, an address or a jump computed from the
//! input or the context, a big-endian space, or a loop past the budget.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_console::engine::ConsoleProgram;
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::VarnodeData;
use kuna_verify::concolic::{self, Branch, Expr, Path, Place};
use kuna_verify::machine::{lift, NO_EFFECT_USEROPS};
use kuna_verify::state::Env;
use kuna_verify::value::{eval, mask, Inputs};

/// The dependency bit of the input; bit `i` below it is context byte `i`.
pub const INPUT: u64 = 1 << 63;
/// In a pin mask, the result (context bytes are bits `0..MAX_CONTEXT`).
pub const RESULT: u64 = INPUT;
pub const PIN_AT: u64 = 0xffff_ffff_0000_0000;
pub const MAX_CONTEXT: usize = 63;
pub const INPUT_BYTES: usize = 4;
const CONTEXT_BASE: u64 = 0x5000_0000;
const STACK_TOP: u64 = 0x7ff0_0000;
const STACK_SIZE: u64 = 0x1_0000;
const RETURN_TO: u64 = 0xdead_0000;
pub const INPUT_SPACE: i32 = -1;
const BUDGET: u64 = 200_000;

/// Where the step function takes its arguments and leaves its result.
pub struct Abi {
    pub context: VarnodeData,
    pub input: VarnodeData,
    pub result: VarnodeData,
    pub sp: VarnodeData,
    /// The register a call leaves its return address in; `None` when the
    /// call pushes it.
    pub link: Option<VarnodeData>,
}

impl Abi {
    pub fn named(
        prog: &ConsoleProgram,
        names: [&str; 4],
        link: Option<&str>,
    ) -> Result<Abi, String> {
        let reg = |n: &str| {
            prog.arch()
                .translate()
                .get_register_data(n)
                .map_err(|_| format!("no register {n}"))
        };
        Ok(Abi {
            context: reg(names[0])?,
            input: reg(names[1])?,
            result: reg(names[2])?,
            sp: reg(names[3])?,
            link: link.map(reg).transpose()?,
        })
    }

    /// The System V x86-64 and AAPCS64 conventions, by the program's
    /// processor.
    pub fn for_program(prog: &ConsoleProgram) -> Result<Abi, String> {
        if Abi::named(prog, ["RDI", "RSI", "EAX", "RSP"], None).is_ok() {
            return Abi::named(prog, ["RDI", "RSI", "EAX", "RSP"], None);
        }
        Abi::named(prog, ["x0", "x1", "w0", "sp"], Some("x30"))
    }
}

#[derive(Clone)]
struct Cell {
    v: u8,
    e: Option<Rc<Expr>>,
    dep: u64,
}

impl Cell {
    fn concrete(v: u8) -> Cell {
        Cell { v, e: None, dep: 0 }
    }
}

struct Value {
    v: u128,
    e: Option<Rc<Expr>>,
    dep: u64,
}

/// What one step did.
#[derive(Clone)]
pub struct StepRun {
    pub result: u64,
    /// The sources the result depends on.
    pub result_dep: u64,
    pub context: Vec<u8>,
    pub context_dep: Vec<u64>,
    /// The branches whose condition depends on the input, with the input's
    /// bytes as variables.
    pub path: Path,
    /// Every conditional branch the step took or did not: instruction, op,
    /// direction. Two runs from one context with the same signature took the
    /// same path.
    pub signature: Vec<(u64, u32, bool)>,
    /// Context bytes a branch condition read.
    pub control_reads: u64,
    /// A branch depended on the input through an op the expressions do not
    /// cover: path enumeration cannot be complete.
    pub incomplete: Option<String>,
    /// Values pinned by a value branch (`RESULT`, context bytes): their
    /// value depends on the input, and the path fixes it.
    pub pinned: u64,
}

struct Step<'a, 'p> {
    env: &'a Env<'p>,
    regs: BTreeMap<u64, Cell>,
    unique: BTreeMap<u64, Cell>,
    ram: BTreeMap<u64, Cell>,
    context_len: u64,
    ops: u64,
    run: StepRun,
}

fn compose(cells: &[Cell]) -> Value {
    let mut v = 0u128;
    let mut dep = 0;
    for (i, c) in cells.iter().enumerate() {
        v |= u128::from(c.v) << (8 * i);
        dep |= c.dep;
    }
    if cells.iter().all(|c| c.e.is_none()) {
        return Value { v, e: None, dep };
    }
    if let Some(Expr::Extract(whole, 7, 0)) = cells[0].e.as_deref() {
        let same = whole.width() as usize == 8 * cells.len()
            && cells.iter().enumerate().all(|(i, c)| match c.e.as_deref() {
                Some(Expr::Extract(x, hi, lo)) => {
                    Rc::ptr_eq(x, whole) && *lo as usize == 8 * i && *hi as usize == 8 * i + 7
                }
                _ => false,
            });
        if same {
            return Value {
                v,
                e: Some(Rc::clone(whole)),
                dep,
            };
        }
    }
    let parts = cells
        .iter()
        .rev()
        .map(|c| {
            c.e.clone()
                .unwrap_or_else(|| Rc::new(Expr::Const(u128::from(c.v), 8)))
        })
        .collect::<Vec<_>>();
    let e = if parts.len() == 1 {
        Rc::clone(&parts[0])
    } else {
        Rc::new(Expr::Concat(parts))
    };
    Value { v, e: Some(e), dep }
}

fn split(value: &Value, size: u32) -> Vec<Cell> {
    (0..size)
        .map(|i| Cell {
            v: (value.v >> (8 * i)) as u8,
            e: value.e.as_ref().map(|e| concolic::byte(e, i)),
            dep: value.dep,
        })
        .collect()
}

impl Step<'_, '_> {
    fn in_context(&self, a: u64) -> bool {
        a >= CONTEXT_BASE && a < CONTEXT_BASE + self.context_len
    }

    fn in_stack(a: u64) -> bool {
        a >= STACK_TOP - STACK_SIZE && a < STACK_TOP + 0x100
    }

    fn mem_byte(&self, spc: &Rc<AddrSpace>, a: u64) -> Result<Cell, String> {
        if self.in_context(a) || Self::in_stack(a) {
            return Ok(self
                .ram
                .get(&a)
                .cloned()
                .unwrap_or_else(|| Cell::concrete(0)));
        }
        let image = self
            .env
            .prog
            .read_bytes_at(&Address::new(Rc::clone(spc), a), 1);
        image
            .map(|b| Cell::concrete(b[0]))
            .ok_or_else(|| format!("a read outside the context, the frame and the image at {a:#x}"))
    }

    fn read(&self, vn: &VarnodeData) -> Result<Value, String> {
        let spc = vn.space.as_ref().ok_or("a varnode without a space")?;
        if spc.get_type() == spacetype::IPTR_CONSTANT {
            return Ok(Value {
                v: u128::from(vn.offset) & mask(vn.size),
                e: None,
                dep: 0,
            });
        }
        if spc.is_big_endian() {
            return Err("a big-endian space".into());
        }
        let cells = (0..u64::from(vn.size))
            .map(|i| {
                let off = spc.wrap_offset(vn.offset.wrapping_add(i));
                if spc.get_index() == self.env.register.get_index() {
                    Ok(self
                        .regs
                        .get(&off)
                        .cloned()
                        .unwrap_or_else(|| Cell::concrete(0)))
                } else if spc.get_type() == spacetype::IPTR_INTERNAL {
                    Ok(self
                        .unique
                        .get(&off)
                        .cloned()
                        .unwrap_or_else(|| Cell::concrete(0)))
                } else {
                    self.mem_byte(spc, off)
                }
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(compose(&cells))
    }

    fn write_cells(
        &mut self,
        spc: &Rc<AddrSpace>,
        off: u64,
        cells: Vec<Cell>,
    ) -> Result<(), String> {
        for (i, c) in cells.into_iter().enumerate() {
            let at = spc.wrap_offset(off.wrapping_add(i as u64));
            if spc.get_index() == self.env.register.get_index() {
                self.regs.insert(at, c);
            } else if spc.get_type() == spacetype::IPTR_INTERNAL {
                self.unique.insert(at, c);
            } else if self.in_context(at) || Self::in_stack(at) {
                self.ram.insert(at, c);
            } else {
                return Err(format!(
                    "a write outside the context and the frame at {at:#x}"
                ));
            }
        }
        Ok(())
    }

    fn write(&mut self, vn: &VarnodeData, value: &Value) -> Result<(), String> {
        let spc = Rc::clone(vn.space.as_ref().ok_or("a varnode without a space")?);
        if spc.is_big_endian() {
            return Err("a big-endian space".into());
        }
        self.write_cells(&spc, vn.offset, split(value, vn.size))
    }

    fn address(
        &self,
        space_id: &VarnodeData,
        ptr: &VarnodeData,
    ) -> Result<(Rc<AddrSpace>, u64), String> {
        let spc = space_id
            .get_space_from_const(self.env.prog.arch().manage())
            .ok_or("a bad LOAD/STORE space id")?;
        let p = self.read(ptr)?;
        if p.dep != 0 {
            return Err("an address computed from the input or the context".into());
        }
        let at = spc.wrap_offset(AddrSpace::address_to_byte(p.v as u64, spc.get_word_size()));
        Ok((spc, at))
    }

    /// Run the instruction at `addr`; where it goes next, `None` for the
    /// step's return.
    fn instruction(&mut self, addr: u64) -> Result<Option<u64>, String> {
        let insn = lift(self.env, addr).map_err(|h| h.render())?;
        self.unique.clear();
        let mut i = 0usize;
        while i < insn.ops.len() {
            self.ops += 1;
            if self.ops > BUDGET {
                return Err(format!("more than {BUDGET} p-code ops in one step"));
            }
            let op = &insn.ops[i];
            let arg = |k: usize| {
                op.ins
                    .get(k)
                    .ok_or_else(|| format!("{} lacks input {k}", get_opname(op.opc)))
            };
            match op.opc {
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    let taken = if op.opc == OpCode::CPUI_BRANCH {
                        true
                    } else {
                        let cond = self.read(arg(1)?)?;
                        let taken = cond.v != 0;
                        self.run.signature.push((addr, i as u32, taken));
                        self.run.control_reads |= cond.dep & !INPUT;
                        if cond.dep & INPUT != 0 {
                            match cond.e {
                                Some(e) => self.run.path.branches.push(Branch {
                                    at: addr,
                                    op: i as u32,
                                    cond: e,
                                    taken,
                                }),
                                None => {
                                    self.run.incomplete.get_or_insert_with(|| {
                                        format!("a branch at {addr:#x} depends on the input through an op with no expression")
                                    });
                                }
                            }
                        }
                        taken
                    };
                    if !taken {
                        i += 1;
                        continue;
                    }
                    let t = arg(0)?;
                    if t.space
                        .as_ref()
                        .is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
                    {
                        i = (t.offset as u32).wrapping_add(i as u32) as i32 as usize;
                        continue;
                    }
                    return Ok(Some(t.offset));
                }
                OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => {
                    let t = self.read(arg(0)?)?;
                    if t.dep != 0 {
                        return Err(format!(
                            "a jump at {addr:#x} to an address computed from data"
                        ));
                    }
                    if op.opc == OpCode::CPUI_RETURN || t.v as u64 == RETURN_TO {
                        return Ok(None);
                    }
                    return Ok(Some(t.v as u64));
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    return Err(format!("a call at {addr:#x}"))
                }
                OpCode::CPUI_CALLOTHER => {
                    let name = kuna_verify::program::userop_name(self.env.prog, arg(0)?.offset);
                    if !(op.out.is_none() && NO_EFFECT_USEROPS.contains(&name.as_str())) {
                        return Err(format!("user op {name} at {addr:#x}"));
                    }
                }
                OpCode::CPUI_LOAD => {
                    let (spc, at) = self.address(arg(0)?, arg(1)?)?;
                    let out = op.out.as_ref().ok_or("LOAD without an output")?;
                    let cells = (0..u64::from(out.size))
                        .map(|k| self.mem_byte(&spc, spc.wrap_offset(at.wrapping_add(k))))
                        .collect::<Result<Vec<_>, String>>()?;
                    let v = compose(&cells);
                    self.write(out, &v)?;
                }
                OpCode::CPUI_STORE => {
                    let (spc, at) = self.address(arg(0)?, arg(1)?)?;
                    let val = arg(2)?;
                    let v = self.read(val)?;
                    self.write_cells(&spc, at, split(&v, val.size))?;
                }
                opc => {
                    let out = op
                        .out
                        .as_ref()
                        .ok_or_else(|| format!("{} without an output", get_opname(opc)))?;
                    let mut ins = Inputs::default();
                    let mut sym = Vec::with_capacity(op.ins.len());
                    let mut dep = 0;
                    for vn in &op.ins {
                        let x = self.read(vn)?;
                        ins.push((x.v, vn.size));
                        dep |= x.dep;
                        sym.push((x.e, x.v, vn.size));
                    }
                    let v = eval(opc, out.size, ins.as_slice()).map_err(|h| h.render())?;
                    let e = concolic::op(opc, out.size, &sym);
                    self.write(out, &Value { v, e, dep })?;
                }
            }
            i += 1;
        }
        Ok(Some(addr.wrapping_add(insn.len)))
    }
}

/// Run one step from `context` on `input`. With `symbolic`, the input's bytes
/// are the variables `x0`..`x3` of the run's path, and each value in `pin`
/// (`RESULT`, context bytes) that the input reaches ends the path with a
/// value branch `value == what it was`: code without a branch (a `setcc`, a
/// `cmov`) still splits into one path per value.
pub fn step(
    env: &Env,
    abi: &Abi,
    entry: u64,
    context: &[u8],
    input: u64,
    symbolic: bool,
    pin: u64,
) -> Result<StepRun, String> {
    assert!(
        context.len() <= MAX_CONTEXT,
        "a context of at most {MAX_CONTEXT} bytes"
    );
    let mut s = Step {
        env,
        regs: BTreeMap::new(),
        unique: BTreeMap::new(),
        ram: BTreeMap::new(),
        context_len: context.len() as u64,
        ops: 0,
        run: StepRun {
            result: 0,
            result_dep: 0,
            context: Vec::new(),
            context_dep: Vec::new(),
            path: Path::default(),
            signature: Vec::new(),
            control_reads: 0,
            incomplete: None,
            pinned: 0,
        },
    };
    for (i, b) in context.iter().enumerate() {
        s.ram.insert(
            CONTEXT_BASE + i as u64,
            Cell {
                v: *b,
                e: None,
                dep: 1 << i,
            },
        );
    }
    let konst = |v: u64| Value {
        v: u128::from(v),
        e: None,
        dep: 0,
    };
    let mut sp = STACK_TOP;
    match &abi.link {
        Some(link) => s.write(link, &konst(RETURN_TO))?,
        None => {
            sp -= 8;
            let ram = Rc::clone(&env.ram);
            s.write_cells(&ram, sp, split(&konst(RETURN_TO), 8))?;
        }
    }
    s.write(&abi.sp, &konst(sp))?;
    s.write(&abi.context, &konst(CONTEXT_BASE))?;
    let mut cells = split(&konst(input & 0xffff_ffff), abi.input.size);
    for (i, c) in cells.iter_mut().enumerate().take(INPUT_BYTES) {
        c.dep = INPUT;
        if symbolic {
            let name: Rc<str> = Rc::from(format!("x{i}"));
            s.run.path.inputs.insert(
                Rc::clone(&name),
                Place {
                    space: INPUT_SPACE,
                    off: i as u64,
                    boolean: false,
                },
            );
            c.e = Some(Rc::new(Expr::Var(name)));
        }
    }
    let input_space = Rc::clone(
        abi.input
            .space
            .as_ref()
            .ok_or("the input register has no space")?,
    );
    s.write_cells(&input_space, abi.input.offset, cells)?;

    let mut pc = entry;
    while let Some(next) = s.instruction(pc)? {
        pc = next;
    }
    let result = s.read(&abi.result)?;
    let mut run = s.run;
    run.result = result.v as u64;
    run.result_dep = result.dep;
    let mut values = vec![(
        RESULT,
        result.v,
        result.e.clone(),
        result.dep,
        abi.result.size,
    )];
    for i in 0..context.len() as u64 {
        let c = s
            .ram
            .get(&(CONTEXT_BASE + i))
            .cloned()
            .unwrap_or_else(|| Cell::concrete(0));
        run.context.push(c.v);
        run.context_dep.push(c.dep);
        values.push((1 << i, u128::from(c.v), c.e, c.dep, 1));
    }
    if symbolic {
        for (bit, v, e, dep, size) in values {
            if pin & bit == 0 || dep & INPUT == 0 {
                continue;
            }
            let Some(e) = e else {
                run.incomplete.get_or_insert_with(|| {
                    "a pinned value depends on the input through an op with no expression".into()
                });
                continue;
            };
            let at = PIN_AT | u64::from(bit.trailing_zeros());
            let cond = Rc::new(Expr::Test("=", e, Rc::new(Expr::Const(v, size * 8))));
            run.path.branches.push(Branch {
                at,
                op: v as u32,
                cond,
                taken: true,
            });
            run.signature.push((at, v as u32, true));
            run.pinned |= bit;
        }
    }
    Ok(run)
}
