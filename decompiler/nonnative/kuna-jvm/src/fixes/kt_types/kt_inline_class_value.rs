//! Fix `kt_inline_class_value`: each expression is typed as a value class `V`
//! held as its underlying type, the underlying value or unknown; then
//! `constructor-impl`, `box-impl`, `unbox-impl` and `f-impl` calls are written as
//! `V(x)`, `v` and `v.f(..)`, locals holding a `V` are declared `V`, and a `V`
//! read as its value is `v.x`, each only where every use is known.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::Type as LoopType;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_function_exprent::KFunctionType;
use crate::k_invocation_exprent::KInvocationExprent;
use crate::k_property::PropertyMethod;
use crate::k_type::{KType, TypeArgument};
use crate::kt_proto::{Projection, Variance};
use crate::k_types::KTypes;
use crate::fixes::kt_names::kt_unescaped_name;
use crate::fixes::kt_util::args_after_receiver;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::statement::{Statement, StatementType};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_processor::{KunaLocalType, VarProcessor};
use crate::var_type::VarType;

const RESULT: &str = "kotlin/Result";
const CREATE_FAILURE: (&str, &str, &str) =
    ("kotlin/ResultKt", "createFailure", "(Ljava/lang/Throwable;)Ljava/lang/Object;");
const MARKER: &str = "kotlin/jvm/internal/DefaultConstructorMarker";

/// How a call the fix rewrites is written.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Form {
    /// `V(arg0)`.
    Constructor,
    /// `Result.success(arg0)`.
    Success,
    /// `Result.failure(e)` for `arg0` = `createFailure(e)`.
    Failure,
    /// `box-impl(v)` and `v.unbox-impl()`: the value alone.
    Value,
    /// `arg0.name(args[1..])`, or `arg0.name` for a property.
    Member { name: Rc<str>, property: bool },
    /// An instance call written by its Kotlin name.
    Renamed(Rc<str>),
    /// An instance call of a property's getter: `instance.name`.
    Property(Rc<str>),
    /// `arg0 == arg1`.
    Equals,
    /// `instance.name`, `instance?.name`: the underlying value.
    Underlying { name: Rc<str>, nullable: bool },
}

/// An inline class as its class file shows it.
struct ValueClass {
    name: String,
    /// The metadata's name (`pkg/Outer.Inner`).
    kotlin_name: Option<String>,
    property: String,
    underlying: VarType,
    property_type: Option<KType>,
    /// Per type parameter: covariant and unbounded, so `Any?` takes any value.
    params: Vec<bool>,
}

/// A value of an inline class held as its underlying type.
#[derive(Clone)]
struct Held {
    class: Rc<ValueClass>,
    nullable: bool,
}

/// What a use takes a value as: a value class (`V?` where `nullable`).
#[derive(Clone)]
enum Use {
    Value(Rc<ValueClass>, bool),
    Raw,
    Unknown,
    Discard,
    /// Compared with `null`: a `V?` held as `T?` is compared as it is.
    NullCheck,
    /// The value stored into the local `index`.
    Local(i32),
    /// The value stored into a local not yet decided.
    Pending,
}

/// What an expression holds, while locals are typed: `Any` is a local not
/// yet decided.
#[derive(Clone)]
enum Ty {
    No,
    Any,
    Is(Held),
}

/// Where an expression sits, for replacing it.
#[derive(Clone, Copy)]
enum Parent {
    Exprent(Exprent),
    Statement(Statement),
}

struct Walk {
    classes: BTreeMap<String, Option<Rc<ValueClass>>>,
    uses: BTreeMap<Exprent, Use>,
    parents: BTreeMap<Exprent, Parent>,
    order: Vec<Exprent>,
    params: BTreeMap<i32, Held>,
    ret: Option<Rc<ValueClass>>,
    defs: BTreeMap<i32, Vec<Exprent>>,
    excluded: BTreeSet<i32>,
    locals: BTreeMap<i32, Option<Rc<ValueClass>>>,
    forms: BTreeMap<Exprent, Form>,
    branches: BTreeSet<Exprent>,
    ret_type: Option<KType>,
    returned: BTreeSet<i32>,
    tys: BTreeMap<Exprent, Ty>,
    call_uses: BTreeMap<Exprent, Vec<Use>>,
    callees: BTreeMap<Exprent, Option<(KElement, Option<PropertyMethod>)>>,
    /// The code is `Result`'s own, which calls its constructor.
    in_result: bool,
    targets: Vec<Exprent>,
}

/// Rewrites the inline-class values of `root`, the body of `mt` in `cl`.
pub fn resugar(root: RootStatement, cl: StructClass, mt: StructMethod, processor: VarProcessor) {
    let mut walk = Walk {
        classes: BTreeMap::new(),
        uses: BTreeMap::new(),
        parents: BTreeMap::new(),
        order: Vec::new(),
        params: BTreeMap::new(),
        ret: None,
        defs: BTreeMap::new(),
        excluded: BTreeSet::new(),
        locals: BTreeMap::new(),
        forms: BTreeMap::new(),
        branches: BTreeSet::new(),
        ret_type: None,
        returned: BTreeSet::new(),
        tys: BTreeMap::new(),
        call_uses: BTreeMap::new(),
        callees: BTreeMap::new(),
        targets: Vec::new(),
        in_result: cl.qualified_name().strip_suffix("$Companion").unwrap_or(&cl.qualified_name()) == RESULT,
    };
    walk.signature(cl, mt);
    walk.statement(root.0);
    walk.type_locals();
    walk.apply(processor);
}

impl Walk {
    fn value_class(&mut self, name: &str) -> Option<Rc<ValueClass>> {
        if let Some(known) = self.classes.get(name) {
            return known.clone();
        }
        let class = load_value_class(name).map(Rc::new);
        self.classes.insert(name.to_string(), class.clone());
        class
    }

