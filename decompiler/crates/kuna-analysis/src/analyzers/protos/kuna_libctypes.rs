//! (kuna `libctypes`) Named libc/POSIX aggregate types in the built-in prototype
//! tables — `FILE *` where the width-stable vocabulary can only say `void *`.
//!
//! [`super::Ty`] is width-stable by construction, so every aggregate pointer in
//! the two shipped tables is spelled `void *`: `fopen` returns one, `fclose`
//! takes one, `stat` fills one, `getopt_long` reads one. That is honest about
//! the width and silent about the pointee, and the pointee is the one thing the
//! table actually knows — `int fclose(FILE *)` is a declaration, not an
//! inference. This module carries the same signatures with the aggregate slots
//! named, plus the stdio entries whose only purpose is to name a parameter
//! (`int __uflow(FILE *)` is the sole type evidence a `-O2` coreutils reader
//! loop has for its stream argument).
//!
//! ## The shells are SIZED, and still incomplete
//!
//! A named shell of size 0 is not opaque, it is broken. `AddTreeState` has no
//! size-0 early out, and `RulePtrsubUndo`'s no-field arm short-circuits on
//! `typesize != 0` (`substrate/dtype.rs`), so a `PTRSUB` into a zero-size
//! pointee is declared MATCHING and survives to the printer, which renders it in
//! FUNCTIONAL form — literal `PTRSUB(p,0x28)` inside the C, on exactly the
//! `stdout + 0x28` and `f + 8` accesses this table exists to type. So each shell
//! carries its real glibc x86-64 width (`sizeof`/`_Alignof`, [`NAMED_AGGREGATES`]),
//! which makes an in-range access render `p->field_0x28` and an out-of-range one
//! fall back to the cast form.
//!
//! The shells are nevertheless kept INCOMPLETE (`flags::type_incomplete` is
//! passed back through `set_fields_struct_raw`, which re-ORs it), so the `.h`
//! emitter prints `typedef struct FILE FILE; /* opaque */` with no body rather
//! than a struct with a width and no members.
//!
//! The widths are the glibc **x86-64** ones, which is the ABI of the corpus this
//! option was measured on. On another ABI a width can be a few bytes off, and the
//! only thing that can change is whether an access at a given offset renders as a
//! field or as a cast — the NAME, which is the whole point, is ABI-independent,
//! and a wrong width can never make a pointer point at the wrong thing.
//!
//! ## A `-g` image already has the real thing
//!
//! On an image that carries debug info the platform's own `struct stat` is
//! interned with its real layout, and THAT is what this table points at — the
//! pass runs after `DwarfPass` for exactly that reason (`passes.rs`), and
//! [`named_aggregate`] adopts a held complete aggregate of the declared width
//! instead of minting a shell. glibc spells `FILE` as `struct _IO_FILE`, so that
//! alias is tried too; without it the image would carry two stream types and the
//! body would cast between them.
//!
//! The order is not cosmetic. A pointee is captured as an `Rc<Datatype>` when the
//! signature is built, and completing a struct re-keys it into a NEW `Rc`
//! (`TypeFactory::set_fields_struct` — the C++ mutates a `TypeStruct` in place,
//! the Rust clones). A shell minted first and completed by DWARF afterwards is
//! therefore completed for everyone EXCEPT the pointers this table already
//! built, and `st->st_mode` degrades to `*(int *)&st->field_0x18`. Running
//! second removes the window: this table never completes, re-keys or alters a
//! type it did not establish, and declines the signature outright when the name
//! is held by something else.
//!
//! ## Names are the bare DWARF spelling
//!
//! `FILE`, `stat`, `passwd`, … — not `struct stat`. kuna's printer spells a
//! named base bare and its `.h` emitter supplies the matching
//! `typedef struct stat stat;`, so the bare form is the one that round-trips
//! through `decompile-project`.
//!
//! ## Provenance
//!
//! Same rule as [`super::kuna_libcsigs`]: no signature here was written from
//! memory. The retargets restate a signature already in the shipped tables with
//! its aggregate slots named; each name new to this module was reduced from
//! `gcc -aux-info` over the platform headers with `_GNU_SOURCE` +
//! `_FORTIFY_SOURCE=2`. `__underflow` was REJECTED for want of one: glibc
//! declares it only in its internal `libioP.h`, which is not installed, so there
//! is no machine-readable declaration to reduce. `fgetc_unlocked` and `fmemopen`
//! were rejected on evidence instead — zero call sites across the 524 stripped
//! O0+O2 decbench ELFs, where every name kept here has at least 17 (`rewind` 117,
//! `__uflow` 77, `fgetc` 57; the census is in `docs/features/libctypes/`).
//!
//! The `v*printf` family is the trap this table must not fall into: the last
//! `void *` of `vasprintf`, `vsnprintf`, `__vasprintf_chk`, `__vfprintf_chk`,
//! `__vsnprintf_chk`, `verr` and `vwarn` is a `va_list`, not a `FILE *`. A
//! blanket `VoidPtr -> NamedPtr` retarget would assert a false type at every one
//! of those call sites, so the retarget is enumerated by hand, slot by slot.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{flags, type_metatype, Datatype, TypeFactory};

