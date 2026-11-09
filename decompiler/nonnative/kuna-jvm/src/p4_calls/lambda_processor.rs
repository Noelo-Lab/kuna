//! Port of `org/jetbrains/java/decompiler/main/rels/LambdaProcessor.java`.
//!
//! kuna fix `self_referencing_lambda` (`kuna_fixes`): before the lambda
//! hierarchy is built, a lambda whose implementation method instantiates it
//! again is written as a method reference (`kuna_fix_self_referencing_lambda`),
//! where Vineflower nests it under itself and throws or overflows the stack.

use crate::classes_processor::{sort_nodes, ClassNode, ClassNodeType};
use crate::code_constants::OPC_INVOKEDYNAMIC;
use crate::decompiler_context::DecompilerContext;
use crate::interpreter_util::InterpreterUtil;
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::p4_calls::kuna_fix_self_referencing_lambda;
use crate::rt::{BitSet, JHashMap};
use crate::struct_general_attribute::StructGeneralAttribute;

#[derive(Default)]
pub struct LambdaProcessor;

impl LambdaProcessor {
    const JAVAC_LAMBDA_CLASS: &'static str = "java/lang/invoke/LambdaMetafactory";
    const JAVAC_LAMBDA_METHOD: &'static str = "metafactory";
    const JAVAC_LAMBDA_ALT_METHOD: &'static str = "altMetafactory";

    pub fn new() -> LambdaProcessor {
        LambdaProcessor
    }

    /// Turns each `invokedynamic` lambda of `node` (and its nested classes) into a
    /// lambda [`ClassNode`] registered in the root class map.
    pub fn process_class(&self, node: ClassNode) {
        let children = node.with(|d| d.nested.to_vec());
        for child in children {
            self.process_class(child);
        }

        let cl_processor = DecompilerContext::get_class_processor();
        let cl = node.with(|d| d.class_struct);

        if !cl.get_version().has_lambdas() {
            return;
        }

        let bootstrap = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_BOOTSTRAP_METHODS);
        let Some(bootstrap) = bootstrap.filter(|b| b.get_methods_number() != 0) else {
            return;
        };

        let mut lambda_methods = BitSet::new();

        for i in 0..bootstrap.get_methods_number() {
            let method_ref = bootstrap
                .get_method_reference(i)
                .expect("bootstrap.getMethodReference(i)");

            if method_ref.classname.as_deref() == Some(Self::JAVAC_LAMBDA_CLASS)
                && (method_ref.elementname == Self::JAVAC_LAMBDA_METHOD
                    || method_ref.elementname == Self::JAVAC_LAMBDA_ALT_METHOD)
            {
                lambda_methods.set(i as usize);
            }
        }

        if lambda_methods.is_empty() {
            return;
        }

        let mut map_methods_lambda: JHashMap<String, String> = JHashMap::new();

        let nested = node.with(|d| d.nested.clone());
        let methods = cl.get_methods().to_vec();
        for mt in methods {
            mt.expand_data(cl);

            let seq = mt.get_instruction_sequence();
            if let Some(seq) = seq {
                for instr in seq.iter() {
                    if instr.opcode() == OPC_INVOKEDYNAMIC {
                        let invoke_dynamic = cl
                            .get_pool()
                            .get_link_constant(instr.operand(0))
                            .expect("getLinkConstant");

                        if lambda_methods.get(invoke_dynamic.index1 as usize) {
                            let bootstrap_arguments =
                                bootstrap.get_method_arguments(invoke_dynamic.index1);
                            let md = MethodDescriptor::parse_descriptor(&invoke_dynamic.descriptor);

                            let lambda_class_name = md.ret.value_str().to_string();
                            let lambda_method_name = invoke_dynamic.elementname.clone();
                            let lambda_method_descriptor = bootstrap_arguments[2]
                                .as_ref()
                                .expect("bootstrap_arguments.get(2)")
                                .as_primitive()
                                .get_string()
                                .to_string();

                            let content_method_handle = bootstrap_arguments[1]
                                .as_ref()
                                .expect("bootstrap_arguments.get(1)")
                                .as_link();

                            let node_lambda = ClassNode::new_lambda(
                                content_method_handle
                                    .classname
                                    .as_deref()
                                    .expect("content_method_handle.classname"),
                                &content_method_handle.elementname,
                                &content_method_handle.descriptor,
                                content_method_handle.index1,
                                &lambda_class_name,
                                &lambda_method_name,
                                &lambda_method_descriptor,
                                cl,
                            );
                            let simple_name = format!(
                                "{}##Lambda_{}_{}",
                                cl.qualified_name(),
                                invoke_dynamic.index1,
                                invoke_dynamic.index2
                            );
                            let enclosing_method = InterpreterUtil::make_unique_key(
                                &mt.get_name(),
                                &mt.get_descriptor(),
                            );
                            node_lambda.with_mut(|d| {
                                d.simple_name = Some(simple_name.clone());
                                d.enclosing_method = Some(enclosing_method);
                            });

                            nested.add(node_lambda);
                            node_lambda.with_mut(|d| d.parent = Some(node));

                            cl_processor
                                .get_map_root_classes()
                                .borrow_mut()
                                .put(simple_name.clone(), node_lambda);
                            let (is_method_reference, content_method_key) = node_lambda.with(|d| {
                                let info =
                                    d.lambda_information.as_ref().expect("lambdaInformation");
                                (info.is_method_reference, info.content_method_key.clone())
                            });
                            if !is_method_reference {
                                map_methods_lambda.put(content_method_key, simple_name);
                            }
                        }
                    }
                }
            }

            mt.release_resources();
        }

        sort_nodes(&nested);

        if kuna_fixes::enabled(KunaFix::SelfReferencingLambda) {
            kuna_fix_self_referencing_lambda::write_recursive_lambdas_as_method_references(
                &nested,
                &mut map_methods_lambda,
            );
        }

        for nd in nested.to_vec() {
            let (type_, enclosing_method) = nd.with(|d| (d.type_, d.enclosing_method.clone()));
            if type_ == ClassNodeType::Lambda {
                let parent_class_name =
                    enclosing_method.and_then(|key| map_methods_lambda.get_cloned(&key));
                if let Some(parent_class_name) = parent_class_name {
                    let parent_class = cl_processor
                        .get_map_root_classes()
                        .borrow()
                        .get_cloned(&parent_class_name)
                        .expect("getMapRootClasses().get(parent_class_name)");

                    if nd == parent_class {
                        let qualified_name = nd.with(|d| d.class_struct).qualified_name();
                        jthrow!(IllegalStateException; "Lambda {} is recursive?!", qualified_name);
                    }
                    parent_class.with(|d| d.nested.clone()).add(nd);
                    nd.with_mut(|d| d.parent = Some(parent_class));
                }
            }
        }
    }
}
