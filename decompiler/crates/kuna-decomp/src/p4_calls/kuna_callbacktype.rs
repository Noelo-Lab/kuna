//! (kuna `callbacktype`) A callback takes the prototype of the slot it is passed to.
//!
//! kuna resolves a declared prototype at a call site, so `qsort(a, n, 0x10,
//! sub_3a72)` already renders with the library's name. Nothing was ever said
//! about `sub_3a72` itself: a function reached only through a function pointer
//! has no call site to recover from, so its own body is all it has, and a
//! comparator that never dereferences past the first word comes out
//! `int sub_3a72(unsigned long *a0, unsigned long *a1)` while a signal handler
//! that ignores its argument comes out `void sub_1100(void)`.
//!
//! The library declares both. `qsort`'s fourth parameter is
//! `int (*)(const void *, const void *)` and `signal`'s second is
//! `void (*)(int)`, so a function whose address is handed to one of those slots
//! is declared by the call, not guessed. This module records, at each call to
//! an entry point in [`SLOTS`], the constant a callback slot carries, and the
//! whole-binary driver parks that declaration on the function and decompiles it
//! again ([`park`]), with each direct caller whose call it changes
//! ([`Ledger::caller_changes`]). A declaration that is exactly what the body
//! already printed is not parked ([`Decline::Agrees`]).
//!
//! # What is refused
//!
//! A declared prototype input-locks the parameters, so the claim has to be
//! exactly right or it fabricates one -- or drops one. Five things refuse it:
//!
//! * a declared prototype already exists -- DWARF, a user `--assert`, the libc
//!   tables -- and outranks this one;
//! * the address reaches anywhere else. Two halves answer that. The driver
//!   answers for the image: the function must not be exported, stored in a
//!   loaded word, a relocation target or the entry point, and every
//!   cross-reference that materializes its address must sit in a function that
//!   handed it to a slot. This module answers for the bodies: every USE of the
//!   address in a body that registered it must be one of those registrations,
//!   so one hoisted `lea` feeding a `signal` call and a second registration is
//!   refused where counting `lea`s saw one reference for two uses. The value is
//!   followed through phis and the `INDIRECT`s calls leave, and writing it into
//!   a global is a use whether or not the body reads it back;
//! * two callback slots disagree about it;
//! * a direct call to it reads more of the return register than the declared
//!   return holds;
//! * its own body contradicts the signature, in either direction. Its own
//!   recovery found more inputs than the declaration passes, or fewer while
//!   something also calls it directly, or an input wider than the storage the
//!   declaration gives that position; it reads the argument register one past
//!   the declared list before writing it; it hands back a value it computed
//!   where the signature says `void`, computes none where the signature hands
//!   one back, hands back more than the declared return holds, or computes
//!   less and leaves the bytes above it anything but zero.
//!
//! The arity halves of that last one are what keep a body the cast lied about
//! honest, and they are not the same question. A three-argument function cast
//! into `qsort`'s two-argument slot never READS the third register if all it
//! does is pass it on (`return memcmp(a, b, n);` compiles to a tail call), so
//! the entry-fact walk states nothing about it; the recovered input count does.
//! In the other direction the declaration is longer than the body, and the
//! damage is at the DIRECT call sites: the closed list materializes an argument
//! at each of them out of whatever the register last held, so
//! `signal(SIGALRM, (void (*)(int))cleanup); ... cleanup();` on a
//! `void cleanup(void)` grows an invented `v = 0x2006;` and an invented
//! argument. With no direct caller there is no site to invent at, and the
//! declared parameter simply goes unread -- which is what a handler that
//! ignores its signal number looks like in DWARF as well, so that shape is
//! parked and the unused parameter is reported (see the option's row in
//! `phases.toml` and chapter 04 of the spec).
//!
//! The width halves are asked with or without a direct caller. A `struct ctx *`
//! routine cast into `signal`'s `void (*)(int)` reads the whole register the
//! slot declares a four-byte `int` in, so the declaration would make its own
//! body rebuild the pointer out of half a register, and a `long` comparator
//! cast into `qsort`'s `int` slot would subtract in the low half only. A
//! return is measured by the bits its value can set, not by the storage its
//! own recovery gave it: every 32-bit write on x86-64 zero-extends into the
//! whole register, so an `int` comparator recovers an 8-byte output whose
//! upper half is provably zero. The CALLERS answer what the body cannot. A
//! `long` holding a zero-extended value writes only `eax`, and a direct call
//! that passes the whole `rax` on is the one thing that shows it; so is a
//! comparator that returns another comparator's result straight through its
//! own `ret`, where declaring the inner one alone would leave the outer one
//! returning a register half of which nothing set.
//!
//! The return halves are asked of the value, not of what the body's own
//! recovery printed. A comparator ending `return strcmp(a, b);` recovers `void`
//! -- nothing in the program reads its result -- yet `strcmp` leaves its
//! declared `int` in the register and nothing after the call writes it, so the
//! `int` slot is computed. A `void` start routine ending in a tail-called
//! `puts` leaves only four of the eight bytes a `void *` slot returns, and is
//! refused. A body that computes a narrower value than the slot returns is
//! asked about the bytes above it, which the declaration prints and its own
//! recovery never did: `mov (%rdi),%al; sub (%rsi),%al` leaves them as the
//! caller left them, `movzbl (%rdi),%eax; sub (%rsi),%al` clears them, and
//! only the machine code can tell the two apart ([`zero_at_every_return`]).
//!
//! An architecture whose code builds an address from two instructions
//! (AArch64 `adrp`+`add`, MIPS `lui`+`addiu`) states nothing: the
//! cross-reference walk the driver asks cannot see the address taken, so the
//! "reaches nowhere else" question has no answer and the refusal stands.

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::p4_calls::fspec::{FuncProto, PrototypePieces};

/// Whether a callback is declared by the slot it is passed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallbackTypeMode {
    /// A callback keeps whatever its own body recovers.
    #[default]
    Off,
    /// A function whose address reaches exactly one kind of declared callback
    /// slot takes that slot's declared prototype.
    On,
}

impl CallbackTypeMode {
    /// The option token.
    pub fn as_str(self) -> &'static str {
        match self {
            CallbackTypeMode::Off => "off",
            CallbackTypeMode::On => "on",
        }
    }

    /// Does a whole-binary run record and park anything?
    pub fn is_on(self) -> bool {
        self == CallbackTypeMode::On
    }
}

/// (kuna) Parse `option callbacktype on|off`.
pub struct OptionCallbackType;

impl OptionCallbackType {
    /// The option name.
    pub const NAME: &'static str = "callbacktype";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(CallbackTypeMode, String)> {
        let mode = match p1 {
            "on" => CallbackTypeMode::On,
            "off" => CallbackTypeMode::Off,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown callbacktype value: {other} (expected on|off)"
                )))
            }
        };
        Ok((mode, format!("Callback prototypes from the declared slot set to {}", mode.as_str())))
    }
}

/// A type slot in a declared signature. Every variant is width-stable on any
/// ILP32/LP64 target, the rule [`crate::p4_calls`]'s library tables keep.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ty {
    /// `void` (return only).
    Void,
    /// `int`.
    Int,
    /// `void *` (a `const void *`, a `FILE *`, an opaque handle).
    VoidPtr,
    /// `char *`.
    CharPtr,
}

/// A declared callback signature.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sig {
    /// What the callback returns.
    pub ret: Ty,
    /// What it takes.
    pub params: &'static [Ty],
}

