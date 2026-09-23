//! (kuna `protoorder`) What a callee's own recovery found, stated for its callers.
//!
//! `kuna decompile-all` decompiles callees before callers (the ordering lives in
//! `kuna-cli/src/decompile_all.rs`); after each function this module decides what
//! may be said about it, and at each later call site what a caller may take.
//!
//! `types` records the recovered parameter types against the storage they were
//! recovered in ([`RecoveredTypes`]).  A caller's argument takes one as a vote in
//! `Varnode::getLocalType`'s fold ([`call_argument_vote`]), refused wherever the
//! caller holds evidence the fold cannot weigh.  Nothing is locked, so no call's
//! arity can move.  `cycles` (the default) is `types` for the members of a
//! call-graph cycle too, which the driver decompiles in an order of its own.
//!
//! `lock` parks the recovered prototype on the callee's `FunctionSymbol`, where
//! `ActionDefaultParams` reads a declared one from ([`park_recovered`]).  That
//! input-locks every call to it, so the decline rules below guard against a list
//! that is not evidence of the same quality as what it overwrites.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::p4_calls::fspec::{FuncCallSpecs, FuncProto, PrototypePieces};

/// What a recovered prototype is allowed to say about a call site.
///
/// [`ProtoOrderMode::Cycles`] is [`ProtoOrderMode::Types`] with the members of
/// a recursive component stating their types as well.  `types` and `lock`
/// differ in ONE thing -- whether the parked prototype can move a
/// call's ARITY -- and that one thing is the whole difference between a type
/// recovery and a rewrite of what the program does.
///
/// [`ProtoOrderMode::Types`] states only the types: the recovered parameter
/// types are matched, BY STORAGE, against the argument list the caller's own
/// recovery already built, and a matching slot takes the callee's type as one
/// more vote in `Varnode::getLocalType`'s existing fold.  Nothing is locked,
/// nothing is added and nothing is removed, so the call renders with exactly the
/// arguments it renders with when the option is off.
///
/// [`ProtoOrderMode::Lock`] parks the recovered prototype in the symbol table,
/// where `ActionDefaultParams` reads a DECLARED one from.  That does move
/// arity -- which is the point where the caller over-recovered and the defect
/// where the callee under-recovered -- and it is measured: over twenty x86-64
/// userland binaries it gains 818 call arguments and loses 28, but 155 of 7,553
/// functions gain a parameter row of their own, and on two ARM Cortex-M images
/// (four argument registers, all of them busy) 630 of 6,589 do.  A fabricated
/// parameter is invisible to `type_match`, so that cost does not show up in the
/// metric at all; the mode is kept for an operator who wants the arity claim on
/// a program where it holds, and it is never the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtoOrderMode {
    /// Decompile in the caller's order and state nothing.
    #[default]
    Off,
    /// Callees first; state the recovered TYPES, never the arity.
    Types,
    /// Callees first; park the recovered prototype as a locked one.
    Lock,
    /// [`ProtoOrderMode::Types`], and a function in a call-graph cycle states
    /// its recovered types too, in the order the driver gives the cycle.
    Cycles,
}

impl ProtoOrderMode {
    /// The option token.
    pub fn as_str(self) -> &'static str {
        match self {
            ProtoOrderMode::Off => "off",
            ProtoOrderMode::Types => "types",
            ProtoOrderMode::Lock => "lock",
            ProtoOrderMode::Cycles => "cycles",
        }
    }

    /// Is a callee-first run wanted at all?
    pub fn is_on(self) -> bool {
        !matches!(self, ProtoOrderMode::Off)
    }

    /// Does this mode state types only, never the arity?
    pub fn states_types_only(self) -> bool {
        matches!(self, ProtoOrderMode::Types | ProtoOrderMode::Cycles)
    }

    /// May a member of a call-graph cycle state its recovered types?
    pub fn states_in_cycles(self) -> bool {
        self == ProtoOrderMode::Cycles
    }

    /// The mode for a `u8` live value (the console's live reader).
    pub fn from_u8(v: u8) -> ProtoOrderMode {
        match v {
            1 => ProtoOrderMode::Types,
            2 => ProtoOrderMode::Lock,
            3 => ProtoOrderMode::Cycles,
            _ => ProtoOrderMode::Off,
        }
    }

    /// The `u8` the seam carries.
    pub fn as_u8(self) -> u8 {
        match self {
            ProtoOrderMode::Off => 0,
            ProtoOrderMode::Types => 1,
            ProtoOrderMode::Lock => 2,
            ProtoOrderMode::Cycles => 3,
        }
    }
}

/// (kuna) Parse `option protoorder off|types|cycles|lock`.
pub struct OptionProtoOrder;

impl OptionProtoOrder {
    /// The option name.
    pub const NAME: &'static str = "protoorder";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(ProtoOrderMode, String)> {
        let mode = match p1 {
            "off" => ProtoOrderMode::Off,
            "types" => ProtoOrderMode::Types,
            "cycles" => ProtoOrderMode::Cycles,
            "lock" => ProtoOrderMode::Lock,
            other => {
                return Err(kuna_base::error::KunaError::parse(format!(
                    "Unknown protoorder value: {other} (expected off|types|cycles|lock)"
                )))
            }
        };
        Ok((mode, format!("Callee-first recovered-prototype ordering set to {}", mode.as_str())))
    }
}

/// A callee's recovered parameter types, against the storage they were
/// recovered in: the whole of what [`ProtoOrderMode::Types`] states.
#[derive(Debug, Clone)]
pub struct RecoveredTypes {
    /// One entry per recovered parameter, in the callee's own order: the storage
    /// recovery put it in, and the type recovery gave it.
    pub inputs: Vec<(Address, int4, Rc<Datatype>)>,
    /// Would [`ProtoOrderMode::Lock`]'s arity policy accept this list -- it does
    /// not end on the register-file boundary a variadic's register-save prologue
    /// leaves, and it is where the convention would put those types?  Computed
    /// only when `option passthrough` is on, the one reader that moves arity on
    /// a stated list ([`crate::p4_calls::kuna_passthrough`]); `false` otherwise.
    pub arity_sound: bool,
    /// The recovered return value -- storage, size and type -- when it is not
    /// `void`.  Recorded only when `option passthrough` is on, whose tail-call
    /// arm is its one reader; `None` otherwise.
    pub output: Option<(Address, int4, Rc<Datatype>)>,
    /// The storage of every parameter in [`Self::inputs`] the callee's body only
    /// ever feeds to a variadic tail
    /// ([`crate::p4_calls::kuna_varargtail`]): read by the callee, but only
    /// where the ABI lets a caller leave the register unset, so
    /// [`crate::p4_calls::kuna_passthrough`] never claims it.  Recorded only when
    /// `option passthrough` is on; empty otherwise.
    pub vararg_tail: Vec<Address>,
}

impl RecoveredTypes {
    /// The callee's recovered type for parameter `index`, but only when that
    /// parameter is in the same storage the CALLER's own recovery put its
    /// argument `index` in.
    ///
    /// Position and storage must agree.  Position alone would re-bind every
    /// later type whenever the two recoveries disagree about how many arguments
    /// there are; storage alone would let a register the conventions reuse carry
    /// a type across a slot boundary.  Together they say "we are talking about
    /// the same argument", which is the only claim a type vote may rest on.
    pub fn param_type_at(&self, index: usize, addr: &Address, size: int4) -> Option<&Rc<Datatype>> {
        let (paddr, psize, ct) = self.inputs.get(index)?;
        if psize != &size || paddr != addr {
            return None;
        }
        Some(ct)
    }
}

/// The callee's recovered type for argument `slot` of the call `op`, offered to
/// `Varnode::getLocalType`'s fold, or `None` when the vote is refused.
///
/// A vote is refused where the caller already holds better evidence about the
/// value than a recovery of the callee: the value is the address of a frame
/// object (a pointer vote there becomes a stack-layout hint and splits the
/// aggregate), its family is declared, global, frame memory or loaded through
/// itself, another reader or writer of the same value -- a declared parameter
/// or return, or another stated parameter -- puts it in a different class, the
/// value or its storage disagrees with the vote about being a float (a float
/// vote on a value any integer op computes with, truncates or takes bytes out
/// of, or that is stored, returned, passed on, produced by a call without a
/// float type, a NaN constant or a parameter in a register the convention never
/// gives a float, is refused), the vote is deeper than the inferred pointer cap,
/// it would make a pointer of a constant inside a function's code, or what the
/// caller does through a pointer disagrees with the pointee
/// ([`pointee_refuses`]).
pub fn call_argument_vote(
    data: &Funcdata,
    op: OpId,
    fc: &FuncCallSpecs,
    slot: int4,
    arg_size: int4,
) -> Option<Rc<Datatype>> {
    let ct = stated_argument_type(data, fc, slot)?;
    if class_of(ct).is_none()
        || ct.get_size() > arg_size
        || crate::p5_types::kuna_ptrdepth::pointer_depth(ct, 3) > crate::p5_types::kuna_ptrdepth::MAX_INFERRED_PTR_DEPTH
    {
        return None;
    }
    let vn = data.obank().get(op)?.get_in(slot)?;
    let (addr, size) = fc.final_input_storage().get((slot - 1) as usize)?;
    if addresses_a_frame_object(data, vn)
        || storage_disagrees(fc.proto(), addr, *size, ct)
        || (class_of(ct) == Some(Class::Pointer) && points_into_code(data, op, vn))
        || (class_of(ct) == Some(Class::Float) && !prints_exactly_as_a_float(data, vn))
        || family_refuses(data, vn, Reading::Argument(op, slot), ct)
        || crate::kuna_structheadless::yields_to_a_declared_record(data, &value_family(data, vn), ct)
    {
        return None;
    }
    Some(Rc::clone(ct))
}

