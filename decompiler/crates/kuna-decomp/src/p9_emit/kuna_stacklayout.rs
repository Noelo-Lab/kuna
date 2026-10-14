//! Synthetic frame views must preserve byte offsets and exact storage extents.

use crate::dtype::Datatype;

pub(crate) fn suffix(datatype: &Datatype) -> String {
    if !datatype.get_name().starts_with("stack_views_")
        && !datatype.get_name().starts_with("stack_slice_")
    {
        return String::new();
    }
    let mut natural = 1;
    let mut displaced = false;
    for index in 0..datatype.num_depend() {
        if let Some(field) = datatype.get_field(index) {
            let alignment = field.field_type.get_alignment().max(1);
            natural = natural.max(alignment);
            displaced |= field.offset % alignment != 0;
        }
    }
    let alignment = datatype.get_alignment().max(1);
    if displaced || alignment != natural || datatype.get_size() % natural != 0 {
        format!(" __attribute__((packed, aligned({alignment})))")
    } else {
        String::new()
    }
}
