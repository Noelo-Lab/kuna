//! (kuna `castarith`) Pointer arithmetic stays in pointer terms.
//!
//! A pointer plus a constant byte offset reaches `ActionSetCasts` as a plain
//! `INT_ADD` whenever nothing in P5 could build a `PTRSUB`/`PTRADD` for it: a
//! `void *` base has no field and no element size, and a typed base whose
//! element does not match the access width has neither either.  The ported cast
//! rules then price the `INT_ADD` as integer arithmetic, casting the pointer to
//! an integer on the way in and the sum back to a pointer on the way out, so a
//! 4-byte read at `+0xac` prints `*(unsigned int *)((long)a0 + 0xac)`.
//!
//! When the sum is used as a `T *` -- the value a lone LOAD or STORE moves
//! through it, else the pointee of its own type, a `void` pointee counting in
//! bytes -- and the offset is a whole number of `T`s, the same address is `k`
//! elements of `T` past the base.  [`rewrite`] turns the `INT_ADD` into
//! `PTRADD(base, #k, #sizeof T)` before the cast rules see it, with the base
//! cast to `T *` unless it is a variable declared as a pointer to `T` (or to an
//! integer of `T`'s width); a base that is an implied cast is retargeted when
//! only this op reads it, and otherwise cast again from its own input, so no
//! cast stacks on another.  The printer then renders it the way it renders
//! every `PTRADD`: `((unsigned int *)a0)[0x2b]` under a dereference and
//! `&((T *)p)[k]` (or `(T *)p + k` with `arraynotation off`) as a value.
//!
//! The value is unchanged: the element size is the access width, C's `sizeof`
//! of the printed `T` is that same size, `k * sizeof(T)` is the original
//! offset exactly, the base is converted pointer-to-pointer (no integer round
//! trip), the result carries the same pointer type the sum had, and the printed
//! index reads back as `k`.  The `sizeof` condition is why an enum element is
//! refused: kuna prints every enum as a plain `enum`, which C sizes by its own
//! rules, while a packed enum, `-fshort-enums` or a C++ `enum class : uint8_t`
//! is 1 or 2 bytes in the binary; the same holds for `long double` and for an
//! integer of a width C names no type for.  The index condition is why a
//! negative index of 2^31 elements or more is refused: C types the literal
//! `0x80000000` through `0xffffffff` as `unsigned int`, so `p[-0x80000000]`
//! would index forward, while the integer form's byte offset is then a `long`
//! literal.  The integer form stays wherever the printed C could convert a
//! value or cost a cast instead: an offset that is not a multiple of
//! `sizeof(T)`, an index that is not a constant, an aggregate or enum target,
//! a word-addressed space, a sum read as an integer or assigned to a variable
//! declared as one, an address several accesses share, a store of a value the
//! pass has not typed yet, a `void *` sum that leaves the function, and a
//! constant that names a global or is
//! address-like beside an integer cast to a pointer (`table[i]` compiles to the
//! same `INT_ADD`, with the table as the constant).

use std::rc::Rc;

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

/// Why an `INT_ADD` keeps its integer form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Leave {
    NoPointer,
    NonConstant,
    OutNotPointer,
    WordAddressed,
    VoidTarget,
    AggregateTarget,
    Unsized,
    NonDividing,
    AddressLike,
    UnsettledStore,
    IntegerUse,
    SharedAddress,
    WideNegativeIndex,
    EnumTarget,
    /// `castindex`: both operands of the sum are pointers.
    TwoPointers,
    /// `castindex`: the index is not an integer the subscript converts as the
    /// integer form does (a pointer, a float, a `bool`, an enum, or a cast the
    /// pass inserted).
    IndexNotInteger,
    /// `castindex`: the scale is not the element's size.
    ScaleMismatch,
    /// `castindex`: the scaling op has not been cast yet.
    ScaleUnsettled,
    /// `castindex`: an operand of the difference is not a pointer.
    DiffNotPointers,
    /// `castindex`: the pointees differ, or are wider than a byte.
    DiffPointee,
    /// `castindex`: the difference is not an integer of the pointer's width.
    DiffResult,
}

/// How the `PTRADD` reaches a `T *` base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Base {
    /// The base already points at exactly `T`.
    Direct,
    /// A new `(T *)` cast over the base.
    Cast,
    /// The base is an implied cast read only here; it is retargeted to `T *`.
    Retype,
    /// The base is an implied cast other ops read too; the new `(T *)` cast
    /// reads that cast's input, so no cast stacks on it.
    Recast(VarnodeId),
}

