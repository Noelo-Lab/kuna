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

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
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
    /// Loaded elements the function returns where its recovered return type
    /// is signed, and unsigned: the callers read the element at that type.
    ret_signed: u32,
    ret_unsigned: u32,
    /// Loaded elements whose readers, folded the way `getLocalType` folds them,
    /// vote a signed integer of the element's width, and an unsigned one: the
    /// type the load has without this rule.
    own_signed: u32,
    own_unsigned: u32,
    /// Uses of an element as an address: dereferenced, passed where a pointer
    /// is expected, or stored from a pointer.
    address: u32,
    /// Uses of an element as a number: arithmetic, ordering, a non-zero literal.
    number: u32,
    /// The pointer types elements are already known to have: a loaded value's
    /// type, a stored value's type.
    elem_types: Vec<Rc<Datatype>>,
    /// Pointees something else declares or compares this value against.
    pointees: Vec<Rc<Datatype>>,
    /// The use that refused the candidate.
    refused: Option<&'static str>,
    /// The candidate is already a pointer (`malloc`'s `void *`): the other
    /// operand of an add through it is an index, since C adds no two pointers.
    base_is_ptr: bool,
    /// The committed element's sign rests on evidence -- a return type, an
    /// extension or ordering, a character compare, a declaration -- rather than
    /// on the default (see [`element_signed`]).
    firm: bool,
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
                "w={:?} var={} const={} s={} u={} chr={} ret={}/{} own={}/{} adr={} num={}",
                self.width,
                self.variable,
                self.constant,
                self.signed,
                self.unsigned,
                self.charlit,
                self.ret_signed,
                self.ret_unsigned,
                self.own_signed,
                self.own_unsigned,
                self.address,
                self.number
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

/// One `ActionInferTypes` pass's memo: a global is decided once per pass from
/// every Varnode that holds it, and each of those Varnodes asks.
#[derive(Default)]
pub struct Cache {
    globals: HashMap<(u64, int4), Option<Rc<Datatype>>>,
    constants: HashMap<(u64, int4), Option<Rc<Datatype>>>,
}

/// The candidate `T *` for `vn`, or `None` when the rule declines.  `cur` is the
/// vote the ordinary fold produced; the candidate replaces it outright, which is
/// why a vote that already names a pointee is never offered.
pub fn element_pointer(data: &Funcdata, vn: VarnodeId, cur: &Rc<Datatype>, cache: &mut Cache) -> Option<Rc<Datatype>> {
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
    // Storage a function holds in many Varnodes -- a global is read before each
    // call and written back after it, a constant is one Varnode per reader --
    // is decided once per address and pass.
    let memo = if v.is_constant() {
        if !only_indexed(data, vn) {
            return None;
        }
        Some(&mut cache.constants)
    } else if v.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, &spc)) {
        Some(&mut cache.globals)
    } else {
        None
    };
    let Some(memo) = memo else {
        if v.get_size() != ptrsize {
            return None;
        }
        let (kind, copies) = candidate_kind(data, vn)?;
        return decide(data, vn, cur, kind, &copies, ptrsize);
    };
    let key = (v.get_offset(), v.get_size());
    if let Some(hit) = memo.get(&key) {
        return hit.clone();
    }
    let got = candidate_kind(data, vn).and_then(|(kind, copies)| {
        if v.get_size() != ptrsize {
            // A global read at another width than a pointer's is not an array's
            // base in this function, and says so to every other function of the batch.
            if kind == Kind::Global {
                data.kuna_elemptr_note(Obj::Held(v.get_offset()), GlobalVerdict::Refused);
            }
            return None;
        }
        decide(data, vn, cur, kind, &copies, ptrsize)
    });
    memo.insert(key, got.clone());
    got
}

/// Is every reader of the constant `cvn` an add or a `PTRADD` it is a base of?
/// A constant also passed, compared or stored is `globalref`'s to name.
fn only_indexed(data: &Funcdata, cvn: VarnodeId) -> bool {
    data.vbank().get(cvn).is_some_and(|v| {
        v.descend_iter().all(|op| {
            data.obank()
                .get(op)
                .is_some_and(|o| matches!(o.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD))
        })
    })
}

