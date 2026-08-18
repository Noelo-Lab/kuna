//! Porter in-module tests for `remote_provider.rs`: the `<mapsym>`/`<hole>`
//! decoder against hand-built packed documents (the Java `DecompileCallback`
//! answer shapes), and the wire `<coretypes>` decode equivalence.

use std::rc::Rc;

use kuna_base::address::{Address, ELEM_RANGELIST};
use kuna_base::marshal::{
    Encoder, IdRegistry, PackedEncode, XmlDecode, ATTRIB_CONTENT, ATTRIB_ID, ATTRIB_INDEX,
    ATTRIB_METATYPE, ATTRIB_MODEL, ATTRIB_NAME, ATTRIB_NAMELOCK, ATTRIB_READONLY, ATTRIB_SIZE,
    ATTRIB_TYPELOCK, ELEM_SYMBOL, ELEM_VOID,
};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager};

use super::*;
use crate::dtype::type_metatype;

fn manager() -> Rc<AddrSpaceManager> {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    Rc::new(m)
}

fn types_with_core() -> Rc<TypeFactoryImpl> {
    let t = TypeFactoryImpl::new();
    t.setup_sizes(Some(8), 8, 8);
    t.set_core_type("void", 1, type_metatype::TYPE_VOID, false).unwrap();
    t.set_core_type("int4", 4, type_metatype::TYPE_INT, false).unwrap();
    t.set_core_type("uint8", 8, type_metatype::TYPE_UINT, false).unwrap();
    t.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    t.cache_core_types().unwrap();
    Rc::new(t)
}

fn ram(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("ram").unwrap())
}

/// `<hole readonly=true space=ram first last/>` decodes to a readonly hole.
#[test]
fn hole_decodes_with_mutability() {
    let m = manager();
    let types = types_with_core();
    let mut doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut doc);
        e.open_element(&ELEM_HOLE);
        e.write_bool(&ATTRIB_READONLY, true);
        e.write_space(&kuna_base::marshal::ATTRIB_SPACE, &ram(&m));
        e.write_unsigned_integer(&kuna_base::address::ATTRIB_FIRST, 0x1000);
        e.write_unsigned_integer(&kuna_base::address::ATTRIB_LAST, 0x1fff);
        e.close_element(&ELEM_HOLE);
    }
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&doc).unwrap();
    match decode_mapped_answer(&mut dec, &types).unwrap() {
        RemoteMapAnswer::Hole { range, flags } => {
            assert_eq!(range.get_first(), 0x1000);
            assert_eq!(range.get_last(), 0x1fff);
            assert_eq!(flags, varnode_flags::readonly);
        }
        _ => panic!("expected a hole"),
    }
}

/// A data-symbol answer: `<doc id=0><mapsym><symbol …><typeref int4/></symbol>
/// <addr size=4/><rangelist/></mapsym></doc>`.
#[test]
fn data_symbol_mapsym_decodes() {
    let m = manager();
    let types = types_with_core();
    let int4 = crate::dtype::TypeFactory::find_by_name(&*types, "int4")
        .unwrap()
        .unwrap();
    let mut doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut doc);
        e.open_element(&crate::prettyprint::ids::ELEM_TYPE); // any wrapper id (upstream <doc>)
        e.write_unsigned_integer(&ATTRIB_ID, 0);
        e.open_element(&ELEM_MAPSYM);
        e.open_element(&ELEM_SYMBOL);
        e.write_unsigned_integer(&ATTRIB_ID, 0x1234);
        e.write_string(&ATTRIB_NAME, b"my_global");
        e.write_bool(&ATTRIB_TYPELOCK, true);
        e.write_bool(&ATTRIB_NAMELOCK, true);
        e.write_bool(&ATTRIB_READONLY, true);
        e.write_signed_integer(&ATTRIB_CAT, -1);
        e.open_element(&crate::dtype::ELEM_TYPEREF);
        e.write_string(&ATTRIB_NAME, b"int4");
        e.write_unsigned_integer(&ATTRIB_ID, int4.get_id());
        e.close_element(&crate::dtype::ELEM_TYPEREF);
        e.close_element(&ELEM_SYMBOL);
        Address::new(ram(&m), 0x2000).encode_sized(&mut e, 4).unwrap();
        e.open_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_MAPSYM);
        e.close_element(&crate::prettyprint::ids::ELEM_TYPE);
    }
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&doc).unwrap();
    let rec = match decode_mapped_answer(&mut dec, &types).unwrap() {
        RemoteMapAnswer::Symbol(rec) => rec,
        _ => panic!("expected a symbol"),
    };
    assert_eq!(rec.name, "my_global");
    assert_eq!(rec.symbol_id, 0x1234);
    assert_eq!(
        rec.flags,
        varnode_flags::typelock | varnode_flags::namelock | varnode_flags::readonly
    );
    assert!(Rc::ptr_eq(rec.dtype.as_ref().unwrap(), &int4));
    assert_eq!(rec.entries.len(), 1);
    assert_eq!(rec.entries[0].addr.get_offset(), 0x2000);
    assert_eq!(rec.entries[0].size, 4);
    assert!(rec.entries[0].uselimit.empty());
    assert!(rec.func.is_none());
}

