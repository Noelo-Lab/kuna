//! (kuna `signedness`) Decide an integer local's *declared* signedness from the
//! operations the body applies to it, instead of from whichever type-inference
//! vote happened to be most specific.
//!
//! ## Why the declaration says `unsigned` when the source said `int`
//!
//! `Datatype::type_order` ranks `SUB_UINT_PLAIN (16)` ahead of
//! `SUB_INT_PLAIN (17)`, so in `get_local_type`'s "keep the most specific" fold a
//! single `uint` vote outranks every `int` vote on the same Varnode, and nothing
//! downstream re-decides.  On `-O2` x86-64 the `uint` votes are structural: a
//! 32-bit operation zero-extends into its 64-bit register (`INT_ZEXT`), and a
//! strength-reduced loop counter is masked and shifted (`INT_AND`, `INT_RIGHT`,
//! `INT_XOR`) - each of which seeds `TYPE_UINT` on both operands.  The result is
//! a counter declared `unsigned int` whose every *comparison* is signed, so the
//! cast machinery has to write the signedness back at each use:
//!
//! ```text
//!   unsigned int v1;
//!   v1 = dat_17108 + a0;
//!   if (0 <= (int)v1)
//!   while (v4 < (int)v1)
//! ```
//!
//! ## What this module decides
//!
//! TRex (USENIX Security 2025, §3.3.5 / §1.6) decides signedness at *rounding*
//! time - after propagation, from the accumulated set of size-tagged operations a
//! value participates in - rather than during propagation, and offers a flag for
//! the case where nothing observed settles it ("C programmers write `int` when it
//! does not matter").  This module is that decision re-derived for kuna's
//! declaration seam: it accumulates the op set per declared HighVariable and
//! picks `TYPE_INT` or `TYPE_UINT` for the *declaration only*.  No Varnode type
//! changes, no inference re-runs, and every cast `ActionSetCasts` inserted stays
//! where it is - except a `(int)`/`(unsigned int)` cast that the new declaration
//! makes a no-op, which is dropped (see [`SignPlan::drop_cast`]).
//!
//! ## The correctness rule
//!
//! Changing a declaration without changing Varnode types is only safe if no
//! operation whose *C semantics depend on the operand's signedness* is left
//! reading the variable through a type it disagrees with.  The constructs that
//! constrain the declaration are these - the C operators whose *meaning* depends
//! on operand signedness, plus `<<`, whose meaning does not but whose
//! *definedness* does:
//!
//! | C construct | p-code | demand |
//! |---|---|---|
//! | `<` `<=` `>` `>=` | `INT_SLESS`/`INT_SLESSEQUAL` | signed |
//! | `<` `<=` `>` `>=` | `INT_LESS`/`INT_LESSEQUAL` | unsigned |
//! | `/` `%` | `INT_SDIV`/`INT_SREM` | signed |
//! | `/` `%` | `INT_DIV`/`INT_REM` | unsigned |
//! | `>>` (operand 0) | `INT_SRIGHT` | signed |
//! | `>>` (operand 0) | `INT_RIGHT` | unsigned |
//! | widening conversion | `INT_SEXT` | signed |
//! | widening conversion | `INT_ZEXT` | unsigned |
//! | `SCARRY`/`SBORROW`/`CARRY` intrinsics | `INT_SCARRY`/`INT_SBORROW`/`INT_CARRY` | signed / unsigned |
//! | a `(T)` cast | `CPUI_CAST` | the cast's own metatype, and only to a plain integer of the same width |
//! | `<<` (operand 0) | `INT_LEFT` | unsigned - a *preference*, see below |
//!
//! Every row but the last is an op whose `getInputCast` passes
//! `care_uint_int = true` in `p9_emit/coreaction_casts.rs`: the set kuna's own
//! cast machinery already treats as signedness-carrying, and the reason a flip
//! can never silently change what an expression means.
//!
//! **`<<` is the exception, and it is a preference, not a soundness rule.**
//! `INT_LEFT` takes the *default* `getInputCast` arm (`care_uint_int = false`),
//! and rightly so: `a << k` shifts the same bits into the same places under
//! either declaration.  An earlier revision justified the row by "shifting a
//! negative value left is undefined in C", which does not hold up - signed
//! overflow of `+ - *` is undefined on exactly the same footing and those are
//! neutral here, and kuna's C models the arithmetic the binary performs rather
//! than being UB-free by construction.  The row is kept anyway, for two reasons
//! that were measured rather than argued: a value the body ORs and shifts left is
//! a bit buffer, which C source spells unsigned (zlib's `deflate` bit writer is
//! the case in the corpus), and a signed `<<` is the one otherwise-neutral
//! operator `-fsanitize=undefined` reports.  It costs 0 declarations under `auto`
//! and 8 under `prefer-signed` over the `fmt`/`ls`/`sort`/`du` `-O0`+`-O2` sweep,
//! and leaves the DWARF-checkable accuracy of every value unchanged; the numbers
//! and the alternative are in `docs/features/signedness/proposal.md`.  Because
//! the row is a preference and not a demand, it also *carries*: the walk
//! continues into whatever reads the shifted value, so `(v << 3) >> 2` is decided
//! by the `>>`.
//!
//! Everything else that this module lets through is signedness-*independent* at a
//! fixed width - `+ - * & | ^ == !=`, unary `~` and `-`, an assignment, a call
//! argument, a `return`, a stored value, a truncation or concatenation - because
//! two's-complement arithmetic and a same-width conversion produce the same bits
//! either way.
//!
//! **"At a fixed width" is a precondition, not a turn of phrase.**  C's integer
//! promotions convert any operand narrower than `int` to `int` *before* the
//! operator runs, and which extension is performed is read off the declaration:
//! `(short)-1 == -1` is true, `(unsigned short)0xffff == -1` is false, and the
//! two disagree at `+ - * & | ^ << == != < <= > >= / % >>` alike.  Below the
//! promotion width no operator is neutral, so this pass does not touch a narrow
//! declaration at all.  The promotion width that decides this is **the `int` of
//! the compiler reading kuna's output, not the target's**: emitted C is read and
//! compiled where `int` is 4 bytes however small the target's cspec
//! `<data_organization><integer_size>` is (it is 2 on avr8gcc, avr8egcc,
//! TI_MSP430, TI_MSP430X, CR16, PIC24 and x86-16).  The floor is therefore
//! `max(4, TypeFactory::get_size_of_int())` - see [`MIN_PROMOTION_SIZE`].
//! (Reduced from `findutils` `find -O2` `sub_bac0`, where re-declaring a `short`
//! unsigned left `if ((v2 == v8) || (v8 == -1)) break;` in place and made the
//! `break` dead; `tests/stages/kuna-signedness.xml` pins that case on x86-64 and
//! `tests/stages/kuna-signedness-int16.xml` pins it on a target whose own `int`
//! is 2 bytes.)
//!
//! At and above the promotion width the neutral list matches kuna's own cast
//! strategy: the demanding ops are precisely the ones whose `getInputCast` passes
//! `care_uint_int = true` (`p9_emit/coreaction_casts.rs`), i.e. the ones upstream
//! itself considers signedness-carrying.  A width change between two integers is
//! an explicit `INT_SEXT`/`INT_ZEXT` in p-code, so a mixed-width expression is
//! constrained by the extension op rather than slipping through as neutral.
//!
//! Three further conditions make the rule sound rather than merely plausible:
//!
//! * **The walk follows implied values.** `v + 1 < 0` is printed as one
//!   expression, so the C type of `v + 1` - and therefore whether `< 0` is a
//!   signed or an unsigned comparison - follows `v`'s *declaration*.  A neutral
//!   op whose output is implied (inlined) therefore extends the walk to that
//!   output's readers.  A neutral op whose output is explicit ends the walk: the
//!   value is assigned into a declaration of its own, and a same-width conversion
//!   is bit-identical.  Two of the *demanding* ops propagate the operand's type
//!   the same way - `>>` prints at the shiftee's promoted type and `/ %` at the
//!   usual-arithmetic-conversion type of their operands - so those record their
//!   demand *and* continue the walk ([`ReaderClass::DemandsCarrying`]), as does
//!   `<<`; a comparison, a cast-printing extension and a carry intrinsic do not,
//!   because the printed result carries a type of its own.
//! * **Anything unclassified vetoes the variable.** `LOAD`/`STORE` addresses,
//!   `PTRADD`/`PTRSUB` indices (`base[v]` with a negative `v` is not the same
//!   object as `base[(unsigned)v]`, and no cast is inserted there), every
//!   `FLOAT_*` op, `BRANCHIND`, `CPOOLREF`, `NEW`, `SEGMENTOP`, `INSERT`,
//!   `ZPULL`/`SPULL`, and a `CPUI_CAST` whose target is anything but a plain
//!   integer of the same width - each leaves the declaration exactly as upstream
//!   chose it.
//! * **The declaration is at least as wide as the reader's `int`** (above), so no
//!   promotion runs before any of the neutral operators.
//!
//! With those in place, `auto` flips to signed only when every signedness-sensitive
//! reader demands signed, so each surviving `(int)` cast on the variable becomes a
//! no-op and the rest of the expression is byte-for-byte what it was.
//!
//! ## What this does *not* claim
//!
//! The rule reads the compiler's instruction selection, not the source.  Where a
//! compiler proved a `size_t` non-negative and emitted a signed compare on it,
//! `auto` declares it signed and the source said unsigned; the emitted C still
//! computes what the binary computes, but the declaration is not the programmer's.
//! `docs/features/signedness/analysis.md` carries the measured agreement rate
//! against DWARF.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{sub_metatype, type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