/// The `PTRADD` an `INT_ADD` becomes.
#[derive(Debug, Clone)]
pub(crate) struct Plan {
    ptr_slot: int4,
    index: i64,
    elem: int4,
    target: Rc<Datatype>,
    base: Base,
}

fn strip_typedefs(mut dt: Rc<Datatype>) -> Rc<Datatype> {
    while let Some(td) = dt.get_typedef() {
        let td = Rc::clone(td);
        dt = td;
    }
    dt
}

fn signed_value(off: uintb, size: int4) -> i64 {
    if size >= 8 {
        return off as i64;
    }
    let bits = (size * 8) as u32;
    ((off << (64 - bits)) as i64) >> (64 - bits)
}

/// The input of the implied cast `vn`, when it is an integer or a pointer a
/// `(T *)` cast can read directly: converting it straight to `T *` keeps the
/// bits the two casts would.
fn implied_cast_source(data: &mut Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    let (def, src) = {
        let v = data.vbank().get(vn)?;
        if !v.is_implied() || v.is_explicit() {
            return None;
        }
        let def = v.get_def()?;
        let d = data.obank().get(def)?;
        if d.code() != OpCode::CPUI_CAST {
            return None;
        }
        (def, d.get_in(0)?)
    };
    if data.vbank().get(src)?.is_constant() {
        return None;
    }
    let t = data.vn_high_type_read_facing(src, def);
    matches!(
        t.get_metatype(),
        type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_UNKNOWN
            | type_metatype::TYPE_PTR
    )
    .then_some(src)
}

fn cast_def_read_only_here(data: &Funcdata, vn: VarnodeId, op: OpId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    if !v.is_implied() || !v.is_written() || v.is_type_lock() {
        return false;
    }
    let is_cast = v
        .get_def()
        .and_then(|d| data.obank().get(d))
        .map(|o| o.code() == OpCode::CPUI_CAST)
        .unwrap_or(false);
    is_cast && data.lone_descend(vn) == Some(op)
}

/// Does a `pointee *` base index `target` elements with no cast?  The same type
/// does; so does another integer of the same size (either signedness, or a
/// value no pass typed beyond its width), because C's own conversion between
/// them keeps every bit and the cast pass already moves a loaded or stored value
/// between them without a token.  A float never counts: converting it changes
/// the bits.
fn same_element(pointee: &Rc<Datatype>, target: &Rc<Datatype>) -> bool {
    if Rc::ptr_eq(pointee, target) {
        return true;
    }
    let int = |t: &Rc<Datatype>| {
        matches!(
            t.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        ) && !t.is_enum_type()
    };
    int(pointee) && int(target) && pointee.get_size() == target.get_size()
}

/// Is `target`'s size the `sizeof` C gives it as printed?  The printed index is
/// scaled by that `sizeof`, so the element must be a type the target's data
/// model names at exactly the width the rewrite measured: an integer of a width
/// the model has, a one-byte `bool`, a `float` or `double`, or a pointer of the
/// model's pointer size.  `long double` is not one (its `sizeof` is 16, 12 or 8
/// by target, never the 10 bytes of the value), and neither is an integer of a
/// width C names no type for.
fn c_sizeof_is_size(types: &crate::dtype::TypeFactoryImpl, target: &Rc<Datatype>) -> bool {
    let size = target.get_size();
    let model = crate::kuna_ctypes::CDataModel::from_types(types);
    match target.get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => {
            model.integer_spelling(size, false).is_some()
        }
        type_metatype::TYPE_BOOL => size == 1,
        type_metatype::TYPE_FLOAT => size == model.float_size || size == model.double_size,
        type_metatype::TYPE_PTR => size == types.get_size_of_pointer(),
        _ => false,
    }
}

/// Is `vn` a variable the printed C declares as `ptype`?  Only then does a bare
/// `vn[k]` scale by the element the rewrite measured: the read must need no
/// PTRADD input cast (its own type is `ptype`) and the declaration a reader sees
/// -- a parameter's prototype type, or a local's declared type -- must point at
/// the same element.
fn declared_as(data: &Funcdata, vn: VarnodeId, op: OpId, ptype: &Rc<Datatype>) -> bool {
    if !Rc::ptr_eq(&data.vn_type_read_facing(vn, op), ptype) {
        return false;
    }
    let Some(decl) =
        crate::printc::declared_variable_type(data, data.get_arch().decl_high_type, vn)
    else {
        return false;
    };
    let elem = |t: &Rc<Datatype>| t.get_ptr_to().map(strip_typedefs);
    decl.get_metatype() == type_metatype::TYPE_PTR
        && decl.get_size() == ptype.get_size()
        && matches!((elem(&decl), elem(ptype)), (Some(a), Some(b)) if Rc::ptr_eq(&a, &b))
}

