//! (kuna `swiftdemangle`) The full-form printer: renders a demangling tree
//! the way `swift-demangle` does (`Swift.String.count.getter : Swift.Int`,
//! `protocol witness for … in conformance …`). Every node kind the parser
//! can build has a rendering; a kind without one fails the whole print so
//! the caller falls back to the mangled spelling.

use super::node::{Kind, Node, NodeRef};
use super::parse::{
    BOX_TO_STACK,
    BOX_TO_VALUE,
    CLOSURE_PROP,
    CP_FLOAT,
    CP_FUNCTION,
    CP_GLOBAL,
    CP_INTEGER,
    CP_KEYPATH,
    CP_STRING,
    DEAD,
    EXISTENTIAL_TO_GENERIC,
    GUARANTEED_TO_OWNED,
    INOUT_TO_OUT,
    OWNED_TO_GUARANTEED,
    SROA,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum TypePrinting {
    NoType,
    WithColon,
    FunctionStyle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sugar {
    None,
    Optional,
    Iuo,
    Array,
    Dictionary,
}

pub struct Printer {
    out: String,
    failed: bool,
    sugar: bool,
    depth: usize,
}

/// Render `node` in the `swift-demangle` format, or `None` when some node
/// kind has no rendering.
pub fn print_full(node: &NodeRef) -> Option<String> {
    print_with(node, true)
}

fn print_with(node: &NodeRef, sugar: bool) -> Option<String> {
    let mut p = Printer {
        out: String::new(),
        failed: false,
        sugar,
        depth: 0,
    };
    p.print(node, false);
    if p.failed { None } else { Some(p.out) }
}

fn is_identifier(n: &Node, text: &str) -> bool {
    n.kind == Kind::Identifier && n.get_text() == text
}

fn is_swift_module(n: &Node) -> bool {
    n.kind == Kind::Module && n.get_text() == "Swift"
}

fn is_simple_type(n: &Node) -> bool {
    match n.kind {
        Kind::AssociatedType
        | Kind::AssociatedTypeRef
        | Kind::BoundGenericClass
        | Kind::BoundGenericEnum
        | Kind::BoundGenericStructure
        | Kind::BoundGenericProtocol
        | Kind::BoundGenericOtherNominalType
        | Kind::BoundGenericTypeAlias
        | Kind::BoundGenericFunction
        | Kind::BuiltinTypeName
        | Kind::BuiltinFixedArray
        | Kind::Class
        | Kind::DependentGenericType
        | Kind::DependentMemberType
        | Kind::DependentGenericParamType
        | Kind::DynamicSelf
        | Kind::Enum
        | Kind::ErrorType
        | Kind::ExistentialMetatype
        | Kind::Integer
        | Kind::NegativeInteger
        | Kind::Metatype
        | Kind::MetatypeRepresentation
        | Kind::Module
        | Kind::Tuple
        | Kind::Pack
        | Kind::PackElement
        | Kind::PackElementLevel
        | Kind::PackExpansion
        | Kind::Protocol
        | Kind::ConstrainedExistential
        | Kind::ConstrainedExistentialRequirementList
        | Kind::ConstrainedExistentialSelf
        | Kind::Structure
        | Kind::OtherNominalType
        | Kind::TupleElementName
        | Kind::TypeAlias
        | Kind::TypeList
        | Kind::LabelList
        | Kind::SugaredOptional
        | Kind::SugaredArray
        | Kind::SugaredDictionary
        | Kind::SugaredParen
        | Kind::OpaqueReturnType
        | Kind::OpaqueReturnTypeIndex
        | Kind::OpaqueType => true,
        Kind::Type => n.first().is_some_and(|c| is_simple_type(c)),
        Kind::ProtocolList => n.first().is_some_and(|l| l.len() <= 1),
        Kind::ProtocolListWithAnyObject | Kind::ProtocolListWithClass => n
            .first()
            .and_then(|p| p.first())
            .is_some_and(|l| l.is_empty()),
        _ => false,
    }
}

fn needs_space_before_type(n: &Node) -> bool {
    match n.kind {
        Kind::Type => n.first().is_some_and(|c| needs_space_before_type(c)),
        Kind::FunctionType
        | Kind::NoEscapeFunctionType
        | Kind::UncurriedFunctionType
        | Kind::DependentGenericType => false,
        _ => true,
    }
}

fn find_sugar(n: &Node) -> Sugar {
    if n.len() == 1 && n.kind == Kind::Type {
        return n.first().map_or(Sugar::None, |c| find_sugar(c));
    }
    if n.len() != 2 {
        return Sugar::None;
    }
    if n.kind != Kind::BoundGenericEnum && n.kind != Kind::BoundGenericStructure {
        return Sugar::None;
    }
    let Some(unbound) = n.first().and_then(|t| t.first()) else {
        return Sugar::None;
    };
    let Some(args) = n.child(1) else {
        return Sugar::None;
    };
    let Some(module) = unbound.first() else {
        return Sugar::None;
    };
    let Some(name) = unbound.child(1) else {
        return Sugar::None;
    };
    if !is_swift_module(module) {
        return Sugar::None;
    }
    if n.kind == Kind::BoundGenericEnum {
        if is_identifier(name, "Optional") && args.len() == 1 {
            return Sugar::Optional;
        }
        if is_identifier(name, "ImplicitlyUnwrappedOptional") && args.len() == 1 {
            return Sugar::Iuo;
        }
        return Sugar::None;
    }
    if is_identifier(name, "Array") && args.len() == 1 {
        return Sugar::Array;
    }
    if is_identifier(name, "Dictionary") && args.len() == 2 {
        return Sugar::Dictionary;
    }
    Sugar::None
}

impl Printer {
    fn s(&mut self, text: &str) {
        self.out.push_str(text);
    }

    fn fail(&mut self) -> Option<NodeRef> {
        self.failed = true;
        None
    }

    fn print_children(&mut self, n: &Node, sep: &str) {
        for (i, c) in n.children.iter().enumerate() {
            if i > 0 {
                self.s(sep);
            }
            self.print(c, false);
        }
    }

    fn print_child(&mut self, n: &Node, i: usize) {
        match n.child(i) {
            Some(c) => {
                self.print(c, false);
            }
            None => self.failed = true,
        }
    }

    /// `true` when `ctx` is printed at all as a qualifier.
    fn print_context(&self, ctx: &Node) -> bool {
        if ctx.kind == Kind::Module {
            // The stdlib and the ObjC module are displayed (`Swift.Int`, `__C.NSObject`).
            return true;
        }
        true
    }

    fn print_with_parens(&mut self, ty: &NodeRef) {
        let parens = !is_simple_type(ty);
        if parens {
            self.s("(");
        }
        self.print(ty, false);
        if parens {
            self.s(")");
        }
    }

    fn print_bound_generic_no_sugar(&mut self, n: &Node) {
        if n.len() < 2 {
            return;
        }
        self.print_child(n, 0);
        self.s("<");
        if let Some(list) = n.child(1) {
            self.print_children(list, ", ");
        }
        self.s(">");
    }

    fn print_bound_generic(&mut self, n: &Node) {
        if n.len() < 2 {
            return;
        }
        if n.len() != 2 {
            self.print_bound_generic_no_sugar(n);
            return;
        }
        if n.kind == Kind::BoundGenericClass || !self.sugar {
            self.print_bound_generic_no_sugar(n);
            return;
        }
        if n.kind == Kind::BoundGenericProtocol {
            if let Some(list) = n.child(1) {
                self.print_children(list, "");
            }
            self.s(" as ");
            self.print_child(n, 0);
            return;
        }
        let args = n.child(1).map(|l| l.children.clone()).unwrap_or_default();
        match find_sugar(n) {
            Sugar::None => self.print_bound_generic_no_sugar(n),
            Sugar::Optional | Sugar::Iuo => {
                let ty = args[0].clone();
                self.print_with_parens(&ty);
                self.s(if find_sugar(n) == Sugar::Optional {
                    "?"
                } else {
                    "!"
                });
            }
            Sugar::Array => {
                self.s("[");
                self.print(&args[0], false);
                self.s("]");
            }
            Sugar::Dictionary => {
                self.s("[");
                self.print(&args[0], false);
                self.s(" : ");
                self.print(&args[1], false);
                self.s("]");
            }
        }
    }

    fn print_function_parameters(&mut self, labels: Option<&NodeRef>, arg_tuple: &Node) {
        if arg_tuple.kind != Kind::ArgumentTuple {
            self.failed = true;
            return;
        }
        let Some(params_ty) = arg_tuple.first() else {
            self.failed = true;
            return;
        };
        if params_ty.kind != Kind::Type {
            self.failed = true;
            return;
        }
        let Some(params) = params_ty.first() else {
            self.failed = true;
            return;
        };
        if params.kind != Kind::Tuple {
            self.s("(");
            self.print(params, false);
            self.s(")");
            return;
        }
        let has_labels = labels.is_some_and(|l| !l.is_empty());
        self.s("(");
        for (i, param) in params.children.iter().enumerate() {
            if i > 0 {
                self.s(", ");
            }
            if has_labels {
                let label = labels.and_then(|l| l.child(i)).cloned();
                match label {
                    Some(l) if l.kind == Kind::Identifier => {
                        let t = l.get_text().to_string();
                        self.s(&t);
                    }
                    Some(_) => self.s("_"),
                    None => {
                        self.failed = true;
                        return;
                    }
                }
                self.s(": ");
            }
            self.print(param, false);
        }
        self.s(")");
    }

    fn print_function_type(&mut self, labels: Option<&NodeRef>, n: &Node) {
        let mut idx = 0;
        let mut diff: Option<NodeRef> = None;
        let mut throws: Option<NodeRef> = None;
        let mut is_sendable = false;
        let mut is_async = false;
        let mut sending_result = false;
        let mut global_actor: Option<NodeRef> = None;
        let mut isolated_any = false;
        let mut nonisolated_caller = false;
        let mut clang: Option<NodeRef> = None;
        while let Some(c) = n.child(idx) {
            match c.kind {
                Kind::ClangType => clang = Some(c.clone()),
                Kind::GlobalActorFunctionType => global_actor = Some(c.clone()),
                Kind::IsolatedAnyFunctionType => isolated_any = true,
                Kind::NonIsolatedCallerFunctionType => nonisolated_caller = true,
                Kind::SendingResultFunctionType => sending_result = true,
                Kind::DifferentiableFunctionType => diff = Some(c.clone()),
                Kind::ThrowsAnnotation | Kind::TypedThrowsAnnotation => throws = Some(c.clone()),
                Kind::ConcurrentFunctionType => is_sendable = true,
                Kind::AsyncAnnotation => is_async = true,
                _ => break,
            }
            idx += 1;
        }
        match n.kind {
            Kind::NoEscapeFunctionType | Kind::FunctionType | Kind::UncurriedFunctionType => {}
            Kind::AutoClosureType | Kind::EscapingAutoClosureType => self.s("@autoclosure "),
            Kind::ThinFunctionType => self.s("@convention(thin) "),
            Kind::CFunctionPointer => {
                self.s("@convention(c");
                if let Some(c) = &clang {
                    self.s(", mangledCType: \"");
                    let t = c.get_text().to_string();
                    self.s(&t);
                    self.s("\"");
                }
                self.s(") ");
            }
            Kind::EscapingObjCBlock => {
                self.s("@escaping @convention(block");
                if let Some(c) = &clang {
                    self.s(", mangledCType: \"");
                    let t = c.get_text().to_string();
                    self.s(&t);
                    self.s("\"");
                }
                self.s(") ");
            }
            Kind::ObjCBlock => {
                self.s("@convention(block");
                if let Some(c) = &clang {
                    self.s(", mangledCType: \"");
                    let t = c.get_text().to_string();
                    self.s(&t);
                    self.s("\"");
                }
                self.s(") ");
            }
            _ => {
                self.failed = true;
                return;
            }
        }
        if let Some(d) = &diff {
            match d.get_text() {
                "_forward" => self.s("@differentiable(_forward) "),
                "reverse" => self.s("@differentiable(reverse) "),
                "_linear" => self.s("@differentiable(_linear) "),
                _ => self.s("@differentiable "),
            }
        }
        if let Some(ga) = &global_actor {
            self.s("@");
            self.print_child(ga, 0);
            self.s(" ");
        } else if isolated_any {
            self.s("@isolated(any) ");
        } else if nonisolated_caller {
            self.s("nonisolated(nonsending) ");
        }
        if is_sendable {
            self.s("@Sendable ");
        }
        let Some(args) = n.child(idx) else {
            self.failed = true;
            return;
        };
        self.print_function_parameters(labels, args);
        if is_async {
            self.s(" async");
        }
        if let Some(t) = &throws {
            if t.kind == Kind::TypedThrowsAnnotation {
                self.s(" throws(");
                self.print_child(t, 0);
                self.s(")");
            } else {
                self.s(" throws");
            }
        }
        let Some(ret) = n.child(idx + 1) else {
            self.failed = true;
            return;
        };
        self.s(" -> ");
        if sending_result {
            self.s("sending ");
        }
        self.print_children(ret, "");
    }

    fn print_entity_type(&mut self, entity: &Node, ty: &NodeRef, generic_list: Option<&NodeRef>) {
        let labels = entity.child_of_kind(Kind::LabelList).cloned();
        if labels.is_some() || generic_list.is_some() {
            let mut ty = ty.clone();
            if let Some(gl) = generic_list {
                self.s("<");
                self.print_children(gl, ", ");
                self.s(">");
            }
            if ty.kind == Kind::DependentGenericType {
                if generic_list.is_none() {
                    self.print_child(&ty, 0);
                }
                let Some(dep) = ty.child(1).cloned() else {
                    self.failed = true;
                    return;
                };
                if needs_space_before_type(&dep) {
                    self.s(" ");
                }
                let Some(inner) = dep.first().cloned() else {
                    self.failed = true;
                    return;
                };
                ty = inner;
            }
            self.print_function_type(labels.as_ref(), &ty);
        } else {
            self.print(ty, false);
        }
    }

    fn print_entity(
        &mut self,
        entity: &NodeRef,
        as_prefix_context: bool,
        type_pr: TypePrinting,
        has_name: bool,
        extra_name: &str,
        extra_index: Option<u64>,
        overwrite_name: &str,
    ) -> Option<NodeRef> {
        let mut entity = entity.clone();
        let mut generic_list: Option<NodeRef> = None;
        if entity.kind == Kind::BoundGenericFunction {
            generic_list = entity.child(1).cloned();
            entity = entity.first()?.clone();
        }
        let mut multi_word = extra_name.contains(' ');
        let local_name = has_name
            && entity
                .child(1)
                .is_some_and(|c| c.kind == Kind::LocalDeclName);
        if local_name {
            multi_word = true;
        }
        if as_prefix_context && (type_pr != TypePrinting::NoType || multi_word) {
            return Some(entity);
        }
        let mut postfix_context: Option<NodeRef> = None;
        let context = entity.first()?.clone();
        if self.print_context(&context) {
            if multi_word {
                postfix_context = Some(context);
            } else {
                let before = self.out.len();
                postfix_context = self.print(&context, true);
                if self.out.len() != before {
                    self.s(".");
                }
            }
        }
        let mut extra_name = extra_name;
        let mut extra_index = extra_index;
        let private = entity.child_of_kind(Kind::PrivateDeclName).cloned();
        if !has_name && overwrite_name.is_empty() && entity.kind == Kind::Constructor {
            if let Some(p) = &private {
                self.print(p, false);
                if !extra_name.is_empty() {
                    self.s(".");
                }
            }
        }
        if has_name || !overwrite_name.is_empty() {
            if !extra_name.is_empty() && multi_word {
                self.s(extra_name);
                if let Some(i) = extra_index {
                    self.s(&i.to_string());
                }
                self.s(" of ");
                extra_name = "";
                extra_index = None;
            }
            let before = self.out.len();
            if !overwrite_name.is_empty() {
                self.s(overwrite_name);
            } else {
                let name = entity.child(1)?.clone();
                if name.kind != Kind::PrivateDeclName {
                    self.print(&name, false);
                }
                if let Some(private) = entity.child_of_kind(Kind::PrivateDeclName).cloned() {
                    self.print(&private, false);
                }
            }
            if self.out.len() != before && !extra_name.is_empty() {
                self.s(".");
            }
        }
        if !extra_name.is_empty() {
            self.s(extra_name);
            if let Some(i) = extra_index {
                self.s(&i.to_string());
            }
        }
        if type_pr != TypePrinting::NoType {
            let ty = entity.child_of_kind(Kind::Type)?.clone();
            let mut type_pr = type_pr;
            let mut t = ty.first()?.clone();
            if type_pr == TypePrinting::FunctionStyle {
                while t.kind == Kind::DependentGenericType {
                    t = t.child(1)?.first()?.clone();
                }
                if !matches!(
                    t.kind,
                    Kind::FunctionType
                        | Kind::NoEscapeFunctionType
                        | Kind::UncurriedFunctionType
                        | Kind::CFunctionPointer
                        | Kind::ThinFunctionType
                ) {
                    type_pr = TypePrinting::WithColon;
                }
            }
            let inner = ty.first()?.clone();
            if type_pr == TypePrinting::WithColon {
                self.s(" : ");
                self.print_entity_type(&entity, &inner, generic_list.as_ref());
            } else {
                if multi_word || needs_space_before_type(&inner) {
                    self.s(" ");
                }
                self.print_entity_type(&entity, &inner, generic_list.as_ref());
            }
        }
        if !as_prefix_context {
            if let Some(pc) = postfix_context.take() {
                if matches!(
                    entity.kind,
                    Kind::DefaultArgumentInitializer
                        | Kind::Initializer
                        | Kind::PropertyWrapperBackingInitializer
                        | Kind::PropertyWrapperInitFromProjectedValue
                ) {
                    self.s(" of ");
                } else {
                    self.s(" in ");
                }
                self.print(&pc, false);
            }
        }
        postfix_context
    }

    fn print_abstract_storage(
        &mut self,
        n: &NodeRef,
        as_prefix_context: bool,
        extra_name: &str,
    ) -> Option<NodeRef> {
        match n.kind {
            Kind::Variable => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::WithColon,
                true,
                extra_name,
                None,
                "",
            ),
            Kind::Subscript => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::WithColon,
                false,
                extra_name,
                None,
                "subscript",
            ),
            _ => self.fail(),
        }
    }

    fn print_simple_entity_prefix(
        &mut self,
        n: &Node,
        as_prefix_context: bool,
        prefix: &str,
    ) -> Option<NodeRef> {
        let _ = as_prefix_context;
        self.s(prefix);
        self.print_child(n, 0);
        None
    }

    fn print_spec_prefix(&mut self, n: &Node, description: &str, param_prefix: &str) {
        self.s(description);
        self.s(" <");
        let mut sep = "";
        let mut arg_num = 0u64;
        for c in &n.children {
            match c.kind {
                Kind::SpecializationPassID | Kind::DroppedArgument => {}
                Kind::IsSerialized => {
                    self.s(sep);
                    sep = ", ";
                    self.print(c, false);
                }
                _ => {
                    if !c.is_empty() {
                        self.s(sep);
                        self.s(param_prefix);
                        sep = ", ";
                        if c.kind == Kind::FunctionSignatureSpecializationParam {
                            self.s("Arg[");
                            self.s(&arg_num.to_string());
                            self.s("] = ");
                        }
                        self.print(c, false);
                    }
                    arg_num += 1;
                }
            }
        }
        self.s("> of ");
    }

    fn print_func_sig_spec_param(&mut self, n: &Node, mut idx: usize) -> usize {
        let Some(first) = n.child(idx) else {
            self.failed = true;
            return idx;
        };
        let v = first.get_index();
        match v {
            BOX_TO_VALUE | BOX_TO_STACK | INOUT_TO_OUT => {
                self.print_child(n, idx);
                idx += 1;
            }
            CP_FUNCTION | CP_GLOBAL => {
                self.s("[");
                self.print_child(n, idx);
                idx += 1;
                self.s(" : ");
                let text = n
                    .child(idx)
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                idx += 1;
                match super::demangle_swift_tree(&text).and_then(|t| print_with(&t, false)) {
                    Some(d) => self.s(&d),
                    None => self.failed = true,
                }
                self.s("]");
            }
            CP_INTEGER | CP_FLOAT => {
                self.s("[");
                self.print_child(n, idx);
                idx += 1;
                self.s(" : ");
                self.print_child(n, idx);
                idx += 1;
                self.s("]");
            }
            CP_STRING => {
                self.s("[");
                self.print_child(n, idx);
                idx += 1;
                self.s(" : ");
                self.print_child(n, idx);
                idx += 1;
                self.s("'");
                self.print_child(n, idx);
                idx += 1;
                self.s("'");
                self.s("]");
            }
            CP_KEYPATH => {
                self.s("[");
                self.print_child(n, idx);
                idx += 1;
                self.s(" : ");
                self.print_child(n, idx);
                idx += 1;
                self.s("<");
                self.print_child(n, idx);
                idx += 1;
                self.s(",");
                self.print_child(n, idx);
                idx += 1;
                self.s(">]");
            }
            CLOSURE_PROP => {
                self.s("[");
                self.print_child(n, idx);
                idx += 1;
                self.s(" : ");
                let name = n
                    .child(idx)
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                self.s(&name);
                idx += 1;
                self.s(", Argument Types : [");
                while idx < n.len() {
                    self.print_child(n, idx);
                    idx += 1;
                }
                self.s("]");
            }
            _ => {
                let mut printed = false;
                let mut flag = |p: &mut Printer, bit: u64, name: &str| {
                    if v & bit != 0 {
                        if printed {
                            p.s(" and ");
                        }
                        printed = true;
                        p.s(name);
                    }
                };
                flag(
                    self,
                    EXISTENTIAL_TO_GENERIC,
                    "Existential To Protocol Constrained Generic",
                );
                flag(self, DEAD, "Dead");
                flag(self, OWNED_TO_GUARANTEED, "Owned To Guaranteed");
                flag(self, GUARANTEED_TO_OWNED, "Guaranteed To Owned");
                flag(self, SROA, "Exploded");
                idx += 1;
            }
        }
        idx
    }

    /// Render `n`; returns the context that could not be printed in prefix
    /// form (the caller prints it as ` in <context>`).
    pub fn print(&mut self, n: &NodeRef, as_prefix_context: bool) -> Option<NodeRef> {
        if self.depth > 512 || self.out.len() > (1 << 20) {
            return self.fail();
        }
        self.depth += 1;
        let r = self.print_node(n, as_prefix_context);
        self.depth -= 1;
        r
    }

    fn print_node(&mut self, n: &NodeRef, as_prefix_context: bool) -> Option<NodeRef> {
        use Kind::*;
        match n.kind {
            Global => {
                self.print_children(n, "");
                None
            }
            Suffix => {
                self.s(" with unmangled suffix \"");
                let t = n.get_text().to_string();
                self.s(&t);
                self.s("\"");
                None
            }
            Identifier | Module | TupleElementName | PrefixOperator | PostfixOperator
            | InfixOperator => {
                let t = n.get_text().to_string();
                self.s(&t);
                match n.kind {
                    PrefixOperator => self.s(" prefix"),
                    PostfixOperator => self.s(" postfix"),
                    InfixOperator => self.s(" infix"),
                    _ => {}
                }
                None
            }
            PrivateDeclName => {
                if n.len() > 1 {
                    self.s("(");
                    self.print_child(n, 1);
                    self.s(" in ");
                    let t = n
                        .first()
                        .map(|c| c.get_text().to_string())
                        .unwrap_or_default();
                    self.s(&t);
                    self.s(")");
                } else {
                    self.s("(in ");
                    let t = n
                        .first()
                        .map(|c| c.get_text().to_string())
                        .unwrap_or_default();
                    self.s(&t);
                    self.s(")");
                }
                None
            }
            LocalDeclName => {
                self.print_child(n, 1);
                self.s(" #");
                let i = n
                    .first()
                    .map(|c| c.get_index())
                    .unwrap_or(0)
                    .saturating_add(1);
                self.s(&i.to_string());
                None
            }
            RelatedEntityDeclName => {
                self.s("related decl '");
                let t = n
                    .first()
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                self.s(&t);
                self.s("' for ");
                self.print_child(n, 1);
                None
            }
            Class | Structure | Enum | Protocol | TypeAlias | OtherNominalType => self
                .print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::NoType,
                    true,
                    "",
                    None,
                    "",
                ),
            Extension => {
                self.s("(extension in ");
                self.print_child(n, 0);
                self.s("):");
                self.print_child(n, 1);
                if let Some(sig) = n.child(2) {
                    let sig = sig.clone();
                    self.print(&sig, false);
                }
                None
            }
            AnonymousContext => {
                self.print_child(n, 1);
                self.s(".(unknown context at ");
                let t = n
                    .first()
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                self.s(&t);
                self.s(")");
                None
            }
            Function | BoundGenericFunction => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::FunctionStyle,
                true,
                "",
                None,
                "",
            ),
            Variable => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::WithColon,
                true,
                "",
                None,
                "",
            ),
            Subscript => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::FunctionStyle,
                false,
                "",
                None,
                "subscript",
            ),
            GenericTypeParamDecl => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                true,
                "",
                None,
                "",
            ),
            Getter => self.print_abstract_storage(n.first()?, as_prefix_context, "getter"),
            Setter => self.print_abstract_storage(n.first()?, as_prefix_context, "setter"),
            MaterializeForSet => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "materializeForSet")
            }
            WillSet => self.print_abstract_storage(n.first()?, as_prefix_context, "willset"),
            DidSet => self.print_abstract_storage(n.first()?, as_prefix_context, "didset"),
            ReadAccessor => self.print_abstract_storage(n.first()?, as_prefix_context, "read"),
            Read2Accessor => self.print_abstract_storage(n.first()?, as_prefix_context, "read2"),
            ModifyAccessor => self.print_abstract_storage(n.first()?, as_prefix_context, "modify"),
            Modify2Accessor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "modify2")
            }
            InitAccessor => self.print_abstract_storage(n.first()?, as_prefix_context, "init"),
            UnsafeMutableAddressor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "unsafeMutableAddressor")
            }
            UnsafeAddressor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "unsafeAddressor")
            }
            OwningMutableAddressor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "owningMutableAddressor")
            }
            OwningAddressor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "owningAddressor")
            }
            NativeOwningMutableAddressor => self.print_abstract_storage(
                n.first()?,
                as_prefix_context,
                "nativeOwningMutableAddressor",
            ),
            NativeOwningAddressor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "nativeOwningAddressor")
            }
            NativePinningMutableAddressor => self.print_abstract_storage(
                n.first()?,
                as_prefix_context,
                "nativePinningMutableAddressor",
            ),
            NativePinningAddressor => {
                self.print_abstract_storage(n.first()?, as_prefix_context, "nativePinningAddressor")
            }
            GlobalGetter => self.print_abstract_storage(n.first()?, as_prefix_context, "getter"),
            Allocator => {
                let name = if n.first().is_some_and(|c| c.kind == Class) {
                    "__allocating_init"
                } else {
                    "init"
                };
                self.print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::FunctionStyle,
                    false,
                    name,
                    None,
                    "",
                )
            }
            Constructor => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::FunctionStyle,
                false,
                "init",
                None,
                "",
            ),
            Destructor => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                false,
                "deinit",
                None,
                "",
            ),
            Deallocator => {
                let name = if n.first().is_some_and(|c| c.kind == Class) {
                    "__deallocating_deinit"
                } else {
                    "deinit"
                };
                self.print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::NoType,
                    false,
                    name,
                    None,
                    "",
                )
            }
            IsolatedDeallocator => {
                let name = if n.first().is_some_and(|c| c.kind == Class) {
                    "__isolated_deallocating_deinit"
                } else {
                    "deinit"
                };
                self.print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::NoType,
                    false,
                    name,
                    None,
                    "",
                )
            }
            IVarInitializer => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                false,
                "__ivar_initializer",
                None,
                "",
            ),
            IVarDestroyer => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                false,
                "__ivar_destroyer",
                None,
                "",
            ),
            Initializer => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                false,
                "variable initialization expression",
                None,
                "",
            ),
            PropertyWrapperBackingInitializer => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                false,
                "property wrapper backing initializer",
                None,
                "",
            ),
            PropertyWrapperInitFromProjectedValue => self.print_entity(
                n,
                as_prefix_context,
                TypePrinting::NoType,
                false,
                "property wrapper init from projected value",
                None,
                "",
            ),
            DefaultArgumentInitializer => {
                let idx = n.child(1).map(|c| c.get_index()).unwrap_or(0);
                self.print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::NoType,
                    false,
                    "default argument ",
                    Some(idx),
                    "",
                )
            }
            ExplicitClosure => {
                let idx = n
                    .child(1)
                    .map(|c| c.get_index())
                    .unwrap_or(0)
                    .saturating_add(1);
                self.print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::FunctionStyle,
                    false,
                    "closure #",
                    Some(idx),
                    "",
                )
            }
            ImplicitClosure => {
                let idx = n
                    .child(1)
                    .map(|c| c.get_index())
                    .unwrap_or(0)
                    .saturating_add(1);
                self.print_entity(
                    n,
                    as_prefix_context,
                    TypePrinting::FunctionStyle,
                    false,
                    "implicit closure #",
                    Some(idx),
                    "",
                )
            }
            Static => {
                self.s("static ");
                let c = n.first()?.clone();
                self.print(&c, as_prefix_context)
            }
            EnumCase => {
                self.s("enum case for ");
                self.print_child(n, 0);
                None
            }
            TypeMetadata => {
                self.print_simple_entity_prefix(n, as_prefix_context, "type metadata for ")
            }
            TypeMetadataAccessFunction => {
                self.print_simple_entity_prefix(n, as_prefix_context, "type metadata accessor for ")
            }
            TypeMetadataCompletionFunction => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "type metadata completion function for ",
            ),
            TypeMetadataInstantiationFunction => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "type metadata instantiation function for ",
            ),
            TypeMetadataInstantiationCache => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "type metadata instantiation cache for ",
            ),
            TypeMetadataSingletonInitializationCache => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "type metadata singleton initialization cache for ",
            ),
            TypeMetadataLazyCache => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "lazy cache variable for type metadata for ",
            ),
            TypeMetadataDemanglingCache => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "demangling cache variable for type metadata for ",
            ),
            FullTypeMetadata => {
                self.print_simple_entity_prefix(n, as_prefix_context, "full type metadata for ")
            }
            GenericTypeMetadataPattern => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "generic type metadata pattern for ",
            ),
            NominalTypeDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "nominal type descriptor for ",
            ),
            NominalTypeDescriptorRecord => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "nominal type descriptor runtime record for ",
            ),
            Metaclass => self.print_simple_entity_prefix(n, as_prefix_context, "metaclass for "),
            ClassMetadataBaseOffset => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "class metadata base offset for ",
            ),
            MethodLookupFunction => {
                self.print_simple_entity_prefix(n, as_prefix_context, "method lookup function for ")
            }
            ObjCMetadataUpdateFunction => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "ObjC metadata update function for ",
            ),
            ObjCResilientClassStub => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "ObjC resilient class stub for ",
            ),
            FullObjCResilientClassStub => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "full ObjC resilient class stub for ",
            ),
            ModuleDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "module descriptor ")
            }
            ExtensionDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "extension descriptor ")
            }
            AnonymousDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "anonymous descriptor ")
            }
            ProtocolDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "protocol descriptor for ")
            }
            ProtocolDescriptorRecord => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol descriptor runtime record for ",
            ),
            ProtocolRequirementsBaseDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol requirements base descriptor for ",
            ),
            ProtocolSelfConformanceDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol self-conformance descriptor for ",
            ),
            ProtocolSelfConformanceWitnessTable => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol self-conformance witness table for ",
            ),
            ProtocolSelfConformanceWitness => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol self-conformance witness for ",
            ),
            ProtocolConformanceDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol conformance descriptor for ",
            ),
            ProtocolConformanceDescriptorRecord => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol conformance descriptor runtime record for ",
            ),
            ProtocolWitnessTable => {
                self.print_simple_entity_prefix(n, as_prefix_context, "protocol witness table for ")
            }
            ProtocolWitnessTableAccessor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol witness table accessor for ",
            ),
            ProtocolWitnessTablePattern => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "protocol witness table pattern for ",
            ),
            GenericProtocolWitnessTable => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "generic protocol witness table for ",
            ),
            GenericProtocolWitnessTableInstantiationFunction => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "instantiation function for generic protocol witness table for ",
            ),
            ResilientProtocolWitnessTable => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "resilient protocol witness table for ",
            ),
            LazyProtocolWitnessTableAccessor => {
                self.s("lazy protocol witness table accessor for type ");
                self.print_child(n, 0);
                self.s(" and conformance ");
                self.print_child(n, 1);
                None
            }
            LazyProtocolWitnessTableCacheVariable => {
                self.s("lazy protocol witness table cache variable for type ");
                self.print_child(n, 0);
                self.s(" and conformance ");
                self.print_child(n, 1);
                None
            }
            AssociatedTypeMetadataAccessor => {
                self.s("associated type metadata accessor for ");
                self.print_child(n, 1);
                self.s(" in ");
                self.print_child(n, 0);
                None
            }
            AssociatedTypeWitnessTableAccessor => {
                self.s("associated type witness table accessor for ");
                self.print_child(n, 1);
                self.s(" : ");
                self.print_child(n, 2);
                self.s(" in ");
                self.print_child(n, 0);
                None
            }
            BaseWitnessTableAccessor => {
                self.s("base witness table accessor for ");
                self.print_child(n, 1);
                self.s(" in ");
                self.print_child(n, 0);
                None
            }
            AssociatedTypeDescriptor => {
                self.s("associated type descriptor for ");
                self.print_child(n, 0);
                None
            }
            AssociatedConformanceDescriptor => {
                self.s("associated conformance descriptor for ");
                self.print_child(n, 0);
                self.s(".");
                self.print_child(n, 1);
                self.s(": ");
                self.print_child(n, 2);
                None
            }
            DefaultAssociatedConformanceAccessor => {
                self.s("default associated conformance accessor for ");
                self.print_child(n, 0);
                self.s(".");
                self.print_child(n, 1);
                self.s(": ");
                self.print_child(n, 2);
                None
            }
            DefaultAssociatedTypeMetadataAccessor => {
                self.s("default associated type metadata accessor for ");
                self.print_child(n, 0);
                None
            }
            BaseConformanceDescriptor => {
                self.s("base conformance descriptor for ");
                self.print_child(n, 0);
                self.s(": ");
                self.print_child(n, 1);
                None
            }
            ProtocolWitness => {
                self.s("protocol witness for ");
                self.print_child(n, 1);
                self.s(" in conformance ");
                self.print_child(n, 0);
                None
            }
            ValueWitness => {
                let t = n.get_text().to_string();
                self.s(&t);
                self.s(" value witness for ");
                self.print_child(n, 0);
                None
            }
            ValueWitnessTable => {
                self.print_simple_entity_prefix(n, as_prefix_context, "value witness table for ")
            }
            FieldOffset => {
                let d = n
                    .first()
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                self.s(&d);
                self.s(" field offset for ");
                self.print_child(n, 1);
                None
            }
            PropertyDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "property descriptor for ")
            }
            MethodDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "method descriptor for ")
            }
            DispatchThunk => {
                self.print_simple_entity_prefix(n, as_prefix_context, "dispatch thunk of ")
            }
            CurryThunk => self.print_simple_entity_prefix(n, as_prefix_context, "curry thunk of "),
            ObjCAttribute => {
                self.s("@objc ");
                None
            }
            NonObjCAttribute => {
                self.s("@nonobjc ");
                None
            }
            DynamicAttribute => {
                self.s("dynamic ");
                None
            }
            DirectMethodReferenceAttribute => {
                self.s("super ");
                None
            }
            VTableAttribute => {
                self.s("override ");
                None
            }
            VTableThunk => {
                self.s("vtable thunk for ");
                self.print_child(n, 0);
                self.s(" dispatching to ");
                self.print_child(n, 1);
                None
            }
            ReabstractionThunk | ReabstractionThunkHelper | ReabstractionThunkHelperWithSelf => {
                self.s("reabstraction thunk ");
                if n.kind != ReabstractionThunk {
                    self.s("helper ");
                }
                let mut idx = 0;
                if n.first()
                    .is_some_and(|c| c.kind == DependentGenericSignature)
                {
                    self.print_child(n, 0);
                    self.s(" ");
                    idx = 1;
                }
                if n.kind == ReabstractionThunkHelperWithSelf {
                    self.s("with self ");
                    self.print_child(n, idx);
                    self.s(" ");
                    idx += 1;
                }
                self.s("from ");
                self.print_child(n, idx);
                self.s(" to ");
                self.print_child(n, idx + 1);
                None
            }
            PartialApplyForwarder => {
                self.s("partial apply forwarder");
                if !n.is_empty() {
                    self.s(" for ");
                    self.print_children(n, "");
                }
                None
            }
            PartialApplyObjCForwarder => {
                self.s("partial apply ObjC forwarder");
                if !n.is_empty() {
                    self.s(" for ");
                    self.print_children(n, "");
                }
                None
            }
            MergedFunction => {
                self.s("merged ");
                None
            }
            OutlinedCopy => self.print_outlined(n, "outlined copy of "),
            OutlinedConsume => self.print_outlined(n, "outlined consume of "),
            OutlinedRetain => self.print_outlined(n, "outlined retain of "),
            OutlinedRelease => self.print_outlined(n, "outlined release of "),
            OutlinedInitializeWithTake => self.print_outlined(n, "outlined init with take of "),
            OutlinedInitializeWithCopy => self.print_outlined(n, "outlined init with copy of "),
            OutlinedAssignWithTake => self.print_outlined(n, "outlined assign with take of "),
            OutlinedAssignWithCopy => self.print_outlined(n, "outlined assign with copy of "),
            OutlinedDestroy => self.print_outlined(n, "outlined destroy of "),
            OutlinedInitializeWithTakeNoValueWitness => {
                self.print_outlined(n, "outlined init with take of ")
            }
            OutlinedInitializeWithCopyNoValueWitness => {
                self.print_outlined(n, "outlined init with copy of ")
            }
            OutlinedAssignWithTakeNoValueWitness => {
                self.print_outlined(n, "outlined assign with take of ")
            }
            OutlinedAssignWithCopyNoValueWitness => {
                self.print_outlined(n, "outlined assign with copy of ")
            }
            OutlinedDestroyNoValueWitness => self.print_outlined(n, "outlined destroy of "),
            OutlinedEnumTagStore => self.print_outlined(n, "outlined enum tag store of "),
            OutlinedEnumProjectDataForLoad => {
                self.print_outlined(n, "outlined enum project data for load of ")
            }
            OutlinedEnumGetTag => self.print_outlined(n, "outlined enum get tag of "),
            OutlinedVariable => {
                self.s("outlined variable #");
                self.s(&n.get_index().to_string());
                self.s(" of ");
                None
            }
            OutlinedReadOnlyObject => {
                self.s("outlined read-only object #");
                self.s(&n.get_index().to_string());
                self.s(" of ");
                None
            }
            AsyncFunctionPointer => {
                self.s("async function pointer to ");
                None
            }
            AsyncAwaitResumePartialFunction => {
                self.s("(");
                self.print_child(n, 0);
                self.s(")");
                self.s(" await resume partial function for ");
                None
            }
            AsyncSuspendResumePartialFunction => {
                self.s("(");
                self.print_child(n, 0);
                self.s(")");
                self.s(" suspend resume partial function for ");
                None
            }
            CoroFunctionPointer => {
                self.s("coro function pointer to ");
                None
            }
            DefaultOverride => {
                self.s("default override of ");
                None
            }
            DynamicallyReplaceableFunctionImpl => {
                self.s("dynamically replaceable thunk for ");
                None
            }
            DynamicallyReplaceableFunctionKey => {
                self.s("dynamically replaceable key for ");
                None
            }
            DynamicallyReplaceableFunctionVar => {
                self.s("dynamically replaceable variable for ");
                None
            }
            BackDeploymentThunk => {
                self.s("back deployment thunk for ");
                None
            }
            BackDeploymentFallback => {
                self.s("back deployment fallback for ");
                None
            }
            HasSymbolQuery => {
                self.s("#_hasSymbol query for ");
                None
            }
            DistributedThunk => {
                self.s("distributed thunk ");
                None
            }
            DistributedAccessor => {
                self.s("distributed accessor for ");
                None
            }
            AccessibleFunctionRecord => {
                self.s("accessible function runtime record for ");
                None
            }
            KeyPathGetterThunkHelper
            | KeyPathSetterThunkHelper
            | KeyPathUnappliedMethodThunkHelper
            | KeyPathAppliedMethodThunkHelper => {
                self.s(match n.kind {
                    KeyPathGetterThunkHelper => "key path getter for ",
                    KeyPathSetterThunkHelper => "key path setter for ",
                    KeyPathUnappliedMethodThunkHelper => "key path unapplied method for ",
                    _ => "key path applied method for ",
                });
                self.print_child(n, 0);
                self.s(" : ");
                for c in n.children.iter().skip(1) {
                    if c.kind == IsSerialized {
                        self.s(", ");
                    }
                    self.print(c, false);
                }
                None
            }
            KeyPathEqualsThunkHelper | KeyPathHashThunkHelper => {
                self.s("key path index ");
                self.s(if n.kind == KeyPathEqualsThunkHelper {
                    "equality"
                } else {
                    "hash"
                });
                self.s(" operator for ");
                let mut last_type_index = n.len();
                let mut has_sig = false;
                let mut has_serialized = false;
                for (i, c) in n.children.iter().enumerate() {
                    match c.kind {
                        DependentGenericSignature => {
                            has_sig = true;
                            last_type_index = last_type_index.min(i);
                        }
                        IsSerialized => {
                            has_serialized = true;
                            last_type_index = last_type_index.min(i);
                        }
                        _ => {}
                    }
                }
                let _ = (has_sig, has_serialized);
                self.s("(");
                let mut first = true;
                for c in n.children.iter().take(last_type_index) {
                    if !first {
                        self.s(", ");
                    }
                    first = false;
                    self.print(c, false);
                }
                self.s(")");
                for c in n.children.iter().skip(last_type_index) {
                    if c.kind == IsSerialized {
                        self.s(", ");
                    }
                    self.print(c, false);
                }
                None
            }
            GlobalVariableOnceFunction | GlobalVariableOnceToken => {
                self.s(if n.kind == GlobalVariableOnceToken {
                    "one-time initialization token for "
                } else {
                    "one-time initialization function for "
                });
                self.print_child(n, 1);
                None
            }
            GlobalVariableOnceDeclList => {
                if n.len() == 1 {
                    self.print_child(n, 0);
                } else {
                    self.s("(");
                    self.print_children(n, ", ");
                    self.s(")");
                }
                None
            }
            CanonicalSpecializedGenericMetaclass => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "specialized generic metaclass for ",
            ),
            CanonicalSpecializedGenericTypeMetadataAccessFunction => self
                .print_simple_entity_prefix(
                    n,
                    as_prefix_context,
                    "canonical specialized generic type metadata accessor for ",
                ),
            MetadataInstantiationCache => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "metadata instantiation cache for ",
            ),
            NoncanonicalSpecializedGenericTypeMetadata => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "noncanonical specialized generic type metadata for ",
            ),
            NoncanonicalSpecializedGenericTypeMetadataCache => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "cache variable for noncanonical specialized generic type metadata for ",
            ),
            CanonicalPrespecializedGenericTypeCachingOnceToken => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "flag for loading of canonical specialized generic type metadata for ",
            ),
            OpaqueTypeDescriptor => {
                self.print_simple_entity_prefix(n, as_prefix_context, "opaque type descriptor for ")
            }
            OpaqueTypeDescriptorAccessor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "opaque type descriptor accessor for ",
            ),
            OpaqueTypeDescriptorAccessorImpl => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "opaque type descriptor accessor impl for ",
            ),
            OpaqueTypeDescriptorAccessorKey => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "opaque type descriptor accessor key for ",
            ),
            OpaqueTypeDescriptorAccessorVar => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "opaque type descriptor accessor var for ",
            ),
            OpaqueTypeDescriptorRecord => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "opaque type descriptor runtime record for ",
            ),
            OpaqueReturnTypeOf => {
                self.s("<<opaque return type of ");
                self.print_children(n, "");
                self.s(">>");
                None
            }
            OpaqueReturnType => {
                self.s("some");
                None
            }
            OpaqueType => {
                self.print_child(n, 0);
                self.s(".");
                self.print_child(n, 1);
                None
            }
            OpaqueReturnTypeIndex => None,
            GenericSpecialization | GenericSpecializationInResilienceDomain => {
                self.print_spec_prefix(n, "generic specialization", "");
                None
            }
            GenericSpecializationPrespecialized => {
                self.print_spec_prefix(n, "generic pre-specialization", "");
                None
            }
            GenericSpecializationNotReAbstracted => {
                self.print_spec_prefix(n, "generic not re-abstracted specialization", "");
                None
            }
            InlinedGenericFunction => {
                self.print_spec_prefix(n, "inlined generic function", "");
                None
            }
            GenericPartialSpecialization => {
                self.print_spec_prefix(n, "generic partial specialization", "Signature = ");
                None
            }
            GenericPartialSpecializationNotReAbstracted => {
                self.print_spec_prefix(
                    n,
                    "generic not-reabstracted partial specialization",
                    "Signature = ",
                );
                None
            }
            GenericSpecializationParam => {
                self.print_child(n, 0);
                for c in n.children.iter().skip(1) {
                    self.s(" with ");
                    self.print(c, false);
                }
                None
            }
            FunctionSignatureSpecialization => {
                self.print_spec_prefix(n, "function signature specialization", "");
                None
            }
            FunctionSignatureSpecializationReturn | FunctionSignatureSpecializationParam => {
                if n.kind == FunctionSignatureSpecializationReturn {
                    self.s("Return = ");
                }
                let mut idx = self.print_func_sig_spec_param(n, 0);
                while idx < n.len() {
                    self.s(" and ");
                    idx = self.print_func_sig_spec_param(n, idx);
                }
                None
            }
            FunctionSignatureSpecializationParamPayload => {
                let t = n.get_text().to_string();
                match super::demangle_swift_full(&t) {
                    Some(d) => self.s(&d),
                    None => self.s(&t),
                }
                None
            }
            FunctionSignatureSpecializationParamKind => {
                let v = n.get_index();
                match v {
                    BOX_TO_VALUE => self.s("Value Promoted from Box"),
                    BOX_TO_STACK => self.s("Stack Promoted from Box"),
                    INOUT_TO_OUT => self.s("InOut Converted to Out"),
                    CP_FUNCTION => self.s("Constant Propagated Function"),
                    CP_GLOBAL => self.s("Constant Propagated Global"),
                    CP_INTEGER => self.s("Constant Propagated Integer"),
                    CP_FLOAT => self.s("Constant Propagated Float"),
                    CP_STRING => self.s("Constant Propagated String"),
                    CP_KEYPATH => self.s("Constant Propagated KeyPath"),
                    CLOSURE_PROP => self.s("Closure Propagated"),
                    _ => {
                        let mut printed = false;
                        for (bit, name) in [
                            (
                                EXISTENTIAL_TO_GENERIC,
                                "Existential To Protocol Constrained Generic",
                            ),
                            (DEAD, "Dead"),
                            (OWNED_TO_GUARANTEED, "Owned To Guaranteed"),
                            (GUARANTEED_TO_OWNED, "Guaranteed To Owned"),
                            (SROA, "Exploded"),
                        ] {
                            if v & bit != 0 {
                                if printed {
                                    self.s(" and ");
                                }
                                printed = true;
                                self.s(name);
                            }
                        }
                    }
                }
                None
            }
            SpecializationPassID => {
                self.s(&n.get_index().to_string());
                None
            }
            IsSerialized => {
                self.s("serialized");
                None
            }
            AsyncRemoved => {
                self.s("async");
                None
            }
            DroppedArgument => None,
            MetatypeRepresentation => {
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }

            ReflectionMetadataBuiltinDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "reflection metadata builtin descriptor ",
            ),
            ReflectionMetadataFieldDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "reflection metadata field descriptor ",
            ),
            ReflectionMetadataAssocTypeDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "reflection metadata associated type descriptor ",
            ),
            ReflectionMetadataSuperclassDescriptor => self.print_simple_entity_prefix(
                n,
                as_prefix_context,
                "reflection metadata superclass descriptor ",
            ),
            ImplFunctionType => {
                self.print_impl_function_type(n);
                None
            }
            ImplParameter | ImplResult | ImplYield => {
                let implicit_leading = n.child_of_kind(ImplParameterImplicitLeading).is_some();
                let shown: Vec<NodeRef> = n
                    .children
                    .iter()
                    .filter(|c| {
                        c.kind != ImplParameterImplicitLeading
                            && (c.kind != ImplParameterIsolated || !implicit_leading)
                    })
                    .cloned()
                    .collect();
                for (i, c) in shown.iter().enumerate() {
                    if i > 0 {
                        self.s(" ");
                    }
                    self.print(c, false);
                }
                None
            }
            ImplErrorResult => {
                self.s("@error ");
                self.print_children(n, " ");
                None
            }
            ImplEscaping => {
                self.s("@escaping");
                None
            }
            ImplErasedIsolation => {
                self.s("@isolated(any)");
                None
            }
            ImplSendingResult => {
                self.s("sending");
                None
            }
            ImplParameterImplicitLeading => None,
            ImplParameterIsolated
            | ImplConvention
            | ImplFunctionAttribute
            | ImplParameterResultDifferentiability
            | ImplParameterSending
            | ImplFunctionConventionName => {
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }
            ImplCoroutineKind => {
                self.s("@");
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }
            ImplDifferentiabilityKind => {
                self.s("@differentiable");
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }
            ImplFunctionConvention => {
                self.s("@convention(");
                let name = n
                    .first()
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                self.s(&name);
                if let Some(ct) = n.child(1) {
                    self.s(", mangledCType: \"");
                    let t = ct.get_text().to_string();
                    self.s(&t);
                    self.s("\"");
                }
                self.s(")");
                None
            }
            ImplPatternSubstitutions | ImplInvocationSubstitutions => self.fail(),
            Type => {
                let c = n.first()?.clone();
                self.print(&c, as_prefix_context)
            }
            TypeMangling => {
                self.print_child(n, 0);
                None
            }
            FunctionType
            | NoEscapeFunctionType
            | EscapingAutoClosureType
            | AutoClosureType
            | ThinFunctionType
            | CFunctionPointer
            | ObjCBlock
            | EscapingObjCBlock
            | UncurriedFunctionType => {
                self.print_function_type(None, n);
                None
            }
            ArgumentTuple => {
                self.print_children(n, "");
                None
            }
            ReturnType => {
                self.s(" -> ");
                self.print_children(n, "");
                None
            }
            Tuple => {
                self.s("(");
                self.print_children(n, ", ");
                self.s(")");
                None
            }
            TupleElement => {
                if let Some(label) = n.child_of_kind(TupleElementName) {
                    let t = label.get_text().to_string();
                    self.s(&t);
                    self.s(": ");
                }
                let ty = n.child_of_kind(Type)?.clone();
                self.print(&ty, false);
                if n.child_of_kind(VariadicMarker).is_some() {
                    self.s("...");
                }
                None
            }
            VariadicMarker | EmptyList | FirstElementMarker => self.fail(),
            LabelList => None,
            TypeList => {
                self.print_children(n, "");
                None
            }
            BoundGenericClass
            | BoundGenericStructure
            | BoundGenericEnum
            | BoundGenericProtocol
            | BoundGenericOtherNominalType
            | BoundGenericTypeAlias => {
                self.print_bound_generic(n);
                None
            }
            Metatype => {
                if n.len() == 2 {
                    self.print_child(n, 0);
                    self.s(" ");
                }
                let ty = n.last()?.clone();
                self.print_with_parens(&ty);
                let existential = ty.first().is_some_and(|c| {
                    matches!(
                        c.kind,
                        ProtocolList | ProtocolListWithAnyObject | ProtocolListWithClass
                    )
                });
                self.s(if existential { ".Protocol" } else { ".Type" });
                None
            }
            ExistentialMetatype => {
                if n.len() == 2 {
                    self.print_child(n, 0);
                    self.s(" ");
                }
                let ty = n.last()?.clone();
                self.print(&ty, false);
                self.s(".Type");
                None
            }
            ProtocolList => {
                let list = n.first()?.clone();
                if list.is_empty() {
                    self.s("Any");
                } else {
                    self.print_children(&list, " & ");
                }
                None
            }
            ProtocolListWithClass => {
                if n.len() < 2 {
                    return self.fail();
                }
                let list = n.first()?.first()?.clone();
                self.print_child(n, 1);
                if !list.is_empty() {
                    self.s(" & ");
                    self.print_children(&list, " & ");
                }
                None
            }
            ProtocolListWithAnyObject => {
                let list = n.first()?.first()?.clone();
                self.print_children(&list, " & ");
                if !list.is_empty() {
                    self.s(" & ");
                }
                self.s("Swift.AnyObject");
                None
            }
            ConstrainedExistential => {
                self.s("any ");
                self.print_child(n, 0);
                self.s("<");
                self.print_child(n, 1);
                self.s(">");
                None
            }
            ConstrainedExistentialRequirementList => {
                self.print_children(n, ", ");
                None
            }
            ConstrainedExistentialSelf => {
                self.s("Self");
                None
            }
            DynamicSelf => {
                self.s("Self");
                None
            }
            InOut => {
                self.s("inout ");
                self.print_child(n, 0);
                None
            }
            Shared => {
                self.s("__shared ");
                self.print_child(n, 0);
                None
            }
            Owned => {
                self.s("__owned ");
                self.print_child(n, 0);
                None
            }
            Isolated => {
                self.s("isolated ");
                self.print_child(n, 0);
                None
            }
            Sending => {
                self.s("sending ");
                self.print_child(n, 0);
                None
            }
            NoDerivative => {
                self.s("@noDerivative ");
                self.print_child(n, 0);
                None
            }
            CompileTimeLiteral => {
                self.s("_const ");
                self.print_child(n, 0);
                None
            }
            Weak => {
                self.s("weak ");
                self.print_child(n, 0);
                None
            }
            Unowned => {
                self.s("unowned ");
                self.print_child(n, 0);
                None
            }
            Unmanaged => {
                self.s("unowned(unsafe) ");
                self.print_child(n, 0);
                None
            }
            ErrorType => {
                self.s("<ERROR TYPE>");
                None
            }
            BuiltinTypeName => {
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }
            BuiltinFixedArray => {
                self.s("Builtin.FixedArray<");
                self.print_child(n, 0);
                self.s(", ");
                self.print_child(n, 1);
                self.s(">");
                None
            }
            SugaredOptional => {
                let ty = n.first()?.clone();
                self.print_with_parens(&ty);
                self.s("?");
                None
            }
            SugaredArray => {
                self.s("[");
                self.print_child(n, 0);
                self.s("]");
                None
            }
            SugaredDictionary => {
                self.s("[");
                self.print_child(n, 0);
                self.s(" : ");
                self.print_child(n, 1);
                self.s("]");
                None
            }
            SugaredParen => {
                self.s("(");
                self.print_child(n, 0);
                self.s(")");
                None
            }
            ThrowsAnnotation
            | TypedThrowsAnnotation
            | AsyncAnnotation
            | ConcurrentFunctionType
            | DifferentiableFunctionType
            | GlobalActorFunctionType
            | IsolatedAnyFunctionType
            | NonIsolatedCallerFunctionType
            | SendingResultFunctionType
            | ClangType => self.fail(),
            SILBoxType => {
                self.s("@box ");
                self.print_child(n, 0);
                None
            }
            SILBoxTypeWithLayout => {
                if n.len() == 3 {
                    self.print_child(n, 1);
                    self.s(" ");
                }
                self.print_child(n, 0);
                if n.len() == 3 {
                    self.s(" <");
                    if let Some(args) = n.child(2) {
                        self.print_children(args, ", ");
                    }
                    self.s(">");
                }
                None
            }
            SILBoxLayout => {
                self.s("{");
                for (i, c) in n.children.iter().enumerate() {
                    if i > 0 {
                        self.s(",");
                    }
                    self.s(" ");
                    self.print(c, false);
                }
                self.s(" }");
                None
            }
            SILBoxMutableField => {
                self.s("var ");
                self.print_child(n, 0);
                None
            }
            SILBoxImmutableField => {
                self.s("let ");
                self.print_child(n, 0);
                None
            }
            Pack => {
                self.s("Pack{");
                self.print_children(n, ", ");
                self.s("}");
                None
            }
            PackExpansion => {
                self.s("repeat ");
                self.print_child(n, 0);
                None
            }
            PackElement => {
                self.s("/* level: ");
                self.print_child(n, 1);
                self.s(" */ each ");
                self.print_child(n, 0);
                None
            }
            PackElementLevel => {
                self.s(&n.get_index().to_string());
                None
            }
            DependentGenericType => {
                self.print_child(n, 0);
                let t = n.child(1)?.clone();
                if needs_space_before_type(&t) {
                    self.s(" ");
                }
                self.print(&t, false);
                None
            }
            DependentGenericSignature => {
                self.s("<");
                let packs: Vec<(u64, u64)> = n
                    .children
                    .iter()
                    .filter(|c| c.kind == DependentGenericParamPackMarker)
                    .filter_map(|c| {
                        c.first().and_then(|t| t.first()).map(|p| {
                            (
                                p.child(0).map(|d| d.get_index()).unwrap_or(0),
                                p.child(1).map(|i| i.get_index()).unwrap_or(0),
                            )
                        })
                    })
                    .collect();
                let mut depth = 0usize;
                let num = n.len();
                while depth < num
                    && n.child(depth)
                        .is_some_and(|c| c.kind == DependentGenericParamCount)
                {
                    if depth != 0 {
                        self.s("><");
                    }
                    let count = n.child(depth).map(|c| c.get_index()).unwrap_or(0);
                    for index in 0..count {
                        if index != 0 {
                            self.s(", ");
                        }
                        if index >= 128 {
                            self.s("...");
                            break;
                        }
                        // `swift-demangle` compares the marker's (depth, index)
                        // swapped, so only a parameter with depth == index
                        // shows as a pack; matched here for identical output.
                        if packs.contains(&(index, depth as u64)) {
                            self.s("each ");
                        }
                        let name = super::parse::generic_parameter_name(depth as u64, index);
                        self.s(&name);
                    }
                    depth += 1;
                }
                let reqs: Vec<NodeRef> = n
                    .children
                    .iter()
                    .skip(depth)
                    .filter(|c| c.kind != DependentGenericParamPackMarker)
                    .cloned()
                    .collect();
                if !reqs.is_empty() {
                    self.s(" where ");
                    for (i, r) in reqs.iter().enumerate() {
                        if i > 0 {
                            self.s(", ");
                        }
                        self.print(r, false);
                    }
                }
                self.s(">");
                None
            }
            DependentGenericParamCount => self.fail(),
            DependentGenericParamType => {
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }
            DependentGenericParamPackMarker => {
                self.s("each ");
                self.print_child(n, 0);
                None
            }
            DependentGenericParamValueMarker => {
                self.s("let ");
                self.print_child(n, 0);
                self.s(": ");
                self.print_child(n, 1);
                None
            }
            DependentGenericConformanceRequirement => {
                self.print_child(n, 0);
                self.s(": ");
                self.print_child(n, 1);
                None
            }
            DependentGenericSameTypeRequirement => {
                self.print_child(n, 0);
                self.s(" == ");
                self.print_child(n, 1);
                None
            }
            DependentGenericSameShapeRequirement => {
                self.print_child(n, 0);
                self.s(".shape == ");
                self.print_child(n, 1);
                self.s(".shape");
                None
            }
            DependentGenericInverseConformanceRequirement => {
                self.print_child(n, 0);
                self.s(": ~");
                let name = match n.child(1).map(|c| c.get_index()) {
                    Some(0) => "Swift.Copyable",
                    Some(1) => "Swift.Escapable",
                    _ => return self.fail(),
                };
                self.s(name);
                None
            }
            DependentGenericLayoutRequirement => {
                self.print_child(n, 0);
                self.s(": ");
                let c = n
                    .child(1)
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                let name = match c.as_str() {
                    "U" => "_UnknownLayout",
                    "R" => "_RefCountedObject",
                    "N" => "_NativeRefCountedObject",
                    "C" => "AnyObject",
                    "D" => "_NativeClass",
                    "T" => "_Trivial",
                    "E" | "e" => "_Trivial",
                    "M" | "m" => "_TrivialAtMost",
                    "B" => "_BridgeObject",
                    "S" => "_TrivialStride",
                    _ => return self.fail(),
                };
                self.s(name);
                if n.len() > 2 {
                    self.s("(");
                    self.print_child(n, 2);
                    if n.len() > 3 {
                        self.s(", ");
                        self.print_child(n, 3);
                    }
                    self.s(")");
                }
                None
            }
            DependentMemberType => {
                self.print_child(n, 0);
                self.s(".");
                self.print_child(n, 1);
                None
            }
            DependentAssociatedTypeRef => {
                if n.len() > 1 {
                    self.print_child(n, 1);
                    self.s(".");
                }
                self.print_child(n, 0);
                None
            }
            AssocTypePath => {
                self.print_children(n, ".");
                None
            }
            AssociatedTypeRef => {
                self.print_child(n, 0);
                self.s(".");
                let t = n
                    .child(1)
                    .map(|c| c.get_text().to_string())
                    .unwrap_or_default();
                self.s(&t);
                None
            }
            DependentAssociatedConformance => {
                self.print_child(n, 0);
                self.s(": ");
                self.print_child(n, 1);
                None
            }
            ProtocolConformance => {
                if n.len() == 4 {
                    // Property behavior conformance.
                    self.s("property behavior storage of ");
                    self.print_child(n, 2);
                    self.s(" in ");
                    self.print_child(n, 0);
                    self.s(" : ");
                    self.print_child(n, 1);
                } else {
                    self.print_child(n, 0);
                    self.s(" : ");
                    self.print_child(n, 1);
                    if n.len() == 3 {
                        self.s(" in ");
                        self.print_child(n, 2);
                    }
                }
                None
            }
            ProtocolConformanceRefInTypeModule => {
                self.print_child(n, 0);
                self.s(" in type's module");
                None
            }
            ProtocolConformanceRefInProtocolModule => {
                self.print_child(n, 0);
                self.s(" in protocol's module");
                None
            }
            ProtocolConformanceRefInOtherModule => {
                self.print_child(n, 0);
                self.s(" in retroactive module ");
                self.print_child(n, 1);
                None
            }
            ConcreteProtocolConformance => {
                self.s("concrete protocol conformance ");
                self.print_child(n, 0);
                self.s(" to ");
                self.print_child(n, 1);
                if let Some(list) = n.child(2) {
                    if !list.is_empty() {
                        self.s(" with conditional requirements: ");
                        let list = list.clone();
                        self.print(&list, false);
                    }
                }
                None
            }
            PackProtocolConformance => {
                self.s("pack protocol conformance ");
                self.print_child(n, 0);
                None
            }
            AnyProtocolConformanceList => {
                if !n.is_empty() {
                    self.s("(");
                    self.print_children(n, ", ");
                    self.s(")");
                }
                None
            }
            DependentProtocolConformanceRoot => {
                self.s("dependent protocol conformance ");
                self.print_child(n, 0);
                self.s(" to ");
                self.print_child(n, 1);
                None
            }
            DependentProtocolConformanceInherited => {
                self.s("dependent inherited protocol conformance ");
                self.print_child(n, 0);
                self.s(" to ");
                self.print_child(n, 1);
                None
            }
            DependentProtocolConformanceAssociated => {
                self.s("dependent associated protocol conformance ");
                self.print_child(n, 1);
                self.s(" from ");
                self.print_child(n, 0);
                None
            }
            DependentProtocolConformanceOpaque => {
                self.s("dependent opaque protocol conformance ");
                self.print_child(n, 1);
                self.s(" from ");
                self.print_child(n, 0);
                None
            }
            RetroactiveConformance => {
                if n.len() != 2 {
                    return self.fail();
                }
                self.s("retroactive @ ");
                self.print_child(n, 0);
                self.s(" ");
                self.print_child(n, 1);
                None
            }
            Index | UnknownIndex | Number => {
                self.s(&n.get_index().to_string());
                None
            }
            Integer => {
                self.s(&n.get_index().to_string());
                None
            }
            NegativeInteger => {
                self.s("-");
                self.s(&n.get_index().to_string());
                None
            }
            Directness => {
                let t = n.get_text().to_string();
                self.s(&t);
                None
            }
            _ => self.fail(),
        }
    }

    fn print_impl_function_type(&mut self, n: &Node) {
        #[derive(PartialEq, PartialOrd)]
        enum State {
            Attrs,
            Inputs,
            Results,
        }
        let mut state = State::Attrs;
        let mut pattern: Option<NodeRef> = None;
        let mut invocation: Option<NodeRef> = None;
        let mut sending = false;
        for c in &n.children {
            match c.kind {
                Kind::ImplParameter => {
                    if state == State::Inputs {
                        self.s(", ");
                    }
                    if state == State::Attrs {
                        if let Some(p) = &pattern {
                            self.s("@substituted ");
                            self.print_child(p, 0);
                            self.s(" ");
                        }
                        self.s("(");
                        state = State::Inputs;
                    }
                    self.print(c, false);
                }
                Kind::ImplResult | Kind::ImplYield | Kind::ImplErrorResult => {
                    if state == State::Results {
                        self.s(", ");
                    }
                    if state == State::Attrs {
                        if let Some(p) = &pattern {
                            self.s("@substituted ");
                            self.print_child(p, 0);
                            self.s(" ");
                        }
                        self.s("(");
                        state = State::Inputs;
                    }
                    if state == State::Inputs {
                        self.s(") -> (");
                        state = State::Results;
                    }
                    self.print(c, false);
                }
                Kind::ImplPatternSubstitutions => pattern = Some(c.clone()),
                Kind::ImplInvocationSubstitutions => invocation = Some(c.clone()),
                Kind::ImplSendingResult => sending = true,
                _ => {
                    self.print(c, false);
                    self.s(" ");
                }
            }
        }
        if state == State::Attrs {
            if let Some(p) = &pattern {
                self.s("@substituted ");
                self.print_child(p, 0);
                self.s(" ");
            }
            self.s("(");
            state = State::Inputs;
        }
        if state == State::Inputs {
            self.s(") -> (");
        }
        if sending {
            self.s("sending ");
        }
        self.s(")");
        if let Some(p) = &pattern {
            self.s(" for <");
            if let Some(l) = p.child(1) {
                self.print_children(l, "");
            }
            self.s(">");
        }
        if let Some(inv) = &invocation {
            self.s(" for <");
            if let Some(l) = inv.first() {
                self.print_children(l, "");
            }
            self.s(">");
        }
    }

    fn print_outlined(&mut self, n: &Node, prefix: &str) -> Option<NodeRef> {
        self.s(prefix);
        self.print_child(n, 0);
        if matches!(
            n.kind,
            Kind::OutlinedCopy
                | Kind::OutlinedConsume
                | Kind::OutlinedRetain
                | Kind::OutlinedRelease
        ) {
            if let Some(sig) = n.child(1) {
                let sig = sig.clone();
                self.print(&sig, false);
            }
        }
        None
    }
}
