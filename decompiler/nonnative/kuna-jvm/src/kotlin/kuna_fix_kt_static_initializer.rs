//! kuna fix `kt_static_initializer` (spec 10.16): what is left of a static
//! initializer is written as Kotlin's `init` block where Kotlin has one.
//!
//! The static fields of a Kotlin `object` are its properties, and those of a
//! class with a companion object are the companion's; kotlinc compiles their
//! initializers and `init` blocks into `<clinit>`. `InitializerProcessor`
//! moves each initializer it can into its property; what it cannot (a
//! `try`, a loop, a value built over several statements) stays in
//! `<clinit>`, which Vineflower's Kotlin writer printed as `@JvmStatic fun {`,
//! no Kotlin at all. With the fix that code is an `init` block: in the
//! object itself, or, moved out of the class, in its companion object, after
//! the companion's properties, as Vineflower's Java writer puts a `static`
//! block after the field initializers. A file class has no `init`; its
//! remainder is left as it was.

use std::cell::RefCell;

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::code_constants::ACC_SYNTHETIC;
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::k_element::KElement;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::ClassKind;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;

thread_local! {
    static MOVED: RefCell<Vec<(String, TextBuffer)>> = const { RefCell::new(Vec::new()) };
}

/// Whether `cl` is an `object` or a companion object, whose static
/// initializer is its own `init` block.
pub fn is_object(cl: StructClass) -> bool {
    let Some(KElement::Class(cls)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return false;
    };
    matches!(
        Flags::CLASS_KIND.get(cls.flags()),
        Some(ClassKind::Object | ClassKind::CompanionObject)
    )
}

/// Whether the static initializer of `cl` is written as an `init` block:
/// `cl` is an object, or `companion` (its companion object's node) takes it.
pub fn writes_init_block(cl: StructClass, companion: Option<ClassNode>) -> bool {
    is_object(cl) || companion.is_some()
}

/// Whether the member class `companion` of the class `outer` is written (as
/// the class body writes its member classes), so a block moved to it is not lost.
pub fn is_written(companion: ClassNode, outer: ClassWrapper) -> bool {
    let (cl, access) = companion.with(|d| (d.class_struct, d.access));
    let synthetic = (access & ACC_SYNTHETIC) != 0 || cl.is_synthetic();
    !(synthetic && DecompilerContext::get_option(IFernflowerPreferences::REMOVE_SYNTHETIC))
        && !outer.get_hidden_members().borrow().contains(&cl.qualified_name())
}

/// Keeps the `init` block `text` of an outer class for its companion `companion`.
pub fn move_to(companion: ClassNode, text: TextBuffer) {
    let name = companion.with(|d| d.class_struct).qualified_name();
    MOVED.with(|m| m.borrow_mut().push((name, text)));
}

/// The `init` block moved to the companion object `cl`, if any.
pub fn take(cl: StructClass) -> Option<TextBuffer> {
    let name = cl.qualified_name();
    MOVED.with(|m| {
        let mut moved = m.borrow_mut();
        let pos = moved.iter().position(|(n, _)| *n == name)?;
        Some(moved.remove(pos).1)
    })
}
