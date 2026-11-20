//! The raw machine state both sides start from.
//!
//! A run's initial state is a pure function of (seed, run, space, offset), so
//! the machine side and the decompiled side read the same register, stack and
//! memory values without either copying from the other. Registers and stack get
//! random words, the first runs and one word in eight are edge values, the stack
//! pointer is fixed, and image-backed memory reads the load image. Each side then
//! keeps its own writes on top.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_console::engine::ConsoleProgram;

use crate::value::{mask, Halt, MAX_SIZE};

/// Runs whose every register word is an edge value.
pub const EDGE_RUNS: u32 = 16;

/// The stack below the entry stack pointer: room for a large frame.
const STACK_BELOW: u64 = 0x80_0000;

/// How far past an escaped pointer a call may write.
const ESCAPE_WINDOW: u64 = 0x1000;
const STACK_ABOVE: u64 = 0x1000;

/// An argument register the ABI promises the caller extended: bytes of
/// `container` outside the parameter repeat its sign, or are zero.
#[derive(Debug, Clone)]
pub struct Extension {
    pub space: i32,
    pub container: (u64, u32),
    pub param: (u64, u32),
    pub signed: bool,
}

/// What a run needs to know about the program: its spaces, the stack layout,
/// the image, and the argument extensions the calling convention promises.
pub struct Env<'p> {
    pub prog: &'p ConsoleProgram,
    pub seed: u64,
    pub register: Rc<AddrSpace>,
    pub ram: Rc<AddrSpace>,
    pub code: Rc<AddrSpace>,
    pub unique: Option<Rc<AddrSpace>>,
    /// The stack pointer's register storage.
    pub sp: (u64, u32),
    /// The stack pointer's value at entry.
    pub sp0: u64,
    granule: u32,
    edges: Vec<u64>,
    readonly: Vec<(u64, u64)>,
    /// The image's code sections, `(start, size)`; empty without a section table.
    code_sections: Vec<(u64, u64)>,
    pub extensions: Vec<Extension>,
    /// Storage the context or the loader gives a known value at the
    /// function's entry (a PowerPC TOC, a MIPS t9, AVR's zero register, the
    /// return address): space, offset, size, value.
    pub tracked: Vec<(i32, u64, u32, u64)>,
    /// The return address the function starts with, when the model names
    /// where it is: a RETURN to it leaves the function.
    pub ret_addr: Option<u64>,
    /// Register parameters of type `bool`: a caller passes 0 or 1.
    pub booleans: Vec<(i32, u64, u32)>,
    image: RefCell<BTreeMap<u64, Option<u8>>>,
    pub(crate) insns: RefCell<BTreeMap<u64, Rc<crate::machine::Insn>>>,
}

fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn edge_values(granule: u32) -> Vec<u64> {
    let bits = granule * 8;
    let m = mask(granule) as u64;
    let mut v = vec![
        0,
        1,
        2,
        m,
        1u64 << (bits - 1),
        m >> 1,
        0x7f,
        0x80,
        0xff,
        0x7fff,
        0x8000,
        0xffff,
        0x1_0000,
        0x7fff_ffff,
        0x8000_0000,
        0xffff_ffff,
        7,
        8,
        15,
        16,
        31,
        32,
        63,
        64,
    ];
    for x in &mut v {
        *x &= m;
    }
    v.sort_unstable();
    v.dedup();
    v
}

