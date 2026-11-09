//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/InvocationExprent.java`.
//!
//! kuna fix `anonymous_static_qualifier` (`kuna_fixes`): in `to_java`,
//! a static call of an anonymous class's method from a class nested in it is
//! written unqualified instead of as `<unrepresentable>.m(...)`
//! (`kuna_fix_anonymous_static_qualifier`).

use std::collections::VecDeque;
use std::rc::Rc;

use crate::check_types_result::CheckTypesResult;
use crate::class_writer::ClassWriter;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::classpath_helper::ClasspathHelper;
use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::expr_util::ExprUtil;
use crate::exprent::{Exprent, ExprentKind, ExprentType};
use crate::function_exprent::FunctionType;
use crate::generic_type::GenericType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_matchable::MatchProperties;
use crate::interpreter_util::InterpreterUtil;
use crate::link_constant::LinkConstant;
use crate::list_stack::ListStack;
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::pair::Pair;
use crate::pooled_constant::PooledConstant;
use crate::rt::{BitSet, JHashMap, JHashSet, JList, JavaEq, JavaHash, JavaListExt};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_class::{GenericMap, StructClass};
use crate::struct_method::StructMethod;
use crate::substrate::kuna_fix_anonymous_static_qualifier;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::type_family::TypeFamily;
use crate::var_exprent::VarExprent;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::{get_class, instance_of, str_equals, VarType};
use crate::var_version_pair::VarVersionPair;
use crate::{java_subclass, jthrow};

/// Java `InvocationExprent.InvocationType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum InvocationType {
    Special,
    Virtual,
    Static,
    Interface,
    Dynamic,
    ConstantDynamic,
}

impl InvocationType {
    pub fn name(self) -> &'static str {
        match self {
            InvocationType::Special => "SPECIAL",
            InvocationType::Virtual => "VIRTUAL",
            InvocationType::Static => "STATIC",
            InvocationType::Interface => "INTERFACE",
            InvocationType::Dynamic => "DYNAMIC",
            InvocationType::ConstantDynamic => "CONSTANT_DYNAMIC",
        }
    }
}

impl JavaHash for InvocationType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for InvocationType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// Java `InvocationExprent.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    General,
    Init,
    Clinit,
}

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::General => "GENERAL",
            Type::Init => "INIT",
            Type::Clinit => "CLINIT",
        }
    }
}

