//! Parameter assertions update the prototype and replay as input storage.

use kuna_base::address::Address;
use kuna_base::types::{int4, uint4};
use kuna_decomp::database::{symbol_category, SymbolId};
use kuna_decomp::dtype::Datatype;
use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::varmap::ScopeLocal;
use std::rc::Rc;

pub(crate) struct Binding {
    slot: int4,
    pieces: ParameterPieces,
}

pub(crate) fn prepare(
    fd: &Funcdata,
    symbol: SymbolId,
    retype: Option<&Rc<Datatype>>,
) -> Result<Option<Binding>, String> {
    let scope = fd.get_scope_local().ok_or("Function has no local scope")?;
    let symbol_info = scope.database().symbol(symbol);
    if symbol_info.get_category() != symbol_category::FUNCTION_PARAMETER {
        return Ok(None);
    }
    let slot = int4::from(symbol_info.get_category_index());
    if !fd.get_func_proto().has_store() {
        return Err("Parameter prototype has no storage".to_string());
    }
    let parameter = fd
        .get_func_proto()
        .get_param(slot)
        .ok_or_else(|| format!("No prototype parameter in slot {slot}"))?;
    let address = parameter.get_address();
    let size = parameter.get_size();
    if !scope
        .symbol_storage(symbol)
        .iter()
        .any(|(at, width)| *at == address && *width == size)
    {
        return Err(format!("Parameter storage no longer matches slot {slot}"));
    }
    if retype.is_some_and(|ty| ty.get_size() != size) {
        return Err(format!("Parameter type must occupy {size} bytes"));
    }
    Ok(Some(Binding {
        slot,
        pieces: ParameterPieces {
            addr: address,
            type_: retype.cloned().or_else(|| parameter.get_type().cloned()),
            flags: parameter.get_flags(),
        },
    }))
}

pub(crate) fn apply(fd: &mut Funcdata, symbol: SymbolId, mut binding: Binding) {
    let name = fd
        .get_scope_local()
        .unwrap()
        .database()
        .symbol(symbol)
        .name
        .clone();
    binding.pieces.flags |= parameter_pieces_flags::TYPELOCK;
    if !name.is_empty() {
        binding.pieces.flags |= parameter_pieces_flags::NAMELOCK;
    }
    fd.get_func_proto_mut()
        .set_param(binding.slot, &name, &binding.pieces);
}

fn touched(fd: &Funcdata) -> Vec<SymbolId> {
    let Some(scope) = fd.get_scope_local() else {
        return Vec::new();
    };
    fd.kuna_directive_symbols()
        .iter()
        .map(|directive| directive.symbol)
        .filter(|&symbol| scope.symbol_category(symbol) == symbol_category::FUNCTION_PARAMETER)
        .collect()
}

pub(crate) fn carried(
    fd: &Funcdata,
    original: &[(int4, String, ParameterPieces)],
) -> Vec<(int4, String, ParameterPieces)> {
    if touched(fd).is_empty() {
        return original.to_vec();
    }
    let mut inputs = original.to_vec();
    let prototype = fd.get_func_proto();
    for slot in 0..prototype.num_params() {
        let Some(parameter) = prototype.get_param(slot) else {
            continue;
        };
        inputs.retain(|(index, _, _)| *index != slot);
        inputs.push((
            slot,
            parameter.get_name().to_owned(),
            ParameterPieces {
                addr: parameter.get_address(),
                type_: parameter.get_type().cloned(),
                flags: parameter.get_flags(),
            },
        ));
    }
    inputs.sort_by_key(|(slot, _, _)| *slot);
    inputs
}

pub(crate) fn exclude_from_locals(
    fd: &Funcdata,
    symbols: Vec<(String, Rc<Datatype>, Address, uint4)>,
) -> Vec<(String, Rc<Datatype>, Address, uint4)> {
    let parameters = touched(fd);
    let Some(scope) = fd.get_scope_local() else {
        return symbols;
    };
    symbols
        .into_iter()
        .filter(|(name, datatype, address, _)| {
            !is_parameter(scope, &parameters, name, datatype, address)
        })
        .collect()
}

pub(crate) fn exclude_from_usepoints(
    fd: &Funcdata,
    symbols: Vec<(String, Rc<Datatype>, Address, uint4, Address, bool)>,
) -> Vec<(String, Rc<Datatype>, Address, uint4, Address, bool)> {
    let parameters = touched(fd);
    let Some(scope) = fd.get_scope_local() else {
        return symbols;
    };
    symbols
        .into_iter()
        .filter(|(name, datatype, address, _, _, _)| {
            !is_parameter(scope, &parameters, name, datatype, address)
        })
        .collect()
}

fn is_parameter(
    scope: &ScopeLocal,
    parameters: &[SymbolId],
    name: &str,
    datatype: &Rc<Datatype>,
    address: &Address,
) -> bool {
    parameters.iter().any(|&parameter| {
        let symbol = scope.database().symbol(parameter);
        symbol.name == name
            && symbol
                .dtype
                .as_ref()
                .is_some_and(|ty| Rc::ptr_eq(ty, datatype))
            && scope
                .symbol_storage(parameter)
                .iter()
                .any(|(at, _)| at == address)
    })
}