/// Walk `vn` (for a global: every Varnode holding it) and commit or decline.
fn decide(
    data: &Funcdata,
    vn: VarnodeId,
    cur: &Rc<Datatype>,
    kind: Kind,
    copies: &[VarnodeId],
    ptrsize: int4,
) -> Option<Rc<Datatype>> {
    let v = data.vbank().get(vn)?;
    let arch = Rc::clone(data.get_arch());
    let spc = Rc::clone(arch.manage().get_default_data_space()?);
    let own = match kind {
        Kind::Global => Some(Obj::Held(v.get_offset())),
        Kind::Constant => Some(Obj::Table(v.get_offset())),
        _ => None,
    };
    if own.is_some_and(|o| data.kuna_elemptr_blocked(o)) {
        return None;
    }
    let globals_held: Vec<u64> = copies
        .iter()
        .filter_map(|&c| data.vbank().get(c))
        .filter(|c| c.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, &spc)))
        .map(|c| c.get_offset())
        .collect();
    if globals_held.iter().any(|&g| data.kuna_elemptr_blocked(Obj::Held(g))) {
        return None;
    }
    let mut ev = Evidence::default();
    ev.base_is_ptr = cur.get_metatype() == type_metatype::TYPE_PTR;
    match kind {
        Kind::Constant => {
            for c in same_constant(data, vn) {
                walk_constant(data, c, &mut ev);
            }
            // The same address used as another pointer -- passed, stored, copied
            // into a typed local -- says what is there too, and the name
            // `globalref` gives it can have only one type.
            for p in other_pointer_uses(data, vn) {
                ev.pointees.push(p);
            }
        }
        Kind::Global => walk(data, &same_storage(data, vn), &mut ev),
        _ => walk(data, &[vn], &mut ev),
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
    let verdict = match (&elem, ev.refused) {
        (Some(e), _) => Some(GlobalVerdict::Typed { elem: elem_key(e), firm: ev.firm }),
        (None, Some(why)) if !matches!(why, "pointer-unnamed" | "elem-unknown" | "pointer-or-number") => {
            Some(GlobalVerdict::Refused)
        }
        _ => None,
    };
    if let Some(verdict) = verdict {
        match kind {
            Kind::Global => data.kuna_elemptr_note(Obj::Held(v.get_offset()), verdict.clone()),
            Kind::Constant => data.kuna_elemptr_note(Obj::Table(v.get_offset()), verdict.clone()),
            _ => {}
        }
        if elem.is_some() {
            for &g in &globals_held {
                data.kuna_elemptr_note(Obj::Held(g), verdict.clone());
            }
        }
    }
    let elem = elem?;
    if kind == Kind::Constant {
        data.kuna_elemptr_note_constant(v.get_offset());
    }
    arch.types()?.get_type_pointer(ptrsize, elem, spc.get_word_size()).ok()
}

/// Every Varnode of the function that holds the same storage as `vn`: a
/// global is read before a call, written back after it, and joined at a phi,
/// and the walk has to see all of them to know what the function does with it.
fn same_storage(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let Some(v) = data.vbank().get(vn) else { return vec![vn] };
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    let Some(spc) = addr.get_space().cloned() else { return vec![vn] };
    let end = Address::new(spc, addr.get_offset().wrapping_add(1));
    let mut out: Vec<VarnodeId> = data
        .vbank()
        .iter_loc_addr_range(&addr, &end)
        .filter(|&id| data.vbank().get(id).is_some_and(|w| w.get_size() == size && !w.is_constant()))
        .collect();
    if !out.contains(&vn) {
        out.push(vn);
    }
    out
}

/// Which kind of candidate `vn` is, with the copies of a call's returned value,
/// or `None` when the rule does not speak about it.
fn candidate_kind(data: &Funcdata, vn: VarnodeId) -> Option<(Kind, Vec<VarnodeId>)> {
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
        return Some((Kind::Constant, Vec::new()));
    }
    let addr = v.get_addr().clone();
    if v.is_input() && data.get_func_proto().possible_input_param(&addr, v.get_size()) {
        return Some((Kind::Param, Vec::new()));
    }
    if crate::kuna_structsynth::is_call_return(data, vn) {
        let copies = copies_alone(data, vn)?;
        let rets = crate::kuna_structsynth::returned_values(data);
        if crate::kuna_structsynth::returned_beside_others(data, &copies, &rets)
            || returns_another_value_in_place(data, vn, &copies, &rets)
        {
            return None;
        }
        return Some((Kind::CallReturn, copies));
    }
    let data_space = arch.manage().get_default_data_space()?;
    let in_data = addr.get_space().is_some_and(|s| Rc::ptr_eq(s, data_space));
    if in_data && crate::kuna_globalref::in_ranges(&arch.elem_ptr_ranges, v.get_offset()) {
        // A global the image names only by a symbol with no type (an ELF symtab
        // entry) is as open as one it does not name at all.
        let open = match arch.query_container_global(&addr, v.get_size(), &Address::new_invalid()) {
            None => true,
            Some(g) => {
                g.entry_addr.get_offset() == v.get_offset()
                    && g.symbol_type.as_ref().is_none_or(|t| t.get_metatype() == type_metatype::TYPE_UNKNOWN)
            }
        };
        if open {
            return Some((Kind::Global, Vec::new()));
        }
    }
    None
}

/// Does the function return, in the register the call's value arrives in, some
/// value that is not a copy of it?  A register holding two values the function
/// computes is one variable once merging starts (`tar`'s `sub_135e3` returns
/// the record it builds in the `rax` its `char *` scratch name arrived in), and
/// a `char *` vote there would retype the other value too.
fn returns_another_value_in_place(data: &Funcdata, vn: VarnodeId, copies: &[VarnodeId], rets: &[VarnodeId]) -> bool {
    let Some(v) = data.vbank().get(vn) else { return true };
    let (off, size) = (v.get_offset(), v.get_size() as u64);
    let Some(spc) = v.get_addr().get_space().map(|s| s.get_index()) else { return true };
    rets.iter().filter(|&&r| !copies.contains(&r) && !is_null(data, r)).filter_map(|&r| data.vbank().get(r)).any(|r| {
        r.get_addr().get_space().is_some_and(|s| s.get_index() == spc)
            && r.get_offset() < off + size
            && off < r.get_offset() + r.get_size() as u64
    })
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
            if !is_null(data, x) {
                return None;
            }
        }
    }
    Some(seen)
}

/// Is `x` the constant zero, perhaps through a few copies (`v8 = NULL;` on a
/// path that reaches the phi through a frame slot)?
fn is_null(data: &Funcdata, x: VarnodeId) -> bool {
    let mut cur = x;
    for _ in 0..4 {
        let Some(v) = data.vbank().get(cur) else { return false };
        if v.is_constant() {
            return v.get_offset() == 0;
        }
        let Some(o) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
        if o.code() != OpCode::CPUI_COPY {
            return false;
        }
        let Some(src) = o.get_in(0) else { return false };
        cur = src;
    }
    false
}

