//! (kuna) A function whose result a caller reads returns it (P4).
//!
//! `call g; ret` is what both `void f(void) { g(); }` and `T f(void) { return
//! g(); }` compile to, and the function alone cannot tell them apart: upstream's
//! `ancestorOpUse` refuses a value that comes straight from a call ("a call is
//! never a good indication of a single parameter"), so every such wrapper was
//! recovered `void`. Its callers settle it. A caller that reads the return
//! register after the call was compiled against a declaration that returns a
//! value, and it printed that read -- `v6 = sub_18a0f(4,v22)` beside `void
//! sub_18a0f(unsigned int a0,char *a1)`, which is not C, and for a float
//! `v1 = (float)qnan()`.
//!
//! In `decompile-all`'s callee-first order the callee is recovered first, so
//! [`record`] files it as `void` and files every later call that reads its
//! return storage. [`due`] then names the functions to decompile again, with
//! the storage their callers read, and `ActionReturnRecovery` accepts the return
//! trial there ([`score_forced`]) when, at every live RETURN, the value is
//! realistic and used only on its way there, a call's result included. It
//! returns no wider than its callers read and than every path sets: a wrapper
//! of two `int` calls returns `eax`, not the `rax` its calls leave half unset.
//! A redone wrapper reads its own callee's result in turn, so the driver repeats
//! until no new function is due, then decompiles again each reader of a
//! function redone after it.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};

use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;

/// What a function's last decompile recovered it returns, when it has no
/// declared prototype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Returns {
    /// Nothing.
    Void,
    /// A float.
    Float,
    /// A value of another type.
    Other,
}

/// How a reader holds a call's result: as a pointer or a float, and whether it
/// adds to it or indexes it in place (`f(a0) + 0x24`, which C scales once `f`
/// returns a pointer; a result first assigned to an integer variable is added
/// to as an integer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Held {
    /// A pointer.
    pub pointer: bool,
    /// A float.
    pub float: bool,
    /// Added to, subtracted from or indexed.
    pub arithmetic: bool,
    /// Used where its sign or width decides the value ([`signed_use`]).
    pub signed: bool,
}

/// How `data` holds the call result `outvn`.
fn held(data: &Funcdata, outvn: crate::context::VarnodeId) -> Option<Held> {
    use kuna_num::opcodes::OpCode;
    let out = data.vbank().get(outvn)?;
    let meta = out.get_type().get_metatype();
    let arithmetic = out.is_implied()
        && out.descend_iter().any(|r| {
            data.obank().get(r).is_some_and(|o| {
                matches!(o.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB)
            })
        });
    Some(Held {
        pointer: meta == crate::dtype::type_metatype::TYPE_PTR,
        float: meta == crate::dtype::type_metatype::TYPE_FLOAT,
        arithmetic,
        signed: signed_use(data, outvn, false).is_some(),
    })
}

/// Does `data` use the call result `outvn` where its sign or width decides the
/// value: an extension, an ordering, a right shift, a division or remainder, a
/// conversion to a float, or an equality with a variable or with a constant
/// whose sign bit is set, after copies, joins and pieces? After arithmetic,
/// which C would carry past the narrow value, any equality or zero-extension
/// does. With
/// `inline`, only within the expression the result is printed in: a value
/// assigned to a variable is converted by the assignment. The answer says
/// whether the use is a zero-extension, which wants the word unsigned.
fn signed_use(data: &Funcdata, outvn: crate::context::VarnodeId, inline: bool) -> Option<bool> {
    use kuna_num::opcodes::OpCode::*;
    let mut work = vec![(outvn, false)];
    let mut seen = BTreeSet::new();
    while let Some((v, carried)) = work.pop() {
        if !seen.insert((v, carried)) || seen.len() > 64 {
            continue;
        }
        let Some(node) = data.vbank().get(v) else { continue };
        if inline && v != outvn && !node.is_implied() {
            continue;
        }
        let sign = 1u64 << (node.get_size().clamp(1, 8) * 8 - 1);
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { continue };
            match o.code() {
                CPUI_INT_SEXT | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL
                | CPUI_INT_RIGHT | CPUI_INT_SRIGHT | CPUI_INT_DIV | CPUI_INT_SDIV | CPUI_INT_REM | CPUI_INT_SREM
                | CPUI_FLOAT_INT2FLOAT => return Some(false),
                CPUI_INT_ZEXT if carried => return Some(true),
                CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL => {
                    let widened = (0..2)
                        .filter_map(|k| o.get_in(k))
                        .filter(|&k| k != v)
                        .filter_map(|k| data.vbank().get(k))
                        .any(|k| carried || !k.is_constant() || k.get_offset() & sign != 0);
                    if widened {
                        return Some(false);
                    }
                }
                CPUI_COPY | CPUI_MULTIEQUAL | CPUI_INDIRECT | CPUI_CAST | CPUI_SUBPIECE | CPUI_PIECE => {
                    work.extend(o.get_out().map(|n| (n, carried)))
                }
                CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_MULT | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR
                | CPUI_INT_NEGATE | CPUI_INT_2COMP | CPUI_INT_LEFT => work.extend(o.get_out().map(|n| (n, true))),
                _ => {}
            }
        }
    }
    None
}

/// The run's record.
#[derive(Debug, Default, Clone)]
pub struct Ledger {
    /// Per called function, the return storage each call to it reads.
    pub read: BTreeMap<(int4, uintb), Vec<(Address, int4)>>,
    /// Per called function, the functions that read it.
    pub readers: BTreeMap<(int4, uintb), BTreeSet<(int4, uintb)>>,
    /// Per function decompiled again, the storage its callers read.
    pub forced: BTreeMap<(int4, uintb), (Address, int4)>,
    /// What each function without a declared prototype was last recovered to
    /// return.
    pub returns: BTreeMap<(int4, uintb), Returns>,
    /// Per function recovered returning a value, the storage it returns it in.
    pub storage: BTreeMap<(int4, uintb), (Address, int4)>,
    /// Per reader and callee, how the reader's last decompile holds the call's
    /// result ([`Held`]).
    pub held: BTreeMap<((int4, uintb), (int4, uintb)), Held>,
    /// Per function, its live p-code ops in its last decompile.
    pub ops: BTreeMap<(int4, uintb), usize>,
    /// Per function returning a float, the readers that keep its result as
    /// another type.
    pub float_refused: BTreeMap<(int4, uintb), BTreeSet<(int4, uintb)>>,
    /// The functions whose float return is withdrawn: redone without the
    /// float-register vote on the return, and without a forced return.
    pub withdrawn: BTreeSet<(int4, uintb)>,
    /// Per function returning a float, the callees whose float return it hands
    /// on as its own ([`float_sources`]).
    pub float_sources: BTreeMap<(int4, uintb), BTreeSet<(int4, uintb)>>,
    /// The functions a forced return left returning a register a call only
    /// clobbers: withdrawn like a refused float return.
    pub unset: BTreeSet<(int4, uintb)>,
    /// Per function without a declared prototype, whether each parameter its
    /// last decompile recovered is a float (`Some(true)`) or an integer or
    /// pointer (`Some(false)`): the type a listing prints for it.
    pub params: BTreeMap<(int4, uintb), Vec<Option<bool>>>,
    /// The functions [`due`] forced to the storage their callers read over a
    /// return recovered in other storage, not yet decompiled again: a float
    /// return they refuse is withdrawn only if the forced decompile keeps it.
    pub displacing: BTreeSet<(int4, uintb)>,
    /// Per called function, the storage of its result each call computes with
    /// ([`used_storage`]), or computes with once a function returning it hands
    /// it back.
    pub used: BTreeMap<(int4, uintb), Vec<(Address, int4)>>,
    /// Per function, the callees whose result it returns as it is ([`hands_on`]).
    pub handed: BTreeMap<(int4, uintb), BTreeSet<(int4, uintb)>>,
    /// Per function whose last decompile returned a zero-extended word trimmed
    /// out of a wider register, the word's storage, and whether the unsigned
    /// vote could change its type: its sign bit may be set and it is typed a
    /// signed or unknown integer.
    pub words: BTreeMap<(int4, uintb), (Address, int4, bool)>,
    /// Per function decompiled again keeping that register whole, the storage
    /// its callers compute with.
    pub wide: BTreeMap<(int4, uintb), (Address, int4)>,
    /// The functions in `words` a caller computed with wider than the word
    /// since [`due`] last ran.
    pub wide_due: BTreeSet<(int4, uintb)>,
}

fn key(entry: &Address) -> Option<(int4, uintb)> {
    Some((entry.get_space()?.get_index(), entry.get_offset()))
}

