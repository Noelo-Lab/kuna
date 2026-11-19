//! (kuna `uefi`) Where an EFI module's service tables and protocol interfaces go.
//!
//! Like every pass that decodes, this runs at the commit point, when the
//! engine's translator has the image. It lifts the code reachable from the
//! entry point and from the code pointers in the data sections once, up to
//! [`MAX_INSNS`] instructions, and walks it in address order with a small
//! abstract state per register and stack slot: a constant, a stack address, an
//! incoming parameter, or a pointer whose type is one of the EFI tables
//! ([`Val`]). The entry point's parameters seed it; a store of a typed pointer
//! into a global types the global, a load from a typed global or through a
//! typed pointer carries the type on, and a call through a service-table
//! member is read with its arguments:
//!
//! - `LocateProtocol`, `HandleProtocol`, `OpenProtocol`, their MM twins and
//!   `LocatePpi` type their interface out-parameter from the GUID argument;
//! - `InstallProtocolInterface`, `InstallMultipleProtocolInterfaces`, their MM
//!   twins and `InstallPpi` / `NotifyPpi` give the interface instance (or the
//!   descriptor list) its struct and name the functions it points at;
//! - a member whose prototype has a `T **` out-parameter (`GetSmstLocation`)
//!   types what it is handed;
//! - a PEI service call states its prototype at the call, because a PEIM
//!   that reads `EFI_PEI_SERVICES **` from below the IDT inline gives the
//!   decompiler no typed pointer to read the member from.
//!
//! A function that only forwards its own parameters to such a member (EDK2's
//! `PeiServicesLocatePpi`) is treated as that member at its call sites, and a
//! function that reads `EFI_PEI_SERVICES **` from below the IDT is EDK2's
//! `GetPeiServicesTablePointer`.
//!
//! Each function is cut into basic blocks and the state at the head of a
//! block is the intersection of what its predecessors leave, iterated to a
//! fixed point around loops, so nothing here is a guess about a path: a value
//! that is not the same on every way in is unknown, and a block only an
//! indirect jump reaches starts from nothing. A call keeps
//! stack addresses, the registers the prototype model preserves and, when the
//! callee calls nothing, the registers it does not write (a compiler that sees
//! the callee relies on exactly that). The walk repeats until the typed
//! globals, function starts and forwarding functions stop growing, at most
//! [`MAX_ROUNDS`] times. A function an installed interface points at that the
//! budget left out is decoded then, from a reserve of its own.
//!
//! `KUNA_UEFI_DEBUG` prints each service call the walk read.

use super::protocols;
use super::types::EfiTypes;
use crate::pass::AnalysisCtx;
use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::fspec::{effect_type, ParameterPieces, PrototypePieces, ProtoModel};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;
use object::{Object, ObjectSection, SectionKind};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;

const MAX_INSNS: usize = 60_000;
const MAX_MEMBER_INSNS: usize = 20_000;
const MAX_ROUNDS: usize = 4;
const MAX_SWEEPS: usize = 8;
const MAX_ARGS: usize = 14;
const MAX_LIST: u64 = 64;
const MAX_SLOTS: u64 = 32;
const MAX_LEAF_INSNS: usize = 64;
const MAX_LEAF_WRITES: usize = 48;
const WRAPPER_INSNS: u32 = 40;

const PPI_TERMINATE: u64 = 0x8000_0000;
const PPI_KIND_PPI: u64 = 0x10;
const PPI_KIND_NOTIFY: u64 = 0x60;

const PEI_SERVICES: &str = "EFI_PEI_SERVICES";
const BOOT_SERVICES: &str = "EFI_BOOT_SERVICES";
const MM_TABLES: [&str; 2] = ["EFI_MM_SYSTEM_TABLE", "EFI_SMM_SYSTEM_TABLE2"];

/// How the module is entered, which decides what its two parameters are.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    /// `(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)`.
    Dxe,
    /// `(EFI_PEI_FILE_HANDLE FileHandle, EFI_PEI_SERVICES **PeiServices)`.
    Pei,
    /// `(EFI_HANDLE ImageHandle, EFI_MM_SYSTEM_TABLE *MmSystemTable)`.
    Mm,
}

impl Kind {
    /// The entry prototype in the vocabulary of [`super::types`].
    pub(super) fn entry_decl(self) -> &'static str {
        match self {
            Kind::Dxe => "EFI_STATUS(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)",
            Kind::Pei => "EFI_STATUS(VOID* FileHandle, EFI_PEI_SERVICES** PeiServices)",
            Kind::Mm => "EFI_STATUS(EFI_HANDLE ImageHandle, EFI_MM_SYSTEM_TABLE* MmSystemTable)",
        }
    }

    fn seeds(self) -> [Option<&'static str>; 2] {
        match self {
            Kind::Dxe => [Some("EFI_HANDLE"), Some("EFI_SYSTEM_TABLE*")],
            Kind::Pei => [None, Some("EFI_PEI_SERVICES**")],
            Kind::Mm => [Some("EFI_HANDLE"), Some("EFI_MM_SYSTEM_TABLE*")],
        }
    }
}

/// A protocol or PPI interface instance the module installs.
#[derive(Clone, Debug)]
pub(super) struct Install {
    pub(super) guid: [u8; 16],
    pub(super) interface: String,
    /// Each function the instance points at: member name and entry.
    pub(super) members: Vec<(String, u64)>,
}

/// A function the walk can name and type.
#[derive(Clone, Debug)]
pub(super) struct Named {
    pub(super) name: String,
    /// `RET(ARGS)`, when the prototype is known.
    pub(super) decl: Option<String>,
}

/// A descriptor of a PPI list.
#[derive(Clone, Debug)]
pub(super) struct Descriptor {
    pub(super) name: String,
    pub(super) notify: bool,
}

/// What one walk of the module found.
#[derive(Default)]
pub(super) struct Found {
    /// Global address to the type of the pointer it holds.
    pub(super) globals: BTreeMap<u64, String>,
    /// Interface instance address to what is installed there.
    pub(super) installs: BTreeMap<u64, Install>,
    pub(super) functions: BTreeMap<u64, Named>,
    /// Call instruction to the prototype that call has.
    pub(super) call_decls: BTreeMap<u64, String>,
    pub(super) descriptors: BTreeMap<u64, Descriptor>,
    /// GUID addresses the tables do not name, with the name given here.
    pub(super) guids: BTreeMap<u64, String>,
    /// Interface lookups: (function, call instruction, GUID).
    pub(super) lookups: Vec<(u64, u64, [u8; 16])>,
    /// Service calls by the table they went through.
    pub(super) owners: BTreeMap<String, usize>,
    /// `KUNA_UEFI_DEBUG`: one line per service call read.
    pub(super) trace: Vec<String>,
}

impl Found {
    pub(super) fn service_calls(&self) -> usize {
        self.owners.values().sum()
    }
}

/// The interface another module of the same firmware volume installs for a
/// GUID this module has no struct for: its name and members.
pub(super) type Foreign = BTreeMap<[u8; 16], (String, Vec<(String, String)>)>;

type Key = (u16, u64, u32);

#[derive(Clone, Debug, PartialEq)]
enum Val {
    Const(u64),
    /// The stack pointer at function entry plus this.
    Stack(i64),
    /// The function's own parameter, untouched.
    Param(u8),
    /// A value of the type the spec spells, plus a byte offset when the type
    /// is a pointer.
    Typed(Rc<str>, i64),
    /// A function pointer read out of a table member.
    Func(Rc<Service>),
    /// The result of x86 `sidt`.
    Idtr,
    /// The IDT base plus this; EDK2 keeps `EFI_PEI_SERVICES **` one pointer
    /// below it.
    IdtBase(i64),
}

#[derive(Debug, PartialEq)]
struct Service {
    owner: String,
    member: String,
    sig: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpaceKind {
    Const,
    Unique,
    Ram,
    Other,
}

#[derive(Clone, Copy)]
struct Vn {
    kind: SpaceKind,
    space: u16,
    size: u32,
    offset: u64,
}

impl Vn {
    fn key(&self) -> Key {
        (self.space, self.offset, self.size)
    }
}

#[derive(Clone, Copy)]
struct Op {
    opc: OpCode,
    out: Option<Vn>,
    ins: [Option<Vn>; 3],
}

/// How control reaches an instruction other than by falling into it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reach {
    Fallthrough,
    /// A branch from a lower address.
    Forward,
    /// A branch from a higher address: a loop head.
    Back,
    /// A function starts here.
    Start,
}

/// Whether an instruction begins a basic block or a function.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Lead {
    None,
    Block,
    Function,
}

struct Insn {
    addr: u64,
    len: u32,
    first: u32,
    count: u32,
    /// A branch inside the instruction makes every write of it conditional.
    conditional: bool,
    reach: Reach,
    lead: Lead,
}

