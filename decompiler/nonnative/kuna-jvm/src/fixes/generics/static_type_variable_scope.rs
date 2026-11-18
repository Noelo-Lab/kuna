//! kuna fix `static_type_variable_scope` (`kuna_fixes`): a class's type
//! variables are not in scope in its static methods and initializers (JLS
//! 6.3, 8.1.3). `Exprent.getNamedGenerics` counts them in scope everywhere
//! inside the class, so a static method's own type variable `V` that a call
//! leaves unresolved is taken for the class's `V` and written in a cast,
//! `getUninterruptibly((Future<V>) future)`, which javac rejects in a static
//! context. With the fix the walk out from the code stops counting class type
//! variables once it has passed a static method.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::CodeConstants;
use crate::method_wrapper::MethodWrapper;

/// Whether `method` is a static method of its class's source. A synthetic
/// method, such as the static implementation method of a lambda, says
/// nothing about the context the lambda is written in.
pub fn is_static_method(node: Option<ClassNode>, method: MethodWrapper) -> bool {
    if node.is_some_and(|n| n.with(|d| d.type_) == ClassNodeType::Lambda) {
        return false;
    }
    let mt = method.with(|d| d.method_struct);
    mt.has_modifier(CodeConstants::ACC_STATIC) && !mt.has_modifier(CodeConstants::ACC_SYNTHETIC)
}
