//! Port of `org/jetbrains/java/decompiler/modules/decompiler/EnumProcessor.java`.

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::INIT_NAME;
use crate::interpreter_util::InterpreterUtil;
use crate::statements::Statements;

pub struct EnumProcessor;

impl EnumProcessor {
    pub fn clear_enum(wrapper: ClassWrapper) {
        let cl = wrapper.get_class_struct();
        let qualified_name = cl.with(|d| d.qualified_name.clone());

        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let mt = method.with(|d| d.method_struct);
            let name = mt.get_name();
            let descriptor = mt.get_descriptor();

            if name == "values" {
                if descriptor == format!("()[L{qualified_name};") {
                    wrapper
                        .get_hidden_members()
                        .borrow_mut()
                        .add(InterpreterUtil::make_unique_key(&name, &descriptor));
                }
            } else if name == "valueOf" {
                if descriptor == format!("(Ljava/lang/String;)L{qualified_name};") {
                    wrapper
                        .get_hidden_members()
                        .borrow_mut()
                        .add(InterpreterUtil::make_unique_key(&name, &descriptor));
                }
            } else if name == INIT_NAME {
                let root = method.with(|d| d.root).expect("MethodWrapper.root");
                if let Some(first_data) = Statements::find_first_data(root.0) {
                    let exprents = first_data.get_exprents().expect("first data exprents");
                    if !exprents.is_empty() {
                        if let Some(inv_expr) = exprents.get(0).try_as_invocation() {
                            if Statements::is_invocation_init_constructor(
                                inv_expr, method, wrapper, false,
                            ) {
                                exprents.remove_at(0);
                            }
                        }
                    }
                }
            }
        }

        for fd in cl.get_fields().to_vec() {
            let descriptor = fd.get_descriptor();
            if fd.is_synthetic() && descriptor == format!("[L{qualified_name};") {
                wrapper
                    .get_hidden_members()
                    .borrow_mut()
                    .add(InterpreterUtil::make_unique_key(
                        &fd.get_name(),
                        &descriptor,
                    ));
            }
        }
    }
}
