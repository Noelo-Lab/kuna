//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/DoStatement.java`.
//!
//! The three one-slot head lists are shared with flow nodes; a Java `null`
//! slot is the empty list.
//!
//! A for-each over a raw iterable casts it to its class parameterized by
//! the loop variable's type (`raw_receiver_erasure`).

use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::if_statement::set_head_slot;
use crate::java_subclass;
use crate::kuna_fix_raw_receiver_erasure;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::{JHashSet, JList, Shared};
use crate::stat_edge::StatEdge;
use crate::statement::{
    add_instanceof_pattern_vars, LastBasicType, Statement, StatementKind, StatementType,
};
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;

/// `DoStatement.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    Infinite,
    DoWhile,
    While,
    For,
    ForEach,
}

pub struct DoStatementData {
    pub looptype: Type,
    pub init_exprent: JList<Exprent>,
    pub condition_exprent: JList<Exprent>,
    pub inc_exprent: JList<Exprent>,
}

/// A head slot the Java code dereferences unconditionally.
#[track_caller]
fn null_slot(slot: Option<Exprent>) -> Exprent {
    slot.expect("NullPointerException: empty loop head slot")
}

java_subclass!(pub DoStatement: Statement, StatementKind::Do(DoStatementData), is_do, as_do, try_as_do);

impl DoStatement {
    /// Java: DoStatement()
    pub fn new_empty() -> DoStatement {
        DoStatement(Statement::new(
            StatementType::Do,
            StatementKind::Do(DoStatementData {
                looptype: Type::Infinite,
                init_exprent: JList::new(),
                condition_exprent: JList::new(),
                inc_exprent: JList::new(),
            }),
        ))
    }

    /// Java: DoStatement(Statement)
    pub fn new(head: Statement) -> DoStatement {
        let this = DoStatement::new_empty();
        this.set_first(head);
        this.get_stats().borrow_mut().add_with_key(head, head.id());
        this
    }

    pub fn is_head(head: Statement) -> Option<Statement> {
        if head.get_last_basic_type() == LastBasicType::General && !head.is_monitor_enter() {
            let lst_succs = head.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            let edge = lst_succs.first().copied();

            if let Some(edge) = edge {
                if edge.get_type() == StatEdge::TYPE_REGULAR && edge.get_destination() == head {
                    return Some(DoStatement::new(head).0);
                }
            }

            if !head.is_do()
                && edge.is_none_or(|e| e.get_type() != StatEdge::TYPE_REGULAR)
                && head
                    .get_continue_set()
                    .borrow()
                    .contains(&head.get_basichead().0)
            {
                return Some(DoStatement::new(head).0);
            }
        }

        None
    }

