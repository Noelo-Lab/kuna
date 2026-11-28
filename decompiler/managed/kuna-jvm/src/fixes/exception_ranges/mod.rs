//! Fix family `exception_ranges`: exception ranges and the handler copies
//! range splitting makes.

pub(crate) mod already_caught;
pub(crate) mod handler_copy_shared_body;
pub(crate) mod nested_handler_copies;
pub(crate) mod overlapping_ranges;
