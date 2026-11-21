//! The BSim corpus file — signatures on disk.
//!
//! A corpus is a set of signed functions grouped by the executable they came
//! from. Ghidra stores this in an H2, PostgreSQL or Elasticsearch database; kuna
//! writes a single self-describing file, which keeps a corpus portable and lets a
//! run be reproduced from a checked-in artifact.
//!
//! # Format
//!
//! ```text
//! magic        "KBSM"                      # 4 bytes
//! version      u32  (= CORPUS_VERSION)
//! settings     u32                         # the signature settings used
//! language     len-prefixed UTF-8          # the weights file's language family
//! n_execs      u32
//! execs[n]:    name: len-prefixed UTF-8
//!              arch: len-prefixed UTF-8
//!              compiler: len-prefixed UTF-8
//!              md5: 32 ASCII hex chars
//!              n_funcs: u32
//!              funcs[m]: name: len-prefixed UTF-8
//!                        address: u64
//!                        flags: u8   (bit 0 unimplemented, bit 1 bad data)
//!                        n_features: u32
//!                        features: n_features × u32
//! ```
//!
//! All integers are **little-endian**; lengths are `u32` byte counts. Records are
//! written in insertion order, so a corpus round-trips byte-identically and a
//! rebuilt corpus diffs cleanly.
//!
//! # Faithfulness
//!
//! The identity fields mirror BSim's `ExecutableRecord`: name, architecture,
//! compiler and the executable's MD5. BSim deduplicates an ingest by that
//! fingerprint, and [`BsimCorpus::insert_executable`] does the same — ingesting
//! the same program twice is not an error and does not duplicate its functions.

use kuna_base::error::{KunaError, KunaResult};

use super::signature::BsimSignature;

/// `"KBSM"` — the corpus file magic.
pub const CORPUS_MAGIC: &[u8; 4] = b"KBSM";

/// The on-disk format version. **Bump on any change to the layout or to what a
/// stored feature means** (a different signature setting makes stored vectors
/// incomparable, and the settings word is checked on load).
pub const CORPUS_VERSION: u32 = 1;

/// Record flag: the body has unimplemented instructions.
pub const FLAG_UNIMPLEMENTED: u8 = 0x01;
/// Record flag: the body's flow ran into bad data.
pub const FLAG_BAD_DATA: u8 = 0x02;

/// One executable's identity and its signed functions.
#[derive(Clone, Debug, PartialEq)]
pub struct CorpusExecutable {
    /// The executable's name, as BSim's `ExecutableRecord.getNameExec` reports it.
    pub name: String,
    /// The Ghidra language id, e.g. `x86:LE:64:default`.
    pub architecture: String,
    /// The compiler spec id.
    pub compiler: String,
    /// The executable's MD5 as 32 lowercase hex characters.
    pub md5: String,
    /// The signed functions, in ingest order.
    pub functions: Vec<BsimSignature>,
}

impl CorpusExecutable {
    /// The functions keyed for a lookup by name.
    pub fn function_named(&self, name: &str) -> Option<&BsimSignature> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// The function at `address`.
    pub fn function_at(&self, address: u64) -> Option<&BsimSignature> {
        self.functions.iter().find(|f| f.address == address)
    }
}

/// What happened when an executable was offered to a corpus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IngestOutcome {
    /// The executable was new; its functions were stored.
    Inserted,
    /// An executable with the same MD5 was already present. Nothing changed —
    /// BSim treats a repeat ingest as success, not an error.
    AlreadyPresent,
    /// The name matched but the MD5 did not: the corpus holds a *different*
    /// build under that name. Refused rather than silently overwritten.
    NameCollision,
}

/// A set of signed executables.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BsimCorpus {
    /// The signature settings every stored vector was built with.
    settings: u32,
    /// The language family the weights were chosen for.
    language: String,
    /// The executables, in insertion order.
    executables: Vec<CorpusExecutable>,
}

impl BsimCorpus {
    /// An empty corpus built under `settings` for `language`.
    pub fn new(settings: u32, language: impl Into<String>) -> Self {
        BsimCorpus { settings, language: language.into(), executables: Vec::new() }
    }

    /// The settings the corpus was built with.
    pub fn settings(&self) -> u32 {
        self.settings
    }

    /// The language family the corpus records.
    pub fn language(&self) -> &str {
        &self.language
    }

    /// The executables, in insertion order.
    pub fn executables(&self) -> &[CorpusExecutable] {
        &self.executables
    }

    /// Total signed functions across every executable.
    pub fn function_count(&self) -> usize {
        self.executables.iter().map(|e| e.functions.len()).sum()
    }

    /// Every `(executable, function)` pair, in corpus order.
    pub fn functions(&self) -> impl Iterator<Item = (&CorpusExecutable, &BsimSignature)> {
        self.executables
            .iter()
            .flat_map(|e| e.functions.iter().map(move |f| (e, f)))
    }

