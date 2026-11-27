//! Port of `org/jetbrains/java/decompiler/modules/decompiler/GenericsProcessor.java`.

use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::statement::Statement;

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
        let mut chain: Vec<InvocationExprent> = Vec::new();

        chain.push(invoc);
        let mut temp = invoc;
        while let Some(instance) = temp.get_instance().filter(|i| i.is_invocation()) {
            temp = instance.as_invocation();
            chain.push(temp);
        }

        if chain.len() <= 1 {
            return false;
        }

        let last = chain[chain.len() - 1];
        let mut found_lambda = false;
        for parameter in last.get_lst_parameters().iter() {
            if let Some(new_exprent) = parameter.try_as_new() {
                if new_exprent.is_lambda() {
                    if new_exprent.is_method_reference()
                        && !new_exprent.does_class_have_methods_named_same()
                    {
                        continue;
                    }

                    found_lambda = true;
                    break;
                }
            }
        }
        if !found_lambda {
            return false;
        }

        last.get_inferred_expr_type(None);
        if last.get_desc().is_none() {
            return false;
        }

        if last.get_desc().expect("getDesc").get_signature().is_none() {
            return false;
        }

        let base_types = last
            .get_desc()
            .expect("getDesc")
            .get_signature()
            .expect("getSignature")
            .type_parameters()
            .clone();
        let ret_type = last.get_descriptor().ret.clone();

        for expr in &chain {
            if !expr
                .get_descriptor()
                .ret
                .higher_equal_in_lattice_than(&ret_type)
            {
                return false;
            }
        }

        for expr in &chain {
            expr.get_inferred_expr_type(None);

            let Some(mt) = expr.get_desc() else {
                return false;
            };

            let Some(descriptor) = mt.get_signature() else {
                return false;
            };

            let cur_params = descriptor.type_parameters().clone();
            for cur_param in &cur_params {
                if base_types.contains(cur_param) {
                    last.d_mut(|d| d.force_generic_qualfication = true);
                    return true;
                }
            }
        }

        false
    }
}
