//! Android DEX input for kuna's JVM engine: each class of a DEX file becomes
//! a JVM class file in memory, which the engine loads like a jar entry.
//!
//! Written from the published DEX format and Dalvik bytecode specifications.
//! [`Dex::parse`] validates the header and id tables; [`Dex::translate`]
//! turns one class definition into class-file bytes without reading any other
//! class's code, so callers translate in parallel or on demand. The input is
//! untrusted: every offset and count is checked, and a malformed file or
//! method is an error, never a panic.

mod backport;
mod class;
mod classfile;
mod code;
mod debug;
mod dex;
mod emit;
mod insn;
mod lambda;
mod nest;
mod reader;
mod record;
mod region;

pub use class::ERROR_ATTRIBUTE;

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

/// How classes are written.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// The lowest class-file major version to write; a class needing more
    /// (`invokedynamic`, method handles, interface code) gets what it needs.
    pub min_version: u16,
    /// Keep classes at version 50 where the class itself allows, so the JVM's
    /// type-inferring verifier checks every method: library interface methods
    /// are then called through `Methodref`, which verifies but fails if run,
    /// and d8's lambda classes are instantiated as d8 wrote them.
    pub verifiable: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            min_version: 50,
            verifiable: false,
        }
    }
}

/// One translated class.
#[derive(Clone, Debug)]
pub struct Translated {
    /// Internal name (`com/a/B`).
    pub name: String,
    pub bytes: Vec<u8>,
    /// `name(descriptor): reason` for each method whose code failed.
    pub failures: Vec<String>,
    /// d8 wrote the class for its desugaring and every use of it is written
    /// back as the JDK call it stands for (backports): a loader leaves it out.
    pub omit: bool,
}

/// Whether `head` starts a DEX file.
pub fn is_dex(head: &[u8]) -> bool {
    head.len() >= 4 && &head[..4] == b"dex\n"
}

impl Dex<'_> {
    /// Translates class definition `idx` to class-file bytes.
    pub fn translate(&self, idx: u32, opts: &Options) -> Result<Translated> {
        self.translate_with(idx, opts, &[])
    }

    /// Translates class definition `idx`, its calls into d8's synthetic
    /// classes of the other DEX files of the input (`peers`) undone too.
    pub fn translate_with(&self, idx: u32, opts: &Options, peers: &[Dex]) -> Result<Translated> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| class::translate(self, idx, opts, peers)))
            .unwrap_or_else(|_| Err(err("internal error in kuna-dex".to_string())))
    }

    /// The internal name of class definition `idx`.
    pub fn class_name(&self, idx: u32) -> Result<String> {
        let desc = self.type_desc(self.class_def(idx)?.class)?;
        let name = desc
            .strip_prefix(b"L")
            .and_then(|d| d.strip_suffix(b";"))
            .unwrap_or(desc);
        Ok(mutf8_to_string(name))
    }

    /// Translates every class on up to `threads` threads, in definition order.
    pub fn translate_all(&self, opts: &Options, threads: usize) -> Vec<Result<Translated>> {
        self.translate_all_with(opts, threads, &[])
    }

    /// [`Dex::translate_all`] with the other DEX files of the input.
    pub fn translate_all_with(&self, opts: &Options, threads: usize, peers: &[Dex]) -> Vec<Result<Translated>> {
        let n = self.class_count() as usize;
        let threads = threads.clamp(1, 64).min(n.div_ceil(64).max(1));
        if threads == 1 {
            return (0..n as u32).map(|i| self.translate_with(i, opts, peers)).collect();
        }
        let next = std::sync::atomic::AtomicUsize::new(0);
        let mut parts: Vec<(usize, Result<Translated>)> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|_| {
                    scope.spawn(|| {
                        let mut out = Vec::new();
                        loop {
                            let start = next.fetch_add(32, std::sync::atomic::Ordering::Relaxed);
                            if start >= n {
                                break;
                            }
                            for i in start..(start + 32).min(n) {
                                out.push((i, self.translate_with(i as u32, opts, peers)));
                            }
                        }
                        out
                    })
                })
                .collect();
            handles
                .into_iter()
                .flat_map(|h| h.join().unwrap_or_default())
                .collect()
        });
        parts.sort_by_key(|p| p.0);
        parts.into_iter().map(|p| p.1).collect()
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