    /// The inline class `t` names, when `jvm` is its underlying type.
    fn held(&mut self, t: &KType, jvm: &VarType) -> Option<Held> {
        let k = t.k();
        if k.type_parameter_name.is_some() {
            return None;
        }
        let name = if t.type_ == CodeType::Object && t.array_dim == 0 {
            t.value_str().to_string()
        } else {
            k.kotlin_type.clone()
        };
        let class = self.value_class(&name)?;
        same_type(&class.underlying, jvm).then_some(Held {
            class,
            nullable: k.is_nullable,
        })
    }

    fn use_of(&mut self, t: &KType, jvm: &VarType) -> Use {
        match self.held(t, jvm) {
            Some(held) => Use::Value(held.class, held.nullable),
            _ => Use::Raw,
        }
    }

    /// The method's parameters and result that hold a value class.
    fn signature(&mut self, cl: StructClass, mt: StructMethod) {
        if !parsed(cl) {
            return;
        }
        let md = mt.method_descriptor();
        let is_static = mt.has_modifier(ACC_STATIC);
        let own = self.value_class(&cl.qualified_name());
        let mut types: Vec<Option<KType>> = Vec::new();
        let mut ret: Option<KType> = None;
        let mut suspend = false;
        let mut from_end = false;
        match KtAttrs::get(mt, &KElement::KEY()) {
            Some(KElement::Function(f)) => {
                if is_static && own.is_some() {
                    types.push(None);
                }
                types.extend(f.context_receiver_types.iter().cloned().map(Some));
                types.extend(f.receiver_type.clone().map(Some));
                types.extend(f.parameters.iter().map(|p| Some(p.type_.clone())));
                suspend = Flags::IS_SUSPEND.get(f.flags);
                ret = Some(f.return_type.clone());
            }
            Some(KElement::Property(p)) => {
                if is_static && own.is_some() {
                    types.push(None);
                }
                if let Some(extension) = &p.extension {
                    types.push(Some(extension.receiver.clone()));
                }
                match KtAttrs::get(mt, &PropertyMethod::KEY()) {
                    Some(PropertyMethod::Getter) => ret = p.type_.clone(),
                    Some(PropertyMethod::Setter) => types.push(p.type_.clone()),
                    None => return,
                }
            }
            Some(KElement::Constructor(c)) => {
                types.extend(c.parameters.iter().map(|p| Some(p.type_.clone())));
                from_end = true;
            }
            _ => return,
        }
        let mut params: Vec<VarType> = md.params.to_vec();
        if from_end
            && params
                .last()
                .is_some_and(|p| p.type_ == CodeType::Object && p.value_str() == MARKER)
        {
            params.pop();
        }
        if suspend {
            params.pop();
        }
        let lead = if from_end {
            if params.len() < types.len() {
                return;
            }
            params.len() - types.len()
        } else {
            if params.len() != types.len() {
                return;
            }
            0
        };
        let mut slot = if is_static { 0 } else { 1 };
        for (i, param) in md.params.iter().enumerate() {
            if i >= lead {
                match types.get(i - lead) {
                    Some(Some(t)) => {
                        if let Some(held) = self.held(t, param) {
                            self.params.insert(slot, held);
                        }
                    }
                    Some(None) => {
                        if let Some(class) = own.clone() {
                            if same_type(&class.underlying, param) {
                                self.params.insert(slot, Held { class, nullable: false });
                            }
                        }
                    }
                    None => {}
                }
            }
            slot += param.stack_size;
        }
        if let Some(t) = ret {
            if let Some(held) = self.held(&t, &md.ret) {
                if !held.nullable {
                    self.ret = Some(held.class);
                    self.ret_type = Some(t);
                }
            }
        }
    }

    fn statement(&mut self, stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for child in stats {
            self.statement(child);
        }
        let parent = Parent::Statement(stat);
        for e in stat.get_var_definitions().to_vec() {
            self.exprent(e, Use::Discard, parent);
        }
        if let Some(exprents) = stat.get_exprents() {
            for e in exprents.to_vec() {
                self.exprent(e, Use::Discard, parent);
            }
        }
        match stat.type_() {
            StatementType::Do => {
                let d = stat.as_do();
                match d.get_looptype() {
                    LoopType::ForEach => {
                        if let Some(init) = d.get_init_exprent() {
                            self.exclude_vars(init);
                            self.exprent(init, Use::Discard, parent);
                        }
                        if let Some(inc) = d.get_inc_exprent() {
                            self.exprent(inc, Use::Raw, parent);
                        }
                    }
                    _ => {
                        for e in d.get_init_exprent().into_iter().chain(d.get_inc_exprent()) {
                            self.exprent(e, Use::Discard, parent);
                        }
                        if let Some(cond) = d.get_condition_exprent() {
                            self.exprent(cond, Use::Raw, parent);
                        }
                    }
                }
            }
            StatementType::TryCatch | StatementType::CatchAll => {
                for e in stat.get_stat_exprents() {
                    self.exclude_vars(e);
                    self.exprent(e, Use::Discard, parent);
                }
            }
            _ => {
                for e in stat.get_stat_exprents() {
                    self.exprent(e, Use::Discard, parent);
                }
            }
        }
    }

    fn exclude_vars(&mut self, e: Exprent) {
        for v in e.get_all_exprents_rec_self(true, true) {
            if let Some(var) = v.try_as_var() {
                self.excluded.insert(var.get_index());
            }
        }
    }

