//! A pointer the program only ever uses on characters is a `char *` (kuna `charptr`, P5).
//!
//! `ActionInferTypes` seeds every Varnode from local op semantics and then pushes
//! the seeds along the def-use graph.  Both halves lose a string pointer the moment
//! the value stops being the call argument itself:
//!
//! ```text
//!   void sub_2b45(unsigned long a0, long a1, bool a2)   // coreutils basename -O0
//!   {                                                   // perform_basename(char *, char *, bool)
//!     v2 = (char *)sub_2fd6(a0);
//!     ...
//!   }
//! ```
//!
//! `sub_2fd6` is declared `char *sub_2fd6(char *)` by the time `a0` is typed, so
//! the fact exists — but `TypeOpCall::getInputLocal` states it about the Varnode
//! that is *the call's operand*, and every hop between the parameter and that
//! operand (an `-O0` spill through the frame, a `MULTIEQUAL`, a `p + k`) is a hop
//! `propagateTypeEdge` will not carry a pointer back over.
//!
//! [`char_pointer_from_evidence`] collects that evidence directly, the way
//! [`crate::kuna_ptrfromuse`] collects "this is a pointer at all": a bounded
//! breadth-first walk over the candidate's transitive descendants through
//! copy-like identity and constant-offset addition, asking of every terminal use
//! whether it is about characters.  Three uses say yes:
//!
//! * **`libc`** — the value reaches argument *i* of a call whose callee has a
//!   DECLARED `char *` there.  Declared means stated from outside the decompile:
//!   a `libproto`/`libcsigs` signature, a `libctypes` shell, DWARF, a demangled
//!   name, `--assert prototype`, or the per-call-site override `formatstring`
//!   installs for a resolved `%s`.  A type another *recovery* voted for is not
//!   evidence here ([`crate::coreaction_infertypes::declared_input_type_local`]).
//! * **`uses`** — the value is dereferenced, and every dereference through it is
//!   one byte wide (`LOAD`/`STORE` of size 1, `PTRADD` of element size 1).
//! * **`uses`** — the value is defined by, or merges with, a constant that
//!   resolves to a NUL-terminated character array in the image.
//!
//! The candidate is one more vote in the same `getLocalType` fold, folded by
//! [`Datatype::type_order`], and it may also *refine* a pointer that points at
//! nothing: `void *` and `undefined1 *` become `char *`, while a pointer to a
//! named or sized thing — `FILE *`, `stat *`, `long *`, a synthesized
//! `struct_3 *` — is left exactly as it was.  A type-locked Varnode and a
//! Varnode seeded from a type-locked symbol are never touched.
//!
//! The walk refuses outright on any use a character pointer does not survive:
//! a dereference wider than a byte, a `PTRADD` whose element is wider, a call
//! argument the callee has declared as something other than a character pointer,
//! and everything [`crate::kuna_ptrfromuse`] refuses (integer arithmetic, shifts,
//! float ops, a comparison against a non-zero literal).  One refusal anywhere
//! withdraws the candidate, so the rule speaks only where the whole function
//! agrees.
//!
//! Only storage a reader sees declared is considered: a function input the
//! prototype model could place a parameter in, or a Varnode in the stack space.
//! A register temporary is neither a parameter nor a declaration.
//!
//! Gated by [`Architecture::char_ptr`](crate::architecture::Architecture)
//! (option `charptr off|libc|uses`); with the option off nothing here is reachable.

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// How many def-use hops the walk follows before giving up; the same bound
/// [`crate::kuna_ptrfromuse`] uses.
const HOP_CAP: usize = 10;

/// (kuna) Which evidence may commit a pointer to `char *`: `charptr off|libc|uses`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharPtrMode {
    /// Upstream: nothing in this module runs.
    #[default]
    Off,
    /// Only a callee's DECLARED `char *` parameter.
    Libc,
    /// Declared parameters, plus byte-only dereference and string constants.
    Uses,
}

impl CharPtrMode {
    /// Is the rule active at all?
    pub fn is_on(self) -> bool {
        self != CharPtrMode::Off
    }

    /// Does this strength weigh how the value is dereferenced?
    fn reads_uses(self) -> bool {
        self == CharPtrMode::Uses
    }
}

