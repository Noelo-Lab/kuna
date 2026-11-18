//! Fix `unseen_class_overload`: for a class outside the input, the methods of
//! that name the input calls stand in for its overloads, and an argument
//! another of them would take is written with the bytecode's type.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::decompiler_context::DecompilerContext;
use crate::invocation_exprent::InvocationExprent;
use crate::rt::BitSet;
use crate::fixes::overloads::vararg_spread_resolution::{
    more_specific, overloads, phase, same_parameters, written_type,
};
use crate::var_type::VarType;

type References = BTreeMap<(String, String), BTreeSet<String>>;

thread_local! {
    static REFERENCED: RefCell<Option<References>> = const { RefCell::new(None) };
}

/// Forgets the method references of the previous class universe.
pub fn reset() {
    REFERENCED.with(|r| *r.borrow_mut() = None);
}

/// The descriptors by which the input's classes call method `name` of
/// `class`, a class outside the input.
pub fn referenced(class: &str, name: &str) -> Vec<String> {
    REFERENCED.with(|r| {
        let mut r = r.borrow_mut();
        let refs = r.get_or_insert_with(collect);
        refs.get(&(class.to_string(), name.to_string()))
            .map(|d| d.iter().cloned().collect())
            .unwrap_or_default()
    })
}

fn collect() -> References {
    let ctx = DecompilerContext::get_struct_context();
    let mut refs = References::new();
    for cl in ctx.get_own_classes() {
        for (owner, name, descriptor) in cl.get_pool().method_refs() {
            if ctx.get_class(&owner).is_none() {
                refs.entry((owner, name)).or_default().insert(descriptor);
            }
        }
    }
    refs
}

/// Marks in `ambiguous` the arguments of `call`, whose class is outside the
/// input, to cast to their parameter types, and forces the boxing and
/// unboxing calls to keep, so that the call resolves to its method.
pub fn mark(call: InvocationExprent, ambiguous: &mut BitSet) {
    let descriptor = call.get_descriptor();
    let params = call.get_lst_parameters();
    if params.len() != descriptor.params.len() {
        return;
    }
    let args: Vec<VarType> = (0..params.len())
        .map(|i| written_type(params.get(i), &descriptor.params[i]))
        .collect();
    let Some(own) = phase(&args, &descriptor.params) else {
        return;
    };
    let mut differ = vec![false; params.len()];
    for (md, _, _) in overloads(call) {
        if same_parameters(&md.params, &descriptor.params) {
            continue;
        }
        let Some(other) = phase(&args, &md.params) else {
            continue;
        };
        if other > own || (other == own && more_specific(&descriptor.params, &md.params)) {
            continue;
        }
        for (i, d) in differ.iter_mut().enumerate() {
            *d |= !md.params[i].equals(&descriptor.params[i]);
        }
    }
    for (i, d) in differ.into_iter().enumerate() {
        if !d {
            continue;
        }
        let param = params.get(i);
        match param.try_as_invocation() {
            Some(inv) if inv.is_boxing_call() => inv.force_boxing(true),
            Some(inv) if inv.is_unboxing_call() => inv.force_unboxing(true),
            _ => ambiguous.set(i),
        }
    }
}
