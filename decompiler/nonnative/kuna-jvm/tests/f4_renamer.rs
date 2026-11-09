//! The renamer package against Vineflower 1.12.0 (`data/GenF4Oracle.java`):
//! `ConverterHelper` on ~430 identifiers (`toBeRenamed` for each element type,
//! the stateful `getNextClassName` sequence, the counters, the static helpers)
//! and `IdentifierConverter.rename` over `data/f4_classes/o`, compared through
//! the exact put order of the resulting `PoolInterceptor` maps.

use std::rc::Rc;

use kuna_jvm::classes_processor::ClassesProcessor;
use kuna_jvm::context_source::Entry;
use kuna_jvm::converter_helper::ConverterHelper;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::i_identifier_renamer::{IIdentifierRenamer, Type};
use kuna_jvm::identifier_converter::IdentifierConverter;
use kuna_jvm::pool_interceptor::PoolInterceptor;
use kuna_jvm::struct_context::StructContext;

use crate::f4_util::{golden, unesc, unhex, MemorySource, NoDecompiledData};

fn rows(tag: &str) -> Vec<Vec<String>> {
    golden("f4_converter_helper.txt")
        .into_iter()
        .filter(|l| l.split('\t').next() == Some(tag))
        .map(|l| l.split('\t').skip(1).map(str::to_string).collect())
        .collect()
}

#[test]
fn to_be_renamed_matches_vineflower() {
    let helper = ConverterHelper::new();
    let mut checked = 0;
    for row in rows("R") {
        let id = unhex(&row[0]);
        let id = id.as_deref();
        let got = [
            helper.to_be_renamed(Type::ElementClass, id, None, None),
            helper.to_be_renamed(Type::ElementField, Some("pkg/Cls"), id, Some("I")),
            helper.to_be_renamed(Type::ElementMethod, Some("pkg/Cls"), id, Some("()V")),
        ];
        let want = [row[1] == "1", row[2] == "1", row[3] == "1"];
        assert_eq!(got, want, "{:?} ({})", id, row[0]);
        checked += 1;
    }
    assert!(checked > 400, "{checked}");
}

#[test]
fn next_names_match_vineflower() {
    let helper = ConverterHelper::new();
    for row in rows("N") {
        let short = unhex(&row[0]);
        let full = format!("pkg/{}", short.as_deref().unwrap_or("X"));
        let got = helper.get_next_class_name(&full, short.as_deref());
        assert_eq!(Some(got), unhex(&row[1]), "{short:?}");
    }
    for row in rows("M") {
        assert_eq!(
            Some(helper.get_next_field_name("pkg/X", "f", "I")),
            unhex(&row[0])
        );
        assert_eq!(
            Some(helper.get_next_method_name("pkg/X", "m", "()V")),
            unhex(&row[1])
        );
    }
    for row in rows("S") {
        let full = unhex(&row[0]).unwrap();
        assert_eq!(
            Some(ConverterHelper::get_simple_class_name(&full)),
            unhex(&row[1])
        );
        assert_eq!(
            Some(ConverterHelper::replace_simple_class_name(&full, "New")),
            unhex(&row[2])
        );
    }
}

#[test]
fn pool_interceptor_maps_both_ways() {
    let interceptor = PoolInterceptor::new();
    interceptor.add_name("a/b", "a/class_0");
    interceptor.add_name("a/b f I", "a/class_0 field_0 I");
    assert_eq!(interceptor.get_name("a/b").as_deref(), Some("a/class_0"));
    assert_eq!(
        interceptor.get_old_name("a/class_0 field_0 I").as_deref(),
        Some("a/b f I")
    );
    assert_eq!(interceptor.get_name("a/c"), None);
    interceptor.add_name("a/b", "a/class_1");
    assert_eq!(interceptor.get_name("a/b").as_deref(), Some("a/class_1"));
    assert_eq!(
        interceptor.get_old_name("a/class_0").as_deref(),
        Some("a/b")
    );
}

#[test]
fn identifier_converter_matches_vineflower() {
    let sc = StructContext::new(Rc::new(NoDecompiledData));
    let interceptor = PoolInterceptor::new();
    let context = DecompilerContext::new(
        IFernflowerPreferences::DEFAULTS(),
        <dyn IFernflowerLogger>::NO_OP(),
        sc,
        ClassesProcessor::new(sc),
        Some(interceptor),
    );
    DecompilerContext::set_current_context(Some(context));
    let mut own = MemorySource::new("own", false);
    for n in ["o/a", "o/b", "o/c", "o/d", "o/e", "o/Good", "o/f"] {
        own = own.cls(n, Entry::BASE_VERSION, n);
    }
    sc.add_space(Rc::new(own), true);
    IdentifierConverter::new(sc, Rc::new(ConverterHelper::new()), interceptor).rename();

    let expected = |tag: &str| -> Vec<(String, String)> {
        golden("f4_identifier_converter.txt")
            .iter()
            .filter_map(|l| {
                let cols: Vec<&str> = l.split('\t').collect();
                (cols[0] == tag).then(|| (unesc(cols[1]).unwrap(), unesc(cols[2]).unwrap()))
            })
            .collect()
    };
    let (old_to_new, new_to_old) = interceptor.with(|d| {
        (
            d.map_old_to_new_names.entry_vec(),
            d.map_new_to_old_names.entry_vec(),
        )
    });
    assert_eq!(old_to_new, expected("O"));
    assert_eq!(new_to_old, expected("N"));
    assert_eq!(old_to_new.len(), 25);
}

#[test]
fn identity_naming_keeps_every_name() {
    use kuna_jvm::byte_cursor::ByteCursor;
    use kuna_jvm::i_variable_naming_factory::IVariableNamingFactory;
    use kuna_jvm::identity_renamer_factory::IdentityRenamerFactory;
    use kuna_jvm::rt::JLinkedHashMap;
    use kuna_jvm::struct_class::StructClass;

    let cl = StructClass::create(ByteCursor::new(crate::f4_util::class_bytes("p/A")), true);
    let provider = IdentityRenamerFactory.create_factory(cl.get_methods().get(0));
    assert!(provider.rename(&JLinkedHashMap::new()).is_none());
    assert_eq!(
        provider.rename_abstract_parameter(Some("x"), 0).as_deref(),
        Some("x")
    );
    assert_eq!(provider.rename_abstract_parameter(None, 1), None);
    provider.add_parent_context(provider.clone());
}