/// The narrowest declaration this pass will re-sign, in bytes.
///
/// C's integer promotions run before every operator, and which extension is
/// performed is read off the declaration, so below the promotion width nothing in
/// [`classify_reader`]'s neutral half is neutral.  The promotion width that
/// decides that is the one of the compiler reading kuna's output - `int` is 4
/// bytes there - and *not* the target's `<data_organization><integer_size>`, which
/// is 2 on avr8gcc, avr8egcc, TI_MSP430, TI_MSP430X, CR16, PIC24 and x86-16.
const MIN_PROMOTION_SIZE: int4 = 4;

/// (kuna) How the declared signedness of an integer local is chosen:
/// `signedness upstream|auto|prefer-signed|prefer-unsigned`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SignPolicy {
    /// Keep what type inference produced (byte-identical to not having the option).
    #[default]
    Upstream,
    /// Flip only when the operation set is unanimous; leave an unobserved value alone.
    Auto,
    /// `Auto`, and declare an unobserved value signed.
    PreferSigned,
    /// `Auto`, and declare an unobserved value unsigned.
    PreferUnsigned,
}

impl SignPolicy {
    /// Is any rounding active?
    #[inline]
    pub fn is_active(self) -> bool {
        self != SignPolicy::Upstream
    }

    /// The catalog spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            SignPolicy::Upstream => "upstream",
            SignPolicy::Auto => "auto",
            SignPolicy::PreferSigned => "prefer-signed",
            SignPolicy::PreferUnsigned => "prefer-unsigned",
        }
    }
}

