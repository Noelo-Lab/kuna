//! kuna fix `kt_unescaped_name`: in Kotlin output, the names the writers
//! Kotlin shares with Java print are written as Kotlin names.
//!
//! `KotlinWriter.toValidKotlinIdentifier` puts a name Kotlin cannot write
//! bare in backticks: a hard keyword (`in`, `is`, `object`), a name with
//! `$` (`$this$map`, `L$0`, `access$getFoo$p`). The Kotlin writers apply it
//! to the names they declare and to `KVarExprent`, but four writers print
//! the class file's name as Java prints it: `VarExprent`, which
//! `NestedClassProcessor` puts in an object's methods for the variables the
//! object captures (`return $this$asList.length`); the lambda headers of
//! `KFieldExprent`, `KotlinWriter.writeLambda` and `KNewExprent`, which take
//! the name from the metadata or the variable processor while the body's
//! `KVarExprent` escapes it (`{ in -> `in` }`); `FieldExprent`
//! (`System.in`, `this.L$0`); and `InvocationExprent` (`access$getFoo$p()`,
//! after Java's own repair of a name Java cannot write). kotlinc rejects all
//! of these. With the fix they go through the same escape; `this`, a
//! `this@label` and the `<VAR_NAMELESS_ENCLOSURE>` marker are left as they
//! are, as `KVarExprent` leaves them. A lambda parameter the source names
//! `_` has the name `<anonymous parameter N>` in the metadata and the
//! LocalVariableTable, which no Kotlin name can spell; a lambda header
//! writes it `_`, as the source does (the body cannot read it). A class's
//! binary name (`Foo$Bar`, a synthetic `$$inlined$` class) is not a name
//! Kotlin can refer to, in backticks or not, and is left as it is.

use crate::kotlin_writer::KotlinWriter;
use crate::kuna_fix_kt_java_cast_syntax::writing_kotlin;
use crate::var_exprent::VarExprent;

/// A variable's name as the Kotlin writer prints it, when the class being
/// written is Kotlin; else `name`.
pub fn variable(name: &str) -> String {
    if !writing_kotlin()
        || name == "this"
        || name.starts_with("this@")
        || name == VarExprent::VAR_NAMELESS_ENCLOSURE
    {
        return name.to_string();
    }
    KotlinWriter::to_valid_kotlin_identifier(name)
}

/// A field's or method's name as the Kotlin writer prints it, when the
/// class being written is Kotlin; else `name`.
pub fn member(name: &str) -> String {
    if !writing_kotlin() {
        return name.to_string();
    }
    KotlinWriter::to_valid_kotlin_identifier(name)
}

/// A lambda parameter's name in the lambda's header: `_` for kotlinc's
/// `<anonymous parameter N>`, else as [`variable`] writes it.
pub fn lambda_parameter(name: &str) -> String {
    let anonymous = name
        .strip_prefix("<anonymous parameter ")
        .and_then(|rest| rest.strip_suffix('>'))
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|c| c.is_ascii_digit()));
    if anonymous && writing_kotlin() {
        return "_".to_string();
    }
    variable(name)
}
