//! Fix `kt_java_class_literal`: a class constant written through Java's
//! `ConstExprent` in a class written as Kotlin is `Name::class.java`.

use crate::const_exprent::ConstExprent;
use crate::k_const_exprent::KConstExprent;
use crate::fixes::kt_util::writing_kotlin;
use crate::text_buffer::TextBuffer;

/// Writes the class constant `constant` in Kotlin's form into `buf`; false
/// when the fix is off or the class is written as Java.
pub fn write(constant: ConstExprent, buf: &mut TextBuffer) -> bool {
    if !writing_kotlin() {
        return false;
    }
    buf.append_buffer(&KConstExprent::new(constant).to_java(0));
    true
}
