//! (kuna) Global blocks: a block literal the compiler laid out in `__const` —
//! `isa = _NSConcreteGlobalBlock, flags, reserved, invoke, descriptor` — is
//! named and typed as a `Block_layout`, and its invoke function is named and,
//! when the descriptor carries a signature, typed from
//! it: `long __pick_block_invoke(Block_layout *block, long arg1)`.
//!
//! A stack block is built by its creator at run time: `isa = _NSConcreteStackBlock`
//! stored to a frame slot, the invoke and descriptor addresses stored 16 and 24
//! bytes after it. [`stack_blocks`] finds those stores in arm64 code (the
//! `adrp`/`ldr` of the isa's GOT slot, the `adrp`/`add` of the two addresses,
//! `str`/`stp` to `sp`) and in x86-64 code (`x86.rs`), names the invoke the
//! way Clang mangles it (`__17-[Runner maker:]_block_invoke`) and types it
//! from the descriptor's signature.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use kuna_decomp::fspec::PrototypePieces;
use object::read::{Object, ObjectSection};

use super::encoding::{decode_ivar_type, split_method, TypeCtx};
use super::sections::MachoImage;
use crate::pass::{AnalysisOutput, ConstGate, ConstRange, FidMatch, LocalFact, SymFact, SymKind, TypedDataFact};

mod x86;

/// `Block_layout.flags` bit: the descriptor has copy/dispose helpers.
const BLOCK_HAS_COPY_DISPOSE: u32 = 1 << 25;

/// The `Block_layout` struct, interned once: `isa`, `flags`, `reserved`,
/// `invoke`, `descriptor`.
pub(super) fn block_layout(tctx: &TypeCtx) -> Option<Rc<Datatype>> {
    let types: &dyn TypeFactory = tctx.types;
    if let Some(t) = types.find_by_name("Block_layout").ok().flatten() {
        return (t.get_metatype() == type_metatype::TYPE_STRUCT).then_some(t);
    }
    let ps = tctx.ptr_size;
    let shell = types.kuna_declare_record("Block_layout", false).ok()?;
    let voidp = decode_ivar_type("^v", tctx)?;
    let int = decode_ivar_type("i", tctx)?;
    let flags = decode_ivar_type("I", tctx)?;
    let fields = vec![
        TypeField::new(0, 0, "isa".to_string(), Rc::clone(&voidp)),
        TypeField::new(ps, ps, "flags".to_string(), flags),
        TypeField::new(ps + 4, ps + 4, "reserved".to_string(), int),
        TypeField::new(2 * ps, 2 * ps, "invoke".to_string(), Rc::clone(&voidp)),
        TypeField::new(3 * ps, 3 * ps, "descriptor".to_string(), voidp),
    ];
    types.set_fields_struct_raw(&shell, fields, Vec::new(), 4 * ps, ps, 0).ok()
}

/// A block descriptor: `reserved`, `size`, the copy/dispose helpers when the
/// flags say so (`Block_descriptor`), then the signature (`Block_descriptor_basic`
/// without helpers). Interned once per shape.
fn descriptor_layout(tctx: &TypeCtx, helpers: bool) -> Option<Rc<Datatype>> {
    let types: &dyn TypeFactory = tctx.types;
    let name = if helpers { "Block_descriptor" } else { "Block_descriptor_basic" };
    if let Some(t) = types.find_by_name(name).ok().flatten() {
        return (t.get_metatype() == type_metatype::TYPE_STRUCT).then_some(t);
    }
    let ps = tctx.ptr_size;
    let shell = types.kuna_declare_record(name, false).ok()?;
    let word = decode_ivar_type("Q", tctx)?;
    let voidp = decode_ivar_type("^v", tctx)?;
    let mut fields = vec![
        TypeField::new(0, 0, "reserved".to_string(), Rc::clone(&word)),
        TypeField::new(ps, ps, "size".to_string(), word),
    ];
    let mut at = 2 * ps;
    if helpers {
        fields.push(TypeField::new(at, at, "copy".to_string(), Rc::clone(&voidp)));
        fields.push(TypeField::new(at + ps, at + ps, "dispose".to_string(), voidp));
        at += 2 * ps;
    }
    fields.push(TypeField::new(at, at, "signature".to_string(), decode_ivar_type("*", tctx)?));
    types.set_fields_struct_raw(&shell, fields, Vec::new(), at + ps, ps, 0).ok()
}

/// What an arm64 register holds while a function is scanned.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Held {
    Page(u64),
    Addr(u64),
    Isa,
    /// A frame address: `sp + off`.
    Stack(u64),
    /// An immediate (`movz`/`movk`, a literal-pool constant).
    Imm(u64),
}

/// One value stored on the frame: where, how wide, what it was.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Store {
    off: u64,
    width: u8,
    /// `d`/`s` register stores (a float), else an integer register.
    float: bool,
    held: Option<Held>,
}

/// A block literal a function builds on its frame.
#[derive(Clone, Debug, PartialEq)]
struct Site {
    /// The pc of the isa store, the pc of the last `ret` before it, and the
    /// function start (`LC_FUNCTION_STARTS`) the scan last passed.
    pc: u64,
    last_ret: u64,
    start: Option<u64>,
    /// The literal's frame offset (from `sp`), and how far `sp` was below the
    /// entry `sp` there, so `frame - depth` is the stack-space offset.
    frame: u64,
    depth: u64,
    invoke: u64,
    descriptor: Option<u64>,
    /// The pool word the flags were loaded from, when a `d` register stored them.
    flags_at: Option<u64>,
    /// Every store into the literal past its header, by frame offset.
    captures: Vec<Store>,
}

/// A `__block` variable's cell on the frame: `isa = 0, forwarding = &cell,
/// flags, size` and the value.
#[derive(Clone, Debug, PartialEq)]
struct ByrefCell {
    frame: u64,
    depth: u64,
    flags: u32,
    size: u32,
    value: Option<Store>,
}

/// The register state of one function scan.
struct Scan<'a> {
    regs: [Option<Held>; 32],
    /// The `d` registers: only a pooled constant is ever known, and where it
    /// was loaded from.
    fregs: [Option<Held>; 32],
    fsrc: [Option<u64>; 32],
    /// Frame offset → the store there (the last one wins).
    stores: Vec<Store>,
    /// The block literals being built: each store after an isa store fills in
    /// the literal it lands in, so a frame slot the function reuses for a
    /// second block keeps the first block's fields.
    pending: Vec<Site>,
    done: Vec<Site>,
    /// `(frame offset, pc, depth)` of each `forwarding = &cell` store.
    cells: Vec<(u64, u64, u64)>,
    /// How far `sp` is below the entry `sp`.
    depth: u64,
    last_ret: u64,
    start: Option<u64>,
    /// The pointer-sized word at an address of the image (literal pools).
    read_ptr: &'a dyn Fn(u64) -> Option<u64>,
    /// Whether an address is code: an invoke is, a descriptor is not.
    is_code: &'a dyn Fn(u64) -> bool,
    isa_slots: &'a [u64],
    /// The global block literals (sorted), and each materialized reference to
    /// one: `(literal, pc, last_ret, start)`.
    globals: &'a [u64],
    global_refs: Vec<(u64, u64, u64, Option<u64>)>,
}