/// Does `vn`, if it is a constant, print as a float literal that compiles back
/// to the same bits?  Every NaN prints as `NAN`, whatever its payload.
fn prints_exactly_as_a_float(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(vn).filter(|n| n.is_constant()) else { return true };
    if node.get_size() > 8 {
        return false;
    }
    data.get_arch().get_float_format(node.get_size()).is_some_and(|f| {
        f.get_host_float(node.get_offset() as u64).1 != kuna_num::float::floatclass::nan
    })
}

/// Is `vn` a constant inside a function's code?  A data pointer there renders
/// as a subscript of the function (`&sub_80014f4[1]` for the Thumb address
/// `0x80014f5`), which is not C.
fn points_into_code(data: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(vn).filter(|n| n.is_constant()) else { return false };
    let Some(at) = data.obank().get(op).map(|o| o.get_addr().clone()) else { return false };
    let glb = data.get_arch();
    let (value, size) = (node.get_offset(), node.get_size());
    glb.infer_ptr_spaces().iter().filter(|spc| spc.get_addr_size() as int4 == size).any(|spc| {
        let mut full = 0;
        glb.resolve_constant(spc, value, size, &at, &mut full).ok().is_some_and(|ram| {
            glb.query_container_global(&ram, 1, &Address::new_invalid()).is_some_and(|e| {
                e.symbol_type.as_ref().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_CODE)
            })
        })
    })
}

fn stated_argument_type<'a>(data: &'a Funcdata, fc: &FuncCallSpecs, slot: int4) -> Option<&'a Rc<Datatype>> {
    let stated = data.kuna_protoorder_types(fc.get_entry_address())?;
    let index = (slot - 1) as usize;
    let (addr, size) = fc.final_input_storage().get(index)?;
    stated.param_type_at(index, addr, *size)
}

/// Is `vn` computed from the stack pointer -- the address of something in the
/// frame?  `gatherOpen` turns a pointer's pointee at a frame address into a
/// range hint, so a vote there re-lays the frame rather than typing a value.
pub(crate) fn addresses_a_frame_object(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut work = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some(v) = work.pop() {
        if seen.contains(&v) || seen.len() >= 32 {
            continue;
        }
        seen.push(v);
        let Some(node) = data.vbank().get(v) else { continue };
        if node.is_spacebase() && !node.is_constant() {
            return true;
        }
        let Some(o) = node.get_def().and_then(|d| data.obank().get(d)) else { continue };
        let inputs: &[int4] = match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD
            | OpCode::CPUI_INT_SUB => &[0],
            OpCode::CPUI_INT_ADD => &[0, 1],
            OpCode::CPUI_MULTIEQUAL => &[0, 1, 2, 3],
            _ => &[],
        };
        work.extend(inputs.iter().filter_map(|&i| o.get_in(i)));
    }
    false
}

/// Is `node` frame memory a vote would re-lay?  Either the frame already holds a
/// struct or array symbol over it, or it starts at a frame address the function
/// takes, or it lies in an indexed region: at or above a frame address the
/// function indexes with a non-constant, below the next address it takes.  Each
/// such address opens a `gatherOpen` range, which a typed element cuts short or
/// merges its neighbour into.
fn inside_a_frame_aggregate(
    data: &Funcdata,
    node: &crate::varnode::Varnode,
    regions: &mut Option<Vec<(i64, Option<i64>)>>,
) -> bool {
    let Some(scope) = data.get_scope_local() else { return false };
    let space = scope.get_space_id();
    if node.get_space().get_index() != space.get_index() {
        return false;
    }
    let db = scope.database();
    if let Some(eref) = db.find_overlap(scope.scope_id(), node.get_addr(), node.get_size()) {
        let entry = db.entry(scope.scope_id(), eref);
        if entry.size > node.get_size()
            && db.symbol(entry.symbol).dtype.as_ref().is_some_and(|t| {
                matches!(t.get_metatype(), type_metatype::TYPE_STRUCT | type_metatype::TYPE_ARRAY)
            })
        {
            return true;
        }
    }
    let shift = 64 - 8 * space.get_addr_size().clamp(1, 8) as u32;
    let signed = |x: kuna_base::types::uintb| ((x << shift) as i64) >> shift;
    let regions = regions.get_or_insert_with(|| {
        let Some(sp) = data.find_spacebase_input(space) else { return Vec::new() };
        let mut bases = Vec::new();
        data.gather_additive_base(sp, &mut bases, false);
        let mut offs: Vec<(i64, bool)> =
            bases.iter().map(|b| (signed(data.gather_offset(b.base)), b.index.is_some())).collect();
        offs.sort_unstable();
        offs.iter()
            .map(|&(c, indexed)| {
                let next = offs.iter().map(|o| o.0).find(|&n| n > c);
                (c, if indexed { next } else { Some(c + 1) })
            })
            .collect()
    });
    let o = signed(node.get_offset());
    regions.iter().any(|&(c, end)| o >= c && end.is_none_or(|n| o < n))
}

/// Is the LOAD `o` reached through an address computed from the value it
/// produces (`p = *p`, `p = p->next`)?  That is `T == ptr(T)`, which no finite
/// type satisfies: a pointer vote seeds it and every inference pass deepens it.
fn loaded_through_itself(
    data: &Funcdata,
    o: &crate::op::PcodeOp,
    members: &std::collections::HashSet<VarnodeId>,
) -> bool {
    let mut at = o.get_in(1);
    for _ in 0..8 {
        let Some(v) = at else { return false };
        if members.contains(&v) {
            return true;
        }
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else {
            return false;
        };
        at = match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB => def.get_in(0),
            _ => return false,
        };
    }
    false
}

