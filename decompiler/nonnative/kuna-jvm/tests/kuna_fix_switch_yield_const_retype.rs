//! `jvm_fix_switch_yield_const_retype`: `ConstExprent::can_be_written_as`
//! accepts exactly the types Vineflower's `ConstExprent.toJava` writes, per
//! the b02 golden rows, and rejects the common supertypes a switch
//! expression's arms can join to.

use kuna_jvm::code_type::CodeType;
use kuna_jvm::const_exprent::{ConstExprent, ConstValue};
use kuna_jvm::var_type::VarType;

use crate::b02_const_exprent::vartype;
use crate::common;
use crate::f2_types::unesc;

const GOLDEN: &str = include_str!("data/b02/const_golden.txt");

fn value(spec: &str) -> Option<ConstValue> {
    if spec == "N" {
        return None;
    }
    let v = &spec[2..];
    Some(match &spec[..1] {
        "I" => ConstValue::Int(v.parse().unwrap()),
        "J" => ConstValue::Long(v.parse().unwrap()),
        "F" => ConstValue::Float(f32::from_bits(u32::from_str_radix(v, 16).unwrap())),
        "D" => ConstValue::Double(f64::from_bits(u64::from_str_radix(v, 16).unwrap())),
        "S" => ConstValue::String(v.to_string()),
        other => panic!("bad value spec {other}"),
    })
}

#[test]
fn writable_types_are_those_vineflower_writes() {
    let _guard = common::install_default_context();
    let mut checked = 0;
    let mut rejected = 0;
    for line in GOLDEN.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f[0] != "R" || f[1] != "T" {
            continue;
        }
        let spec = unesc(f[3]).expect("value spec");
        let type_ = vartype(f[2]);
        let c = ConstExprent::new(type_.clone(), value(&spec), None);
        let invalid = f[5].starts_with("EXC:RuntimeException:invalid constant type");
        assert_eq!(c.can_be_written_as(&type_), !invalid, "{line}");
        checked += 1;
        rejected += usize::from(invalid);
    }
    assert!(checked > 5_000, "only {checked} rows");
    assert!(rejected > 0, "no row of the golden rejects a type");
}

#[test]
fn a_literal_is_not_writable_as_a_join_of_unrelated_arms() {
    let _guard = common::install_default_context();
    let string = ConstExprent::new(
        VarType::VARTYPE_STRING(),
        Some(ConstValue::String("s".into())),
        None,
    );
    let class = ConstExprent::new(
        VarType::VARTYPE_CLASS(),
        Some(ConstValue::String("java/lang/String".into())),
        None,
    );
    let int = ConstExprent::new(VarType::VARTYPE_BYTECHAR(), Some(ConstValue::Int(97)), None);
    let joins = [
        VarType::VARTYPE_OBJECT(),
        VarType::new_signature("Ljava/io/Serializable;"),
        VarType::new_signature("Ljava/lang/Comparable;"),
        VarType::new_signature("Ljava/lang/Number;"),
        VarType::new_signature("TT;"),
    ];
    for join in &joins {
        assert!(!string.can_be_written_as(join), "String as {join}");
        assert!(!class.can_be_written_as(join), "Class as {join}");
        assert!(!int.can_be_written_as(join), "int as {join}");
    }
    assert!(string.can_be_written_as(&VarType::VARTYPE_STRING()));
    assert!(class.can_be_written_as(&VarType::VARTYPE_CLASS()));
    for primitive in [
        VarType::VARTYPE_CHAR(),
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_BOOLEAN(),
        VarType::VARTYPE_CHARACTER(),
        VarType::VARTYPE_INTEGER(),
    ] {
        assert!(int.can_be_written_as(&primitive), "int as {primitive}");
    }
    let no_value = ConstExprent::new(VarType::VARTYPE_STRING(), None, None);
    assert!(no_value.can_be_written_as(&VarType::new_signature("Ljava/io/Serializable;")));
    assert!(no_value.can_be_written_as(&VarType::new(CodeType::Null)));
}