use super::{
    resolved_import_addrs, seed_named_prototypes, seed_resolved_prototypes,
    unambiguous_imported_function_names, Sig, Ty,
};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

pub(super) mod glibc;

/// How much of an aggregate this table says — the `libctypes` value, as the
/// option's own gate module spells it.
pub(super) use kuna_decomp::kuna_libctypes::LibcTypesLayout as Layout;

/// Seed the named-aggregate libc signatures. Registered after
/// [`super::LibProtoPass`] and [`super::kuna_libcsigs::LibcSigsPass`] so its
/// prototypes are committed last and win for the names it restates.
pub struct LibcTypesPass;

/// Whether the named libc aggregate types are enabled for this process (the
/// `libctypes` env bridge — the shells are interned inside `load file`, upstream
/// of every `option` command).
pub(super) fn enabled() -> bool {
    kuna_decomp::kuna_libctypes::libctypes_enabled()
}

/// A named aggregate the table may point at: its bare DWARF spelling and its
/// glibc x86-64 ABI width and alignment (`sizeof`/`_Alignof`, measured with the
/// platform headers — see `docs/features/libctypes/analysis.md`).
///
/// `DIR` is the one true opaque: glibc publishes no layout for it at all, so it
/// takes width 1 — a non-zero width, which is the only property the pointer
/// arithmetic seam needs.
pub(super) struct NamedAggregate {
    /// The bare name the type is interned and printed under.
    pub name: &'static str,
    /// The spelling the platform's own debug info uses for the same aggregate,
    /// when it differs. glibc's `FILE` is a typedef of `struct _IO_FILE`, so a
    /// `-g` image defines the layout under THAT tag; adopting it is what keeps
    /// one type where there would otherwise be two (and the decompiled body free
    /// of `(FILE *)` casts between them).
    pub dwarf_alias: Option<&'static str>,
    /// Byte width of the aggregate.
    pub size: int4,
    /// Byte alignment of the aggregate.
    pub align: int4,
}