/// Is `vn` an integer the cast pass has already turned into a pointer?  With an
/// address-like constant beside it, that is `table[i]`: the constant is the
/// table and the "pointer" is the subscript.
fn integer_made_pointer(data: &mut Funcdata, vn: VarnodeId) -> bool {
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else {
        return false;
    };
    let Some(input) = data
        .obank()
        .get(def)
        .filter(|o| o.code() == OpCode::CPUI_CAST)
        .and_then(|o| o.get_in(0))
    else {
        return false;
    };
    data.vn_high_type_read_facing(input, def).get_metatype() != type_metatype::TYPE_PTR
}

/// Does the value of `out` stay inside this function?  A `void *` sum stepped in
/// bytes has type `char *`, and an argument or return value carries its type to
/// the whole-program votes on other functions' prototypes, which must not learn
/// a `char *` the analysis never derived.  A variable keeps its declared type.
fn stays_in_function(data: &Funcdata, out: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(out) else {
        return false;
    };
    if v.is_explicit() {
        return true;
    }
    let mut readers = v.descend_iter().peekable();
    readers.peek().is_some()
        && readers.all(|r| {
            data.obank().get(r).is_some_and(|o| {
                !matches!(
                    o.code(),
                    OpCode::CPUI_CALL
                        | OpCode::CPUI_CALLIND
                        | OpCode::CPUI_CALLOTHER
                        | OpCode::CPUI_RETURN
                        | OpCode::CPUI_INDIRECT
                )
            })
        })
}

/// Is the implied sum `out` read as an integer?  Integer arithmetic on it, or a
/// store of it into an integer slot, converts it straight back with a `(long)`,
/// which the integer form already folds into its one cast.
fn read_as_integer(data: &mut Funcdata, out: VarnodeId) -> bool {
    let readers: Vec<OpId> = match data.vbank().get(out) {
        Some(v) if !v.is_explicit() => v.descend_iter().collect(),
        _ => return false,
    };
    readers.into_iter().any(|reader| {
        let Some((code, slot, addr)) = data
            .obank()
            .get(reader)
            .map(|r| (r.code(), r.get_slot(out), r.get_in(1)))
        else {
            return false;
        };
        match code {
            OpCode::CPUI_STORE if slot == 2 => addr.is_some_and(|a| {
                let at = data.vn_high_type_read_facing(a, reader);
                at.get_metatype() != type_metatype::TYPE_PTR
                    || at.get_ptr_to().is_some_and(|p| {
                        strip_typedefs(p).get_metatype() != type_metatype::TYPE_PTR
                    })
            }),
            _ => integer_result(code) && !is_comparison(code),
        }
    })
}

/// Is `out` a variable the printed C declares as something other than a
/// pointer?  The sum then lands in an integer, which the integer form fills
/// with no cast and pointer arithmetic would fill through two.
fn declared_non_pointer(data: &Funcdata, out: VarnodeId) -> bool {
    data.vbank().get(out).is_some_and(|v| v.is_explicit())
        && crate::printc::declared_variable_type(data, data.get_arch().decl_high_type, out)
            .is_some_and(|t| t.get_metatype() != type_metatype::TYPE_PTR)
}

/// Is the implied `out` the address of a LOAD or STORE that is not its only
/// reader?  Only a lone access fixes the element (`access_type`); several would
/// each have to agree with it, and a store of another kind of value would be
/// converted.  A variable keeps its own type, so its accesses cast as before.
fn shared_address(data: &Funcdata, out: VarnodeId) -> bool {
    data.vbank().get(out).is_some_and(|v| {
        !v.is_explicit()
            && v.descend_iter().any(|r| {
                data.obank().get(r).is_some_and(|o| {
                    matches!(o.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
                        && o.get_slot(out) == 1
                })
            })
    })
}

fn is_comparison(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
    )
}

/// Does `a` run before `b` in the cast pass's walk (blocks in index order, ops
/// in block order)?
fn runs_before(data: &Funcdata, a: OpId, b: OpId) -> bool {
    let (Some(oa), Some(ob)) = (data.obank().get(a), data.obank().get(b)) else {
        return false;
    };
    let (Some(pa), Some(pb)) = (oa.get_parent(), ob.get_parent()) else {
        return false;
    };
    if pa == pb {
        return oa.get_seq_num().get_order() < ob.get_seq_num().get_order();
    }
    let g = data.bblocks_ref();
    g.block(pa).get_index() < g.block(pb).get_index()
}