/// (kuna) The `signedness upstream|auto|prefer-signed|prefer-unsigned` option.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionSignedness;

impl OptionSignedness {
    /// The option name.
    pub const NAME: &'static str = "signedness";

    /// Parse + validate the value and return the policy plus the confirmation
    /// message (the caller stores it in `Architecture::signedness`).
    pub fn apply(&self, p1: &str) -> KunaResult<(SignPolicy, String)> {
        let policy = match p1 {
            "upstream" => SignPolicy::Upstream,
            "auto" => SignPolicy::Auto,
            "prefer-signed" => SignPolicy::PreferSigned,
            "prefer-unsigned" => SignPolicy::PreferUnsigned,
            _ => {
                return Err(KunaError::parse(
                    "signedness must be \"upstream\", \"auto\", \"prefer-signed\" or \"prefer-unsigned\"",
                ))
            }
        };
        Ok((
            policy,
            format!("Declared integer signedness set to {}", policy.as_str()),
        ))
    }
}

/// What the accumulated operation set says about one value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Evidence {
    /// No signedness-sensitive operation was observed.
    None,
    /// Every signedness-sensitive operation is signed.
    Signed,
    /// Every signedness-sensitive operation is unsigned.
    Unsigned,
    /// Both appear, or an unclassified operation was reached.
    Veto,
}