struct Lifter<'a> {
    ops: Vec<Op>,
    code: &'a Rc<AddrSpace>,
    spaces: Vec<Option<Rc<AddrSpace>>>,
}

impl Lifter<'_> {
    fn vn(&mut self, v: &VarnodeData) -> Option<Vn> {
        let space = v.space.as_ref()?;
        let index = u16::try_from(space.get_index()).ok()?;
        let kind = match space.get_type() {
            spacetype::IPTR_CONSTANT => SpaceKind::Const,
            spacetype::IPTR_INTERNAL => SpaceKind::Unique,
            _ if Rc::ptr_eq(space, self.code) => SpaceKind::Ram,
            _ => SpaceKind::Other,
        };
        if kind == SpaceKind::Other {
            let slot = usize::from(index);
            if self.spaces.len() <= slot {
                self.spaces.resize(slot + 1, None);
            }
            if self.spaces[slot].is_none() {
                self.spaces[slot] = Some(Rc::clone(space));
            }
        }
        Some(Vn { kind, space: index, size: v.size, offset: v.offset })
    }
}

impl PcodeEmit for Lifter<'_> {
    /// Keep the op unless it only computes a flag or a sub-word temporary:
    /// nothing the walk tracks is that narrow.
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        let out = outvar.and_then(|v| self.vn(v));
        let effect = matches!(
            opc,
            OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_STORE
        );
        let narrow_temporary = out.is_some_and(|o| o.size < 4 && o.kind == SpaceKind::Unique);
        let flag = out.is_some_and(|o| o.size == 1)
            && matches!(
                opc,
                OpCode::CPUI_INT_EQUAL
                    | OpCode::CPUI_INT_NOTEQUAL
                    | OpCode::CPUI_INT_LESS
                    | OpCode::CPUI_INT_SLESS
                    | OpCode::CPUI_INT_LESSEQUAL
                    | OpCode::CPUI_INT_SLESSEQUAL
                    | OpCode::CPUI_INT_CARRY
                    | OpCode::CPUI_INT_SCARRY
                    | OpCode::CPUI_INT_SBORROW
                    | OpCode::CPUI_BOOL_AND
                    | OpCode::CPUI_BOOL_OR
                    | OpCode::CPUI_BOOL_XOR
                    | OpCode::CPUI_BOOL_NEGATE
            );
        if !effect && (narrow_temporary || flag) {
            return;
        }
        let mut ins = [None; 3];
        for (slot, v) in ins.iter_mut().zip(vars) {
            *slot = self.vn(v);
        }
        self.ops.push(Op { opc, out, ins });
    }
}

/// What is known at one point: a handful of registers and stack slots, kept
/// as short lists because a lookup scans fewer entries than a hash costs.
#[derive(Clone, Default, PartialEq)]
struct State {
    regs: Vec<(Key, Val)>,
    stack: Vec<(i64, Val)>,
}

impl State {
    fn meet(&mut self, other: &State) {
        self.regs.retain(|(k, v)| other.reg(k) == Some(v));
        self.stack.retain(|(k, v)| other.slot(*k) == Some(v));
    }

    fn reg(&self, key: &Key) -> Option<&Val> {
        self.regs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    fn slot(&self, off: i64) -> Option<&Val> {
        self.stack.iter().find(|(k, _)| *k == off).map(|(_, v)| v)
    }

    /// Forget every register overlapping `key`, then record `val` there.
    fn set_reg(&mut self, key: Key, val: Option<Val>) {
        self.regs.retain(|(k, _)| !overlaps(k, &key));
        if let Some(val) = val {
            self.regs.push((key, val));
        }
    }

    fn set_slot(&mut self, off: i64, val: Val) {
        self.stack.retain(|(k, _)| *k != off);
        self.stack.push((off, val));
    }
}

fn overlaps(a: &Key, b: &Key) -> bool {
    a.0 == b.0 && a.1 < b.1.wrapping_add(u64::from(b.2)) && b.1 < a.1.wrapping_add(u64::from(a.2))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Loc {
    Reg(Key),
    Stack(i64),
}

#[derive(Clone, Debug, PartialEq)]
enum Source {
    Param(u8),
    Const(u64),
    Unknown,
}

/// A function that hands its own parameters to one service.
#[derive(Clone, Debug, PartialEq)]
struct Wrapper {
    service: Rc<Service>,
    args: Vec<Source>,
}

enum Role {
    Locate { guid: usize, out: usize },
    Install { guid: usize, interface: usize },
    InstallMultiple,
    PpiList { list: usize, notify: bool },
}

fn role(owner: &str, member: &str) -> Option<Role> {
    if owner == PEI_SERVICES {
        return match member {
            "LocatePpi" => Some(Role::Locate { guid: 1, out: 4 }),
            "InstallPpi" => Some(Role::PpiList { list: 1, notify: false }),
            "ReInstallPpi" => Some(Role::PpiList { list: 2, notify: false }),
            "NotifyPpi" => Some(Role::PpiList { list: 1, notify: true }),
            _ => None,
        };
    }
    if owner != BOOT_SERVICES && !MM_TABLES.contains(&owner) {
        return None;
    }
    let member = member.strip_prefix("Smm").or_else(|| member.strip_prefix("Mm")).unwrap_or(member);
    match member {
        "LocateProtocol" => Some(Role::Locate { guid: 0, out: 2 }),
        "HandleProtocol" | "OpenProtocol" => Some(Role::Locate { guid: 1, out: 2 }),
        "InstallProtocolInterface" | "ReinstallProtocolInterface" => Some(Role::Install { guid: 1, interface: 3 }),
        "InstallMultipleProtocolInterfaces" if owner == BOOT_SERVICES => Some(Role::InstallMultiple),
        _ => None,
    }
}

/// `fn RET(T a, U b, ...)` as its return type and `(type, name)` parameters.
pub(super) fn sig_parts(sig: &str) -> Option<(&str, Vec<(String, String)>, bool)> {
    let sig = sig.strip_prefix("fn ").unwrap_or(sig);
    let open = sig.find('(')?;
    let args = sig.get(open + 1..)?.strip_suffix(')')?;
    let mut out = Vec::new();
    let mut varargs = false;
    for arg in args.split(',').map(str::trim).filter(|a| !a.is_empty()) {
        if arg == "..." {
            varargs = true;
            continue;
        }
        match arg.rsplit_once(' ') {
            Some((ty, name)) if !name.ends_with('*') => out.push((ty.to_string(), name.to_string())),
            _ => out.push((arg.to_string(), String::new())),
        }
    }
    Some((&sig[..open], out, varargs))
}

fn sig_join(ret: &str, params: &[(String, String)], varargs: bool) -> String {
    let mut args: Vec<String> = params.iter().map(|(t, n)| format!("{t} {n}").trim().to_string()).collect();
    if varargs {
        args.push("...".to_string());
    }
    format!("{ret}({})", args.join(", "))
}

fn is_function(spec: &str) -> bool {
    spec.starts_with("fn ") || spec == "FUNC"
}

fn guid_data1(raw: &[u8; 16]) -> u32 {
    u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]])
}

/// What a walk keeps between rounds.
#[derive(Default)]
struct Learned {
    /// Global address to the type stored there; `None` once two types were.
    globals: BTreeMap<u64, Option<Rc<str>>>,
    /// Function entry to the types its callers pass in each parameter.
    seeds: BTreeMap<u64, BTreeMap<u8, Option<Rc<str>>>>,
    /// Function entry to the type it returns.
    rets: BTreeMap<u64, Option<Rc<str>>>,
    wrappers: BTreeMap<u64, Option<Wrapper>>,
    /// GUID to the struct made up for its interface.
    synthesized: BTreeMap<[u8; 16], String>,
    /// Functions that read the IDT register.
    reads_idt: BTreeSet<u64>,
    getters: BTreeSet<u64>,
}

/// The walk over one module.
pub(super) struct Walk<'a, 'e> {
    ctx: &'a AnalysisCtx<'a>,
    efi: &'e EfiTypes<'a>,
    foreign: &'e Foreign,
    /// Functions whose return type the caller already knows.
    returns: &'e [(u64, &'static str)],
    ptr: u32,
    thumb_bit: u64,
    code: Rc<AddrSpace>,
    model: Rc<ProtoModel>,
    /// The executable sections.
    text: Vec<(u64, u64)>,
    /// In address order.
    insns: Vec<Insn>,
    ops: Vec<Op>,
    seen: HashSet<u64>,
    /// Branches between instructions, as (from, to).
    branches: Vec<(u64, u64)>,
    /// Where functions start.
    starts: Vec<u64>,
    /// Functions an interface names that are not decoded yet.
    wanted: Vec<u64>,
    spaces: Vec<Option<Rc<AddrSpace>>>,
    sections: Vec<(u64, u64, bool)>,
    idtr_op: Option<u64>,
    sp: Option<Key>,
    ret: Option<Key>,
    params: Vec<Loc>,
    extrapop: Option<i64>,
    unaffected: HashMap<Key, bool>,
    /// Function entry to the registers a function that calls nothing writes.
    leaf_writes: BTreeMap<u64, Vec<Key>>,
    entry: u64,
    learned: Learned,
    changed: bool,
    found: Found,
    temps: Vec<(Key, Val)>,
    /// A pass that only computes states: nothing is recorded.
    dry: bool,
    fn_insns: BTreeMap<u64, u32>,
    fn_services: BTreeMap<u64, u32>,
    trace: bool,
}