/// The breadth-first walk from a non-constant candidate.  The second component
/// of a work item is the literal byte offset of that Varnode from the base.
fn walk(data: &Funcdata, starts: &[VarnodeId], ev: &mut Evidence) {
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: VecDeque<(VarnodeId, intb, usize)> = VecDeque::new();
    for &start in starts {
        if seen.insert(start) {
            work.push_back((start, 0, 0));
        }
    }
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

/// Every constant of the function naming the same address as `cvn`, at its
/// width, that is only ever the base of an add.
fn same_constant(data: &Funcdata, cvn: VarnodeId) -> Vec<VarnodeId> {
    let Some(v) = data.vbank().get(cvn) else { return vec![cvn] };
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    let Some(spc) = addr.get_space().cloned() else { return vec![cvn] };
    let end = Address::new(spc, addr.get_offset().wrapping_add(1));
    let mut out: Vec<VarnodeId> = data
        .vbank()
        .iter_loc_addr_range(&addr, &end)
        .filter(|&id| data.vbank().get(id).is_some_and(|w| w.get_size() == size && w.is_constant()))
        .filter(|&id| only_indexed(data, id))
        .collect();
    if !out.contains(&cvn) {
        out.push(cvn);
    }
    out
}

/// The pointees of every other constant of the function naming the same
/// address, where one is typed a pointer at something.
fn other_pointer_uses(data: &Funcdata, cvn: VarnodeId) -> Vec<Rc<Datatype>> {
    let Some(v) = data.vbank().get(cvn) else { return Vec::new() };
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    let Some(spc) = addr.get_space().cloned() else { return Vec::new() };
    let end = Address::new(spc, addr.get_offset().wrapping_add(1));
    data.vbank()
        .iter_loc_addr_range(&addr, &end)
        .filter_map(|id| data.vbank().get(id).map(|w| (id, w)))
        .filter(|(id, w)| w.get_size() == size && w.is_constant() && !only_indexed(data, *id))
        .filter_map(|(_, w)| {
            let t = w.get_type();
            (t.get_metatype() == type_metatype::TYPE_PTR && !points_at_nothing(t)).then(|| t.get_ptr_to()).flatten()
        })
        .collect()
}

/// A constant candidate's evidence: the accesses it is the base of.
fn walk_constant(data: &Funcdata, cvn: VarnodeId, ev: &mut Evidence) {
    let descend: Vec<OpId> = match data.vbank().get(cvn) {
        Some(v) => v.descend_iter().collect(),
        None => return,
    };
    for op in descend {
        let _ = base_use(data, op, cvn, 0, ev);
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
                // The candidate is the index into something else's base.
                ev.refuse("index-of-other");
                return None;
            }
            let elem = konst(2)?;
            match konst(1) {
                Some(i) => out.map(|x| (x, off + i * elem)),
                None => {
                    let (t, c) = inner_stride(data, o.get_in(1)?);
                    indexed(data, out?, elem * t, off + elem * c, ev);
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
                // A constant already typed a pointer is the base, and the
                // candidate the offset into it.
                if !is_sub
                    && o.get_in(other).is_some_and(|c| {
                        data.vn_type_read_facing(c, op).get_metatype() == type_metatype::TYPE_PTR
                    })
                {
                    ev.refuse("index-of-other");
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
            // An operand already known to be a pointer -- by its type, or by a
            // use only a pointer has -- is the base, whatever else the program
            // does with it, and this value is its index.
            if data.vn_type_read_facing(other_vn, op).get_metatype() == type_metatype::TYPE_PTR
                || has_pointer_use(data, other_vn)
            {
                return None;
            }
            let scale = index_scale(data, other_vn, INDEX_DEPTH).or_else(|| ev.base_is_ptr.then_some((1, 0)));
            match scale {
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
            // An element's address kept in a variable already typed a pointer at
            // something (`v9 = &a3[v13]` with `unsigned short *v9`) names the element.
            if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL) {
                if let Some(t) = out.and_then(|x| data.vbank().get(x)).map(|x| Rc::clone(x.get_type())) {
                    if t.get_metatype() == type_metatype::TYPE_PTR && !points_at_nothing(&t) {
                        if let Some(p) = t.get_ptr_to() {
                            ev.pointees.push(p);
                        }
                    }
                }
            }
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
                    let pointer_other = o
                        .get_in(other)
                        .is_some_and(|x| data.vn_type_read_facing(x, op).get_metatype() == type_metatype::TYPE_PTR);
                    match k {
                        Some(_) if pointer_other => ev.refuse("index-of-other"),
                        Some(k) => next = out.map(|x| (x, off + k)),
                        // A second index of the same scale is one more term of
                        // the element's index (`p + i + j`); anything else is a
                        // record's stride or another object's base.
                        None => match o.get_in(other).and_then(|x| index_scale(data, x, INDEX_DEPTH)) {
                            Some((s, k)) if s == scale => next = out.map(|x| (x, off + k)),
                            _ => ev.refuse("sum-plus-unknown"),
                        },
                    }
                }
                OpCode::CPUI_PTRADD => {
                    let elem = o.get_in(2).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant());
                    let idx = o.get_in(1).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant());
                    match (slot, elem, idx) {
                        (0, Some(e), Some(i)) => {
                            let d = sign_extend(i.get_offset(), i.get_size()) * sign_extend(e.get_offset(), e.get_size());
                            next = out.map(|x| (x, off + d));
                        }
                        _ => ev.refuse("index-of-other"),
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
/// `LOAD`/`STORE` `op`.  Reads what the element is: how a loaded value is
/// extended or ordered (its sign), compared (a character), and, for an element
/// as wide as a pointer, whether it is used as an address or as a number.
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
        if let Some(val) = o.get_in(2) {
            stored_value(data, val, ev);
        }
        return;
    }
    let Some(val) = o.get_out() else { return };
    if let Some(t) = data.vbank().get(val).map(|v| Rc::clone(v.get_type())) {
        if t.get_metatype() == type_metatype::TYPE_PTR && !points_at_nothing(&t) {
            ev.elem_types.push(t);
        }
    }
    loaded_sign(data, val, w, ev);
    let readers: Vec<OpId> = data.vbank().get(val).map(|v| v.descend_iter().collect()).unwrap_or_default();
    for r in readers {
        let Some(ro) = data.obank().get(r) else { continue };
        let rslot = ro.get_slot(val);
        let other_const = || {
            ro.get_in(if rslot == 0 { 1 } else { 0 })
                .and_then(|x| data.vbank().get(x))
                .filter(|x| x.is_constant())
                .map(|x| x.get_offset())
        };
        match ro.code() {
            OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_SREM => {
                ev.signed += 1;
                ev.number += 1;
            }
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_REM => {
                ev.unsigned += 1;
                ev.number += 1;
            }
            // Two pointers are ordered unsigned too: only a literal, or another
            // operand already typed, says which this is.
            OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL => {
                ev.unsigned += 1;
                let other = ro.get_in(if rslot == 0 { 1 } else { 0 });
                match other_const() {
                    Some(0) => {}
                    Some(_) => ev.number += 1,
                    None => {
                        if other.is_some_and(|x| data.vn_type_read_facing(x, r).get_metatype() == type_metatype::TYPE_PTR) {
                            ev.address += 1;
                        }
                    }
                }
            }
            OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP => ev.number += 1,
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => match other_const() {
                Some(c) if w == 1 && is_char_literal(c) => ev.charlit += 1,
                Some(0) => {}
                Some(_) => ev.number += 1,
                None => {
                    let other = ro.get_in(if rslot == 0 { 1 } else { 0 });
                    if other.is_some_and(|x| data.vn_type_read_facing(x, r).get_metatype() == type_metatype::TYPE_PTR) {
                        ev.address += 1;
                    }
                }
            },
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE if rslot == 1 => ev.address += 1,
            OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB if rslot == 0 => ev.address += 1,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if rslot > 0 => {
                let t = crate::coreaction_infertypes::input_type_local(data, r, rslot);
                match t.get_metatype() {
                    type_metatype::TYPE_PTR => ev.address += 1,
                    type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL => ev.number += 1,
                    type_metatype::TYPE_FLOAT => ev.refuse("float-elem"),
                    _ => {}
                }
            }
            code if crate::kuna_ptrfromuse::is_float_op(code) => ev.refuse("float-elem"),
            _ => {}
        }
    }
}

/// The sign the loaded element `val` has without this rule: the fold of what
/// its readers vote, and the function's recovered return type where the
/// element is what the function returns.  The return is the one reader whose
/// type C carries past the function: a callee that zero-extends a `short`
/// into `rax` is read as `unsigned short` by every caller, and an element
/// declared signed would make those callers sign-extend it.
fn loaded_sign(data: &Funcdata, val: VarnodeId, w: int4, ev: &mut Evidence) {
    let sign = |t: &Datatype| -> Option<bool> {
        (t.get_size() == w).then_some(()).and_then(|_| match t.get_metatype() {
            type_metatype::TYPE_INT => Some(true),
            type_metatype::TYPE_UINT => Some(false),
            _ => None,
        })
    };
    let Some(v) = data.vbank().get(val) else { return };
    let mut own: Option<Rc<Datatype>> = None;
    for r in v.descend_iter() {
        let slot = data.obank().get(r).map(|o| o.get_slot(val)).unwrap_or(0);
        let t = crate::coreaction_infertypes::input_type_local(data, r, slot);
        own = Some(match own {
            Some(cur) if t.type_order(&cur).unwrap_or(0) >= 0 => cur,
            _ => t,
        });
    }
    match own.as_deref().and_then(sign) {
        Some(true) => ev.own_signed += 1,
        Some(false) => ev.own_unsigned += 1,
        None => {}
    }
    // The element's copies, a few hops on, and the returns they reach.
    let mut seen = vec![val];
    let mut i = 0;
    while i < seen.len() && seen.len() <= 8 {
        let cur = seen[i];
        i += 1;
        let Some(cv) = data.vbank().get(cur) else { continue };
        for r in cv.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            let slot = o.get_slot(cur);
            match o.code() {
                OpCode::CPUI_RETURN if slot >= 1 => {
                    match sign(&crate::coreaction_infertypes::input_type_local(data, r, slot)) {
                        Some(true) => ev.ret_signed += 1,
                        Some(false) => ev.ret_unsigned += 1,
                        None => {}
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_CAST => {
                    if let Some(out) = o.get_out().filter(|x| !seen.contains(x)) {
                        seen.push(out);
                    }
                }
                OpCode::CPUI_INDIRECT if slot == 0 => {
                    if let Some(out) = o.get_out().filter(|x| !seen.contains(x)) {
                        seen.push(out);
                    }
                }
                _ => {}
            }
        }
    }
}

/// What the value a `STORE` writes into an element says about the element.
fn stored_value(data: &Funcdata, val: VarnodeId, ev: &mut Evidence) {
    let Some(v) = data.vbank().get(val) else { return };
    // A value already typed says what sign its element holds; a byte is the
    // plain `char` whatever it is typed.
    if v.get_size() > 1 && !v.is_constant() {
        match v.get_type().get_metatype() {
            type_metatype::TYPE_UINT => ev.unsigned += 1,
            type_metatype::TYPE_INT => ev.signed += 1,
            _ => {}
        }
    }
    match v.get_type().get_metatype() {
        type_metatype::TYPE_FLOAT => {
            ev.refuse("float-elem");
            return;
        }
        type_metatype::TYPE_PTR => {
            ev.address += 1;
            if !points_at_nothing(v.get_type()) {
                ev.elem_types.push(Rc::clone(v.get_type()));
            }
            return;
        }
        _ => {}
    }
    if v.is_constant() {
        if v.get_offset() != 0 {
            ev.number += 1;
        }
        return;
    }
    let Some(o) = v.get_def().and_then(|d| data.obank().get(d)) else { return };
    match o.code() {
        OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_DIV
        | OpCode::CPUI_INT_SDIV
        | OpCode::CPUI_INT_REM
        | OpCode::CPUI_INT_SREM
        | OpCode::CPUI_INT_LEFT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_SUBPIECE
        | OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS => ev.number += 1,
        OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => ev.address += 1,
        code if crate::kuna_ptrfromuse::is_float_op(code) => ev.refuse("float-elem"),
        _ => {}
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
    if is_counter(data, x) {
        return Some((1, 0));
    }
    let o = data.obank().get(def)?;
    let konst = |i: int4| -> Option<intb> {
        o.get_in(i)
            .and_then(|v| data.vbank().get(v))
            .filter(|v| v.is_constant())
            .map(|v| sign_extend(v.get_offset(), v.get_size()))
    };
    match o.code() {
        OpCode::CPUI_INT_MULT => {
            let (s, inner) = match (konst(1), konst(0)) {
                (Some(s), _) => (s, o.get_in(0)?),
                (None, Some(s)) => (s, o.get_in(1)?),
                _ => return None,
            };
            let (t, c) = inner_stride(data, inner);
            (s > 0).then_some((s * t, s * c))
        }
        OpCode::CPUI_INT_LEFT => {
            let s = konst(1)?;
            if !(0..6).contains(&s) {
                return None;
            }
            let (t, c) = inner_stride(data, o.get_in(0)?);
            Some(((1 << s) * t, (1 << s) * c))
        }
        OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_INT_DIV
        | OpCode::CPUI_INT_REM => Some((1, 0)),
        // A small mask bounds an index; a wide one (`& ~0xf`) aligns a pointer.
        OpCode::CPUI_INT_AND => konst(1).or_else(|| konst(0)).filter(|&m| (0..0x10000).contains(&m)).map(|_| (1, 0)),
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

/// `x` as `atom * t + c`, when `x` is itself scaled: `i * 2` is `(2, 0)` and
/// `i * 2 + 1` is `(2, 1)`.  An index `i * 2` scaled again by an 8-byte element
/// walks 16-byte records, and `(i * 2 + 1) * 8` is the second field of one.
fn inner_stride(data: &Funcdata, x: VarnodeId) -> (intb, intb) {
    let Some(o) = data.vbank().get(x).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return (1, 0);
    };
    let konst = |i: int4| -> Option<intb> {
        o.get_in(i)
            .and_then(|v| data.vbank().get(v))
            .filter(|v| v.is_constant())
            .map(|v| sign_extend(v.get_offset(), v.get_size()))
    };
    match o.code() {
        OpCode::CPUI_INT_MULT => match konst(1).or_else(|| konst(0)) {
            Some(t) if t > 1 => (t, 0),
            _ => (1, 0),
        },
        OpCode::CPUI_INT_LEFT => match konst(1) {
            Some(k) if (1..6).contains(&k) => (1 << k, 0),
            _ => (1, 0),
        },
        OpCode::CPUI_INT_ADD => {
            let (Some(c), Some(inner)) = (konst(1), o.get_in(0)) else { return (1, 0) };
            match inner_stride(data, inner) {
                (1, _) => (1, 0),
                (t, c2) => (t, c + c2),
            }
        }
        // `(long)(i * 2)`: the widening keeps the stride.
        OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_COPY | OpCode::CPUI_CAST => match o.get_in(0) {
            Some(inner) => inner_stride(data, inner),
            None => (1, 0),
        },
        _ => (1, 0),
    }
}

/// Is `x` a loop counter: a phi that starts at a small literal and is
/// otherwise only ever itself plus a literal?  `for (i = 0; ...; i++)` at -O0
/// keeps `i` in a frame slot, so the add that indexes with it has two 8-byte
/// operands and nothing else to say which is the base.
fn is_counter(data: &Funcdata, x: VarnodeId) -> bool {
    let through_copies = |mut v: VarnodeId| -> VarnodeId {
        for _ in 0..3 {
            match data.vbank().get(v).and_then(|w| w.get_def()).and_then(|d| data.obank().get(d)) {
                Some(o) if o.code() == OpCode::CPUI_COPY => match o.get_in(0) {
                    Some(src) => v = src,
                    None => break,
                },
                _ => break,
            }
        }
        v
    };
    let phi = through_copies(x);
    let Some(o) = data.vbank().get(phi).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return false;
    };
    if o.code() != OpCode::CPUI_MULTIEQUAL {
        return false;
    }
    let mut start = false;
    for k in 0..o.num_input() {
        let Some(input) = o.get_in(k).map(through_copies) else { return false };
        let Some(iv) = data.vbank().get(input) else { return false };
        if iv.is_constant() {
            if iv.get_offset() >= 0x10000 {
                return false;
            }
            start = true;
            continue;
        }
        let Some(step) = iv.get_def().and_then(|d| data.obank().get(d)) else { return false };
        let steps_self = matches!(step.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB)
            && step.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant())
            && step.get_in(0).map(through_copies) == Some(phi);
        if !steps_self {
            return false;
        }
    }
    start
}

/// Is `x` dereferenced, or passed where a callee declares a pointer?  A value
/// used that way is an address, and cannot be the index of the add it is in.
fn has_pointer_use(data: &Funcdata, x: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(x) else { return false };
    v.descend_iter().any(|op| {
        let Some(o) = data.obank().get(op) else { return false };
        let slot = o.get_slot(x);
        match o.code() {
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE => slot == 1,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if slot > 0 => {
                crate::coreaction_infertypes::declared_input_type_local(data, op, slot).get_metatype()
                    == type_metatype::TYPE_PTR
            }
            _ => false,
        }
    })
}

/// Does the program read `x` as a number somewhere -- multiply, divide or shift
/// it, or order it against a non-zero literal?  Such a value is not the base of
/// the add it also appears in.  A mask is not such a use: `p & 7` is how a
/// program tests a pointer's alignment.
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
            | OpCode::CPUI_INT_SRIGHT => true,
            // Ordered against a literal other than zero: a count, not an address.
            OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL => (0..2).any(|i| {
                o.get_in(i)
                    .filter(|&c| c != x)
                    .and_then(|c| data.vbank().get(c))
                    .is_some_and(|c| c.is_constant() && c.get_offset() != 0 && c.get_offset() < 0x10000)
            }),
            _ => false,
        }
    })
}