/// (kuna) Parse `option charptr off|libc|uses`; the caller writes the live field.
pub struct OptionCharPtr;

impl OptionCharPtr {
    /// The option name.
    pub const NAME: &'static str = "charptr";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(CharPtrMode, String)> {
        let mode = match p1 {
            "off" => CharPtrMode::Off,
            "libc" => CharPtrMode::Libc,
            "uses" => CharPtrMode::Uses,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown charptr value: {other} (expected off|libc|uses)"
                )))
            }
        };
        Ok((mode, format!("Character-pointer evidence set to {p1}")))
    }
}

/// What the walk found.  Kept per kind so the `KUNA_CHARPTR_CENSUS` dump can
/// report which evidence a candidate rests on, and which use refused it.
#[derive(Default)]
pub(crate) struct Evidence {
    /// Reaches a call argument the callee DECLARES `char *`.
    pub libc: u32,
    /// ...and that call is a `formatstring`-resolved `%s` conversion.
    pub fmt: u32,
    /// Dereferenced, and every dereference through it is one byte wide.
    pub byte: u32,
    /// Defined by, or compared against, a character-array constant.
    pub strconst: u32,
    /// The use that withdrew the candidate, if any.
    pub refused: Option<&'static str>,
}

impl Evidence {
    /// Does the collected evidence commit the value to `char *`?
    fn commits(&self) -> bool {
        self.refused.is_none() && (self.libc + self.byte + self.strconst) > 0
    }

    /// The census label: which evidence this candidate rests on.
    fn label(&self) -> String {
        if let Some(r) = self.refused {
            return format!("refuse:{r}");
        }
        let mut parts: Vec<&str> = Vec::new();
        if self.fmt > 0 {
            parts.push("fmt");
        }
        if self.libc > self.fmt {
            parts.push("libc");
        }
        if self.byte > 0 {
            parts.push("byte");
        }
        if self.strconst > 0 {
            parts.push("str");
        }
        if parts.is_empty() {
            "none".to_string()
        } else {
            parts.join("+")
        }
    }
}

/// Is the `KUNA_CHARPTR_CENSUS` dump on?  Read once; it prints one line per
/// candidate the rule considers and changes nothing about the decompile.
fn census_on() -> bool {
    use std::sync::OnceLock;
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("KUNA_CHARPTR_CENSUS").is_some())
}