impl<'a> Scan<'a> {
    /// End the function being scanned: its literals with an invoke and its
    /// `__block` cells are found, and the register and frame state starts over.
    fn flush(&mut self, sites: &mut Vec<Site>, cells: &mut Vec<ByrefCell>) {
        let finished = self.done.drain(..).chain(self.pending.drain(..));
        for mut site in finished {
            if site.invoke == 0 {
                continue;
            }
            site.captures.sort_by_key(|s| s.off);
            sites.push(site);
        }
        for &(frame, _, depth) in &self.cells {
            let at = |o: u64| self.stores.iter().find(|s| s.off == frame.wrapping_add(o));
            let (flags, size) = match (at(16), at(20)) {
                (Some(Store { width: 8, held: Some(Held::Imm(v)), .. }), _) => ((*v & 0xFFFF_FFFF) as u32, (*v >> 32) as u32),
                (Some(Store { width: 4, held: Some(Held::Imm(f)), .. }), Some(Store { width: 4, held: Some(Held::Imm(s)), .. })) => (*f as u32, *s as u32),
                _ => continue,
            };
            let value_at = if flags & BLOCK_HAS_COPY_DISPOSE != 0 { 40 } else { 24 };
            cells.push(ByrefCell { frame, depth, flags, size, value: at(value_at).copied() });
        }
        self.cells.clear();
        self.stores.clear();
        self.regs = [None; 32];
        self.fregs = [None; 32];
        self.fsrc = [None; 32];
        self.depth = 0;
    }

