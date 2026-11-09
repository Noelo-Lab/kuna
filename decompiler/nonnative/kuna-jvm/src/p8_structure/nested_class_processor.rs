//! Port of `org/jetbrains/java/decompiler/main/rels/NestedClassProcessor.java`.

#![allow(dead_code, unused_variables)]

use std::collections::VecDeque;
use std::fmt;

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_FINAL, ACC_PRIVATE};
use crate::decompiler_context::DecompilerContext;
use crate::direct_graph::DirectGraph;
use crate::exprent::Exprent;
use crate::i_fernflower_logger::Severity;
use crate::invocation_exprent::InvocationExprent;
use crate::method_wrapper::MethodWrapper;
use crate::rt::jstring::java_string_hash;
use crate::rt::{JHashMap, JHashSet, JavaEq, JavaHash};
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_object, unported};

/// `AbstractCollection.toString()` over strings: `[a, b]`.
fn collection_to_string<'a>(items: impl Iterator<Item = &'a String>) -> String {
    let parts: Vec<&str> = items.map(String::as_str).collect();
    format!("[{}]", parts.join(", "))
}

/// Recovers the outer-variable captures of local, anonymous and lambda classes
/// and places local class definitions in their enclosing methods.
#[derive(Default)]
pub struct NestedClassProcessor;

impl NestedClassProcessor {
    pub fn new() -> NestedClassProcessor {
        NestedClassProcessor
    }

    pub fn process_class(&self, root: ClassNode, node: ClassNode) {
        unported!("NestedClassProcessor.processClass")
    }

    fn set_lambda_vars(parent: ClassNode, child: ClassNode) {
        unported!("NestedClassProcessor.setLambdaVars")
    }

    fn check_not_found_classes(root: ClassNode, node: ClassNode) {
        unported!("NestedClassProcessor.checkNotFoundClasses")
    }

    fn insert_nested_class(root: ClassNode, child: ClassNode) -> bool {
        let set_enclosing = child.with(|d| d.enclosing_classes.clone());

        let mut stack: VecDeque<ClassNode> = VecDeque::new();
        stack.push_back(root);

        while let Some(node) = stack.pop_front() {
            let qualified_name = node.with(|d| d.class_struct).qualified_name();
            if set_enclosing.borrow().contains(&qualified_name) {
                let nested = node.with(|d| d.nested.clone());
                nested.add(child);
                child.with_mut(|d| d.parent = Some(node));
                let mut sorted = nested.to_vec();
                sorted.sort_by(|a, b| a.compare_to(*b));
                nested.set_all(sorted);

                return true;
            }

            stack.extend(node.with(|d| d.nested.to_vec()));
        }

        false
    }

    fn compute_local_vars_and_definitions(node: ClassNode) {
        unported!("NestedClassProcessor.computeLocalVarsAndDefinitions")
    }

    fn find_enclosing_method(child: ClassNode) -> Option<MethodWrapper> {
        let (enclosing_method, parent, type_, enclosing_classes, class_struct) = child.with(|d| {
            (
                d.enclosing_method.clone(),
                d.parent,
                d.type_,
                d.enclosing_classes.clone(),
                d.class_struct,
            )
        });
        let enclosing_method = enclosing_method?;
        if let Some(parent) = parent {
            let from_parent = parent
                .get_wrapper()
                .unwrap()
                .get_methods()
                .borrow()
                .get_with_key(&enclosing_method);
            if from_parent.is_some() {
                return from_parent;
            }
        }
        let enclosing_class_names = enclosing_classes.borrow().to_vec();
        for enclosing_class_name in enclosing_class_names {
            let enclosing_class = DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .get_cloned(&enclosing_class_name);
            if let Some(enclosing_class) = enclosing_class {
                let from_enclosing = enclosing_class
                    .get_wrapper()
                    .unwrap()
                    .get_methods()
                    .borrow()
                    .get_with_key(&enclosing_method);
                if from_enclosing.is_some() {
                    return from_enclosing;
                }
            }
        }
        if type_ != ClassNodeType::Member {
            let qualified_name = class_struct.qualified_name();
            let enclosing = collection_to_string(enclosing_classes.borrow().iter());
            DecompilerContext::get_logger().write_message(
                &format!(
                    "Couldn't find enclosing method \"{enclosing_method}\" of {qualified_name} in {enclosing}"
                ),
                Severity::Warn,
            );
        }
        None
    }

    fn insert_local_vars(parent: ClassNode, child: ClassNode) {
        unported!("NestedClassProcessor.insertLocalVars")
    }

    fn get_mask_local_vars(wrapper: ClassWrapper) -> JHashMap<String, Vec<Option<VarFieldPair>>> {
        unported!("NestedClassProcessor.getMaskLocalVars")
    }

    fn get_enclosing_var_field(
        cl: StructClass,
        method: MethodWrapper,
        graph: DirectGraph,
        index: i32,
        outer_idx: i32,
        use_heuristic: bool,
    ) -> Option<(String, bool)> {
        unported!("NestedClassProcessor.getEnclosingVarField")
    }

    fn possibly_synthetic_field(fd: StructField) -> bool {
        fd.get_name().contains('$') && fd.has_modifier(ACC_FINAL) && fd.has_modifier(ACC_PRIVATE)
    }

