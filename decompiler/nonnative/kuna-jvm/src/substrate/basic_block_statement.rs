//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/BasicBlockStatement.java`.
//!
//! kuna fix `copied_finally` (spec 10.16): a copy's block records the
//! block it was copied from.

use crate::basic_block::BasicBlock;
use crate::code_constants::{GROUP_JUMP, GROUP_SWITCH, OPC_GOTO, OPC_MONITOREXIT};
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::instruction_sequence::InstructionSequence;
use crate::java_subclass;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::JList;
use crate::start_end_pair::StartEndPair;
use crate::statement::{
    add_instanceof_pattern_vars, LastBasicType, Statement, StatementKind, StatementType,
};
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;

pub struct BasicBlockStatementData {
    pub block: BasicBlock,
    pub removable_monitorexit: bool,
}

java_subclass!(pub BasicBlockStatement: Statement, StatementKind::BasicBlock(BasicBlockStatementData), is_basic_block, as_basic_block, try_as_basic_block);

impl BasicBlockStatement {
    pub fn new(block: BasicBlock) -> BasicBlockStatement {
        let block_id = block.with(|d| d.id);
        let this = BasicBlockStatement(Statement::new_with_id(
            StatementType::BasicBlock,
            block_id,
            StatementKind::BasicBlock(BasicBlockStatementData {
                block,
                removable_monitorexit: false,
            }),
        ));

        let coun = DecompilerContext::get_counter_container();
        let id = this.id();
        if id >= coun.get_counter(CounterContainer::STATEMENT_COUNTER) {
            coun.set_counter(CounterContainer::STATEMENT_COUNTER, id + 1);
        }

        if let Some(instr) = block.get_last_instruction() {
            let (group, opcode) = instr.with(|d| (d.group, d.opcode));
            if group == GROUP_JUMP && opcode != OPC_GOTO {
                this.with_mut(|d| d.last_basic_type = LastBasicType::If);
            } else if group == GROUP_SWITCH {
                this.with_mut(|d| d.last_basic_type = LastBasicType::Switch);
            }
        }

        this.build_monitor_flags();
        this
    }

    pub fn mark_monitorexit_dead(self) {
        let seq = self.get_block().get_seq();

        if !seq.is_empty() {
            for i in 0..seq.length() {
                if seq.get_instr(i).with(|d| d.opcode) == OPC_MONITOREXIT {
                    self.set_removable_monitorexit(true);
                    break;
                }
            }
        }
    }

    pub fn get_basichead(self) -> BasicBlockStatement {
        self
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut tb = ExprProcessor::list_to_java(Some(&self.get_var_definitions()), indent);
        let islabeled = self.is_labeled();
        if islabeled {
            tb.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append_char(':')
                .append("// $VF: Invalid label!")
                .append_line_separator();
        }

        tb.append_buffer(&ExprProcessor::list_to_java(
            self.get_exprents().as_ref(),
            indent,
        ));
        tb
    }

    pub fn get_simple_copy(self) -> Statement {
        let newblock = BasicBlock::new(
            DecompilerContext::get_counter_container()
                .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER),
        );

        let seq = InstructionSequence::new();
        let old_seq = self.get_block().get_seq();
        for i in 0..old_seq.length() {
            seq.add_instruction(old_seq.get_instr(i).java_clone());
        }

        newblock.set_seq(seq);
        if kuna_fixes::enabled(KunaFix::CopiedFinally) {
            newblock.set_copy_of(Some(self.get_block()));
        }

        BasicBlockStatement::new(newblock).0
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        let exps = self.get_exprents()?;
        if exps.is_empty() {
            return None;
        }

        let mut vars = Vec::new();
        for exp in exps.iter() {
            let mut inner = exp.get_all_exprents_rec(true);
            inner.push(exp);

            add_instanceof_pattern_vars(&inner, &mut vars);
        }

        Some(vars)
    }

    pub fn is_removable_monitorexit(self) -> bool {
        self.d(|d| d.removable_monitorexit)
    }

    pub fn set_removable_monitorexit(self, removable_monitorexit: bool) {
        self.d_mut(|d| d.removable_monitorexit = removable_monitorexit)
    }

    pub fn create() -> BasicBlockStatement {
        let stat = BasicBlockStatement::new(BasicBlock::new(
            DecompilerContext::get_counter_container()
                .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER),
        ));
        stat.set_exprents(JList::new());
        stat
    }

    pub fn has_basic_succ_edge(self) -> bool {
        true
    }

    pub fn get_block(self) -> BasicBlock {
        self.d(|d| d.block)
    }

    pub fn replace_exprent(self, oldexpr: Exprent, newexpr: Exprent) {
        let exprents = self.get_exprents().expect("exprents");
        for i in 0..exprents.len() {
            if exprents.get(i) == oldexpr {
                exprents.set(i, newexpr);
            }
        }
    }

    pub fn get_start_end_range(self) -> StartEndPair {
        let block = self.get_block();
        if block.size() > 0 {
            StartEndPair::new(block.get_start_instruction(), block.get_end_instruction())
        } else {
            StartEndPair::new(0, 0)
        }
    }
}
