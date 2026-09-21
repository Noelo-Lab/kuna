//! Unit tests for the named libc aggregate table.

use super::*;
use kuna_decomp::dtype::TypeFactoryImpl;

/// The `libctypes` gate is a process environment variable and these tests run as
/// threads of one process, so the three that write it have to take turns. Without
/// this, `the_declared_lookup_follows_the_gate` reads a value another test set.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// A type factory shaped like the x86-64 one the passes run against.
fn factory() -> TypeFactoryImpl {
    let types = TypeFactoryImpl::new();
    types.set_default_alignment_map();
    types.set_max_basetype_size(8);
    types.setup_sizes(Some(8), 8, 4);
    types.set_core_type("char", 1, type_metatype::TYPE_INT, true).expect("char core type");
    types.cache_core_types().expect("cache core types");
    types
}

/// Minting the same aggregate twice must hand back the SAME interned type, not a
/// second definition: `fopen`'s result and `fclose`'s parameter have to be one
/// `FILE`, and `get_type_struct` alone would fail the second call (its size-0
/// probe no longer compares equal to the sized shell).
#[test]
fn a_named_aggregate_is_minted_once() {
    let types = factory();
    let first = named_aggregate("FILE", &types, 1, Layout::Opaque).expect("mint FILE");
    let second = named_aggregate("FILE", &types, 1, Layout::Opaque).expect("re-find FILE");
    assert!(Rc::ptr_eq(&first, &second), "FILE must intern once");
    assert_eq!(first.get_size(), 216, "the glibc x86-64 width, not 0");
}

/// The width is the whole point: a zero-width pointee survives `RulePtrsubUndo`
/// (its no-field arm short-circuits on `typesize != 0`) and the printer then
/// emits the functional `PTRSUB(p,0x28)` form inside the C.
#[test]
fn every_named_aggregate_is_sized_and_incomplete() {
    let types = factory();
    for agg in NAMED_AGGREGATES {
        let ct = named_aggregate(agg.name, &types, 1, Layout::Opaque).expect("mint");
        assert_eq!(ct.get_size(), agg.size, "{}: width", agg.name);
        assert!(ct.get_size() > 0, "{}: a 0-width pointee keeps PTRSUB alive", agg.name);
        assert_eq!(ct.get_metatype(), type_metatype::TYPE_STRUCT, "{}", agg.name);
        assert!(
            ct.is_incomplete(),
            "{}: the shell stays incomplete so DWARF may complete it in place",
            agg.name
        );
    }
}

/// A name a stronger source already defined is left exactly as it is: this table
/// never alters somebody else's definition (which `find_add` would refuse
/// anyway, with "Trying to alter definition of type").
#[test]
fn a_completed_name_is_left_alone() {
    let types = factory();
    let shell = types.get_type_struct("stat").expect("shell");
    let int4t = types.get_base(4, type_metatype::TYPE_INT).expect("int");
    let field = kuna_decomp::dtype::TypeField::new(0, 0, "st_dev", int4t);
    let complete = types
        .set_fields_struct_raw(&shell, vec![field], Vec::new(), 144, 8, 0)
        .expect("complete stat");
    let got = named_aggregate("stat", &types, 1, Layout::Opaque).expect("find the held stat");
    assert!(Rc::ptr_eq(&complete, &got), "the held definition is the answer");
    assert!(!got.is_incomplete(), "and it stays complete");
}

/// A name held by a struct of a DIFFERENT width is a program's own type that
/// merely shares the spelling; pointing a libc signature at it would assert a
/// type rather than withhold one, so the whole signature is declined.
#[test]
fn a_different_type_under_the_same_name_declines() {
    let types = factory();
    let shell = types.get_type_struct("option").expect("shell");
    let int4t = types.get_base(4, type_metatype::TYPE_INT).expect("int");
    let field = kuna_decomp::dtype::TypeField::new(0, 0, "mine", int4t);
    types
        .set_fields_struct_raw(&shell, vec![field], Vec::new(), 24, 8, 0)
        .expect("a 24-byte `option` that is not the platform's");
    assert!(named_aggregate("option", &types, 1, Layout::Opaque).is_err(), "declined, not adopted");
}

/// glibc spells `FILE` as `struct _IO_FILE`, so that is the tag a `-g` image
/// defines the layout under. The table adopts it rather than minting a second,
/// empty stream type beside the real one.
#[test]
fn the_platform_spelling_of_the_same_type_is_adopted() {
    let types = factory();
    let shell = types.get_type_struct("_IO_FILE").expect("shell");
    let int4t = types.get_base(4, type_metatype::TYPE_INT).expect("int");
    let field = kuna_decomp::dtype::TypeField::new(0, 0, "_flags", int4t);
    let complete = types
        .set_fields_struct_raw(&shell, vec![field], Vec::new(), 216, 8, 0)
        .expect("complete _IO_FILE");
    let got = named_aggregate("FILE", &types, 1, Layout::Opaque).expect("adopt the platform spelling");
    assert!(Rc::ptr_eq(&complete, &got), "`FILE` resolves to the held `_IO_FILE`");
    assert_eq!(got.get_name(), "_IO_FILE");
}

/// A DWARF forward declaration (`DW_AT_declaration`, no `DW_AT_byte_size`) is
/// interned at width 0. Pointing at it is the one thing a sized shell exists to
/// prevent — `RulePtrsubUndo` keeps the `PTRSUB` alive on a zero-width pointee
/// and the printer emits it in functional form — and completing it here would
/// re-key somebody else's type. So the signature is declined instead.
#[test]
fn a_zero_width_forward_declaration_declines() {
    let types = factory();
    types.get_type_struct("stat").expect("a width-0 forward declaration");
    assert!(named_aggregate("stat", &types, 1, Layout::Opaque).is_err(), "declined, not completed");
}