/// File what the function entered at `entry` returns, and the return storage
/// each of its calls reads. Callers are not always recorded after their callees
/// (a call the call graph missed, a cycle), so every read is filed and [`due`]
/// asks which callee is `void`.
pub fn record(arch: &mut Architecture, entry: &Address, data: &Funcdata) {
    let Some(own) = key(entry) else { return };
    let proto = data.get_func_proto();
    let declared = arch.symboltab.function_proto_pieces_across_scopes(entry).is_some();
    let returns = match proto.get_output_type().map(|t| t.get_metatype()) {
        _ if declared || !proto.has_store() || proto.is_output_locked() => None,
        None | Some(crate::dtype::type_metatype::TYPE_VOID) => proto.has_model().then_some(Returns::Void),
        Some(crate::dtype::type_metatype::TYPE_FLOAT) => Some(Returns::Float),
        Some(_) => Some(Returns::Other),
    };
    match returns {
        Some(r) => {
            arch.kuna_voidret.returns.insert(own, r);
        }
        None => {
            arch.kuna_voidret.returns.remove(&own);
        }
    }
    let stored = matches!(returns, Some(Returns::Float | Returns::Other))
        .then(|| proto.get_output())
        .filter(|out| !out.get_address().is_invalid() && out.get_size() > 0)
        .map(|out| (out.get_address(), out.get_size()));
    let word = stored.as_ref().and_then(|(addr, size)| {
        let &(ref noted, width, sign) = data.kuna_zext_word()?;
        let signed = proto.get_output_type().is_some_and(|t| {
            matches!(t.get_metatype(), crate::dtype::type_metatype::TYPE_INT | crate::dtype::type_metatype::TYPE_UNKNOWN)
        });
        (noted == addr && width == *size).then(|| (addr.clone(), *size, sign && signed))
    });
    match stored {
        Some(s) => {
            arch.kuna_voidret.storage.insert(own, s);
        }
        None => {
            arch.kuna_voidret.storage.remove(&own);
        }
    }
    file_word(arch, own, word);
    if declared || !proto.has_store() || proto.is_input_locked() {
        arch.kuna_voidret.params.remove(&own);
    } else {
        let params = (0..proto.num_params())
            .map(|i| {
                use crate::dtype::type_metatype::*;
                proto.get_param(i).and_then(|p| p.get_type()).and_then(|t| match t.get_metatype() {
                    TYPE_FLOAT => Some(true),
                    TYPE_INT | TYPE_UINT | TYPE_BOOL | TYPE_UNKNOWN | TYPE_PTR | TYPE_ENUM_INT | TYPE_ENUM_UINT => Some(false),
                    _ => None,
                })
            })
            .collect();
        arch.kuna_voidret.params.insert(own, params);
    }
    for refused in arch.kuna_voidret.float_refused.values_mut() {
        refused.remove(&own);
    }
    arch.kuna_voidret.displacing.remove(&own);
    arch.kuna_voidret.ops.insert(own, data.obank().iter_alive().count());
    if returns == Some(Returns::Float) && converts_its_return(data) {
        arch.kuna_voidret.float_refused.entry(own).or_default().insert(own);
    }
    let sources = if returns == Some(Returns::Float) { float_sources(data) } else { BTreeSet::new() };
    if sources.is_empty() {
        arch.kuna_voidret.float_sources.remove(&own);
    } else {
        arch.kuna_voidret.float_sources.insert(own, sources);
    }
    if !data.kuna_forced_return().is_empty() && returns_a_call_clobber(data) {
        arch.kuna_voidret.unset.insert(own);
    }
    let mut handed = BTreeSet::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let Some(callee) = key(fc.get_entry_address()) else { continue };
        if callee == own || fc.proto().is_output_locked() {
            continue;
        }
        let Some(outvn) = data.obank().get(fc.get_op()).filter(|o| !o.is_dead()).and_then(|o| o.get_out()) else {
            continue;
        };
        let result = holder(data, outvn);
        if hands_on(data, result) {
            handed.insert(callee);
        }
        if arch.kuna_voidret.returns.get(&callee) == Some(&Returns::Float)
            && !wider_than_return(arch, callee, data, outvn)
            && !held_as_float(data, outvn, result)
        {
            arch.kuna_voidret.float_refused.entry(callee).or_default().insert(own);
        }
        if let Some(h) = held(data, result) {
            arch.kuna_voidret.held.insert((own, callee), h);
        }
        arch.kuna_voidret.readers.entry(callee).or_default().insert(own);
        if let Some(used) = used_storage(data, result) {
            file_use(arch, callee, used);
        }
        let Some(storage) = data.vbank().get(result).and_then(read_storage) else { continue };
        file_claim(arch, own, callee, storage);
    }
    for (callee, storage) in data.kuna_forced_claims().to_vec() {
        file_claim(arch, own, callee, storage);
    }
    file_handed(arch, own, handed);
}

/// File the callees whose result the function keyed `own` returns as it is,
/// and hand them what its callers already compute with.
fn file_handed(arch: &mut Architecture, own: (int4, uintb), handed: BTreeSet<(int4, uintb)>) {
    if handed.is_empty() {
        arch.kuna_voidret.handed.remove(&own);
        return;
    }
    let uses = arch.kuna_voidret.used.get(&own).cloned().unwrap_or_default();
    for &callee in &handed {
        for storage in &uses {
            file_use(arch, callee, storage.clone());
        }
    }
    arch.kuna_voidret.handed.insert(own, handed);
}

/// The Varnode holding the call result `outvn` in the caller's storage: the
/// call's output, or, where `ActionSetCasts` converted the output and left the
/// call writing a temporary only the conversion reads, the conversion's output,
/// which took the register over.
fn holder(data: &Funcdata, outvn: crate::context::VarnodeId) -> crate::context::VarnodeId {
    let temporary = data
        .vbank()
        .get(outvn)
        .and_then(|n| n.get_addr().get_space())
        .is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL);
    if temporary {
        crate::kuna_callrettype::converted_result(data, outvn)
    } else {
        outvn
    }
}

/// Does the call write more of the register than `callee` returns: a 16-byte
/// `q0` read whole after a call that returns a double in `d0`?  The caller
/// keeps the register, not the callee's value, which says nothing about the
/// type it holds the result as.
fn wider_than_return(arch: &Architecture, callee: (int4, uintb), data: &Funcdata, outvn: crate::context::VarnodeId) -> bool {
    let Some((_, size)) = arch.kuna_voidret.storage.get(&callee) else { return false };
    data.vbank().get(outvn).is_some_and(|v| v.get_size() > *size)
}

/// Does the caller keep the call result `outvn` as a float: typed one, and
/// never converted to anything else?  A reader that holds a float callee's
/// result as an integer (`unsigned int v3 = clampf(..)`), converts it
/// (`(unsigned int)clampf(..)`), or hands it where C converts it -- to a
/// parameter declared or recovered as an integer (`f2u(getf(p))`), stored
/// through an `unsigned int *`, or returned as an integer -- converts it by
/// value, where the binary moved its bits. Where `ActionSetCasts` converted the
/// output ([`holder`]), the reader holds the result as the conversion's output,
/// and the temporary the call writes carries only the call's own type. An
/// untyped value only cast to a float holds one, and a same-width cast to an
/// integer is the float's bits moved to an integer register (`fmov x0,d0`),
/// which the C prints as a reinterpretation, not a conversion.
fn held_as_float(data: &Funcdata, outvn: crate::context::VarnodeId, result: crate::context::VarnodeId) -> bool {
    use kuna_num::opcodes::OpCode;
    let float_ty = |t: &crate::dtype::Datatype| t.get_metatype() == crate::dtype::type_metatype::TYPE_FLOAT;
    let float = |v: crate::context::VarnodeId| data.vbank().get(v).is_some_and(|n| float_ty(&n.get_type()));
    let family = crate::kuna_protoorder::value_family(data, result);
    let reinterpreted = |o: &crate::op::PcodeOp| {
        o.code() == OpCode::CPUI_CAST
            && o.get_in(0).and_then(|i| data.vbank().get(i)).is_some_and(|i| float_ty(&i.get_type()))
            && o.get_out().and_then(|x| data.vbank().get(x)).is_some_and(|x| {
                use crate::dtype::type_metatype::*;
                matches!(x.get_type().get_metatype(), TYPE_INT | TYPE_UINT | TYPE_UNKNOWN)
                    && Some(x.get_size()) == o.get_in(0).and_then(|i| data.vbank().get(i)).map(|i| i.get_size())
            })
    };
    family.iter().filter(|&&v| result == outvn || v != outvn).all(|&v| {
        let Some(node) = data.vbank().get(v) else { return true };
        if node.get_def().and_then(|d| data.obank().get(d)).is_some_and(|o| reinterpreted(o)) {
            return true;
        }
        let untyped_into_float = node.get_type().get_metatype() == crate::dtype::type_metatype::TYPE_UNKNOWN
            && node.descend_iter().all(|r| {
                data.obank().get(r).is_some_and(|o| o.code() == OpCode::CPUI_CAST && o.get_out().is_some_and(float))
            });
        (node.is_constant() || float(v) || untyped_into_float)
            && node.descend_iter().all(|r| {
                let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { return true };
                match o.code() {
                    OpCode::CPUI_CAST => o.get_out().is_some_and(float) || reinterpreted(o),
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => (1..o.num_input())
                        .filter(|&s| o.get_in(s) == Some(v))
                        .all(|s| !crate::kuna_protoorder::reads_other_than_a_float(data, r, s)),
                    OpCode::CPUI_STORE if o.get_in(2) == Some(v) => {
                        let pointee = o.get_in(1).and_then(|p| data.vbank().get(p)).and_then(|p| p.get_type().get_ptr_to());
                        !pointee.is_some_and(|t| {
                            use crate::dtype::type_metatype::*;
                            matches!(t.get_metatype(), TYPE_INT | TYPE_UINT | TYPE_BOOL | TYPE_PTR | TYPE_ENUM_INT | TYPE_ENUM_UINT)
                        })
                    }
                    OpCode::CPUI_RETURN => {
                        let proto = data.get_func_proto();
                        proto.get_output_type().is_some_and(|t| float_ty(t))
                    }
                    _ => true,
                }
            })
    })
}

/// Does a live RETURN of `data` hand back, on some path, what a call left in a
/// register without returning it there: an INDIRECT creation the call's output
/// never replaced?  The function then returns a variable nothing assigns --
/// `f2u(..); return v1;` where the call's argument, joined from two registers,
/// kept its output from being recovered.
fn returns_a_call_clobber(data: &Funcdata) -> bool {
    use kuna_num::opcodes::OpCode;
    let mut work: Vec<crate::context::VarnodeId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|r| data.obank().get(r).filter(|o| !o.is_dead() && o.get_halt_type() == 0))
        .flat_map(|o| (1..o.num_input()).filter_map(|s| o.get_in(s)).collect::<Vec<_>>())
        .collect();
    let mut seen = BTreeSet::new();
    while let Some(v) = work.pop() {
        if !seen.insert(v) || seen.len() > 256 {
            continue;
        }
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else { continue };
        match def.code() {
            OpCode::CPUI_INDIRECT if def.is_indirect_creation() => return true,
            OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_SUBPIECE | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT => work.extend(def.get_in(0)),
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE => work.extend((0..def.num_input()).filter_map(|k| def.get_in(k))),
            _ => {}
        }
    }
    false
}

