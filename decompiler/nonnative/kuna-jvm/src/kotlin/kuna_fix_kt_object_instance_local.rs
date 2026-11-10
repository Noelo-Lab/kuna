//! kuna fix `kt_object_instance_local` (spec 10.16): a Kotlin 1.3 object's
//! static initializer reads its instance from `INSTANCE`.
//!
//! kotlinc up to 1.3 starts an `object`'s `<clinit>` with `new X; dup;
//! invokespecial; astore_0; aload_0; putstatic INSTANCE` and reads local 0
//! wherever an initializer or `init` block of the object uses `this`;
//! kotlinc 2.0 stores straight into `INSTANCE` and reads it back with
//! `getstatic`. The local stopped `InitializerProcessor` at the first
//! initializer that used it, and printed as is it would be a second instance
//! of the object (`val var0 = Hpack(); INSTANCE = var0`). Here
//! `var0 = new X(); X.INSTANCE = var0` becomes `X.INSTANCE = new X()` and
//! every other read of the local reads `X.INSTANCE`, which holds the same
//! value from then on (the field is final and the local never written
//! again): the code kotlinc 2.0 emits for the same source.

use crate::exprent::{Exprent, ExprentType};
use crate::kotlin::kuna_fix_kt_static_initializer;
use crate::root_statement::RootStatement;
use crate::rt::JList;
use crate::statement::Statement;
use crate::statements::Statements;
use crate::struct_class::StructClass;

/// Rewrites the instance local of the static initializer `root` of `cl`.
pub fn rewrite(root: RootStatement, cl: StructClass) -> bool {
    if !kuna_fix_kt_static_initializer::is_object(cl) {
        return false;
    }
    let Some(first) = Statements::find_first_data(root.0) else {
        return false;
    };
    let Some(exprents) = first.get_exprents() else {
        return false;
    };
    if exprents.len() < 2 {
        return false;
    }
    let (Some(create), Some(store)) =
        (exprents.get(0).try_as_assignment(), exprents.get(1).try_as_assignment())
    else {
        return false;
    };
    let (Some(local), Some(new)) = (create.get_left().try_as_var(), create.get_right().try_as_new())
    else {
        return false;
    };
    let own_type = format!("L{};", cl.qualified_name());
    let instance = store.get_left().try_as_field().filter(|f| {
        f.is_static()
            && f.get_classname() == cl.qualified_name()
            && f.get_descriptor().descriptor_string == own_type
    });
    let stored = store.get_right().try_as_var().is_some_and(|v| {
        v.get_index() == local.get_index() && v.get_version() == local.get_version()
    });
    let Some(instance) = instance else {
        return false;
    };
    if !stored
        || new.get_new_type().value.as_deref() != Some(cl.qualified_name().as_str())
        || writes(root.0, local.get_index(), create.0)
    {
        return false;
    }
    store.set_right(create.get_right());
    exprents.remove_at(0);
    replace_reads(root.0, local.get_index(), instance.0);
    true
}

fn lists(stat: Statement) -> Vec<JList<Exprent>> {
    let mut lists: Vec<JList<Exprent>> = stat.get_exprents().into_iter().collect();
    if let Some(ifstat) = stat.try_as_if() {
        lists.push(ifstat.get_headexprent_list());
    } else if let Some(switch) = stat.try_as_switch() {
        lists.push(switch.get_headexprent_list());
    } else if let Some(sync) = stat.try_as_synchronized() {
        lists.push(sync.get_headexprent_list());
    } else if let Some(dostat) = stat.try_as_do() {
        lists.push(dostat.get_init_exprent_list());
        lists.push(dostat.get_condition_exprent_list());
        lists.push(dostat.get_inc_exprent_list());
    }
    lists
}

/// Whether local `index` is written anywhere under `stat` but by `except`.
fn writes(stat: Statement, index: i32, except: Exprent) -> bool {
    let mut stats = vec![stat];
    while let Some(st) = stats.pop() {
        for list in lists(st) {
            for e in list.iter() {
                for sub in e.get_all_exprents_rec_self(true, true) {
                    if sub == except {
                        continue;
                    }
                    let target = match sub.get_type() {
                        ExprentType::Assignment => Some(sub.as_assignment().get_left()),
                        ExprentType::Function => {
                            let f = sub.as_function();
                            f.get_func_type().is_ppmm().then(|| f.get_lst_operands().get(0))
                        }
                        _ => None,
                    };
                    if target.and_then(|t| t.try_as_var()).is_some_and(|v| v.get_index() == index)
                    {
                        return true;
                    }
                }
            }
        }
        stats.extend(st.get_stats().borrow().to_vec());
    }
    false
}

fn replace_reads(stat: Statement, index: i32, field: Exprent) {
    let mut stats = vec![stat];
    while let Some(st) = stats.pop() {
        for list in lists(st) {
            for i in 0..list.len() {
                let e = list.get(i);
                if e.try_as_var().is_some_and(|v| v.get_index() == index) {
                    list.set(i, field.copy());
                } else {
                    replace_in(e, index, field);
                }
            }
        }
        stats.extend(st.get_stats().borrow().to_vec());
    }
}

fn replace_in(expr: Exprent, index: i32, field: Exprent) {
    for sub in expr.get_all_exprents() {
        if sub.try_as_var().is_some_and(|v| v.get_index() == index) {
            expr.replace_exprent(sub, field.copy());
        } else {
            replace_in(sub, index, field);
        }
    }
}
