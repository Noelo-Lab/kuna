//! (kuna `structsynth`) Synthesize a structure type from the constant-offset
//! dereferences of a pointer parameter, or of a pointer a call returned.
//!
//! # The gap
//!
//! A stripped binary keeps no record of the aggregate a pointer points at, so a
//! function that reads two fields of one object renders every read as raw
//! offset arithmetic:
//!
//! ```text
//! void sub_3000(uint8 *a0)
//!   v2 = *(uint4 *)&a0[1];
//!   v3 = (uint1 *)*a0;
//! ```
//!
//! The reader has to carry "offset 8 is a 4-byte count, offset 0 is a character
//! pointer" in their head for every function that touches the same object.
//!
//! # The fix
//!
//! Once type propagation has settled, collect every `LOAD`/`STORE` whose address
//! peels to `(parameter, constant)`, bucket the accesses by offset, and -- when
//! the evidence is unambiguous -- complete a `struct_N` with one field per
//! observed offset and type-lock the parameter to `struct_N *`. The whole
//! rendering half already exists: `down_chain`, `get_type_pointer_rel` and the
//! pointer-arithmetic rules turn the same p-code into `a0->field_0x8`.
//!
//! # Why the precondition is "already a pointer"
//!
//! Pointer-ness is the one fact this pass does not invent. A parameter the
//! engine believes is an integer may be an integer: `fmt`'s `get_line` takes the
//! real `FILE *` typed `int8`, and synthesizing a two-field structure over it
//! would lock a phantom layout onto a libc type that a named-type pass can still
//! recover. So the base must already carry `TYPE_PTR`, and a pointee that is a
//! *named* composite (DWARF, a parsed declaration, a libc shell, an earlier
//! synthesized structure) always wins.
//!
//! # Why phi chains are not chased
//!
//! Peeling `COPY`/`MULTIEQUAL` to a root turns a strided walk into constant
//! offsets on the root: `while (*p) p = &p[1]` reports offsets `{0, 1}` on a
//! `char *` and would synthesize a two-field structure over a string. The
//! collector therefore peels only `COPY`, `CAST` and constant pointer
//! arithmetic, and a base that feeds a `MULTIEQUAL` at all declines -- an
//! induction variable is an array walk, which is a different hypothesis.
//!
//! For the same reason an index does not have to be dereferenced *here* to be an
//! index: `ls`'s `mpsort` reads `base[0]` and `base[1]` itself and passes
//! `&base[n]` to its recursive callee, and `base` is a `void **`.
//!
//! # Why a uniform run declines
//!
//! Howard NDSS'11 §4.5's rule, statically: one width over a regular grid is an
//! element walk, not a layout, and the less common layout is the one to prefer.
//! Three or more equal-width offsets on that grid is the evidence outright; two
//! counts only for an element narrower than a pointer, so `int pipefds[2]` and
//! `arg[0]`/`arg[1]` over a `char *` decline while a pair of pointer-sized
//! fields -- far more often a record than a `long[2]` -- does not.
//!
//! A gap does not break the run below pointer width: a function that writes ten
//! of a buffer's twelve bytes is ordinary, and demanding exact contiguity made
//! any such buffer a structure. At pointer width a gap does break it, because a
//! record whose observed members are all pointer-sized with an untouched one
//! between them is the common case.
//!
//! An optimized `int fd[2]` reaches this rule in a second shape: one 8-byte
//! save/restore of the whole pair alongside the two 4-byte element accesses.
//! The widths differ, so the uniform-run test says nothing -- the layout rule
//! below is what declines it, because both elements sit inside the byte range
//! the wide access already claimed and nothing is left but a single field.
//!
//! # Why the widest access wins an offset
//!
//! A field wider than an access renders as a cast of the field
//! (`v2 = (uint4)w->b`); a field narrower than an access loses the field name
//! altogether (`v3 = *(uint1 **)w`). Both are wrong, only one is readable.
//!
//! # Why the layout has to be offset-preserving
//!
//! The synthesized declaration is exported as C source
//! (`kuna decompile-project`), so it is only worth anything if a compiler
//! reading it back puts `field_0xK` at offset K. Two shapes of evidence break
//! that silently -- a field at an offset its own alignment does not divide (the
//! compiler pads it forward) and a field minted inside the byte range a wider
//! field already claimed (it moves every later field) -- and neither shows up in
//! the body text, which keeps naming the fields as if the header agreed. Both
//! accesses are therefore dropped from the evidence, and two accesses that
//! overlap only partially decline the base outright.
//!
//! # Why the layout has no holes
//!
//! `printc` resolves an address inside a structure to the field that contains
//! it, and when no field does it invents the member name `field_0x<hex>` anyway
//! (`printc.cc:1015-1033`). The exported header renders the same gap as
//! `undefined1 _pad<hex>[N]`, so a body that names a hole names a member the
//! header does not declare -- and holes are reachable, because address
//! arithmetic gets there without a dereference: `p + 8` handed to a callee
//! prints `&p->field_0x8`. So the layout covers every byte from 0 to its end, a
//! gap becoming an `undefined1 field_0x<hex>[N]` member of alignment 1 at the
//! offset it is named for.
//!
//! # Why the size is rounded
//!
//! For the same reason one step out. A layout whose last field is a byte at
//! 0x14 is 0x15 bytes here and 0x18 to the compiler reading the header back, so
//! `a0[1]` in the exported body is a different address -- and `chibios`'s
//! `chPipeReset` indexes its parameter exactly that way. The size is the last
//! field rounded up to the widest field, with the tail covered by filler like
//! any other gap.
//!
//! # Records a call returned (`locals`)
//!
//! A function that fills in the record an allocator returned
//! (`v1 = (unsigned long *)xmalloc(0x18); *(char *)((long)v1 + 0x14) = 0x6e`)
//! or reads the one a lookup returned has the same evidence a parameter has,
//! over a value that is not a parameter. Under `locals` the value a `CALL` or
//! `CALLIND` returns is a base too: single assignment makes it the local's only
//! definition, and every condition a parameter is held to still applies. Three
//! more are about the value rather than its accesses:
//!
//! * a callee declared to return a pointer to something (`char *`, `struct
//!   stat *`) has said what the pointer is; only `void *` says nothing;
//! * the value must not share a variable with anything else. A copy is merged
//!   with what it copies, a phi input with the other inputs, and a value in
//!   address-tied storage with everything else stored there, so a value that
//!   reaches a phi through a copy, lands in tied storage, or overlaps a return
//!   register some phi joins (it is tied once merging starts, and `kmod`'s
//!   `rax` is merged with the `eax` its `int` function returns) declines: the
//!   locked record type would otherwise become the whole variable's, and
//!   `tar`'s `wordsplit_add_segm` would return its status as a `struct_N *`;
//! * a function has one return type, so a record returned on one path while
//!   another path returns a different value (a name, a count, an error code)
//!   declines; only a constant zero may be returned beside it;
//! * a buffer whose every access stores the bytes of a string literal -- as
//!   constants, or loaded from the literal in read-only memory -- is text, not
//!   a record, however unevenly the compiler split the copy;
//! * an address formed at or past the end of every access (`&node->lock` just
//!   past the fields `sortlines` reads, a flexible array member) says the
//!   record is wider than this function shows; typed on its accesses, that
//!   address would print as `&v[1]` and give the mutex the record's type.
//!
//! `all` is `locals` and `nest` together. Globals are not bases: every access to
//! a global record is an absolute address of its own, and a global pointer is
//! re-read after every call, so it always reaches a phi.
//!
//! # Known limits
//!
//! * The structure is as wide as its *surviving* fields, rounded up to the
//!   widest of them the way a C compiler rounds, so a read past that end is
//!   spelled as an element of the structure array plus a byte
//!   offset -- `*(uint4 *)((int8)&a0[0x1000].field_0x0 + 1)` for a read at
//!   0x10001 of a 16-byte layout. That compiles and computes the right address;
//!   it is readable only by accident.
//! * Offsets nobody dereferenced, and the offsets the layout prune dropped, are
//!   `undefined1` filler: named and declared at their own offset, but carrying
//!   no type.
//! * A buffer whose *last* element is written wider than the rest escapes both
//!   the uniform-run rule (the widths differ) and the prune (the wide access is
//!   past the narrow ones, not over them). `ls`'s `strmode` fills a `char[12]`
//!   with ten one-byte stores and one two-byte store and is synthesized as a
//!   structure.
//! * `struct_N` names are program-wide only where one process decompiles the
//!   whole program (`kuna decompile-all`, `kuna decompile-project`);
//!   `kuna decompile` spawns one `decomp_dbg` per function, so each function
//!   starts its numbering again.
//! * Under `param`, a field whose value is itself a synthesis base keeps the
//!   scalar type the accesses agree on; `nest` types it as a pointer to a
//!   synthesized record of its own ([`nest`]).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::types::{int4, intb, uintb};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::spacetype;

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactory, TypeField};
use crate::funcdata::Funcdata;