/// Every aggregate [`Ty::NamedPtr`] may name. Sorted by name; the table is
/// searched linearly (16 rows, a handful of times per load).
pub(super) const NAMED_AGGREGATES: &[NamedAggregate] = &[
    NamedAggregate { name: "DIR", dwarf_alias: None, size: 1, align: 1 },
    NamedAggregate { name: "FILE", dwarf_alias: Some("_IO_FILE"), size: 216, align: 8 },
    NamedAggregate { name: "dirent", dwarf_alias: None, size: 280, align: 8 },
    NamedAggregate { name: "group", dwarf_alias: None, size: 32, align: 8 },
    NamedAggregate { name: "mbstate_t", dwarf_alias: None, size: 8, align: 4 },
    NamedAggregate { name: "option", dwarf_alias: None, size: 32, align: 8 },
    NamedAggregate { name: "passwd", dwarf_alias: None, size: 48, align: 8 },
    NamedAggregate { name: "pthread_mutex_t", dwarf_alias: None, size: 40, align: 8 },
    NamedAggregate { name: "sigaction", dwarf_alias: None, size: 152, align: 8 },
    NamedAggregate { name: "sigset_t", dwarf_alias: None, size: 128, align: 8 },
    NamedAggregate { name: "sockaddr", dwarf_alias: None, size: 16, align: 2 },
    NamedAggregate { name: "stat", dwarf_alias: None, size: 144, align: 8 },
    NamedAggregate { name: "termios", dwarf_alias: None, size: 60, align: 4 },
    NamedAggregate { name: "timespec", dwarf_alias: None, size: 16, align: 8 },
    NamedAggregate { name: "timeval", dwarf_alias: None, size: 16, align: 8 },
    NamedAggregate { name: "tm", dwarf_alias: None, size: 56, align: 8 },
];

/// The aggregate this table will point at for `name`: the platform's own
/// definition when the image carries one, else a named, sized, empty shell.
///
/// The pass runs after `DwarfPass`, so "the platform's own" means simply
/// "already interned". What is adopted is decided by METATYPE AND WIDTH only —
/// a struct of the declared width is taken whether it is complete or still a
/// shell. Three outcomes:
///
/// * a struct of the declared width is held under `name` or under its
///   [`NamedAggregate::dwarf_alias`] — adopted verbatim. That covers the real
///   `struct stat` this file was built against (complete, with its fields), a
///   shell the DWARF importer is still populating (it completes in place, under
///   the pointers already handed out), and the shell an earlier slot of this
///   same table minted;
/// * anything else is held under `name` — a struct of a DIFFERENT width, a
///   non-struct, or the width-0 type a bare forward declaration interns as
///   (whose pointee would survive `RulePtrsubUndo` and print as a functional
///   `PTRSUB`) — and the whole signature is declined. This table never alters,
///   completes or re-keys a definition it did not establish: completion mints a
///   NEW `Rc` (`TypeFactory::set_fields_struct`), so re-keying another owner's
///   shell would strand every pointer already minted against it;
/// * nothing is held — mint the sized, still-incomplete shell.
///
/// Completeness is deliberately not part of the test. Requiring it would make
/// slot 2 of a signature decline the shell slot 1 just minted, and the table
/// would silently degrade to `void *` everywhere.
///
/// Idempotent by construction: the second call finds the interned type by name
/// and hands back the same `Rc`, so every `FILE *` in the table is the same
/// pointee object.
pub(super) fn named_aggregate(
    name: &str,
    types: &dyn TypeFactory,
    word_size: uint4,
    layout: Layout,
) -> KunaResult<Rc<Datatype>> {
    let Some(agg) = NAMED_AGGREGATES.iter().find(|a| a.name == name) else {
        return Err(KunaError::lowlevel(format!(
            "libctypes: no width is known for `{name}`"
        )));
    };
    // The declared width is the identity test. A struct of that width under that
    // name is either the platform's own definition (complete, with its fields) or
    // the shell an earlier slot of this same table minted — adopt either. Anything
    // else is somebody else's `stat`: a program type that shares the spelling, a
    // non-struct, or a zero-width forward declaration whose pointee would survive
    // `RulePtrsubUndo` as a functional `PTRSUB`.
    let usable = |held: &Rc<Datatype>| {
        held.get_metatype() == type_metatype::TYPE_STRUCT && held.get_size() == agg.size
    };
    if let Some(held) = types.find_by_name(name)? {
        if !usable(&held) {
            return Err(KunaError::lowlevel(format!(
                "libctypes: `{name}` is already held by a different type"
            )));
        }
        return Ok(held);
    };
    if let Some(alias) = agg.dwarf_alias {
        if let Some(held) = types.find_by_name(alias)? {
            if usable(&held) {
                return Ok(held);
            }
        }
    }
    // `glibc` and a layout to install: the fields are interned FIRST, so the
    // shell is minted only once the whole layout is known and is never handed
    // out half-built (no field names this aggregate, which is what makes that
    // order possible — see `glibc`'s module header). The completed struct drops
    // `type_incomplete`: it has its members, and the `.h` emitter should print
    // them.
    if layout == Layout::Glibc {
        if let Some(rows) = glibc::layout_for(name) {
            let fields = glibc::build_fields(rows, types, word_size)?;
            let shell = types.get_type_struct(name)?;
            return types
                .set_fields_struct_raw(&shell, fields, Vec::new(), agg.size, agg.align, 0);
        }
    }
    let shell = types.get_type_struct(name)?;
    // The width is the point (a zero-size pointee survives RulePtrsubUndo and
    // prints as functional `PTRSUB(p,off)`); `type_incomplete` is passed back so
    // the DWARF importer may still complete this shell in place.
    types.set_fields_struct_raw(
        &shell,
        Vec::new(),
        Vec::new(),
        agg.size,
        agg.align,
        flags::type_incomplete,
    )
}

