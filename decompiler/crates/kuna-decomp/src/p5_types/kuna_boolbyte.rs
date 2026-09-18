//! A byte that is only ever a truth value is a `bool` (kuna `boolbyte`, P5).
//!
//! `ActionInferTypes` seeds every live Varnode from local op semantics
//! (`getLocalType`, `coreaction_infertypes.rs`) and `TYPE_BOOL` is only ever
//! minted as the *output* of a comparison (`TypeOpEqual` and friends return
//! `getBase(size, TYPE_BOOL)`).  Nothing mints it for the value being compared:
//! `TypeOpEqual::getInputLocal` votes `getBase(1, TYPE_INT)`, and the cached
//! one-byte `TYPE_INT` is the ASCII `char` ("Char is preferred over other int
//! types", `TypeFactory::cacheCoreTypes`).  So a byte whose every use is
//! `if (flag)` is declared `char`:
//!
//! ```text
//!   void sub_2b45(unsigned long a0, long a1, char a2)   // coreutils basename -O0
//!   {                                                   // DWARF: _Bool use_nuls
//!     ...
//!     v1 = (a2) ? 0 : 10;
//!   }
//! ```
//!
//! [`truth_value_type`] supplies the missing candidate.  It walks the byte's
//! transitive readers through value-preserving identity (COPY / MULTIEQUAL /
//! INT_ZEXT, breadth-first with a visited set and a hop cap, so the cap is a
//! property of the graph rather than of the pop order) and offers `bool` when
//! every terminal read is a truth test and at least one of them is a real one.
//! CPUI_INDIRECT is deliberately NOT in that set, on either side of the walk: an
//! INDIRECT output is the value after the op it annotates -- a call whose callee
//! may have written this storage, or a store that may alias it -- so it is
//! exactly the op that says the value may have been replaced.  Crossing it
//! declares `bool` for a byte a callee fills with 200, and that is a real change
//! of behaviour on a round trip through the emitted C, so both halves refuse
//! there.  A value that is written inside the function must also carry a non-zero
//! mask of at most 1 -- `ActionNonzeroMask` runs immediately before
//! `ActionInferTypes`, and its mask is exactly the "every def writes 0 or 1, or
//! copies something that does" property, computed transitively and for free --
//! and, separately, every def reaching it has to be a constant 0/1, a comparison
//! result, or a COPY/phi/ZEXT of one of those.  A
//! function *input* has no def to prove anything about, so for it the use shape
//! is the whole of the evidence: a byte the function only ever branches on is
//! what a `_Bool` parameter looks like from the inside.
//!
//! The candidate is FOLDED into the same `getLocalType` fold by
//! `Datatype::type_order`, never installed as a replacement: `SUB_BOOL` is 10 and
//! beats `SUB_INT_CHAR` 19 and `SUB_UINT_PLAIN` 16, so `bool` wins over `char` and
//! `uint1` -- and loses to anything more specific, such as a callee's locked
//! parameter type.  Nothing else in the type system changes: the `TYPE_BOOL`
//! propagation gate (`propagate_type_edge`, which refuses to carry bool into a
//! Varnode whose mask exceeds 1) is left exactly as upstream wrote it, so the
//! seed cannot leak along a copy chain into a value that can hold more than 0/1.
//!
//! Gated by [`Architecture::bool_byte`](crate::architecture::Architecture) (option
//! `boolbyte on|off`, default on); with the option off nothing in this module is
//! reachable.

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use kuna_base::space::spacetype;

/// How many def-use hops the walk follows before giving up.  The same bound
/// [`crate::kuna_ptrfromuse`] uses.
const HOP_CAP: usize = 10;

/// What one reader of a walked Varnode tells us.
enum Use {
    /// A real truth test: this value is read as a condition.
    Truth,
    /// Value-preserving identity: keep walking from `vn`.
    Forward(VarnodeId),
    /// Compatible with a truth value, but no evidence either way.
    Neutral,
    /// Inconsistent with a truth value: the whole candidate is refused.
    Refuse,
}