/// What a lone LOAD or STORE moves through the implied address `out`.
enum Access {
    /// The value type, final by the time the reader is cast.
    Value(Rc<Datatype>),
    /// A STORE of a temporary whose defining op the pass has not reached: its
    /// type can still change there (a float op's result is retyped `float`), so
    /// no element type chosen now is safe.
    Unsettled,
    /// Not the address of a lone LOAD or STORE.
    None,
}

fn access_type(data: &mut Funcdata, out: VarnodeId, op: OpId) -> Access {
    let Some(v) = data.vbank().get(out) else {
        return Access::None;
    };
    if v.is_explicit() || v.is_type_lock() {
        return Access::None;
    }
    let Some(reader) = data.lone_descend(out) else {
        return Access::None;
    };
    let (code, slot, value) = {
        let Some(r) = data.obank().get(reader) else {
            return Access::None;
        };
        let value = match r.code() {
            OpCode::CPUI_LOAD => r.get_out(),
            OpCode::CPUI_STORE if !r.does_special_printing() => r.get_in(2),
            _ => None,
        };
        let Some(value) = value else {
            return Access::None;
        };
        (r.code(), r.get_slot(out), value)
    };
    if slot != 1 {
        return Access::None;
    }
    let t = if code == OpCode::CPUI_LOAD {
        data.vn_high_type_def_facing(value)
    } else {
        data.vn_high_type_read_facing(value, reader)
    };
    let size = data.vbank().get(value).map(|v| v.get_size()).unwrap_or(0);
    if t.get_size() != size {
        return Access::None;
    }
    if code == OpCode::CPUI_STORE && !store_value_settled(data, value, op, &t) {
        return Access::Unsettled;
    }
    Access::Value(t)
}

/// Will the value a STORE writes still be of `t`'s kind when the STORE is cast?
/// A variable, a constant, an input and the result of an op the pass has already
/// cast keep their type.  The result of an op still ahead is retyped there to
/// the op's own token type, which the opcode fixes to a float for the float
/// arithmetic and to an integer (or a pointer, printed with a bit-preserving cast)
/// for the integer ops, where a `bool` element would convert rather than keep
/// the bits; any other op could produce anything.
fn store_value_settled(data: &Funcdata, value: VarnodeId, op: OpId, t: &Rc<Datatype>) -> bool {
    let Some(v) = data.vbank().get(value) else {
        return false;
    };
    if v.is_constant() || v.is_explicit() || !v.is_written() {
        return true;
    }
    let Some(def) = v.get_def() else { return false };
    if runs_before(data, def, op) {
        return true;
    }
    let Some(code) = data.obank().get(def).map(|o| o.code()) else {
        return false;
    };
    let float = t.get_metatype() == type_metatype::TYPE_FLOAT;
    if float_result(code) {
        return float;
    }
    !float && t.get_metatype() != type_metatype::TYPE_BOOL && integer_result(code)
}

fn float_result(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_INT2FLOAT
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
    )
}

fn integer_result(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_POPCOUNT
            | OpCode::CPUI_LZCOUNT
            | OpCode::CPUI_FLOAT_TRUNC
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN
    )
}