/// (kuna `structsynth`) The program-wide layout ledger: which minted `struct_N`
/// a freshly measured layout is, and which ones a later layout took over.
pub mod ledger;

/// (kuna `structsynth`) The ledger as data, for a whole-program run split across
/// worker processes: record, replay, install, force.
pub mod shard;

/// (kuna `structsynth nest`) A field loaded and dereferenced in turn is a
/// pointer to a record of its own, or to the record that holds it.
mod nest;


/// The largest constant offset that is still believed to be a field.
const MAX_FIELD_OFFSET: intb = 0x8000;

/// The deepest `COPY`/`CAST`/pointer-arithmetic chain the address peel follows.
const MAX_PEEL_DEPTH: u32 = 8;

/// (kuna `calleevote fields`) The narrowest access a lone field may be.
const LONE_FIELD_MIN_WIDTH: int4 = 4;

/// The access widths that may become a field: the ones whose C spelling in the
/// exported header is the same number of bytes the decompiler believes, and
/// whose alignment therefore divides their offset when the offset does.
const FIELD_WIDTHS: [int4; 4] = [1, 2, 4, 8];

/// (kuna) Which bases `structsynth` may synthesize a structure over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StructSynthMode {
    /// Upstream: a dereferenced pointer keeps whatever pointee inference gave it.
    #[default]
    Off,
    /// Function parameters only.
    Param,
    /// Function parameters, and the pointers calls return.
    Locals,
    /// Function parameters, and the records their pointer fields point at.
    Nest,
    /// `locals` and `nest` together.
    All,
}

impl StructSynthMode {
    /// Does the pass act at all?
    pub fn fires(self) -> bool {
        self != StructSynthMode::Off
    }

    /// Is a pointer field loaded and dereferenced in turn given a record?
    pub fn nests(self) -> bool {
        matches!(self, StructSynthMode::Nest | StructSynthMode::All)
    }

    /// Is a pointer a call returned given a record?
    pub fn locals(self) -> bool {
        matches!(self, StructSynthMode::Locals | StructSynthMode::All)
    }
}

/// (kuna) Parse `option structsynth off|param|locals|nest|all`; the caller
/// writes the live field.
pub struct OptionStructSynth;

impl OptionStructSynth {
    /// The option name.
    pub const NAME: &'static str = "structsynth";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(StructSynthMode, String)> {
        let mode = match p1 {
            "off" => StructSynthMode::Off,
            "param" => StructSynthMode::Param,
            "locals" => StructSynthMode::Locals,
            "nest" => StructSynthMode::Nest,
            "all" => StructSynthMode::All,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown structsynth value: {other} (expected off|param|locals|nest|all)"
                )))
            }
        };
        Ok((mode, format!("Structure synthesis set to {p1}")))
    }
}

/// What one offset of one base was seen to hold.
#[derive(Default, Clone)]
struct Slot {
    /// The widest access seen at this offset.
    width: int4,
    /// The type the widest access carried, when one was readable.
    ctype: Option<Rc<Datatype>>,
    /// What every access of that width said the bytes are.
    seen: Kinds,
}

/// The value classes the accesses of one slot carried.
#[derive(Default, Clone, Copy)]
struct Kinds {
    signed: bool,
    /// Unsigned or undefined.
    unsigned: bool,
    float: bool,
    /// A readable type that is none of the above (a pointer, a bool).
    other: bool,
}

impl Kinds {
    fn note(&mut self, t: &Datatype) {
        match t.get_metatype() {
            type_metatype::TYPE_INT => self.signed = true,
            type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => self.unsigned = true,
            type_metatype::TYPE_FLOAT => self.float = true,
            _ => self.other = true,
        }
    }
}

impl Slot {
    /// The bytes are read both as a float and as something else: a union member.
    fn reinterpreted(&self) -> bool {
        self.seen.float && (self.seen.signed || self.seen.unsigned || self.seen.other)
    }

    /// The type the field commits to, or `None` when the accesses disagree. Sign
    /// is contested only for an integer field: a pointer has no sign to get wrong.
    fn committed(&self) -> Option<&Rc<Datatype>> {
        let ct = self.ctype.as_ref()?;
        let integer = matches!(
            ct.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        );
        if self.reinterpreted() || (integer && self.seen.signed && self.seen.unsigned) {
            return None;
        }
        Some(ct)
    }
}

/// Everything one candidate base accumulated.
#[derive(Default, Clone)]
struct Evidence {
    /// Observed offset -> widest access.
    slots: BTreeMap<intb, Slot>,
    /// A dereference whose offset was not a constant (the array signal).
    dynamic_offset: bool,
    /// The base reached an operation that treats it as an integer.
    integer_use: bool,
    /// The base flows into a phi, so some dereference of it is loop-carried.
    phi_reached: bool,
    /// (kuna `calleevote fields`) An address the base yields by a constant
    /// offset reaches a phi: a pointer stepped through a loop, which walks an
    /// array, where a lone field would read one element as a record.
    derived_walk: bool,
    /// `(offset, width)` of every access the prune dropped: bytes this
    /// function touched without claiming a field for them.
    unclaimed: Vec<(intb, int4)>,
    /// Offset -> `(width, value)` of every `LOAD` through this base, pruned or
    /// not: the values a nested record is measured on.
    loads: BTreeMap<intb, Vec<(int4, VarnodeId)>>,
    /// A `STORE` of a constant whose bytes read as text.
    text_store: bool,
    /// An access that is neither such a store nor a store of zero.
    other_access: bool,
}

