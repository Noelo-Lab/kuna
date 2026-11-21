//! Port of `org/vineflower/kotlin/util/KEnumProcessor.java`.

use crate::class_wrapper::ClassWrapper;
use crate::enum_processor::EnumProcessor;
use crate::interpreter_util::InterpreterUtil;

pub struct KEnumProcessor;

impl KEnumProcessor {
    const ENUM_ENTRIES: &'static str = "Lkotlin/enums/EnumEntries;";

    pub fn clear_enum(wrapper: ClassWrapper) {
        EnumProcessor::clear_enum(wrapper);

        for fd in wrapper.get_class_struct().get_fields().to_vec() {
            let descriptor = fd.get_descriptor();
            if fd.is_synthetic() && descriptor == KEnumProcessor::ENUM_ENTRIES {
                wrapper
                    .get_hidden_members()
                    .borrow_mut()
                    .add(InterpreterUtil::make_unique_key(
                        &fd.get_name(),
                        &descriptor,
                    ));
            }
        }
    }
}
