//! kuna fix `kt_java_class_literal`: a class constant in a class written as
//! Kotlin is written `Name::class.java`.
//!
//! The plugin turns the expressions of each method into its own, which write
//! a class constant (`ldc C`) as Kotlin's `C::class.java`; constants that
//! reach the text through Java's `ConstExprent` instead (the initializers the
//! initializer processor extracts after that pass, the bodies of `init`
//! blocks) keep Java's `C.class`, which Kotlin does not parse
//! (`Logger.getLogger(Http2.class.getName())`). With the fix (called from
//! `ConstExprent::to_java`) such a constant is written as the plugin writes
//! its own, in a class written as Kotlin.

use crate::const_exprent::ConstExprent;
use crate::k_const_exprent::KConstExprent;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;

/// Writes the class constant `constant` in Kotlin's form into `buf`; false
/// when the fix is off or the class is written as Java.
pub fn write(constant: ConstExprent, buf: &mut TextBuffer) -> bool {
    if !kuna_fixes::enabled(KunaFix::KtJavaClassLiteral)
        || !kuna_fix_kt_java_cast_syntax::writing_kotlin()
    {
        return false;
    }
    buf.append_buffer(&KConstExprent::new(constant).to_java(0));
    true
}