impl<'p> Env<'p> {
    pub fn new(prog: &'p ConsoleProgram, seed: u64) -> Result<Self, Halt> {
        let manage = prog.arch().manage();
        let register = manage
            .get_space_by_name("register")
            .cloned()
            .ok_or_else(|| Halt::Harness("no register space".into()))?;
        let data = manage
            .get_default_data_space()
            .cloned()
            .ok_or_else(|| Halt::Harness("no default data space".into()))?;
        let code = manage
            .get_default_code_space()
            .cloned()
            .ok_or_else(|| Halt::Harness("no default code space".into()))?;
        let unique = manage.get_unique_space().cloned();
        let stack = manage
            .get_stack_space()
            .ok_or_else(|| Halt::Harness("no stack space".into()))?;
        let base = stack
            .get_spacebase(0)
            .map_err(|e| Halt::Harness(format!("stack pointer: {}", e.explain())))?;
        let ram = stack.get_contain().cloned().unwrap_or(data);
        let ram_mask = ram.get_highest();
        let mut sp0 = (ram_mask >> 1) & !0xffff;
        if sp0 == 0 {
            sp0 = (ram_mask >> 1) & !0xff;
        }
        let granule = ram.get_addr_size().clamp(1, 8);
        let mut ranges = RangeList::new();
        prog.arch().translate().loader_rc().borrow().get_readonly(&mut ranges);
        let readonly = ranges
            .iter()
            .filter(|r| r.get_space().get_index() == ram.get_index() || r.get_space().get_index() == code.get_index())
            .map(|r| (r.get_first(), r.get_last()))
            .collect();
        let code_sections = prog
            .sections()
            .into_iter()
            .filter(|&(_, _, flags)| flags & kuna_sleigh::loadimage::section_flags::CODE != 0)
            .map(|(start, size, _)| (start, size))
            .collect();
        Ok(Env {
            prog,
            seed,
            register,
            ram,
            code,
            unique,
            sp: (base.offset, base.size),
            sp0,
            granule,
            edges: edge_values(granule),
            readonly,
            code_sections,
            extensions: Vec::new(),
            tracked: Vec::new(),
            ret_addr: None,
            booleans: Vec::new(),
            image: RefCell::new(BTreeMap::new()),
            insns: RefCell::new(BTreeMap::new()),
        })
    }

    pub fn in_stack(&self, addr: u64, size: u64) -> bool {
        let below = STACK_BELOW.min(self.sp0 / 2);
        addr >= self.sp0.wrapping_sub(below) && addr.wrapping_add(size) <= self.sp0.wrapping_add(STACK_ABOVE)
    }

    fn is_readonly(&self, addr: u64, size: u64) -> bool {
        let last = addr.wrapping_add(size.saturating_sub(1));
        self.readonly.iter().any(|&(lo, hi)| addr >= lo && last <= hi && last >= addr)
    }

    pub fn code_mapped(&self, addr: u64) -> bool {
        self.prog.entry_bytes_mapped(&Address::new(Rc::clone(&self.code), addr))
    }

    /// Whether `addr` is mapped code: in a section the loader marks as code,
    /// or anywhere mapped in an image without a section table.
    pub fn is_code(&self, addr: u64) -> bool {
        self.code_mapped(addr)
            && (self.code_sections.is_empty() || self.code_sections.iter().any(|&(start, size)| addr >= start && addr - start < size))
    }

    fn image_byte(&self, addr: u64) -> Option<u8> {
        if let Some(b) = self.image.borrow().get(&addr) {
            return *b;
        }
        let b = self
            .prog
            .read_bytes_at(&Address::new(Rc::clone(&self.ram), addr), 1)
            .map(|v| v[0]);
        self.image.borrow_mut().insert(addr, b);
        b
    }

    fn word(&self, run: u32, space: i32, index: u64) -> u64 {
        let h = splitmix(
            self.seed
                ^ u64::from(run).wrapping_mul(0xd6e8_feb8_6659_fd93)
                ^ ((space as u64) << 56)
                ^ index.wrapping_mul(0xc2b2_ae3d_27d4_eb4f),
        );
        let v = if run < EDGE_RUNS || h.is_multiple_of(8) {
            self.edges[(h >> 8) as usize % self.edges.len()]
        } else {
            splitmix(h)
        };
        v & mask(self.granule) as u64
    }

    fn byte_of(word: u64, pos: u64, width: u64, big: bool) -> u8 {
        let shift = if big { width - 1 - pos } else { pos };
        (word >> (shift * 8)) as u8
    }

    fn random_byte(&self, run: u32, space: &AddrSpace, off: u64) -> u8 {
        let g = u64::from(self.granule);
        Self::byte_of(self.word(run, space.get_index(), off / g), off % g, g, space.is_big_endian())
    }