/// What one reader of a walked Varnode tells us.
enum Use {
    /// This value is used on characters.
    Char(CharKind),
    /// Used on characters, and the same address travels on from this Varnode.
    CharForward(CharKind, VarnodeId),
    /// Identity or constant-offset addition: keep walking from this Varnode.
    Forward(VarnodeId),
    /// Nothing either way.
    Neutral,
    /// Inconsistent with a character pointer: the whole candidate is refused,
    /// with the reason the census dump reports.
    Refuse(&'static str),
}

/// Is `t` the one-byte ASCII `char` core type?
fn is_char(t: &Datatype) -> bool {
    t.get_size() == 1 && t.get_metatype() == type_metatype::TYPE_INT && t.is_ascii()
}

/// Is `t` a pointer at a single character?
fn is_char_pointer(t: &Datatype) -> bool {
    t.get_metatype() == type_metatype::TYPE_PTR
        && t.get_ptr_to().as_deref().is_some_and(is_char)
}

/// Does `cur` point at nothing this rule would be overriding?
///
/// `void *` and a pointer to one unknown byte are placeholders: they say the
/// value is an address and nothing about what is there, which is exactly what
/// this rule can supply.  A pointer at anything else — a named aggregate, a
/// sized primitive, a synthesized struct — is a claim, and a claim outranks
/// use evidence.
fn points_at_nothing(cur: &Datatype) -> bool {
    match cur.get_ptr_to() {
        Some(p) => {
            p.get_metatype() == type_metatype::TYPE_VOID
                || (p.get_metatype() == type_metatype::TYPE_UNKNOWN && p.get_size() <= 1)
        }
        None => false,
    }
}

/// May this rule speak about `vn` at all?
///
/// A function input the prototype model could place a parameter in, or a
/// Varnode in the stack space: the two kinds of storage that reach the reader
/// as a declaration.  A type-locked Varnode already carries a definitive type.
fn is_declared_storage(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_type_lock() || v.is_annotation() {
        return false;
    }
    let addr = v.get_addr().clone();
    let size = v.get_size();
    if v.is_input() && data.get_func_proto().possible_input_param(&addr, size) {
        return true;
    }
    let arch = data.get_arch();
    match (addr.get_space(), arch.manage().get_stack_space()) {
        (Some(s), Some(stack)) => Rc::ptr_eq(s, stack),
        _ => false,
    }
}

/// The candidate `char *` data-type for `vn`, or `None` when the rule declines.
///
/// `cur` is the vote the ordinary fold produced, and is consulted only to refuse
/// a pointer that already names what it points at.
pub fn char_pointer_from_evidence(
    data: &Funcdata,
    vn: VarnodeId,
    cur: &Rc<Datatype>,
    mode: CharPtrMode,
) -> Option<Rc<Datatype>> {
    if !mode.is_on() {
        return None;
    }
    if cur.get_metatype() == type_metatype::TYPE_PTR && !points_at_nothing(cur) {
        return None;
    }
    if !is_declared_storage(data, vn) {
        return None;
    }
    let ptrsize = data.vbank().get(vn)?.get_size();
    let arch = Rc::clone(data.get_arch());
    let spc = Rc::clone(arch.manage().get_default_data_space()?);
    // Only a pointer-width value can be a pointer; `get_type_pointer` would mint a
    // four-byte pointer on a 64-bit image otherwise.
    if ptrsize != spc.get_addr_size() as int4 {
        return None;
    }
    let ev = walk_is_char(data, vn, mode);
    if census_on() {
        let v = data.vbank().get(vn)?;
        eprintln!(
            "CHARPTR fn={:#x} {} size={} cur={} ev={}",
            data.get_address().get_offset(),
            if v.is_input() { format!("input@{}", v.get_addr().get_offset()) } else { format!("stack@{}", v.get_addr().get_offset() as i64) },
            ptrsize,
            cur.get_name(),
            ev.label()
        );
    }
    if !ev.commits() {
        return None;
    }
    let tlst = arch.types()?;
    let ch = tlst.get_base(1, type_metatype::TYPE_INT).ok()?;
    if !is_char(&ch) {
        return None;
    }
    tlst.get_type_pointer(ptrsize, ch, spc.get_word_size()).ok()
}

/// Bounded breadth-first walk over the transitive descendants of `start`,
/// collecting what every use says about the element type.
fn walk_is_char(data: &Funcdata, start: VarnodeId, mode: CharPtrMode) -> Evidence {
    let mut ev = Evidence::default();
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: VecDeque<(VarnodeId, usize)> = VecDeque::new();
    work.push_back((start, 0));
    seen.insert(start);
    while let Some((vn, hops)) = work.pop_front() {
        if mode.reads_uses() && defined_by_string_constant(data, vn) {
            ev.strconst += 1;
        }
        let descend: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            let mut forward = None;
            match classify_use(data, op, vn, mode) {
                Use::Refuse(why) => {
                    ev.refused = Some(why);
                    if !census_on() {
                        return ev;
                    }
                }
                Use::Char(kind) => count(&mut ev, kind, data, op),
                Use::Neutral => {}
                Use::CharForward(kind, next) => {
                    count(&mut ev, kind, data, op);
                    forward = Some(next);
                }
                Use::Forward(next) => forward = Some(next),
            }
            if let Some(next) = forward {
                if hops + 1 < HOP_CAP && seen.insert(next) {
                    work.push_back((next, hops + 1));
                }
            }
        }
    }
    ev
}

/// Which use said "characters".
#[derive(Clone, Copy)]
enum CharKind {
    /// A callee's declared `char *` parameter.
    Declared,
    /// A one-byte dereference or a one-byte element step.
    Byte,
    /// A character-array constant.
    Str,
}

