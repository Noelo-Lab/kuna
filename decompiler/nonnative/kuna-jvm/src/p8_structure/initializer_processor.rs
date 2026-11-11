//! Port of `org/jetbrains/java/decompiler/modules/decompiler/InitializerProcessor.java`.
//!
//! kuna fix `kt_hidden_empty_super_call`: `hide_empty_super` keeps the call
//! it removes on the method, for the Kotlin writer.
//!
//! kuna fix `enum_constant_in_values_array`: `extract_static_initializers`
//! first takes enum constants built inside the `$VALUES` array
//! (`kuna_fix_enum_constant_in_values_array`).

use crate::assignment_exprent::AssignmentExprent;
use crate::cancelation_manager::CanceledException;
use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{
    ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_STATIC, ACC_SYNTHETIC, CLINIT_NAME, INIT_NAME,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_util::ExprUtil;
use crate::exprent::{Exprent, ExprentType};
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_enum_constant_in_values_array;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::{rethrow, try_catch, JHashMap, JHashSet, JList, JavaListExt};
use crate::statements::Statements;
use crate::struct_class::StructClass;
use crate::var_version_pair::VarVersionPair;

pub struct InitializerProcessor;

fn root_class_node(name: Option<&str>) -> Option<ClassNode> {
    let map = DecompilerContext::get_class_processor().get_map_root_classes();
    let node = name.and_then(|n| map.borrow().get_cloned(&n.to_string()));
    node
}

impl InitializerProcessor {
    pub fn extract_initializers(wrapper: ClassWrapper) {
        let method = wrapper.get_method_wrapper(CLINIT_NAME, "()V");
        let result = try_catch(|| {
            if let Some(method) = method.filter(|m| m.with(|d| d.root.is_some())) {
                Self::extract_static_initializers(wrapper, method);
            }
        });
        if let Err(t) = result {
            if CanceledException::is(&t) {
                rethrow(t);
            }
            let method = method.expect("method");
            let mt = method.with(|d| d.method_struct);
            let message = format!(
                "Method {} {} in class {} couldn't be written.",
                mt.get_name(),
                mt.get_descriptor(),
                wrapper.get_class_struct().qualified_name()
            );
            DecompilerContext::get_logger().write_message_ex(&message, Severity::Warn, &t);

            method.with_mut(|d| d.decompile_error = Some(t));
        }

        Self::extract_dynamic_initializers(wrapper);

        Self::lift_constructor(wrapper);

        if DecompilerContext::get_option(IFernflowerPreferences::HIDE_EMPTY_SUPER) {
            Self::hide_empty_super(wrapper);
        }
    }

    fn lift_constructor(wrapper: ClassWrapper) {
        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let (method_struct, root) = method.with(|d| (d.method_struct, d.root));
            let Some(root) = root.filter(|_| INIT_NAME == method_struct.get_name()) else {
                continue;
            };
            let Some(first_data) = Statements::find_first_data(root.0) else {
                return;
            };

            let mut index = 0;
            let lst_exprents = first_data.get_exprents().expect("exprents");

            for exprent in lst_exprents.iter() {
                let mut action = 0;

                if let Some(assign_expr) = exprent.try_as_assignment() {
                    if assign_expr.get_left().is_field() && assign_expr.get_right().is_var() {
                        let f_expr = assign_expr.get_left().as_field();
                        if f_expr.get_classname() == wrapper.get_class_struct().qualified_name() {
                            let struct_field = wrapper.get_class_struct().get_field(
                                &f_expr.get_name(),
                                &f_expr.get_descriptor().descriptor_string,
                            );
                            if struct_field.is_some_and(|f| f.has_modifier(ACC_FINAL)) {
                                action = 1;
                            }
                        }
                    }
                } else if index > 0
                    && exprent.is_invocation()
                    && Statements::is_invocation_init_constructor(
                        exprent.as_invocation(),
                        method,
                        wrapper,
                        true,
                    )
                {
                    let removed = lst_exprents.remove_at(index);
                    lst_exprents.add_at(0, removed);
                    action = 2;
                }

                if action != 1 {
                    break;
                }

                index += 1;
            }
        }
    }

    fn hide_empty_super(wrapper: ClassWrapper) {
        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let (method_struct, root) = method.with(|d| (d.method_struct, d.root));
            let Some(root) = root.filter(|_| INIT_NAME == method_struct.get_name()) else {
                continue;
            };
            let first_data = root.get_basichead();
            let first_exprents = first_data.get_exprents().expect("exprents");
            if first_exprents.is_empty() {
                return;
            }

            let exprent = first_exprents.get(0);
            if let Some(inv_expr) = exprent.try_as_invocation() {
                if Statements::is_invocation_init_constructor(inv_expr, method, wrapper, false) {
                    let mask = ExprUtil::get_synthetic_parameters_mask_name(
                        &inv_expr.get_classname(),
                        &inv_expr.get_string_descriptor(),
                        inv_expr.get_lst_parameters().len() as i32,
                    );
                    let mut hide_super = true;

                    let params = inv_expr.get_descriptor().params.clone();
                    for (i, type_) in params.iter().enumerate() {
                        if mask.as_ref().is_some_and(|m| m[i].is_some()) {
                            continue;
                        }
                        if type_.type_ == CodeType::Object {
                            let node = root_class_node(type_.value.as_deref());
                            if node.is_some_and(|n| {
                                n.with(|d| {
                                    d.type_ == ClassNodeType::Anonymous
                                        || (d.access & ACC_SYNTHETIC) != 0
                                })
                            }) {
                                break;
                            }
                        }
                        hide_super = false;
                        break;
                    }

                    if hide_super {
                        first_exprents.remove_at(0);
                        if kuna_fixes::enabled(KunaFix::KtHiddenEmptySuperCall) {
                            method.with_mut(|d| d.hidden_super = Some(inv_expr));
                        }
                    }
                }
            }
        }
    }

    pub fn hide_initalizers(wrapper: ClassWrapper) {
        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let mt = method.with(|d| d.method_struct);
            let name = mt.get_name();
            let desc = mt.get_descriptor();

            if mt.is_synthetic() && INIT_NAME == name {
                let md = MethodDescriptor::parse_descriptor(&desc);
                if !md.params.is_empty() {
                    let type_ = &md.params[md.params.len() - 1];
                    if type_.type_ == CodeType::Object {
                        let node = root_class_node(type_.value.as_deref());
                        if node.is_some_and(|n| {
                            n.with(|d| {
                                d.type_ == ClassNodeType::Anonymous
                                    || (d.access & ACC_SYNTHETIC) != 0
                            })
                        }) {
                            wrapper
                                .get_hidden_members()
                                .borrow_mut()
                                .add(InterpreterUtil::make_unique_key(&name, &desc));
                        }
                    }
                }
            }
        }
    }

    fn extract_static_initializers(wrapper: ClassWrapper, method: MethodWrapper) {
        let root = method.with(|d| d.root).expect("root");
        let cl = wrapper.get_class_struct();
        let mut whitelist: JHashSet<String> = JHashSet::new();

        if let Some(first_data) = Statements::find_first_data(root.0) {
            let inline_initializers = cl.has_modifier(ACC_INTERFACE) || cl.has_modifier(ACC_ENUM);
            let mut exprents_to_remove: Vec<Exprent> = Vec::new();
            let mut non_field_assigns: JHashMap<i32, AssignmentExprent> = JHashMap::new();

            let mut seen: Vec<String> = Vec::new();
            let mut multi_assign: Vec<String> = Vec::new();

            let first_exprents = first_data.get_exprents().expect("exprents");
            for exprent in first_exprents.iter() {
                if let Some(assign_expr) = exprent.try_as_assignment() {
                    if let Some(f_expr) = assign_expr.get_left().try_as_field() {
                        let name = f_expr.get_name();
                        if seen.contains(&name) {
                            if !multi_assign.contains(&name) {
                                multi_assign.push(name);
                            }
                        } else {
                            seen.push(name);
                        }
                    }
                }
            }

            if kuna_fixes::enabled(KunaFix::EnumConstantInValuesArray) {
                kuna_fix_enum_constant_in_values_array::extract(
                    wrapper,
                    cl,
                    &first_exprents,
                    &mut whitelist,
                    &multi_assign,
                    |field, value, whitelist, fidx| {
                        Self::is_exprent_independent(
                            field,
                            value,
                            method,
                            cl,
                            whitelist,
                            &multi_assign,
                            &[],
                            fidx,
                            true,
                        )
                    },
                );
            }

            let mut not_inlined: Vec<FieldExprent> = Vec::new();

            let mut itr = 0;
            while itr < first_exprents.len() {
                let exprent = first_exprents.get(itr);
                let mut remove = false;

                if let Some(assign_expr) = exprent.try_as_assignment() {
                    if let Some(f_expr) = assign_expr.get_left().try_as_field() {
                        if f_expr.is_static()
                            && f_expr.get_classname() == cl.qualified_name()
                            && cl.has_field(
                                &f_expr.get_name(),
                                &f_expr.get_descriptor().descriptor_string,
                            )
                        {
                            let key_field = InterpreterUtil::make_unique_key(
                                &f_expr.get_name(),
                                &f_expr.get_descriptor().descriptor_string,
                            );
                            let exprent_independent = Self::is_exprent_independent(
                                f_expr,
                                assign_expr.get_right(),
                                method,
                                cl,
                                &whitelist,
                                &multi_assign,
                                &not_inlined,
                                cl.get_fields().get_index_by_key(&key_field) as i32,
                                true,
                            );
                            if inline_initializers || exprent_independent {
                                let initializers = wrapper.get_static_field_initializers();
                                if !initializers.borrow().contains_key(&key_field) {
                                    if exprent_independent {
                                        initializers.borrow_mut().add_with_key(
                                            assign_expr.get_right(),
                                            key_field.clone(),
                                        );
                                        whitelist.add(key_field);
                                        remove = true;
                                    } else if let Some(new_exprent) =
                                        assign_expr.get_right().try_as_new()
                                    {
                                        if let Some(constructor) = new_exprent.get_constructor() {
                                            let instance = constructor.get_instance();
                                            let index = instance
                                                .and_then(|i| i.try_as_var())
                                                .map(|v| v.get_index())
                                                .filter(|ix| non_field_assigns.contains_key(ix));
                                            if let Some(index) = index {
                                                let non_field_assignment =
                                                    non_field_assigns.remove(&index).unwrap();
                                                constructor.set_instance(Some(
                                                    non_field_assignment.get_right(),
                                                ));
                                                exprents_to_remove.push(non_field_assignment.0);
                                                initializers.borrow_mut().add_with_key(
                                                    assign_expr.get_right(),
                                                    key_field.clone(),
                                                );
                                                whitelist.add(key_field);
                                                remove = true;
                                            }
                                        }
                                    }
                                }
                            } else {
                                not_inlined.push(f_expr);
                            }
                        }
                    } else if inline_initializers {
                        if let Some(left) = assign_expr.get_left().try_as_var() {
                            non_field_assigns.put(left.get_index(), assign_expr);
                        }
                    }
                }

                if remove {
                    first_exprents.remove_at(itr);
                } else {
                    itr += 1;
                }
            }
            if !exprents_to_remove.is_empty() {
                first_exprents
                    .borrow_mut()
                    .retain(|e| !exprents_to_remove.j_contains(e));
            }
        }

        if cl.has_modifier(ACC_ENUM) {
            for fd in cl.get_fields().iter() {
                if fd.has_modifier(ACC_ENUM) {
                    let key =
                        InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor());
                    if wrapper
                        .get_static_field_initializers()
                        .borrow()
                        .get_with_key(&key)
                        .is_none()
                    {
                        method.add_comment("$VF: Failed to inline enum fields");
                        method.with_mut(|d| d.add_error_comment = true);
                        break;
                    }
                }
            }
        }
    }

    fn extract_dynamic_initializers(wrapper: ClassWrapper) {
        let cl = wrapper.get_class_struct();

        let is_anonymous = root_class_node(Some(&cl.qualified_name()))
            .expect("root class node")
            .with(|d| d.type_)
            == ClassNodeType::Anonymous;

        let mut lst_first: Vec<JList<Exprent>> = Vec::new();
        let mut lst_method_wrappers: Vec<MethodWrapper> = Vec::new();

        let methods = wrapper.get_methods().borrow().to_vec();
        for method in methods {
            let (method_struct, root) = method.with(|d| (d.method_struct, d.root));
            let Some(root) = root.filter(|_| INIT_NAME == method_struct.get_name()) else {
                continue;
            };
            let Some(first_data) = Statements::find_first_data(root.0) else {
                continue;
            };
            let first_exprents = first_data.get_exprents().expect("exprents");
            if first_exprents.is_empty() {
                continue;
            }

            let exprent = first_exprents.get(0);
            if !is_anonymous
                && (!exprent.is_invocation()
                    || !Statements::is_invocation_init_constructor(
                        exprent.as_invocation(),
                        method,
                        wrapper,
                        false,
                    ))
            {
                continue;
            }
            lst_first.push(first_exprents);
            lst_method_wrappers.push(method);
        }

        if lst_first.is_empty() {
            return;
        }

        let mut whitelist: JHashSet<String> = JHashSet::from_collection(
            wrapper
                .get_static_field_initializers()
                .borrow()
                .get_lst_keys()
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<String>>(),
        );
        let mut prev_fidx = 0;

        loop {
            let mut field_with_descr: Option<String> = None;
            let mut value: Option<Exprent> = None;

            for i in 0..lst_first.len() {
                let lst = &lst_first[i];

                if lst.len() < if is_anonymous { 1 } else { 2 } {
                    return;
                }

                let exprent = lst.get(if is_anonymous { 0 } else { 1 });

                let mut found = false;

                if let Some(assign_expr) = exprent.try_as_assignment() {
                    if let Some(f_expr) = assign_expr.get_left().try_as_field() {
                        if !f_expr.is_static()
                            && f_expr.get_classname() == cl.qualified_name()
                            && cl.has_field(
                                &f_expr.get_name(),
                                &f_expr.get_descriptor().descriptor_string,
                            )
                        {
                            let field_key = InterpreterUtil::make_unique_key(
                                &f_expr.get_name(),
                                &f_expr.get_descriptor().descriptor_string,
                            );
                            let fidx = cl.get_fields().get_index_by_key(&field_key) as i32;
                            if prev_fidx <= fidx
                                && Self::is_exprent_independent(
                                    f_expr,
                                    assign_expr.get_right(),
                                    lst_method_wrappers[i],
                                    cl,
                                    &whitelist,
                                    &[],
                                    &[],
                                    fidx,
                                    false,
                                )
                            {
                                prev_fidx = fidx;
                                if field_with_descr.is_none() {
                                    field_with_descr = Some(field_key);
                                    value = Some(assign_expr.get_right());
                                } else if field_with_descr.as_deref() != Some(field_key.as_str())
                                    || !value.unwrap().equals(assign_expr.get_right())
                                {
                                    return;
                                }
                                found = true;
                            }
                        }
                    }
                }

                if !found {
                    return;
                }
            }

            let field_with_descr = field_with_descr.expect("fieldWithDescr");
            let dynamic_initializers = wrapper.get_dynamic_field_initializers();
            if !dynamic_initializers
                .borrow()
                .contains_key(&field_with_descr)
            {
                let value = Self::process_dynamic_initializer(value.expect("value"));
                dynamic_initializers
                    .borrow_mut()
                    .add_with_key(value, field_with_descr.clone());
                whitelist.add(field_with_descr);

                for lst in &lst_first {
                    lst.remove_at(if is_anonymous { 0 } else { 1 });
                }
            } else {
                return;
            }
        }
    }

    fn process_dynamic_initializer(expr: Exprent) -> Exprent {
        let mut expr = expr;

        if expr.is_function() {
            let mut temp = expr;
            while let Some(func) = temp.try_as_function() {
                let func_type = func.get_func_type();
                if func_type.cast_type().is_none() && func_type != FunctionType::Cast {
                    break;
                }
                temp = func.get_lst_operands().get(0);
            }

            if let Some(func) = temp.try_as_function() {
                func.unwrap_box();

                expr = func.0;
            }
        } else {
            expr = Self::process_boxing_cast(expr);
        }

        expr
    }

    fn process_boxing_cast(expr: Exprent) -> Exprent {
        let mut expr = expr;

        if let Some(invocation) = expr.try_as_invocation() {
            if invocation.is_unboxing_call() {
                if let Some(inner) = invocation.get_instance() {
                    if inner
                        .try_as_function()
                        .is_some_and(|f| f.get_func_type() == FunctionType::Cast)
                    {
                        let bytecode = expr.with(|d| d.bytecode.clone());
                        inner.add_bytecode_offsets(bytecode.as_ref());
                        expr = inner;
                    }
                }
            }
        }

        expr
    }

    #[allow(clippy::too_many_arguments)]
    fn is_exprent_independent(
        field: FieldExprent,
        exprent: Exprent,
        method: MethodWrapper,
        cl: StructClass,
        whitelist: &JHashSet<String>,
        multi_assign: &[String],
        not_inlined: &[FieldExprent],
        fidx: i32,
        is_static: bool,
    ) -> bool {
        let _key_field = InterpreterUtil::make_unique_key(
            &field.get_name(),
            &field.get_descriptor().descriptor_string,
        );
        let lst = exprent.get_all_exprents_rec_self(true, true);

        let varproc = method.with(|d| d.varproc);
        for expr in lst {
            match expr.get_type() {
                ExprentType::Var => {
                    let var_pair = VarVersionPair::from_var(expr.as_var());
                    if !varproc.get_external_vars().borrow().contains(&var_pair) {
                        let var_name = varproc.get_var_name(var_pair);
                        match var_name {
                            None => return false,
                            Some(name) => {
                                if name != "this" && !name.ends_with(".this") {
                                    return false;
                                }
                            }
                        }
                    }
                }
                ExprentType::Field => {
                    let fexpr = expr.as_field();
                    if not_inlined.j_contains(&fexpr) {
                        return false;
                    }

                    if cl.has_field(&fexpr.get_name(), &fexpr.get_descriptor().descriptor_string) {
                        let key = InterpreterUtil::make_unique_key(
                            &fexpr.get_name(),
                            &fexpr.get_descriptor().descriptor_string,
                        );
                        if is_static {
                            if multi_assign.contains(&fexpr.get_name()) {
                                return false;
                            }

                            if !fexpr.is_static() {
                                return false;
                            } else if cl.get_fields().get_index_by_key(&key) as i32 >= fidx {
                                fexpr.force_qualified(true);
                            }
                        } else if !whitelist.contains(&key) {
                            return false;
                        } else if cl.get_fields().get_index_by_key(&key) as i32 > fidx {
                            return false;
                        }
                    } else if !fexpr.is_static() && fexpr.get_instance().is_none() {
                        return false;
                    }
                }
                ExprentType::New => {
                    Self::qualify_field_references(expr.as_new(), cl, fidx);
                }
                _ => {}
            }
        }

        true
    }

    fn qualify_field_references(nexpr: NewExprent, cl: StructClass, fidx: i32) {
        let is_static = cl.get_fields().get(fidx as usize).has_modifier(ACC_STATIC);
        if is_static && nexpr.is_lambda() && !nexpr.is_method_reference() {
            let child = root_class_node(nexpr.get_new_type().value.as_deref()).expect("child");
            let (parent, content_method_key) = child.with(|d| {
                (
                    d.parent,
                    d.lambda_information
                        .as_ref()
                        .expect("lambdaInformation")
                        .content_method_key
                        .clone(),
                )
            });
            let wrapper = parent
                .expect("parent")
                .get_wrapper()
                .expect("wrapper")
                .get_methods()
                .borrow()
                .get_with_key(&content_method_key)
                .expect("method wrapper");

            let mut s: JHashSet<Exprent> = JHashSet::new();
            wrapper
                .get_or_build_graph()
                .expect("graph")
                .iterate_exprents_deep(|e: Exprent| {
                    if e.is_field() || e.is_new() {
                        s.add(e);
                    }
                    0
                });
            for e in s.to_vec() {
                match e.get_type() {
                    ExprentType::Field => {
                        let fe = e.as_field();
                        if cl.qualified_name() == fe.get_classname()
                            && fe.is_static()
                            && cl.has_field(&fe.get_name(), &fe.get_descriptor().descriptor_string)
                        {
                            let key = InterpreterUtil::make_unique_key(
                                &fe.get_name(),
                                &fe.get_descriptor().descriptor_string,
                            );
                            if fe.get_instance().is_none()
                                && cl.get_fields().get_index_by_key(&key) as i32 > fidx
                            {
                                fe.force_qualified(true);
                            }
                        }
                    }
                    ExprentType::New => {
                        Self::qualify_field_references(e.as_new(), cl, fidx);
                    }
                    _ => {}
                }
            }
        }
    }
}
