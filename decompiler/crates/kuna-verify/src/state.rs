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

const STACK_BELOW: u64 = 0x10000;
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
    pub extensions: Vec<Extension>,
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
        let ram = manage
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
            extensions: Vec::new(),
            image: RefCell::new(BTreeMap::new()),
            insns: RefCell::new(BTreeMap::new()),
        })
    }

    pub fn in_stack(&self, addr: u64, size: u64) -> bool {
        addr >= self.sp0.wrapping_sub(STACK_BELOW) && addr.wrapping_add(size) <= self.sp0.wrapping_add(STACK_ABOVE)
    }

    fn is_readonly(&self, addr: u64, size: u64) -> bool {
        let last = addr.wrapping_add(size.saturating_sub(1));
        self.readonly.iter().any(|&(lo, hi)| addr >= lo && last <= hi && last >= addr)
    }

    /// A load is allowed from the stack and from read-only image data.
    pub fn check_load(&self, space: &AddrSpace, addr: u64, size: u32) -> Result<(), Halt> {
        let ram = space.get_index() == self.ram.get_index();
        if (ram && self.in_stack(addr, size as u64)) || self.is_readonly(addr, size as u64) {
            return Ok(());
        }
        Err(Halt::Memory(format!("load {size} bytes at {}:{addr:#x}", space.get_name())))
    }

    /// A store is allowed only into the function's own frame, below the entry
    /// stack pointer.
    pub fn check_store(&self, space: &AddrSpace, addr: u64, size: u32) -> Result<(), Halt> {
        let ram = space.get_index() == self.ram.get_index();
        if ram && self.in_stack(addr, size as u64) && addr.wrapping_add(size as u64) <= self.sp0 {
            return Ok(());
        }
        Err(Halt::Memory(format!("store {size} bytes at {}:{addr:#x}", space.get_name())))
    }

    pub fn code_mapped(&self, addr: u64) -> bool {
        self.prog.entry_bytes_mapped(&Address::new(Rc::clone(&self.code), addr))
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

    pub fn initial_byte(&self, run: u32, space: &AddrSpace, off: u64) -> u8 {
        let idx = space.get_index();
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

/// One side's view: the run's initial state plus that side's writes.
pub struct State<'e, 'p> {
    pub env: &'e Env<'p>,
    pub run: u32,
    written: BTreeMap<(i32, u64), u8>,
}

impl<'e, 'p> State<'e, 'p> {
    pub fn new(env: &'e Env<'p>, run: u32) -> Self {
        State { env, run, written: BTreeMap::new() }
    }

    fn byte(&self, space: &AddrSpace, off: u64) -> u8 {
        match self.written.get(&(space.get_index(), off)) {
            Some(&b) => b,
            None => self.env.initial_byte(self.run, space, off),
        }
    }

    fn read_wide(&self, space: &AddrSpace, off: u64, size: u32) -> Result<u128, Halt> {
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

    /// Read a processor or temporary space value.
    pub fn read(&self, space: &AddrSpace, off: u64, size: u32) -> Result<u128, Halt> {
        self.read_wide(space, off, size)
    }

    pub fn write(&mut self, space: &AddrSpace, off: u64, size: u32, val: u128) -> Result<(), Halt> {
        if size > MAX_SIZE {
            return Err(Halt::Unsupported(format!("{size}-byte value")));
        }
        for i in 0..u64::from(size) {
            let shift = if space.is_big_endian() { u64::from(size) - 1 - i } else { i };
            let b = (val >> (shift * 8)) as u8;
            self.written.insert((space.get_index(), space.wrap_offset(off.wrapping_add(i))), b);
        }
        Ok(())
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
                let rec = self
                    .env
                    .prog
                    .arch()
                    .manage()
                    .find_join(off)
                    .map_err(|e| Halt::Harness(format!("join: {}", e.explain())))?;
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

    /// Whether every byte of the range was written by this side.
    pub fn wrote(&self, space: &AddrSpace, off: u64, size: u32) -> bool {
        (0..u64::from(size)).all(|i| {
            self.written
                .contains_key(&(space.get_index(), space.wrap_offset(off.wrapping_add(i))))
        })
    }

    /// Read any storage a varnode or prototype can name, up to sixteen bytes:
    /// constants, the stack space (relative to the entry stack pointer), joined
    /// registers (most significant piece first), and processor spaces.
    pub fn read_storage(&self, space: &Rc<AddrSpace>, off: u64, size: u32) -> Result<u128, Halt> {
        let m = mask(size);
        match space.get_type() {
            spacetype::IPTR_CONSTANT => Ok(u128::from(off) & m),
            spacetype::IPTR_SPACEBASE => {
                let addr = self.env.ram.wrap_offset(self.env.sp0.wrapping_add(off));
                self.read_wide(&self.env.ram, addr, size)
            }
            spacetype::IPTR_JOIN => {
                let rec = self
                    .env
                    .prog
                    .arch()
                    .manage()
                    .find_join(off)
                    .map_err(|e| Halt::Harness(format!("join: {}", e.explain())))?;
                let mut acc = 0u128;
                for i in 0..rec.num_pieces() {
                    let p = rec.get_piece(i);
                    let ps = p.space.as_ref().ok_or_else(|| Halt::Harness("join piece without space".into()))?;
                    let v = self.read_storage(ps, p.offset, p.size)?;
                    acc = if p.size >= 16 { v } else { (acc << (p.size * 8)) | v };
                }
                Ok(acc & m)
            }
            spacetype::IPTR_PROCESSOR | spacetype::IPTR_INTERNAL => self.read_wide(space, off, size),
            other => Err(Halt::Harness(format!("storage in a {other:?} space"))),
        }
    }
}