impl Evidence {
    fn join(self, other: Evidence) -> Evidence {
        match (self, other) {
            (Evidence::Veto, _) | (_, Evidence::Veto) => Evidence::Veto,
            (Evidence::None, e) | (e, Evidence::None) => e,
            (a, b) if a == b => a,
            _ => Evidence::Veto,
        }
    }
}

/// The per-function decision: which declared locals change signedness, and which
/// `CPUI_CAST` ops the change makes redundant.
#[derive(Debug, Default)]
pub struct SignPlan {
    /// Declared type override, per HighVariable.
    decls: HashMap<HighVariableId, Rc<Datatype>>,
    /// The declaration overrides the printer actually emitted.
    applied: HashMap<HighVariableId, Rc<Datatype>>,
}

impl SignPlan {
    /// Is there nothing to do?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.decls.is_empty()
    }

    /// The declared type this pass wants for `high`, if any.
    #[inline]
    pub fn decl_type(&self, high: HighVariableId) -> Option<&Rc<Datatype>> {
        self.decls.get(&high)
    }

    /// Record that the printer emitted `ty` as `high`'s declaration, which is what
    /// authorizes [`Self::drop_cast`] to suppress a now-redundant cast on it.
    pub fn record_applied(&mut self, high: HighVariableId, ty: Rc<Datatype>) {
        self.applied.insert(high, ty);
    }

    /// Clear the plan between documents.
    pub fn clear(&mut self) {
        self.decls.clear();
        self.applied.clear();
    }

    /// Is this `CPUI_CAST` a no-op against the declaration that was emitted?
    ///
    /// True only when the cast's operand is an explicit member of a high whose
    /// declaration this pass changed, and the cast target is the very type that
    /// was declared - so removing the token leaves an expression of exactly the
    /// same C type.
    pub fn drop_cast(&self, fd: &Funcdata, op: OpId) -> bool {
        if self.applied.is_empty() {
            return false;
        }
        let Some(o) = fd.obank().get(op) else { return false };
        if o.code() != OpCode::CPUI_CAST {
            return false;
        }
        let Some(invn) = o.get_in(0) else { return false };
        let Some(outvn) = o.get_out() else { return false };
        let Some(inv) = fd.vbank().get(invn) else { return false };
        if !inv.is_explicit() {
            return false;
        }
        let Some(high) = inv.get_high() else { return false };
        let Some(declared) = self.applied.get(&high) else { return false };
        let Some(outv) = fd.vbank().get(outvn) else { return false };
        let target = outv.get_type_def_facing();
        target.get_size() == declared.get_size()
            && target.get_metatype() == declared.get_metatype()
            && target.get_sub_meta() == declared.get_sub_meta()
            && target.get_name() == declared.get_name()
    }
}

