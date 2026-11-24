//! The raw machine state both sides start from.
//!
//! A run's initial state is a pure function of (seed, run, space, offset), so
//! the machine side and the decompiled side read the same register, stack and
//! memory values without either copying from the other. Registers and stack get
//! random words, the first runs and one word in eight are edge values, the stack
//! pointer is fixed, and image-backed memory reads the load image. Each side then
//! keeps its own writes on top.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::BuildHasherDefault;
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

/// A register the ABI promises is extended: an argument the caller passes, or
/// a value a callee returns. Bytes of `container` outside the value repeat its
/// sign, or are zero.
#[derive(Debug, Clone)]
pub struct Extension {
    pub space: i32,
    pub container: (u64, u32),
    pub param: (u64, u32),
    pub signed: bool,
    /// A `bool` return: the callee leaves 0 or 1 in `param`.
    pub boolean: bool,
}

/// Pointer-sized values passed to calls so far, and the bytes a callee
/// given them can reach, as sorted, disjoint `[first, last]` ranges.
#[derive(Clone, Debug, Default)]
pub struct Escaped {
    pointers: Vec<u64>,
    reach: Vec<(u64, u64)>,
}

impl Escaped {
    pub fn add(&mut self, env: &Env, pointers: impl Iterator<Item = u64>) {
        let before = self.pointers.len();
        self.pointers.extend(pointers);
        self.pointers.sort_unstable();
        self.pointers.dedup();
        if self.pointers.len() == before {
            return;
        }
        let mut reach = Vec::with_capacity(self.pointers.len());
        for &p in &self.pointers {
            env.reach_of(p, &mut reach);
        }
        self.reach = merge_ranges(reach);
    }

    pub fn reaches(&self, off: u64) -> bool {
        let i = self.reach.partition_point(|r| r.0 <= off);
        i > 0 && self.reach[i - 1].1 >= off
    }
}

/// The trap a store into read-only memory or code raises.
pub const READONLY_STORE: &str = "store to read-only memory";

