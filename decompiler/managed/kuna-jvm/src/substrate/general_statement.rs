//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/GeneralStatement.java`.

use crate::java_subclass;
use crate::rt::JLinkedHashSet;
use crate::statement::{Statement, StatementKind, StatementType};
use crate::text_buffer::TextBuffer;

pub struct GeneralStatementData {
    pub placeholder: bool,
}

java_subclass!(pub GeneralStatement: Statement, StatementKind::General(GeneralStatementData), is_general, as_general, try_as_general);

impl GeneralStatement {
    /// `statements` in the iteration order of the Java collection passed.
    pub fn new(
        head: Statement,
        statements: Vec<Statement>,
        post: Option<Statement>,
    ) -> GeneralStatement {
        let this = GeneralStatement(Statement::new(
            StatementType::General,
            StatementKind::General(GeneralStatementData { placeholder: false }),
        ));

        this.set_first(head);
        let stats = this.get_stats();
        stats.borrow_mut().add_with_key(head, head.id());

        let mut set = JLinkedHashSet::new();
        for st in statements {
            set.add(st);
        }
        set.remove(&head);

        for st in set.to_vec() {
            stats.borrow_mut().add_with_key(st, st.id());
        }

        this.with_mut(|d| d.post = post);
        this
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();

        if self.is_labeled() {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append(":")
                .append_line_separator();
        }

        buf.append_indent(indent)
            .append("abstract statement {")
            .append_line_separator();
        let stats = self.get_stats().borrow().to_vec();
        for stat in stats {
            buf.append_buffer(&stat.to_java(indent + 1));
        }
        buf.append_indent(indent).append("}");
        buf.append_line_separator();

        buf
    }

    pub fn set_placeholder(self, placeholder: bool) {
        self.d_mut(|d| d.placeholder = placeholder)
    }

    pub fn is_placeholder(self) -> bool {
        self.d(|d| d.placeholder)
    }
}
