//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/SynchronizedStatement.java`.
//!
//! `headexprent` is the one-slot list shared with flow nodes (empty while the
//! Java slot is `null`).

use crate::code_constants::OPC_MONITOREXIT;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::if_statement::set_head_slot;
use crate::java_subclass;
use crate::rt::JList;
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementKind, StatementType};
use crate::text_buffer::TextBuffer;

pub struct SynchronizedStatementData {
    pub body: Option<Statement>,
    pub headexprent: JList<Exprent>,
}

java_subclass!(pub SynchronizedStatement: Statement, StatementKind::Synchronized(SynchronizedStatementData), is_synchronized, as_synchronized, try_as_synchronized);

impl SynchronizedStatement {
    /// Java: SynchronizedStatement()
    pub fn new_empty() -> SynchronizedStatement {
        SynchronizedStatement(Statement::new(
            StatementType::Synchronized,
            StatementKind::Synchronized(SynchronizedStatementData {
                body: None,
                headexprent: JList::new(),
            }),
        ))
    }

    /// Java: SynchronizedStatement(Statement, Statement, Statement)
    pub fn new(head: Statement, body: Statement, exc: Statement) -> SynchronizedStatement {
        let this = SynchronizedStatement::new_empty();

        this.set_first(head);
        let stats = this.get_stats();
        stats.borrow_mut().add_with_key(head, head.id());

        this.d_mut(|d| d.body = Some(body));
        stats.borrow_mut().add_with_key(body, body.id());

        stats.borrow_mut().add_with_key(exc, exc.id());

        let lst_succs = body.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        if let Some(&edge) = lst_succs.first() {
            if edge.get_type() == StatEdge::TYPE_REGULAR {
                let post = edge.get_destination();
                this.with_mut(|d| d.post = Some(post));
            }
        }
        this
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));
        buf.append_buffer(&self.get_first().expect("first").to_java(indent));

        if self.is_labeled() {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append(":")
                .append_line_separator();
        }

        let head_expr = self.get_headexprent_list().first();
        buf.append_indent(indent);
        match head_expr {
            Some(head_expr) => buf.append_buffer(&head_expr.to_java(indent)),
            None => buf.append("synchronized <null condition> "),
        };
        buf.append(" {").append_line_separator();

        buf.append_buffer(&ExprProcessor::jmp_wrapper(
            self.get_body(),
            indent + 1,
            true,
        ));

        buf.append_indent(indent).append("}");
        self.map_monitor_exit_instr(&mut buf);
        buf.append_line_separator();

        buf
    }

    fn map_monitor_exit_instr(self, buffer: &mut TextBuffer) {
        let block = self.get_body().get_basichead().get_block();
        if !block.get_seq().is_empty()
            && block
                .get_last_instruction()
                .expect("NullPointerException")
                .with(|d| d.opcode)
                == OPC_MONITOREXIT
        {
            let offset = block.get_old_offset(block.size() - 1);
            if offset > -1 {
                buffer.add_bytecode_mapping_offset(offset);
            }
        }
    }

    pub fn init_exprents(self) {
        let exprents = self
            .get_first()
            .expect("first")
            .get_exprents()
            .expect("exprents");
        let monitor = exprents.remove_at(exprents.len() - 1);
        set_head_slot(&self.get_headexprent_list(), monitor);
    }

    pub fn get_stat_exprents(self) -> Vec<Exprent> {
        self.get_headexprent_list().to_vec()
    }

    pub fn replace_exprent(self, oldexpr: Exprent, newexpr: Exprent) {
        let head = self.get_headexprent_list();
        if head.first() == Some(oldexpr) {
            head.set(0, newexpr);
        }
    }

    pub fn replace_statement(self, oldstat: Statement, newstat: Statement) {
        self.d_mut(|d| {
            if d.body == Some(oldstat) {
                d.body = Some(newstat);
            }
        });

        self.0.replace_statement_base(oldstat, newstat);
    }

    pub fn remove_exc(self) {
        let stats = self.get_stats();
        let exc = stats.borrow().get(2);
        SequenceHelper::destroy_statement_content(exc, true);

        stats.borrow_mut().remove_with_key(&exc.id());
    }

    pub fn get_simple_copy(self) -> Statement {
        SynchronizedStatement::new_empty().0
    }

    pub fn init_simple_copy(self) {
        let stats = self.get_stats();
        let (first, body) = {
            let stats = stats.borrow();
            (stats.get(0), stats.get(1))
        };
        self.set_first(first);
        self.d_mut(|d| d.body = Some(body));
    }

    pub fn get_body(self) -> Statement {
        self.d(|d| d.body).expect("synchronized body")
    }

    pub fn set_body(self, body: Statement) {
        self.d_mut(|d| d.body = Some(body))
    }

    /// The live one-slot list (empty while the Java slot is `null`).
    pub fn get_headexprent_list(self) -> JList<Exprent> {
        self.d(|d| d.headexprent.clone())
    }

    pub fn get_headexprent(self) -> Exprent {
        self.d(|d| d.headexprent.first())
            .expect("synchronized head exprent")
    }
}
