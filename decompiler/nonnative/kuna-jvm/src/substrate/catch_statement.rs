//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/CatchStatement.java`.
//!
//! kuna fix `catch_loop_back_successor` (`kuna_fixes`): `is_head`
//! builds no try/catch whose successor would be one of its own members (the
//! try block, when the handlers jump back to it as a loop's continue); the
//! loop is then isolated first, as Vineflower already does for one handler.

use crate::code_constants::OPC_ASTORE;
use crate::code_type::CodeType;
use crate::counter_container::CounterContainer;
use crate::dec_helper::DecHelper;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::java_subclass;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::{BitSet, JHashSet, JList};
use crate::stat_edge::StatEdge;
use crate::statement::{LastBasicType, Statement, StatementKind, StatementType};
use crate::text_buffer::TextBuffer;
use crate::var_exprent::{DefinitionLocker, VarExprent};
use crate::var_type::VarType;

pub struct CatchStatementData {
    pub exctstrings: JList<Vec<String>>,
    pub vars: JList<VarExprent>,
    pub resources: JList<Exprent>,
}

java_subclass!(pub CatchStatement: Statement, StatementKind::Catch(CatchStatementData), is_catch, as_catch, try_as_catch);

/// `new VarExprent(VAR_COUNTER++, new VarType(CodeType.OBJECT, 0, type), DecompilerContext.getVarProcessor())`.
pub(crate) fn new_object_var(type_name: &str) -> VarExprent {
    let index = DecompilerContext::get_counter_container()
        .get_counter_and_increment(CounterContainer::VAR_COUNTER);
    let var_type = VarType::new_value(CodeType::Object, 0, type_name);
    VarExprent::new(
        index,
        Some(var_type),
        Some(DecompilerContext::get_var_processor()),
    )
}

impl CatchStatement {
    /// Java: CatchStatement()
    pub fn new_empty() -> CatchStatement {
        CatchStatement(Statement::new(
            StatementType::TryCatch,
            StatementKind::Catch(CatchStatementData {
                exctstrings: JList::new(),
                vars: JList::new(),
                resources: JList::new(),
            }),
        ))
    }

    /// Java: CatchStatement(Statement, Statement, Set<Statement>)
    pub fn new(
        head: Statement,
        next: Option<Statement>,
        set_handlers: &JHashSet<Statement>,
    ) -> CatchStatement {
        let this = CatchStatement::new_empty();

        this.set_first(head);
        let stats = this.get_stats();
        stats.borrow_mut().add_with_key(head, head.id());

        let (exctstrings, vars) = this.d(|d| (d.exctstrings.clone(), d.vars.clone()));
        for edge in head.get_successor_edges(StatEdge::TYPE_EXCEPTION) {
            let stat = edge.get_destination();

            if set_handlers.contains(&stat) {
                stats.borrow_mut().add_with_key(stat, stat.id());
                let exceptions = edge.get_exceptions().expect("handler exceptions");
                exctstrings.add(exceptions.clone());

                vars.add(new_object_var(&exceptions[0]));
            }
        }

        if let Some(next) = next {
            this.with_mut(|d| d.post = Some(next));
        }
        this
    }