/// Does the LOAD or STORE `o` reach memory through a frame address?  The value
/// it moves is then frame memory, and typing it types the frame.
fn through_the_frame(data: &Funcdata, o: &crate::op::PcodeOp) -> bool {
    o.get_in(1).is_some_and(|p| addresses_a_frame_object(data, p))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Class {
    Pointer,
    Integer,
    Float,
}

fn class_of(ct: &Datatype) -> Option<Class> {
    match ct.get_metatype() {
        type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => Some(Class::Pointer),
        type_metatype::TYPE_FLOAT => Some(Class::Float),
        type_metatype::TYPE_INT
        | type_metatype::TYPE_UINT
        | type_metatype::TYPE_BOOL
        | type_metatype::TYPE_ENUM_INT
        | type_metatype::TYPE_ENUM_UINT => Some(Class::Integer),
        _ => None,
    }
}

/// A float vote in a general register is ordinary (soft-float ABIs); an integer
/// or pointer vote in a float register is a bit reinterpretation.
fn storage_disagrees(proto: &FuncProto, addr: &Address, size: int4, ct: &Datatype) -> bool {
    if ct.get_metatype() == type_metatype::TYPE_FLOAT || !proto.has_model() {
        return false;
    }
    let Some(list) = proto.model().input_opt() else { return false };
    list.find_entry(addr, size, true)
        .is_some_and(|i| list.get_entry()[i].get_type() == crate::dtype::type_class::TYPECLASS_FLOAT)
}

/// The value `vn` carries: every varnode joined to it by COPY, CAST, MULTIEQUAL
/// or INDIRECT, in either direction -- the edges type propagation crosses, so
/// the varnodes a vote on `vn` would end up typing.  At -O0 one parameter is
/// reloaded for every call it is passed to and merged through its stack slot's
/// phi-nodes, so the readers that speak about the same value are spread over
/// many varnodes.
pub(crate) fn value_family(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let joins = |o: &crate::op::PcodeOp| {
        matches!(
            o.code(),
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT
        )
    };
    let mut family = vec![vn];
    let mut seen: std::collections::HashSet<VarnodeId> = std::collections::HashSet::from([vn]);
    let mut at = 0;
    while at < family.len() && family.len() < 128 {
        let Some(node) = data.vbank().get(family[at]) else { break };
        at += 1;
        let mut next: Vec<VarnodeId> = Vec::new();
        if let Some(o) = node.get_def().and_then(|d| data.obank().get(d)).filter(|o| joins(o)) {
            let n = if o.code() == OpCode::CPUI_INDIRECT { 1 } else { o.num_input() };
            next.extend((0..n).filter_map(|i| o.get_in(i)));
        }
        for r in node.descend_iter() {
            if let Some(out) = data.obank().get(r).filter(|o| joins(o)).and_then(|o| o.get_out()) {
                next.push(out);
            }
        }
        for v in next {
            if data.vbank().get(v).is_none_or(|n| n.is_constant()) || !seen.insert(v) {
                continue;
            }
            family.push(v);
        }
    }
    family
}

/// Where a type is being asked of a value: a call argument's own vote, or a
/// float member of a pointer vote's pointee, reached by the loads and stores in
/// the set.
#[derive(Clone, Copy)]
enum Reading<'a> {
    Argument(OpId, int4),
    Member(&'a std::collections::HashSet<OpId>),
    Input,
}

/// (kuna `calleevote`) Does anything the function does with its input `vn`
/// outrank a caller's type `ct` for it?  The refusals a callee's vote meets at
/// a call site, asked of the callee's own uses, and one more for a character
/// pointer ([`splits_a_wide_constant`]).
pub(crate) fn input_refuses(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> bool {
    class_of(ct).is_none()
        || family_refuses(data, vn, Reading::Input, ct)
        || indexes_a_pointer(data, vn)
        || splits_a_wide_constant(data, &value_family(data, vn), ct, 0)
}

/// Is the value `vn` the index of a pointer addition (`((char *)0x1018)[a0]`,
/// a constant the recovery took for the pointer)?  The function then adds it
/// to an address as a number, and a pointer type for it prints as a cast at
/// every such use (`[(long)a0]`).
fn indexes_a_pointer(data: &Funcdata, vn: VarnodeId) -> bool {
    value_family(data, vn).into_iter().any(|v| {
        data.vbank().get(v).is_some_and(|node| {
            node.descend_iter().any(|r| {
                data.obank().get(r).is_some_and(|o| o.code() == OpCode::CPUI_PTRADD && o.get_in(1) == Some(v))
            })
        })
    })
}

/// (kuna `calleevote`) Is the value `vn` handed to a call whose declared
/// prototype types that parameter a pointer to something (`pipe (int *)`)?
pub(crate) fn handed_to_a_declared_pointer(data: &Funcdata, vn: VarnodeId) -> bool {
    value_family(data, vn).into_iter().any(|v| {
        let Some(node) = data.vbank().get(v) else { return false };
        node.descend_iter().any(|r| {
            let Some(o) = data.obank().get(r) else { return false };
            if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                return false;
            }
            let Some(fc) = data.get_call_specs_index(r).map(|i| data.get_call_specs(i)) else { return false };
            (1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).any(|s| {
                fc.proto().get_param(s - 1).is_some_and(|p| {
                    p.is_type_locked()
                        && p.get_type().and_then(|t| t.get_ptr_to()).is_some_and(|pt| {
                            !matches!(pt.get_metatype(), type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN)
                        })
                })
            })
        })
    })
}

/// (kuna `calleevote`) Does the function load or store through its input `vn`
/// anywhere but the one pointee of `ct` at offset zero?
pub(crate) fn reaches_past_the_pointee(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> bool {
    let Some(pointee) = ct.get_ptr_to() else { return false };
    let Some((accesses, _)) = accesses_through(data, &value_family(data, vn)) else { return true };
    accesses.iter().any(|a| a.stride != 0 || a.at < 0 || a.at + a.size as i64 > pointee.get_size() as i64)
}

/// Would `ct` on the values in `family` make the printer spell a constant
/// stored through a `char *` wider than a byte as one character store per
/// byte?  For a `char *` the stores are the ones through the value itself; for
/// a pointer to a pointer, the ones through every pointer-width value loaded
/// through it or stored through it, which take the pointee.
fn splits_a_wide_constant(data: &Funcdata, family: &[VarnodeId], ct: &Datatype, depth: u32) -> bool {
    let Some(pointee) = ct.get_ptr_to() else { return false };
    let character = pointee.get_size() == 1 && pointee.is_char_print();
    if !character && (pointee.get_metatype() != type_metatype::TYPE_PTR || depth >= 2) {
        return false;
    }
    let family = with_sibling_loads(data, family);
    let Some((accesses, _)) = accesses_through(data, &family) else { return true };
    if character {
        return accesses
            .iter()
            .any(|a| a.store && a.size > 1 && data.vbank().get(a.value).is_some_and(|n| n.is_constant()));
    }
    accesses.iter().filter(|a| a.size == pointee.get_size()).any(|a| {
        data.vbank().get(a.value).is_some_and(|n| !n.is_constant())
            && splits_a_wide_constant(data, &value_family(data, a.value), &pointee, depth + 1)
    })
}

/// Does anything else about the value `vn` outrank the vote `ct`?
///
/// A float vote also needs every other reader and writer of the value to be a
/// float or to pass the bits through untouched.  A store, and a return, another
/// call or a call's result outside a float register, each hand the value to a
/// type decided elsewhere, and the printer bridges a float to an integer there
/// with a value conversion: `a2[1] = (int)v1` for a float stored through an
/// `int *`.  So does a parameter of the caller's own in a register the
/// convention never passes a float in, and, for a member, a load through
/// another pointer that nothing reads as a float.
fn family_refuses(data: &Funcdata, vn: VarnodeId, reading: Reading, ct: &Datatype) -> bool {
    let class = class_of(ct);
    let mut float_seen = false;
    let mut integer_seen = false;
    let mut bits_seen = false;
    let mut handed_on = false;
    let mut loaded_elsewhere = false;
    let float_vote = class == Some(Class::Float);
    let own = |op: OpId| matches!(reading, Reading::Member(set) if set.contains(&op));
    let mut regions: Option<Vec<(i64, Option<i64>)>> = None;
    let family = value_family(data, vn);
    let members: std::collections::HashSet<VarnodeId> = family.iter().copied().collect();
    for &v in &family {
        let Some(node) = data.vbank().get(v) else { continue };
        if node.is_type_lock()
            || node.is_persist()
            || inside_a_frame_aggregate(data, node, &mut regions)
            || (v != vn && addresses_a_frame_object(data, v))
        {
            return true;
        }
        if float_vote && node.is_input() && !a_float_can_arrive_in(data, node) {
            return true;
        }
        if let Some(def) = node.get_def() {
            let o = data.obank().get(def);
            if o.is_some_and(|o| {
                o.code() == OpCode::CPUI_LOAD && (through_the_frame(data, o) || loaded_through_itself(data, o, &members))
            }) {
                return true;
            }
            match o.map(|o| o.code()) {
                Some(OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) => match writer_class(data, def) {
                    Some(c) if Some(c) != class => return true,
                    Some(_) => {}
                    None if float_vote => handed_on |= !in_a_float_register(call_model(data, def, false), node),
                    None => {}
                },
                Some(OpCode::CPUI_LOAD) if matches!(reading, Reading::Member(_)) && !own(def) => {
                    loaded_elsewhere = true;
                }
                Some(code) => match produced_class(code) {
                    Some(Class::Float) => float_seen = true,
                    Some(Class::Integer) => integer_seen = true,
                    _ => bits_seen |= integer_but_pointer_neutral(code),
                },
                None => {}
            }
        }
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            match o.code() {
                OpCode::CPUI_STORE if o.get_in(2) == Some(v) => {
                    if through_the_frame(data, o) {
                        return true;
                    }
                    handed_on |= !own(r);
                }
                OpCode::CPUI_RETURN if float_vote => handed_on |= returned_as_other_than_float(data, o, v, node),
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    for s in 1..o.num_input() {
                        if o.get_in(s) != Some(v) || matches!(reading, Reading::Argument(c, k) if (c, k) == (r, s)) {
                            continue;
                        }
                        match reader_class(data, r, s) {
                            Some(c) if Some(c) != class => return true,
                            Some(_) => {}
                            None if float_vote => {
                                handed_on |= !in_a_float_register(call_model(data, r, true), node)
                            }
                            None => {}
                        }
                    }
                }
                code => match read_class(code) {
                    Some(Class::Float) => float_seen = true,
                    Some(Class::Integer) => integer_seen = true,
                    _ => bits_seen |= integer_but_pointer_neutral(code),
                },
            }
        }
    }
    match class {
        Some(Class::Float) => integer_seen || bits_seen || handed_on || (loaded_elsewhere && !float_seen),
        Some(Class::Pointer) => float_seen || integer_seen || pointee_refuses(data, &family, ct, 0),
        _ => float_seen,
    }
}

