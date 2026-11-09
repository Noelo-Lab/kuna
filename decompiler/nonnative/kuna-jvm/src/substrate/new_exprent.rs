//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/NewExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_type::CodeType;
use crate::dec_helper::DecHelper;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentKind};
use crate::invocation_exprent::InvocationExprent;
use crate::list_stack::ListStack;
use crate::rt::{BitSet, JHashMap, JList, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

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

/// `DecompilerContext.getClassProcessor().getMapRootClasses().get(name)`.
fn root_class_node(name: &str) -> Option<ClassNode> {
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
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
            let node = root_class_node(
                new_type
                    .value
                    .as_deref()
                    .expect("NullPointerException: newType.value"),
            );
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
            let node = root_class_node(
                new_type
                    .value
                    .as_deref()
                    .expect("NullPointerException: newType.value"),
            )
            .expect("NullPointerException: anonymous class node");
            node.with(|n| n.anonymous_class_type.clone())
                .expect("NullPointerException: anonymousClassType")
        } else {
            new_type
        }
    }

    pub fn get_inferred_expr_type(self, _upper_bound: Option<&VarType>) -> VarType {
        unported!("NewExprent.getInferredExprType")
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("NewExprent.checkExprTypeBounds")
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
        ) = self.d(|d| {
            (
                d.new_type.clone(),
                d.lst_dims.clone(),
                d.constructor,
                d.lst_array_elements.clone(),
                d.direct_array_init,
                d.anonymous,
                d.enum_const,
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
        ret.0
    }

    pub fn get_precedence(self) -> i32 {
        1
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("NewExprent.toJava")
    }

    pub fn probably_synthetic_parameter(class_name: &str) -> bool {
        root_class_node(class_name)
            .is_some_and(|node| node.with(|n| n.type_) == ClassNodeType::Anonymous)
    }

    pub fn get_qualified_new_instance(
        _classname: &str,
        _lst_params: &[Exprent],
        _indent: i32,
    ) -> Option<TextBuffer> {
        unported!("NewExprent.getQualifiedNewInstance")
    }

    #[allow(dead_code)]
    fn get_lambda_return_type(
        _node: ClassNode,
        _desc: StructMethod,
        _upper_bound: Option<&VarType>,
        _generics_map: &JHashMap<VarType, Option<VarType>>,
    ) -> Option<VarType> {
        unported!("NewExprent.getLambdaReturnType")
    }

    #[allow(dead_code)]
    fn set_lambda_generic_types(self) {
        unported!("NewExprent.setLambdaGenericTypes")
    }

    pub fn does_class_have_methods_named_same(self) -> bool {
        unported!("NewExprent.doesClassHaveMethodsNamedSame")
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

    pub fn get_lambda_method_key(self) -> String {
        unported!("NewExprent.getLambdaMethodKey")
    }

    pub fn get_lambda_content_method_key(self) -> String {
        unported!("NewExprent.getLambdaContentMethodKey")
    }

    pub fn set_invocation_instance(self) {
        if let Some(constructor) = self.get_constructor() {
            constructor.set_invocation_instance();
        }
    }

    pub fn process_sforms(
        self,
        _s_forms_constructor: &mut SFormsConstructor,
        _var_maps: &mut VarMapHolder,
        _stat: Statement,
        _calc_live_vars: bool,
    ) {
        unported!("NewExprent.processSforms")
    }
}
