//! (kuna devirtualize) Devirtualization of VM-protected functions: the
//! `devirtualize` technique of `option deobfuscate`.
//!
//! # The algorithm is Pushan's
//!
//! This technique is a port of **Pushan**: Sudhir, Basque, Gibbs et al.,
//! "Pushan: Trace-Free Deobfuscation of Virtualization-Obfuscated Binaries",
//! NDSS 2027, and of its reference implementation, angr PR #7680
//! (`VMDeobfuscation` / `CFGVMDeobfuscation`).  The code is named for what the
//! technique does, but the method is the paper's and none of it is kuna's
//! invention: the interpreter is emulated symbolically with no path
//! constraints, every block is keyed by its native address and the virtual
//! program counter (VPC sensitivity), locations that differ where paths meet
//! are symbolized over further rounds until the graph is stable, and the
//! protected program's own control flow comes out as a flat graph that an
//! ordinary decompiler then simplifies.  What is kuna's is the carrier: p-code
//! in place of VEX, an evaluation-based identity test in place of the solver,
//! and the synthetic-overlay emission.  Anyone changing the exploration
//! ([`kuna_devirtflow`](super::kuna_devirtflow)), the emission
//! ([`kuna_devirtemit`](super::kuna_devirtemit)) or the residue rules
//! ([`kuna_residuerules`](super::kuna_residuerules)) should read the paper
//! first; `docs/spec/10-deobfuscation.md` §10.2 maps its stages onto this code.
//!
//! # What it does
//!
//! A function whose entry jumps into the interpreter of a VM-based protector
//! (VMProtect, Themida) decompiles, by default, to the interpreter's dispatch
//! loop or to a two-line stub.  With this on, the flow follower is handed a
//! synthetic program instead: the protected function's own control flow,
//! recovered by the exploration and emitted as p-code with the interpreter's
//! constant machinery folded away.  The rest of the decompiler (SSA, dead
//! code, stack variables, types, structuring) does the paper's stage 2 and 3
//! on that program unchanged.
//!
//! It is a member of the deobfuscation plane
//! ([`kuna_deobf`](super::kuna_deobf)): [`Devirtualize`] recovers the body at
//! the front, or after the plain flow for an interpreter that is the
//! function's own body, states the body's facts and analysis policy, and
//! audits the decompiled result.
//!
//! Under `deobfuscate auto` (the default) it acts on a function whose direct
//! native flow reaches a branch or call into a separate executable section of
//! at least 64 KiB, the stub a protector leaves at the original entry, and on
//! one whose plain flow is an interpreter over file data.  Named
//! (`deobfuscate devirtualize`), it treats the requested function as
//! protected regardless.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::kuna_pcodeoverlay::PcodeOverlay;
use kuna_sleigh::loadimage::{section_flags, LoadImageSection};
use kuna_sleigh::translate::{PcodeEmit, UniqueLayout};

use super::kuna_devirtemit::{emit, EmitConfig, EmitStats};
use super::kuna_symemu::{classify_space, LiftedInsn, SymTarget, RawOp, Space};
use super::kuna_devirtflow::{Abi, Config, Explorer, NodeExit, Stats};
use super::kuna_symstate::SymImage;
use super::kuna_body::{BodyFacts, BodyPolicy, RecoveredBody};
use super::kuna_deobf::{Deobfuscator, Engage};
use crate::architecture::Architecture;
use crate::fspec::EXTRAPOP_UNKNOWN;
use crate::funcdata::Funcdata;

/// The technique's name in `option deobfuscate`.
pub const NAME: &str = "devirtualize";

/// The devirtualized program for one function entry.
pub struct DevirtProgram {
    pub entry: u64,
    /// Registers (offset, size) an external call leaves undefined.
    pub call_kills: Vec<(u64, u8)>,
    /// The placeholder registers the emitter reads for "never defined" values.
    pub placeholders: Vec<(u64, u8)>,
    /// Registers (offset, size) every external call leaves untouched: the
    /// interpreter's own memory, privatized by the emitter.
    pub call_preserves: Vec<(u64, u32)>,
    /// Entry-stack offsets whose address the program handed to an external
    /// call (in a register or an outgoing stack slot), over every round.
    pub escapes: Vec<i64>,
    pub overlay: Rc<PcodeOverlay>,
    /// Read-only string literals the program refers to by a constant pointer:
    /// (address, bytes including the terminator).
    pub strings: Vec<(u64, u32)>,
    /// The protector's own sections: the candidate interpreter sections
    /// ([`Section::vm_candidate`]) the exploration ran VM code in.
    pub protector: Vec<(u64, u64)>,
    /// Import slots the exploration resolved a call or a tail call through.
    pub slots: Vec<u64>,
    pub stats: Stats,
    pub emit_stats: EmitStats,
    pub log: Vec<String>,
}

struct Section {
    start: u64,
    end: u64,
    flags: u32,
    /// The section's name, when the format has one.
    name: String,
}

impl Section {
    /// A section a protector's interpreter may live in: loaded code of at
    /// least [`MIN_VM_SECTION`] bytes that no compiler or linker emits for
    /// every program ([`toolchain_code_section`]).
    fn vm_candidate(&self) -> bool {
        self.flags & section_flags::CODE != 0
            && self.flags & section_flags::NOLOAD == 0
            && self.end - self.start >= MIN_VM_SECTION
            && !toolchain_code_section(&self.name)
    }
}

/// A code section the toolchain emits, never a protector's: ELF PLTs,
/// init/fini code and every `.text*` split (`.text.unlikely`, `.text.hot`),
/// Mach-O text and stub sections, and a Windows driver's `PAGE*`/`INIT`.
pub(crate) fn toolchain_code_section(name: &str) -> bool {
    name.starts_with(".plt")
        || name.to_ascii_lowercase().contains("text")
        || matches!(name, ".iplt" | ".init" | ".fini" | "__stubs" | "__stub_helper" | "__auth_stubs" | "INIT")
        || name.starts_with("PAGE")
}

/// A fixed, benign thread environment block for a Windows image: the
/// protector's checks against the TEB and PEB (debugger flags, heap flags,
/// OS version, the block's own address) read the values an ordinary process
/// sees, so they fold instead of forking the exploration on an unknown.
#[derive(Clone, Debug)]
struct ThreadBlock {
    base: u64,
    ptr: u8,
    image_base: u64,
}

const TEB_SPAN: u64 = 0x4000;

impl ThreadBlock {
    fn new(ptr: u8, image_base: u64) -> ThreadBlock {
        let base = if ptr == 8 { 0x7ff6_0000_0000 } else { 0x7ffd_c000 };
        ThreadBlock { base, ptr, image_base }
    }
    fn peb(&self) -> u64 {
        self.base + 0x2000
    }
    fn heap(&self) -> u64 {
        self.base + 0x3000
    }
    fn contains(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.base + TEB_SPAN
    }
    /// The (offset, size, value) fields that are not zero.
    fn fields(&self) -> Vec<(u64, u8, u64)> {
        let p = self.ptr;
        let (teb, peb, heap) = (self.base, self.peb(), self.heap());
        if p == 8 {
            vec![
                (teb + 0x30, 8, teb),
                (teb + 0x40, 8, 0x1000),
                (teb + 0x48, 8, 0x1004),
                (teb + 0x60, 8, peb),
                (peb + 0x10, 8, self.image_base),
                (peb + 0x30, 8, heap),
                (peb + 0x118, 4, 10),
                (peb + 0x120, 2, 19041),
                (peb + 0x124, 4, 2),
                (heap + 0x70, 4, 2),
            ]
        } else {
            vec![
                (teb + 0x18, 4, teb),
                (teb + 0x20, 4, 0x1000),
                (teb + 0x24, 4, 0x1004),
                (teb + 0x30, 4, peb),
                (peb + 0x08, 4, self.image_base),
                (peb + 0x18, 4, heap),
                (peb + 0xa4, 4, 10),
                (peb + 0xac, 2, 19041),
                (peb + 0xb0, 4, 2),
                (heap + 0x40, 4, 2),
            ]
        }
    }
    fn byte(&self, addr: u64) -> u8 {
        for (off, size, value) in self.fields() {
            if addr >= off && addr < off + size as u64 {
                return (value >> (8 * (addr - off))) as u8;
            }
        }
        0
    }
}