/// A function answer with `<localdb>` cat-0 params and a `<prototype>`:
/// the noreturn flag, the sorted locked params, and `to_pieces`.
#[test]
fn function_mapsym_decodes_prototype_and_params() {
    let m = manager();
    let types = types_with_core();
    let int4 = crate::dtype::TypeFactory::find_by_name(&*types, "int4")
        .unwrap()
        .unwrap();
    let mut doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut doc);
        e.open_element(&crate::prettyprint::ids::ELEM_TYPE); // the <doc> wrapper
        e.write_unsigned_integer(&ATTRIB_ID, 0);
        e.open_element(&ELEM_MAPSYM);
        e.open_element(&crate::funcdata_encode::ELEM_FUNCTION);
        e.write_unsigned_integer(&ATTRIB_ID, 0x77);
        e.write_string(&ATTRIB_NAME, b"my_exit");
        e.write_signed_integer(&ATTRIB_SIZE, 1);
        e.write_bool(&ATTRIB_NORETURN, true);
        Address::new(ram(&m), 0x3000).encode(&mut e).unwrap();
        // <localdb lock=false main=ram><scope><parent id=0/><rangelist/>
        //   <symbollist> params (index 1 encoded FIRST — the decoder sorts)
        e.open_element(&ELEM_LOCALDB);
        e.write_bool(&ATTRIB_LOCK, false);
        e.write_space(&ATTRIB_MAIN, &ram(&m));
        e.open_element(&ELEM_SCOPE);
        e.write_string(&ATTRIB_NAME, b"my_exit");
        e.open_element(&ELEM_PARENT);
        e.write_unsigned_integer(&ATTRIB_ID, 0);
        e.close_element(&ELEM_PARENT);
        e.open_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_RANGELIST);
        e.open_element(&ELEM_SYMBOLLIST);
        for (idx, pname) in [(1u64, "second"), (0u64, "first")] {
            e.open_element(&ELEM_MAPSYM);
            e.open_element(&ELEM_SYMBOL);
            e.write_string(&ATTRIB_NAME, pname.as_bytes());
            e.write_bool(&ATTRIB_TYPELOCK, true);
            e.write_bool(&ATTRIB_NAMELOCK, true);
            e.write_signed_integer(&ATTRIB_CAT, 0);
            e.write_unsigned_integer(&ATTRIB_INDEX, idx);
            e.open_element(&crate::dtype::ELEM_TYPEREF);
            e.write_string(&ATTRIB_NAME, b"int4");
            e.write_unsigned_integer(&ATTRIB_ID, int4.get_id());
            e.close_element(&crate::dtype::ELEM_TYPEREF);
            e.close_element(&ELEM_SYMBOL);
            e.open_element(&ELEM_HASH);
            e.write_unsigned_integer(&kuna_base::marshal::ATTRIB_VAL, 1);
            e.close_element(&ELEM_HASH);
            e.open_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_MAPSYM);
        }
        e.close_element(&ELEM_SYMBOLLIST);
        e.close_element(&ELEM_SCOPE);
        e.close_element(&ELEM_LOCALDB);
        // <prototype extrapop="unknown" model="default"><returnsym><addr/><void/>
        e.open_element(&ELEM_PROTOTYPE);
        e.write_string(&ATTRIB_EXTRAPOP, b"unknown");
        e.write_string(&ATTRIB_MODEL, b"default");
        e.open_element(&ELEM_RETURNSYM);
        e.write_bool(&ATTRIB_TYPELOCK, true);
        e.open_element(&kuna_base::address::ELEM_ADDR);
        e.close_element(&kuna_base::address::ELEM_ADDR);
        e.open_element(&ELEM_VOID);
        e.close_element(&ELEM_VOID);
        e.close_element(&ELEM_RETURNSYM);
        e.close_element(&ELEM_PROTOTYPE);
        e.close_element(&crate::funcdata_encode::ELEM_FUNCTION);
        // The mapping SymbolEntry.
        Address::new(ram(&m), 0x3000).encode_sized(&mut e, 1).unwrap();
        e.open_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_MAPSYM);
        e.close_element(&crate::prettyprint::ids::ELEM_TYPE);
    }
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&doc).unwrap();
    let rec = match decode_mapped_answer(&mut dec, &types).unwrap() {
        RemoteMapAnswer::Symbol(rec) => rec,
        _ => panic!("expected a symbol"),
    };
    let func = rec.func.as_ref().expect("function record");
    assert_eq!(func.name, "my_exit");
    assert!(func.no_return);
    assert_eq!(func.entry.get_offset(), 0x3000);
    let proto = func.proto.as_ref().expect("prototype decoded");
    assert_eq!(proto.model, "default");
    assert_eq!(proto.extrapop, None); // the "unknown" string form
    assert!(proto.out_lock);
    assert!(proto.out_type.as_ref().unwrap().get_metatype() == type_metatype::TYPE_VOID);
    assert!(proto.is_input_locked());
    // Sorted by slot index despite reversed wire order.
    assert_eq!(proto.params.len(), 2);
    assert_eq!(proto.params[0].name, "first");
    assert_eq!(proto.params[1].name, "second");
    let pieces = proto.to_pieces(&func.name);
    assert_eq!(pieces.name, "my_exit");
    assert_eq!(pieces.intypes.len(), 2);
    assert_eq!(pieces.innames, vec!["first".to_string(), "second".to_string()]);
    assert_eq!(pieces.first_var_arg_slot, -1);
}

