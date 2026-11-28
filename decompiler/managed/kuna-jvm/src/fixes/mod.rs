//! kuna's fixes of Vineflower's defects, one module per family (a folder for a
//! large family), called from the ported files; what each fix does is its row of
//! the fix table in `docs/jvm.md`, the mechanisms the fixes share are spec
//! section 11.16.

pub(crate) mod anonymous_classes;
pub(crate) mod asserts;
pub(crate) mod class_writing;
pub(crate) mod constructor_calls;
pub(crate) mod control_flow;
pub(crate) mod declaration_scope;
pub(crate) mod exception_ranges;
pub(crate) mod failure_comments;
pub(crate) mod finally_sync;
pub(crate) mod generic_casts;
pub(crate) mod generics;
pub(crate) mod lambdas;
pub(crate) mod nested_class_vars;
pub(crate) mod numeric_types;
pub(crate) mod overloads;
pub(crate) mod patterns;
pub(crate) mod stack_var_inlining;
pub(crate) mod switches;
pub(crate) mod twr;
pub(crate) mod type_annotations;
pub(crate) mod util;
pub(crate) mod var_merge;

pub(crate) mod kt_anonymous_objects;
pub(crate) mod kt_calls;
pub(crate) mod kt_compiler_idioms;
pub(crate) mod kt_constructors;
pub(crate) mod kt_declarations;
pub(crate) mod kt_default_args;
pub(crate) mod kt_delegation_calls;
pub(crate) mod kt_extensions;
pub(crate) mod kt_for_loops;
pub(crate) mod kt_generated_members;
pub(crate) mod kt_initializers;
pub(crate) mod kt_java_syntax;
pub(crate) mod kt_jumps;
pub(crate) mod kt_names;
pub(crate) mod kt_null_checks;
pub(crate) mod kt_operators;
pub(crate) mod kt_properties;
pub(crate) mod kt_suspend;
pub(crate) mod kt_types;
pub(crate) mod kt_util;
pub(crate) mod kt_when;
