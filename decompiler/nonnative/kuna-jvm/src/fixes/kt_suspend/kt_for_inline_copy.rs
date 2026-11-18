//! kuna fix `kt_for_inline_copy` (spec 10.16): in Kotlin output, the
//! `$$forInline` copy kotlinc adds next to a suspend function of an object
//! declared in an inline function is not written.
//!
//! When an inline function's body declares an object with a suspend
//! function (kotlinx-coroutines' `filter`, `map` and `collect { .. }`
//! return or pass `object : FlowCollector<T> { override suspend fun
//! emit(..) }`), kotlinc gives the object's class a second method, `emit$$forInline`,
//! the function compiled without its state machine and with
//! `InlineMarker`s: the template its inliner copies into the class it
//! regenerates at each call site. No code calls it and the source does not
//! declare it. Vineflower writes it as another function of the object, with
//! the continuation parameter, the markers and the continuation class
//! instantiated in its first statement, which Kotlin cannot compile. With
//! the fix a method `f$$forInline` of a class that also declares `f` with the
//! same descriptor is not written, as the plugin already leaves out the
//! other members kotlinc generates (`f$default`).

use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const SUFFIX: &str = "$$forInline";

/// Whether `mt`, a method of `cl`, is the `$$forInline` copy of another of
/// its methods.
pub fn is_copy(cl: StructClass, mt: StructMethod) -> bool {
    let name = mt.get_name();
    let Some(original) = name.strip_suffix(SUFFIX).filter(|n| !n.is_empty()) else {
        return false;
    };
    cl.get_method(original, &mt.get_descriptor()).is_some()
}