/// The adopt test is metatype + declared width, NOT completeness: a struct of
/// the right width that is still a shell is adopted exactly like a populated
/// one. That is what keeps the table IDEMPOTENT (slot 2 of a signature meets the
/// shell slot 1 minted) and it is what lets the DWARF importer finish populating
/// a partially-built `stat` underneath a pointer already handed out.
#[test]
fn an_incomplete_struct_of_the_declared_width_is_adopted() {
    let types = factory();
    let shell = types.get_type_struct("stat").expect("shell");
    let held = types
        .set_fields_struct_raw(&shell, Vec::new(), Vec::new(), 144, 8, flags::type_incomplete)
        .expect("a 144-byte shell somebody else is populating");
    assert!(held.is_incomplete(), "the fixture is the incomplete case");
    let got = named_aggregate("stat", &types, 1, Layout::Opaque).expect("adopted");
    assert!(Rc::ptr_eq(&held, &got), "the held shell is the answer, not a second definition");
}

/// Every `NamedPtr` in either table must name a row of `NAMED_AGGREGATES` —
/// otherwise `build_ty` has no width for it and the signature is dropped
/// silently.
#[test]
fn every_named_slot_has_a_width() {
    for (name, sig) in LIBC_NAMED
        .iter()
        .chain(LIBC_EXT_NAMED.iter())
        .chain(LIBC_DEFINED_NAMED.iter())
        .chain(LIBC_IMPORTED_OBSTACK.iter())
    {
        for t in std::iter::once(&sig.ret).chain(sig.params.iter()) {
            if let Ty::NamedPtr(n) = t {
                assert!(
                    NAMED_AGGREGATES.iter().any(|a| a.name == *n),
                    "{name}: no width is known for `{n}`"
                );
            }
        }
    }
}

/// The named tables restate the shipped ones: same arity, same vararg slot, and
/// every slot either identical or a `void *` that became a `NamedPtr`. A
/// retarget that changed the arity would shift every argument at the call site.
#[test]
fn a_retarget_only_names_a_void_pointer() {
    let shipped = |name: &str| {
        super::super::LIBC
            .iter()
            .chain(super::super::kuna_libcsigs::LIBC_EXT.iter())
            .find(|(n, _)| *n == name)
            .map(|(_, sig)| sig)
    };
    let same = |a: &Ty, b: &Ty| -> bool {
        match (a, b) {
            (Ty::NamedPtr(_), Ty::VoidPtr) => true,
            _ => std::mem::discriminant(a) == std::mem::discriminant(b),
        }
    };
    for (name, sig) in LIBC_NAMED.iter().chain(LIBC_EXT_NAMED.iter()) {
        let Some(base) = shipped(name) else {
            continue; // a name new to this table has nothing to restate
        };
        assert_eq!(sig.params.len(), base.params.len(), "{name}: arity moved");
        assert_eq!(sig.vararg, base.vararg, "{name}: vararg slot moved");
        assert!(same(&sig.ret, &base.ret), "{name}: the return type is not a naming of the shipped one");
        for (i, (new, old)) in sig.params.iter().zip(base.params.iter()).enumerate() {
            assert!(same(new, old), "{name}: p{i} is not a naming of the shipped slot");
        }
    }
}

/// The `va_list` trap: the last slot of the `v*` family is a `va_list`, and a
/// blanket retarget would call it a `FILE *` at every one of those call sites.
#[test]
fn the_va_list_family_is_never_retargeted() {
    for name in ["vasprintf", "vsnprintf", "__vasprintf_chk", "__vsnprintf_chk", "verr", "vwarn"] {
        assert!(
            !LIBC_EXT_NAMED.iter().any(|(n, _)| *n == name),
            "{name} takes a va_list, not an aggregate pointer"
        );
    }
    let (_, chk) = LIBC_EXT_NAMED
        .iter()
        .find(|(n, _)| *n == "__vfprintf_chk")
        .expect("__vfprintf_chk names its stream");
    assert!(matches!(chk.params[0], Ty::NamedPtr("FILE")), "p0 is the stream");
    assert!(matches!(chk.params[3], Ty::VoidPtr), "p3 is the va_list and stays void *");
}

/// `getdelim` takes the delimiter where `getline` takes nothing, so its stream
/// is the FOURTH slot. Pinned because the off-by-one is invisible in output.
#[test]
fn getdelim_names_its_fourth_slot() {
    let (_, sig) = LIBC_EXT_NAMED.iter().find(|(n, _)| *n == "getdelim").expect("getdelim");
    assert_eq!(sig.params.len(), 4, "char **, size_t *, int, FILE *");
    assert!(matches!(sig.params[2], Ty::Int), "the delimiter");
    assert!(matches!(sig.params[3], Ty::NamedPtr("FILE")), "the stream");
    let (_, line) = LIBC_EXT_NAMED.iter().find(|(n, _)| *n == "getline").expect("getline");
    assert_eq!(line.params.len(), 3);
    assert!(matches!(line.params[2], Ty::NamedPtr("FILE")));
}