/// Does a live RETURN of `data` hand back a value converted from another type:
/// `return (float)a0[3];` of an `int *`, the float the return register made the
/// function return arguing with the type its body reads the value as?  A cast
/// of the result of a callee recovered returning a float converts nothing, and
/// neither does a cast of a float's own bits ([`float_bits`]) or of an untyped
/// call result left in a floating register ([`float_register_call_result`]).
fn converts_its_return(data: &Funcdata) -> bool {
    use kuna_num::opcodes::OpCode;
    let mut work: Vec<crate::context::VarnodeId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|r| data.obank().get(r).filter(|o| !o.is_dead() && o.get_halt_type() == 0).and_then(|o| o.get_in(1)))
        .collect();
    let mut seen = BTreeSet::new();
    while let Some(v) = work.pop() {
        if !seen.insert(v) || seen.len() > 64 {
            continue;
        }
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else { continue };
        match def.code() {
            OpCode::CPUI_CAST => {
                let from = def.get_in(0).and_then(|i| data.vbank().get(i));
                if from.is_some_and(|i| {
                    i.get_type().get_metatype() != crate::dtype::type_metatype::TYPE_FLOAT
                        && !float_call_result(data, i)
                        && !float_register_call_result(data, i, v)
                }) && !def.get_in(0).is_some_and(|i| float_bits(data, i))
                {
                    return true;
                }
            }
            OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => work.extend(def.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => work.extend((0..def.num_input()).filter_map(|k| def.get_in(k))),
            _ => {}
        }
    }
    false
}

/// Is `vn` a bit pattern rather than an integer value: built through bit
/// operations from a float's own bits (`copysign`'s `x ^ (x ^ y) & sign`,
/// `nextafter`'s `bits + 1`, `scalbn`'s exponent field `(k + 0x3ff) << 52`
/// beside the mantissa), or from constants alone (a sign chosen between two
/// constants)?  Moved into a floating register (`fmov d0,x0`), that is a float
/// again, which the C prints as a reinterpretation of the bits, not a
/// conversion of a value.
fn float_bits(data: &Funcdata, vn: crate::context::VarnodeId) -> bool {
    let mut leaves = Leaves::default();
    bit_leaves(data, vn, &mut Vec::new(), &mut leaves);
    leaves.complete && (leaves.float || !leaves.integer)
}

/// What the bit operations behind a value start from.
struct Leaves {
    float: bool,
    integer: bool,
    complete: bool,
}

impl Default for Leaves {
    fn default() -> Self {
        Leaves { float: false, integer: false, complete: true }
    }
}

fn bit_leaves(data: &Funcdata, vn: crate::context::VarnodeId, seen: &mut Vec<crate::context::VarnodeId>, leaves: &mut Leaves) {
    use kuna_num::opcodes::OpCode;
    if seen.contains(&vn) {
        return;
    }
    if seen.len() >= 64 {
        leaves.complete = false;
        return;
    }
    seen.push(vn);
    let Some(node) = data.vbank().get(vn) else {
        leaves.complete = false;
        return;
    };
    if node.is_constant() {
        return;
    }
    if node.get_type().get_metatype() == crate::dtype::type_metatype::TYPE_FLOAT {
        leaves.float = true;
        return;
    }
    let def = node.get_def().and_then(|d| data.obank().get(d));
    let inputs = match def.map(|o| o.code()) {
        Some(
            OpCode::CPUI_CAST
            | OpCode::CPUI_COPY
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_MULTIEQUAL,
        ) => def.map(|o| (0..o.num_input()).filter_map(|k| o.get_in(k)).collect::<Vec<_>>()).unwrap_or_default(),
        Some(OpCode::CPUI_INDIRECT) if def.is_some_and(|o| !o.is_indirect_creation()) => {
            def.and_then(|o| o.get_in(0)).into_iter().collect()
        }
        _ => {
            leaves.integer = true;
            return;
        }
    };
    for i in inputs {
        bit_leaves(data, i, seen, leaves);
    }
}

/// Is `node` an untyped call result in a floating register: what the callee
/// left in `d0`, which no integer use gave a type to read it as?  Where the call
/// writes a temporary only the cast reads, the cast's output `held` holds the
/// register.
fn float_register_call_result(data: &Funcdata, node: &crate::varnode::Varnode, held: crate::context::VarnodeId) -> bool {
    use kuna_num::opcodes::OpCode;
    if node.get_type().get_metatype() != crate::dtype::type_metatype::TYPE_UNKNOWN {
        return false;
    }
    let Some(d) = node.get_def() else { return false };
    if !data.obank().get(d).is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)) {
        return false;
    }
    let storage = if node.get_space().get_type() == spacetype::IPTR_INTERNAL {
        match data.vbank().get(held) {
            Some(h) => h,
            None => return false,
        }
    } else {
        node
    };
    let proto = data.get_func_proto();
    let Some(out) = proto.has_model().then(|| proto.model().output_list()).flatten() else { return false };
    out.get_entry().iter().any(|e| {
        e.get_type() == crate::dtype::type_class::TYPECLASS_FLOAT
            && e.justified_contain(storage.get_addr(), storage.get_size()) >= 0
    })
}

/// Is `node` the result of a call whose callee returns a float: a locked float
/// output, or a float return the callee was last recovered with?
fn float_call_result(data: &Funcdata, node: &crate::varnode::Varnode) -> bool {
    use kuna_num::opcodes::OpCode;
    let Some(d) = node.get_def() else { return false };
    if !data.obank().get(d).is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)) {
        return false;
    }
    let Some(fc) = data.get_call_specs_index(d).map(|i| data.get_call_specs(i)) else { return false };
    let proto = fc.proto();
    if proto.is_output_locked() {
        return proto.get_output_type().is_some_and(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_FLOAT);
    }
    key(fc.get_entry_address()).is_some_and(|k| data.kuna_callee_returns(k) == Some(Returns::Float))
}

/// The functions returning a float that a reader keeps as another type, not yet
/// withdrawn: each is decompiled again without the float-register vote on its
/// return and without a forced return, so the listing declares what every
/// reader takes (`unsigned int`, or `void` as before a redo made it return),
/// and the readers are decompiled again against that.
///
/// A function that hands on a callee's float return keeps returning a float
/// once withdrawn, `double wrapd(..) { return getd(..); }` beside a `getd` the
/// vote made return `double`, and its reader's `dat_4060 = wrapd(..)` then
/// converts by value. So the callees whose float it hands on are withdrawn
/// with it, and theirs in turn, down the chain ([`float_sources`]).
pub fn withdrawals(arch: &mut Architecture) -> BTreeSet<(int4, uintb)> {
    let ledger = &mut arch.kuna_voidret;
    let float = |k: &(int4, uintb)| ledger.returns.get(k) == Some(&Returns::Float);
    let refused: Vec<(int4, uintb)> = ledger
        .float_refused
        .iter()
        .filter(|(k, readers)| !readers.is_empty() && float(k) && !ledger.displacing.contains(k))
        .map(|(k, _)| *k)
        .collect();
    let mut out: BTreeSet<(int4, uintb)> =
        refused.iter().chain(ledger.unset.iter()).copied().filter(|k| !ledger.withdrawn.contains(k)).collect();
    let mut work = refused;
    let mut seen = BTreeSet::new();
    while let Some(k) = work.pop() {
        if !seen.insert(k) {
            continue;
        }
        for s in ledger.float_sources.get(&k).into_iter().flatten().filter(|s| float(s)) {
            if !ledger.withdrawn.contains(s) {
                out.insert(*s);
            }
            work.push(*s);
        }
    }
    ledger.withdrawn.extend(out.iter().copied());
    out
}

/// The callees, last recovered returning a float, whose result reaches a live
/// RETURN of `data` through copies and joins: the float the function returns
/// is theirs.
fn float_sources(data: &Funcdata) -> BTreeSet<(int4, uintb)> {
    use kuna_num::opcodes::OpCode;
    let mut work: Vec<crate::context::VarnodeId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|r| data.obank().get(r).filter(|o| !o.is_dead() && o.get_halt_type() == 0))
        .flat_map(|o| (1..o.num_input()).filter_map(|s| o.get_in(s)).collect::<Vec<_>>())
        .collect();
    let mut out = BTreeSet::new();
    let mut seen = BTreeSet::new();
    while let Some(v) = work.pop() {
        if !seen.insert(v) || seen.len() > 256 {
            continue;
        }
        let Some((d, def)) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| Some((d, data.obank().get(d)?)))
        else {
            continue;
        };
        let fc = match def.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => data.get_call_specs_index(d).map(|i| data.get_call_specs(i)),
            OpCode::CPUI_INDIRECT if def.is_indirect_creation() => call_of(data, def),
            OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_CAST | OpCode::CPUI_SUBPIECE => {
                work.extend(def.get_in(0));
                continue;
            }
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE => {
                work.extend((0..def.num_input()).filter_map(|k| def.get_in(k)));
                continue;
            }
            _ => continue,
        };
        let Some(fc) = fc.filter(|fc| !fc.proto().is_output_locked()) else { continue };
        let Some(k) = key(fc.get_entry_address()) else { continue };
        if data.kuna_callee_returns(k) == Some(Returns::Float) {
            out.insert(k);
        }
    }
    out
}

/// File that `reader` reads `storage` of what `callee` returns.
fn file_claim(arch: &mut Architecture, reader: (int4, uintb), callee: (int4, uintb), storage: (Address, int4)) {
    let claims = arch.kuna_voidret.read.entry(callee).or_default();
    if !claims.contains(&storage) {
        claims.push(storage);
    }
    arch.kuna_voidret.readers.entry(callee).or_default().insert(reader);
}

/// File that a call to `callee` computes with `storage` of its result, and so
/// with the result of each callee whose result `callee` returns as it is,
/// making each due when that is wider than a zero-extended word it returns.
fn file_use(arch: &mut Architecture, callee: (int4, uintb), storage: (Address, int4)) {
    let ledger = &mut arch.kuna_voidret;
    let mut work = vec![callee];
    while let Some(k) = work.pop() {
        let uses = ledger.used.entry(k).or_default();
        if uses.contains(&storage) {
            continue;
        }
        uses.push(storage.clone());
        if ledger.words.get(&k).is_some_and(|w| widens(&storage, w)) {
            ledger.wide_due.insert(k);
        }
        work.extend(ledger.handed.get(&k).into_iter().flatten().copied());
    }
}