    fn exprent(&mut self, e: Exprent, use_: Use, parent: Parent) {
        if self.uses.contains_key(&e) {
            return;
        }
        self.uses.insert(e, use_.clone());
        self.parents.insert(e, parent);
        self.order.push(e);
        let me = Parent::Exprent(e);
        if let Some(a) = e.try_as_assignment() {
            let (left, right) = (a.get_left(), a.get_right());
            let right_use = if a.get_cond_type().is_some() {
                self.exclude_vars(left);
                Use::Raw
            } else if let Some(var) = left.try_as_var() {
                self.targets.push(left);
                let index = var.get_index();
                if self.params.contains_key(&index) || self.is_param_slot(var) {
                    self.excluded.insert(index);
                    match self.params.get(&index) {
                        Some(held) => Use::Value(held.class.clone(), held.nullable),
                        None => Use::Unknown,
                    }
                } else {
                    self.defs.entry(index).or_default().push(right);
                    Use::Local(index)
                }
            } else if let Some(f) = left.try_as_field() {
                self.field_use(f.get_classname(), &f.get_name(), &f.get_descriptor().descriptor_string)
            } else {
                Use::Raw
            };
            if let Some(f) = left.try_as_field() {
                if let Some(instance) = f.get_instance() {
                    self.exprent(instance, Use::Raw, me);
                }
            } else if left.try_as_var().is_none() {
                for c in left.get_all_exprents() {
                    self.exprent(c, Use::Raw, Parent::Exprent(left));
                }
            }
            self.exprent(right, right_use, me);
            return;
        }
        if let Some(exit) = e.try_as_exit() {
            if let Some(value) = exit.get_value() {
                if let Some(var) = value.try_as_var().filter(|_| exit.get_exit_type() == ExitType::Return) {
                    self.returned.insert(var.get_index());
                }
                let u = match (exit.get_exit_type(), &self.ret) {
                    (ExitType::Return, Some(class)) => Use::Value(class.clone(), false),
                    _ => Use::Raw,
                };
                self.exprent(value, u, me);
            }
            return;
        }
        if let Some(inv) = e.try_as_invocation() {
            if let Some(instance) = inv.get_instance() {
                self.exprent(instance, Use::Raw, me);
            }
            let uses = self.call_uses(inv);
            for (i, arg) in inv.get_lst_parameters().to_vec().into_iter().enumerate() {
                if matches!(uses.get(i), Some(Use::Unknown)) {
                    self.exclude_vars(arg);
                }
                self.exprent(arg, uses.get(i).cloned().unwrap_or(Use::Unknown), me);
            }
            return;
        }
        if let Some(new) = e.try_as_new() {
            let uses = self.new_uses(e);
            for (i, c) in e.get_all_exprents().into_iter().enumerate() {
                let u = uses.as_ref().map_or(Use::Raw, |u| u.get(i).cloned().unwrap_or(Use::Unknown));
                if matches!(u, Use::Unknown) || new.is_lambda() || new.is_anonymous() {
                    self.exclude_vars(c);
                }
                self.exprent(c, u, me);
            }
            return;
        }
        if let Some(f) = e.try_as_function() {
            let operands = f.get_lst_operands().to_vec();
            let k_type = e.try_as_k_function().map(|k| k.kd(|d| d.k_type));
            let func = f.get_func_type();
            if matches!(func, FunctionType::Ppi | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Imm) {
                for o in &operands {
                    self.exclude_vars(*o);
                }
            }
            let null_check = matches!(func, FunctionType::Eq | FunctionType::Ne)
                && operands.iter().any(|o| is_null(*o));
            for (i, o) in operands.into_iter().enumerate() {
                if null_check {
                    self.exprent(o, Use::NullCheck, me);
                    continue;
                }
                let inherits = match k_type {
                    Some(KFunctionType::IfNull) | Some(KFunctionType::NotNull) => true,
                    Some(KFunctionType::None) | None => func == FunctionType::Ternary && i > 0,
                    _ => false,
                };
                if inherits {
                    self.branches.insert(o);
                }
                self.exprent(o, if inherits { use_.clone() } else { Use::Raw }, me);
            }
            return;
        }
        for c in e.get_all_exprents() {
            self.exprent(c, Use::Raw, me);
        }
    }

    fn is_param_slot(&self, var: crate::var_exprent::VarExprent) -> bool {
        var.get_version() == 0 && var.get_processor().is_some_and(|p| {
            let method = p.get_method();
            let mut slot = if method.has_modifier(ACC_STATIC) { 0 } else { 1 };
            for param in method.method_descriptor().params.iter() {
                slot += param.stack_size;
            }
            var.get_index() < slot
        })
    }

    fn field_use(&mut self, classname: String, name: &str, descriptor: &str) -> Use {
        let Some(cl) = DecompilerContext::get_struct_context().get_class(&classname) else {
            return if is_java_name(&classname) { Use::Raw } else { Use::Unknown };
        };
        if KotlinMetadataAnnotation::find(cl).is_none() {
            return Use::Raw;
        }
        let Some(field) = cl.get_field(name, descriptor).filter(|_| parsed(cl)) else {
            return Use::Unknown;
        };
        match KtAttrs::get(field, &KElement::KEY()) {
            Some(KElement::Property(p)) => match &p.type_ {
                Some(t) => self.use_of(&t.clone(), &VarType::new_signature(descriptor)),
                None => Use::Unknown,
            },
            _ => Use::Unknown,
        }
    }

    fn field_held(&mut self, classname: String, name: &str, descriptor: &str) -> Option<Held> {
        let cl = DecompilerContext::get_struct_context().get_class(&classname).filter(|cl| parsed(*cl))?;
        let field = cl.get_field(name, descriptor)?;
        match KtAttrs::get(field, &KElement::KEY()) {
            Some(KElement::Property(p)) => {
                let t = p.type_.clone()?;
                self.held(&t, &VarType::new_signature(descriptor))
            }
            _ => None,
        }
    }

