//! Preserve numeric signedness when extending Rust's unsigned text storage.

use crate::dtype::{type_metatype, Datatype};

pub(crate) fn signed_text_input(input: &Datatype, output: &Datatype) -> Option<&'static str> {
    if input.get_metatype() != type_metatype::TYPE_INT
        || !input.is_char_print()
        || !input.is_core_type()
        || output.get_size() <= input.get_size()
        || !matches!(output.get_size(), 2 | 4 | 8 | 16)
        || !matches!(output.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
    {
        return None;
    }
    match input.get_size() {
        1 => Some("i8"),
        2 => Some("i16"),
        4 => Some("i32"),
        8 => Some("i64"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::flags;

    #[test]
    fn only_signed_text_storage_needs_reinterpretation() {
        let mut input = Datatype::new_with_align(1, -1, type_metatype::TYPE_INT);
        input.flags |= flags::chartype | flags::coretype;
        let mut output = Datatype::new_with_align(8, -1, type_metatype::TYPE_INT);
        assert_eq!(signed_text_input(&input, &output), Some("i8"));
        output.metatype = type_metatype::TYPE_PTR;
        assert_eq!(signed_text_input(&input, &output), None);
        output.metatype = type_metatype::TYPE_FLOAT;
        assert_eq!(signed_text_input(&input, &output), None);
        output.metatype = type_metatype::TYPE_UINT;
        assert_eq!(signed_text_input(&input, &output), Some("i8"));
        input.flags &= !flags::coretype;
        assert_eq!(signed_text_input(&input, &output), None);
        input.flags |= flags::coretype;
        input.metatype = type_metatype::TYPE_UINT;
        assert_eq!(signed_text_input(&input, &output), None);
    }
}