/// Is `dt` a plain signed/unsigned integer this pass may re-sign?
///
/// Deliberately narrow: `char`, `wchar`, enums, bitfields, bools, typedefs and
/// anything with a name of its own keep whatever type inference gave them.
fn is_plain_integer(dt: &Datatype) -> bool {
    if dt.get_typedef().is_some() {
        return false;
    }
    if !matches!(dt.get_size(), 1 | 2 | 4 | 8) {
        return false;
    }
    matches!(
        dt.get_sub_meta(),
        sub_metatype::SUB_INT_PLAIN | sub_metatype::SUB_UINT_PLAIN
    ) && matches!(
        dt.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT
    )
}

/// How one reader of the value affects the decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReaderClass {
    /// The operand's signedness decides what this operator means in C, and the
    /// result prints a type of its own (a condition, a cast, an intrinsic call).
    Demands(Evidence),
    /// The operand's signedness is constrained *and* the printed result keeps
    /// carrying the operand's declared type - `>> / % <<`.  Both hold: the demand
    /// is recorded and the walk continues into the readers of the result, so a
    /// second operator further out cannot disagree unseen.
    DemandsCarrying(Evidence),
    /// Signedness-independent at a fixed width, and the result keeps carrying the
    /// operand's declared C type into the surrounding expression.
    Carries,
    /// Signedness-independent, and the result re-establishes a type of its own (a
    /// printed cast, an intrinsic) or prints no value at all.
    Opaque,
    /// Not classified - leave this declaration exactly as inference chose it.
    Veto,
}