    fn new(read_ptr: &'a dyn Fn(u64) -> Option<u64>, is_code: &'a dyn Fn(u64) -> bool, isa_slots: &'a [u64], globals: &'a [u64]) -> Scan<'a> {
        Scan {
            regs: [None; 32],
            fregs: [None; 32],
            fsrc: [None; 32],
            stores: Vec::new(),
            pending: Vec::new(),
            done: Vec::new(),
            cells: Vec::new(),
            depth: 0,
            last_ret: 0,
            start: None,
            read_ptr,
            is_code,
            isa_slots,
            globals,
            global_refs: Vec::new(),
        }
    }

    fn held(&self, r: usize) -> Option<Held> {
        if r == 31 { Some(Held::Stack(0)) } else { self.regs[r] }
    }

    /// The frame offset `[xn, #imm]` names, when `xn` holds a frame address.
    fn frame(&self, rn: usize, imm: i64) -> Option<u64> {
        match self.held(rn) {
            Some(Held::Stack(b)) => Some(b.wrapping_add(imm as u64)),
            _ => None,
        }
    }

    fn global_ref(&mut self, pc: u64, a: u64) {
        if self.globals.binary_search(&a).is_ok() {
            self.global_refs.push((a, pc, self.last_ret, self.start));
        }
    }

    fn store(&mut self, pc: u64, off: u64, width: u8, float: bool, rt: usize) {
        let held = if float { self.fregs[rt] } else if rt == 31 { Some(Held::Imm(0)) } else { self.regs[rt] };
        self.store_held(pc, off, width, float, held, if float { self.fsrc[rt] } else { None });
    }

    /// Record one frame store of `held` (a float register's pool word at
    /// `pool`) and fill in the literals it lands in.
    fn store_held(&mut self, pc: u64, off: u64, width: u8, float: bool, held: Option<Held>, pool: Option<u64>) {
        self.stores.retain(|s| s.off != off);
        let store = Store { off, width, float, held };
        self.stores.push(store);
        match held {
            Some(Held::Isa) if width == 8 => {
                // A literal rebuilt in the same slot finishes the earlier one.
                let (same, rest): (Vec<Site>, Vec<Site>) = self.pending.drain(..).partition(|s| s.frame == off);
                self.done.extend(same);
                self.pending = rest;
                // Captures the creator stored before the isa are already in place.
                let captures: Vec<Store> = self.stores.iter().filter(|s| (32..0x400).contains(&s.off.wrapping_sub(off))).copied().collect();
                self.pending.push(Site { pc, last_ret: self.last_ret, start: self.start, frame: off, depth: self.depth, invoke: 0, descriptor: None, flags_at: None, captures });
            }
            Some(Held::Stack(cell)) if width == 8 && off == cell.wrapping_add(8) => self.cells.push((cell, pc, self.depth)),
            _ => {}
        }
        // The first code address at +16 is the invoke and the first data
        // address at +24 the descriptor; a capture is the first store to its
        // slot once the header is there. A later store to the same frame bytes
        // is the frame reused, not the literal rebuilt.
        for site in &mut self.pending {
            let rel = off.wrapping_sub(site.frame);
            match (rel, held) {
                (16, Some(Held::Addr(a))) if site.invoke == 0 && (self.is_code)(a) => site.invoke = a,
                (24, Some(Held::Addr(a))) if site.descriptor.is_none() && !(self.is_code)(a) => site.descriptor = Some(a),
                (8, _) if float && width == 8 && site.invoke == 0 => site.flags_at = pool,
                (32..=0x3FF, _) if site.invoke == 0 || !site.captures.iter().any(|s| s.off == off) => {
                    site.captures.retain(|s| s.off != off);
                    site.captures.push(store);
                }
                _ => {}
            }
        }
    }

    fn step(&mut self, pc: u64, w: u32) {
        let rd = (w & 0x1f) as usize;
        let rn = ((w >> 5) & 0x1f) as usize;
        let imm12 = ((w >> 10) & 0xFFF) as u64;
        let imm9 = ((((w >> 12) & 0x1FF) as i16) << 7 >> 7) as i64;
        let imm7 = ((((w >> 15) & 0x7F) as i8) << 1 >> 1) as i64;
        let rt2 = ((w >> 10) & 0x1f) as usize;
        let adr_imm = ((((w >> 5) & 0x7FFFF) << 2 | (w >> 29) & 0x3) as i32) << 11 >> 11;
        let lit_imm = (((w >> 5) & 0x7FFFF) as i32) << 13 >> 13;
        let lit = pc.wrapping_add((lit_imm as i64 * 4) as u64);
        if w & 0x9F00_0000 == 0x9000_0000 {
            self.regs[rd] = Some(Held::Page((pc & !0xFFF).wrapping_add((adr_imm as i64 as u64) << 12)));
        } else if w & 0x9F00_0000 == 0x1000_0000 {
            let a = pc.wrapping_add(adr_imm as i64 as u64);
            self.regs[rd] = Some(Held::Addr(a));
            self.global_ref(pc, a);
        } else if w & 0xFF00_0000 == 0x5800_0000 {
            // `ldr x, <literal>`: the GOT slot's isa, or a constant.
            self.regs[rd] = if self.isa_slots.contains(&lit) { Some(Held::Isa) } else { (self.read_ptr)(lit).map(Held::Imm) };
        } else if w & 0xFF00_0000 == 0x5C00_0000 {
            // `ldr d, <literal>`: a constant the compiler pooled (flags words).
            self.fregs[rd] = (self.read_ptr)(lit).map(Held::Imm);
            self.fsrc[rd] = Some(lit);
        } else if w & 0xFFC0_0000 == 0xFD40_0000 {
            // `ldr d, [xn, #imm]` off a page: a pooled constant (flags words).
            let at = match self.held(rn) {
                Some(Held::Page(p)) if rn != 31 => Some(p.wrapping_add(imm12 * 8)),
                _ => None,
            };
            self.fregs[rd] = at.and_then(|a| (self.read_ptr)(a)).map(Held::Imm);
            self.fsrc[rd] = at;
        } else if w & 0xFC00_0000 == 0x9400_0000 || w & 0xFEFF_F000 == 0xD63F_0000 {
            // `bl` / `blr` / `blraa`: the caller-saved registers are the callee's.
            self.regs[..19].fill(None);
            self.fregs[..8].fill(None);
            self.fregs[16..].fill(None);
            self.fsrc[..8].fill(None);
            self.fsrc[16..].fill(None);
        } else if w & 0xFF80_0000 == 0x9100_0000 || w & 0xFF80_0000 == 0xD100_0000 {
            // `add`/`sub xd, xn, #imm`
            let imm = if w & (1 << 22) != 0 { imm12 << 12 } else { imm12 };
            let imm = if w & 0xFF80_0000 == 0xD100_0000 { imm.wrapping_neg() } else { imm };
            if rd == 31 && rn == 31 {
                self.depth = self.depth.wrapping_sub(imm);
                return;
            }
            self.regs[rd] = match self.held(rn) {
                Some(Held::Page(p)) if rn != 31 => Some(Held::Addr(p.wrapping_add(imm))),
                Some(Held::Stack(b)) => Some(Held::Stack(b.wrapping_add(imm))),
                _ => None,
            };
            if let Some(Held::Addr(a)) = self.regs[rd] {
                self.global_ref(pc, a);
            }
        } else if w & 0xFF80_0000 == 0x5280_0000 {
            // `movz wd, #imm16, lsl #hw`
            let hw = ((w >> 21) & 3) * 16;
            self.regs[rd] = Some(Held::Imm((((w >> 5) & 0xFFFF) as u64) << hw));
        } else if w & 0xFF80_0000 == 0xD280_0000 {
            let hw = ((w >> 21) & 3) * 16;
            self.regs[rd] = Some(Held::Imm((((w >> 5) & 0xFFFF) as u64) << hw));
        } else if w & 0xFF80_0000 == 0xF280_0000 {
            // `movk xd, #imm16, lsl #hw`
            let hw = ((w >> 21) & 3) * 16;
            if let Some(Held::Imm(v)) = self.regs[rd] {
                self.regs[rd] = Some(Held::Imm((v & !(0xFFFFu64 << hw)) | ((((w >> 5) & 0xFFFF) as u64) << hw)));
            } else {
                self.regs[rd] = None;
            }
        } else if w & 0x7FE0_FFE0 == 0x2A00_03E0 {
            // `mov xd, xm` (`orr xd, xzr, xm`)
            let rm = ((w >> 16) & 0x1f) as usize;
            self.regs[rd] = self.regs[rm];
        } else if w & 0xFFC0_0000 == 0xF940_0000 {
            let slot = match self.held(rn) {
                Some(Held::Page(p)) => p.wrapping_add(imm12 * 8),
                _ => 0,
            };
            self.regs[rd] = self.isa_slots.contains(&slot).then_some(Held::Isa);
        } else if w & 0xFFC0_0000 == 0xF900_0000 {
            if let Some(off) = self.frame(rn, (imm12 * 8) as i64) { self.store(pc, off, 8, false, rd) }
        } else if w & 0xFFC0_0000 == 0xB900_0000 {
            if let Some(off) = self.frame(rn, (imm12 * 4) as i64) { self.store(pc, off, 4, false, rd) }
        } else if w & 0xFFC0_0000 == 0xFD00_0000 {
            if let Some(off) = self.frame(rn, (imm12 * 8) as i64) { self.store(pc, off, 8, true, rd) }
        } else if w & 0xFFC0_0000 == 0xBD00_0000 {
            if let Some(off) = self.frame(rn, (imm12 * 4) as i64) { self.store(pc, off, 4, true, rd) }
        } else if w & 0xFFC0_0000 == 0x3D80_0000 {
            if let Some(off) = self.frame(rn, (imm12 * 16) as i64) { self.store(pc, off, 16, true, rd) }
        } else if w & 0xFFE0_0C00 == 0xF800_0000 {
            // `stur xt, [xn, #imm9]`
            if let Some(off) = self.frame(rn, imm9) { self.store(pc, off, 8, false, rd) }
        } else if w & 0xFFE0_0C00 == 0xB800_0000 {
            if let Some(off) = self.frame(rn, imm9) { self.store(pc, off, 4, false, rd) }
        } else if w & 0xFFE0_0C00 == 0xFC00_0000 {
            if let Some(off) = self.frame(rn, imm9) { self.store(pc, off, 8, true, rd) }
        } else if w & 0xFFE0_0C00 == 0xBC00_0000 {
            if let Some(off) = self.frame(rn, imm9) { self.store(pc, off, 4, true, rd) }
        } else if w & 0xFFC0_0000 == 0xA900_0000 || w & 0xFFC0_0000 == 0xA980_0000 {
            // `stp xt, xt2, [xn, #imm7*8]` (signed offset, or pre-indexed: the
            // base moves first and `sp` is not tracked through it)
            if w & 0xFFC0_0000 == 0xA980_0000 {
                // `stp xt, xt2, [sp, #-N]!`: the frame grows first.
                if rn == 31 {
                    self.depth = self.depth.wrapping_sub((imm7 * 8) as u64);
                }
                return;
            }
            if let Some(off) = self.frame(rn, imm7 * 8) {
                self.store(pc, off, 8, false, rd);
                self.store(pc, off.wrapping_add(8), 8, false, rt2);
            }
        } else if w & 0xFFC0_0000 == 0x2900_0000 {
            if let Some(off) = self.frame(rn, imm7 * 4) {
                self.store(pc, off, 4, false, rd);
                self.store(pc, off.wrapping_add(4), 4, false, rt2);
            }
        } else if w & 0xFFC0_0000 == 0x6D00_0000 {
            if let Some(off) = self.frame(rn, imm7 * 8) {
                self.store(pc, off, 8, true, rd);
                self.store(pc, off.wrapping_add(8), 8, true, rt2);
            }
        }
    }
}

