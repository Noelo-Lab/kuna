//! Fix family `failure_comments`: the failure comment a method or class gets,
//! and its note. In this file, `kt_anonymous_class_failure_reason` names the
//! class and the reason in the failure of an anonymous class the Kotlin writer
//! cannot write.

use crate::classes_processor::ClassNode;
use crate::code_constants::INIT_NAME;
use crate::decompiler_context::DecompilerContext;
use crate::fixes::util::top_level;

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
    let reason = if SUSPEND_LAMBDAS.contains(&superclass.as_str()) {
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

/// Whether a constructor of the reference class `node` takes captured
/// variables: anything but nothing (unbound) or one `Object` (bound receiver).
fn captures(node: ClassNode) -> bool {
    let cl = node.with(|d| d.class_struct);
    cl.get_methods().as_slice().iter().any(|m| {
        m.get_name() == INIT_NAME
            && !matches!(&*m.get_descriptor(), "()V" | "(Ljava/lang/Object;)V")
    })
}