    pub fn is_head(head: Statement) -> Option<Statement> {
        if head.get_last_basic_type() != LastBasicType::General {
            return None;
        }

        let mut set_handlers = DecHelper::get_unique_pred_exceptions(head);
        if !set_handlers.is_empty() {
            let mut hnextcount = 0;

            let mut next: Option<Statement> = None;
            let lst_head_succs = head.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            if let Some(&edge) = lst_head_succs.first() {
                if edge.get_type() == StatEdge::TYPE_REGULAR {
                    next = Some(edge.get_destination());
                    hnextcount = 2;
                }
            }

            for edge in head.get_successor_edges(StatEdge::TYPE_EXCEPTION) {
                let stat = edge.get_destination();

                let mut handlerok = true;

                if edge.get_exceptions().is_some() && set_handlers.contains(&stat) {
                    if stat.get_last_basic_type() != LastBasicType::General {
                        handlerok = false;
                    } else {
                        let lst_stat_succs =
                            stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
                        if let Some(&succ) = lst_stat_succs.first() {
                            if succ.get_type() == StatEdge::TYPE_REGULAR {
                                let statn = succ.get_destination();

                                if next.is_none() {
                                    next = Some(statn);
                                } else if next != Some(statn) {
                                    handlerok = false;
                                }

                                if handlerok {
                                    hnextcount += 1;
                                }
                            }
                        }
                    }
                } else {
                    handlerok = false;
                }

                if !handlerok {
                    set_handlers.remove(&stat);
                }
            }

            if kuna_fixes::enabled(KunaFix::CatchLoopBackSuccessor)
                && next.is_some_and(|n| n == head || set_handlers.contains(&n))
            {
                return None;
            }

            if hnextcount != 1 && !set_handlers.is_empty() {
                let mut lst = vec![head];
                lst.extend(set_handlers.to_vec());

                for st in &lst {
                    if st.is_monitor_enter() {
                        return None;
                    }
                }

                if DecHelper::invalid_head_merge(head) {
                    return None;
                }

                if DecHelper::check_statement_exceptions(&lst) {
                    return Some(CatchStatement::new(head, next, &set_handlers).0);
                }
            }
        }
        None
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

        let resources = self.get_resources();
        if resources.is_empty() {
            buf.append_indent(indent)
                .append("try {")
                .append_line_separator();
        } else {
            buf.append_indent(indent).append("try (");

            if resources.len() > 1 {
                buf.append_line_separator();
                buf.append_buffer(&ExprProcessor::list_to_java(Some(&resources), indent + 1));
                buf.append_indent(indent);
            } else {
                buf.append_buffer(&resources.get(0).to_java(indent + 1));
            }
            buf.append(") {").append_line_separator();
        }

        let first = self.get_first().expect("first");
        buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, true));
        buf.append_indent(indent).append("}");

        let stats = self.get_stats();
        let mut i = 1;
        while i < stats.borrow().len() {
            let stat = stats.borrow().get(i);
            let block = stat.get_basichead().get_block();
            if !block.get_seq().is_empty()
                && block.get_instruction(0).with(|d| d.opcode) == OPC_ASTORE
            {
                let offset = block.get_old_offset(0);
                if offset > -1 {
                    buf.add_bytecode_mapping_offset(offset);
                }
            }

            buf.append(" catch (");

            let exception_types = self.get_exct_strings().get(i - 1);
            for exc_index in 0..exception_types.len() {
                let name = ExprProcessor::get_cast_type_name(&VarType::new_value(
                    CodeType::Object,
                    0,
                    exception_types[exc_index].as_str(),
                ));
                if exception_types.len() > 1 && exc_index > 0 {
                    buf.append(" | ");
                }
                buf.append(&name);
            }

            buf.append(" ");

            let var = self.get_vars().get(i - 1);

            self.validate_type(&exception_types, &var.get_var_type());

            {
                let _v = DefinitionLocker::new(var);
                buf.append_buffer(&var.0.to_java(indent));
            }

            buf.append(") {").append_line_separator();
            buf.append_buffer(&ExprProcessor::jmp_wrapper(stat, indent + 1, false))
                .append_indent(indent)
                .append("}");
            i += 1;
        }
        buf.append_line_separator();

        buf
    }

    fn validate_type(self, _ex_types: &[String], _ex_var_type: &VarType) {}

    pub fn get_stat_exprents(self) -> Vec<Exprent> {
        let mut lst = self.get_resources().to_vec();
        lst.extend(self.get_vars().iter().map(|v| v.0));
        lst
    }

    pub fn get_simple_copy(self) -> Statement {
        let cs = CatchStatement::new_empty();
        let (cs_exctstrings, cs_vars) = cs.d(|d| (d.exctstrings.clone(), d.vars.clone()));

        for exc in self.get_exct_strings().iter() {
            cs_exctstrings.add(exc.clone());
            cs_vars.add(new_object_var(&exc[0]));
        }

        cs.0
    }

    pub fn get_offset(self, values: &mut BitSet) {
        self.0.get_offset_base(values);

        for exp in self.get_resources().iter() {
            exp.get_bytecode_range(Some(&mut *values));
        }
    }

    /// The live list.
    pub fn get_exct_strings(self) -> JList<Vec<String>> {
        self.d(|d| d.exctstrings.clone())
    }

    /// The live list.
    pub fn get_vars(self) -> JList<VarExprent> {
        self.d(|d| d.vars.clone())
    }

    /// The live list (shared with flow nodes).
    pub fn get_resources(self) -> JList<Exprent> {
        self.d(|d| d.resources.clone())
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        let mut vars = self.get_vars().to_vec();

        for exp in self.get_resources().iter() {
            if let Some(assignment) = exp.try_as_assignment() {
                vars.push(assignment.get_left().as_var());
            }
        }

        Some(vars)
    }
}
