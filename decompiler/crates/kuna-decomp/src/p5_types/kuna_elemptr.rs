//! A pointer used only as an array of one element type is declared as that
//! pointer (kuna `elemptr`, P5).
//!
//! A textbook base64 decoder indexes three byte arrays -- its input, a table it
//! allocated and kept in a global, and a constant table in `.data` -- and on a
//! stripped image every one of them reaches the reader as an integer or as
//! `void *`:
//!
//! ```text
//!   void * sub_123f(long a0,unsigned long a1,unsigned long *a2)
//!   v2 = (*(char *)(a0 + v7) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(a0 + v7) + dat_4020) : 0;
//!   *(char *)((long)v6 + (long)v8) = (char)(v5 >> 0x10);
//!   *(char *)((unsigned long)*(unsigned char *)((long)v1 + 0x4020) + (long)dat_4070) = (char)v1;
//! ```
//!
//! `TypeOpIntAdd` votes an integer for both operands of `a0 + v7`, and nothing
//! carries the `char *` the byte load implies back over the add, so the base is
//! declared `long` (a parameter, a global read before it is written), `void *`
//! (what `malloc` returns), or nothing at all (a constant address).
//!
//! [`element_pointer`] supplies that candidate as one more vote in the
//! `getLocalType` fold, the way [`crate::kuna_ptrfromuse`] supplies "this is a
//! pointer at all".  A bounded breadth-first walk follows the candidate through
//! copy-like identity and literal offsets and asks of every use whether it is an
//! ELEMENT ACCESS: a load or store of width `W` through the base plus an index
//! scaled by `W` (`x * W`, `x << log2 W`, or an unscaled integer for `W == 1`).
//! It commits to `T *`, `T` the integer of width `W`, when
//!
//! * every access through the base, at any literal offset, is `W` wide and
//!   lands on a whole element, and
//! * at least one index is a value the program computes.  Constant offsets
//!   alone are a record's fields and stay with `ptrfromuse` and `structsynth`
//!   (a qsort comparator's `void const *` parameters are why `ptrfromuse`'s
//!   `void` is worth its default).
//!
//! An index is recognised by how it was made -- scaled, extended from a
//! narrower integer, masked, shifted, or offset from a value the program also
//! uses as a number -- never by its declared type, which for the other operand
//! of the same add is the same integer vote this rule exists to overrule.  An
//! add whose other operand could as well be the base says nothing.
//!
//! `T` is signed or unsigned when every extension and ordering of a loaded
//! element agrees, and plain (`char`, `int`) otherwise; a byte compared against
//! a character literal is plain `char`.
//!
//! Four kinds of value are candidates: a function input the prototype model
//! could place a parameter in; the value a call returns when its callee left
//! the pointee open (`void *`, an allocator), alone in its variable; a global
//! the image holds no symbol for (`dat_4020`); and a constant address inside a
//! section of program data that is the base of an indexed access, which
//! `globalref` then prints as the array it names (`dat_4020[v1]`).
//!
//! The walk refuses on anything a pointer to `T` does not survive: an access of
//! another width (a record, not an array), an index scaled by something that is
//! not the element, a record stride (`p + i * 16 + 8`), a non-zero field
//! (`PTRSUB`), integer arithmetic on the pointer itself, a float, a comparison
//! against a non-zero literal, a call argument the callee declares as a scalar
//! or as a pointer to something of another width, and a comparison or
//! difference with a pointer to something of another width.  A declared, DWARF,
//! libc or asserted type is never touched: a type-locked Varnode, one seeded
//! from a locked symbol, and a vote that already points at something named or
//! sized all keep what they have.
//!
//! Gated by [`ArchContext::elem_ptr`](crate::context::ArchContext) (option
//! `elemptr on|off`); with the option off nothing here is reachable.

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// How many def-use hops the walk follows; the bound `ptrfromuse` uses.
const HOP_CAP: usize = 10;

/// How deep an index's definition is followed through literal offsets.
const INDEX_DEPTH: usize = 3;

/// `elemptr on|off`.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionElemPtr;

impl OptionElemPtr {
    /// The option name.
    pub const NAME: &'static str = "elemptr";

    /// Parse + validate the `on`/`off` value; the caller writes the live field.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Element-pointer typing turned {prop}")))
    }
}

/// Is the `KUNA_ELEMPTR_TRACE` dump on?  One line per candidate the walk
/// reaches a verdict on; changes nothing about the decompile.
fn trace_on() -> bool {
    use std::sync::OnceLock;
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("KUNA_ELEMPTR_TRACE").is_some())
}