/// The segment base register holding the thread block's address.
fn thread_block_register(arch: &Architecture, ptr: u8) -> Option<u64> {
    let name = if ptr == 8 { "GS_OFFSET" } else { "FS_OFFSET" };
    arch.translate().probe_register_varnode(name.as_bytes()).map(|v| v.offset)
}

/// [`SymTarget`] over the live [`Architecture`].
pub struct ArchTarget<'a> {
    arch: &'a Architecture,
    ram: Rc<AddrSpace>,
    sections: Vec<Section>,
    entry_section: Option<(u64, u64)>,
    /// The synthetic thread block of a Windows image, when modelled.
    teb: Option<ThreadBlock>,
    lifts: RefCell<BTreeMap<u64, Rc<LiftedInsn>>>,
    userops: Vec<String>,
    /// Entropy of each 256-byte block, computed on demand.
    entropy: RefCell<BTreeMap<u64, f64>>,
    /// Entropy of the window around each address [`SymTarget::bytecode_near`] tested.
    near_entropy: RefCell<BTreeMap<u64, f64>>,
    /// A Windows image (`int 0x29` is `__fastfail`).
    windows: bool,
}

struct Collect(Vec<RawOp>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.0.push(RawOp { opc, out: outvar.cloned(), ins: vars.to_vec() });
    }
}

impl<'a> ArchTarget<'a> {
    pub fn new(arch: &'a Architecture, entry: u64) -> Option<Self> {
        let ram = Rc::clone(arch.manage().get_default_code_space()?);
        let mut sections = Vec::new();
        {
            let loader_rc = arch.translate().loader_rc();
            let loader = loader_rc.borrow();
            loader.open_section_info();
            loop {
                let mut rec = LoadImageSection::default();
                let more = loader.get_next_section(&mut rec);
                if !rec.address.is_invalid() && rec.size > 0 {
                    let start = rec.address.get_offset();
                    sections.push(Section { start, end: start.saturating_add(rec.size), flags: rec.flags, name: String::new() });
                }
                if !more {
                    break;
                }
            }
            loader.close_section_info();
            let names = loader.kuna_section_names();
            for s in sections.iter_mut() {
                if let Some((_, _, n)) = names.iter().find(|(v, sz, _)| *v == s.start && v.saturating_add(*sz) == s.end) {
                    s.name = n.clone();
                }
            }
            if sections.is_empty() {
                for (vma, size, flags) in loader.get_segments() {
                    sections.push(Section { start: vma, end: vma.saturating_add(size), flags, name: String::new() });
                }
            }
        }
        let entry_section = sections.iter().find(|s| s.start <= entry && entry < s.end).map(|s| (s.start, s.end));
        let mut userops = Vec::new();
        arch.translate().get_user_op_names(&mut userops);
        let ptr = ram.get_addr_size() as u8;
        let image_base = sections.iter().map(|s| s.start).min().unwrap_or(0) & !0xffff;
        let windows = crate::kuna_fastfailnoreturn::archid_is_windows(&arch.archid);
        let teb = windows.then(|| ThreadBlock::new(ptr, image_base));
        Some(ArchTarget {
            arch,
            ram,
            teb,
            sections,
            entry_section,
            lifts: RefCell::new(BTreeMap::new()),
            userops,
            entropy: RefCell::new(BTreeMap::new()),
            near_entropy: RefCell::new(BTreeMap::new()),
            windows,
        })
    }

    fn section_of(&self, addr: u64) -> Option<&Section> {
        self.sections.iter().find(|s| s.start <= addr && addr < s.end)
    }

    /// The byte length, terminator included, of the string literal at `addr`
    /// in a read-only data section ([`literal_len`]).
    fn string_at(&self, addr: u64, refs: &BTreeSet<u64>) -> Option<u32> {
        let s = self.section_of(addr)?;
        if s.flags & section_flags::READONLY == 0 || s.flags & section_flags::CODE != 0 {
            return None;
        }
        let mut buf = vec![0u8; (s.end - addr).min(STRING_MAX) as usize];
        if !self.read_bytes(addr, &mut buf) {
            return None;
        }
        literal_len(&buf, addr, self.ram.get_addr_size() as usize, |a| self.section_of(a).is_some(), refs)
    }

    /// The bits a user-mode stack address never has set, from the PE headers
    /// mapped at the image base: bit 31 for a 32-bit image without
    /// `IMAGE_FILE_LARGE_ADDRESS_AWARE`, bits 47 to 63 for a 64-bit one. None
    /// for a native-subsystem image (a driver runs on a kernel stack) or when
    /// the headers cannot be read.
    fn pe_user_stack_clear_bits(&self) -> u128 {
        let base = self.sections.iter().map(|s| s.start).min().unwrap_or(0) & !0xffff;
        let mut mz = [0u8; 0x40];
        if !self.read_bytes(base, &mut mz) || &mz[..2] != b"MZ" {
            return 0;
        }
        let pe = base + u32::from_le_bytes([mz[0x3c], mz[0x3d], mz[0x3e], mz[0x3f]]) as u64;
        let mut hdr = [0u8; 0x5e];
        if !self.read_bytes(pe, &mut hdr) || &hdr[..4] != b"PE\0\0" {
            return 0;
        }
        const SUBSYSTEM_NATIVE: u16 = 1;
        if u16::from_le_bytes([hdr[0x5c], hdr[0x5d]]) == SUBSYSTEM_NATIVE {
            return 0;
        }
        let large = u16::from_le_bytes([hdr[0x16], hdr[0x17]]) & 0x0020 != 0;
        match self.ptr_size() {
            8 => 0xffff_8000_0000_0000,
            4 if !large => 0x8000_0000,
            _ => 0,
        }
    }

    /// Highest mapped address, for placing the synthetic range.
    pub fn highest_mapped(&self) -> u64 {
        self.sections.iter().map(|s| s.end).max().unwrap_or(0)
    }
}

impl SymImage for ArchTarget<'_> {
    fn read_bytes(&self, addr: u64, buf: &mut [u8]) -> bool {
        if let Some(teb) = self.teb.as_ref().filter(|t| t.contains(addr)) {
            for (i, b) in buf.iter_mut().enumerate() {
                *b = teb.byte(addr + i as u64);
            }
            return true;
        }
        let loader_rc = self.arch.translate().loader_rc();
        let Ok(mut loader) = loader_rc.try_borrow_mut() else { return false };
        loader.load_fill(buf, &Address::new(Rc::clone(&self.ram), addr)).is_ok()
    }
    fn is_writable(&self, addr: u64) -> bool {
        if self.teb.as_ref().is_some_and(|t| t.contains(addr)) {
            return false;
        }
        match self.section_of(addr) {
            Some(s) => s.flags & section_flags::READONLY == 0 && s.flags & section_flags::CODE == 0,
            None => true,
        }
    }
    /// A writable data section's bytes may have changed before the function
    /// runs (another function wrote them), except at an import slot, which
    /// the loader fills in (a function symbol at the address; Themida's
    /// writable `.idata`).
    fn is_mutable(&self, addr: u64) -> bool {
        if self.section_of(addr).is_none() || !self.is_writable(addr) {
            return false;
        }
        let ps = self.ram.get_addr_size() as u64;
        !self.is_function_entry(addr - addr % ps)
    }
}