    /// The uses of a call's arguments.
    fn call_uses(&mut self, inv: InvocationExprent) -> Vec<Use> {
        if let Some(uses) = self.call_uses.get(&inv.0) {
            return uses.clone();
        }
        let uses = self.compute_call_uses(inv);
        self.call_uses.insert(inv.0, uses.clone());
        uses
    }

    fn callee(&mut self, inv: InvocationExprent) -> Option<(KElement, Option<PropertyMethod>)> {
        if let Some(known) = self.callees.get(&inv.0) {
            return known.clone();
        }
        let callee = calling_data(inv);
        self.callees.insert(inv.0, callee.clone());
        callee
    }

    fn compute_call_uses(&mut self, inv: InvocationExprent) -> Vec<Use> {
        let n = inv.get_lst_parameters().len();
        let md = inv.get_descriptor();
        let classname = inv.get_classname();
        let owner = self.value_class(&classname);
        if let Some(class) = &owner {
            if let Some(uses) = self.impl_uses(inv, class.clone()) {
                return uses;
            }
        }
        let calling = self.callee(inv);
        match calling {
            Some((KElement::Function(f), _)) => {
                let mut types: Vec<Option<KType>> = Vec::new();
                if inv.is_static() && owner.is_some() && f.class_struct.is_some_and(|c| c.qualified_name() == classname) {
                    types.push(None);
                }
                types.extend(f.context_receiver_types.iter().cloned().map(Some));
                types.extend(f.receiver_type.clone().map(Some));
                types.extend(f.parameters.iter().map(|p| Some(p.type_.clone())));
                let suspend = Flags::IS_SUSPEND.get(f.flags) as usize;
                if n != types.len() + suspend || md.params.len() != n {
                    return vec![Use::Unknown; n];
                }
                let mut uses = Vec::new();
                for (i, t) in types.iter().enumerate() {
                    uses.push(match t {
                        Some(t) => self.use_of(t, &md.params[i]),
                        None => Use::Value(owner.clone().expect("owner"), false),
                    });
                }
                uses.resize(n, Use::Raw);
                uses
            }
            Some((KElement::Property(p), method)) => {
                let mut types: Vec<Option<KType>> = Vec::new();
                if inv.is_static() && owner.is_some() && p.class_struct.qualified_name() == classname {
                    types.push(None);
                }
                if let Some(extension) = &p.extension {
                    types.push(Some(extension.receiver.clone()));
                }
                if method == Some(PropertyMethod::Setter) {
                    types.push(p.type_.clone());
                }
                if n != types.len() || md.params.len() != n {
                    return vec![Use::Unknown; n];
                }
                let mut uses = Vec::new();
                for (i, t) in types.iter().enumerate() {
                    uses.push(match t {
                        Some(t) => self.use_of(t, &md.params[i]),
                        None => Use::Value(owner.clone().expect("owner"), false),
                    });
                }
                uses
            }
            Some(_) => vec![Use::Unknown; n],
            None => {
                if inv.get_invocation_type() == crate::invocation_exprent::InvocationType::Dynamic {
                    return vec![Use::Unknown; n];
                }
                match DecompilerContext::get_struct_context().get_class(&classname) {
                    Some(cl) => {
                        if KotlinMetadataAnnotation::find(cl).is_none() {
                            vec![Use::Raw; n]
                        } else {
                            vec![Use::Unknown; n]
                        }
                    }
                    None if is_java_name(&classname) => vec![Use::Raw; n],
                    None => vec![Use::Unknown; n],
                }
            }
        }
    }

    /// The uses of the arguments of `V`'s own plumbing.
    fn impl_uses(&mut self, inv: InvocationExprent, class: Rc<ValueClass>) -> Option<Vec<Use>> {
        let name = inv.get_name();
        let md = inv.get_descriptor();
        let n = inv.get_lst_parameters().len();
        let first = md.params.first().is_some_and(|p| same_type(&class.underlying, p));
        if !inv.is_static() {
            return None;
        }
        match name.as_str() {
            "constructor-impl" if n == 1 && first => {
                let u = match class.property_type.clone() {
                    Some(t) => self.use_of(&t, &md.params[0]),
                    None => Use::Raw,
                };
                Some(vec![u])
            }
            "box-impl" | "toString-impl" | "hashCode-impl" if n == 1 && first => {
                Some(vec![Use::Value(class, false)])
            }
            "equals-impl0" if n == 2 && first && same_type(&class.underlying, &md.params[1]) => {
                Some(vec![Use::Value(class.clone(), false), Use::Value(class, false)])
            }
            "equals-impl" if n == 2 && first => Some(vec![Use::Value(class, false), Use::Raw]),
            _ => None,
        }
    }

