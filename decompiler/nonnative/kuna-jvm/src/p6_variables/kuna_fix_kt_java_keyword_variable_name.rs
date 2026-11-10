//! kuna fix `kt_java_keyword_variable_name`: in a class written as Kotlin, a
//! debug variable name that is a Java keyword but not reserved in Kotlin is
//! the variable's name.
//!
//! `VarProcessor.setDebugVarNames` takes a LocalVariableTable name only when
//! `TextUtil.isValidIdentifier` accepts it, a test of Java's rules that
//! rejects Java's keywords. Kotlin reserves other words: `new`, `default`,
//! `char`, `int` and `byte` are ordinary Kotlin names, and kotlinc writes
//! them to the table. The variable then keeps its generated name (`var2`)
//! while the Kotlin writer prints the declaring header from the metadata
//! (`fun f(default: Charset)`, `{ old, new -> ..}`), so the body reads a
//! variable nothing declares. In Kotlin output the table's name is taken
//! when it is a Java identifier, keyword or not; the Kotlin variable writer
//! puts a Kotlin hard keyword in backticks. `this` stays refused, as the
//! Kotlin writer prints a variable named `this` as the receiver.

use crate::kuna_fix_kt_java_cast_syntax::writing_kotlin;
use crate::text_util::TextUtil;

/// Whether `name`, a debug name `TextUtil.isValidIdentifier` refused, names
/// the variable in the class being decompiled.
pub fn accepts(name: &str) -> bool {
    name != "this" && TextUtil::is_java_identifier(name) && writing_kotlin()
}
