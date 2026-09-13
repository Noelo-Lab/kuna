//! (kuna `pebnames`) Type the Windows thread-environment segment base, so reads
//! through it render as the `TEB`/`PEB` fields a reverser recognises.
//!
//! # The gap
//!
//! A Windows user-mode thread keeps its Thread Environment Block at the base of
//! a segment: `GS` on x86-64, `FS` on x86. The SLEIGH x86 constructors lower a
//! segment-prefixed memory operand to `GS_OFFSET + disp` / `FS_OFFSET + disp`,
//! where the segment base is an ordinary register input nothing in the function
//! writes. Nothing tells the engine what that register points at, so the
//! anti-debug probe
//!
//! ```text
//! MOV RAX, GS:[0x60]          ; TEB.ProcessEnvironmentBlock
//! MOVZX ECX, byte [RAX+2]     ; PEB.BeingDebugged
//! MOV EAX, [RAX+0xBC]         ; PEB.NtGlobalFlag
//! ```
//!
//! renders as `long long v1; // gs_offset` followed by
//! `*(char *)(*(long long *)(v1 + 0x60) + 2)` -- magic offsets into nothing.
//!
//! # The fix
//!
//! Before heritage, when the function reads the segment-base register and never
//! writes through it, map a type-locked, name-locked local Symbol `TEB *teb` over
//! its input storage. The `TEB` and `PEB` structures are laid out with the real
//! offsets of the fields this knows (the rest are holes), and ordinary
//! pointer-arithmetic and field rendering does everything else:
//! `teb->ProcessEnvironmentBlock->BeingDebugged`,
//! `teb->Self->ProcessEnvironmentBlock->NtGlobalFlag`, `teb->ThreadLocalStoragePointer`.
//!
//! # Why a write through the base declines
//!
//! A store through a typed base gives the stored value the field's type. On x86
//! nearly every such store is an exception frame linking its registration record
//! into `ExceptionList`, and that record's stack layout is recovered from how the
//! untyped base is used: typing the base split MSVC's `{Next, handler, state}`
//! record into unrelated scalars, and modelling the record type instead spread a
//! record-pointer type onto whatever kuna's stack analysis believed was stored
//! there. A function that writes through the base is therefore left as upstream
//! renders it.
//!
//! # Why a wrong name cannot appear
//!
//! * A field is named only when it has the same offset, name and size in every
//!   PDB-derived layout the Vergilius Project publishes for that architecture:
//!   x64 `_TEB`/`_PEB` from XP SP2 to Windows 11 25H2, and x86 `_TEB`/`_PEB` from
//!   XP SP3 to Windows 10 22H2 together with every x64 kernel's WOW64
//!   `_TEB32`/`_PEB32`. A name that changed between releases (`CrossProcessFlags`,
//!   `ApiSetMap`, the x86 `BitField`) is left a hole, and a read in a hole prints
//!   upstream's offset-named `field_0x<off>`.
//! * Both structures grew with nearly every release, so they are variable-length
//!   types carrying the largest published size. Pointer arithmetic therefore never
//!   splits an offset past the end into an index of a neighbouring `TEB` (which
//!   would name that neighbour's field); such a read stays an integer addition.
//! * The segment base is only a TEB in user mode. Kernel code keeps its `KPCR` at
//!   the same base (`gs:[0x188]` is the current thread there) and firmware keeps
//!   nothing there, so the default `auto` acts only on an image the loader proved
//!   is a Windows GUI/console PE; `on` trusts the Windows compiler spec alone
//!   (a raw shellcode blob, a stage bytechunk) and still needs it.
//! * The register is `GS_OFFSET` on x86-64 and `FS_OFFSET` on x86, so the x86-64
//!   `fs:0x28` and x86 `gs:0x14` canaries never reach this; every mode also needs
//!   a Windows compiler spec, which keeps a non-Windows image that does read the
//!   GS base (x86-64) or FS base (x86) untyped.
//! * A Symbol a user already mapped over the register wins, and a `TEB`/`PEB` type
//!   of a different layout already in the program is left alone (the pass declines).
//!   The type-locked Symbol survives a second decompile of the same function, which
//!   recognises it instead of mapping another.
//!
//! # Known limits
//!
//! * A variable that holds `teb->Self` or `teb->ProcessEnvironmentBlock` on one
//!   path and an unrelated value on another takes the pointer type, so the
//!   unrelated value gains a cast; a parameter compared or merged with such a value
//!   can take it too.
//! * The store check sees only the instructions that read the segment register: a
//!   store through a pointer loaded from the TEB (`mov eax,fs:[0x18]; mov [eax],esp`)
//!   is typed, as `teb->Self->ExceptionList = ...`.
//! * `auto` needs `image_windows_user`, which only the object loader writes, so it
//!   never fires in the Ghidra front-end; `on` does.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::dtype::{flags, type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