/// What one scan of the text finds: the stack blocks, the `__block` cells and
/// the references to global block literals `(literal, pc, last_ret, start)`.
struct Found {
    sites: Vec<Site>,
    cells: Vec<ByrefCell>,
    global_refs: Vec<(u64, u64, u64, Option<u64>)>,
}

/// `ret`, `retaa`, `retab`.
fn is_ret(w: u32) -> bool {
    matches!(w, 0xD65F_03C0 | 0xD65F_0BFF | 0xD65F_0FFF)
}

/// The stack blocks and `__block` cells of every arm64 function. The scan
/// starts over at each `ret` and at each function start in `starts` (sorted),
/// so a function that ends in a tail call does not hand its frame to the next.
fn stack_block_sites(
    read_ptr: &dyn Fn(u64) -> Option<u64>,
    is_code: &dyn Fn(u64) -> bool,
    text: &[(u64, &[u8])],
    isa_slots: &[u64],
    globals: &[u64],
    starts: &[u64],
) -> Found {
    let mut sites = Vec::new();
    let mut cells = Vec::new();
    let mut global_refs = Vec::new();
    for &(base, data) in text {
        let mut scan = Scan::new(read_ptr, is_code, isa_slots, globals);
        let mut next_start = starts.partition_point(|&a| a < base);
        for (i, c) in data.chunks_exact(4).enumerate() {
            let w = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let pc = base.wrapping_add(4 * i as u64);
            while starts.get(next_start).is_some_and(|&a| a < pc) {
                next_start += 1;
            }
            if starts.get(next_start) == Some(&pc) {
                scan.flush(&mut sites, &mut cells);
                scan.start = Some(pc);
                next_start += 1;
            }
            if is_ret(w) {
                scan.flush(&mut sites, &mut cells);
                scan.last_ret = pc;
            } else {
                scan.step(pc, w);
            }
        }
        scan.flush(&mut sites, &mut cells);
        global_refs.append(&mut scan.global_refs);
    }
    Found { sites, cells, global_refs }
}

/// What a copy helper says each capture is: `(offset, kind)` with the kind the
/// `_Block_object_assign` flags (3 object, 7 block, 8 byref, 16 weak), or 3 for
/// a capture the helper retains itself (`objc_retain`), 7 for `objc_retainBlock`.
fn copy_helper_kinds(img: &MachoImage, helper: u64, runtime: &HashMap<u64, String>) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut last_ldr: [Option<u64>; 32] = [None; 32];
    let mut imm: [Option<u32>; 32] = [None; 32];
    for k in 0..64u64 {
        let pc = helper + 4 * k;
        let Some(w) = img.read_u32(pc) else { break };
        let rd = (w & 0x1f) as usize;
        if w & 0xFFC0_0000 == 0xF940_0000 {
            last_ldr[rd] = Some(((w >> 10) & 0xFFF) as u64 * 8);
        } else if w & 0xFF80_0000 == 0x5280_0000 {
            imm[rd] = Some((w >> 5) & 0xFFFF);
        } else if w & 0x7C00_0000 == 0x1400_0000 {
            // `b`/`bl`
            let target = pc.wrapping_add(((((w & 0x3FF_FFFF) as i32) << 6 >> 6) as i64 * 4) as u64);
            match runtime.get(&target).map(String::as_str) {
                Some("objc_retain") => out.extend(last_ldr[0].map(|o| (o, 3))),
                Some("objc_retainBlock") => out.extend(last_ldr[0].map(|o| (o, 7))),
                Some("_Block_object_assign") => out.extend(last_ldr[1].zip(imm[2]).map(|(o, f)| (o, f))),
                _ => {}
            }
            if w & 0x8000_0000 == 0 {
                break;
            }
        } else if w == 0xD65F_03C0 {
            break;
        }
    }
    out
}

/// The signature a block descriptor carries, at +16 (no copy/dispose helpers)
/// or +32 (with them): the one that reads as a block encoding, and whether
/// the helpers are there.
fn descriptor_signature(img: &MachoImage, desc: u64) -> Option<(String, bool)> {
    [16u64, 32].iter().find_map(|&at| {
        let s = img.rebase_ptr(desc.wrapping_add(at)).and_then(|p| img.read_cstr(p))?;
        s.contains("@?").then_some((s, at == 32))
    })
}

/// A capture's type: what the copy helper says it is, else what the store
/// that filled it was.
fn capture_type(store: &Store, kind: Option<u32>, cells: &[ByrefCell], tctx: &TypeCtx, layout: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    let byref = |cell: Option<&ByrefCell>| -> Option<Rc<Datatype>> {
        let record = byref_layout(cell, tctx)?;
        tctx.types.get_type_pointer(tctx.ptr_size, record, tctx.word_size).ok()
    };
    match (kind, store.held) {
        (Some(3), _) | (Some(16), _) => decode_ivar_type("@", tctx),
        (Some(7), _) => tctx.types.get_type_pointer(tctx.ptr_size, Rc::clone(layout), tctx.word_size).ok(),
        (Some(8), Some(Held::Stack(frame))) => byref(cells.iter().find(|c| c.frame == frame)),
        (Some(8), _) => byref(None),
        (_, Some(Held::Stack(_))) | (_, Some(Held::Addr(_))) => decode_ivar_type("^v", tctx),
        _ => decode_ivar_type(
            match (store.width, store.float) {
                (8, true) => "d",
                (4, true) => "f",
                (4, false) => "i",
                (16, _) => "[16C]",
                _ => "q",
            },
            tctx,
        ),
    }
}