impl JavaHash for Type {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for Type {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// Java `InvocationExprent.BoxState`.
#[derive(Clone, Copy, Default, Debug)]
pub struct BoxState {
    pub force_boxing: bool,
    pub force_unboxing: bool,
    pub keep_cast: bool,
}

/// `name`, `classname` and `string_descriptor` are empty (Java: `null`) only
/// between `InvocationExprent::new()` and the setters every caller runs next.
pub struct InvocationExprentData {
    pub name: String,
    pub classname: String,
    pub is_static: bool,
    pub can_ignore_boxing: bool,
    pub functype: Type,
    pub instance: Option<Exprent>,
    pub desc: Option<StructMethod>,
    pub descriptor: Option<MethodDescriptor>,
    pub string_descriptor: String,
    pub invoke_dynamic_class_suffix: Option<String>,
    pub invocation_type: InvocationType,
    pub lst_parameters: JList<Exprent>,
    pub bootstrap_method: Option<LinkConstant>,
    pub bootstrap_arguments: Option<Vec<Option<PooledConstant>>>,
    pub generic_args: Vec<VarType>,
    pub remapped_inst_type: Option<VarType>,
    pub force_generic_qualfication: bool,
    pub generics_map: JHashMap<VarType, Option<VarType>>,
    pub is_invocation_instance: bool,
    pub is_qualifier: bool,
    pub boxing: BoxState,
    pub is_synthetic_null_check: bool,
    pub was_lazy_condy: bool,
}

impl InvocationExprentData {
    fn new() -> InvocationExprentData {
        InvocationExprentData {
            name: String::new(),
            classname: String::new(),
            is_static: false,
            can_ignore_boxing: true,
            functype: Type::General,
            instance: None,
            desc: None,
            descriptor: None,
            string_descriptor: String::new(),
            invoke_dynamic_class_suffix: None,
            invocation_type: InvocationType::Virtual,
            lst_parameters: JList::new(),
            bootstrap_method: None,
            bootstrap_arguments: None,
            generic_args: Vec::new(),
            remapped_inst_type: None,
            force_generic_qualfication: false,
            generics_map: JHashMap::new(),
            is_invocation_instance: false,
            is_qualifier: false,
            boxing: BoxState::default(),
            is_synthetic_null_check: false,
            was_lazy_condy: false,
        }
    }
}

/// Java `BiFunction<StructMethod, MethodDescriptor, Boolean>` of `getMatchedDescriptors`.
pub type ParamMatcher<'a> = &'a dyn Fn(StructMethod, &MethodDescriptor) -> bool;

java_subclass!(pub InvocationExprent: Exprent, ExprentKind::Invocation(Box<InvocationExprentData>),
               is_invocation, as_invocation, try_as_invocation);

impl JavaEq for InvocationExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for InvocationExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

thread_local! {
    static JAVA_NIO_BUFFER_TYPE: VarType = VarType::new_value(CodeType::Object, 0, "java/nio/Buffer");
}

/// `DecompilerContext.getClassProcessor().getMapRootClasses().get(name)`.
fn root_class_node(name: &str) -> Option<ClassNode> {
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

/// `List<String>.contains(value)` where `value` is a possibly-null `String`.
fn contains_value(list: &[String], value: &Option<Rc<str>>) -> bool {
    match value.as_deref() {
        Some(v) => list.iter().any(|s| s.as_str() == v),
        None => false,
    }
}

fn mask_allows(mask: &Option<Vec<Option<VarVersionPair>>>, i: usize) -> bool {
    match mask {
        None => true,
        Some(m) => m[i].is_none(),
    }
}

impl InvocationExprent {
    fn empty_bit_set() -> BitSet {
        BitSet::new()
    }

    #[allow(non_snake_case)]
    fn JAVA_NIO_BUFFER() -> VarType {
        JAVA_NIO_BUFFER_TYPE.with(|t| t.clone())
    }

    /// Java: `InvocationExprent()`.
    #[allow(clippy::new_without_default)]
    pub fn new() -> InvocationExprent {
        let id = Exprent::next_id();
        InvocationExprent(Exprent::alloc_with_id(
            id,
            ExprentKind::Invocation(Box::new(InvocationExprentData::new())),
        ))
    }

    /// Java: `InvocationExprent(int, LinkConstant, LinkConstant, List<PooledConstant>, ListStack<? extends Exprent>, BitSet)`.
    /// The id is taken first (`this()`), before any operand is popped.
    pub fn new_invoke(
        opcode: i32,
        cn: &LinkConstant,
        bootstrap_method: Option<LinkConstant>,
        bootstrap_arguments: Option<Vec<Option<PooledConstant>>>,
        stack: &mut ListStack<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> InvocationExprent {
        let id = Exprent::next_id();
        let mut data = InvocationExprentData::new();

        data.name = cn.elementname.clone();
        data.classname = cn.classname.clone().unwrap_or_default();
        data.bootstrap_method = bootstrap_method.clone();
        data.bootstrap_arguments = bootstrap_arguments.clone();
        match opcode {
            CodeConstants::OPC_INVOKESTATIC => {
                data.invocation_type = InvocationType::Static;
            }
            CodeConstants::OPC_INVOKESPECIAL => {
                data.invocation_type = InvocationType::Special;
            }
            CodeConstants::OPC_INVOKEVIRTUAL => {
                data.invocation_type = InvocationType::Virtual;
            }
            CodeConstants::OPC_INVOKEINTERFACE => {
                data.invocation_type = InvocationType::Interface;
            }
            CodeConstants::OPC_INVOKEDYNAMIC => {
                data.invocation_type = InvocationType::Dynamic;

                let bm = bootstrap_method.as_ref().expect("NullPointerException");
                data.classname = bm.classname.clone().unwrap_or_default();
                data.invoke_dynamic_class_suffix =
                    Some(format!("##Lambda_{}_{}", cn.index1, cn.index2));
            }
            CodeConstants::OPC_LDC | CodeConstants::OPC_LDC_W | CodeConstants::OPC_LDC2_W => {
                data.invocation_type = InvocationType::ConstantDynamic;
                let bm = bootstrap_method.as_ref().expect("NullPointerException");
                data.classname = bm.classname.clone().unwrap_or_default();
                data.invoke_dynamic_class_suffix =
                    Some(format!("##Condy_{}_{}", cn.index1, cn.index2));
            }
            _ => {}
        }

        if data.name == CodeConstants::INIT_NAME {
            data.functype = Type::Init;
        } else if data.name == CodeConstants::CLINIT_NAME {
            data.functype = Type::Clinit;
        }

        data.string_descriptor = cn.descriptor.clone();
        if data.invocation_type == InvocationType::ConstantDynamic {
            data.string_descriptor = format!("(){}", data.string_descriptor);
        }
        let descriptor = MethodDescriptor::parse_descriptor(&data.string_descriptor);

        let mut lst_parameters: Vec<Exprent> = Vec::new();
        for _ in 0..descriptor.params.len() {
            lst_parameters.insert(0, stack.pop());
        }
        data.descriptor = Some(descriptor);

        if opcode == CodeConstants::OPC_INVOKEDYNAMIC
            || data.invocation_type == InvocationType::ConstantDynamic
        {
            let mut dynamic_invocation_type = bootstrap_method
                .as_ref()
                .expect("NullPointerException")
                .index1;
            if let Some(args) = &bootstrap_arguments {
                if args.len() > 1 {
                    if let Some(PooledConstant::Link(link)) = &args[1] {
                        dynamic_invocation_type = link.index1;
                    }
                }
            }
            if dynamic_invocation_type == CodeConstants::CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC {
                data.is_static = true;
            } else if !lst_parameters.is_empty() {
                data.instance = Some(lst_parameters[0]);
            }
        } else if opcode == CodeConstants::OPC_INVOKESTATIC {
            data.is_static = true;
        } else {
            data.instance = Some(stack.pop());
        }
        data.lst_parameters = JList::from(lst_parameters);

        let e = Exprent::alloc_with_id(id, ExprentKind::Invocation(Box::new(data)));
        e.add_bytecode_offsets(bytecode_offsets);
        InvocationExprent(e)
    }

    /// Java: `InvocationExprent(InvocationExprent)` (the copy constructor). The
    /// id is taken before the instance and parameters are copied.
    pub fn new_copy(expr: InvocationExprent) -> InvocationExprent {
        let id = Exprent::next_id();

        let mut data = InvocationExprentData::new();
        let instance = expr.d(|s| {
            data.name = s.name.clone();
            data.classname = s.classname.clone();
            data.is_static = s.is_static;
            data.can_ignore_boxing = s.can_ignore_boxing;
            data.functype = s.functype;
            data.invocation_type = s.invocation_type;
            data.invoke_dynamic_class_suffix = s.invoke_dynamic_class_suffix.clone();
            data.string_descriptor = s.string_descriptor.clone();
            data.descriptor = s.descriptor.clone();
            data.bootstrap_method = s.bootstrap_method.clone();
            data.bootstrap_arguments = s.bootstrap_arguments.clone();
            data.is_synthetic_null_check = s.is_synthetic_null_check;
            data.was_lazy_condy = s.was_lazy_condy;
            s.instance
        });
        data.instance = instance.map(|i| i.copy());

        let mut lst_parameters = expr.get_lst_parameters().to_vec();
        ExprProcessor::copy_entries(&mut lst_parameters);

        if data.invocation_type == InvocationType::Dynamic
            && !data.is_static
            && data.instance.is_some()
            && !lst_parameters.is_empty()
        {
            data.instance = Some(lst_parameters[0]);
        }
        data.lst_parameters = JList::from(lst_parameters);

        let bytecode = expr.get_bytecode();
        let e = Exprent::alloc_with_id(id, ExprentKind::Invocation(Box::new(data)));
        e.add_bytecode_offsets(bytecode.as_ref());
        InvocationExprent(e)
    }

    pub fn get_expr_type(self) -> VarType {
        self.get_descriptor().ret.clone()
    }

    fn gm_contains(self, key: &VarType) -> bool {
        self.d(|d| d.generics_map.contains_key(key))
    }

    fn gm_get(self, key: &VarType) -> Option<VarType> {
        self.d(|d| d.generics_map.get(key).cloned().flatten())
    }

    fn gm_put(self, key: VarType, value: Option<VarType>) {
        self.d_mut(|d| {
            d.generics_map.put(key, value);
        })
    }

    fn remap_with_generics_map(self, type_: &VarType) -> Option<VarType> {
        self.d(|d| type_.remap(&d.generics_map))
    }

    pub fn get_inferred_expr_type(self, upper_bound: Option<&VarType>) -> VarType {
        let (classname, name, string_descriptor) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.string_descriptor.clone(),
            )
        });
        let struct_context = DecompilerContext::get_struct_context();

        if self.d(|d| d.desc.is_none()) {
            let cl = struct_context.get_class(&classname);
            let desc = cl.and_then(|cl| cl.get_method_recursive(&name, &string_descriptor));
            self.d_mut(|d| d.desc = desc);
        }

        self.d_mut(|d| {
            d.generic_args.clear();
            d.generics_map.clear();
            d.remapped_inst_type = None;
        });

        let mth_cls = struct_context.get_class(&classname);

        if self.is_unboxing_call() {
            if let Some(upper_bound) = upper_bound {
                if let Some(func) = self.get_instance().and_then(|i| i.try_as_function()) {
                    if func.get_func_type() == FunctionType::Cast {
                        let inferred = func
                            .get_lst_operands()
                            .get(0)
                            .get_inferred_expr_type(Some(upper_bound));

                        let unboxed = VarType::UNBOXING_TYPES().get(&inferred).cloned();
                        if unboxed.is_none_or(|u| !u.equals(upper_bound))
                            && (inferred.type_family == TypeFamily::Object || inferred.is_generic())
                        {
                            self.d_mut(|d| d.boxing.keep_cast = true);
                        }
                    }
                }
            }
        }

        let desc = self.d(|d| d.desc);
        if let (Some(desc), Some(mth_cls)) = (desc, mth_cls) {
            let is_new = self.get_functype() == Type::Init;
            let is_gen_new = is_new && mth_cls.get_signature().is_some();
            if desc.get_signature().is_some() || is_gen_new {
                let named = self.0.get_named_generics();
                let mut bounds = self.get_generic_bounds(mth_cls);

                let fparams: Vec<String> = if is_gen_new {
                    mth_cls.get_signature().unwrap().fparameters().clone()
                } else {
                    desc.get_signature().unwrap().type_parameters().clone()
                };
                let ret: VarType = if is_gen_new {
                    mth_cls.get_signature().unwrap().generic_type().0
                } else {
                    desc.get_signature()
                        .unwrap()
                        .return_type()
                        .expect("NullPointerException")
                };

                let mut temp_map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
                let mut upper_bounds_map: JHashMap<VarType, VarType> = JHashMap::new();
                let mut hierarchy_map: GenericMap = Rc::new(JHashMap::new());

                let desc_class = desc.get_class_qualified_name();
                if classname != desc_class {
                    let hierarchy = mth_cls.get_all_generics();
                    if hierarchy.contains_key(&desc_class) {
                        hierarchy_map = hierarchy.get(&desc_class).unwrap().clone();
                        for (from, to) in hierarchy_map.iter() {
                            if to.type_ == CodeType::Genvar {
                                if bounds.contains_key(to) && !bounds.contains_key(from) {
                                    let to_bounds = bounds.get(to).unwrap().clone();
                                    bounds.put(from.clone(), to_bounds);
                                }
                            } else if !bounds.contains_key(from) {
                                self.gm_put(from.clone(), Some(to.clone()));
                            }
                        }
                    }
                }

                if let Some(upper) = upper_bound {
                    if !upper.equals(&VarType::VARTYPE_OBJECT())
                        && (upper.type_ != CodeType::Genvar || named.contains_key(upper))
                    {
                        let mut ub: Option<VarType> = Some(upper.clone());
                        let mut r: VarType = ret.clone();
                        if upper.type_ != CodeType::Genvar
                            && r.type_ != CodeType::Genvar
                            && !str_equals(&upper.value, &r.value)
                        {
                            if instance_of(&upper.value, &r.value) {
                                ub = GenericType::get_generic_super_type(upper, &r);
                            } else {
                                r = GenericType::get_generic_super_type(&r, upper)
                                    .expect("NullPointerException");
                            }
                        }

                        if r.type_ == CodeType::Genvar
                            && (upper.type_family == TypeFamily::Object || upper.is_generic())
                        {
                            upper_bounds_map.put(
                                r.resize_array_dim(0),
                                upper.resize_array_dim((upper.array_dim - r.array_dim).max(0)),
                            );
                        } else {
                            self.0.gather_generics(ub.as_ref(), &r, &mut temp_map);
                            for (from, to) in temp_map.entry_vec() {
                                if self.gm_contains(&from) {
                                    continue;
                                }
                                let Some(to) = to else { continue };
                                if to.type_ != CodeType::Genvar || named.contains_key(&to) {
                                    let mut ok = true;

                                    if to.type_ == CodeType::Genvar
                                        && named.contains_key(&from)
                                        && bounds.contains_key(&from)
                                        && bounds
                                            .get(&from)
                                            .unwrap()
                                            .j_contains(&VarType::VARTYPE_OBJECT())
                                        && !bounds.contains_key(&to)
                                        && !named.get(&to).unwrap().j_contains(&from)
                                    {
                                        ok = false;
                                    }

                                    if ok
                                        && self.is_mapping_in_bounds(
                                            &from,
                                            Some(&to),
                                            &named,
                                            &bounds,
                                            &mut JHashSet::new(),
                                        )
                                    {
                                        upper_bounds_map.put(from, to);
                                    }
                                }
                            }
                            temp_map.clear();
                        }
                    }
                }

                for p in &fparams {
                    let t = GenericType::parse(&format!("T{p};"));
                    if !upper_bounds_map.contains_key(&t) {
                        upper_bounds_map.put(t, GenericType::DUMMY_VAR().0);
                    }
                }
                if let Some(sig) = mth_cls.get_signature() {
                    let cls_fparams = sig.fparameters().clone();
                    for p in &cls_fparams {
                        let t = GenericType::parse(&format!("T{p};"));
                        if !upper_bounds_map.contains_key(&t) {
                            upper_bounds_map.put(t, GenericType::DUMMY_VAR().0);
                        }
                    }
                }

                let mut inst_type: Option<VarType> = None;

                if let Some(instance) = self.get_instance().filter(|_| !is_new) {
                    instance.set_invocation_instance();

                    let inst_ub = match mth_cls.get_signature() {
                        Some(sig) => sig.generic_type().remap(&upper_bounds_map),
                        None => upper_bound.cloned(),
                    };
                    let mut it = match instance
                        .try_as_function()
                        .filter(|f| f.get_func_type() == FunctionType::Cast)
                    {
                        Some(func) => func
                            .get_lst_operands()
                            .get(0)
                            .get_inferred_expr_type(inst_ub.as_ref()),
                        None => instance.get_inferred_expr_type(inst_ub.as_ref()),
                    };

                    if it.type_ == CodeType::Genvar && named.contains_key(&it) {
                        it = named.get(&it).unwrap()[0].clone();
                    }

                    if it.is_generic() && it.type_ != CodeType::Genvar {
                        let ginstance = it.as_generic();

                        let cls = get_class(&it.value);
                        if let Some(sig) = cls.and_then(|cls| cls.get_signature()) {
                            sig.generic_type()
                                .map_gen_vars_to(&ginstance, &mut temp_map);
                            for (from, to) in temp_map.entry_vec() {
                                if !contains_value(&fparams, &from.value) {
                                    self.process_generic_mapping(
                                        &from,
                                        to.as_ref(),
                                        &named,
                                        &bounds,
                                    );
                                }
                            }
                            temp_map.clear();
                        }
                    }
                    inst_type = Some(it);
                }

                if upper_bound.is_none() && is_gen_new {
                    if let Some(current_cls) = DecompilerContext::get_current_class_node() {
                        let current_struct = current_cls.with(|d| d.class_struct);
                        if mth_cls == current_struct {
                            let vars = mth_cls
                                .get_signature()
                                .expect("NullPointerException")
                                .generic_type()
                                .get_all_generic_vars();
                            for var in vars {
                                self.gm_put(var.0.clone(), Some(var.0));
                            }
                        } else {
                            let hierarchy = current_struct.get_all_generics();
                            let mth_cls_name = mth_cls.qualified_name();
                            if hierarchy.contains_key(&mth_cls_name) {
                                let map = hierarchy.get(&mth_cls_name).unwrap().clone();
                                for (k, v) in map.iter() {
                                    self.gm_put(k.clone(), Some(v.clone()));
                                }
                            }
                        }
                    }
                }

                if !self.d(|d| d.is_invocation_instance) {
                    for (k, v) in upper_bounds_map.iter() {
                        if contains_value(&fparams, &k.value)
                            && !GenericType::DUMMY_VAR().equals(v)
                            && !self.gm_contains(k)
                        {
                            self.gm_put(k.clone(), Some(v.clone()));
                        }
                    }
                }

                let mut param_generics: JHashSet<VarType> = JHashSet::new();
                let lst_parameters = self.get_lst_parameters();
                if !lst_parameters.is_empty() && desc.get_signature().is_some() {
                    let mut mask: Option<Vec<Option<VarVersionPair>>> = None;
                    let mut start: usize = 0;
                    let new_node = root_class_node(&classname);
                    if let Some(new_node) = new_node {
                        if is_new {
                            mask = ExprUtil::get_synthetic_parameters_mask(
                                new_node,
                                &string_descriptor,
                                lst_parameters.len() as i32,
                            );
                            start = if new_node
                                .with(|d| d.class_struct)
                                .has_modifier(CodeConstants::ACC_ENUM)
                            {
                                2
                            } else {
                                0
                            };
                        } else if !new_node
                            .with(|d| d.enclosing_classes.clone())
                            .borrow()
                            .is_empty()
                        {
                            start =
                                if (new_node.with(|d| d.access) & CodeConstants::ACC_STATIC) == 0 {
                                    1
                                } else {
                                    0
                                };
                        }
                    }

                    let mut common_generics: JHashSet<VarType> = JHashSet::new();
                    let current_node = DecompilerContext::get_current_class_node();
                    let method_wrapper = DecompilerContext::get_current_method_wrapper();
                    if let Some(method_wrapper) = method_wrapper {
                        let current_method = method_wrapper.with(|d| d.method_struct);
                        if let (Some(new_node), Some(current_node)) = (new_node, current_node) {
                            if !desc.has_modifier(CodeConstants::ACC_STATIC)
                                && !current_method.has_modifier(CodeConstants::ACC_STATIC)
                            {
                                let mut parents: Vec<ClassNode> = Vec::new();
                                let mut search = Some(current_node);
                                while let Some(s) = search {
                                    parents.push(s);
                                    search = Self::static_parent(s);
                                }

                                search = Some(new_node);
                                while let Some(s) = search {
                                    let s_struct = s.with(|d| d.class_struct);
                                    if parents.contains(&s) && s_struct.get_signature().is_some() {
                                        let generics =
                                            s_struct.get_signature().unwrap().fparameters().clone();
                                        common_generics.add_all(
                                            generics
                                                .iter()
                                                .map(|generic| {
                                                    GenericType::parse(&format!("T{generic};"))
                                                })
                                                .collect::<Vec<_>>(),
                                        );
                                    }
                                    search = Self::static_parent(s);
                                }
                            }
                        }
                    }

                    let desc_sig = desc.get_signature().unwrap();
                    let mut j: usize = 0;
                    for i in start..lst_parameters.len() {
                        if !mask_allows(&mask, i) {
                            continue;
                        }
                        if desc_sig.parameter_types().len() <= j {
                            continue;
                        }

                        let mut param_type = desc_sig.parameter_types()[j].clone();
                        j += 1;
                        if !param_type.is_generic() {
                            continue;
                        }

                        let parameter = lst_parameters.get(i);
                        let mut excluded: JHashSet<VarType> = JHashSet::new();
                        if parameter.get_type() == ExprentType::New {
                            let new_exprent = parameter.as_new();
                            if new_exprent.is_lambda() {
                                self.collect_lambda_excluded(
                                    new_exprent,
                                    &param_type,
                                    &mut excluded,
                                );
                            }
                        }

                        let mut combined = self.d(|d| d.generics_map.clone());
                        for (k, v) in upper_bounds_map.iter() {
                            if !combined.contains_key(k) {
                                combined.put(k.clone(), Some(v.clone()));
                            }
                        }
                        let param_ub = param_type
                            .remap(&*hierarchy_map)
                            .expect("NullPointerException")
                            .remap(&combined);

                        let mut argtype = match parameter
                            .try_as_function()
                            .filter(|f| f.get_func_type() == FunctionType::Cast)
                        {
                            Some(func) => func
                                .get_lst_operands()
                                .get(0)
                                .get_inferred_expr_type(param_ub.as_ref()),
                            None => parameter.get_inferred_expr_type(param_ub.as_ref()),
                        };

                        if param_ub.is_none()
                            && inst_type.as_ref().is_some_and(|t| t.is_generic_type())
                            && combined.contains_key(&param_type)
                            && combined.get(&param_type).unwrap().is_none()
                            && !VarType::VARTYPE_NULL().equals(&argtype)
                        {
                            combined.put(param_type.clone(), Some(argtype.clone()));

                            let base_type =
                                inst_type.as_ref().unwrap().as_generic().find_base_type();
                            if let Some(base_type) = base_type {
                                let remapped = base_type
                                    .remap(&*hierarchy_map)
                                    .expect("NullPointerException")
                                    .remap(&combined);
                                self.d_mut(|d| d.remapped_inst_type = remapped);
                            }
                        }

                        let param_cls = get_class(&param_type.value);
                        let cls = if argtype.type_ != CodeType::Genvar {
                            get_class(&argtype.value)
                        } else {
                            None
                        };

                        if cls.is_some() && param_cls.is_some() {
                            if param_type.is_generic()
                                && !str_equals(&param_type.value, &argtype.value)
                            {
                                argtype =
                                    GenericType::get_generic_super_type(&argtype, &param_type)
                                        .expect("NullPointerException");
                            }

                            if param_type.is_generic() && argtype.is_generic() {
                                let gen_param_type = param_type.as_generic();
                                let gen_arg_type = argtype.as_generic();

                                gen_param_type.map_gen_vars_to(&gen_arg_type, &mut temp_map);
                                GenericType::clean_lowered_generic_types(
                                    &mut temp_map,
                                    &gen_param_type,
                                    &gen_arg_type,
                                    Some(&common_generics),
                                );
                                for (from, to) in temp_map.entry_vec() {
                                    if !excluded.contains(&from) {
                                        param_generics.add(from.clone());
                                    }
                                    if !self.process_generic_mapping(
                                        &from,
                                        to.as_ref(),
                                        &named,
                                        &bounds,
                                    ) {
                                        let current = self.gm_get(&from);
                                        if let (Some(to), Some(current)) = (&to, &current) {
                                            if !current.is_generic() && !to.is_generic() {
                                                self.put_generic_mapping(
                                                    &from,
                                                    Some(to),
                                                    &named,
                                                    &bounds,
                                                );
                                            }
                                        }
                                    }
                                }
                                temp_map.clear();
                            }
                        } else if param_type.type_ == CodeType::Genvar
                            && !param_type.equals(&argtype)
                            && argtype.array_dim >= param_type.array_dim
                        {
                            if param_type.array_dim > 0 {
                                argtype = argtype
                                    .resize_array_dim(argtype.array_dim - param_type.array_dim);
                                param_type = param_type.resize_array_dim(0);
                            }
                            if !excluded.contains(&param_type) {
                                param_generics.add(param_type.clone());
                            }
                            self.process_generic_mapping(
                                &param_type,
                                Some(&argtype),
                                &named,
                                &bounds,
                            );
                        }
                    }
                }

                for (k, v) in upper_bounds_map.entry_vec() {
                    if contains_value(&fparams, &k.value) && !GenericType::DUMMY_VAR().equals(&v) {
                        let current = self.gm_get(&k);
                        if let Some(current) = &current {
                            if !current.is_generic()
                                && !v.is_generic()
                                && !instance_of(&current.value, &v.value)
                            {
                                continue;
                            }
                        }
                        self.process_generic_mapping(&k, Some(&v), &named, &bounds);
                    }
                }

                if !self.d(|d| d.generics_map.is_empty()) {
                    let remapped_ret = ret.remap(&*hierarchy_map).expect("NullPointerException");

                    let mut skip_args = true;
                    if !fparams.is_empty() && remapped_ret.is_generic() {
                        for gen_var in remapped_ret.as_generic().get_all_generic_vars() {
                            if contains_value(&fparams, &gen_var.value) {
                                skip_args = false;
                                break;
                            }
                        }
                    }

                    let mut new_ret = self.remap_with_generics_map(&remapped_ret);
                    if new_ret.is_none() && bounds.get(&ret).is_some_and(|b| !b.is_empty()) {
                        let first_bound = bounds.get(&ret).unwrap()[0].clone();
                        new_ret = self.remap_with_generics_map(&first_bound);
                    }

                    if !skip_args && (!is_new || is_gen_new) {
                        let mut missing = param_generics.is_empty();

                        if !missing {
                            for param in &fparams {
                                if !param_generics
                                    .contains(&GenericType::parse(&format!("T{param};")))
                                {
                                    missing = true;
                                    break;
                                }
                            }
                        }

                        let is_invocation_instance = self.d(|d| d.is_invocation_instance);
                        let mut suppress = (!missing || !is_invocation_instance)
                            && new_ret.as_ref().is_some_and(|nr| {
                                upper_bound.is_none()
                                    || !nr.is_generic()
                                    || instance_of(&nr.value, &upper_bound.unwrap().value)
                            });

                        if self.d(|d| d.force_generic_qualfication) {
                            suppress = false;
                        }

                        if !suppress
                            || DecompilerContext::get_option(
                                IFernflowerPreferences::EXPLICIT_GENERIC_ARGUMENTS,
                            )
                        {
                            let generics_map = self.d(|d| d.generics_map.clone());
                            let mut generic_args =
                                self.d_mut(|d| std::mem::take(&mut d.generic_args));
                            self.0.get_generic_args_into(
                                &fparams,
                                &generics_map,
                                &mut generic_args,
                            );
                            self.d_mut(|d| d.generic_args = generic_args);
                        } else if is_gen_new {
                            self.d_mut(|d| d.generic_args.push(GenericType::DUMMY_VAR().0));
                        }
                    }

                    if let Some(nr) = new_ret {
                        if !nr.ptr_eq(&ret) {
                            let named_keys = JHashSet::from_collection(named.key_vec());
                            if !(nr.is_generic()
                                && nr.as_generic().has_unknown_generic_type(&named_keys))
                            {
                                return nr;
                            }
                        }
                    }
                }

                if ret.is_generic() && ret.as_generic().get_all_generic_vars().is_empty() {
                    return ret;
                }
            }
        }

        self.get_expr_type()
    }