/// The name the segment-base Symbol is given when it is free.
pub const TEB_SYMBOL_NAME: &str = "teb";

/// (kuna) When to type the segment base: `pebnames off|auto|on`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PebNamesMode {
    /// Upstream: the segment base is an untyped register input.
    #[default]
    Off,
    /// Only on an image the loader identified as a Windows user-mode PE.
    Auto,
    /// On any image whose compiler spec is a Windows one.
    On,
}

impl PebNamesMode {
    /// Does the pass act for an image with this compiler spec and container fact?
    pub fn fires(self, windows_cspec: bool, image_windows_user: bool) -> bool {
        match self {
            PebNamesMode::Off => false,
            PebNamesMode::Auto => windows_cspec && image_windows_user,
            PebNamesMode::On => windows_cspec,
        }
    }
}

/// (kuna) Parse `option pebnames off|auto|on`; the caller writes the live field.
pub struct OptionPebNames;

impl OptionPebNames {
    /// The option name.
    pub const NAME: &'static str = "pebnames";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(PebNamesMode, String)> {
        let mode = match p1 {
            "off" => PebNamesMode::Off,
            "auto" => PebNamesMode::Auto,
            "on" => PebNamesMode::On,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown pebnames value: {other} (expected off|auto|on)"
                )))
            }
        };
        Ok((mode, format!("TEB/PEB segment-base typing set to {p1}")))
    }
}

/// The shape of one known field. A pointer whose target this does not model is
/// `Opaque`: pointer-sized and of unknown type, so the name renders without the
/// field pushing `void *` into whatever variable its value merges with.
#[derive(Clone, Copy)]
enum Kind {
    Opaque,
    TebPtr,
    PebPtr,
    U1,
    U2,
    U4,
    OpaqueArray(int4),
}