/// Classify one reader of the value at input `slot`.
///
/// The demanding set is the C constructs whose *meaning* changes with the
/// operand's signedness - the ops whose `getInputCast` passes
/// `care_uint_int = true` in `p9_emit/coreaction_casts.rs` - plus `INT_LEFT`,
/// which is a stated preference rather than a requirement (module header).
/// Everything not named here is `Veto`: a pointer index, a dereference, a float
/// conversion and an indirect branch all read the value in a way this pass does
/// not model.
///
/// Every classification here assumes the operand is at least [`MIN_PROMOTION_SIZE`]
/// wide; `plan` enforces that before any of this is consulted.
fn classify_reader(opc: OpCode, slot: int4) -> ReaderClass {
    use OpCode::*;
    use ReaderClass::*;
    match opc {
        // `<` `<=` `>` `>=` - both operands carry the sign; the result is a
        // condition, which re-establishes a type of its own.
        CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL => Demands(Evidence::Signed),
        CPUI_INT_LESS | CPUI_INT_LESSEQUAL => Demands(Evidence::Unsigned),
        // `/` `%` - both operands carry the sign, and the quotient is printed at
        // the usual-arithmetic-conversion type of the two, so the walk continues.
        CPUI_INT_SDIV | CPUI_INT_SREM => DemandsCarrying(Evidence::Signed),
        CPUI_INT_DIV | CPUI_INT_REM => DemandsCarrying(Evidence::Unsigned),
        // `>>` - only the shiftee; the count is a count.  The printed result is
        // the shiftee's promoted type, so the walk continues into whatever reads
        // it (`(v >> 2) < 0` is one expression).
        CPUI_INT_SRIGHT => {
            if slot == 0 {
                DemandsCarrying(Evidence::Signed)
            } else {
                Opaque
            }
        }
        CPUI_INT_RIGHT => {
            if slot == 0 {
                DemandsCarrying(Evidence::Unsigned)
            } else {
                Opaque
            }
        }
        // `<<` - the one entry here that is a stated PREFERENCE rather than a
        // soundness requirement, and the module header says why.  The shifted
        // bits do not depend on the shiftee's signedness, so nothing breaks
        // either way; the unsigned pull is kept because a value the body ORs and
        // shifts left is a bit buffer, which C source spells unsigned, and
        // because a signed `<<` is the one otherwise-neutral operator UBSan
        // reports.  It carries as well as demands: the printed result keeps the
        // shiftee's type, so `(v << 3) >> 2` is still constrained by the `>>`.
        CPUI_INT_LEFT => {
            if slot == 0 {
                DemandsCarrying(Evidence::Unsigned)
            } else {
                Opaque
            }
        }
        // A widening conversion: which extension C performs is decided by the
        // source operand's type.
        CPUI_INT_SEXT => Demands(Evidence::Signed),
        CPUI_INT_ZEXT => Demands(Evidence::Unsigned),
        // The carry/borrow intrinsics spell their own signedness in the callee
        // name, so they constrain nothing - but they still say what the value is.
        CPUI_INT_SCARRY | CPUI_INT_SBORROW => Demands(Evidence::Signed),
        CPUI_INT_CARRY => Demands(Evidence::Unsigned),
        // `+ - * & | ^ ~` and the value-moving ops: at or above the promotion
        // width two's complement makes these bit-identical either way, and the
        // printed result inherits the operand's C type.
        CPUI_INT_ADD
        | CPUI_INT_SUB
        | CPUI_INT_MULT
        | CPUI_INT_AND
        | CPUI_INT_OR
        | CPUI_INT_XOR
        | CPUI_INT_NEGATE
        | CPUI_INT_2COMP
        | CPUI_COPY
        | CPUI_MULTIEQUAL
        | CPUI_INDIRECT => Carries,
        // `==` `!=` compare bits at the promotion width this pass guarantees
        // (a narrower declaration is never re-signed, see `plan`); a truncation, a
        // concatenation and the bit-count intrinsics print their own type; a
        // condition, a call argument, a return value and a stored value all
        // convert at a fixed width.
        CPUI_INT_EQUAL
        | CPUI_INT_NOTEQUAL
        | CPUI_SUBPIECE
        | CPUI_PIECE
        | CPUI_POPCOUNT
        | CPUI_LZCOUNT
        | CPUI_CBRANCH
        | CPUI_RETURN
        | CPUI_BOOL_NEGATE
        | CPUI_BOOL_AND
        | CPUI_BOOL_OR
        | CPUI_BOOL_XOR => Opaque,
        // A call argument converts to the parameter's type at a fixed width; the
        // call target itself is not a value this pass understands.
        CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER => {
            if slot == 0 {
                Veto
            } else {
                Opaque
            }
        }
        // `*p = v` stores the value at a fixed width; the address is not ours.
        CPUI_STORE => {
            if slot == 2 {
                Opaque
            } else {
                Veto
            }
        }
        _ => Veto,
    }
}

/// The evidence a *defining* op contributes.  A definition never constrains the
/// declaration - `v = <expr>;` converts at a fixed width, which is bit-identical
/// either way - so it can only vote, never veto.  Reads constrain; definitions
/// only vote.
///
/// Only the divide-and-shift family votes, because only there does the operator
/// itself say what kind of number came out: `a / b` under `INT_SDIV` is the signed
/// quotient of two signed values and cannot be re-read as unsigned.
///
/// The *extension* ops are deliberately silent, in both directions.  An extension
/// describes the operand it widened, not the widened value: `int v = *p;` with an
/// `unsigned char *p` is ordinary C (TRex's `ZeroExtendTgt => None`, and on x86-64
/// it is the difference between seeing a variable at all and vetoing every value a
/// 32-bit instruction widened into its 64-bit register), and symmetrically
/// `movslq %eax,%rdx` into a `uintmax_t` is ordinary C too - that `INT_SEXT`
/// def-vote is what declared `fmt::main`'s `uintmax_t max` as `long` before this
/// pass stopped counting it.  `INT_2COMP` is silent for the same reason: `-x` has
/// the same bits whichever way `x` is read.
fn def_evidence(opc: OpCode) -> Evidence {
    use OpCode::*;
    match opc {
        CPUI_INT_SDIV | CPUI_INT_SREM | CPUI_INT_SRIGHT => Evidence::Signed,
        CPUI_INT_DIV | CPUI_INT_REM | CPUI_INT_RIGHT => Evidence::Unsigned,
        _ => Evidence::None,
    }
}

