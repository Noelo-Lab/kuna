//! kuna fix `enum_body_separator` (`kuna_fixes`): Java's enum body is
//! `{ [constants] [; members] }`, so members need the `;` even when there is no
//! constant before them. `ClassWriter.writeClass` writes the `;` only after the
//! last constant; with the fix, an enum whose first written member has no
//! constant before it gets the `;` on a line of its own first.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::ACC_ENUM;
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;

/// The `;` a class body written by `ClassWriter::write_class` still owes its members.
pub struct EnumBodySeparator {
    pending: bool,
    indent: i32,
}

impl EnumBodySeparator {
    /// Owed when the fix is on and `node` is declared with the `enum` keyword
    /// (the test of `write_class_definition`); `indent` is the class's.
    pub fn new(node: ClassNode, indent: i32) -> EnumBodySeparator {
        let (type_, access, cl) = node.with(|d| (d.type_, d.access, d.class_struct));
        let flags = if type_ == ClassNodeType::Root {
            cl.get_access_flags()
        } else {
            access
        };
        let pending = kuna_fixes::enabled(KunaFix::EnumBodySeparator)
            && type_ != ClassNodeType::Anonymous
            && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM)
            && (flags & ACC_ENUM) != 0;
        EnumBodySeparator {
            pending,
            indent: indent + 1,
        }
    }

    /// A constant was written: the `;` after the constants is Vineflower's.
    pub fn constant_written(&mut self) {
        self.pending = false;
    }

    /// Called before each member: writes the owed `;` line before the first
    /// one and returns true then.
    pub fn before_member(&mut self, buffer: &mut TextBuffer) -> bool {
        if !self.pending {
            return false;
        }
        self.pending = false;
        buffer
            .append_indent(self.indent)
            .append(";")
            .append_line_separator();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_separator_once_and_only_when_owed() {
        let mut buffer = TextBuffer::with_format("\n", "   ", 160);
        let mut sep = EnumBodySeparator {
            pending: true,
            indent: 1,
        };
        assert!(sep.before_member(&mut buffer));
        assert!(!sep.before_member(&mut buffer));
        assert!(buffer.content_equals("   ;\n"));

        let mut buffer = TextBuffer::with_format("\n", "   ", 160);
        let mut sep = EnumBodySeparator {
            pending: true,
            indent: 1,
        };
        sep.constant_written();
        assert!(!sep.before_member(&mut buffer));
        assert!(buffer.content_equals(""));
    }
}