/// A known field: offset, name, shape.
type Field = (int4, &'static str, Kind);

/// The per-bitness layout of the two structures.
pub struct Layout {
    /// The segment-base register holding the TEB address.
    pub register: &'static str,
    /// Pointer width.
    pub ptr_size: int4,
    teb_size: int4,
    teb: &'static [Field],
    peb_size: int4,
    peb: &'static [Field],
}

/// The x64 fields every published `_TEB`/`_PEB` layout agrees on (XP SP2 to
/// Windows 11 25H2); the `NT_TIB` header and the `CLIENT_ID` pair are flattened
/// into the `TEB`.
const TEB64: &[Field] = &[
    (0x0, "ExceptionList", Kind::Opaque),
    (0x8, "StackBase", Kind::Opaque),
    (0x10, "StackLimit", Kind::Opaque),
    (0x18, "SubSystemTib", Kind::Opaque),
    (0x20, "FiberData", Kind::Opaque),
    (0x28, "ArbitraryUserPointer", Kind::Opaque),
    (0x30, "Self", Kind::TebPtr),
    (0x38, "EnvironmentPointer", Kind::Opaque),
    (0x40, "UniqueProcess", Kind::Opaque),
    (0x48, "UniqueThread", Kind::Opaque),
    (0x50, "ActiveRpcHandle", Kind::Opaque),
    (0x58, "ThreadLocalStoragePointer", Kind::Opaque),
    (0x60, "ProcessEnvironmentBlock", Kind::PebPtr),
    (0x68, "LastErrorValue", Kind::U4),
    (0x6c, "CountOfOwnedCriticalSections", Kind::U4),
    (0x70, "CsrClientThread", Kind::Opaque),
    (0x78, "Win32ThreadInfo", Kind::Opaque),
    (0x108, "CurrentLocale", Kind::U4),
    (0x1250, "LastStatusValue", Kind::U4),
    (0x1480, "TlsSlots", Kind::OpaqueArray(64)),
    (0x1780, "TlsExpansionSlots", Kind::Opaque),
];

const PEB64: &[Field] = &[
    (0x0, "InheritedAddressSpace", Kind::U1),
    (0x1, "ReadImageFileExecOptions", Kind::U1),
    (0x2, "BeingDebugged", Kind::U1),
    (0x3, "BitField", Kind::U1),
    (0x8, "Mutant", Kind::Opaque),
    (0x10, "ImageBaseAddress", Kind::Opaque),
    (0x18, "Ldr", Kind::Opaque),
    (0x20, "ProcessParameters", Kind::Opaque),
    (0x28, "SubSystemData", Kind::Opaque),
    (0x30, "ProcessHeap", Kind::Opaque),
    (0x38, "FastPebLock", Kind::Opaque),
    (0x58, "KernelCallbackTable", Kind::Opaque),
    (0x70, "TlsExpansionCounter", Kind::U4),
    (0x78, "TlsBitmap", Kind::Opaque),
    (0xb8, "NumberOfProcessors", Kind::U4),
    (0xbc, "NtGlobalFlag", Kind::U4),
    (0xe8, "NumberOfHeaps", Kind::U4),
    (0xf0, "ProcessHeaps", Kind::Opaque),
    (0x118, "OSMajorVersion", Kind::U4),
    (0x11c, "OSMinorVersion", Kind::U4),
    (0x120, "OSBuildNumber", Kind::U2),
    (0x124, "OSPlatformId", Kind::U4),
    (0x2c0, "SessionId", Kind::U4),
];

/// The x86 fields every published 32-bit layout agrees on: `_TEB`/`_PEB` from
/// XP SP3 to Windows 10 22H2 and the WOW64 `_TEB32`/`_PEB32` to Windows 11 25H2.
const TEB32: &[Field] = &[
    (0x0, "ExceptionList", Kind::Opaque),
    (0x4, "StackBase", Kind::Opaque),
    (0x8, "StackLimit", Kind::Opaque),
    (0xc, "SubSystemTib", Kind::Opaque),
    (0x10, "FiberData", Kind::Opaque),
    (0x14, "ArbitraryUserPointer", Kind::Opaque),
    (0x18, "Self", Kind::TebPtr),
    (0x1c, "EnvironmentPointer", Kind::Opaque),
    (0x20, "UniqueProcess", Kind::Opaque),
    (0x24, "UniqueThread", Kind::Opaque),
    (0x28, "ActiveRpcHandle", Kind::Opaque),
    (0x2c, "ThreadLocalStoragePointer", Kind::Opaque),
    (0x30, "ProcessEnvironmentBlock", Kind::PebPtr),
    (0x34, "LastErrorValue", Kind::U4),
    (0x38, "CountOfOwnedCriticalSections", Kind::U4),
    (0x3c, "CsrClientThread", Kind::Opaque),
    (0x40, "Win32ThreadInfo", Kind::Opaque),
    (0xc4, "CurrentLocale", Kind::U4),
    (0xbf4, "LastStatusValue", Kind::U4),
    (0xe10, "TlsSlots", Kind::OpaqueArray(64)),
    (0xf94, "TlsExpansionSlots", Kind::Opaque),
];

const PEB32: &[Field] = &[
    (0x0, "InheritedAddressSpace", Kind::U1),
    (0x1, "ReadImageFileExecOptions", Kind::U1),
    (0x2, "BeingDebugged", Kind::U1),
    (0x4, "Mutant", Kind::Opaque),
    (0x8, "ImageBaseAddress", Kind::Opaque),
    (0xc, "Ldr", Kind::Opaque),
    (0x10, "ProcessParameters", Kind::Opaque),
    (0x14, "SubSystemData", Kind::Opaque),
    (0x18, "ProcessHeap", Kind::Opaque),
    (0x1c, "FastPebLock", Kind::Opaque),
    (0x2c, "KernelCallbackTable", Kind::Opaque),
    (0x3c, "TlsExpansionCounter", Kind::U4),
    (0x40, "TlsBitmap", Kind::Opaque),
    (0x64, "NumberOfProcessors", Kind::U4),
    (0x68, "NtGlobalFlag", Kind::U4),
    (0x88, "NumberOfHeaps", Kind::U4),
    (0x90, "ProcessHeaps", Kind::Opaque),
    (0xa4, "OSMajorVersion", Kind::U4),
    (0xa8, "OSMinorVersion", Kind::U4),
    (0xac, "OSBuildNumber", Kind::U2),
    (0xb0, "OSPlatformId", Kind::U4),
    (0x1d4, "SessionId", Kind::U4),
];

/// The x86-64 layout: the TEB is at the `GS` base.
pub const LAYOUT64: Layout =
    Layout { register: "GS_OFFSET", ptr_size: 8, teb_size: 0x1878, teb: TEB64, peb_size: 0x7d0, peb: PEB64 };

/// The x86 layout: the TEB is at the `FS` base.
pub const LAYOUT32: Layout =
    Layout { register: "FS_OFFSET", ptr_size: 4, teb_size: 0x1038, teb: TEB32, peb_size: 0x488, peb: PEB32 };

impl Layout {
    fn offset_of(&self, fields: &[Field], name: &str) -> int4 {
        fields.iter().find(|f| f.1 == name).map(|f| f.0).unwrap_or(-1)
    }
}

/// The layout and the segment-base register storage for this language, or `None`
/// on anything but a 32/64-bit x86 SLEIGH language.
pub fn resolve_register(data: &Funcdata) -> Option<(&'static Layout, Address)> {
    let manage = data.get_arch().manage();
    let addr_size = manage.get_default_code_space()?.get_addr_size() as int4;
    let layout = match addr_size {
        8 => &LAYOUT64,
        4 => &LAYOUT32,
        _ => return None,
    };
    let st = manage.register_lookup()?.probe_register(layout.register)?;
    if st.size as int4 != layout.ptr_size {
        return None;
    }
    Some((layout, Address::new(st.space.clone()?, st.offset)))
}

/// Is `ct` a completed variable-length structure of `size` bytes whose field
/// `marker` sits at `marker_off`?
fn matches_layout(ct: &Datatype, size: int4, marker: &str, marker_off: int4) -> bool {
    if ct.get_metatype() != type_metatype::TYPE_STRUCT
        || ct.is_incomplete()
        || !ct.is_variable_length()
        || ct.get_size() != size
    {
        return false;
    }
    (0..ct.num_depend()).filter_map(|i| ct.get_field(i)).any(|f| f.name == marker && f.offset == marker_off)
}

/// Is `ct` the `TEB *` this pass builds for `layout`?
pub fn is_teb_pointer(ct: &Datatype, layout: &Layout) -> bool {
    ct.get_metatype() == type_metatype::TYPE_PTR
        && ct.get_ptr_to().is_some_and(|t| {
            t.get_name() == "TEB"
                && matches_layout(
                    &t,
                    layout.teb_size,
                    "ProcessEnvironmentBlock",
                    layout.offset_of(layout.teb, "ProcessEnvironmentBlock"),
                )
        })
}

/// A completed structure already in the program under `name`: `Ok(Some)` when it
/// is this layout, `Err` when the name belongs to something else.
fn existing(types: &dyn TypeFactory, name: &str, size: int4, marker: &str, off: int4) -> Result<Option<Rc<Datatype>>, ()> {
    match types.find_by_name(name) {
        Ok(None) => Ok(None),
        Ok(Some(t)) if matches_layout(&t, size, marker, off) => Ok(Some(t)),
        _ => Err(()),
    }
}

fn fields_of(
    types: &dyn TypeFactory,
    layout: &Layout,
    spec: &[Field],
    teb: Option<&Rc<Datatype>>,
    peb: Option<&Rc<Datatype>>,
) -> Option<Vec<TypeField>> {
    let ps = layout.ptr_size;
    let opaque = types.get_base(ps, type_metatype::TYPE_UNKNOWN).ok()?;
    let mut out = Vec::with_capacity(spec.len());
    for (i, &(off, name, kind)) in spec.iter().enumerate() {
        let ct = match kind {
            Kind::Opaque => Rc::clone(&opaque),
            Kind::TebPtr => match teb {
                Some(t) => types.get_type_pointer(ps, Rc::clone(t), 1).ok()?,
                None => Rc::clone(&opaque),
            },
            Kind::PebPtr => types.get_type_pointer(ps, Rc::clone(peb?), 1).ok()?,
            Kind::U1 => types.get_base(1, type_metatype::TYPE_UINT).ok()?,
            Kind::U2 => types.get_base(2, type_metatype::TYPE_UINT).ok()?,
            Kind::U4 => types.get_base(4, type_metatype::TYPE_UINT).ok()?,
            Kind::OpaqueArray(n) => types.get_type_array(n, Rc::clone(&opaque)).ok()?,
        };
        out.push(TypeField::new(i as int4, off, name, ct));
    }
    Some(out)
}

/// The structure `name` with `layout`'s `spec`, reusing one already built.
fn build_struct(
    types: &dyn TypeFactory,
    layout: &Layout,
    name: &str,
    spec: &[Field],
    size: int4,
    marker: &str,
    teb: Option<&Rc<Datatype>>,
    peb: Option<&Rc<Datatype>>,
) -> Option<Rc<Datatype>> {
    if let Some(t) = existing(types, name, size, marker, layout.offset_of(spec, marker)).ok()? {
        return Some(t);
    }
    let shell = types.get_type_struct(name).ok()?;
    let fields = fields_of(types, layout, spec, teb, peb)?;
    types.set_fields_struct_raw(&shell, fields, Vec::new(), size, layout.ptr_size, flags::variable_length).ok()
}

/// The `TEB *` type for `layout`, building `PEB`, `_TEB` and `TEB` on first use.
/// `None` when any of the names is already taken by a different type.
///
/// Completing a structure mints a new `Rc`, so a field cannot point at the
/// structure that holds it: `TEB.Self` points at the identical `_TEB` (the Windows
/// structure tag), whose own `Self` is opaque. `teb->Self->ProcessEnvironmentBlock`
/// therefore resolves; a second `->Self` stays an untyped value.
pub fn teb_pointer_type(types: &dyn TypeFactory, layout: &Layout) -> Option<Rc<Datatype>> {
    let marker = "ProcessEnvironmentBlock";
    let peb = build_struct(types, layout, "PEB", layout.peb, layout.peb_size, "BeingDebugged", None, None)?;
    let inner = build_struct(types, layout, "_TEB", layout.teb, layout.teb_size, marker, None, Some(&peb))?;
    let teb = build_struct(types, layout, "TEB", layout.teb, layout.teb_size, marker, Some(&inner), Some(&peb))?;
    types.get_type_pointer(layout.ptr_size, teb, 1).ok()
}

/// How the raw p-code uses the segment-base register: is it read, and is any
/// `STORE` made through it?
///
/// A segment override lowers to `tmp = SEG_OFFSET + addr` feeding the
/// instruction's `LOAD` or `STORE`. Raw p-code is not yet SSA, so an address
/// derived from the register is tracked by storage within each instruction that
/// reads it, through any non-`LOAD` op that consumes it.
fn segment_use(data: &Funcdata, reg: &Address, size: int4) -> (bool, bool) {
    let end = reg + size as i64;
    let mut insns = std::collections::BTreeSet::new();
    for vn in data.vbank().iter_loc_addr_range(reg, &end) {
        let Some(v) = data.vbank().get(vn) else { continue };
        if v.get_addr() != reg || v.get_size() != size {
            continue;
        }
        insns.extend(v.descend_iter().filter_map(|op| data.obank().get(op)).map(|o| o.get_addr().clone()));
    }
    let storage = |vn: Option<crate::context::VarnodeId>| {
        vn.and_then(|v| data.vbank().get(v)).map(|v| (v.get_addr().clone(), v.get_size()))
    };
    for at in &insns {
        let mut derived = vec![(reg.clone(), size)];
        for (_, id) in data.obank().iter_at(at) {
            let Some(op) = data.obank().get(id) else { continue };
            let reads_derived = (0..op.num_input()).any(|i| storage(op.get_in(i)).is_some_and(|st| derived.contains(&st)));
            if !reads_derived {
                continue;
            }
            match op.code() {
                OpCode::CPUI_STORE if storage(op.get_in(1)).is_some_and(|st| derived.contains(&st)) => {
                    return (true, true)
                }
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {}
                _ => {
                    if let Some(out) = storage(op.get_out()) {
                        derived.push(out);
                    }
                }
            }
        }
    }
    (!insns.is_empty(), false)
}

/// (kuna) `ActionPebNames` -- map `TEB *teb` over the segment-base register input
/// (option `pebnames`).
pub struct ActionPebNames {
    base: ActionBase,
}

impl ActionPebNames {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionPebNames {
            base: ActionBase::new(crate::action::ruleflags::rule_onceperfunc, "pebnames", g),
        })
    }
}

