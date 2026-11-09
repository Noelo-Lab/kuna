//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ExprUtil.java`.

use std::rc::Rc;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_STATIC, INIT_NAME};
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::jthrow;
use crate::rt::JHashMap;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
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
        class_name: &str,
        descriptor: &str,
        parameters: i32,
    ) -> Option<Vec<Option<VarVersionPair>>> {
        let node = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get_cloned(&class_name.to_string());
        match node {
            Some(node) => ExprUtil::get_synthetic_parameters_mask(node, descriptor, parameters),
            None => None,
        }
    }

    /// Java: `getSyntheticParametersMask(ClassNode, String, int)`. The mask is
    /// a copy of the constructor's `synthParameters` (callers only read it).
    pub fn get_synthetic_parameters_mask(
        node: ClassNode,
        descriptor: &str,
        parameters: i32,
    ) -> Option<Vec<Option<VarVersionPair>>> {
        let mut mask: Option<Vec<Option<VarVersionPair>>> = None;

        let wrapper = node.get_wrapper();
        if let Some(wrapper) = wrapper {
            let method_wrapper = wrapper.get_method_wrapper(INIT_NAME, descriptor);
            let Some(method_wrapper) = method_wrapper else {
                if DecompilerContext::get_option(IFernflowerPreferences::IGNORE_INVALID_BYTECODE) {
                    return None;
                }
                let qualified_name = node.with(|d| d.class_struct).qualified_name();
                jthrow!(
                    "Constructor {}.{}{} not found",
                    qualified_name,
                    INIT_NAME,
                    descriptor
                );
            };
            mask = method_wrapper.with(|d| d.synth_parameters.clone());
        } else {
            let (type_, class_struct) = node.with(|d| (d.type_, d.class_struct));
            if parameters > 0
                && type_ == ClassNodeType::Member
                && !ExprUtil::is_static(class_struct)
            {
                let mut non_static = vec![None; parameters as usize];
                non_static[0] = Some(VarVersionPair::new(-1, 0));
                mask = Some(non_static);
            }
        }

        mask
    }

    fn is_static(struct_: StructClass) -> bool {
        if struct_.has_modifier(ACC_STATIC) {
            return true;
        }
        if struct_.has_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES) {
            let attr = struct_
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)
                .expect("InnerClasses attribute");
            let qualified_name = struct_.qualified_name();
            for entry in attr.get_entries() {
                if entry.inner_name == qualified_name {
                    return (entry.access_flags & ACC_STATIC) == ACC_STATIC;
                }
            }
        }
        false
    }
}