/// `__uflow` is the type evidence a `-O2` reader loop has for its stream
/// argument, and only this table carries it — so `libctypes off` stays exactly
/// the shipped behavior.
#[test]
fn uflow_is_new_to_this_table() {
    let (_, sig) = LIBC_EXT_NAMED.iter().find(|(n, _)| *n == "__uflow").expect("__uflow");
    assert!(matches!(sig.ret, Ty::Int), "int __uflow(FILE *)");
    assert_eq!(sig.params.len(), 1);
    assert!(matches!(sig.params[0], Ty::NamedPtr("FILE")));
    assert!(
        !super::super::LIBC.iter().any(|(n, _)| *n == "__uflow")
            && !super::super::kuna_libcsigs::LIBC_EXT.iter().any(|(n, _)| *n == "__uflow"),
        "a name only this table carries"
    );
}

/// Both tables are duplicate-free (a duplicate would make which signature wins
/// depend on iteration order).
#[test]
fn the_tables_have_no_duplicate_names() {
    let mut across: Vec<&str> = Vec::new();
    for table in [LIBC_NAMED, LIBC_EXT_NAMED, LIBC_DEFINED_NAMED] {
        across.extend(table.iter().map(|(n, _)| *n));
        let mut names: Vec<&str> = table.iter().map(|(n, _)| *n).collect();
        let before = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), before, "duplicate name in a libctypes table");
    }
    // And across them: `declared_named_prototype` searches all three in order,
    // so one name in two tables would make the answer depend on that order.
    let before = across.len();
    across.sort_unstable();
    across.dedup();
    assert_eq!(across.len(), before, "a name is carried by two libctypes tables");
    let mut aggs: Vec<&str> = NAMED_AGGREGATES.iter().map(|a| a.name).collect();
    let before = aggs.len();
    aggs.sort_unstable();
    aggs.dedup();
    assert_eq!(aggs.len(), before, "duplicate aggregate name");
}

/// The two tables matched against an obstack entry point are reserved-namespace
/// names and nothing else. A plain spelling matched against a DEFINED name
/// would retype a function the image wrote itself.
#[test]
fn the_defined_table_is_obstack_entry_points_only() {
    for (name, sig) in LIBC_DEFINED_NAMED.iter().chain(LIBC_IMPORTED_OBSTACK.iter()) {
        assert!(
            name.starts_with("_obstack_"),
            "{name}: only the reserved obstack entry points may match a DEFINED name"
        );
        assert!(
            matches!(sig.params.first(), Some(Ty::NamedPtr("obstack"))),
            "{name}: the handle is the first slot"
        );
        assert!(
            !super::super::LIBC.iter().any(|(n, _)| n == name)
                && !super::super::kuna_libcsigs::LIBC_EXT.iter().any(|(n, _)| n == name),
            "{name}: a name only this table carries, so `libctypes off` is unchanged"
        );
    }
    assert_eq!(LIBC_DEFINED_NAMED.len(), 5, "`_obstack_allocated_p` has no installed declaration");
}

/// The two obstack tables are the SAME five names with the SAME shape, and
/// differ in exactly the slots the two published headers disagree about: the
/// sizes and the byte count. Anything else differing would mean one channel
/// silently lost a slot.
#[test]
fn the_two_obstack_tables_differ_only_in_the_size_slots() {
    let same = |a: &Ty, b: &Ty| match (a, b) {
        (Ty::NamedPtr(x), Ty::NamedPtr(y)) => x == y,
        _ => std::mem::discriminant(a) == std::mem::discriminant(b),
    };
    assert_eq!(LIBC_DEFINED_NAMED.len(), LIBC_IMPORTED_OBSTACK.len());
    for ((dname, dsig), (iname, isig)) in LIBC_DEFINED_NAMED.iter().zip(LIBC_IMPORTED_OBSTACK) {
        assert_eq!(dname, iname, "the two tables are keyed the same way");
        assert_eq!(dsig.params.len(), isig.params.len(), "{dname}: same arity");
        assert_eq!(dsig.vararg, isig.vararg);
        for (d, i) in dsig.params.iter().zip(isig.params) {
            assert!(
                matches!((d, i), (Ty::Size, Ty::Int)) || same(d, i),
                "{dname}: only a size slot may differ between the channels"
            );
        }
        assert!(
            matches!((&dsig.ret, &isig.ret), (Ty::Size, Ty::Int)) || same(&dsig.ret, &isig.ret),
            "{dname}: and the same for the return"
        );
    }
}

/// gnulib's copy of `obstack.c` defines `_OBSTACK_SIZE_T` as `size_t` and the
/// installed glibc header spells plain `int`
/// (`gcc -aux-info`: `extern void _obstack_newchunk (struct obstack *, int)`).
/// Which one a call obeys is decided by the CHANNEL — a linked-in definition or
/// an import — so each table carries its own header's spelling.
#[test]
fn each_obstack_channel_keeps_its_own_size_spelling() {
    let chunk = |t: &'static [(&str, Sig)]| {
        &t.iter().find(|(n, _)| *n == "_obstack_newchunk").expect("newchunk").1
    };
    let used = |t: &'static [(&str, Sig)]| {
        &t.iter().find(|(n, _)| *n == "_obstack_memory_used").expect("used").1
    };
    assert!(matches!(chunk(LIBC_DEFINED_NAMED).params[1], Ty::Size), "gnulib: size_t");
    assert!(matches!(used(LIBC_DEFINED_NAMED).ret, Ty::Size), "gnulib: size_t");
    assert!(matches!(chunk(LIBC_IMPORTED_OBSTACK).params[1], Ty::Int), "glibc: int");
    assert!(matches!(used(LIBC_IMPORTED_OBSTACK).ret, Ty::Int), "glibc: int");
}

