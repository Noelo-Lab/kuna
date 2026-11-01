//! Fold Go compiler check branches whose failure arm only calls a panic helper.
//! The Go language gate and exact runtime names exclude application panics.
//! Stores, other calls, volatile operations, and returning arms are retained.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// Compiler-generated checks, excluding explicit panic and type assertions.
pub fn is_go_check_name(name: &str) -> bool {
    let name = name
        .strip_suffix(".abi0")
        .or_else(|| name.strip_suffix(".abiinternal"))
        .unwrap_or(name);
    let Some(name) = name.strip_prefix("runtime.") else {
        return false;
    };
    if matches!(
        name,
        "panicBounds"
            | "panicBounds32"
            | "panicBounds64"
            | "panicdivide"
            | "panicshift"
            | "panicoverflow"
            | "panicunsafeslicelen"
            | "panicunsafeslicelen1"
            | "panicunsafeslicenilptr"
            | "panicunsafeslicenilptr1"
            | "panicunsafestringlen"
            | "panicunsafestringlen1"
            | "panicunsafestringnilptr"
            | "panicunsafestringnilptr1"
    ) {
        return true;
    }
    ["panic", "goPanic", "panicExtend", "goPanicExtend"]
        .iter()
        .any(|prefix| {
            name.strip_prefix(prefix).is_some_and(|kind| {
                matches!(
                    kind,
                    "Index"
                        | "IndexU"
                        | "SliceAlen"
                        | "SliceAlenU"
                        | "SliceAcap"
                        | "SliceAcapU"
                        | "SliceB"
                        | "SliceBU"
                        | "Slice3Alen"
                        | "Slice3AlenU"
                        | "Slice3Acap"
                        | "Slice3AcapU"
                        | "Slice3B"
                        | "Slice3BU"
                        | "Slice3C"
                        | "Slice3CU"
                        | "SliceConvert"
                )
            })
        })
}

fn panic_only(data: &Funcdata, block: BlockId) -> bool {
    if !matches!(data.bblocks_ref().block(block).kind(), BlockKind::Basic(_))
        || data.bblocks_ref().block(block).size_out() != 0
    {
        return false;
    }
    let mut found = false;
    for op in data.bb_ops(block) {
        let operation = data.obank().get(op).expect("gobounds op");
        if operation.code() == OpCode::CPUI_INDIRECT || operation.is_return_copy() {
            continue;
        }
        if operation.get_out().is_some_and(|id| {
            let value = data.vbank().get(id).expect("gobounds output");
            value.is_persist() || value.is_volatile()
        }) || (0..operation.num_input()).any(|slot| {
            operation
                .get_in(slot)
                .is_some_and(|id| data.vbank().get(id).expect("gobounds input").is_volatile())
        }) {
            return false;
        }
        let code = operation.code();
        match code {
            OpCode::CPUI_CALL => {
                if found {
                    return false;
                }
                let Some(idx) = data.get_call_specs_index(op) else {
                    return false;
                };
                if !is_go_check_name(data.get_call_specs(idx).get_name()) {
                    return false;
                }
                found = true;
            }
            OpCode::CPUI_STORE | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => return false,
            _ => {}
        }
    }
    found
}

pub struct ActionGoBounds {
    base: ActionBase,
}

impl ActionGoBounds {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "gobounds", group),
        })
    }
}

impl Action for ActionGoBounds {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(|| {
            Box::new(Self {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().go_bounds
            || !data.get_arch().source_lang.is_go()
            || data.num_calls() == 0
        {
            return 0;
        }
        for i in 0..data.bblocks_get_size() {
            let guard = data.bblocks_get_block(i);
            if data.bblocks_ref().block(guard).size_out() != 2 {
                continue;
            }
            let Some(tail) = data.bb_op_tail(guard) else {
                continue;
            };
            if data.obank().get(tail).expect("gobounds guard").code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            let candidates: Vec<_> = (0..2)
                .filter(|&edge| panic_only(data, data.bblocks_ref().block(guard).get_out(edge)))
                .collect();
            if candidates.len() != 1 {
                continue;
            }
            data.remove_branch(guard, candidates[0])
                .expect("gobounds remove branch");
            data.remove_unreachable_blocks(false, true)
                .expect("gobounds remove unreachable");
            self.base.count += 1;
            return 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_runtime_names_and_abi_wrappers() {
        for name in [
            "runtime.panicIndex",
            "runtime.goPanicSlice3CU",
            "runtime.panicExtendIndexU",
            "runtime.panicBounds64",
            "runtime.panicdivide",
            "runtime.panicunsafestringlen1",
        ] {
            for suffix in ["", ".abi0", ".abiinternal"] {
                assert!(is_go_check_name(&format!("{name}{suffix}")));
            }
        }
        for name in [
            "panicIndex",
            "main.panicIndex",
            "runtime.panicIndexExtra",
            "runtime.gopanic",
            "runtime.panicdottypeE",
            "runtime.panicIndex.abi0.extra",
            "runtime.panic",
            "",
        ] {
            assert!(!is_go_check_name(name), "{name}");
        }
    }
}