/// The element `T` a sum of the pointer `ptype` and an offset steps in, and the
/// `T *` it is printed as: the value a lone LOAD reads or a lone STORE writes
/// through the sum (the sum's own pointee when that is the same type or an
/// integer of the same width), else the sum's own pointee, a `void` pointee
/// counting in bytes for a value that stays in the function.  `T` must be a
/// scalar or pointer C sizes at exactly its width.
fn element(
    data: &mut Funcdata,
    op: OpId,
    out: VarnodeId,
    ptype: &Rc<Datatype>,
) -> Result<(Rc<Datatype>, Rc<Datatype>), Leave> {
    let out_size = data.vbank().get(out).map(|v| v.get_size()).unwrap_or(0);
    if ptype.get_word_size() != Some(1) || ptype.get_size() != out_size {
        return Err(Leave::WordAddressed);
    }
    let tlst = data.get_arch().types_rc().ok_or(Leave::WordAddressed)?;
    let outty = data.vn_high_type_def_facing(out);
    let out_pointee = (outty.get_metatype() == type_metatype::TYPE_PTR
        && outty.get_word_size() == Some(1)
        && outty.get_size() == out_size)
        .then(|| outty.get_ptr_to().map(strip_typedefs))
        .flatten();
    let (target, target_ptr) = match access_type(data, out, op) {
        Access::Unsettled => return Err(Leave::UnsettledStore),
        Access::Value(a) => {
            let a_elem = strip_typedefs(Rc::clone(&a));
            match out_pointee {
                Some(p) if p.get_size() == a_elem.get_size() && same_element(&p, &a_elem) => {
                    (p, outty)
                }
                _ => {
                    let ptr = tlst
                        .get_type_pointer(out_size, a, 1)
                        .map_err(|_| Leave::Unsized)?;
                    (a_elem, ptr)
                }
            }
        }
        Access::None => {
            if outty.get_metatype() != type_metatype::TYPE_PTR {
                return Err(Leave::OutNotPointer);
            }
            if read_as_integer(data, out) || declared_non_pointer(data, out) {
                return Err(Leave::IntegerUse);
            }
            if shared_address(data, out) {
                return Err(Leave::SharedAddress);
            }
            let pointee = out_pointee.ok_or(Leave::WordAddressed)?;
            if pointee.get_metatype() != type_metatype::TYPE_VOID {
                (pointee, outty)
            } else if stays_in_function(data, out) {
                let byte = tlst
                    .get_base(1, type_metatype::TYPE_INT)
                    .map_err(|_| Leave::VoidTarget)?;
                let ptr = tlst
                    .get_type_pointer(out_size, Rc::clone(&byte), 1)
                    .map_err(|_| Leave::VoidTarget)?;
                (byte, ptr)
            } else {
                return Err(Leave::VoidTarget);
            }
        }
    };
    if target.is_enum_type() {
        return Err(Leave::EnumTarget);
    }
    match target.get_metatype() {
        type_metatype::TYPE_INT
        | type_metatype::TYPE_UINT
        | type_metatype::TYPE_BOOL
        | type_metatype::TYPE_FLOAT
        | type_metatype::TYPE_PTR
        | type_metatype::TYPE_UNKNOWN => {}
        type_metatype::TYPE_VOID => return Err(Leave::VoidTarget),
        _ => return Err(Leave::AggregateTarget),
    }
    let elem = target.get_size();
    if elem <= 0 || elem != target.get_align_size() || !c_sizeof_is_size(&tlst, &target) {
        return Err(Leave::Unsized);
    }
    Ok((target, target_ptr))
}

/// Decide whether the `INT_ADD` `op` becomes pointer arithmetic.
pub(crate) fn plan(data: &mut Funcdata, op: OpId) -> Result<Plan, Leave> {
    let (out, ins) = {
        let o = data.obank().get(op).ok_or(Leave::NoPointer)?;
        if o.code() != OpCode::CPUI_INT_ADD || o.num_input() != 2 {
            return Err(Leave::NoPointer);
        }
        let out = o.get_out().ok_or(Leave::NoPointer)?;
        (
            out,
            [
                o.get_in(0).ok_or(Leave::NoPointer)?,
                o.get_in(1).ok_or(Leave::NoPointer)?,
            ],
        )
    };
    let mut found: Option<(int4, Rc<Datatype>)> = None;
    let mut saw_pointer = false;
    for slot in 0..2 {
        let vn = ins[slot];
        let other = ins[1 - slot];
        if data
            .vbank()
            .get(vn)
            .map(|v| v.is_constant() || v.is_annotation())
            .unwrap_or(true)
        {
            continue;
        }
        let pt = data.vn_high_type_read_facing(vn, op);
        if pt.get_metatype() != type_metatype::TYPE_PTR {
            continue;
        }
        saw_pointer = true;
        if data
            .vbank()
            .get(other)
            .map(|v| v.is_constant())
            .unwrap_or(false)
        {
            found = Some((slot as int4, pt));
            break;
        }
    }
    let (ptr_slot, ptype) = match found {
        Some(f) => f,
        None if saw_pointer => return Err(Leave::NonConstant),
        None => return Err(Leave::NoPointer),
    };
    let (target, target_ptr) = element(data, op, out, &ptype)?;
    let elem = target.get_size();
    let (koff, ksize) = {
        let c = data
            .vbank()
            .get(ins[1 - ptr_slot as usize])
            .ok_or(Leave::NonConstant)?;
        (c.get_offset(), c.get_size())
    };
    let k = signed_value(koff, ksize);
    if k % elem as i64 != 0 {
        return Err(Leave::NonDividing);
    }
    let index = k / elem as i64;
    if index < -(i32::MAX as i64) {
        return Err(Leave::WideNegativeIndex);
    }
    let cvn = ins[1 - ptr_slot as usize];
    let base_vn = ins[ptr_slot as usize];
    if crate::kuna_ptrfromuse::constant_is_global_base(data, op, cvn)
        || (integer_made_pointer(data, base_vn)
            && crate::kuna_ptrfromuse::constant_may_be_global_base(data, op, cvn))
    {
        return Err(Leave::AddressLike);
    }
    let pointee = ptype.get_ptr_to().map(strip_typedefs);
    let base = if pointee.is_some_and(|p| same_element(&p, &target))
        && declared_as(data, base_vn, op, &ptype)
    {
        Base::Direct
    } else if cast_def_read_only_here(data, base_vn, op) {
        Base::Retype
    } else if let Some(src) = implied_cast_source(data, base_vn) {
        Base::Recast(src)
    } else {
        Base::Cast
    };
    Ok(Plan {
        ptr_slot,
        index,
        elem,
        target: target_ptr,
        base,
    })
}