/// The struct of a `__block` cell: `isa`, `forwarding` (its own type), `flags`,
/// `size`, the copy/dispose helpers when `flags` says so, then the value as
/// wide as the store that initialized it (`Block_byref_<size>_<width>`); with
/// no cell found, the bare header `Block_byref`.
fn byref_layout(cell: Option<&ByrefCell>, tctx: &TypeCtx) -> Option<Rc<Datatype>> {
    let types: &dyn TypeFactory = tctx.types;
    let ps = tctx.ptr_size;
    let (name, size, helpers, value) = match cell {
        Some(c) if c.size as i32 >= 3 * ps && c.size <= 1 << 16 => {
            let value = c.value.map(|s| (s.width, s.float)).unwrap_or((8, false));
            (format!("Block_byref_{}_{}{}", c.size, value.0, if value.1 { "f" } else { "" }), c.size as i32, c.flags & BLOCK_HAS_COPY_DISPOSE != 0, Some(value))
        }
        _ => ("Block_byref".to_string(), 3 * ps, false, None),
    };
    if let Some(t) = types.find_by_name(&name).ok().flatten() {
        return (t.get_metatype() == type_metatype::TYPE_STRUCT).then_some(t);
    }
    let shell = types.kuna_declare_record(&name, false).ok()?;
    let voidp = decode_ivar_type("^v", tctx)?;
    let int = decode_ivar_type("i", tctx)?;
    let me = types.get_type_pointer(ps, Rc::clone(&shell), tctx.word_size).ok()?;
    let mut fields = vec![
        TypeField::new(0, 0, "isa".to_string(), Rc::clone(&voidp)),
        TypeField::new(ps, ps, "forwarding".to_string(), me),
        TypeField::new(2 * ps, 2 * ps, "flags".to_string(), Rc::clone(&int)),
        TypeField::new(2 * ps + 4, 2 * ps + 4, "size".to_string(), int),
    ];
    let mut at = 3 * ps;
    if helpers {
        fields.push(TypeField::new(at, at, "copy".to_string(), Rc::clone(&voidp)));
        fields.push(TypeField::new(at + ps, at + ps, "dispose".to_string(), voidp));
        at += 2 * ps;
    }
    if let Some((width, float)) = value {
        let code = match (width, float) { (8, true) => "d", (4, true) => "f", (4, false) => "i", (16, _) => "[16C]", _ => "q" };
        let ty = decode_ivar_type(code, tctx)?;
        let align = (width as i32).min(ps);
        at = (at + align - 1) / align * align;
        if at + width as i32 <= size {
            fields.push(TypeField::new(at, at, "value".to_string(), ty));
        }
    }
    types.set_fields_struct_raw(&shell, fields, Vec::new(), size, ps, 0).ok()
}

/// The struct of one block with captures: `Block_layout` at 0, then one
/// field per captured value — typed by the copy helper (an object, a block, a
/// `__block` cell) or by the store that filled it — up to the descriptor's
/// `size` (`Block_layout_<invoke>`). The plain `Block_layout` for a block that
/// captures nothing.
fn captured_layout(
    img: &MachoImage,
    site: &Site,
    kinds: &[(u64, u32)],
    cells: &[ByrefCell],
    tctx: &TypeCtx,
    layout: &Rc<Datatype>,
) -> Rc<Datatype> {
    let ps = tctx.ptr_size;
    let Some(size) = site.descriptor.and_then(|d| img.read_ptr(d.wrapping_add(8))).map(|s| s as i32) else { return Rc::clone(layout) };
    if size <= 4 * ps || size > 1 << 16 {
        return Rc::clone(layout);
    }
    let name = format!("Block_layout_{:x}", site.invoke);
    let types: &dyn TypeFactory = tctx.types;
    if let Some(t) = types.find_by_name(&name).ok().flatten() {
        return t;
    }
    let mut fields = vec![TypeField::new(0, 0, "block".to_string(), Rc::clone(layout))];
    let mut end = 4 * ps;
    for (k, store) in site.captures.iter().enumerate() {
        let off = store.off.wrapping_sub(site.frame) as i32;
        if off < end || off + store.width as i32 > size {
            continue;
        }
        let kind = kinds.iter().find(|(o, _)| *o as i32 == off).map(|(_, k)| *k);
        let Some(ty) = capture_type(store, kind, cells, tctx, layout) else { continue };
        fields.push(TypeField::new(off, off, format!("capture_{k}"), ty));
        end = off + store.width as i32;
    }
    types
        .kuna_declare_record(&name, false)
        .ok()
        .and_then(|shell| types.set_fields_struct_raw(&shell, fields, Vec::new(), size, ps, 0).ok())
        .unwrap_or_else(|| Rc::clone(layout))
}

/// The invoke function's prototype from the block signature
/// (`q16@?0q8` → `long f(Block_layout *block, long arg1)`).
fn invoke_prototype(name: &str, signature: &str, tctx: &TypeCtx, layout: &Rc<Datatype>) -> Option<PrototypePieces> {
    let (ret, args) = split_method(signature)?;
    if args.first().map(String::as_str) != Some("@?") {
        return None;
    }
    let mut p = PrototypePieces {
        name: name.to_string(),
        outtype: Some(decode_ivar_type(&ret, tctx)?),
        first_var_arg_slot: -1,
        ..Default::default()
    };
    p.intypes.push(tctx.types.get_type_pointer(tctx.ptr_size, Rc::clone(layout), tctx.word_size).ok()?);
    p.innames.push("block".into());
    for (i, a) in args.iter().enumerate().skip(1) {
        p.intypes.push(decode_ivar_type(a, tctx)?);
        p.innames.push(format!("arg{i}"));
    }
    Some(p)
}

/// A global block literal and its invoke function.
pub struct GlobalBlock {
    pub literal: u64,
    pub invoke: u64,
}

/// Type every global block literal of `file` and its invoke function. The
/// invoke is named by [`stack_blocks`], after the function that references
/// the literal.
pub fn emit(img: &MachoImage, file: &object::File, tctx: &TypeCtx, out: &mut AnalysisOutput) -> Vec<GlobalBlock> {
    let ps = img.ptr_size();
    let mut layout = None;
    let mut found = Vec::new();
    for sec in file.sections() {
        if sec.name().ok() != Some("__const") {
            continue;
        }
        for i in 0..sec.size() / ps {
            let va = sec.address() + i * ps;
            if img.bind_name(va).map(|n| n.trim_start_matches('_')) != Some("NSConcreteGlobalBlock") {
                continue;
            }
            let Some(invoke) = img.rebase_ptr(va + 2 * ps) else { continue };
            let Some(layout) = layout.get_or_insert_with(|| block_layout(tctx)).as_ref() else { return found };
            out.typed_data.push(TypedDataFact {
                addr: va,
                name: format!("block_literal_{va:x}"),
                type_: Rc::clone(layout),
            });
            found.push(GlobalBlock { literal: va, invoke });
            let label = format!("block_invoke_{invoke:x}");
            let Some(desc) = img.rebase_ptr(va + 3 * ps) else { continue };
            let Some((sig, _)) = descriptor_signature(img, desc) else { continue };
            if let Some(p) = invoke_prototype(&label, &sig, tctx, layout) {
                out.prototypes_at.push((invoke, p));
            }
        }
    }
    found.sort_by_key(|g| g.literal);
    found
}

/// The prefix Clang names a block's invoke from: `__` and the creator, an
/// Objective-C method with its length first (`__16-[A b:]`), or the creator's
/// own prefix when the creator is itself an invoke (`__f_block_invoke_2` → `__f`:
/// a nested block counts on under the outermost function).
fn block_base(creator: &str) -> String {
    if let Some(i) = creator.find("_block_invoke") {
        if creator.starts_with("__") && i > 2 {
            return creator[..i].to_string();
        }
    }
    if creator.starts_with("-[") || creator.starts_with("+[") {
        format!("__{}{}", creator.len(), creator)
    } else {
        format!("__{creator}")
    }
}

