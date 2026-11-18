//! kuna fix `var_merge_declared_type` (`kuna_fixes`): `VarDefinitionHelper.mergeVars`
//! merges two variables of one slot whose types are equal in its lattice, which
//! compares erasures. Two locals the LocalVariableTable declares with one erased
//! type and different generic types (`Box<String> box` in one `static {}` block and `Box<Integer>
//! box` in the next, or two sibling `{ }` blocks) then become one variable
//! declared with the first one's type, and the second one's code does not
//! compile against it (`box.add(1)` on a `Box<String>`). Its lattice also
//! puts `int` above `char`, `short` and `byte`: a `char c` and a later `int
//! start` of one slot become one variable typed by the first one's entry,
//! `char`, and the `int` store is narrowed (`c = (char)i;`, wrong for values
//! past 65535; Kotlin `start = i.toChar()`). With the fix a merge of two
//! variables the LocalVariableTable declares with different types (erased or
//! generic) is refused: they are two variables, as in the source.

use crate::exprent::Exprent;
use crate::statement::Statement;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// Whether `from` and `to` are stored by two LocalVariableTable entries whose
/// types, erased or generic (`LocalVariableTypeTable`), differ. The method is
/// searched only when the entries of `from`'s slot have a generic type or
/// more than one erased type.
pub fn declared_apart(
    root: Statement,
    varproc: VarProcessor,
    from: VarVersionPair,
    to: VarVersionPair,
) -> bool {
    let candidates = varproc
        .get_var_original_index(from.var)
        .and_then(|slot| varproc.get_candidates(slot))
        .unwrap_or_default();
    let generic_slot = candidates.iter().any(|l| l.signature.is_some());
    let mixed_slot = candidates
        .iter()
        .any(|l| l.descriptor != candidates[0].descriptor);
    if !generic_slot && !mixed_slot {
        return false;
    }
    let (mut from_lvt, mut to_lvt) = (None, None);
    find_lvts(root, from, to, &mut from_lvt, &mut to_lvt);
    match (from_lvt, to_lvt) {
        (Some(a), Some(b)) => a.descriptor != b.descriptor || a.signature != b.signature,
        _ => false,
    }
}

pub(crate) fn find_lvts(
    stat: Statement,
    from: VarVersionPair,
    to: VarVersionPair,
    from_lvt: &mut Option<LocalVariable>,
    to_lvt: &mut Option<LocalVariable>,
) {
    let mut top: Vec<Exprent> = stat.get_var_definitions().iter().collect();
    match stat.get_exprents() {
        Some(exprents) => top.extend(exprents.iter()),
        None => {
            top.extend(stat.get_stat_exprents());
            for st in stat.get_stats().borrow().iter() {
                find_lvts(*st, from, to, from_lvt, to_lvt);
            }
        }
    }
    for e in top {
        let mut all = e.get_all_exprents_rec(true);
        all.push(e);
        for v in all.into_iter().filter_map(|x| x.try_as_var()) {
            let pair = VarVersionPair::from_var(v);
            let slot = if pair == from {
                &mut *from_lvt
            } else if pair == to {
                &mut *to_lvt
            } else {
                continue;
            };
            if slot.is_none() {
                *slot = v.get_lvt();
            }
        }
    }
}
