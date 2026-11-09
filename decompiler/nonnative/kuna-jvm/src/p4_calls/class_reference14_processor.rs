//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ClassReference14Processor.java`.

use std::cell::Cell;

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationFuncType};
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::{JHashMap, JHashSet};
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

thread_local! {
    static STATICS: Cell<Option<(ExitExprent, ExitExprent)>> = const { Cell::new(None) };
    static CLASS_INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

pub struct ClassReference14Processor;

impl ClassReference14Processor {
    /// The `return Class.forName(var0);` body pattern of a synthetic `class$`.
    #[allow(non_snake_case)]
    fn BODY_EXPR() -> ExitExprent {
        Self::clinit().0
    }

    /// The `throw new NoClassDefFoundError().initCause(var2);` handler pattern.
    #[allow(non_snake_case)]
    fn HANDLER_EXPR() -> ExitExprent {
        Self::clinit().1
    }

    /// Java's static initializer: it runs once, on the first `processClassReferences`,
    /// and takes its expression ids from that class's counter. The exprents are
    /// class-scope, so later rebuilds hand their ids back to keep the counter as Java has it.
    fn clinit() -> (ExitExprent, ExitExprent) {
        if let Some((body, handler)) = STATICS.with(Cell::get) {
            if body.is_live() && handler.is_live() {
                return (body, handler);
            }
        }
        let rebuild = CLASS_INITIALIZED.with(|c| c.replace(true));
        let counter = DecompilerContext::get_counter_container();
        let next_id = counter.get_counter(CounterContainer::EXPRESSION_COUNTER);
        let statics = Self::build_statics();
        if rebuild {
            counter.set_counter(CounterContainer::EXPRESSION_COUNTER, next_id);
        }
        STATICS.with(|s| s.set(Some(statics)));
        statics
    }

    fn build_statics() -> (ExitExprent, ExitExprent) {
        let inv_for = InvocationExprent::new();
        inv_for.set_name("forName".to_string());
        inv_for.set_classname("java/lang/Class".to_string());
        inv_for.set_string_descriptor("(Ljava/lang/String;)Ljava/lang/Class;".to_string());
        inv_for.set_descriptor(MethodDescriptor::parse_descriptor(
            "(Ljava/lang/String;)Ljava/lang/Class;",
        ));
        inv_for.set_static(true);
        inv_for.set_lst_parameters(vec![
            VarExprent::new(0, Some(VarType::VARTYPE_STRING()), None).0,
        ]);
        let body_expr = ExitExprent::new(
            ExitType::Return,
            Some(inv_for.0),
            Some(VarType::VARTYPE_CLASS()),
            None,
            None,
        );

        let ctor = InvocationExprent::new();
        ctor.set_name(CodeConstants::INIT_NAME.to_string());
        ctor.set_classname("java/lang/NoClassDefFoundError".to_string());
        ctor.set_string_descriptor("()V".to_string());
        ctor.set_functype(InvocationFuncType::Init);
        ctor.set_descriptor(MethodDescriptor::parse_descriptor("()V"));
        let new_expr = NewExprent::new(
            VarType::new_value(CodeType::Object, 0, "java/lang/NoClassDefFoundError"),
            Vec::<Exprent>::new(),
            None,
        );
        new_expr.set_constructor(Some(ctor));
        let inv_cause = InvocationExprent::new();
        inv_cause.set_name("initCause".to_string());
        inv_cause.set_classname("java/lang/NoClassDefFoundError".to_string());
        inv_cause.set_string_descriptor("(Ljava/lang/Throwable;)Ljava/lang/Throwable;".to_string());
        inv_cause.set_descriptor(MethodDescriptor::parse_descriptor(
            "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
        ));
        inv_cause.set_instance(Some(new_expr.0));
        inv_cause.set_lst_parameters(vec![
            VarExprent::new(
                2,
                Some(VarType::new_value(
                    CodeType::Object,
                    0,
                    "java/lang/ClassNotFoundException",
                )),
                None,
            )
            .0,
        ]);
        let handler_expr = ExitExprent::new(ExitType::Throw, Some(inv_cause.0), None, None, None);

        (body_expr, handler_expr)
    }

    pub fn process_class_references(node: ClassNode) {
        Self::clinit();

        let mut map_class_meths: JHashMap<ClassWrapper, MethodWrapper> = JHashMap::new();
        Self::map_class_methods(node, &mut map_class_meths);
        if map_class_meths.is_empty() {
            return;
        }

        let mut set_found: JHashSet<ClassWrapper> = JHashSet::new();
        Self::process_class_rec(node, &map_class_meths, &mut set_found);

        if !set_found.is_empty() {
            for wrp in set_found.to_vec() {
                let mt = map_class_meths
                    .get(&wrp)
                    .expect("NullPointerException: mapClassMeths.get(wrp)")
                    .with(|d| d.method_struct);
                wrp.get_hidden_members()
                    .borrow_mut()
                    .add(InterpreterUtil::make_unique_key(
                        &mt.get_name(),
                        &mt.get_descriptor(),
                    ));
            }
        }
    }

    fn process_class_rec(
        node: ClassNode,
        map_class_meths: &JHashMap<ClassWrapper, MethodWrapper>,
        set_found: &mut JHashSet<ClassWrapper>,
    ) {
        let wrapper = node
            .get_wrapper()
            .expect("NullPointerException: node.getWrapper()");

        let methods = wrapper.get_methods().borrow().to_vec();
        for meth in methods {
            let root = meth.with(|d| d.root);
            if root.is_some() {
                let graph = meth
                    .get_or_build_graph()
                    .expect("NullPointerException: meth.getOrBuildGraph()");
                let entries = map_class_meths.entry_vec();
                graph.iterate_exprents(|exprent: Exprent| {
                    for &(key, value) in &entries {
                        if Self::replace_invocations(exprent, key, value) {
                            set_found.add(key);
                        }
                    }
                    0
                });
            }
        }

        for j in 0..2 {
            let initializers = if j == 0 {
                wrapper.get_static_field_initializers()
            } else {
                wrapper.get_dynamic_field_initializers()
            };

            let mut i = 0;
            while i < initializers.borrow().size() {
                for (key, value) in map_class_meths.entry_vec() {
                    let exprent = initializers.borrow().get(i);
                    if Self::replace_invocations(exprent, key, value) {
                        set_found.add(key);
                    }

                    let cl = Self::is_class14_invocation(exprent, key, value);
                    if let Some(cl) = cl {
                        let bytecode = exprent.get_bytecode();
                        let class_const = ConstExprent::new(
                            VarType::VARTYPE_CLASS(),
                            Some(ConstValue::String(cl.replace('.', "/"))),
                            bytecode.as_ref(),
                        );
                        initializers.borrow_mut().set(i, class_const.0);
                        set_found.add(key);
                    }
                }
                i += 1;
            }
        }

        let nested = node.with(|d| d.nested.to_vec());
        for nd in nested {
            Self::process_class_rec(nd, map_class_meths, set_found);
        }
    }

    fn map_class_methods(node: ClassNode, map: &mut JHashMap<ClassWrapper, MethodWrapper>) {
        let no_synth_flag =
            DecompilerContext::get_option(IFernflowerPreferences::SYNTHETIC_NOT_SET);

        let wrapper = node
            .get_wrapper()
            .expect("NullPointerException: node.getWrapper()");

        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let mt = method.with(|d| d.method_struct);

            if (no_synth_flag || mt.is_synthetic())
                && mt.get_descriptor() == "(Ljava/lang/String;)Ljava/lang/Class;"
                && mt.has_modifier(CodeConstants::ACC_STATIC)
            {
                let root = method.with(|d| d.root);
                let Some(cst) = root
                    .and_then(|root| root.get_first())
                    .and_then(|first| first.try_as_catch())
                else {
                    continue;
                };
                let stats = cst.get_stats();
                if stats.borrow().size() == 2
                    && cst.get_first().is_some_and(|first| first.is_basic_block())
                    && stats.borrow().get(1).is_basic_block()
                    && cst
                        .get_vars()
                        .get(0)
                        .get_var_type()
                        .equals(&VarType::new_value(
                            CodeType::Object,
                            0,
                            "java/lang/ClassNotFoundException",
                        ))
                {
                    let body = cst
                        .get_first()
                        .expect("NullPointerException: cst.getFirst()")
                        .as_basic_block();
                    let handler = stats.borrow().get(1).as_basic_block();

                    let body_exprents = body
                        .get_exprents()
                        .expect("NullPointerException: body.getExprents()");
                    let handler_exprents = handler
                        .get_exprents()
                        .expect("NullPointerException: handler.getExprents()");
                    if body_exprents.len() == 1
                        && handler_exprents.len() == 1
                        && Self::BODY_EXPR().equals(body_exprents.get(0))
                        && Self::HANDLER_EXPR().equals(handler_exprents.get(0))
                    {
                        map.put(wrapper, method);
                        break;
                    }
                }
            }
        }

        let nested = node.with(|d| d.nested.to_vec());
        for nd in nested {
            Self::map_class_methods(nd, map);
        }
    }

    fn replace_invocations(exprent: Exprent, wrapper: ClassWrapper, meth: MethodWrapper) -> bool {
        let mut res = false;

        loop {
            let mut found = false;

            for expr in exprent.get_all_exprents() {
                let cl = Self::is_class14_invocation(expr, wrapper, meth);
                if let Some(cl) = cl {
                    let bytecode = expr.get_bytecode();
                    let class_const = ConstExprent::new(
                        VarType::VARTYPE_CLASS(),
                        Some(ConstValue::String(cl.replace('.', "/"))),
                        bytecode.as_ref(),
                    );
                    exprent.replace_exprent(expr, class_const.0);
                    found = true;
                    res = true;
                    break;
                }

                res |= Self::replace_invocations(expr, wrapper, meth);
            }

            if !found {
                break;
            }
        }

        res
    }

    fn is_class14_invocation(
        exprent: Exprent,
        wrapper: ClassWrapper,
        meth: MethodWrapper,
    ) -> Option<String> {
        let fexpr = exprent.try_as_function()?;
        if fexpr.get_func_type() != FunctionType::Ternary {
            return None;
        }
        let operands = fexpr.get_lst_operands();
        let headexpr = operands.get(0).try_as_function()?;
        if headexpr.get_func_type() != FunctionType::Eq {
            return None;
        }
        let head_operands = headexpr.get_lst_operands();
        if !(head_operands.get(0).is_field()
            && head_operands
                .get(1)
                .try_as_const()
                .is_some_and(|c| c.get_const_type().equals(&VarType::VARTYPE_NULL())))
        {
            return None;
        }

        let field = head_operands.get(0).as_field();
        let fieldnode = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get_cloned(&field.get_classname());

        let class_struct = wrapper.get_class_struct();
        let fieldnode = fieldnode?;
        if fieldnode.with(|d| d.class_struct).qualified_name() != class_struct.qualified_name() {
            return None;
        }
        let fd =
            class_struct.get_field(&field.get_name(), &field.get_descriptor().descriptor_string)?;

        if !(fd.has_modifier(CodeConstants::ACC_STATIC)
            && (fd.is_synthetic()
                || DecompilerContext::get_option(IFernflowerPreferences::SYNTHETIC_NOT_SET)))
        {
            return None;
        }

        if !(operands.get(1).is_assignment() && operands.get(2).equals(field.0)) {
            return None;
        }
        let asexpr = operands.get(1).as_assignment();

        if !(asexpr.get_left().equals(field.0) && asexpr.get_right().is_invocation()) {
            return None;
        }
        let invexpr = asexpr.get_right().as_invocation();

        let meth_struct = meth.with(|d| d.method_struct);
        if !(invexpr.get_classname() == class_struct.qualified_name()
            && invexpr.get_name() == meth_struct.get_name()
            && invexpr.get_string_descriptor() == meth_struct.get_descriptor())
        {
            return None;
        }

        let param = invexpr.get_lst_parameters().get(0).try_as_const()?;
        wrapper
            .get_hidden_members()
            .borrow_mut()
            .add(InterpreterUtil::make_unique_key(
                &fd.get_name(),
                &fd.get_descriptor(),
            ));
        Some(
            param
                .get_value()
                .expect("NullPointerException: getValue()")
                .java_to_string(),
        )
    }
}
