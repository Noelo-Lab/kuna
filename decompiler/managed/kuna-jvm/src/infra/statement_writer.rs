//! Port of `org/jetbrains/java/decompiler/api/plugin/StatementWriter.java`, with
//! `ClassWriter implements StatementWriter`.

use crate::class_writer::ClassWriter;
use crate::classes_processor::ClassNode;
use crate::exprent::Exprent;
use crate::import_collector::ImportCollector;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;

pub trait StatementWriter {
    fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    );

    fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32);

    fn ends_with_semicolon(&self, expr: Exprent) -> bool;
}

impl StatementWriter for ClassWriter {
    fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    ) {
        ClassWriter::write_class_header(self, cl, buffer, import_collector)
    }

    fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        ClassWriter::write_class(self, node, buffer, indent)
    }

    fn ends_with_semicolon(&self, expr: Exprent) -> bool {
        ClassWriter::ends_with_semicolon(self, expr)
    }
}
