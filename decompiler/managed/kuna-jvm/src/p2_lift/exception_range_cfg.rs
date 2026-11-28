//! Port of `org/jetbrains/java/decompiler/code/cfg/ExceptionRangeCFG.java`.

use crate::basic_block::BasicBlock;
use crate::decompiler_context::DecompilerContext;
use crate::rt::JList;
use crate::{arena_identity, arena_object};

pub struct ExceptionRangeCFGData {
    pub protected_range: JList<BasicBlock>,
    pub handler: BasicBlock,
    pub exception_types: Option<Vec<String>>,
}

arena_object!(pub ExceptionRangeCFG => ExceptionRangeCFGData, Class);
arena_identity!(ExceptionRangeCFG);

impl ExceptionRangeCFG {
    /// Java: `ExceptionRangeCFG(List<BasicBlock>, BasicBlock, List<String>)`; the range list is aliased.
    pub fn new(
        protected_range: JList<BasicBlock>,
        handler: BasicBlock,
        exception_type: Option<Vec<String>>,
    ) -> ExceptionRangeCFG {
        ExceptionRangeCFG::alloc(ExceptionRangeCFGData {
            protected_range,
            handler,
            exception_types: exception_type,
        })
    }

    /// Java: `ExceptionRangeCFG(List<BasicBlock>, BasicBlock, String)`; the range list is aliased.
    pub fn new_single(
        protected_range: JList<BasicBlock>,
        handler: BasicBlock,
        exception_type: Option<String>,
    ) -> ExceptionRangeCFG {
        ExceptionRangeCFG::alloc(ExceptionRangeCFGData {
            protected_range,
            handler,
            exception_types: exception_type.map(|t| vec![t]),
        })
    }

    pub fn is_circular(self) -> bool {
        self.get_protected_range().contains(&self.get_handler())
    }

    pub fn get_handler(self) -> BasicBlock {
        self.with(|d| d.handler)
    }

    pub fn set_handler(self, handler: BasicBlock) {
        self.with_mut(|d| d.handler = handler)
    }

    pub fn get_protected_range(self) -> JList<BasicBlock> {
        self.with(|d| d.protected_range.clone())
    }

    pub fn get_exception_types(self) -> Option<Vec<String>> {
        self.with(|d| d.exception_types.clone())
    }

    pub fn add_exception_type(self, exception_type: Option<String>) {
        self.with_mut(|d| {
            if d.exception_types.is_none() {
                return;
            }

            match exception_type {
                None => d.exception_types = None,
                Some(t) => d.exception_types.as_mut().expect("types").push(t),
            }
        })
    }

    pub fn get_unique_exceptions_string(self) -> Option<String> {
        self.with(|d| {
            d.exception_types.as_ref().map(|types| {
                let mut distinct: Vec<&String> = Vec::new();
                for t in types {
                    if !distinct.contains(&t) {
                        distinct.push(t);
                    }
                }
                distinct
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(":")
            })
        })
    }
}

impl std::fmt::Display for ExceptionRangeCFG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let new_line_separator = DecompilerContext::get_new_line_separator().to_string();
        let mut buf = String::new();

        buf.push_str("exceptionType:");

        match self.get_exception_types() {
            None => buf.push_str(" null"),
            Some(types) => {
                for exception_type in types {
                    buf.push(' ');
                    buf.push_str(&exception_type);
                }
            }
        }

        buf.push_str(&new_line_separator);

        buf.push_str(&format!("handler: {}", self.get_handler().get_id()));
        buf.push_str(&new_line_separator);
        buf.push_str("range: ");
        for block in self.get_protected_range().iter() {
            buf.push_str(&format!("{} ", block.get_id()));
        }
        buf.push_str(&new_line_separator);

        f.write_str(&buf)
    }
}