/// Could the calling convention pass a float in the register `node`, the caller's
/// own input?  A soft-float convention (no float-class entry at all) always can.
/// Otherwise the model is asked where a float would go at `node`'s position among
/// the general registers: `rsi` gets an integer on SysV (a float at that position
/// goes to `xmm0`), while MIPS o32 puts a float after an integer in `a1`.
fn a_float_can_arrive_in(data: &Funcdata, node: &crate::varnode::Varnode) -> bool {
    if !crate::kuna_calleearitybody::is_register(node.get_addr()) {
        return true;
    }
    let proto = data.get_func_proto();
    let Some(list) = proto.has_model().then(|| proto.model().input_opt()).flatten() else { return true };
    let float_class = |e: &crate::p4_calls::fspec::ParamEntry| e.get_type() == crate::dtype::type_class::TYPECLASS_FLOAT;
    if !list.get_entry().iter().any(float_class) {
        return true;
    }
    let Some(at) = list.find_entry(node.get_addr(), node.get_size(), true) else { return false };
    if float_class(&list.get_entry()[at]) {
        return true;
    }
    let before = list.get_entry()[..at].iter().filter(|e| !float_class(e)).count();
    let arch = data.get_arch();
    let (Some(types), Some(dfp)) = (arch.types(), arch.default_fp()) else { return true };
    let (Ok(void_ty), Ok(word), Ok(float)) = (
        types.get_type_void(),
        types.get_base(types.get_size_of_pointer(), type_metatype::TYPE_UINT),
        types.get_base(node.get_size(), type_metatype::TYPE_FLOAT),
    ) else {
        return false;
    };
    let mut intypes = vec![word; before];
    intypes.push(float);
    let pieces = PrototypePieces {
        name: String::new(),
        outtype: None,
        innames: (0..intypes.len()).map(|i| format!("a{i}")).collect(),
        first_var_arg_slot: -1,
        intypes,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut fp = FuncProto::new();
    if fp.seed_locked_from_pieces(&pieces, Rc::clone(dfp), void_ty, types, arch.manage()).is_err() {
        return true;
    }
    fp.get_param(before as int4).is_some_and(|p| {
        let a = p.get_address();
        a.get_space().map(|s| s.get_index()) == node.get_addr().get_space().map(|s| s.get_index())
            && list.find_entry(&a, p.get_size(), true) == Some(at)
    })
}

/// `family` plus every other load of the memory a member of it was loaded from:
/// the same base and constant offset, at the same width.  A type the vote gives
/// one of those loads reaches the field it was loaded from, and through it every
/// other load of that field (`v2 = a0->field_0x0` elsewhere in the function).
pub(crate) fn with_sibling_loads(data: &Funcdata, family: &[VarnodeId]) -> Vec<VarnodeId> {
    let mut out = family.to_vec();
    let place = |mut v: VarnodeId| -> Option<(VarnodeId, uintb)> {
        let mut off: uintb = 0;
        for _ in 0..4 {
            let def = data.vbank().get(v)?.get_def().and_then(|d| data.obank().get(d));
            match def {
                Some(o) if matches!(o.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB) => {
                    let k = o.get_in(1).and_then(|c| data.vbank().get(c)).filter(|n| n.is_constant())?;
                    off = off.wrapping_add(k.get_offset());
                    v = o.get_in(0)?;
                }
                Some(o) if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) => v = o.get_in(0)?,
                _ => return Some((v, off)),
            }
        }
        Some((v, off))
    };
    for &m in family {
        let Some(node) = data.vbank().get(m) else { continue };
        let Some(o) = node.get_def().and_then(|d| data.obank().get(d)).filter(|o| o.code() == OpCode::CPUI_LOAD) else {
            continue;
        };
        if through_the_frame(data, o) {
            continue;
        }
        let Some((base, off)) = o.get_in(1).and_then(|a| place(a)) else { continue };
        let Some(bnode) = data.vbank().get(base) else { continue };
        let mut addrs: Vec<VarnodeId> = if off == 0 { vec![base] } else { Vec::new() };
        for r in bnode.descend_iter() {
            let Some(u) = data.obank().get(r) else { continue };
            if matches!(u.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB)
                && u.get_in(0) == Some(base)
                && u.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == off)
            {
                addrs.extend(u.get_out());
            }
        }
        for a in addrs {
            let Some(anode) = data.vbank().get(a) else { continue };
            for r in anode.descend_iter() {
                let Some(l) = data.obank().get(r).filter(|l| l.code() == OpCode::CPUI_LOAD && l.get_in(1) == Some(a))
                else {
                    continue;
                };
                let Some(lo) = l.get_out() else { continue };
                if out.contains(&lo) || data.vbank().get(lo).is_none_or(|n| n.get_size() != node.get_size()) {
                    continue;
                }
                for v in value_family(data, lo) {
                    if !out.contains(&v) {
                        out.push(v);
                    }
                }
                if out.len() > 256 {
                    return out;
                }
            }
        }
    }
    out
}

/// A load or store through a pointer, at `at` bytes from where the vote's value
/// points (plus any multiple of `stride` when the pointer is indexed or stepped).
pub(crate) struct Access {
    pub(crate) op: OpId,
    pub(crate) value: VarnodeId,
    pub(crate) at: i64,
    pub(crate) stride: i64,
    pub(crate) size: int4,
    pub(crate) store: bool,
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a as i64
}

/// Every load and store the caller makes through the pointer `family` holds,
/// following constant offsets, indexing and copies, and where every address it
/// derives points.  `None` when there are too many to follow.
pub(crate) fn accesses_through(data: &Funcdata, family: &[VarnodeId]) -> Option<(Vec<Access>, Vec<(i64, i64)>)> {
    let mut place: std::collections::HashMap<VarnodeId, (i64, i64)> =
        family.iter().map(|&v| (v, (0, 0))).collect();
    let mut work: Vec<VarnodeId> = family.to_vec();
    let mut budget = 512;
    let constant = |v: Option<VarnodeId>| {
        v.and_then(|v| data.vbank().get(v)).filter(|n| n.is_constant()).map(|n| {
            let shift = 64 - 8 * n.get_size().clamp(1, 8) as u32;
            ((n.get_offset() << shift) as i64) >> shift
        })
    };
    let step_of = |v: Option<VarnodeId>| {
        let def = v.and_then(|v| data.vbank().get(v)).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d));
        match def.map(|o| (o.code(), constant(o.get_in(1)))) {
            Some((OpCode::CPUI_INT_MULT, Some(k))) => k,
            Some((OpCode::CPUI_INT_LEFT, Some(k))) if (0..32).contains(&k) => 1 << k,
            _ => 1,
        }
    };
    let mut pointers: Vec<(OpId, VarnodeId, VarnodeId, bool)> = Vec::new();
    while let Some(v) = work.pop() {
        budget -= 1;
        if budget == 0 {
            return None;
        }
        let (c, stride) = place[&v];
        let Some(node) = data.vbank().get(v) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            let out = o.get_out();
            let next = match o.code() {
                OpCode::CPUI_LOAD if o.get_in(1) == Some(v) => {
                    if let Some(out) = out {
                        pointers.push((r, v, out, false));
                    }
                    None
                }
                OpCode::CPUI_STORE if o.get_in(1) == Some(v) => {
                    if let Some(val) = o.get_in(2) {
                        pointers.push((r, v, val, true));
                    }
                    None
                }
                OpCode::CPUI_INT_ADD => {
                    let other = if o.get_in(0) == Some(v) { o.get_in(1) } else { o.get_in(0) };
                    match constant(other) {
                        Some(k) => Some((c.wrapping_add(k), stride)),
                        None => Some((c, gcd(stride, step_of(other)))),
                    }
                }
                OpCode::CPUI_INT_SUB if o.get_in(0) == Some(v) => {
                    constant(o.get_in(1)).map(|k| (c.wrapping_sub(k), stride)).or(Some((c, 1)))
                }
                OpCode::CPUI_PTRSUB if o.get_in(0) == Some(v) => {
                    constant(o.get_in(1)).map(|k| (c.wrapping_add(k), stride)).or(Some((c, 1)))
                }
                OpCode::CPUI_PTRADD if o.get_in(0) == Some(v) => {
                    let elsize = constant(o.get_in(2)).unwrap_or(1);
                    match constant(o.get_in(1)) {
                        Some(k) => Some((c.wrapping_add(k.wrapping_mul(elsize)), stride)),
                        None => Some((c, gcd(stride, elsize))),
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => Some((c, stride)),
                OpCode::CPUI_INDIRECT if o.get_in(0) == Some(v) => Some((c, stride)),
                _ => None,
            };
            let (Some((nc, ns)), Some(out)) = (next, out) else { continue };
            match place.get(&out).copied() {
                None => {
                    place.insert(out, (nc, ns));
                    work.push(out);
                }
                Some((pc, ps)) => {
                    let merged = gcd(gcd(ps, ns), nc.wrapping_sub(pc));
                    if merged != ps {
                        place.insert(out, (pc, merged));
                        work.push(out);
                    }
                }
            }
        }
    }
    let accesses = pointers
        .into_iter()
        .filter_map(|(op, ptr, value, store)| {
            let (at, stride) = place[&ptr];
            let size = data.vbank().get(value)?.get_size();
            Some(Access { op, value, at, stride, size, store })
        })
        .collect();
    Some((accesses, place.into_values().collect()))
}

enum Member {
    Exact(Rc<Datatype>),
    Gap,
    Split,
}

/// The component of `ct` an access of `size` bytes at `off` lands on: exactly one
/// (nested) member, a hole in a structure, or anything else -- part of a member,
/// or several at once, which the split rules print piece by piece
/// (`v6._0_1_ = a4[1].field_0x0`).
fn member_at(ct: &Rc<Datatype>, off: i64, size: int4) -> Member {
    let composite = |t: &Datatype| {
        matches!(
            t.get_metatype(),
            type_metatype::TYPE_STRUCT
                | type_metatype::TYPE_ARRAY
                | type_metatype::TYPE_UNION
                | type_metatype::TYPE_PARTIALSTRUCT
                | type_metatype::TYPE_PARTIALUNION
        )
    };
    let mut t = Rc::clone(ct);
    let mut o = off;
    for _ in 0..16 {
        if o == 0 && t.get_size() == size && !composite(&t) {
            return Member::Exact(t);
        }
        match t.get_sub_type(o) {
            Ok((Some(sub), newoff)) if newoff >= 0 && newoff + size as i64 <= sub.get_size() as i64 => {
                t = sub;
                o = newoff;
            }
            Ok((None, _)) if t.get_metatype() == type_metatype::TYPE_STRUCT => return Member::Gap,
            _ => return Member::Split,
        }
    }
    Member::Split
}