/// `[first, last]` ranges sorted, with overlapping and adjacent ones joined.
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable();
    let mut out: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    for (first, last) in ranges {
        match out.last_mut() {
            Some(prev) if first <= prev.1.saturating_add(1) => prev.1 = prev.1.max(last),
            _ => out.push((first, last)),
        }
    }
    out
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
    /// The image's code sections, `[first, last]`, merged; empty without a
    /// section table.
    code_sections: Vec<(u64, u64)>,
    pub extensions: Vec<Extension>,
    /// Storage the context or the loader gives a known value at the
    /// function's entry (a PowerPC TOC, a MIPS t9, AVR's zero register, the
    /// return address): space, offset, size, value.
    pub tracked: Vec<(i32, u64, u32, u64)>,
    /// The return address the function starts with, when the model names
    /// where it is: a RETURN to it leaves the function.
    pub ret_addr: Option<u64>,
    /// Bytes a call pushes for its return address on the stack (0 when the
    /// model keeps it in a register).
    pub ret_slot: u32,
    /// Register parameters of type `bool`: a caller passes 0 or 1.
    pub booleans: Vec<(i32, u64, u32)>,
    /// The machine's system calls write the kernel's result registers.
    pub kernel: Cell<bool>,
    /// C5: input bytes a z3 model sets for this run, over the random state.
    pub concolic_inputs: RefCell<Option<crate::concolic::Inputs>>,
    /// C5: the machine records its path on this run.
    pub record_path: Cell<bool>,
    /// CSIC: the machine side's values at kuna's outputs, when observed.
    pub probe: RefCell<Option<crate::intervene::Probe>>,
    /// ARM and AArch64 divide by zero to 0 instead of trapping.
    pub divide_by_zero_is_zero: bool,
    /// Image pages by base address; `None` for a page not mapped whole.
    image: RefCell<HashMap<u64, Option<Box<[u8]>>, BuildHasherDefault<Fast>>>,
    /// The loader's segments and sections, `[first, last]`: no image byte lies
    /// outside them. `None` when the loader lists no segments.
    mapped: Option<Vec<(u64, u64)>>,
    /// Bytes of the pages not mapped whole.
    image_bytes: RefCell<HashMap<u64, Option<u8>, BuildHasherDefault<Fast>>>,
    /// Where a call to each target goes once thunks are followed.
    pub(crate) thunks: RefCell<HashMap<u64, u64>>,
    pub(crate) insns: RefCell<HashMap<u64, Rc<crate::machine::Insn>, BuildHasherDefault<Fast>>>,
    entries: std::cell::OnceCell<BTreeSet<u64>>,
    stack: Rc<AddrSpace>,
    /// The decompiled function, whose local symbols give the extent of a
    /// stack object whose address escapes.
    pub fd: Option<&'p kuna_decomp::funcdata::Funcdata>,
    locals: RefCell<BTreeMap<u64, Option<(u64, u64)>>>,
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
            .collect::<Vec<_>>();
        let readonly = merge_ranges(readonly);
        let segments = prog.segments();
        let mapped = (!segments.is_empty()).then(|| {
            let spans = segments.into_iter().chain(prog.sections()).filter(|&(_, size, _)| size > 0);
            merge_ranges(spans.map(|(start, size, _)| (start, start.saturating_add(size - 1))).collect())
        });
        let code_sections = prog
            .sections()
            .into_iter()
            .filter(|&(_, size, flags)| size > 0 && flags & kuna_sleigh::loadimage::section_flags::CODE != 0)
            .map(|(start, size, _)| (start, start.saturating_add(size - 1)))
            .collect();
        let code_sections = merge_ranges(code_sections);
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
            ret_slot: 0,
            booleans: Vec::new(),
            kernel: Cell::new(true),
            concolic_inputs: RefCell::new(None),
            record_path: Cell::new(false),
            probe: RefCell::new(None),
            divide_by_zero_is_zero: ["ARM", "AARCH64"].iter().any(|a| prog.arch().archid.starts_with(a)),
            image: RefCell::new(HashMap::default()),
            mapped,
            image_bytes: RefCell::new(HashMap::default()),
            thunks: RefCell::new(HashMap::new()),
            insns: RefCell::new(HashMap::default()),
            entries: std::cell::OnceCell::new(),
            stack: Rc::clone(stack),
            fd: None,
            locals: RefCell::new(BTreeMap::new()),
        })
    }

    pub fn in_stack(&self, addr: u64, size: u64) -> bool {
        let below = STACK_BELOW.min(self.sp0 / 2);
        addr >= self.sp0.wrapping_sub(below) && addr.wrapping_add(size) <= self.sp0.wrapping_add(STACK_ABOVE)
    }

    fn is_readonly(&self, addr: u64, size: u64) -> bool {
        let last = addr.wrapping_add(size.saturating_sub(1));
        let i = self.readonly.partition_point(|r| r.0 <= addr);
        last >= addr && i > 0 && self.readonly[i - 1].1 >= last
    }

    fn entries(&self) -> &BTreeSet<u64> {
        self.entries.get_or_init(|| self.prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect())
    }

    /// Whether a function the program knows starts at `addr`.
    pub fn function_starts_at(&self, addr: u64) -> bool {
        self.entries().contains(&addr)
    }

    /// A function other than the one at `addr`: the next one, or the first.
    pub fn another_function(&self, addr: u64) -> Option<u64> {
        let e = self.entries();
        e.range(addr.wrapping_add(1)..).next().or_else(|| e.iter().next()).copied().filter(|&f| f != addr)
    }

    pub fn code_mapped(&self, addr: u64) -> bool {
        self.prog.entry_bytes_mapped(&Address::new(Rc::clone(&self.code), addr))
    }

    /// Whether `addr` lies in a section the loader marks as code: no callee
    /// writes there, literal pools included.
    fn in_code_section(&self, addr: u64) -> bool {
        let i = self.code_sections.partition_point(|r| r.0 <= addr);
        i > 0 && self.code_sections[i - 1].1 >= addr
    }

    /// Whether `addr` is mapped code: in a section the loader marks as code,
    /// or anywhere mapped in an image without a section table.
    pub fn is_code(&self, addr: u64) -> bool {
        self.code_mapped(addr)
            && (self.code_sections.is_empty() || self.in_code_section(addr))
    }

    /// A byte of the loaded image, read a page at a time. A page the loader
    /// does not map whole is read a byte at a time, as each byte is asked for:
    /// a random pointer lands in one far more often than in the image.
    pub(crate) fn image_byte(&self, addr: u64) -> Option<u8> {
        if let Some(m) = &self.mapped {
            let i = m.partition_point(|r| r.0 <= addr);
            if i == 0 || m[i - 1].1 < addr {
                return None;
            }
        }
        let page = addr & !(IMAGE_PAGE - 1);
        let at = (addr - page) as usize;
        let known = self.image.borrow().get(&page).map(|p| p.as_ref().map(|bytes| bytes[at]));
        match known {
            Some(Some(b)) => return Some(b),
            Some(None) => {}
            None => {
                let base = Address::new(Rc::clone(&self.ram), page);
                let whole = self
                    .prog
                    .read_bytes_at(&base, IMAGE_PAGE as usize)
                    .filter(|v| v.len() == IMAGE_PAGE as usize)
                    .map(Vec::into_boxed_slice);
                let b = whole.as_ref().map(|bytes| bytes[at]);
                self.image.borrow_mut().insert(page, whole);
                if b.is_some() {
                    return b;
                }
            }
        }
        if let Some(b) = self.image_bytes.borrow().get(&addr) {
            return *b;
        }
        let b = self.prog.read_bytes_at(&Address::new(Rc::clone(&self.ram), addr), 1).map(|v| v[0]);
        self.image_bytes.borrow_mut().insert(addr, b);
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
        if let Some(b) = self.concolic_inputs.borrow().as_ref().and_then(|m| m.get(&(space.get_index(), off))) {
            return *b;
        }
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
    /// a pointer that escaped into an earlier call, never read-only data or code. This
    /// is no more than kuna itself treats as changed by a call, so a value it
    /// keeps across one stays equal unless kuna is wrong to keep it.
    pub fn callee_writable(&self, escaped: &Escaped, space: &AddrSpace, off: u64) -> bool {
        if !self.is_memory(space) || self.is_readonly(off, 1) || self.in_code_section(off) {
            return false;
        }
        escaped.reaches(off) || (space.get_index() == self.ram.get_index() && !self.in_stack(off, 1) && self.image_byte(off).is_some())
    }

    /// The bytes a callee given `p` reaches, `[first, last]`: the local object
    /// kuna gives `p` when it points into the frame, else a window past it.
    fn reach_of(&self, p: u64, out: &mut Vec<(u64, u64)>) {
        if self.in_stack(p, 1) {
            if let Some((start, end)) = self.stack_object(p).filter(|(s, e)| e > s) {
                out.push((start, end - 1));
            }
            return;
        }
        let last = p.wrapping_add(ESCAPE_WINDOW - 1);
        if last >= p {
            out.push((p, last));
        } else {
            out.push((p, u64::MAX));
            out.push((0, last));
        }
    }

    /// The `[start, end)` of the local symbol of kuna's that holds frame
    /// address `p`, or of the register save area when `varargsave` folded it:
    /// the bytes `va_start` defines, not the padding kuna's symbol may take in.
    pub fn stack_object(&self, p: u64) -> Option<(u64, u64)> {
        if let Some(hit) = self.locals.borrow().get(&p) {
            return *hit;
        }
        let found = self.fd.and_then(|fd| {
            if let Some(f) = fd.vararg_folded() {
                let start = self.ram.wrap_offset(self.sp0.wrapping_add(f.area as u64));
                let end = start.wrapping_add(kuna_decomp::kuna_varargsave::SAVE_AREA as u64);
                if p >= start && p < end {
                    return Some((start, end));
                }
            }
            let local = fd.get_scope_local()?;
            let db = local.database();
            let at = Address::new(Rc::clone(&self.stack), self.stack.wrap_offset(p.wrapping_sub(self.sp0)));
            let entry = db.entry(local.scope_id(), db.find_container_ignore_usepoint(local.scope_id(), &at, 1)?);
            let start = self.ram.wrap_offset(self.sp0.wrapping_add(entry.get_first()));
            let size = u64::try_from(entry.get_size()).ok().filter(|&s| s > 0)?;
            start.checked_add(size).map(|end| (start, end))
        });
        self.locals.borrow_mut().insert(p, found);
        found
    }

    /// The byte at `off` with no write by a side since its last call: the
    /// initial state before any call, what the calls left after one.
    pub fn base_byte(&self, run: u32, epoch: u32, escaped: &Escaped, space: &AddrSpace, off: u64) -> u8 {
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

    /// C5: where an input byte is, when the run's random state chooses the
    /// byte at `off`: not a tracked register, the stack pointer, the extended
    /// part of an argument, the image, the frame below the entry stack
    /// pointer, or a temporary.
    pub fn input_place(&self, space: &AddrSpace, off: u64) -> Option<crate::concolic::Place> {
        let idx = space.get_index();
        let place = |boolean| Some(crate::concolic::Place { space: idx, off, boolean });
        if self.tracked.iter().any(|&(sp, t, size, _)| sp == idx && off >= t && off < t + u64::from(size)) {
            return None;
        }
        if let Some(&(_, b, size)) = self.booleans.iter().find(|&&(sp, b, size)| sp == idx && off >= b && off < b + u64::from(size)) {
            let low = if space.is_big_endian() { b + u64::from(size) - 1 } else { b };
            return if off == low { place(true) } else { None };
        }
        if self.unique.as_ref().is_some_and(|u| u.get_index() == idx) {
            return None;
        }
        if idx == self.register.get_index() {
            let (sp_off, sp_size) = self.sp;
            if off >= sp_off && off < sp_off + u64::from(sp_size) {
                return None;
            }
            let extended = self.extensions.iter().any(|x| {
                let (c, cs) = x.container;
                let (p, ps) = x.param;
                x.space == idx && off >= c && off < c + u64::from(cs) && !(off >= p && off < p + u64::from(ps))
            });
            return if extended { None } else { place(false) };
        }
        if idx == self.ram.get_index() {
            if self.in_stack(off, 1) {
                return if off >= self.sp0 { place(false) } else { None };
            }
            if self.image_byte(off).is_some() {
                return None;
            }
        }
        place(false)
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

/// The frame bytes a callee run for real reads.
pub type FrameReads = HashSet<u64, BuildHasherDefault<Fast>>;

/// Memory this side wrote, in 64-byte pages each with a mask of the bytes set.
/// Every reader that needs an order collects into a sorted map, so the pages
/// need none.
#[derive(Clone, Default)]
struct Mem {
    /// Shared until written: cloning a state to run a callee for real copies
    /// the page pointers, not the pages.
    pages: HashMap<(i32, u64), Rc<Page>, BuildHasherDefault<Fast>>,
}

#[derive(Clone)]
struct Page {
    bytes: [u8; 64],
    set: u64,
}

impl Mem {
    fn get(&self, sp: i32, off: u64) -> Option<u8> {
        let p = self.pages.get(&(sp, off >> 6))?;
        let i = off & 63;
        (p.set >> i & 1 == 1).then(|| p.bytes[i as usize])
    }

    fn contains(&self, sp: i32, off: u64) -> bool {
        self.get(sp, off).is_some()
    }

    fn insert(&mut self, sp: i32, off: u64, b: u8) {
        let p = Rc::make_mut(self.pages.entry((sp, off >> 6)).or_insert_with(|| Rc::new(Page { bytes: [0; 64], set: 0 })));
        let i = off & 63;
        p.bytes[i as usize] = b;
        p.set |= 1 << i;
    }

    /// Every byte, in no order.
    fn iter(&self) -> impl Iterator<Item = ((i32, u64), u8)> + '_ {
        self.pages.iter().flat_map(|(&(sp, page), p)| {
            (0..64u64).filter(move |i| p.set >> i & 1 == 1).map(move |i| ((sp, page << 6 | i), p.bytes[i as usize]))
        })
    }

    /// The bytes of `sp` in `[start, end)`, in no order: the range's pages
    /// looked up, or every page scanned when there are fewer of those.
    fn range(&self, sp: i32, start: u64, end: u64) -> Vec<(u64, u8)> {
        if end <= start {
            return Vec::new();
        }
        let (first, last) = (start >> 6, (end - 1) >> 6);
        let inside = |off: u64| off >= start && off < end;
        if last - first < self.pages.len() as u64 {
            (first..=last)
                .filter_map(|page| self.pages.get(&(sp, page)).map(|p| (page, p)))
                .flat_map(|(page, p)| (0..64u64).filter(move |i| p.set >> i & 1 == 1).map(move |i| (page << 6 | i, p.bytes[i as usize])))
                .filter(|&(off, _)| inside(off))
                .collect()
        } else {
            self.iter().filter(|&((s, off), _)| s == sp && inside(off)).map(|((_, off), b)| (off, b)).collect()
        }
    }

    fn retain(&mut self, mut keep: impl FnMut(i32, u64) -> bool) {
        self.pages.retain(|&(sp, page), p| {
            let gone = (0..64u64).filter(|&i| p.set >> i & 1 == 1 && !keep(sp, page << 6 | i)).fold(0u64, |m, i| m | 1 << i);
            if gone != 0 {
                Rc::make_mut(p).set &= !gone;
            }
            p.set != 0
        });
    }
}

/// The temporaries of the current instruction, one entry per value written:
/// p-code reads a temporary as it wrote it, so a read is usually one lookup.
#[derive(Clone, Default)]
struct Temps {
    entries: Vec<(u64, u32, u128, bool)>,
}

impl Temps {
    fn exact(&self, off: u64, size: u32) -> Option<u128> {
        self.entries.iter().rev().find(|e| e.0 == off && e.1 == size).map(|e| e.2)
    }

    fn byte(&self, off: u64) -> Option<u8> {
        self.entries.iter().rev().find(|e| off >= e.0 && off - e.0 < u64::from(e.1)).map(|&(at, size, v, big)| {
            let i = off - at;
            let shift = if big { u64::from(size) - 1 - i } else { i };
            (v >> (shift * 8)) as u8
        })
    }

    fn set(&mut self, off: u64, size: u32, v: u128, big: bool) {
        let end = off + u64::from(size);
        if self.entries.iter().any(|e| e.0 < end && off < e.0 + u64::from(e.1)) {
            let old = std::mem::take(&mut self.entries);
            for (at, n, val, b) in old {
                if at < end && off < at + u64::from(n) {
                    for i in 0..u64::from(n) {
                        let o = at + i;
                        if o < off || o >= end {
                            let shift = if b { u64::from(n) - 1 - i } else { i };
                            self.entries.push((o, 1, (val >> (shift * 8)) & 0xff, b));
                        }
                    }
                } else {
                    self.entries.push((at, n, val, b));
                }
            }
        }
        self.entries.push((off, size, v & crate::value::mask(size), big));
    }
}

/// A multiplicative hasher for the temporaries' byte offsets: they are
/// cleared every instruction, so the map keeps its capacity and its hashing
/// costs one multiply.
#[derive(Default, Clone, Copy)]
pub struct Fast(u64);

impl std::hash::Hasher for Fast {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ u64::from(b)).wrapping_mul(0x51_7cc1_b727_220a_95);
        }
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7cc1_b727_220a_95);
    }
}

