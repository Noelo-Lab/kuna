//! Port of `org/vineflower/kotlin/expr/KExitExprent.java`.

use crate::exit_exprent::ExitExprent;
use crate::exprent::Exprent;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

pub struct KExitExprentData {
    pub lambda_name: Option<String>,
}

kotlin_exprent!(pub KExitExprent: ExitExprent, Exit(KExitExprentData),
                is_k_exit, as_k_exit, try_as_k_exit);

impl KExitExprent {
    pub fn set_lambda_name(self, name: Option<String>) {
        self.kd_mut(|d| d.lambda_name = name)
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KExitExprent.toJava")
    }

    pub fn copy(self) -> Exprent {
        unported!("KExitExprent.copy")
    }
}
