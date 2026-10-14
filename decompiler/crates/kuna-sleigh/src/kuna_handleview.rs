//! Select handle fields without copying unrelated address-space references.

use std::borrow::Cow;
use std::rc::Rc;

use kuna_base::space::AddrSpace;

use crate::context::FixedHandle;

#[inline]
pub(crate) fn resolved_space(handle: Cow<'_, FixedHandle>) -> Option<Rc<AddrSpace>> {
    match handle {
        Cow::Borrowed(handle) => {
            if handle.offset_space.is_none() { handle.space.clone() } else { handle.temp_space.clone() }
        }
        Cow::Owned(handle) => {
            if handle.offset_space.is_none() { handle.space } else { handle.temp_space }
        }
    }
}

#[inline]
pub(crate) fn export_space(handle: Cow<'_, FixedHandle>) -> Option<Rc<AddrSpace>> {
    match handle {
        Cow::Borrowed(handle) => handle.space.clone(),
        Cow::Owned(handle) => handle.space,
    }
}

#[inline]
pub(crate) fn export_offset(handle: Cow<'_, FixedHandle>, destination: &mut FixedHandle) {
    destination.offset_offset = handle.offset_offset;
    destination.offset_size = handle.offset_size;
    destination.temp_offset = handle.temp_offset;
    let (offset_space, temp_space) = match handle {
        Cow::Borrowed(handle) => (handle.offset_space.clone(), handle.temp_space.clone()),
        Cow::Owned(handle) => (handle.offset_space, handle.temp_space),
    };
    destination.offset_space = offset_space;
    destination.temp_space = temp_space;
}
