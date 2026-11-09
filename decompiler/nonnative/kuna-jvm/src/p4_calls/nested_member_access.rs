//! Port of `org/jetbrains/java/decompiler/main/rels/NestedMemberAccess.java`.

#![allow(clippy::collapsible_if)]

use std::collections::VecDeque;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_STATIC;
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::rt::{JHashMap, JHashSet, JavaEq, JavaHash};
use crate::var_exprent::VarExprent;
use crate::var_names_collector::VarNamesCollector;
use crate::var_version_pair::VarVersionPair;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum MethodAccess {
    Normal,
    FieldGet,
    FieldSet,
    Method,
    Function,
}

impl JavaHash for MethodAccess {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for MethodAccess {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// Replaces calls to synthetic accessor methods (`access$000`) with the field
/// or method access they wrap.
#[derive(Default)]
pub struct NestedMemberAccess {
    no_synth_flag: bool,
    map_method_type: JHashMap<MethodWrapper, MethodAccess>,
}

fn is_var_index(exprent: Option<Exprent>, index: i32) -> bool {
    exprent
        .and_then(|e| e.try_as_var())
        .is_some_and(|v| v.get_index() == index)
}

impl NestedMemberAccess {
    pub fn new() -> NestedMemberAccess {
        NestedMemberAccess::default()
    }

    pub fn propagate_member_access(&mut self, root: ClassNode) {
        if root.with(|d| d.nested.is_empty()) {
            return;
        }

        self.no_synth_flag =
            DecompilerContext::get_option(IFernflowerPreferences::SYNTHETIC_NOT_SET);

        self.compute_method_types(root);

        self.eliminate_static_access(root);
    }

    fn compute_method_types(&mut self, node: ClassNode) {
        if node.with(|d| d.type_) == ClassNodeType::Lambda {
            return;
        }

        let nested = node.with(|d| d.nested.to_vec());
        for nd in nested {
            self.compute_method_types(nd);
        }

        let methods = node.get_wrapper().unwrap().get_methods().borrow().to_vec();
        for method in methods {
            self.compute_method_type(node, method);
        }
    }