/// Bytes of the image read at once.
const IMAGE_PAGE: u64 = 256;

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
    unique: Temps,
    mem: Mem,
    /// Calls made so far.
    pub epoch: u32,
    /// Pointer-sized values passed to calls so far.
    pub escaped: Escaped,
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
    /// While a callee runs for real: the frame bytes it reads.
    pub frame_reads: Option<FrameReads>,
    /// Code other than the function's own is running (a callee run for real
    /// or followed); what it writes in the frame is not the function's.
    pub in_callee_code: bool,
    /// While a callee runs for real: the argument-register bytes it reads
    /// before writing them.
    pub live: Option<LiveIn>,
    /// Frame objects a call received a pointer to so far.
    passed_objects: Vec<(u64, u64)>,
    /// Varies what a stub leaves in the registers it kills.
    pub kill_salt: u32,
    /// Registers `kill_salt` leaves alone.
    pub kill_salt_skip: Vec<(u64, u32)>,
    /// Frame bytes last written by such code: a frame object's comparison at
    /// a call leaves them out, as a helper's leftovers in a buffer C never
    /// initialized.
    callee_written: HashSet<u64, BuildHasherDefault<Fast>>,
}

/// The argument-register bytes a callee run for real reads before writing
/// them: what it takes from its caller. A read whose value only goes to a
/// stack store (a spill, a `push`, a varargs prologue) is not one.
#[derive(Clone, Debug)]
pub struct LiveIn {
    /// Per register byte up to the last argument register's end: an
    /// argument register's byte, and whether the callee wrote it.
    tracked: Vec<bool>,
    written: Vec<bool>,
    pub read: BTreeMap<u64, u8>,
    /// Reads by the current op, committed when the next op starts.
    last: Vec<(u64, u8)>,
    /// Reads a COPY moved into a temporary this instruction, by its offset.
    pending: Vec<(u64, Vec<(u64, u8)>)>,
}