impl Evidence {
    /// Record one access.
    ///
    /// A field's type is the one every access of its width agrees on. A signed
    /// access beside an unsigned or undefined one leaves the field `undefined<N>`:
    /// a signed field would make the C sign-extend a read the binary zero-extends
    /// (`movzwl` into a call argument whose extension the call absorbed). A float
    /// beside anything else makes the field raw bytes, `undefined1[N]`: C has no
    /// scalar that reads the same bits as both, and a `long` or `undefined8`
    /// field read as `double` prints `(double)p->field_0x8`, a value conversion of
    /// bits the binary reinterprets (`movsd 0x8(%rdi)`), where a byte array makes
    /// every access cast the address instead.
    fn record(&mut self, off: intb, width: int4, ctype: Option<Rc<Datatype>>) {
        let slot = self.slots.entry(off).or_default();
        if width < slot.width {
            return;
        }
        if width > slot.width {
            slot.width = width;
            slot.seen = Kinds::default();
            slot.ctype = ctype.clone();
        }
        if let Some(t) = ctype.as_deref() {
            slot.seen.note(t);
        }
    }

    /// Take another base's accesses in as if they had been made through this
    /// one, in order after its own: the same widest-wins and agreement rules
    /// [`Evidence::record`] applies, and the other base's negative evidence.
    fn absorb(&mut self, other: &Evidence) {
        for (off, theirs) in other.slots.iter() {
            let slot = self.slots.entry(*off).or_default();
            if theirs.width < slot.width {
                continue;
            }
            if theirs.width > slot.width {
                *slot = theirs.clone();
                continue;
            }
            slot.seen.signed |= theirs.seen.signed;
            slot.seen.unsigned |= theirs.seen.unsigned;
            slot.seen.float |= theirs.seen.float;
            slot.seen.other |= theirs.seen.other;
        }
        self.dynamic_offset |= other.dynamic_offset;
        self.integer_use |= other.integer_use;
        self.phi_reached |= other.phi_reached;
        self.text_store |= other.text_store;
        self.other_access |= other.other_access;
        self.derived_walk |= other.derived_walk;
        for (off, vs) in other.loads.iter() {
            self.loads.entry(*off).or_default().extend(vs.iter().copied());
        }
    }

    /// This evidence with the layout prune applied.
    fn pruned(&self) -> Evidence {
        let mut e = self.clone();
        e.prune();
        e
    }

    /// Prune the evidence to a layout a C compiler lays out byte for byte.
    ///
    /// The exported header is source, so a synthesized structure is only worth
    /// anything if `offsetof(struct_N, field_0xK) == K` in the compiler that
    /// reads it back. Three shapes of access break that and none of them is
    /// visible in the body text, which keeps naming the fields as if the header
    /// agreed:
    ///
    /// * a width the exported prelude does not round-trip (`undefined3` is a
    ///   4-byte `unsigned int` there, `undefined5`/`6`/`7` are 8-byte);
    /// * an offset that is not a multiple of the access width -- a field's own
    ///   alignment divides its size, so the compiler pads it forward and moves
    ///   every later field;
    /// * an access inside the byte range a wider access already claimed --
    ///   minting a second field there moves every later field by its width.
    ///
    /// Each is therefore dropped rather than minted as a field at a lie of an
    /// offset; the byte range it covered becomes `undefined1` filler.
    ///
    /// What is left is naturally aligned and power-of-two wide, and two such
    /// ranges either nest or are disjoint, so the surviving layout cannot
    /// overlap at all: one sweep in offset order decides it.
    fn prune(&mut self) {
        let mut dropped: Vec<(intb, int4)> = Vec::new();
        self.slots.retain(|off, s| {
            let keep = FIELD_WIDTHS.contains(&s.width) && off.rem_euclid(s.width as intb) == 0;
            if !keep {
                dropped.push((*off, s.width));
            }
            keep
        });
        let mut end: intb = intb::MIN;
        for (off, s) in self.slots.iter() {
            if *off < end {
                dropped.push((*off, s.width));
            } else {
                end = *off + s.width as intb;
            }
        }
        for (off, _) in dropped.iter() {
            self.slots.remove(off);
        }
        self.unclaimed = dropped;
    }

    /// Where the last byte this function accessed through the base ends.
    fn extent(&self) -> intb {
        let fields = self.slots.iter().map(|(o, s)| o.saturating_add(s.width as intb));
        let dropped = self.unclaimed.iter().map(|(o, w)| o.saturating_add(*w as intb));
        fields.chain(dropped).max().unwrap_or(0)
    }

    /// Every byte range this function dereferenced without a typed field of its
    /// own: the accesses the prune dropped and the fields read as raw bytes. The
    /// ledger never answers with a structure that lays other members over them.
    /// Offsets beyond `int4` are clamped; they lie past every member either way.
    fn unclaimed_ranges(&self) -> Vec<(int4, int4)> {
        let clamp = |v: intb| v.clamp(int4::MIN as intb, int4::MAX as intb) as int4;
        let raw = self.slots.iter().filter(|(_, s)| s.reinterpreted()).map(|(o, s)| (*o, s.width));
        self.unclaimed.iter().copied().chain(raw).map(|(o, w)| (clamp(o), w)).collect()
    }
}

/// The `(base, offset)` an address varnode peels to, or `None` when the peel hit
/// something that is not constant pointer arithmetic over a single root.
///
/// `budget` bounds the whole walk, the re-entry that records a non-constant term
/// against its own root included, so a pathological chain cannot recurse deeply.
fn peel(
    data: &Funcdata,
    mut vn: VarnodeId,
    ev: &mut BTreeMap<VarnodeId, Evidence>,
    budget: u32,
) -> Option<(VarnodeId, intb)> {
    let mut off: intb = 0;
    for step in 0..budget {
        let v = data.vbank().get(vn)?;
        if v.is_input() || v.is_constant() {
            return Some((vn, off));
        }
        let def = v.get_def()?;
        let op = data.obank().get(def)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                vn = op.get_in(0)?;
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => {
                let a = op.get_in(0)?;
                let b = op.get_in(1)?;
                let bv = data.vbank().get(b)?;
                if !bv.is_constant() {
                    // A non-constant term is the array signal, recorded against
                    // whatever root the other operand reaches.
                    if let Some((root, _)) = peel(data, a, ev, budget - step - 1) {
                        ev.entry(root).or_default().dynamic_offset = true;
                    }
                    return None;
                }
                off = off.wrapping_add(sign_extend(bv.get_offset(), bv.get_size()));
                vn = a;
            }
            OpCode::CPUI_PTRADD => {
                let a = op.get_in(0)?;
                let idx = op.get_in(1)?;
                let sz = op.get_in(2)?;
                let iv = data.vbank().get(idx)?;
                let sv = data.vbank().get(sz)?;
                if !iv.is_constant() || !sv.is_constant() {
                    if let Some((root, _)) = peel(data, a, ev, budget - step - 1) {
                        ev.entry(root).or_default().dynamic_offset = true;
                    }
                    return None;
                }
                let scaled = sign_extend(iv.get_offset(), iv.get_size())
                    .wrapping_mul(sv.get_offset() as intb);
                off = off.wrapping_add(scaled);
                vn = a;
            }
            _ => return Some((vn, off)),
        }
    }
    None
}

/// The signed value of a constant varnode of `size` bytes.
fn sign_extend(val: uintb, size: int4) -> intb {
    if size <= 0 || size >= 8 {
        return val as intb;
    }
    let bits = (size as u32) * 8;
    let shift = 64 - bits;
    ((val << shift) as intb) >> shift
}

