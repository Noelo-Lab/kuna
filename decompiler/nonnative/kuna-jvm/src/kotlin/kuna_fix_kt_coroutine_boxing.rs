//! kuna fix `kt_coroutine_boxing` (spec 10.16): in Kotlin output, the boxing
//! kotlinc emits in coroutine code is written as the implicit boxing it
//! compiles.
//!
//! In suspend functions and suspend lambdas kotlinc boxes a primitive with
//! `kotlin.coroutines.jvm.internal.Boxing.boxInt` (`boxLong`, `boxBoolean`,
//! ..) instead of `Integer.valueOf`, so that the JIT can remove the
//! allocation; Kotlin source writes no call there (`return count` in a
//! suspend function returning `Int`). Vineflower does not know these
//! functions and writes `return boxInt(count)`, which kotlinc rejects (they
//! are `internal` to the standard library). With the fix each such call is
//! the boxing the plugin already writes implicitly, the wrapper's `valueOf`
//! for the same primitive type; kotlinc compiles the implicit boxing in
//! coroutine code back to the `Boxing` call.

use crate::exprent::Exprent;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_method::StructMethod;

const BOXING: &str = "kotlin/coroutines/jvm/internal/Boxing";

/// The `Boxing` functions and the wrapper class whose `valueOf` boxes the same
/// primitive.
const BOXES: [(&str, &str, &str); 8] = [
    ("boxBoolean", "Z", "java/lang/Boolean"),
    ("boxByte", "B", "java/lang/Byte"),
    ("boxShort", "S", "java/lang/Short"),
    ("boxInt", "I", "java/lang/Integer"),
    ("boxLong", "J", "java/lang/Long"),
    ("boxFloat", "F", "java/lang/Float"),
    ("boxDouble", "D", "java/lang/Double"),
    ("boxChar", "C", "java/lang/Character"),
];

/// Turns each coroutine boxing call under `root`, the body of `mt`, into its
/// `valueOf`; whether there was one. kotlinc boxes with `Boxing` only in
/// coroutine code: methods in the suspend calling convention and suspend
/// lambdas' `invokeSuspend`.
pub fn rewrite(root: RootStatement, mt: StructMethod) -> bool {
    let coroutine = mt.get_descriptor().ends_with("Lkotlin/coroutines/Continuation;)Ljava/lang/Object;")
        || mt.name_eq("invokeSuspend");
    if !coroutine {
        return false;
    }
    let mut exprents = Vec::new();
    collect(root.0, &mut exprents);
    let mut res = false;
    for e in exprents {
        let Some(inv) = e.try_as_invocation() else {
            continue;
        };
        if !inv.is_static() || inv.get_classname() != BOXING || inv.get_lst_parameters().len() != 1 {
            continue;
        }
        let name = inv.get_name();
        let descriptor = inv.get_string_descriptor();
        let Some((_, primitive, wrapper)) = BOXES
            .iter()
            .find(|(n, p, w)| *n == name && descriptor == format!("({p})L{w};"))
        else {
            continue;
        };
        let value_of = format!("({primitive})L{wrapper};");
        inv.set_classname(wrapper.to_string());
        inv.set_name("valueOf".to_string());
        inv.set_descriptor(MethodDescriptor::parse_descriptor(&value_of));
        inv.set_string_descriptor(value_of);
        if let Some(k) = e.try_as_k_invocation() {
            k.kd_mut(|d| {
                d.calling_data = None;
                d.property_method = None;
            });
        }
        res = true;
    }
    res
}

fn collect(stat: Statement, out: &mut Vec<Exprent>) {
    let top: Vec<Exprent> = match stat.get_exprents() {
        Some(exprents) => exprents.iter().collect(),
        None => {
            for st in stat.get_stats().borrow().iter() {
                collect(*st, out);
            }
            stat.get_stat_exprents()
        }
    };
    for e in top {
        out.extend(e.get_all_exprents_rec(true));
        out.push(e);
    }
}