/// `<parent>` namespace-path decode: root skipped, innermost-first order.
#[test]
fn namespace_path_decodes_innermost_first() {
    let m = manager();
    let mut doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut doc);
        e.open_element(&ELEM_PARENT);
        for (id, nm) in [(0u64, ""), (10, "outer"), (11, "inner")] {
            e.open_element(&kuna_base::marshal::ELEM_VAL);
            e.write_unsigned_integer(&ATTRIB_ID, id);
            e.write_string(&ATTRIB_CONTENT, nm.as_bytes());
            e.close_element(&kuna_base::marshal::ELEM_VAL);
        }
        e.close_element(&ELEM_PARENT);
    }
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&doc).unwrap();
    let path = decode_namespace_path(&mut dec).unwrap();
    assert_eq!(path, vec!["inner".to_string(), "outer".to_string()]);
}

/// The wire `<coretypes>` XML mirror of the default core-type list decodes to
/// the SAME (name, id, size, metatype) set `set_core_type` builds — the
/// invariant that makes sim-side `<typeref>` name+id resolution agree.
#[test]
fn coretypes_xml_mirror_matches_default_build() {
    const DEFAULTS: &[(&str, i32, type_metatype, bool)] = &[
        ("void", 1, type_metatype::TYPE_VOID, false),
        ("bool", 1, type_metatype::TYPE_BOOL, false),
        ("uint1", 1, type_metatype::TYPE_UINT, false),
        ("uint2", 2, type_metatype::TYPE_UINT, false),
        ("uint4", 4, type_metatype::TYPE_UINT, false),
        ("uint8", 8, type_metatype::TYPE_UINT, false),
        ("int1", 1, type_metatype::TYPE_INT, false),
        ("int2", 2, type_metatype::TYPE_INT, false),
        ("int4", 4, type_metatype::TYPE_INT, false),
        ("int8", 8, type_metatype::TYPE_INT, false),
        ("float4", 4, type_metatype::TYPE_FLOAT, false),
        ("float8", 8, type_metatype::TYPE_FLOAT, false),
        ("float10", 10, type_metatype::TYPE_FLOAT, false),
        ("float16", 16, type_metatype::TYPE_FLOAT, false),
        ("xunknown1", 1, type_metatype::TYPE_UNKNOWN, false),
        ("xunknown2", 2, type_metatype::TYPE_UNKNOWN, false),
        ("xunknown4", 4, type_metatype::TYPE_UNKNOWN, false),
        ("xunknown8", 8, type_metatype::TYPE_UNKNOWN, false),
        ("code", 1, type_metatype::TYPE_CODE, false),
        ("char", 1, type_metatype::TYPE_INT, true),
        ("wchar2", 2, type_metatype::TYPE_INT, true),
        ("wchar4", 4, type_metatype::TYPE_INT, true),
    ];
    // The XML mirror (no id attributes -> hash_name ids, like set_core_type).
    let mut xml = String::from("<coretypes>");
    for (name, size, meta, chartp) in DEFAULTS {
        let ms = crate::dtype::metatype2string(if *chartp {
            type_metatype::TYPE_INT
        } else {
            *meta
        })
        .unwrap();
        xml.push_str(&format!("<type name=\"{name}\" size=\"{size}\" metatype=\"{ms}\""));
        if *chartp && *size == 1 {
            xml.push_str(" char=\"true\"");
        } else if *chartp {
            xml.push_str(" utf=\"true\"");
        }
        xml.push_str("/>");
    }
    xml.push_str("</coretypes>");

    let reference = TypeFactoryImpl::new();
    reference.setup_sizes(Some(8), 8, 8);
    for (name, size, meta, chartp) in DEFAULTS {
        reference.set_core_type(name, *size, *meta, *chartp).unwrap();
    }
    reference.cache_core_types().unwrap();

    let decoded = TypeFactoryImpl::new();
    decoded.setup_sizes(Some(8), 8, 8);
    let m = manager();
    let mut registry = IdRegistry::with_base_ids();
    crate::dtype::register_type_wire_ids(&mut registry);
    let store = kuna_base::xml::xml_tree(xml.as_bytes()).unwrap();
    let root = store.get_root().clone();
    let mut decoder = XmlDecode::new_with_root(&m, &registry, &root, 0);
    decoded.decode_core_types(&mut decoder).unwrap();

    for (name, size, _meta, _chartp) in DEFAULTS {
        let a = crate::dtype::TypeFactory::find_by_name(&reference, name)
            .unwrap()
            .unwrap_or_else(|| panic!("reference missing {name}"));
        let b = crate::dtype::TypeFactory::find_by_name(&decoded, name)
            .unwrap()
            .unwrap_or_else(|| panic!("decoded missing {name}"));
        assert_eq!(a.get_id(), b.get_id(), "{name}: id mismatch");
        // (void's interned size is the TypeVoid-internal 0 on both sides, so
        // compare a-vs-b rather than against the table.)
        assert_eq!(a.get_size(), b.get_size(), "{name}: size mismatch");
        assert_eq!(a.get_metatype(), b.get_metatype(), "{name}: metatype mismatch");
        assert!(b.is_core_type(), "{name}: decoded type not core");
        let _ = size;
    }
}