    pub fn build_continue_set(self) -> Shared<JHashSet<Statement>> {
        self.0.build_continue_set_base();

        let continue_set = self.get_continue_set();
        let basichead = self.get_first().expect("first").get_basichead().0;
        continue_set.borrow_mut().remove(&basichead);

        continue_set
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();

        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        if self.is_labeled() {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append(":")
                .append_line_separator();
        }

        let first = self.get_first().expect("first");
        match self.get_looptype() {
            Type::Infinite => {
                buf.append_indent(indent)
                    .append("while (true) {")
                    .append_line_separator();
                buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, false));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
            Type::DoWhile => {
                buf.append_indent(indent)
                    .append("do {")
                    .append_line_separator();
                buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, false));
                buf.append_indent(indent).append("} while (");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline();
                buf.append_buffer(&null_slot(self.get_condition_exprent()).to_java(indent));
                buf.append_possible_newline_dedent("", true);
                buf.pop_newline_group();
                buf.append(");").append_line_separator();
            }
            Type::While => {
                buf.append_indent(indent).append("while (");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline();
                buf.append_buffer(&null_slot(self.get_condition_exprent()).to_java(indent));
                buf.append_possible_newline_dedent("", true);
                buf.pop_newline_group();
                buf.append(") {").append_line_separator();
                buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, false));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
            Type::For => {
                buf.append_indent(indent);
                buf.push_newline_group(indent, 1);
                buf.append("for (");
                if let Some(init) = self.get_init_exprent() {
                    buf.append_buffer(&init.to_java(indent));
                }
                buf.append(";")
                    .append_possible_newline_alt(" ")
                    .append_buffer(&null_slot(self.get_condition_exprent()).to_java(indent))
                    .append(";")
                    .append_possible_newline_alt(" ")
                    .append_buffer(&null_slot(self.get_inc_exprent()).to_java(indent))
                    .append_possible_newline_dedent("", true);
                buf.pop_newline_group();
                buf.append(") {").append_line_separator();
                buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, false));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
            Type::ForEach => {
                buf.append_indent(indent)
                    .append("for (")
                    .append_buffer(&null_slot(self.get_init_exprent()).to_java(indent));
                null_slot(self.get_inc_exprent()).get_inferred_expr_type(None);
                buf.append(" : ");
                let iterable = null_slot(self.get_inc_exprent());
                let cast = if kuna_fixes::enabled(KunaFix::RawReceiverErasure) {
                    null_slot(self.get_init_exprent())
                        .try_as_var()
                        .and_then(|v| {
                            kuna_fix_raw_receiver_erasure::foreach_cast_type(
                                iterable,
                                &v.get_definition_var_type(),
                            )
                        })
                } else {
                    None
                };
                match cast {
                    Some(cast) => kuna_fix_raw_receiver_erasure::append_cast(
                        &mut buf, iterable, &cast, indent,
                    ),
                    None => {
                        buf.append_buffer(&iterable.to_java(indent));
                    }
                }
                buf.append(") {").append_line_separator();
                buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, true));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
        }

        buf
    }

    pub fn get_stat_exprents(self) -> Vec<Exprent> {
        let mut lst = Vec::new();
        let looptype = self.get_looptype();

        match looptype {
            Type::For | Type::While => {
                if looptype == Type::For {
                    if let Some(init) = self.get_init_exprent() {
                        lst.push(init);
                    }
                }
                lst.extend(self.get_condition_exprent());
            }
            Type::ForEach => {
                lst.extend(self.get_init_exprent());
                lst.extend(self.get_inc_exprent());
            }
            _ => {}
        }

        match looptype {
            Type::DoWhile => lst.extend(self.get_condition_exprent()),
            Type::For => lst.extend(self.get_inc_exprent()),
            _ => {}
        }

        lst
    }

    pub fn replace_exprent(self, oldexpr: Exprent, newexpr: Exprent) {
        for list in [
            self.get_init_exprent_list(),
            self.get_condition_exprent_list(),
            self.get_inc_exprent_list(),
        ] {
            if list.first() == Some(oldexpr) {
                list.set(0, newexpr);
            }
        }
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        let mut vars = Vec::new();

        if self.get_looptype() == Type::ForEach {
            DoStatement::get_patterns(self.get_inc_exprent().expect("foreach iterable"), &mut vars);
            return Some(vars);
        }

        let condition = self.get_condition_exprent()?;

        DoStatement::get_patterns(condition, &mut vars);

        Some(vars)
    }

    fn get_patterns(cond: Exprent, vars: &mut Vec<VarExprent>) {
        let condition_list = cond.get_all_exprents_rec_self(true, true);
        add_instanceof_pattern_vars(&condition_list, vars);
    }

    pub fn has_basic_succ_edge(self) -> bool {
        self.get_looptype() != Type::Infinite
    }

    pub fn get_simple_copy(self) -> Statement {
        DoStatement::new_empty().0
    }

    /// The live one-slot list (empty while the Java slot is `null`).
    pub fn get_init_exprent_list(self) -> JList<Exprent> {
        self.d(|d| d.init_exprent.clone())
    }

    /// The live one-slot list (empty while the Java slot is `null`).
    pub fn get_condition_exprent_list(self) -> JList<Exprent> {
        self.d(|d| d.condition_exprent.clone())
    }

    /// The live one-slot list (empty while the Java slot is `null`).
    pub fn get_inc_exprent_list(self) -> JList<Exprent> {
        self.d(|d| d.inc_exprent.clone())
    }

    pub fn get_condition_exprent(self) -> Option<Exprent> {
        self.d(|d| d.condition_exprent.first())
    }

    pub fn set_condition_exprent(self, condition_exprent: Exprent) {
        set_head_slot(&self.get_condition_exprent_list(), condition_exprent);
    }

    pub fn get_inc_exprent(self) -> Option<Exprent> {
        self.d(|d| d.inc_exprent.first())
    }

    pub fn set_inc_exprent(self, inc_exprent: Exprent) {
        set_head_slot(&self.get_inc_exprent_list(), inc_exprent);
    }

    pub fn get_init_exprent(self) -> Option<Exprent> {
        self.d(|d| d.init_exprent.first())
    }

    pub fn set_init_exprent(self, init_exprent: Exprent) {
        set_head_slot(&self.get_init_exprent_list(), init_exprent);
    }

    pub fn get_looptype(self) -> Type {
        self.d(|d| d.looptype)
    }

    pub fn set_looptype(self, looptype: Type) {
        self.d_mut(|d| d.looptype = looptype)
    }
}
