//! Port of `org/jetbrains/java/decompiler/code/ExceptionTable.java`.

use crate::exception_handler::ExceptionHandler;

#[derive(Clone, Debug, Default)]
pub struct ExceptionTable {
    handlers: Vec<ExceptionHandler>,
}

impl ExceptionTable {
    #[allow(non_snake_case)]
    pub fn EMPTY() -> ExceptionTable {
        ExceptionTable::new(Vec::new())
    }

    pub fn new(handlers: Vec<ExceptionHandler>) -> ExceptionTable {
        ExceptionTable { handlers }
    }

    pub fn get_handlers(&self) -> Vec<ExceptionHandler> {
        self.handlers.clone()
    }
}
