//! Attach resolved Android targets to their native calls and callback entries.
use super::*;
use crate::kuna_androidcalls::JavaCall;

static ANNOTATED_CALL: OpToken = op_token("", "", 2, 62, false, TokenType::Space, 1, 0);

fn java_note(call: &JavaCall) -> String {
    let mut targets = call
        .targets
        .iter()
        .map(|target| target.key().unwrap_or_else(|| target.display()))
        .collect::<Vec<_>>();
    targets.dedup();
    if call.incomplete {
        targets.push("unresolved target".into());
    }
    format!("/* {} */", comment_safe(&targets.join(" | ")))
}

impl PrintC {
    pub(super) fn push_java_call_note(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        if arch.android_calls.java.is_empty() {
            return;
        }
        let Some(site) = fd.obank().get(op) else {
            return;
        };
        if let Some(call) = arch
            .android_calls
            .java_at(fd.get_address().get_offset(), site.get_addr().get_offset())
        {
            self.push_op(&ANNOTATED_CALL, Some(op_key(op)));
            self.push_atom(&Atom::syntax(
                java_note(call),
                TagType::Syntax,
                crate::printlanguage::SyntaxHighlight::comment_color,
            ));
        }
    }

    pub(super) fn emit_binder_transactions(
        &mut self,
        arch: &Architecture,
        space: &std::rc::Rc<AddrSpace>,
        entry: u64,
    ) {
        self.binder_notes.clear();
        let mut header = std::collections::BTreeSet::new();
        for service in arch
            .android_calls
            .binder
            .iter()
            .filter(|s| s.transact == entry)
        {
            if service.transactions.is_empty() {
                header.insert(format!("Binder {}", service.descriptor));
            }
            for transaction in &service.transactions {
                let mut note = format!(
                    "Binder {} transaction {}",
                    service.descriptor, transaction.code
                );
                if let Some(name) = &transaction.name {
                    note.push_str(" = ");
                    note.push_str(name);
                }
                if let Some(site) = transaction.site {
                    self.binder_notes
                        .entry((site, transaction.code))
                        .or_default()
                        .insert(note);
                } else {
                    header.insert(note);
                }
            }
        }
        for note in header {
            self.emit.tag_line();
            self.emit.tag_comment(
                &format!("/* {} */", comment_safe(&note)),
                SyntaxHighlight::CommentColor,
                space,
                entry,
            );
        }
    }

    pub(super) fn emit_binder_branch_note(&mut self, fd: &Funcdata, branch: OpId) {
        if self.binder_notes.is_empty() {
            return;
        }
        let Some(op) = fd.obank().get(branch) else {
            return;
        };
        if op.code() != OpCode::CPUI_CBRANCH {
            return;
        }
        let Some(test) = op.get_in(1).and_then(|vn| fd.vbank().get(vn)) else {
            return;
        };
        let Some(compare) = test.get_def().and_then(|id| fd.obank().get(id)) else {
            return;
        };
        if !matches!(
            compare.code(),
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL
        ) {
            return;
        }
        let value = (0..compare.num_input())
            .filter_map(|i| fd.vbank().get(compare.get_in(i)?))
            .find(|v| v.is_constant())
            .and_then(|v| u32::try_from(v.get_offset()).ok());
        let (Some(value), Some(space)) = (value, op.get_addr().get_space()) else {
            return;
        };
        for site in [op.get_addr().get_offset(), compare.get_addr().get_offset()] {
            self.emit_binder_site(space, site, value);
        }
    }

    pub(super) fn emit_binder_case_note(
        &mut self,
        fd: &Funcdata,
        blk: BlockId,
        case: &crate::block::CaseOrder,
        index: int4,
        value: u64,
    ) {
        if self.binder_notes.is_empty() {
            return;
        }
        let Some(table) = fd.sblocks_ref().block(blk).switch_jt_index() else {
            return;
        };
        let Some(block) = case.basicblock else { return };
        let table = fd.get_jump_table(table as int4);
        let Ok(index) = table.get_index_by_block(fd, block, index) else {
            return;
        };
        let Some(target) = table.address_table().get(index as usize) else {
            return;
        };
        let (Some(space), Ok(code)) = (target.get_space(), u32::try_from(value)) else {
            return;
        };
        self.emit_binder_site(space, target.get_offset(), code);
    }

    fn emit_binder_site(&mut self, space: &std::rc::Rc<AddrSpace>, site: u64, code: u32) {
        if let Some(notes) = self.binder_notes.remove(&(site, code)) {
            for note in notes {
                self.emit.spaces(1, 0);
                self.emit.tag_comment(
                    &format!("/* {} */", comment_safe(&note)),
                    SyntaxHighlight::CommentColor,
                    space,
                    site,
                );
            }
        }
    }

    pub(super) fn emit_remaining_binder_notes(&mut self, fd: &Funcdata) {
        if self.binder_notes.is_empty() {
            return;
        }
        let Some(space) = fd.get_address().get_space() else {
            return;
        };
        for ((site, _), notes) in std::mem::take(&mut self.binder_notes) {
            for note in notes {
                self.emit.tag_line();
                self.emit.tag_comment(
                    &format!("/* {} */", comment_safe(&note)),
                    SyntaxHighlight::CommentColor,
                    space,
                    site,
                );
            }
        }
    }
}