/// The record readers of the three account databases all end their argument
/// list with the stream they read from or write to, and all carry the record
/// type the database is named for. An off-by-one here is invisible in output.
#[test]
fn the_account_database_slots_name_both_aggregates() {
    for (name, record) in [
        ("fgetpwent", "passwd"),
        ("fgetgrent", "group"),
        ("fgetspent", "spwd"),
    ] {
        let (_, sig) = LIBC_EXT_NAMED.iter().find(|(n, _)| *n == name).expect(name);
        assert!(matches!(sig.ret, Ty::NamedPtr(r) if r == record), "{name}: returns the record");
        assert_eq!(sig.params.len(), 1);
        assert!(matches!(sig.params[0], Ty::NamedPtr("FILE")), "{name}: reads a stream");
    }
    for (name, record) in [("putpwent", "passwd"), ("putgrent", "group"), ("putspent", "spwd")] {
        let (_, sig) = LIBC_EXT_NAMED.iter().find(|(n, _)| *n == name).expect(name);
        assert_eq!(sig.params.len(), 2);
        assert!(matches!(sig.params[0], Ty::NamedPtr(r) if r == record), "{name}: the record");
        assert!(matches!(sig.params[1], Ty::NamedPtr("FILE")), "{name}: then the stream");
    }
    // The reentrant lookups end in a `struct X **` result slot the vocabulary
    // cannot spell; the RECORD is the second slot, never the last.
    for (name, record) in [
        ("getpwnam_r", "passwd"),
        ("getgrnam_r", "group"),
        ("getspnam_r", "spwd"),
    ] {
        let (_, sig) = LIBC_EXT_NAMED.iter().find(|(n, _)| *n == name).expect(name);
        assert_eq!(sig.params.len(), 5);
        assert!(matches!(sig.params[1], Ty::NamedPtr(r) if r == record), "{name}: p1 is the record");
        assert!(matches!(sig.params[4], Ty::VoidPtr), "{name}: p4 is the result pointer");
    }
}

/// The gate decides whether a declared name is answered in its named form; with
/// it off `declared_libc_prototype` falls through to the `void *` tables.
#[test]
fn the_declared_lookup_follows_the_gate() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let types = factory();
    kuna_decomp::kuna_libctypes::set_libctypes_env_layout(kuna_decomp::kuna_libctypes::LibcTypesLayout::Off);
    let off = super::super::declared_libc_prototype("fopen", &types, 1, Layout::Opaque)
        .expect("fopen off");
    let off_base = off.outtype.as_ref().and_then(|t| t.get_ptr_to());
    assert_eq!(
        off_base.as_ref().map(|p| p.get_metatype()),
        Some(type_metatype::TYPE_VOID),
        "off: the shipped void * signature"
    );
    kuna_decomp::kuna_libctypes::set_libctypes_env_layout(kuna_decomp::kuna_libctypes::LibcTypesLayout::Opaque);
    let on = super::super::declared_libc_prototype("fopen", &types, 1, Layout::Opaque)
        .expect("fopen on");
    let on_base = on.outtype.as_ref().and_then(|t| t.get_ptr_to());
    assert_eq!(
        on_base.as_ref().map(|p| p.get_name().to_string()),
        Some("FILE".to_string()),
        "on: the named signature"
    );
    std::env::remove_var(kuna_decomp::kuna_libctypes::LIBCTYPES_ENV);
}

/// A declared name whose named form cannot be built degrades to the width-stable
/// signature instead of vanishing: withholding the prototype entirely would lose
/// the arity too, which is the part `--define-function 0x…=stat` is asked for.
#[test]
fn a_declined_aggregate_degrades_to_the_width_stable_signature() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let types = factory();
    kuna_decomp::kuna_libctypes::set_libctypes_env_layout(kuna_decomp::kuna_libctypes::LibcTypesLayout::Opaque);
    let shell = types.get_type_struct("stat").expect("shell");
    let int4t = types.get_base(4, type_metatype::TYPE_INT).expect("int");
    let field = kuna_decomp::dtype::TypeField::new(0, 0, "mine", int4t);
    types
        .set_fields_struct_raw(&shell, vec![field], Vec::new(), 24, 8, 0)
        .expect("a 24-byte `stat` of the program's own");
    assert!(named_aggregate("stat", &types, 1, Layout::Opaque).is_err(), "the fixture declines the named form");
    let pieces = super::super::declared_libc_prototype("stat", &types, 1, Layout::Opaque)
        .expect("still answered, from the void * tables");
    assert_eq!(pieces.intypes.len(), 2, "the shipped arity survives");
    let p1 = pieces.intypes[1].get_ptr_to().expect("a pointer slot");
    assert_eq!(p1.get_metatype(), type_metatype::TYPE_VOID, "degraded to void *");
    std::env::remove_var(kuna_decomp::kuna_libctypes::LIBCTYPES_ENV);
}

// -- the `glibc` field layouts ---------------------------------------------

/// Every published layout names an aggregate this table knows the width of, and
/// every field fits inside that width at an offset no other field claims. A
/// layout that overruns or overlaps is a transcription error, and the shells are
/// interned at `load file` where nothing would catch it.
#[test]
fn glibc_layouts_fit_their_aggregate() {
    let types = factory();
    for (name, rows) in glibc::GLIBC_LAYOUTS {
        let agg = NAMED_AGGREGATES
            .iter()
            .find(|a| a.name == *name)
            .unwrap_or_else(|| panic!("`{name}` has a layout but no width"));
        let fields = glibc::build_fields(rows, &types, 1).expect("build the layout");
        let mut end = 0;
        for (row, field) in rows.iter().zip(&fields) {
            assert!(
                row.off >= end,
                "{name}::{} at {:#x} overlaps the field before it",
                row.name,
                row.off
            );
            end = row.off + field.field_type.get_size();
            assert!(
                end <= agg.size,
                "{name}::{} runs past the {}-byte aggregate",
                row.name,
                agg.size
            );
            assert_eq!(field.offset, row.off, "the field offset is the declared one");
            assert_eq!(field.ident, row.off, "ident is the offset, as the DWARF importer does it");
        }
    }
}