/// The data-type the engine currently believes this varnode has.
///
/// `HighVariable`s do not exist yet in `mainloop` -- merging runs later -- so the
/// Varnode's own type, which is what `ActionInferTypes::write_back` sets, is the
/// reading here; a high, when one exists, wins.
fn vn_type(data: &mut Funcdata, vn: VarnodeId) -> Option<Rc<Datatype>> {
    if let Some(t) = data.high_get_type(vn) {
        return Some(t);
    }
    data.vbank().get(vn).map(|v| Rc::clone(v.get_type()))
}

/// Walk the op bank once and accumulate the access evidence per base.
fn collect(data: &mut Funcdata) -> BTreeMap<VarnodeId, Evidence> {
    let mut ev: BTreeMap<VarnodeId, Evidence> = BTreeMap::new();

    for opc in [OpCode::CPUI_LOAD, OpCode::CPUI_STORE] {
        let ops: Vec<_> = data.obank().iter_code(opc).collect();
        for id in ops {
            let Some(op) = data.obank().get(id) else { continue };
            let Some(addr) = op.get_in(1) else { continue };
            let value = if opc == OpCode::CPUI_LOAD { op.get_out() } else { op.get_in(2) };
            let Some(value) = value else { continue };
            let Some((base, off)) = peel(data, addr, &mut ev, MAX_PEEL_DEPTH) else { continue };
            let Some(width) = data.vbank().get(value).map(|v| v.get_size()) else { continue };
            let ctype = vn_type(data, value);
            let stored = if opc == OpCode::CPUI_STORE { stored_text(data, value, width) } else { None };
            let e = ev.entry(base).or_default();
            e.record(off, width, ctype);
            match stored {
                Some(Some(true)) => e.text_store = true,
                Some(None) => {}
                _ => e.other_access = true,
            }
            if opc == OpCode::CPUI_LOAD {
                e.loads.entry(off).or_default().push((width, value));
            }
        }
    }

    let bases: Vec<VarnodeId> = ev.keys().copied().collect();
    for base in bases {
        let e = ev.entry(base).or_default();
        note_uses(data, base, e);
    }
    ev
}

/// Does a stored value copy text: a constant whose bytes read as text, or bytes
/// read from a read-only string literal (`cp`'s `mempcpy` tail filled from
/// `"CuXXXXXX"`)?  `None` for a value that is neither, `Some(None)` for zero.
/// A call's clobber of read-only memory changes nothing, so a literal the heap
/// model re-defines at a call is still the image's bytes.
fn stored_text(data: &Funcdata, value: VarnodeId, width: int4) -> Option<Option<bool>> {
    let root = copy_root(data, value);
    let v = data.vbank().get(root)?;
    if v.is_constant() {
        let n = width.clamp(0, 8) as u32;
        let bytes: Vec<u8> = (0..n).map(|i| ((v.get_offset() >> (8 * i)) & 0xff) as u8).collect();
        return Some(bytes_text(&bytes));
    }
    let clobber = v.get_def().and_then(|d| data.obank().get(d)).is_some_and(|o| o.code() == OpCode::CPUI_INDIRECT);
    if (v.is_written() && !clobber) || !v.is_read_only() || v.get_size() != width || !(1..=MAX_LITERAL).contains(&width) {
        return None;
    }
    if v.get_addr().get_space()?.get_type() != spacetype::IPTR_PROCESSOR {
        return None;
    }
    let mut buf = vec![0u8; width as usize];
    data.get_arch().loader_fill(&mut buf, v.get_addr()).ok()?;
    Some(bytes_text(&buf))
}

/// The widest read of a string literal [`stored_text`] reads back: one vector
/// register.
const MAX_LITERAL: int4 = 16;

/// Do these stored bytes read as text?  `None` when they are all zero, which a
/// record's initializer stores as often as a string's terminator.
fn bytes_text(bytes: &[u8]) -> Option<bool> {
    if bytes.iter().all(|&b| b == 0) {
        return None;
    }
    let printable = |b: &u8| (0x20..0x7f).contains(b);
    Some(
        bytes.iter().all(|b| *b == 0 || *b == b'\t' || *b == b'\n' || printable(b))
            && bytes.iter().filter(|b| printable(b)).count() * 2 >= bytes.len().max(1),
    )
}

/// Negative evidence that is not about an address: the base used as a plain
/// integer, as an index, or flowing into a phi (an induction variable).
fn note_uses(data: &Funcdata, base: VarnodeId, e: &mut Evidence) {
    let Some(v) = data.vbank().get(base) else { return };
    let uses: Vec<_> = v.descend_iter().collect();
    for u in uses {
        let Some(op) = data.obank().get(u) else { continue };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => e.phi_reached = true,
            // `p + n` for a non-constant `n` is an index, whether or not
            // this function dereferences the result: `mpsort` passes
            // `&base[n]` to its recursive callee and reads `base[0]` and
            // `base[1]` itself, and it is indexing a `void **`, not reading
            // the first two fields of a record.
            OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB => {
                let other = if op.code() == OpCode::CPUI_PTRADD {
                    op.get_in(1)
                } else if op.get_in(0) == Some(base) {
                    op.get_in(1)
                } else {
                    op.get_in(0)
                };
                let constant = other
                    .and_then(|o| data.vbank().get(o))
                    .map(|v| v.is_constant())
                    .unwrap_or(false);
                if !constant {
                    e.dynamic_offset = true;
                } else if data.kuna_calleevote_closed() {
                    e.derived_walk |= op.get_out().is_some_and(|out| address_walks(data, out, 0));
                }
            }
            // An address is added to, subtracted from, compared for
            // (in)equality and passed around; everything else treats it as a
            // number.
            OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_SUBPIECE => e.integer_use = true,
            _ => {}
        }
    }
}

/// (kuna `calleevote fields`) Does the address `vn` reach a phi through copies,
/// casts and further constant offsets?
fn address_walks(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    v.descend_iter().any(|u| {
        let Some(op) = data.obank().get(u) else { return false };
        let onward = |data: &Funcdata| {
            depth < MAX_PEEL_DEPTH && op.get_out().is_some_and(|o| address_walks(data, o, depth + 1))
        };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => true,
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => onward(data),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => {
                let term = if op.code() == OpCode::CPUI_PTRADD || op.get_in(0) == Some(vn) {
                    op.get_in(1)
                } else {
                    op.get_in(0)
                };
                term.and_then(|t| data.vbank().get(t)).is_some_and(|t| t.is_constant()) && onward(data)
            }
            _ => false,
        }
    })
}

/// Is `ct` a pointer whose pointee is a named aggregate a person or a named-type
/// pass supplied?  Such a type always wins over a synthesized one.
fn points_at_named_composite(ct: &Datatype) -> bool {
    let Some(pt) = ct.get_ptr_to() else { return false };
    matches!(
        pt.get_metatype(),
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION | type_metatype::TYPE_ARRAY
    ) && !pt.get_name().is_empty()
}

