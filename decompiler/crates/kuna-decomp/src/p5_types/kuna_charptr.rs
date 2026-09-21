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
//! * the value reaches argument *i* of a call whose callee has a DECLARED
//!   `char *` there, AT THE BASE.  Declared means stated from outside the
//!   decompile: a `libproto`/`libcsigs` signature, a `libctypes` shell, DWARF,
//!   a demangled name, `--assert prototype`, or the per-call-site override
//!   `formatstring` installs for a resolved `%s`.  A type another *recovery*
//!   voted for is not evidence here
//!   ([`crate::coreaction_infertypes::declared_input_type_local`]); counting
//!   `protoorder`'s callee vote was measured and scored lower.  At a FIXED
//!   non-zero offset the callee is being told about a FIELD, exactly as a
//!   one-byte read there is: cronie `crond` -O0 `sub_6715` calls
//!   `strcmp(base + 0x13, ".cron.hostname")` on a `struct dirent *`, and 0x13
//!   is `offsetof(struct dirent, d_name)`.
//! * the value is dereferenced *at the base* and every such dereference is one
//!   byte wide, or it is indexed one byte at a time by an index the program
//!   computes (`PTRADD` of element size one whose index is not a constant).
//!   Nothing at a FIXED non-zero offset says anything: a byte read there is a
//!   one-byte field, and a constant displacement is a field whether it is an
//!   `INT_ADD` or, once the base is typed, a `PTRADD` -- `&d->d_name[0]` on a
//!   `struct dirent *` is `PTRADD(d, 0x13, 1)`, and walking the array from there
//!   is walking the field.  So is skipping a string's first character
//!   (`p + 1`), which the walk cannot tell from a field at offset one.
//! * the value is defined by, or compared against, a constant that resolves to
//!   a character array in the image.
//!
//! The candidate is one more vote in the same `getLocalType` fold, folded by
//! [`Datatype::type_order`], and it may also *refine* a pointer vote that points
//! at nothing: `void *` and `undefined1 *` become `char *`, while a pointer vote
//! at a named or sized thing — `FILE *`, `stat *`, `long *`, a synthesized
//! `struct_3 *` — is left as it was.  The guard reads the vote in flight, so a
//! pointee that would only arrive by later propagation is not protected:
//! e2fsck's `ext2fs_bitcount(unsigned int *a0, ...)` becomes `char *a0`.  A
//! type-locked Varnode and a Varnode seeded from a type-locked symbol are never
//! touched.
//!
//! The walk refuses outright on any use a character pointer does not survive:
//! a dereference wider than a byte, a `PTRADD` whose element is wider, a call
//! argument the callee has declared as something other than a character pointer,
//! and everything [`crate::kuna_ptrfromuse`] refuses (integer arithmetic, shifts,
//! float ops, a comparison against a non-zero literal).  A refusal withdraws the
//! Varnode it was collected for and nothing else: the walk runs once per
//! Varnode, the vote is folded per Varnode, and the type that reaches the reader
//! is the fold over every Varnode of the merged variable.  So a slot walked with
//! byte evidence in one flow and refused in another prints the commitment --
//! grep -O0 `sub_6fdd`'s stack slot at -0x20 is walked eight times with `byte`
//! evidence and four times refused (`KUNA_CHARPTR_CENSUS=1`).  The refusals are
//! about element WIDTH and contradicting declarations, not about arithmetic
//! shape: a constant-stride walk is never refused, because a literal-addend
//! `INT_ADD` only marks the value as offset from its base.  coreutils od -O2
//! `sub_4270` shows both halves -- its third parameter commits on byte
//! dereferences at the base and then prints `a2 = &a2[0x10]`.
//!
//! Only storage a reader sees declared is considered: a function input the
//! prototype model could place a parameter in, or a Varnode in the stack space.
//! A register temporary is neither a parameter nor a declaration.
//!
//! Gated by [`Architecture::char_ptr`](crate::architecture::Architecture)
//! (option `charptr on|off`, shipped `off`); with the option off nothing here is
//! reachable.

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// How many def-use hops the walk follows before giving up; the same bound
/// [`crate::kuna_ptrfromuse`] uses.
const HOP_CAP: usize = 10;

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
    /// Identity: keep walking from this Varnode, at the same distance from the base.
    Forward(VarnodeId),
    /// A fixed non-zero offset from the base: keep walking, but nothing from
    /// there on is about the base.
    ForwardOffset(VarnodeId),
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
    t.get_metatype() == type_metatype::TYPE_PTR && t.get_ptr_to().as_deref().is_some_and(is_char)
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
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
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
    enabled: bool,
) -> Option<Rc<Datatype>> {
    if !enabled {
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
    let ev = walk_is_char(data, vn);
    if census_on() {
        let v = data.vbank().get(vn)?;
        eprintln!(
            "CHARPTR fn={:#x} {} size={} cur={} ev={}",
            data.get_address().get_offset(),
            if v.is_input() {
                format!("input@{}", v.get_addr().get_offset())
            } else {
                format!("stack@{}", v.get_addr().get_offset() as i64)
            },
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
fn walk_is_char(data: &Funcdata, start: VarnodeId) -> Evidence {
    let mut ev = Evidence::default();
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    // The third component is "this Varnode is the base plus a fixed offset".  A
    // byte read THERE is a one-byte struct field, not a character: coreutils
    // `ginstall`'s `announce_mkdir(char *dir, void *options)` reads the `bool` at
    // `options + 0x3c`, and nothing about that says the object is a string.
    let mut work: VecDeque<(VarnodeId, usize, bool)> = VecDeque::new();
    work.push_back((start, 0, false));
    seen.insert(start);
    while let Some((vn, hops, offsetted)) = work.pop_front() {
        if defined_by_string_constant(data, vn) {
            ev.strconst += 1;
        }
        let descend: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            let mut forward = None;
            match classify_use(data, op, vn, offsetted) {
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
                    forward = Some((next, offsetted));
                }
                Use::Forward(next) => forward = Some((next, offsetted)),
                Use::ForwardOffset(next) => forward = Some((next, true)),
            }
            if let Some((next, off)) = forward {
                if hops + 1 < HOP_CAP && seen.insert(next) {
                    work.push_back((next, hops + 1, off));
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
fn classify_use(data: &Funcdata, op: OpId, vn: VarnodeId, offsetted: bool) -> Use {
    let Some(o) = data.obank().get(op) else {
        return Use::Neutral;
    };
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
            match out.and_then(|o| data.vbank().get(o)).map(|v| v.get_size()) {
                Some(1) if offsetted => Use::Neutral,
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
            match o
                .get_in(2)
                .and_then(|v| data.vbank().get(v))
                .map(|v| v.get_size())
            {
                Some(1) if offsetted => Use::Neutral,
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
        OpCode::CPUI_PTRADD => {
            if slot != 0 {
                return Use::Neutral;
            }
            let constant_in = |i: int4| {
                o.get_in(i)
                    .and_then(|v| data.vbank().get(v))
                    .filter(|v| v.is_constant())
                    .map(|v| v.get_offset())
            };
            match (ptradd_step(constant_in(2), constant_in(1), offsetted), out) {
                (PtraddStep::Char, Some(o)) => Use::CharForward(CharKind::Byte, o),
                (PtraddStep::Char, None) => Use::Char(CharKind::Byte),
                (PtraddStep::Same, Some(o)) => Use::Forward(o),
                (PtraddStep::Offset, Some(o)) => Use::ForwardOffset(o),
                (PtraddStep::Refuse, _) => Use::Refuse("elem-wider"),
                _ => Use::Neutral,
            }
        }
        OpCode::CPUI_PTRSUB => match (slot, out) {
            (0, Some(o)) => {
                let zero = o_const_is_zero(data, op, 1);
                if zero {
                    Use::Forward(o)
                } else {
                    Use::ForwardOffset(o)
                }
            }
            _ => Use::Neutral,
        },
        // `p + k` with a literal k is still the same object, unless the literal is
        // itself the address of a mapped global object — then the constant is the
        // base and the walked value is an index into it.  The grading is
        // `kuna_ptrfromuse`'s, and this rule reuses it.
        OpCode::CPUI_INT_ADD => {
            let other = if slot == 0 { 1 } else { 0 };
            let Some(other_vn) = o.get_in(other) else {
                return Use::Neutral;
            };
            let is_const = data
                .vbank()
                .get(other_vn)
                .map(|v| v.is_constant())
                .unwrap_or(false);
            if !is_const {
                return Use::Neutral;
            }
            if crate::kuna_ptrfromuse::constant_is_global_base(data, op, other_vn) {
                return Use::Refuse("global-base");
            }
            let zero = data
                .vbank()
                .get(other_vn)
                .map(|v| v.get_offset() == 0)
                .unwrap_or(false);
            match out {
                Some(o) if zero => Use::Forward(o),
                Some(o) => Use::ForwardOffset(o),
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
            let declared = crate::coreaction_infertypes::declared_input_type_local(data, op, slot);
            if is_char_pointer(&declared) {
                // At a fixed non-zero offset the callee is being told about a
                // FIELD, exactly as a one-byte load there is about a field:
                // `strcmp(base + 0x13, ".cron.hostname")` on cronie's `crond`
                // reads `d_name` out of a `struct dirent`, and says nothing
                // about the base.  Same guard, same reason as the byte arm.
                return if offsetted {
                    Use::Neutral
                } else {
                    Use::Char(CharKind::Declared)
                };
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
            let Some(other_vn) = o.get_in(other) else {
                return Use::Neutral;
            };
            let Some(ov) = data.vbank().get(other_vn) else {
                return Use::Neutral;
            };
            if !ov.is_constant() {
                return Use::Neutral;
            }
            if ov.get_offset() == 0 {
                return Use::Neutral;
            }
            if constant_is_string(data, op, other_vn) {
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

/// What a `PTRADD(base, index, elemsize)` step on the walked value says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PtraddStep {
    /// A one-byte element at a varying index from the base: character indexing.
    Char,
    /// Index zero: the same address, at the same distance from the base.
    Same,
    /// A fixed non-zero distance from the base, or any step once the walk is
    /// already there: an element of a field, which says nothing about the base.
    Offset,
    /// An element wider than a byte.
    Refuse,
    /// The element size is not a constant.
    Neutral,
}

/// Classify a `PTRADD` from its constant element size and index (`None` when
/// that input is not a constant) and from whether the walked value already sits
/// at a fixed offset from the base.
///
/// `&s->name[0]` with `s` a `struct dirent *` is `PTRADD(s, 0x13, 1)`: a
/// one-byte element, but at `offsetof(d_name)`, and walking the array from there
/// is walking a field.  Only an index the program varies, from the base itself,
/// is evidence that the base is a character array.
fn ptradd_step(elem: Option<uintb>, index: Option<uintb>, offsetted: bool) -> PtraddStep {
    match (elem, index) {
        (None, _) => PtraddStep::Neutral,
        (Some(1), Some(0)) => PtraddStep::Same,
        (Some(1), Some(_)) => PtraddStep::Offset,
        (Some(1), None) if offsetted => PtraddStep::Offset,
        (Some(1), None) => PtraddStep::Char,
        (Some(_), _) => PtraddStep::Refuse,
    }
}

/// Is input `slot` of `op` the constant zero?
fn o_const_is_zero(data: &Funcdata, op: OpId, slot: int4) -> bool {
    data.obank()
        .get(op)
        .and_then(|o| o.get_in(slot))
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.is_constant() && v.get_offset() == 0)
        .unwrap_or(false)
}

/// Is `vn` written by a COPY or MULTIEQUAL whose source is a constant that
/// resolves to a character array in the image?  `char *p = "literal";` is the
/// plainest statement a program makes about a pointer's element type.
fn defined_by_string_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    if !v.is_written() {
        return false;
    }
    let Some(def) = v.get_def() else { return false };
    let Some(o) = data.obank().get(def) else {
        return false;
    };
    if !matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL) {
        return false;
    }
    (0..o.num_input()).any(|i| {
        o.get_in(i)
            .and_then(|iv| data.vbank().get(iv))
            .map(|c| c.is_constant())
            .unwrap_or(false)
            && o.get_in(i)
                .is_some_and(|iv| constant_is_string(data, def, iv))
    })
}

/// Does the constant `cvn` resolve to a NUL-terminated character array kuna has
/// already recovered at that address?  The resolution is
/// `ActionConstantPtr::isPointer`'s (`coreaction.cc:1167`), and the answer is
/// the covering global symbol's own data-type.
fn constant_is_string(data: &Funcdata, op: OpId, cvn: VarnodeId) -> bool {
    let glb = data.get_arch();
    let Some(v) = data.vbank().get(cvn) else {
        return false;
    };
    let (off, size) = (v.get_offset(), v.get_size());
    let Some(spc) = glb.manage().get_default_data_space().map(Rc::clone) else {
        return false;
    };
    if spc.get_pointer_lower_bound() > off || spc.get_pointer_upper_bound() < off {
        return false;
    }
    let Some(point) = data.obank().get(op).map(|o| o.get_addr().clone()) else {
        return false;
    };
    let mut full_encoding: uintb = 0;
    let Ok(rampoint) = glb.resolve_constant(&spc, off, size, &point, &mut full_encoding) else {
        return false;
    };
    if rampoint.is_invalid() {
        return false;
    }
    let invalid = Address::new_invalid();
    match glb.query_container_global(&rampoint, 1, &invalid) {
        Some(entry) => {
            if census_on() {
                eprintln!(
                    "CHARPTR-STR ram={:#x} entry={:#x} ty={} sz={}",
                    rampoint.get_offset(),
                    entry.entry_addr.get_offset(),
                    entry
                        .symbol_type
                        .as_ref()
                        .map(|t| t.get_name().to_string())
                        .unwrap_or_default(),
                    entry
                        .symbol_type
                        .as_ref()
                        .map(|t| t.get_size())
                        .unwrap_or(0),
                );
            }
            entry
                .symbol_type
                .as_ref()
                .filter(|t| t.get_metatype() == type_metatype::TYPE_ARRAY)
                .and_then(|t| t.get_array_base())
                .is_some_and(|e| e.get_size() == 1 && e.is_ascii())
        }
        None => false,
    }
}

/// Is `cand` more specific than `cur` in the `getLocalType` fold?
pub fn folds_over(cand: &Rc<Datatype>, cur: &Rc<Datatype>) -> bool {
    0 > cand.type_order(cur).unwrap_or(0)
}

#[cfg(test)]
mod tests;
