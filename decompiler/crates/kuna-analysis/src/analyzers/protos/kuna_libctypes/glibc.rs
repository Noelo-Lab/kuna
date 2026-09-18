//! (kuna `libctypes glibc`) The PUBLISHED glibc x86-64 field layouts of the nine
//! aggregates whose members are part of the platform's API.
//!
//! [`super`]'s `opaque` value gives each named aggregate a real width and no
//! members, which is enough to keep the pointer arithmetic seam honest but says
//! nothing about what is at an offset: a `-O2` coreutils reader loop renders
//! `*(unsigned char **)(f + 8)` as `f->field_0x8` and the loaded value stays
//! `unsigned char *` only because the cast said so. This module carries the
//! layouts themselves, so the same access is `f->_IO_read_ptr` and the value
//! takes the FIELD's type.
//!
//! ## What is here, and what is deliberately not
//!
//! Only aggregates glibc publishes the members of in an installed header, and
//! whose members an application is meant to read: `FILE`
//! (`bits/types/struct_FILE.h`), `stat` (`bits/struct_stat.h`), `timespec`,
//! `timeval`, `tm`, `passwd`, `group`, `option` and `dirent`. `DIR`,
//! `sigaction`, `sigset_t`, `mbstate_t`, `termios`, `sockaddr` and
//! `pthread_mutex_t` keep the sized opaque shell: either glibc publishes no
//! layout at all (`DIR`), or the layout is a union of reserved words that no
//! caller reads by name.
//!
//! ## Provenance
//!
//! Every offset, width and alignment below was measured on the machine this was
//! written on, by compiling a program of `offsetof`/`sizeof`/`_Alignof` against
//! the installed headers — not restated from memory, and not copied out of a
//! header by eye. The program and its output are in
//! `docs/features/libctypes/glibc.md`.
//!
//! ## Three rules the tables obey
//!
//! * **No row glibc reserves for itself.** `stat::__pad0`,
//!   `stat::__glibc_reserved`, `_IO_FILE::__pad5` and `_IO_FILE::_unused2` are
//!   left as holes, which the printer renders as the offset form. A reserved
//!   member is one no program may read, so every truthful use of its name is a
//!   whole-struct copy spilling padding — which a reader learns nothing from —
//!   while every use that carries meaning is a lie: when a `stat *` lands on the
//!   first member of a bigger struct, that struct's own members at 0x78 and up
//!   get spelled `__glibc_reserved[1]` instead of the neutral `field_0x80`.
//!   Measured over 16 binaries, the rows fired 28 times: 19 padding copies a
//!   reader gains nothing from, 8 lies about diffutils' `file_data::desc` and
//!   `::name`, and one address computation the name made worse
//!   (`(long)v32.__glibc_reserved + v10 * 4 - 0x78`, now `(long)&v32.st_dev +
//!   v10 * 4`). The hole wins on every count that carries information.
//!
//! * **No self-reference.** `_IO_FILE::_chain` and `_IO_FILE::_freeres_list` are
//!   `struct _IO_FILE *` in the header and `void *` here. Completing a struct
//!   re-keys it into a NEW `Rc` (`TypeFactory::set_fields_struct`), so a
//!   `FILE *` built against the shell WHILE the shell is being filled would be
//!   stranded on the incomplete one and the program would carry two stream
//!   types. Nothing reads `_chain`; the cost is a name, the alternative is a
//!   split type.
//! * **At most one level of by-value nesting.** `stat` holds three `timespec`s
//!   by value; `timespec` holds no aggregate. [`tests`](super::tests) asserts
//!   that, so the recursive mint in [`super::named_aggregate`] terminates by
//!   construction rather than by a depth counter.

use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use std::rc::Rc;

use super::{named_aggregate, Layout};