/// Does the call result `out` reach a live RETURN of `data` as it is, through
/// copies and joins only?
fn hands_on(data: &Funcdata, out: crate::context::VarnodeId) -> bool {
    use kuna_num::opcodes::OpCode::*;
    let mut work = vec![out];
    let mut seen = BTreeSet::new();
    while let Some(v) = work.pop() {
        if !seen.insert(v) || seen.len() > 64 {
            continue;
        }
        let Some(node) = data.vbank().get(v) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { continue };
            match o.code() {
                CPUI_RETURN if o.get_halt_type() == 0 && o.get_in(0) != Some(v) => return true,
                CPUI_COPY | CPUI_MULTIEQUAL | CPUI_INDIRECT | CPUI_CAST => work.extend(o.get_out()),
                _ => {}
            }
        }
    }
    false
}

/// File the zero-extended word the function keyed `own` returns, if its last
/// decompile trimmed one out of a wider register, making the function due when
/// a caller already computes with more of the register.
fn file_word(arch: &mut Architecture, own: (int4, uintb), word: Option<(Address, int4, bool)>) {
    let ledger = &mut arch.kuna_voidret;
    let Some(word) = word else {
        ledger.words.remove(&own);
        return;
    };
    if ledger.used.get(&own).is_some_and(|uses| uses.iter().any(|u| widens(u, &word))) {
        ledger.wide_due.insert(own);
    }
    ledger.words.insert(own, word);
}

/// Would a caller computing with `storage` change what returning `word` is
/// recovered as? It must take the register holding the word and more of it:
/// more than four bytes, which keeps the register whole, or up to four of a
/// word the unsigned vote retypes.
fn widens(storage: &(Address, int4), word: &(Address, int4, bool)) -> bool {
    crate::kuna_zextreturn::wider_over(&storage.0, storage.1, &word.0, word.1) && (storage.1 > 4 || word.2)
}

/// The storage of the call result `out` that an operation of `data` computes
/// with: the bytes, from the register's least significant end, holding the
/// bits its ops use. A RETURN, a call's argument, and a copy, join or piece
/// of the value on its way to one only hand the register on, and count for
/// nothing: `call f; sete %al; ret` leaves `f`'s upper bits in a return the
/// function was not narrowed to, but computes only with `eax`. A shift on the
/// way moves the upper bits where the RETURN or the argument takes them:
/// `return (long)z6(x) >> 1` hands on bit 32 of the result.
fn used_storage(data: &Funcdata, out: crate::context::VarnodeId) -> Option<(Address, int4)> {
    let node = data.vbank().get(out)?;
    let addr = node.get_addr();
    let size = node.get_size();
    if addr.get_space()?.get_type() != spacetype::IPTR_PROCESSOR || !(1..=8).contains(&size) {
        return None;
    }
    let bits = operated_bits(data, out, size);
    if bits == 0 {
        return None;
    }
    let bytes = (64 - bits.leading_zeros() as int4 + 7) / 8;
    let width = (bytes as u32).next_power_of_two().min(size as u32) as int4;
    let low = if addr.is_big_endian() { addr + ((size - width) as i64) } else { addr.clone() };
    Some((low, width))
}

/// The bits of the `size`-byte call result `out` the ops of `data` compute
/// with, following the value through copies, joins, pieces, extensions, shifts
/// and masks by constants, which move bits without computing with them (each
/// Varnode with the shift from its bits to the result's, and the mask of its
/// bits that are the result's).
fn operated_bits(data: &Funcdata, out: crate::context::VarnodeId, size: int4) -> u64 {
    use kuna_base::address::calc_mask;
    use kuna_num::opcodes::OpCode::*;
    let covering = |x: u64| if x == 0 { 0 } else { u64::MAX >> x.leading_zeros() };
    let to_result = |bits: u64, shift: i32| match shift {
        0..=63 => bits << shift,
        -63..=-1 => bits >> -shift,
        _ => 0,
    };
    let mut bits = 0u64;
    let mut work = vec![(out, 0i32, calc_mask(size))];
    let mut seen = BTreeSet::new();
    while let Some((v, shift, valid)) = work.pop() {
        if valid == 0 || !seen.insert((v, shift)) || seen.len() > 64 {
            continue;
        }
        let Some(node) = data.vbank().get(v) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { continue };
            let Some(next) = o.get_out() else {
                if shift != 0 || !matches!(o.code(), CPUI_RETURN | CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER) {
                    bits |= to_result(valid, shift);
                }
                continue;
            };
            let out_size = data.vbank().get(next).map_or(0, |n| n.get_size());
            let constant = |k: int4| {
                o.get_in(k).filter(|&i| i != v).and_then(|i| data.vbank().get(i)).filter(|i| i.is_constant()).map(|i| i.get_offset())
            };
            let used = match o.code() {
                CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER if shift == 0 => continue,
                CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER => valid,
                CPUI_COPY | CPUI_MULTIEQUAL | CPUI_INDIRECT | CPUI_CAST | CPUI_INT_ZEXT => {
                    work.push((next, shift, valid));
                    continue;
                }
                CPUI_SUBPIECE => {
                    let c = o.get_in(1).and_then(|c| data.vbank().get(c)).map_or(0, |c| c.get_offset() as i32 * 8);
                    work.push((next, shift + c, to_result(valid, -c) & calc_mask(out_size)));
                    continue;
                }
                CPUI_PIECE => {
                    let lo = o.get_in(1).and_then(|l| data.vbank().get(l)).map_or(0, |l| l.get_size() * 8);
                    if o.get_in(1) == Some(v) {
                        work.push((next, shift, valid));
                    } else {
                        work.push((next, shift - lo, to_result(valid, lo)));
                    }
                    continue;
                }
                CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT if constant(1).is_some() => {
                    let c = constant(1).unwrap_or(0).min(64) as i32;
                    let c = if o.code() == CPUI_INT_LEFT { -c } else { c };
                    work.push((next, shift + c, to_result(valid, -c) & calc_mask(out_size)));
                    continue;
                }
                CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR if constant(0).or(constant(1)).is_some() => {
                    let c = constant(0).or(constant(1)).unwrap_or(0);
                    let kept = match o.code() {
                        CPUI_INT_AND => c,
                        CPUI_INT_OR => !c,
                        _ => u64::MAX,
                    };
                    work.push((next, shift, valid & kept));
                    continue;
                }
                CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_MULT | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR
                | CPUI_INT_NEGATE | CPUI_INT_2COMP | CPUI_INT_LEFT => {
                    covering(data.vbank().get(next).map_or(0, |n| n.get_consume()))
                }
                _ => u64::MAX,
            };
            bits |= to_result(used & valid, shift);
        }
    }
    bits & calc_mask(size)
}

/// The storage a caller reads of the call result `out`: the bytes its uses
/// consume (a `movss` of an `xmm0` result reads four), at the least
/// significant end of the register.
fn read_storage(out: &crate::varnode::Varnode) -> Option<(Address, int4)> {
    let addr = out.get_addr();
    let size = out.get_size();
    match addr.get_space()?.get_type() {
        spacetype::IPTR_JOIN => return Some((addr.clone(), size)),
        spacetype::IPTR_PROCESSOR => {}
        _ => return None,
    }
    let consume = out.get_consume();
    let bits = 64 - consume.leading_zeros() as int4;
    let width = if consume == 0 || (size > 8 && consume == u64::MAX) {
        size
    } else {
        (((bits + 7) / 8).max(1) as u32).next_power_of_two().min(size as u32) as int4
    };
    let low = if addr.is_big_endian() { addr + ((size - width) as i64) } else { addr.clone() };
    Some((low, width))
}

/// The functions to decompile again: `void` ones some caller reads a result
/// from, and ones returning in storage their callers do not read
/// ([`displaced`]), each forced to return in the widest storage its callers
/// read, and forced ones a caller decompiled since reads wider (the first
/// reader decided the width, and the driver settles after every function). Callers that
/// disagree on the register refuse the function, and one already forced is
/// withdrawn: it returns nothing again, as before the redo. Also each function
/// returning a zero-extended word a caller reads wider, kept whole at the
/// widest such read ([`crate::kuna_zextreturn::zero_extended_word`]).
pub fn due(arch: &mut Architecture) -> BTreeSet<(int4, uintb)> {
    let ledger = &mut arch.kuna_voidret;
    let mut force: Vec<((int4, uintb), (Address, int4))> = Vec::new();
    let mut withdraw: Vec<(int4, uintb)> = Vec::new();
    let mut displace: Vec<(int4, uintb)> = Vec::new();
    for (callee, claims) in &ledger.read {
        let Some(first) = claims.first() else { continue };
        let space = |a: &Address| a.get_space().map(|s| s.get_index());
        let agree = claims.iter().all(|(a, _)| space(a) == space(&first.0) && a.get_offset() == first.0.get_offset());
        let widest = claims.iter().max_by_key(|(_, s)| *s).cloned().unwrap_or_else(|| first.clone());
        match ledger.forced.get(callee) {
            None if agree && ledger.returns.get(callee) == Some(&Returns::Void) => force.push((*callee, widest)),
            None if agree && displaced(ledger, callee, &widest) => {
                displace.push(*callee);
                force.push((*callee, widest));
            }
            Some(_) if ledger.withdrawn.contains(callee) => {}
            Some(_) if !agree => withdraw.push(*callee),
            Some((_, size)) if widest.1 > *size => force.push((*callee, widest)),
            _ => {}
        }
    }
    ledger.displacing.extend(displace);
    let mut out = BTreeSet::new();
    for (callee, storage) in force {
        ledger.forced.insert(callee, storage);
        out.insert(callee);
    }
    for callee in withdraw {
        ledger.withdrawn.insert(callee);
        out.insert(callee);
    }
    for callee in std::mem::take(&mut ledger.wide_due) {
        let Some(word) = ledger.words.get(&callee) else { continue };
        let widest = ledger.used.get(&callee).and_then(|u| u.iter().filter(|u| widens(u, word)).max_by_key(|u| u.1));
        let Some(read) = widest.cloned() else { continue };
        if ledger.wide.get(&callee).is_some_and(|w| w.1 >= read.1) {
            continue;
        }
        ledger.wide.insert(callee, read);
        out.insert(callee);
    }
    out
}