impl<'a, 'e> Walk<'a, 'e> {
    /// Lift the module. `None` when the image has no default prototype model
    /// or no code.
    pub(super) fn new(
        ctx: &'a AnalysisCtx<'a>,
        efi: &'e EfiTypes<'a>,
        foreign: &'e Foreign,
        returns: &'e [(u64, &'static str)],
        entry: u64,
    ) -> Option<Self> {
        let arch = ctx.arch;
        let code = Rc::clone(arch.manage().get_default_code_space()?);
        let model = Rc::clone(arch.default_fp()?);
        let ptr = u32::try_from(efi.ptr).ok()?;
        let mut names = Vec::new();
        arch.translate().get_user_op_names(&mut names);
        let idtr_op = names.iter().position(|n| n == "InterruptDescriptorTableRegister").map(|i| i as u64);
        let mut sections = Vec::new();
        for s in ctx.file.sections() {
            let writable = !matches!(s.kind(), SectionKind::Text | SectionKind::ReadOnlyData | SectionKind::ReadOnlyString);
            if s.data().is_ok_and(|d| !d.is_empty()) {
                sections.push((s.address(), s.address().saturating_add(s.size()), writable));
            }
        }
        let mut me = Walk {
            ctx,
            efi,
            foreign,
            returns,
            ptr,
            thumb_bit: u64::from(super::is_arm32(ctx.bytes)),
            code,
            model,
            text: Vec::new(),
            insns: Vec::new(),
            ops: Vec::new(),
            seen: HashSet::new(),
            branches: Vec::new(),
            starts: Vec::new(),
            wanted: Vec::new(),
            spaces: Vec::new(),
            sections,
            idtr_op,
            sp: None,
            ret: None,
            params: Vec::new(),
            extrapop: None,
            unaffected: HashMap::new(),
            leaf_writes: BTreeMap::new(),
            entry,
            learned: Learned::default(),
            changed: false,
            found: Found::default(),
            temps: Vec::new(),
            dry: false,
            fn_insns: BTreeMap::new(),
            fn_services: BTreeMap::new(),
            trace: std::env::var_os("KUNA_UEFI_DEBUG").is_some(),
        };
        me.abi()?;
        me.lift();
        (!me.insns.is_empty()).then_some(me)
    }

    /// Where the prototype model puts pointer parameters and the return
    /// value, and what a call does to the stack pointer.
    fn abi(&mut self) -> Option<()> {
        let arch = self.ctx.arch;
        let void = arch.types().get_type_void().ok()?;
        let pointer = arch.types().get_type_pointer(self.efi.ptr, void, 1).ok()?;
        let pieces = PrototypePieces {
            outtype: Some(pointer.clone()),
            intypes: vec![pointer; MAX_ARGS],
            innames: vec![String::new(); MAX_ARGS],
            first_var_arg_slot: -1,
            ..Default::default()
        };
        let mut res = Vec::new();
        self.model.assign_parameter_storage(&pieces, &mut res, true, arch.types(), arch.manage()).ok()?;
        let stack = arch.manage().get_stack_space()?;
        let ptr = self.ptr;
        let loc = |p: &ParameterPieces| -> Option<Loc> {
            let space = p.addr.get_space()?;
            if Rc::ptr_eq(space, stack) {
                return Some(Loc::Stack(p.addr.get_offset() as i64));
            }
            Some(Loc::Reg((u16::try_from(space.get_index()).ok()?, p.addr.get_offset(), ptr)))
        };
        if let Some(Loc::Reg(k)) = res.first().and_then(loc) {
            self.ret = Some(k);
        }
        self.params = res.iter().skip(1).map_while(loc).collect();
        let base = stack.get_spacebase(0).ok()?;
        let space = base.space.as_ref()?;
        self.sp = Some((u16::try_from(space.get_index()).ok()?, base.offset, base.size));
        let pop = i64::from(self.model.get_extra_pop());
        self.extrapop = (0..=64).contains(&pop).then_some(pop);
        (self.params.len() >= 4).then_some(())
    }

    /// Decode the code reachable from the entry point and from every code
    /// pointer the image holds, and keep the p-code the walk reads. The budget
    /// of [`MAX_INSNS`] instructions is shared between the functions the
    /// entry reaches and those the pointers name, each taken nearest first in
    /// call depth.
    fn lift(&mut self) {
        self.text = self
            .ctx
            .file
            .sections()
            .filter(|s| s.kind() == SectionKind::Text)
            .map(|s| (s.address(), s.address().saturating_add(s.data().map_or(0, |d| d.len() as u64))))
            .collect();
        let pointers = self.code_pointers();
        self.starts.push(self.entry);
        self.starts.extend(&pointers);
        self.decode([VecDeque::from([self.entry]), VecDeque::from(pointers)], MAX_INSNS);
        self.layout();
    }

    /// The instruction a code pointer names: on ARM its low bit selects
    /// Thumb and is not part of the address.
    fn code_target(&self, pointer: u64) -> u64 {
        pointer & !self.thumb_bit
    }

    fn text_end(&self, addr: u64) -> Option<u64> {
        self.text.iter().find(|&&(lo, hi)| addr >= lo && addr < hi).map(|&(_, hi)| hi)
    }

    /// Decode from the addresses in `queues`, alternating between the two, up
    /// to `budget` instructions in all. A function is decoded in address
    /// order from its start: past a return or jump only while a branch
    /// already seen lands further on, so the cases of a jump table are
    /// included and the data behind a function is not.
    fn decode(&mut self, mut queues: [VecDeque<u64>; 2], budget: usize) {
        let arch = self.ctx.arch;
        let code = Rc::clone(&self.code);
        let mut lifter =
            Lifter { ops: std::mem::take(&mut self.ops), code: &code, spaces: std::mem::take(&mut self.spaces) };
        let mut turn = 0;
        let _scope = arch.translate().decode_context_scope();
        while queues.iter().any(|q| !q.is_empty()) && self.insns.len() < budget {
            turn = if queues[1 - turn].is_empty() { turn } else { 1 - turn };
            let Some(start) = queues[turn].pop_front() else { continue };
            let Some(end) = self.text_end(start) else { continue };
            let mut addr = start;
            let mut furthest = start;
            while addr < end && self.insns.len() < budget && self.seen.insert(addr) {
                let first = lifter.ops.len();
                let decoded = arch.translate().one_instruction(&mut lifter, &Address::new(Rc::clone(&code), addr));
                let Some(next) = decoded.ok().filter(|len| *len > 0).and_then(|len| addr.checked_add(len as u64)) else {
                    lifter.ops.truncate(first);
                    break;
                };
                let mut conditional = false;
                let mut ends = false;
                for op in &lifter.ops[first..] {
                    ends |= matches!(op.opc, OpCode::CPUI_RETURN | OpCode::CPUI_BRANCH);
                    let referenced = op.ins[0].filter(|v| {
                        op.opc == OpCode::CPUI_COPY && v.kind == SpaceKind::Const && v.size == self.ptr && v.offset != next
                    });
                    if let Some(pointer) = referenced.filter(|v| self.text_end(v.offset).is_some()) {
                        queues[turn].push_back(self.code_target(pointer.offset));
                    }
                    let Some(dest) = op.ins[0] else { continue };
                    let jump = matches!(op.opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH);
                    if jump && (dest.kind != SpaceKind::Ram || (dest.offset >= addr && dest.offset <= next)) {
                        conditional = true;
                    } else if jump {
                        self.branches.push((addr, dest.offset));
                        furthest = furthest.max(dest.offset);
                        if dest.offset < start {
                            queues[turn].push_back(dest.offset);
                        }
                    } else if op.opc == OpCode::CPUI_CALL && dest.kind == SpaceKind::Ram {
                        self.starts.push(dest.offset);
                        queues[turn].push_back(dest.offset);
                    }
                }
                self.insns.push(Insn {
                    addr,
                    len: (next - addr) as u32,
                    first: first as u32,
                    count: (lifter.ops.len() - first) as u32,
                    conditional,
                    reach: Reach::Fallthrough,
                    lead: Lead::None,
                });
                if ends && furthest < next {
                    break;
                }
                addr = next;
            }
        }
        self.ops = lifter.ops;
        self.spaces = lifter.spaces;
    }

    /// Put the decoded instructions in address order and mark how each is
    /// reached and where blocks and functions begin: a function at a known
    /// start and after a return, a jump or a gap that no branch lands behind,
    /// a block at a branch target and after a branch.
    fn layout(&mut self) {
        self.insns.sort_by_key(|i| i.addr);
        for insn in &mut self.insns {
            insn.reach = Reach::Fallthrough;
        }
        for b in 0..self.branches.len() {
            let (from, to) = self.branches[b];
            if let Some(i) = self.index_of(to) {
                let reach = &mut self.insns[i].reach;
                if from >= to {
                    *reach = Reach::Back;
                } else if *reach == Reach::Fallthrough {
                    *reach = Reach::Forward;
                }
            }
        }
        self.starts.sort_unstable();
        self.starts.dedup();
        for s in 0..self.starts.len() {
            if let Some(i) = self.index_of(self.starts[s]) {
                self.insns[i].reach = Reach::Start;
            }
        }
        let mut transfers = false;
        let mut stops = true;
        let mut prev_end = None;
        for i in 0..self.insns.len() {
            let gap = prev_end != Some(self.insns[i].addr);
            let reach = self.insns[i].reach;
            let landed = matches!(reach, Reach::Forward | Reach::Back);
            self.insns[i].lead = if reach == Reach::Start || ((gap || stops) && !landed) {
                Lead::Function
            } else if landed || transfers || gap {
                Lead::Block
            } else {
                Lead::None
            };
            let (addr, next) = (self.insns[i].addr, self.insns[i].addr + u64::from(self.insns[i].len));
            transfers = false;
            stops = false;
            for op in self.ops_of(&self.insns[i]) {
                let internal = op.ins[0].is_some_and(|d| d.kind != SpaceKind::Ram || (d.offset >= addr && d.offset <= next));
                match op.opc {
                    OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH if internal => {}
                    OpCode::CPUI_CBRANCH => (transfers, stops) = (true, false),
                    OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => (transfers, stops) = (true, true),
                    _ => {}
                }
            }
            prev_end = Some(next);
        }
        self.leaf_writes.clear();
        for s in 0..self.starts.len() {
            let start = self.starts[s];
            if let Some(writes) = self.leaf_writes_of(start) {
                self.leaf_writes.insert(start, writes);
            }
        }
    }

    /// Every pointer-sized, pointer-aligned value of a data section that
    /// points into code: the functions an interface instance, a descriptor or
    /// a callback table names.
    fn code_pointers(&self) -> Vec<u64> {
        let step = self.ptr as usize;
        let little = self.ctx.file.is_little_endian();
        let mut out = Vec::new();
        for section in self.ctx.file.sections() {
            if !matches!(section.kind(), SectionKind::Data | SectionKind::ReadOnlyData) {
                continue;
            }
            let Ok(data) = section.data() else { continue };
            for chunk in data.chunks_exact(step) {
                let value = chunk.iter().enumerate().fold(0u64, |acc, (i, b)| {
                    let shift = if little { i } else { step - 1 - i };
                    acc | u64::from(*b) << (8 * shift)
                });
                if self.text_end(value).is_some() {
                    out.push(self.code_target(value));
                }
            }
        }
        out
    }

    fn index_of(&self, addr: u64) -> Option<usize> {
        self.insns.binary_search_by_key(&addr, |i| i.addr).ok()
    }

    fn is_start(&self, addr: u64) -> bool {
        self.index_of(addr).is_some_and(|i| self.insns[i].reach == Reach::Start)
    }

    fn ops_of(&self, insn: &Insn) -> &[Op] {
        &self.ops[insn.first as usize..(insn.first + insn.count) as usize]
    }

    /// The registers the function at `start` writes, when it calls nothing: a
    /// call to it leaves every other register as it was.
    fn leaf_writes_of(&self, start: u64) -> Option<Vec<Key>> {
        let first = self.index_of(start)?;
        let mut writes: Vec<Key> = Vec::new();
        for insn in self.insns.get(first..)?.iter().take(MAX_LEAF_INSNS) {
            if insn.addr != start && insn.lead == Lead::Function {
                return Some(writes);
            }
            for op in self.ops_of(insn) {
                match op.opc {
                    OpCode::CPUI_CALLOTHER if self.idtr_op.is_some() && op.ins[0].map(|v| v.offset) == self.idtr_op => {}
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCHIND => return None,
                    OpCode::CPUI_BRANCH if op.ins[0].is_some_and(|d| d.kind == SpaceKind::Ram && self.is_start(d.offset)) => {
                        return None;
                    }
                    _ => {}
                }
                if let Some(out) = op.out.filter(|o| o.kind == SpaceKind::Other && !writes.contains(&o.key())) {
                    writes.push(out.key());
                }
            }
            if writes.len() > MAX_LEAF_WRITES {
                return None;
            }
        }
        None
    }

    /// Walk the module as the kind `forced` names, or as each kind it could be
    /// until one reading resolves service calls: a module whose calls go
    /// through `EFI_PEI_SERVICES` is a PEIM whatever its entry was read as.
    pub(super) fn run(mut self, forced: Option<Kind>) -> (Kind, Found) {
        if let Some(kind) = forced {
            return (kind, self.walk(kind));
        }
        let dxe = self.walk(Kind::Dxe);
        let calls = |found: &Found, owner: &str| found.owners.get(owner).copied().unwrap_or(0);
        if calls(&dxe, PEI_SERVICES) > calls(&dxe, BOOT_SERVICES) {
            return (Kind::Pei, self.walk(Kind::Pei));
        }
        if dxe.service_calls() > 0 {
            return (Kind::Dxe, dxe);
        }
        for kind in [Kind::Pei, Kind::Mm] {
            let found = self.walk(kind);
            if found.service_calls() > 0 {
                return (kind, found);
            }
        }
        (Kind::Dxe, dxe)
    }

    fn walk(&mut self, kind: Kind) -> Found {
        self.learned = Learned::default();
        let mut seeds = BTreeMap::new();
        for (i, seed) in kind.seeds().into_iter().enumerate() {
            if let Some(spec) = seed {
                seeds.insert(i as u8, Some(Rc::from(spec)));
            }
        }
        self.learned.seeds.insert(self.entry, seeds);
        for (addr, spec) in self.returns {
            self.learned.rets.insert(*addr, Some(Rc::from(*spec)));
        }
        for _ in 0..MAX_ROUNDS {
            self.round();
            if !self.wanted.is_empty() {
                let wanted = std::mem::take(&mut self.wanted);
                self.starts.extend(&wanted);
                self.decode([VecDeque::from(wanted), VecDeque::new()], MAX_INSNS + MAX_MEMBER_INSNS);
                self.layout();
                self.changed = true;
            }
            if !self.changed {
                break;
            }
        }
        self.finish()
    }

    fn finish(&mut self) -> Found {
        let mut found = std::mem::take(&mut self.found);
        for (addr, spec) in &self.learned.globals {
            if let Some(spec) = spec {
                found.globals.insert(*addr, spec.to_string());
            }
        }
        for addr in &self.learned.getters {
            found.functions.insert(
                *addr,
                Named { name: "GetPeiServicesTablePointer".to_string(), decl: Some("EFI_PEI_SERVICES**()".to_string()) },
            );
        }
        for (addr, wrapper) in &self.learned.wrappers {
            let Some(wrapper) = wrapper else { continue };
            let short = self.fn_insns.get(addr).is_some_and(|n| *n <= WRAPPER_INSNS);
            let single = self.fn_services.get(addr) == Some(&1);
            if wrapper.service.owner != PEI_SERVICES || !short || !single || found.functions.contains_key(addr) {
                continue;
            }
            found.functions.insert(
                *addr,
                Named { name: format!("PeiServices{}", wrapper.service.member), decl: wrapper_decl(wrapper, None) },
            );
        }
        found
    }

    /// One pass over every function, in address order.
    fn round(&mut self) {
        self.changed = false;
        self.found = Found::default();
        self.fn_insns.clear();
        self.fn_services.clear();
        let mut first = 0;
        while first < self.insns.len() {
            let rest = &self.insns[first + 1..];
            let end = first + 1 + rest.iter().position(|i| i.lead == Lead::Function).unwrap_or(rest.len());
            self.function(first, end);
            first = end;
        }
    }

    /// The function made of instructions `first..end`: find the state at the
    /// head of each of its blocks, then read its calls with those states.
    ///
    /// A block's state is the meet of what its predecessors leave. Without a
    /// loop one pass in address order sees every predecessor first. With one,
    /// the passes repeat without recording anything (`dry`) until no state
    /// shrinks; a block nothing reached by then (the cases of a jump table)
    /// starts from nothing and the passes resume. If that does not settle in
    /// [`MAX_SWEEPS`] passes, every block but the first starts from nothing.
    fn function(&mut self, first: usize, end: usize) {
        let func = self.insns[first].addr;
        self.fn_insns.insert(func, (end - first) as u32);
        let leaders: Vec<usize> = (first..end).filter(|i| *i == first || self.insns[*i].lead != Lead::None).collect();
        let mut heads: Vec<Option<State>> = vec![None; leaders.len()];
        heads[0] = Some(self.enter(func));
        if !self.insns[first..end].iter().any(|i| i.reach == Reach::Back) {
            self.sweep(func, &leaders, end, &mut heads, true);
            return;
        }
        self.dry = true;
        let mut settled = false;
        for _ in 0..MAX_SWEEPS {
            if self.sweep(func, &leaders, end, &mut heads, true) {
                continue;
            }
            if heads.iter().all(Option::is_some) {
                settled = true;
                break;
            }
            for head in heads.iter_mut().filter(|h| h.is_none()) {
                *head = Some(State::default());
            }
        }
        self.dry = false;
        if !settled {
            for head in heads.iter_mut().skip(1) {
                *head = Some(State::default());
            }
        }
        self.sweep(func, &leaders, end, &mut heads, false);
    }

    /// Run each block that has a state at its head; with `propagate`, hand
    /// what a block leaves to its successors inside the function. Returns
    /// whether a head state changed. A block with no state runs from nothing
    /// when the pass records (`!dry`).
    fn sweep(&mut self, func: u64, leaders: &[usize], end: usize, heads: &mut [Option<State>], propagate: bool) -> bool {
        let mut changed = false;
        for (b, &lead) in leaders.iter().enumerate() {
            let Some(mut state) = heads[b].clone().or_else(|| (!self.dry).then(State::default)) else { continue };
            if heads[b].is_none() && propagate {
                heads[b] = Some(State::default());
            }
            let stop = leaders.get(b + 1).copied().unwrap_or(end);
            let mut hand = |to: usize, state: &State| {
                let Some(head) = leaders.binary_search(&to).ok().filter(|_| propagate).map(|block| &mut heads[block]) else {
                    return;
                };
                let before = head.as_ref().map(|h| (h.regs.len(), h.stack.len()));
                match head {
                    Some(head) => head.meet(state),
                    None => *head = Some(state.clone()),
                }
                changed |= before != head.as_ref().map(|h| (h.regs.len(), h.stack.len()));
            };
            let falls = self.block(func, lead..stop, &mut state, &mut hand);
            let last = &self.insns[stop - 1];
            if falls && stop < end && last.addr + u64::from(last.len) == self.insns[stop].addr {
                hand(stop, &state);
            }
        }
        changed
    }

    /// Run the instructions of one block over `state`, handing the state at
    /// each branch to `hand` with the index of the instruction it lands on.
    /// Returns whether control falls out of the block's end.
    fn block(
        &mut self,
        func: u64,
        insns: std::ops::Range<usize>,
        state: &mut State,
        hand: &mut dyn FnMut(usize, &State),
    ) -> bool {
        let mut falls = true;
        for i in insns {
            let (addr, next) = (self.insns[i].addr, self.insns[i].addr + u64::from(self.insns[i].len));
            let conditional = self.insns[i].conditional;
            self.temps.clear();
            for o in self.insns[i].first..self.insns[i].first + self.insns[i].count {
                let op = self.ops[o as usize];
                let dest = op.ins[0];
                match op.opc {
                    OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                        let Some(dest) = dest.filter(|d| d.kind == SpaceKind::Ram && (d.offset < addr || d.offset > next)) else {
                            continue;
                        };
                        let jump = op.opc == OpCode::CPUI_BRANCH;
                        match self.index_of(dest.offset) {
                            Some(to) if self.insns[to].reach != Reach::Start => hand(to, state),
                            Some(_) if jump => self.direct_call(state, func, addr, dest.offset, false),
                            _ => {}
                        }
                        falls = !jump;
                    }
                    OpCode::CPUI_BRANCHIND => {
                        if self.indirect_call(state, func, addr, dest).is_some() {
                            self.found.call_decls.remove(&addr);
                        }
                        falls = false;
                    }
                    OpCode::CPUI_CALL => match dest.filter(|d| d.kind == SpaceKind::Ram) {
                        Some(dest) => self.direct_call(state, func, addr, dest.offset, true),
                        None => self.after_call(state, None, None),
                    },
                    OpCode::CPUI_CALLIND => {
                        let service = self.indirect_call(state, func, addr, dest);
                        let returns = service
                            .and_then(|s| sig_parts(&s.sig).and_then(|(ret, ..)| self.typed_pointer(ret)));
                        self.after_call(state, returns, None);
                    }
                    OpCode::CPUI_RETURN => {
                        self.returned(func, state);
                        falls = false;
                    }
                    _ => self.exec(state, func, &op, conditional),
                }
            }
        }
        falls
    }

    /// A call or jump through `target`: read it as a service call when the
    /// target is a table member.
    fn indirect_call(&mut self, state: &mut State, func: u64, site: u64, target: Option<Vn>) -> Option<Rc<Service>> {
        let Some(Val::Func(service)) = target.and_then(|v| self.value(state, &v)) else { return None };
        let args = self.args(state);
        self.service_call(state, func, site, &service, args, None);
        Some(service)
    }

    /// Record what the function returns: a type it returns on every path.
    fn returned(&mut self, func: u64, state: &State) {
        if self.dry {
            return;
        }
        let spec = match self.ret.and_then(|k| state.reg(&k)) {
            Some(Val::Typed(spec, 0)) => Some(Rc::clone(spec)),
            _ => None,
        };
        if self.learned.reads_idt.contains(&func) && spec.as_deref() == Some("EFI_PEI_SERVICES**") {
            self.learned.getters.insert(func);
        }
        match self.learned.rets.get(&func) {
            None => {
                self.changed |= spec.is_some();
                self.learned.rets.insert(func, spec);
            }
            Some(Some(old)) if Some(old) != spec.as_ref() => {
                self.learned.rets.insert(func, None);
            }
            _ => {}
        }
    }

    /// The state a function starts in: the stack pointer and its parameters.
    /// A parameter a caller passes a typed pointer in starts typed, except in
    /// a function that forwards its parameters to a service, whose parameters
    /// must stay recognizable as such.
    fn enter(&self, func: u64) -> State {
        let mut state = State::default();
        if let Some(sp) = self.sp {
            state.regs.push((sp, Val::Stack(0)));
        }
        let forwards = matches!(self.learned.wrappers.get(&func), Some(Some(_)));
        let seeds = self.learned.seeds.get(&func).filter(|_| !forwards);
        for (i, loc) in self.params.iter().enumerate() {
            let i = i as u8;
            let val = match seeds.and_then(|s| s.get(&i)) {
                Some(Some(spec)) => Val::Typed(Rc::clone(spec), 0),
                _ => Val::Param(i),
            };
            match *loc {
                Loc::Reg(k) => state.regs.push((k, val)),
                Loc::Stack(off) => state.stack.push((off, val)),
            }
        }
        state
    }

    fn args(&self, state: &State) -> Vec<Option<Val>> {
        let sp = self.sp.and_then(|k| match state.reg(&k) {
            Some(Val::Stack(off)) => Some(*off),
            _ => None,
        });
        self.params
            .iter()
            .map(|loc| match *loc {
                Loc::Reg(k) => state.reg(&k).cloned(),
                Loc::Stack(off) => state.slot(sp?.checked_add(off)?).cloned(),
            })
            .collect()
    }

    /// A call (or, with `returns` false, a tail jump) to the function at
    /// `callee`: hand it the typed pointers it is passed, and read the call
    /// as a service call when the callee forwards to one.
    fn direct_call(&mut self, state: &mut State, func: u64, site: u64, callee: u64, returns: bool) {
        let args = self.args(state);
        for (i, arg) in args.iter().enumerate() {
            let spec = match arg {
                Some(Val::Typed(spec, 0)) if spec.ends_with('*') => Some(Rc::clone(spec)),
                _ => None,
            };
            if self.dry {
                break;
            }
            let slot = self.learned.seeds.entry(callee).or_default();
            match slot.get(&(i as u8)) {
                None if spec.is_some() => {
                    slot.insert(i as u8, spec);
                    self.changed = true;
                }
                Some(Some(old)) if Some(old) != spec.as_ref() && callee != self.entry => {
                    slot.insert(i as u8, None);
                    self.changed = true;
                }
                _ => {}
            }
        }
        if let Some(Some(wrapper)) = self.learned.wrappers.get(&callee).cloned() {
            let mapped = wrapper
                .args
                .iter()
                .map(|src| match src {
                    Source::Param(i) => args.get(*i as usize).cloned().flatten(),
                    Source::Const(c) => Some(Val::Const(*c)),
                    Source::Unknown => None,
                })
                .collect();
            self.service_call(state, func, site, &wrapper.service, mapped, Some(&wrapper));
        }
        if returns {
            let ret = self.learned.rets.get(&callee).cloned().flatten();
            self.after_call(state, ret, Some(callee));
        }
    }

    /// A pointer type worth carrying: one whose pointee is a struct the
    /// tables know.
    fn typed_pointer(&self, spec: &str) -> Option<Rc<str>> {
        let base = spec.trim_end_matches('*');
        (base.len() < spec.len() && self.efi.fields_of(base).is_some()).then(|| Rc::from(spec))
    }

    /// What a call leaves: stack addresses, the registers the prototype model
    /// preserves and, after a call to a function that calls nothing, every
    /// register that function does not write.
    fn after_call(&mut self, state: &mut State, returns: Option<Rc<str>>, callee: Option<u64>) {
        let leaf = callee.and_then(|c| self.leaf_writes.get(&c)).cloned();
        let mut regs = std::mem::take(&mut state.regs);
        regs.retain(|(key, val)| {
            let untouched = leaf.as_ref().is_some_and(|writes| !writes.iter().any(|w| overlaps(w, key)));
            untouched || matches!(val, Val::Stack(_)) || self.is_unaffected(*key)
        });
        state.regs = regs;
        if let Some(sp) = self.sp {
            let moved = match (state.reg(&sp), self.extrapop) {
                (Some(Val::Stack(off)), Some(pop)) => Some(Val::Stack(off.wrapping_add(pop))),
                _ => None,
            };
            state.set_reg(sp, moved);
        }
        if let (Some(ret), Some(spec)) = (self.ret, returns) {
            state.set_reg(ret, Some(Val::Typed(spec, 0)));
        }
    }

    fn is_unaffected(&mut self, key: Key) -> bool {
        if let Some(known) = self.unaffected.get(&key) {
            return *known;
        }
        let answer = self.spaces.get(usize::from(key.0)).and_then(|s| s.as_ref()).is_some_and(|space| {
            let addr = Address::new(Rc::clone(space), key.1);
            self.model.has_effect(&addr, key.2 as i32) == effect_type::UNAFFECTED
        });
        self.unaffected.insert(key, answer);
        answer
    }

    fn value(&self, state: &State, vn: &Vn) -> Option<Val> {
        match vn.kind {
            SpaceKind::Const => Some(Val::Const(vn.offset)),
            SpaceKind::Ram => self.load(state, &Val::Const(vn.offset), vn.size),
            SpaceKind::Unique => self.temps.iter().find(|(k, _)| *k == vn.key()).map(|(_, v)| v.clone()),
            SpaceKind::Other => state.reg(&vn.key()).cloned(),
        }
    }

    /// One p-code op that is not a control transfer. An op the walk does not
    /// model, and a write that may not happen (`conditional`), leave their
    /// output unknown.
    fn exec(&mut self, state: &mut State, func: u64, op: &Op, conditional: bool) {
        if op.opc == OpCode::CPUI_STORE {
            let at = op.ins[1].and_then(|v| self.value(state, &v));
            let val = op.ins[2].and_then(|v| self.value(state, &v));
            if let (Some(at), false) = (at, conditional) {
                self.store(state, &at, val, op.ins[2].map_or(0, |v| v.size));
            }
            return;
        }
        let value = self.compute(state, func, op);
        let Some(out) = op.out else { return };
        let value = value.filter(|_| !conditional).and_then(|value| match value {
            Val::Const(c) if out.size >= 8 => Some(Val::Const(c)),
            Val::Const(c) if out.size >= 4 => Some(Val::Const(c & ((1u64 << (out.size * 8)) - 1))),
            Val::Const(_) => None,
            Val::Idtr => Some(Val::Idtr),
            other => (out.size == self.ptr).then_some(other),
        });
        match out.kind {
            SpaceKind::Ram if !conditional => self.store(state, &Val::Const(out.offset), value, out.size),
            SpaceKind::Unique => {
                self.temps.retain(|(k, _)| *k != out.key());
                if let Some(value) = value {
                    self.temps.push((out.key(), value));
                }
            }
            SpaceKind::Other => state.set_reg(out.key(), value),
            _ => {}
        }
    }

    fn compute(&mut self, state: &mut State, func: u64, op: &Op) -> Option<Val> {
        let a = op.ins[0].and_then(|v| self.value(state, &v));
        match op.opc {
            OpCode::CPUI_COPY => a,
            OpCode::CPUI_INT_ZEXT => a.filter(|v| matches!(v, Val::Const(_))),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let negate = op.opc == OpCode::CPUI_INT_SUB;
                let b = op.ins[1].and_then(|v| self.value(state, &v));
                match (a?, b?) {
                    (Val::Const(a), Val::Const(b)) => Some(Val::Const(if negate { a.wrapping_sub(b) } else { a.wrapping_add(b) })),
                    (base, Val::Const(c)) => offset(base, c, op.ins[1]?.size, negate),
                    (Val::Const(c), base) if !negate => offset(base, c, op.ins[0]?.size, false),
                    _ => None,
                }
            }
            OpCode::CPUI_INT_AND => match (a?, op.ins[1].and_then(|v| self.value(state, &v))?) {
                (Val::Const(a), Val::Const(b)) => Some(Val::Const(a & b)),
                (Val::Stack(off), Val::Const(_)) => Some(Val::Stack(off)),
                (Val::Func(service), Val::Const(mask)) if mask & 0xffff_fffe == 0xffff_fffe => Some(Val::Func(service)),
                _ => None,
            },
            OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_OR => {
                let xor = op.opc == OpCode::CPUI_INT_XOR;
                let (x, y) = (op.ins[0]?, op.ins[1]?);
                if xor && x.kind != SpaceKind::Const && x.key() == y.key() {
                    return Some(Val::Const(0));
                }
                match (a?, self.value(state, &y)?) {
                    (Val::Const(a), Val::Const(b)) => Some(Val::Const(if xor { a ^ b } else { a | b })),
                    _ => None,
                }
            }
            OpCode::CPUI_LOAD => {
                let at = op.ins[1].and_then(|v| self.value(state, &v))?;
                self.load(state, &at, op.out?.size)
            }
            OpCode::CPUI_CALLOTHER => {
                if self.idtr_op.is_some() && op.ins[0].map(|v| v.offset) == self.idtr_op {
                    self.learned.reads_idt.insert(func);
                    return Some(Val::Idtr);
                }
                None
            }
            _ => None,
        }
    }

    fn load(&self, state: &State, at: &Val, size: u32) -> Option<Val> {
        if size != self.ptr {
            return None;
        }
        match at {
            Val::Const(addr) => match self.learned.globals.get(addr) {
                Some(Some(spec)) => Some(Val::Typed(Rc::clone(spec), 0)),
                Some(None) => None,
                None => self.read_const(*addr, size).map(Val::Const),
            },
            Val::Stack(off) => state.slot(*off).cloned(),
            Val::Typed(spec, off) => self.deref(spec, *off),
            Val::IdtBase(off) if *off == -i64::from(self.ptr) => Some(Val::Typed(Rc::from("EFI_PEI_SERVICES**"), 0)),
            _ => None,
        }
    }

    /// What a pointer of type `spec`, moved by `off`, points at.
    fn deref(&self, spec: &str, off: i64) -> Option<Val> {
        let inner = spec.strip_suffix('*')?;
        if inner.ends_with('*') {
            return (off == 0).then(|| Val::Typed(Rc::from(inner), 0));
        }
        let layout = self.efi.layout(inner)?;
        let member = layout.iter().find(|m| m.offset == off)?;
        if is_function(&member.spec) {
            return Some(Val::Func(Rc::new(Service {
                owner: inner.to_string(),
                member: member.name.clone(),
                sig: member.spec.clone(),
            })));
        }
        member.spec.ends_with('*').then(|| Val::Typed(Rc::from(member.spec.as_str()), 0))
    }

    fn store(&mut self, state: &mut State, at: &Val, val: Option<Val>, size: u32) {
        match at {
            Val::Stack(off) => {
                let lo = off.wrapping_sub(i64::from(self.ptr));
                let hi = off.wrapping_add(i64::from(size));
                state.stack.retain(|(k, _)| *k <= lo || *k >= hi);
                match val {
                    Some(Val::Idtr) => state.set_slot(off.wrapping_add(2), Val::IdtBase(0)),
                    Some(val) if size == self.ptr => state.set_slot(*off, val),
                    _ => {}
                }
            }
            Val::Const(addr) if size == self.ptr => {
                if let Some(Val::Typed(spec, 0)) = val {
                    self.note_global(*addr, &spec);
                }
            }
            _ => {}
        }
    }

    fn note_global(&mut self, addr: u64, spec: &str) {
        if self.dry || !self.sections.iter().any(|&(lo, hi, writable)| writable && addr >= lo && addr < hi) {
            return;
        }
        match self.learned.globals.get(&addr) {
            None => {
                self.learned.globals.insert(addr, Some(Rc::from(spec)));
                self.changed = true;
            }
            Some(Some(old)) if &**old != spec => {
                self.learned.globals.insert(addr, None);
                self.changed = true;
            }
            _ => {}
        }
    }

    fn readable(&self, addr: u64, len: u64) -> bool {
        self.sections.iter().any(|&(lo, hi, _)| addr >= lo && addr.checked_add(len).is_some_and(|end| end <= hi))
    }

    fn read(&self, addr: u64, size: u32) -> Option<u64> {
        if !self.readable(addr, u64::from(size)) {
            return None;
        }
        self.ctx.arch.read_loadimage_value(&Address::new(Rc::clone(&self.code), addr), size as i32).ok()
    }

    /// A pointer-sized constant of a section the module does not write.
    fn read_const(&self, addr: u64, size: u32) -> Option<u64> {
        let fixed = self.sections.iter().any(|&(lo, hi, writable)| !writable && addr >= lo && addr < hi);
        fixed.then(|| self.read(addr, size)).flatten()
    }

    fn read_guid(&self, addr: u64) -> Option<[u8; 16]> {
        let mut raw = [0u8; 16];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = self.read(addr.checked_add(i as u64)?, 1)? as u8;
        }
        (raw.iter().filter(|&&b| b == 0).count() < 8).then_some(raw)
    }

    /// Make the pointer `target` holds address read as type `spec`.
    fn set_pointee(&mut self, state: &mut State, target: Option<&Val>, spec: &str) {
        match target {
            Some(Val::Const(addr)) => self.note_global(*addr, spec),
            Some(Val::Stack(off)) => state.set_slot(*off, Val::Typed(Rc::from(spec), 0)),
            _ => {}
        }
    }

    /// The GUID at `addr` and the struct of its interface, when one is known.
    fn interface_at(&mut self, addr: u64) -> Option<([u8; 16], Option<String>)> {
        let raw = self.read_guid(addr)?;
        let name = super::guid_names().get(&raw).copied();
        let known = name.and_then(protocols::interface_of).map(str::to_string);
        let interface = known.or_else(|| self.learned.synthesized.get(&raw).cloned()).or_else(|| {
            let (name, fields) = self.foreign.get(&raw)?;
            self.efi.define(name, fields.clone());
            Some(name.clone())
        });
        if name.is_none() && !self.dry {
            self.found.guids.entry(addr).or_insert_with(|| format!("gProtocol_{:08X}_Guid", guid_data1(&raw)));
        }
        Some((raw, interface.filter(|i| self.efi.get(i).is_some())))
    }

    /// A call through the table member `service` with `args`; `via` is the
    /// forwarding function a direct call went through.
    fn service_call(
        &mut self,
        state: &mut State,
        func: u64,
        site: u64,
        service: &Rc<Service>,
        args: Vec<Option<Val>>,
        via: Option<&Wrapper>,
    ) {
        let Some((ret, params, varargs)) = sig_parts(&service.sig) else { return };
        if !self.dry {
            *self.found.owners.entry(service.owner.clone()).or_insert(0) += 1;
            *self.fn_services.entry(func).or_insert(0) += 1;
            if self.trace {
                let shown: Vec<_> = args.iter().take(6).collect();
                self.found.trace.push(format!("{site:#x} in {func:#x}: {}.{} {shown:x?}", service.owner, service.member));
            }
            self.note_wrapper(func, service, &args, params.len());
        }
        for (j, (ty, _)) in params.iter().enumerate() {
            let Some(pointee) = ty.strip_suffix('*') else { continue };
            if self.typed_pointer(pointee).is_some() {
                self.set_pointee(state, args.get(j).and_then(|a| a.as_ref()), pointee);
            }
        }
        if via.is_none() && service.owner == PEI_SERVICES && !varargs && !self.dry {
            self.found.call_decls.insert(site, sig_join(ret, &params, false));
        }
        let role = role(&service.owner, &service.member);
        if self.dry && !matches!(role, Some(Role::Locate { .. })) {
            return;
        }
        match role {
            Some(Role::Locate { guid, out }) => {
                let Some(Some(Val::Const(guid_addr))) = args.get(guid) else { return };
                let Some((raw, interface)) = self.interface_at(*guid_addr) else { return };
                if !self.dry {
                    self.found.lookups.push((func, site, raw));
                }
                let Some(interface) = interface else { return };
                self.set_pointee(state, args.get(out).and_then(|a| a.as_ref()), &format!("{interface}*"));
                if self.dry {
                    return;
                }
                let retyped = format!("{interface}**");
                let decl = match via {
                    Some(wrapper) => wrapper_decl(wrapper, Some((out, retyped))),
                    None => {
                        let mut typed = params.clone();
                        typed.get_mut(out).map(|slot| slot.0 = retyped).map(|()| sig_join(ret, &typed, varargs))
                    }
                };
                if let Some(decl) = decl {
                    self.found.call_decls.insert(site, decl);
                }
            }
            Some(Role::Install { guid, interface }) => {
                if let (Some(Some(Val::Const(g))), Some(Some(Val::Const(i)))) = (args.get(guid), args.get(interface)) {
                    self.install(*g, *i);
                }
            }
            Some(Role::InstallMultiple) => {
                for pair in args.get(1..).unwrap_or_default().chunks_exact(2) {
                    let (Some(Val::Const(g)), Some(Val::Const(i))) = (&pair[0], &pair[1]) else { break };
                    if *g == 0 {
                        break;
                    }
                    self.install(*g, *i);
                }
            }
            Some(Role::PpiList { list, notify }) => {
                if let Some(Some(Val::Const(addr))) = args.get(list) {
                    self.ppi_list(*addr, notify);
                }
            }
            None => {}
        }
    }

    /// Remember `func` as forwarding to `service` when one of the arguments
    /// is its own parameter; two different forwardings cancel.
    fn note_wrapper(&mut self, func: u64, service: &Rc<Service>, args: &[Option<Val>], count: usize) {
        let sources: Vec<Source> = args
            .iter()
            .take(count)
            .map(|a| match a {
                Some(Val::Param(i)) => Source::Param(*i),
                Some(Val::Const(c)) => Source::Const(*c),
                _ => Source::Unknown,
            })
            .collect();
        if !sources.iter().any(|s| matches!(s, Source::Param(_))) {
            return;
        }
        let wrapper = Wrapper { service: Rc::clone(service), args: sources };
        match self.learned.wrappers.get(&func) {
            None => {
                self.learned.wrappers.insert(func, Some(wrapper));
                self.changed = true;
            }
            Some(Some(old)) if *old != wrapper => {
                self.learned.wrappers.insert(func, None);
                self.changed = true;
            }
            _ => {}
        }
    }

    /// Record the interface instance at `addr` installed under the GUID at
    /// `guid_addr`, and the functions it points at.
    fn install(&mut self, guid_addr: u64, addr: u64) {
        if addr == 0 || !self.readable(addr, u64::from(self.ptr)) || self.index_of(addr).is_some() {
            return;
        }
        let Some((raw, interface)) = self.interface_at(guid_addr) else { return };
        let Some(interface) = interface.or_else(|| self.synthesize(&raw, addr)) else { return };
        let Some(layout) = self.efi.layout(&interface) else { return };
        let mut members = Vec::new();
        for member in layout.iter().filter(|m| is_function(&m.spec)) {
            let Some(at) = u64::try_from(member.offset).ok().and_then(|o| addr.checked_add(o)) else { continue };
            let Some(target) = self.read(at, self.ptr).map(|t| self.code_target(t)) else { continue };
            if self.index_of(target).is_none() {
                if self.text_end(target).is_some() && !self.seen.contains(&target) {
                    self.wanted.push(target);
                }
                continue;
            }
            let name = member.name.replace('.', "");
            members.push((name.clone(), target));
            let decl = member.spec.strip_prefix("fn ").map(str::to_string);
            self.name_function(target, format!("{}{name}", protocols::stem(&interface)), decl);
        }
        self.found.installs.entry(addr).or_insert(Install { guid: raw, interface, members });
    }

    /// Name the function at `addr`, start a function there on the next round
    /// and hand it the parameter types of `decl`.
    fn name_function(&mut self, addr: u64, name: String, decl: Option<String>) {
        if let Some(i) = self.index_of(addr).filter(|i| self.insns[*i].reach != Reach::Start) {
            self.insns[i].reach = Reach::Start;
            self.insns[i].lead = Lead::Function;
            self.starts.push(addr);
            self.changed = true;
        }
        if let Some((_, params, _)) = decl.as_deref().and_then(sig_parts) {
            for (i, (ty, _)) in params.iter().enumerate().take(self.params.len()) {
                let Some(spec) = self.typed_pointer(ty) else { continue };
                let slot = self.learned.seeds.entry(addr).or_default();
                if !slot.contains_key(&(i as u8)) {
                    slot.insert(i as u8, Some(spec));
                    self.changed = true;
                }
            }
        }
        self.found.functions.entry(addr).or_insert(Named { name, decl });
    }

    /// A struct for an interface no header describes: one member per pointer
    /// slot of the instance, up to its last function pointer. The scan stops
    /// at another known object and after three slots in a row that are not
    /// function pointers.
    fn synthesize(&mut self, raw: &[u8; 16], addr: u64) -> Option<String> {
        let step = u64::from(self.ptr);
        let mut slots = Vec::new();
        let mut last_function = None;
        let mut gap = 0;
        for i in 0..MAX_SLOTS {
            let at = addr.checked_add(i * step)?;
            let known_guid = self.read_guid(at).is_some_and(|g| super::guid_names().contains_key(&g));
            if i != 0 && (self.found.installs.contains_key(&at) || known_guid) {
                break;
            }
            let Some(value) = self.read(at, self.ptr) else { break };
            let function = self.index_of(self.code_target(value)).is_some_and(|i| self.insns[i].lead == Lead::Function);
            if function {
                last_function = Some(i as usize);
                gap = 0;
            } else {
                gap += 1;
                if gap > 2 {
                    break;
                }
            }
            slots.push(function);
        }
        let last = last_function?;
        let name = match super::guid_names().get(raw) {
            Some(guid_name) => protocols::struct_name_for(guid_name),
            None => format!("PROTOCOL_{:08X}", guid_data1(raw)),
        };
        let fields = slots[..=last]
            .iter()
            .enumerate()
            .map(|(i, function)| {
                let off = i as u64 * step;
                if *function {
                    (format!("Function_{off:02x}"), "FUNC".to_string())
                } else {
                    (format!("Field_{off:02x}"), "UINTN".to_string())
                }
            })
            .collect();
        self.efi.define(&name, fields);
        self.efi.get(&name)?;
        self.learned.synthesized.insert(*raw, name.clone());
        self.changed = true;
        Some(name)
    }

    /// The `EFI_PEI_PPI_DESCRIPTOR` or `EFI_PEI_NOTIFY_DESCRIPTOR` list at
    /// `addr`, up to the entry flagged as its last.
    fn ppi_list(&mut self, addr: u64, notify: bool) {
        let step = u64::from(self.ptr);
        for i in 0..MAX_LIST {
            let Some(at) = i.checked_mul(3 * step).and_then(|o| addr.checked_add(o)) else { return };
            let (Some(flags), Some(guid_addr), Some(third)) =
                (self.read(at, self.ptr), self.read(at + step, self.ptr), self.read(at + 2 * step, self.ptr))
            else {
                return;
            };
            let kind = flags & (PPI_TERMINATE - 1);
            let well_formed = if notify { kind != 0 && kind & !PPI_KIND_NOTIFY == 0 } else { kind == PPI_KIND_PPI };
            if !well_formed || flags >> 32 != 0 {
                return;
            }
            let Some((raw, interface)) = self.interface_at(guid_addr) else { return };
            let core = match super::guid_names().get(&raw) {
                Some(name) => name.strip_prefix('g').unwrap_or(name).strip_suffix("Guid").unwrap_or(name).to_string(),
                None => format!("Ppi_{:08X}", guid_data1(&raw)),
            };
            if notify {
                self.found.descriptors.entry(at).or_insert(Descriptor { name: format!("m{core}NotifyDescriptor"), notify });
                let third = self.code_target(third);
                if self.index_of(third).is_some() {
                    let ppi = interface.map_or("VOID*".to_string(), |i| format!("{i}*"));
                    let decl = format!(
                        "EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_PEI_NOTIFY_DESCRIPTOR* NotifyDescriptor, {ppi} Ppi)"
                    );
                    self.name_function(third, format!("{core}Notify"), Some(decl));
                }
            } else {
                self.found.descriptors.entry(at).or_insert(Descriptor { name: format!("m{core}Descriptor"), notify });
                self.install(guid_addr, third);
            }
            if flags & PPI_TERMINATE != 0 {
                return;
            }
        }
    }
}