/// The element type the evidence commits to, or `None` (a refusal it recorded).
fn element_type(data: &Funcdata, ev: &mut Evidence) -> Option<Rc<Datatype>> {
    let w = ev.width?;
    let arch = data.get_arch();
    let tlst = arch.types()?;
    let ptrsize = arch.manage().get_default_data_space()?.get_addr_size() as int4;
    // What another pointer at this object says the element is.  An integer of
    // the element's width agrees; a pointer is a pointer element, whose type it
    // names when every such pointee names the same one; anything else -- a
    // record, a float, another width -- is not this array.
    let mut named: Option<Rc<Datatype>> = None;
    let mut named_agree = true;
    let mut declared_int: Option<Rc<Datatype>> = None;
    let mut declared_agree = true;
    for t in &ev.elem_types {
        match &named {
            None => named = Some(Rc::clone(t)),
            Some(n) if n.type_order(t).map(|o| o == 0).unwrap_or(false) => {}
            Some(_) => named_agree = false,
        }
    }
    let pointees = std::mem::take(&mut ev.pointees);
    for p in &pointees {
        if p.get_size() != w {
            ev.refuse("other-pointee");
            return None;
        }
        match p.get_metatype() {
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => {
                if p.get_metatype() != type_metatype::TYPE_UNKNOWN {
                    if w == ptrsize {
                        ev.number += 1;
                    }
                    match &declared_int {
                        None => declared_int = Some(Rc::clone(p)),
                        Some(d) if d.type_order(p).map(|o| o == 0).unwrap_or(false) => {}
                        Some(_) => declared_agree = false,
                    }
                }
            }
            type_metatype::TYPE_PTR if w == ptrsize => {
                ev.address += 1;
                match &named {
                    None => named = Some(Rc::clone(p)),
                    Some(n) if n.type_order(p).map(|o| o == 0).unwrap_or(false) => {}
                    Some(_) => named_agree = false,
                }
            }
            _ => {
                ev.refuse("other-pointee");
                return None;
            }
        }
    }
    // An element as wide as a pointer is a pointer or a number, and only its
    // uses say which: both, or neither, declines.
    if w == ptrsize {
        match (ev.address > 0, ev.number > 0) {
            // A pointer element is committed only to a pointee something else
            // names: `void **` would say less than a caller's `char **` argv,
            // which `calleevote` states on a later decompile.
            (true, false) => {
                if let Some(n) = named.filter(|_| named_agree) {
                    ev.firm = true;
                    return Some(n);
                }
                ev.refuse("pointer-unnamed");
                return None;
            }
            (false, true) => {}
            (true, true) => {
                ev.refuse("pointer-or-number");
                return None;
            }
            (false, false) => {
                ev.refuse("elem-unknown");
                return None;
            }
        }
    } else if ev.address > 0 {
        ev.refuse("narrow-address");
        return None;
    }
    // A callee that declares the element (`strftime`'s `char *`) says what its
    // sign is; the loads only guess.
    if let Some(d) = declared_int.filter(|_| declared_agree) {
        ev.firm = true;
        return Some(d);
    }
    let (signed, firm) = element_signed(w, ev);
    ev.firm = firm;
    let meta = if signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
    tlst.get_base(w, meta).ok()
}

