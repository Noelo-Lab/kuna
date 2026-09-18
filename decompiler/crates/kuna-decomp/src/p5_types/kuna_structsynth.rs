//! (kuna `structsynth`) Synthesize a structure type from the constant-offset
//! dereferences of a pointer parameter.
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
//! * A field whose value is itself a synthesis base is not nested in this
//!   version; it keeps the scalar type the accesses agree on.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::types::{int4, intb, uintb};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::spacetype;

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;

/// The largest constant offset that is still believed to be a field.
const MAX_FIELD_OFFSET: intb = 0x8000;

/// The deepest `COPY`/`CAST`/pointer-arithmetic chain the address peel follows.
const MAX_PEEL_DEPTH: u32 = 8;

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
}

impl StructSynthMode {
    /// Does the pass act at all?
    pub fn fires(self) -> bool {
        self != StructSynthMode::Off
    }
}

/// (kuna) Parse `option structsynth off|param`; the caller writes the live field.
pub struct OptionStructSynth;

impl OptionStructSynth {
    /// The option name.
    pub const NAME: &'static str = "structsynth";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(StructSynthMode, String)> {
        let mode = match p1 {
            "off" => StructSynthMode::Off,
            "param" => StructSynthMode::Param,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown structsynth value: {other} (expected off|param)"
                )))
            }
        };
        Ok((mode, format!("Structure synthesis set to {p1}")))
    }
}

/// What one offset of one base was seen to hold.
#[derive(Default)]
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
#[derive(Default)]
struct Evidence {
    /// Observed offset -> widest access.
    slots: BTreeMap<intb, Slot>,
    /// A dereference whose offset was not a constant (the array signal).
    dynamic_offset: bool,
    /// The base reached an operation that treats it as an integer.
    integer_use: bool,
    /// The base flows into a phi, so some dereference of it is loop-carried.
    phi_reached: bool,
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
        self.slots
            .retain(|off, s| FIELD_WIDTHS.contains(&s.width) && off.rem_euclid(s.width as intb) == 0);
        let mut interior: Vec<intb> = Vec::new();
        let mut end: intb = intb::MIN;
        for (off, s) in self.slots.iter() {
            if *off < end {
                interior.push(*off);
            } else {
                end = *off + s.width as intb;
            }
        }
        for off in interior {
            self.slots.remove(&off);
        }
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
            ev.entry(base).or_default().record(off, width, ctype);
        }
    }

    // Negative evidence that is not about an address: the base used as a plain
    // integer, and the base flowing into a phi (an induction variable).
    let bases: Vec<VarnodeId> = ev.keys().copied().collect();
    for base in bases {
        let Some(v) = data.vbank().get(base) else { continue };
        let uses: Vec<_> = v.descend_iter().collect();
        let e = ev.entry(base).or_default();
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
        e.prune();
    }
    ev
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

