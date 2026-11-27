//! Port of `org/jetbrains/java/decompiler/modules/decompiler/PrimitiveExprsList.java`.

use crate::exprent::Exprent;
use crate::list_stack::ListStack;
use crate::rt::JList;

/// The expressions and operand stack of one block during the stack-to-expression lift.
///
/// A plain owned value: `ExprProcessor` keeps one per `DirectNode` in a local map and
/// hands it to `processBlock` by `&mut`. `lst_exprents` is a [`JList`] because it
/// becomes the block's exprent list (`block.setExprents(data.getLstExprents())`).
pub struct PrimitiveExprsList {
    lst_exprents: JList<Exprent>,
    stack: ListStack<Exprent>,
}

impl Default for PrimitiveExprsList {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveExprsList {
    pub fn new() -> PrimitiveExprsList {
        PrimitiveExprsList {
            lst_exprents: JList::new(),
            stack: ListStack::new(),
        }
    }

    pub fn copy_stack(&self) -> PrimitiveExprsList {
        let mut prlst = PrimitiveExprsList::new();
        prlst.set_stack(self.stack.clone());
        prlst
    }

    pub fn get_lst_exprents(&self) -> JList<Exprent> {
        self.lst_exprents.clone()
    }

    pub fn get_stack(&mut self) -> &mut ListStack<Exprent> {
        &mut self.stack
    }

    pub fn set_stack(&mut self, stack: ListStack<Exprent>) {
        self.stack = stack;
    }
}