/// Is the `w`-byte integer element signed, and does that rest on evidence?
/// Only where something says so.  The function's return type comes first,
/// since callers read it; then how the program extends, orders, shifts or
/// divides a loaded element, when every such use agrees; a byte compared
/// against a character is the plain `char`.  Without any of that the element
/// takes the type the load has without this rule -- its readers' fold -- and
/// a wider element nothing votes for is unsigned, the one reading that never
/// widens a value past what its bits hold.  Such a default is not evidence: a
/// batch lets a function with evidence decide a table's sign over it.
fn element_signed(w: int4, ev: &Evidence) -> (bool, bool) {
    let one = |s: u32, u: u32| -> Option<bool> {
        match (s > 0, u > 0) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            _ => None,
        }
    };
    if let Some(signed) = one(ev.ret_signed, ev.ret_unsigned) {
        return (signed, true);
    }
    if w == 1 {
        if ev.charlit > 0 {
            return (true, true);
        }
        return match one(ev.signed, ev.unsigned) {
            Some(signed) => (signed, true),
            None => (true, false),
        };
    }
    match one(ev.signed, ev.unsigned) {
        Some(signed) => (signed, true),
        None => (one(ev.own_signed, ev.own_unsigned).unwrap_or(false), false),
    }
}

/// (printer) When the constant `vn` is the base of the `PTRADD` `op` indexed
/// by a computed value, and this rule typed it: `Some` of the largest index it
/// can take, `None` inside when nothing bounds it.  The outer `None` means `vn`
/// is not such a base.
pub fn literal_index_bound(data: &Funcdata, op: OpId, vn: VarnodeId) -> Option<Option<uintb>> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_PTRADD || o.get_in(0) != Some(vn) {
        return None;
    }
    // Only a table this rule typed: a character-array constant another pass
    // recovered keeps the spelling it had.
    if !data.kuna_elemptr_typed_constant(data.vbank().get(vn)?.get_offset()) {
        return None;
    }
    let idx = o.get_in(1)?;
    if data.vbank().get(idx)?.is_constant() {
        return None;
    }
    Some(index_bound(data, idx, INDEX_DEPTH))
}