    /// The uses of the values a `new` takes; `None` for an array's.
    fn new_uses(&mut self, e: Exprent) -> Option<Vec<Use>> {
        let new = e.as_new();
        let ctor = new.get_constructor()?;
        let count = e.get_all_exprents().len();
        if new.is_lambda() || new.is_anonymous() || new.is_method_reference() {
            return Some(vec![Use::Unknown; count]);
        }
        let mut uses = Vec::new();
        if ctor.get_instance().is_some() {
            uses.push(Use::Raw);
        }
        let classname = ctor.get_classname();
        let Some(cl) = DecompilerContext::get_struct_context().get_class(&classname) else {
            let u = if is_java_name(&classname) { Use::Raw } else { Use::Unknown };
            return Some(vec![u; count]);
        };
        if KotlinMetadataAnnotation::find(cl).is_none() {
            return Some(vec![Use::Raw; count]);
        }
        if !parsed(cl) {
            return Some(vec![Use::Unknown; count]);
        }
        let md = ctor.get_descriptor();
        let params = ctor.get_lst_parameters().len();
        let method = cl.get_method("<init>", &ctor.get_string_descriptor());
        let constructor = match method.and_then(|m| KtAttrs::get(m, &KElement::KEY())) {
            Some(KElement::Constructor(c)) => c,
            _ => return Some(vec![Use::Unknown; count]),
        };
        let mut jvm: Vec<VarType> = md.params.to_vec();
        let marker = jvm
            .last()
            .is_some_and(|p| p.type_ == CodeType::Object && p.value_str() == MARKER);
        if marker {
            jvm.pop();
        }
        if jvm.len() < constructor.parameters.len() || md.params.len() != params {
            return Some(vec![Use::Unknown; count]);
        }
        let lead = jvm.len() - constructor.parameters.len();
        for (i, param) in md.params.iter().enumerate() {
            if i < lead || i >= jvm.len() {
                uses.push(Use::Raw);
            } else {
                let t = constructor.parameters[i - lead].type_.clone();
                uses.push(self.use_of(&t, param));
            }
        }
        Some(uses)
    }

    fn resolved(&self, u: &Use) -> Use {
        match u {
            Use::Local(index) => match self.locals.get(index) {
                Some(Some(class)) => Use::Value(class.clone(), false),
                Some(None) => Use::Pending,
                None => Use::Raw,
            },
            other => other.clone(),
        }
    }

    fn known_use(&self, e: Exprent) -> bool {
        !matches!(self.resolved(self.uses.get(&e).unwrap_or(&Use::Unknown)), Use::Unknown)
    }

    /// Whether `e` holds a `class` (or a `class?` held as `T?`, which a
    /// null check before the use smart-casts), as far as the typing knows.
    fn holds(&mut self, e: Exprent, class: &Rc<ValueClass>, _nullable: bool) -> bool {
        match self.ty(e) {
            Ty::Any => true,
            Ty::Is(held) => held.class.name == class.name,
            Ty::No => false,
        }
    }

    /// What `e` holds, and the form it is written in (`forms`).
    fn ty(&mut self, e: Exprent) -> Ty {
        if let Some(known) = self.tys.get(&e) {
            return known.clone();
        }
        let ty = self.compute_ty(e);
        self.tys.insert(e, ty.clone());
        ty
    }

