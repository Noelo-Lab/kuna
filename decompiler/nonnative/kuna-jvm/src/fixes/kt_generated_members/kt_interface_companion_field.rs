//! kuna fix `kt_interface_companion_field`: an interface's companion field is
//! hidden like a class's.
//!
//! A class's static `Companion` field is initialized `new Companion(null)`,
//! which `parse_with_companion` takes for the companion object's instance and
//! hides. An interface's companion keeps its instance in its own `$$INSTANCE`
//! field, and kotlinc initializes the interface's field from it
//! (`Companion = Companion.$$INSTANCE`); the plugin did not hide that one, and
//! the Kotlin writer declared it in Java form inside the interface:
//! `Interceptor.Companion Companion = Interceptor.Companion.$$INSTANCE;` (okhttp
//! `Interceptor`, kotlinx-coroutines `Job.Key`, `Channel.Factory`). With the
//! fix such a field is the companion's instance too: the `companion object`
//! the writer prints declares it.

use crate::exprent::Exprent;
use crate::struct_class::StructClass;

/// Whether `initializer` reads the static `$$INSTANCE` field of `companion`, an
/// interface companion's instance.
pub fn is_instance_read(initializer: &Exprent, companion: StructClass) -> bool {
    let Some(field) = initializer.try_as_field() else {
        return false;
    };
    let descriptor = format!("L{};", companion.qualified_name());
    field.is_static()
        && field.get_instance().is_none()
        && field.get_classname() == companion.qualified_name()
        && field.get_name() == "$$INSTANCE"
        && field.get_descriptor().descriptor_string == descriptor
        && companion.has_field("$$INSTANCE", &descriptor)
}