/// The largest value the index `x` can hold, when its definition says: a
/// zero-extended narrower value, a mask, an unsigned remainder.
fn index_bound(data: &Funcdata, x: VarnodeId, depth: usize) -> Option<uintb> {
    let o = data.vbank().get(x)?.get_def().and_then(|d| data.obank().get(d))?;
    let konst = |i: int4| -> Option<uintb> {
        o.get_in(i).and_then(|v| data.vbank().get(v)).filter(|v| v.is_constant()).map(|v| v.get_offset())
    };
    match o.code() {
        OpCode::CPUI_INT_ZEXT => {
            let s = data.vbank().get(o.get_in(0)?)?.get_size();
            (s < 8).then(|| (1u64 << (8 * s as u32)) - 1)
        }
        OpCode::CPUI_INT_AND => konst(1).or_else(|| konst(0)),
        OpCode::CPUI_INT_REM => konst(1).filter(|&m| m > 0).map(|m| m - 1),
        OpCode::CPUI_COPY | OpCode::CPUI_CAST if depth > 0 => index_bound(data, o.get_in(0)?, depth - 1),
        _ => None,
    }
}

/// What a verdict is about: the pointer a global holds (`dat_5068`, typed
/// `char *`), or the table at a constant address (`dat_5020[v7]`, declared
/// `unsigned char dat_5020[]`).  Both end up as one declaration in a project's
/// header, which every function's body is compiled against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Obj {
    Held(u64),
    Table(u64),
}