/// One library entry point that declares a function-pointer parameter.
pub struct Entry {
    /// The imported name.
    pub name: &'static str,
    /// The entry point's OWN parameter types, up to and including the last
    /// callback slot: the prototype model assigns storage positionally from
    /// them, so the slot's argument register is read from the convention rather
    /// than from an architecture table.
    pub params: &'static [Ty],
    /// `(parameter index, what that parameter declares)`.
    pub slots: &'static [(usize, Sig)],
}

const CMP2: Sig = Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr] };
const HANDLER: Sig = Sig { ret: Ty::Void, params: &[Ty::Int] };
const THREAD: Sig = Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr] };
const DTOR: Sig = Sig { ret: Ty::Void, params: &[Ty::VoidPtr] };
const NULLARY: Sig = Sig { ret: Ty::Void, params: &[] };
const ONEXIT: Sig = Sig { ret: Ty::Void, params: &[Ty::Int, Ty::VoidPtr] };
const GLOBERR: Sig = Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::Int] };
const FTWFN: Sig = Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::VoidPtr, Ty::Int] };
const NFTWFN: Sig =
    Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::VoidPtr, Ty::Int, Ty::VoidPtr] };
const DIRFILT: Sig = Sig { ret: Ty::Int, params: &[Ty::VoidPtr] };

const P: Ty = Ty::VoidPtr;
const I: Ty = Ty::Int;

/// The library entry points that declare a function-pointer parameter.
///
/// Each signature is the POSIX/glibc declaration reduced to [`Ty`] by the rule
/// `kuna_libcsigs` uses: a `const void *` is a `void *`, an opaque handle is a
/// `void *`, and a slot with no width-stable spelling would keep the entry out.
/// `sigaction` is absent on purpose -- its handler is a member of a structure,
/// not an argument -- and so is `qsort` from the library prototype tables,
/// which is why the slot table lives here rather than beside them. So is a
/// name whose C libraries disagree about where the callback goes
/// ([`_QSORT_R_IS_NOT_A_SLOT`]).
pub const SLOTS: &[Entry] = &[
    Entry { name: "qsort", params: &[P, P, P, P], slots: &[(3, CMP2)] },
    Entry { name: "bsearch", params: &[P, P, P, P, P], slots: &[(4, CMP2)] },
    Entry { name: "lfind", params: &[P, P, P, P, P], slots: &[(4, CMP2)] },
    Entry { name: "lsearch", params: &[P, P, P, P, P], slots: &[(4, CMP2)] },
    Entry { name: "signal", params: &[I, P], slots: &[(1, HANDLER)] },
    Entry { name: "__sysv_signal", params: &[I, P], slots: &[(1, HANDLER)] },
    Entry { name: "bsd_signal", params: &[I, P], slots: &[(1, HANDLER)] },
    Entry { name: "sigset", params: &[I, P], slots: &[(1, HANDLER)] },
    Entry { name: "atexit", params: &[P], slots: &[(0, NULLARY)] },
    Entry { name: "on_exit", params: &[P, P], slots: &[(0, ONEXIT)] },
    Entry { name: "pthread_create", params: &[P, P, P, P], slots: &[(2, THREAD)] },
    Entry { name: "pthread_once", params: &[P, P], slots: &[(1, NULLARY)] },
    Entry { name: "pthread_key_create", params: &[P, P], slots: &[(1, DTOR)] },
    Entry {
        name: "pthread_atfork",
        params: &[P, P, P],
        slots: &[(0, NULLARY), (1, NULLARY), (2, NULLARY)],
    },
    Entry { name: "scandir", params: &[P, P, P, P], slots: &[(2, DIRFILT), (3, CMP2)] },
    Entry { name: "scandirat", params: &[I, P, P, P, P], slots: &[(3, DIRFILT), (4, CMP2)] },
    Entry { name: "ftw", params: &[P, P, I], slots: &[(1, FTWFN)] },
    Entry { name: "nftw", params: &[P, P, I, I], slots: &[(1, NFTWFN)] },
    Entry { name: "tsearch", params: &[P, P, P], slots: &[(2, CMP2)] },
    Entry { name: "tfind", params: &[P, P, P], slots: &[(2, CMP2)] },
    Entry { name: "tdelete", params: &[P, P, P], slots: &[(2, CMP2)] },
    Entry { name: "tdestroy", params: &[P, P], slots: &[(1, DTOR)] },
    Entry { name: "glob", params: &[P, I, P, P], slots: &[(2, GLOBERR)] },
];

/// `__cxa_atexit` is DELIBERATELY absent. glibc's `<stdlib.h>` turns
/// `atexit(f)` into `__cxa_atexit(f, 0, __dso_handle)` with `f` cast from
/// `void (*)(void)`, so the slot's declared `void (*)(void *)` would give every
/// `atexit` handler in a glibc program a parameter the source never wrote. The
/// `atexit` entry above covers the same registration where the image really
/// does import `atexit`.
const _CXA_ATEXIT_IS_NOT_A_SLOT: () = ();

/// `qsort_r` is DELIBERATELY absent. glibc and musl declare
/// `qsort_r(base, n, size, compar, arg)` with a three-argument comparator
/// FOURTH; macOS and FreeBSD before 14 declare
/// `qsort_r(base, n, size, thunk, compar)` with the opaque `thunk` fourth and a
/// comparator taking it FIRST. The import's name does not say which library
/// the image links, and a declaration taken from the wrong one parks a
/// comparator's prototype on whatever the caller passed as its data pointer.
const _QSORT_R_IS_NOT_A_SLOT: () = ();

/// One callback argument: a constant a declared slot carried.
#[derive(Debug, Clone)]
pub struct Fact {
    /// The address the constant names.
    pub value: uintb,
    /// The library entry point whose slot carried it.
    pub declared_by: &'static str,
    /// The signature that slot declares.
    pub sig: Sig,
    /// The call instruction, for the trace. NOT the instruction that
    /// materialized the address: by the time the argument is read, type
    /// propagation has rebuilt it as a `PTRSUB` sited at the call, so the
    /// `lea` the image's cross-reference walk reports is no longer named by
    /// any op in the function.
    pub at: uintb,
}

/// What every callback slot in the run said about one address.
#[derive(Debug, Clone)]
pub struct Seen {
    /// The library entry point that declared it.
    pub declared_by: &'static str,
    /// The signature that slot declares.
    pub sig: Sig,
    /// The entries of the functions that handed it to a callback slot.
    pub owners: std::collections::BTreeSet<uintb>,
    /// How many callback arguments carried it, over the whole run.
    pub args: usize,
    /// Two slots disagreed about it.
    pub conflict: bool,
    /// Some body that handed it to a slot also used the same address for
    /// something else -- another registration, a store, a comparison.
    pub other_use: bool,
}

/// What a decompiled function's own body proved about itself.
#[derive(Debug, Clone, Default)]
pub struct OwnFacts {
    /// Every live RETURN hands back a value the body computed, and the
    /// recovered output is not `void`.
    pub returns_a_value: bool,
    /// The storage of every input its own first decompile recovered, in order.
    pub inputs: Vec<Range>,
    /// The storage of the output it recovered, when that is not `void`.
    pub output: Option<Range>,
    /// The part of [`OwnFacts::output`] a value it returns can set: the low
    /// bytes that hold every bit any live RETURN's value may have non-zero.
    /// An 8-byte `rax` output holding only a zero-extended 32-bit result sets
    /// the low four.
    pub output_set: Option<Range>,
    /// When its own recovery returns nothing, the storage the calls every live
    /// RETURN is reached from hand back ([`call_return`]).
    pub call_return: Option<Range>,
    /// The instructions of the calls its flow does not continue past.
    pub noreturn_calls: Vec<uintb>,
    /// The signature its first decompile printed, when every part has a type.
    pub printed: Option<PrintedSig>,
}