/// What the walk learned about one candidate base.
#[derive(Default)]
struct Evidence {
    /// The one access width every element access agrees on.
    width: Option<int4>,
    /// Element accesses through an index the program computes.
    variable: u32,
    /// Element accesses at a literal offset.
    constant: u32,
    /// Extensions and orderings of a loaded element that read it signed.
    signed: u32,
    /// ... and unsigned.
    unsigned: u32,
    /// A loaded byte compared against a character literal.
    charlit: u32,
    /// Pointees something else declares or compares this value against.
    pointees: Vec<Rc<Datatype>>,
    /// The use that refused the candidate.
    refused: Option<&'static str>,
}

impl Evidence {
    fn refuse(&mut self, why: &'static str) {
        if self.refused.is_none() {
            self.refused = Some(why);
        }
    }

    /// Record one element access of `w` bytes; a second width refuses.
    fn access(&mut self, w: int4, variable: bool) {
        match self.width {
            Some(have) if have != w => self.refuse("mixed-width"),
            _ => self.width = Some(w),
        }
        if variable {
            self.variable += 1;
        } else {
            self.constant += 1;
        }
    }

    fn label(&self) -> String {
        match self.refused {
            Some(r) => format!("refuse:{r}"),
            None => format!(
                "w={:?} var={} const={} s={} u={} chr={}",
                self.width, self.variable, self.constant, self.signed, self.unsigned, self.charlit
            ),
        }
    }
}

/// What kind of candidate a Varnode is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Param,
    CallReturn,
    Global,
    Constant,
}

impl Kind {
    fn as_str(self) -> &'static str {
        match self {
            Kind::Param => "param",
            Kind::CallReturn => "ret",
            Kind::Global => "global",
            Kind::Constant => "const",
        }
    }
}

/// Does `cur` point at nothing this rule would be overriding?  `void *` and a
/// pointer to one unknown byte say the value is an address and nothing about
/// what is there; any other pointee is a claim, and a claim outranks use.
fn points_at_nothing(cur: &Datatype) -> bool {
    match cur.get_ptr_to() {
        Some(p) => {
            p.get_metatype() == type_metatype::TYPE_VOID
                || (p.get_metatype() == type_metatype::TYPE_UNKNOWN && p.get_size() <= 1)
        }
        None => false,
    }
}

/// May this rule replace the vote `cur`?  An integer or unknown vote, or a
/// pointer at nothing.
fn may_replace(cur: &Datatype) -> bool {
    match cur.get_metatype() {
        type_metatype::TYPE_PTR => points_at_nothing(cur),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => true,
        _ => false,
    }
}

/// The candidate `T *` for `vn`, or `None` when the rule declines.  `cur` is the
/// vote the ordinary fold produced; the candidate replaces it outright, which is
/// why a vote that already names a pointee is never offered.
pub fn element_pointer(data: &Funcdata, vn: VarnodeId, cur: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    if !data.get_arch().elem_ptr || !may_replace(cur) {
        return None;
    }
    let v = data.vbank().get(vn)?;
    if v.is_type_lock() || v.is_annotation() {
        return None;
    }
    let arch = Rc::clone(data.get_arch());
    let spc = Rc::clone(arch.manage().get_default_data_space()?);
    let ptrsize = spc.get_addr_size() as int4;
    if v.get_size() != ptrsize {
        return None;
    }
    let kind = candidate_kind(data, vn)?;
    let mut ev = Evidence::default();
    match kind {
        Kind::Constant => walk_constant(data, vn, &mut ev),
        _ => walk(data, vn, &mut ev),
    }
    let elem = if ev.refused.is_none() && ev.variable > 0 { element_type(data, &mut ev) } else { None };
    if trace_on() {
        eprintln!(
            "[elemptr] fn={:#x} {} {}@{:#x} cur={} {} -> {}",
            data.get_address().get_offset(),
            kind.as_str(),
            v.get_addr().get_space().map(|s| s.get_name().to_string()).unwrap_or_default(),
            v.get_offset(),
            cur.get_name(),
            ev.label(),
            elem.as_ref().map(|e| e.get_name().to_string()).unwrap_or_else(|| "-".into())
        );
    }
    let elem = elem?;
    arch.types()?.get_type_pointer(ptrsize, elem, spc.get_word_size()).ok()
}

