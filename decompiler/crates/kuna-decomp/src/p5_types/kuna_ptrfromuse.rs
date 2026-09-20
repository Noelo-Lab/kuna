//! A function input used only as a memory base is a pointer (kuna `ptrfromuse`, P5).
//!
//! `ActionInferTypes` seeds every live Varnode from local op semantics
//! (`getLocalType`, `coreaction_infertypes.rs`) and then pushes those seeds along
//! the def-use graph.  Neither half can give a *parameter* a pointer type when the
//! function only ever dereferences it at a constant offset:
//!
//! ```text
//!   void sub_3420(long a0, unsigned int a1)      // coreutils fmt -O2, get_line
//!   {
//!     v10 = *(unsigned char **)(a0 + 8);
//!     if (*(unsigned char **)(a0 + 0x10) <= v10) { ... }
//!   }
//! ```
//!
//! The seed fold sees three `INT_ADD` readers, and `TypeOpIntAdd::getInputLocal`
//! votes `int8` for each of them, so `a0` is declared `long`.  The pointer
//! candidate does exist — `TypeOp::propagateToPointer` types the `a0 + 8` result
//! `undefined8 *` — but `TypeOpIntAdd::propagateType` refuses to carry a pointer
//! from an output back to an input (`typeop.cc:1217`, transcribed at
//! `coreaction_infertypes::propagate_int_add`), so it never reaches `a0`.
//!
//! [`pointer_from_use`] supplies that missing candidate directly, as one more vote
//! in the *same* fold rather than as a replacement: it walks the input's transitive
//! descendants through copy-like identity and constant-offset addition, and returns
//! a pointer when at least one terminal use is the address operand of a `LOAD` or
//! `STORE` and nothing on the way is inconsistent with a pointer.  Because it folds
//! by `Datatype::type_order`, a more specific vote still wins: a `FILE *` arriving
//! from a callee's locked prototype (`SUB_PTR_STRUCT`) outranks this rule's
//! `SUB_PTR` candidate and is kept.
//!
//! Only function inputs are considered.  Parameters are the metric's first,
//! name-independent match pass, and the blast radius is one declaration per
//! function instead of one per local.
//!
//! Gated by [`Architecture::ptr_from_use`](crate::architecture::Architecture)
//! (option `ptrfromuse off|byte|void`, shipped `void`); with the option off nothing
//! in this module is reachable.

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;


use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use crate::context::{OpId, VarnodeId};


/// How many def-use hops the walk follows before giving up.  RecStruct's
/// access-chain walk uses the same bound.
const HOP_CAP: usize = 10;

/// (kuna) What a use-derived pointer points at: `ptrfromuse off|byte|void`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PtrFromUseMode {
    /// Upstream: a dereferenced-only input keeps its integer vote.
    Off,
    /// Point at one unknown byte (`undefined1 *`, rendered `char *` by `realtypes`).
    Byte,
    /// Point at nothing (`void *`).  The shipped default.
    #[default]
    Void,
}

impl PtrFromUseMode {
    /// Is the rule active at all?
    pub fn is_on(self) -> bool {
        self != PtrFromUseMode::Off
    }
}

/// (kuna) Parse `option ptrfromuse off|byte|void`; the caller writes the live field.
pub struct OptionPtrFromUse;

impl OptionPtrFromUse {
    /// The option name.
    pub const NAME: &'static str = "ptrfromuse";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(PtrFromUseMode, String)> {
        let mode = match p1 {
            "off" => PtrFromUseMode::Off,
            "byte" => PtrFromUseMode::Byte,
            "void" => PtrFromUseMode::Void,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown ptrfromuse value: {other} (expected off|byte|void)"
                )))
            }
        };
        Ok((mode, format!("Use-derived parameter pointer typing set to {p1}")))
    }
}

/// What one reader of a walked Varnode tells us.
enum Use {
    /// The address operand of a LOAD or STORE: this value is a memory base.
    Base,
    /// Identity or constant-offset addition: keep walking from `vn`.
    Forward(VarnodeId),
    /// Nothing either way.
    Neutral,
    /// Inconsistent with a pointer: the whole candidate is refused.
    Refuse,
}