    fn extended_byte(&self, run: u32, space: &AddrSpace, off: u64) -> Option<u8> {
        let x = self.extensions.iter().find(|x| {
            let (c, cs) = x.container;
            let (p, ps) = x.param;
            x.space == space.get_index() && off >= c && off < c + u64::from(cs) && !(off >= p && off < p + u64::from(ps))
        })?;
        let (p, ps) = x.param;
        let mut v = 0u128;
        for i in 0..u64::from(ps) {
            let b = u128::from(self.random_byte(run, space, p + i));
            v = if space.is_big_endian() { (v << 8) | b } else { v | (b << (8 * i)) };
        }
        let negative = x.signed && (v >> (ps * 8 - 1)) & 1 == 1;
        let ext = if negative { v | !mask(ps) } else { v };
        let (c, cs) = x.container;
        let pos = off - c;
        let shift = if space.is_big_endian() { u64::from(cs) - 1 - pos } else { pos };
        Some((ext >> (shift * 8)) as u8)
    }

    /// A byte of writable memory after the `epoch`-th call: the callee may
    /// have written any of it.
    fn clobbered_byte(&self, run: u32, epoch: u32, space: &AddrSpace, off: u64) -> u8 {
        let h = splitmix(
            self.seed
                ^ u64::from(run).wrapping_mul(0xd6e8_feb8_6659_fd93)
                ^ u64::from(epoch).wrapping_mul(0x9e37_79b9_7f4a_7c15)
                ^ ((space.get_index() as u64) << 56)
                ^ off.wrapping_mul(0xc2b2_ae3d_27d4_eb4f),
        );
        h as u8
    }

    fn is_memory(&self, space: &AddrSpace) -> bool {
        space.get_index() != self.register.get_index()
            && !self.unique.as_ref().is_some_and(|u| u.get_index() == space.get_index())
    }

    /// Memory either side's caller can observe after the function: anything
    /// but registers, temporaries and the stack.
    pub fn observable(&self, space: &AddrSpace, off: u64) -> bool {
        self.is_memory(space) && !(space.get_index() == self.ram.get_index() && self.in_stack(off, 1))
    }

    /// Memory a call can write: the image's writable data, and the bytes near
    /// a pointer that escaped into an earlier call, never read-only data. This
    /// is no more than kuna itself treats as changed by a call, so a value it
    /// keeps across one stays equal unless kuna is wrong to keep it.
    pub fn callee_writable(&self, escaped: &[u64], space: &AddrSpace, off: u64) -> bool {
        if !self.is_memory(space) || self.is_readonly(off, 1) {
            return false;
        }
        escaped.iter().any(|&p| off.wrapping_sub(p) < ESCAPE_WINDOW)
            || (space.get_index() == self.ram.get_index() && !self.in_stack(off, 1) && self.image_byte(off).is_some())
    }

    /// The byte at `off` with no write by a side since its last call: the
    /// initial state before any call, what the calls left after one.
    pub fn base_byte(&self, run: u32, epoch: u32, escaped: &[u64], space: &AddrSpace, off: u64) -> u8 {
        if epoch > 0 && self.callee_writable(escaped, space, off) {
            self.clobbered_byte(run, epoch, space, off)
        } else {
            self.initial_byte(run, space, off)
        }
    }

    /// The value a call leaves in a register byte it kills.
    pub fn killed_byte(&self, run: u32, call: u32, off: u64) -> u8 {
        self.clobbered_byte(run, call + 1, &self.register, off)
    }

