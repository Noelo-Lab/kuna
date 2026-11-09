//! Port of `org/jetbrains/java/decompiler/main/rels/NestedMemberAccess.java`.

#![allow(dead_code, unused_variables)]

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::invocation_exprent::InvocationExprent;
use crate::method_wrapper::MethodWrapper;
use crate::rt::{JHashMap, JavaEq, JavaHash};
use crate::unported;

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
        unported!("NestedMemberAccess.computeMethodType")
    }

    fn eliminate_static_access(&mut self, node: ClassNode) {
        unported!("NestedMemberAccess.eliminateStaticAccess")
    }

    fn replace_invocations(
        &mut self,
        caller: ClassNode,
        meth: MethodWrapper,
        exprent: Exprent,
    ) -> bool {
        unported!("NestedMemberAccess.replaceInvocations")
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
        unported!("NestedMemberAccess.replaceAccessExprent")
    }

    fn replace_function(invexpr: InvocationExprent, source: Exprent) -> Option<Exprent> {
        unported!("NestedMemberAccess.replaceFunction")
    }
}
