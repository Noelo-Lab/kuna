//! Detect shared temporary exports between constructor operands
//! (`checkLocalExports`, Ghidra's slgh_compile.cc:2217).

use std::collections::BTreeMap;

use kuna_base::space::spacetype;
use kuna_sleigh::semantics::ConstType;
use kuna_sleigh::sleighbase::SleighBase;
use kuna_sleigh::slghsymbol::{Constructor, SymbolKind};

use crate::slgh_compile::SymbolId;

#[expect(
    clippy::disallowed_types,
    reason = "Only visited membership is observed; operand and constructor order drive traversal"
)]
type VisitedSymbols = std::collections::HashSet<SymbolId>;

pub(crate) fn find_collision(base: &SleighBase, ct: &Constructor) -> Option<(SymbolId, SymbolId)> {
    let template = base.templates().get(ct.get_templ()?)?;
    let operands = ct.get_operands();
    if operands.len() < 2 || template.build_only() {
        return None;
    }
    let mut owners = BTreeMap::new();
    let mut visited = VisitedSymbols::new();
    let mut offsets = Vec::new();
    for (index, &operand) in operands.iter().enumerate() {
        visited.clear();
        offsets.clear();
        collect_symbol(base, operand, &mut visited, &mut offsets);
        for &offset in &offsets {
            let owner = *owners.entry(offset).or_insert(index);
            if owner != index {
                return Some((operands[owner], operand));
            }
        }
    }
    None
}

fn collect_symbol(
    base: &SleighBase,
    id: SymbolId,
    visited: &mut VisitedSymbols,
    offsets: &mut Vec<u64>,
) {
    if !visited.insert(id) {
        return;
    }
    let Some(symbol) = base.symtab().find_symbol_by_id(id) else {
        return;
    };
    match symbol.kind() {
        SymbolKind::Varnode(varnode) => {
            let fixed = varnode.get_fixed_varnode();
            if fixed
                .space
                .as_ref()
                .is_some_and(|space| space.get_type() == spacetype::IPTR_INTERNAL)
            {
                offsets.push(fixed.offset);
            }
        }
        SymbolKind::Operand(operand) => {
            if let Some(definition) = operand.get_defining_symbol() {
                collect_symbol(base, definition, visited, offsets);
            }
        }
        SymbolKind::Subtable(table) => {
            for index in 0..table.get_num_constructors() {
                let ct = table
                    .get_constructor(index as u32)
                    .expect("constructor index in table");
                collect_constructor(base, ct, visited, offsets);
            }
        }
        _ => {}
    }
}

fn collect_constructor(
    base: &SleighBase,
    ct: &Constructor,
    visited: &mut VisitedSymbols,
    offsets: &mut Vec<u64>,
) {
    let Some(handle) = ct
        .get_templ()
        .and_then(|h| base.templates().get(h))
        .and_then(|t| t.get_result())
    else {
        return;
    };
    if handle.get_space().is_const_space() {
        return;
    }
    if handle.get_ptr_space().get_type() != ConstType::Real {
        if handle.get_temp_space().is_unique_space() {
            offsets.push(handle.get_temp_offset().get_real());
        }
    } else if handle.get_space().is_unique_space() {
        offsets.push(handle.get_ptr_offset().get_real());
    } else if handle.get_space().get_type() == ConstType::Handle {
        let operand = ct
            .get_operand(handle.get_space().get_handle_index())
            .expect("export operand in constructor");
        collect_symbol(base, operand, visited, offsets);
    }
}