    pub fn initial_byte(&self, run: u32, space: &AddrSpace, off: u64) -> u8 {
        let idx = space.get_index();
        if let Some(&(_, t, size, val)) =
            self.tracked.iter().find(|&&(sp, t, size, _)| sp == idx && off >= t && off < t + u64::from(size))
        {
            return Self::byte_of(val, off - t, u64::from(size), space.is_big_endian());
        }
        if let Some(&(_, b, size)) =
            self.booleans.iter().find(|&&(sp, b, size)| sp == idx && off >= b && off < b + u64::from(size))
        {
            let low = if space.is_big_endian() { b + u64::from(size) - 1 } else { b };
            return if off == low { self.random_byte(run, space, off) & 1 } else { 0 };
        }
        if self.unique.as_ref().is_some_and(|u| u.get_index() == idx) {
            return 0;
        }
        if idx == self.register.get_index() {
            let (sp_off, sp_size) = self.sp;
            if off >= sp_off && off < sp_off + u64::from(sp_size) {
                return Self::byte_of(self.sp0, off - sp_off, u64::from(sp_size), space.is_big_endian());
            }
            if let Some(b) = self.extended_byte(run, space, off) {
                return b;
            }
        } else if idx == self.ram.get_index() && !self.in_stack(off, 1) {
            if let Some(b) = self.image_byte(off) {
                return b;
            }
        }
        self.random_byte(run, space, off)
    }
}

/// Register offsets past this go through the sparse map.
const DENSE_REGISTERS: u64 = 1 << 20;

const PRESENT: u8 = 1;
const WRITTEN: u8 = 2;

/// One side's view: the run's initial state plus that side's writes.
///
/// Registers live in a dense file that memoizes their initial bytes on first
/// read; temporaries and memory keep only the bytes this side wrote.
#[derive(Clone)]
pub struct State<'e, 'p> {
    pub env: &'e Env<'p>,
    pub run: u32,
    regs: Vec<u8>,
    flags: Vec<u8>,
    unique: BTreeMap<u64, u8>,
    mem: BTreeMap<(i32, u64), u8>,
    /// Calls made so far.
    pub epoch: u32,
    /// Pointer-sized values passed to calls so far.
    pub escaped: Vec<u64>,
    /// While a callee runs for real: what it does to memory, in order.
    pub log: Option<Vec<Event>>,
    /// This side read a byte of the function's own frame that nothing wrote:
    /// an uninitialized local, whose value C does not define.
    pub read_uninitialized: bool,
    /// Stack bytes that hold a callee-saved register's entry value, by the
    /// register offset they came from: the prologue's spills.
    saved: BTreeMap<u64, u64>,
    /// This side read a spilled entry value back into anything but the
    /// register it came from: what C sees there is uninitialized too.
    pub read_saved: bool,
}

/// One thing a callee run for real did to memory.
#[derive(Clone, Debug)]
pub enum Event {
    Write(i32, u64, u8),
    /// A call it made in turn, with the killed registers and its arguments.
    Call(Vec<(u64, u32)>, Vec<u128>),
}

/// What a callee in the image did when the machine ran it, for the decompiled
/// side to replay: its memory events, and the bytes it left in the registers
/// the call kills.
#[derive(Clone, Debug, Default)]
pub struct Effect {
    pub events: Vec<Event>,
    pub regs: Vec<(u64, u8)>,
}

/// The memory a side wrote outside the stack since its last call, and how many
/// calls preceded it: what a callee, or the caller after the return, can read.
#[derive(Clone, Debug, Default)]
pub struct Writes {
    pub bytes: BTreeMap<(i32, u64), u8>,
    pub epoch: u32,
    pub escaped: Vec<u64>,
}

impl<'e, 'p> State<'e, 'p> {
    pub fn new(env: &'e Env<'p>, run: u32) -> Self {
        State {
            env,
            run,
            regs: Vec::new(),
            flags: Vec::new(),
            unique: BTreeMap::new(),
            mem: BTreeMap::new(),
            epoch: 0,
            escaped: Vec::new(),
            log: None,
            read_uninitialized: false,
            saved: BTreeMap::new(),
            read_saved: false,
        }
    }

    /// Mark a stack store as the spill of a callee-saved register's entry value.
    pub fn mark_saved(&mut self, at: u64, size: u32, reg: u64) {
        for i in 0..u64::from(size) {
            self.saved.insert(at.wrapping_add(i), reg);
        }
    }