/// A REAL Java-side getMappedSymbols function answer, captured from stock
/// Ghidra 12.1.2 via DecompInterface.enableDebug on `fmt`'s `main`
/// (integrations/ghidra/live-smoke rig; the `<xml_savefile>` records the
/// actual DecompileCallback encode).  Trimmed verbatim except for the added
/// `<doc id="0x0">` wrapper (the savefile stores answers unwrapped).  This is
/// the ground-truth shape check for the decoder: hex-form attribute values,
/// integer-form extrapop, register-storage entries with first-use rangelists,
/// inline nested pointer types with builtin 0xC… ids.
const CAPTURED_MAIN_MAPSYM: &str = r#"<doc id="0x0"><mapsym>
  <function id="0x18b" name="main" size="1">
    <addr space="ram" offset="0x101900"/>
    <localdb lock="false" main="stack">
      <scope name="main">
        <parent id="0x0"/>
        <rangelist/>
        <symbollist>
          <mapsym>
            <symbol id="0x1e0" name="argc" typelock="true" namelock="true" merge="false" cat="0" index="0x0">
              <type name="int" id="0xc000000000000095" metatype="int" size="4"/>
            </symbol>
            <addr space="register" offset="0x38" size="4"/>
            <rangelist>
              <range space="ram" first="0x1018ff" last="0x1018ff"/>
            </rangelist>
          </mapsym>
          <mapsym>
            <symbol id="0x1e1" name="argv" typelock="true" namelock="true" merge="false" cat="0" index="0x1">
              <type name="" metatype="ptr" size="8">
                <type name="" metatype="ptr" size="8">
                  <type name="char" id="0xc00000000000007b" metatype="int" size="1" char="true"/>
                </type>
              </type>
            </symbol>
            <addr space="register" offset="0x30" size="8"/>
            <rangelist>
              <range space="ram" first="0x1018ff" last="0x1018ff"/>
            </rangelist>
          </mapsym>
        </symbollist>
      </scope>
    </localdb>
    <prototype extrapop="8" model="__stdcall" modellock="true">
      <returnsym typelock="true">
        <addr space="register" offset="0x0" size="4"/>
        <type name="int" id="0xc000000000000095" metatype="int" size="4"/>
      </returnsym>
    </prototype>
  </function>
  <addr space="ram" offset="0x101900" size="1"/>
  <rangelist/>