/// Retargets of [`super::LIBC`] entries — matched against IMPORTED names, with
/// the same ambiguity guard (see [`LibcTypesPass::run`] for why this table is
/// stricter than the one it restates).
pub(super) const LIBC_NAMED: &[(&str, Sig)] = &[
    ("fopen", Sig { ret: Ty::NamedPtr("FILE"), params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("fprintf", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::CharPtr], vararg: 2 }),
    ("fputs", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::NamedPtr("FILE")], vararg: -1 }),
];

/// Retargets of [`super::kuna_libcsigs::LIBC_EXT`] entries, plus the stdio names
/// neither shipped table carries. Matched, like `LIBC_EXT`, against IMPORTED
/// names only: a coincidental spelling the image defines itself must not be
/// retyped, and that judgement does not change because the pointee got a name.
pub(super) const LIBC_EXT_NAMED: &[(&str, Sig)] = &[
    // stdio.h — the retargets
    ("__fpending", Sig { ret: Ty::Size, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("__fprintf_chk", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::Int, Ty::CharPtr], vararg: 3 }),
    ("__fpurge", Sig { ret: Ty::Void, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("__freading", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("__isoc99_fscanf", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::CharPtr], vararg: 2 }),
    ("__overflow", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::Int], vararg: -1 }),
    // `__vfprintf_chk(FILE *, int, const char *, va_list)` — p0 only; the LAST
    // slot is the `va_list` and stays `void *`.
    ("__vfprintf_chk", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::Int, Ty::CharPtr, Ty::VoidPtr], vararg: -1 }),
    ("clearerr_unlocked", Sig { ret: Ty::Void, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fclose", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fdopen", Sig { ret: Ty::NamedPtr("FILE"), params: &[Ty::Int, Ty::CharPtr], vararg: -1 }),
    ("feof", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("ferror", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("ferror_unlocked", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fflush", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fflush_unlocked", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fgets", Sig { ret: Ty::CharPtr, params: &[Ty::CharPtr, Ty::Int, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fileno", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fputc", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fputs_unlocked", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fread", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::Size, Ty::Size, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fscanf", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::CharPtr], vararg: 2 }),
    ("fseek", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::Long, Ty::Int], vararg: -1 }),
    ("ftell", Sig { ret: Ty::Long, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fwrite", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::Size, Ty::Size, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fwrite_unlocked", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::Size, Ty::Size, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("getc", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("getc_unlocked", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("getline", Sig { ret: Ty::Long, params: &[Ty::CharPtrPtr, Ty::VoidPtr, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("putc", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("putc_unlocked", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("setvbuf", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE"), Ty::CharPtr, Ty::Int, Ty::Size], vararg: -1 }),
    ("ungetc", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("FILE")], vararg: -1 }),
    // stdio.h — names neither shipped table carries. `int __uflow(FILE *)` is the
    // whole reason a `-O2` coreutils reader loop can be told what its stream
    // argument is: the inlined `getc` refill path calls it and nothing else in
    // the body says `FILE`.
    ("__uflow", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("fgetc", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("flockfile", Sig { ret: Ty::Void, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("freopen", Sig { ret: Ty::NamedPtr("FILE"), params: &[Ty::CharPtr, Ty::CharPtr, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("funlockfile", Sig { ret: Ty::Void, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    // `ssize_t getdelim(char **, size_t *, int delim, FILE *)` — the stream is the
    // FOURTH slot (the delimiter sits where `getline` has none).
    ("getdelim", Sig { ret: Ty::Long, params: &[Ty::CharPtrPtr, Ty::VoidPtr, Ty::Int, Ty::NamedPtr("FILE")], vararg: -1 }),
    ("pclose", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    ("popen", Sig { ret: Ty::NamedPtr("FILE"), params: &[Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("rewind", Sig { ret: Ty::Void, params: &[Ty::NamedPtr("FILE")], vararg: -1 }),
    // dirent.h
    ("closedir", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("DIR")], vararg: -1 }),
    ("dirfd", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("DIR")], vararg: -1 }),
    ("fdopendir", Sig { ret: Ty::NamedPtr("DIR"), params: &[Ty::Int], vararg: -1 }),
    ("opendir", Sig { ret: Ty::NamedPtr("DIR"), params: &[Ty::CharPtr], vararg: -1 }),
    ("readdir", Sig { ret: Ty::NamedPtr("dirent"), params: &[Ty::NamedPtr("DIR")], vararg: -1 }),
    // sys/stat.h
    ("fstat", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("stat")], vararg: -1 }),
    ("fstatat", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtr, Ty::NamedPtr("stat"), Ty::Int], vararg: -1 }),
    ("lstat", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::NamedPtr("stat")], vararg: -1 }),
    ("stat", Sig { ret: Ty::Int, params: &[Ty::CharPtr, Ty::NamedPtr("stat")], vararg: -1 }),
    // pwd.h / grp.h
    ("getgrgid", Sig { ret: Ty::NamedPtr("group"), params: &[Ty::UInt], vararg: -1 }),
    ("getgrnam", Sig { ret: Ty::NamedPtr("group"), params: &[Ty::CharPtr], vararg: -1 }),
    ("getpwnam", Sig { ret: Ty::NamedPtr("passwd"), params: &[Ty::CharPtr], vararg: -1 }),
    ("getpwuid", Sig { ret: Ty::NamedPtr("passwd"), params: &[Ty::UInt], vararg: -1 }),
    // time.h — `localtime`'s ARGUMENT is a `const time_t *`, which has no
    // width-stable spelling, so only the results and the explicit `struct tm *`
    // slots are named.
    ("localtime", Sig { ret: Ty::NamedPtr("tm"), params: &[Ty::VoidPtr], vararg: -1 }),
    ("localtime_r", Sig { ret: Ty::NamedPtr("tm"), params: &[Ty::VoidPtr, Ty::NamedPtr("tm")], vararg: -1 }),
    ("strftime", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::Size, Ty::CharPtr, Ty::NamedPtr("tm")], vararg: -1 }),
    ("clock_gettime", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("timespec")], vararg: -1 }),
    ("gettimeofday", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("timeval"), Ty::VoidPtr], vararg: -1 }),
    // getopt.h — the `(void *)0x…` constant a coreutils `main` passes is the
    // long-option table.
    ("getopt_long", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::CharPtrPtr, Ty::CharPtr, Ty::NamedPtr("option"), Ty::IntPtr], vararg: -1 }),
    // wchar.h — `mbrtowc`'s first slot is a `wchar_t *`, already spelled by
    // `WCharPtr` nowhere in this table's reach; it stays as the shipped entry has it.
    ("mbrlen", Sig { ret: Ty::Size, params: &[Ty::CharPtr, Ty::Size, Ty::NamedPtr("mbstate_t")], vararg: -1 }),
    ("mbrtowc", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::CharPtr, Ty::Size, Ty::NamedPtr("mbstate_t")], vararg: -1 }),
    ("mbsinit", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("mbstate_t")], vararg: -1 }),
    // signal.h
    ("sigaction", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("sigaction"), Ty::NamedPtr("sigaction")], vararg: -1 }),
    ("sigaddset", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("sigset_t"), Ty::Int], vararg: -1 }),
    ("sigemptyset", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("sigset_t")], vararg: -1 }),
    ("sigprocmask", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::NamedPtr("sigset_t"), Ty::NamedPtr("sigset_t")], vararg: -1 }),
    // termios.h / sys/socket.h / pthread.h
    ("getnameinfo", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("sockaddr"), Ty::UInt, Ty::CharPtr, Ty::UInt, Ty::CharPtr, Ty::UInt, Ty::Int], vararg: -1 }),
    ("pthread_mutex_lock", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("pthread_mutex_t")], vararg: -1 }),
    ("pthread_mutex_unlock", Sig { ret: Ty::Int, params: &[Ty::NamedPtr("pthread_mutex_t")], vararg: -1 }),
    ("tcsetattr", Sig { ret: Ty::Int, params: &[Ty::Int, Ty::Int, Ty::NamedPtr("termios")], vararg: -1 }),
];