/// Does `callee` return a value in storage that shares no byte with `read`,
/// what its callers read after the call?  gcc's `-fzero-call-used-regs` ends a
/// function `int put(..)` by zeroing every call-clobbered register but `eax`,
/// so `pxor %xmm0,%xmm0` made `put` return the constant `0.0` its callers never
/// read; to them it returns nothing, and it is forced like a `void` one.
fn displaced(ledger: &Ledger, callee: &(int4, uintb), read: &(Address, int4)) -> bool {
    let register = |a: &Address| a.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR);
    !ledger.withdrawn.contains(callee)
        && register(&read.0)
        && matches!(ledger.returns.get(callee), Some(Returns::Float | Returns::Other))
        && ledger
            .storage
            .get(callee)
            .is_some_and(|(addr, size)| register(addr) && !overlaps(addr, *size, &read.0, read.1))
}

/// The functions that read a function whose return a redo changed after they
/// were decompiled, given when each function was last decompiled (`stamp_of`)
/// and when each changed function's return last changed (`changed`): the type
/// that callee now states reaches them only in another decompile.  Such a
/// reader typed the result itself, against a callee it saw return nothing, and
/// its text no longer agrees with the callee's declaration: `unsigned long v10
/// = sub_18a0f(4,a1)` beside `void *sub_18a0f(..)`, or `sub_ef32(a0) + 0x24`,
/// which C scales by the pointee once `sub_ef32` returns a `struct_56 *`.  A
/// callee that states nothing to `callrettype` (and returns no float, and had
/// no float return withdrawn) has nothing to hand its readers, and only a
/// reader still waiting on it (a wrapper forced to return what it returns,
/// recovered `void` until it did) is decompiled again.
///
/// A function whose zero-extended return a redo widened has every such reader
/// decompiled again, whatever it states: one that took the result as the
/// narrow value (`z32m(..) < 0` of an `int`) tests a sign the callee's new
/// declaration no longer has. A reader over the size cap is redone only where
/// it uses the result's sign or width ([`signed_use`]); e2fsck's `main`, which
/// tests one such result for zero and passes it on, cost a 3-second redo for
/// a renamed variable.
///
/// A reader over [`crate::kuna_callrettype::AUDIT_MAX_OPS`] live ops is decompiled
/// again only where its text computes a wrong value: an offset from a result the
/// callee now declares a pointer to something wider than a byte (which C
/// scales) and the reader held as something else, a float result held as
/// something else, or a float it held from a callee whose float return was
/// withdrawn. Its other stale text is a conversion the listing leaves out; a
/// second decompile of sort -O2's `main` alone cost ten seconds, two thirds of
/// the first.
pub fn stale_readers(
    arch: &Architecture,
    stamp_of: &BTreeMap<(int4, uintb), usize>,
    changed: &BTreeMap<(int4, uintb), usize>,
) -> BTreeSet<(int4, uintb)> {
    let mut out = BTreeSet::new();
    for (callee, &at) in changed {
        let float = arch.kuna_voidret.returns.get(callee) == Some(&Returns::Float);
        let withdrawn = arch.kuna_voidret.withdrawn.contains(callee);
        let widened = arch.kuna_voidret.wide.contains_key(callee);
        let states = float || withdrawn || arch.kuna_callret_types.contains_key(callee);
        let scaled = arch.kuna_callret_types.get(callee).is_some_and(|s| {
            s.ct.get_metatype() == crate::dtype::type_metatype::TYPE_PTR
                && s.ct.get_ptr_to().is_some_and(|p| {
                    p.get_size() > 1 && p.get_metatype() != crate::dtype::type_metatype::TYPE_VOID
                })
        });
        for reader in arch.kuna_voidret.readers.get(callee).into_iter().flatten() {
            if reader == callee || stamp_of.get(reader).is_none_or(|&s| s >= at) {
                continue;
            }
            let waiting = arch.kuna_voidret.forced.contains_key(reader)
                && arch.kuna_voidret.returns.get(reader) == Some(&Returns::Void);
            if !states && !waiting && !widened {
                continue;
            }
            let large = arch.kuna_voidret.ops.get(reader).is_some_and(|&n| n > crate::kuna_callrettype::AUDIT_MAX_OPS);
            let wrong = arch
                .kuna_voidret
                .held
                .get(&(*reader, *callee))
                .is_some_and(|h| {
                    (scaled && !h.pointer && h.arithmetic)
                        || (float && !h.float)
                        || (withdrawn && h.float)
                        || (widened && h.signed)
                });
            if !large || wrong {
                out.insert(*reader);
            }
        }
    }
    out
}

/// The type the callee of the call `op` was last recovered to return, when it
/// returns more of the register than the call's output holds and the output is
/// printed in place. C computes `f(..)` at the callee's declared width, so the
/// narrow value needs its truncation spelled where the expression it is printed
/// in uses its sign or width ([`signed_use`]). Beside `unsigned long z32m(..)`,
/// `a1 == z32m(a0)` compares a sign-extended `a1` with the zero-extended
/// result where the binary compares `eax`. A value tested for zero, or assigned
/// to a variable, which the assignment converts, keeps the call as it is. The
/// flag asks for the word unsigned: a sum C must wrap at 32 bits before it is
/// zero-extended, `(unsigned int)z32m(a0) + 1 == a1`.
pub fn narrowed_call_result(
    data: &Funcdata,
    op: crate::context::OpId,
) -> Option<(std::rc::Rc<crate::dtype::Datatype>, bool)> {
    let o = data.obank().get(op)?;
    if !matches!(o.code(), kuna_num::opcodes::OpCode::CPUI_CALL | kuna_num::opcodes::OpCode::CPUI_CALLIND) {
        return None;
    }
    let out = data.vbank().get(o.get_out()?)?;
    if !out.is_implied() || out.is_type_lock() {
        return None;
    }
    let fc = data.get_call_specs(data.get_call_specs_index(op)?);
    if fc.proto().is_output_locked() {
        return None;
    }
    let k = key(fc.get_entry_address())?;
    if data.kuna_callee_returns(k) != Some(Returns::Other) {
        return None;
    }
    let (addr, size) = data.kuna_callee_return_storage(k)?.clone();
    let register = addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR);
    if !register || size > 8 || !crate::kuna_zextreturn::wider_over(&addr, size, out.get_addr(), out.get_size()) {
        return None;
    }
    let unsigned = signed_use(data, o.get_out()?, true)?;
    let stated = data.kuna_callret_stated(k).filter(|s| s.size == size && s.ct.get_size() == size);
    let wide = match stated {
        Some(s) => std::rc::Rc::clone(&s.ct),
        None => data.get_arch().types()?.get_base(size, crate::dtype::type_metatype::TYPE_UINT).ok()?,
    };
    Some((wide, unsigned))
}

/// What the function keyed `k` was last recovered to return, for [`restore`].
pub fn returns(arch: &Architecture, k: (int4, uintb)) -> Option<Returns> {
    arch.kuna_voidret.returns.get(&k).copied()
}

/// Was the function keyed `k` decompiled again for a caller computing with
/// more of its register than the zero-extended value it returned?
pub fn widened(arch: &Architecture, k: (int4, uintb)) -> bool {
    arch.kuna_voidret.wide.contains_key(&k)
}

/// The storage the function keyed `k` last returned a value in, for [`restore`].
pub fn return_storage(arch: &Architecture, k: (int4, uintb)) -> Option<(Address, int4)> {
    arch.kuna_voidret.storage.get(&k).cloned()
}

/// Put back what the function keyed `k` returned, and in what storage, before
/// a decompile the run then discarded. A forced decompile that displaced its
/// return is over either way, so a float return its callers refuse is withdrawn
/// as before.
pub fn restore(arch: &mut Architecture, k: (int4, uintb), returns: Option<Returns>, storage: Option<(Address, int4)>) {
    arch.kuna_voidret.displacing.remove(&k);
    match returns {
        Some(r) => {
            arch.kuna_voidret.returns.insert(k, r);
        }
        None => {
            arch.kuna_voidret.returns.remove(&k);
        }
    }
    match storage {
        Some(s) => {
            arch.kuna_voidret.storage.insert(k, s);
        }
        None => {
            arch.kuna_voidret.storage.remove(&k);
        }
    }
}