    /// The callee-saved registers whose spilled entry values the range holds.
    pub fn saved_in(&self, at: u64, size: u32) -> Vec<u64> {
        if self.saved.is_empty() || size == 0 {
            return Vec::new();
        }
        let mut regs: Vec<u64> = self.saved.range(at..=at.saturating_add(u64::from(size) - 1)).map(|(_, &r)| r).collect();
        regs.dedup();
        regs
    }

    /// Forget what this side wrote to the stack below `sp`: after a call
    /// returns, what is there (its return address, its frame) is no local.
    pub fn forget_below(&mut self, sp: u64) {
        let ram = self.env.ram.get_index();
        let below = STACK_BELOW.min(self.env.sp0 / 2);
        let lo = self.env.sp0.wrapping_sub(below);
        if sp <= lo {
            return;
        }
        let dead: Vec<(i32, u64)> = self.mem.range((ram, lo)..(ram, sp)).map(|(k, _)| *k).collect();
        for k in dead {
            self.mem.remove(&k);
        }
        let dead: Vec<u64> = self.saved.range(lo..sp).map(|(k, _)| *k).collect();
        for k in dead {
            self.saved.remove(&k);
        }
    }

    fn is_register(&self, space: &AddrSpace, off: u64) -> bool {
        space.get_index() == self.env.register.get_index() && off < DENSE_REGISTERS
    }

    fn is_unique(&self, space: &AddrSpace) -> bool {
        self.env.unique.as_ref().is_some_and(|u| u.get_index() == space.get_index())
    }

    fn grow(&mut self, end: usize) {
        if self.regs.len() < end {
            self.regs.resize(end, 0);
            self.flags.resize(end, 0);
        }
    }

    fn byte(&mut self, space: &AddrSpace, off: u64) -> u8 {
        if self.is_register(space, off) {
            let i = off as usize;
            self.grow(i + 1);
            if self.flags[i] & PRESENT == 0 {
                self.regs[i] = self.env.initial_byte(self.run, space, off);
                self.flags[i] |= PRESENT;
            }
            return self.regs[i];
        }
        let found = if self.is_unique(space) {
            self.unique.get(&off).copied()
        } else {
            self.mem.get(&(space.get_index(), off)).copied()
        };
        if found.is_none()
            && space.get_index() == self.env.ram.get_index()
            && off < self.env.sp0
            && self.env.in_stack(off, 1)
        {
            self.read_uninitialized = true;
        }
        found.unwrap_or_else(|| self.env.base_byte(self.run, self.epoch, &self.escaped, space, off))
    }

    fn put(&mut self, space: &AddrSpace, off: u64, b: u8) {
        if self.is_register(space, off) {
            let i = off as usize;
            self.grow(i + 1);
            self.regs[i] = b;
            self.flags[i] |= PRESENT | WRITTEN;
        } else if self.is_unique(space) {
            self.unique.insert(off, b);
        } else {
            if let Some(log) = self.log.as_mut() {
                log.push(Event::Write(space.get_index(), off, b));
            }
            self.mem.insert((space.get_index(), off), b);
        }
    }

    /// Do what a callee did when the machine ran it.
    pub fn replay(&mut self, effect: &Effect) {
        let manage = self.env.prog.arch().manage();
        for ev in &effect.events {
            match ev {
                Event::Write(sp, off, b) => {
                    if let Some(space) = manage.get_space(*sp).cloned() {
                        self.put(&space, *off, *b);
                    }
                }
                Event::Call(killed, args) => self.call(killed, args),
            }
        }
        let register = Rc::clone(&self.env.register);
        for &(off, b) in &effect.regs {
            self.put(&register, off, b);
        }
    }

    /// The bytes now in the given register ranges.
    pub fn register_bytes(&mut self, ranges: &[(u64, u32)]) -> Vec<(u64, u8)> {
        let register = Rc::clone(&self.env.register);
        ranges
            .iter()
            .flat_map(|&(off, size)| (0..u64::from(size)).map(move |i| off + i))
            .map(|o| (o, self.byte(&register, o)))
            .collect()
    }

