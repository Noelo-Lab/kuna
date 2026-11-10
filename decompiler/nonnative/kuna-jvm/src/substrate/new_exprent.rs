//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/NewExprent.java`.
//!
//! kuna fix `lambda_wildcard_return` (`kuna_fixes`): in
//! `get_inferred_expr_type`, a method reference whose return type is the
//! receiver's `?` type argument keeps the generic mapping already inferred,
//! where Vineflower dereferences the missing return type.
//!
//! kuna fix `external_class_diamond`: in `get_inferred_expr_type`,
//! `new C(..)` of a class outside the class universe whose expected type is a
//! parameterization of `C` gets the diamond (`kuna_fix_external_class_diamond`),
//! where Vineflower writes it raw.
//!
//! kuna fix `implicit_outer_instance`: in `to_java`, the creation of an inner
//! class whose outer instance is the one an unqualified creation passes, and
//! whose null check javac emits for a qualified one is absent, is written
//! unqualified (`kuna_fix_implicit_outer_instance`).
//!
//! kuna fix `qualified_super_call`: in `to_java`, an anonymous class whose
//! constructor passes one of its own parameters to its inner superclass's as
//! the outer instance is created `o.new S(..) {..}`
//! (`kuna_fix_qualified_super_call`).
//!
//! kuna fix `vararg_spread_resolution`: in `to_java`, the lone element of a
//! spread varargs array that converts to the array type is cast to the
//! element type (`kuna_fix_vararg_spread_resolution`).

use std::collections::VecDeque;