/// Copy onto `data` what each of its callees was last recovered to return, and
/// the return storage its callers read when it is due: the register pieces of a
/// joined return (a `struct timespec` in `rax:rdx`), or the one register.
pub fn seed(arch: &Architecture, data: &mut Funcdata) {
    let returns: BTreeMap<(int4, uintb), Returns> = (0..data.num_calls())
        .filter_map(|i| key(data.get_call_specs(i).get_entry_address()))
        .filter_map(|k| Some((k, *arch.kuna_voidret.returns.get(&k)?)))
        .collect();
    data.kuna_set_callee_returns(returns);
    let params: BTreeMap<(int4, uintb), Vec<Option<bool>>> = (0..data.num_calls())
        .filter_map(|i| key(data.get_call_specs(i).get_entry_address()))
        .filter_map(|k| Some((k, arch.kuna_voidret.params.get(&k)?.clone())))
        .collect();
    data.kuna_set_callee_params(params);
    let storage: BTreeMap<(int4, uintb), (Address, int4)> = (0..data.num_calls())
        .filter_map(|i| key(data.get_call_specs(i).get_entry_address()))
        .filter_map(|k| Some((k, arch.kuna_voidret.storage.get(&k)?.clone())))
        .collect();
    data.kuna_set_callee_return_storage(storage);
    let own = key(data.get_address());
    data.kuna_set_wide_return(own.and_then(|k| arch.kuna_voidret.wide.get(&k).cloned()));
    let withdrawn = own.is_some_and(|k| arch.kuna_voidret.withdrawn.contains(&k));
    data.kuna_set_float_return_withdrawn(withdrawn);
    let Some((addr, size)) = own.filter(|_| !withdrawn).and_then(|k| arch.kuna_voidret.forced.get(&k).cloned()) else {
        data.kuna_set_forced_return(Vec::new());
        return;
    };
    let joined = addr.get_space().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_JOIN);
    let pieces = if !joined {
        vec![(addr, size)]
    } else {
        match arch.manage().find_join(addr.get_offset()) {
            Ok(join) => (0..join.num_pieces())
                .filter_map(|i| {
                    let p = join.get_piece(i);
                    Some((Address::new(std::rc::Rc::clone(p.space.as_ref()?), p.offset), p.size as int4))
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    };
    data.kuna_set_forced_return(pieces);
}

/// Give every live RETURN a read of the storage the function's callers read,
/// and the function a return trial there, when no op of the function names it:
/// heritage registers a return trial only for a range some op reads or writes,
/// and `call g; ret` names no return register at all.
pub fn plant(data: &mut Funcdata) {
    if data.get_func_proto().is_output_locked() || data.get_active_output().is_none() {
        return;
    }
    for (addr, size) in data.kuna_forced_return().to_vec() {
        plant_piece(data, addr, size);
    }
}

pub(crate) fn plant_piece(data: &mut Funcdata, addr: Address, size: int4) {
    if crate::p4_calls::kuna_passthrough::suppresses_return_trial(data, &addr, size) || touched_beyond(data, &addr, size) {
        return;
    }
    let rets: Vec<crate::context::OpId> = data
        .obank()
        .iter_code(kuna_num::opcodes::OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    let nins: Vec<int4> = rets.iter().filter_map(|&r| data.obank().get(r).map(|o| o.num_input())).collect();
    if rets.is_empty() || nins.len() != rets.len() || nins.iter().any(|&n| n != nins[0]) {
        return;
    }
    let slot = nins[0];
    for &r in &rets {
        let vn = data.new_varnode(size, &addr, None);
        if data.op_insert_input(r, vn, slot).is_err() {
            return;
        }
    }
    if let Some(active) = data.get_active_output_mut() {
        active.register_trial(&addr, size);
        let t = active.get_num_trials() - 1;
        active.get_trial_mut(t).set_slot(slot);
    }
    data.kuna_note_forced_return_planted(addr, size);
}

/// Does a Varnode of the function share a byte with `[addr, addr+size)` and
/// reach outside it?  Heritage then registers a return trial on the wider
/// range, which covers the callers' read.  One the function names only inside
/// it (the `eax` of an earlier call's result beside a `rax` its callers read)
/// would make the trial as narrow as that name.
fn touched_beyond(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(space) = addr.get_space() else { return true };
    let off = addr.get_offset();
    let end = off.wrapping_add(size as u64);
    let lo = Address::new(std::rc::Rc::clone(space), off.saturating_sub(64));
    let hi = addr + size as i64;
    data.vbank().iter_loc_addr_range(&lo, &hi).any(|id| {
        data.vbank().get(id).is_some_and(|v| {
            let (voff, vend) = (v.get_offset(), v.get_offset().wrapping_add(v.get_size() as u64));
            voff < end && off < vend && (voff < off || vend > end)
        })
    })
}

/// Must heritage leave `[addr, addr+size)` out of the function's RETURN trials
/// because [`plant`] made the trial itself?  Only the pieces it planted: the
/// `edx` of `call zsum; or $0xff,%edx` beside a planted `eax` keeps its trial.
pub fn planted_overlaps(data: &Funcdata, addr: &Address, size: int4) -> bool {
    data.kuna_forced_return_planted().iter().any(|(paddr, psize)| overlaps(addr, size, paddr, *psize))
}

fn overlaps(addr: &Address, size: int4, other: &Address, osize: int4) -> bool {
    let (Some(a), Some(b)) = (addr.get_space(), other.get_space()) else { return false };
    let (off, ooff) = (addr.get_offset(), other.get_offset());
    a.get_index() == b.get_index()
        && off < ooff.wrapping_add(osize.max(0) as u64)
        && ooff < off.wrapping_add(size.max(0) as u64)
}

/// Mark active each return trial on the storage the function's callers read
/// whose value is the return value at every live RETURN: realistic, and used
/// only on its way there (`ancestor_op_use`), where a call's result counts too
/// (upstream refuses one outright, which is what left `call g; ret` void). A
/// register the function also uses as scratch (a stream pointer in a `getc`
/// loop, an `error` message on a path that never returns) is not the return
/// value everywhere, and the function stays as it was.
///
/// The value is returned no wider than the callers read it and than every path
/// sets it ([`defined_width`]): `if (tz) return setenv(..); return unsetenv(..);`
/// sets `eax` and leaves the rest of `rax` to the calls, so it returns an `int`
/// ([`narrow`]); a path that sets none of it refuses the trial.
pub fn score_forced(
    data: &mut Funcdata,
    active: &mut crate::fspec::ParamActive,
    return_ops: &[crate::context::OpId],
    maxancestor: int4,
) {
    if data.kuna_forced_return().is_empty() {
        return;
    }
    let live: Vec<crate::context::OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    for anchored in [true, false] {
        let beside = (0..active.get_num_trials()).any(|i| {
            let t = active.get_trial(i);
            t.is_active() && read_width(data, t.get_address(), t.get_size()).is_some()
        });
        if !anchored && !beside {
            break;
        }
        for i in 0..active.get_num_trials() {
            score_trial(data, active, i, anchored, &live, maxancestor);
        }
    }
}

/// After the model has mapped the return trials, keep a forced return whole or
/// return nothing: a trial on the callers' storage that every path sets but
/// the model left out (the `edx` of `call big; ret`, which upstream takes for a
/// clobbered register) would leave its callers reading a value the function
/// no longer returns.
pub fn whole_or_none(data: &Funcdata, active: &mut crate::fspec::ParamActive) {
    if data.kuna_forced_return().is_empty() {
        return;
    }
    let live: Vec<crate::context::OpId> = data
        .obank()
        .iter_code(kuna_num::opcodes::OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    let trials: Vec<int4> = (0..active.get_num_trials())
        .filter(|&i| {
            let t = active.get_trial(i);
            forced(data, t.get_address(), t.get_size())
        })
        .collect();
    let set = |slot: int4| {
        slot >= 0
            && !live.is_empty()
            && live.iter().all(|&r| {
                data.obank()
                    .get(r)
                    .and_then(|o| o.get_in(slot))
                    .is_some_and(|vn| defined_width(data, vn, &mut BTreeSet::new()) > 0)
            })
    };
    let left = trials.iter().any(|&i| !active.get_trial(i).is_used() && set(active.get_trial(i).get_slot()));
    if left && trials.iter().any(|&i| active.get_trial(i).is_used()) {
        for &i in &trials {
            active.get_trial_mut(i).mark_no_use();
        }
    }
}

/// A forced function whose RETURN carries no value once return recovery is
/// done returns `void`, as upstream's `updateOutputTypes` makes it. A restart
/// keeps the output an earlier pass recovered (the port's `Funcdata::clear`
/// does not clear an unlocked output, and `ActionOutputPrototype` leaves the
/// output alone when the RETURN has no value), so a trial the pass before the
/// restart took and the pass after it refused printed `unsigned int f(..)`
/// around a bare `return;`.
pub fn void_unless_returned(data: &mut Funcdata) {
    if data.kuna_forced_return().is_empty()
        || !data.get_func_proto().has_store()
        || data.get_func_proto().is_output_locked()
        || data.get_first_return_op().and_then(|r| data.obank().get(r)).is_some_and(|o| o.num_input() > 1)
    {
        return;
    }
    let glb = std::rc::Rc::clone(data.get_arch());
    if let Some(types) = glb.types() {
        let _ = data.get_func_proto_mut().clear_unlocked_output(types);
    }
}

/// [`score_forced`] for trial `i`, when it is (`anchored`) or is not a trial at
/// the least significant end of the storage the callers read: the upper half of
/// a `double` split in two trials is returned only beside its lower half.
fn score_trial(
    data: &mut Funcdata,
    active: &mut crate::fspec::ParamActive,
    i: int4,
    anchored: bool,
    live: &[crate::context::OpId],
    maxancestor: int4,
) -> bool {
    let (addr, size, slot) = {
        let t = active.get_trial(i);
        (t.get_address().clone(), t.get_size(), t.get_slot())
    };
    if active.get_trial(i).is_active()
        || !forced(data, &addr, size)
        || live.is_empty()
        || read_width(data, &addr, size).is_some() != anchored
    {
        return false;
    }
    let mut width = read_width(data, &addr, size).unwrap_or(size);
    for &r in live {
        let set = data
            .obank()
            .get(r)
            .and_then(|o| o.get_in(slot))
            .map_or(0, |vn| defined_width(data, vn, &mut BTreeSet::new()));
        width = width.min(set);
    }
    if width <= 0 {
        let callers = callers_storage(data, &addr, size);
        for &r in live {
            if let Some(vn) = data.obank().get(r).and_then(|o| o.get_in(slot)) {
                for (callee, storage) in void_results(data, vn) {
                    data.kuna_note_forced_claim((callee, as_wide_as(storage, callers.as_ref())));
                }
            }
        }
        return false;
    }
    if hands_on_unstated(data, &addr, size, width, live, slot) || merges_in_pieces(data, live, slot) {
        return false;
    }
    let every = live.iter().all(|&r| {
        let Some(vn) = data.obank().get(r).and_then(|o| o.get_in(slot)) else { return false };
        let (killed, cond) = (active.get_trial(i).is_killed_by_call(), active.get_trial(i).has_cond_exe_effect());
        let mut ancestor = crate::funcdata_varnode::AncestorRealistic::new();
        let (realistic, solid) = ancestor.execute(data, r, slot, size, cond, killed, false);
        if !(realistic || solid) {
            return false;
        }
        data.kuna_set_forced_scoring(true);
        let only = data.ancestor_op_use(maxancestor, vn, r, active.get_trial_mut(i), 0, 0);
        data.kuna_set_forced_scoring(false);
        only
    });
    if !every || (width < size && !narrow(data, active, i, live, slot, width)) {
        return false;
    }
    active.get_trial_mut(i).mark_active();
    true
}

/// Does a live RETURN read the trial as pieces that each merge on their own?
/// Heritage refines a planted range wider than every write at the narrower
/// name the function uses, and where paths meet before the RETURN each piece
/// gets its own MULTIEQUAL, which no rule joins again: gcc -O1's `wrap`, `call
/// pick; test %eax,%eax` and then one of two `getname` calls before a shared
/// `ret`, returned `CONCAT44(dat_4,v2)` and stored the upper half of
/// `getname`'s pointer into `dat_4`, a register piece printed as a global, on
/// each path.  The function stays as it was.
fn merges_in_pieces(data: &Funcdata, live: &[crate::context::OpId], slot: int4) -> bool {
    use kuna_num::opcodes::OpCode;
    let def = |vn: crate::context::VarnodeId| data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    live.iter().any(|&r| {
        data.obank().get(r).and_then(|o| o.get_in(slot)).and_then(def).is_some_and(|p| {
            p.code() == OpCode::CPUI_PIECE
                && (0..p.num_input()).filter_map(|k| p.get_in(k)).any(|i| def(i).is_some_and(|d| d.code() == OpCode::CPUI_MULTIEQUAL))
        })
    })
}

/// What a call's callee is known to return, for [`hands_on_unstated`].
enum CalleeReturn {
    /// Its declared, stated or last recovered return storage (nothing, for
    /// `void`).
    Stored(Address, int4),
    /// A float, of a width nothing states.
    Float,
    /// A value of another type, of a width nothing states.
    Other,
    /// Nothing: no declaration, no statement, and no decompile of its own.
    Unknown,
}

fn callee_return(data: &Funcdata, fc: &crate::p4_calls::fspec::FuncCallSpecs) -> CalleeReturn {
    let proto = fc.proto();
    if proto.is_output_locked() {
        let out = proto.get_output();
        if out.get_type().is_some_and(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_VOID) {
            return CalleeReturn::Stored(out.get_address(), 0);
        }
        return CalleeReturn::Stored(out.get_address(), out.get_size());
    }
    let Some(k) = key(fc.get_entry_address()) else { return CalleeReturn::Unknown };
    if let Some(stated) = data.kuna_callret_stated(k) {
        return CalleeReturn::Stored(stated.addr.clone(), stated.size);
    }
    match (data.kuna_callee_returns(k), data.kuna_callee_return_storage(k)) {
        (Some(Returns::Void), _) => CalleeReturn::Stored(Address::new_invalid(), 0),
        (Some(_), Some((addr, size))) => CalleeReturn::Stored(addr.clone(), *size),
        (Some(Returns::Float), None) => CalleeReturn::Float,
        (Some(Returns::Other), None) => CalleeReturn::Other,
        (None, _) => CalleeReturn::Unknown,
    }
}

/// Does trial `[addr, addr+size)`, about to return its least significant
/// `width` bytes, hand on a call's result the listing cannot return as it is?
/// Where the callers read wider than `width`, a path returning what a callee
/// returns wider, or of a width nothing states, would be cut short:
/// `call key_type; call type_name; ret` names only `eax`, and printed
/// `unsigned int` around `type_name`'s pointer.  Where the storage may carry
/// a float (a floating-point register, or any register on an image that does
/// not state a hard-float convention), a path returning what a named callee
/// nothing declares returns converts it by value once the listing is compiled
/// against the callee's real declaration: a relocatable object's `strtod`
/// printed `unsigned long parse(..) { return strtod(..); }`.  A call through a
/// pointer is typed by the listing itself.  Either keeps the function as it
/// was.
fn hands_on_unstated(
    data: &Funcdata,
    addr: &Address,
    size: int4,
    width: int4,
    live: &[crate::context::OpId],
    slot: int4,
) -> bool {
    let cut = callers_width(data, addr, size) > width;
    let float = may_carry_a_float(data, addr, size);
    if !cut && !float {
        return false;
    }
    let low = if addr.is_big_endian() { addr + (size - 1) as i64 } else { addr.clone() };
    live.iter().any(|&r| {
        let Some(vn) = data.obank().get(r).and_then(|o| o.get_in(slot)) else { return true };
        let Some(calls) = result_calls(data, vn) else { return true };
        calls.into_iter().filter_map(|c| data.get_call_specs_index(c).map(|i| data.get_call_specs(i))).any(|fc| {
            match callee_return(data, fc) {
                CalleeReturn::Stored(raddr, rsize) => cut && covered_from(&low, &raddr, rsize).is_none_or(|n| n > width),
                CalleeReturn::Float | CalleeReturn::Other => cut,
                CalleeReturn::Unknown => cut || (float && key(fc.get_entry_address()).is_some()),
            }
        })
    })
}

/// How many bytes of storage `[raddr, raddr+rsize)` hold from the byte `low`
/// up (down, big-endian), or `None` when they are in another space.
fn covered_from(low: &Address, raddr: &Address, rsize: int4) -> Option<int4> {
    if rsize <= 0 {
        return Some(0);
    }
    if low.get_space().map(|s| s.get_index()) != raddr.get_space().map(|s| s.get_index()) {
        return None;
    }
    let (off, roff, rend) = (low.get_offset(), raddr.get_offset(), raddr.get_offset().wrapping_add(rsize as u64));
    if off < roff || off >= rend {
        return Some(0);
    }
    Some(if low.is_big_endian() { off + 1 - roff } else { rend - off } as int4)
}

/// The widest storage the callers read anchored at the trial's least
/// significant end.
fn callers_storage(data: &Funcdata, addr: &Address, size: int4) -> Option<(Address, int4)> {
    data.kuna_forced_return()
        .iter()
        .filter(|(fa, fs)| same_low_end(fa, *fs, addr, size))
        .max_by_key(|(_, fs)| *fs)
        .cloned()
}

fn callers_width(data: &Funcdata, addr: &Address, size: int4) -> int4 {
    callers_storage(data, addr, size).map_or(0, |(_, fs)| fs)
}

fn same_low_end(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    let low_end = |x: &Address, s: int4| if x.is_big_endian() { x.get_offset().wrapping_add(s as u64) } else { x.get_offset() };
    a.get_space().map(|s| s.get_index()) == b.get_space().map(|s| s.get_index()) && low_end(a, asize) == low_end(b, bsize)
}

/// The storage a wrapper asks a `void` callee to return in: all of what its
/// own callers read, where the piece it hands on lies inside that.  A wrapper
/// whose body names only `eax` asked for `eax` alone, and a callee handing on a
/// pointer then returned `unsigned int`; one that hands on `rax` in two pieces
/// asked for each, and the callee, asked for two registers, stayed `void`.
fn as_wide_as(storage: (Address, int4), callers: Option<&(Address, int4)>) -> (Address, int4) {
    match callers {
        Some((fa, fs)) if *fs > storage.1 && within(&storage.0, storage.1, fa, *fs) => (fa.clone(), *fs),
        _ => storage,
    }
}

fn within(addr: &Address, size: int4, outer: &Address, osize: int4) -> bool {
    addr.get_space().map(|s| s.get_index()) == outer.get_space().map(|s| s.get_index())
        && outer.get_offset() <= addr.get_offset()
        && addr.get_offset().wrapping_add(size.max(0) as u64) <= outer.get_offset().wrapping_add(osize.max(0) as u64)
}

/// Can `[addr, addr+size)` hold a float a callee returns: a floating-point
/// register of the model's outputs, or any storage where the image does not
/// state that floats travel in floating-point registers?
fn may_carry_a_float(data: &Funcdata, addr: &Address, size: int4) -> bool {
    if data.get_arch().float_arg_registers != Some(true) {
        return true;
    }
    let proto = data.get_func_proto();
    proto.has_model()
        && proto.model().output().get_entry().iter().any(|e| {
            e.get_type() == crate::dtype::type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) >= 0
        })
}

/// The calls whose result the value `vn` is on some path into it, through
/// copies, joins and pieces, or `None` when the paths are too many to follow.
/// A call an INDIRECT guards may replace the value it carries past the call,
/// and a piece of a register the function returns whole keeps the rest of the
/// call's result beside it (`s0` of a `d0` a call set).
fn result_calls(data: &Funcdata, vn: crate::context::VarnodeId) -> Option<Vec<crate::context::OpId>> {
    use kuna_num::opcodes::OpCode;
    let mut out = Vec::new();
    let mut stack = vec![vn];
    let mut seen = BTreeSet::new();
    let guarded = |ind: &crate::op::PcodeOp| {
        ind.get_in(1)
            .and_then(|i| data.vbank().get(i))
            .filter(|i| i.get_space().get_type() == spacetype::IPTR_IOP)
            .map(|i| crate::context::OpId::from(slotmap::KeyData::from_ffi(i.get_offset())))
            .filter(|&c| data.obank().get(c).is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)))
    };
    while let Some(v) = stack.pop() {
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > 256 {
            return None;
        }
        let Some((d, def)) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| Some((d, data.obank().get(d)?)))
        else {
            continue;
        };
        match def.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => out.push(d),
            OpCode::CPUI_INDIRECT => {
                out.extend(guarded(def));
                if !def.is_indirect_creation() {
                    stack.extend(def.get_in(0));
                }
            }
            OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE => stack.extend(def.get_in(0)),
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE => {
                stack.extend((0..def.num_input()).filter_map(|k| def.get_in(k)))
            }
            _ => {}
        }
    }
    Some(out)
}