impl SymTarget for ArchTarget<'_> {
    fn lift_count(&self) -> usize {
        self.lifts.borrow().len()
    }
    fn lift(&self, addr: u64) -> Result<Rc<LiftedInsn>, String> {
        if let Some(i) = self.lifts.borrow().get(&addr) {
            return Ok(Rc::clone(i));
        }
        let a = Address::new(Rc::clone(&self.ram), addr);
        let mut c = Collect(Vec::new());
        let t = self.arch.translate();
        let lifted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| t.one_instruction(&mut c, &a)));
        let len = match lifted {
            Ok(Ok(len)) => len,
            Ok(Err(e)) => return Err(e.to_string()),
            Err(_) => return Err("translator panic".into()),
        };
        let (mut m, mut b) = (String::new(), String::new());
        let _ = t.print_assembly_into(&a, &mut m, &mut b);
        let asm = if b.is_empty() { m } else { format!("{m} {b}") };
        let insn = Rc::new(LiftedInsn { addr, len: len.max(1) as u32, ops: c.0, asm });
        self.lifts.borrow_mut().insert(addr, Rc::clone(&insn));
        Ok(insn)
    }
    fn ptr_size(&self) -> u8 {
        self.ram.get_addr_size() as u8
    }
    fn space_by_index(&self, index: i32) -> Option<Rc<AddrSpace>> {
        let m = self.arch.manage();
        if index < 0 || index >= m.num_spaces() {
            return None;
        }
        m.get_space(index).cloned()
    }
    fn is_foreign_code(&self, addr: u64) -> bool {
        match self.section_of(addr) {
            Some(s) => s.flags & section_flags::CODE != 0 && Some((s.start, s.end)) != self.entry_section,
            None => false,
        }
    }
    fn is_executable(&self, addr: u64) -> bool {
        match self.section_of(addr) {
            Some(s) => s.flags & section_flags::CODE != 0,
            None => self.sections.is_empty() && self.is_mapped(addr),
        }
    }
    fn is_mapped(&self, addr: u64) -> bool {
        if self.section_of(addr).is_some() {
            return true;
        }
        let mut b = [0u8; 1];
        self.read_bytes(addr, &mut b)
    }
    fn userop_name(&self, index: u64) -> Option<String> {
        self.userops.get(index as usize).cloned()
    }
    fn is_function_entry(&self, addr: u64) -> bool {
        let a = Address::new(Rc::clone(&self.ram), addr);
        self.arch.symboltab.find_function_across_scopes(&a).is_some()
    }
    fn callee_cleans(&self) -> bool {
        self.ram.get_addr_size() == 4 && self.arch.default_fp().is_some_and(|m| m.get_extra_pop() == EXTRAPOP_UNKNOWN)
    }
    fn declared_pop(&self, addr: u64) -> Option<u32> {
        let a = Address::new(Rc::clone(&self.ram), addr);
        let (sid, _) = self.arch.symboltab.find_function_across_scopes(&a)?;
        let sym = self.arch.symboltab.symbol(sid);
        if let Some(m) = self.arch.function_prototype_model(&sym.name) {
            let ep = m.get_extra_pop();
            if ep != EXTRAPOP_UNKNOWN {
                return Some((ep as u32).saturating_sub(self.ram.get_addr_size()));
            }
        }
        let p = self.arch.symboltab.function_proto_pieces_of(sid)?;
        if p.first_var_arg_slot >= 0 {
            return Some(0);
        }
        Some(p.intypes.iter().map(|t| (t.get_size().max(0) as u32 + 3) & !3).sum())
    }
    fn user_stack_clear_bits(&self) -> u128 {
        if self.windows {
            self.pe_user_stack_clear_bits()
        } else {
            0
        }
    }
    fn noreturn_interrupt(&self, vector: u64) -> bool {
        self.windows && vector == crate::kuna_fastfailnoreturn::FASTFAIL_VECTOR as u64
    }
    fn file_data(&self, addr: u64) -> bool {
        self.section_of(addr).is_some_and(|s| s.flags & (section_flags::CODE | section_flags::NOLOAD | section_flags::UNALLOC) == 0)
    }
    fn noreturn_function(&self, addr: u64) -> bool {
        self.arch.symboltab.function_is_no_return_across_scopes(&Address::new(Rc::clone(&self.ram), addr))
    }
    fn copy_routine(&self, addr: u64) -> bool {
        let a = Address::new(Rc::clone(&self.ram), addr);
        let Some((sid, _)) = self.arch.symboltab.find_function_across_scopes(&a) else { return false };
        copy_routine_name(&self.arch.symboltab.symbol(sid).name)
    }
    fn bytecode_like(&self, addr: u64) -> bool {
        if self.section_of(addr).is_none() {
            return false;
        }
        let block = addr & !0xFF;
        if let Some(h) = self.entropy.borrow().get(&block) {
            return *h >= BYTECODE_ENTROPY;
        }
        let mut buf = [0u8; 256];
        let h = if self.read_bytes(block, &mut buf) { super::kuna_symemu::entropy(&buf) } else { 0.0 };
        self.entropy.borrow_mut().insert(block, h);
        h >= BYTECODE_ENTROPY
    }
    fn bytecode_near(&self, addr: u64) -> bool {
        let Some(s) = self.section_of(addr) else { return false };
        if let Some(h) = self.near_entropy.borrow().get(&addr) {
            return *h >= BYTECODE_ENTROPY;
        }
        let start = if addr < s.start.saturating_add(128) {
            addr
        } else if addr.saturating_add(128) > s.end {
            addr.saturating_sub(256).max(s.start)
        } else {
            addr - 128
        };
        let mut buf = [0u8; 256];
        let h = if self.read_bytes(start, &mut buf) { super::kuna_symemu::entropy(&buf) } else { return self.bytecode_like(addr) };
        self.near_entropy.borrow_mut().insert(addr, h);
        h >= BYTECODE_ENTROPY
    }
}

/// Longest string literal [`ArchTarget::string_at`] reads.
const STRING_MAX: u64 = 1024;

/// The byte length, terminator included, of the string literal `buf` holds
/// at `addr`: a run of printable characters closed by a NUL, which is neither
/// a pointer-aligned slot holding a `mapped` address nor the head of an array
/// of 2- or 4-byte character codes whose second element `refs` does not also
/// name (`operand_refs`' test for a native operand, `kuna-analysis`
/// `operand_refs::emit_facts`).
fn literal_len(buf: &[u8], addr: u64, ptr: usize, mapped: impl Fn(u64) -> bool, refs: &BTreeSet<u64>) -> Option<u32> {
    let printable = |b: u8| (0x20..=0x7e).contains(&b) || matches!(b, 0x09 | 0x0a | 0x0d);
    let len = buf.iter().position(|&b| !printable(b))?;
    if len == 0 || buf[len] != 0 {
        return None;
    }
    if addr.is_multiple_of(ptr as u64) && buf.len() >= ptr {
        let word = buf[..ptr].iter().rev().fold(0u64, |w, &b| (w << 8) | b as u64);
        if mapped(word) {
            return None;
        }
    }
    let unit_is = |width: usize, k: usize, nul_ok: bool| {
        buf.get(k * width..(k + 1) * width)
            .is_some_and(|u| (printable(u[0]) || (nul_ok && u[0] == 0)) && u[1..].iter().all(|&b| b == 0))
    };
    let wide = [(4u64, 2usize), (2, 3)]
        .into_iter()
        .find(|&(width, units)| addr.is_multiple_of(width) && (0..units).all(|k| unit_is(width as usize, k, k == 2)));
    if wide.is_some_and(|(width, _)| !refs.contains(&(addr + width))) {
        return None;
    }
    Some(len as u32 + 1)
}

/// The string literals the emitted program refers to: every pointer-sized
/// constant a COPY or STORE writes (the values the emitter folded, and the
/// immediates of the native code it kept) that [`ArchTarget::string_at`]
/// accepts.
fn string_constants(target: &ArchTarget<'_>, overlay: &PcodeOverlay) -> Vec<(u64, u32)> {
    let ptr = target.ram.get_addr_size();
    let mut refs = BTreeSet::new();
    for insn in overlay.insns.values() {
        for op in &insn.ops {
            let value = match op.opc {
                OpCode::CPUI_COPY => op.ins.first(),
                OpCode::CPUI_STORE => op.ins.get(2),
                _ => None,
            };
            if let Some(v) = value.filter(|v| v.size == ptr && v.space.as_ref().map(classify_space) == Some(Space::Const)) {
                refs.insert(v.offset);
            }
        }
    }
    refs.iter().filter_map(|&a| target.string_at(a, &refs).map(|len| (a, len))).collect()
}