    /// Put back every register byte `before` held, except those in `keep`:
    /// a callee run for real leaves the registers the calling convention says
    /// it preserves as they were. Returns the offsets the callee had changed.
    pub fn preserve_registers(&mut self, before: &State, keep: &[(u64, u32)]) -> Vec<u64> {
        let kept = |o: u64| keep.iter().any(|&(off, size)| o >= off && o < off + u64::from(size));
        let mut changed = Vec::new();
        let n = self.regs.len().max(before.regs.len());
        self.grow(n);
        for i in 0..n {
            let o = i as u64;
            if kept(o) {
                continue;
            }
            let (b, f) = (before.regs.get(i).copied().unwrap_or(0), before.flags.get(i).copied().unwrap_or(0));
            let present_now = self.flags[i] & PRESENT != 0;
            if present_now && f & PRESENT != 0 && self.regs[i] != b {
                changed.push(o);
            }
            if present_now || f & PRESENT != 0 {
                self.regs[i] = b;
                self.flags[i] = f;
            }
        }
        changed
    }

    /// What this side wrote outside the stack since its last call.
    pub fn writes(&self) -> Writes {
        let bytes = self
            .mem
            .iter()
            .filter(|((sp, off), _)| self.env.prog.arch().manage().get_space(*sp).is_some_and(|s| self.env.observable(s, *off)))
            .map(|(k, v)| (*k, *v))
            .collect();
        Writes { bytes, epoch: self.epoch, escaped: self.escaped.clone() }
    }

    /// A call's effect: it kills the registers in `killed`, `args` escape into
    /// it, and it may write any memory it can reach. The memory is written
    /// lazily: every byte this side did not write since reads what
    /// [`Env::base_byte`] says for the new epoch.
    pub fn call(&mut self, killed: &[(u64, u32)], args: &[u128]) {
        if let Some(log) = self.log.as_mut() {
            log.push(Event::Call(killed.to_vec(), args.to_vec()));
        }
        let call = self.epoch;
        let register = Rc::clone(&self.env.register);
        for &(off, size) in killed {
            for i in 0..u64::from(size) {
                let b = self.env.killed_byte(self.run, call, off + i);
                self.put(&register, off + i, b);
            }
        }
        let size = u64::from(self.env.ram.get_addr_size()).min(8);
        let ptr_mask = if size >= 8 { u64::MAX } else { (1u64 << (size * 8)) - 1 };
        self.escaped.extend(args.iter().map(|&a| a as u64 & ptr_mask).filter(|&p| p > 0xffff));
        self.escaped.sort_unstable();
        self.escaped.dedup();
        let env = self.env;
        let escaped = std::mem::take(&mut self.escaped);
        let manage = env.prog.arch().manage();
        self.mem.retain(|(sp, off), _| manage.get_space(*sp).is_some_and(|s| !env.callee_writable(&escaped, s, *off)));
        self.escaped = escaped;
        self.epoch += 1;
    }

    /// Forget the temporaries: they live for one instruction.
    pub fn end_instruction(&mut self) {
        self.unique.clear();
    }

    /// Read a processor or temporary space value of up to sixteen bytes.
    pub fn read(&mut self, space: &AddrSpace, off: u64, size: u32) -> Result<u128, Halt> {
        if size > MAX_SIZE {
            return Err(Halt::Unsupported(format!("{size}-byte value")));
        }
        let mut v = 0u128;
        for i in 0..u64::from(size) {
            let b = self.byte(space, space.wrap_offset(off.wrapping_add(i)));
            if space.is_big_endian() {
                v = (v << 8) | u128::from(b);
            } else {
                v |= u128::from(b) << (8 * i);
            }
        }
        Ok(v)
    }