impl LiveIn {
    pub fn new(regs: Vec<(u64, u32)>) -> Self {
        let end = regs.iter().map(|&(o, n)| (o + u64::from(n)) as usize).max().unwrap_or(0).min(DENSE_REGISTERS as usize);
        let mut tracked = vec![false; end];
        for &(o, n) in &regs {
            for at in o..o + u64::from(n) {
                if let Some(t) = tracked.get_mut(at as usize) {
                    *t = true;
                }
            }
        }
        LiveIn { written: vec![false; end], tracked, read: BTreeMap::new(), last: Vec::new(), pending: Vec::new() }
    }

    fn saw(&mut self, off: u64, b: u8) {
        let i = off as usize;
        if self.tracked.get(i) == Some(&true) && !self.written[i] && !self.read.contains_key(&off) {
            self.last.push((off, b));
        }
    }

    fn wrote(&mut self, off: u64) {
        if let Some(w) = self.written.get_mut(off as usize) {
            *w = true;
        }
    }

    /// An op starts: what the previous op read is an input.
    pub fn op(&mut self) {
        for (off, b) in self.last.drain(..) {
            self.read.entry(off).or_insert(b);
        }
    }

    /// The current op copied what it read into temporary `u`.
    pub fn copied(&mut self, u: u64) {
        let reads = std::mem::take(&mut self.last);
        self.pending.push((u, reads));
    }