/// The `bool` candidate for `vn`, or `None` when the rule declines.
///
/// `vn` must be one byte wide.  `cur` is the vote `getLocalType` already has, and
/// the fold against it is tested FIRST: on a big function most one-byte Varnodes
/// are comparison results that are already `bool`, and walking their readers to
/// re-discover a candidate that cannot win is the whole cost of the rule.  The
/// returned type is the architecture's one-byte `TYPE_BOOL` core type, offered as
/// one more vote in that fold.
pub fn truth_value_type(
    data: &Funcdata,
    vn: VarnodeId,
    cur: &Rc<Datatype>,
) -> Option<Rc<Datatype>> {
    if !data.get_arch().bool_byte {
        return None;
    }
    let cand = data.get_arch().types()?.get_base(1, type_metatype::TYPE_BOOL).ok()?;
    if !folds_over(&cand, cur) {
        return None; // the fold is already at least this specific
    }
    let v = data.vbank().get(vn)?;
    if v.get_size() != 1 || v.is_annotation() || v.is_constant() || v.is_type_lock() {
        return None;
    }
    // Only a value that can carry a DECLARATION this function owns.  A temporary
    // in the unique space is an expression kuna prints inline -- `x & 1` inside an
    // `if` -- and typing it bool changes nothing a reader can see except the
    // `(bool)` cast `ActionSetCasts` then has to insert to reconcile the op's own
    // output type.  A persistent (global) Varnode has the same problem from the
    // other side: its declaration belongs to the global scope, whose Symbol this
    // rule does not touch, so the disagreement between the two prints as
    // `if (!(bool)dat_10978)` on every read.  Deciding a global's type needs the
    // whole program, not one function.
    if v.get_space().get_type() == spacetype::IPTR_INTERNAL || v.is_persist() {
        return None;
    }
    // Not every function INPUT is a parameter.  A read of a stack slot the
    // function never writes -- the frame buffer a callee fills through a pointer
    // is the common one -- is an input Varnode too, and the "this is what a
    // `_Bool` parameter looks like from the inside" argument this rule makes for
    // an input says nothing about it.  Ask the prototype model whether the
    // storage could carry a parameter at all, as `kuna_ptrfromuse` does.
    if v.is_input() && !data.get_func_proto().possible_input_param(&v.get_addr().clone(), 1) {
        return None;
    }
    // A byte that is one element of a bigger declared object is not a
    // declaration of its own: the declaration that would change is the
    // container's.  cronie `crond` reads the first byte of a 131088-byte `fgets`
    // line buffer as the loop's terminator test, and typing that byte `bool`
    // retypes the whole buffer to `bool[131088]` while it is still handed to
    // `fgets`, `strlen` and `strcmp`.
    if covered_by_larger_symbol(data, vn) {
        return None;
    }
    // A value written inside the function has to prove itself on the def side as
    // well.  The non-zero mask is the cheap half -- `ActionNonzeroMask` runs
    // immediately before this pass and computes "this value only ever has bit 0
    // set" transitively -- but it is not enough on its own: `x & 1` has mask 1 and
    // is a parity test, not a flag.  So the defs are walked too, and only a
    // constant 0/1, a comparison result, or a COPY/phi/ZEXT of one of those
    // counts -- an INDIRECT def refuses, because its input is the value before
    // the annotated call or store wrote the storage.  An input has no def and
    // always carries the full 0xff mask, so for a parameter the use shape below
    // is the whole of the evidence.
    if v.is_written() {
        if v.get_nz_mask() > 1 || !def_is_truth_valued(data, vn, 0) {
            return None;
        }
    }
    if !walk_is_truth_valued(data, vn) {
        return None;
    }
    Some(cand)
}

/// Whether the candidate `cand` is more specific than the current vote `cur`,
/// the same fold `getLocalType` applies to every other candidate.
pub fn folds_over(cand: &Rc<Datatype>, cur: &Rc<Datatype>) -> bool {
    0 > cand.type_order(cur).unwrap_or(0)
}