/// Minimum entropy (bits/byte) of a 256-byte window for a VPC fetch (the paper's 5.0).
const BYTECODE_ENTROPY: f64 = 5.0;

/// The register facts for the supported ISAs (x86-64 and x86): the
/// Microsoft x64 convention on a Windows image, the System V one elsewhere.
fn abi_for(arch: &Architecture, target: &ArchTarget<'_>) -> Option<Abi> {
    let t = arch.translate();
    let reg = |n: &str| t.probe_register_varnode(n.as_bytes()).map(|v| (v.offset, v.size as u8));
    let (sp, ptr) = reg("RSP").or_else(|| reg("ESP"))?;
    let x64 = ptr == 8;
    let names: &[&str] = if x64 {
        &["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15"]
    } else {
        &["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"]
    };
    let mut gprs = Vec::new();
    for n in names {
        gprs.push(reg(n)?.0);
    }
    let return_reg = reg(if x64 { "RAX" } else { "EAX" })?.0;
    let sysv = x64 && !target.windows;
    let clobber_names: &[&str] = match (x64, sysv) {
        (true, false) => &["RCX", "RDX", "R8", "R9", "R10", "R11"],
        (true, true) => &["RCX", "RDX", "RSI", "RDI", "R8", "R9", "R10", "R11"],
        _ => &["ECX", "EDX"],
    };
    let mut clobbers: Vec<(u64, u8)> = clobber_names.iter().filter_map(|n| reg(n)).collect();
    for f in ["CF", "PF", "AF", "ZF", "SF", "OF"] {
        if let Some((o, s)) = reg(f) {
            clobbers.push((o, s));
        }
    }
    let cf = reg("CF")?.0;
    let id = reg("ID").map(|r| r.0 + 1).unwrap_or(cf + 0x16);
    let saved_names: &[&str] = match (x64, sysv) {
        (true, false) => &["RBX", "RBP", "RDI", "RSI", "RSP", "R12", "R13", "R14", "R15"],
        (true, true) => &["RBX", "RBP", "RSP", "R12", "R13", "R14", "R15"],
        _ => &["EBX", "EBP", "ESI", "EDI", "ESP"],
    };
    let arg_names: &[&str] = match (x64, sysv) {
        (true, false) => &["RCX", "RDX", "R8", "R9", "RSP"],
        (true, true) => &["RDI", "RSI", "RDX", "RCX", "R8", "R9", "RSP"],
        _ => &["ESP"],
    };
    let callee_saved = saved_names.iter().filter_map(|n| reg(n)).collect();
    let args = arg_names.iter().filter_map(|n| reg(n)).collect();
    let fixed_regs = match (&target.teb, thread_block_register(arch, ptr)) {
        (Some(teb), Some(off)) => vec![(off, ptr, teb.base)],
        _ => Vec::new(),
    };
    Some(Abi {
        fixed_regs,
        ptr_size: ptr,
        sp,
        return_reg,
        clobbers,
        flag_regs: (cf, id),
        gprs,
        big_endian: false,
        df: reg("DF").map(|r| r.0),
        pc: reg(if x64 { "RIP" } else { "EIP" }),
        stack_args: if x64 { 0 } else { 8 },
        call_home_words: if x64 && target.teb.is_some() { 4 } else { 0 },
        call_arg_words: 16,
        fixed_flags: [("TF", 0u8), ("IF", 1), ("NT", 0), ("RF", 0), ("VM", 0), ("AC", 0), ("VIF", 0), ("VIP", 0), ("ID", 0), ("IOPL", 0)]
            .iter()
            .filter_map(|(n, v)| reg(n).map(|r| (r.0, *v)))
            .collect(),
        callee_saved,
        args,
    })
}

/// Smallest executable section a protector plausibly injected (PLT/init
/// stubs and cold-text splits stay below it).
const MIN_VM_SECTION: u64 = 0x10000;
const DETECT_MAX_INSNS: usize = 4096;

/// The sections a protector's interpreter may live in ([`Section::vm_candidate`]).
fn large_code_sections(arch: &Architecture) -> Vec<(u64, u64)> {
    let Some(t) = ArchTarget::new(arch, 0) else { return Vec::new() };
    t.sections.iter().filter(|s| s.vm_candidate()).map(|s| (s.start, s.end)).collect()
}

/// A protected function: somewhere on its direct native flow (fall-through,
/// direct jumps, both sides of conditionals; calls not followed) a transfer
/// enters a candidate interpreter section other than its own -- the entry
/// stub a protector leaves at the original address, or the SDK-marked
/// region's `push imm; call vm_entry` -- at code that is not an import thunk.
/// A call to a known function is passed over: the exploration summarizes it
/// rather than entering it.  Returns the first such target.
pub fn detect_vm_entry(target: &ArchTarget<'_>, entry: u64) -> Option<u64> {
    if !target.sections.iter().any(|s| s.vm_candidate() && Some((s.start, s.end)) != target.entry_section) {
        return None;
    }
    let foreign = |addr: u64| -> bool {
        target.is_foreign_code(addr) && target.section_of(addr).is_some_and(|s| s.vm_candidate()) && !import_thunk(target, addr)
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![entry];
    let mut count = 0usize;
    while let Some(addr) = work.pop() {
        if !seen.insert(addr) || count >= DETECT_MAX_INSNS {
            continue;
        }
        count += 1;
        let Ok(insn) = target.lift(addr) else { continue };
        let mut falls = true;
        for op in &insn.ops {
            match op.opc {
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL => {
                    let Some(t) = op.ins.first() else { continue };
                    match t.space.as_ref().map(classify_space) {
                        Some(Space::Ram) => {
                            let summarized = op.opc == OpCode::CPUI_CALL && t.offset != entry && target.is_function_entry(t.offset);
                            if foreign(t.offset) && !summarized {
                                return Some(t.offset);
                            }
                            if op.opc != OpCode::CPUI_CALL && target.section_of(t.offset).is_some() {
                                work.push(t.offset);
                            }
                            if op.opc == OpCode::CPUI_BRANCH {
                                falls = false;
                            }
                        }
                        _ => {}
                    }
                }
                OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => falls = false,
                _ => {}
            }
        }
        if falls {
            work.push(addr + insn.len as u64);
        }
    }
    None
}

impl DevirtProgram {
    /// What the recovered program may be missing or get wrong, for the
    /// function's header ([`caveats_of`]).
    pub fn caveats(&self) -> Option<String> {
        caveats_of(&self.stats)
    }
}

/// What a program explored with these statistics may be missing or get
/// wrong: computed jumps left unresolved, paths that stopped, an exploration
/// that ended before its fixpoint (naming the limit that ended it), paths
/// joined without merging their values, and stores the emulation could not
/// place.
pub fn caveats_of(s: &Stats) -> Option<String> {
    let mut out = Vec::new();
    if s.unresolved > 0 {
        out.push(format!("{} unresolved computed jump{}", s.unresolved, if s.unresolved == 1 { "" } else { "s" }));
    }
    if s.halts > 0 {
        out.push(format!("{} path{} not followed", s.halts, if s.halts == 1 { "" } else { "s" }));
    }
    if s.misplaced_tail_calls > 0 {
        let n = s.misplaced_tail_calls;
        out.push(format!("{n} jump{} to another function with the stack away from its entry depth", if n == 1 { "" } else { "s" }));
    }
    if let Some(limit) = s.cut_short {
        out.push(format!("exploration cut short by its {}", limit.name()));
    } else if !s.converged {
        out.push("no fixpoint within the round limit".into());
    }
    if s.unmerged > 0 {
        out.push(format!("{} path{} joined at another stack depth unmerged", s.unmerged, if s.unmerged == 1 { "" } else { "s" }));
    }
    if s.lost_sp > 0 {
        out.push(format!("{} stack pointer write{} the emulation could not place", s.lost_sp, if s.lost_sp == 1 { "" } else { "s" }));
    }
    if s.dropped_stores > 0 {
        out.push(format!("{} store{} to unresolved addresses not modelled", s.dropped_stores, if s.dropped_stores == 1 { "" } else { "s" }));
    }
    (!out.is_empty()).then(|| format!("devirtualize: incomplete: {}", out.join(", ")))
}

/// The process's peak resident set (`VmHWM`), for the exploration log.
fn peak_rss() -> Option<String> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find(|l| l.starts_with("VmHWM:")).map(|l| l.trim_start_matches("VmHWM:").trim().to_string())
}