    pub fn write(&mut self, space: &AddrSpace, off: u64, size: u32, val: u128) -> Result<(), Halt> {
        if !self.saved.is_empty() && space.get_index() == self.env.ram.get_index() {
            let dead: Vec<u64> = self.saved.range(off..=off.saturating_add(u64::from(size).max(1) - 1)).map(|(k, _)| *k).collect();
            for k in dead {
                self.saved.remove(&k);
            }
        }
        if size > MAX_SIZE {
            return Err(Halt::Unsupported(format!("{size}-byte value")));
        }
        for i in 0..u64::from(size) {
            let shift = if space.is_big_endian() { u64::from(size) - 1 - i } else { i };
            self.put(space, space.wrap_offset(off.wrapping_add(i)), (val >> (shift * 8)) as u8);
        }
        Ok(())
    }

    /// Whether every byte of the range was written by this side.
    pub fn wrote(&self, space: &AddrSpace, off: u64, size: u32) -> bool {
        (0..u64::from(size)).all(|i| {
            let at = space.wrap_offset(off.wrapping_add(i));
            if self.is_register(space, at) {
                self.flags.get(at as usize).is_some_and(|f| f & WRITTEN != 0)
            } else if self.is_unique(space) {
                self.unique.contains_key(&at)
            } else {
                self.mem.contains_key(&(space.get_index(), at))
            }
        })
    }

    fn join(&self, off: u64) -> Result<Rc<kuna_base::space::JoinRecord>, Halt> {
        self.env
            .prog
            .arch()
            .manage()
            .find_join(off)
            .map_err(|e| Halt::Harness(format!("join: {}", e.explain())))
    }

    /// Read any storage a varnode or prototype can name, up to sixteen bytes:
    /// constants, the stack space (relative to the entry stack pointer), joined
    /// registers (most significant piece first), and processor spaces.
    pub fn read_storage(&mut self, space: &Rc<AddrSpace>, off: u64, size: u32) -> Result<u128, Halt> {
        let m = mask(size);
        match space.get_type() {
            spacetype::IPTR_CONSTANT => Ok(u128::from(off) & m),
            spacetype::IPTR_SPACEBASE => {
                let ram = Rc::clone(&self.env.ram);
                let addr = ram.wrap_offset(self.env.sp0.wrapping_add(off));
                self.read(&ram, addr, size)
            }
            spacetype::IPTR_JOIN => {
                let rec = self.join(off)?;
                let mut acc = 0u128;
                for i in 0..rec.num_pieces() {
                    let p = rec.get_piece(i);
                    let ps = p.space.clone().ok_or_else(|| Halt::Harness("join piece without space".into()))?;
                    let v = self.read_storage(&ps, p.offset, p.size)?;
                    acc = if p.size >= 16 { v } else { (acc << (p.size * 8)) | v };
                }
                Ok(acc & m)
            }
            spacetype::IPTR_PROCESSOR | spacetype::IPTR_INTERNAL => self.read(space, off, size),
            other => Err(Halt::Harness(format!("storage in a {other:?} space"))),
        }
    }

    /// Write storage a varnode can name: the inverse of [`Self::read_storage`].
    pub fn write_storage(&mut self, space: &Rc<AddrSpace>, off: u64, size: u32, val: u128) -> Result<(), Halt> {
        match space.get_type() {
            spacetype::IPTR_SPACEBASE => {
                let ram = Rc::clone(&self.env.ram);
                let addr = ram.wrap_offset(self.env.sp0.wrapping_add(off));
                self.write(&ram, addr, size, val)
            }
            spacetype::IPTR_JOIN => {
                let rec = self.join(off)?;
                let mut rest = val;
                for i in (0..rec.num_pieces()).rev() {
                    let p = rec.get_piece(i);
                    let ps = p.space.clone().ok_or_else(|| Halt::Harness("join piece without space".into()))?;
                    self.write_storage(&ps, p.offset, p.size, rest & mask(p.size))?;
                    rest = if p.size >= 16 { 0 } else { rest >> (p.size * 8) };
                }
                Ok(())
            }
            spacetype::IPTR_PROCESSOR | spacetype::IPTR_INTERNAL => self.write(space, off, size, val),
            other => Err(Halt::Harness(format!("storage in a {other:?} space"))),
        }
    }
}