    fn merge_list_signatures(
        first: &mut [Option<VarFieldPair>],
        second: &mut [Option<VarFieldPair>],
        both: bool,
    ) {
        let mut i: i32 = 1;

        while first.len() as i32 > i && second.len() as i32 > i {
            let fi = (first.len() as i32 - i) as usize;
            let si = (second.len() as i32 - i) as usize;
            let f_obj = first[fi];
            let s_obj = second[si];

            if !Self::is_equal(both, f_obj, s_obj) {
                first[fi] = None;
                if both {
                    second[si] = None;
                }
            } else if let Some(f_obj) = f_obj {
                Self::merge_var_pairs(f_obj, s_obj.unwrap());
            }

            i += 1;
        }

        let mut j: i32 = 1;
        while j <= first.len() as i32 - i {
            first[j as usize] = None;
            j += 1;
        }

        if both {
            let mut j: i32 = 1;
            while j <= second.len() as i32 - i {
                second[j as usize] = None;
                j += 1;
            }
        }

        if first.is_empty() {
            if !second.is_empty() && both {
                second[0] = None;
            }
        } else if second.is_empty() {
            first[0] = None;
        } else {
            let f_obj = first[0];
            let s_obj = second[0];

            if !Self::is_equal(both, f_obj, s_obj) {
                first[0] = None;
                if both {
                    second[0] = None;
                }
            } else if let Some(f_obj) = f_obj {
                Self::merge_var_pairs(f_obj, s_obj.unwrap());
            }
        }
    }

    /// The `fObj.varPair = sObj.varPair` / `sObj.varPair = fObj.varPair` step of `mergeListSignatures`.
    fn merge_var_pairs(f_obj: VarFieldPair, s_obj: VarFieldPair) {
        let f_pair = f_obj.with(|d| d.var_pair);
        if f_pair.var == -1 {
            let s_pair = s_obj.with(|d| d.var_pair);
            f_obj.with_mut(|d| d.var_pair = s_pair);
        } else {
            s_obj.with_mut(|d| d.var_pair = f_pair);
        }
    }

    fn is_equal(both: bool, f_obj: Option<VarFieldPair>, s_obj: Option<VarFieldPair>) -> bool {
        let (f_obj, s_obj) = match (f_obj, s_obj) {
            (Some(f), Some(s)) => (f, s),
            (f, s) => return f == s,
        };

        let mut eq = true;
        let (f_assigned, f_key) = f_obj.with(|d| (d.assigned, d.field_key.clone()));
        let (s_assigned, s_key) = s_obj.with(|d| (d.assigned, d.field_key.clone()));
        if !f_assigned || f_key.is_empty() {
            f_obj.with_mut(|d| {
                d.field_key = s_key;
                d.assigned = s_assigned;
            });
        } else if !s_assigned || s_key.is_empty() {
            if both {
                s_obj.with_mut(|d| {
                    d.field_key = f_key;
                    d.assigned = f_assigned;
                });
            }
        } else {
            eq = f_key == s_key;
        }
        eq
    }

    fn set_local_class_definition(method: MethodWrapper, node: ClassNode) {
        unported!("NestedClassProcessor.setLocalClassDefinition")
    }

    fn find_first_block(stat: Statement, set_stats: &JHashSet<Statement>) -> Option<Statement> {
        unported!("NestedClassProcessor.findFirstBlock")
    }

    fn get_def_statement(
        stat: Statement,
        class_type: &VarType,
        set_stats: &mut JHashSet<Statement>,
    ) -> Option<Statement> {
        unported!("NestedClassProcessor.getDefStatement")
    }

    fn search_for_class(exprent: Exprent, class_type: &VarType) -> bool {
        unported!("NestedClassProcessor.searchForClass")
    }

    /// Java: containsType(InvocationExprent, VarType)
    fn contains_type(haystack: Option<InvocationExprent>, needle: &VarType) -> bool {
        unported!("NestedClassProcessor.containsType")
    }

    /// Java: containsType(VarType, VarType)
    fn contains_type_var_type(haystack: Option<&VarType>, needle: Option<&VarType>) -> bool {
        unported!("NestedClassProcessor.containsType")
    }

    /// `processExprent` returns `None` to remove the exprent, or its replacement.
    fn iterate_exprents(graph: DirectGraph, iter: &mut ExprentIteratorWithReplace<'_>) {
        unported!("NestedClassProcessor.iterateExprents")
    }
}

/// `ExprentIteratorWithReplace.processExprent`.
type ExprentIteratorWithReplace<'a> = dyn FnMut(Exprent) -> Option<Exprent> + 'a;

struct VarFieldPairData {
    field_key: String,
    var_pair: VarVersionPair,
    assigned: bool,
}

arena_object!(VarFieldPair => VarFieldPairData, Class);

impl VarFieldPair {
    fn new(field: &str, var_pair: VarVersionPair, assigned: bool) -> VarFieldPair {
        VarFieldPair::alloc(VarFieldPairData {
            field_key: field.to_string(),
            var_pair,
            assigned,
        })
    }

    fn equals(self, o: VarFieldPair) -> bool {
        if o == self {
            return true;
        }

        let (a_key, a_pair, a_assigned) =
            self.with(|d| (d.field_key.clone(), d.var_pair, d.assigned));
        let (b_key, b_pair, b_assigned) = o.with(|d| (d.field_key.clone(), d.var_pair, d.assigned));
        a_key == b_key && a_pair == b_pair && a_assigned == b_assigned
    }
}

impl JavaEq for VarFieldPair {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(*other)
    }
}

impl JavaHash for VarFieldPair {
    fn java_hash(&self) -> i32 {
        self.with(|d| {
            java_string_hash(&d.field_key)
                .wrapping_add(d.var_pair.java_hash())
                .wrapping_add(d.assigned.java_hash())
        })
    }
}

impl fmt::Display for VarFieldPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (field_key, var_pair, assigned) =
            self.with(|d| (d.field_key.clone(), d.var_pair, d.assigned));
        let link = if assigned {
            " assigned to "
        } else {
            " stored in variable "
        };
        write!(f, "{var_pair}{link}{field_key}")
    }
}
