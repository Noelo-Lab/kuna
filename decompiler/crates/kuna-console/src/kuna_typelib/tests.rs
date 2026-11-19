//! Tests for the prebuilt type library reader ([`super`]).

use kuna_decomp::dtype::{type_metatype as meta, TypeFactory, TypeFactoryImpl};

use super::{Library, Reader, BUILTIN};
use crate::grammar::DataOrg;

const ORG: DataOrg = DataOrg { addr_size: 8, word_size: 1 };

/// An x86-64 SysV type factory, wired the way an architecture wires it after
/// loading its compiler spec.
fn factory() -> TypeFactoryImpl {
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
    use kuna_decomp::fspec::ProtoModel;
    use std::rc::Rc;
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(16);
    for (name, size, kind, chr) in [
        ("void", 0, meta::TYPE_VOID, false),
        ("char", 1, meta::TYPE_INT, true),
        ("bool", 1, meta::TYPE_BOOL, false),
        ("uint1", 1, meta::TYPE_UINT, false),
        ("int1", 1, meta::TYPE_INT, false),
        ("int2", 2, meta::TYPE_INT, false),
        ("uint2", 2, meta::TYPE_UINT, false),
        ("int4", 4, meta::TYPE_INT, false),
        ("uint4", 4, meta::TYPE_UINT, false),
        ("int8", 8, meta::TYPE_INT, false),
        ("uint8", 8, meta::TYPE_UINT, false),
        ("float4", 4, meta::TYPE_FLOAT, false),
        ("float8", 8, meta::TYPE_FLOAT, false),
        ("float10", 10, meta::TYPE_FLOAT, false),
        ("wchar4", 4, meta::TYPE_INT, true),
    ] {
        f.set_core_type(name, size, kind, chr).unwrap();
    }
    f.cache_core_types().unwrap();
    f.set_size_of_char(1);
    f.set_size_of_short(2);
    f.set_size_of_int(4);
    f.set_size_of_long(8);
    f.set_size_of_long_long(8);
    f.set_size_of_wchar(4);
    f.set_size_of_float(4);
    f.set_size_of_double(8);
    f.set_size_of_long_double(10);
    let mut mgr = AddrSpaceManager::new();
    mgr.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    let ram =
        AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 1, addrspace_flags::hasphysical, 1, 1);
    mgr.insert_space(Rc::new(ram)).unwrap();
    mgr.set_default_code_space(1).unwrap();
    mgr.set_default_data_space(1).unwrap();
    let mgr = Rc::new(mgr);
    let mut model = ProtoModel::new(&mgr);
    model.set_name("__cdecl");
    model.build_param_list("standard").unwrap();
    let model = Rc::new(model);
    let mut models = std::collections::BTreeMap::new();
    models.insert("__cdecl".to_string(), Rc::clone(&model));
    f.set_proto_context(Rc::clone(&mgr));
    f.set_proto_models(Some(model), models);
    f
}

/// The members of a record as `(offset, name)`.
fn fields(ty: &kuna_decomp::dtype::Datatype) -> Vec<(i32, String)> {
    (0..ty.num_depend()).filter_map(|i| ty.get_field(i)).map(|f| (f.offset, f.name.clone())).collect()
}

fn builtin() -> Library<'static> {
    Library::parse(BUILTIN[0].1).unwrap()
}

const SMALL: &str = "kuna-typelib 1\ntarget linux x86_64\nsource test 1\n\
T nfds_t *\tunsigned long\n\
T handle_t m=unsigned long\tstruct handle *\n\
S pollfd * 8 4\n 0 fd\tint\n 4 events\tshort\n 6 revents\tshort\n\
S node * 16 8\n 0 next\tstruct node *\n 8 owner\tstruct tree *\n\
S tree * 8 8\n 0 root\tstruct node *\n\
S packed * 12 1\n 0 events\tunsigned int\n 4 data\tunsigned long\n\
S private m=24,8 16 8\n 0 a\tlong\n 8 b\tlong\n\
U value * 8 8\n 0 ptr\tvoid *\n 0 fd\tint\n\
F poll *\tint poll(struct pollfd *, nfds_t, int)\n\
F walk *\tint walk(struct node *, int (*)(const struct node *, void *), void *)\n\
F only_here m\tint only_here(int)\n\
F opaque *\tint opaque(struct never_defined *, struct private *, handle_t)\n\
F uses_packed *\tint uses_packed(struct packed *, union value *)\n\
F private *\tint private(int)\n\
F bad *\tint bad(struct pollfd *, no_such_type)\n";

#[test]
fn a_record_takes_the_offsets_the_library_states() {
    let lib = Library::parse(SMALL).unwrap();
    assert_eq!((lib.target, lib.source, lib.counts()), ("linux x86_64", "test 1", (7, 8)));
    let f = factory();
    let mut reader = Reader::new(&lib, false, &f, ORG);
    let poll = reader.prototype("poll").unwrap();
    assert_eq!(poll.intypes.len(), 3);
    assert_eq!(poll.intypes[1].get_name(), "nfds_t");
    let pollfd = f.find_by_name("pollfd").unwrap().unwrap();
    assert_eq!((pollfd.get_size(), pollfd.is_incomplete()), (8, false));
    assert_eq!(fields(&pollfd), [(0, "fd".into()), (4, "events".into()), (6, "revents".into())]);
    assert!(f.find_by_name("node").unwrap().is_none(), "only what a prototype names is interned");
    assert!(reader.prototype("walk").is_some());
    let node = f.find_by_name("node").unwrap().unwrap();
    let tree = f.find_by_name("tree").unwrap().unwrap();
    assert_eq!((node.get_size(), tree.get_size()), (16, 8));
    assert_eq!(fields(&node).len(), 2);
}