/// The published layouts nest at most one level deep and no aggregate holds
/// itself, which is what makes the recursive mint in `named_aggregate`
/// terminate without a depth counter. `_IO_FILE::_chain` is the self-pointer
/// this rule costs us; it is `void *` on purpose.
#[test]
fn glibc_layouts_nest_at_most_one_level() {
    fn named(ty: &glibc::FTy) -> Option<&'static str> {
        match ty {
            glibc::FTy::Named(n) => Some(n),
            glibc::FTy::Arr(inner, _) => named(inner),
            _ => None,
        }
    }
    for (name, rows) in glibc::GLIBC_LAYOUTS {
        for row in rows.iter() {
            let Some(inner) = named(&row.ty) else { continue };
            assert_ne!(inner, *name, "{name}::{} holds its own aggregate", row.name);
            let nested = glibc::layout_for(inner)
                .unwrap_or_else(|| panic!("{name}::{} names `{inner}`, which has no layout", row.name));
            for deep in nested.iter() {
                assert!(
                    named(&deep.ty).is_none(),
                    "{name} -> {inner} -> {} is a second level of nesting",
                    deep.name
                );
            }
        }
    }
}

/// The three published offsets the option exists for, end to end through the
/// mint: the shell is complete, carries its real width, and answers at the
/// offsets a `getc_unlocked` body reads.
#[test]
fn glibc_mints_a_complete_file_with_its_published_fields() {
    let types = factory();
    let file = named_aggregate("FILE", &types, 1, Layout::Glibc).expect("mint FILE");
    assert_eq!(file.get_size(), 216, "the glibc x86-64 width");
    assert!(!file.is_incomplete(), "a struct with its members is not incomplete");
    for (off, want, size) in [(0, "_flags", 4), (8, "_IO_read_ptr", 8), (0x10, "_IO_read_end", 8), (0x70, "_fileno", 4)] {
        let field = (0..file.num_depend())
            .filter_map(|i| file.get_field(i))
            .find(|f| f.offset == off)
            .unwrap_or_else(|| panic!("no field at {off:#x}"));
        assert_eq!(field.name, want, "the field at {off:#x}");
        assert_eq!(field.field_type.get_size(), size, "{want} width");
    }
}

/// `stat::st_atim` is a `timespec` BY VALUE, so the nested aggregate has to be
/// minted first and be the same interned object the table would hand out on its
/// own -- two `timespec`s would print as two types and break `dependent_order`'s
/// definition-before-use walk.
#[test]
fn glibc_stat_nests_one_interned_timespec() {
    let types = factory();
    let stat = named_aggregate("stat", &types, 1, Layout::Glibc).expect("mint stat");
    assert_eq!(stat.get_size(), 144);
    let atim = (0..stat.num_depend())
        .filter_map(|i| stat.get_field(i))
        .find(|f| f.offset == 0x48)
        .expect("st_atim");
    assert_eq!(atim.name, "st_atim");
    assert_eq!(atim.field_type.get_size(), 16, "a timespec by value");
    let timespec = named_aggregate("timespec", &types, 1, Layout::Opaque).expect("find timespec");
    assert!(
        Rc::ptr_eq(&atim.field_type, &timespec),
        "the nested timespec is the interned one"
    );
    assert_eq!(timespec.get_field(0).expect("tv_sec").name, "tv_sec");
    assert_eq!(timespec.get_field(1).expect("tv_nsec").name, "tv_nsec");
}

/// Under `glibc` too, a definition somebody else established is ADOPTED, never
/// overwritten: the DWARF importer runs first, and completing a struct re-keys
/// it into a new `Rc` that the pointers already handed out would not follow.
#[test]
fn glibc_adopts_a_held_definition_instead_of_installing_over_it() {
    let types = factory();
    let shell = types.get_type_struct("stat").expect("shell");
    let int4t = types.get_base(4, type_metatype::TYPE_INT).expect("int");
    let field = kuna_decomp::dtype::TypeField::new(0, 0, "theirs", int4t);
    let theirs = types
        .set_fields_struct_raw(&shell, vec![field], Vec::new(), 144, 8, 0)
        .expect("somebody else's 144-byte `stat`");
    let got = named_aggregate("stat", &types, 1, Layout::Glibc).expect("adopt");
    assert!(Rc::ptr_eq(&got, &theirs), "the held definition is the answer");
    assert_eq!(
        got.get_field(0).expect("their field").name,
        "theirs",
        "no published layout was installed over it"
    );
}

/// An aggregate glibc publishes no usable layout for stays the sized, still
/// incomplete shell under `glibc` -- the value adds field names, it does not
/// invent them.
#[test]
fn glibc_leaves_the_layoutless_aggregates_opaque() {
    let types = factory();
    for name in ["DIR", "termios", "sigset_t", "pthread_mutex_t"] {
        assert!(glibc::layout_for(name).is_none(), "`{name}` must have no layout");
        let ct = named_aggregate(name, &types, 1, Layout::Glibc).expect("mint");
        assert!(ct.is_incomplete(), "`{name}` stays an opaque shell");
        assert_eq!(ct.num_depend(), 0, "`{name}` has no members");
    }
}