/// Which kind of candidate `vn` is, or `None` when the rule does not speak
/// about it.
fn candidate_kind(data: &Funcdata, vn: VarnodeId) -> Option<Kind> {
    let v = data.vbank().get(vn)?;
    let arch = data.get_arch();
    if v.is_constant() {
        let off = v.get_offset();
        if !crate::kuna_globalref::in_ranges(&arch.elem_ptr_ranges, off) {
            return None;
        }
        let spc = Rc::clone(arch.manage().get_default_data_space()?);
        let addr = Address::new(spc, off);
        if arch.query_container_global(&addr, 1, &Address::new_invalid()).is_some() {
            return None;
        }
        return Some(Kind::Constant);
    }
    let addr = v.get_addr().clone();
    if v.is_input() {
        if data.get_func_proto().possible_input_param(&addr, v.get_size()) {
            return Some(Kind::Param);
        }
    }
    if crate::kuna_structsynth::is_call_return(data, vn) {
        let copies = copies_alone(data, vn)?;
        let rets = crate::kuna_structsynth::returned_values(data);
        if crate::kuna_structsynth::returned_beside_others(data, &copies, &rets) {
            return None;
        }
        return Some(Kind::CallReturn);
    }
    let data_space = arch.manage().get_default_data_space()?;
    let in_data = addr.get_space().is_some_and(|s| Rc::ptr_eq(s, data_space));
    if in_data
        && crate::kuna_globalref::in_ranges(&arch.elem_ptr_ranges, v.get_offset())
        && arch.query_container_global(&addr, v.get_size(), &Address::new_invalid()).is_none()
    {
        return Some(Kind::Global);
    }
    None
}

/// The most copies of one returned value [`copies_alone`] follows.
const MAX_COPIES: usize = 64;

/// The copies of the value a call returned, or `None` when a phi joins it with
/// some other value.  The candidate is a vote, so whatever shares a variable
/// with the value shares its type: a phi whose other inputs are copies of the
/// same value (a loop) or a null constant keeps it alone, and anything else --
/// a second buffer, a status code -- declines.
fn copies_alone(data: &Funcdata, vn: VarnodeId) -> Option<Vec<VarnodeId>> {
    let mut seen = vec![vn];
    let mut phis: Vec<OpId> = Vec::new();
    let mut i = 0;
    while i < seen.len() {
        let cur = seen[i];
        i += 1;
        let v = data.vbank().get(cur)?;
        for u in v.descend_iter() {
            let Some(op) = data.obank().get(u) else { continue };
            let follows = match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => true,
                OpCode::CPUI_INDIRECT => op.get_in(0) == Some(cur),
                OpCode::CPUI_MULTIEQUAL => {
                    phis.push(u);
                    true
                }
                _ => false,
            };
            if !follows {
                continue;
            }
            let Some(out) = op.get_out() else { continue };
            if !seen.contains(&out) {
                if seen.len() >= MAX_COPIES {
                    return None;
                }
                seen.push(out);
            }
        }
    }
    for phi in phis {
        let o = data.obank().get(phi)?;
        for k in 0..o.num_input() {
            let x = o.get_in(k)?;
            if seen.contains(&x) {
                continue;
            }
            let xv = data.vbank().get(x)?;
            if !(xv.is_constant() && xv.get_offset() == 0) {
                return None;
            }
        }
    }
    Some(seen)
}

/// The breadth-first walk from a non-constant candidate.  The second component
/// of a work item is the literal byte offset of that Varnode from the base.
fn walk(data: &Funcdata, start: VarnodeId, ev: &mut Evidence) {
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: VecDeque<(VarnodeId, intb, usize)> = VecDeque::new();
    seen.insert(start);
    work.push_back((start, 0, 0));
    while let Some((vn, off, hops)) = work.pop_front() {
        if ev.refused.is_some() {
            return;
        }
        let descend: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            if let Some((next, noff)) = base_use(data, op, vn, off, ev) {
                if hops + 1 < HOP_CAP && seen.insert(next) {
                    work.push_back((next, noff, hops + 1));
                }
            }
            if ev.refused.is_some() {
                return;
            }
        }
    }
}

