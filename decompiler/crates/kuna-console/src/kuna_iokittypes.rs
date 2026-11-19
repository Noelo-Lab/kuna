//! (kuna `iokitclasses on`) Commit what the typed IOKit classes say about
//! functions and data, once [`kuna_decomp::kuna_vtabletypes::install`] has
//! built the class structures.
//!
//! Each virtual member, constructor and destructor gets an unlocked `x0`
//! input hint `this` of its class's pointer type, so its other parameters
//! are still recovered from its body. `X::MetaClass::alloc` is declared to
//! return `X *`. The slots of each typed vtable, from the address point a
//! constructor stores, become a data object `X_vtable` of type `X_vtbl`.

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use kuna_analysis::iokit::IokitCxxFacts;
use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_decomp::dtype::Datatype;
use kuna_decomp::kuna_inputhints::InputHint;
use kuna_decomp::varnode::varnode_flags;

use crate::engine::ConsoleProgram;

pub(crate) fn commit(prog: &mut ConsoleProgram, code_space: &Rc<AddrSpace>, facts: &IokitCxxFacts) -> KunaResult<()> {
    let ws = code_space.get_word_size();
    let class_pointer = |prog: &ConsoleProgram, class: &str| -> KunaResult<Option<Rc<Datatype>>> {
        let types = prog.arch().types();
        let Some(st) = types.find_by_name(class)?.filter(|t| !t.is_incomplete()) else { return Ok(None) };
        Ok(Some(types.get_type_pointer(types.get_size_of_pointer(), st, ws)?))
    };

    if let (false, Ok(loc)) = (facts.this_types.is_empty(), prog.arch().get_register_varnode(b"x0")) {
        if let Some(space) = loc.space.clone() {
            let mut pointers: HashMap<&str, Option<Rc<Datatype>>> = HashMap::new();
            let mut hints = BTreeMap::new();
            for (func, class) in &facts.this_types {
                if !pointers.contains_key(class.as_str()) {
                    pointers.insert(class, class_pointer(prog, class)?);
                }
                let Some(Some(ct)) = pointers.get(class.as_str()) else { continue };
                let hint = InputHint {
                    addr: Address::new(Rc::clone(&space), loc.offset),
                    size: loc.size as i32,
                    name: "this".to_string(),
                    ct: Some(Rc::clone(ct)),
                };
                hints.insert(Address::new(Rc::clone(code_space), *func), Rc::new(vec![hint]));
            }
            prog.arch_mut().kuna_this_hints = hints;
        }
    }

    for (func, class) in &facts.allocs {
        let Some(out) = class_pointer(prog, class)? else { continue };
        let addr = Address::new(Rc::clone(code_space), *func);
        if prog.arch().symboltab.function_proto_pieces_across_scopes(&addr).is_some() {
            continue;
        }
        let types = prog.arch().types();
        let pieces = kuna_decomp::fspec::PrototypePieces {
            name: format!("{class}::MetaClass::alloc"),
            outtype: Some(out),
            intypes: vec![types.get_type_pointer(types.get_size_of_pointer(), types.get_type_void()?, ws)?],
            innames: vec!["this".to_string()],
            first_var_arg_slot: -1,
            ..Default::default()
        };
        prog.arch_mut().set_function_prototype_pieces_at(&addr, pieces);
    }

    for (point, class) in &facts.vtable_points {
        let addr = Address::new(Rc::clone(code_space), *point);
        let arch = prog.arch_mut();
        let Some(vtbl) = arch.types().find_by_name(&format!("{class}_vtbl"))?.filter(|t| !t.is_incomplete()) else {
            continue;
        };
        let Some(global) = arch.symboltab.get_global_scope() else { break };
        if arch.symboltab.find_container_matching(global, &addr, 1, &Address::new_invalid(), |_| true).is_some() {
            continue;
        }
        let (sid, _) =
            arch.symboltab.add_symbol_mapped(global, &format!("{class}_vtable"), vtbl, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, varnode_flags::typelock | varnode_flags::namelock);
    }
    Ok(())
}