/// Bounded breadth-first walk over the transitive readers of `start`.  True when
/// at least one terminal read is a truth test and no read refuses.
fn walk_is_truth_valued(data: &Funcdata, start: VarnodeId) -> bool {
    // Cheap first sweep.  The overwhelmingly common answer is "the first reader
    // refuses", and paying for a HashSet and a VecDeque to discover that is the
    // difference between a rule that costs nothing and one that shows up in the
    // profile of a 3000-line function.
    {
        let v = match data.vbank().get(start) {
            Some(v) => v,
            None => return false,
        };
        let mut any = false;
        let mut forwards = false;
        let mut truth = false;
        for op in v.descend_iter() {
            any = true;
            match classify_use(data, op, start) {
                Use::Refuse => return false,
                Use::Truth => truth = true,
                Use::Neutral => {}
                Use::Forward(_) => forwards = true,
            }
        }
        if !any {
            return false;
        }
        if !forwards {
            return truth; // one hop was the whole walk
        }
    }
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: VecDeque<(VarnodeId, usize)> = VecDeque::new();
    work.push_back((start, 0));
    seen.insert(start);
    let mut found_truth = false;
    while let Some((vn, hops)) = work.pop_front() {
        let descend: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        // A Varnode nothing reads proves nothing; the accept condition below wants
        // a real test somewhere in the walk, so an empty descend list just ends
        // this branch.
        for op in descend {
            match classify_use(data, op, vn) {
                Use::Refuse => return false,
                Use::Truth => found_truth = true,
                Use::Neutral => {}
                Use::Forward(next) => {
                    // Widening keeps the value; narrowing does not, so only a
                    // same-or-wider destination is walked.  (A SUBPIECE never
                    // reaches here -- `classify_use` refuses it.)  Running out of
                    // hops is a refusal, not an acceptance: every use has to
                    // agree, and a use the walk never reached has not agreed.
                    if hops + 1 >= HOP_CAP {
                        return false;
                    }
                    if seen.insert(next) {
                        work.push_back((next, hops + 1));
                    }
                }
            }
        }
    }
    found_truth
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
        // The condition of a conditional branch is a truth value by definition
        // (`TypeOpCbranch::getInputLocal` says so for slot 1).
        OpCode::CPUI_CBRANCH => {
            if slot == 1 {
                Use::Truth
            } else {
                Use::Refuse
            }
        }
        // Every boolean operator takes and returns truth values.
        OpCode::CPUI_BOOL_NEGATE
        | OpCode::CPUI_BOOL_AND
        | OpCode::CPUI_BOOL_OR
        | OpCode::CPUI_BOOL_XOR => Use::Truth,
        // `x == 0` / `x != 0` is the C `!x` / `x`.  A comparison against 1 says the
        // same thing only when the value is already known to be 0 or 1 -- against
        // an unproven byte it distinguishes 1 from 2, which a bool cannot.
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let other = if slot == 0 { 1 } else { 0 };
            match o
                .get_in(other)
                .and_then(|c| data.vbank().get(c))
                .filter(|c| c.is_constant())
                .map(|c| c.get_offset())
            {
                Some(0) => Use::Truth,
                Some(1) if is_mask_proven(data, vn) => Use::Truth,
                _ => Use::Refuse,
            }
        }
        // `x & 1` / `x ^ 1` is the identity / the negation of a proven 0-or-1
        // value.  On an unproven byte it is a parity test, which says nothing.
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_XOR => {
            let other = if slot == 0 { 1 } else { 0 };
            let one = o
                .get_in(other)
                .and_then(|c| data.vbank().get(c))
                .map(|c| c.is_constant() && c.get_offset() == 1)
                .unwrap_or(false);
            if one && is_mask_proven(data, vn) {
                Use::Truth
            } else {
                Use::Refuse
            }
        }
        // Identity and widening: the same truth value travels on, so whatever
        // reads the copy has to be a truth test too.
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INT_ZEXT => match out {
            Some(o) => Use::Forward(o),
            None => Use::Neutral,
        },
        // An INDIRECT is NOT identity.  Its output is the value AFTER the op it
        // annotates -- a call whose callee may have written this storage, or a
        // store that may alias it -- so a truth test on the output says nothing
        // about the input, and the input's proof says nothing about the output.
        // Crossing it declares `bool` for a byte a callee fills with 200.
        OpCode::CPUI_INDIRECT => Use::Refuse,
        // Returning the value neither proves nor denies anything, so it is
        // allowed once the value is PROVEN to be 0 or 1 -- a `_Bool`-returning
        // function is indistinguishable from a `char`-returning one from in
        // here.  On an unproven byte (an input) the use shape is the whole of
        // the evidence, and "the function hands it back" is not evidence.
        OpCode::CPUI_RETURN => {
            if is_mask_proven(data, vn) {
                Use::Neutral
            } else {
                Use::Refuse
            }
        }
        // Everything else -- arithmetic, a store, a call argument, an ordering
        // comparison, a PIECE, a narrowing SUBPIECE, a LOAD address -- is either a
        // use no truth value has or evidence this rule is not allowed to read as
        // boolean.  A byte passed to a call is the common one: `putchar(c)` must
        // not make `c` a bool just because something else also tests it.
        _ => Use::Refuse,
    }
}

