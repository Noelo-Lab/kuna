//! The kuna fixes' pure helpers on inputs their cases do not reach.

use kuna_jvm::code_type::CodeType;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::testing::{
    accessor_flags, default_import, escape, escape_qualified, form, implicit_widening_conversion as conversion,
    is_function_invoke, java_cast_conversion as cast_conversion, java_property, kotlin_text, mangled, narrows,
    nullable, promotes, split_signature, strictly_below, Form, KotlinCast,
};
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

use crate::common;

#[test]
fn strictly_below_is_a_proper_reference_subtype() {
    let _guard = common::install_default_context();
    let object = VarType::VARTYPE_OBJECT();
    let integer = VarType::VARTYPE_INTEGER();
    let strings = VarType::VARTYPE_STRING().resize_array_dim(1);
    assert!(strictly_below(&integer, &object));
    assert!(strictly_below(&strings, &object));
    assert!(!strictly_below(&object, &integer));
    assert!(!strictly_below(&integer, &integer));
    assert!(!strictly_below(&VarType::VARTYPE_INT(), &object));
    assert!(!strictly_below(&integer, &VarType::VARTYPE_NULL()));
    assert!(!strictly_below(&integer, &VarType::VARTYPE_STRING()));
    assert!(narrows(&integer, &integer, &object));
    assert!(narrows(&integer, &object, &integer));
    assert!(!narrows(&object, &integer, &object));
}

#[test]
fn promotes_is_javac_typing_operands_together() {
    let int = Some(VarType::VARTYPE_INT());
    let long = Some(VarType::VARTYPE_LONG());
    let integer = Some(VarType::VARTYPE_INTEGER());
    let boolean = Some(VarType::VARTYPE_BOOLEAN());
    let object = Some(VarType::VARTYPE_OBJECT());
    assert!(promotes(&[int.clone(), long.clone()]));
    assert!(promotes(&[int.clone(), integer.clone()]));
    assert!(promotes(&[
        integer.clone(),
        Some(VarType::VARTYPE_LONG_OBJ())
    ]));
    assert!(promotes(&[
        boolean.clone(),
        Some(VarType::VARTYPE_BOOLEAN_OBJ())
    ]));
    assert!(promotes(&[
        int.clone(),
        Some(VarType::VARTYPE_CHAR()),
        long
    ]));
    assert!(!promotes(&[int.clone(), int.clone()]));
    assert!(!promotes(&[integer.clone(), integer]));
    assert!(!promotes(&[int.clone(), object]));
    assert!(!promotes(&[boolean, int.clone()]));
    assert!(!promotes(&[int, None]));
    assert!(!promotes(&[]));
}

#[test]
fn signature_is_split_at_its_parenthesis() {
    let split = |s: &str, name: &str, desc: &str| {
        assert_eq!(
            split_signature(s),
            Some((name.to_string(), desc.to_string()))
        );
    };
    split("mark-1pYyLzM(II)I", "mark-1pYyLzM", "(II)I");
    split("add(II)I", "add", "(II)I");
    split("<init>(I)V", "<init>", "(I)V");
    assert_eq!(split_signature("add"), None);
}

#[test]
fn names_kotlin_mangles_escapes_or_imports_by_default() {
    let _guard = common::install_default_context();
    assert!(mangled("take-dXQi75k", "take"));
    assert!(!mangled("bump$main", "bump"));
    assert!(!mangled("-deprecated_size", "size"));
    assert_eq!(java_property("getClosed").as_deref(), Some("closed"));
    assert_eq!(java_property("isEmpty").as_deref(), Some("isEmpty"));
    assert_eq!(java_property("get"), None);
    assert!(default_import("kotlin.collections"));
    assert!(default_import("kotlin"));
    assert!(!default_import("kotlin.math"));
    assert!(!default_import("okio"));
    assert_eq!(escape("-DeprecatedOkio"), "`-DeprecatedOkio`");
    assert_eq!(escape("in"), "`in`");
    assert_eq!(escape("Okio"), "Okio");
    assert_eq!(escape("Foo$Bar"), "Foo$Bar");
    assert_eq!(escape("<unrepresentable>"), "<unrepresentable>");
    assert_eq!(escape("kotlin/RequiresOptIn"), "kotlin/RequiresOptIn");
    assert_eq!(
        escape_qualified("pkg.Outer.make.1.Sub"),
        "pkg.Outer.make.1.Sub"
    );
    assert_eq!(
        escape_qualified("okio.-DeprecatedOkio"),
        "okio.`-DeprecatedOkio`"
    );
    assert_eq!(escape_qualified("Outer.in.Deep"), "Outer.`in`.Deep");
    assert_eq!(escape_qualified("java.util.List"), "java.util.List");
}

