//! Unit tests for the named libc aggregate table.

use super::*;
use kuna_decomp::dtype::TypeFactoryImpl;

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
    let first = named_aggregate("FILE", &types).expect("mint FILE");
    let second = named_aggregate("FILE", &types).expect("re-find FILE");
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
        let ct = named_aggregate(agg.name, &types).expect("mint");
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
    let got = named_aggregate("stat", &types).expect("find the held stat");
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
    assert!(named_aggregate("option", &types).is_err(), "declined, not adopted");
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
    let types = factory();
    kuna_decomp::kuna_libctypes::set_libctypes_env(false);
    let off = super::super::declared_libc_prototype("fopen", &types, 1).expect("fopen off");
    let off_base = off.outtype.as_ref().and_then(|t| t.get_ptr_to());
    assert_eq!(
        off_base.as_ref().map(|p| p.get_metatype()),
        Some(type_metatype::TYPE_VOID),
        "off: the shipped void * signature"
    );
    kuna_decomp::kuna_libctypes::set_libctypes_env(true);
    let on = super::super::declared_libc_prototype("fopen", &types, 1).expect("fopen on");
    let on_base = on.outtype.as_ref().and_then(|t| t.get_ptr_to());
    assert_eq!(
        on_base.as_ref().map(|p| p.get_name().to_string()),
        Some("FILE".to_string()),
        "on: the named signature"
    );
    std::env::remove_var(kuna_decomp::kuna_libctypes::LIBCTYPES_ENV);
}
