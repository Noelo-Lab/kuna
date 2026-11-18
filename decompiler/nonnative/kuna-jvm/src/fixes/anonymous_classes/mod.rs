//! Fix family `anonymous_classes` (Java output): finding, nesting and
//! constructing anonymous and local classes.

pub(crate) mod anonymous_class_created_elsewhere;
pub(crate) mod anonymous_static_qualifier;
pub(crate) mod constructor_signature_alignment;
pub(crate) mod local_class_anonymous_subclass;
pub(crate) mod unlisted_anonymous_class;