/// The layouts are glibc's and x86-64's, so the pass installs them only where
/// both are true. An ARM ELF and a PE are refused on the architecture and the
/// format; the x86-64 ELF that the stage test decompiles is accepted on its
/// `.dynstr`.
#[test]
fn only_a_glibc_x86_64_elf_takes_the_layouts() {
    for (name, want) in [
        ("libctypes_glibc_x86_64", true),
        ("libctypes_stat_x86_64", true),
        ("armlibcmain_le32", false),
        ("win32sigs_pe_i386.exe", false),
        // Two glibc images the layouts are NOT true of: `.dynstr` names
        // `libc.so.6` and a `GLIBC_2.x` version in both, and every pointer in
        // them is 4 bytes. The second carries debug info defining `stat` and
        // `timespec`, which is what the ARCHITECTURE half has to refuse on its
        // own.
        ("mips_gp_le32", false),
        ("libctypes_mips32_glibc_le32", false),
    ] {
        let path = format!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/{}"), name);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {name}: {e}"));
        let file = object::File::parse(bytes.as_slice()).unwrap_or_else(|e| panic!("parse {name}: {e}"));
        assert_eq!(
            glibc::target_is_glibc_x86_64(&file),
            want,
            "`{name}` should {} the published glibc x86-64 layouts",
            if want { "take" } else { "refuse" }
        );
    }
}

/// No table carries a member glibc reserves for itself. A reserved name is only
/// ever truthful in a whole-struct copy a reader gains nothing from, and is a
/// confident lie about the enclosing struct's real member whenever a named
/// pointee lands on the first member of something bigger -- so those offsets
/// stay holes and print in the neutral offset form.
#[test]
fn no_layout_names_a_reserved_member() {
    for (name, rows) in glibc::GLIBC_LAYOUTS {
        for row in rows.iter() {
            assert!(
                !(row.name.starts_with("__pad")
                    || row.name.starts_with("__glibc_reserved")
                    || row.name.starts_with("_unused")),
                "{name}::{} is reserved for the implementation and must stay a hole",
                row.name
            );
        }
    }
    let types = factory();
    let stat = named_aggregate("stat", &types, 1, Layout::Glibc).expect("mint stat");
    assert_eq!(stat.get_size(), 144, "the holes do not shrink the aggregate");
    for off in [0x24, 0x78, 0x80, 0x88] {
        assert!(
            (0..stat.num_depend()).filter_map(|i| stat.get_field(i)).all(|f| f.offset != off),
            "stat has no member at {off:#x}"
        );
    }
}

/// `declared_libc_prototype` runs with the object file out of reach, so it may
/// NOT decide for itself whether the glibc layouts are true of this image — it
/// is told, by the console, which passes on what the load-time pass decided
/// (`AnalysisOutput::libctypes_glibc`).
///
/// The fixture here is the shape that used to defeat the old program-read
/// heuristic and is the whole reason this is a parameter: a factory already
/// holding a field-filled `stat` whose first member is `st_dev` at offset 0.
/// That is what a MIPS32 or musl image's OWN debug info looks like from inside
/// this call — `st_dev` sits at offset 0 of every one of them — and it must not
/// buy a 216-byte x86-64 `FILE`.
#[test]
fn a_refused_target_cannot_reach_the_glibc_layouts() {
    use kuna_decomp::kuna_libctypes::{set_libctypes_env_layout, LibcTypesLayout, LIBCTYPES_ENV};
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    // The run asked for `glibc` and the program looks the part. The load-time
    // target gate still refused, so `Opaque` is what arrives here, and `Opaque`
    // is what is minted.
    set_libctypes_env_layout(LibcTypesLayout::Glibc);
    let refused = factory();
    named_aggregate("stat", &refused, 1, Layout::Glibc).expect("a laid-out `stat`");
    let shelled = super::super::declared_libc_prototype("fopen", &refused, 1, Layout::Opaque)
        .expect("fopen");
    let shell = shelled.outtype.as_ref().and_then(|t| t.get_ptr_to()).expect("FILE *");
    assert_eq!(shell.get_name(), "FILE", "the name is still the table's");
    assert!(
        shell.is_incomplete(),
        "a refused target gets the fieldless shell, whatever the program holds"
    );
    assert_eq!(shell.num_depend(), 0, "and no fields to read an offset under");

    // The gate passed: the same call mints the published layout.
    let accepted = factory();
    let pieces = super::super::declared_libc_prototype("fopen", &accepted, 1, Layout::Glibc)
        .expect("fopen");
    let file = pieces.outtype.as_ref().and_then(|t| t.get_ptr_to()).expect("FILE *");
    assert_eq!(file.get_name(), "FILE");
    assert!(!file.is_incomplete(), "an accepted target gets the fields");
    assert_eq!(file.get_size(), 216, "the published x86-64 width");

    // The option value alone can no longer reach the layouts from here: it is
    // necessary (the pass reads it before the target gate) and not sufficient.
    set_libctypes_env_layout(LibcTypesLayout::Opaque);
    let still = super::super::declared_libc_prototype("fopen", &factory(), 1, Layout::Opaque)
        .expect("fopen");
    let shell = still.outtype.as_ref().and_then(|t| t.get_ptr_to()).expect("FILE *");
    assert!(shell.is_incomplete(), "`opaque` mints the fieldless shell");
    std::env::remove_var(LIBCTYPES_ENV);
}