/// Does this offset set read as an ARRAY rather than a structure?
///
/// Howard NDSS'11 §4.5's rule, in its static form: one uniform width over a
/// regular grid is a fill or an element walk, not a layout, and the less common
/// layout is the one to prefer. Three or more equal-width offsets in a row is
/// that evidence outright. Two is weak either way, so it counts only for an
/// element narrower than a pointer -- `int pipefds[2]` and `arg[0]`/`arg[1]`
/// over a `char *` are arrays, while a pair of pointer-sized fields is far more
/// often a record than a `long[2]` parameter.
///
/// A run need not be contiguous. One element the function never touches is
/// ordinary -- `strmode` writes ten of its twelve bytes -- and demanding exact
/// contiguity made any such buffer a structure. Below pointer width the grid
/// alone is the evidence, so gaps are tolerated there; at pointer width they are
/// not, because a record whose observed fields are all pointer-sized with an
/// untouched field between them is the common case (`gnulib`'s
/// `struct hash_table` is ten pointer-sized members and its readers touch
/// different subsets of them, most three or four wide).
fn is_array_shaped(slots: &BTreeMap<intb, Slot>, ptr_size: int4) -> bool {
    if slots.len() < 2 {
        return false;
    }
    let width = slots.values().next().map(|s| s.width).unwrap_or(0);
    if width <= 0 || slots.values().any(|s| s.width != width) {
        return false;
    }
    let first = *slots.keys().next().unwrap();
    let mut expect = first;
    let mut contiguous = true;
    for off in slots.keys() {
        if (*off - first).rem_euclid(width as intb) != 0 {
            return false;
        }
        if *off != expect {
            contiguous = false;
        }
        expect = *off + width as intb;
    }
    if contiguous {
        slots.len() >= 3 || width < ptr_size
    } else {
        // A gap weakens the evidence, so only the outright tier counts.
        slots.len() >= 3 && width < ptr_size
    }
}

/// Does this type's C spelling occupy exactly the bytes the decompiler thinks it
/// does?  A scalar or a pointer does.  An aggregate carries its own padding and
/// alignment into the exported header, so an aggregate-typed value degrades to
/// the honest `undefined<N>` of the access width rather than moving the fields
/// after it.
fn spells_its_own_width(ct: &Datatype) -> bool {
    matches!(
        ct.get_metatype(),
        type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_BOOL
            | type_metatype::TYPE_FLOAT
            | type_metatype::TYPE_PTR
            | type_metatype::TYPE_UNKNOWN
    )
}

/// Build the field list for one accepted base: every byte of the structure is
/// covered by a member, and the structure ends where the evidence does.
///
/// Both properties are about the printer, not about taste. `printc` resolves an
/// address inside a structure by looking for the field that contains it, and
/// when no field does -- an offset in a hole -- it invents the member name
/// `field_0x<hex>` anyway (`printc.cc:1015-1033`). The exported header renders
/// the same hole as `undefined1 _pad<hex>[N]`, so the body would name a member
/// the header does not declare. Address arithmetic reaches offsets no
/// dereference did (`&p->field_0x8` for a `p + 8` handed to a callee), so the
/// holes are reachable in practice.
///
/// So a gap becomes a real member: `undefined1 field_0x<hex>[N]`, alignment 1 at
/// the offset it is named for, which is what the compiler reading the header
/// back puts there. Past the last field there is no member and no invented name:
/// the printer spells the address as an element of the structure array plus a
/// byte offset, which is still the address the access meant.
fn fields_for(
    types: &dyn TypeFactory,
    ev: &Evidence,
) -> Option<(Vec<TypeField>, int4)> {
    let (plan, size) = layout_plan(&ev.slots)?;
    let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
    let mut fields: Vec<TypeField> = Vec::with_capacity(plan.len());
    for (i, (off, width, filled)) in plan.iter().enumerate() {
        let ct = if *filled {
            // Always an array, even for one byte: a member of array type is how
            // the instruments tell filler from a field the pass actually claims.
            types.get_type_array(*width, Rc::clone(&byte)).ok()?
        } else {
            match ev.slots.get(&(*off as intb)).and_then(Slot::committed) {
                // A value type wider or narrower than the access is not this
                // offset's field; fall back to the honest unknown of the access
                // width, which prints `undefined<N>`.
                Some(t) if t.get_size() == *width && spells_its_own_width(t) => Rc::clone(t),
                _ => types.get_base(*width, type_metatype::TYPE_UNKNOWN).ok()?,
            }
        };
        fields.push(TypeField::new(i as int4, *off, format!("field_0x{off:x}"), ct));
    }
    Some((fields, size))
}

/// The member list as `(offset, width, is_filler)` plus the structure size, from
/// the pruned slots alone.
///
/// Every byte from 0 to the structure's end belongs to exactly one member, and
/// the end is the last field rounded up to the structure's own alignment -- the
/// widest field it has, since every field here is a scalar or a pointer whose C
/// spelling aligns to its size. That rounding is what a C compiler does, and
/// the body depends on it: `a0[1]` on a structure the decompiler sized 0x15 and
/// the compiler sizes 0x18 is two different addresses, which is a round-trip
/// failure no amount of correct `offsetof` catches.
fn layout_plan(slots: &BTreeMap<intb, Slot>) -> Option<(Vec<(int4, int4, bool)>, int4)> {
    let mut plan: Vec<(int4, int4, bool)> = Vec::with_capacity(slots.len());
    let mut next: int4 = 0;
    let mut align: int4 = 1;
    for (off, slot) in slots.iter() {
        let off = int4::try_from(*off).ok()?;
        if off < next {
            return None;
        }
        if off > next {
            plan.push((next, off - next, true));
        }
        plan.push((off, slot.width, slot.reinterpreted()));
        align = align.max(slot.width);
        next = off.checked_add(slot.width)?;
    }
    let size = next.checked_add(align - 1)? / align * align;
    if size > next {
        plan.push((next, size - next, true));
    }
    Some((plan, size))
}

/// The one pass: decide every candidate base and install the accepted ones.
/// Returns whether anything was installed.
fn synthesize(data: &mut Funcdata) -> bool {
    let Some(types) = data.get_arch().types_rc() else { return false };
    let raw = collect(data);
    let nests = data.get_arch().struct_synth.nests();
    let ptrsize = types.get_size_of_pointer();
    let mut asks = Vec::new();
    for (base, e) in raw.iter() {
        let e = e.pruned();
        census_base(data, *base, &e);
        if !accepts(data, *base, &e) {
            continue;
        }
        let Some((mut fields, size)) = fields_for(types.as_ref(), &e) else { continue };
        let selfs = if nests {
            let cx = nest::Nesting { ev: &raw, types: types.as_ref(), ptr_size: ptrsize };
            nest::nest_fields(data, &cx, &e, &mut fields, size, 0)
        } else {
            Vec::new()
        };
        asks.push((*base, e.slots.len(), fields, size, e.unclaimed_ranges(), selfs));
    }
    // A function whose own later layout contains its earlier one would see its
    // earlier answer superseded, which a `--jobs` replay cannot reproduce; asked
    // widest first, a later layout never contains an earlier one. Parameters
    // alone keep the order `param` always had.
    if data.get_arch().struct_synth.locals() {
        asks.sort_by(|a, b| b.1.cmp(&a.1));
    }

    let mut installs: Vec<(VarnodeId, Rc<Datatype>)> = Vec::new();
    for (base, _, fields, size, unclaimed, selfs) in asks {
        let Some(st) = answer(data, types.as_ref(), fields, size, &unclaimed, &selfs) else {
            continue;
        };
        // The pointer is taken only once the structure is COMPLETE: completing a
        // structure mints a fresh `Rc`, and merge compares high types by `Rc`
        // identity, so a pointer to the incomplete shell would never match.
        let Ok(ptr) = types.get_type_pointer(ptrsize, st, 1) else { continue };
        installs.push((base, ptr));
    }

    let mut changed = false;
    for (vn, ct) in installs {
        if data.vn_update_type_locked(vn, ct, true, false) {
            changed = true;
        }
    }
    changed
}