/// Rewrite the `INT_ADD` `op` as the `PTRADD` `plan` describes.
pub(crate) fn apply(data: &mut Funcdata, op: OpId, plan: &Plan) -> Option<()> {
    let (base_vn, addr, out) = {
        let o = data.obank().get(op)?;
        (o.get_in(plan.ptr_slot)?, o.get_addr().clone(), o.get_out()?)
    };
    let ptrsize = data.vbank().get(out)?.get_size();
    let tlst = data.get_arch().types_rc()?;
    let index_type = tlst.get_base(ptrsize, type_metatype::TYPE_INT).ok()?;
    let base = match plan.base {
        Base::Direct => base_vn,
        Base::Retype => {
            let _ = data.vn_update_type(base_vn, Rc::clone(&plan.target));
            base_vn
        }
        Base::Cast | Base::Recast(_) => {
            let src = match plan.base {
                Base::Recast(src) => src,
                _ => base_vn,
            };
            let cast = data.new_op(1, addr);
            let cvn = data.new_unique_out(ptrsize, cast).ok()?;
            let _ = data.vn_update_type(cvn, Rc::clone(&plan.target));
            if let Some(v) = data.vbank_mut().get_mut(cvn) {
                v.set_implied();
            }
            data.op_set_opcode_code(cast, OpCode::CPUI_CAST);
            data.op_set_input(cast, src, 0).ok()?;
            data.op_insert_before(cast, op);
            cvn
        }
    };
    let mask = kuna_base::address::calc_mask(ptrsize);
    let index = data.new_constant(ptrsize, (plan.index as uintb) & mask);
    let _ = data.vn_update_type(index, index_type);
    let elem = data.new_constant(ptrsize, plan.elem as uintb);
    data.op_set_all_input(op, &[base, index, elem]).ok()?;
    data.op_set_opcode_code(op, OpCode::CPUI_PTRADD);
    Some(())
}

/// Turn a qualifying `INT_ADD` into pointer arithmetic; `true` if it did.
pub(crate) fn rewrite(data: &mut Funcdata, op: OpId) -> bool {
    match plan(data, op) {
        Ok(p) => apply(data, op, &p).is_some(),
        Err(_) => false,
    }
}

/// The index an `INT_ADD`'s variable operand stands for: the operand itself
/// counted in bytes, or, when the operand is an implied `x * S` (or `x << s`)
/// read only by the add, `x` counted in `S`-byte steps.
struct Scaled {
    index: VarnodeId,
    scale: i64,
    by: Option<OpId>,
}

fn scaled_index(data: &Funcdata, vn: VarnodeId, op: OpId) -> Option<Scaled> {
    let v = data.vbank().get(vn)?;
    if v.is_explicit() || v.is_type_lock() || data.lone_descend(vn) != Some(op) {
        return None;
    }
    let def = v.get_def()?;
    let d = data.obank().get(def)?;
    let konst = |slot: int4| {
        d.get_in(slot)
            .and_then(|k| data.vbank().get(k))
            .filter(|k| k.is_constant())
            .map(|k| signed_value(k.get_offset(), k.get_size()))
    };
    let (index, scale) = match d.code() {
        OpCode::CPUI_INT_MULT => match (konst(0), konst(1)) {
            (None, Some(k)) => (d.get_in(0)?, k),
            (Some(k), None) => (d.get_in(1)?, k),
            _ => return None,
        },
        OpCode::CPUI_INT_LEFT => match konst(1) {
            Some(s) if (0..31).contains(&s) => (d.get_in(0)?, 1i64 << s),
            _ => return None,
        },
        _ => return None,
    };
    if !(1..=i32::MAX as i64).contains(&scale) || data.vbank().get(index)?.is_constant() {
        return None;
    }
    Some(Scaled {
        index,
        scale,
        by: Some(def),
    })
}