/// Does what the caller does through a pointer disagree with the pointee a vote
/// `ct` would give it?  A non-character primitive, float or composite pointee is
/// checked.
///
/// A non-character primitive pointee refuses any constant stored through it
/// wider than itself: `SplitDatatype` reads such a pointer as an array of its
/// pointee, so an `unsigned char *` vote prints the field write
/// `*(long *)(a0 + 0x5c) = 0x100` as eight byte stores, and a buffer filled a
/// word at a time as one store per byte of each word.  A wider store of a
/// computed value prints as a cast and is allowed.  A character pointee is left
/// alone, because the byte stores it produces are what the string-copy idiom
/// prints as `builtin_strncpy`.  Every check refuses when there are too many
/// addresses to follow.
///
/// A composite pointee refuses an access or an address outside it
/// (`a0[0x33].field_0x4` for a caller that reads offset 0x19c of a callee's
/// partial `struct_N`), a pointer stepped or indexed by other than one whole
/// element (`a0 = &a0[4].field_0x8` for `a0 += 0x48` over a 16-byte structure,
/// `v18[-2].field_0x8` for 32-byte records), and an access that is not exactly
/// one member (an 8-byte copy over an `int` and a `float`, printed as four byte
/// pieces or split into two stores).  A pointer member loaded or stored there is
/// held to the same rule for its own pointee, two levels deep.  A float element
/// or member refuses a value the caller computes with as an integer, stores from
/// anything but a float, or hands on: the float-vote rule one level down, so
/// `u->l[0] = v + 1` through a union a callee reads as `double *` does not print
/// as `*a0 = (double)(a1 + 1)`.
fn pointee_refuses(data: &Funcdata, family: &[VarnodeId], ct: &Datatype, depth: u32) -> bool {
    let (pointee, base) = match ct.get_metatype() {
        type_metatype::TYPE_PTR => (ct.get_ptr_to(), 0),
        type_metatype::TYPE_PTRREL => (ct.get_rel_parent(), ct.get_byte_offset().unwrap_or(0) as i64),
        _ => return false,
    };
    let Some(pointee) = pointee else { return false };
    if matches!(
        pointee.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL | type_metatype::TYPE_PTR
    ) && !pointee.is_char_print()
    {
        let width = pointee.get_align_size() as i64;
        let family = with_sibling_loads(data, family);
        let Some((accesses, _)) = accesses_through(data, &family) else { return true };
        return accesses.iter().any(|a| {
            a.store && a.size as i64 > width && data.vbank().get(a.value).is_some_and(|n| n.is_constant())
        });
    }
    let float = pointee.get_metatype() == type_metatype::TYPE_FLOAT;
    if !float
        && !matches!(
            pointee.get_metatype(),
            type_metatype::TYPE_STRUCT
                | type_metatype::TYPE_ARRAY
                | type_metatype::TYPE_UNION
                | type_metatype::TYPE_PARTIALSTRUCT
                | type_metatype::TYPE_PARTIALUNION
        )
    {
        return false;
    }
    let size = pointee.get_size() as i64;
    let family = with_sibling_loads(data, family);
    let Some((accesses, places)) = accesses_through(data, &family) else { return true };
    if size <= 0 {
        return !accesses.is_empty();
    }
    if !float
        && places.iter().any(|&(c, stride)| {
            if stride == 0 { !(0..=size).contains(&c.wrapping_add(base)) } else { stride.abs() != size }
        })
    {
        return true;
    }
    let mut floats: Vec<(&Access, Rc<Datatype>)> = Vec::new();
    for a in &accesses {
        let at = a.at.wrapping_add(base);
        if a.stride % size != 0 || (!float && a.stride != 0 && a.stride.abs() != size) {
            return true;
        }
        let off = if float || a.stride != 0 { at.rem_euclid(size) } else { at };
        if off < 0 || off + a.size as i64 > size {
            return true;
        }
        match member_at(&pointee, off, a.size) {
            Member::Split => return true,
            Member::Gap => {}
            Member::Exact(t) if t.get_metatype() == type_metatype::TYPE_FLOAT => floats.push((a, t)),
            Member::Exact(t) if depth < 2 && t.get_metatype() == type_metatype::TYPE_PTR => {
                if !data.vbank().get(a.value).is_some_and(|n| n.is_constant())
                    && pointee_refuses(data, &value_family(data, a.value), &t, depth + 1)
                {
                    return true;
                }
            }
            Member::Exact(_) => {}
        }
    }
    let own: std::collections::HashSet<OpId> = floats.iter().map(|(a, _)| a.op).collect();
    floats.iter().any(|(a, t)| {
        if a.store && data.vbank().get(a.value).is_some_and(|n| n.is_constant()) {
            return !prints_exactly_as_a_float(data, a.value);
        }
        family_refuses(data, a.value, Reading::Member(&own), t)
    })
}

/// Does the RETURN `o` hand `node` back as anything but a float?  An undeclared
/// return type follows the value, so a float vote there retypes the function's
/// result for every caller -- unless the result travels in a float register.
fn returned_as_other_than_float(
    data: &Funcdata,
    o: &crate::op::PcodeOp,
    v: VarnodeId,
    node: &crate::varnode::Varnode,
) -> bool {
    if !(1..o.num_input()).any(|s| o.get_in(s) == Some(v)) {
        return false;
    }
    let proto = data.get_func_proto();
    if proto.is_output_locked() {
        return proto.get_output_type().and_then(|t| class_of(t)) != Some(Class::Float);
    }
    !in_a_float_register(proto.has_model().then(|| proto.model().output_list()).flatten(), node)
}

/// The input (`input`) or output resource list of the call `call`'s model.
fn call_model(data: &Funcdata, call: OpId, input: bool) -> Option<&crate::p4_calls::fspec::ParamListStandard> {
    let proto = data.get_call_specs(data.get_call_specs_index(call)?).proto();
    if !proto.has_model() {
        return None;
    }
    if input { proto.model().input_opt() } else { proto.model().output_list() }
}

/// Does `node` live in a register `list` assigns to floats?  A value a call reads
/// or produces there is a float to that call whatever its declaration says.
fn in_a_float_register(list: Option<&crate::p4_calls::fspec::ParamListStandard>, node: &crate::varnode::Varnode) -> bool {
    list.and_then(|l| l.find_entry(node.get_addr(), node.get_size(), true).map(|i| (l, i)))
        .is_some_and(|(l, i)| l.get_entry()[i].get_type() == crate::dtype::type_class::TYPECLASS_FLOAT)
}

/// The class a call reads its argument `slot` as: the declared parameter's when
/// the parameter is type-locked, else the type a callee stated for it.
fn reader_class(data: &Funcdata, call: OpId, slot: int4) -> Option<Class> {
    let fc = data.get_call_specs(data.get_call_specs_index(call)?);
    if let Some(param) = fc.proto().get_param(slot - 1) {
        if param.is_type_locked() {
            return param.get_type().and_then(|t| class_of(t));
        }
    }
    stated_argument_type(data, fc, slot).and_then(|t| class_of(t))
}

/// The class a call's result is declared as, when its output is locked.
fn writer_class(data: &Funcdata, call: OpId) -> Option<Class> {
    let proto = data.get_call_specs(data.get_call_specs_index(call)?).proto();
    if !proto.is_output_locked() {
        return None;
    }
    proto.get_output_type().and_then(|t| class_of(t))
}

fn produced_class(code: OpCode) -> Option<Class> {
    use OpCode::*;
    match code {
        CPUI_FLOAT_ADD | CPUI_FLOAT_SUB | CPUI_FLOAT_MULT | CPUI_FLOAT_DIV | CPUI_FLOAT_NEG
        | CPUI_FLOAT_ABS | CPUI_FLOAT_SQRT | CPUI_FLOAT_INT2FLOAT | CPUI_FLOAT_FLOAT2FLOAT
        | CPUI_FLOAT_CEIL | CPUI_FLOAT_FLOOR | CPUI_FLOAT_ROUND => Some(Class::Float),
        CPUI_INT_MULT | CPUI_INT_DIV | CPUI_INT_SDIV | CPUI_INT_REM | CPUI_INT_SREM | CPUI_INT_AND
        | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT
        | CPUI_INT_2COMP | CPUI_INT_NEGATE | CPUI_FLOAT_TRUNC => Some(Class::Integer),
        _ => None,
    }
}

/// Integer ops a pointer also meets -- it is added to, subtracted, compared,
/// extended, truncated and pieced together -- so they say nothing against a
/// pointer vote.  A float passed in a general register that meets one is a bit
/// pattern the caller computes with, and a float vote would print that as a value
/// conversion: `(int)v1 + 3`, or `(short)((unsigned int)v1 >> 0x10)` for a
/// halfword taken out of the bits.
fn integer_but_pointer_neutral(code: OpCode) -> bool {
    use OpCode::*;
    matches!(
        code,
        CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_ZEXT | CPUI_INT_SEXT | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL
            | CPUI_INT_LESS | CPUI_INT_SLESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESSEQUAL | CPUI_INT_CARRY
            | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_POPCOUNT | CPUI_LZCOUNT | CPUI_SUBPIECE | CPUI_PIECE
    )
}