    /// `(search.access & ACC_STATIC) == 0 ? search.parent : null`.
    fn static_parent(search: ClassNode) -> Option<ClassNode> {
        let (access, parent) = search.with(|d| (d.access, d.parent));
        if (access & CodeConstants::ACC_STATIC) == 0 {
            parent
        } else {
            None
        }
    }

    /// The lambda-parameter block of `getInferredExprType` that fills `excluded`.
    fn collect_lambda_excluded(
        self,
        new_exprent: NewExprent,
        param_type: &VarType,
        excluded: &mut JHashSet<VarType>,
    ) {
        let node = new_exprent
            .get_new_type()
            .value
            .as_deref()
            .and_then(root_class_node)
            .expect("NullPointerException");
        let lambda_information = node
            .with(|d| d.lambda_information.clone())
            .expect("NullPointerException");
        let mut potential_method_count: i32 = i32::MAX;
        if lambda_information.is_method_reference {
            let content = DecompilerContext::get_struct_context()
                .get_class(&lambda_information.content_class_name);

            if let Some(content) = content {
                let current_cls = DecompilerContext::get_current_class();
                let mut count: i64 = 0;
                for method in content.get_methods().to_vec() {
                    if self.can_access(current_cls, method)
                        && lambda_information.content_method_name == method.get_name()
                    {
                        count += 1;
                    }
                }
                potential_method_count = count as i32;
            }
        }
        if potential_method_count <= 1 {
            return;
        }

        let Some(base) = get_class(&new_exprent.0.get_expr_type().value) else {
            return;
        };
        let mut found: Option<StructMethod> = None;
        for method in base.get_methods().to_vec() {
            if !method.has_modifier(CodeConstants::ACC_STATIC)
                && method.get_instruction_sequence().is_none()
            {
                found = Some(method);
                break;
            }
        }
        let Some(found) = found else {
            return;
        };

        let mut genvars: JHashMap<VarType, Option<VarType>> = JHashMap::new();
        if let (Some(base_sig), Some(found_sig)) = (base.get_signature(), found.get_signature()) {
            base_sig
                .generic_type()
                .map_gen_vars_to(&param_type.as_generic(), &mut genvars);
            let parameter_types = found_sig.parameter_types().clone();
            let mut collected: Vec<VarType> = Vec::new();
            for pt in parameter_types.iter().filter(|pt| pt.is_generic()) {
                for var in pt.as_generic().get_all_generic_vars() {
                    let Some(mapped) = genvars.get(&var.0).cloned().flatten() else {
                        continue;
                    };
                    if !mapped.is_generic() {
                        continue;
                    }
                    for inner in mapped.as_generic().get_all_generic_vars() {
                        collected.push(inner.0);
                    }
                }
            }
            excluded.add_all(collected);
        }
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let mut result = CheckTypesResult::new();

        if let Some(instance) = self.get_instance() {
            result.add_expr_lower_bound(
                instance,
                &VarType::find_family_bottom(instance.get_expr_type().type_family),
            );
            result.add_expr_upper_bound(instance, &instance.get_expr_type());
        }

        let lst_parameters = self.get_lst_parameters();
        for i in 0..lst_parameters.len() {
            let parameter = lst_parameters.get(i);

            let left_type = self.get_descriptor().params[i].clone();

            result.add_expr_lower_bound(
                parameter,
                &VarType::find_family_bottom(left_type.type_family),
            );
            result.add_expr_upper_bound(parameter, &left_type);
        }

        Some(result)
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            if let Some(instance) = d.instance {
                lst.push(instance);
            }
            lst.extend(d.lst_parameters.iter());
        });
    }

    pub fn copy(self) -> Exprent {
        InvocationExprent::new_copy(self).0
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();

        if self.d(|d| d.was_lazy_condy) {
            buf.append("/* $VF: constant dynamic replaced with non-lazy method call */ ");
        }

        let mut super_qualifier: Option<String> = None;
        let mut is_instance_this = false;

        let instance = self.get_instance();
        if let Some(inv) = instance.and_then(|i| i.try_as_invocation()) {
            inv.mark_using_boxing_result();
        }

        let mut pushed_call_chain_group = false;

        let (classname, name, invocation_type, is_static) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.invocation_type,
                d.is_static,
            )
        });
        let struct_context = DecompilerContext::get_struct_context();

        if is_static
            || invocation_type == InvocationType::Dynamic
            || invocation_type == InvocationType::ConstantDynamic
        {
            if self.is_boxing_call()
                && self.d(|d| d.can_ignore_boxing)
                && !self.d(|d| d.boxing.force_boxing)
            {
                ExprProcessor::get_casted_exprent_full(
                    self.get_lst_parameters().get(0),
                    &self.get_descriptor().params[0],
                    &mut buf,
                    indent,
                    NullCastType::DontCast,
                    false,
                    true,
                    false,
                );
                buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                return buf;
            }

            if invocation_type == InvocationType::ConstantDynamic {
                buf.append("(")
                    .append_cast_type_name(&self.get_descriptor().ret)
                    .append(")");
            }

            let node = DecompilerContext::get_current_class_node();
            if node.is_none_or(|n| classname != n.with(|d| d.class_struct).qualified_name()) {
                let short_name = DecompilerContext::get_import_collector()
                    .get_short_name_in_class_context(&ExprProcessor::build_java_class_name(
                        &classname,
                    ));
                if !kuna_fix_anonymous_static_qualifier::drop_qualifier(
                    &classname,
                    &self.get_name(),
                    &short_name,
                ) {
                    buf.append_all_classes(&short_name, Some(&classname));
                }
            }
        } else {
            if let Some(inst_var) = instance.and_then(|i| i.try_as_var()) {
                let var_pair = VarVersionPair::from_var(inst_var);

                let mut var_proc = inst_var.get_processor();
                if var_proc.is_none() {
                    if let Some(current_method) = DecompilerContext::get_current_method_wrapper() {
                        var_proc = Some(current_method.with(|d| d.varproc));
                    }
                }

                let mut this_classname: Option<String> = None;
                if let Some(var_proc) = var_proc {
                    this_classname = var_proc.get_this_vars().borrow().get(&var_pair).cloned();
                }

                if let Some(this_classname) = this_classname {
                    is_instance_this = true;

                    if invocation_type == InvocationType::Special && classname != this_classname {
                        let cl = struct_context.get_class(&classname);
                        let is_interface =
                            cl.is_some_and(|cl| cl.has_modifier(CodeConstants::ACC_INTERFACE));
                        super_qualifier = Some(if !is_interface {
                            this_classname
                        } else {
                            classname.clone()
                        });
                    }
                }
            }

            if CodeConstants::is_return_polymorphic(&classname, &name)
                && !self.get_descriptor().ret.equals(&VarType::VARTYPE_VOID())
            {
                buf.append("(")
                    .append_cast_type_name(&self.get_descriptor().ret)
                    .append(")");
            }

            if self.get_functype() == Type::General {
                if let Some(super_qualifier) = &super_qualifier {
                    TextUtil::write_qualified_super(&mut buf, super_qualifier);
                } else if let Some(instance) = instance {
                    let cl = struct_context.get_class(&classname);

                    let mut left_type: Option<VarType> =
                        Some(VarType::new_value(CodeType::Object, 0, classname.as_str()));
                    if !self.d(|d| d.generics_map.is_empty()) {
                        if let Some(sig) = cl.and_then(|cl| cl.get_signature()) {
                            let generic_type = sig.generic_type();
                            let new = self.remap_with_generics_map(&generic_type.0);
                            if !new.as_ref().is_some_and(|n| n.ptr_eq(&generic_type.0)) {
                                left_type = new;
                            }
                        }
                    }

                    instance.set_invocation_instance();
                    let right_type = instance.get_inferred_expr_type(left_type.as_ref());

                    if self.is_unboxing_call() && !self.d(|d| d.boxing.force_unboxing) {
                        buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                        if let Some(func) = instance.try_as_function() {
                            if func.get_func_type() == FunctionType::Cast
                                && func.get_lst_operands().get(1).is_const()
                                && !self.d(|d| d.boxing.keep_cast)
                            {
                                let constexpr = func.get_lst_operands().get(1).as_const();
                                let mut skip_cast = false;

                                let first_param = func.get_lst_operands().get(0);
                                if first_param.is_var() || first_param.is_field() {
                                    let inferred =
                                        first_param.get_inferred_expr_type(left_type.as_ref());
                                    skip_cast = (inferred.type_ != CodeType::Object
                                        && inferred.type_ != CodeType::Genvar)
                                        || inferred.value.as_deref().is_some_and(|v| {
                                            struct_context.instance_of(v, &classname)
                                        });
                                } else if constexpr.get_const_type().value.as_deref()
                                    == Some(classname.as_str())
                                {
                                    skip_cast = true;
                                }

                                if skip_cast {
                                    buf.append_buffer(&first_param.to_java(indent));
                                    return buf;
                                }
                            }
                        }

                        buf.append_buffer(&instance.to_java(indent));
                        return buf;
                    }

                    instance.set_is_qualifier();

                    if !self.d(|d| d.is_qualifier) {
                        buf.push_newline_group(indent, 1);
                        pushed_call_chain_group = true;
                    }
                    let res = instance.to_java(indent);

                    let inst_node = root_class_node(&classname);
                    let java_nio_buffer = Self::JAVA_NIO_BUFFER();
                    let remapped_inst_type = self.d(|d| d.remapped_inst_type.clone());
                    if right_type.equals(&VarType::VARTYPE_OBJECT())
                        && !left_type
                            .as_ref()
                            .expect("NullPointerException")
                            .equals(&right_type)
                        && inst_node
                            .is_some_and(|n| n.with(|d| d.type_) != ClassNodeType::Anonymous)
                    {
                        self.append_inst_cast(
                            &mut buf,
                            left_type.as_ref().expect("NullPointerException"),
                            res,
                        );
                    } else if let Some(remapped_inst_type) = remapped_inst_type {
                        self.append_inst_cast(&mut buf, &remapped_inst_type, res);
                    } else if instance.get_precedence() > self.0.get_precedence()
                        && !self.can_skip_paren_enclose(instance)
                    {
                        buf.append("(").append_buffer(&res).append(")");
                    } else if java_nio_buffer.equals(&self.get_descriptor().ret)
                        && !java_nio_buffer.equals(&right_type)
                        && instance_of(&right_type.value, &java_nio_buffer.value)
                    {
                        buf.append("((")
                            .append_cast_type_name(&java_nio_buffer)
                            .append(")")
                            .append_buffer(&res)
                            .append(")");
                    } else {
                        buf.append_buffer(&res);
                    }
                    if instance.allow_newline_after_qualifier() {
                        buf.append_possible_newline();
                    }
                }
            }
        }

        match self.get_functype() {
            Type::General => {
                if buf.content_equals(VarExprent::VAR_NAMELESS_ENCLOSURE) {
                    buf.set_length(0);
                }

                if buf.length() > 0 {
                    buf.append(".");
                    let generic_args = self.get_generic_args();
                    self.0.append_parameters(&mut buf, &generic_args);
                }

                buf.add_bytecode_mapping(self.get_bytecode().as_ref());

                let valid_name =
                    ClassWriter::to_valid_java_identifier(Some(&name)).unwrap_or_default();
                let descriptor = self.get_descriptor();

                if invocation_type == InvocationType::Dynamic
                    || invocation_type == InvocationType::ConstantDynamic
                {
                    match self.get_bootstrap_method() {
                        None => {
                            buf.append("<").append_method_descriptor(
                                &valid_name,
                                false,
                                &classname,
                                &name,
                                &descriptor,
                            );
                            if valid_name != name {
                                buf.append("/* $VF was: ").append(&name).append(" */");
                            }

                            if invocation_type == InvocationType::Dynamic {
                                buf.append(">invokedynamic");
                            } else {
                                buf.append(">ldc");
                            }
                        }
                        Some(bootstrap_method) => {
                            buf.append(&bootstrap_method.elementname);
                            buf.append("<\"");
                            buf.append_method_descriptor(
                                &valid_name,
                                false,
                                &classname,
                                &name,
                                &descriptor,
                            );

                            if valid_name != name {
                                buf.append("/* $VF was: ").append(&name).append(" */");
                            }

                            buf.append("\"");
                            let bootstrap_arguments = self
                                .get_bootstrap_arguments()
                                .expect("NullPointerException");
                            for arg in &bootstrap_arguments {
                                buf.append(",");
                                Self::append_bootstrap_argument(&mut buf, arg.as_ref());
                            }
                            buf.append(">");
                        }
                    }
                } else {
                    buf.append_method_descriptor(
                        &valid_name,
                        false,
                        &classname,
                        &name,
                        &descriptor,
                    );
                    if valid_name != name {
                        buf.append("/* $VF was: ").append(&name).append(" */");
                    }
                }

                buf.append("(");
            }
            Type::Clinit => {
                jthrow!("Explicit invocation of {}", CodeConstants::CLINIT_NAME)
            }
            Type::Init => {
                buf.add_bytecode_mapping(self.get_bytecode().as_ref());
                if super_qualifier.is_some() {
                    buf.append("super(");
                } else if is_instance_this {
                    buf.append("this(");
                } else if let Some(instance) = instance {
                    let mut s = String::from(".");
                    if DecompilerContext::get_option(IFernflowerPreferences::DECOMPILER_COMMENTS) {
                        s.push_str("/* $VF: Unable to resugar constructor */");
                    }
                    s.push_str("<init>(");

                    buf.append_buffer(&instance.to_java(indent)).append(&s);
                } else {
                    jthrow!("Unrecognized invocation of {}", CodeConstants::INIT_NAME)
                }
            }
        }

        let param_list = self.append_param_list(indent);
        buf.append_buffer(&param_list).append(")");
        if pushed_call_chain_group {
            buf.pop_newline_group();
        }
        buf
    }

    fn append_inst_cast(self, buf: &mut TextBuffer, left_type: &VarType, mut res: TextBuffer) {
        buf.append("((")
            .append_cast_type_name(left_type)
            .append(")");

        let instance = self.get_instance().expect("NullPointerException");
        if instance.get_precedence() >= FunctionType::Cast.precedence() {
            res.enclose_with_parens();
        }
        buf.append_buffer(&res).append(")");
    }

    fn can_skip_paren_enclose(self, instance: Exprent) -> bool {
        let Some(new_expr) = instance.try_as_new() else {
            return false;
        };

        if !new_expr.is_anonymous() && !new_expr.is_lambda() && !new_expr.is_method_reference() {
            return self.get_functype() == Type::General;
        }

        false
    }

    fn append_bootstrap_argument(buf: &mut TextBuffer, arg: Option<&PooledConstant>) {
        match arg {
            Some(PooledConstant::Primitive(prim)) => {
                let string_value = match &prim.value {
                    Some(value) => value.to_java_string(),
                    None => "null".to_string(),
                };
                if prim.type_ == CodeConstants::CONSTANT_CLASS {
                    buf.append_cast_type_name(&VarType::new_signature(&string_value));
                } else if prim.type_ == CodeConstants::CONSTANT_STRING {
                    buf.append("\"")
                        .append(&crate::const_exprent::ConstExprent::convert_string_to_java(
                            &string_value,
                            false,
                        ))
                        .append("\"");
                } else {
                    buf.append(&string_value);
                }
            }
            Some(PooledConstant::Link(link)) => {
                if let Some(classname) = &link.classname {
                    buf.append_cast_type_name(&VarType::new_signature(classname))
                        .append("::")
                        .append(&link.elementname);
                } else if !link.descriptor.is_empty() {
                    buf.append("/* VF: Constant Dynamic */ (")
                        .append_cast_type_name(&VarType::new_signature(&link.descriptor))
                        .append(") ")
                        .append(&link.elementname);
                }
            }
            None => {}
        }
    }

    pub fn append_param_list(self, indent: i32) -> TextBuffer {
        let (classname, name, string_descriptor) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.string_descriptor.clone(),
            )
        });
        let struct_context = DecompilerContext::get_struct_context();
        let lst_parameters = self.get_lst_parameters();
        let descriptor = self.get_descriptor();

        let mut mask: Option<Vec<Option<VarVersionPair>>> = None;
        let mut is_enum = false;
        if self.get_functype() == Type::Init {
            if let Some(new_node) = root_class_node(&classname) {
                mask = ExprUtil::get_synthetic_parameters_mask(
                    new_node,
                    &string_descriptor,
                    lst_parameters.len() as i32,
                );
                is_enum = new_node
                    .with(|d| d.class_struct)
                    .has_modifier(CodeConstants::ACC_ENUM)
                    && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM);
            }
        }
        let curr_cls = DecompilerContext::get_current_class_node();
        let matches = self.get_matched_descriptors(false);
        let ambiguous_vararg = self.is_varargs_ambiguous(&self.get_matched_descriptors(true));
        let set_ambiguous_parameters = self.get_ambiguous_parameters(&matches);

        if lst_parameters.len() == descriptor.params.len()
            && self.is_var_arg_call()
            && !lst_parameters.is_empty()
        {
            let last_param = lst_parameters.get(lst_parameters.len() - 1);
            if let Some(last_new) = last_param.try_as_new() {
                if last_param.get_expr_type().array_dim >= 1
                    && (!ambiguous_vararg || last_new.get_lst_array_elements().len() != 1)
                {
                    last_new.set_var_arg_param(true);
                }
            }
        }

        let start: usize = if is_enum { 2 } else { 0 };
        let mut parameters: Vec<Exprent> = lst_parameters.to_vec();
        let mut types: Vec<VarType> = descriptor.params.clone();
        for i in start..parameters.len() {
            let par = parameters[i];
            let Some(inv) = par.try_as_invocation() else {
                continue;
            };
            if inv.is_boxing_call() {
                let value = inv.get_lst_parameters().get(0);
                types[i] = value.get_expr_type();
                if types[i].type_family == TypeFamily::Integer {
                    let inv_classname = inv.get_classname();
                    types[i] = if inv_classname == "java/lang/Short" {
                        VarType::VARTYPE_SHORT()
                    } else if inv_classname == "java/lang/Byte" {
                        VarType::VARTYPE_BYTE()
                    } else if inv_classname == "java/lang/Integer" {
                        VarType::VARTYPE_INT()
                    } else {
                        VarType::VARTYPE_CHAR()
                    };
                }

                let mut count: usize = 0;
                let st_class = struct_context.get_class(&classname);
                if st_class.is_some() {
                    let types_now = &types;
                    let matcher = |_mt: StructMethod, md: &MethodDescriptor| -> bool {
                        if md.params.len() == descriptor.params.len() {
                            for x in 0..md.params.len() {
                                if md.params[x].type_family != descriptor.params[x].type_family
                                    && md.params[x].type_family != types_now[x].type_family
                                {
                                    return false;
                                }

                                if md.params[x].array_dim != descriptor.params[x].array_dim
                                    && md.params[x].array_dim != types_now[x].array_dim
                                {
                                    return false;
                                }
                            }
                            return true;
                        }

                        false
                    };
                    let custom_matched_descriptors =
                        self.get_matched_descriptors_custom(false, Some(&matcher));
                    count = custom_matched_descriptors.len();
                }

                if count != matches.len() {
                    types[i] = descriptor.params[i].clone();
                    inv.force_boxing(true);
                } else {
                    value.add_bytecode_offsets(inv.get_bytecode().as_ref());
                    parameters[i] = value;
                }
            } else if inv.is_unboxing_call() && !inv.should_force_unboxing() {
                let st_class = struct_context.get_class(&classname);

                if let Some(st_class) = st_class {
                    for mt in st_class.get_methods().to_vec() {
                        if name == mt.get_name()
                            && curr_cls.is_none_or(|c| {
                                self.can_access(Some(c.with(|d| d.class_struct)), mt)
                            })
                            && string_descriptor != mt.get_descriptor()
                        {
                            let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
                            if md.params.len() == descriptor.params.len()
                                && md.params[i].type_ == CodeType::Object
                                && instance_of(
                                    &inv.get_instance()
                                        .expect("NullPointerException")
                                        .get_expr_type()
                                        .value,
                                    &md.params[i].value,
                                )
                            {
                                inv.force_unboxing(true);
                                break;
                            }
                        }
                    }
                }
            }
        }

        if self.d(|d| d.desc.is_none()) {
            self.get_inferred_expr_type(None);

            let instance = self.get_instance();
            if self.d(|d| d.generics_map.is_empty())
                && instance.is_some()
                && self.get_functype() != Type::Init
            {
                let inst_type = instance.unwrap().get_inferred_expr_type(None);
                if inst_type.is_generic() && inst_type.type_ != CodeType::Genvar {
                    let ginstance = inst_type.as_generic();

                    let cls = get_class(&inst_type.value);
                    if let Some(sig) = cls.and_then(|cls| cls.get_signature()) {
                        let mut generics_map =
                            self.d_mut(|d| std::mem::take(&mut d.generics_map));
                        sig.generic_type()
                            .map_gen_vars_to(&ginstance, &mut generics_map);
                        self.d_mut(|d| d.generics_map = generics_map);
                    }
                }
            }
        }
        let desc = self.d(|d| d.desc);
        if let Some((desc, desc_sig)) =
            desc.and_then(|desc| desc.get_signature().map(|s| (desc, s)))
        {
            let mut hierarchy_map: GenericMap = Rc::new(JHashMap::new());
            let desc_class = desc.get_class_qualified_name();
            if classname != desc_class {
                if let Some(mth_cls) = struct_context.get_class(&classname) {
                    let hierarchy = mth_cls.get_all_generics();
                    if hierarchy.contains_key(&desc_class) {
                        hierarchy_map = hierarchy.get(&desc_class).unwrap().clone();
                    }
                }
            }

            let named_gens = JHashSet::from_collection(self.0.get_named_generics().key_vec());
            let mut y: usize = 0;
            for x in start..types.len() {
                if !mask_allows(&mask, x) {
                    continue;
                }
                if desc_sig.parameter_types().len() <= y {
                    continue;
                }

                let parameter_type = desc_sig.parameter_types()[y].clone();
                y += 1;
                let type_ = self.remap_with_generics_map(
                    &parameter_type
                        .remap(&*hierarchy_map)
                        .expect("NullPointerException"),
                );
                if let Some(type_) = type_ {
                    if !(type_.is_generic()
                        && type_.as_generic().has_unknown_generic_type(&named_gens))
                    {
                        types[x] = type_;
                    }
                }
            }
        }

        let mut buf = TextBuffer::new();

        let mut first_parameter = true;
        if !lst_parameters.is_empty() {
            buf.push_newline_group(indent, 1);
            buf.append_possible_newline();
            buf.push_newline_group(indent, 0);
        }

        for i in start..lst_parameters.len() {
            if !mask_allows(&mask, i) {
                continue;
            }
            let mut buff = TextBuffer::new();
            let ambiguous = set_ambiguous_parameters.get(i);

            if i == parameters.len() - 1
                && lst_parameters
                    .get(i)
                    .get_expr_type()
                    .ptr_eq(&VarType::VARTYPE_NULL())
                && descriptor.params[i]
                    .value
                    .as_deref()
                    .is_some_and(NewExprent::probably_synthetic_parameter)
            {
                break;
            }

            ExprProcessor::get_casted_exprent_full(
                lst_parameters.get(i),
                &types[i],
                &mut buff,
                indent,
                if ambiguous {
                    NullCastType::Cast
                } else {
                    NullCastType::DontCastAtAll
                },
                ambiguous,
                true,
                true,
            );

            if buff.length() > 0 {
                if !first_parameter {
                    buf.append(",").append_possible_newline_alt(" ");
                }
                buf.append_buffer(&buff);
            }

            first_parameter = false;
        }

        if !lst_parameters.is_empty() {
            buf.pop_newline_group();
            buf.append_possible_newline_dedent("", true);
            buf.pop_newline_group();
        }

        buf
    }

    fn is_var_arg_call(self) -> bool {
        let (classname, name, string_descriptor) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.string_descriptor.clone(),
            )
        });
        let cl = DecompilerContext::get_struct_context().get_class(&classname);
        if let Some(cl) = cl {
            let mut mt =
                cl.get_method_key(&InterpreterUtil::make_unique_key(&name, &string_descriptor));
            if mt.is_none() {
                mt = cl.get_method_recursive(&name, &string_descriptor);
            }
            if let Some(mt) = mt {
                return mt.has_modifier(CodeConstants::ACC_VARARGS);
            }
        } else {
            let mtd = ClasspathHelper::find_method(&classname, &name, &self.get_descriptor());
            return mtd.is_some_and(|m| m.is_var_args());
        }
        false
    }

    pub fn is_boxing_call(self) -> bool {
        let (is_static, name, classname, lst_parameters) = self.d(|d| {
            (
                d.is_static,
                d.name.clone(),
                d.classname.clone(),
                d.lst_parameters.clone(),
            )
        });
        if is_static && name == "valueOf" && lst_parameters.len() == 1 {
            let param = lst_parameters.get(0);
            let param_type = param.get_expr_type().type_;

            if param.is_const() {
                if param.get_expr_type().type_family == TypeFamily::Integer
                    && classname == "java/lang/Integer"
                {
                    return true;
                }

                if (param_type == CodeType::Bytechar || param_type == CodeType::Shortchar)
                    && (classname == "java/lang/Character" || classname == "java/lang/Short")
                {
                    return true;
                }
            }

            return InvocationExprent::get_class_name_for_primitive_type(param_type)
                == Some(classname.as_str());
        }

        false
    }

    pub fn mark_using_boxing_result(self) {
        self.d_mut(|d| d.can_ignore_boxing = false)
    }

    pub fn set_is_qualifier(self) {
        self.d_mut(|d| d.is_qualifier = true)
    }

    fn get_class_name_for_primitive_type(type_: CodeType) -> Option<&'static str> {
        match type_ {
            CodeType::Boolean => Some("java/lang/Boolean"),
            CodeType::Byte | CodeType::Bytechar => Some("java/lang/Byte"),
            CodeType::Char => Some("java/lang/Character"),
            CodeType::Short | CodeType::Shortchar => Some("java/lang/Short"),
            CodeType::Int => Some("java/lang/Integer"),
            CodeType::Long => Some("java/lang/Long"),
            CodeType::Float => Some("java/lang/Float"),
            CodeType::Double => Some("java/lang/Double"),
            _ => None,
        }
    }

    /// Java `UNBOXING_METHODS.get(name)`.
    fn unboxing_method_class(name: &str) -> Option<&'static str> {
        match name {
            "booleanValue" => Some("java/lang/Boolean"),
            "byteValue" => Some("java/lang/Byte"),
            "shortValue" => Some("java/lang/Short"),
            "intValue" => Some("java/lang/Integer"),
            "longValue" => Some("java/lang/Long"),
            "floatValue" => Some("java/lang/Float"),
            "doubleValue" => Some("java/lang/Double"),
            "charValue" => Some("java/lang/Character"),
            _ => None,
        }
    }

    pub fn is_unboxing_call(self) -> bool {
        self.d(|d| {
            !d.is_static
                && d.lst_parameters.is_empty()
                && InvocationExprent::unboxing_method_class(&d.name) == Some(d.classname.as_str())
        })
    }

    pub fn force_boxing(self, value: bool) {
        self.d_mut(|d| d.boxing.force_boxing = value)
    }

    pub fn should_force_boxing(self) -> bool {
        self.d(|d| d.boxing.force_boxing)
    }

    pub fn force_unboxing(self, value: bool) {
        self.d_mut(|d| d.boxing.force_unboxing = value)
    }

    pub fn should_force_unboxing(self) -> bool {
        self.d(|d| d.boxing.force_unboxing)
    }

    /// Java: `getMatchedDescriptors(boolean)`.
    fn get_matched_descriptors(self, varargs: bool) -> Vec<StructMethod> {
        self.get_matched_descriptors_custom(varargs, None)
    }

    /// Java: `getMatchedDescriptors(boolean, BiFunction<StructMethod, MethodDescriptor, Boolean>)`.
    fn get_matched_descriptors_custom(
        self,
        varargs: bool,
        custom_param_matcher: Option<ParamMatcher<'_>>,
    ) -> Vec<StructMethod> {
        let mut matches: Vec<StructMethod> = Vec::new();
        let curr_cls = DecompilerContext::get_current_class_node();
        let struct_context = DecompilerContext::get_struct_context();
        let (classname, name, string_descriptor) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.string_descriptor.clone(),
            )
        });
        let Some(cl) = struct_context.get_class(&classname) else {
            return matches;
        };
        let current_method =
            cl.get_method_key(&InterpreterUtil::make_unique_key(&name, &string_descriptor));
        let descriptor = self.get_descriptor();

        let mut visited: JHashSet<String> = JHashSet::new();
        let mut que: VecDeque<StructClass> = VecDeque::new();
        que.push_back(cl);

        while let Some(cls) = que.pop_front() {
            for mt in cls.get_methods().to_vec() {
                if mt.name_eq(&name) {
                    let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
                    let matched_params = match custom_param_matcher {
                        None => {
                            if varargs {
                                self.matches_varargs(
                                    &md.params,
                                    &descriptor.params,
                                    mt,
                                    current_method,
                                )
                            } else {
                                self.matches(&md.params, &descriptor.params)
                            }
                        }
                        Some(matcher) => matcher(mt, &md),
                    };
                    if matched_params
                        && curr_cls
                            .is_none_or(|c| self.can_access(Some(c.with(|d| d.class_struct)), mt))
                    {
                        matches.push(mt);
                    }
                }
            }

            if cls == cl && !matches.is_empty() {
                return matches;
            }

            visited.add(cls.qualified_name());
            if let Some(super_class) = cls.super_class() {
                let super_name = super_class.get_string().to_string();
                if !visited.contains(&super_name) {
                    if let Some(tmp) = struct_context.get_class(&super_name) {
                        que.push_back(tmp);
                    }
                }
            }

            for intf in cls.get_interface_names() {
                if !visited.contains(&intf) {
                    if let Some(tmp) = struct_context.get_class(&intf) {
                        que.push_back(tmp);
                    }
                }
            }
        }

        matches
    }

    fn matches(self, left: &[VarType], right: &[VarType]) -> bool {
        if left.len() == right.len() {
            let lst_parameters = self.get_lst_parameters();
            for i in 0..left.len() {
                let left_family = left[i].type_family;
                let right_family = right[i].type_family;
                if left_family != right_family
                    && !(left_family.is_numeric() && right_family.is_numeric())
                {
                    return false;
                }

                if (i >= lst_parameters.len()
                    || lst_parameters.get(i).get_expr_type().type_ != CodeType::Null)
                    && left[i].array_dim != right[i].array_dim
                {
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn matches_varargs(
        self,
        left: &[VarType],
        right: &[VarType],
        left_method: StructMethod,
        right_method: Option<StructMethod>,
    ) -> bool {
        if left.len() == right.len() {
            let lst_parameters = self.get_lst_parameters();
            for i in 0..left.len() {
                let left_family = left[i].type_family;
                let right_family = right[i].type_family;
                if left_family != right_family
                    && !(left_family.is_numeric() && right_family.is_numeric())
                {
                    return false;
                }

                let mut left_type = left[i].clone();
                if left_method.has_modifier(CodeConstants::ACC_VARARGS) && i == left.len() - 1 {
                    left_type = left_type.decrease_array_dim();
                }
                let mut right_type = right[i].clone();
                if right_method.is_some_and(|m| m.has_modifier(CodeConstants::ACC_VARARGS))
                    && i == right.len() - 1
                {
                    right_type = right_type.decrease_array_dim();
                }

                if (i >= lst_parameters.len()
                    || lst_parameters.get(i).get_expr_type().type_ != CodeType::Null)
                    && left_type.array_dim != right_type.array_dim
                {
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// `curr_cls` is Java's possibly-null `StructClass`; it is dereferenced only
    /// for non-public members, as in Java.
    fn can_access(self, curr_cls: Option<StructClass>, mt: StructMethod) -> bool {
        if mt.has_modifier(CodeConstants::ACC_PUBLIC) {
            true
        } else if mt.has_modifier(CodeConstants::ACC_PRIVATE) {
            mt.get_class_qualified_name()
                == curr_cls.expect("NullPointerException").qualified_name()
        } else if mt.has_modifier(CodeConstants::ACC_PROTECTED) {
            let curr_name = curr_cls.expect("NullPointerException").qualified_name();
            let mt_class = mt.get_class_qualified_name();
            let same_package = self.is_in_same_package(&curr_name, &mt_class);
            same_package
                || DecompilerContext::get_struct_context().instance_of(&curr_name, &mt_class)
        } else {
            let curr_name = curr_cls.expect("NullPointerException").qualified_name();
            self.is_in_same_package(&curr_name, &mt.get_class_qualified_name())
        }
    }

    fn is_in_same_package(self, class1: &str, class2: &str) -> bool {
        let pos1 = class1.rfind('/');
        let pos2 = class2.rfind('/');
        if pos1 != pos2 {
            return false;
        }

        match (pos1, pos2) {
            (Some(p1), Some(p2)) => class1[..p1] == class2[..p2],
            _ => true,
        }
    }

    fn is_varargs_ambiguous(self, matches: &[StructMethod]) -> bool {
        let mut varargs: JHashSet<StructMethod> = JHashSet::new();
        for mt in matches {
            if mt.has_modifier(CodeConstants::ACC_VARARGS) {
                varargs.add(*mt);
            }
        }

        let varargs = varargs.to_vec();
        for &m in matches {
            for &vararg in &varargs {
                if m == vararg {
                    continue;
                }

                let md1 = m.method_descriptor();
                let md2 = vararg.method_descriptor();
                if md1.params.len() != md2.params.len() {
                    continue;
                }

                if md1.params[md1.params.len() - 1]
                    .equals(&md2.params[md2.params.len() - 1].decrease_array_dim())
                {
                    return true;
                }
            }
        }
        false
    }

    fn get_ambiguous_parameters(self, matches: &[StructMethod]) -> BitSet {
        let (classname, name, string_descriptor) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.string_descriptor.clone(),
            )
        });
        let cl = DecompilerContext::get_struct_context().get_class(&classname);
        let Some(cl) = cl.filter(|_| matches.len() != 1) else {
            return Self::empty_bit_set();
        };

        let lst_parameters = self.get_lst_parameters();
        let mut missed = BitSet::new();

        if CodeConstants::are_parameters_polymorphic(&classname, &name) {
            missed.set_range(0, lst_parameters.len());
            return missed;
        }

        let current_method =
            cl.get_method_key(&InterpreterUtil::make_unique_key(&name, &string_descriptor));

        let mut possible: JHashSet<StructMethod> = JHashSet::new();
        let mut exacts: JHashSet<StructMethod> = JHashSet::new();

        let mut ambiguous = BitSet::new();
        for &mt in matches {
            let md = mt.method_descriptor();

            let mut exact = true;
            for i in 0..md.params.len() {
                let type_ = Self::peek_non_casting(lst_parameters.get(i)).get_expr_type();

                exact &= md.params[i].equals(&type_) || type_.type_ == CodeType::Null;

                if md.params[i].higher_cross_family_than(&type_, true) {
                    possible.add(mt);
                }
            }

            if exact {
                exacts.add(mt);
            }
        }

        if possible.len() == 1 {
            return Self::empty_bit_set();
        }

        if exacts.is_empty() {
            return Self::empty_bit_set();
        } else if exacts.len() == 1 {
            let exact = exacts.to_vec()[0];

            if Some(exact) == current_method {
                return Self::empty_bit_set();
            }
        }

        let md = match current_method {
            None => MethodDescriptor::parse_descriptor(&string_descriptor),
            Some(current_method) => current_method.method_descriptor(),
        };
        for p in possible.to_vec() {
            for i in 0..md.params.len() {
                let type_ = Self::peek_non_casting(lst_parameters.get(i)).get_expr_type();

                let pmd = p.method_descriptor();
                if !md.params[i].equals(&pmd.params[i]) {
                    if md.params[i].higher_cross_family_than(&pmd.params[i], false)
                        || type_.equals(&pmd.params[i])
                        || type_.type_ == CodeType::Null
                    {
                        ambiguous.set(i);
                    }
                }
            }
        }

        ambiguous
    }

    /// `exp instanceof FunctionExprent f && !f.doesCast() ? f.getLstOperands().get(0) : exp`.
    fn peek_non_casting(exp: Exprent) -> Exprent {
        match exp.try_as_function() {
            Some(f) if !f.does_cast() => f.get_lst_operands().get(0),
            _ => exp,
        }
    }

    fn process_generic_mapping(
        self,
        from: &VarType,
        to: Option<&VarType>,
        named: &JHashMap<VarType, Vec<VarType>>,
        bounds: &JHashMap<VarType, Vec<VarType>>,
    ) -> bool {
        if VarType::VARTYPE_NULL().equals_opt(to)
            || to.is_some_and(|to| to.type_ == CodeType::Genvar && !named.contains_key(to))
        {
            return false;
        }

        let current = self.gm_get(from);
        if !self.gm_contains(from) {
            self.put_generic_mapping(from, to, named, bounds);

            return true;
        } else if let (Some(to), Some(current)) = (to, &current) {
            if !to.equals(current) {
                if named.contains_key(current) {
                    return false;
                }

                if current.type_ != CodeType::Genvar
                    && to.type_ == CodeType::Genvar
                    && named.contains_key(to)
                {
                    let bound = &named.get(to).unwrap()[0];
                    if !bound.equals(&VarType::VARTYPE_OBJECT())
                        && instance_of(&bound.value, &current.value)
                    {
                        return false;
                    }
                }

                if to.is_generic()
                    && current.is_generic()
                    && GenericType::is_assignable(to, current, named)
                {
                    self.put_generic_mapping(from, Some(to), named, bounds);

                    return true;
                }
            }
        }

        false
    }

    fn put_generic_mapping(
        self,
        from: &VarType,
        to: Option<&VarType>,
        named: &JHashMap<VarType, Vec<VarType>>,
        bounds: &JHashMap<VarType, Vec<VarType>>,
    ) {
        if self.is_mapping_in_bounds(from, to, named, bounds, &mut JHashSet::new()) {
            self.gm_put(from.clone(), to.cloned());
        }
    }

    fn is_mapping_in_bounds(
        self,
        from: &VarType,
        to: Option<&VarType>,
        named: &JHashMap<VarType, Vec<VarType>>,
        bounds: &JHashMap<VarType, Vec<VarType>>,
        recursively_seen: &mut JHashSet<Pair<VarType, VarType>>,
    ) -> bool {
        if !bounds.contains_key(from) {
            return false;
        }

        let Some(to) = to.filter(|to| !(to.type_ == CodeType::Genvar && !named.contains_key(to)))
        else {
            return true;
        };

        let to_verify: Vec<VarType> = if to.type_ == CodeType::Genvar {
            named.get(to).expect("NullPointerException").clone()
        } else {
            vec![to.clone()]
        };

        let from_bounds = bounds.get(from).unwrap().clone();
        for bound in &from_bounds {
            let mut any = false;
            for v in &to_verify {
                if self.mapping_verifier(
                    from,
                    to,
                    named,
                    bounds,
                    recursively_seen,
                    v,
                    bound.clone(),
                ) {
                    any = true;
                    break;
                }
            }
            if !any {
                return false;
            }
        }
        true
    }

    /// The `verifier` lambda of `isMappingInBounds(from, to, ...)`, applied to `(new_to, bound)`.
    #[allow(clippy::too_many_arguments)]
    fn mapping_verifier(
        self,
        from: &VarType,
        to: &VarType,
        named: &JHashMap<VarType, Vec<VarType>>,
        bounds: &JHashMap<VarType, Vec<VarType>>,
        recursively_seen: &mut JHashSet<Pair<VarType, VarType>>,
        new_to: &VarType,
        mut bound: VarType,
    ) -> bool {
        if bound.type_ == CodeType::Genvar {
            let map = |e: &VarType| -> Option<VarType> {
                let mut mapped = self.gm_get(e);
                if mapped.is_none() {
                    mapped = named.get(e).map(|list| list[0].clone());
                }
                mapped
            };
            let mapped = map(&bound);

            let mut seen: JHashMap<VarType, Option<VarType>> = JHashMap::new();
            if mapped.as_ref().is_some_and(|m| !m.equals(&bound)) {
                let mut last = bound.clone();
                let mut current = Some(bound.clone());
                while let Some(b) = current {
                    last = b.clone();
                    current = map(&b);

                    if last.equals_opt(current.as_ref()) {
                        break;
                    }
                    if seen.contains_key(&last)
                        && seen
                            .get(&last)
                            .unwrap()
                            .as_ref()
                            .expect("NullPointerException")
                            .equals_opt(current.as_ref())
                    {
                        break;
                    } else {
                        seen.put(last.clone(), current.clone());
                    }
                }
                bound = last;

                if bound.type_ != CodeType::Genvar {
                    return instance_of(&new_to.value, &bound.value);
                }
            }

            let pair = Pair::of(bound.clone(), new_to.clone());
            if !recursively_seen.contains(&pair) {
                recursively_seen.add(pair);
                return self.is_mapping_in_bounds(
                    &bound,
                    Some(new_to),
                    named,
                    bounds,
                    recursively_seen,
                );
            }
        }

        if new_to.type_.ordinal() < CodeType::Object.ordinal() {
            return bound.equals(&VarType::VARTYPE_OBJECT()) || bound.equals(new_to);
        }

        if new_to.type_ != CodeType::Genvar && !instance_of(&new_to.value, &bound.value) {
            return false;
        }

        if bound.is_generic() && !bound.as_generic().get_arguments().is_empty() {
            let genbound = bound.as_generic();
            let mut new = new_to.clone();

            if !str_equals(&new_to.value, &bound.value) {
                new = GenericType::get_generic_super_type(new_to, &bound)
                    .expect("NullPointerException");
            }

            if !new.is_generic()
                || new.as_generic().get_arguments().len() != genbound.get_arguments().len()
            {
                return false;
            }

            let mut to_add: JHashMap<VarType, Option<VarType>> = JHashMap::new();
            let gen_new = new.as_generic();
            for i in 0..genbound.get_arguments().len() {
                let Some(bound_arg) = genbound.get_arguments()[i].clone() else {
                    continue;
                };
                let new_arg = gen_new.get_arguments()[i].clone();

                if !bound_arg.equals_opt(new_arg.as_ref()) {
                    if from.equals(&bound_arg) && to.equals_opt(new_arg.as_ref()) {
                        continue;
                    }

                    if bounds.contains_key(&bound_arg) {
                        let pair = Pair::of(bound.clone(), new_to.clone());
                        if !recursively_seen.contains(&pair) {
                            recursively_seen.add(pair);
                            if self.is_mapping_in_bounds(
                                &bound_arg,
                                new_arg.as_ref(),
                                named,
                                bounds,
                                recursively_seen,
                            ) {
                                to_add.put(bound_arg, new_arg);
                                continue;
                            }
                        }
                    }
                    return false;
                }
            }
            for (k, v) in to_add.entry_vec() {
                self.process_generic_mapping(&k, v.as_ref(), named, bounds);
            }
        }
        true
    }

    fn get_generic_bounds(self, mth_cls: StructClass) -> JHashMap<VarType, Vec<VarType>> {
        let mut bounds: JHashMap<VarType, Vec<VarType>> = JHashMap::new();

        let desc = self.d(|d| d.desc).expect("NullPointerException");
        if let Some(sig) = desc.get_signature() {
            let type_parameters = sig.type_parameters().clone();
            let type_parameter_bounds = sig.type_parameter_bounds().clone();
            for x in 0..type_parameters.len() {
                bounds.put_if_absent(
                    GenericType::parse(&format!("T{};", type_parameters[x])),
                    type_parameter_bounds[x].clone(),
                );
            }
        }

        if let Some(sig) = mth_cls.get_signature() {
            Self::put_class_bounds(&mut bounds, &sig);
        }

        let mut cn =
            root_class_node(&mth_cls.qualified_name()).and_then(|cn| cn.with(|d| d.parent));

        while let Some(node) = cn {
            if let Some(sig) = node.with(|d| d.class_struct).get_signature() {
                Self::put_class_bounds(&mut bounds, &sig);
            }
            cn = node.with(|d| d.parent);
        }

        bounds
    }

    fn put_class_bounds(
        bounds: &mut JHashMap<VarType, Vec<VarType>>,
        sig: &crate::generic_class_descriptor::GenericClassDescriptor,
    ) {
        let fparameters = sig.fparameters().clone();
        let fbounds = sig.fbounds().clone();
        for x in 0..fparameters.len() {
            bounds.put_if_absent(
                GenericType::parse(&format!("T{};", fparameters[x])),
                fbounds[x].clone(),
            );
        }
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        let lst_parameters = self.d_mut(|d| {
            if Some(old_expr) == d.instance {
                d.instance = Some(new_expr);
            }
            d.lst_parameters.clone()
        });

        for i in 0..lst_parameters.len() {
            if old_expr == lst_parameters.get(i) {
                lst_parameters.set(i, new_expr);
            }
        }
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(it) = o.try_as_invocation() else {
            return false;
        };
        let (name, classname, is_static, instance, descriptor, functype, lst_parameters) =
            self.d(|d| {
                (
                    d.name.clone(),
                    d.classname.clone(),
                    d.is_static,
                    d.instance,
                    d.descriptor.clone(),
                    d.functype,
                    d.lst_parameters.to_vec(),
                )
            });
        let (o_name, o_classname, o_is_static, o_descriptor, o_functype) = it.d(|d| {
            (
                d.name.clone(),
                d.classname.clone(),
                d.is_static,
                d.descriptor.clone(),
                d.functype,
            )
        });
        name == o_name
            && classname == o_classname
            && is_static == o_is_static
            && instance.java_equals(&it.get_instance())
            && match (&descriptor, &o_descriptor) {
                (None, None) => true,
                (Some(a), Some(b)) => a.equals(b),
                _ => false,
            }
            && functype == o_functype
            && lst_parameters.java_equals(&it.get_lst_parameters().to_vec())
    }

    /// The parameter list itself (aliased, like the Java reference).
    pub fn get_lst_parameters(self) -> JList<Exprent> {
        self.d(|d| d.lst_parameters.clone())
    }

    pub fn set_lst_parameters(self, lst_parameters: impl Into<JList<Exprent>>) {
        let lst_parameters = lst_parameters.into();
        self.d_mut(|d| d.lst_parameters = lst_parameters)
    }

    /// Throws `NullPointerException` before `set_descriptor` on a `new()` expression.
    pub fn get_descriptor(self) -> MethodDescriptor {
        match self.d(|d| d.descriptor.clone()) {
            Some(descriptor) => descriptor,
            None => jthrow!(NullPointerException; "InvocationExprent.descriptor is null"),
        }
    }

    pub fn set_descriptor(self, descriptor: MethodDescriptor) {
        self.d_mut(|d| d.descriptor = Some(descriptor))
    }

    pub fn get_classname(self) -> String {
        self.d(|d| d.classname.clone())
    }

    pub fn set_classname(self, classname: String) {
        self.d_mut(|d| d.classname = classname)
    }

    pub fn get_functype(self) -> Type {
        self.d(|d| d.functype)
    }

    pub fn set_functype(self, functype: Type) {
        self.d_mut(|d| d.functype = functype)
    }

    pub fn get_instance(self) -> Option<Exprent> {
        self.d(|d| d.instance)
    }

    pub fn set_instance(self, instance: Option<Exprent>) {
        self.d_mut(|d| d.instance = instance)
    }

    pub fn is_static(self) -> bool {
        self.d(|d| d.is_static)
    }

    pub fn set_static(self, is_static: bool) {
        self.d_mut(|d| d.is_static = is_static)
    }

    pub fn get_name(self) -> String {
        self.d(|d| d.name.clone())
    }

    pub fn set_name(self, name: String) {
        self.d_mut(|d| d.name = name)
    }

    pub fn get_string_descriptor(self) -> String {
        self.d(|d| d.string_descriptor.clone())
    }

    pub fn set_string_descriptor(self, string_descriptor: String) {
        self.d_mut(|d| d.string_descriptor = string_descriptor)
    }

    pub fn get_invocation_type(self) -> InvocationType {
        self.d(|d| d.invocation_type)
    }

    pub fn get_invoke_dynamic_class_suffix(self) -> Option<String> {
        self.d(|d| d.invoke_dynamic_class_suffix.clone())
    }

    pub fn get_bootstrap_method(self) -> Option<LinkConstant> {
        self.d(|d| d.bootstrap_method.clone())
    }

    pub fn get_bootstrap_arguments(self) -> Option<Vec<Option<PooledConstant>>> {
        self.d(|d| d.bootstrap_arguments.clone())
    }

    pub fn set_synthetic_null_check(self) {
        self.d_mut(|d| d.is_synthetic_null_check = true)
    }

    pub fn is_synthetic_null_check(self) -> bool {
        self.d(|d| d.is_synthetic_null_check)
    }

    pub fn get_generic_args(self) -> Vec<VarType> {
        self.d(|d| d.generic_args.clone())
    }

    pub fn get_generics_map(self) -> JHashMap<VarType, Option<VarType>> {
        self.d(|d| d.generics_map.clone())
    }

    pub fn get_desc(self) -> Option<StructMethod> {
        if self.d(|d| d.desc.is_none()) {
            let (classname, name, string_descriptor) = self.d(|d| {
                (
                    d.classname.clone(),
                    d.name.clone(),
                    d.string_descriptor.clone(),
                )
            });
            let cl = DecompilerContext::get_struct_context().get_class(&classname);
            let desc = cl.and_then(|cl| cl.get_method_recursive(&name, &string_descriptor));
            self.d_mut(|d| d.desc = desc);
        }
        self.d(|d| d.desc)
    }

    pub fn set_invocation_instance(self) {
        self.d_mut(|d| d.is_invocation_instance = true)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let (lst_parameters, instance) = self.d(|d| (d.lst_parameters.to_vec(), d.instance));
        Exprent::measure_bytecode_list(values.as_deref_mut(), &lst_parameters);
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), instance);
        self.measure_bytecode(values);
    }

    pub fn mark_was_lazy_condy(self) -> InvocationExprent {
        self.d_mut(|d| d.was_lazy_condy = true);
        self
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        Exprent::process_sforms_base(self.0, s_forms_constructor, var_maps, stat, calc_live_vars);

        if s_forms_constructor.track_field_vars {
            var_maps.get_normal().borrow_mut().remove_all_fields();
        }
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if !Exprent::match_base(self.0, match_node, engine) {
            return false;
        }

        let (classname, name, string_descriptor, lst_parameters) = self.d(|d| {
            (
                d.classname.clone(),
                d.name.clone(),
                d.string_descriptor.clone(),
                d.lst_parameters.clone(),
            )
        });
        let rule_value = |value: &Option<MatchValue>| -> MatchValue {
            value.clone().expect("NullPointerException")
        };
        match_node.iterate_rules(|key, value| match key {
            MatchProperties::ExprentParameter => {
                !value.is_variable()
                    || ((value.parameter as i64) < lst_parameters.len() as i64
                        && engine.check_and_set_variable_value(
                            &rule_value(&value.value).to_java_string(),
                            Some(MatchValue::Exprent(
                                lst_parameters.get(value.parameter as usize),
                            )),
                        ))
            }
            MatchProperties::ExprentInvocationClass => {
                rule_value(&value.value).equals(&MatchValue::String(classname.clone()))
            }
            MatchProperties::ExprentInvocationSignature => rule_value(&value.value)
                .equals(&MatchValue::String(format!("{name}{string_descriptor}"))),
            MatchProperties::ExprentName => {
                rule_value(&value.value).equals(&MatchValue::String(name.clone()))
            }
            _ => true,
        })
    }
}