/// `base` moved by the `size`-byte constant `c`.
fn offset(base: Val, c: u64, size: u32, negate: bool) -> Option<Val> {
    let bits = size.saturating_mul(8).clamp(8, 64);
    let signed = ((c << (64 - bits)) as i64) >> (64 - bits);
    let delta = if negate { signed.checked_neg()? } else { signed };
    match base {
        Val::Stack(off) => Some(Val::Stack(off.checked_add(delta)?)),
        Val::Typed(spec, off) if spec.ends_with('*') => Some(Val::Typed(spec, off.checked_add(delta)?)),
        Val::IdtBase(off) => Some(Val::IdtBase(off.checked_add(delta)?)),
        _ => None,
    }
}

/// The prototype of a function that forwards its parameters to a service:
/// each parameter takes the type of the service argument it becomes, and one
/// the function does not forward is a word. `retype` replaces the type of one
/// service argument first.
fn wrapper_decl(wrapper: &Wrapper, retype: Option<(usize, String)>) -> Option<String> {
    let (ret, mut params, varargs) = sig_parts(&wrapper.service.sig)?;
    if varargs {
        return None;
    }
    if let Some((slot, ty)) = retype {
        params.get_mut(slot)?.0 = ty;
    }
    let param_of = |s: &Source| if let Source::Param(i) = s { Some(usize::from(*i)) } else { None };
    let count = wrapper.args.iter().filter_map(param_of).max()? + 1;
    let mut own = Vec::with_capacity(count);
    for i in 0..count {
        match wrapper.args.iter().position(|s| param_of(s) == Some(i)) {
            Some(from) => own.push(params.get(from)?.clone()),
            None => own.push(("UINTN".to_string(), String::new())),
        }
    }
    Some(sig_join(ret, &own, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signatures_split_and_join() {
        let (ret, params, varargs) = sig_parts("fn EFI_STATUS(EFI_GUID* Protocol, VOID*, VOID** Interface)").unwrap();
        assert_eq!(ret, "EFI_STATUS");
        assert_eq!(params[0], ("EFI_GUID*".to_string(), "Protocol".to_string()));
        assert_eq!(params[1], ("VOID*".to_string(), String::new()));
        assert!(!varargs);
        assert_eq!(sig_join(ret, &params, false), "EFI_STATUS(EFI_GUID* Protocol, VOID*, VOID** Interface)");
        assert!(sig_parts("fn EFI_STATUS(EFI_HANDLE*, ...)").unwrap().2);
        assert!(sig_parts("FUNC").is_none());
    }

    #[test]
    fn a_forwarding_function_takes_the_service_types() {
        let service = Rc::new(Service {
            owner: PEI_SERVICES.to_string(),
            member: "LocatePpi".to_string(),
            sig: "fn EFI_STATUS(EFI_PEI_SERVICES** PeiServices, EFI_GUID* Guid, UINTN Instance, VOID** PpiDescriptor, VOID** Ppi)".to_string(),
        });
        let wrapper = Wrapper {
            service,
            args: vec![Source::Unknown, Source::Param(0), Source::Const(0), Source::Param(2), Source::Param(3)],
        };
        assert_eq!(
            wrapper_decl(&wrapper, Some((4, "PCD_PPI**".to_string()))).as_deref(),
            Some("EFI_STATUS(EFI_GUID* Guid, UINTN, VOID** PpiDescriptor, PCD_PPI** Ppi)")
        );
    }

    #[test]
    fn roles_cover_the_three_phases() {
        assert!(matches!(role(BOOT_SERVICES, "OpenProtocol"), Some(Role::Locate { guid: 1, out: 2 })));
        assert!(matches!(role("EFI_SMM_SYSTEM_TABLE2", "SmmLocateProtocol"), Some(Role::Locate { guid: 0, out: 2 })));
        assert!(matches!(role("EFI_MM_SYSTEM_TABLE", "MmInstallProtocolInterface"), Some(Role::Install { .. })));
        assert!(matches!(role(PEI_SERVICES, "NotifyPpi"), Some(Role::PpiList { notify: true, .. })));
        assert!(role("EFI_RUNTIME_SERVICES", "LocateProtocol").is_none());
    }

    #[test]
    fn pointer_offsets_are_signed_and_checked() {
        assert_eq!(offset(Val::Stack(-8), 0xffff_ffff_ffff_fff8, 8, false), Some(Val::Stack(-16)));
        assert_eq!(offset(Val::Stack(0), 0x10, 8, true), Some(Val::Stack(-16)));
        assert_eq!(offset(Val::Stack(i64::MAX), 1, 8, false), None);
        assert_eq!(offset(Val::Param(0), 8, 8, false), None);
    }
}