/// (kuna `calleevote fields`) Does a closed function with a lone-field candidate
/// need one more propagation pass before its lattice can be read?
///
/// The plateau signal is set by a propagation pass that changes nothing, and a
/// short function's main loop can end on the pass that first typed it: nothing
/// after it changes an op, so no second pass runs and the lattice is never seen
/// settled. Asking for one pass there, once, is what lets a one-field getter be
/// read at all.
fn wants_settle_pass(data: &mut Funcdata) -> bool {
    if !data.kuna_calleevote_closed()
        || !data.has_type_recovery_started()
        || data.is_type_recovery_exceeded()
        || data.kuna_infertypes_settled()
    {
        return false;
    }
    let headless = data.get_arch().struct_headless.fires();
    let raw = collect(data);
    raw.iter().any(|(base, e)| {
        let e = e.pruned();
        (is_lone_field(&e) || (headless && crate::kuna_structheadless::is_headless(e.slots.keys())))
            && accepts(data, *base, &e)
    })
}

/// The program-wide structure for one measured layout: the shard hook's answer
/// in a `--jobs` worker, the ledger's otherwise.
fn answer(
    data: &Funcdata,
    types: &dyn TypeFactory,
    fields: Vec<TypeField>,
    size: int4,
    unclaimed: &[(int4, int4)],
    selfs: &[int4],
) -> Option<Rc<Datatype>> {
    match &data.get_arch().struct_synth_shard {
        Some(hook) => shard::lookup(hook, types, fields, size, unclaimed, selfs),
        None => ledger::lookup_or_mint(
            types,
            fields,
            size,
            unclaimed,
            selfs,
            data.get_arch().struct_merge,
        ),
    }
}

/// TEMPORARY census: why each base with constant-offset accesses is refused.
fn census_base(data: &mut Funcdata, base: VarnodeId, e: &Evidence) {
    if std::env::var_os("KUNA_SSCENSUS").is_none() {
        return;
    }
    let fa = data.get_address().get_offset();
    let Some(v) = data.vbank().get(base) else { return };
    let (input, lock, persist, spacebase, constant) = (
        v.is_input(),
        v.is_type_lock(),
        v.is_persist(),
        v.is_spacebase() || v.get_space().get_type() == spacetype::IPTR_SPACEBASE,
        v.is_constant(),
    );
    let stor = format!(
        "{}:{:x}:{}",
        v.get_space().get_name(),
        v.get_offset(),
        v.get_size()
    );
    let defop = v.get_def().and_then(|d| data.obank().get(d)).map(|o| format!("{:?}", o.code()));
    let call_ret = is_call_return(data, base);
    let kind = if input {
        "param".to_string()
    } else if spacebase {
        "frame".to_string()
    } else if constant {
        "const".to_string()
    } else if persist {
        "global".to_string()
    } else if call_ret {
        "callret".to_string()
    } else {
        format!("value-{}", defop.unwrap_or_else(|| "none".into()).trim_start_matches("CPUI_").to_lowercase())
    };
    let ct = vn_type(data, base);
    let is_ptr = ct.as_ref().is_some_and(|c| c.get_metatype() == type_metatype::TYPE_PTR);
    let named = ct.as_ref().is_some_and(|c| points_at_named_composite(c));
    let pointee = match ct.as_ref().and_then(|c| c.get_ptr_to()) {
        Some(p) => format!("{:?}", p.get_metatype()).trim_start_matches("TYPE_").to_lowercase(),
        None => format!("{:?}", ct.as_ref().map(|c| c.get_metatype())).to_lowercase(),
    };
    let offs: Vec<String> = e.slots.iter().map(|(o, s)| format!("{o:x}/{}", s.width)).collect();
    let ok = accepts(data, base, e);
    let why = if ok {
        "ACCEPT"
    } else if !input && !call_ret {
        "notbase"
    } else if !input && !data.get_arch().struct_synth.locals() {
        "localsoff"
    } else if !input && !is_local_base(data, base, e) {
        "localrule"
    } else if lock {
        "typelock"
    } else if spacebase || persist {
        "frameglobal"
    } else if !is_ptr {
        "notptr"
    } else if named {
        "named"
    } else if e.dynamic_offset {
        "dynoff"
    } else if e.integer_use {
        "intuse"
    } else if e.phi_reached {
        "phi"
    } else if e.slots.keys().any(|o| *o < 0 || *o >= MAX_FIELD_OFFSET) {
        "range"
    } else if e.slots.is_empty() {
        "noslots"
    } else if e.slots.len() < 2 {
        "onefield"
    } else if !e.slots.contains_key(&0) {
        "nozero"
    } else {
        let ps = data.get_arch().types().map(|t| t.get_size_of_pointer()).unwrap_or(8);
        if is_array_shaped(&e.slots, ps) {
            "arrayrun"
        } else {
            "other"
        }
    };
    eprintln!(
        "SSCENSUS base {fa:x} {kind} {stor} {} {pointee} {why} {} {}",
        e.slots.len(),
        if offs.is_empty() { "-".to_string() } else { offs.join(",") },
        data.kuna_calleevote_closed() as u8
    );
}

/// Every decline condition for a base, in one place.
fn accepts(data: &mut Funcdata, base: VarnodeId, e: &Evidence) -> bool {
    let Some(v) = data.vbank().get(base) else { return false };

    // A record is synthesized over a parameter, under `locals` over a pointer a
    // call returned, and under `nest` over what a record's pointer fields hold
    // ([`nest`]).
    if !v.is_input() && !(data.get_arch().struct_synth.locals() && is_local_base(data, base, e)) {
        return false;
    }
    // A user/DWARF/declared type is authoritative.
    if v.is_type_lock() {
        return false;
    }
    // Never wrap a frame slot or a global: the stack layout is recovered by its
    // own machinery and a global's type is program-wide.
    if v.is_spacebase() || v.is_persist() || v.get_space().get_type() == spacetype::IPTR_SPACEBASE {
        return false;
    }
    let Some(ct) = vn_type(data, base) else { return false };
    let lone = data.kuna_calleevote_closed() && !pointee_is_given(data, base, &ct, e);
    let headless = lone && crate::kuna_structheadless::admits(data, base);
    accepts_record(data, &ct, e, lone, headless)
}

/// Does `locals` measure `base`: a value a call returned, alone in its
/// variable, that is not a buffer of text and whose record ends where its
/// accesses end?
fn is_local_base(data: &Funcdata, base: VarnodeId, e: &Evidence) -> bool {
    if !is_call_return(data, base) || (e.text_store && !e.other_access) {
        return false;
    }
    let rets = returned_values(data);
    let Some(copies) = copies_alone(data, base, &rets) else { return false };
    !returned_beside_others(data, &copies, &rets) && !points_past(data, &copies, e.extent())
}

