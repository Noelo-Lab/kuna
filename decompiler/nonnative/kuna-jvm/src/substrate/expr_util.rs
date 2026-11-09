//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ExprUtil.java`.

use std::rc::Rc;

use crate::classes_processor::ClassNode;
use crate::rt::JHashMap;
use crate::struct_class::StructClass;
use crate::unported;
use crate::var_version_pair::VarVersionPair;

thread_local! {
    static PRIMITIVE_TYPES: Rc<JHashMap<String, String>> = Rc::new(JHashMap::from_entries(
        [
            ("java/lang/Boolean", "boolean"),
            ("java/lang/Byte", "byte"),
            ("java/lang/Character", "char"),
            ("java/lang/Short", "short"),
            ("java/lang/Integer", "int"),
            ("java/lang/Long", "long"),
            ("java/lang/Float", "float"),
            ("java/lang/Double", "double"),
            ("java/lang/Void", "void"),
        ]
        .map(|(k, v)| (k.to_string(), v.to_string())),
    ));
}

pub struct ExprUtil;

impl ExprUtil {
    /// Java `ExprUtil.PRIMITIVE_TYPES` (an immutable map; only looked up).
    #[allow(non_snake_case)]
    pub fn PRIMITIVE_TYPES() -> Rc<JHashMap<String, String>> {
        PRIMITIVE_TYPES.with(Rc::clone)
    }

    /// Java: `getSyntheticParametersMask(String, String, int)`.
    pub fn get_synthetic_parameters_mask_name(
        _class_name: &str,
        _descriptor: &str,
        _parameters: i32,
    ) -> Option<Vec<Option<VarVersionPair>>> {
        unported!("ExprUtil.getSyntheticParametersMask(String, String, int)")
    }

    /// Java: `getSyntheticParametersMask(ClassNode, String, int)`.
    pub fn get_synthetic_parameters_mask(
        _node: ClassNode,
        _descriptor: &str,
        _parameters: i32,
    ) -> Option<Vec<Option<VarVersionPair>>> {
        unported!("ExprUtil.getSyntheticParametersMask(ClassNode, String, int)")
    }

    #[allow(dead_code)]
    fn is_static(_struct_: StructClass) -> bool {
        unported!("ExprUtil.isStatic")
    }
}
