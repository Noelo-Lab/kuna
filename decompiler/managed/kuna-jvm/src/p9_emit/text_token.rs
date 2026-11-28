//! Port of `org/jetbrains/java/decompiler/util/token/TextToken.java`, reduced to
//! the one kind kuna reads: a zero-width mark of `kuna_spans`, which the buffer
//! moves through every later edit as Vineflower moves its tokens.

#[derive(Clone, Copy)]
pub struct TextToken {
    pub start: i32,
    pub id: u32,
}

impl TextToken {
    pub fn shift(&mut self, amount: i32) {
        self.start += amount;
    }

    pub fn get_start(&self) -> i32 {
        self.start
    }
}