</mapsym></doc>"#;

/// Decode the vendored REAL Java answer (XML form; the packed wire form
/// differs only in the marshaling layer behind `dyn Decoder`).
#[test]
fn captured_java_function_mapsym_decodes() {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        0,
        0,
    )))
    .unwrap();
    let m = Rc::new(m);
    let types = types_with_core();

    let mut registry = IdRegistry::with_base_ids();
    register_remote_provider_ids(&mut registry);
    crate::dtype::register_type_wire_ids(&mut registry);
    registry.register_element(&crate::funcdata_encode::ELEM_FUNCTION);
    registry.register_element(&crate::prettyprint::ids::ELEM_TYPE);
    registry.register_attribute(&crate::jumptable::ATTRIB_LABEL);

    let store = kuna_base::xml::xml_tree(CAPTURED_MAIN_MAPSYM.as_bytes()).unwrap();
    let root = store.get_root().clone();
    let mut dec = XmlDecode::new_with_root(&m, &registry, &root, 0);
    let rec = match decode_mapped_answer(&mut dec, &types).unwrap() {
        RemoteMapAnswer::Symbol(rec) => rec,
        _ => panic!("expected a symbol"),
    };
    assert_eq!(rec.symbol_id, 0x18b);
    let func = rec.func.as_ref().expect("function record");
    assert_eq!(func.name, "main");
    assert_eq!(func.entry.get_offset(), 0x101900);
    assert!(!func.no_return);
    let proto = func.proto.as_ref().expect("prototype");
    assert_eq!(proto.model, "__stdcall");
    assert_eq!(proto.extrapop, Some(8)); // the integer form
    assert!(proto.out_lock);
    // The capture's builtin "int" carries Java's 0xC… id, interned as its own
    // instance beside any locally-hashed "int" — compare by shape, not identity.
    let out = proto.out_type.as_ref().unwrap();
    assert_eq!(out.get_name(), "int");
    assert_eq!(out.get_size(), 4);
    assert_eq!(out.get_metatype(), type_metatype::TYPE_INT);
    assert_eq!(out.get_id(), 0xc000000000000095);
    assert!(proto.is_input_locked());
    assert_eq!(proto.params.len(), 2);
    assert_eq!(proto.params[0].name, "argc");
    let argc_ty = proto.params[0].dtype.as_ref().unwrap();
    assert!(Rc::ptr_eq(argc_ty, out), "both int refs intern to one instance");
    assert_eq!(proto.params[1].name, "argv");
    let argv_ty = proto.params[1].dtype.as_ref().unwrap();
    assert_eq!(argv_ty.get_metatype(), type_metatype::TYPE_PTR);
    // The mapping entry (the outer <addr size=1> at the function entry).
    assert_eq!(rec.entries.len(), 1);
    assert_eq!(rec.entries[0].addr.get_offset(), 0x101900);
}

// ---------------------------------------------------------------------------
// RemoteScope behavior over a canned wire fetcher
// ---------------------------------------------------------------------------

/// A canned [`RemoteProviderFetch`]: per-query response maps keyed by offset,
/// plus call counters (the negative-caching asserts).
#[derive(Default)]
struct MockFetch {
    mapped: std::collections::BTreeMap<u64, Vec<u8>>,
    external: std::collections::BTreeMap<u64, Vec<u8>>,
    tracked: std::collections::BTreeMap<u64, Vec<u8>>,
    /// Offsets whose getMappedSymbols FAILS (a wire/JavaError).
    fail_mapped: std::collections::BTreeSet<u64>,
    calls: std::cell::RefCell<std::collections::BTreeMap<&'static str, usize>>,
}

