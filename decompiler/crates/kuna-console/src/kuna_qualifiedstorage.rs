//! Keep qualified storage declarations bound to the same target as prototypes.
use crate::engine::ConsoleProgram;
use crate::interface::{IfaceError, IfaceResult};
use kuna_decomp::fspec::PrototypePieces;
use std::collections::BTreeMap;

pub(crate) fn update(
    program: &mut ConsoleProgram,
    pending: &mut BTreeMap<String, PrototypePieces>,
    addressed: &mut BTreeMap<kuna_base::address::Address, PrototypePieces>,
    selector: &str,
    merge: impl FnOnce(&mut PrototypePieces) -> IfaceResult<()>,
) -> IfaceResult<()> {
    let target = crate::assertions::resolve_proto_target(program, selector)
        .map_err(IfaceError::execution)?;
    let existing = match &target {
        crate::assertions::ProtoTarget::At(addr, _) => addressed.get(addr),
        crate::assertions::ProtoTarget::Named(_) => None,
    }
    .or_else(|| pending.get(target.name()));
    let mut pieces = existing.cloned().unwrap_or(PrototypePieces {
        name: target.name().to_owned(),
        first_var_arg_slot: -1,
        ..Default::default()
    });
    merge(&mut pieces)?;
    crate::assertions::park_proto_pieces(program, &target, &mut pieces);
    // An input-only or return-only declaration must retain the existing
    // partial-prototype semantics. A full declaration can refresh its TypeCode.
    if pieces.outtype.is_some() {
        let model = program
            .arch()
            .function_prototype_model(target.name())
            .cloned();
        crate::assertions::lock_prototype_on_symbol(program, &target, &pieces, model.as_ref());
    }
    remember(pending, addressed, &target, pieces);
    Ok(())
}

/// Retain an exact entry through the console's later Funcdata rebuilds.
pub(crate) fn remember(
    named: &mut BTreeMap<String, PrototypePieces>,
    addressed: &mut BTreeMap<kuna_base::address::Address, PrototypePieces>,
    target: &crate::assertions::ProtoTarget,
    pieces: PrototypePieces,
) {
    match target {
        crate::assertions::ProtoTarget::At(addr, _) => {
            addressed.insert(addr.clone(), pieces);
        }
        crate::assertions::ProtoTarget::Named(name) => {
            named.insert(name.clone(), pieces);
        }
    }
}