fn read_class(code: OpCode) -> Option<Class> {
    use OpCode::*;
    match code {
        CPUI_FLOAT_ADD | CPUI_FLOAT_SUB | CPUI_FLOAT_MULT | CPUI_FLOAT_DIV | CPUI_FLOAT_NEG
        | CPUI_FLOAT_ABS | CPUI_FLOAT_SQRT | CPUI_FLOAT_FLOAT2FLOAT | CPUI_FLOAT_CEIL
        | CPUI_FLOAT_FLOOR | CPUI_FLOAT_ROUND | CPUI_FLOAT_EQUAL | CPUI_FLOAT_NOTEQUAL
        | CPUI_FLOAT_LESS | CPUI_FLOAT_LESSEQUAL | CPUI_FLOAT_NAN | CPUI_FLOAT_TRUNC => {
            Some(Class::Float)
        }
        CPUI_INT_MULT | CPUI_INT_DIV | CPUI_INT_SDIV | CPUI_INT_REM | CPUI_INT_SREM | CPUI_INT_AND
        | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT
        | CPUI_INT_2COMP | CPUI_INT_NEGATE | CPUI_FLOAT_INT2FLOAT => Some(Class::Integer),
        _ => None,
    }
}

/// Why a recovered prototype was not parked.
///
/// Every variant is a class of evidence that is weaker than what a locked
/// prototype claims; the driver reports the tally so a run can be audited
/// without re-deriving it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decline {
    /// The function's decompile failed: there is no recovered prototype at all.
    DecompileError,
    /// The recovered prototype has no parameter store (the `setScope` boundary),
    /// so nothing was recovered to state.
    NoRecovery,
    /// No parameters and a `void` return: the prototype says nothing a caller
    /// does not already assume.
    VoidVoid,
    /// The recovered prototype carries a `...`.
    ///
    /// This is NOT the variadic rule, and must not be presented as one.
    /// `FuncProto::is_dotdotdot` is only ever set from a DECLARED signature, and
    /// a declared entry is rejected one branch earlier by [`Decline::Declared`],
    /// so on a stripped image this variant cannot fire at all -- measured at
    /// zero `variadic` declines across 878 parks on four binaries.  It stays as
    /// the converter's own guard (`recovered_pieces` is public and is tested
    /// standalone); what actually answers for a variadic on stripped input is
    /// [`Decline::UnderRecovered`], which reads the register-save prologue in
    /// the callee's own body.
    Variadic,
    /// Model selection did not settle on a known prototype model, so the storage
    /// the parked types would be re-bound to is itself a guess.
    ModelUnknown,
    /// A hidden return pointer or an indirect-storage parameter: the pieces
    /// vocabulary drops hidden parameters, so the slot numbering the storage
    /// carry depends on would not survive the round trip.
    HiddenParam,
    /// A parameter recovered with no data-type.
    UntypedParam,
    /// A parameter whose recovered storage is not a real address.
    InvalidStorage,
    /// The recovered parameters are not where the calling convention would put
    /// those types.  A parked prototype is re-bound from the model
    /// (`set_pieces` -> `update_all_types` -> `assign_parameter_storage`), so
    /// such a prototype cannot be stated faithfully -- and the population is
    /// dominated by one artifact: a variadic register-save prologue reads every
    /// argument register there is, so recovery reports fourteen parameters for a
    /// two-argument function and a lock would promote fourteen live-in registers
    /// to each caller's OWN parameter list.
    NonCanonicalStorage,
    /// The model could not assign storage to the recovered types at all.
    StorageUnderivable,
    /// A prototype is already declared for this function -- a `--assert
    /// prototype`, a libc/`libctypes` table entry, a DWARF (`cppproto`) or
    /// demangled (`cppsig`) signature.  A declared fact outranks a recovered
    /// one, always.
    Declared,
    /// The function is in a call-graph cycle with more than one member, or calls
    /// itself: "callees first" has no meaning inside a cycle.  `types` and
    /// `lock` state nothing there; `cycles` states the types and the driver
    /// orders the cycle ([`ProtoOrderMode::Cycles`]).
    Scc,
    /// The callee's own body PROVABLY reads the argument register that would
    /// hold its next parameter, so its recovered list is short of what the
    /// function really consumes ([`under_recovered`]).  This is the rule that
    /// catches what [`Decline::Variadic`] cannot: a SysV variadic's
    /// register-save prologue reads every argument register there is, and a
    /// forwarding thunk (`jmp *(%rdi)`) passes them through without ever
    /// naming them, and NEITHER carries a declared `...` for
    /// `FuncProto::is_dotdotdot` to report.
    UnderRecovered,
    /// The recovered list ends on the LAST argument register the calling
    /// convention has, so the next argument would arrive on the stack -- the one
    /// place [`under_recovered`] cannot look, and the exact shape a variadic's
    /// register-save prologue leaves behind ([`ends_at_the_register_boundary`]).
    RegisterFileFull,
}

impl Decline {
    /// The token the driver's trace and the feature docs use.
    pub fn as_str(self) -> &'static str {
        match self {
            Decline::DecompileError => "decompile-error",
            Decline::NoRecovery => "no-recovery",
            Decline::VoidVoid => "void-void",
            Decline::Variadic => "variadic",
            Decline::ModelUnknown => "model-unknown",
            Decline::HiddenParam => "hidden-param",
            Decline::UntypedParam => "untyped-param",
            Decline::InvalidStorage => "invalid-storage",
            Decline::NonCanonicalStorage => "non-canonical-storage",
            Decline::StorageUnderivable => "storage-underivable",
            Decline::Declared => "declared",
            Decline::Scc => "scc",
            Decline::UnderRecovered => "under-recovered",
            Decline::RegisterFileFull => "register-file-full",
        }
    }
}

/// A recovered prototype that passed the policy, ready to park.
#[derive(Debug, Clone)]
pub struct Recovered {
    /// The pieces to park.
    pub pieces: PrototypePieces,
    /// How many trailing parameters [`trim_unjustified_tail`] removed before parking.
    pub trimmed: usize,
}

/// Convert a function's recovered prototype into the pieces a caller's
/// `ActionDefaultParams` reads back, or say why it must not be stated.
///
/// The types are the point, but the STORAGE is what decides whether the round
/// trip is faithful at all: `seed_locked_from_pieces` -> `set_pieces` ->
/// `update_all_types` re-derives each parameter's storage from the model, so a
/// callee whose recovered parameters are not where the convention puts those
/// types -- a skipped register, a parameter recovered on the stack while an
/// argument register is free -- would be silently re-bound to different storage
/// at every call site.  The recovered storage is returned alongside the pieces
/// so [`park_recovered`] can compare it with [`model_storage`] and decline the
/// mismatch ([`Decline::NonCanonicalStorage`]).
///
/// Parameter NAMES are carried as recovered (`a0`, `a1` on a stripped image).
///
/// A recovered VOID return is not stated: `set_pieces` output-locks whatever the
/// pieces name, and a locked void output removes the call's result at every
/// site, so a caller reading the return register would read a value nothing
/// writes.  The parked list is a FLOOR (`first_var_arg_slot` past the recovered
/// parameters): a closed list skips `init_active_input` for the locked call spec
/// and deletes every argument past the parked ones, while a variable tail keeps
/// the caller's own recovery on (`coreaction_protos.rs`, `if !inputlocked ||
/// varargs`).
pub fn recovered_pieces(
    proto: &FuncProto,
    name: &str,
) -> Result<(PrototypePieces, Vec<(Address, int4)>), Decline> {
    if !proto.has_store() {
        return Err(Decline::NoRecovery);
    }
    if proto.is_dotdotdot() {
        return Err(Decline::Variadic);
    }
    if !proto.has_model() || proto.is_model_unknown() {
        return Err(Decline::ModelUnknown);
    }
    let nparams = proto.num_params();
    let outtype = proto.get_output_type().cloned();
    let out_is_void = outtype.as_ref().is_none_or(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_VOID);
    if nparams == 0 && out_is_void {
        return Err(Decline::VoidVoid);
    }
    let outtype = if out_is_void { None } else { outtype };
    let mut pieces = PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes: Vec::new(),
        innames: Vec::new(),
        first_var_arg_slot: nparams,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut storage: Vec<(Address, int4)> = Vec::new();
    for i in 0..nparams {
        let Some(param) = proto.get_param(i) else {
            return Err(Decline::NoRecovery);
        };
        if param.is_hidden_return() || param.is_indirect_storage() || param.is_this_pointer() {
            return Err(Decline::HiddenParam);
        }
        let Some(ct) = param.get_type() else {
            return Err(Decline::UntypedParam);
        };
        let addr = param.get_address();
        if addr.is_invalid() || param.get_size() <= 0 {
            return Err(Decline::InvalidStorage);
        }
        pieces.intypes.push(Rc::clone(ct));
        let pname = param.get_name();
        pieces.innames.push(if pname.is_empty() { format!("a{i}") } else { pname.to_string() });
        storage.push((addr, param.get_size()));
    }
    Ok((pieces, storage))
}

/// The storage the prototype model would assign to `pieces`' types, or `None`
/// when the assignment cannot be derived (no default model, no type factory, or
/// an un-ported storage boundary).
///
/// This is the same seam a caller takes: `ActionDefaultParams` builds the
/// callee's `FuncProto` with `seed_locked_from_pieces`, and that is what decides
/// where each parked type lands.
pub fn model_storage(pieces: &PrototypePieces, arch: &Architecture) -> Option<Vec<(Address, int4)>> {
    let dfp = arch.default_fp()?.clone();
    let void_ty = arch.types().get_type_void().ok()?;
    let mut fp = FuncProto::new();
    let types_only = PrototypePieces {
        input_storage: Vec::new(),
        output_storage: None,
        ..pieces.clone()
    };
    fp.seed_locked_from_pieces(&types_only, dfp, void_ty, arch.types(), arch.manage()).ok()?;
    let n = fp.num_params();
    if n != pieces.intypes.len() as int4 {
        return None;
    }
    let mut out = Vec::with_capacity(n as usize);
    for i in 0..n {
        let param = fp.get_param(i)?;
        out.push((param.get_address(), param.get_size()));
    }
    Some(out)
}