#[test]
fn a_packed_record_keeps_its_measured_layout() {
    let lib = Library::parse(SMALL).unwrap();
    let f = factory();
    assert!(Reader::new(&lib, false, &f, ORG).prototype("uses_packed").is_some());
    let packed = f.find_by_name("packed").unwrap().unwrap();
    assert_eq!(packed.get_size(), 12);
    assert_eq!(fields(&packed)[1].0, 4);
    let value = f.find_by_name("value").unwrap().unwrap();
    assert_eq!((value.get_metatype(), value.get_size()), (meta::TYPE_UNION, 8));
}

#[test]
fn the_other_c_library_is_offered_only_what_it_agrees_on() {
    let lib = Library::parse(SMALL).unwrap();
    let f = factory();
    let mut source = Reader::new(&lib, false, &f, ORG);
    assert!(source.prototype("only_here").is_some());
    assert!(source.prototype("opaque").is_some());
    assert_eq!(fields(&f.find_by_name("private").unwrap().unwrap()).len(), 2);
    assert_eq!(f.find_by_name("never_defined").unwrap().unwrap().get_size(), 1);

    let f = factory();
    let mut other = Reader::new(&lib, true, &f, ORG);
    assert!(other.prototype("only_here").is_none());
    assert!(other.prototype("private").is_some(), "a function's own name is not a type");
    let opaque = other.prototype("opaque").unwrap();
    let private = f.find_by_name("private").unwrap().unwrap();
    assert_eq!((private.get_size(), private.is_incomplete()), (24, true));
    assert_eq!(opaque.intypes[2].get_size(), 8);
    assert_eq!(opaque.intypes[2].get_metatype(), meta::TYPE_UINT);
}

#[test]
fn a_held_name_is_never_redefined_and_a_bad_entry_is_only_itself() {
    let lib = Library::parse(SMALL).unwrap();
    let f = factory();
    let own = f.get_type_struct("pollfd").unwrap();
    let mut reader = Reader::new(&lib, false, &f, ORG);
    assert!(reader.prototype("bad").is_none());
    assert!(reader.prototype("missing").is_none());
    assert!(reader.prototype("poll").is_some());
    assert!(std::rc::Rc::ptr_eq(&own, &f.find_by_name("pollfd").unwrap().unwrap()));
}

#[test]
fn malformed_libraries_are_refused_with_their_line() {
    for (text, want) in [
        ("", "kuna-typelib 1"),
        ("kuna-typelib 2\ntarget a\nsource b\n", "kuna-typelib 1"),
        ("kuna-typelib 1\nsource b\n", "target"),
        ("kuna-typelib 1\ntarget a\nsource b\nX name *\tint\n", "line 4: unknown entry kind"),
        ("kuna-typelib 1\ntarget a\nsource b\nT name\n", "line 4"),
        ("kuna-typelib 1\ntarget a\nsource b\nT name ?\tint\n", "line 4: unknown library field"),
        ("kuna-typelib 1\ntarget a\nsource b\n 0 fd\tint\n", "line 4: a member outside a record"),
        ("kuna-typelib 1\ntarget a\nsource b\nS r * 99999999999 8\n", "line 4: a record size out of range"),
        ("kuna-typelib 1\ntarget a\nsource b\nS r * 8 8\n -4 fd\tint\n", "line 5: a member offset out of range"),
    ] {
        let err = Library::parse(text).unwrap_err();
        assert!(err.contains(want), "{text:?}: {err}");
    }
    let long = format!("kuna-typelib 1\ntarget a\nsource b\nT t *\t{}\n", "x".repeat(5000));
    assert!(Library::parse(&long).unwrap_err().contains("line 4: longer"));
}

#[test]
fn members_that_do_not_fit_are_holes_and_cycles_end() {
    let text = "kuna-typelib 1\ntarget a\nsource b\n\
S r * 8 4\n 0 a\tint\n 2 overlap\tint\n 6 past\tint\n 4 b\tint\n\
T loop_a *\tloop_b\nT loop_b *\tloop_a\n\
S self * 8 8\n 0 me\tstruct self\n\
F f *\tint f(struct r *, struct self *)\nF g *\tint g(loop_a)\n";
    let lib = Library::parse(text).unwrap();
    let f = factory();
    let mut reader = Reader::new(&lib, false, &f, ORG);
    assert!(reader.prototype("f").is_some());
    let r = f.find_by_name("r").unwrap().unwrap();
    assert_eq!(fields(&r), [(0, "a".into()), (4, "b".into())]);
    assert_eq!(f.find_by_name("self").unwrap().unwrap().get_size(), 8);
    assert!(reader.prototype("g").is_none());
}

/// Every entry of the built-in library is read by the grammar, for both C
/// libraries of the target, and every record lands at its stated size.
#[test]
fn the_builtin_library_reads_whole() {
    let lib = builtin();
    assert_eq!(lib.target, "linux x86_64");
    for other in [false, true] {
        let f = factory();
        let mut reader = Reader::new(&lib, other, &f, ORG);
        let mut unread = Vec::new();
        for (name, (held, text)) in &lib.functions {
            let offered = !other || *held == super::Other::Same;
            if offered && !text.contains("long double") && reader.prototype(name).is_none() {
                unread.push(*name);
            }
        }
        assert_eq!(unread, [] as [&str; 0], "other={other}");
        for (name, (_, decl)) in &lib.types {
            let super::Decl::Record { size, .. } = decl else { continue };
            if let Some(ty) = f.find_by_name(name).unwrap() {
                if !ty.is_incomplete() {
                    assert_eq!(ty.get_size(), *size, "{name}");
                }
            }
        }
    }
}