/// What one function's walks said about one global or table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalVerdict {
    /// An array of `elem`, spelled by [`elem_key`]: signedness included, since
    /// a body reading the element at the other sign computes another value.
    /// `firm` when evidence decided the sign, not the default.
    Typed { elem: String, firm: bool },
    /// Not an array this function agrees with: another width, a record, a
    /// number.
    Refused,
}

/// The element type `e` as a verdict compares it: width, kind, sign, and a
/// pointer's pointee.
pub fn elem_key(e: &Datatype) -> String {
    match e.get_ptr_to() {
        Some(p) if e.get_metatype() == type_metatype::TYPE_PTR => format!("*{}", elem_key(&p)),
        _ => format!("{}:{:?}:{}", e.get_size(), e.get_metatype(), e.get_name()),
    }
}

impl GlobalVerdict {
    /// Two verdicts of one function: the same array, or a disagreement.
    pub fn merge(self, other: GlobalVerdict) -> GlobalVerdict {
        match (self, other) {
            (GlobalVerdict::Typed { elem: a, firm: fa }, GlobalVerdict::Typed { elem: b, firm: fb }) if a == b => {
                GlobalVerdict::Typed { elem: a, firm: fa || fb }
            }
            _ => GlobalVerdict::Refused,
        }
    }
}

/// (kuna `elemptr`) The batch's record of what every function said about every
/// global and table, and the ones each function must not type.  A global is
/// typed only where every function of the batch that says something about it
/// agrees -- on the element, sign included; a table only where every function
/// that types it types the same element (a function that reads it another way
/// prints its own cast, and compiles against any declaration).  The functions
/// that typed an object another function disagrees about are decompiled again
/// with it blocked.
#[derive(Debug, Default)]
pub struct Ledger {
    /// A batch is running and recording.
    pub recording: bool,
    verdicts: BTreeMap<Obj, BTreeMap<u64, GlobalVerdict>>,
    blocked: BTreeMap<u64, Rc<BTreeSet<Obj>>>,
    /// Objects some function of the batch has already disagreed about: every
    /// function decompiled after that leaves them alone, so only the ones
    /// decompiled before it are decided again.
    disputed: BTreeSet<Obj>,
}