/// The next invoke name under `base`: `<base>_block_invoke`, then `_2`, `_3`,
/// skipping a name the image already has.
fn next_label(base: &str, counters: &mut HashMap<String, u32>, taken: &mut HashSet<String>) -> String {
    let n = counters.entry(base.to_string()).or_default();
    loop {
        *n = n.saturating_add(1);
        let label = if *n == 1 { format!("{base}_block_invoke") } else { format!("{base}_block_invoke_{n}") };
        if taken.insert(label.clone()) || *n == u32::MAX {
            return label;
        }
    }
}

/// Whether the image lists its function starts (`LC_FUNCTION_STARTS`): the
/// code between two starts is then one function.
fn lists_function_starts(file: &object::File) -> bool {
    let object::File::MachO64(macho) = file else { return false };
    let Ok(mut commands) = macho.macho_load_commands() else { return false };
    while let Ok(Some(command)) = commands.next() {
        if command.cmd() == object::macho::LC_FUNCTION_STARTS {
            let table = command.data::<object::macho::LinkeditDataCommand<object::Endianness>>();
            return table.is_ok_and(|t| t.datasize.get(macho.endian()) > 0);
        }
    }
    false
}

/// One block whose invoke gets a name: a stack literal (its index in the
/// sites) or a global literal's first reference.
enum Built {
    Stack(usize),
    Global { invoke: u64, pc: u64, last_ret: u64, start: Option<u64> },
}