/// Is `vn` the value a `CALL` or `CALLIND` returned, and did the callee's
/// declaration leave its pointee open?  Single assignment makes that value the
/// local's only definition; a declared `char *` or `struct stat *` return is
/// what the pointer is, and only `void *` (an allocator) says nothing.
fn is_call_return(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    if !matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) || op.get_out() != Some(vn) {
        return false;
    }
    let Some(i) = data.get_call_specs_index(def) else { return true };
    let proto = data.get_call_specs(i).proto();
    if !proto.is_output_locked() {
        return true;
    }
    proto.get_output_type().and_then(|t| t.get_ptr_to()).is_some_and(|p| {
        matches!(p.get_metatype(), type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN)
    })
}

/// The most copies of one returned value [`joins_other_values`] follows.
const MAX_COPIES: usize = 64;

/// The copies of the value `vn` holds, or `None` when it will share a variable
/// with some other value.
///
/// A copy is merged with the value it copies, a phi input with every other
/// input of the phi, and a value in address-tied storage with everything else
/// stored there, so a returned value that reaches a phi through a copy, or
/// lands in tied storage, ends up in one variable with whatever else that
/// storage held -- and a locked member decides the whole variable's type.
/// `tar`'s `wordsplit_add_segm` keeps a record from `calloc` in the `rax` it
/// returns its status in, and typing the record would declare the status a
/// `struct_N *` too.
fn copies_alone(data: &Funcdata, vn: VarnodeId, rets: &[VarnodeId]) -> Option<Vec<VarnodeId>> {
    let mut seen = vec![vn];
    let mut i = 0;
    while i < seen.len() {
        let cur = seen[i];
        i += 1;
        if storage_is_tied(data, cur, rets) {
            return None;
        }
        let v = data.vbank().get(cur)?;
        for u in v.descend_iter() {
            let Some(op) = data.obank().get(u) else { continue };
            match op.code() {
                OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => return None,
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                    let Some(out) = op.get_out() else { continue };
                    if !seen.contains(&out) {
                        if seen.len() >= MAX_COPIES {
                            return None;
                        }
                        seen.push(out);
                    }
                }
                _ => {}
            }
        }
    }
    Some(seen)
}

/// Does a copy of the base form an address at or past `extent`, the end of
/// every access?  `sortlines` takes `&node->lock` just past the fields it
/// reads, and `mountlist` keeps `&me->me_next` as its list tail: the record is
/// wider than this function's accesses show, and a record measured on them
/// would type `&node[1]` -- the mutex, the tail -- as a record too.
fn points_past(data: &Funcdata, copies: &[VarnodeId], extent: intb) -> bool {
    copies.iter().filter_map(|&c| data.vbank().get(c).map(|v| (c, v))).any(|(c, v)| {
        v.descend_iter().filter_map(|u| data.obank().get(u)).any(|op| {
            let term = |slot: int4| op.get_in(slot).and_then(|t| data.vbank().get(t)).filter(|t| t.is_constant());
            let off = match op.code() {
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => {
                    let other = if op.get_in(0) == Some(c) { 1 } else { 0 };
                    term(other).map(|t| sign_extend(t.get_offset(), t.get_size()))
                }
                OpCode::CPUI_PTRADD if op.get_in(0) == Some(c) => term(1).zip(term(2)).map(|(i, s)| {
                    sign_extend(i.get_offset(), i.get_size()).wrapping_mul(s.get_offset() as intb)
                }),
                _ => None,
            };
            off.is_some_and(|k| k >= extent)
        })
    })
}

/// The value every live `RETURN` returns.
fn returned_values(data: &Funcdata) -> Vec<VarnodeId> {
    data.obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|r| data.obank().get(r))
        .filter(|o| !o.is_dead() && o.get_halt_type() == 0 && o.num_input() >= 2)
        .filter_map(|o| o.get_in(1))
        .collect()
}

/// Is one of `copies` returned while some `RETURN` returns a different value?
/// A function has one return type, so a value returned on one path types what
/// every other path returns: a record returned where another path returns a
/// name or a count declines. A constant zero is the null every pointer can be.
fn returned_beside_others(data: &Funcdata, copies: &[VarnodeId], rets: &[VarnodeId]) -> bool {
    if !rets.iter().any(|r| copies.contains(r)) {
        return false;
    }
    rets.iter().any(|&r| {
        let root = copy_root(data, r);
        !copies.contains(&root)
            && !data.vbank().get(root).is_some_and(|v| v.is_constant() && v.get_offset() == 0)
    })
}

/// The value `vn` copies, through any chain of `COPY` and `CAST`.
fn copy_root(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..MAX_COPIES {
        let Some(op) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            break;
        };
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            break;
        }
        let Some(src) = op.get_in(0) else { break };
        vn = src;
    }
    vn
}

/// Will every value stored where `vn` lives be one variable?  That is so when a
/// value overlapping it is address-tied, and when it overlaps the storage a
/// `RETURN` reads and a phi writes that return register somewhere: the return
/// register is then tied as a whole-function local once merging starts
/// (`mark_output_storage_addr_tied`), long after this pass has run, and a
/// `rax` value is merged with the `eax` an `int` function returns.
fn storage_is_tied(data: &Funcdata, vn: VarnodeId, rets: &[VarnodeId]) -> bool {
    let Some(v) = data.vbank().get(vn) else { return true };
    if v.is_addr_tied() {
        return true;
    }
    let Some(spc) = v.get_addr().get_space().cloned() else { return false };
    if spc.get_type() == spacetype::IPTR_INTERNAL {
        return false;
    }
    let (off, size) = (v.get_offset(), v.get_size());
    let joined_return = rets.iter().filter_map(|&r| data.vbank().get(r)).any(|r| {
        r.get_addr().get_space().is_some_and(|s| s.get_index() == spc.get_index())
            && overlaps(off, size, r.get_offset(), r.get_size())
            && overlapping(data, &spc, r.get_offset(), r.get_size()).any(|id| {
                let def = data.vbank().get(id).and_then(|w| w.get_def());
                def.and_then(|d| data.obank().get(d)).is_some_and(|op| {
                    matches!(op.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT)
                })
            })
    });
    joined_return
        || overlapping(data, &spc, off, size).any(|id| data.vbank().get(id).is_some_and(|w| w.is_addr_tied()))
}

/// Do the byte ranges `[a, a+asize)` and `[b, b+bsize)` intersect?
fn overlaps(a: uintb, asize: int4, b: uintb, bsize: int4) -> bool {
    a < b.saturating_add(bsize as uintb) && b < a.saturating_add(asize as uintb)
}

/// Every varnode in `spc` whose bytes intersect `[off, off+size)`.
fn overlapping<'a>(
    data: &'a Funcdata,
    spc: &Rc<kuna_base::space::AddrSpace>,
    off: uintb,
    size: int4,
) -> impl Iterator<Item = VarnodeId> + 'a {
    let lo = kuna_base::address::Address::new(Rc::clone(spc), off.saturating_sub(MAX_OVERLAP_BACK));
    let hi = kuna_base::address::Address::new(Rc::clone(spc), off.saturating_add(size as uintb));
    data.vbank()
        .iter_loc_addr_range(&lo, &hi)
        .filter(move |&id| data.vbank().get(id).is_some_and(|w| overlaps(w.get_offset(), w.get_size(), off, size)))
}

/// How far below a value's storage an overlapping wider value may start.
const MAX_OVERLAP_BACK: uintb = 16;