/// The parameter and return types a decompile printed.
#[derive(Debug, Clone)]
pub struct PrintedSig {
    /// Each parameter's type, in order.
    pub inputs: Vec<Rc<Datatype>>,
    /// The return type, `None` for `void`.
    pub output: Option<Rc<Datatype>>,
    /// It printed a `...`.
    pub dotdotdot: bool,
}

/// A storage range, `(space index, offset, size)`.
pub type Range = (int4, uintb, int4);

/// A storage range no other range is in or holds.
const NO_STORAGE: Range = (-1, 0, 0);

/// The [`Range`] of `size` bytes at `addr`.
fn range_of(addr: &Address, size: int4) -> Range {
    addr.get_space().map_or(NO_STORAGE, |sp| (sp.get_index(), addr.get_offset(), size))
}

/// Do `a` and `b` share a byte?
fn overlaps(a: Range, b: Range) -> bool {
    a.0 == b.0
        && a.1 < b.1.saturating_add(b.2 as uintb)
        && b.1 < a.1.saturating_add(a.2 as uintb)
}

/// Does `outer` hold every byte of `inner`?
fn contains(outer: Range, inner: Range) -> bool {
    outer.0 == inner.0
        && inner.1 >= outer.1
        && inner.1.saturating_add(inner.2 as uintb) <= outer.1.saturating_add(outer.2 as uintb)
}

/// What one decompile of one function said about callback addresses.
#[derive(Debug, Clone, Default)]
pub struct CallerFacts {
    /// The constants its declared callback slots carried.
    pub facts: Vec<Fact>,
    /// Of those addresses, the ones this body ALSO used somewhere no callback
    /// slot accounts for.
    pub other_use: std::collections::BTreeSet<uintb>,
    /// `(callee entry, the part of the return storage this call site reads)`
    /// for every direct call whose returned value the body uses.
    pub reads: Vec<(uintb, Range)>,
    /// `(callee entry, arguments passed, the returned value is consumed)` for
    /// every direct call.
    pub calls: Vec<(uintb, usize, bool)>,
}

/// The whole-binary run's record of callback arguments.
#[derive(Debug, Default)]
pub struct Ledger {
    /// The driver is recording the decompiles it runs.
    pub recording: bool,
    /// Caller key -> what that decompile said. Keyed by caller so a redo
    /// replaces what the first body said instead of counting it twice.
    pub by_caller: HashMap<(int4, uintb), CallerFacts>,
    /// Function key -> what its own first decompile proved.
    pub own: HashMap<(int4, uintb), OwnFacts>,
}

/// Why a declared callback prototype was not parked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decline {
    /// A declared prototype already covers the function.
    Declared,
    /// Two callback slots disagree about the function.
    Conflict,
    /// The address reaches somewhere the callback slots do not explain.
    Escapes,
    /// The function is not one of the run's targets.
    NotATarget,
    /// This run never decompiled the function, so nothing is known about the
    /// body the declaration would be locked onto.
    BodyNotSeen,
    /// The body's own recovery found more inputs than the declaration passes.
    MoreInputsThanDeclared,
    /// The body's own recovery found fewer inputs than the declaration passes,
    /// and something calls the function directly.
    FewerInputsThanDeclared,
    /// An input the body's own recovery found is not inside the storage the
    /// declaration gives that position: a pointer read from the whole of a
    /// register the slot declares an `int` in.
    WiderInputThanDeclared,
    /// The value the body hands back is not inside the storage the declared
    /// return occupies: a `long` in a slot that returns `int`.
    WiderOutputThanDeclared,
    /// The value the body computes is narrower than the declared return, and
    /// some path reaches a RETURN with the bytes above it not provably zero:
    /// a `char` comparator whose `sub (%rsi),%al` leaves the rest of `eax` as
    /// the caller left it.
    NarrowerOutputThanDeclared,
    /// A direct call site reads more of the return register than the declared
    /// return holds.
    CallerReadsAWiderReturn,
    /// The body reads the argument register one past the declared list.
    ReadsPastTheList,
    /// The signature says `void` and the body hands back a value.
    ReturnsAValue,
    /// The signature hands back a value and the body computes none.
    ReturnsNoValue,
    /// The prototype model could not be asked, or put the parameters elsewhere.
    StorageUnderivable,
    /// The declaration is exactly the signature the body already printed, so
    /// parking it would only pay for decompiling it again.
    Agrees,
}

impl Decline {
    /// The trace token.
    pub fn as_str(self) -> &'static str {
        match self {
            Decline::Declared => "declared",
            Decline::Conflict => "conflict",
            Decline::Escapes => "escapes",
            Decline::NotATarget => "not-a-target",
            Decline::BodyNotSeen => "body-not-seen",
            Decline::MoreInputsThanDeclared => "recovered-more-inputs",
            Decline::FewerInputsThanDeclared => "recovered-fewer-inputs",
            Decline::WiderInputThanDeclared => "recovered-wider-input",
            Decline::WiderOutputThanDeclared => "recovered-wider-output",
            Decline::NarrowerOutputThanDeclared => "recovered-narrower-output",
            Decline::CallerReadsAWiderReturn => "caller-reads-a-wider-return",
            Decline::ReadsPastTheList => "reads-past-the-list",
            Decline::ReturnsAValue => "returns-a-value",
            Decline::ReturnsNoValue => "returns-no-value",
            Decline::StorageUnderivable => "storage-underivable",
            Decline::Agrees => "body-agrees",
        }
    }
}

/// Is the decision trace on (`KUNA_CALLBACKTYPE_TRACE=1`)?
pub fn trace() -> bool {
    std::env::var("KUNA_CALLBACKTYPE_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}

/// The declared slot table entry for a callee name.
fn entry_for(name: &str) -> Option<&'static Entry> {
    SLOTS.iter().find(|e| e.name == name)
}

/// Build the kuna [`Datatype`] for a [`Ty`].
fn build_ty(t: Ty, arch: &Architecture) -> Option<Rc<Datatype>> {
    let types = arch.types();
    let ptr = types.get_size_of_pointer();
    let word = arch.manage().get_default_code_space()?.get_word_size();
    match t {
        Ty::Void => types.get_type_void().ok(),
        Ty::Int => types.get_base(4, type_metatype::TYPE_INT).ok(),
        Ty::VoidPtr => {
            let v = types.get_type_void().ok()?;
            types.get_type_pointer(ptr, v, word).ok()
        }
        Ty::CharPtr => {
            let c = types.get_type_char(types.get_size_of_char()).ok()?;
            types.get_type_pointer(ptr, c, word).ok()
        }
    }
}

/// The [`PrototypePieces`] a signature declares, named `name`.
fn pieces_of(sig: &Sig, name: &str, arch: &Architecture) -> Option<PrototypePieces> {
    let mut intypes = Vec::with_capacity(sig.params.len());
    for t in sig.params {
        intypes.push(build_ty(*t, arch)?);
    }
    // Unnamed, so each parameter takes the default name every other function's
    // parameters get, on every surface that prints one.
    let innames = vec![String::new(); intypes.len()];
    let outtype = match sig.ret {
        Ty::Void => None,
        other => Some(build_ty(other, arch)?),
    };
    Some(PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes,
        innames,
        // A DECLARATION, not a floor: the list is closed, so no call site
        // recovers an argument past it and the function prints without a `...`.
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    })
}

