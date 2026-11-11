//! Port of `org/jetbrains/java/decompiler/util/token/TextTokenDumpVisitor.java`.
//!
//! It reads positions from, and finally appends its dump to, the buffer being
//! visited, so it holds that buffer as `Shared` and is run through
//! `TextBuffer::visit_tokens_shared`.

use crate::field_descriptor::FieldDescriptor;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::Shared;
use crate::text_buffer::TextBuffer;
use crate::text_range::TextRange;
use crate::text_token_visitor::TextTokenVisitor;

pub struct TextTokenDumpVisitor {
    next: Option<Box<dyn TextTokenVisitor>>,
    buffer: Shared<TextBuffer>,
    text: Option<TextBuffer>,
}

impl TextTokenDumpVisitor {
    pub fn new(
        next: Box<dyn TextTokenVisitor>,
        buffer: Shared<TextBuffer>,
    ) -> TextTokenDumpVisitor {
        TextTokenDumpVisitor {
            next: Some(next),
            buffer,
            text: None,
        }
    }

    fn text(&mut self) -> &mut TextBuffer {
        self.text
            .as_mut()
            .expect("TextTokenDumpVisitor used before start()")
    }

    fn range(&mut self, range: TextRange) -> &mut TextBuffer {
        let start = self.buffer.borrow().get_pos(range.start);
        let end = self.buffer.borrow().get_pos(range.get_end());
        let text = self.text();
        text.append("(").append(&start);
        text.append(", ").append(&end);
        text.append(")")
    }

    fn declaration(&mut self, declaration: bool) -> &mut TextBuffer {
        self.text().append(if declaration {
            "[declaration]"
        } else {
            "[reference]"
        })
    }

    fn visit_variable(
        &mut self,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        self.declaration(declaration).append(" ");
        let text = self.text();
        text.append(class_name);
        text.append("#").append(method_name);
        text.append(&method_descriptor.to_string());
        text.append("(").append_int(index);
        text.append(":").append(name).append(")");
        text.append_line_separator();
    }
}

impl TextTokenVisitor for TextTokenDumpVisitor {
    fn get_next(&mut self) -> Option<&mut dyn TextTokenVisitor> {
        match &mut self.next {
            Some(next) => Some(next.as_mut()),
            None => None,
        }
    }

    fn start(&mut self, content: &str) {
        self.start_base(content);
        let mut text = TextBuffer::new();
        text.append_line_separator()
            .append("/*")
            .append_line_separator()
            .append("Tokens:")
            .append_line_separator();
        self.text = Some(text);
    }

    fn visit_class(&mut self, range: TextRange, declaration: bool, name: &str) {
        self.visit_class_base(range, declaration, name);
        self.range(range).append(" class ");
        self.declaration(declaration).append(" ");
        let text = self.text();
        text.append(name);
        text.append_line_separator();
    }

    fn visit_field(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &FieldDescriptor,
    ) {
        self.visit_field_base(range, declaration, class_name, name, descriptor);
        self.range(range).append(" field ");
        self.declaration(declaration).append(" ");
        let text = self.text();
        text.append(class_name);
        text.append("#").append(name);
        text.append(":").append(&descriptor.descriptor_string);
        text.append_line_separator();
    }

    fn visit_method(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &MethodDescriptor,
    ) {
        self.visit_method_base(range, declaration, class_name, name, descriptor);
        self.range(range).append(" method ");
        self.declaration(declaration).append(" ");
        let text = self.text();
        text.append(class_name);
        text.append("#").append(name);
        text.append(&descriptor.to_string());
        text.append_line_separator();
    }

    fn visit_parameter(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        self.visit_parameter_base(
            range,
            declaration,
            class_name,
            method_name,
            method_descriptor,
            index,
            name,
        );
        self.range(range).append(" parameter ");
        self.visit_variable(
            declaration,
            class_name,
            method_name,
            method_descriptor,
            index,
            name,
        );
    }

    fn visit_local(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        self.visit_local_base(
            range,
            declaration,
            class_name,
            method_name,
            method_descriptor,
            index,
            name,
        );
        self.range(range).append(" local ");
        self.visit_variable(
            declaration,
            class_name,
            method_name,
            method_descriptor,
            index,
            name,
        );
    }

    fn end(&mut self) {
        self.end_base();
        let dump = {
            let text = self.text();
            text.append("*/").append_line_separator();
            text.convert_to_string_and_allow_data_discard()
        };
        self.buffer.borrow_mut().append(&dump);
    }
}