/// The built-in signature for a name the OPERATOR declared, in its named-type
/// form. `None` when the gate is off or neither named table knows the name, in
/// which case [`super::declared_libc_prototype`] answers from the `void *`
/// tables exactly as before.
pub(super) fn declared_named_prototype(name: &str) -> Option<&'static Sig> {
    if !enabled() {
        return None;
    }
    LIBC_NAMED
        .iter()
        .chain(LIBC_EXT_NAMED.iter())
        .find(|(n, _)| *n == name)
        .map(|(_, sig)| sig)
}

impl AnalysisPass for LibcTypesPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "libctypes"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // The gate is read HERE, not at the commit: the shells are interned into
        // the type factory by `build_pieces` below, and with the gate off not one
        // of them may exist (an interned `stat` is exactly what the DWARF
        // importer would meet). Off is therefore the shipped tables, untouched.
        if !enabled() {
            return out;
        }
        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();
        // `glibc` asks for the published field layouts; whether they are TRUE of
        // this image is a separate question, and the only one that can make a
        // field name wrong. Refused => the `opaque` shells, exactly.
        let layout = match kuna_decomp::kuna_libctypes::libctypes_layout() {
            Layout::Glibc if glibc::target_is_glibc_x86_64(ctx.file) => Layout::Glibc,
            Layout::Off => return out,
            _ => Layout::Opaque,
        };
        // IMPORTED names only, for both tables — where `LibProtoPass` also matches
        // a name the image DEFINES. A defined `fopen` is this image's own
        // function, and on a `-g` image it has a DWARF prototype that this pass,
        // merged after `DwarfPass`, would otherwise outrank. Retargeting the
        // import is the whole job; the definition belongs to whoever declared it.
        let resolved = resolved_import_addrs(ctx.file, ctx.bytes);
        let imported = unambiguous_imported_function_names(ctx.file, ctx.bytes);
        seed_named_prototypes(&mut out, &imported, LIBC_NAMED, types, word_size, layout);
        seed_resolved_prototypes(&mut out, &resolved, LIBC_NAMED, types, word_size, layout);
        seed_named_prototypes(&mut out, &imported, LIBC_EXT_NAMED, types, word_size, layout);
        seed_resolved_prototypes(&mut out, &resolved, LIBC_EXT_NAMED, types, word_size, layout);
        out
    }
}

#[cfg(test)]
mod tests;