/// Walk the def chain: is every value that can reach `vn` a literal 0/1 or a
/// comparison result?  This is the def half of the predicate, and the reason a
/// parity test (`x & 1`, whose mask is also 1) is not read as a flag.
fn def_is_truth_valued(data: &Funcdata, vn: VarnodeId, depth: usize) -> bool {
    if depth > HOP_CAP {
        return false;
    }
    let v = match data.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    if v.is_constant() {
        return v.get_offset() <= 1;
    }
    if !v.is_written() {
        return false; // an input: nothing here can prove its value
    }
    let def = match v.get_def() {
        Some(d) => d,
        None => return false,
    };
    let o = match data.obank().get(def) {
        Some(o) => o,
        None => return false,
    };
    let opcode = o.code();
    // A `booloutput` op already produces a truth value -- `getOutputLocal` says
    // `TYPE_BOOL` for it, which is why this half of the rule usually has nothing
    // to add.  It matters for the copy chains that lead AWAY from one.
    if o.is_bool_output() {
        return true;
    }
    match opcode {
        // Identity and widening: prove the source instead.  An INDIRECT is not
        // one of these and falls through to the refusal below: its input is the
        // value BEFORE the annotated call or store, which proves nothing about
        // the output.
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => o
            .get_in(0)
            .map(|i| def_is_truth_valued(data, i, depth + 1))
            .unwrap_or(false),
        // Every arm of the merge has to prove itself.
        OpCode::CPUI_MULTIEQUAL => {
            let n = o.num_input();
            if n == 0 {
                return false;
            }
            (0..n).all(|i| {
                o.get_in(i)
                    .map(|a| def_is_truth_valued(data, a, depth + 1))
                    .unwrap_or(false)
            })
        }
        _ => false,
    }
}

/// Is `vn` one byte of a mapped object bigger than one byte -- a stack array, a
/// struct field's neighbour, a global buffer?  `ScopeLocal::findContainer` for
/// the local frame, the global scope's snapshot for everything else.
fn covered_by_larger_symbol(data: &Funcdata, vn: VarnodeId) -> bool {
    let v = match data.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    let addr = v.get_addr().clone();
    let usepoint = data.vn_use_point(vn);
    if let Some(entry) =
        data.get_scope_local().and_then(|lm| lm.query_container(&addr, 1, &usepoint))
    {
        return entry.size > 1;
    }
    data.get_arch()
        .query_container_global(&addr, 1, &usepoint)
        .and_then(|g| g.symbol_type.as_ref().map(|t| t.get_size() > 1))
        .unwrap_or(false)
}

/// Does a truncation into this rule's `bool` print as a `(bool)` cast?
///
/// `CastStrategyC::isSubpieceCast` (cast.cc:411-432) lists the destination
/// metatypes a SUBPIECE may print as a cast, and `TYPE_BOOL` is not one of them
/// -- upstream never puts a bool there, because nothing upstream mints
/// `TYPE_BOOL` for anything but a comparison's own one-byte output, which is
/// never the destination of a truncation.  This rule does, and without this arm
/// the printer falls to the functional form and emits the raw `SUB41(x,0)`
/// p-code intrinsic: an undeclared identifier, the class `subright` exists to
/// keep out of the emitted C.  tar -O2 `sub_41370` is the measured case
/// (`v = (char)v15;` -> `v = SUB41(v15,0);`).
///
/// The truncation is only rewritten, never removed: the cast is the same
/// narrowing `(char)` printed there with the option off, and the seed that put
/// `bool` on the destination already required the value reaching it to carry a
/// non-zero mask of at most 1, so `(bool)x` and the low byte of `x` agree.
/// Inert with the option off.
pub fn truncation_prints_as_cast(
    enabled: bool,
    outtype: &Rc<Datatype>,
    intype: &Rc<Datatype>,
    offset: u32,
) -> bool {
    if !enabled || offset != 0 {
        return false;
    }
    if outtype.get_metatype() != type_metatype::TYPE_BOOL {
        return false;
    }
    matches!(
        intype.get_metatype(),
        type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_UNKNOWN
            | type_metatype::TYPE_BOOL
    )
}

/// Is this Varnode's value *proven* to be 0 or 1 (rather than merely used as
/// though it were)?  True for a written Varnode whose non-zero mask is at most 1.
fn is_mask_proven(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank()
        .get(vn)
        .map(|v| v.is_written() && v.get_nz_mask() <= 1)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests;
