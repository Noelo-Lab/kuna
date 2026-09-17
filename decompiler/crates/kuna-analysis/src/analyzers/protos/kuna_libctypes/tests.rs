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
    for (name, sig) in LIBC_NAMED.iter().chain(LIBC_EXT_NAMED.iter()) {
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
    for table in [LIBC_NAMED, LIBC_EXT_NAMED] {
        let mut names: Vec<&str> = table.iter().map(|(n, _)| *n).collect();
        let before = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), before, "duplicate name in a libctypes table");
    }
    let mut aggs: Vec<&str> = NAMED_AGGREGATES.iter().map(|a| a.name).collect();
    let before = aggs.len();
    aggs.sort_unstable();
    aggs.dedup();
    assert_eq!(aggs.len(), before, "duplicate aggregate name");
}

/// The gate decides whether a declared name is answered in its named form; with
/// it off `declared_libc_prototype` falls through to the `void *` tables.
#[test]
fn the_declared_lookup_follows_the_gate() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let types = factory();
    kuna_decomp::kuna_libctypes::set_libctypes_env_layout(kuna_decomp::kuna_libctypes::LibcTypesLayout::Off);
    let off = super::super::declared_libc_prototype("fopen", &types, 1).expect("fopen off");
    let off_base = off.outtype.as_ref().and_then(|t| t.get_ptr_to());
    assert_eq!(
        off_base.as_ref().map(|p| p.get_metatype()),
        Some(type_metatype::TYPE_VOID),
        "off: the shipped void * signature"
    );
    kuna_decomp::kuna_libctypes::set_libctypes_env_layout(kuna_decomp::kuna_libctypes::LibcTypesLayout::Opaque);
    let on = super::super::declared_libc_prototype("fopen", &types, 1).expect("fopen on");
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
    let pieces = super::super::declared_libc_prototype("stat", &types, 1)
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

/// `declared_libc_prototype` runs with no object file, so it reads the layout off
/// the OPTION and off the program: the value has to be `glibc` and the program
/// has to carry a field-filled aggregate, which is what proves the target gate
/// passed here. Either half missing means `Opaque`.
#[test]
fn the_live_layout_is_read_off_the_program() {
    use kuna_decomp::kuna_libctypes::{set_libctypes_env_layout, LibcTypesLayout, LIBCTYPES_ENV};
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    set_libctypes_env_layout(LibcTypesLayout::Glibc);
    let empty = factory();
    assert_eq!(live_layout(&empty), Layout::Opaque, "nothing held: no evidence");

    let opaque = factory();
    named_aggregate("FILE", &opaque, 1, Layout::Opaque).expect("opaque FILE");
    assert_eq!(live_layout(&opaque), Layout::Opaque, "a fieldless shell is not evidence");

    let glibc_types = factory();
    named_aggregate("stat", &glibc_types, 1, Layout::Glibc).expect("glibc stat");
    assert_eq!(live_layout(&glibc_types), Layout::Glibc, "a field-filled `stat` is");

    // The option half. A run that asked for `opaque` gets `opaque` even where the
    // factory holds a laid-out aggregate -- which is what a musl image carrying
    // its own DWARF `stat` looks like from here.
    set_libctypes_env_layout(LibcTypesLayout::Opaque);
    assert_eq!(live_layout(&glibc_types), Layout::Opaque, "the value the run asked for wins");
    let shelled = super::super::declared_libc_prototype("fopen", &glibc_types, 1).expect("fopen");
    let shell = shelled.outtype.as_ref().and_then(|t| t.get_ptr_to()).expect("FILE *");
    assert!(shell.is_incomplete(), "`opaque` mints the fieldless shell");

    // The consequence of both halves holding: a name the image never imported,
    // declared by hand, is minted at the layout the program already carries.
    set_libctypes_env_layout(LibcTypesLayout::Glibc);
    let types = factory();
    named_aggregate("stat", &types, 1, Layout::Glibc).expect("glibc stat");
    let pieces = super::super::declared_libc_prototype("fopen", &types, 1).expect("fopen");
    let file = pieces.outtype.as_ref().and_then(|t| t.get_ptr_to()).expect("FILE *");
    assert_eq!(file.get_name(), "FILE");
    assert!(!file.is_incomplete(), "the declared `FILE` agrees with the program's `stat`");
    std::env::remove_var(LIBCTYPES_ENV);
}