    fn compute_ty(&mut self, e: Exprent) -> Ty {
        if let Some(var) = e.try_as_var() {
            let index = var.get_index();
            if self.is_param_slot(var) {
                if let Some(held) = self.params.get(&index) {
                    return Ty::Is(held.clone());
                }
            }
            return match self.locals.get(&index) {
                Some(Some(class)) => Ty::Is(Held { class: class.clone(), nullable: false }),
                Some(None) => Ty::Any,
                None => Ty::No,
            };
        }
        if let Some(f) = e.try_as_field() {
            return match self.field_held(f.get_classname(), &f.get_name(), &f.get_descriptor().descriptor_string) {
                Some(held) => Ty::Is(held),
                None => Ty::No,
            };
        }
        if let Some(f) = e.try_as_function() {
            let operands = f.get_lst_operands().to_vec();
            let k_type = e.try_as_k_function().map(|k| k.kd(|d| d.k_type));
            let branches: Vec<Exprent> = match k_type {
                Some(KFunctionType::IfNull) => operands,
                Some(KFunctionType::NotNull) => {
                    return match operands.first().map(|o| self.ty(*o)) {
                        Some(Ty::Is(held)) => Ty::Is(Held { nullable: false, ..held }),
                        Some(other) => other,
                        None => Ty::No,
                    };
                }
                Some(KFunctionType::None) | None if f.get_func_type() == FunctionType::Ternary => {
                    operands[1..].to_vec()
                }
                _ => return Ty::No,
            };
            let mut out = Ty::Any;
            for b in branches {
                out = match (out, self.ty(b)) {
                    (Ty::No, _) | (_, Ty::No) => Ty::No,
                    (Ty::Any, t) | (t, Ty::Any) => t,
                    (Ty::Is(a), Ty::Is(b)) if a.class.name == b.class.name => {
                        Ty::Is(Held { class: a.class, nullable: a.nullable || b.nullable })
                    }
                    _ => Ty::No,
                };
            }
            return out;
        }
        let Some(inv) = e.try_as_invocation() else {
            return Ty::No;
        };
        self.forms.remove(&e);
        let classname = inv.get_classname();
        let name = inv.get_name();
        let args = inv.get_lst_parameters().to_vec();
        let md = inv.get_descriptor();
        let owner = self.value_class(&classname);
        if let Some(class) = owner.clone().filter(|_| inv.is_static()) {
            match self.impl_uses(inv, class.clone()).map(|_| name.as_str()) {
                Some("constructor-impl") => {
                    let fits = match self.resolved(self.uses.get(&e).unwrap_or(&Use::Unknown)) {
                        Use::Value(c, _) => c.name == class.name,
                        Use::Discard | Use::Pending => true,
                        _ => false,
                    };
                    if !fits {
                        return Ty::No;
                    }
                    let form = if class.name == RESULT && !self.in_result {
                        if is_create_failure(args[0]) { Form::Failure } else { Form::Success }
                    } else {
                        Form::Constructor
                    };
                    self.forms.insert(e, form);
                    return Ty::Is(Held { class, nullable: false });
                }
                Some("box-impl") => {
                    if self.holds(args[0], &class, false) {
                        self.forms.insert(e, Form::Value);
                    }
                    return Ty::No;
                }
                Some("equals-impl0") => {
                    if self.holds(args[0], &class, false) && self.holds(args[1], &class, false) {
                        self.forms.insert(e, Form::Equals);
                    }
                    return Ty::No;
                }
                Some(member @ ("toString-impl" | "hashCode-impl" | "equals-impl"))
                    if self.callee(inv).is_none() =>
                {
                    if self.holds(args[0], &class, false) {
                        let name = member.strip_suffix("-impl").expect("-impl");
                        self.forms.insert(e, Form::Member { name: Rc::from(name), property: false });
                    }
                    return Ty::No;
                }
                _ => {}
            }
        }
        if let Some(class) = owner.clone().filter(|_| !inv.is_static() && name == "unbox-impl" && args.is_empty()) {
            if same_type(&class.underlying, &md.ret)
                && matches!(self.resolved(self.uses.get(&e).unwrap_or(&Use::Unknown)), Use::Value(..) | Use::Raw | Use::Pending)
            {
                self.forms.insert(e, Form::Value);
                return Ty::Is(Held { class, nullable: false });
            }
            return Ty::No;
        }
        let Some((element, method)) = self.callee(inv) else {
            return Ty::No;
        };
        let uses = self.call_uses(inv);
        let mut proven = !uses.iter().any(|u| matches!(u, Use::Unknown));
        for (i, u) in uses.iter().enumerate() {
            if let Use::Value(class, nullable) = u {
                proven &= self.holds(args[i], class, *nullable);
            }
        }
        let mangled = name.contains('-');
        let (result, member) = match &element {
            KElement::Function(f) => {
                let plain = f.receiver_type.is_none()
                    && f.context_receiver_types.is_empty()
                    && !Flags::IS_INFIX.get(f.flags);
                if Flags::IS_SUSPEND.get(f.flags) {
                    (None, None)
                } else {
                    (Some(f.return_type.clone()), Some((f.name.clone(), false, plain)))
                }
            }
            KElement::Property(p) if method == Some(PropertyMethod::Getter) => {
                (p.type_.clone(), Some((p.name.clone(), true, p.extension.is_none())))
            }
            _ => (None, None),
        };
        let held = result.and_then(|t| self.held(&t, &md.ret));
        let static_member = inv.is_static()
            && owner.is_some()
            && match &element {
                KElement::Function(f) => f.class_struct.is_some_and(|c| c.qualified_name() == classname),
                KElement::Property(p) => p.class_struct.qualified_name() == classname,
                _ => false,
            };
        let needs_use = held.is_some() && !self.known_use(e);
        let mut written = !mangled;
        if let Some((kotlin, property, plain)) = member {
            if static_member {
                written = false;
                if proven && !needs_use && plain {
                    self.forms.insert(e, Form::Member { name: Rc::from(kotlin.as_str()), property });
                    written = true;
                }
            } else if mangled && !inv.is_static() && proven && !needs_use && plain && !kotlin.is_empty() {
                let form = if !property {
                    Form::Renamed(Rc::from(kotlin.as_str()))
                } else if args.is_empty() && !written_by_plugin(inv) {
                    Form::Property(Rc::from(kotlin.as_str()))
                } else {
                    return Ty::No;
                };
                self.forms.insert(e, form);
                written = true;
            } else if property {
                written = written_by_plugin(inv);
            }
        }
        match held {
            Some(held) if written => Ty::Is(held),
            _ => Ty::No,
        }
    }

    /// Types the locals: the greatest set whose every assignment stores a
    /// value of one class.
    fn type_locals(&mut self) {
        let candidates: Vec<i32> = self
            .defs
            .keys()
            .copied()
            .filter(|i| !self.excluded.contains(i) && !self.params.contains_key(i))
            .collect();
        for index in &candidates {
            self.locals.insert(*index, None);
        }
        loop {
            self.tys.clear();
            let mut changed = false;
            for index in &candidates {
                if !self.locals.contains_key(index) {
                    continue;
                }
                let defs = self.defs.get(index).cloned().unwrap_or_default();
                let mut class: Option<Rc<ValueClass>> = None;
                let mut ok = true;
                for d in defs {
                    match self.ty(d) {
                        Ty::Is(held) if !held.nullable => match &class {
                            Some(c) if c.name != held.class.name => ok = false,
                            _ => class = Some(held.class),
                        },
                        Ty::Any => {}
                        _ => ok = false,
                    }
                }
                let current = self.locals.get(index).cloned().flatten();
                if !ok {
                    self.locals.remove(index);
                    changed = true;
                } else if class.is_some() && current.as_ref().map(|c| c.name.clone()) != class.as_ref().map(|c| c.name.clone()) {
                    self.locals.insert(*index, class);
                    changed = true;
                }
            }
            if !changed {
                let pending: Vec<i32> = self
                    .locals
                    .iter()
                    .filter(|(_, c)| c.is_none())
                    .map(|(i, _)| *i)
                    .collect();
                if pending.is_empty() {
                    break;
                }
                for index in pending {
                    self.locals.remove(&index);
                }
            }
        }
    }