/// Accumulate the op set reachable from `high`'s explicit members and fold it into
/// one verdict.
fn evidence_for(fd: &Funcdata, high: HighVariableId) -> Evidence {
    let Some(h) = fd.high_bank().get(high) else { return Evidence::Veto };
    let mut verdict = Evidence::None;
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: Vec<VarnodeId> = Vec::new();
    for i in 0..h.num_instances() {
        let vn = h.get_instance(i);
        // A member that is not printed by name carries no declaration, so it
        // contributes only through the expression it is inlined into - which the
        // implied walk below reaches from whichever explicit member roots it.
        if fd.vbank().get(vn).map(|v| v.is_explicit()).unwrap_or(false) {
            work.push(vn);
        }
        // A type-locked member is a user/DWARF decision; never re-sign it.
        if fd.vbank().get(vn).map(|v| v.is_type_lock()).unwrap_or(false) {
            return Evidence::Veto;
        }
    }
    if work.is_empty() {
        return Evidence::Veto;
    }
    // The declared members' own definitions vote but cannot veto.
    for &vn in &work {
        if let Some(def) = fd.vbank().get(vn).and_then(|v| v.get_def()) {
            if let Some(o) = fd.obank().get(def) {
                verdict = verdict.join(def_evidence(o.code()));
            }
        }
    }
    while let Some(vn) = work.pop() {
        if !seen.insert(vn) {
            continue;
        }
        let descend: Vec<OpId> = match fd.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            let Some(o) = fd.obank().get(op) else { continue };
            let opc = o.code();
            if opc == OpCode::CPUI_CAST {
                // A cast re-establishes the type, so the walk stops here - but a
                // cast to a plain integer of the same width is precisely a
                // statement that this value is read signed (or unsigned) here.
                // Any other target (a pointer, a float, a `char`, an enum, a
                // typedef, a different width) converts in a way this pass does not
                // model, so it vetoes like any other unclassified reader.
                let insize = fd.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
                let cast_ev = o
                    .get_out()
                    .and_then(|out| fd.vbank().get(out))
                    .map(|v| v.get_type_def_facing().clone())
                    .filter(|t| is_plain_integer(t) && t.get_size() == insize)
                    .map(|t| match t.get_metatype() {
                        type_metatype::TYPE_INT => Evidence::Signed,
                        _ => Evidence::Unsigned,
                    })
                    .unwrap_or(Evidence::Veto);
                verdict = verdict.join(cast_ev);
                if verdict == Evidence::Veto {
                    return Evidence::Veto;
                }
                continue;
            }
            let mut carries = false;
            // A value used in more than one slot of one op is judged once per slot.
            for slot in 0..o.num_input() {
                if o.get_in(slot) != Some(vn) {
                    continue;
                }
                match classify_reader(opc, slot) {
                    ReaderClass::Demands(ev) => verdict = verdict.join(ev),
                    ReaderClass::DemandsCarrying(ev) => {
                        verdict = verdict.join(ev);
                        carries = true;
                    }
                    ReaderClass::Carries => carries = true,
                    ReaderClass::Opaque => {}
                    ReaderClass::Veto => verdict = Evidence::Veto,
                }
            }
            if verdict == Evidence::Veto {
                return Evidence::Veto;
            }
            // An op that keeps printing the operand's own C type extends the walk
            // into whatever expression its (implied) result lands in: `v + 1 < 0`
            // is one printed expression, and the comparison's signedness follows
            // `v`'s declaration.
            if carries {
                if let Some(out) = o.get_out() {
                    let implied = fd.vbank().get(out).map(|v| v.is_implied()).unwrap_or(false);
                    if implied {
                        work.push(out);
                    }
                }
            }
        }
    }
    verdict
}

