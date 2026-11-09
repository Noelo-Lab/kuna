//! (kuna pushan) Devirtualization of VM-protected functions — `option pushan`.
//!
//! Port of Pushan (Sudhir et al., "Pushan: Trace-Free Deobfuscation of
//! Virtualization-Obfuscated Binaries", NDSS 2027; angr PR #7680) onto p-code.
//! A function whose entry jumps into the interpreter of a VM-based protector
//! (VMProtect, Themida) decompiles, by default, to the interpreter's dispatch
//! loop or to a two-line stub.  With this on, the flow follower is handed a
//! synthetic program instead: the protected function's own control flow,
//! recovered by VPC-sensitive, constraint-free symbolic emulation of the
//! interpreter ([`kuna_pushanflow`](super::kuna_pushanflow)) and emitted as
//! p-code with the interpreter's constant machinery folded away
//! ([`kuna_pushanemit`](super::kuna_pushanemit)).  The rest of the decompiler
//! (SSA, dead code, stack variables, types, structuring) does the paper's
//! stage 2 and 3 on that program unchanged.
//!
//! `auto` (the default) devirtualizes a function whose direct native flow
//! reaches a branch or call into a separate executable section of at least
//! 64 KiB — the stub a protector leaves at the original entry; `on` treats the requested function
//! as protected regardless; `off` restores the plain flow.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Write;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::kuna_pcodeoverlay::PcodeOverlay;
use kuna_sleigh::loadimage::{section_flags, LoadImageSection};
use kuna_sleigh::translate::{PcodeEmit, UniqueLayout};

use super::kuna_pushanemit::{emit, EmitConfig, EmitStats};
use super::kuna_pushanemu::{classify_space, LiftedInsn, PushanTarget, RawOp, Space};
use super::kuna_pushanflow::{Abi, Config, Explorer, NodeExit, Stats};
use super::kuna_pushanstate::PushanImage;
use crate::architecture::Architecture;
use crate::funcdata::Funcdata;

pub const MODE_OFF: u8 = 0;
pub const MODE_AUTO: u8 = 1;
pub const MODE_ON: u8 = 2;

pub struct OptionPushan;

impl OptionPushan {
    pub const NAME: &'static str = "pushan";

    pub fn apply(&self, p1: &str) -> KunaResult<(u8, String)> {
        match p1 {
            "off" | "false" | "no" => Ok((MODE_OFF, "VM devirtualization turned off".into())),
            "auto" => Ok((MODE_AUTO, "VM devirtualization set to auto (protected entry stubs)".into())),
            "on" | "true" | "yes" | "force" => Ok((MODE_ON, "VM devirtualization forced on".into())),
            other => Err(KunaError::parse(format!("pushan expects auto|on|off, got {other}"))),
        }
    }
}

/// The devirtualized program for one function entry.
pub struct PushanProgram {
    pub entry: u64,
    /// Registers (offset, size) an external call leaves undefined.
    pub call_kills: Vec<(u64, u8)>,
    /// The placeholder registers the emitter reads for "never defined" values.
    pub placeholders: Vec<(u64, u8)>,
    /// Registers (offset, size) every external call leaves untouched: the
    /// interpreter's own memory, privatized by the emitter.
    pub call_preserves: Vec<(u64, u32)>,
    pub overlay: Rc<PcodeOverlay>,
    pub stats: Stats,
    pub emit_stats: EmitStats,
    pub log: Vec<String>,
}