/// Tally one piece of evidence, splitting a declared `char *` that came from a
/// `formatstring`-resolved call site out of the plain libc one.
fn count(ev: &mut Evidence, kind: CharKind, data: &Funcdata, op: OpId) {
    match kind {
        CharKind::Declared => {
            ev.libc += 1;
            if is_format_site(data, op) {
                ev.fmt += 1;
            }
        }
        CharKind::Byte => ev.byte += 1,
        CharKind::Str => ev.strconst += 1,
    }
}

/// Is `op` a call whose prototype came from a resolved format string?  Only the
/// census label needs the distinction, so the scan is behind that flag.
fn is_format_site(data: &Funcdata, op: OpId) -> bool {
    if !census_on() {
        return false;
    }
    (0..data.num_calls()).any(|i| {
        let fc = data.get_call_specs(i);
        fc.get_op() == op && fc.format_arity().is_some()
    })
}

/// Classify what `op` does with `vn`.
fn classify_use(data: &Funcdata, op: OpId, vn: VarnodeId, mode: CharPtrMode) -> Use {
    let Some(o) = data.obank().get(op) else { return Use::Neutral };
    let opcode = o.code();
    let slot = o.get_slot(vn);
    let out = o.get_out();
    match opcode {
        // A dereference through this value.  One byte is the accept condition;
        // anything wider is a different element and refuses the candidate.
        OpCode::CPUI_LOAD => {
            if slot != 1 {
                return Use::Neutral;
            }
            if !mode.reads_uses() {
                return Use::Neutral;
            }
            match out.and_then(|o| data.vbank().get(o)).map(|v| v.get_size()) {
                Some(1) => Use::Char(CharKind::Byte),
                Some(_) => Use::Refuse("load-wider"),
                None => Use::Neutral,
            }
        }
        OpCode::CPUI_STORE => {
            if slot != 1 {
                // The stored VALUE being this pointer says nothing about it.
                return Use::Neutral;
            }
            if !mode.reads_uses() {
                return Use::Neutral;
            }
            match o.get_in(2).and_then(|v| data.vbank().get(v)).map(|v| v.get_size()) {
                Some(1) => Use::Char(CharKind::Byte),
                Some(_) => Use::Refuse("store-wider"),
                None => Use::Neutral,
            }
        }
        // Copy-like identity: the same address travels on.
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_CAST => {
            match out {
                Some(o) => Use::Forward(o),
                None => Use::Neutral,
            }
        }
        // `PTRADD(base, index, elemsize)`: the element size is the pointee's width.
        OpCode::CPUI_PTRADD => {
            if slot != 0 {
                return Use::Neutral;
            }
            let elem = o
                .get_in(2)
                .and_then(|v| data.vbank().get(v))
                .filter(|v| v.is_constant())
                .map(|v| v.get_offset());
            match (elem, out) {
                // Indexing by one byte is character indexing; the element address
                // travels on, so `strlen(p + i)` still counts.
                (Some(1), Some(o)) if mode.reads_uses() => Use::CharForward(CharKind::Byte, o),
                (Some(1), None) if mode.reads_uses() => Use::Char(CharKind::Byte),
                (Some(1), _) => Use::Neutral,
                (Some(_), _) => Use::Refuse("elem-wider"),
                (None, _) => Use::Neutral,
            }
        }
        OpCode::CPUI_PTRSUB => match (slot, out) {
            (0, Some(o)) => Use::Forward(o),
            _ => Use::Neutral,
        },
        // `p + k` with a literal k is still the same object, unless the literal is
        // itself the address of a mapped global object — then the constant is the
        // base and the walked value is an index into it.  The grading is
        // `kuna_ptrfromuse`'s, and this rule reuses it.
        OpCode::CPUI_INT_ADD => {
            let other = if slot == 0 { 1 } else { 0 };
            let Some(other_vn) = o.get_in(other) else { return Use::Neutral };
            let is_const =
                data.vbank().get(other_vn).map(|v| v.is_constant()).unwrap_or(false);
            if !is_const {
                return Use::Neutral;
            }
            if crate::kuna_ptrfromuse::constant_is_global_base(data, op, other_vn) {
                return Use::Refuse("global-base");
            }
            match out {
                Some(o) => Use::Forward(o),
                None => Use::Neutral,
            }
        }
        // What the callee has DECLARED for this argument.  A character pointer is
        // the evidence; a pointer at something else, or a committed non-pointer,
        // contradicts it.
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            if slot <= 0 {
                return Use::Neutral;
            }
            let declared =
                crate::coreaction_infertypes::declared_input_type_local(data, op, slot);
            if is_char_pointer(&declared) {
                return Use::Char(CharKind::Declared);
            }
            let meta = declared.get_metatype();
            if meta == type_metatype::TYPE_PTR {
                return if points_at_nothing(&declared) {
                    Use::Neutral
                } else {
                    Use::Refuse("callee-other-pointer")
                };
            }
            if meta == type_metatype::TYPE_UNKNOWN {
                return Use::Neutral;
            }
            Use::Refuse("callee-scalar")
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
        | OpCode::CPUI_PIECE => Use::Refuse("arithmetic"),
        // A comparison against a non-zero literal is a value test, not a null test.
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL => {
            let other = if slot == 0 { 1 } else { 0 };
            let Some(other_vn) = o.get_in(other) else { return Use::Neutral };
            let Some(ov) = data.vbank().get(other_vn) else { return Use::Neutral };
            if !ov.is_constant() {
                return Use::Neutral;
            }
            if ov.get_offset() == 0 {
                return Use::Neutral;
            }
            if mode.reads_uses() && constant_is_string(data, op, other_vn) {
                return Use::Char(CharKind::Str);
            }
            Use::Refuse("compare-literal")
        }
        _ => {
            if crate::kuna_ptrfromuse::is_float_op(opcode) {
                Use::Refuse("float")
            } else {
                Use::Neutral
            }
        }
    }
}