/// The storage the prototype model gives the library entry point's own
/// parameters, so a callback slot's argument register is read from the
/// program's convention rather than from an architecture table.
fn slot_storage(e: &Entry, arch: &Architecture) -> Option<Vec<(Address, int4)>> {
    let sig = Sig { ret: Ty::Int, params: e.params };
    let pieces = pieces_of(&sig, e.name, arch)?;
    crate::kuna_protoorder::model_storage(&pieces, arch)
}

fn key_of(a: &Address) -> Option<(int4, uintb)> {
    Some((a.get_space()?.get_index(), a.get_offset()))
}

/// Record a finished decompile: every constant it hands to a declared callback
/// slot, and what its own body proved about itself.
///
/// A redo of the same function records its callback arguments again, replacing
/// what the first decompile said, so a body the driver discarded leaves nothing
/// behind. The own-facts row is written once, by the first decompile, because
/// later ones already carry whatever was parked.
pub fn record(arch: &mut Architecture, entry: &Address, fd: &Funcdata) {
    let Some(me) = key_of(entry) else { return };
    let mut found: Vec<Fact> = Vec::new();
    let mut accounted: std::collections::HashSet<(crate::context::OpId, int4)> =
        std::collections::HashSet::new();
    for i in 0..fd.num_calls() {
        let (name, op) = {
            let fc = fd.get_call_specs(i);
            (fc.get_name().to_string(), fc.get_op())
        };
        let Some(e) = entry_for(&name) else { continue };
        let storage = fd.get_call_specs(i).final_input_storage().to_vec();
        let Some(model) = slot_storage(e, arch) else {
            if trace() {
                eprintln!("[callbacktype] {name}: no model storage");
            }
            continue;
        };
        let Some(o) = fd.obank().get(op) else { continue };
        if o.is_dead() || !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            continue;
        }
        let call_at = o.get_addr().get_offset();
        for (slot, sig) in e.slots {
            let Some((want, _)) = model.get(*slot) else { continue };
            // Match by address only: a 4-byte `int` slot and an 8-byte pointer
            // slot share a register address on a little-endian machine, and the
            // call recovered whichever width it saw.
            let Some(slot_at) = storage.iter().position(|(a, _)| a == want) else { continue };
            let Some(vn) = o.get_in(slot_at as int4 + 1) else { continue };
            let Some(value) = code_address(fd, vn, 0).filter(|v| *v != 0) else {
                if trace() {
                    eprintln!(
                        "[callbacktype] {name} at 0x{call_at:x} slot {slot}: argument is not a code address"
                    );
                }
                continue;
            };
            accounted.insert((op, slot_at as int4 + 1));
            found.push(Fact { value, declared_by: e.name, sig: *sig, at: call_at });
        }
    }
    let other_use = other_uses(fd, &found, &accounted);
    let reads = return_reads(fd);
    let calls = direct_calls(fd);
    let own = if arch.kuna_callbacktype.own.contains_key(&me) {
        None
    } else {
        let proto = fd.get_func_proto();
        let inputs = (0..proto.num_params())
            .map(|i| {
                proto.get_param(i).map_or(NO_STORAGE, |p| range_of(&p.get_address(), p.get_size()))
            })
            .collect();
        let output = proto
            .get_output_type()
            .is_some_and(|t| t.get_metatype() != type_metatype::TYPE_VOID)
            .then(|| range_of(&proto.get_output().get_address(), proto.get_output().get_size()));
        let computes = returns_a_value(fd);
        let output_set = output.and_then(|o| {
            let addr = proto.get_output().get_address();
            set_part(o, returned_bits(fd), addr.get_space()?.is_big_endian())
        });
        Some(OwnFacts {
            returns_a_value: computes,
            inputs,
            output,
            output_set,
            call_return: if computes { None } else { call_return(fd, arch) },
            noreturn_calls: noreturn_calls(fd),
            printed: printed_sig(proto),
        })
    };
    let ledger = &mut arch.kuna_callbacktype;
    ledger.by_caller.insert(me, CallerFacts { facts: found, other_use, reads, calls });
    if let Some(own) = own {
        ledger.own.insert(me, own);
    }
}

/// What every direct call in this body reads of its callee's return: the low
/// bytes of the call's output storage that hold every bit the body consumes.
fn return_reads(fd: &Funcdata) -> Vec<(uintb, Range)> {
    let mut out = Vec::new();
    for i in 0..fd.num_calls() {
        let (callee, op) = {
            let fc = fd.get_call_specs(i);
            (fc.get_entry_address().clone(), fc.get_op())
        };
        if callee.is_invalid() {
            continue;
        }
        let Some(o) = fd.obank().get(op) else { continue };
        if o.is_dead() || o.code() != OpCode::CPUI_CALL {
            continue;
        }
        let Some(v) = o.get_out().and_then(|vn| fd.vbank().get(vn)) else { continue };
        let Some(space) = v.get_addr().get_space() else { continue };
        let storage = (space.get_index(), v.get_addr().get_offset(), v.get_size());
        if let Some(read) = set_part(storage, v.get_consume(), space.is_big_endian()) {
            out.push((callee.get_offset(), read));
        }
    }
    out
}

/// Every direct call in this body: its callee, how many arguments it passes,
/// and whether anything consumes what it returns.
fn direct_calls(fd: &Funcdata) -> Vec<(uintb, usize, bool)> {
    let mut out = Vec::new();
    for i in 0..fd.num_calls() {
        let (callee, op) = {
            let fc = fd.get_call_specs(i);
            (fc.get_entry_address().clone(), fc.get_op())
        };
        if callee.is_invalid() {
            continue;
        }
        let Some(o) = fd.obank().get(op) else { continue };
        if o.is_dead() || o.code() != OpCode::CPUI_CALL {
            continue;
        }
        let consumed = o
            .get_out()
            .and_then(|vn| fd.vbank().get(vn))
            .is_some_and(|v| !v.has_no_descend() && v.get_consume() != 0);
        out.push((callee.get_offset(), (o.num_input() as usize).saturating_sub(1), consumed));
    }
    out
}

/// The signature `proto` prints, or `None` when some part of it has no type.
fn printed_sig(proto: &FuncProto) -> Option<PrintedSig> {
    let inputs = (0..proto.num_params())
        .map(|i| proto.get_param(i).and_then(|p| p.get_type().cloned()))
        .collect::<Option<Vec<_>>>()?;
    let output = match proto.get_output_type() {
        Some(t) if t.get_metatype() == type_metatype::TYPE_VOID => None,
        Some(t) => Some(Rc::clone(t)),
        None => None,
    };
    Some(PrintedSig { inputs, output, dotdotdot: proto.is_dotdotdot() })
}

/// The same type: one factory entry, or two that order equal.
fn same_type(a: &Rc<Datatype>, b: &Rc<Datatype>) -> bool {
    Rc::ptr_eq(a, b) || a.type_order(b).is_ok_and(|o| o == 0)
}

/// Does the declaration say exactly what the body's first decompile printed:
/// the same parameter types in the same order, the same return, no `...`?
fn agrees(printed: Option<&PrintedSig>, pieces: &PrototypePieces) -> bool {
    let Some(sig) = printed else { return false };
    !sig.dotdotdot
        && sig.inputs.len() == pieces.intypes.len()
        && sig.inputs.iter().zip(&pieces.intypes).all(|(a, b)| same_type(a, b))
        && match (&sig.output, &pieces.outtype) {
            (None, None) => true,
            (Some(a), Some(b)) => same_type(a, b),
            _ => false,
        }
}