/// Build the per-function plan: for every HighVariable whose declaration this
/// printer would write as a plain integer, the type the operation set asks for.
///
/// Only highs that are *candidates for a local declaration* are considered - a
/// parameter's type comes from the prototype, and a high mapped onto a composite
/// Symbol is declared at the Symbol's type.
pub fn plan(
    fd: &Funcdata,
    policy: SignPolicy,
    types: &dyn TypeFactory,
    decl_type_of: impl Fn(HighVariableId) -> Option<Rc<Datatype>>,
) -> SignPlan {
    let mut out = SignPlan::default();
    if !policy.is_active() {
        return out;
    }
    let highs: Vec<HighVariableId> = fd.high_bank().iter().map(|(id, _)| id).collect();
    for high in highs {
        let Some(h) = fd.high_bank().get(high) else { continue };
        if h.kuna_name().is_none() {
            continue;
        }
        // A high the Symbol table describes as a composite, or a piece of one, is
        // declared from the Symbol - not from this type.
        if h.kuna_symbol_offset() >= 0 {
            continue;
        }
        if h.kuna_symbol_type().is_some_and(|t| {
            matches!(
                t.get_metatype(),
                type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION | type_metatype::TYPE_ARRAY
            )
        }) {
            continue;
        }
        // A prototype parameter prints its type in the signature.
        let is_param = (0..h.num_instances())
            .any(|i| fd.vbank().get(h.get_instance(i)).map(|v| v.is_input()).unwrap_or(false));
        if is_param {
            continue;
        }
        let Some(cur) = decl_type_of(high) else { continue };
        if !is_plain_integer(&cur) {
            continue;
        }
        // C promotes anything narrower than `int` to `int` before any operator
        // runs, and the declaration decides which extension that is - so below the
        // promotion width every operator is signedness-sensitive, `== !=` included
        // (`(short)-1 == -1` is true, `(unsigned short)0xffff == -1` is false).
        // The neutral half of `classify_reader` is only neutral at a fixed width,
        // so a narrow local is simply never re-declared.
        //
        // The width that matters is the `int` of whoever reads and compiles this
        // C, not the target's: emitted C is read on a host where `int` is 4 bytes
        // whatever `<integer_size>` the target's cspec declares (it is 2 on
        // avr8gcc, avr8egcc, TI_MSP430, TI_MSP430X, CR16, PIC24 and x86-16).  The
        // floor is therefore 4, raised to the target's own promotion width when
        // that is wider.
        if cur.get_size() < types.get_size_of_int().max(MIN_PROMOTION_SIZE) {
            continue;
        }
        let ev = evidence_for(fd, high);
        let want = match (ev, policy) {
            (Evidence::Veto, _) | (Evidence::None, SignPolicy::Auto) => continue,
            (Evidence::Signed, _) => type_metatype::TYPE_INT,
            (Evidence::Unsigned, _) => type_metatype::TYPE_UINT,
            (Evidence::None, SignPolicy::PreferSigned) => type_metatype::TYPE_INT,
            (Evidence::None, SignPolicy::PreferUnsigned) => type_metatype::TYPE_UINT,
            (Evidence::None, SignPolicy::Upstream) => continue,
        };
        if cur.get_metatype() == want {
            continue;
        }
        // `get_base_no_char` so a re-signed byte declares `int1`, never `char`:
        // widening a value to a character type is a different decision with its own
        // option.
        let Ok(newty) = (match want {
            type_metatype::TYPE_INT => types.get_base_no_char(cur.get_size(), want),
            _ => types.get_base(cur.get_size(), want),
        }) else {
            continue;
        };
        if !is_plain_integer(&newty) {
            continue;
        }
        out.decls.insert(high, newty);
    }
    out
}

#[cfg(test)]
#[path = "kuna_typeround/tests.rs"]
mod tests;