/// The callee-body entry-liveness summary for `entry`, cached on the
/// `Architecture` for the whole run.
///
/// The same probe `calleedeadarg`/`calleearitylive` take
/// ([`crate::kuna_calleedeadarg::probe_callee_entry_dead`]) and the same cache,
/// so a body decoded for one of them is not decoded again here.  It answers one
/// question per register range, one-sidedly: does every path from the entry
/// write these bytes before reading them (`proves_dead`), or does some path read
/// them first (`proves_read`)?  Everything it cannot see answers `false` to
/// both, which makes both rules below decline to act rather than guess.
fn entry_facts(
    arch: &mut Architecture,
    entry: &Address,
) -> Option<Rc<crate::kuna_calleedeadarg::CalleeEntryDead>> {
    let reg_idx = arch.manage().get_space_by_name("register").map(|s| s.get_index())?;
    if reg_idx < 0 {
        return None;
    }
    let sp = entry.get_space()?;
    let key = (sp.get_index(), entry.get_offset());
    if let Some(d) = arch.kuna_callee_dead_cache.get(&key) {
        return Some(Rc::clone(d));
    }
    let probed = Rc::new(crate::kuna_calleedeadarg::probe_callee_entry_dead(
        arch.translate(),
        entry,
        reg_idx,
    ));
    arch.kuna_callee_dead_cache.insert(key, Rc::clone(&probed));
    Some(probed)
}

/// Where the model would put ONE more parameter past the ones in `pieces` --
/// the storage the callee's next argument would arrive in.
///
/// Asked of the model rather than of an architecture table, so the answer is
/// whatever the program's own calling convention says (`rsi` after `rdi` on
/// SysV, `w1` after `w0` on AArch64, a stack slot once the register file runs
/// out).
fn next_slot_storage(pieces: &PrototypePieces, arch: &Architecture) -> Option<(Address, int4)> {
    let ptr = arch.types().get_base(arch.types().get_size_of_pointer(), type_metatype::TYPE_UINT).ok()?;
    let mut probe = pieces.clone();
    probe.intypes.push(ptr);
    probe.innames.push("next".to_string());
    probe.first_var_arg_slot = -1;
    model_storage(&probe, arch)?.pop()
}

/// Drop the trailing parameters the callee's own body does not prove it
/// consumes.
///
/// Recovery over-counts at the END of an argument list far more often than it
/// under-counts: an argument register an earlier call left live, or one a
/// nested call reads without the body ever writing it, looks exactly like a
/// parameter, so `save_cwd(struct saved_cwd *)` -- one parameter in the
/// DWARF -- is recovered with three.  Stating that list makes every caller
/// materialise the two extra slots, and materialising a slot is not free at the
/// caller: the fabricated register reads join the caller's own dataflow and can
/// take a register away from an EARLIER, ordinary call in the same body.
///
/// So the tail carries the burden of proof, not the other way round.  A
/// trailing parameter is stated only when the callee's body reads the register
/// it would arrive in before writing it, for a value that can reach something
/// ([`CalleeEntryDead::proves_input`] -- the register-zeroing idiom `xor
/// esi,esi` is not a use).  Everything else is dropped, including a parameter
/// the body neither reads nor writes, and a parameter on the stack, which the
/// entry walk cannot speak about at all.
///
/// Dropping a real parameter is the cheap direction here and only because the
/// parked list is a FLOOR (`first_var_arg_slot`): the caller still recovers
/// whatever it passes past the stated prefix, so an under-stated tail costs a
/// type on that slot, while an over-stated one fabricates an argument.  Only
/// the tail is trimmed: removing an interior parameter would leave a hole the
/// convention re-packs, silently re-binding every later type.
///
/// A walk that did not cover every path (`!is_complete`) is no evidence in
/// either direction, so nothing is trimmed and the recovered list stands or
/// falls on the rest of the policy.
///
/// Returns how many were dropped.
fn trim_unjustified_tail(
    pieces: &mut PrototypePieces,
    storage: &mut Vec<(Address, int4)>,
    facts: &crate::kuna_calleedeadarg::CalleeEntryDead,
) -> usize {
    if !facts.is_complete() {
        return 0;
    }
    let mut dropped = 0usize;
    while let Some((addr, size)) = storage.last().cloned() {
        if facts.proves_input(&addr, size) {
            break;
        }
        storage.pop();
        pieces.intypes.pop();
        pieces.innames.pop();
        dropped += 1;
    }
    dropped
}

/// Does the callee's own body prove its recovered list is SHORT -- that it reads
/// the register its next argument would arrive in?
///
/// This is the guard `Decline::Variadic` cannot be: `FuncProto::is_dotdotdot`
/// is only ever set from a DECLARED signature, and a declared entry never
/// reaches the policy at all ([`Decline::Declared`] fires first), so on a
/// stripped image it can never fire.  A body-level read of the next argument
/// register is the shape that CAN, and it is what both dangerous populations
/// look like from the outside: a SysV variadic's register-save prologue
/// (`test al,al; movaps %xmm0,0x50(%rsp); ...`) reads every argument register
/// there is, and a forwarding thunk (`mov 0x50(%rdi),%rax; jmp *%rax`) reads
/// the ones it passes through without writing any of them.  Both recover a
/// short list; stating it would type the wrong slots.
fn under_recovered(
    next: Option<(Address, int4)>,
    facts: &crate::kuna_calleedeadarg::CalleeEntryDead,
) -> bool {
    slot_is_read(next, facts)
}

/// Does the recovered list end exactly where the calling convention runs out of
/// argument registers?
///
/// This is the hole [`under_recovered`] cannot cover.  That rule asks the
/// callee's body whether it reads the register its NEXT argument would arrive
/// in; once the recovered list fills the register file, the next argument
/// arrives on the stack, at an offset the entry walk cannot speak about, so the
/// question comes back `false` for a variadic and for a real N-argument function
/// alike.  The answer is not evidence -- it is the absence of a witness.
///
/// And the population sitting on that boundary is not neutral.  A SysV variadic
/// that calls `va_start` opens with the register-save prologue -- `mov
/// %rsi,0x28(%rsp) ... mov %r9,0x48(%rsp); test %al,%al; movaps %xmm0,...` --
/// which READS every argument register the convention has, so recovery reports
/// exactly six parameters for `xlog(const char *fmt, ...)`; AAPCS builds the
/// same list with `push {r0-r3}` and reports exactly four.  Stating that list
/// gives every caller five (or three) fabricated arguments that the machine code
/// at the call site never writes.
///
/// So a list that saturates the register file is declined.  The cost is the
/// genuine six-integer-argument function on SysV and the genuine four-argument
/// one on AAPCS, which lose their types at their call sites -- the option's OFF
/// behaviour, not a wrong answer.
///
/// The test is asked of the MODEL, not of an architecture table, and it is
/// two-sided on purpose: the last recovered parameter must itself be in a
/// register and the next slot must not.  A convention that passes everything on
/// the stack (i386 `cdecl`) never has the last parameter in a register, so it
/// never trips this at all.  A storage the model cannot name says nothing, in
/// either position.
fn ends_at_the_register_boundary(
    last: Option<&(Address, int4)>,
    next: Option<(Address, int4)>,
) -> bool {
    let (Some((last, _)), Some((next, _))) = (last, next) else {
        return false;
    };
    crate::kuna_calleearitybody::is_register(last)
        && !crate::kuna_calleearitybody::is_register(&next)
}

/// The decision [`under_recovered`] makes once the next slot's storage is
/// known: a storage the model cannot name says nothing.
fn slot_is_read(
    next: Option<(Address, int4)>,
    facts: &crate::kuna_calleedeadarg::CalleeEntryDead,
) -> bool {
    match next {
        Some((addr, size)) => facts.proves_read(&addr, size),
        None => false,
    }
}