/// The exact-layout signature of a completed structure: `(size, [(offset, field
/// size, metatype, pointee name)])`.
fn signature_of(ct: &Datatype) -> Option<Vec<(int4, int4, u8, String)>> {
    if ct.get_metatype() != type_metatype::TYPE_STRUCT {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..ct.num_depend() {
        let f = ct.get_field(i)?;
        let pointee = f
            .field_type
            .get_ptr_to()
            .map(|p| p.get_name().to_string())
            .unwrap_or_default();
        out.push((f.offset, f.field_type.get_size(), f.field_type.get_metatype() as u8, pointee));
    }
    Some(out)
}

/// The same signature for a layout that has not been built yet.
fn signature_of_fields(fields: &[TypeField]) -> Vec<(int4, int4, u8, String)> {
    fields
        .iter()
        .map(|f| {
            let pointee = f
                .field_type
                .get_ptr_to()
                .map(|p| p.get_name().to_string())
                .unwrap_or_default();
            (f.offset, f.field_type.get_size(), f.field_type.get_metatype() as u8, pointee)
        })
        .collect()
}

/// Find or mint the program-wide `struct_N` for this layout.
///
/// `find_add` rejects a second, different definition of a held name with a hard
/// `Err`, so every probe declines rather than propagating: a name already taken
/// by a different layout simply moves the search to `N+1`.
fn ledger_type(
    types: &dyn TypeFactory,
    fields: Vec<TypeField>,
    size: int4,
) -> Option<Rc<Datatype>> {
    let want = signature_of_fields(&fields);
    for n in 0..1024u32 {
        let name = format!("struct_{n}");
        match types.find_by_name(&name) {
            Ok(None) => {
                let shell = types.get_type_struct(&name).ok()?;
                return types
                    .set_fields_struct_raw(&shell, fields, Vec::new(), size, 1, 0)
                    .ok();
            }
            Ok(Some(t)) => {
                if t.get_size() == size && signature_of(&t).as_ref() == Some(&want) {
                    return Some(t);
                }
            }
            Err(_) => return None,
        }
    }
    None
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
    let ev = collect(data);
    let mut installs: Vec<(VarnodeId, Rc<Datatype>)> = Vec::new();

    for (base, e) in ev.iter() {
        if !accepts(data, *base, e) {
            continue;
        }
        let Some((fields, size)) = fields_for(types.as_ref(), e) else { continue };
        let Some(st) = ledger_type(types.as_ref(), fields, size) else { continue };
        // The pointer is taken only once the structure is COMPLETE: completing a
        // structure mints a fresh `Rc`, and merge compares high types by `Rc`
        // identity, so a pointer to the incomplete shell would never match.
        let ptrsize = types.get_size_of_pointer();
        let Ok(ptr) = types.get_type_pointer(ptrsize, st, 1) else { continue };
        installs.push((*base, ptr));
    }

    let mut changed = false;
    for (vn, ct) in installs {
        if data.vn_update_type_locked(vn, ct, true, false) {
            changed = true;
        }
    }
    changed
}

/// Every decline condition, in one place.
fn accepts(data: &mut Funcdata, base: VarnodeId, e: &Evidence) -> bool {
    let Some(v) = data.vbank().get(base) else { return false };

    // `param` is the only mode this version ships.
    if !v.is_input() {
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
    // Pointer-ness is not invented here.
    let Some(ct) = vn_type(data, base) else { return false };
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return false;
    }
    if points_at_named_composite(&ct) {
        return false;
    }
    // Negative evidence.
    if e.dynamic_offset || e.integer_use || e.phi_reached {
        return false;
    }
    // Two distinct offsets is the minimum evidence for a layout; a lone field at
    // offset 0 is the pointee, not a structure.
    if e.slots.len() < 2 {
        return false;
    }
    if e.slots.keys().any(|off| *off < 0 || *off >= MAX_FIELD_OFFSET) {
        return false;
    }
    if !e.slots.contains_key(&0) {
        return false;
    }
    let ptr_size = data.get_arch().types().map(|t| t.get_size_of_pointer()).unwrap_or(8);
    if is_array_shaped(&e.slots, ptr_size) {
        return false;
    }
    true
}

/// (kuna) `ActionStructSynth` -- synthesize `struct_N` over a dereferenced
/// pointer parameter (option `structsynth`).
pub struct ActionStructSynth {
    base: ActionBase,
    /// Has this function already been offered to the synthesizer?
    fired: bool,
}

impl ActionStructSynth {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionStructSynth { base: ActionBase::new(0, "structsynth", g), fired: false })
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
        Some(Box::new(ActionStructSynth { base: self.base.clone(), fired: false }))
    }
    fn reset(&mut self, _data: &mut Funcdata) {
        self.fired = false;
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().struct_synth.fires() || self.fired {
            return 0;
        }
        // A lattice that is still moving has not decided what is a pointer, and
        // one that never settled is not evidence at all.
        if !data.has_type_recovery_started()
            || data.is_type_recovery_exceeded()
            || !data.kuna_infertypes_settled()
        {
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
