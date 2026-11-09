//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/InvocationExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::expr_processor::ExprProcessor;
use crate::exprent::{Exprent, ExprentKind};
use crate::link_constant::LinkConstant;
use crate::list_stack::ListStack;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::method_descriptor::MethodDescriptor;
use crate::pair::Pair;
use crate::pooled_constant::PooledConstant;
use crate::rt::{BitSet, JHashMap, JHashSet, JList, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::type_family::TypeFamily;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

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

impl InvocationExprent {
    #[allow(dead_code)]
    fn empty_bit_set() -> BitSet {
        BitSet::new()
    }

    #[allow(dead_code, non_snake_case)]
    fn JAVA_NIO_BUFFER() -> VarType {
        unported!("InvocationExprent.JAVA_NIO_BUFFER")
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
        _opcode: i32,
        _cn: &LinkConstant,
        _bootstrap_method: Option<LinkConstant>,
        _bootstrap_arguments: Option<Vec<Option<PooledConstant>>>,
        _stack: &mut ListStack<Exprent>,
        _bytecode_offsets: Option<&BitSet>,
    ) -> InvocationExprent {
        unported!("InvocationExprent.<init>(int, LinkConstant, ...)")
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

    pub fn get_inferred_expr_type(self, _upper_bound: Option<&VarType>) -> VarType {
        unported!("InvocationExprent.getInferredExprType")
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("InvocationExprent.checkExprTypeBounds")
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

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("InvocationExprent.toJava")
    }

    #[allow(dead_code)]
    fn append_inst_cast(self, _buf: &mut TextBuffer, _left_type: &VarType, _res: TextBuffer) {
        unported!("InvocationExprent.appendInstCast")
    }

    #[allow(dead_code)]
    fn can_skip_paren_enclose(self, instance: Exprent) -> bool {
        let Some(new_expr) = instance.try_as_new() else {
            return false;
        };

        if !new_expr.is_anonymous() && !new_expr.is_lambda() && !new_expr.is_method_reference() {
            return self.get_functype() == Type::General;
        }

        false
    }

    #[allow(dead_code)]
    fn append_bootstrap_argument(_buf: &mut TextBuffer, _arg: &PooledConstant) {
        unported!("InvocationExprent.appendBootstrapArgument")
    }

    pub fn append_param_list(self, _indent: i32) -> TextBuffer {
        unported!("InvocationExprent.appendParamList")
    }

    #[allow(dead_code)]
    fn is_var_arg_call(self) -> bool {
        unported!("InvocationExprent.isVarArgCall")
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
    #[allow(dead_code)]
    fn get_matched_descriptors(self, varargs: bool) -> Vec<StructMethod> {
        self.get_matched_descriptors_custom(varargs, None)
    }

    /// Java: `getMatchedDescriptors(boolean, BiFunction<StructMethod, MethodDescriptor, Boolean>)`.
    #[allow(dead_code)]
    fn get_matched_descriptors_custom(
        self,
        _varargs: bool,
        _custom_param_matcher: Option<ParamMatcher<'_>>,
    ) -> Vec<StructMethod> {
        unported!("InvocationExprent.getMatchedDescriptors")
    }

    #[allow(dead_code)]
    fn matches(self, _left: &[VarType], _right: &[VarType]) -> bool {
        unported!("InvocationExprent.matches")
    }

    #[allow(dead_code)]
    fn matches_varargs(
        self,
        _left: &[VarType],
        _right: &[VarType],
        _left_method: StructMethod,
        _right_method: Option<StructMethod>,
    ) -> bool {
        unported!("InvocationExprent.matchesVarargs")
    }

    #[allow(dead_code)]
    fn can_access(self, _curr_cls: StructClass, _mt: StructMethod) -> bool {
        unported!("InvocationExprent.canAccess")
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn is_varargs_ambiguous(self, _matches: &[StructMethod]) -> bool {
        unported!("InvocationExprent.isVarargsAmbiguous")
    }

    #[allow(dead_code)]
    fn get_ambiguous_parameters(self, _matches: &[StructMethod]) -> BitSet {
        unported!("InvocationExprent.getAmbiguousParameters")
    }

    #[allow(dead_code)]
    fn process_generic_mapping(
        self,
        _from: &VarType,
        _to: Option<&VarType>,
        _named: &JHashMap<VarType, Vec<VarType>>,
        _bounds: &JHashMap<VarType, Vec<VarType>>,
    ) -> bool {
        unported!("InvocationExprent.processGenericMapping")
    }

    #[allow(dead_code)]
    fn put_generic_mapping(
        self,
        _from: &VarType,
        _to: Option<&VarType>,
        _named: &JHashMap<VarType, Vec<VarType>>,
        _bounds: &JHashMap<VarType, Vec<VarType>>,
    ) {
        unported!("InvocationExprent.putGenericMapping")
    }

    #[allow(dead_code)]
    fn is_mapping_in_bounds(
        self,
        _from: &VarType,
        _to: Option<&VarType>,
        _named: &JHashMap<VarType, Vec<VarType>>,
        _bounds: &JHashMap<VarType, Vec<VarType>>,
        _recursively_seen: &mut JHashSet<Pair<VarType, VarType>>,
    ) -> bool {
        unported!("InvocationExprent.isMappingInBounds")
    }

    #[allow(dead_code)]
    fn get_generic_bounds(self, _mth_cls: StructClass) -> JHashMap<VarType, Vec<VarType>> {
        unported!("InvocationExprent.getGenericBounds")
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
            None => crate::jthrow!(NullPointerException; "InvocationExprent.descriptor is null"),
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
        unported!("InvocationExprent.getDesc")
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
        _s_forms_constructor: &mut SFormsConstructor,
        _var_maps: &mut VarMapHolder,
        _stat: Statement,
        _calc_live_vars: bool,
    ) {
        unported!("InvocationExprent.processSforms")
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, _match_node: &MatchNode, _engine: &MatchEngine) -> bool {
        unported!("InvocationExprent.match")
    }
}
