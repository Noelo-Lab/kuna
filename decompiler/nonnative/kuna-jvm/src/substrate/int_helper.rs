//! Port of `org/jetbrains/java/decompiler/util/IntHelper.java`.

pub struct IntHelper;

impl IntHelper {
    fn special_value(value: i32) -> Option<&'static str> {
        match value as u32 {
            0xFF => Some("0xFF"),
            0xFF00 => Some("0xFF00"),
            0xFF_0000 => Some("0xFF0000"),
            0xFF00_0000 => Some("0xFF000000"),
            _ => None,
        }
    }

    /// The color-mask constants print in hex; everything else as `String.valueOf(int)`.
    pub fn adjusted_int_representation(value: i32) -> String {
        if let Some(special) = IntHelper::special_value(value) {
            return special.to_string();
        }
        value.to_string()
    }
}
