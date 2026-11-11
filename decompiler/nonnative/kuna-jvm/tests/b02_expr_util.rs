//! `ExprUtil.getSyntheticParametersMask` on class nodes without a wrapper:
//! a non-static member class masks its outer-instance parameter, a static one
//! (static by its own `InnerClasses` entry) and a root class get none, and an
//! unknown class name yields `null`.

use std::path::Path;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::expr_util::ExprUtil;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::var_version_pair::VarVersionPair;

use crate::common;

fn load(name: &str) -> StructClass {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/b02/b02")
        .join(name);
    let bytes = std::fs::read(&path).expect("fixture class");
    StructClass::create(&mut ByteCursor::new(bytes), true)
}

#[test]
fn synthetic_parameters_mask() {
    let _guard = common::install_default_context();

    let inner = ClassNode::new(ClassNodeType::Member, load("B02Fields$Inner.class"));
    assert_eq!(
        ExprUtil::get_synthetic_parameters_mask(inner, "(Lb02/B02Fields;)V", 3),
        Some(vec![Some(VarVersionPair::new(-1, 0)), None, None])
    );
    assert_eq!(
        ExprUtil::get_synthetic_parameters_mask(inner, "()V", 0),
        None
    );

    let nested = ClassNode::new(ClassNodeType::Member, load("B02Fields$Nested.class"));
    assert_eq!(
        ExprUtil::get_synthetic_parameters_mask(nested, "()V", 2),
        None
    );

    let root = ClassNode::new(ClassNodeType::Root, load("B02Fields.class"));
    assert_eq!(
        ExprUtil::get_synthetic_parameters_mask(root, "()V", 2),
        None
    );

    assert_eq!(
        ExprUtil::get_synthetic_parameters_mask_name("does/not/Exist", "()V", 1),
        None
    );
}
