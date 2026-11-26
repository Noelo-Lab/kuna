//! Android DEX input for kuna's JVM engine: each class of a DEX file becomes
//! a JVM class file in memory, which the engine loads like a jar entry.
//!
//! Written from the published DEX format and Dalvik bytecode specifications.
//! [`Dex::parse`] validates the header and id tables; [`Dex::translate`]
//! turns one class definition into class-file bytes without reading any other
//! class's code, so callers translate in parallel or on demand. The input is
//! untrusted: every offset and count is checked, and a malformed file or
//! method is an error, never a panic.

mod debug;
mod dex;
mod insn;
mod reader;

pub use debug::{DebugInfo, Local};
pub use dex::{
    Annotation, AnnotationsDirectory, ClassData, ClassDef, CodeItem, Dex, EncodedAnnotation,
    EncodedField, EncodedMethod, FieldId, MethodHandle, MethodId, Proto, Try, Value, NO_INDEX,
};
pub use insn::{decode, fill_array, packed_switch, sparse_switch, Insn};
pub use reader::mutf8_to_string;

/// A translation or parse failure, with a message naming what was wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn err(message: String) -> Error {
    Error(message)
}

/// Whether `head` starts a DEX file.
pub fn is_dex(head: &[u8]) -> bool {
    head.len() >= 4 && &head[..4] == b"dex\n"
}

impl Dex<'_> {
    /// The internal name of class definition `idx`.
    pub fn class_name(&self, idx: u32) -> Result<String> {
        let desc = self.type_desc(self.class_def(idx)?.class)?;
        let name = desc
            .strip_prefix(b"L")
            .and_then(|d| d.strip_suffix(b";"))
            .unwrap_or(desc);
        Ok(mutf8_to_string(name))
    }

    /// Parses one method's debug info against its code item.
    pub fn debug_info(
        &self,
        code: &CodeItem,
        params: &[(u32, u32)],
        this_name: u32,
    ) -> Result<DebugInfo> {
        if code.debug_off == 0 {
            return Ok(DebugInfo::default());
        }
        debug::parse(
            self.data,
            code.debug_off,
            (code.insns.len() / 2) as u32,
            params,
            this_name,
        )
    }
}