/// Is `vn` written by a COPY or MULTIEQUAL whose source is a constant that
/// resolves to a character array in the image?  `char *p = "literal";` is the
/// plainest statement a program makes about a pointer's element type.
fn defined_by_string_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.is_written() {
        return false;
    }
    let Some(def) = v.get_def() else { return false };
    let Some(o) = data.obank().get(def) else { return false };
    if !matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL) {
        return false;
    }
    (0..o.num_input()).any(|i| {
        o.get_in(i)
            .and_then(|iv| data.vbank().get(iv))
            .map(|c| c.is_constant())
            .unwrap_or(false)
            && o.get_in(i).is_some_and(|iv| constant_is_string(data, def, iv))
    })
}

/// Does the constant `cvn` resolve to a NUL-terminated character array kuna has
/// already recovered at that address?  The resolution is
/// `ActionConstantPtr::isPointer`'s (`coreaction.cc:1167`), and the answer is
/// the covering global symbol's own data-type.
fn constant_is_string(data: &Funcdata, op: OpId, cvn: VarnodeId) -> bool {
    let glb = data.get_arch();
    let Some(v) = data.vbank().get(cvn) else { return false };
    let (off, size) = (v.get_offset(), v.get_size());
    let Some(spc) = glb.manage().get_default_data_space().map(Rc::clone) else { return false };
    if spc.get_pointer_lower_bound() > off || spc.get_pointer_upper_bound() < off {
        return false;
    }
    let Some(point) = data.obank().get(op).map(|o| o.get_addr().clone()) else { return false };
    let mut full_encoding: uintb = 0;
    let Ok(rampoint) = glb.resolve_constant(&spc, off, size, &point, &mut full_encoding) else {
        return false;
    };
    if rampoint.is_invalid() {
        return false;
    }
    let invalid = Address::new_invalid();
    match glb.query_container_global(&rampoint, 1, &invalid) {
        Some(entry) => entry
            .symbol_type
            .as_ref()
            .filter(|t| t.get_metatype() == type_metatype::TYPE_ARRAY)
            .and_then(|t| t.get_array_base())
            .is_some_and(|e| e.get_size() == 1 && e.is_ascii()),
        None => false,
    }
}

/// Is `cand` more specific than `cur` in the `getLocalType` fold?
pub fn folds_over(cand: &Rc<Datatype>, cur: &Rc<Datatype>) -> bool {
    0 > cand.type_order(cur).unwrap_or(0)
}

#[cfg(test)]
mod tests;