/// Does the subscript `p[vn]`, read by `op`, index by exactly the value the
/// integer form adds?  C converts an integer subscript to the pointer's width by
/// its own type's signedness, which is the extension the pass made explicit (an
/// implied `SEXT`/`ZEXT` prints bare under a `PTRADD` only when its input's
/// type extends that way, and the pass casts the input where it does not); a
/// pointer-width integer converts to itself.  A pointer, a float, a `bool`, an
/// enum and a value the pass has already cast (`(long)p`) are not such indexes.
fn integer_index(data: &mut Funcdata, vn: VarnodeId, op: OpId, width: int4) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    if v.get_size() != width || v.is_constant() {
        return false;
    }
    let cast_def = v
        .get_def()
        .and_then(|d| data.obank().get(d))
        .is_some_and(|d| d.code() == OpCode::CPUI_CAST);
    if cast_def {
        return false;
    }
    let t = data.vn_high_type_read_facing(vn, op);
    matches!(
        t.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    ) && !t.is_enum_type()
}

/// The `PTRADD` an `INT_ADD` of a pointer and a variable index becomes.
#[derive(Debug, Clone)]
pub(crate) struct IndexPlan {
    ptr_slot: int4,
    index: VarnodeId,
    by: Option<OpId>,
    elem: int4,
    target: Rc<Datatype>,
    base: Base,
}

/// Decide whether the `INT_ADD` `op` of a pointer and a variable becomes a
/// subscript (`castindex`).  The element is chosen exactly as for a constant
/// offset; the index is the variable when the element is one byte, or the `x`
/// of an implied `x * sizeof(T)` the add alone reads.
pub(crate) fn plan_index(data: &mut Funcdata, op: OpId) -> Result<IndexPlan, Leave> {
    let (out, ins) = {
        let o = data.obank().get(op).ok_or(Leave::NoPointer)?;
        if o.code() != OpCode::CPUI_INT_ADD || o.num_input() != 2 {
            return Err(Leave::NoPointer);
        }
        let out = o.get_out().ok_or(Leave::NoPointer)?;
        (
            out,
            [
                o.get_in(0).ok_or(Leave::NoPointer)?,
                o.get_in(1).ok_or(Leave::NoPointer)?,
            ],
        )
    };
    let mut pointers = Vec::new();
    for (slot, &vn) in ins.iter().enumerate() {
        let v = data.vbank().get(vn).ok_or(Leave::NoPointer)?;
        if v.is_constant() || v.is_annotation() {
            return Err(Leave::NoPointer);
        }
        let t = data.vn_high_type_read_facing(vn, op);
        if t.get_metatype() == type_metatype::TYPE_PTR {
            pointers.push((slot as int4, t));
        }
    }
    let (ptr_slot, ptype) = match pointers.len() {
        0 => return Err(Leave::NoPointer),
        1 => pointers.pop().ok_or(Leave::NoPointer)?,
        _ => return Err(Leave::TwoPointers),
    };
    let other = ins[1 - ptr_slot as usize];
    let (target, target_ptr) = element(data, op, out, &ptype)?;
    let elem = target.get_size();
    let width = ptype.get_size();
    let scaled = scaled_index(data, other, op).filter(|s| s.scale == elem as i64);
    let (index, by) = match scaled {
        Some(s) => {
            let by = s.by.ok_or(Leave::ScaleMismatch)?;
            if !runs_before(data, by, op) {
                return Err(Leave::ScaleUnsettled);
            }
            (s.index, Some(by))
        }
        None if elem == 1 => (other, None),
        None => return Err(Leave::ScaleMismatch),
    };
    let reader = by.unwrap_or(op);
    if !integer_index(data, index, reader, width) {
        return Err(Leave::IndexNotInteger);
    }
    let base_vn = ins[ptr_slot as usize];
    let pointee = ptype.get_ptr_to().map(strip_typedefs);
    let base = if pointee.is_some_and(|p| same_element(&p, &target))
        && declared_as(data, base_vn, op, &ptype)
    {
        Base::Direct
    } else if cast_def_read_only_here(data, base_vn, op) {
        Base::Retype
    } else if let Some(src) = implied_cast_source(data, base_vn) {
        Base::Recast(src)
    } else {
        Base::Cast
    };
    Ok(IndexPlan {
        ptr_slot,
        index,
        by,
        elem,
        target: target_ptr,
        base,
    })
}

