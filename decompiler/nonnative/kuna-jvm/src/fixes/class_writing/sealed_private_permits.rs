//! kuna fix `sealed_private_permits` (`kuna_fixes`): `ClassWriter.writeClassDefinition`
//! writes a sealed class's `PermittedSubclasses` attribute as a `permits`
//! clause. The header of a top-level class is outside its body, so a private
//! member class (or one nested in a private member class) cannot be named
//! there (JLS 6.6.1): javac rejects the clause and every subclass with it. A
//! source can only produce such a class by leaving the clause out, which
//! makes the permitted subclasses those of the same compilation unit
//! (JLS 8.1.6). With the fix the clause is left out for a top-level class
//! whose permitted subclasses are all written in its own file and include an
//! inaccessible one.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_PRIVATE;
use crate::decompiler_context::DecompilerContext;

/// Whether `node`'s `permits` clause, naming `permitted`, is left out.
pub fn omit_permits(node: ClassNode, permitted: &[String]) -> bool {
    if node.with(|d| d.type_) != ClassNodeType::Root {
        return false;
    }
    let roots = DecompilerContext::get_class_processor().get_map_root_classes();
    let mut private = false;
    for name in permitted {
        let Some(mut current) = roots.borrow().get(name).copied() else {
            return false;
        };
        loop {
            if current == node {
                break;
            }
            if current.with(|d| d.type_) != ClassNodeType::Member {
                return false;
            }
            private |= current.with(|d| d.access) & ACC_PRIVATE != 0;
            match current.with(|d| d.parent) {
                Some(parent) => current = parent,
                None => return false,
            }
        }
    }
    private
}
