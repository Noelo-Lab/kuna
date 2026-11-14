//! (kuna) A user op a processor spec names for a rounding to an integral value
//! that p-code has no operator for takes one float and returns a float of the
//! same width (P5).
//!
//! p-code rounds a float to an integral float only toward minus infinity
//! (`FLOAT_FLOOR`), toward plus infinity (`FLOAT_CEIL`) and to nearest with ties
//! away from zero (`FLOAT_ROUND`). A spec lifts the other modes to user ops named
//! after their C library functions: `FLOAT_ROUNDEVEN` (ties to even, `roundeven`),
//! and the current rounding mode with (`FLOAT_RINT`, `rint`) and without
//! (`FLOAT_NEARBYINT`, `nearbyint`) the inexact exception. A CALLOTHER carries no
//! type, so `double g(double x, double y) { return rint(x) * y; }` printed its
//! parameter as `unsigned long` and the product through a bit-reinterpreting
//! union. [`output_type`] and [`input_type`] give such an op's result and operand
//! the float type of their width, as `FLOAT_ROUND`'s are, wherever the processor
//! has a float format of that width.

use std::rc::Rc;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// The user-op names a spec gives the float-to-float roundings p-code lacks.
pub const USEROP_NAMES: [&[u8]; 3] = [b"FLOAT_ROUNDEVEN", b"FLOAT_RINT", b"FLOAT_NEARBYINT"];

/// The float type of a `size`-byte value at a rounding user op `op`.
fn float_of(data: &Funcdata, op: OpId, size: int4) -> Option<Rc<Datatype>> {
    let arch = data.get_arch();
    if arch.float_rounding_userops.is_empty() {
        return None;
    }
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_CALLOTHER || o.num_input() != 2 {
        return None;
    }
    let id = data.vbank().get(o.get_in(0)?)?.get_offset();
    if !arch.float_rounding_userops.iter().any(|&u| u64::from(u) == id) {
        return None;
    }
    arch.get_float_format(size)?;
    arch.types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()
}

/// The result type of a rounding user op: a float of its width.
pub(crate) fn output_type(data: &Funcdata, op: OpId) -> Option<Rc<Datatype>> {
    let o = data.obank().get(op)?;
    let size = data.vbank().get(o.get_out()?)?.get_size();
    float_of(data, op, size)
}

/// The operand type of a rounding user op: a float of its width.
pub(crate) fn input_type(data: &Funcdata, op: OpId, slot: int4) -> Option<Rc<Datatype>> {
    if slot != 1 {
        return None;
    }
    let o = data.obank().get(op)?;
    let size = data.vbank().get(o.get_in(1)?)?.get_size();
    float_of(data, op, size)
}