/// Rewrite the `INT_ADD` `op` as the subscript `plan` describes, dropping the
/// scaling op it no longer reads.
pub(crate) fn apply_index(data: &mut Funcdata, op: OpId, plan: &IndexPlan) -> Option<()> {
    let (base_vn, addr, out) = {
        let o = data.obank().get(op)?;
        (o.get_in(plan.ptr_slot)?, o.get_addr().clone(), o.get_out()?)
    };
    let ptrsize = data.vbank().get(out)?.get_size();
    let base = match plan.base {
        Base::Direct => base_vn,
        Base::Retype => {
            let _ = data.vn_update_type(base_vn, Rc::clone(&plan.target));
            base_vn
        }
        Base::Cast | Base::Recast(_) => {
            let src = match plan.base {
                Base::Recast(src) => src,
                _ => base_vn,
            };
            let cast = data.new_op(1, addr);
            let cvn = data.new_unique_out(ptrsize, cast).ok()?;
            let _ = data.vn_update_type(cvn, Rc::clone(&plan.target));
            if let Some(v) = data.vbank_mut().get_mut(cvn) {
                v.set_implied();
            }
            data.op_set_opcode_code(cast, OpCode::CPUI_CAST);
            data.op_set_input(cast, src, 0).ok()?;
            data.op_insert_before(cast, op);
            cvn
        }
    };
    let elem = data.new_constant(ptrsize, plan.elem as uintb);
    data.op_set_all_input(op, &[base, plan.index, elem]).ok()?;
    data.op_set_opcode_code(op, OpCode::CPUI_PTRADD);
    if let Some(by) = plan.by {
        data.op_destroy(by);
    }
    Some(())
}

/// Turn a pointer plus a variable index into a subscript; `true` if it did.
pub(crate) fn rewrite_index(data: &mut Funcdata, op: OpId) -> bool {
    match plan_index(data, op) {
        Ok(p) => apply_index(data, op, &p).is_some(),
        Err(_) => false,
    }
}

/// Is the `INT_SUB` `op` the difference of two pointers C subtracts to the same
/// value (`castindex`)?  Both operands point at the same one-byte integer type,
/// so the element difference `p - q` is the byte difference the integer form
/// computes, and the result is an integer of the pointer's width, the width of
/// the `ptrdiff_t` that `p - q` has and of the signed integer the pass would
/// cast both operands to.  The cast pass then leaves both operands as they are,
/// and casts the result from the returned token, that signed integer, exactly as
/// it cast the integer form's.
pub(crate) fn pointer_difference(data: &mut Funcdata, op: OpId) -> Result<Rc<Datatype>, Leave> {
    let (out, a, b) = {
        let o = data.obank().get(op).ok_or(Leave::DiffNotPointers)?;
        if o.code() != OpCode::CPUI_INT_SUB || o.num_input() != 2 {
            return Err(Leave::DiffNotPointers);
        }
        (
            o.get_out().ok_or(Leave::DiffNotPointers)?,
            o.get_in(0).ok_or(Leave::DiffNotPointers)?,
            o.get_in(1).ok_or(Leave::DiffNotPointers)?,
        )
    };
    let tlst = data.get_arch().types_rc().ok_or(Leave::DiffNotPointers)?;
    let mut pointee: Option<Rc<Datatype>> = None;
    let mut width = 0;
    for vn in [a, b] {
        if data.vbank().get(vn).is_none_or(|v| v.is_constant() || v.is_annotation()) {
            return Err(Leave::DiffNotPointers);
        }
        let t = data.vn_high_type_read_facing(vn, op);
        if t.get_metatype() != type_metatype::TYPE_PTR || t.get_word_size() != Some(1) {
            return Err(Leave::DiffNotPointers);
        }
        width = t.get_size();
        let p = t.get_ptr_to().map(strip_typedefs).ok_or(Leave::DiffPointee)?;
        let byte = matches!(
            p.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        ) && !p.is_enum_type()
            && p.get_size() == 1
            && c_sizeof_is_size(&tlst, &p);
        if !byte || pointee.as_ref().is_some_and(|q| !Rc::ptr_eq(q, &p)) {
            return Err(Leave::DiffPointee);
        }
        pointee = Some(p);
    }
    let ot = data.vn_high_type_def_facing(out);
    let osize = data.vbank().get(out).map(|v| v.get_size()).unwrap_or(0);
    if osize != width
        || !matches!(
            ot.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
        || ot.is_enum_type()
    {
        return Err(Leave::DiffResult);
    }
    tlst.get_base(width, type_metatype::TYPE_INT)
        .map_err(|_| Leave::DiffResult)
}

#[cfg(test)]
#[path = "kuna_castarith/tests.rs"]
mod tests;