/// Park the function's recovered prototype on its own `FunctionSymbol`, so the
/// callers decompiled after it read it through `ActionDefaultParams`.
///
/// Declines where a prototype is already declared for this entry: a
/// `--assert prototype`, a libc/`libctypes` signature, a DWARF or demangled one
/// are all parked in the same slot, and every one of them is a stated fact that
/// outranks a recovered guess.
///
/// The parked list is always a FLOOR -- `first_var_arg_slot` marks the first
/// slot past the recovered parameters as the start of a variable tail -- because
/// a closed list is answered entirely from the parked parameters and truncates
/// every call that passes more.  Closing it where the entry walk decoded the
/// callee's whole body (every path ending at a `RETURN`, so "no other argument
/// register is read" is a statement about the body and not about its first few
/// instructions) was measured and is a null result: over twenty-two binaries it
/// left the fabricated-parameter counters untouched (785 functions gaining
/// `variables[]` arg rows either way) and deleted nineteen more arguments.  The
/// callees that fabricate are the ones that call something, which is exactly
/// where that walk stops.
pub fn park_recovered(
    arch: &mut Architecture,
    entry: &Address,
    name: &str,
    data: &crate::funcdata::Funcdata,
    proto: &FuncProto,
    mode: ProtoOrderMode,
) -> Result<Recovered, Decline> {
    if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        return Err(Decline::Declared);
    }
    let (mut pieces, mut storage) = recovered_pieces(proto, name)?;
    if mode.states_types_only() {
        let arity_sound = arch.pass_through && arity_claim_sound(arch, entry, &pieces, &storage);
        let output = if arch.pass_through { recovered_output(proto, data) } else { None };
        let vararg_tail = if arch.pass_through {
            crate::kuna_varargtail::vararg_tail_inputs(data, &storage)
        } else {
            Vec::new()
        };
        return state_recovered_types(arch, entry, pieces, storage, arity_sound, output, vararg_tail);
    }
    let mut trimmed = 0usize;
    if let Some(facts) = entry_facts(arch, entry) {
        trimmed = trim_unjustified_tail(&mut pieces, &mut storage, &facts);
        let next = next_slot_storage(&pieces, arch);
        if under_recovered(next.clone(), &facts) {
            return Err(Decline::UnderRecovered);
        }
        if ends_at_the_register_boundary(storage.last(), next) {
            return Err(Decline::RegisterFileFull);
        }
    }
    if pieces.intypes.is_empty() && pieces.outtype.is_none() {
        return Err(Decline::VoidVoid);
    }
    pieces.first_var_arg_slot = pieces.intypes.len() as int4;
    match model_storage(&pieces, arch) {
        None => return Err(Decline::StorageUnderivable),
        Some(model) if model != storage => return Err(Decline::NonCanonicalStorage),
        Some(_) => {}
    }
    arch.set_function_prototype_pieces_at(entry, pieces.clone());
    Ok(Recovered { pieces, trimmed })
}

/// Would [`park_recovered`]'s locking branch accept `storage` as a statement of
/// the callee's arity?
///
/// The three declines that answer for the list as a whole, and between them
/// for a variadic's register-save prologue however much of it recovery kept: a
/// body that reads the register the next parameter would arrive in
/// ([`Decline::UnderRecovered`] -- gnulib `rpl_fcntl(int,int,...)` recovers
/// `(int,int,unsigned long)` and then saves `rcx`, `r8` and `r9` too), a list
/// ending on the last argument register the convention has
/// ([`Decline::RegisterFileFull`]), and a list the model would put elsewhere
/// ([`Decline::NonCanonicalStorage`]). A list that already reaches the stack
/// has crossed that boundary too, and a walk that cannot see the callee's body
/// states nothing sound.
///
/// The next-slot read is asked with the register-zeroing idiom excluded
/// (`proves_input`, not the locking branch's `proves_read`): `xor esi,esi` in
/// gnulib `last_component(char const *)` writes a constant, and a save prologue
/// stores the register's value, which is a read either way.
fn arity_claim_sound(
    arch: &mut Architecture,
    entry: &Address,
    pieces: &PrototypePieces,
    storage: &[(Address, int4)],
) -> bool {
    let Some(facts) = entry_facts(arch, entry) else { return false };
    if !storage.iter().all(|(a, _)| crate::kuna_calleearitybody::is_register(a)) {
        return false;
    }
    let next = next_slot_storage(pieces, arch);
    let next_read = next.as_ref().is_some_and(|(a, sz)| facts.proves_input(a, *sz));
    if next_read || ends_at_the_register_boundary(storage.last(), next) {
        return false;
    }
    matches!(model_storage(pieces, arch), Some(model) if model == storage)
}

/// The recovered return value of `proto` -- storage, size, type -- or `None`
/// for a `void` or storage-less one, or one the function never computed.
///
/// A recovered return is stated only when every live RETURN hands back a value
/// the body produced ([`crate::kuna_returnuncomputed::every_return_computes`]).
/// A function that ends on a call and keeps that call's return-register clobber
/// is recovered as returning it -- gnulib's `void version_etc_arn` comes out
/// `long` -- and a wrapper told so hands the same wrong value back.
fn recovered_output(
    proto: &FuncProto,
    data: &crate::funcdata::Funcdata,
) -> Option<(Address, int4, Rc<Datatype>)> {
    let out = proto.get_output();
    let ct = out.get_type()?;
    if ct.get_metatype() == type_metatype::TYPE_VOID {
        return None;
    }
    if !crate::kuna_returnuncomputed::every_return_computes(data) {
        return None;
    }
    let addr = out.get_address();
    if addr.is_invalid() || out.get_size() <= 0 {
        return None;
    }
    Some((addr, out.get_size(), Rc::clone(ct)))
}

/// State the callee's recovered TYPES for the callers still ahead of it, without
/// stating anything about its arity ([`ProtoOrderMode::Types`]).
///
/// Nothing is written to the symbol table, so `ActionDefaultParams` never sees a
/// prototype here and no call spec is input-locked: every call site recovers its
/// own argument list exactly as it does with the option off, and the recovered
/// types reach it as one more vote in the fold `Varnode::getLocalType` already
/// runs over every reader of a Varnode.  That is why the arity rules the locking
/// mode needs -- the unjustified-tail trim, the under-recovery walk, the
/// register-file boundary, the canonical-storage round trip -- are not applied
/// here: every one of them exists to stop a parked list from FABRICATING an
/// argument, and in this mode a parked list cannot.  A short recovered list
/// states fewer types; a long one states types for slots the caller does not
/// have, and those never match a caller argument at all.
///
/// It is also why this mode is cheap.  The locking mode parks through
/// `set_function_proto_pieces`, which bumps `Database::kuna_generation` and so
/// drops the memoized `build_callee_proto_pieces` snapshot every later function
/// rebuilds; this table is the module's own, and the symbol table does not move.
fn state_recovered_types(
    arch: &mut Architecture,
    entry: &Address,
    pieces: PrototypePieces,
    storage: Vec<(Address, int4)>,
    arity_sound: bool,
    output: Option<(Address, int4, Rc<Datatype>)>,
    vararg_tail: Vec<Address>,
) -> Result<Recovered, Decline> {
    let inputs: Vec<(Address, int4, Rc<Datatype>)> = storage
        .iter()
        .zip(pieces.intypes.iter())
        .map(|((addr, size), ct)| (addr.clone(), *size, Rc::clone(ct)))
        .collect();
    if inputs.is_empty() {
        return Err(Decline::VoidVoid);
    }
    let Some(key) = stated_key(entry) else {
        return Err(Decline::InvalidStorage);
    };
    arch.kuna_protoorder_types
        .insert(key, Rc::new(RecoveredTypes { inputs, arity_sound, output, vararg_tail }));
    Ok(Recovered { pieces, trimmed: 0 })
}

/// The key a function's statement is filed under in
/// `Architecture::kuna_protoorder_types`.
pub fn stated_key(entry: &Address) -> Option<(int4, uintb)> {
    Some((entry.get_space()?.get_index(), entry.get_offset()))
}

/// Forget every statement with a parameter type that names one of `names`,
/// through any depth of pointer.
///
/// The `structsynth` convergence sweep calls this with the superseded structures
/// before it decompiles anything again.  Such a statement was made before the
/// survivor existed, and a redo that read it would type an argument with the
/// structure the redo exists to replace.  A callee the sweep redoes states again.
pub fn forget_statements_naming(arch: &mut Architecture, names: &[String]) {
    if names.is_empty() {
        return;
    }
    arch.kuna_protoorder_types
        .retain(|_, stated| !stated.inputs.iter().any(|(_, _, ct)| names_type(ct, names)));
}

/// Is `ct`, or what it points at at any depth, named one of `names`?
pub(crate) fn names_type(ct: &Rc<Datatype>, names: &[String]) -> bool {
    let mut cur = Rc::clone(ct);
    for _ in 0..8 {
        if names.iter().any(|n| n == cur.get_name()) {
            return true;
        }
        match cur.get_ptr_to() {
            Some(next) => cur = next,
            None => return false,
        }
    }
    false
}

/// Copy onto `data` the types every callee it calls stated about itself.
///
/// The `Architecture` holding the table is not reachable from the type-inference
/// seam that reads it (`Funcdata::get_arch` is the per-function `ArchHandle`), so
/// the facts are copied across once per function, after the flow build has
/// created the call specs and before the pipeline runs -- the same bridge
/// `calleedeadarg` and `rustabi` take for their own callee-body probes, and at
/// the same two points.  A map lookup per call; inert unless `option protoorder
/// types` is live and a callee was decompiled first.
///
/// A function never reads its own statement: at its call to itself the
/// statement is what an earlier decompile of this same function said, and a
/// redo exists because that answer changed.
pub fn seed_protoorder_types(arch: &Architecture, data: &mut crate::substrate::funcdata::Funcdata) {
    if arch.kuna_protoorder_types.is_empty() {
        return;
    }
    let own = stated_key(data.get_address());
    let mut entries: Vec<Address> = Vec::new();
    for i in 0..data.num_calls() {
        let e = data.get_call_specs(i).get_entry_address().clone();
        if e.is_invalid() {
            continue;
        }
        entries.push(e);
    }
    for e in entries {
        let Some(key) = stated_key(&e) else { continue };
        if Some(key) == own {
            continue;
        }
        if let Some(stated) = arch.kuna_protoorder_types.get(&key) {
            data.kuna_set_protoorder_types(&e, Rc::clone(stated));
        }
    }
}

#[cfg(test)]
mod tests;