/// How many of the trial's least significant bytes the callers read: the
/// widest read anchored at that end, if any is.
fn read_width(data: &Funcdata, addr: &Address, size: int4) -> Option<int4> {
    callers_storage(data, addr, size).map(|(_, fs)| fs.min(size))
}

/// How many of `vn`'s least significant bytes hold a value on every path into
/// it.  None of a register a call kills (an INDIRECT creation on an
/// indirect-zero) and none of the function's entry value of a register no
/// parameter arrives in; a `PIECE` holds its low piece, and its high one too
/// when the low piece is whole.  A value met again around a loop is taken as
/// set, the answer the first path decides.
fn defined_width(data: &Funcdata, vn: crate::context::VarnodeId, seen: &mut BTreeSet<crate::context::VarnodeId>) -> int4 {
    use kuna_num::opcodes::OpCode;
    let Some(node) = data.vbank().get(vn) else { return 0 };
    let size = node.get_size();
    if node.is_constant() || !seen.insert(vn) || seen.len() > 256 {
        return size;
    }
    let Some(def) = node.get_def().and_then(|d| data.obank().get(d)) else {
        return if node.is_input() && !parameter_register(data, node) { 0 } else { size };
    };
    let from = |k: int4, seen: &mut BTreeSet<_>| def.get_in(k).map_or(0, |v| defined_width(data, v, seen));
    let piece_size = |k: int4| def.get_in(k).and_then(|v| data.vbank().get(v)).map_or(0, |v| v.get_size());
    let set = match def.code() {
        OpCode::CPUI_INDIRECT if def.is_indirect_creation() => {
            if def.get_in(0).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_indirect_zero())
                || !node.get_def().is_some_and(|d| becomes_the_calls_output(data, d))
            {
                0
            } else {
                returned_by_the_call(data, def, node)
            }
        }
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => from(0, seen),
        OpCode::CPUI_MULTIEQUAL => (0..def.num_input()).map(|k| from(k, seen)).min().unwrap_or(0),
        OpCode::CPUI_PIECE => {
            let low = from(1, seen);
            if low < piece_size(1) { low } else { low + from(0, seen) }
        }
        OpCode::CPUI_SUBPIECE => {
            let skip = def.get_in(1).and_then(|v| data.vbank().get(v)).map_or(0, |v| v.get_offset() as int4);
            from(0, seen) - skip
        }
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            let whole = piece_size(0);
            let set = from(0, seen);
            if set >= whole { size } else { set }
        }
        _ => size,
    };
    set.clamp(0, size)
}

