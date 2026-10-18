//! Explicit return storage suppresses model-generated hidden return arguments.
use crate::dtype::TypeFactory;
use crate::fspec::PrototypePieces;
use kuna_base::error::KunaResult;
use std::borrow::Cow;

pub(crate) fn input_assignment<'a>(
    pieces: &'a PrototypePieces,
    types: &dyn TypeFactory,
) -> KunaResult<Cow<'a, PrototypePieces>> {
    if pieces.output_storage.is_none() {
        return Ok(Cow::Borrowed(pieces));
    }
    let mut inputs = pieces.clone();
    inputs.outtype = Some(types.get_type_void()?);
    Ok(Cow::Owned(inputs))
}
