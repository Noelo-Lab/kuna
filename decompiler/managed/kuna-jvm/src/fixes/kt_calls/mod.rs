//! Fix family `kt_calls` (Kotlin output): calls through facades, extensions,
//! JVM names, synthetic accessors and objects.

pub(crate) mod kt_explicit_type_arguments;
pub(crate) mod kt_extension_call;
pub(crate) mod kt_facade_call;
pub(crate) mod kt_jvm_name_call;
pub(crate) mod kt_mapped_java_member;
pub(crate) mod kt_object_instance;
pub(crate) mod kt_object_monitor_call;
pub(crate) mod kt_synthetic_accessor_call;