/// The code at `addr` is an import thunk: its first instruction, after an
/// optional `endbr`, is an indirect jump (a PLT or stub entry).
fn import_thunk(target: &ArchTarget<'_>, addr: u64) -> bool {
    let Ok(mut insn) = target.lift(addr) else { return false };
    if insn.asm.to_ascii_uppercase().starts_with("ENDBR") {
        let Ok(next) = target.lift(addr + insn.len as u64) else { return false };
        insn = next;
    }
    let flow = insn.ops.iter().rev().find(|op| {
        matches!(op.opc, OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN)
    });
    flow.is_some_and(|op| op.opc == OpCode::CPUI_BRANCHIND)
}

/// Why [`devirtualize`] gives a function up once its drive's memory limit
/// ([`crate::kuna_fnmemory`]) stopped the exploration: the drive ends with
/// that limit's error, so nothing is emitted for it.
const OVER_FN_MEMORY: &str = "the per-function memory limit stopped the exploration";

/// Run the whole pipeline for a function entry.  On x86 the emitted calls
/// keep ECX and EDX as possible arguments besides the stack: `__fastcall` and
/// `__thiscall` pass them, and MSVC's `__security_check_cookie` takes the
/// cookie in ECX.
pub fn devirtualize(arch: &Architecture, entry: u64) -> Result<DevirtProgram, String> {
    let target = ArchTarget::new(arch, entry).ok_or("no code space")?;
    let abi = abi_for(arch, &target).ok_or("unsupported architecture for devirtualization (x86/x86-64 only)")?;
    let sp_reg = abi.sp;
    let ptr = abi.ptr_size;
    let mut live_at_exit = abi.callee_saved.clone();
    live_at_exit.push((abi.return_reg, ptr));
    let mut call_args = abi.args.clone();
    if ptr == 4 {
        call_args.extend(abi.clobbers.iter().copied().filter(|(o, s)| *s == ptr && abi.gprs.contains(o)));
    }
    let callee_saved = abi.callee_saved.clone();
    let return_reg = abi.return_reg;
    let mut call_kills = abi.clobbers.clone();
    call_kills.push((abi.return_reg, ptr));
    let probe = detect_vm_entry(&target, entry).is_none();
    let mut explorer = Explorer::new(&target, abi.clone(), Config::default(), entry);
    explorer.probe = probe;
    let mut cfg = explorer.run();
    if crate::kuna_fnmemory::tripped() {
        return Err(OVER_FN_MEMORY.into());
    }
    if explorer.stats.rep_copies > 0 && explorer.stats.copy_read_unfolded {
        // A copied array the program itself reads: run the copies.
        let mut first = std::mem::take(&mut explorer.log);
        first.push("a rep movs copy is read outside folded VPC fetches: explored again with the copies run".into());
        drop(std::mem::take(&mut cfg));
        explorer = Explorer::new(&target, abi, Config::default(), entry);
        explorer.probe = probe;
        explorer.rep_copies = false;
        cfg = explorer.run();
        if crate::kuna_fnmemory::tripped() {
            return Err(OVER_FN_MEMORY.into());
        }
        first.append(&mut explorer.log);
        explorer.log = first;
    }
    let m = arch.manage();
    let ram = Rc::clone(&target.ram);
    let const_space = Rc::clone(m.get_constant_space().ok_or("no constant space")?);
    let unique = Rc::clone(m.get_unique_space().ok_or("no unique space")?);
    let register = Rc::clone(m.get_space_by_name("register").ok_or("no register space")?);
    let stride = 0x2000u64;
    let highest = ram.get_highest();
    let need = (cfg.nodes.len() as u64 + 1) * stride;
    let align = 0x1000_0000u64;
    let mut syn_base = (target.highest_mapped() / align + 2) * align;
    if ptr == 8 {
        syn_base = syn_base.max(1u64 << 48);
    }
    if syn_base.saturating_add(need) > highest {
        syn_base = highest.saturating_sub(need).saturating_sub(align) / align * align;
    }
    let conf = EmitConfig {
        syn_base,
        stride,
        ram,
        const_space,
        unique,
        register,
        scratch: arch.translate().get_unique_start(UniqueLayout::INJECT) as u64 + 0x4000,
        ptr_size: ptr,
        sp_reg,
        sp0: explorer.sp0(),
        fold_constants: true,
        remat_sp: true,
        live_at_exit,
        call_args,
        callee_saved,
        return_reg,
        call_arg_bytes: if ptr == 8 { 0xa8 } else { 0x44 },
        call_home_bytes: if ptr == 8 && target.teb.is_some() { 0x20 } else { 0 },
        eliminate_dead: std::env::var("KUNA_DEVIRT_NO_DCE").is_err(),
        slot_base: {
            let mut regs = std::collections::BTreeMap::new();
            arch.translate().get_all_registers(&mut regs);
            let top = regs.keys().map(|v| v.offset + v.size as u64).max().unwrap_or(0x1000);
            (top + 0xfff) & !0xfff
        },
        pure_userops: target
            .userops
            .iter()
            .enumerate()
            .filter(|(_, n)| matches!(n.as_str(), "rdtsc" | "rdtscp" | "pause" | "lfence" | "mfence" | "sfence" | "prefetch" | "prefetchw" | "nop" | "LOCK" | "UNLOCK"))
            .map(|(i, _)| i as u64)
            .collect(),
        lock_userops: target.userops.iter().enumerate().filter(|(_, n)| matches!(n.as_str(), "LOCK" | "UNLOCK")).map(|(i, _)| i as u64).collect(),
    };
    // The placeholders the emitter reads for "never defined" registers are
    // killed by every call too, so no call guard lets a value flow through one.
    let mut placeholders: Vec<(u64, u8)> = conf
        .call_args
        .iter()
        .filter(|(o, _)| *o != sp_reg)
        .map(|&(off, size)| (super::kuna_devirtemit::undef_register(&conf, off), size))
        .collect();
    placeholders.push((super::kuna_devirtemit::undef_register(&conf, return_reg), ptr));
    call_kills.extend(placeholders.iter().copied());
    let explored_hwm = peak_rss();
    let (overlay, emit_stats) = emit(&cfg, &mut explorer.arena, &target, &conf, entry);
    explorer.stats.user_stack_assumed = explorer.arena.sp_bound_used;
    let strings = string_constants(&target, &overlay);
    let mut log = std::mem::take(&mut explorer.log);
    for n in std::mem::take(&mut explorer.arena.sp_bound_notes) {
        log.push(format!("identity over the user-mode stack range only (emission): {n}"));
    }
    if let (Some(a), Some(b)) = (explored_hwm, peak_rss()) {
        log.push(format!("peak rss after exploration: {a}, after emission: {b}"));
    }
    for s in &target.sections {
        log.push(format!("section {:#x}-{:#x} flags={:#x}{}", s.start, s.end, s.flags, if Some((s.start, s.end)) == target.entry_section { " (entry)" } else { "" }));
    }
    log.push(format!("string literals: {:x?}", strings));
    log.push(format!("user stack clear bits: {:#x}", target.user_stack_clear_bits()));
    log.push(format!(
        "{} nodes ({} in VM), {} instructions, {} symbolized, {} unresolved, {} halts, {} rounds; emitted {} insns/{} ops, {} folded, {} sp-rematerialized, {} dead, {} privatized",
        explorer.stats.nodes,
        explorer.stats.vm_nodes,
        explorer.stats.insns,
        explorer.stats.symbolized,
        explorer.stats.unresolved,
        explorer.stats.halts,
        explorer.stats.rounds,
        emit_stats.insns,
        emit_stats.ops,
        emit_stats.folded,
        emit_stats.remat,
        emit_stats.dead,
        emit_stats.privatized
    ));
    if let Ok(path) = std::env::var("KUNA_DEVIRT_DUMP") {
        if let Ok(mut f) = std::fs::File::create(&path) {
            for (addr, insn) in &overlay.insns {
                let _ = writeln!(f, "{addr:#x}: {} {}", insn.mnemonic, insn.body);
                for op in &insn.ops {
                    let fmt_vn = |v: &VarnodeData| -> String {
                        format!(
                            "({},{:#x},{})",
                            v.space.as_ref().map(|s| s.get_name().to_string()).unwrap_or_else(|| "?".into()),
                            v.offset,
                            v.size
                        )
                    };
                    let _ = writeln!(
                        f,
                        "    {} {} {}",
                        op.out.as_ref().map(|v| fmt_vn(v) + " =").unwrap_or_default(),
                        kuna_num::opcodes::get_opname(op.opc),
                        op.ins.iter().map(fmt_vn).collect::<Vec<_>>().join(", ")
                    );
                }
            }
        }
    }
    if let Ok(path) = std::env::var("KUNA_DEVIRT_LOG") {
        if let Ok(mut f) = std::fs::File::create(&path) {
            for l in &log {
                let _ = writeln!(f, "{l}");
            }
            let mut keys: Vec<(usize, &super::kuna_devirtflow::NodeKey)> =
                explorer.to_symbolize.iter().map(|(k, v)| (v.len(), k)).collect();
            keys.sort_by(|a, b| b.0.cmp(&a.0));
            let _ = writeln!(f, "escaped frame offsets: {:?}", cfg.escapes);
            let _ = writeln!(f, "symbolized keys: {}", keys.len());
            for (n, k) in keys.iter().take(12) {
                let _ = writeln!(f, "  key {:#x}/{:?}: {} locations", k.addr, k.ctx.map(|c| format!("{c:#x}")), n);
                if let Some(locs) = explorer.to_symbolize.get(k) {
                    for (l, vals) in locs.iter().take(24) {
                        let _ = writeln!(f, "      {:?} values={:?}", l, vals.iter().map(|v| format!("{v:#x}")).collect::<Vec<_>>());
                    }
                }
            }
            for (i, n) in cfg.nodes.iter().enumerate() {
                let self_loop = matches!(n.exit, NodeExit::Branch(s) if s as usize == i);
                let dump = std::env::var("KUNA_DEVIRT_LOG_INSNS").is_ok()
                    || self_loop
                    || matches!(n.exit, NodeExit::Unresolved(_) | NodeExit::Halt(_));
                let _ = writeln!(
                    f,
                    "node {} key={:#x}/{:?} vm={} vpc_reg={:?} vpc_cell={:x?} insns={} exit={}",
                    i,
                    n.key.addr,
                    n.key.ctx.map(|c| format!("{c:#x}")),
                    n.entry_ctx.in_vm,
                    n.entry_ctx.vpc_reg,
                    n.entry_ctx.vpc_cell,
                    n.insns.len(),
                    describe_exit(&n.exit)
                );
                if dump && matches!(n.exit, NodeExit::Unresolved(_)) {
                    for p in &n.preds {
                        let pn = &cfg.nodes[*p as usize];
                        let _ = writeln!(f, "  pred node {} key={:#x} insns={}", p, pn.key.addr, pn.insns.len());
                        for (k, insn) in pn.insns.iter().enumerate().rev().take(40).collect::<Vec<_>>().into_iter().rev() {
                            let _ = writeln!(f, "      [{k}] {:#x} {}", insn.addr, insn.asm);
                        }
                    }
                }
                if dump {
                    for (k, insn) in n.insns.iter().enumerate() {
                        let _ = writeln!(f, "    [{k}] {:#x} {}", insn.addr, insn.asm);
                        if let Some(t) = n.traces.get(k) {
                            for a in &t.accesses {
                                let sym = if a.addr.is_none() && std::env::var("KUNA_DEVIRT_LOG_SYM").is_ok() {
                                    let r = explorer.arena.affine_probe(a.addr_expr);
                                    format!(" probe={r:?} expr={}", explorer.arena.fmt(a.addr_expr).chars().take(400).collect::<String>())
                                } else {
                                    String::new()
                                };
                                let _ = writeln!(
                                    f,
                                    "         {}{} via={:?} addr={}{sym}",
                                    if a.is_store { "store" } else { "load" },
                                    if a.lost { " (lost)" } else { "" },
                                    a.via.map(|(r, d)| format!("reg{r:x}{d:+}")),
                                    a.addr.map(|x| format!("{x:#x}")).unwrap_or_else(|| "sym".into())
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    let call_preserves = emit_stats
        .private_regs
        .into_iter()
        .chain(emit_stats.frame_reg)
        .chain(emit_stats.slot_regs)
        .chain(emit_stats.entry_regs.iter().map(|&(o, s)| (o, s as u32)))
        .collect();
    let escapes = cfg.escapes.iter().filter_map(|&o| i64::try_from(o).ok()).collect();
    let protector = target
        .sections
        .iter()
        .filter(|s| s.vm_candidate() && Some((s.start, s.end)) != target.entry_section)
        .filter(|s| cfg.nodes.iter().any(|n| n.entry_ctx.in_vm && s.start <= n.key.addr && n.key.addr < s.end))
        .map(|s| (s.start, s.end))
        .collect();
    let mut slots: Vec<u64> = cfg
        .nodes
        .iter()
        .filter_map(|n| match n.exit {
            NodeExit::Call { slot: Some(s), .. } | NodeExit::TailCallSlot(s) => Some(s),
            _ => None,
        })
        .collect();
    slots.sort_unstable();
    slots.dedup();
    Ok(DevirtProgram {
        entry,
        call_kills,
        placeholders,
        call_preserves,
        escapes,
        overlay: Rc::new(overlay),
        strings,
        protector,
        slots,
        stats: explorer.stats.clone(),
        emit_stats,
        log,
    })
}

fn describe_exit(e: &NodeExit) -> String {
    match e {
        NodeExit::Branch(s) => format!("branch -> {s}"),
        NodeExit::CondBranch { arms, fall } => format!("cbranch -> {:?} / {fall}", arms.iter().map(|(_, t)| *t).collect::<Vec<_>>()),
        NodeExit::Select { targets, by_atoms, .. } => {
            format!("select{} -> {:?}", if *by_atoms { "" } else { "(symbol)" }, targets.iter().map(|(_, s)| *s).collect::<Vec<_>>())
        }
        NodeExit::Call { target, ret, slot } => match slot {
            Some(s) => format!("call [{s:#x}] -> {ret}"),
            None => format!("call {:?} -> {ret}", target.map(|t| format!("{t:#x}"))),
        },
        NodeExit::Return => "return".into(),
        NodeExit::TailCall(c) => format!("tailcall {c:#x}"),
        NodeExit::TailCallSlot(c) => format!("tailcall [{c:#x}]"),
        NodeExit::NoReturn => "no return".into(),
        NodeExit::Unresolved(m) => format!("unresolved: {}", m.chars().take(400).collect::<String>()),
        NodeExit::Halt(m) => format!("halt: {m}"),
    }
}

/// [`devirtualize`] off the drive's clock ([`crate::decompile_drive::uncharge`]).
fn devirtualize_unclocked(arch: &mut Architecture, entry: u64) -> Result<DevirtProgram, String> {
    let started = std::time::Instant::now();
    let program = devirtualize(arch, entry);
    crate::decompile_drive::uncharge(arch, started.elapsed());
    program
}

/// How a devirtualized body is analysed.  Its frame is rebuilt from the
/// emulated interpreter, which reaches it through stack addresses it reloads
/// from its own operand stack and reads and writes it in halves, so every
/// STORE guards the stack ranges, a split argument slot is passed whole and
/// only a directly spilled slot bounds a LOAD guard.  The local-alias test is
/// off for the call argument trials: it takes every non-additive use of a
/// stack address as a pointer escaping to the locals above it, and the
/// interpreter makes such uses of addresses deep in the frame throughout (its
/// virtual stack pointer compared against the native one, its context copied
/// down the frame, the flags of that arithmetic pushed), so the boundary falls
/// below every call's outgoing arguments and each stack argument is rejected
/// as aliased.  The trials are judged by their data flow alone, as angr's
/// call-site maker reads the slots above the return address.  The structure
/// is rebuilt once the interpreter's dead stores are gone, and the residue
/// rules ([`kuna_residuerules`](super::kuna_residuerules)) apply.
const POLICY: BodyPolicy = BodyPolicy {
    guard_stack_stores: true,
    join_stack_inputs: true,
    direct_spills: true,
    inputs_by_dataflow: true,
    restructure_after_deadcode: true,
    residue_rules: true,
    loads_before_frame_writes: true,
};

impl DevirtProgram {
    /// The program as the plane's recovered body.  Every external call kills
    /// the ABI's scratch registers outright: the interpreter re-saves its
    /// whole context right after each call, reading registers the callee just
    /// clobbered, and the prototype model's "unknown effect" on those would
    /// make that garbage read look like a live use of the argument the caller
    /// had placed there.  A COPY out of a placeholder register (an input
    /// nothing wrote, or a call's indirect creation), or into or out of a
    /// register holding an argument register's entry value (the function's
    /// own input, which counts as an argument only where a later one does),
    /// is incidental.
    fn into_body(self) -> RecoveredBody {
        let mut header = vec![format!(
            "devirtualize: VM-protected function recovered ({} nodes, {} instructions, {} rounds{}{})",
            self.stats.nodes,
            self.stats.insns,
            self.stats.rounds,
            if self.stats.writable_bytecode { "; assumes its bytecode in writable memory is what the file holds" } else { "" },
            if self.stats.woven > 1 { format!("; follows the first of {} interleaved bytecode programs", self.stats.woven) } else { String::new() }
        )];
        header.extend(self.caveats());
        let entries = &self.emit_stats.entry_regs;
        RecoveredBody {
            by: NAME,
            overlay: Rc::clone(&self.overlay),
            facts: BodyFacts {
                call_kills: self.call_kills.iter().map(|&(o, s)| (o, s as int4)).collect(),
                call_preserves: self.call_preserves.iter().map(|&(o, s)| (o, s as int4)).collect(),
                call_floor: self.emit_stats.call_floor.map(|k| k as i64),
                escapes: self.escapes.clone(),
                incidental_from: self.placeholders.iter().chain(entries).copied().collect(),
                incidental_to: entries.clone(),
                policy: POLICY,
            },
            strings: self.strings.clone(),
            header,
            detail: Box::new(self),
        }
    }
}

/// The VM devirtualizer as a member of the deobfuscation plane.
pub struct Devirtualize;

impl Deobfuscator for Devirtualize {
    fn name(&self) -> &'static str {
        NAME
    }

    /// Forced, the function is devirtualized; on detection, only one whose
    /// direct native flow enters an interpreter section ([`detect_vm_entry`]).
    /// A program that never entered an interpreter is dropped: the plain flow
    /// is the right answer.
    fn front(&self, arch: &mut Architecture, fd: &mut Funcdata, how: Engage) -> Option<RecoveredBody> {
        let entry = fd.get_address().get_offset();
        if how == Engage::Detect {
            if arch.devirt_vm_sections.is_none() {
                arch.devirt_vm_sections = Some(large_code_sections(arch));
            }
            let candidates = arch.devirt_vm_sections.as_deref().unwrap_or(&[]);
            if !candidates.iter().any(|&(s, e)| !(s <= entry && entry < e)) {
                return None;
            }
            ArchTarget::new(arch, entry).and_then(|t| detect_vm_entry(&t, entry))?;
        }
        match devirtualize_unclocked(arch, entry) {
            Ok(p) if p.stats.vm_nodes == 0 => None,
            Ok(p) => Some(p.into_body()),
            Err(e) => {
                fd.warning_header(&format!("devirtualize: failed: {e}"));
                None
            }
        }
    }

    fn after_flow(&self, arch: &mut Architecture, fd: &Funcdata, how: Engage) -> Option<RecoveredBody> {
        if how != Engage::Detect {
            return None;
        }
        interpreter_after_flow(arch, fd)
    }

    fn audit(&self, arch: &Architecture, fd: &mut Funcdata, body: &RecoveredBody) {
        let Some(p) = body.detail.downcast_ref::<DevirtProgram>() else { return };
        warn_unshown_escapes(p, fd);
        warn_protector_state(arch, p, fd);
        warn_lost_stores(p, fd);
    }
}

/// Detection on a function whose plain flow has a computed jump: probe it for an
/// interpreter that is the function's own body
/// ([`Explorer::probe_interpreter`]) and, when one is found and its
/// exploration decides a dispatch, cache the devirtualized program, so the
/// caller follows the flow again from it. Every other function pays one scan
/// of its ops; a front-end whose translator is not SLEIGH (Ghidra's), which
/// cannot serve the overlay, pays nothing.
fn interpreter_after_flow(arch: &mut Architecture, fd: &Funcdata) -> Option<RecoveredBody> {
    let entry = fd.get_address().get_offset();
    let started = std::time::Instant::now();
    if !interpreter_shape(arch, fd) {
        if let Ok(path) = std::env::var("KUNA_DEVIRT_PROBE_LOG") {
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
                let _ = writeln!(f, "{entry:#x} scan {:.3}ms", started.elapsed().as_secs_f64() * 1000.0);
            }
        }
        return None;
    }
    let mut lap = [0f64; 3];
    let found = {
        let target = ArchTarget::new(arch, entry)?;
        lap[0] = started.elapsed().as_secs_f64() * 1000.0;
        let abi = abi_for(arch, &target)?;
        let mut probe = Explorer::new(&target, abi, Config::default(), entry);
        lap[1] = started.elapsed().as_secs_f64() * 1000.0;
        let r = !probe.probe_interpreter().cells.is_empty();
        lap[2] = target.lifts.borrow().len() as f64;
        r
    };
    if let Ok(path) = std::env::var("KUNA_DEVIRT_PROBE_LOG") {
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{entry:#x} found={found} target={:.3} setup={:.3} lifts={} {:.3}ms", lap[0], lap[1], lap[2], started.elapsed().as_secs_f64() * 1000.0);
        }
    }
    if !found {
        return None;
    }
    match devirtualize_unclocked(arch, entry) {
        Ok(p) if p.stats.vm_nodes > 0 && p.stats.data_dispatches > 0 => Some(p.into_body()),
        _ => None,
    }
}

/// The plain flow has an in-function interpreter's shape: a computed jump
/// inside a cycle of the flow graph (a switch dispatch in its loop), or a jump
/// the flow could not recover and treats as a call (a CALLIND with an
/// artificial RETURN at its address: threaded code) in a function that copies
/// memory (a call to a `memcpy`-like routine, or a `rep movs`, which branches
/// back to itself).
fn interpreter_shape(arch: &Architecture, fd: &Funcdata) -> bool {
    let mut jumps = Vec::new();
    let mut calls = BTreeSet::new();
    let mut returns = BTreeSet::new();
    let mut copies = false;
    for o in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(o) else { continue };
        let at = op.get_addr().get_offset();
        let target = op.get_in(0).and_then(|v| fd.vbank().get(v)).filter(|v| v.get_space().get_name() != "const").map(|v| v.get_offset());
        match op.code() {
            OpCode::CPUI_BRANCHIND => jumps.extend(op.get_parent()),
            OpCode::CPUI_CALLIND => {
                calls.insert(at);
            }
            OpCode::CPUI_RETURN => {
                returns.insert(at);
            }
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH if target == Some(at) => copies = true,
            _ => {}
        }
    }
    for i in 0..fd.num_calls() {
        let entry = fd.get_call_specs(i).get_entry_address();
        if !entry.is_invalid() {
            if let Some((sid, _)) = arch.symboltab.find_function_across_scopes(entry) {
                copies |= copy_routine_name(&arch.symboltab.symbol(sid).name);
            }
        }
    }
    jumps.iter().any(|b| in_cycle(fd, *b)) || (copies && calls.intersection(&returns).next().is_some())
}

/// Block `b` is reachable from its own successors.
fn in_cycle(fd: &Funcdata, b: crate::context::BlockId) -> bool {
    let graph = fd.bblocks_ref();
    let mut seen = BTreeSet::new();
    let mut work: Vec<crate::context::BlockId> = (0..graph.block(b).size_out()).map(|i| graph.block(b).get_out(i)).collect();
    while let Some(x) = work.pop() {
        if x == b {
            return true;
        }
        if seen.insert(x) {
            work.extend((0..graph.block(x).size_out()).map(|i| graph.block(x).get_out(i)));
        }
    }
    false
}

/// A C library routine with `memcpy`'s contract (destination, source, count).
fn copy_routine_name(name: &str) -> bool {
    matches!(name.split('@').next().unwrap_or(""), "memcpy" | "memmove" | "__memcpy_chk" | "__memmove_chk")
}

/// Warn when the decompiled body of a devirtualized function still reads or
/// writes a local whose address an external call of the body receives (as
/// the exploration saw it), while no op of the body computes that address:
/// printed so, the local looks as if no call could change it.  Run once the
/// body is decompiled.
fn warn_unshown_escapes(p: &DevirtProgram, fd: &mut Funcdata) {
    let mut escapes: BTreeSet<i64> = BTreeSet::new();
    let mut taken: BTreeSet<i64> = BTreeSet::new();
    for op in fd.obank().iter_alive() {
        let Some(o) = fd.obank().get(op) else { continue };
        match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                if let Some(e) = p.emit_stats.call_escapes.get(&o.get_addr().get_offset()) {
                    escapes.extend(e.iter().copied());
                }
            }
            OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let Some(a) = o.get_out().and_then(|out| crate::kuna_stackranges::frame_address(fd, out)) else { continue };
                if a.terms.is_empty() && a.first == a.last {
                    taken.insert(a.first);
                }
            }
            _ => {}
        }
    }
    if escapes.is_empty() {
        return;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space().cloned() else { return };
    let bits = 8 * stack.get_addr_size() as u32;
    let signed = |off: u64| -> i64 { if bits >= 64 { off as i64 } else { ((off << (64 - bits)) as i64) >> (64 - bits) } };
    let unshown = escapes
        .iter()
        .filter(|&&e| {
            !taken.contains(&e)
                && fd.vbank().iter_loc().any(|vn| {
                    fd.vbank().get(vn).is_some_and(|v| {
                        let lo = signed(v.get_offset());
                        v.get_space().get_index() == stack.get_index()
                            && lo <= e
                            && e < lo + v.get_size() as i64
                            && (v.is_written() || !v.has_no_descend())
                    })
                })
        })
        .count();
    if unshown > 0 {
        fd.warning_header(&format!(
            "devirtualize: incomplete: {unshown} local{} whose address a call receives printed as never passed",
            if unshown == 1 { "" } else { "s" }
        ));
    }
}

/// Warn when the decompiled body of a devirtualized function still reads or
/// writes the protector's own memory ([`DevirtProgram::protector`]): a cell
/// of the interpreter's context the emulation did not fold or privatize
/// survives as a global, so the body computes with VM state the program
/// never had.  Counted over the final ops (heritage's joins and call guards
/// excluded): a read or write of a memory location inside the sections, and
/// a load or store whose address is a constant inside them plus an offset.
/// Reads of an import slot a call went through are the program's own.
/// Run once the body is decompiled.
fn warn_protector_state(arch: &Architecture, p: &DevirtProgram, fd: &mut Funcdata) {
    if p.protector.is_empty() {
        return;
    }
    let Some(ram) = fd.get_arch().manage().get_default_data_space().map(|s| s.get_index()) else { return };
    let inside = |a: u64, size: u64| p.protector.iter().any(|&(s, e)| a < e && a.saturating_add(size.max(1)) > s);
    let ptr = arch.manage().get_default_code_space().map_or(8, |s| s.get_addr_size() as u64);
    let slot = |a: u64| p.slots.iter().any(|&s| a >= s && a < s + ptr);
    let mem = |v: crate::context::VarnodeId| -> Option<(u64, u64)> {
        let vn = fd.vbank().get(v)?;
        (vn.get_space().get_index() == ram).then(|| (vn.get_offset(), vn.get_size().max(0) as u64))
    };
    let based = |v: Option<crate::context::VarnodeId>| -> bool {
        let mut work: Vec<(crate::context::VarnodeId, u32)> = v.into_iter().map(|v| (v, 0)).collect();
        while let Some((v, depth)) = work.pop() {
            let Some(vn) = fd.vbank().get(v) else { continue };
            if vn.is_constant() {
                if inside(vn.get_offset(), 1) {
                    return true;
                }
                continue;
            }
            let Some(def) = vn.get_def().and_then(|d| fd.obank().get(d)) else { continue };
            if depth < 4 && matches!(def.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_CAST | OpCode::CPUI_COPY) {
                work.extend((0..def.num_input()).filter_map(|i| def.get_in(i)).map(|x| (x, depth + 1)));
            }
        }
        false
    };
    let (mut reads, mut writes) = (0usize, 0usize);
    for op in fd.obank().iter_alive() {
        let Some(o) = fd.obank().get(op) else { continue };
        let code = o.code();
        let out = o.get_out().and_then(mem);
        let kept = matches!(code, OpCode::CPUI_INDIRECT | OpCode::CPUI_COPY) && out.is_some() && o.get_in(0).and_then(mem) == out;
        if code == OpCode::CPUI_MULTIEQUAL || kept {
            continue;
        }
        if out.is_some_and(|(a, sz)| inside(a, sz)) {
            writes += 1;
        }
        if code == OpCode::CPUI_INDIRECT {
            continue;
        }
        let access = matches!(code, OpCode::CPUI_LOAD | OpCode::CPUI_STORE);
        if access && based(o.get_in(1)) {
            if code == OpCode::CPUI_LOAD {
                reads += 1;
            } else {
                writes += 1;
            }
        }
        for i in usize::from(access) as int4..o.num_input() {
            if o.get_in(i).and_then(mem).is_some_and(|(a, sz)| inside(a, sz) && !slot(a)) {
                reads += 1;
            }
        }
    }
    if reads + writes > 0 {
        let part = |n: usize, what: &str| format!("{n} {what}{}", if n == 1 { "" } else { "s" });
        let what = match (reads, writes) {
            (r, 0) => part(r, "read"),
            (0, w) => part(w, "write"),
            (r, w) => format!("{} and {}", part(r, "read"), part(w, "write")),
        };
        fd.warning_header(&format!("devirtualize: incomplete: {what} of the protector's own memory left in the body"));
    }
}

/// Warn when a store the exploration lost ([`EmitStats::lost_stores`]:
/// placed nowhere, while memory its address depends on kept its contents)
/// lies on a path of the decompiled body: every value the exploration read
/// from that memory after it, and folded into the program, may be the one
/// from before it, and the store itself may be found dead once those reads
/// are folded.  A store counts when a basic block the final function keeps
/// covers its instruction, so a store in removed code counts nothing.  Run
/// once the body is decompiled.
fn warn_lost_stores(p: &DevirtProgram, fd: &mut Funcdata) {
    let Some(space) = fd.get_address().get_space().cloned() else { return };
    if p.emit_stats.lost_stores.is_empty() {
        return;
    }
    let blocks = fd.bblocks_ref();
    let covers: Vec<&kuna_base::address::RangeList> = (0..fd.bblocks_get_size())
        .filter_map(|i| match blocks.block(fd.bblocks_get_block(i)).kind() {
            crate::block::BlockKind::Basic(bd) => Some(&bd.cover),
            _ => None,
        })
        .collect();
    let n = p.emit_stats.lost_stores.iter().filter(|&&a| covers.iter().any(|c| c.in_range(&Address::new(space.clone(), a), 1))).count();
    if n > 0 {
        fd.warning_header(&format!("devirtualize: incomplete: {n} store{} through an address the emulation could not place on a path of the body", if n == 1 { "" } else { "s" }));
    }
}

#[cfg(test)]
mod tests;
