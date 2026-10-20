//! (kuna `partialaccess`) Print a partial access of a variable that no member
//! names as C instead of Ghidra's `name._<off>_<size>_`.
//!
//! `PrintC::pushPartialSymbol` (printc.cc:2106-2117) renders the bytes of a
//! variable that no field covers as an artificial member, `v1._4_2_` for the two
//! bytes at offset 4. That is not C: no compiler accepts it and a reader has to
//! know the convention. With the option on, an access of a width C has a scalar
//! for prints as the pointer access it is, the spelling kuna already uses for
//! the same bytes reached through a pointer:
//!
//! * `((T *)&v1)[k]` when the offset is a multiple of the width (`k` = offset /
//!   width), which is IDA's `WORDn`/`BYTEn` reading;
//! * `*(T *)&v1` at offset 0;
//! * `*(T *)((char *)&v1 + off)` for an unaligned offset;
//! * `*(T *)&buf[off]`, and `*(T *)buf` at offset 0, when the bytes are taken
//!   out of a byte array, whose subscripts already count bytes;
//! * `((char *)&v1)[off]` where the caller takes the address of a position
//!   inside the variable rather than reading bytes.
//!
//! `T` is the accessed value's own type when it is a scalar of exactly the
//! access width, else the unknown type of that width -- or a float of that width
//! when an unknown-typed access carries a float value (a float operation wrote
//! or reads it, or it copies one), since an integer `T` would make C convert
//! the value where the binary moves its bits. A 16-byte access on a 64-bit
//! target reads through `unsigned __int128`, the GCC and Clang 16-byte integer.
//! The member path in front of the token is kept
//! (`((unsigned int *)&v1.flagfield)[1]`), every form is an lvalue so a partial
//! store prints the same way, and the offset is the byte offset into the
//! variable's storage, as it is for the token it replaces, so the reading is the
//! target's memory order on every architecture.
//!
//! C has no scalar of 3, 5, 6, 7 or 9 to 15 bytes. A plain store or copy of such
//! a width prints as `memset`/`memcpy` of exactly its bytes (the statement form
//! lives in the printer); a read of 3, 5, 6 or 7 bytes out of a scalar prints as
//! the `SUB<in><out>(v, k)` intrinsic the surrounding output already uses for a
//! truncation, with `k` the byte offset counted from the value's low end. Any
//! other access of such a width keeps Ghidra's token, which no compiler accepts,
//! rather than a spelling that would compile to another value.

use std::rc::Rc;

use kuna_base::types::{int4, int8};

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// Which pointer form an access takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    /// `((T *)&v)[k]`.
    Index(int8),
    /// `*(T *)&v`.
    Base,
    /// `*(T *)((char *)&v + off)`.
    Bytes(int8),
    /// `*(T *)&v[k]`, in a byte array.
    Element(int8),
    /// `*(T *)v`, the start of a byte array.
    Decay,
}

/// What an access reads through.
pub enum Pointee {
    /// `T *` for a type the factory holds.
    Typed(Rc<Datatype>),
    /// `unsigned __int128 *`, which the factory has no type for.
    Int128 { volatile: bool },
}

/// The planned rendering of one partial access.
pub enum Piece {
    /// A pointer access; `bytes` is `char *`, for [`Form::Bytes`].
    Pointer { form: Form, pointee: Pointee, bytes: Option<Rc<Datatype>> },
    /// `SUB<in><out>(v, value_offset)`.
    Sub { name: String, value_offset: int8 },
}

/// Is `size` the width of a C scalar the factory spells?
pub fn is_scalar_width(size: int4) -> bool {
    matches!(size, 1 | 2 | 4 | 8)
}

/// A scalar value type spelled at exactly `size` bytes: the only kind a
/// dereference can assign or compare as a whole (an array or an aggregate of
/// the access width would print `*(char (*)[16])&v = 0`).
pub(crate) fn readable(ct: &Datatype, size: int4) -> bool {
    ct.get_size() == size
        && (matches!(size, 1 | 2 | 4 | 8) || ct.get_metatype() == type_metatype::TYPE_FLOAT)
        && matches!(
            ct.get_metatype(),
            type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_BOOL
                | type_metatype::TYPE_FLOAT
                | type_metatype::TYPE_PTR
                | type_metatype::TYPE_UNKNOWN
        )
}