    /// Offer an executable to the corpus, deduplicating by MD5 as BSim does.
    pub fn insert_executable(&mut self, exec: CorpusExecutable) -> IngestOutcome {
        if self.executables.iter().any(|e| e.md5 == exec.md5) {
            return IngestOutcome::AlreadyPresent;
        }
        if self.executables.iter().any(|e| e.name == exec.name) {
            return IngestOutcome::NameCollision;
        }
        self.executables.push(exec);
        IngestOutcome::Inserted
    }

    /// Serialize to the on-disk corpus format.
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(CORPUS_MAGIC);
        out.extend_from_slice(&CORPUS_VERSION.to_le_bytes());
        out.extend_from_slice(&self.settings.to_le_bytes());
        write_str(&mut out, &self.language);
        out.extend_from_slice(&(self.executables.len() as u32).to_le_bytes());
        for exec in &self.executables {
            write_str(&mut out, &exec.name);
            write_str(&mut out, &exec.architecture);
            write_str(&mut out, &exec.compiler);
            write_str(&mut out, &exec.md5);
            out.extend_from_slice(&(exec.functions.len() as u32).to_le_bytes());
            for func in &exec.functions {
                write_str(&mut out, &func.name);
                out.extend_from_slice(&func.address.to_le_bytes());
                let mut flags = 0u8;
                if func.has_unimplemented {
                    flags |= FLAG_UNIMPLEMENTED;
                }
                if func.has_bad_data {
                    flags |= FLAG_BAD_DATA;
                }
                out.push(flags);
                out.extend_from_slice(&(func.features.len() as u32).to_le_bytes());
                for feature in &func.features {
                    out.extend_from_slice(&feature.to_le_bytes());
                }
            }
        }
        out
    }

    /// Parse the on-disk corpus format.
    ///
    /// **Never panics**: every read is bounds-checked, and a truncated or foreign
    /// file is an `Err`. A corpus whose settings differ from the run's is a load
    /// success — the caller decides whether those vectors are comparable.
    pub fn load(bytes: &[u8]) -> KunaResult<Self> {
        let mut r = Reader::new(bytes);
        let magic = r.take(4)?;
        if magic != CORPUS_MAGIC {
            return Err(KunaError::bad_data(format!(
                "bsim corpus: bad magic {magic:02x?} (expected {CORPUS_MAGIC:02x?})"
            )));
        }
        let version = r.u32()?;
        if version != CORPUS_VERSION {
            return Err(KunaError::bad_data(format!(
                "bsim corpus: version {version} != supported {CORPUS_VERSION}"
            )));
        }
        let settings = r.u32()?;
        let language = r.string()?;
        let n_execs = r.u32()? as usize;
        // A declared count larger than the remaining bytes could hold is a
        // corrupt file, never something to pre-allocate for.
        if n_execs > bytes.len() {
            return Err(KunaError::bad_data(format!(
                "bsim corpus: executable count {n_execs} exceeds the file size"
            )));
        }
        let mut executables = Vec::with_capacity(n_execs);
        for _ in 0..n_execs {
            let name = r.string()?;
            let architecture = r.string()?;
            let compiler = r.string()?;
            let md5 = r.string()?;
            let n_funcs = r.u32()? as usize;
            if n_funcs > bytes.len() {
                return Err(KunaError::bad_data(format!(
                    "bsim corpus: function count {n_funcs} exceeds the file size"
                )));
            }
            let mut functions = Vec::with_capacity(n_funcs);
            for _ in 0..n_funcs {
                let fname = r.string()?;
                let address = r.u64()?;
                let flags = r.u8()?;
                let n_features = r.u32()? as usize;
                if n_features > bytes.len() {
                    return Err(KunaError::bad_data(format!(
                        "bsim corpus: feature count {n_features} exceeds the file size"
                    )));
                }
                let mut features = Vec::with_capacity(n_features);
                for _ in 0..n_features {
                    features.push(r.u32()?);
                }
                functions.push(BsimSignature {
                    name: fname,
                    address,
                    features,
                    has_unimplemented: flags & FLAG_UNIMPLEMENTED != 0,
                    has_bad_data: flags & FLAG_BAD_DATA != 0,
                });
            }
            executables.push(CorpusExecutable {
                name,
                architecture,
                compiler,
                md5,
                functions,
            });
        }
        Ok(BsimCorpus { settings, language, executables })
    }
}