/// Of the addresses this body handed to a callback slot, the ones it ALSO used
/// somewhere else.
///
/// The image's cross-reference walk counts the INSTRUCTIONS that take an
/// address, which says nothing about how many times the value is then used: one
/// hoisted `lea` can feed a `signal` call and a second registration through the
/// same register, and counting `lea`s sees one reference for two uses. This
/// counts uses. The address is followed from every place it is materialized
/// through whatever merely carries it along (a copy, a cast, the `PTRSUB` a
/// global address is built as, a phi, the `INDIRECT` a call leaves behind), and
/// every operation that then consumes it is a use unless it is one of the
/// callback arguments just recorded. Anything left -- another registration, a
/// comparison, an indirect call through a phi that also holds another
/// function -- is not explained by the slots, and neither is a carrier that
/// writes the address into memory the image walk cannot follow (a global, an
/// address-taken local), even when nothing in this body reads it back.
fn other_uses(
    fd: &Funcdata,
    found: &[Fact],
    accounted: &std::collections::HashSet<(crate::context::OpId, int4)>,
) -> std::collections::BTreeSet<uintb> {
    let mut out = std::collections::BTreeSet::new();
    if found.is_empty() {
        return out;
    }
    let values: std::collections::BTreeSet<uintb> = found.iter().map(|f| f.value).collect();
    let mut marked: HashMap<VarnodeId, uintb> = HashMap::new();
    let mut work: Vec<VarnodeId> = Vec::new();
    for op in fd.obank().iter_alive() {
        let Some(o) = fd.obank().get(op) else { continue };
        let ins = (0..o.num_input()).filter_map(|slot| o.get_in(slot));
        for vn in ins.chain(o.get_out()) {
            if marked.contains_key(&vn) {
                continue;
            }
            if let Some(v) = code_address(fd, vn, 0).filter(|v| values.contains(v)) {
                marked.insert(vn, v);
                work.push(vn);
            }
        }
    }
    while let Some(vn) = work.pop() {
        let value = marked[&vn];
        let Some(var) = fd.vbank().get(vn) else { continue };
        if var.is_persist() || var.is_addr_tied() {
            out.insert(value);
        }
        for op in var.descend_iter() {
            let Some(o) = fd.obank().get(op) else { continue };
            if o.is_dead() {
                continue;
            }
            if carries_a_value(o.code()) {
                if let Some(next) = o.get_out().filter(|n| !marked.contains_key(n)) {
                    marked.insert(next, value);
                    work.push(next);
                }
                continue;
            }
            let unexplained = (0..o.num_input())
                .any(|slot| o.get_in(slot) == Some(vn) && !accounted.contains(&(op, slot)));
            if unexplained {
                out.insert(value);
            }
        }
    }
    out
}

/// Does this opcode move a value rather than consume it?
fn carries_a_value(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INDIRECT
    )
}

impl Ledger {
    /// Does `caller`'s recorded body have to be decompiled again once `callee`
    /// takes a declaration of `arity` parameters?
    ///
    /// Only a call site that the declaration changes: one that passes a
    /// different number of arguments, or that consumes the returned value. A
    /// call that passes `arity` arguments and ignores the result prints the
    /// same call under either prototype. A caller the run never recorded is
    /// decompiled again.
    pub fn caller_changes(&self, caller: uintb, callee: uintb, arity: usize) -> bool {
        let mut rows = self.by_caller.iter().filter(|(k, _)| k.1 == caller).peekable();
        if rows.peek().is_none() {
            return true;
        }
        rows.any(|(_, c)| {
            c.calls.iter().any(|(to, n, consumed)| *to == callee && (*n != arity || *consumed))
        })
    }

    /// What each recorded direct call to `callee` reads of its return.
    pub fn reads_of(&self, callee: uintb) -> impl Iterator<Item = Range> + '_ {
        self.by_caller
            .values()
            .flat_map(|c| c.reads.iter())
            .filter(move |(to, _)| *to == callee)
            .map(|(_, read)| *read)
    }

    /// Every address a callback slot named, what the slots between them said
    /// about it, which functions handed it out and how many arguments carried
    /// it. Both of the last two are what the driver checks the image against:
    /// a reference taking the address from a function that handed it to no
    /// slot is an escape, and so are more references than there were
    /// arguments (one function can hold one `lea` for a `qsort` call and
    /// another for a store).
    pub fn decided(&self) -> BTreeMap<uintb, Seen> {
        let mut out: BTreeMap<uintb, Seen> = BTreeMap::new();
        let mut rows: Vec<(&(int4, uintb), &Fact)> = self
            .by_caller
            .iter()
            .flat_map(|(k, v)| v.facts.iter().map(move |f| (k, f)))
            .collect();
        rows.sort_unstable_by_key(|(k, f)| (f.value, **k, f.at, f.declared_by));
        for (caller, f) in rows {
            let row = out.entry(f.value).or_insert(Seen {
                declared_by: f.declared_by,
                sig: f.sig,
                owners: std::collections::BTreeSet::new(),
                args: 0,
                conflict: false,
                other_use: false,
            });
            if row.sig != f.sig {
                row.conflict = true;
            }
            row.owners.insert(caller.1);
            row.args += 1;
        }
        for facts in self.by_caller.values() {
            for value in &facts.other_use {
                if let Some(row) = out.get_mut(value) {
                    row.other_use = true;
                }
            }
        }
        out
    }
}

/// The code address a callback argument names, through the forms a global
/// address takes on the way to a call: a plain constant, the `PTRSUB` off the
/// constant spacebase a `&DAT_3a72` is built as, and the copies and casts
/// between them. Anything else -- a load, a phi, arithmetic on a base -- is not
/// one address and answers `None`.
fn code_address(fd: &Funcdata, vn: VarnodeId, depth: u32) -> Option<uintb> {
    if depth > 6 {
        return None;
    }
    let v = fd.vbank().get(vn)?;
    if v.is_constant() {
        return Some(v.get_offset());
    }
    let def = v.get_def().and_then(|d| fd.obank().get(d))?;
    match def.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT => {
            code_address(fd, def.get_in(0)?, depth + 1)
        }
        OpCode::CPUI_PTRSUB => {
            let base = fd.vbank().get(def.get_in(0)?)?;
            if !base.is_constant() || base.get_offset() != 0 {
                return None;
            }
            let off = fd.vbank().get(def.get_in(1)?)?;
            off.is_constant().then(|| off.get_offset())
        }
        _ => None,
    }
}

/// Does every live RETURN hand back a value the body computed, for a recovered
/// output that is not `void`?
fn returns_a_value(fd: &Funcdata) -> bool {
    let proto = fd.get_func_proto();
    let non_void = proto
        .get_output_type()
        .is_some_and(|t| t.get_metatype() != type_metatype::TYPE_VOID);
    non_void && crate::kuna_returnuncomputed::every_return_computes(fd)
}

/// Every bit a live RETURN's value may have set: the union of the non-zero
/// masks of what each one hands back.
fn returned_bits(fd: &Funcdata) -> uintb {
    let mut bits: uintb = 0;
    for ret in fd.obank().iter_code(OpCode::CPUI_RETURN) {
        let Some(r) = fd.obank().get(ret) else { continue };
        if r.is_dead() || r.get_halt_type() != 0 || r.num_input() < 2 {
            continue;
        }
        bits |= r.get_in(1).and_then(|v| fd.vbank().get(v)).map_or(uintb::MAX, |v| v.get_nz_mask());
    }
    bits
}

/// The low bytes of `storage` that hold every bit of `bits`.
fn set_part(storage: Range, bits: uintb, big_endian: bool) -> Option<Range> {
    let used = (uintb::BITS - bits.leading_zeros()).div_ceil(8).max(1) as int4;
    if used >= storage.2 {
        return Some(storage);
    }
    let offset = if big_endian { storage.1 + (storage.2 - used) as uintb } else { storage.1 };
    Some((storage.0, offset, used))
}