    fn compute_method_type(&mut self, node: ClassNode, method: MethodWrapper) {
        let mut type_ = MethodAccess::Normal;

        if method.with(|d| d.root.is_some()) {
            let graph = method.get_or_build_graph().expect("DirectGraph");

            let mt = method.with(|d| d.method_struct);
            if (self.no_synth_flag || mt.is_synthetic()) && mt.has_modifier(ACC_STATIC) {
                if graph.with(|d| d.nodes.len()) == 2 {
                    let first_exprents = graph.first().expect("DirectGraph.first").exprents();
                    let node_qualified_name = node.with(|d| d.class_struct).qualified_name();
                    if first_exprents.len() == 1 {
                        let exprent = first_exprents.get(0);

                        let mtdesc = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
                        let parcount = mtdesc.params.len() as i32;

                        let mut expr_core = exprent;

                        if let Some(exexpr) = exprent.try_as_exit() {
                            if exexpr.get_exit_type() == ExitType::Return {
                                if let Some(value) = exexpr.get_value() {
                                    expr_core = value;
                                }
                            }
                        }

                        match expr_core.get_type() {
                            ExprentType::Field => {
                                let fexpr = expr_core.as_field();
                                if (parcount == 1 && !fexpr.is_static())
                                    || (parcount == 0 && fexpr.is_static())
                                {
                                    if fexpr.get_classname() == node_qualified_name {
                                        if fexpr.is_static()
                                            || is_var_index(fexpr.get_instance(), 0)
                                        {
                                            type_ = MethodAccess::FieldGet;
                                        }
                                    }
                                }
                            }
                            ExprentType::Var => {
                                if parcount == 1 {
                                    if expr_core.as_var().get_index() != 0 {
                                        type_ = MethodAccess::FieldGet;
                                    }
                                }
                            }
                            ExprentType::Function => {
                                let function_exprent = expr_core.as_function();
                                if function_exprent.get_func_type().is_ppmm() {
                                    if function_exprent.get_lst_operands().get(0).is_field() {
                                        type_ = MethodAccess::Function;
                                    }
                                }
                            }
                            ExprentType::Invocation => {
                                type_ = MethodAccess::Method;
                            }
                            ExprentType::Assignment => {
                                let asexpr = expr_core.as_assignment();
                                if asexpr.get_left().is_field() && asexpr.get_right().is_var() {
                                    let fexpras = asexpr.get_left().as_field();
                                    if (parcount == 2 && !fexpras.is_static())
                                        || (parcount == 1 && fexpras.is_static())
                                    {
                                        if fexpras.get_classname() == node_qualified_name {
                                            if fexpras.is_static()
                                                || is_var_index(fexpras.get_instance(), 0)
                                            {
                                                if asexpr.get_right().as_var().get_index()
                                                    == parcount - 1
                                                {
                                                    type_ = MethodAccess::FieldSet;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }

                        if type_ == MethodAccess::Method {
                            type_ = MethodAccess::Normal;

                            let invexpr = expr_core.as_invocation();

                            let is_static = invexpr.is_static();
                            let lst_parameters = invexpr.get_lst_parameters();
                            if (is_static && lst_parameters.len() as i32 == parcount)
                                || (!is_static
                                    && is_var_index(invexpr.get_instance(), 0)
                                    && lst_parameters.len() as i32 == parcount - 1)
                            {
                                let mut equalpars = true;

                                let mut index = if is_static { 0 } else { 1 };
                                let mut i = 0;
                                while i < invexpr.get_lst_parameters().len() {
                                    let parexpr = invexpr.get_lst_parameters().get(i);
                                    if !is_var_index(Some(parexpr), index) {
                                        equalpars = false;
                                        break;
                                    }
                                    index +=
                                        mtdesc.params[i + if is_static { 0 } else { 1 }].stack_size;
                                    i += 1;
                                }

                                if equalpars {
                                    type_ = MethodAccess::Method;
                                }
                            }
                        }
                    } else if first_exprents.len() == 2 {
                        let exprent_first = first_exprents.get(0);
                        let exprent_second = first_exprents.get(1);

                        if exprent_first.is_assignment() && exprent_second.is_exit() {
                            let mtdesc = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
                            let parcount = mtdesc.params.len() as i32;

                            let asexpr = exprent_first.as_assignment();
                            if asexpr.get_left().is_field() && asexpr.get_right().is_var() {
                                let fexpras = asexpr.get_left().as_field();
                                if (parcount == 2 && !fexpras.is_static())
                                    || (parcount == 1 && fexpras.is_static())
                                {
                                    if fexpras.get_classname() == node_qualified_name {
                                        if fexpras.is_static()
                                            || is_var_index(fexpras.get_instance(), 0)
                                        {
                                            if asexpr.get_right().as_var().get_index()
                                                == parcount - 1
                                            {
                                                let exexpr = exprent_second.as_exit();
                                                if exexpr.get_exit_type() == ExitType::Return {
                                                    if let Some(value) = exexpr.get_value() {
                                                        if value.is_var()
                                                            && asexpr
                                                                .get_right()
                                                                .as_var()
                                                                .get_index()
                                                                == parcount - 1
                                                        {
                                                            type_ = MethodAccess::FieldSet;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if type_ != MethodAccess::Normal {
            self.map_method_type.put(method, type_);
        } else {
            self.map_method_type.remove(&method);
        }
    }

    fn eliminate_static_access(&mut self, node: ClassNode) {
        if node.with(|d| d.type_) == ClassNodeType::Lambda {
            return;
        }

        let methods = node.get_wrapper().unwrap().get_methods().borrow().to_vec();
        for meth in methods {
            if meth.with(|d| d.root.is_some()) {
                let mut replaced = false;

                let graph = meth.get_or_build_graph().expect("DirectGraph");

                let mut set_visited: JHashSet<DirectNode> = JHashSet::new();
                let mut stack: VecDeque<DirectNode> = VecDeque::new();
                stack.push_back(graph.first().expect("DirectGraph.first"));

                while let Some(nd) = stack.pop_front() {
                    if set_visited.contains(&nd) {
                        continue;
                    }
                    set_visited.add(nd);

                    let mut i = 0;
                    while i < nd.exprents().len() {
                        let exprent = nd.exprents().get(i);

                        replaced |= self.replace_invocations(node, meth, exprent);

                        if let Some(invexpr) = exprent.try_as_invocation() {
                            let ret = self.replace_access_exprent(node, meth, invexpr);

                            if let Some(ret) = ret {
                                nd.exprents().set(i, ret);
                                replaced = true;
                            }
                        }
                        i += 1;
                    }

                    for suc in nd.get_successors(DirectEdgeType::Regular).iter() {
                        stack.push_back(suc.get_destination());
                    }
                }

                if replaced {
                    self.compute_method_type(node, meth);
                }
            }
        }

        let nested = node.with(|d| d.nested.to_vec());
        for child in nested {
            self.eliminate_static_access(child);
        }
    }

    fn replace_invocations(
        &mut self,
        caller: ClassNode,
        meth: MethodWrapper,
        exprent: Exprent,
    ) -> bool {
        let mut res = false;

        for expr in exprent.get_all_exprents() {
            res |= self.replace_invocations(caller, meth, expr);
        }

        loop {
            let mut found = false;

            for expr in exprent.get_all_exprents() {
                if let Some(invexpr) = expr.try_as_invocation() {
                    let newexpr = self.replace_access_exprent(caller, meth, invexpr);
                    if let Some(newexpr) = newexpr {
                        exprent.replace_exprent(expr, newexpr);
                        found = true;
                        res = true;
                        break;
                    }
                }
            }

            if !found {
                break;
            }
        }

        res
    }

    fn same_tree(caller: ClassNode, callee: ClassNode) -> bool {
        let caller_name = caller.with(|d| d.class_struct).qualified_name();
        let callee_name = callee.with(|d| d.class_struct).qualified_name();
        if caller_name == callee_name {
            return false;
        }

        let mut caller = caller;
        while let Some(parent) = caller.with(|d| d.parent) {
            caller = parent;
        }

        let mut callee = callee;
        while let Some(parent) = callee.with(|d| d.parent) {
            callee = parent;
        }

        caller == callee
    }

    fn replace_access_exprent(
        &mut self,
        caller: ClassNode,
        methdest: MethodWrapper,
        invexpr: InvocationExprent,
    ) -> Option<Exprent> {
        let node = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get_cloned(&invexpr.get_classname());

        let mut methsource: Option<MethodWrapper> = None;
        if let Some(node) = node {
            if let Some(wrapper) = node.get_wrapper() {
                methsource = wrapper
                    .get_method_wrapper(&invexpr.get_name(), &invexpr.get_string_descriptor());
            }
        }

        let methsource = match methsource {
            Some(m) if self.map_method_type.contains_key(&m) => m,
            _ => return None,
        };
        let node = node.unwrap();

        let source_struct = methsource.with(|d| d.method_struct);
        let dest_struct = methdest.with(|d| d.method_struct);
        if node.with(|d| d.class_struct).qualified_name()
            == caller.with(|d| d.class_struct).qualified_name()
            && source_struct.get_name() == dest_struct.get_name()
            && source_struct.get_descriptor() == dest_struct.get_descriptor()
        {
            return None;
        }

        let type_ = *self.map_method_type.get(&methsource).unwrap();

        if !Self::same_tree(caller, node) {
            return None;
        }

        let graph = methsource.get_or_build_graph().expect("DirectGraph");
        let mut source = graph.first().expect("DirectGraph.first").exprents().get(0);

        let mut retexprent: Option<Exprent> = None;

        match type_ {
            MethodAccess::FieldGet => {
                let exsource = source.as_exit();
                if let Some(var) = exsource.get_value().and_then(|v| v.try_as_var()) {
                    let varname = methsource
                        .with(|d| d.varproc)
                        .get_var_name(VarVersionPair::from_var(var));

                    let (dest_varproc, dest_counter) = methdest.with(|d| (d.varproc, d.counter));
                    let contained = varname
                        .as_ref()
                        .is_some_and(|n| methdest.with(|d| d.set_outer_var_names.contains(n)));
                    if !contained {
                        let vnc = VarNamesCollector::new();
                        if let Some(varname) = &varname {
                            vnc.add_name(varname.clone());
                        }

                        dest_varproc.refresh_var_names(vnc);
                        if let Some(varname) = &varname {
                            methdest.with_mut(|d| d.set_outer_var_names.add(varname.clone()));
                        }
                    }

                    let index =
                        dest_counter.get_counter_and_increment(CounterContainer::VAR_COUNTER);
                    let ret = VarExprent::new(index, Some(var.get_var_type()), Some(dest_varproc));
                    if let Some(varname) = varname {
                        dest_varproc.set_var_name(VarVersionPair::new(index, 0), varname);
                    }

                    retexprent = Some(ret.0);
                } else {
                    let ret = exsource
                        .get_value()
                        .expect("ExitExprent.value")
                        .copy()
                        .as_field();
                    if !ret.is_static() {
                        ret.replace_exprent(
                            ret.get_instance().expect("FieldExprent.instance"),
                            invexpr.get_lst_parameters().get(0),
                        );
                    }
                    retexprent = Some(ret.0);
                }
            }
            MethodAccess::FieldSet => {
                let ret = if let Some(extex) = source.try_as_exit() {
                    extex
                        .get_value()
                        .expect("ExitExprent.value")
                        .copy()
                        .as_assignment()
                } else {
                    source.copy().as_assignment()
                };
                let fexpr = ret.get_left().as_field();

                if fexpr.is_static() {
                    ret.replace_exprent(ret.get_right(), invexpr.get_lst_parameters().get(0));
                } else {
                    ret.replace_exprent(ret.get_right(), invexpr.get_lst_parameters().get(1));
                    fexpr.replace_exprent(
                        fexpr.get_instance().expect("FieldExprent.instance"),
                        invexpr.get_lst_parameters().get(0),
                    );
                }

                ret.get_left().set_bytecode(None);
                ret.get_right().set_bytecode(None);

                retexprent = Some(ret.0);
            }
            MethodAccess::Function => {
                retexprent = Self::replace_function(invexpr, source);
            }
            MethodAccess::Method => {
                if let Some(exit) = source.try_as_exit() {
                    source = exit.get_value().expect("ExitExprent.value");
                }

                let invret = source.copy().as_invocation();

                let mut index = 0;
                if !invret.is_static() {
                    invret.replace_exprent(
                        invret.get_instance().expect("InvocationExprent.instance"),
                        invexpr.get_lst_parameters().get(0),
                    );
                    index = 1;
                }

                let mut i = 0;
                while i < invret.get_lst_parameters().len() {
                    invret.replace_exprent(
                        invret.get_lst_parameters().get(i),
                        invexpr.get_lst_parameters().get(i + index),
                    );
                    i += 1;
                }

                retexprent = Some(invret.0);
            }
            MethodAccess::Normal => {}
        }

        if let Some(retexprent) = retexprent {
            retexprent.set_bytecode(None);
            retexprent.add_bytecode_offsets(invexpr.get_bytecode().as_ref());

            let mut hide = true;

            let (node_type, node_access) = node.with(|d| (d.type_, d.access));
            if node_type == ClassNodeType::Root || (node_access & ACC_STATIC) != 0 {
                let mt = methsource.with(|d| d.method_struct);
                if !mt.is_synthetic() {
                    hide = false;
                }
            }
            if hide {
                node.get_wrapper()
                    .unwrap()
                    .get_hidden_members()
                    .borrow_mut()
                    .add(InterpreterUtil::make_unique_key(
                        &invexpr.get_name(),
                        &invexpr.get_string_descriptor(),
                    ));
            }
        }

        retexprent
    }

    fn replace_function(invexpr: InvocationExprent, source: Exprent) -> Option<Exprent> {
        let function_exprent = source
            .as_exit()
            .get_value()
            .expect("ExitExprent.value")
            .copy()
            .as_function();

        let lst_parameters = invexpr.get_lst_parameters();

        let field_exprent = function_exprent.get_lst_operands().get(0).as_field();
        if field_exprent.is_static() {
            if !lst_parameters.is_empty() {
                return None;
            }
            return Some(function_exprent.0);
        }

        if lst_parameters.len() != 1 {
            return None;
        }

        field_exprent.replace_exprent(
            field_exprent.get_instance().expect("FieldExprent.instance"),
            lst_parameters.get(0),
        );
        Some(function_exprent.0)
    }
}