/// The candidate pointer data-type for `vn`, or `None` when the rule declines.
///
/// `vn` must be a function input Varnode.  Returns a pointer of `vn`'s own width
/// whose target is one unknown byte (`Byte`) or `void` (`Void`).
pub fn pointer_from_use(
    data: &Funcdata,
    vn: VarnodeId,
    mode: PtrFromUseMode,
) -> Option<Rc<Datatype>> {
    if !mode.is_on() {
        return None;
    }
    let v = data.vbank().get(vn)?;
    if !v.is_input() || v.is_type_lock() || v.is_annotation() {
        return None;
    }
    let ptrsize = v.get_size();
    // Not every function input is a parameter.  A segment-base register (the
    // x86-64 `FS_OFFSET` the stack canary is read through) and the stack pointer
    // are inputs too, and typing those as pointers moves code that has nothing to
    // do with the function's arguments -- on coreutils `ls` it stops `stackguard`
    // from recognising the canary, so the check is emitted instead of stripped.
    // Ask the prototype model whether this storage could carry a parameter.
    let vn_addr = v.get_addr().clone();
    if !data.get_func_proto().possible_input_param(&vn_addr, ptrsize) {
        return None;
    }
    let arch = Rc::clone(data.get_arch());
    let tlst = arch.types()?;
    // Only a pointer-width value can be a pointer.  `get_type_pointer` would mint a
    // 4-byte pointer on a 64-bit image otherwise.
    let spc = Rc::clone(arch.manage().get_default_data_space()?);
    if ptrsize != spc.get_addr_size() as int4 {
        return None;
    }

    if !walk_is_base(data, vn) {
        return None;
    }

    let word_size = spc.get_word_size();
    let ptrto = match mode {
        PtrFromUseMode::Byte => tlst.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?,
        PtrFromUseMode::Void => tlst.get_type_void().ok()?,
        PtrFromUseMode::Off => return None,
    };
    tlst.get_type_pointer(ptrsize, ptrto, word_size).ok()
}

/// Bounded breadth-first walk over the transitive descendants of `start`.  True
/// when at least one terminal use is a LOAD/STORE address and no use refuses.
/// Breadth first makes the hop cap a property of the graph -- everything within
/// [`HOP_CAP`] hops of the input is seen -- rather than of the traversal order.
fn walk_is_base(data: &Funcdata, start: VarnodeId) -> bool {
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    // Breadth first, so a Varnode is recorded at its SHORTEST hop distance from
    // the input.  With a depth-first order the cap would depend on which path
    // reached a shared Varnode first, and a refusing use just past the boundary
    // could be reachable on one traversal order and not on another.
    let mut work: VecDeque<(VarnodeId, usize)> = VecDeque::new();
    work.push_back((start, 0));
    seen.insert(start);
    let mut found_base = false;
    while let Some((vn, hops)) = work.pop_front() {
        let descend: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            match classify_use(data, op, vn) {
                Use::Refuse => return false,
                Use::Base => found_base = true,
                Use::Neutral => {}
                Use::Forward(next) => {
                    if hops + 1 < HOP_CAP && seen.insert(next) {
                        work.push_back((next, hops + 1));
                    }
                }
            }
        }
    }
    found_base
}

/// Classify what `op` does with `vn`.
fn classify_use(data: &Funcdata, op: OpId, vn: VarnodeId) -> Use {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return Use::Neutral,
    };
    let opcode = o.code();
    let slot = o.get_slot(vn);
    let out = o.get_out();
    match opcode {
        // The address operand of a dereference: this is the accept condition.
        // `LOAD(space, addr)` and `STORE(space, addr, value)` both carry it at 1.
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
            if slot == 1 {
                Use::Base
            } else {
                // The stored VALUE being this input says nothing about it.
                Use::Neutral
            }
        }
        // Copy-like identity: the same address travels on.
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => match out {
            Some(o) => Use::Forward(o),
            None => Use::Neutral,
        },
        // The pointer-arithmetic pool rewrites `INT_ADD(p, k)` into `PTRADD`/`PTRSUB`
        // once `p` is a pointer, so a rule that did not follow those would find no
        // base on the next propagation pass, withdraw the candidate, and let the
        // rewrite be undone -- an oscillation that runs the seven-pass settle
        // ceiling.  The base is slot 0 of both.
        OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => match (slot, out) {
            (0, Some(o)) => Use::Forward(o),
            _ => Use::Neutral,
        },
        // `p + k` with a literal k is still the same object -- unless the literal
        // is itself the address of a mapped global object, in which case the roles
        // are reversed: the constant is the base and the walked value is an index
        // into it (`table[i]` compiles to `INT_ADD(i, &table)`).  A variable addend
        // is ambiguous (either operand could be the base), so it neither carries
        // the walk nor refuses it.
        OpCode::CPUI_INT_ADD => {
            let other = if slot == 0 { 1 } else { 0 };
            let other_vn = match o.get_in(other) {
                Some(v) => v,
                None => return Use::Neutral,
            };
            let other_const =
                data.vbank().get(other_vn).map(|v| v.is_constant()).unwrap_or(false);
            if !other_const {
                return Use::Neutral;
            }
            match const_base_evidence(data, op, other_vn) {
                // A named global object: the constant IS the base, so `vn` is a
                // subscript and nothing else about it can make it a pointer.
                BaseEvidence::Object => Use::Refuse,
                // Address-like, but kuna knows of nothing there.  Ambiguous in
                // exactly the way a variable addend is, so say nothing: a
                // parameter with no other use stops being a candidate, while one
                // the function also dereferences at a field offset keeps it.
                BaseEvidence::Maybe => Use::Neutral,
                BaseEvidence::No => match out {
                    Some(o) => Use::Forward(o),
                    None => Use::Neutral,
                },
            }
        }
        // Arithmetic no pointer survives.
        OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_DIV
        | OpCode::CPUI_INT_SDIV
        | OpCode::CPUI_INT_REM
        | OpCode::CPUI_INT_SREM
        | OpCode::CPUI_INT_2COMP
        | OpCode::CPUI_INT_NEGATE
        | OpCode::CPUI_INT_LEFT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_PIECE => Use::Refuse,
        // A comparison against a non-zero literal is a value test, not a null test.
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL => {
            let other = if slot == 0 { 1 } else { 0 };
            let bad = o
                .get_in(other)
                .and_then(|v| data.vbank().get(v))
                .map(|v| v.is_constant() && v.get_offset() != 0)
                .unwrap_or(false);
            if bad {
                Use::Refuse
            } else {
                Use::Neutral
            }
        }
        // A callee whose prototype is locked has already said what this argument is;
        // a committed non-pointer there outranks use evidence.
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            if slot > 0 {
                let vote = crate::coreaction_infertypes::input_type_local(data, op, slot);
                let meta = vote.get_metatype();
                if meta != type_metatype::TYPE_UNKNOWN && meta != type_metatype::TYPE_PTR {
                    return Use::Refuse;
                }
            }
            Use::Neutral
        }
        _ => {
            if is_float_op(opcode) {
                Use::Refuse
            } else {
                Use::Neutral
            }
        }
    }
}