use crate::check_types_result::CheckTypesResult;
use crate::class_writer::ClassWriter;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_STATIC, INIT_NAME};
use crate::code_type::CodeType;
use crate::dec_helper::DecHelper;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::{Exprent, ExprentKind, ExprentType};
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::generic_type::GenericType;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::java_subclass;
use crate::kuna_fixes::{self, KunaFix};
use crate::substrate::kuna_fix_external_class_diamond;
use crate::substrate::kuna_fix_implicit_outer_instance;
use crate::substrate::kuna_fix_qualified_super_call;
use crate::substrate::kuna_fix_vararg_spread_resolution;
use crate::list_stack::ListStack;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::{BitSet, JHashMap, JHashSet, JList, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub struct NewExprentData {
    /// `None` for arrays.
    pub constructor: Option<InvocationExprent>,
    pub new_type: VarType,
    pub lst_dims: JList<Exprent>,
    pub lst_array_elements: JList<Exprent>,
    pub direct_array_init: bool,
    pub is_var_arg_param: bool,
    pub anonymous: bool,
    pub lambda: bool,
    pub method_reference: bool,
    pub enum_const: bool,
    pub generic_args: Vec<VarType>,
    pub inferred_lambda_type: Option<VarType>,
    /// kuna fix `implicit_outer_instance`: javac's outer-instance null check
    /// of this creation was removed (a qualified `x.new Inner()`).
    pub outer_checked: bool,
}

java_subclass!(pub NewExprent: Exprent, ExprentKind::New(NewExprentData),
               is_new, as_new, try_as_new);

impl JavaEq for NewExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for NewExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

/// `DecompilerContext.getClassProcessor().getMapRootClasses().get(name)`; a null name maps to null.
fn root_class_node(name: Option<&str>) -> Option<ClassNode> {
    let name = name?;
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

/// `DecompilerContext.getStructContext().getClass(name)`; a null name maps to null.
fn struct_class(name: Option<&str>) -> Option<StructClass> {
    name.and_then(|name| DecompilerContext::get_struct_context().get_class(name))
}

/// `typename.substring(typename.lastIndexOf('.') + 1)`.
fn after_last_dot(typename: &str) -> String {
    typename[typename.rfind('.').map_or(0, |i| i + 1)..].to_string()
}

impl NewExprent {
    /// Java: `NewExprent(VarType, ListStack<Exprent>, int, BitSet)`.
    pub fn new_stack(
        new_type: VarType,
        stack: &mut ListStack<Exprent>,
        array_dim: i32,
        bytecode_offsets: Option<&BitSet>,
    ) -> NewExprent {
        let lst_dims = NewExprent::get_dimensions(array_dim, stack);
        NewExprent::new(new_type, lst_dims, bytecode_offsets)
    }

    /// Java: `NewExprent(VarType, List<Exprent>, BitSet)`.
    pub fn new(
        new_type: VarType,
        lst_dims: impl Into<JList<Exprent>>,
        bytecode_offsets: Option<&BitSet>,
    ) -> NewExprent {
        let id = Exprent::next_id();
        let lst_dims = lst_dims.into();

        let mut anonymous = false;
        let mut lambda = false;
        let mut method_reference = false;
        if new_type.type_ == CodeType::Object && new_type.array_dim == 0 {
            let node = root_class_node(new_type.value.as_deref());
            if let Some(node) = node {
                let (node_type, is_method_reference) = node.with(|n| {
                    (
                        n.type_,
                        n.lambda_information.as_ref().map(|l| l.is_method_reference),
                    )
                });
                if node_type == ClassNodeType::Anonymous || node_type == ClassNodeType::Lambda {
                    anonymous = true;
                    if node_type == ClassNodeType::Lambda {
                        lambda = true;
                        method_reference =
                            is_method_reference.expect("NullPointerException: lambdaInformation");
                    }
                }
            }
        }

        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::New(NewExprentData {
                constructor: None,
                new_type,
                lst_dims,
                lst_array_elements: JList::new(),
                direct_array_init: false,
                is_var_arg_param: false,
                anonymous,
                lambda,
                method_reference,
                enum_const: false,
                generic_args: Vec::new(),
                inferred_lambda_type: None,
                outer_checked: false,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        NewExprent(e)
    }

    fn get_dimensions(array_dim: i32, stack: &mut ListStack<Exprent>) -> Vec<Exprent> {
        let mut lst_dims = Vec::new();
        for _ in 0..array_dim {
            lst_dims.insert(0, stack.pop());
        }
        lst_dims
    }

    pub fn get_expr_type(self) -> VarType {
        let (anonymous, new_type) = self.d(|d| (d.anonymous, d.new_type.clone()));
        if anonymous {
            let node = root_class_node(new_type.value.as_deref())
                .expect("NullPointerException: anonymous class node");
            node.with(|n| n.anonymous_class_type.clone())
                .expect("NullPointerException: anonymousClassType")
        } else {
            new_type
        }
    }

    pub fn get_inferred_expr_type(self, upper_bound: Option<&VarType>) -> VarType {
        self.d_mut(|d| d.generic_args.clear());
        let (lambda, anonymous, new_type, constructor) =
            self.d(|d| (d.lambda, d.anonymous, d.new_type.clone(), d.constructor));
        if !lambda && new_type.type_ == CodeType::Object {
            let node = struct_class(new_type.value.as_deref());

            if node.is_none() && kuna_fixes::enabled(KunaFix::ExternalClassDiamond) {
                if let Some(type_) = kuna_fix_external_class_diamond::infer(self, upper_bound) {
                    return type_;
                }
            }

            match node.and_then(|n| n.get_signature()) {
                Some(sig) => {
                    if anonymous {
                        let superclass = sig.superclass();
                        let superinterfaces = sig.superinterfaces().clone();
                        if VarType::VARTYPE_OBJECT().equals(&superclass)
                            && !superinterfaces.is_empty()
                        {
                            return superinterfaces[0].clone();
                        }
                        return superclass;
                    } else if new_type.array_dim == 0 && !sig.fparameters().is_empty() {
                        if let Some(constructor) = constructor {
                            let ret = constructor.get_inferred_expr_type(upper_bound);
                            return if ret.type_ != CodeType::Void {
                                ret
                            } else {
                                self.get_expr_type()
                            };
                        } else {
                            let generic_type = sig.generic_type();
                            let mut generics_map: JHashMap<VarType, Option<VarType>> =
                                JHashMap::new();
                            self.0
                                .gather_generics(upper_bound, &generic_type, &mut generics_map);
                            let fparameters = sig.fparameters().clone();
                            let mut generic_args =
                                self.d_mut(|d| std::mem::take(&mut d.generic_args));
                            self.0.get_generic_args_into(
                                &fparameters,
                                &generics_map,
                                &mut generic_args,
                            );
                            self.d_mut(|d| d.generic_args = generic_args);
                            let new = generic_type.remap(&generics_map);
                            if !new.as_ref().is_some_and(|n| n.ptr_eq(&generic_type)) {
                                return new.expect("NewExprent.getInferredExprType: null type");
                            }
                        }
                    }
                }
                None => {
                    let lst_array_elements = self.get_lst_array_elements();
                    let object = VarType::VARTYPE_OBJECT();
                    if new_type.array_dim > 0
                        && !lst_array_elements.is_empty()
                        && object.value.as_deref() == Some(new_type.value_str())
                    {
                        let first = lst_array_elements.get(0).get_inferred_expr_type(None);
                        if first.type_ == CodeType::Genvar {
                            let mut matches = true;
                            let mut i = 1;
                            while i < lst_array_elements.len() {
                                let type_ = lst_array_elements.get(i).get_inferred_expr_type(None);
                                if !type_.equals(&first) {
                                    matches = false;
                                    break;
                                }
                                i += 1;
                            }
                            if matches {
                                return first.resize_array_dim(new_type.array_dim);
                            }
                        }
                    }
                }
            }
        }

        if lambda {
            let node = root_class_node(new_type.value.as_deref());

            if let Some(node) = node {
                let class_type = node
                    .with(|d| d.anonymous_class_type.clone())
                    .expect("NullPointerException: anonymousClassType");
                let cls = struct_class(class_type.value.as_deref());
                let lambda_information = node
                    .with(|d| d.lambda_information.clone())
                    .expect("NullPointerException: lambdaInformation");
                let desc =
                    MethodDescriptor::parse_descriptor(&lambda_information.method_descriptor);
                let method_cls = struct_class(Some(&lambda_information.content_class_name));

                let cls_sig = cls.and_then(|c| c.get_signature());
                if let (Some(cls), Some(cls_sig), Some(method_cls)) = (cls, cls_sig, method_cls) {
                    let ref_method = cls.get_method_key(&self.get_lambda_method_key());
                    let method = method_cls.get_method(
                        &lambda_information.content_method_name,
                        &lambda_information.content_method_descriptor,
                    );

                    let ref_sig = ref_method.and_then(|m| m.get_signature());
                    if let (Some(method), Some(ref_method), Some(ref_sig)) =
                        (method, ref_method, ref_sig)
                    {
                        let ret = cls_sig.generic_type();

                        let mut generics_map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
                        let named = self.0.get_named_generics();

                        self.0.gather_generics(upper_bound, &ret, &mut generics_map);

                        let mut instance_map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
                        if self.is_method_reference() {
                            if let Some(method_cls_sig) = method_cls.get_signature() {
                                let _first = ret.get_arguments()[0].clone();
                                let constructor =
                                    constructor.expect("NullPointerException: constructor");
                                if let Some(instance) = constructor.get_instance() {
                                    let instance_type = instance.get_inferred_expr_type(None);
                                    if instance_type.is_generic() {
                                        method_cls_sig.generic_type().map_gen_vars_to(
                                            &instance_type.as_generic(),
                                            &mut instance_map,
                                        );
                                    }
                                } else if let Some(method_sig) = method.get_signature() {
                                    let method_params = method_sig.parameter_types().clone();
                                    let ref_params = ref_sig.parameter_types().clone();
                                    for (i, mtype) in method_params.iter().enumerate() {
                                        let rtype = &ref_params[i];
                                        if mtype.type_ == CodeType::Genvar
                                            && rtype.type_ == CodeType::Genvar
                                            && generics_map.contains_key(rtype)
                                        {
                                            instance_map.put(
                                                mtype.clone(),
                                                generics_map.get(rtype).cloned().flatten(),
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        let types: Vec<VarType> = match method.get_signature() {
                            Some(method_sig) => method_sig.parameter_types().clone(),
                            None => desc.params.clone(),
                        };
                        for (i, real) in types.iter().enumerate() {
                            let parameter_types = ref_sig.parameter_types().clone();
                            if i < parameter_types.len()
                                && parameter_types[i].type_ == CodeType::Genvar
                                && !generics_map.contains_key(&parameter_types[i])
                            {
                                let mut real_type = real.clone();
                                let type_cls = struct_class(real_type.value.as_deref());
                                if let Some(type_sig) = type_cls.and_then(|c| c.get_signature()) {
                                    if !real_type.equals(&type_sig.generic_type()) {
                                        real_type = type_sig
                                            .generic_type()
                                            .resize_array_dim(real_type.array_dim);
                                    }
                                }
                                generics_map.put(parameter_types[i].clone(), Some(real_type));
                            }
                        }

                        let ref_return_type = ref_sig
                            .return_type()
                            .expect("NullPointerException: returnType");
                        if ref_return_type.type_ == CodeType::Genvar {
                            let key = ref_return_type;
                            if method.get_name() == INIT_NAME {
                                if let Some(method_cls_sig) = method_cls.get_signature() {
                                    generics_map
                                        .put(key, Some(method_cls_sig.generic_type().0.clone()));
                                } else {
                                    generics_map.put(
                                        key,
                                        Some(GenericType::parse(&format!(
                                            "L{};",
                                            method_cls.qualified_name()
                                        ))),
                                    );
                                }
                            } else if method.get_signature().is_some()
                                || !desc.ret.equals(&VarType::VARTYPE_OBJECT())
                            {
                                let current = generics_map.get(&key).cloned().flatten();
                                let mut return_type: Option<VarType> = match method.get_signature()
                                {
                                    Some(method_sig) => method_sig
                                        .return_type()
                                        .expect("NullPointerException: returnType")
                                        .remap(&instance_map),
                                    None => Some(desc.ret.clone()),
                                };
                                let ret_cls = match &return_type {
                                    None => None,
                                    Some(rt) => struct_class(rt.value.as_deref()),
                                };

                                if !self.is_method_reference() {
                                    if let Some(ret_sig) = ret_cls.and_then(|c| c.get_signature()) {
                                        let rt = return_type.clone().expect("returnType");
                                        if !ret_sig.generic_type().equals_exact(&rt) {
                                            let ret_ub = match &current {
                                                Some(c) if c.equals(&rt) => c.clone(),
                                                _ => rt.clone(),
                                            };
                                            let real_type = NewExprent::get_lambda_return_type(
                                                node,
                                                ref_method,
                                                Some(&ret_ub),
                                                &generics_map,
                                            );
                                            if real_type.is_some() {
                                                return_type = real_type;
                                            }
                                        }
                                    }
                                }

                                let add = match (&current, &return_type) {
                                    (None, _) | (_, None) => true,
                                    (Some(current), Some(rt)) => {
                                        rt.is_generic()
                                            || (!rt.equals_opt(
                                                generics_map.get(&key).cloned().flatten().as_ref(),
                                            ) && (current.type_ != CodeType::Genvar
                                                || !named.contains_key(current)))
                                    }
                                };
                                if add {
                                    let prior = generics_map.get(&key).cloned().flatten();
                                    let mut add_to_map = true;
                                    if let Some(prior) = prior {
                                        if return_type.is_none()
                                            && kuna_fixes::enabled(KunaFix::LambdaWildcardReturn)
                                        {
                                            if prior.is_generic()
                                                && !prior.as_generic().is_type_unfinished()
                                            {
                                                add_to_map = false;
                                            }
                                        } else if prior.is_generic()
                                            && !prior.as_generic().is_type_unfinished()
                                            && DecompilerContext::get_struct_context().instance_of(
                                                return_type
                                                    .as_ref()
                                                    .expect("NullPointerException: returnType")
                                                    .value_str(),
                                                prior.value_str(),
                                            )
                                        {
                                            add_to_map = false;
                                        }
                                    }

                                    if add_to_map {
                                        generics_map.put(key, return_type);
                                    }
                                }
                            }
                        }

                        for from in ret.get_all_generic_vars() {
                            if generics_map.get(&from).cloned().flatten().is_none() {
                                generics_map
                                    .put(from.0.clone(), Some(GenericType::DUMMY_VAR().0.clone()));
                            }
                        }

                        if !generics_map.is_empty() {
                            let new = ret.remap(&generics_map);
                            if !new.as_ref().is_some_and(|n| n.ptr_eq(&ret)) {
                                let new = new.expect("NullPointerException: _new");
                                let named_keys = JHashSet::from_collection(named.key_vec());
                                if !new.is_generic()
                                    || !new.as_generic().has_unknown_generic_type(&named_keys)
                                {
                                    let inferred = new.clone();
                                    self.d_mut(|d| d.inferred_lambda_type = Some(inferred));
                                }
                                return new;
                            }
                        }
                    }
                } else {
                    self.d_mut(|d| d.inferred_lambda_type = Some(class_type));
                }
            }
        }

        self.get_expr_type()
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let mut result = CheckTypesResult::new();
        let (new_type, constructor) = self.d(|d| (d.new_type.clone(), d.constructor));

        if new_type.array_dim != 0 {
            for dim in self.get_lst_dims().to_vec() {
                result.add_expr_lower_bound(dim, &VarType::VARTYPE_BYTECHAR());
                result.add_expr_upper_bound(dim, &VarType::VARTYPE_INT());
            }

            if new_type.array_dim == 1 {
                let left_type = new_type.decrease_array_dim();
                for element in self.get_lst_array_elements().to_vec() {
                    result.add_expr_lower_bound(
                        element,
                        &VarType::find_family_bottom(left_type.type_family),
                    );
                    result.add_expr_upper_bound(element, &left_type);
                }
            }
        } else if let Some(constructor) = constructor {
            return constructor.check_expr_type_bounds();
        }

        Some(result)
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        let (array_dim, lst_dims, lst_array_elements, constructor) = self.d(|d| {
            (
                d.new_type.array_dim,
                d.lst_dims.clone(),
                d.lst_array_elements.clone(),
                d.constructor,
            )
        });
        if array_dim != 0 {
            lst.extend(lst_dims.iter());
            lst.extend(lst_array_elements.iter());
        } else if let Some(constructor) = constructor {
            if let Some(instance) = constructor.get_instance() {
                lst.push(instance);
            }
            lst.extend(constructor.get_lst_parameters().iter());
        }
    }

    pub fn copy(self) -> Exprent {
        let (
            new_type,
            lst_dims,
            constructor,
            lst_array_elements,
            direct_array_init,
            anonymous,
            enum_const,
            outer_checked,
        ) = self.d(|d| {
            (
                d.new_type.clone(),
                d.lst_dims.clone(),
                d.constructor,
                d.lst_array_elements.clone(),
                d.direct_array_init,
                d.anonymous,
                d.enum_const,
                d.outer_checked,
            )
        });
        let mut lst = Vec::new();
        for expr in lst_dims.iter() {
            lst.push(expr.copy());
        }

        let bytecode = self.get_bytecode();
        let ret = NewExprent::new(new_type, lst, bytecode.as_ref());
        ret.set_constructor(constructor.map(|c| c.0.copy().as_invocation()));
        ret.set_lst_array_elements(DecHelper::copy_exprent_list(&lst_array_elements.to_vec()));
        ret.set_direct_array_init(direct_array_init);
        ret.set_anonymous(anonymous);
        ret.set_enum_const(enum_const);
        ret.d_mut(|d| d.outer_checked = outer_checked);
        ret.0
    }

    pub fn get_precedence(self) -> i32 {
        1
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let (
            anonymous,
            lambda,
            enum_const,
            direct_array_init,
            is_var_arg_param,
            new_type,
            constructor,
        ) = self.d(|d| {
            (
                d.anonymous,
                d.lambda,
                d.enum_const,
                d.direct_array_init,
                d.is_var_arg_param,
                d.new_type.clone(),
                d.constructor,
            )
        });

        if anonymous {
            let child = root_class_node(new_type.value.as_deref());

            let self_reference = DecompilerContext::get_current_class_node() == child;
            let anonymous_class_type = || {
                child
                    .expect("NullPointerException: child")
                    .with(|d| d.anonymous_class_type.clone())
                    .expect("NullPointerException: anonymousClassType")
            };

            if !enum_const
                && (!lambda
                    || DecompilerContext::get_option(
                        IFernflowerPreferences::LAMBDA_TO_ANONYMOUS_CLASS,
                    ))
            {
                let mut enclosing: Option<TextBuffer> = None;

                if !lambda {
                    if let Some(constructor) = constructor {
                        let qualifier = if kuna_fixes::enabled(KunaFix::QualifiedSuperCall) {
                            child.and_then(|child| {
                                kuna_fix_qualified_super_call::anonymous_qualifier(
                                    child,
                                    constructor,
                                )
                            })
                        } else {
                            None
                        };
                        if kuna_fixes::enabled(KunaFix::QualifiedSuperCall) {
                            constructor.set_kuna_skip_arg(qualifier);
                        }
                        enclosing = match qualifier {
                            Some(arg) => Some(kuna_fix_qualified_super_call::qualifier_text(
                                constructor.get_lst_parameters().get(arg),
                                indent,
                            )),
                            None => NewExprent::get_qualified_new_instance_opt(
                                anonymous_class_type().value.as_deref(),
                                &constructor.get_lst_parameters().to_vec(),
                                indent,
                            ),
                        };
                        if let Some(enclosing) = &enclosing {
                            buf.append_buffer(enclosing).append_char('.');
                        }
                    }
                }

                buf.append("new ");

                if self_reference {
                    buf.append("<anonymous constructor>");
                } else {
                    let child_type = anonymous_class_type();
                    let mut typename = Some(ExprProcessor::get_cast_type_name(&child_type));
                    if enclosing.is_some() {
                        let anonymous_node = root_class_node(child_type.value.as_deref());
                        if let Some(anonymous_node) = anonymous_node {
                            typename = anonymous_node.with(|d| d.simple_name.clone());
                        } else {
                            typename = typename.as_deref().map(after_last_dot);
                        }
                    }

                    let mut append_type = true;
                    let child_node = child.expect("NullPointerException: child");
                    if let Some(wrapper) = child_node.get_wrapper() {
                        let descriptor = wrapper.get_class_struct().get_signature();
                        if let Some(descriptor) = descriptor {
                            let superinterfaces = descriptor.superinterfaces().clone();
                            if superinterfaces.is_empty() {
                                buf.append_cast_type_name(&descriptor.superclass());
                            } else {
                                if superinterfaces.len() > 1 && !lambda {
                                    let qualified_name =
                                        child_node.with(|d| d.class_struct).qualified_name();
                                    DecompilerContext::get_logger().write_message(
                                        &format!(
                                            "Inconsistent anonymous class signature: {}",
                                            qualified_name
                                        ),
                                        Severity::Warn,
                                    );
                                }
                                buf.append_cast_type_name(&superinterfaces[0]);
                            }
                            append_type = false;
                        }
                    }

                    if append_type {
                        buf.append_cast_type_name_str(
                            &typename.expect("NullPointerException: typename"),
                            &child_type,
                        );
                    }
                }
            }

            match constructor {
                Some(constructor) if !lambda => {
                    self.0
                        .append_parameters(&mut buf, &constructor.get_generic_args());
                    buf.append_char('(')
                        .append_buffer(&constructor.append_param_list(indent));
                }
                _ => {
                    let generic_args = self.d(|d| d.generic_args.clone());
                    self.0.append_parameters(&mut buf, &generic_args);
                    buf.append_char('(');
                }
            }

            buf.append_char(')');

            if enum_const && buf.length() == 2 {
                buf.set_length(0);
            }

            if lambda {
                if !DecompilerContext::get_option(IFernflowerPreferences::LAMBDA_TO_ANONYMOUS_CLASS)
                {
                    buf.set_length(0);
                }
                self.set_lambda_generic_types();
                let method_object = constructor.and_then(|c| c.get_instance());
                ClassWriter::new().class_lambda_to_java(
                    child.expect("NullPointerException: child"),
                    &mut buf,
                    method_object,
                    indent,
                );
            } else if !self_reference {
                ClassWriter::new().write_class(
                    child.expect("NullPointerException: child"),
                    &mut buf,
                    indent,
                );
            }
        } else if direct_array_init {
            let left_type = new_type.decrease_array_dim();
            let lst_array_elements = self.get_lst_array_elements();
            buf.append_char('{');
            if !lst_array_elements.is_empty() {
                buf.push_newline_group(indent, 2);
                buf.append_possible_newline();
                buf.push_newline_group(indent, 0);
            }
            let mut i = 0;
            while i < lst_array_elements.len() {
                if i > 0 {
                    buf.append(",").append_possible_newline_alt(" ");
                }
                ExprProcessor::get_casted_exprent(
                    lst_array_elements.get(i),
                    &left_type,
                    &mut buf,
                    indent,
                    false,
                );
                i += 1;
            }
            if !lst_array_elements.is_empty() {
                buf.pop_newline_group();
                buf.append_possible_newline_dedent("", true);
                buf.pop_newline_group();
            }
            buf.append_char('}');
        } else if new_type.array_dim == 0 {
            if !enum_const {
                let mut enclosing: Option<TextBuffer> = None;

                if let Some(constructor) = constructor {
                    let params = constructor.get_lst_parameters().to_vec();
                    let implicit = kuna_fixes::enabled(KunaFix::ImplicitOuterInstance)
                        && new_type.value.as_deref().is_some_and(|name| {
                            kuna_fix_implicit_outer_instance::is_implicit(self, name, &params)
                        });
                    if !implicit {
                        enclosing = NewExprent::get_qualified_new_instance_opt(
                            new_type.value.as_deref(),
                            &params,
                            indent,
                        );
                    }
                    if let Some(enclosing) = &enclosing {
                        buf.append_buffer(enclosing).append_char('.');
                    }
                }

                buf.append("new ");

                let mut typename = Some(ExprProcessor::get_type_name(&new_type));
                if enclosing.is_some() {
                    let new_node = root_class_node(new_type.value.as_deref());
                    if let Some(new_node) = new_node {
                        typename = new_node.with(|d| d.simple_name.clone());
                    } else {
                        typename = typename.as_deref().map(after_last_dot);
                    }
                }
                buf.append_type_name_str(
                    &typename.expect("NullPointerException: typename"),
                    &new_type,
                );
            }

            if let Some(constructor) = constructor {
                let start = if enum_const { 2 } else { 0 };
                if !enum_const || start < constructor.get_lst_parameters().len() {
                    self.0
                        .append_parameters(&mut buf, &constructor.get_generic_args());
                    buf.append_char('(')
                        .append_buffer(&constructor.append_param_list(indent))
                        .append_char(')');
                }
            }
        } else if is_var_arg_param {
            let left_type = new_type.decrease_array_dim();
            let lst_array_elements = self.get_lst_array_elements();
            let lone_cast = kuna_fixes::enabled(KunaFix::VarargSpreadResolution)
                && lst_array_elements.len() == 1
                && kuna_fix_vararg_spread_resolution::lone_element_cast(
                    &new_type,
                    lst_array_elements.get(0),
                );
            let mut i = 0;
            while i < lst_array_elements.len() {
                if i > 0 {
                    buf.append(",").append_possible_newline_alt(" ");
                }

                let element = lst_array_elements.get(i);
                if let Some(element) = element.try_as_new() {
                    element.set_direct_array_init(false);
                }
                if lone_cast {
                    ExprProcessor::get_casted_exprent_full(
                        element,
                        &left_type,
                        &mut buf,
                        indent,
                        NullCastType::Cast,
                        true,
                        false,
                        false,
                    );
                } else {
                    ExprProcessor::get_casted_exprent(element, &left_type, &mut buf, indent, false);
                }
                i += 1;
            }

            if lst_array_elements.len() == 1
                && !kuna_fixes::enabled(KunaFix::VarargSpreadResolution)
            {
                let element_type = lst_array_elements.get(0).get_expr_type();
                if element_type.type_ == CodeType::Object
                    && element_type.value_str() == "java/lang/Object"
                    && element_type.array_dim >= 1
                {
                    buf.prepend("(Object)");
                    buf.add_class_token(1, 6, "java/lang/Object");
                }
            }
        } else {
            buf.append("new ").append_type_name(&new_type);

            let lst_array_elements = self.get_lst_array_elements();
            if lst_array_elements.is_empty() {
                let lst_dims = self.get_lst_dims();
                for i in 0..new_type.array_dim {
                    buf.append_char('[');
                    if (i as usize) < lst_dims.len() {
                        let dim = lst_dims.get(i as usize);
                        if dim.get_type() == ExprentType::Const {
                            dim.as_const().adjust_const_type(&VarType::VARTYPE_INT());
                        }
                        buf.append_buffer(&lst_dims.get(i as usize).to_java(indent));
                    }
                    buf.append_char(']');
                }
            } else {
                for _ in 0..new_type.array_dim {
                    buf.append("[]");
                }

                let left_type = new_type.decrease_array_dim();
                buf.append_char('{');
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline();
                buf.push_newline_group(indent, 0);
                let mut i = 0;
                while i < lst_array_elements.len() {
                    if i > 0 {
                        buf.append(",").append_possible_newline_alt(" ");
                    }
                    ExprProcessor::get_casted_exprent(
                        lst_array_elements.get(i),
                        &left_type,
                        &mut buf,
                        indent,
                        false,
                    );
                    i += 1;
                }
                buf.pop_newline_group();
                buf.append_possible_newline_dedent("", true);
                buf.append_char('}');
                buf.pop_newline_group();
            }
        }

        buf
    }

    pub fn probably_synthetic_parameter(class_name: &str) -> bool {
        root_class_node(Some(class_name))
            .is_some_and(|node| node.with(|n| n.type_) == ClassNodeType::Anonymous)
    }

    pub fn get_qualified_new_instance(
        classname: &str,
        lst_params: &[Exprent],
        indent: i32,
    ) -> Option<TextBuffer> {
        NewExprent::get_qualified_new_instance_opt(Some(classname), lst_params, indent)
    }

    /// `getQualifiedNewInstance` with Java's null `classname` allowed (it finds no node).
    fn get_qualified_new_instance_opt(
        classname: Option<&str>,
        lst_params: &[Exprent],
        indent: i32,
    ) -> Option<TextBuffer> {
        let node = root_class_node(classname);

        if let Some(node) = node {
            let (type_, access) = node.with(|d| (d.type_, d.access));
            if type_ != ClassNodeType::Root
                && type_ != ClassNodeType::Local
                && (access & ACC_STATIC) == 0
                && !lst_params.is_empty()
            {
                let enclosing = lst_params[0];

                let mut is_qualified_new = false;

                if let Some(var_enclosing) = enclosing.try_as_var() {
                    let current_class = DecompilerContext::get_current_class_node()
                        .expect("NullPointerException: current class node")
                        .with(|d| d.class_struct);
                    let this_classname = var_enclosing
                        .get_processor()
                        .expect("NullPointerException: processor")
                        .get_this_vars()
                        .borrow()
                        .get_cloned(&var_enclosing.get_var_version_pair());

                    if this_classname.as_deref() != Some(current_class.qualified_name().as_str()) {
                        is_qualified_new = true;
                    }
                } else {
                    is_qualified_new = true;
                }

                if is_qualified_new {
                    return Some(enclosing.to_java(indent));
                }
            }
        }

        None
    }

    fn get_lambda_return_type(
        node: ClassNode,
        desc: StructMethod,
        upper_bound: Option<&VarType>,
        generics_map: &JHashMap<VarType, Option<VarType>>,
    ) -> Option<VarType> {
        let wrapper = node.get_wrapper();

        if let Some(wrapper) = wrapper {
            let lambda_information = node
                .with(|d| d.lambda_information.clone())
                .expect("NullPointerException: lambdaInformation");
            let mt = wrapper.get_method_wrapper(
                &lambda_information.content_method_name,
                &lambda_information.content_method_descriptor,
            );

            if let Some(mt) = mt {
                let (root, varproc) = mt.with(|d| (d.root, d.varproc));
                if let Some(root) = root {
                    let mut param_names: Vec<Option<String>> = Vec::new();

                    let md_content = MethodDescriptor::parse_descriptor(
                        &lambda_information.content_method_descriptor,
                    );
                    let md_lambda =
                        MethodDescriptor::parse_descriptor(&lambda_information.method_descriptor);

                    let mut index = if lambda_information.is_content_method_static {
                        0
                    } else {
                        1
                    };
                    let start_index =
                        md_content.params.len() as i32 - md_lambda.params.len() as i32;

                    let mut j = 0;
                    for i in 0..md_content.params.len() {
                        if i as i32 >= start_index {
                            let vpp = VarVersionPair::new(index, 0);
                            let cur_type = varproc.get_var_type(vpp);
                            let inf_type = desc
                                .get_signature()
                                .expect("NullPointerException: signature")
                                .parameter_types()[j]
                                .remap(generics_map);
                            j += 1;

                            if let Some(inf_type) = inf_type {
                                if !inf_type.equals(&VarType::VARTYPE_VOID()) {
                                    let cur_type = cur_type.expect("NullPointerException: curType");
                                    if !cur_type.equals(&inf_type)
                                        || (inf_type.is_generic()
                                            && !inf_type.as_generic().equals_exact(&cur_type))
                                    {
                                        let var_name = varproc.get_var_name(vpp);
                                        param_names.push(var_name.clone());
                                        if let Some(var_name) = var_name {
                                            Exprent::with_inferred_lambda_types(|m| {
                                                m.put(var_name, inf_type.clone())
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        index += md_content.params[i].stack_size;
                    }

                    let remove_param_names = |param_names: &[Option<String>]| {
                        for name in param_names.iter().flatten() {
                            Exprent::with_inferred_lambda_types(|m| m.remove(name));
                        }
                    };

                    let dummy_exit = root.get_dummy_exit();

                    for edge in dummy_exit.get_all_predecessor_edges() {
                        let source = edge.get_source();
                        let lst_expr = source.get_exprents();

                        if let Some(lst_expr) = lst_expr {
                            if !lst_expr.is_empty() {
                                let expr = lst_expr.get(lst_expr.len() - 1);
                                if let Some(ex) = expr.try_as_exit() {
                                    if ex.get_exit_type() == ExitType::Return {
                                        let real_ret_type = ex
                                            .get_value()
                                            .expect("NullPointerException: value")
                                            .get_inferred_expr_type(upper_bound);
                                        if real_ret_type.is_generic() {
                                            remove_param_names(&param_names);
                                            return Some(real_ret_type);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    remove_param_names(&param_names);
                }
            }
        }
        None
    }

    fn set_lambda_generic_types(self) {
        let Some(mut inferred_lambda_type) = self.d(|d| d.inferred_lambda_type.clone()) else {
            return;
        };
        let new_type = self.get_new_type();
        let node = root_class_node(new_type.value.as_deref());
        let mut cls = struct_class(inferred_lambda_type.value.as_deref());

        let (Some(node), Some(found_cls)) = (node, cls) else {
            return;
        };
        let desc = found_cls.get_method_key(&self.get_lambda_method_key());
        let wrapper = node.get_wrapper();
        let method_wrapper = match wrapper {
            Some(wrapper) => {
                let lambda_information = node
                    .with(|d| d.lambda_information.clone())
                    .expect("NullPointerException: lambdaInformation");
                wrapper.get_method_wrapper(
                    &lambda_information.content_method_name,
                    &lambda_information.content_method_descriptor,
                )
            }
            None => None,
        };

        let Some(desc) = desc else {
            return;
        };
        let Some(desc_sig) = desc.get_signature() else {
            return;
        };
        let Some(method_wrapper) = method_wrapper else {
            return;
        };
        let (root, varproc) = method_wrapper.with(|d| (d.root, d.varproc));
        let Some(root) = root else {
            return;
        };

        if inferred_lambda_type.value.as_deref() != Some(desc.get_class_qualified_name().as_str()) {
            let candidate = struct_class(Some(&desc.get_class_qualified_name()))
                .expect("NullPointerException: candidate");
            if let Some(candidate_sig) = candidate.get_signature() {
                cls = Some(candidate);
                let super_type = GenericType::get_generic_super_type(
                    &inferred_lambda_type,
                    &candidate_sig.generic_type(),
                );
                let stored = super_type.clone();
                self.d_mut(|d| d.inferred_lambda_type = stored);
                inferred_lambda_type =
                    super_type.expect("NullPointerException: inferredLambdaType");
            }
        }
        let cls = cls.expect("cls");

        let mut temp_map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
        if inferred_lambda_type.is_generic() {
            cls.get_signature()
                .expect("NullPointerException: signature")
                .generic_type()
                .map_gen_vars_to(&inferred_lambda_type.as_generic(), &mut temp_map);
        }

        let lambda_information = node
            .with(|d| d.lambda_information.clone())
            .expect("NullPointerException: lambdaInformation");
        let md_content =
            MethodDescriptor::parse_descriptor(&lambda_information.content_method_descriptor);
        let md_lambda = MethodDescriptor::parse_descriptor(&lambda_information.method_descriptor);

        let mut index = if lambda_information.is_content_method_static {
            0
        } else {
            1
        };
        let start_index = md_content.params.len() as i32 - md_lambda.params.len() as i32;

        let mut j = 0;
        for i in 0..md_content.params.len() {
            if i as i32 >= start_index {
                let vpp = VarVersionPair::new(index, 0);
                let cur_type = varproc.get_var_type(vpp);
                let inf_type = desc_sig.parameter_types()[j].remap(&temp_map);
                j += 1;

                if let (Some(inf_type), Some(cur_type)) = (inf_type, cur_type) {
                    if !inf_type.equals(&VarType::VARTYPE_VOID())
                        && (!cur_type.equals(&inf_type)
                            || (inf_type.is_generic()
                                && !inf_type.as_generic().equals_exact(&cur_type)))
                    {
                        varproc.set_var_type(vpp, &inf_type);
                        let param_name = varproc.get_var_name(vpp);

                        let mut nested: VecDeque<ClassNode> =
                            node.with(|d| d.nested.to_vec()).into_iter().collect();
                        while let Some(child_node) = nested.pop_front() {
                            let (child_nested, child_type, child_lambda_information) = child_node
                                .with(|d| {
                                    (d.nested.to_vec(), d.type_, d.lambda_information.clone())
                                });
                            nested.extend(child_nested);

                            if child_type == ClassNodeType::Lambda {
                                let child_lambda_information = child_lambda_information
                                    .expect("NullPointerException: lambdaInformation");
                                if !child_lambda_information.is_method_reference {
                                    let enclosed_method = wrapper
                                        .expect("NullPointerException: wrapper")
                                        .get_method_wrapper(
                                            &child_lambda_information.content_method_name,
                                            &child_lambda_information.content_method_descriptor,
                                        );

                                    if let Some(enclosed_method) = enclosed_method {
                                        let enclosed_varproc = enclosed_method.with(|d| d.varproc);
                                        let param_name = param_name
                                            .as_deref()
                                            .expect("NullPointerException: paramName");
                                        if enclosed_varproc.get_var_name(vpp).as_deref()
                                            == Some(param_name)
                                        {
                                            enclosed_varproc.set_var_type(vpp, &inf_type);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            index += md_content.params[i].stack_size;
        }

        let cur_type = md_content.ret.clone();
        let inf_type = desc_sig
            .return_type()
            .expect("NullPointerException: returnType")
            .remap(&temp_map);

        if let Some(inf_type) = inf_type {
            if !inf_type.equals(&VarType::VARTYPE_VOID())
                && (!cur_type.equals(&inf_type)
                    || (inf_type.is_generic() && !inf_type.as_generic().equals_exact(&cur_type)))
            {
                let gen_desc = GenericMethodDescriptor::new(
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Some(inf_type),
                    Vec::new(),
                );
                let dummy_exit = root.get_dummy_exit();

                for edge in dummy_exit.get_all_predecessor_edges() {
                    let source = edge.get_source();
                    let lst_expr = source.get_exprents();

                    if let Some(lst_expr) = lst_expr {
                        if !lst_expr.is_empty() {
                            let expr = lst_expr.get(lst_expr.len() - 1);
                            if let Some(ex) = expr.try_as_exit() {
                                if ex.get_exit_type() == ExitType::Return {
                                    ex.get_method_descriptor()
                                        .expect("NullPointerException: methodDescriptor")
                                        .set_generic_info(Some(gen_desc.clone()));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn does_class_have_methods_named_same(self) -> bool {
        let (lambda, method_reference, new_type) =
            self.d(|d| (d.lambda, d.method_reference, d.new_type.clone()));
        if lambda && method_reference {
            let node = root_class_node(new_type.value.as_deref());

            if let Some(node) = node {
                let lambda_information = node
                    .with(|d| d.lambda_information.clone())
                    .expect("NullPointerException: lambdaInformation");
                let name = lambda_information.content_method_name.clone();
                let content_class = root_class_node(Some(&lambda_information.content_class_name));

                let Some(content_class) = content_class else {
                    return false;
                };

                let mut found_one = false;
                let methods = content_class
                    .with(|d| d.class_struct)
                    .get_methods()
                    .to_vec();
                for method in methods {
                    if method.get_name() == name {
                        if found_one {
                            return true;
                        } else {
                            found_one = true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        let constructor = self.d_mut(|d| {
            if d.constructor.map(|c| c.0) == Some(old_expr) {
                d.constructor = Some(new_expr.as_invocation());
            }
            d.constructor
        });

        if let Some(constructor) = constructor {
            constructor.0.replace_exprent(old_expr, new_expr);
        }

        let lst_dims = self.get_lst_dims();
        for i in 0..lst_dims.len() {
            if old_expr == lst_dims.get(i) {
                lst_dims.set(i, new_expr);
            }
        }

        let lst_array_elements = self.get_lst_array_elements();
        for i in 0..lst_array_elements.len() {
            if old_expr == lst_array_elements.get(i) {
                lst_array_elements.set(i, new_expr);
            }
        }
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(ne) = o.try_as_new() else {
            return false;
        };
        let (new_type, lst_dims, constructor, direct_array_init, lst_array_elements) =
            self.d(|d| {
                (
                    d.new_type.clone(),
                    d.lst_dims.to_vec(),
                    d.constructor,
                    d.direct_array_init,
                    d.lst_array_elements.to_vec(),
                )
            });
        new_type.equals(&ne.get_new_type())
            && lst_dims.java_equals(&ne.get_lst_dims().to_vec())
            && constructor
                .map(|c| c.0)
                .java_equals(&ne.get_constructor().map(|c| c.0))
            && direct_array_init == ne.d(|d| d.direct_array_init)
            && lst_array_elements.java_equals(&ne.get_lst_array_elements().to_vec())
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let (lst_array_elements, lst_dims, constructor) = self.d(|d| {
            (
                d.lst_array_elements.to_vec(),
                d.lst_dims.to_vec(),
                d.constructor,
            )
        });
        Exprent::measure_bytecode_list(values.as_deref_mut(), &lst_array_elements);
        Exprent::measure_bytecode_list(values.as_deref_mut(), &lst_dims);
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), constructor.map(|c| c.0));
        self.measure_bytecode(values);
    }

    pub fn get_constructor(self) -> Option<InvocationExprent> {
        self.d(|d| d.constructor)
    }

    pub fn set_constructor(self, constructor: Option<InvocationExprent>) {
        self.d_mut(|d| d.constructor = constructor)
    }

    /// The dimension list itself (aliased, like the Java reference).
    pub fn get_lst_dims(self) -> JList<Exprent> {
        self.d(|d| d.lst_dims.clone())
    }

    pub fn get_new_type(self) -> VarType {
        self.d(|d| d.new_type.clone())
    }

    /// The element list itself (aliased, like the Java reference).
    pub fn get_lst_array_elements(self) -> JList<Exprent> {
        self.d(|d| d.lst_array_elements.clone())
    }

    pub fn set_lst_array_elements(self, lst_array_elements: impl Into<JList<Exprent>>) {
        let lst_array_elements = lst_array_elements.into();
        self.d_mut(|d| d.lst_array_elements = lst_array_elements)
    }

    pub fn set_direct_array_init(self, direct_array_init: bool) {
        self.d_mut(|d| d.direct_array_init = direct_array_init)
    }

    pub fn is_direct_array_init(self) -> bool {
        self.d(|d| d.direct_array_init)
    }

    pub fn set_var_arg_param(self, is_var_arg_param: bool) {
        self.d_mut(|d| d.is_var_arg_param = is_var_arg_param)
    }

    pub fn is_var_arg_param(self) -> bool {
        self.d(|d| d.is_var_arg_param)
    }

    pub fn is_lambda(self) -> bool {
        self.d(|d| d.lambda)
    }

    pub fn is_anonymous(self) -> bool {
        self.d(|d| d.anonymous)
    }

    pub fn set_anonymous(self, anonymous: bool) {
        self.d_mut(|d| d.anonymous = anonymous)
    }

    pub fn is_enum_const(self) -> bool {
        self.d(|d| d.enum_const)
    }

    pub fn set_enum_const(self, enum_const: bool) {
        self.d_mut(|d| d.enum_const = enum_const)
    }

    pub fn is_method_reference(self) -> bool {
        self.d(|d| d.method_reference)
    }

    pub fn is_outer_checked(self) -> bool {
        self.d(|d| d.outer_checked)
    }

    pub fn set_outer_checked(self) {
        self.d_mut(|d| d.outer_checked = true)
    }

    pub fn get_lambda_method_key(self) -> String {
        let (new_type, constructor) = self.d(|d| (d.new_type.clone(), d.constructor));
        let node = root_class_node(new_type.value.as_deref());
        if let (Some(node), Some(constructor)) = (node, constructor) {
            let descriptor = constructor
                .get_bootstrap_arguments()
                .expect("NullPointerException: bootstrapArguments")[0]
                .clone()
                .expect("NullPointerException: bootstrap argument")
                .as_primitive()
                .get_string();
            let lambda_information = node
                .with(|d| d.lambda_information.clone())
                .expect("NullPointerException: lambdaInformation");
            return InterpreterUtil::make_unique_key(&lambda_information.method_name, &descriptor);
        }
        String::new()
    }

    pub fn get_lambda_content_method_key(self) -> String {
        let (new_type, constructor) = self.d(|d| (d.new_type.clone(), d.constructor));
        let node = root_class_node(new_type.value.as_deref());
        if let (Some(node), Some(_)) = (node, constructor) {
            let lambda_information = node
                .with(|d| d.lambda_information.clone())
                .expect("NullPointerException: lambdaInformation");
            return InterpreterUtil::make_unique_key(
                &lambda_information.content_method_name,
                &lambda_information.content_method_descriptor,
            );
        }
        String::new()
    }

    pub fn set_invocation_instance(self) {
        if let Some(constructor) = self.get_constructor() {
            constructor.set_invocation_instance();
        }
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        Exprent::process_sforms_base(self.0, s_forms_constructor, var_maps, stat, calc_live_vars);

        if s_forms_constructor.track_field_vars && self.get_new_type().type_ == CodeType::Object {
            var_maps.get_normal().borrow_mut().remove_all_fields();
        }
    }
}
