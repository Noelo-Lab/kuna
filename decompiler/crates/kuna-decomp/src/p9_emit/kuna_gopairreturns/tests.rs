use super::*;
use crate::dtype::{DatatypeKind, TypeFactoryImpl, TypeField};
use crate::fspec::{FuncProto, ParameterPieces, ProtoModel};
use crate::kuna_lang::OutLang;
use crate::kuna_langfacet::LangFacet;
use crate::kuna_langgo::GO_SPELLER;
use crate::kuna_langtypes::TypeSpeller;
use kuna_base::address::Address;

fn pair(factory: &TypeFactoryImpl, name: &str) -> Rc<Datatype> {
    let shell = factory.get_type_struct(name).unwrap();
    let fields = [type_metatype::TYPE_INT, type_metatype::TYPE_UINT]
        .into_iter()
        .enumerate()
        .map(|(i, meta)| {
            TypeField::new(
                i as i32 * 8,
                i as i32 * 8,
                format!("result{i}"),
                factory.get_base(8, meta).unwrap(),
            )
        })
        .collect();
    factory
        .set_fields_struct_raw(&shell, fields, Vec::new(), 16, 8, 0)
        .unwrap()
}

#[test]
fn paired_function_results_and_storage_have_distinct_go_types() {
    let factory = Rc::new(TypeFactoryImpl::new());
    factory.setup_sizes(Some(8), 8, 8);
    factory.set_max_basetype_size(8);
    let ty = pair(&factory, "verified");
    factory
        .kuna_register_lang_facet(
            "verified",
            LangFacet::GoPairResults {
                signed: [true, false],
            },
        )
        .unwrap();
    let cx = SpellCtx {
        lang: OutLang::Go,
        go_pair_types: Some(Rc::clone(&factory)),
        ..SpellCtx::OFF
    };
    assert_eq!(
        GO_SPELLER.declarator(&cx, &ty).0,
        "struct { result0 int64; result1 uint64 }"
    );
    assert_eq!(result_type(&cx, &ty).as_deref(), Some("(int64, uint64)"));
    let manager = kuna_base::space::AddrSpaceManager::new();
    let mut proto = FuncProto::new();
    proto.set_internal(
        Rc::new(ProtoModel::new(&manager)),
        Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)),
    );
    proto.set_param(
        0,
        "x",
        &ParameterPieces {
            addr: Address::new_invalid(),
            type_: Some(factory.get_base(8, type_metatype::TYPE_INT).unwrap()),
            flags: 0,
        },
    );
    proto.set_output(&ParameterPieces {
        addr: Address::new_invalid(),
        type_: Some(ty),
        flags: 0,
    });
    let mut code = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
    code.kind = DatatypeKind::Code {
        proto: Some(Rc::new(proto)),
    };
    let pointer = factory.get_type_pointer(8, Rc::new(code), 1).unwrap();
    assert_eq!(
        GO_SPELLER.declarator(&cx, &pointer).0,
        "func(int64) (int64, uint64)"
    );
}

#[test]
fn ordinary_record_and_stale_metadata_never_imply_pair_results() {
    let factory = Rc::new(TypeFactoryImpl::new());
    factory.setup_sizes(Some(8), 8, 8);
    factory.set_max_basetype_size(8);
    let ty = pair(&factory, "__kuna_go_pair_i64_u64");
    let cx = SpellCtx {
        lang: OutLang::Go,
        go_pair_types: Some(Rc::clone(&factory)),
        ..SpellCtx::OFF
    };
    assert!(storage_type(&cx, &ty).is_none());
    factory
        .kuna_register_lang_facet(
            ty.get_name(),
            LangFacet::GoPairResults {
                signed: [true, true],
            },
        )
        .unwrap();
    assert!(result_type(&cx, &ty).is_none());
    factory
        .kuna_register_lang_facet(
            ty.get_name(),
            LangFacet::GoPairResults {
                signed: [true, false],
            },
        )
        .unwrap();
    let off = SpellCtx {
        lang: OutLang::Go,
        ..SpellCtx::OFF
    };
    assert!(result_type(&off, &ty).is_none());
    assert!(!GO_SPELLER.declarator(&off, &ty).0.starts_with("struct {"));
}