/// Start a batch: forget the previous one, and record while the option is on.
pub fn start(arch: &mut crate::architecture::Architecture) {
    arch.kuna_elemptr = Ledger { recording: arch.elem_ptr, ..Ledger::default() };
}

/// End a batch's recording; the blocked sets stay for any later redo.
pub fn stop(arch: &mut crate::architecture::Architecture) {
    arch.kuna_elemptr.recording = false;
}

/// Hand a function the objects it must not type.
pub fn seed(arch: &crate::architecture::Architecture, data: &mut Funcdata) {
    if !arch.elem_ptr {
        return;
    }
    let me = data.get_address().get_offset();
    let ledger = &arch.kuna_elemptr;
    let own = ledger.blocked.get(&me);
    if ledger.disputed.is_empty() {
        if let Some(b) = own {
            data.kuna_set_elemptr_blocked(Some(Rc::clone(b)));
        }
        return;
    }
    let mut all = ledger.disputed.clone();
    if let Some(b) = own {
        all.extend(b.iter().copied());
    }
    data.kuna_set_elemptr_blocked(Some(Rc::new(all)));
}

/// File what a decompiled function said about each object, replacing what an
/// earlier decompile of the same function said.
pub fn record(arch: &mut crate::architecture::Architecture, data: &Funcdata) {
    if !arch.kuna_elemptr.recording {
        return;
    }
    arch.kuna_elemptr.file(data.get_address().get_offset(), data.kuna_elemptr_verdicts());
}

/// The functions to decompile again: every one that typed an object some other
/// function disagrees about, with that object now blocked for it.
pub fn disagreements(arch: &mut crate::architecture::Architecture) -> BTreeSet<u64> {
    let redo = arch.kuna_elemptr.disagreements();
    if trace_on() {
        eprintln!("[elemptr] batch disagreements: {} functions to redo", redo.len());
    }
    redo
}

/// The functions among `per` that must not type `obj`, or none when they
/// agree.  Two elements decided by evidence that differ block every function
/// that typed one; so do two defaulted ones, and, for a global, a refusal beside
/// a type.  Where evidence decided one element, only the functions that
/// defaulted to another are blocked: a default is what the load reads without
/// this rule, and it gives way.
fn to_block(obj: Obj, per: &BTreeMap<u64, GlobalVerdict>) -> Vec<u64> {
    let typed: Vec<(u64, &str, bool)> = per
        .iter()
        .filter_map(|(&f, v)| match v {
            GlobalVerdict::Typed { elem, firm } => Some((f, elem.as_str(), *firm)),
            GlobalVerdict::Refused => None,
        })
        .collect();
    let all = || typed.iter().map(|t| t.0).collect::<Vec<u64>>();
    if typed.is_empty() {
        return Vec::new();
    }
    if matches!(obj, Obj::Held(_)) && per.values().any(|x| *x == GlobalVerdict::Refused) {
        return all();
    }
    let firm: BTreeSet<&str> = typed.iter().filter(|t| t.2).map(|t| t.1).collect();
    match firm.len() {
        0 => {
            let weak: BTreeSet<&str> = typed.iter().map(|t| t.1).collect();
            if weak.len() > 1 {
                all()
            } else {
                Vec::new()
            }
        }
        1 => {
            let decided = firm.first().copied().unwrap_or_default();
            typed.iter().filter(|t| t.1 != decided).map(|t| t.0).collect()
        }
        _ => all(),
    }
}

impl Ledger {
    /// Replace what the function entered at `me` said about every object.
    pub(crate) fn file(&mut self, me: u64, said: BTreeMap<Obj, GlobalVerdict>) {
        for per in self.verdicts.values_mut() {
            per.remove(&me);
        }
        for (g, v) in said {
            let per = self.verdicts.entry(g).or_default();
            per.insert(me, v);
            // A function decompiled after a dispute leaves the object alone;
            // only a dispute that blocks everyone who typed it is one.
            let refused_global = matches!(g, Obj::Held(_)) && per.values().any(|x| *x == GlobalVerdict::Refused);
            let typed = per.values().filter(|x| matches!(x, GlobalVerdict::Typed { .. })).count();
            if refused_global || (typed > 0 && to_block(g, per).len() == typed) {
                self.disputed.insert(g);
            }
        }
    }

    /// Block every disagreed object for the functions that typed it, and return
    /// the functions whose blocked set grew.
    pub(crate) fn disagreements(&mut self) -> BTreeSet<u64> {
        let mut redo = BTreeSet::new();
        for (&g, per) in &self.verdicts {
            for f in to_block(g, per) {
                let entry = self.blocked.entry(f).or_default();
                if !entry.contains(&g) {
                    Rc::make_mut(entry).insert(g);
                    redo.insert(f);
                }
            }
        }
        redo
    }

    /// Is `obj` blocked for the function entered at `me`?
    #[cfg(test)]
    pub(crate) fn is_blocked(&self, me: u64, obj: Obj) -> bool {
        self.blocked.get(&me).is_some_and(|b| b.contains(&obj))
    }
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