    /// The current op stored what it read, or temporary `u`, to the stack.
    pub fn spilled(&mut self, u: Option<u64>) {
        self.last.clear();
        if let Some(u) = u {
            self.pending.retain(|(t, _)| *t != u);
        }
    }

    /// An instruction ends: copies it did not spill are inputs.
    pub fn instruction(&mut self) {
        self.op();
        for (_, reads) in std::mem::take(&mut self.pending) {
            for (off, b) in reads {
                self.read.entry(off).or_insert(b);
            }
        }
    }
}

/// One thing a callee run for real did to memory.
#[derive(Clone, Debug)]
pub enum Event {
    Write(i32, u64, u8),
    /// A call it made in turn, with the killed registers, its arguments and
    /// the extension of its return value.
    Call(Vec<(u64, u32)>, Vec<u128>, Option<Extension>),
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
    /// Frame bytes a callee's code left in an object this side passes: not
    /// compared on either side.
    pub skip: BTreeSet<u64>,
    /// The frame objects, `[start, end)`, this side passes to the call. Only
    /// those both sides pass are compared.
    pub objects: Vec<(u64, u64)>,
    pub epoch: u32,
    pub escaped: Escaped,
}

impl<'e, 'p> State<'e, 'p> {
    pub fn new(env: &'e Env<'p>, run: u32) -> Self {
        State {
            env,
            run,
            regs: Vec::new(),
            flags: Vec::new(),
            unique: Temps::default(),
            mem: Mem::default(),
            epoch: 0,
            escaped: Escaped::default(),
            log: None,
            read_uninitialized: false,
            saved: BTreeMap::new(),
            read_saved: false,
            frame_reads: None,
            in_callee_code: false,
            callee_written: HashSet::default(),
            live: None,
            passed_objects: Vec::new(),
            kill_salt: 0,
            kill_salt_skip: Vec::new(),
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
        self.mem.retain(|s, off| s != ram || off < lo || off >= sp);
        let dead: Vec<u64> = self.saved.range(lo..sp).map(|(k, _)| *k).collect();
        for k in dead {
            self.saved.remove(&k);
        }
    }

    fn is_register(&self, space: &AddrSpace, off: u64) -> bool {
        space.get_index() == self.env.register.get_index() && off < DENSE_REGISTERS
    }

    /// C5: whether the byte at `off` still holds the run's initial value: this
    /// side has not written it, and for memory, no call has run yet.
    pub fn untouched(&self, space: &AddrSpace, off: u64) -> bool {
        if self.is_register(space, off) {
            return self.flags.get(off as usize).is_none_or(|f| f & WRITTEN == 0);
        }
        if self.is_unique(space) {
            return false;
        }
        self.epoch == 0 && !self.mem.contains(space.get_index(), off)
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
            let b = self.regs[i];
            if let Some(l) = self.live.as_mut() {
                l.saw(off, b);
            }
            return b;
        }
        let found = if self.is_unique(space) { self.unique.byte(off) } else { self.mem.get(space.get_index(), off) };
        if let Some(reads) = self.frame_reads.as_mut() {
            if space.get_index() == self.env.ram.get_index() && self.env.in_stack(off, 1) {
                reads.insert(off);
            }
        }
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
            if let Some(l) = self.live.as_mut() {
                l.wrote(off);
            }
        } else if self.is_unique(space) {
            self.unique.set(off, 1, u128::from(b), false);
        } else {
            if let Some(log) = self.log.as_mut() {
                log.push(Event::Write(space.get_index(), off, b));
            }
            self.mem.insert(space.get_index(), off, b);
            if space.get_index() == self.env.ram.get_index() && self.env.in_stack(off, 1) {
                if self.in_callee_code {
                    self.callee_written.insert(off);
                } else if !self.callee_written.is_empty() {
                    self.callee_written.remove(&off);
                }
            }
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
                Event::Call(killed, args, ext) => self.call(killed, args, ext.as_ref()),
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

    /// Every register byte this side holds, written or read: `(offset, byte)`.
    pub fn register_file(&self) -> Vec<(u64, u8)> {
        self.flags.iter().enumerate().filter(|(_, f)| **f & PRESENT != 0).map(|(i, _)| (i as u64, self.regs[i])).collect()
    }

    /// Put back every register byte `before` held, except those in `keep`:
    /// a callee run for real leaves the registers the calling convention says
    /// it preserves as they were. Returns the offsets the callee had changed.
    pub fn preserve_registers(&mut self, before: &State, keep: &[(u64, u32)]) -> Vec<u64> {
        let mut changed = Vec::new();
        let n = self.regs.len().max(before.regs.len());
        self.grow(n);
        let mut kept = vec![false; n];
        for &(off, size) in keep {
            let lo = (off as usize).min(n);
            let hi = (off as usize).saturating_add(size as usize).min(n);
            kept[lo..hi].fill(true);
        }
        for i in 0..n {
            let o = i as u64;
            if kept[i] {
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
        let manage = self.env.prog.arch().manage();
        let bytes = self.mem.iter().filter(|((sp, off), _)| manage.get_space(*sp).is_some_and(|s| self.env.observable(s, *off))).collect();
        Writes { bytes, epoch: self.epoch, escaped: self.escaped.clone(), skip: BTreeSet::new(), objects: Vec::new() }
    }

    /// [`Self::writes`] at a call passing `args`, plus the frame objects a
    /// callee can reach: one an argument points to, one whose address this side
    /// stored outside the frame (`global = &x`) or in an object already
    /// reachable (`h.p = &x; f(&h)`), and one an earlier call could reach.
    pub fn writes_at_call(&mut self, args: &[u128]) -> Writes {
        let mut w = self.writes();
        let ram = self.env.ram.get_index();
        let mut found: Vec<u64> = args.iter().map(|&a| a as u64).collect();
        let env = self.env;
        found.extend(self.stored_pointers(|off| !env.in_stack(off, 1)));
        let mut scanned = 0;
        loop {
            for &a in &found {
                let Some(o) = self.env.stack_object(a).filter(|_| self.env.in_stack(a, 1)) else { continue };
                if !self.passed_objects.contains(&o) {
                    self.passed_objects.push(o);
                }
            }
            if scanned == self.passed_objects.len() {
                break;
            }
            let objects = self.passed_objects[scanned..].to_vec();
            scanned = self.passed_objects.len();
            found = self.stored_pointers(|off| objects.iter().any(|&(s, e)| off >= s && off < e));
        }
        for &(start, end) in &self.passed_objects {
            if w.objects.contains(&(start, end)) {
                continue;
            }
            w.objects.push((start, end));
            for (off, v) in self.mem.range(ram, start, end) {
                if self.callee_written.contains(&off) {
                    w.skip.insert(off);
                } else {
                    w.bytes.insert((ram, off), v);
                }
            }
        }
        w
    }

    /// The pointer-sized, aligned words this side wrote in RAM at offsets
    /// `keep` accepts.
    fn stored_pointers(&self, keep: impl Fn(u64) -> bool) -> Vec<u64> {
        let ram = self.env.ram.get_index();
        let size = u64::from(self.env.ram.get_addr_size()).clamp(1, 8);
        let big = self.env.ram.is_big_endian();
        let mut words = Vec::new();
        for ((sp, off), _) in self.mem.iter() {
            if sp != ram || off % size != 0 || !keep(off) {
                continue;
            }
            let mut v = 0u64;
            let mut whole = true;
            for i in 0..size {
                let Some(b) = self.mem.get(ram, off + i) else {
                    whole = false;
                    break;
                };
                let shift = if big { size - 1 - i } else { i };
                v |= u64::from(b) << (shift * 8);
            }
            if whole {
                words.push(v);
            }
        }
        words
    }

    /// Extend the value in `x.param` across `x.container`, as the ABI says a
    /// callee leaves its return value.
    fn extend(&mut self, x: &Extension) {
        let Some(space) = self.env.prog.arch().manage().get_space(x.space) else { return };
        let (p, ps) = x.param;
        let (c, cs) = x.container;
        let Ok(v) = self.read(&space, p, ps) else { return };
        let v = if x.boolean { v & 1 } else { v };
        let negative = x.signed && ps > 0 && (v >> (ps * 8 - 1)) & 1 == 1;
        let ext = if negative { v | !mask(ps) } else { v & mask(ps) };
        let _ = self.write(&space, c, cs, ext & mask(cs));
    }

    /// A call's effect: it kills the registers in `killed`, `args` escape into
    /// it, and it may write any memory it can reach. The memory is written
    /// lazily: every byte this side did not write since reads what
    /// [`Env::base_byte`] says for the new epoch.
    pub fn call(&mut self, killed: &[(u64, u32)], args: &[u128], ret: Option<&Extension>) {
        if let Some(log) = self.log.as_mut() {
            log.push(Event::Call(killed.to_vec(), args.to_vec(), ret.cloned()));
        }
        let call = self.epoch;
        let register = Rc::clone(&self.env.register);
        for &(off, size) in killed {
            for i in 0..u64::from(size) {
                let salt = if self.kill_salt_skip.iter().any(|&(o, n)| off + i >= o && off + i < o + u64::from(n)) { 0 } else { self.kill_salt };
                let b = self.env.killed_byte(self.run ^ salt, call, off + i);
                self.put(&register, off + i, b);
            }
        }
        if let Some(x) = ret {
            self.extend(x);
        }
        let size = u64::from(self.env.ram.get_addr_size()).min(8);
        let ptr_mask = if size >= 8 { u64::MAX } else { (1u64 << (size * 8)) - 1 };
        self.escaped.add(self.env, args.iter().map(|&a| a as u64 & ptr_mask).filter(|&p| p > 0xffff));
        let env = self.env;
        let escaped = std::mem::take(&mut self.escaped);
        let manage = env.prog.arch().manage();
        self.mem.retain(|sp, off| manage.get_space(sp).is_some_and(|s| !env.callee_writable(&escaped, s, off)));
        self.escaped = escaped;
        self.epoch += 1;
    }

    /// Forget the temporaries: they live for one instruction.
    pub fn end_instruction(&mut self) {
        self.unique.entries.clear();
    }

    /// Read a processor or temporary space value of up to sixteen bytes.
    pub fn read(&mut self, space: &AddrSpace, off: u64, size: u32) -> Result<u128, Halt> {
        if size > MAX_SIZE {
            return Err(Halt::Unsupported(format!("{size}-byte value")));
        }
        if self.is_unique(space) {
            if let Some(v) = self.unique.exact(off, size) {
                return Ok(v);
            }
        }
        let end = off.wrapping_add(u64::from(size));
        if self.is_register(space, off) && end <= DENSE_REGISTERS && end > off {
            self.grow(end as usize);
            for i in off as usize..end as usize {
                if self.flags[i] & PRESENT == 0 {
                    self.regs[i] = self.env.initial_byte(self.run, space, i as u64);
                    self.flags[i] |= PRESENT;
                }
                if let Some(l) = self.live.as_mut() {
                    l.saw(i as u64, self.regs[i]);
                }
            }
            let bytes = &self.regs[off as usize..end as usize];
            let fold = |v: u128, &b: &u8| (v << 8) | u128::from(b);
            return Ok(if space.is_big_endian() { bytes.iter().fold(0, fold) } else { bytes.iter().rev().fold(0, fold) });
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

    /// Store `size` bytes. A store that changes the image's read-only data or
    /// code traps, as it does on the target; one that leaves it as it is (the
    /// decompiler's value of a global, kept across a call) does nothing.
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
        if self.is_unique(space) {
            self.unique.set(off, size, val, space.is_big_endian());
            return Ok(());
        }
        let end = off.wrapping_add(u64::from(size));
        if self.is_register(space, off) && end <= DENSE_REGISTERS && end > off {
            self.grow(end as usize);
            for i in 0..u64::from(size) {
                let shift = if space.is_big_endian() { u64::from(size) - 1 - i } else { i };
                let at = (off + i) as usize;
                self.regs[at] = (val >> (shift * 8)) as u8;
                self.flags[at] |= PRESENT | WRITTEN;
                if let Some(l) = self.live.as_mut() {
                    l.wrote(at as u64);
                }
            }
            return Ok(());
        }
        if space.get_index() == self.env.ram.get_index()
            && (self.env.is_readonly(off, size.into()) || self.env.in_code_section(off))
        {
            if self.read(space, off, size)? == val & crate::value::mask(size) {
                return Ok(());
            }
            return Err(Halt::Trap(format!("{READONLY_STORE} at {off:#x}")));
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
                self.unique.byte(at).is_some()
            } else {
                self.mem.contains(space.get_index(), at)
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