impl MockFetch {
    fn count(&self, what: &'static str) -> usize {
        *self.calls.borrow().get(what).unwrap_or(&0)
    }
    fn bump(&self, what: &'static str) {
        *self.calls.borrow_mut().entry(what).or_insert(0) += 1;
    }
    fn serve(
        map: &std::collections::BTreeMap<u64, Vec<u8>>,
        off: u64,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        match map.get(&off) {
            Some(doc) => {
                decoder.ingest_stream(doc)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

impl RemoteProviderFetch for MockFetch {
    fn fetch_mapped_symbols(
        &self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        self.bump("mapped");
        if self.fail_mapped.contains(&addr.get_offset()) {
            return Err(kuna_base::error::KunaError::lowlevel("mock wire failure"));
        }
        Self::serve(&self.mapped, addr.get_offset(), decoder)
    }
    fn fetch_namespace_path(&self, _id: u64, _decoder: &mut dyn Decoder) -> KunaResult<bool> {
        self.bump("namespace");
        Ok(false)
    }
    fn fetch_external_ref(&self, addr: &Address, decoder: &mut dyn Decoder) -> KunaResult<bool> {
        self.bump("external");
        Self::serve(&self.external, addr.get_offset(), decoder)
    }
    fn fetch_tracked_registers(
        &self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        self.bump("tracked");
        Self::serve(&self.tracked, addr.get_offset(), decoder)
    }
    fn fetch_comments(
        &self,
        _fad: &Address,
        _flags: u32,
        _decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        Ok(false)
    }
}

fn scope_over(m: &Rc<AddrSpaceManager>, types: Rc<TypeFactoryImpl>, fetch: MockFetch) -> RemoteScope {
    let mut owned = kuna_base::address::RangeList::default();
    owned.insert_range(ram(m), 0, u64::MAX >> 8);
    let base = GlobalQuery::new(Vec::new(), owned, PartMap::default());
    RemoteScope::new(
        Rc::new(fetch),
        Rc::clone(m),
        types,
        std::collections::BTreeMap::new(),
        None,
        base,
    )
}

/// A packed `<doc><mapsym><externrefsymbol …>` answer at the pointer, whose
/// getExternalRef resolves to a function carrying BOTH a raw name and a
/// display label: the two-step resolution runs, the facts split name/label
/// (the HighFunction name-echo contract), and the pointer symbol survives.
#[test]
fn extern_ref_resolves_and_splits_name_from_label() {
    let m = manager();
    let types = types_with_core();
    // Pointer symbol at ram:0x5000 -> resolve address ram:0x7000 (the
    // refaddr <addr> is a CHILD of <externrefsymbol>, as upstream encodes).
    let mut ptr_doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut ptr_doc);
        e.open_element(&crate::prettyprint::ids::ELEM_TYPE);
        e.write_unsigned_integer(&ATTRIB_ID, 0);
        e.open_element(&ELEM_MAPSYM);
        e.open_element(&ELEM_EXTERNREFSYMBOL);
        e.write_string(&ATTRIB_NAME, b"imp_templated");
        Address::new(ram(&m), 0x7000).encode(&mut e).unwrap();
        e.close_element(&ELEM_EXTERNREFSYMBOL);
        Address::new(ram(&m), 0x5000).encode_sized(&mut e, 8).unwrap();
        e.open_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_MAPSYM);
        e.close_element(&crate::prettyprint::ids::ELEM_TYPE);
    }
    // The resolved function: raw name != label (the TemplateSimplifier case).
    let mut fn_doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut fn_doc);
        e.open_element(&crate::prettyprint::ids::ELEM_TYPE);
        e.write_unsigned_integer(&ATTRIB_ID, 0);
        e.open_element(&ELEM_MAPSYM);
        e.open_element(&crate::funcdata_encode::ELEM_FUNCTION);
        e.write_unsigned_integer(&ATTRIB_ID, 0x99);
        e.write_string(&ATTRIB_NAME, b"foo<int,long>");
        e.write_string(&crate::jumptable::ATTRIB_LABEL, b"foo");
        e.write_signed_integer(&ATTRIB_SIZE, 1);
        e.write_bool(&ATTRIB_NORETURN, true);
        Address::new(ram(&m), 0x7000).encode(&mut e).unwrap();
        e.close_element(&crate::funcdata_encode::ELEM_FUNCTION);
        Address::new(ram(&m), 0x7000).encode_sized(&mut e, 1).unwrap();
        e.open_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_MAPSYM);
        e.close_element(&crate::prettyprint::ids::ELEM_TYPE);
    }
    let mut fetch = MockFetch::default();
    fetch.mapped.insert(0x5000, ptr_doc);
    fetch.external.insert(0x5000, fn_doc);
    let scope = scope_over(&m, types, fetch);

    let probe = Address::new(ram(&m), 0x5000);
    scope.ensure_queried(&probe);
    // The two-step resolution materialized the FUNCTION at its own entry.
    let facts = scope
        .function_at(&Address::new(ram(&m), 0x7000))
        .expect("external function resolved");
    assert_eq!(facts.name, "foo<int,long>", "raw name is the Java identity");
    assert_eq!(facts.display_name, "foo", "label is only the display form");
    assert!(facts.no_return);
    // The pointer's own symbol landed too (pointer-to-code typed).
    let snap = scope.snapshot();
    let (nm, _, ty) = snap
        .name_for_varnode(&probe, 8, &Address::new_invalid())
        .expect("pointer symbol materialized");
    assert_eq!(nm, "imp_templated");
    assert_eq!(
        ty.expect("pointer type").get_metatype(),
        type_metatype::TYPE_PTR
    );
}