struct Section {
    start: u64,
    end: u64,
    flags: u32,
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

/// [`PushanTarget`] over the live [`Architecture`].
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
                    sections.push(Section { start, end: start.saturating_add(rec.size), flags: rec.flags });
                }
                if !more {
                    break;
                }
            }
            loader.close_section_info();
            if sections.is_empty() {
                for (vma, size, flags) in loader.get_segments() {
                    sections.push(Section { start: vma, end: vma.saturating_add(size), flags });
                }
            }
        }
        let entry_section = sections.iter().find(|s| s.start <= entry && entry < s.end).map(|s| (s.start, s.end));
        let mut userops = Vec::new();
        arch.translate().get_user_op_names(&mut userops);
        let ptr = ram.get_addr_size() as u8;
        let image_base = sections.iter().map(|s| s.start).min().unwrap_or(0) & !0xffff;
        let teb = super::kuna_fastfailnoreturn::archid_is_windows(&arch.archid).then(|| ThreadBlock::new(ptr, image_base));
        Some(ArchTarget {
            arch,
            ram,
            teb,
            sections,
            entry_section,
            lifts: RefCell::new(BTreeMap::new()),
            userops,
            entropy: RefCell::new(BTreeMap::new()),
        })
    }

    fn section_of(&self, addr: u64) -> Option<&Section> {
        self.sections.iter().find(|s| s.start <= addr && addr < s.end)
    }

    /// Highest mapped address, for placing the synthetic range.
    pub fn highest_mapped(&self) -> u64 {
        self.sections.iter().map(|s| s.end).max().unwrap_or(0)
    }
}

impl PushanImage for ArchTarget<'_> {
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
}

impl PushanTarget for ArchTarget<'_> {
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
    fn callee_extra_pop(&self, _site: u64) -> Option<i64> {
        None
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
        let h = if self.read_bytes(block, &mut buf) { super::kuna_pushanemu::entropy(&buf) } else { 0.0 };
        self.entropy.borrow_mut().insert(block, h);
        h >= BYTECODE_ENTROPY
    }
}

/// Minimum entropy (bits/byte) of a 256-byte window for a VPC fetch (the paper's 5.0).
const BYTECODE_ENTROPY: f64 = 5.0;

/// The register facts for the supported ISAs (x86-64 and x86).
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
    let clobber_names: &[&str] = if x64 { &["RCX", "RDX", "R8", "R9", "R10", "R11"] } else { &["ECX", "EDX"] };
    let mut clobbers: Vec<(u64, u8)> = clobber_names.iter().filter_map(|n| reg(n)).collect();
    for f in ["CF", "PF", "AF", "ZF", "SF", "OF"] {
        if let Some((o, s)) = reg(f) {
            clobbers.push((o, s));
        }
    }
    let cf = reg("CF")?.0;
    let id = reg("ID").map(|r| r.0 + 1).unwrap_or(cf + 0x16);
    let saved_names: &[&str] = if x64 { &["RBX", "RBP", "RDI", "RSI", "RSP", "R12", "R13", "R14", "R15"] } else { &["EBX", "EBP", "ESI", "EDI", "ESP"] };
    let arg_names: &[&str] = if x64 { &["RCX", "RDX", "R8", "R9", "RSP"] } else { &["ESP"] };
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