/// Does the access carry a floating-point value its own unknown type does not
/// say: written by a float operation or a copy of a float, or read by a float
/// operation?  An integer `T` would make C convert the value instead of moving
/// its bits.
pub(crate) fn carries_float(fd: &Funcdata, vn: VarnodeId, op: OpId) -> bool {
    use kuna_num::opcodes::OpCode;
    let Some(v) = fd.vbank().get(vn) else { return false };
    let Some(o) = fd.obank().get(op) else { return false };
    if v.get_def() == Some(op) {
        let input_float = o
            .get_in(0)
            .and_then(|i| fd.vbank().get(i))
            .is_some_and(|i| i.get_type().get_metatype() == type_metatype::TYPE_FLOAT);
        return match o.code() {
            OpCode::CPUI_COPY => input_float,
            c => crate::kuna_ptrfromuse::is_float_op(c)
                && !matches!(
                    c,
                    OpCode::CPUI_FLOAT_EQUAL
                        | OpCode::CPUI_FLOAT_NOTEQUAL
                        | OpCode::CPUI_FLOAT_LESS
                        | OpCode::CPUI_FLOAT_LESSEQUAL
                        | OpCode::CPUI_FLOAT_NAN
                        | OpCode::CPUI_FLOAT_TRUNC
                ),
        };
    }
    match o.code() {
        OpCode::CPUI_COPY => o
            .get_out()
            .and_then(|out| fd.vbank().get(out))
            .is_some_and(|out| out.get_type().get_metatype() == type_metatype::TYPE_FLOAT),
        c => crate::kuna_ptrfromuse::is_float_op(c) && c != OpCode::CPUI_FLOAT_INT2FLOAT,
    }
}

impl Piece {
    /// Plan the access of `size` bytes at `off` inside `container`, the type the
    /// walk took the artificial member out of; `vn` is the accessed Varnode and
    /// `op` the operation printing it; `position` is the address-of-a-position
    /// case. `None` keeps Ghidra's token.
    #[allow(clippy::too_many_arguments)]
    pub fn plan(
        fd: &Funcdata,
        arch: &Architecture,
        vn: VarnodeId,
        op: OpId,
        off: int8,
        size: int4,
        position: bool,
        container: Option<&Datatype>,
    ) -> Option<Piece> {
        let types = arch.types();
        let ptr_size = types.get_size_of_pointer();
        let char_t = types.get_type_char(1).ok()?;
        let char_ptr = types.get_type_pointer(ptr_size, char_t, 1).ok()?;
        if position {
            return Some(Piece::Pointer { form: Form::Index(off), pointee: Pointee::Typed(char_ptr), bytes: None });
        }
        let own = fd
            .vbank()
            .get(vn)
            .map(|v| Rc::clone(v.get_type()))
            .filter(|t| readable(t, size))
            .filter(|t| t.get_metatype() != type_metatype::TYPE_UNKNOWN || !carries_float(fd, vn, op));
        let volatile = fd.vbank().get(vn).is_some_and(|v| v.is_annotation() && v.is_volatile());
        let pointee = match own {
            Some(t) => Pointee::Typed(types.get_type_pointer(ptr_size, t, 1).ok()?),
            None if matches!(size, 4 | 8) && carries_float(fd, vn, op) => {
                let t = types.get_base(size, type_metatype::TYPE_FLOAT).ok()?;
                Pointee::Typed(types.get_type_pointer(ptr_size, t, 1).ok()?)
            }
            None if is_scalar_width(size) => {
                let t = types.get_base(size, type_metatype::TYPE_UNKNOWN).ok()?;
                Pointee::Typed(types.get_type_pointer(ptr_size, t, 1).ok()?)
            }
            None if size == 16 && ptr_size == 8 => Pointee::Int128 { volatile },
            None if volatile => return None,
            None => return Self::sub(fd, vn, op, off, size, container),
        };
        let pointee = match pointee {
            Pointee::Typed(ptr) if volatile => {
                let target = types.get_qualified_type(ptr.get_ptr_to()?, crate::kuna_typequal::VOLATILE).ok()?;
                Pointee::Typed(types.get_type_pointer(ptr_size, target, 1).ok()?)
            }
            other => other,
        };
        let bytes_array = container.is_some_and(|c| {
            c.get_metatype() == type_metatype::TYPE_ARRAY && c.get_array_base().is_some_and(|e| e.get_size() == 1)
        });
        let form = if bytes_array && off == 0 {
            Form::Decay
        } else if bytes_array {
            Form::Element(off)
        } else if off == 0 {
            Form::Base
        } else if off % size as int8 == 0 {
            Form::Index(off / size as int8)
        } else {
            Form::Bytes(off)
        };
        let bytes = matches!(form, Form::Bytes(_)).then_some(char_ptr);
        Some(Piece::Pointer { form, pointee, bytes })
    }

    /// A 3-, 5-, 6- or 7-byte read out of a scalar as the `SUB` intrinsic, its
    /// offset counted from the value's low end: the memory offset on a
    /// little-endian space, the bytes after the access on a big-endian one.
    fn sub(fd: &Funcdata, vn: VarnodeId, op: OpId, off: int8, size: int4, container: Option<&Datatype>) -> Option<Piece> {
        let v = fd.vbank().get(vn)?;
        let c = container?;
        let scalar = matches!(
            c.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_PTR
        );
        if v.get_def() == Some(op) || !matches!(size, 3 | 5 | 6 | 7) || !scalar || c.get_size() > 8 {
            return None;
        }
        let big = v.get_addr().get_space().is_some_and(|s| s.is_big_endian());
        let value_offset = if big { c.get_size() as int8 - off - size as int8 } else { off };
        (value_offset >= 0).then(|| Piece::Sub { name: format!("SUB{}{}", c.get_size(), size), value_offset })
    }
}