/// A constant candidate's evidence: it must be the base of an indexed access.
fn walk_constant(data: &Funcdata, cvn: VarnodeId, ev: &mut Evidence) {
    let descend: Vec<OpId> = match data.vbank().get(cvn) {
        Some(v) => v.descend_iter().collect(),
        None => return,
    };
    for op in descend {
        let Some(o) = data.obank().get(op) else { continue };
        let slot = o.get_slot(cvn);
        match o.code() {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => {
                let _ = base_use(data, op, cvn, 0, ev);
            }
            // Any other reader of the address -- a call argument, a compare, a
            // store of the address itself -- is not an indexed access, and the
            // name is `globalref`'s to decide.
            _ => {
                let _ = slot;
                ev.refuse("const-other-use");
            }
        }
        if ev.refused.is_some() {
            return;
        }
    }
}

/// One use of a Varnode at literal offset `off` from the base.  Returns the
/// Varnode the base travels on to, with its offset, when it does.
fn base_use(data: &Funcdata, op: OpId, vn: VarnodeId, off: intb, ev: &mut Evidence) -> Option<(VarnodeId, intb)> {
    let o = data.obank().get(op)?;
    let slot = o.get_slot(vn);
    let out = o.get_out();
    let konst = |i: int4| -> Option<intb> {
        o.get_in(i)
            .and_then(|v| data.vbank().get(v))
            .filter(|v| v.is_constant())
            .map(|v| sign_extend(v.get_offset(), v.get_size()))
    };
    match o.code() {
        OpCode::CPUI_LOAD => {
            if slot == 1 {
                let w = out.and_then(|x| data.vbank().get(x)).map(|x| x.get_size())?;
                element_access(data, op, w, off, false, ev);
            }
            None
        }
        OpCode::CPUI_STORE => {
            if slot == 1 {
                let w = o.get_in(2).and_then(|x| data.vbank().get(x)).map(|x| x.get_size())?;
                element_access(data, op, w, off, false, ev);
            }
            None
        }
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => out.map(|x| (x, off)),
        OpCode::CPUI_INDIRECT => {
            if slot == 0 {
                out.map(|x| (x, off))
            } else {
                None
            }
        }
        OpCode::CPUI_PTRSUB => {
            if slot != 0 {
                return None;
            }
            match konst(1) {
                Some(0) => out.map(|x| (x, off)),
                _ => {
                    ev.refuse("field");
                    None
                }
            }
        }
        OpCode::CPUI_PTRADD => {
            if slot != 0 {
                return None;
            }
            let elem = konst(2)?;
            match konst(1) {
                Some(i) => out.map(|x| (x, off + i * elem)),
                None => {
                    indexed(data, out?, elem, off, ev);
                    None
                }
            }
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
            let other = if slot == 0 { 1 } else { 0 };
            let is_sub = o.code() == OpCode::CPUI_INT_SUB;
            if let Some(k) = konst(other) {
                if is_sub && slot == 1 {
                    ev.refuse("negated");
                    return None;
                }
                if data.vbank().get(vn).is_some_and(|v| v.is_constant()) {
                    return None;
                }
                let k = if is_sub { -k } else { k };
                return out.map(|x| (x, off + k));
            }
            if is_sub {
                // A difference: of two pointers when the other side is one.
                if let Some(p) = pointee_of(data, o.get_in(other)?, op) {
                    ev.pointees.push(p);
                }
                return None;
            }
            let other_vn = o.get_in(other)?;
            match index_scale(data, other_vn, INDEX_DEPTH) {
                Some((scale, k)) => {
                    indexed(data, out?, scale, off + k, ev);
                    None
                }
                None => None,
            }
        }
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL => {
            let other = o.get_in(if slot == 0 { 1 } else { 0 })?;
            let ov = data.vbank().get(other)?;
            if ov.is_constant() {
                if ov.get_offset() != 0 {
                    ev.refuse("compare-literal");
                }
            } else if let Some(p) = pointee_of(data, other, op) {
                ev.pointees.push(p);
            }
            None
        }
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            if slot > 0 {
                call_argument(data, op, slot, ev);
            }
            None
        }
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
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_PIECE
        | OpCode::CPUI_SUBPIECE => {
            ev.refuse("arithmetic");
            None
        }
        code => {
            if crate::kuna_ptrfromuse::is_float_op(code) {
                ev.refuse("float");
            }
            None
        }
    }
}