/// Can the INDIRECT creation `ind` become its call's output?  The call's output
/// recovery looks only at the INDIRECT ops right before the call, so a creation
/// with another op between it and the call (the `PIECE` of an argument joined
/// from two registers) is never the call's result, and a function returning it
/// returns a variable nothing assigns.
fn becomes_the_calls_output(data: &Funcdata, ind: crate::context::OpId) -> bool {
    use kuna_num::opcodes::OpCode;
    let Some(call) = data
        .obank()
        .get(ind)
        .and_then(|o| o.get_in(1))
        .and_then(|i| data.vbank().get(i))
        .map(|i| crate::context::OpId::from(slotmap::KeyData::from_ffi(i.get_offset())))
        .filter(|&c| data.obank().get(c).is_some())
    else {
        return false;
    };
    let mut at = data.op_previous_op(call);
    while let Some(o) = at {
        if o == ind {
            return true;
        }
        if data.obank().get(o).is_none_or(|op| op.code() != OpCode::CPUI_INDIRECT) {
            return false;
        }
        at = data.op_previous_op(o);
    }
    false
}

/// How much of `node`, a possible result of the call its INDIRECT creation
/// `ind` guards, the callee returns: the least significant bytes of `node` its
/// declared or stated return storage covers, none when that storage misses
/// `node`'s least significant byte (the rest of `xmm0` beside a `float`) or the
/// callee was recovered `void`, and all of it when the callee's return is
/// unknown.
fn returned_by_the_call(data: &Funcdata, ind: &crate::op::PcodeOp, node: &crate::varnode::Varnode) -> int4 {
    let size = node.get_size();
    let Some(fc) = call_of(data, ind) else { return size };
    let proto = fc.proto();
    let (raddr, rsize) = if proto.is_output_locked() {
        let out = proto.get_output();
        if out.get_type().is_some_and(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_VOID) {
            return 0;
        }
        (out.get_address(), out.get_size())
    } else {
        let entry = fc.get_entry_address();
        let Some(k) = entry.get_space().map(|s| (s.get_index(), entry.get_offset())) else { return size };
        match data.kuna_callret_stated(k) {
            Some(stated) => (stated.addr.clone(), stated.size),
            None if data.kuna_callee_returns(k) == Some(Returns::Void) => return 0,
            None => return size,
        }
    };
    let (a, r) = (node.get_addr(), &raddr);
    if a.get_space().map(|s| s.get_index()) != r.get_space().map(|s| s.get_index()) {
        return size;
    }
    let (off, roff, rend) = (a.get_offset(), r.get_offset(), r.get_offset().wrapping_add(rsize as u64));
    let low = if a.is_big_endian() { off + size as u64 - 1 } else { off };
    if low < roff || low >= rend {
        return 0;
    }
    let covered = if a.is_big_endian() { low + 1 - roff } else { rend - low };
    (covered as int4).min(size)
}

/// The results of `void` callees the value `vn` would be, by callee: a wrapper
/// of such a callee returns nothing until the callee does, and asks for it
/// ([`record`] files these as the wrapper's reads).
fn void_results(data: &Funcdata, vn: crate::context::VarnodeId) -> Vec<((int4, uintb), (Address, int4))> {
    use kuna_num::opcodes::OpCode;
    let mut out = Vec::new();
    let mut stack = vec![vn];
    let mut seen = BTreeSet::new();
    while let Some(v) = stack.pop() {
        if !seen.insert(v) || seen.len() > 256 {
            continue;
        }
        let Some(node) = data.vbank().get(v) else { continue };
        let Some(def) = node.get_def().and_then(|d| data.obank().get(d)) else { continue };
        match def.code() {
            OpCode::CPUI_INDIRECT if def.is_indirect_creation() => {
                let Some(fc) = call_of(data, def) else { continue };
                let entry = fc.get_entry_address();
                let Some(k) = entry.get_space().map(|s| (s.get_index(), entry.get_offset())) else { continue };
                if !fc.proto().is_output_locked() && data.kuna_callee_returns(k) == Some(Returns::Void) {
                    out.push((k, (node.get_addr().clone(), node.get_size())));
                }
            }
            OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_SUBPIECE => stack.extend(def.get_in(0)),
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE => stack.extend((0..def.num_input()).filter_map(|k| def.get_in(k))),
            _ => {}
        }
    }
    out
}

/// The call an INDIRECT creation guards.
fn call_of<'a>(data: &'a Funcdata, ind: &crate::op::PcodeOp) -> Option<&'a crate::p4_calls::fspec::FuncCallSpecs> {
    let call = ind
        .get_in(1)
        .and_then(|i| data.vbank().get(i))
        .map(|i| crate::context::OpId::from(slotmap::KeyData::from_ffi(i.get_offset())))
        .filter(|&c| data.obank().get(c).is_some())?;
    data.get_call_specs_index(call).map(|i| data.get_call_specs(i))
}

/// Could a parameter arrive in `node`'s storage?  Its entry value is then an
/// argument (`mov rax, rdi` returns one); otherwise it is whatever the caller
/// left there.
fn parameter_register(data: &Funcdata, node: &crate::varnode::Varnode) -> bool {
    let proto = data.get_func_proto();
    proto.has_model()
        && proto
            .model()
            .input_opt()
            .is_some_and(|l| l.find_entry(node.get_addr(), node.get_size(), true).is_some())
}

/// Return the least significant `width` bytes of trial `i` at every live
/// RETURN (a `SUBPIECE` into the narrower register) and shrink the trial to
/// them, when the model returns a value there.
fn narrow(
    data: &mut Funcdata,
    active: &mut crate::fspec::ParamActive,
    i: int4,
    live: &[crate::context::OpId],
    slot: int4,
    width: int4,
) -> bool {
    let (addr, size) = {
        let t = active.get_trial(i);
        (t.get_address().clone(), t.get_size())
    };
    let low = if addr.is_big_endian() { &addr + ((size - width) as i64) } else { addr.clone() };
    if !active.test_shrink(i, &low, width)
        || data.get_func_proto().characterize_as_output(&low, width) != crate::fspec::Containment::ContainsJustified
    {
        return false;
    }
    for &r in live {
        let Some((vn, at)) = data.obank().get(r).and_then(|o| Some((o.get_in(slot)?, o.get_addr().clone()))) else {
            return false;
        };
        let op = data.new_op(2, at);
        data.op_set_opcode_code(op, kuna_num::opcodes::OpCode::CPUI_SUBPIECE);
        let Ok(out) = data.new_varnode_out(width, &low, op) else { return false };
        if let Some(v) = data.vbank_mut().get_mut(out) {
            v.set_write_mask();
        }
        data.op_insert_before(op, r);
        let zero = data.new_constant(4, 0);
        let _ = data.op_set_input(op, vn, 0);
        let _ = data.op_set_input(op, zero, 1);
        let _ = data.op_set_input(r, out, slot);
    }
    active.shrink(i, low, width);
    true
}

/// Is the return trial `[addr, addr+size)` on the storage the function's
/// callers read?
pub fn forced(data: &Funcdata, addr: &Address, size: int4) -> bool {
    data.kuna_forced_return().iter().any(|(faddr, fsize)| overlaps(addr, size, faddr, *fsize))
}

/// Is `opmatch` a RETURN whose forced return trial [`score_forced`] is scoring?
/// A caller reads the register after the call, so the value there is the
/// function's result whatever else the function does with it, and
/// `only_op_use` lets it be stored and branched on as well ([`returned_use`]).
pub fn scoring_return(data: &Funcdata, opmatch: crate::context::OpId) -> bool {
    data.kuna_forced_scoring()
        && data.obank().get(opmatch).is_some_and(|o| o.code() == kuna_num::opcodes::OpCode::CPUI_RETURN)
}

/// Is `op`'s use of `vn` one a returned value may also have: a branch on it
/// (`if (r > 7) gj++; return r;`), a store of it as the stored value (`*p = r;
/// return r;`; a copy into a global, `gi = r`, is the same store), or a RETURN
/// handing it back in another register (`lea 1(%rax),%edx` leaves `x + 1` in
/// `edx`, which the callers do not read)?  Upstream's `onlyOpUse` refuses all
/// three, to tell a value passed in a register from a variable that happens to
/// sit there. A load or store through the value, and a call reading it (the
/// stream pointer of a `getc` loop, the message of an `error` call), still
/// refuse it.
pub fn returned_use(data: &Funcdata, op: crate::context::OpId, vn: crate::context::VarnodeId) -> bool {
    use kuna_num::opcodes::OpCode;
    data.obank().get(op).is_some_and(|o| match o.code() {
        OpCode::CPUI_CBRANCH | OpCode::CPUI_RETURN => true,
        OpCode::CPUI_STORE => o.get_in(2) == Some(vn) && o.get_in(1) != Some(vn),
        _ => false,
    })
}

#[cfg(test)]
mod tests;
