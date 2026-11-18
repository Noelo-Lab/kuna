//! kuna fix `local_class_anonymous_subclass` (`kuna_fixes`): a local class's
//! declaration must come before its first use in the method. Vineflower puts
//! it before the first statement that `NestedClassProcessor.searchForClass`
//! sees using the class (a class literal, a field or method of it, `new` of
//! it, a variable of its type), but `new` of an anonymous class that extends
//! the local class (`class Local<T> {} ... new Local<String>() {}`) names only
//! the anonymous class, so the declaration lands after that statement, at a
//! later use, and javac cannot resolve the anonymous class's supertype. With
//! the fix, `new` of an anonymous class whose superclass or interface is the
//! local class is a use of it.

use crate::decompiler_context::DecompilerContext;
use crate::var_type::VarType;

/// Whether the class `new_type` names extends or implements `classname`.
pub fn extends(new_type: &VarType, classname: &str) -> bool {
    let Some(value) = new_type.value.as_deref() else {
        return false;
    };
    let Some(class) = DecompilerContext::get_struct_context().get_class(value) else {
        return false;
    };
    class.super_class().is_some_and(|s| &*s.get_string() == classname)
        || class.get_interface_names().iter().any(|i| i == classname)
}