/// The address `sum` is the base, at literal offset `off`, plus an index
/// scaled by `scale`.  Every access through it must be one `scale`-wide
/// element; a further literal offset must be a whole number of elements.
fn indexed(data: &Funcdata, sum: VarnodeId, scale: intb, off: intb, ev: &mut Evidence) {
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: Vec<(VarnodeId, intb, usize)> = vec![(sum, off, 0)];
    seen.insert(sum);
    while let Some((vn, off, hops)) = work.pop() {
        let descend: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            let Some(o) = data.obank().get(op) else { continue };
            let slot = o.get_slot(vn);
            let out = o.get_out();
            let mut next = None;
            match o.code() {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE if slot == 1 => {
                    let w = if o.code() == OpCode::CPUI_LOAD {
                        out.and_then(|x| data.vbank().get(x)).map(|x| x.get_size())
                    } else {
                        o.get_in(2).and_then(|x| data.vbank().get(x)).map(|x| x.get_size())
                    };
                    let Some(w) = w else { continue };
                    if intb::from(w) != scale {
                        ev.refuse("scale-mismatch");
                    } else {
                        element_access(data, op, w, off, true, ev);
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => next = out.map(|x| (x, off)),
                OpCode::CPUI_INT_ADD => {
                    let other = if slot == 0 { 1 } else { 0 };
                    let k = o
                        .get_in(other)
                        .and_then(|x| data.vbank().get(x))
                        .filter(|x| x.is_constant())
                        .map(|x| sign_extend(x.get_offset(), x.get_size()));
                    if let Some(k) = k {
                        next = out.map(|x| (x, off + k));
                    }
                }
                OpCode::CPUI_PTRSUB => {
                    let zero = o
                        .get_in(1)
                        .and_then(|x| data.vbank().get(x))
                        .is_some_and(|x| x.is_constant() && x.get_offset() == 0);
                    if zero {
                        next = out.map(|x| (x, off));
                    } else {
                        ev.refuse("record-stride");
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if slot > 0 => call_argument(data, op, slot, ev),
                OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_DIV
                | OpCode::CPUI_INT_REM
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_RIGHT
                | OpCode::CPUI_INT_SRIGHT
                | OpCode::CPUI_INT_AND => ev.refuse("arithmetic-sum"),
                _ => {}
            }
            if let Some((n, noff)) = next {
                if hops + 1 < HOP_CAP && seen.insert(n) {
                    work.push((n, noff, hops + 1));
                }
            }
            if ev.refused.is_some() {
                return;
            }
        }
    }
}

/// One access of `w` bytes at literal offset `off` from the base, through the
/// `LOAD`/`STORE` `op`.  Reads how a loaded element is used, for the sign.
fn element_access(data: &Funcdata, op: OpId, w: int4, off: intb, variable: bool, ev: &mut Evidence) {
    if !matches!(w, 1 | 2 | 4 | 8) {
        ev.refuse("width");
        return;
    }
    if off.rem_euclid(intb::from(w)) != 0 {
        ev.refuse("misaligned");
        return;
    }
    ev.access(w, variable);
    let Some(o) = data.obank().get(op) else { return };
    if o.code() == OpCode::CPUI_STORE {
        if let Some(val) = o.get_in(2).and_then(|x| data.vbank().get(x)) {
            let t = val.get_type();
            if t.get_metatype() == type_metatype::TYPE_FLOAT {
                ev.refuse("float-elem");
            } else if t.get_metatype() == type_metatype::TYPE_PTR && w > 1 {
                ev.refuse("pointer-elem");
            }
        }
        return;
    }
    let Some(val) = o.get_out() else { return };
    let readers: Vec<OpId> = data.vbank().get(val).map(|v| v.descend_iter().collect()).unwrap_or_default();
    for r in readers {
        let Some(ro) = data.obank().get(r) else { continue };
        let rslot = ro.get_slot(val);
        match ro.code() {
            OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_SREM => ev.signed += 1,
            OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_REM => ev.unsigned += 1,
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL if w == 1 => {
                let other = ro.get_in(if rslot == 0 { 1 } else { 0 }).and_then(|x| data.vbank().get(x));
                if other.is_some_and(|x| x.is_constant() && is_char_literal(x.get_offset())) {
                    ev.charlit += 1;
                }
            }
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE if rslot == 1 => ev.refuse("pointer-elem"),
            code if crate::kuna_ptrfromuse::is_float_op(code) => ev.refuse("float-elem"),
            _ => {}
        }
    }
}

/// A printable ASCII character, the kind a program compares a text byte to.
fn is_char_literal(c: uintb) -> bool {
    (0x20..0x7f).contains(&c)
}

/// Record what a call's declared parameter says about the value passed in
/// `slot`: a scalar refuses; a pointer at something is checked against the
/// element once its width is known.
fn call_argument(data: &Funcdata, op: OpId, slot: int4, ev: &mut Evidence) {
    let declared = crate::coreaction_infertypes::declared_input_type_local(data, op, slot);
    match declared.get_metatype() {
        type_metatype::TYPE_PTR => {
            if !points_at_nothing(&declared) {
                if let Some(p) = declared.get_ptr_to() {
                    ev.pointees.push(p);
                }
            }
        }
        type_metatype::TYPE_UNKNOWN => {}
        _ => ev.refuse("callee-scalar"),
    }
}

/// The pointee of `vn` as `op` reads it, when that is a pointer at something.
fn pointee_of(data: &Funcdata, vn: VarnodeId, op: OpId) -> Option<Rc<Datatype>> {
    let t = data.vn_type_read_facing(vn, op);
    if t.get_metatype() != type_metatype::TYPE_PTR || points_at_nothing(&t) {
        return None;
    }
    t.get_ptr_to()
}

/// If `x` is an index, the element size it is scaled by and the literal offset
/// folded into it: `i * 4` is `(4, 0)`, `(long)i + 1` is `(1, 1)`.  `None` when
/// `x` could as well be the base.
fn index_scale(data: &Funcdata, x: VarnodeId, depth: usize) -> Option<(intb, intb)> {
    let v = data.vbank().get(x)?;
    if v.is_constant() {
        return None;
    }
    let Some(def) = v.get_def() else {
        return used_as_number(data, x).then_some((1, 0));
    };
    let o = data.obank().get(def)?;
    let konst = |i: int4| -> Option<intb> {
        o.get_in(i)
            .and_then(|v| data.vbank().get(v))
            .filter(|v| v.is_constant())
            .map(|v| sign_extend(v.get_offset(), v.get_size()))
    };
    match o.code() {
        OpCode::CPUI_INT_MULT => {
            let s = konst(1).or_else(|| konst(0))?;
            (s > 0).then_some((s, 0))
        }
        OpCode::CPUI_INT_LEFT => {
            let s = konst(1)?;
            (0..6).contains(&s).then_some((1 << s, 0))
        }
        OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_INT_DIV
        | OpCode::CPUI_INT_REM => Some((1, 0)),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if depth > 0 => {
            let (inner, k) = if let Some(k) = konst(1) {
                (o.get_in(0)?, k)
            } else if o.code() == OpCode::CPUI_INT_ADD {
                (o.get_in(1)?, konst(0)?)
            } else {
                return None;
            };
            let k = if o.code() == OpCode::CPUI_INT_SUB { -k } else { k };
            let (s, k2) = index_scale(data, inner, depth - 1)?;
            Some((s, k + k2))
        }
        OpCode::CPUI_COPY | OpCode::CPUI_CAST if depth > 0 => index_scale(data, o.get_in(0)?, depth - 1),
        _ => used_as_number(data, x).then_some((1, 0)),
    }
}

/// Does the program read `x` as a number somewhere -- multiply, divide, shift
/// or mask it, or order it against a non-zero literal?  Such a value is not the
/// base of the add it also appears in.
fn used_as_number(data: &Funcdata, x: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(x) else { return false };
    v.descend_iter().any(|op| {
        let Some(o) = data.obank().get(op) else { return false };
        match o.code() {
            OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_AND => true,
            _ => false,
        }
    })
}

/// The element type the evidence commits to, or `None` (a refusal it recorded).
fn element_type(data: &Funcdata, ev: &mut Evidence) -> Option<Rc<Datatype>> {
    let w = ev.width?;
    for p in &ev.pointees {
        if p.get_size() != w || !matches!(p.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN) {
            ev.refuse("other-pointee");
            return None;
        }
    }
    let tlst = data.get_arch().types()?;
    let unsigned = ev.unsigned > 0 && ev.signed == 0 && ev.charlit == 0;
    let meta = if unsigned { type_metatype::TYPE_UINT } else { type_metatype::TYPE_INT };
    tlst.get_base(w, meta).ok()
}

fn sign_extend(v: uintb, size: int4) -> intb {
    if size >= 8 {
        return v as intb;
    }
    let bits = (size * 8) as u32;
    let shift = 64 - bits;
    ((v << shift) as intb) >> shift
}

#[cfg(test)]
#[path = "kuna_elemptr/tests.rs"]
mod tests;