/// Write a length-prefixed (`u32` LE) UTF-8 string.
fn write_str(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

/// A bounds-checked little-endian reader; every accessor returns `Err` rather
/// than panic on a short input.
struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0 }
    }

    fn take(&mut self, n: usize) -> KunaResult<&'a [u8]> {
        let end = self
            .pos
            .checked_add(n)
            .ok_or_else(|| KunaError::bad_data("bsim corpus: length overflow"))?;
        if end > self.buf.len() {
            return Err(KunaError::bad_data(format!(
                "bsim corpus: truncated (need {n} bytes at {}, have {})",
                self.pos,
                self.buf.len()
            )));
        }
        let slice = &self.buf[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u8(&mut self) -> KunaResult<u8> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> KunaResult<u32> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn u64(&mut self) -> KunaResult<u64> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    fn string(&mut self) -> KunaResult<String> {
        let len = self.u32()? as usize;
        let raw = self.take(len)?;
        std::str::from_utf8(raw)
            .map(str::to_owned)
            .map_err(|_| KunaError::bad_data("bsim corpus: non-UTF-8 string"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(name: &str, address: u64, features: Vec<u32>) -> BsimSignature {
        BsimSignature {
            name: name.into(),
            address,
            features,
            has_unimplemented: false,
            has_bad_data: false,
        }
    }

    fn exec(name: &str, md5: &str, functions: Vec<BsimSignature>) -> CorpusExecutable {
        CorpusExecutable {
            name: name.into(),
            architecture: "x86:LE:64:default".into(),
            compiler: "gcc".into(),
            md5: md5.into(),
            functions,
        }
    }

    #[test]
    fn a_corpus_round_trips_byte_identically() {
        let mut corpus = BsimCorpus::new(0x49, "x86:LE:64:default");
        assert_eq!(
            corpus.insert_executable(exec(
                "app.exe",
                "0123456789abcdef0123456789abcdef",
                vec![sig("main", 0x401000, vec![1, 2, 3])],
            )),
            IngestOutcome::Inserted
        );
        let bytes = corpus.serialize();
        let back = BsimCorpus::load(&bytes).expect("load");
        assert_eq!(back, corpus);
        assert_eq!(back.serialize(), bytes);
        assert_eq!(back.settings(), 0x49);
        assert_eq!(back.function_count(), 1);
    }

    #[test]
    fn flags_survive_the_round_trip() {
        let mut corpus = BsimCorpus::new(0x49, "x86");
        let mut f = sig("noisy", 0x10, vec![7]);
        f.has_unimplemented = true;
        f.has_bad_data = true;
        corpus.insert_executable(exec("a", "aa", vec![f]));
        let back = BsimCorpus::load(&corpus.serialize()).expect("load");
        let stored = &back.executables()[0].functions[0];
        assert!(stored.has_unimplemented);
        assert!(stored.has_bad_data);
    }

    #[test]
    fn ingesting_the_same_md5_twice_is_not_an_error() {
        let mut corpus = BsimCorpus::new(0x49, "x86");
        let md5 = "0123456789abcdef0123456789abcdef";
        assert_eq!(corpus.insert_executable(exec("app", md5, vec![])), IngestOutcome::Inserted);
        assert_eq!(
            corpus.insert_executable(exec("app", md5, vec![])),
            IngestOutcome::AlreadyPresent
        );
        assert_eq!(corpus.executables().len(), 1);
    }

    #[test]
    fn a_different_build_under_the_same_name_is_refused() {
        let mut corpus = BsimCorpus::new(0x49, "x86");
        corpus.insert_executable(exec("app", "aaaa", vec![]));
        assert_eq!(
            corpus.insert_executable(exec("app", "bbbb", vec![])),
            IngestOutcome::NameCollision
        );
        assert_eq!(corpus.executables().len(), 1);
    }

    #[test]
    fn a_foreign_or_truncated_file_is_rejected_not_panicked_on() {
        assert!(BsimCorpus::load(b"").is_err());
        assert!(BsimCorpus::load(b"NOPE").is_err());
        let mut corpus = BsimCorpus::new(0x49, "x86");
        corpus.insert_executable(exec("a", "aa", vec![sig("f", 1, vec![1, 2, 3])]));
        let bytes = corpus.serialize();
        for cut in [3usize, 5, 9, 20, bytes.len() - 1] {
            assert!(BsimCorpus::load(&bytes[..cut]).is_err(), "cut at {cut} must fail");
        }
    }

    #[test]
    fn a_corrupt_giant_count_is_refused_without_allocating() {
        let mut corpus = BsimCorpus::new(0x49, "x86");
        corpus.insert_executable(exec("a", "aa", vec![]));
        let mut bytes = corpus.serialize();
        // Overwrite the executable count with a huge value.
        let off = 4 + 4 + 4 + 4 + "x86".len();
        bytes[off..off + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(BsimCorpus::load(&bytes).is_err());
    }

    #[test]
    fn functions_are_reachable_by_name_and_address() {
        let mut corpus = BsimCorpus::new(0x49, "x86");
        corpus.insert_executable(exec("a", "aa", vec![sig("want", 0x40, vec![1])]));
        let e = &corpus.executables()[0];
        assert!(e.function_named("want").is_some());
        assert!(e.function_at(0x40).is_some());
        assert!(e.function_named("absent").is_none());
        assert_eq!(corpus.functions().count(), 1);
    }
}
