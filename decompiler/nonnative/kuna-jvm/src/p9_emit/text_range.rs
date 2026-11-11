//! Port of `org/jetbrains/java/decompiler/util/token/TextRange.java`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextRange {
    pub start: i32,
    pub length: i32,
}

impl TextRange {
    pub fn new(start: i32, length: i32) -> TextRange {
        TextRange { start, length }
    }

    pub fn get_end(&self) -> i32 {
        self.start + self.length
    }
}