/// The storage a body's calls hand back, when its own recovery says it
/// returns nothing.
///
/// Every live RETURN has to be reached from a direct CALL along a path the
/// machine code runs straight through ([`straight_to_return`]), and the callee
/// has to DECLARE its output; the answer is the narrowest declared output over
/// every RETURN. A comparator ending `return strcmp(a, b);` recovers `void` on
/// its own -- nothing in the program reads its result -- but it hands back what
/// `strcmp` left in the register, whether the call is a `jmp` (the CALL and the
/// RETURN are one instruction) or `call strcmp; leave; ret`. `None` when any
/// RETURN is reached otherwise or its callee declares nothing.
fn call_return(fd: &Funcdata, arch: &Architecture) -> Option<Range> {
    let mut out: Option<Range> = None;
    for ret in fd.obank().iter_code(OpCode::CPUI_RETURN).collect::<Vec<_>>() {
        let Some(r) = fd.obank().get(ret) else { continue };
        if r.is_dead() || r.get_halt_type() != 0 {
            continue;
        }
        let call = crate::kuna_passthrough::producing_call(fd, ret)?;
        let c = fd.obank().get(call)?;
        let i = (0..fd.num_calls()).find(|&i| fd.get_call_specs(i).get_op() == call)?;
        let proto = fd.get_call_specs(i).proto();
        let declared = proto
            .get_output_type()
            .is_some_and(|t| t.get_metatype() != type_metatype::TYPE_VOID);
        if !proto.is_output_locked() || !declared {
            return None;
        }
        let param = proto.get_output();
        let addr = param.get_address();
        let here = (addr.get_space()?.get_index(), addr.get_offset(), param.get_size());
        if !straight_to_return(arch, c.get_addr(), r.get_addr(), here) {
            return None;
        }
        out = Some(match out {
            None => here,
            Some(o) if contains(o, here) => here,
            Some(o) if contains(here, o) => o,
            Some(_) => return None,
        });
    }
    out
}

/// How many instructions [`straight_to_return`] reads before giving up.
const STRAIGHT_PATH_INSTRUCTIONS: usize = 16;

/// Does the machine code run from the CALL instruction at `call` to the RETURN
/// instruction at `ret` without a conditional or indirect branch, another call,
/// or a write to any byte of `value`?
///
/// Deliberately a straight line: a value that reaches the RETURN through a join
/// (`if (r) return r;` over two calls) or past a conditional branch (a stack
/// protector's check) is not proved, and the park is refused.
///
/// Read from the image, one instruction at a time, because the p-code of a body
/// that returns nothing has already dropped a write to the return register as
/// dead. An unconditional jump is followed; a `jmp` to another function is the
/// CALL and the RETURN at one instruction, which is the empty path.
fn straight_to_return(arch: &Architecture, call: &Address, ret: &Address, value: Range) -> bool {
    if call == ret {
        return true;
    }
    let tr = arch.translate();
    let mut at = call.clone();
    for step in 0..STRAIGHT_PATH_INSTRUCTIONS {
        let mut emit = RawEmit::default();
        let len = match tr.one_instruction(&mut emit, &at) {
            Ok(n) if n > 0 => n,
            _ => return false,
        };
        let mut next = &at + len as i64;
        for (opc, out, ins) in &emit.ops {
            if step == 0 {
                continue;
            }
            let wrote = out.as_ref().and_then(|o| {
                let sp = o.space.as_ref()?;
                Some((sp.get_index(), o.offset, o.size as int4))
            });
            if wrote.is_some_and(|w| overlaps(w, value)) {
                return false;
            }
            match opc {
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_RETURN => return false,
                // A store through the register space is an indexed register
                // file: which register it writes is not known.
                OpCode::CPUI_STORE if ins.first().is_some_and(|v| v.offset as int4 == value.0) => {
                    return false
                }
                OpCode::CPUI_BRANCH => match ins.first().and_then(|v| v.space.clone()) {
                    Some(sp) if sp.get_type() == spacetype::IPTR_PROCESSOR => {
                        next = Address::new(sp, ins[0].offset);
                    }
                    _ => return false,
                },
                _ => {}
            }
        }
        if next == *ret {
            return true;
        }
        at = next;
    }
    false
}

/// The raw p-code of one machine instruction.
#[derive(Default)]
struct RawEmit {
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
}

impl kuna_sleigh::translate::PcodeEmit for RawEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

/// The instructions of the calls a body's flow does not continue past.
fn noreturn_calls(fd: &Funcdata) -> Vec<uintb> {
    (0..fd.num_calls())
        .filter(|&i| fd.get_call_specs(i).proto().is_no_return())
        .filter_map(|i| fd.obank().get(fd.get_call_specs(i).get_op()))
        .map(|o| o.get_addr().get_offset())
        .collect()
}

/// How many instructions [`zero_at_every_return`] reads before giving up.
const ZERO_WALK_INSTRUCTIONS: usize = 4096;

/// A raw varnode, reduced to what [`zero_mask_of`] asks of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Raw {
    kind: RawKind,
    space: int4,
    offset: uintb,
    size: int4,
}

/// Where a [`Raw`] lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RawKind {
    /// A constant: `offset` is its value.
    Const,
    /// An instruction-local temporary.
    Unique,
    /// Any other space, `space` naming it.
    Space,
}

impl Raw {
    fn of(v: &VarnodeData) -> Option<Raw> {
        let sp = v.space.as_ref()?;
        let kind = match sp.get_type() {
            spacetype::IPTR_CONSTANT => RawKind::Const,
            spacetype::IPTR_INTERNAL => RawKind::Unique,
            _ => RawKind::Space,
        };
        Some(Raw { kind, space: sp.get_index(), offset: v.offset, size: v.size as int4 })
    }
}

/// The bytes of a return window provably zero, as a mask over the window's
/// addresses (bit `i` is the byte at `window.1 + i`), with the temporaries of
/// the instruction being read.
#[derive(Clone, Debug, Default)]
struct ZeroState {
    window: u32,
    temps: HashMap<(uintb, int4), u32>,
}

/// The window byte a value's byte `i` (least significant first) lives at.
fn window_byte(at: &Raw, i: int4, window: Range, big_endian: bool) -> Option<u32> {
    let addr = if big_endian { at.offset + (at.size - 1 - i) as uintb } else { at.offset + i as uintb };
    (at.kind == RawKind::Space && at.space == window.0)
        .then_some(())
        .filter(|_| addr >= window.1 && addr < window.1 + window.2 as uintb)
        .map(|_| (addr - window.1) as u32)
}

/// The bytes of `v` provably zero, least significant first.
fn zero_mask_of(v: &Raw, st: &ZeroState, window: Range, big_endian: bool) -> u32 {
    match v.kind {
        RawKind::Const => (0..v.size.min(8))
            .filter(|i| (v.offset >> (8 * i)) & 0xff == 0)
            .fold(0, |m, i| m | 1 << i),
        RawKind::Unique => st.temps.get(&(v.offset, v.size)).copied().unwrap_or(0),
        RawKind::Space => {
            let mut m = 0;
            for i in 0..v.size.min(32) {
                match window_byte(v, i, window, big_endian) {
                    Some(b) if st.window & (1 << b) != 0 => m |= 1 << i,
                    Some(_) => {}
                    None => return 0,
                }
            }
            m
        }
    }
}