/// Read one of the crate's binary fixtures.
fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// An executable binds a stream with a copy relocation, and the `.bss` word the
/// relocation names holds libc's own `FILE *stdout`. So the slot is typed
/// `FILE *` and carries the stream's own name.
#[test]
fn a_copy_relocated_stream_slot_is_the_stream_pointer() {
    let bytes = fixture_bytes("libctypes_streams_x86_64");
    let file = object::File::parse(&bytes[..]).expect("parse the exe fixture");
    let types = factory();
    let facts = streams::stream_data_symbols(&file, &types, 1, Layout::Opaque);

    let mut got: Vec<(String, u64)> =
        facts.iter().map(|f| (f.name.clone(), f.addr)).collect();
    got.sort();
    assert_eq!(
        got,
        vec![("stdin".to_string(), 0x4020), ("stdout".to_string(), 0x4010)],
        "the two copy-relocated slots, under the stream's own name"
    );
    for f in &facts {
        let pointee = f.type_.get_ptr_to().expect("FILE *");
        assert_eq!(pointee.get_name(), "FILE", "{}: one indirection to the stream", f.name);
    }
}

/// A shared object cannot copy-relocate, so its GOT word holds the stream's
/// ADDRESS, not the stream pointer. Both halves of that have to show: one more
/// indirection in the type, and a name that does not claim to be the stream.
#[test]
fn a_shared_object_names_the_got_slot_as_a_pointer_to_the_stream() {
    let bytes = fixture_bytes("libctypes_streams_so_x86_64.so");
    let file = object::File::parse(&bytes[..]).expect("parse the .so fixture");
    let types = factory();
    let facts = streams::stream_data_symbols(&file, &types, 1, Layout::Opaque);

    let mut got: Vec<(String, u64)> =
        facts.iter().map(|f| (f.name.clone(), f.addr)).collect();
    got.sort();
    assert_eq!(
        got,
        vec![("stdin_ptr".to_string(), 0x3fe0), ("stdout_ptr".to_string(), 0x3fd8)],
        "the GOT slots, named for what they hold"
    );
    for f in &facts {
        let once = f.type_.get_ptr_to().expect("FILE **");
        let twice = once.get_ptr_to().expect("the stream behind it");
        assert_eq!(twice.get_name(), "FILE", "{}: two indirections", f.name);
    }
}

/// The two shapes never cross: an executable's copy slot is not a `_ptr` and a
/// library's GOT slot is never handed the stream's bare name. Getting this wrong
/// is a silent double indirection in the emitted C.
#[test]
fn the_two_stream_shapes_do_not_borrow_each_others_names() {
    let types = factory();
    for (fixture, want_ptr) in
        [("libctypes_streams_x86_64", false), ("libctypes_streams_so_x86_64.so", true)]
    {
        let bytes = fixture_bytes(fixture);
        let file = object::File::parse(&bytes[..]).expect("parse");
        for f in streams::stream_data_symbols(&file, &types, 1, Layout::Opaque) {
            assert_eq!(f.name.ends_with("_ptr"), want_ptr, "{fixture}: {}", f.name);
        }
    }
}

/// `<stream>_ptr` is a name kuna invents, so an image that already answers to it
/// takes it: the GOT slot keeps its address-shaped rendering rather than
/// borrowing an identifier that also names a `.data` word somewhere else. The
/// colliding global here is `static`, so only `.symtab` carries it — a lookup
/// that read `.dynsym` alone would miss exactly this case. `stdin` is untouched,
/// which is the other half: the decline is per name, not a bail.
#[test]
fn a_minted_stream_name_the_image_already_owns_is_declined() {
    use object::{Object, ObjectSymbol};
    let bytes = fixture_bytes("libctypes_streams_collide_so_x86_64.so");
    let file = object::File::parse(&bytes[..]).expect("parse the colliding .so fixture");
    let types = factory();
    let facts = streams::stream_data_symbols(&file, &types, 1, Layout::Opaque);

    let got: Vec<(String, u64)> = facts.iter().map(|f| (f.name.clone(), f.addr)).collect();
    assert_eq!(
        got,
        vec![("stdin_ptr".to_string(), 0x3fe0)],
        "stdout's GOT slot says nothing; stdin's is unaffected"
    );
    assert!(
        object::File::parse(&bytes[..])
            .expect("parse")
            .symbols()
            .any(|s| s.name() == Ok("stdout_ptr")),
        "the fixture really does own the minted spelling"
    );
}

/// The stream slots point at the same interned `FILE` the prototype table's own
/// `fclose(FILE *)` does. A second definition would give the binary two stream
/// types and make the body cast between them.
#[test]
fn a_stream_slot_shares_the_tables_own_file_type() {
    let bytes = fixture_bytes("libctypes_streams_x86_64");
    let file = object::File::parse(&bytes[..]).expect("parse");
    let types = factory();
    let table = named_aggregate("FILE", &types, 1, Layout::Opaque).expect("mint FILE");
    let facts = streams::stream_data_symbols(&file, &types, 1, Layout::Opaque);
    assert!(!facts.is_empty());
    for f in &facts {
        let pointee = f.type_.get_ptr_to().expect("FILE *");
        assert!(Rc::ptr_eq(&pointee, &table), "{}: one FILE, not two", f.name);
    }
}

/// `glibc` reaches the stream slots too: the pointee is the published layout,
/// which is what turns `stdout->field_0x28` into `stdout->_IO_write_ptr`.
#[test]
fn the_glibc_layout_reaches_a_stream_slot() {
    let bytes = fixture_bytes("libctypes_streams_x86_64");
    let file = object::File::parse(&bytes[..]).expect("parse");
    let types = factory();
    let facts = streams::stream_data_symbols(&file, &types, 1, Layout::Glibc);
    let pointee = facts[0].type_.get_ptr_to().expect("FILE *");
    assert!(!pointee.is_incomplete(), "the published layout, not the shell");
    assert!(pointee.num_depend() > 0, "with its members");
}