    fn apply(&mut self, processor: VarProcessor) {
        self.tys.clear();
        self.forms.clear();
        let order = self.order.clone();
        let mut held: Vec<(Exprent, Held)> = Vec::new();
        for e in &order {
            if let Ty::Is(h) = self.ty(*e) {
                held.push((*e, h));
            }
        }
        for (e, form) in std::mem::take(&mut self.forms) {
            e.as_invocation().d_mut(|d| d.kuna_inline_class = Some(form));
        }
        let typed: BTreeMap<Exprent, String> =
            held.iter().map(|(e, h)| (*e, h.class.name.clone())).collect();
        for (e, h) in held {
            let joined = self.branches.contains(&e)
                && matches!(self.parents.get(&e), Some(Parent::Exprent(p)) if typed.get(p) == Some(&h.class.name));
            if joined {
                continue;
            }
            let raw = match self.resolved(self.uses.get(&e).unwrap_or(&Use::Unknown)) {
                Use::Raw => true,
                Use::NullCheck => !h.nullable,
                Use::Value(class, _) => class.name != h.class.name,
                _ => false,
            };
            if raw {
                self.wrap(e, &h);
            }
        }
        for e in self.targets.iter().chain(&self.order) {
            let local = e.try_as_var().is_some_and(|v| v.is_definition() && self.locals.contains_key(&v.get_index()));
            if let Some(var) = e.try_as_k_var().filter(|_| local) {
                var.kd_mut(|d| d.nullable = false);
            }
        }
        for (index, class) in &self.locals {
            if let Some(class) = class {
                let type_ = match &self.ret_type {
                    Some(t) if self.returned.contains(index)
                        && !class.params.is_empty()
                        && t.k().type_parameter_name.is_none()
                        && !t.k().is_nullable
                        && t.value.as_deref() == Some(class.name.as_str()) =>
                    {
                        t.0.clone()
                    }
                    _ => declared(class),
                };
                processor.set_kuna_typed(*index, KunaLocalType { type_, mutable: false, nullable: Vec::new() });
            }
        }
    }

    /// Replaces `e`, a `V` read as its underlying value, by `e.x`.
    fn wrap(&self, e: Exprent, held: &Held) {
        let Some(parent) = self.parents.get(&e).copied() else {
            return;
        };
        let class = &held.class;
        let inv = InvocationExprent::new();
        inv.set_name("unbox-impl".to_string());
        inv.set_classname(class.name.clone());
        let desc = format!("(){}", class.underlying);
        inv.set_string_descriptor(desc.clone());
        inv.set_descriptor(MethodDescriptor::parse_descriptor(&desc));
        inv.set_static(false);
        let k = KInvocationExprent::new(inv);
        k.0.set_instance(Some(e));
        k.0.d_mut(|d| {
            d.kuna_inline_class = Some(Form::Underlying {
                name: Rc::from(class.property.as_str()),
                nullable: held.nullable,
            })
        });
        let wrapped = k.0 .0;
        match parent {
            Parent::Exprent(p) => p.replace_exprent(e, wrapped),
            Parent::Statement(s) => s.replace_exprent(e, wrapped),
        }
    }
}

/// The declared type of a local holding `class`: its type arguments `Any?`
/// where the parameter is covariant and unbounded, else `*`.
fn declared(class: &ValueClass) -> VarType {
    let erasure = VarType::new_value(CodeType::Object, 0, class.name.as_str());
    let Some(kotlin_name) = class.kotlin_name.as_deref().filter(|_| !class.params.is_empty()) else {
        return erasure;
    };
    let any = KType::new(&VarType::new_signature("Ljava/lang/Object;"), "kotlin/Any", true, false, None, None, None);
    let args: Vec<TypeArgument> = class
        .params
        .iter()
        .map(|covariant| {
            TypeArgument::new(any.clone(), if *covariant { Projection::Inv } else { Projection::Star })
        })
        .collect();
    KType::new(&erasure, kotlin_name, false, false, Some(Rc::from(args)), None, None).0
}

/// The text of a local's declared type that is a Kotlin type.
pub fn declared_text(t: &VarType) -> String {
    KType(t.clone()).stringify(0).convert_to_string_and_allow_data_discard()
}

/// Whether the Kotlin writer found the callee of `inv` itself, and so writes
/// a property's accessor call as the property.
fn written_by_plugin(inv: InvocationExprent) -> bool {
    inv.0
        .try_as_k_invocation()
        .is_some_and(|k| k.kd(|d| d.calling_data.is_some()))
}

/// The metadata of the callee of `inv`: what the Kotlin writer found, else,
/// for a mangled name, the part of a multifile facade or the supertype that
/// declares the method, when their metadata is read.
fn calling_data(inv: InvocationExprent) -> Option<(KElement, Option<PropertyMethod>)> {
    let k = inv.0.try_as_k_invocation()?;
    let (element, method) = k.kd(|d| (d.calling_data.clone(), d.property_method));
    if let Some(element) = element {
        return Some((element, method));
    }
    let (name, descriptor) = (inv.get_name(), inv.get_string_descriptor());
    if !name.contains('-') {
        return None;
    }
    let context = DecompilerContext::get_struct_context();
    let cl = context.get_class(&inv.get_classname()).filter(|cl| parsed(*cl))?;
    let part = match KtAttrs::get(cl, &KElement::KEY()) {
        Some(KElement::MultifileFacade(facade)) => facade
            .files
            .iter()
            .filter_map(|part| context.get_class(part).filter(|p| parsed(*p)))
            .find_map(|part| part.get_method(&name, &descriptor)),
        _ => None,
    };
    let method = part.or_else(|| {
        cl.get_method_recursive(&name, &descriptor)
            .filter(|m| context.get_class(&m.get_class_qualified_name()).is_some_and(parsed))
    })?;
    let element = KtAttrs::get(method, &KElement::KEY())?;
    Some((element, KtAttrs::get(method, &PropertyMethod::KEY())))
}

/// Whether the metadata of `cl` has been read; the fix reads none but a
/// value class's, so that it changes nothing the order of reading decides.
fn parsed(cl: StructClass) -> bool {
    KtAttrs::has(cl, &KElement::KEY())
}

fn is_null(e: Exprent) -> bool {
    e.try_as_const().is_some_and(|c| c.get_const_type().type_ == CodeType::Null)
}