/// The bytes of an operation's output provably zero, least significant first.
fn zero_mask_out(code: OpCode, out: &Raw, ins: &[Raw], st: &ZeroState, window: Range, be: bool) -> u32 {
    let m = |k: usize| ins.get(k).map_or(0, |v| zero_mask_of(v, st, window, be));
    let low = |k: usize| ins.get(k).map_or(0, |v| (1u32 << v.size.clamp(0, 31)) - 1);
    let all = if out.size >= 32 { u32::MAX } else { (1u32 << out.size.max(0)) - 1 };
    let mask = match code {
        OpCode::CPUI_COPY => m(0),
        OpCode::CPUI_INT_ZEXT if !ins.is_empty() => m(0) | (all & !low(0)),
        OpCode::CPUI_INT_SEXT if !ins.is_empty() => {
            let top = (low(0) + 1) >> 1;
            m(0) | if m(0) & top != 0 { all & !low(0) } else { 0 }
        }
        OpCode::CPUI_INT_AND => m(0) | m(1),
        OpCode::CPUI_INT_OR => m(0) & m(1),
        OpCode::CPUI_INT_XOR if ins.len() == 2 && ins[0] == ins[1] && ins[0].kind != RawKind::Const => all,
        OpCode::CPUI_INT_XOR => m(0) & m(1),
        OpCode::CPUI_SUBPIECE => {
            let cut = ins.get(1).map_or(0, |c| c.offset as u32);
            if cut >= 32 { 0 } else { m(0) >> cut }
        }
        OpCode::CPUI_PIECE => m(1) | (m(0) << ins.get(1).map_or(0, |v| v.size.clamp(0, 31))),
        _ => 0,
    };
    mask & all
}

/// Apply one raw operation's write to `st`. A `conditional` write may not
/// happen, so a byte it zeroes is zero only if it already was.
fn write_zero(code: OpCode, out: &Raw, ins: &[Raw], st: &mut ZeroState, window: Range, be: bool, conditional: bool) {
    let mask = zero_mask_out(code, out, ins, st, window, be);
    match out.kind {
        RawKind::Unique => {
            st.temps.insert((out.offset, out.size), mask);
        }
        RawKind::Space => {
            for i in 0..out.size.min(32) {
                let Some(b) = window_byte(out, i, window, be) else { continue };
                let zero = mask & (1 << i) != 0 && (!conditional || st.window & (1 << b) != 0);
                if zero {
                    st.window |= 1 << b;
                } else {
                    st.window &= !(1 << b);
                }
            }
        }
        RawKind::Const => {}
    }
}

/// The bytes of `window` provably zero at every RETURN the machine code from
/// `entry` reaches, as a mask over the window's addresses.
///
/// A body's own recovery can return fewer bytes than a slot declares, and the
/// declaration then prints the rest of the register too. Whether those bytes
/// are clean is a property of the machine code, which a byte write does not
/// settle: `movzbl (%rdi),%eax; sub (%rsi),%al` leaves zero above `al`,
/// `mov (%rdi),%al; sub (%rsi),%al` and `setg %al` leave whatever the caller
/// left there. So the machine code is walked from the entry, every path, with
/// each byte of the window held zero only where every path into an
/// instruction agrees: a constant with a zero byte, a zero-extension, a
/// 32-bit write on x86-64 (the specification writes it as one), an `and`
/// with a zero byte, an `xor` of a register with itself. A call clobbers the
/// window, and a call the body's own flow does not continue past (`stop_at`)
/// ends the path. `None` when the walk cannot finish: an indirect branch, an
/// instruction it cannot decode, more than [`ZERO_WALK_INSTRUCTIONS`], or no
/// RETURN at all.
fn zero_at_every_return(arch: &Architecture, entry: &Address, window: Range, stop_at: &[uintb]) -> Option<u32> {
    let tr = arch.translate();
    let be = entry.get_space()?.is_big_endian();
    let mut seen: HashMap<uintb, u32> = HashMap::new();
    let mut work: Vec<(Address, u32)> = vec![(entry.clone(), 0)];
    let mut at_return: Option<u32> = None;
    let mut budget = ZERO_WALK_INSTRUCTIONS;
    while let Some((at, incoming)) = work.pop() {
        let state = match seen.get(&at.get_offset()) {
            Some(old) if old & incoming == *old => continue,
            Some(old) => old & incoming,
            None => incoming,
        };
        seen.insert(at.get_offset(), state);
        budget = budget.checked_sub(1)?;
        let mut emit = RawEmit::default();
        let len = match tr.one_instruction(&mut emit, &at) {
            Ok(n) if n > 0 => n,
            _ => return None,
        };
        let next = &at + len as i64;
        let relative = |v: &Vec<VarnodeData>| {
            v.first().and_then(|t| t.space.as_ref()).is_some_and(|sp| sp.get_type() == spacetype::IPTR_CONSTANT)
        };
        let conditional = emit.ops.iter().any(|(opc, _, ins)| {
            matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) && relative(ins)
        });
        let mut st = ZeroState { window: state, temps: HashMap::new() };
        let mut falls_through = true;
        for (opc, out, ins) in &emit.ops {
            match opc {
                OpCode::CPUI_BRANCHIND => return None,
                OpCode::CPUI_RETURN => {
                    at_return = Some(at_return.map_or(st.window, |m| m & st.window));
                    falls_through = false;
                    break;
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    st.window = 0;
                    if stop_at.contains(&at.get_offset()) {
                        falls_through = false;
                        break;
                    }
                }
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH if !relative(ins) => {
                    let sp = ins.first().and_then(|t| t.space.clone())?;
                    work.push((Address::new(sp, ins[0].offset), st.window));
                    if *opc == OpCode::CPUI_BRANCH {
                        falls_through = false;
                        break;
                    }
                }
                OpCode::CPUI_STORE if ins.first().is_some_and(|v| v.offset as int4 == window.0) => {
                    st.window = 0;
                }
                _ => {
                    let Some(o) = out.as_ref().and_then(Raw::of) else { continue };
                    let raws: Vec<Raw> = ins.iter().filter_map(Raw::of).collect();
                    let code = if raws.len() == ins.len() { *opc } else { OpCode::CPUI_LOAD };
                    write_zero(code, &o, &raws, &mut st, window, be, conditional);
                }
            }
        }
        if falls_through {
            work.push((next, st.window));
        }
    }
    at_return
}

/// `(declared, value)` when the body computes a return narrower than the
/// declared one, so the declaration would also print the bytes above it.
fn narrower_output(own: &OwnFacts, sig: &Sig, declared_out: Option<Range>) -> Option<(Range, Range)> {
    if sig.ret == Ty::Void || !own.returns_a_value {
        return None;
    }
    let (value, declared) = (own.output?, declared_out?);
    (contains(declared, value) && value != declared).then_some((declared, value))
}

/// Are the bytes of the declared return `declared` that the computed value
/// `value` does not occupy provably zero at every RETURN? `zero` is
/// [`zero_at_every_return`] over `declared`.
fn rest_is_zero(declared: Range, value: Range, zero: Option<u32>) -> bool {
    let Some(zero) = zero else { return false };
    (0..declared.2.min(32)).all(|i| {
        let byte = (declared.0, declared.1 + i as uintb, 1);
        contains(value, byte) || zero & (1 << i) != 0
    })
}

/// The storage the prototype model gives a declared return value.
fn output_storage(pieces: &PrototypePieces, arch: &Architecture) -> Option<Range> {
    let dfp = arch.default_fp()?.clone();
    let void_ty = arch.types().get_type_void().ok()?;
    let types_only =
        PrototypePieces { input_storage: Vec::new(), output_storage: None, ..pieces.clone() };
    let mut fp = FuncProto::new();
    fp.seed_locked_from_pieces(&types_only, dfp, void_ty, arch.types(), arch.manage()).ok()?;
    let out = fp.get_output();
    let addr = out.get_address();
    Some((addr.get_space()?.get_index(), addr.get_offset(), out.get_size()))
}