/// How much a constant addend looks like the base of the expression.
enum BaseEvidence {
    /// It resolves to a global object kuna knows about.
    Object,
    /// It is address-like but names nothing: could be a base, could be a large
    /// field offset.
    Maybe,
    /// An ordinary field offset.
    No,
}

/// Is the constant `cvn` the base of its `INT_ADD` rather than a field offset?
///
/// `table[i]` compiles to `INT_ADD(i, &table)`, the same shape as `p->field`,
/// and the addend is the only thing that tells them apart.  The test is
/// `ActionConstantPtr::isPointer`'s own (`coreaction.cc:1167`): the default data
/// space's pointer bounds, `resolveConstant` into that space, then the global
/// scope.  A symbol there settles it.  Address-like with no symbol does not,
/// and must not be read as one either way -- a stripped image's `.bss` table
/// carries no symbol at all, while a struct big enough to reach past the bound
/// (bzip2's 0x13f0-byte `bzFile`) is a real pointer reached at a real offset.
fn const_base_evidence(data: &Funcdata, op: OpId, cvn: VarnodeId) -> BaseEvidence {
    let glb = data.get_arch();
    let (off, size) = match data.vbank().get(cvn) {
        Some(v) => (v.get_offset(), v.get_size()),
        None => return BaseEvidence::No,
    };
    let spc = match glb.manage().get_default_data_space() {
        Some(s) => Rc::clone(s),
        None => return BaseEvidence::No,
    };
    if spc.get_pointer_lower_bound() > off || spc.get_pointer_upper_bound() < off {
        return BaseEvidence::No;
    }
    let point = match data.obank().get(op) {
        Some(o) => o.get_addr().clone(),
        None => return BaseEvidence::No,
    };
    let mut full_encoding: uintb = 0;
    let rampoint = match glb.resolve_constant(&spc, off, size, &point, &mut full_encoding) {
        Ok(r) => r,
        Err(_) => return BaseEvidence::No,
    };
    if rampoint.is_invalid() {
        return BaseEvidence::No;
    }
    let invalid = Address::new_invalid();
    match glb.query_container_global(&rampoint, 1, &invalid) {
        Some(_) => BaseEvidence::Object,
        None => BaseEvidence::Maybe,
    }
}

/// Any floating-point opcode: a pointer is never one of these operands.
fn is_float_op(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN
            | OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_INT2FLOAT
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_TRUNC
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
    )
}

/// Is `cand` more specific than `cur` in the `getLocalType` fold?  The seed is one
/// more vote, never a replacement — a `FILE *` (`SUB_PTR_STRUCT`) keeps its place.
pub fn folds_over(cand: &Rc<Datatype>, cur: &Rc<Datatype>) -> bool {
    0 > cand.type_order(cur).unwrap_or(0)
}

#[cfg(test)]
mod tests;