fn is_java_name(classname: &str) -> bool {
    ["java/", "javax/", "jdk/", "sun/"].iter().any(|p| classname.starts_with(p))
}

fn is_create_failure(e: Exprent) -> bool {
    e.try_as_invocation().is_some_and(|i| {
        i.is_static()
            && i.get_classname() == CREATE_FAILURE.0
            && i.get_name() == CREATE_FAILURE.1
            && i.get_string_descriptor() == CREATE_FAILURE.2
            && i.get_lst_parameters().len() == 1
    })
}

fn same_type(a: &VarType, b: &VarType) -> bool {
    a.type_ == b.type_ && a.array_dim == b.array_dim && a.value == b.value
}

/// The inline class `name`, when its class file is loaded and its metadata
/// says it is one: its one instance field holds the underlying value.
fn load_value_class(name: &str) -> Option<ValueClass> {
    let cl = DecompilerContext::get_struct_context().get_class(name)?;
    let methods = cl.get_methods().to_vec();
    if !["box-impl", "unbox-impl"].iter().all(|m| methods.iter().any(|x| x.get_name() == *m)) {
        return None;
    }
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::Class(k)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    if !Flags::IS_VALUE_CLASS.get(k.flags()) {
        return None;
    }
    let fields: Vec<_> = cl
        .get_fields()
        .to_vec()
        .into_iter()
        .filter(|f| !f.has_modifier(ACC_STATIC))
        .collect();
    let [field] = fields.as_slice() else {
        return None;
    };
    let property_type = match KtAttrs::get(*field, &KElement::KEY()) {
        Some(KElement::Property(p)) => p.type_.clone(),
        _ => None,
    };
    let params = (0..k.proto.get_type_parameter_count())
        .map(|i| {
            let p = k.proto.get_type_parameter(i);
            p.get_variance() == Variance::Out && p.get_upper_bound_count() == 0
        })
        .collect();
    Some(ValueClass {
        name: name.to_string(),
        kotlin_name: k.resolver.as_ref().map(|r| r.resolve(k.proto.get_fq_name())),
        property: field.get_name(),
        underlying: VarType::new_signature(&field.get_descriptor()),
        property_type,
        params,
    })
}

/// Whether `e`'s printed text is its operand's (`Value`), for precedence.
pub fn written_operand(inv: InvocationExprent) -> Option<Exprent> {
    match inv.d(|d| d.kuna_inline_class.clone())? {
        Form::Value => inv.get_instance().or_else(|| inv.get_lst_parameters().to_vec().first().copied()),
        _ => None,
    }
}

/// The precedence of a call written as `==`.
pub fn precedence(inv: InvocationExprent) -> Option<i32> {
    match inv.d(|d| d.kuna_inline_class.clone())? {
        Form::Value => written_operand(inv).map(|o| o.get_precedence()),
        Form::Equals => Some(FunctionType::Eq.precedence()),
        _ => None,
    }
}

/// The call as the fix writes it, when it does.
pub fn write(inv: InvocationExprent, indent: i32) -> Option<TextBuffer> {
    let form = inv.d(|d| d.kuna_inline_class.clone())?;
    let args = inv.get_lst_parameters().to_vec();
    let mut buf = TextBuffer::new();
    match &form {
        Form::Constructor | Form::Success | Form::Failure => {
            let call = if form == Form::Failure { args.first()?.try_as_invocation()? } else { inv };
            let name = KTypes::get_kotlin_type(Some(&VarType::new_value(
                CodeType::Object,
                0,
                inv.get_classname().as_str(),
            )));
            buf.append(&name);
            match form {
                Form::Success => buf.append(".success"),
                Form::Failure => buf.append(".failure"),
                _ => &mut buf,
            };
            buf.append("(");
            buf.append_buffer(&call.append_param_list(indent));
            buf.append(")");
        }
        Form::Value => {
            let operand = written_operand(inv)?;
            buf.append_buffer(&operand.to_java(indent));
        }
        Form::Member { name, property } => {
            let recv = *args.first()?;
            let mut recv_buf = recv.to_java(indent);
            if recv.get_precedence() > inv.0.get_precedence_base() {
                recv_buf.enclose("(", ")");
            }
            buf.append_buffer(&recv_buf).append(".");
            buf.append(&kt_unescaped_name::member(name));
            if !property {
                buf.append("(");
                buf.append_buffer(&args_after_receiver(inv, indent)).append(")");
            }
        }
        Form::Renamed(name) => {
            return Some(crate::fixes::kt_calls::kt_jvm_name_call::to_java_named(inv, name.to_string(), indent));
        }
        Form::Equals => {
            let precedence = FunctionType::Eq.precedence();
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    buf.append(" == ");
                }
                let mut a_buf = a.to_java(indent);
                if a.get_precedence() >= precedence {
                    a_buf.enclose("(", ")");
                }
                buf.append_buffer(&a_buf);
            }
        }
        Form::Property(name) => {
            let instance = inv.get_instance()?;
            let mut instance_buf = instance.to_java(indent);
            if instance.get_precedence() > inv.0.get_precedence_base() {
                instance_buf.enclose("(", ")");
            }
            buf.append_buffer(&instance_buf).append(".");
            buf.append(&kt_unescaped_name::member(name));
        }
        Form::Underlying { name, nullable } => {
            let instance = inv.get_instance()?;
            let mut instance_buf = instance.to_java(indent);
            if instance.get_precedence() > inv.0.get_precedence_base() {
                instance_buf.enclose("(", ")");
            }
            buf.append_buffer(&instance_buf);
            buf.append(if *nullable { "?." } else { "." });
            buf.append(&kt_unescaped_name::member(name));
        }
    }
    buf.add_bytecode_mapping(inv.get_bytecode().as_ref());
    Some(buf)
}