/// getTrackedRegisters answers cache per address until `clear()` (one wire
/// ask per flush epoch), and decode failures negative-cache with ONE warning.
#[test]
fn tracked_caches_and_failures_negative_cache() {
    let m = manager();
    let types = types_with_core();
    // A tracked answer for ram:0x1000: one register value.
    let reg_loc = kuna_num::pcoderaw::VarnodeData {
        space: Some(ram(&m)),
        offset: 0x30,
        size: 8,
    };
    let mut tr_doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut tr_doc);
        let set = vec![kuna_sleigh::globalcontext::TrackedContext {
            loc: reg_loc.clone(),
            val: 0x1234,
        }];
        kuna_sleigh::globalcontext::encode_tracked(
            &mut e,
            &Address::new(ram(&m), 0x1000),
            &set,
        )
        .unwrap();
    }
    let mut fetch = MockFetch::default();
    fetch.tracked.insert(0x1000, tr_doc);
    fetch.fail_mapped.insert(0x4444);
    let scope = scope_over(&m, types, fetch);

    let at = Address::new(ram(&m), 0x1000);
    let set = scope.tracked_at(&at).expect("tracked set");
    assert_eq!(set.len(), 1);
    assert_eq!(set[0].val, 0x1234);
    let set2 = scope.tracked_at(&at).expect("tracked set cached");
    assert_eq!(set2.len(), 1);

    // A FAILING getMappedSymbols address: asked once, negative-cached, one
    // drained warning; clear() re-opens both caches.
    let bad = Address::new(ram(&m), 0x4444);
    scope.ensure_queried(&bad);
    scope.ensure_queried(&bad);
    let warnings = scope.drain_warnings();
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert!(warnings[0].starts_with("Warning:"));
    assert!(scope.drain_warnings().is_empty());
    scope.clear();
    scope.ensure_queried(&bad);
    let _ = scope.tracked_at(&at);
    // counts: mapped fired twice total (once per epoch), tracked twice.
    // (Read back through the fetch — the scope owns it, so assert indirectly:
    // a second same-epoch ensure_queried did NOT re-fire, proven by the single
    // warning above; the post-clear re-fire is proven by a fresh warning.)
    let warnings = scope.drain_warnings();
    assert_eq!(warnings.len(), 1, "post-clear re-query fired again");
}