impl Action for ActionPebNames {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionPebNames { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let Some((layout, reg)) = resolve_register(data) else { return 0 };
        let size = layout.ptr_size;
        let mapped = data.get_scope_local().and_then(|lm| {
            let (sym, ty) = lm.containing_symbol_for_storage(&reg)?;
            let ours = !lm.symbol_isolated(sym) && ty.is_some_and(|t| is_teb_pointer(&t, layout));
            Some((sym, ours))
        });
        let wanted = data.get_arch().peb_names && {
            let (read, stored_through) = segment_use(data, &reg, size);
            read && !stored_through
        };
        if !wanted {
            if let Some((sym, true)) = mapped {
                if let Some(lm) = data.get_scope_local_mut() {
                    lm.remove_symbol(sym);
                }
            }
            return 0;
        }
        if mapped.is_some() {
            return 0;
        }
        let Some(ct) = data.get_arch().types().and_then(|t| teb_pointer_type(t, layout)) else {
            return 0;
        };
        let usepoint = data.get_address() + -1;
        let Some(lm) = data.get_scope_local_mut() else { return 0 };
        let name = if lm.local_name_used(TEB_SYMBOL_NAME) { "" } else { TEB_SYMBOL_NAME };
        let Ok(sym) = lm.add_symbol(name, ct, &reg, &usepoint) else { return 0 };
        let mut lock = varnode_flags::typelock;
        if !name.is_empty() {
            lock |= varnode_flags::namelock;
        }
        lm.set_attribute(sym, lock);
        1
    }
}

#[cfg(test)]
mod tests;
