//! Port of `org/jetbrains/java/decompiler/modules/decompiler/GenericsProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::statement::Statement;
use crate::unported;

pub struct GenericsProcessor;

impl GenericsProcessor {
    pub fn qualify_chains(stat: Statement) -> bool {
        let mut res = false;

        if let Some(exprents) = stat.get_exprents() {
            for exprent in exprents.iter() {
                res |= Self::qualify_chain(exprent);
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::qualify_chains(st);
        }

        res
    }

    /// Java: qualifyChain(Exprent)
    fn qualify_chain(exp: Exprent) -> bool {
        let mut res = false;

        for expr in exp.get_all_exprents() {
            res |= Self::qualify_chain(expr);
        }

        if let Some(invocation) = exp.try_as_invocation() {
            res |= Self::qualify_chain_invocation(invocation);
        }

        res
    }

    /// Java: qualifyChain(InvocationExprent)
    fn qualify_chain_invocation(invoc: InvocationExprent) -> bool {
        unported!("GenericsProcessor.qualifyChain")
    }
}