/// A protected function: somewhere on its direct native flow (fall-through,
/// direct jumps, both sides of conditionals; calls not followed) a transfer
/// enters a large executable section other than its own -- the entry stub a
/// protector leaves at the original address, or the SDK-marked region's
/// `push imm; call vm_entry`.  Returns the first such target.
pub fn detect_vm_entry(target: &ArchTarget<'_>, entry: u64) -> Option<u64> {
    if !target.sections.iter().any(|s| {
        s.flags & section_flags::CODE != 0 && s.end - s.start >= MIN_VM_SECTION && Some((s.start, s.end)) != target.entry_section
    }) {
        return None;
    }
    let foreign = |addr: u64| -> bool {
        target.is_foreign_code(addr)
            && target.section_of(addr).is_some_and(|s| s.end - s.start >= MIN_VM_SECTION)
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
                            if foreign(t.offset) {
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

/// Run the whole pipeline for a function entry.
pub fn devirtualize(arch: &Architecture, entry: u64) -> Result<PushanProgram, String> {
    let target = ArchTarget::new(arch, entry).ok_or("no code space")?;
    let abi = abi_for(arch, &target).ok_or("unsupported architecture for pushan (x86/x86-64 only)")?;
    let sp_reg = abi.sp;
    let ptr = abi.ptr_size;
    let mut live_at_exit = abi.callee_saved.clone();
    live_at_exit.push((abi.return_reg, ptr));
    let call_args = abi.args.clone();
    let callee_saved = abi.callee_saved.clone();
    let return_reg = abi.return_reg;
    let mut call_kills = abi.clobbers.clone();
    call_kills.push((abi.return_reg, ptr));
    let mut explorer = Explorer::new(&target, abi, Config::default(), entry);
    let cfg = explorer.run();
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
        call_arg_bytes: if ptr == 8 { 0x60 } else { 0x28 },
        eliminate_dead: std::env::var("KUNA_PUSHAN_NO_DCE").is_err(),
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
        .map(|&(off, size)| (super::kuna_pushanemit::undef_register(&conf, off), size))
        .collect();
    placeholders.push((super::kuna_pushanemit::undef_register(&conf, return_reg), ptr));
    call_kills.extend(placeholders.iter().copied());
    let (overlay, emit_stats) = emit(&cfg, &mut explorer.arena, &target, &conf, entry);
    let mut log = std::mem::take(&mut explorer.log);
    for s in &target.sections {
        log.push(format!("section {:#x}-{:#x} flags={:#x}{}", s.start, s.end, s.flags, if Some((s.start, s.end)) == target.entry_section { " (entry)" } else { "" }));
    }
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
    if let Ok(path) = std::env::var("KUNA_PUSHAN_DUMP") {
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
    if let Ok(path) = std::env::var("KUNA_PUSHAN_LOG") {
        if let Ok(mut f) = std::fs::File::create(&path) {
            for l in &log {
                let _ = writeln!(f, "{l}");
            }
            let mut keys: Vec<(usize, &super::kuna_pushanflow::NodeKey)> =
                explorer.to_symbolize.iter().map(|(k, v)| (v.len(), k)).collect();
            keys.sort_by(|a, b| b.0.cmp(&a.0));
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
                let dump = std::env::var("KUNA_PUSHAN_LOG_INSNS").is_ok()
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
                                let _ = writeln!(
                                    f,
                                    "         {} via={:?} addr={}",
                                    if a.is_store { "store" } else { "load" },
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
    let call_preserves = emit_stats.private_regs.into_iter().collect();
    Ok(PushanProgram { entry, call_kills, placeholders, call_preserves, overlay: Rc::new(overlay), stats: explorer.stats.clone(), emit_stats, log })
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
        NodeExit::Unresolved(m) => format!("unresolved: {}", m.chars().take(400).collect::<String>()),
        NodeExit::Halt(m) => format!("halt: {m}"),
    }
}

/// Decide whether `fd` is devirtualized, install the overlay the flow follower
/// will lift, and return the program.  Clears a stale overlay otherwise.
pub fn prepare_flow(arch: &mut Architecture, fd: &mut Funcdata) -> Option<Rc<PushanProgram>> {
    let mode = arch.pushan_mode;
    let entry = fd.get_address().get_offset();
    if mode == MODE_OFF {
        install(arch, None);
        return None;
    }
    if let Some(p) = arch.pushan_cache.get(&entry).cloned() {
        install(arch, Some(Rc::clone(&p.overlay)));
        adopt(&p, fd);
        return Some(p);
    }
    if mode == MODE_AUTO {
        let detected = ArchTarget::new(arch, entry).and_then(|t| detect_vm_entry(&t, entry));
        if detected.is_none() {
            install(arch, None);
            return None;
        }
    }
    match devirtualize(arch, entry) {
        Ok(p) if p.stats.vm_nodes == 0 => {
            // Never entered an interpreter: the plain flow is the right answer.
            install(arch, None);
            None
        }
        Ok(p) => {
            let p = Rc::new(p);
            arch.pushan_cache.insert(entry, Rc::clone(&p));
            install(arch, Some(Rc::clone(&p.overlay)));
            adopt(&p, fd);
            Some(p)
        }
        Err(e) => {
            fd.warning_header(&format!("pushan: devirtualization failed: {e}"));
            install(arch, None);
            None
        }
    }
}

/// Point the function at its synthetic program: the flow follower lifts the
/// overlay instead of a byte range, and every external call kills the ABI's
/// scratch registers outright. The interpreter re-saves its whole context
/// right after each call, reading registers the callee just clobbered, and
/// the prototype model's "unknown effect" on those would make that garbage
/// read look like a live use of the argument the caller had placed there.
fn adopt(p: &PushanProgram, fd: &mut Funcdata) {
    fd.kuna_set_size(0);
    fd.kuna_set_call_kills(p.call_kills.iter().map(|&(o, s)| (o, s as int4)).collect());
    fd.kuna_set_call_preserves(p.call_preserves.iter().map(|&(o, s)| (o, s as int4)).collect());
}

/// Mark every COPY out of a placeholder register as incidental, once the flow
/// follower has built the ops. The parameter analysis then walks through the
/// copy into the placeholder itself (an input nothing wrote, or a call's
/// indirect creation) instead of taking the copy as a solid movement of a
/// value into the argument register.
pub fn mark_undefined_copies(p: &PushanProgram, fd: &mut Funcdata) {
    let is_placeholder = |off: u64, size: i32| p.placeholders.iter().any(|&(o, s)| off >= o && off + size as u64 <= o + s as u64);
    let copies: Vec<crate::context::OpId> = fd.obank().iter_alive().filter(|&o| fd.obank().get(o).is_some_and(|op| op.code() == OpCode::CPUI_COPY)).collect();
    for op in copies {
        let from = fd
            .obank()
            .get(op)
            .and_then(|o| o.get_in(0))
            .and_then(|v| fd.vbank().get(v))
            .filter(|v| v.get_space().get_name() == "register")
            .map(|v| (v.get_offset(), v.get_size()));
        if let Some((off, size)) = from {
            if is_placeholder(off, size) {
                if let Some(o) = fd.obank_mut().get_mut(op) {
                    o.set_additional_flag(crate::op::pcodeop_addlflags::incidental_copy);
                }
            }
        }
    }
}

/// Does a call inside a devirtualized body kill the register range `addr:size`
/// beyond what its prototype model says? Consulted by `Heritage::guardCalls`.
pub fn call_kills(fd: &Funcdata, addr: &Address, size: int4) -> bool {
    let kills = fd.kuna_call_kills();
    if kills.is_empty() {
        return false;
    }
    let Some(spc) = addr.get_space() else { return false };
    if spc.get_name() != "register" {
        return false;
    }
    let lo = addr.get_offset();
    let hi = lo.saturating_add(size.max(0) as u64);
    kills.iter().any(|&(o, s)| o <= lo && hi <= o.saturating_add(s.max(0) as u64))
}

/// Does every call inside a devirtualized body leave `addr:size` untouched?
/// True for the registers standing for the interpreter's own memory.
pub fn call_preserves(fd: &Funcdata, addr: &Address, size: int4) -> bool {
    let keep = fd.kuna_call_preserves();
    if keep.is_empty() {
        return false;
    }
    let Some(spc) = addr.get_space() else { return false };
    if spc.get_name() != "register" {
        return false;
    }
    let lo = addr.get_offset();
    let hi = lo.saturating_add(size.max(0) as u64);
    keep.iter().any(|&(o, s)| o <= lo && hi <= o.saturating_add(s.max(0) as u64))
}

/// Point the translator at `overlay`. Through the shared handle: taking the
/// translator mutably resets per-image state other passes keep.
fn install(arch: &mut Architecture, overlay: Option<Rc<PcodeOverlay>>) {
    if let Some(s) = arch.translate().as_sleigh() {
        if overlay.is_some() || s.pcode_overlay().is_some() {
            s.set_pcode_overlay(overlay);
        }
    }
}
