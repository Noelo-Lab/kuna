//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ClassReference14Processor.java`.
#![allow(unused_variables, dead_code)]

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::exit_exprent::ExitExprent;
use crate::exprent::Exprent;
use crate::method_wrapper::MethodWrapper;
use crate::rt::{JHashMap, JHashSet};
use crate::unported;

pub struct ClassReference14Processor;

impl ClassReference14Processor {
    /// The `return Class.forName(var0);` body pattern of a synthetic `class$`.
    /// Exprents are class-scope arena objects: rebuild once the cached handle is no longer live.
    #[allow(non_snake_case)]
    fn BODY_EXPR() -> ExitExprent {
        unported!("ClassReference14Processor.BODY_EXPR")
    }

    /// The `throw new NoClassDefFoundError().initCause(var2);` handler pattern; cached like `BODY_EXPR`.
    #[allow(non_snake_case)]
    fn HANDLER_EXPR() -> ExitExprent {
        unported!("ClassReference14Processor.HANDLER_EXPR")
    }

    pub fn process_class_references(node: ClassNode) {
        unported!("ClassReference14Processor.processClassReferences")
    }

    fn process_class_rec(
        node: ClassNode,
        map_class_meths: &JHashMap<ClassWrapper, MethodWrapper>,
        set_found: &mut JHashSet<ClassWrapper>,
    ) {
        unported!("ClassReference14Processor.processClassRec")
    }

    fn map_class_methods(node: ClassNode, map: &mut JHashMap<ClassWrapper, MethodWrapper>) {
        unported!("ClassReference14Processor.mapClassMethods")
    }

    fn replace_invocations(exprent: Exprent, wrapper: ClassWrapper, meth: MethodWrapper) -> bool {
        unported!("ClassReference14Processor.replaceInvocations")
    }

    fn is_class14_invocation(
        exprent: Exprent,
        wrapper: ClassWrapper,
        meth: MethodWrapper,
    ) -> Option<String> {
        unported!("ClassReference14Processor.isClass14Invocation")
    }
}
