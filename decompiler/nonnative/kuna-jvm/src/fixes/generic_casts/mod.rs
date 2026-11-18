//! Fix family `generic_casts` (Java output): the unchecked casts and type
//! arguments a returned, assigned or passed generic value needs.

pub(crate) mod argument_generic_cast;
pub(crate) mod bounded_inference_cast;
pub(crate) mod disjoint_generic_cast;
pub(crate) mod external_supertype_arguments;
pub(crate) mod raw_receiver_erasure;
pub(crate) mod receiver_type_arguments;
pub(crate) mod unchecked_call_result_cast;
pub(crate) mod unchecked_generic_cast;