#[test]
fn kotlin_conversions_follow_java() {
    let _guard = common::install_default_context();
    let suffix = |to: CodeType, from: CodeType| {
        let value = VarExprent::new(1, Some(VarType::new(from)), None).0;
        match conversion(value, &VarType::new(to), &VarType::new(from)) {
            Some(KotlinCast::Convert(s)) => Some(s),
            _ => None,
        }
    };
    assert_eq!(suffix(CodeType::Int, CodeType::Byte), Some(".toInt()"));
    assert_eq!(suffix(CodeType::Int, CodeType::Char), Some(".code"));
    assert_eq!(suffix(CodeType::Long, CodeType::Short), Some(".toLong()"));
    assert_eq!(
        suffix(CodeType::Double, CodeType::Char),
        Some(".code.toDouble()")
    );
    assert_eq!(suffix(CodeType::Short, CodeType::Byte), Some(".toShort()"));
    assert_eq!(suffix(CodeType::Char, CodeType::Byte), None);
    assert_eq!(suffix(CodeType::Short, CodeType::Char), None);
    assert_eq!(suffix(CodeType::Int, CodeType::Long), None);
    assert_eq!(suffix(CodeType::Int, CodeType::Int), None);
    let constant = ConstExprent::new_int(7, false, None).0;
    let (long, int) = (VarType::new(CodeType::Long), VarType::new(CodeType::Int));
    assert!(conversion(constant, &long, &int).is_none());
    use CodeType::*;
    for (from, to, text) in [
        (Long, Int, ".toInt()"),
        (Char, Int, ".code"),
        (Char, Long, ".code.toLong()"),
        (Int, Char, ".toChar()"),
        (Long, Char, ".toInt().toChar()"),
        (Double, Byte, ".toInt().toByte()"),
        (Float, Short, ".toInt().toShort()"),
        (Long, Short, ".toShort()"),
        (Int, Double, ".toDouble()"),
    ] {
        assert_eq!(cast_conversion(from, to), text, "{from:?} -> {to:?}");
    }
}

#[test]
fn kotlin_text_forms() {
    let public_final_annotated = 0b0000_0111;
    let protected_open = (2 << 1) | (1 << 4);
    assert_eq!(accessor_flags(true, 0, public_final_annotated), 0);
    assert_eq!(
        accessor_flags(false, 0, public_final_annotated),
        0b0000_0111
    );
    assert_eq!(accessor_flags(false, 0, protected_open), protected_open);
    assert_eq!(
        accessor_flags(false, 0, protected_open | (1 << 6)),
        protected_open
    );

    assert_eq!(nullable("String"), "String?");
    assert_eq!(nullable("Node?"), "Node?");
    assert_eq!(nullable("MutableList<String>"), "MutableList<String>?");
    assert_eq!(nullable("(Int) -> Unit"), "((Int) -> Unit)?");

    assert_eq!(kotlin_text("$this$x", false), "\\$this\\$x");
    assert_eq!(kotlin_text("${a}", false), "\\${a}");
    assert_eq!(kotlin_text("a $ b $", false), "a $ b $");
    assert_eq!(kotlin_text("cost $5", false), "cost $5");
    assert_eq!(kotlin_text("page\\fbreak", false), "page\\u000cbreak");
    assert_eq!(kotlin_text("back\\\\fslash", false), "back\\\\fslash");
    assert_eq!(kotlin_text("a $ b", true), "a \\$ b");

    let obj = "Ljava/lang/Object;";
    let invoke = |owner: &str, name: &str, args: usize| {
        is_function_invoke(owner, name, &format!("({}){obj}", obj.repeat(args)))
    };
    assert!(invoke("kotlin/jvm/functions/Function1", "invoke", 1));
    assert!(invoke("kotlin/jvm/functions/Function3", "invoke", 3));
    assert!(!invoke("kotlin/jvm/functions/Function0", "invoke", 0));
    assert!(!invoke("kotlin/jvm/functions/Function2", "invoke", 1));
    assert!(!invoke("kotlin/jvm/functions/Function1", "apply", 1));
    assert!(!invoke("pkg/Function1", "invoke", 1));

    for (owner, name, desc, want) in [
        (
            "java/lang/String",
            "length",
            "()I",
            Form::Property("length"),
        ),
        ("java/lang/CharSequence", "charAt", "(I)C", Form::Index),
        ("java/util/ArrayList", "size", "()I", Form::Property("size")),
        (
            "java/util/HashMap",
            "entrySet",
            "()Ljava/util/Set;",
            Form::Property("entries"),
        ),
        (
            "java/lang/Number",
            "intValue",
            "()I",
            Form::Renamed("toInt"),
        ),
        (
            "java/util/List",
            "remove",
            "(I)Ljava/lang/Object;",
            Form::Renamed("removeAt"),
        ),
        (
            "pkg/Anything",
            "getClass",
            "()Ljava/lang/Class;",
            Form::Property("javaClass"),
        ),
    ] {
        assert_eq!(form(owner, name, desc), Some(want), "{owner}.{name}");
    }
}
