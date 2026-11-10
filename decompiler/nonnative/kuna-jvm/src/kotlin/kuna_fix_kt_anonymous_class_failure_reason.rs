//! kuna fix `kt_anonymous_class_failure_reason`: the failure of an
//! anonymous class the Kotlin writer cannot write says what the class is.
//!
//! `KotlinWriter.writeClassDefinition` throws `Anonymous class does not have
//! Class Kotlin metadata` for every anonymous class without Class metadata
//! that no other path writes (the fixes `kt_synthetic_anonymous_object`,
//! `kt_reference_anonymous_super_call` and `kt_property_reference` write
//! the ones that have a Kotlin form). What remains is a structural limit, and
//! the message names the class and the reason, so a reader of the failure
//! comment knows what the method is:
//!
//! - the continuation of a suspend function (`ContinuationImpl`) or a suspend
//!   lambda (`SuspendLambda`): the method is a coroutine state machine, which
//!   the plugin prints as compiled and which has no Kotlin form (the
//!   function's `instanceof`/cast of its own continuation class cannot be
//!   written with an `object` expression, and Kotlin source never handles
//!   the continuation); with the fix `kt_synthetic_object_super_call`, which
//!   writes a continuation as an `object` expression, one it cannot write
//!   gets the last reason below;
//! - a callable-reference class of a shape `stringifyReference` does not
//!   read, such as a reference to a local function that captures variables;
//! - an object created in another class than the one whose method declares
//!   it (kotlinc copies an inline function's object creation into the
//!   `$$inlined$` classes of its callers): the object's body was decompiled
//!   against its declaring method's variables, which do not exist where it is
//!   created;
//! - otherwise, an object whose superclass constructor call or captured
//!   values an `object` expression cannot express at this `new`.

use crate::classes_processor::ClassNode;
use crate::code_constants::INIT_NAME;
use crate::decompiler_context::DecompilerContext;
use crate::kuna_fixes::{self, KunaFix};

const CONTINUATIONS: [&str; 2] = [
    "kotlin/coroutines/jvm/internal/ContinuationImpl",
    "kotlin/coroutines/jvm/internal/RestrictedContinuationImpl",
];

const SUSPEND_LAMBDAS: [&str; 2] = [
    "kotlin/coroutines/jvm/internal/SuspendLambda",
    "kotlin/coroutines/jvm/internal/RestrictedSuspendLambda",
];

fn dotted(name: &str) -> String {
    name.replace('/', ".")
}

/// The text of the exception thrown for the anonymous class `node`.
pub fn message(node: ClassNode) -> String {
    let (cl, parent, enclosing_method) =
        node.with(|d| (d.class_struct, d.parent, d.enclosing_method.clone()));
    let name = dotted(&cl.qualified_name());
    let superclass = cl
        .super_class()
        .map(|s| s.get_string().to_string())
        .unwrap_or_default();
    let reason = if CONTINUATIONS.contains(&superclass.as_str())
        && !kuna_fixes::enabled(KunaFix::KtSyntheticObjectSuperCall)
    {
        format!(
            "{name} is the continuation of a suspend function: the method is the function's \
             coroutine state machine, which has no Kotlin form"
        )
    } else if SUSPEND_LAMBDAS.contains(&superclass.as_str()) {
        format!("{name} is a suspend lambda: its coroutine state machine has no Kotlin form")
    } else if superclass.starts_with("kotlin/jvm/internal/") && superclass.contains("Reference") {
        if captures(node) {
            format!("{name} is a reference to a local function that captures variables")
        } else {
            format!("{name} is a callable reference of a shape kuna does not read")
        }
    } else {
        let site = DecompilerContext::get_current_class();
        let outside = match (site, parent) {
            (Some(site), Some(_)) => {
                let site_root = DecompilerContext::get_class_processor()
                    .get_map_root_classes()
                    .borrow()
                    .get(&site.qualified_name())
                    .copied()
                    .map(top_level);
                site_root.is_some_and(|r| r != top_level(node))
            }
            _ => false,
        };
        if outside {
            let declaring = parent
                .map(|p| dotted(&p.with(|d| d.class_struct.qualified_name())))
                .unwrap_or_default();
            let method = enclosing_method
                .as_deref()
                .and_then(|key| key.split(' ').next())
                .unwrap_or("?")
                .to_string();
            let site = dotted(&site.map(|s| s.qualified_name()).unwrap_or_default());
            format!(
                "{name} is created in {site}, outside the method that declares it \
                 ({declaring}.{method}): the variables it captures cannot be named there"
            )
        } else {
            format!(
                "{name} has no Kotlin form here: an object expression cannot pass its \
                 superclass constructor arguments or the values it captures"
            )
        }
    };
    format!("Anonymous class does not have Class Kotlin metadata: {reason}")
}

/// The top-level class `node` is nested in (itself when it is one).
fn top_level(node: ClassNode) -> ClassNode {
    let mut node = node;
    while let Some(parent) = node.with(|d| d.parent) {
        node = parent;
    }
    node
}

/// Whether a constructor of the reference class `node` takes captured
/// variables: anything but nothing (unbound) or one `Object` (bound receiver).
fn captures(node: ClassNode) -> bool {
    let cl = node.with(|d| d.class_struct);
    cl.get_methods().as_slice().iter().any(|m| {
        m.get_name() == INIT_NAME
            && !matches!(&*m.get_descriptor(), "()V" | "(Ljava/lang/Object;)V")
    })
}