/// A relocatable object names a different address space (the `relocobjects`
/// layout path rebases it), and a non-ELF image has no dynamic relocation to
/// read. Both yield nothing rather than a fact at a made-up address.
#[test]
fn a_relocatable_or_non_elf_image_yields_no_stream_symbol() {
    let types = factory();
    for fixture in ["arm_thumb_le32.o", "armv4t_thumb_pe.exe"] {
        let bytes = fixture_bytes(fixture);
        let file = object::File::parse(&bytes[..]).expect("parse");
        assert!(
            streams::stream_data_symbols(&file, &types, 1, Layout::Opaque).is_empty(),
            "{fixture}: nothing to say"
        );
    }
}

/// Two kinds of slot stay out because naming them splits the caller's frame
/// object: a two-element time array (one element named shrinks the object to
/// it) and a `sigset_t` a caller hands as `&sa.sa_mask` of a `struct sigaction`.
#[test]
fn the_frame_splitting_slots_stay_out() {
    for name in [
        "utimensat", "futimens", "utimes", "futimesat", "sigfillset", "sigsuspend",
        "pthread_sigmask", "sigdelset", "sigismember", "sigwait",
    ] {
        assert!(
            !LIBC_NAMED.iter().chain(LIBC_EXT_NAMED.iter()).any(|(n, _)| *n == name),
            "{name}: naming this slot splits the caller's frame object"
        );
    }
}

/// The aggregate WIDTHS are glibc's x86-64 ones, and a width larger than the real
/// one folds a caller's neighbouring locals into the struct it hands over. So no
/// slot is named on any other ABI: i386 (the stage test's witness), ARM, AArch64,
/// MIPS, an x86-64 PE and an x86-64 Mach-O are refused. An x86-64 ELF with no
/// `.dynstr` (a relocatable object) says nothing against glibc and is accepted.
#[test]
fn only_an_x86_64_glibc_abi_takes_the_widths() {
    for (name, want) in [
        ("libctypes_glibc_x86_64", true),
        ("libctypes_stat_x86_64", true),
        ("libctypes_container_x86_64", true),
        ("fid_lib_x86_64.o", true),
        ("libctypes_widths_i386", false),
        ("i386_pie_nl", false),
        ("armlibcmain_le32", false),
        ("et_rel_status_aarch64.o", false),
        ("libctypes_mips32_glibc_le32", false),
        ("win32sigs_pe_i386.exe", false),
        ("crtmain_x86_64.exe", false),
        ("macho_imports", false),
    ] {
        let bytes = fixture_bytes(name);
        let file = object::File::parse(bytes.as_slice()).unwrap_or_else(|e| panic!("parse {name}: {e}"));
        assert_eq!(
            glibc::target_takes_the_widths(&file),
            want,
            "`{name}` should {} the glibc x86-64 aggregate widths",
            if want { "take" } else { "refuse" }
        );
    }
}

/// The pass and `formatstring static` share one decision, and on a refused target
/// it is `Off` whatever the option asked for: the i386 witness gets the shipped
/// `void *` tables, and so does a name the operator declares there.
#[test]
fn a_refused_target_is_off_whatever_was_asked() {
    use kuna_decomp::kuna_libctypes::{set_libctypes_env_layout, LibcTypesLayout, LIBCTYPES_ENV};
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    for asked in [LibcTypesLayout::Opaque, LibcTypesLayout::Glibc] {
        set_libctypes_env_layout(asked);
        let i386 = fixture_bytes("libctypes_widths_i386");
        let file = object::File::parse(i386.as_slice()).expect("parse the i386 witness");
        assert_eq!(super::super::effective_libctypes_layout(&file), Layout::Off, "{asked:?} on i386");
        let x86_64 = fixture_bytes("libctypes_container_x86_64");
        let file = object::File::parse(x86_64.as_slice()).expect("parse the x86-64 fixture");
        assert_ne!(super::super::effective_libctypes_layout(&file), Layout::Off, "{asked:?} on x86-64");
    }
    std::env::remove_var(LIBCTYPES_ENV);
    let types = factory();
    let pieces = super::super::declared_libc_prototype("fopen", &types, 1, Layout::Off).expect("fopen");
    let pointee = pieces.outtype.as_ref().and_then(|t| t.get_ptr_to()).expect("a pointer result");
    assert_eq!(pointee.get_metatype(), kuna_decomp::dtype::type_metatype::TYPE_VOID, "`void *`, not `FILE *`");
    assert!(types.find_by_name("FILE").expect("lookup").is_none(), "and no shell is minted");
}

/// The P5 fit rule (`kuna_decomp::kuna_libcfit`) recognises this table's
/// aggregates by name, from a list kept beside the option's gate; it must name
/// exactly what this table can mint or adopt.
#[test]
fn the_fit_rule_knows_every_aggregate_this_table_names() {
    let mut ours: Vec<&str> = NAMED_AGGREGATES
        .iter()
        .flat_map(|a| std::iter::once(a.name).chain(a.dwarf_alias))
        .collect();
    let mut theirs = kuna_decomp::kuna_libctypes::AGGREGATE_NAMES.to_vec();
    ours.sort_unstable();
    theirs.sort_unstable();
    assert_eq!(ours, theirs);
}