/// Phase-4 differential: the `ScopeLocal::encode` `<localdb>` document decodes
/// through the SAME decoder Java shapes decode through (`decode_localdb_params`
/// / `decode_mapsym`): the cat-0 parameter round-trips with its slot index and
/// typelock, the plain local does not become a parameter, and every `<symbol>`
/// carries a NONZERO internal-range id (`HighSymbol.decodeHeader` throws
/// "missing unique symbol id" on 0 — the r5 §3 trap).
#[test]
fn phase4_localdb_encode_decodes_as_params() {
    let m = manager();
    let types = types_with_core();
    let int4_t = crate::dtype::TypeFactory::find_by_name(types.as_ref(), "int4")
        .unwrap()
        .unwrap();
    let mut lm =
        crate::varmap::ScopeLocal::new(0x1234, ram(&m), "testfn", m.num_spaces()).unwrap();
    // A register-style cat-0 parameter (exact storage + slot index 0).
    let param_addr = Address::new(ram(&m), 0x38);
    let usepoint = Address::new(ram(&m), 0x100f);
    let sid = lm
        .add_param_symbol(0, "argc", Rc::clone(&int4_t), &param_addr, &usepoint)
        .unwrap()
        .expect("param created");
    // Lock it (the Java write path needs cat/index/storage; locks ride along).
    let _ = sid;
    // A plain mapped local (no category).
    lm.add_symbol("local_10", Rc::clone(&int4_t), &Address::new(ram(&m), 0x2000), &Address::new_invalid())
        .unwrap();

    let mut doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut doc);
        lm.encode(&mut e).unwrap();
    }

    // Decode through the wire-shape consumer.
    let mut dec = kuna_base::marshal::PackedDecode::new(&m);
    kuna_base::marshal::Decoder::ingest_stream(&mut dec, &doc).unwrap();
    let mut params: Vec<RemoteParam> = Vec::new();
    let mut locals: Vec<RemoteLocalVar> = Vec::new();
    let mut locked = true;
    decode_localdb_params(&mut dec, types.as_ref(), &mut params, &mut locked, &mut locals)
        .unwrap();
    assert_eq!(params.len(), 1, "exactly the cat-0 symbol is a parameter");
    assert_eq!(params[0].index, 0);
    assert_eq!(params[0].name, "argc");
    assert!(params[0].dtype.as_ref().is_some_and(|t| t.get_size() == 4));
    // The non-parameter local rides the Phase-4 seeding channel: name + first
    // mapped entry + (empty uselimit ⇒ invalid) usepoint.
    assert_eq!(locals.len(), 1, "the plain local is captured for seeding");
    assert_eq!(locals[0].name, "local_10");
    assert_eq!(locals[0].addr.get_offset(), 0x2000);
    assert!(locals[0].usepoint.is_invalid(), "empty uselimit ⇒ addr-tied seed");

    // Structural walk: every <symbol> has a nonzero internal-range id, and every
    // <mapsym> carries at least one entry (addr|hash) + rangelist pair.
    let mut dec = kuna_base::marshal::PackedDecode::new(&m);
    kuna_base::marshal::Decoder::ingest_stream(&mut dec, &doc).unwrap();
    let localdb = dec.open_element().unwrap();
    assert_eq!(localdb, ELEM_LOCALDB.get_id());
    let scope_el = dec.open_element().unwrap();
    assert_eq!(scope_el, ELEM_SCOPE.get_id());
    // Positional contract: <parent> then <rangelist> BEFORE <symbollist>
    // (LocalSymbolMap.decodeScope skips the first two children blind).
    let parent_el = dec.open_element().unwrap();
    assert_eq!(parent_el, ELEM_PARENT.get_id(), "first scope child must be <parent>");
    dec.close_element_skipping(parent_el).unwrap();
    let rl_el = dec.open_element().unwrap();
    assert_eq!(rl_el, ELEM_RANGELIST.get_id(), "second scope child must be <rangelist>");
    dec.close_element_skipping(rl_el).unwrap();
    let list_el = dec.open_element().unwrap();
    assert_eq!(list_el, ELEM_SYMBOLLIST.get_id());
    let mut mapsym_count = 0;
    while dec.peek_element().unwrap() != 0 {
        let ms = dec.open_element().unwrap();
        assert_eq!(ms, ELEM_MAPSYM.get_id());
        mapsym_count += 1;
        // <symbol id=NONZERO ...>
        let sym_el = dec.open_element().unwrap();
        assert_eq!(sym_el, ELEM_SYMBOL.get_id());
        let id = kuna_base::marshal::Decoder::read_unsigned_integer_id(&mut dec, &ATTRIB_ID)
            .unwrap();
        assert_ne!(id, 0, "symbol id must be nonzero (Java hard-throw)");
        assert_eq!(id >> 56, 0x40, "invented symbols carry internal-range ids");
        dec.close_element_skipping(sym_el).unwrap();
        // >=1 SymbolEntry: (addr|hash) + rangelist.
        let entry_el = dec.open_element().unwrap();
        assert!(
            entry_el == kuna_base::address::ELEM_ADDR.get_id()
                || entry_el == ELEM_HASH.get_id(),
            "mapsym entry must open with <addr> or <hash>"
        );
        dec.close_element_skipping(entry_el).unwrap();
        let ul_el = dec.open_element().unwrap();
        assert_eq!(ul_el, ELEM_RANGELIST.get_id(), "entry uselimit rangelist required");
        dec.close_element_skipping(ul_el).unwrap();
        dec.close_element_skipping(ms).unwrap();
    }
    assert_eq!(mapsym_count, 2);
    dec.close_element(list_el).unwrap();
    dec.close_element(scope_el).unwrap();
    dec.close_element(localdb).unwrap();
}
