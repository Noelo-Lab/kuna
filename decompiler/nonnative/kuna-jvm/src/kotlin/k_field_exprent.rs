//! Port of `org/vineflower/kotlin/expr/KFieldExprent.java`.

use crate::classes_processor::ClassNode;
use crate::decompiler_context::DecompilerContext;
use crate::expr_util::ExprUtil;
use crate::field_exprent::FieldExprent;
use crate::interpreter_util::InterpreterUtil;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_element::KElement;
use crate::k_types::KTypes;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_exprent;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::method_wrapper::MethodWrapper;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

kotlin_exprent!(pub KFieldExprent: FieldExprent, Field,
                is_k_field, as_k_field, try_as_k_field);

fn npe(what: &str) -> ! {
    jthrow!(NullPointerException; "{}", what)
}

/// `DecompilerContext.getClassProcessor().getMapRootClasses().get(name)`.
fn root_class_node(name: &str) -> Option<ClassNode> {
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

/// The `finally` that puts the outer method wrapper back.
struct RestoreMethodWrapper(Option<MethodWrapper>);

impl Drop for RestoreMethodWrapper {
    fn drop(&mut self) {
        if DecompilerContext::get_current_context().is_some() {
            DecompilerContext::set_current_method_wrapper(self.0);
        }
    }
}

impl KFieldExprent {
    /// Java: KFieldExprent(FieldExprent)
    pub fn new(field: FieldExprent) -> KFieldExprent {
        let bytecode = field.get_bytecode();
        let kfield = FieldExprent::new(
            field.get_name(),
            field.get_classname(),
            field.is_static(),
            field.get_instance(),
            field.get_descriptor(),
            bytecode.as_ref(),
        );
        kfield.0.set_kt(Some(KExprentExt::Field));
        KFieldExprent(kfield)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let name = self.get_name();
        let classname = self.get_classname();

        if name == "TYPE" && ExprUtil::PRIMITIVE_TYPES().contains_key(&classname) {
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            let type_ = VarType::new_signature_cl(&classname, true);
            buf.append(&KTypes::get_kotlin_type(Some(&type_)));
            buf.append("::class.javaPrimitiveType");
            return buf;
        }
        if name == "INSTANCE" {
            let Some(cl) = DecompilerContext::get_struct_context().get_class(&classname) else {
                return FieldExprent::to_java(self.0, indent);
            };

            KotlinChooser::parse_metadata_for(cl);

            let Some(kt_data) = KtAttrs::get(cl, &KElement::KEY()) else {
                return FieldExprent::to_java(self.0, indent);
            };

            match kt_data {
                KElement::Class(cls) => {
                    if cls.proto.has_companion_object_name() {
                        let name = match &cls.resolver {
                            None => cl.qualified_name(),
                            Some(resolver) => {
                                resolver.resolve(cls.proto.get_companion_object_name())
                            }
                        };
                        let short_name = DecompilerContext::get_import_collector()
                            .get_short_name(&cl.qualified_name())
                            .unwrap_or_else(|| npe("ImportCollector.getShortName()"));
                        buf.append_class(&short_name, false, &name);
                        buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                        return buf;
                    }
                }
                KElement::Function(function) => {
                    let wrapper = root_class_node(&classname)
                        .unwrap_or_else(|| npe("ClassNode"))
                        .get_wrapper()
                        .unwrap_or_else(|| npe("ClassNode.getWrapper()"));
                    let method = (function.method_supplier)(wrapper);

                    buf.append("{");

                    let mut index = 0;
                    for param in function.parameters.iter() {
                        if index > 0 {
                            buf.append(", ");
                        } else {
                            buf.append(" ");
                        }
                        let class_struct = function
                            .class_struct
                            .unwrap_or_else(|| npe("KFunction.classStruct()"));
                        let method_struct = function
                            .method_struct
                            .unwrap_or_else(|| npe("KFunction.methodStruct()"));
                        buf.append_variable(
                            &param.name,
                            true,
                            true,
                            &class_struct.qualified_name(),
                            &method_struct.get_name(),
                            &method_struct.get_descriptor(),
                            index,
                            &param.name,
                        );
                        index += param.type_.stack_size;
                    }

                    if !function.parameters.is_empty() {
                        buf.append(" ->");
                    }

                    buf.append_line_separator();

                    let method_or_npe = || method.unwrap_or_else(|| npe("method wrapper"));
                    let outer_method = DecompilerContext::get_current_method_wrapper();
                    {
                        let _finally = RestoreMethodWrapper(outer_method);
                        let method = method_or_npe();
                        if method.with(|d| d.decompile_error.is_none()) {
                            let root = method.with(|d| d.root);
                            if let Some(root) = root {
                                DecompilerContext::set_current_method_wrapper(Some(method));
                                let mut body = root.0.to_java(indent + 1);
                                body.add_bytecode_mapping(
                                    root.get_dummy_exit().bytecode().as_ref(),
                                );
                                let method_struct = method.with(|d| d.method_struct);
                                buf.append_buffer_mapped(
                                    &body,
                                    Some(&cl.qualified_name()),
                                    Some(&InterpreterUtil::make_unique_key(
                                        &method_struct.get_name(),
                                        &method_struct.get_descriptor(),
                                    )),
                                );
                            }
                        }
                    }

                    let method = method_or_npe();
                    if method.with(|d| d.decompile_error.is_some()) {
                        KotlinWriter::dump_error(&mut buf, method, indent + 1);
                    }

                    buf.append_indent(indent).append("}");
                    buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                    return buf;
                }
                KElement::FunctionReference(_) => {
                    let node = root_class_node(&classname).unwrap_or_else(|| npe("ClassNode"));
                    let buffer = KotlinWriter::stringify_reference(
                        indent,
                        node,
                        self.get_bytecode().as_ref(),
                        None,
                    );
                    if let Some(buffer) = buffer {
                        return buffer;
                    }
                }
                _ => {}
            }
        }
        FieldExprent::to_java(self.0, indent)
    }
}