/// The decline conditions every synthesized record shares, a parameter's or a
/// nested field's: `ct` is the type the base already carries. `lone` admits a
/// parameter read at one constant offset other than zero ([`is_lone_field`]),
/// `headless` one read at two or more with none of them zero
/// ([`crate::kuna_structheadless`]).
fn accepts_record(data: &Funcdata, ct: &Datatype, e: &Evidence, lone: bool, headless: bool) -> bool {
    // Pointer-ness is not invented here.
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return false;
    }
    if points_at_named_composite(ct) {
        return false;
    }
    // Negative evidence.
    if e.dynamic_offset || e.integer_use || e.phi_reached {
        return false;
    }
    if e.slots.keys().any(|off| *off < 0 || *off >= MAX_FIELD_OFFSET) {
        return false;
    }
    if lone && is_lone_field(e) {
        return true;
    }
    // Two distinct offsets is the minimum evidence for a layout; a lone field at
    // offset 0 is the pointee, not a structure.
    if e.slots.len() < 2 {
        return false;
    }
    if !e.slots.contains_key(&0) && !headless {
        return false;
    }
    let ptr_size = data.get_arch().types().map(|t| t.get_size_of_pointer()).unwrap_or(8);
    if is_array_shaped(&e.slots, ptr_size) {
        return false;
    }
    true
}

/// (kuna `calleevote fields`) Is this one field, read at a constant offset other
/// than zero, at least [`LONE_FIELD_MIN_WIDTH`] bytes wide, with no address
/// derived from the pointer stepped through a loop?
///
/// One field is evidence of a record only in a function every caller of which
/// is known (`Funcdata::kuna_calleevote_closed`): a function reached through a
/// pointer -- a `qsort` comparator, a hash-table callback -- takes the generic
/// `void *` its contract declares and casts inside, and reads one field of what
/// it was handed exactly the way a getter does. A byte or halfword at a fixed
/// offset is as often a character of a buffer as a field, so those still need
/// a second field.
fn is_lone_field(e: &Evidence) -> bool {
    e.slots.len() == 1
        && !e.derived_walk
        && e.slots.iter().all(|(off, s)| *off > 0 && s.width >= LONE_FIELD_MIN_WIDTH)
}

/// (kuna `calleevote fields`) Did something outside the function's own reading
/// give the parameter `base` its pointee `ct`, so that its lone field `e` does
/// not make it a record?  A declared prototype it is handed to (`pipe (int *)`)
/// always did. The type every caller passes, which the input took, did for a
/// record, and for a `char **` where the field is one of its elements, a whole
/// pointer at a multiple of the pointer width (`argv[1]`). An `int` read at offset 4 of a
/// `char **`, or a word read out of a `char *`, is the function's own evidence
/// against its callers. A pointee the recovery guessed (a callee's `int *` for
/// a record whose first member is an enum) gives way too.
fn pointee_is_given(data: &Funcdata, base: VarnodeId, ct: &Datatype, e: &Evidence) -> bool {
    let Some(pointee) = ct.get_ptr_to() else { return false };
    if matches!(pointee.get_metatype(), type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN) {
        return false;
    }
    if crate::kuna_protoorder::handed_to_a_declared_pointer(data, base) {
        return true;
    }
    let width = pointee.get_size() as intb;
    let fits = match pointee.get_metatype() {
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION => true,
        type_metatype::TYPE_PTR => width > 0 && e.slots.iter().all(|(off, s)| s.width as intb == width && off % width == 0),
        _ => false,
    };
    fits && crate::kuna_calleevote::took_stated_type(data, base, ct)
}

/// (kuna `calleevote fields`) Is `ct` a pointer to a synthesized record that
/// claims exactly one field -- what a lone field read mints? Such a record
/// gives way to the one every caller of the function passes.
pub fn points_at_lone_record(ct: &Datatype) -> bool {
    let Some(pt) = ct.get_ptr_to() else { return false };
    ledger::minted_number(&pt).is_some() && ledger::layout_of(&pt).is_some_and(|l| l.fields.len() == 1)
}

/// (kuna `structsynth nest`) The completed record a pointer to its own shell
/// stands for, or `None` for any other type.
///
/// A record that points at itself is minted around its incomplete shell
/// (`ledger::mint`), so the field's pointer names a type with no members, and a
/// value loaded through it would print every access as raw offset arithmetic
/// (`*(long **)((long)v1 + 8)`). A LOAD or STORE whose value type is such a
/// pointer takes the pointer to the completed record instead. Only a shell the
/// completed synthesized record itself points at is resolved; a DWARF forward
/// declaration, or any other incomplete type, is left alone.
pub fn resolve_self_pointer(types: &dyn TypeFactory, ct: &Datatype) -> Option<Rc<Datatype>> {
    let DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize } = &ct.kind else {
        return None;
    };
    if !ptrto.is_incomplete()
        || ptrto.get_metatype() != type_metatype::TYPE_STRUCT
        || !ptrto.get_name().starts_with("struct_")
    {
        return None;
    }
    let full = types.find_by_name(ptrto.get_name()).ok().flatten()?;
    if full.is_incomplete() || ledger::minted_number(&full).is_none() {
        return None;
    }
    let holds = (0..full.num_depend())
        .filter_map(|i| full.get_field(i))
        .any(|f| f.field_type.get_ptr_to().is_some_and(|p| Rc::ptr_eq(&p, ptrto)));
    if !holds {
        return None;
    }
    types.get_type_pointer(ct.get_size(), full, *wordsize).ok()
}

/// (kuna) `ActionStructSynth` -- synthesize `struct_N` over a dereferenced
/// pointer parameter or returned pointer (option `structsynth`).
pub struct ActionStructSynth {
    base: ActionBase,
    /// Has this function already been offered to the synthesizer?
    fired: bool,
    /// Has one more propagation pass been asked for ([`wants_settle_pass`])?
    requested: bool,
}

impl ActionStructSynth {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionStructSynth { base: ActionBase::new(0, "structsynth", g), fired: false, requested: false })
    }
}

impl Action for ActionStructSynth {
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
        Some(Box::new(ActionStructSynth { base: self.base.clone(), fired: false, requested: false }))
    }
    fn reset(&mut self, _data: &mut Funcdata) {
        self.fired = false;
        self.requested = false;
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().struct_synth.fires() || self.fired {
            return 0;
        }
        if !self.requested && wants_settle_pass(data) {
            self.requested = true;
            self.base.count += 1;
            return 0;
        }
        // A lattice that is still moving has not decided what is a pointer, and
        // one that never settled is not evidence at all.
        if !data.has_type_recovery_started()
            || data.is_type_recovery_exceeded()
            || !data.kuna_infertypes_settled()
        {
            if std::env::var_os("KUNA_SSCENSUS").is_some() {
                eprintln!(
                    "SSCENSUS unsettled {:x} {} {} {}",
                    data.get_address().get_offset(),
                    data.has_type_recovery_started() as u8,
                    data.is_type_recovery_exceeded() as u8,
                    data.kuna_infertypes_settled() as u8
                );
            }
            return 0;
        }
        self.fired = true;
        if synthesize(data) {
            // The change signal is `count`, not the return value: `Action::perform`
            // compares `lcount < count` and reads the result only for `res < 0`.
            self.base.count += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests;