/// Name every stack block's invoke function after the function that builds it
/// and type it from the block descriptor's signature, its captures from the
/// copy helper and the creator's stores, a `__block` cell from the frame it
/// was built on. A global block's invoke is named after the first function
/// that references the literal, every name follows Clang's
/// (`__<len><creator>_block_invoke_<n>`), and an invoke the image already
/// names keeps its name.
pub fn stack_blocks(
    img: &MachoImage,
    file: &object::File,
    bytes: &[u8],
    tctx: &TypeCtx,
    creators: &[(u64, String)],
    globals: &[GlobalBlock],
    out: &mut AnalysisOutput,
) {
    let symbolized: HashSet<u64> = creators.iter().map(|(a, _)| *a).collect();
    let mut named: HashSet<u64> = HashSet::new();
    let fallback = |globals: &[GlobalBlock], named: &HashSet<u64>, out: &mut AnalysisOutput| {
        for g in globals.iter().filter(|g| !symbolized.contains(&g.invoke) && !named.contains(&g.invoke)) {
            let label = format!("block_invoke_{:x}", g.invoke);
            out.symbols.push(SymFact { addr: g.invoke, name: label.clone(), kind: SymKind::Function });
            out.fid_names.push(FidMatch { addr: g.invoke, name: label, by_signature: false });
        }
    };
    let arch = file.architecture();
    if !matches!(arch, object::Architecture::Aarch64 | object::Architecture::X86_64) {
        return fallback(globals, &named, out);
    }
    let ps = img.ptr_size();
    let mut isa_slots = Vec::new();
    for sec in file.sections() {
        if !sec.name().ok().is_some_and(|n| n.ends_with("got") || n.ends_with("symbol_ptr")) {
            continue;
        }
        for i in 0..sec.size() / ps {
            let va = sec.address() + i * ps;
            if img.bind_name(va).map(|n| n.trim_start_matches('_')) == Some("NSConcreteStackBlock") {
                isa_slots.push(va);
                if let Some(voidp) = decode_ivar_type("^v", tctx) {
                    out.typed_data.push(TypedDataFact { addr: va, name: "NSConcreteStackBlock_ptr".to_string(), type_: voidp });
                }
            }
        }
    }
    let global_lits: Vec<u64> = globals.iter().map(|g| g.literal).collect();
    if isa_slots.is_empty() && global_lits.is_empty() {
        return fallback(globals, &named, out);
    }
    let text: Vec<(u64, &[u8])> = file
        .sections()
        .filter(|s| s.name().ok() == Some("__text"))
        .filter_map(|s| Some((s.address(), s.data().ok()?)))
        .collect();
    let runtime: HashMap<u64, String> = crate::loader::macho_stubs::resolve_macho_imports(file, bytes)
        .into_iter()
        .filter_map(|i| Some((i.addr, String::from_utf8(i.name).ok()?)))
        .collect();
    let mut starts = crate::analyzers::entry::macho_function_starts(file, bytes);
    starts.sort_unstable();
    starts.dedup();
    let mut creators: Vec<(u64, String)> = creators
        .iter()
        .cloned()
        .collect();
    creators.sort();
    let mut taken: HashSet<String> = creators.iter().map(|(_, n)| n.clone()).collect();
    let read_ptr = |va: u64| img.read_ptr(va);
    let in_text = |a: u64| text.iter().any(|(base, data)| a >= *base && a < base + data.len() as u64);
    let read_only: Vec<(u64, u64)> = file
        .sections()
        .filter(|s| s.segment_name().ok().flatten() == Some("__TEXT"))
        .map(|s| (s.address(), s.address().saturating_add(s.size())))
        .collect();
    let mut pools: Vec<u64> = Vec::new();
    let found = match arch {
        object::Architecture::X86_64 => {
            let mut bounds: Vec<u64> = starts.iter().chain(creators.iter().map(|(a, _)| a)).copied().collect();
            bounds.sort_unstable();
            bounds.dedup();
            x86::sites(&read_ptr, &in_text, &text, &isa_slots, &global_lits, &bounds, lists_function_starts(file))
        }
        _ => stack_block_sites(&read_ptr, &in_text, &text, &isa_slots, &global_lits, &starts),
    };
    let (mut sites, cells) = (found.sites, found.cells);
    sites.sort_by_key(|s| s.pc);
    let mut built: Vec<(u64, Built)> = sites.iter().enumerate().map(|(i, s)| (s.pc, Built::Stack(i))).collect();
    let mut refs = found.global_refs;
    refs.sort_unstable_by_key(|r| (r.1, r.0));
    let mut seen_lit: HashSet<u64> = HashSet::new();
    for (lit, pc, last_ret, start) in refs {
        let Ok(at) = globals.binary_search_by_key(&lit, |g| g.literal) else { continue };
        if seen_lit.insert(lit) {
            built.push((pc, Built::Global { invoke: globals[at].invoke, pc, last_ret, start }));
        }
    }
    built.sort_by_key(|(pc, b)| (*pc, matches!(b, Built::Global { .. })));
    let mut layout = None;
    let mut counters: HashMap<String, u32> = HashMap::new();
    let mut per_func: HashMap<u64, u32> = HashMap::new();
    let mut descriptors: HashSet<u64> = HashSet::new();
    let mut locals: Vec<LocalFact> = Vec::new();
    for (_, b) in built {
        let (invoke, pc, last_ret, start) = match b {
            Built::Stack(i) => (sites[i].invoke, sites[i].pc, sites[i].last_ret, sites[i].start),
            Built::Global { invoke, pc, last_ret, start } => (invoke, pc, last_ret, start),
        };
        if let Built::Stack(i) = b {
            // The invoke is code and the descriptor is not; a site that reads
            // otherwise was built from stale registers.
            let site = &sites[i];
            if !in_text(site.invoke) || site.descriptor.is_some_and(in_text) {
                continue;
            }
            // A flags word loaded from a read-only pool is a constant: folded,
            // the literal reads `flags = 0xc2000000` and not a load of the pool.
            // The refcount half of a literal's flags is always zero.
            if let Some(at) = site.flags_at {
                let ro = read_only.iter().any(|&(lo, hi)| at >= lo && at.saturating_add(8) <= hi);
                if ro && img.read_ptr(at).is_some_and(|v| v & 0xFFFF == 0) {
                    pools.push(at);
                }
            }
        }
        // The linker folds identical invoke bodies: the first creator names it.
        if !named.insert(invoke) {
            continue;
        }
        // The creator is the nearest label at or before the store, unless a
        // `ret` or a function start lies between them: then the store is in an
        // unlabelled function, `sub_<start>` when its start is known.
        let (creator, creator_start) = match creators.iter().rev().find(|(s, _)| *s <= pc) {
            Some((s, name)) if *s > last_ret && start.is_none_or(|f| *s >= f) => (name.clone(), Some(*s)),
            _ => match start {
                Some(f) => (format!("sub_{f:x}"), Some(f)),
                _ => (format!("block_{invoke:x}"), None),
            },
        };
        let label = if symbolized.contains(&invoke) {
            creators.iter().find(|(a, _)| *a == invoke).map(|(_, n)| n.clone()).unwrap_or_default()
        } else {
            let label = next_label(&block_base(&creator), &mut counters, &mut taken);
            out.symbols.push(SymFact { addr: invoke, name: label.clone(), kind: SymKind::Function });
            out.fid_names.push(FidMatch { addr: invoke, name: label.clone(), by_signature: false });
            label
        };
        // The invoke is a function of its own: a block built inside it is its
        // block (`…_block_invoke_2`), not the outer creator's.
        if let Err(at) = creators.binary_search_by_key(&invoke, |(a, _)| *a) {
            creators.insert(at, (invoke, label.clone()));
        }
        let Built::Stack(i) = b else { continue };
        let site = &mut sites[i];
        // The literal ends where its descriptor says; what the creator stored
        // past that is its own (a `__block` cell, another local).
        if let Some(size) = site.descriptor.and_then(|d| img.read_ptr(d.wrapping_add(8))) {
            let frame = site.frame;
            site.captures.retain(|s| s.off.wrapping_sub(frame) < size);
        }
        let site = &sites[i];
        let Some((sig, helpers)) = site.descriptor.and_then(|d| descriptor_signature(img, d)) else { continue };
        let Some(layout) = layout.get_or_insert_with(|| block_layout(tctx)).as_ref() else { return };
        let kinds = match (helpers, site.descriptor.and_then(|d| img.rebase_ptr(d.wrapping_add(16)))) {
            (true, Some(copy)) if arch == object::Architecture::X86_64 => x86::copy_helper_kinds(img, copy, &runtime),
            (true, Some(copy)) => copy_helper_kinds(img, copy, &runtime),
            _ => Vec::new(),
        };
        let this = captured_layout(img, site, &kinds, &cells, tctx, layout);
        // Blocks of one shape share a descriptor: it is named by its address.
        if let Some(desc) = site.descriptor.filter(|d| descriptors.insert(*d)) {
            if let Some(ty) = descriptor_layout(tctx, helpers) {
                out.typed_data.push(TypedDataFact { addr: desc, name: format!("block_descriptor_{desc:x}"), type_: ty });
            }
        }
        if let Some(p) = invoke_prototype(&label, &sig, tctx, &this) {
            out.prototypes_at.push((site.invoke, p));
        }
        // The copy and dispose helpers, named after the invoke.
        if helpers {
            for (at, what) in [(16u64, "copy_helper"), (24, "dispose_helper")] {
                let Some(h) = site.descriptor.and_then(|d| img.rebase_ptr(d.wrapping_add(at))) else { continue };
                if in_text(h) && !symbolized.contains(&h) && named.insert(h) {
                    let name = format!("{label}_{what}");
                    out.symbols.push(SymFact { addr: h, name: name.clone(), kind: SymKind::Function });
                    out.fid_names.push(FidMatch { addr: h, name, by_signature: false });
                }
            }
        }
        // The creator's frame: the literal and each `__block` cell it captures
        // are typed locals, so the build reads as field stores.
        let Some(func_addr) = creator_start else { continue };
        let blocks_here = {
            let n = per_func.entry(func_addr).or_default();
            *n += 1;
            *n
        };
        let local = |name: String, type_: Rc<Datatype>, frame: u64, depth: u64| LocalFact {
            func_addr,
            name,
            type_,
            stack_offset: (frame as i64).wrapping_sub(depth as i64),
        };
        locals.push(local(if blocks_here == 1 { "block".to_string() } else { format!("block_{blocks_here}") }, this, site.frame, site.depth));
        for (k, store) in site.captures.iter().enumerate() {
            let Some(Held::Stack(frame)) = store.held else { continue };
            let Some(cell) = cells.iter().find(|c| c.frame == frame) else { continue };
            let Some(ty) = byref_layout(Some(cell), tctx) else { continue };
            let name = if blocks_here == 1 { format!("cell_{k}") } else { format!("cell_{blocks_here}_{k}") };
            locals.push(local(name, ty, cell.frame, cell.depth));
        }
    }
    fallback(globals, &named, out);
    pools.sort_unstable();
    pools.dedup();
    out.const_ranges.extend(pools.into_iter().map(|first| ConstRange { first, last_open: first + 8, gate: ConstGate::ObjcRefs }));
    // A frame slot one creator reuses for two literals cannot be two locals;
    // neither is declared, the invokes keep their types.
    let overlaps = |a: &LocalFact, b: &LocalFact| {
        a.func_addr == b.func_addr
            && a.stack_offset < b.stack_offset + b.type_.get_size() as i64
            && b.stack_offset < a.stack_offset + a.type_.get_size() as i64
    };
    let keep: Vec<bool> = locals.iter().enumerate().map(|(i, a)| !locals.iter().enumerate().any(|(j, b)| i != j && overlaps(a, b))).collect();

    out.locals.extend(locals.into_iter().zip(keep).filter(|(_, k)| *k).map(|(l, _)| l));
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_decomp::dtype::TypeFactoryImpl;

    /// `-[Runner maker:]` at 0x1000: `adrp x9, got; ldr x9, [x9, #0x10]` (the
    /// `_NSConcreteStackBlock` slot at 0x8010), `str x9, [sp, #8]`, then the
    /// invoke and descriptor materialized and stored as a pair at `[sp, #24]`,
    /// a double capture at `[sp, #40]`, an int at `[sp, #48]`, and a `__block`
    /// cell at `[sp, #64]` (`isa = 0, forwarding = &cell, flags|size, value`)
    /// whose address is captured at `[sp, #56]`.
    #[test]
    fn stack_block_stores_name_the_invoke_captures_and_cell() {
        let words: [u32; 18] = [
            0x9000_0009, // adrp x9, page (imm 0: the page of pc, 0x1000)
            0xF940_0929, // ldr x9, [x9, #0x10]
            0xF900_07E9, // str x9, [sp, #8]
            0x9000_0009, // adrp x9, page
            0x9100_8129, // add x9, x9, #0x20
            0x9000_000A, // adrp x10, page
            0x9101_014A, // add x10, x10, #0x40
            0xA901_ABE9, // stp x9, x10, [sp, #24]
            0xFD00_17E0, // str d0, [sp, #40]
            0xB900_33E8, // str w8, [sp, #48]
            0x9101_03E8, // add x8, sp, #64
            0xA904_23FF, // stp xzr, x8, [sp, #64]
            0xD2C0_0409, // movz x9, #0x20, lsl #32  (size 32, flags 0)
            0xF900_2BE9, // str x9, [sp, #80]
            0xF900_2FFF, // str xzr, [sp, #88]
            0xF900_1FE8, // str x8, [sp, #56]
            0xD65F_03C0, // ret
            0xD503_201F, // nop
        ];
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        let code = |a: u64| (0x1000..0x2000).contains(&a);
        let found = stack_block_sites(&|_| None, &code, &[(0x1000, &bytes)], &[0x1010], &[], &[]);
        let (sites, cells) = (found.sites, found.cells);
        assert_eq!(sites.len(), 1);
        let s = &sites[0];
        assert_eq!((s.pc, s.frame, s.invoke, s.descriptor, s.last_ret), (0x1008, 8, 0x1020, None, 0), "0x1040 is code, not a descriptor");
        let caps: Vec<(u64, u8, bool, Option<Held>)> = s.captures.iter().map(|c| (c.off - s.frame, c.width, c.float, c.held)).collect();
        assert_eq!(caps[0], (32, 8, true, None), "the double at +32");
        assert_eq!(caps[1], (40, 4, false, None), "the int at +40");
        assert_eq!(caps[2], (48, 8, false, Some(Held::Stack(64))), "the cell's address at +48");
        assert_eq!(cells, vec![ByrefCell { frame: 64, depth: 0, flags: 0, size: 32, value: Some(Store { off: 88, width: 8, float: false, held: Some(Held::Imm(0)) }) }]);
        assert!(stack_block_sites(&|_| None, &code, &[(0x1000, &bytes)], &[0x9999], &[], &[]).sites.is_empty(), "another slot is not the isa");
    }

    /// A function ending in a tail call (`b`) does not hand its frame depth to
    /// the next function start; an arm64e `retab` ends a function; a later
    /// store over the literal's invoke slot (the frame reused) keeps the first.
    #[test]
    fn function_starts_bound_the_scan_and_the_first_invoke_wins() {
        let words: [u32; 14] = [
            0xD101_03FF, // sub sp, sp, #0x40
            0x1400_0000, // b . (tail call)
            0xD100_83FF, // sub sp, sp, #0x20      <- the next function at 0x1008
            0x9000_0009, // adrp x9, page
            0xF940_0929, // ldr x9, [x9, #0x10]   (the isa slot)
            0xF900_03E9, // str x9, [sp]
            0x9000_0009, // adrp x9, page
            0x9101_0129, // add x9, x9, #0x40     (the invoke, code)
            0x9000_000A, // adrp x10, page
            0x9108_014A, // add x10, x10, #0x200  (the descriptor, data)
            0xA901_2BE9, // stp x9, x10, [sp, #16]
            0x9100_2129, // add x9, x9, #0x8
            0xA901_2BE9, // stp x9, x10, [sp, #16]
            0xD65F_0FFF, // retab
        ];
        let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        let code = |a: u64| (0x1000..0x1100).contains(&a);
        let found = stack_block_sites(&|_| None, &code, &[(0x1000, &bytes)], &[0x1010], &[], &[0x1000, 0x1008]);
        assert_eq!(found.sites.len(), 1);
        let s = &found.sites[0];
        assert_eq!((s.start, s.depth, s.frame, s.invoke, s.descriptor), (Some(0x1008), 0x20, 0, 0x1040, Some(0x1200)));
        let unbounded = stack_block_sites(&|_| None, &code, &[(0x1000, &bytes)], &[0x1010], &[], &[]);
        assert_eq!(unbounded.sites[0].depth, 0x60, "without the start the frames add up");
        assert!(is_ret(0xD65F_03C0) && is_ret(0xD65F_0BFF) && is_ret(0xD65F_0FFF));
    }

    /// Clang's invoke names: `__<len><method>`, `__<function>`, a nested
    /// block on under the outermost function, a taken name skipped.
    #[test]
    fn invoke_names_follow_clang() {
        assert_eq!(block_base("-[Chain build:]"), "__15-[Chain build:]");
        assert_eq!(block_base("sum"), "__sum");
        assert_eq!(block_base("__sum_block_invoke"), "__sum");
        assert_eq!(block_base("__15-[Chain build:]_block_invoke_3"), "__15-[Chain build:]");
        let (mut counters, mut taken) = (HashMap::new(), HashSet::from(["__f_block_invoke_2".to_string()]));
        let names: Vec<String> = (0..3).map(|_| next_label("__f", &mut counters, &mut taken)).collect();
        assert_eq!(names, ["__f_block_invoke", "__f_block_invoke_3", "__f_block_invoke_4"]);
    }

    #[test]
    fn signature_names_the_block_first() {
        let f = TypeFactoryImpl::new();
        f.set_default_alignment_map();
        f.set_max_basetype_size(8);
        f.setup_sizes(Some(8), 8, 4);
        let _ = f.cache_core_types();
        let tctx = TypeCtx::new(&f, 8, 1);
        let layout = block_layout(&tctx).unwrap();
        assert_eq!(layout.get_size(), 32);
        let p = invoke_prototype("block_invoke_1", "q16@?0q8", &tctx, &layout).unwrap();
        assert_eq!(p.innames, ["block", "arg1"]);
        assert_eq!(p.intypes[0].get_ptr_to().unwrap().get_name(), "Block_layout");
        assert!(invoke_prototype("b", "v16@0:8", &tctx, &layout).is_none(), "a method signature is not a block's");
    }
}
