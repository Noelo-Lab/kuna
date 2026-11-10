//! kuna fix `compact_record_constructor` (`kuna_fixes`), from upstream
//! Vineflower 1506b2f4 ("Fix incorrect compact record constructors"):
//! `RecordHelper.isCompactCanonicalConstructor` looks only at the last
//! expression of the method (`this.c = c` from a named parameter), and a
//! compact constructor then loses every field assignment. A canonical
//! constructor that computes some fields (`this.node =
//! Objects.requireNonNull(node); this.reason = reason;`) lost those
//! computations, and the record printed with no constructor at all. With the
//! fix a canonical constructor is compact only when its one exit that does
//! not throw ends with `this.c = c` for every component, in order, from
//! increasing parameter slots (upstream's test), besides Vineflower's own; and
//! only assignments to fields of `this` are removed (a static field keeps its
//! store).

use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::root_statement::RootStatement;
use crate::struct_record_component::StructRecordComponent;

/// Whether the one non-throwing exit of `root` ends with an assignment of
/// each component's field from a parameter, in component order.
pub fn assigns_components_last(root: RootStatement, components: &[StructRecordComponent]) -> bool {
    if components.is_empty() {
        return false;
    }
    let mut found = false;
    for edge in root.get_dummy_exit().get_all_predecessor_edges() {
        let exit = edge.get_source();
        if !exit.is_basic_block() {
            continue;
        }
        let Some(exprents) = exit.get_exprents().filter(|e| !e.is_empty()) else {
            continue;
        };
        let last = exprents.get(exprents.len() - 1);
        if last
            .try_as_exit()
            .is_some_and(|e| e.get_exit_type() == ExitType::Throw)
        {
            continue;
        }
        if found || exprents.len() < components.len() {
            return false;
        }
        let offset = exprents.len() - components.len();
        let mut last_index = 0;
        for (i, component) in components.iter().enumerate() {
            let Some(index) = component_store(exprents.get(offset + i), component) else {
                return false;
            };
            if index <= last_index {
                return false;
            }
            last_index = index;
        }
        found = true;
    }
    found
}

/// The variable index of `this.<component> = <variable>`.
fn component_store(e: Exprent, component: &StructRecordComponent) -> Option<i32> {
    let assignment = e.try_as_assignment()?;
    let field = assignment.get_left().try_as_field()?;
    if field.get_name() != component.get_name() || !is_this(field.get_instance()) {
        return None;
    }
    Some(assignment.get_right().try_as_var()?.get_index())
}

/// Whether `e` is an assignment to a field of `this`.
pub fn is_this_field_store(e: Exprent) -> bool {
    e.try_as_assignment()
        .and_then(|a| a.get_left().try_as_field())
        .is_some_and(|f| is_this(f.get_instance()))
}

fn is_this(instance: Option<Exprent>) -> bool {
    instance
        .and_then(|i| i.try_as_var())
        .is_some_and(|v| v.get_index() == 0)
}