/// Does the body's own recovery contradict the declaration `sig`?
///
/// `called_directly` is whether anything calls or tail-jumps to the function;
/// `declared_in` the storage the prototype model gives `sig`'s parameters, in
/// order, and `declared_out` the storage it gives `sig`'s return.
///
/// * A `void` slot on a body that hands back a value it computed.
/// * More recovered inputs than the declaration passes. The list is closed, so
///   the extra one is dropped at every direct call site and read uninitialized
///   in the body; a forwarder that passes the register on without reading it
///   trips only this, not [`crate::kuna_protoorder::reads_past_the_list`].
/// * Fewer, when something calls the function directly: the closed list
///   materializes an argument at each such site out of whatever the register
///   last held, so `signal(SIGALRM, cleanup); ... cleanup();` on a
///   `void cleanup(void)` grows an invented assignment and argument. With no
///   direct caller there is no site to invent at and the declared parameter is
///   simply unread, which is what a handler that ignores its signal number
///   looks like in DWARF too; that shape is parked.
/// * A recovered input outside the declared storage at its position, with or
///   without a direct caller. A `struct ctx *` routine cast into `signal`'s
///   `void (*)(int)` reads all eight bytes of the register the slot declares a
///   four-byte `int` in: the body would rebuild its pointer from the declared
///   half and a register nothing set, and every direct call would truncate its
///   argument to `int`.
/// * A value-returning slot on a body that computes none. The register's
///   leftover prints as an invented expression -- the high half of a `void *`
///   a tail-called `puts` never wrote reads as a global that is not in the
///   image. A body whose calls leave a declared value covering the whole
///   declared return, with nothing after them writing it, does compute it
///   ([`call_return`]).
/// * A value-returning slot on a body that hands back more than the declared
///   return holds, whether it computed the value itself or a call left it:
///   a `long` comparator cast into `qsort`'s `int` slot would compute in the
///   low half, and a direct caller reading the whole register would get its
///   high half back as an invented global. What counts is the value, not the
///   storage its own recovery gave it: every 32-bit write on x86-64
///   zero-extends into the whole register, so an `int` comparator ending
///   `movzbl %al,%eax; cmovl %edx,%eax` recovers an 8-byte output whose upper
///   half is provably zero. Such a value fits the declared return
///   ([`OwnFacts::output_set`]) and stands -- unless something calls the
///   function directly, since a direct caller that reads the whole register
///   is the one thing that tells a `long` holding a zero-extended value from
///   an `int`.
fn body_contradicts(
    own: &OwnFacts,
    sig: &Sig,
    called_directly: bool,
    declared_in: &[Range],
    declared_out: Option<Range>,
) -> Option<Decline> {
    if sig.ret == Ty::Void && own.returns_a_value {
        return Some(Decline::ReturnsAValue);
    }
    if own.inputs.len() > sig.params.len() {
        return Some(Decline::MoreInputsThanDeclared);
    }
    if own.inputs.len() < sig.params.len() && called_directly {
        return Some(Decline::FewerInputsThanDeclared);
    }
    let outside = own
        .inputs
        .iter()
        .enumerate()
        .any(|(i, got)| declared_in.get(i).is_none_or(|want| !contains(*want, *got)));
    if outside {
        return Some(Decline::WiderInputThanDeclared);
    }
    if sig.ret != Ty::Void {
        let handed_back = if own.returns_a_value { own.output } else { own.call_return };
        let Some(value) = handed_back else { return Some(Decline::ReturnsNoValue) };
        let Some(declared) = declared_out else { return Some(Decline::ReturnsNoValue) };
        if !own.returns_a_value && !contains(value, declared) {
            return Some(Decline::ReturnsNoValue);
        }
        let fits = own.returns_a_value
            && !called_directly
            && own.output_set.is_some_and(|set| contains(declared, set));
        if !contains(declared, value) && !fits {
            return Some(Decline::WiderOutputThanDeclared);
        }
    }
    None
}

/// What the image says about a function, which this crate cannot read.
#[derive(Debug, Clone, Copy, Default)]
pub struct ImageFacts {
    /// The address reaches somewhere the recorded callback arguments do not
    /// explain.
    pub escapes: bool,
    /// The run decompiles the function at all.
    pub is_target: bool,
    /// Something calls or tail-jumps to the function directly.
    pub called_directly: bool,
}

/// Park the declared prototype of the callback slot `value` was handed to.
pub fn park(
    arch: &mut Architecture,
    entry: &Address,
    name: &str,
    image: ImageFacts,
) -> Result<PrototypePieces, Decline> {
    let Some(row) = arch.kuna_callbacktype.decided().remove(&entry.get_offset()) else {
        return Err(Decline::NotATarget);
    };
    if !image.is_target {
        return Err(Decline::NotATarget);
    }
    if row.conflict {
        return Err(Decline::Conflict);
    }
    if image.escapes || row.other_use {
        return Err(Decline::Escapes);
    }
    if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        return Err(Decline::Declared);
    }
    let key = key_of(entry).ok_or(Decline::StorageUnderivable)?;
    let own = arch.kuna_callbacktype.own.get(&key).cloned().ok_or(Decline::BodyNotSeen)?;
    let pieces = pieces_of(&row.sig, name, arch).ok_or(Decline::StorageUnderivable)?;
    let storage =
        crate::kuna_protoorder::model_storage(&pieces, arch).ok_or(Decline::StorageUnderivable)?;
    if storage.len() != pieces.intypes.len() {
        return Err(Decline::StorageUnderivable);
    }
    let declared_in: Vec<Range> = storage.iter().map(|(a, size)| range_of(a, *size)).collect();
    let declared_out = output_storage(&pieces, arch);
    if let Some(why) =
        body_contradicts(&own, &row.sig, image.called_directly, &declared_in, declared_out)
    {
        return Err(why);
    }
    if arch.kuna_callbacktype.reads_of(entry.get_offset()).any(|read| {
        declared_out.is_none_or(|declared| !contains(declared, read))
    }) {
        return Err(Decline::CallerReadsAWiderReturn);
    }
    if let Some((declared, value)) = narrower_output(&own, &row.sig, declared_out) {
        let zero = zero_at_every_return(arch, entry, declared, &own.noreturn_calls);
        if !rest_is_zero(declared, value, zero) {
            return Err(Decline::NarrowerOutputThanDeclared);
        }
    }
    if trace() && row.sig.ret != Ty::Void && !own.returns_a_value {
        eprintln!("[callbacktype] {name}: the return is a call's declared value");
    }
    if crate::kuna_protoorder::reads_past_the_list(arch, entry, &pieces, &storage) {
        return Err(Decline::ReadsPastTheList);
    }
    if agrees(own.printed.as_ref(), &pieces) {
        return Err(Decline::Agrees);
    }
    arch.set_function_prototype_pieces_at(entry, pieces.clone());
    // The declaration replaces what the first pass recovered. `protoorder`
    // filed the recovered types for the callback's own call sites and
    // declines to file again once a prototype is parked, so the stale
    // statement would outlive the body it came from; `calleevote` would keep
    // voting on a function that now has a declared prototype. Both are
    // dropped here, and the redo files whatever is true of the new body.
    if let Some(key) = crate::kuna_protoorder::stated_key(entry) {
        arch.kuna_protoorder_types.remove(&key);
        arch.kuna_calleevote.forget(key);
        arch.kuna_calleevote.own.remove(&key);
    }
    Ok(pieces)
}

#[cfg(test)]
mod tests;