/// The type of one field of a published layout. Spelled in C terms rather than
/// in glibc's `__dev_t`/`__off_t` typedefs, which carry no more information than
/// the width and signedness they expand to.
pub(super) enum FTy {
    /// `int` (4-byte signed).
    Int,
    /// `unsigned int` — `__mode_t`, `__uid_t`, `__gid_t`.
    UInt,
    /// `long` — `__off_t`, `__blkcnt_t`, `__time_t`, `__syscall_slong_t`.
    Long,
    /// `unsigned long` — `__dev_t`, `__ino_t`, `__nlink_t`, `size_t`.
    ULong,
    /// `unsigned short` — `_IO_FILE::_cur_column`, `dirent::d_reclen`.
    UShort,
    /// `unsigned char` — `dirent::d_type`.
    UChar,
    /// `signed char` — `_IO_FILE::_vtable_offset`.
    SChar,
    /// `char *`.
    CharPtr,
    /// `char **` — `group::gr_mem`.
    CharPtrPtr,
    /// `int *` — `option::flag`.
    IntPtr,
    /// `void *`, for a pointer to an aggregate this table does not name.
    VoidPtr,
    /// An array of `count` elements of the given type.
    Arr(&'static FTy, int4),
    /// The named aggregate, BY VALUE (`stat::st_atim`).
    Named(&'static str),
}

/// One field of a published layout: its byte offset inside the aggregate, its
/// name and its type.
pub(super) struct GlibcField {
    pub off: int4,
    pub name: &'static str,
    pub ty: FTy,
}

/// Shorthand for a row.
const fn f(off: int4, name: &'static str, ty: FTy) -> GlibcField {
    GlibcField { off, name, ty }
}

/// `struct _IO_FILE`, `sizeof` 216 — `bits/types/struct_FILE.h`. `_markers`,
/// `_chain`, `_lock`, `_codecvt`, `_wide_data`, `_freeres_list` and
/// `_freeres_buf` point at aggregates glibc does not publish (or at `FILE`
/// itself, which this table may not name — see the module header), so they are
/// `void *`. 0xb8 (`__pad5`) and 0xc4..0xd8 (`_unused2`) are reserved and stay
/// holes.
const FILE_FIELDS: &[GlibcField] = &[
    f(0x00, "_flags", FTy::Int),
    f(0x08, "_IO_read_ptr", FTy::CharPtr),
    f(0x10, "_IO_read_end", FTy::CharPtr),
    f(0x18, "_IO_read_base", FTy::CharPtr),
    f(0x20, "_IO_write_base", FTy::CharPtr),
    f(0x28, "_IO_write_ptr", FTy::CharPtr),
    f(0x30, "_IO_write_end", FTy::CharPtr),
    f(0x38, "_IO_buf_base", FTy::CharPtr),
    f(0x40, "_IO_buf_end", FTy::CharPtr),
    f(0x48, "_IO_save_base", FTy::CharPtr),
    f(0x50, "_IO_backup_base", FTy::CharPtr),
    f(0x58, "_IO_save_end", FTy::CharPtr),
    f(0x60, "_markers", FTy::VoidPtr),
    f(0x68, "_chain", FTy::VoidPtr),
    f(0x70, "_fileno", FTy::Int),
    f(0x74, "_flags2", FTy::Int),
    f(0x78, "_old_offset", FTy::Long),
    f(0x80, "_cur_column", FTy::UShort),
    f(0x82, "_vtable_offset", FTy::SChar),
    f(0x83, "_shortbuf", FTy::Arr(&FTy::SChar, 1)),
    f(0x88, "_lock", FTy::VoidPtr),
    f(0x90, "_offset", FTy::Long),
    f(0x98, "_codecvt", FTy::VoidPtr),
    f(0xa0, "_wide_data", FTy::VoidPtr),
    f(0xa8, "_freeres_list", FTy::VoidPtr),
    f(0xb0, "_freeres_buf", FTy::VoidPtr),
    f(0xc0, "_mode", FTy::Int),
];

/// `struct stat`, `sizeof` 144 — `bits/struct_stat.h`, x86-64 arm. 0x24
/// (`__pad0`) and 0x78..0x90 (`__glibc_reserved[3]`) are reserved and stay
/// holes.
const STAT_FIELDS: &[GlibcField] = &[
    f(0x00, "st_dev", FTy::ULong),
    f(0x08, "st_ino", FTy::ULong),
    f(0x10, "st_nlink", FTy::ULong),
    f(0x18, "st_mode", FTy::UInt),
    f(0x1c, "st_uid", FTy::UInt),
    f(0x20, "st_gid", FTy::UInt),
    f(0x28, "st_rdev", FTy::ULong),
    f(0x30, "st_size", FTy::Long),
    f(0x38, "st_blksize", FTy::Long),
    f(0x40, "st_blocks", FTy::Long),
    f(0x48, "st_atim", FTy::Named("timespec")),
    f(0x58, "st_mtim", FTy::Named("timespec")),
    f(0x68, "st_ctim", FTy::Named("timespec")),
];

/// `struct timespec`, `sizeof` 16 — `bits/types/struct_timespec.h`.
const TIMESPEC_FIELDS: &[GlibcField] =
    &[f(0x00, "tv_sec", FTy::Long), f(0x08, "tv_nsec", FTy::Long)];

/// `struct timeval`, `sizeof` 16 — `bits/types/struct_timeval.h`.
const TIMEVAL_FIELDS: &[GlibcField] =
    &[f(0x00, "tv_sec", FTy::Long), f(0x08, "tv_usec", FTy::Long)];

/// `struct tm`, `sizeof` 56 — `time.h`. 0x24..0x28 is alignment padding and
/// carries no member.
const TM_FIELDS: &[GlibcField] = &[
    f(0x00, "tm_sec", FTy::Int),
    f(0x04, "tm_min", FTy::Int),
    f(0x08, "tm_hour", FTy::Int),
    f(0x0c, "tm_mday", FTy::Int),
    f(0x10, "tm_mon", FTy::Int),
    f(0x14, "tm_year", FTy::Int),
    f(0x18, "tm_wday", FTy::Int),
    f(0x1c, "tm_yday", FTy::Int),
    f(0x20, "tm_isdst", FTy::Int),
    f(0x28, "tm_gmtoff", FTy::Long),
    f(0x30, "tm_zone", FTy::CharPtr),
];

/// `struct passwd`, `sizeof` 48 — `pwd.h`.
const PASSWD_FIELDS: &[GlibcField] = &[
    f(0x00, "pw_name", FTy::CharPtr),
    f(0x08, "pw_passwd", FTy::CharPtr),
    f(0x10, "pw_uid", FTy::UInt),
    f(0x14, "pw_gid", FTy::UInt),
    f(0x18, "pw_gecos", FTy::CharPtr),
    f(0x20, "pw_dir", FTy::CharPtr),
    f(0x28, "pw_shell", FTy::CharPtr),
];

/// `struct group`, `sizeof` 32 — `grp.h`. 0x14..0x18 is alignment padding.
const GROUP_FIELDS: &[GlibcField] = &[
    f(0x00, "gr_name", FTy::CharPtr),
    f(0x08, "gr_passwd", FTy::CharPtr),
    f(0x10, "gr_gid", FTy::UInt),
    f(0x18, "gr_mem", FTy::CharPtrPtr),
];

/// `struct option`, `sizeof` 32 — `getopt.h`. 0x0c..0x10 and the last four bytes
/// are alignment padding.
const OPTION_FIELDS: &[GlibcField] = &[
    f(0x00, "name", FTy::CharPtr),
    f(0x08, "has_arg", FTy::Int),
    f(0x10, "flag", FTy::IntPtr),
    f(0x18, "val", FTy::Int),
];

/// `struct dirent`, `sizeof` 280 — `bits/dirent.h` (the 64-bit arm, which is
/// what `readdir` returns on x86-64). The five bytes after `d_name` are tail
/// padding.
const DIRENT_FIELDS: &[GlibcField] = &[
    f(0x00, "d_ino", FTy::ULong),
    f(0x08, "d_off", FTy::Long),
    f(0x10, "d_reclen", FTy::UShort),
    f(0x12, "d_type", FTy::UChar),
    f(0x13, "d_name", FTy::Arr(&FTy::SChar, 256)),
];

/// Every published layout, keyed by the bare name `super::NAMED_AGGREGATES`
/// interns the aggregate under.
pub(super) const GLIBC_LAYOUTS: &[(&str, &[GlibcField])] = &[
    ("FILE", FILE_FIELDS),
    ("dirent", DIRENT_FIELDS),
    ("group", GROUP_FIELDS),
    ("option", OPTION_FIELDS),
    ("passwd", PASSWD_FIELDS),
    ("stat", STAT_FIELDS),
    ("timespec", TIMESPEC_FIELDS),
    ("timeval", TIMEVAL_FIELDS),
    ("tm", TM_FIELDS),
];

/// The published layout of `name`, or `None` for an aggregate that stays an
/// opaque shell under `glibc` too.
pub(super) fn layout_for(name: &str) -> Option<&'static [GlibcField]> {
    GLIBC_LAYOUTS.iter().find(|(n, _)| *n == name).map(|(_, fields)| *fields)
}

/// Intern the type of one field.
fn field_type(ty: &FTy, types: &dyn TypeFactory, word_size: uint4) -> KunaResult<Rc<Datatype>> {
    let ptr = types.get_size_of_pointer();
    match ty {
        FTy::Int => types.get_base(4, type_metatype::TYPE_INT),
        FTy::UInt => types.get_base(4, type_metatype::TYPE_UINT),
        FTy::Long => types.get_base(ptr, type_metatype::TYPE_INT),
        FTy::ULong => types.get_base(ptr, type_metatype::TYPE_UINT),
        FTy::UShort => types.get_base(2, type_metatype::TYPE_UINT),
        FTy::UChar => types.get_base(1, type_metatype::TYPE_UINT),
        FTy::SChar => types.get_type_char(types.get_size_of_char()),
        FTy::CharPtr => {
            let c = types.get_type_char(types.get_size_of_char())?;
            types.get_type_pointer(ptr, c, word_size)
        }
        FTy::CharPtrPtr => {
            let c = types.get_type_char(types.get_size_of_char())?;
            let cp = types.get_type_pointer(ptr, c, word_size)?;
            types.get_type_pointer(ptr, cp, word_size)
        }
        FTy::IntPtr => {
            let i = types.get_base(4, type_metatype::TYPE_INT)?;
            types.get_type_pointer(ptr, i, word_size)
        }
        FTy::VoidPtr => {
            let v = types.get_type_void()?;
            types.get_type_pointer(ptr, v, word_size)
        }
        FTy::Arr(elem, count) => {
            let e = field_type(elem, types, word_size)?;
            types.get_type_array(*count, e)
        }
        // One level deep by construction (`stat` holds `timespec`s and nothing
        // else nests), so this recursion has no cycle to fall into; the unit
        // test `glibc_layouts_nest_at_most_one_level` is what keeps it that way.
        FTy::Named(n) => named_aggregate(n, types, word_size, Layout::Glibc),
    }
}

/// The published layout of `name` as interned [`TypeField`]s, in offset order.
/// `ident` is the offset, which is the convention the DWARF importer uses.
pub(super) fn build_fields(
    fields: &'static [GlibcField],
    types: &dyn TypeFactory,
    word_size: uint4,
) -> KunaResult<Vec<TypeField>> {
    let mut out = Vec::with_capacity(fields.len());
    for field in fields {
        let ty = field_type(&field.ty, types, word_size)?;
        out.push(TypeField::new(field.off, field.off, field.name, ty));
    }
    Ok(out)
}

/// Is this image one the layouts above are TRUE of?
///
/// They are glibc's x86-64 layouts and nobody else's. Another libc's `FILE` is a
/// different object with a different width and a different member order, and a
/// `stat` that came off one shares with this table only the names the kernel ABI
/// fixes. Naming a field is a claim about what is at an offset, so it is made
/// only where the claim is checkable —
///
/// * the image is an **ELF** for **x86-64**, and
/// * its dynamic string table names glibc: the `libc.so.6` soname, or a
///   `GLIBC_2.x` symbol version (`.gnu.version_r`'s version names live in
///   `.dynstr`).
///
/// A statically linked image has no `.dynstr` and is refused, which costs
/// nothing: the tables these layouts serve are matched against IMPORTED names
/// only, so a static image gets no named aggregate to lay out in the first
/// place. Everything refused here keeps the sized opaque shells, which is the
/// `opaque` value's behaviour exactly.
pub(crate) fn target_is_glibc_x86_64(file: &object::File) -> bool {
    use object::{Object, ObjectSection};
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
    {
        return false;
    }
    let Some(dynstr) = file.section_by_name(".dynstr") else {
        return false;
    };
    let Ok(data) = dynstr.data() else {
        return false;
    };
    data.split(|b| *b == 0)
        .any(|s| s == b"libc.so.6" || s.starts_with(b"GLIBC_"))
}
