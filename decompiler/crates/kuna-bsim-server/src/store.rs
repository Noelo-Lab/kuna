use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use kuna_analysis::analyzers::bsim::corpus::CorpusExecutable;
use kuna_analysis::analyzers::bsim::lsh::{score_candidate, LshCosineVector};
use kuna_analysis::analyzers::bsim::lshweights::{LshProfile, LshWeights};
use kuna_analysis::analyzers::bsim::signature::BsimSignature;
use md5::Md5;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tempfile::NamedTempFile;
use thiserror::Error;

use crate::{
    canonical_sha256, is_lower_hex, MachOSlice, Match, ProfileInfo, ProfilesResponse, QueryFilters,
    QueryRequest, QueryResponse, StoredFunction, MAX_ARTIFACT_BYTES, MAX_MATCHES, MAX_NAME_BYTES,
    MAX_QUERY_FEATURES,
};

const PROFILES: [LshProfile; 6] = [
    LshProfile::Size32,
    LshProfile::Size64,
    LshProfile::Size64With32Abi,
    LshProfile::NoSize,
    LshProfile::Jvm,
    LshProfile::Dalvik,
];
const RECORD_VERSION: u32 = 3;
const MAX_META_BYTES: u64 = 128 * 1024;
const MAX_RECORD_BYTES: u64 = 256 * 1024 * 1024;
const MAX_SIGNATURE_BYTES: u64 = 1024 * 1024;
const TEMP_PREFIX: &str = ".pending-";

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("{0}")]
    Invalid(String),
    #[error("BSim store corruption: {0}")]
    Corrupt(String),
    #[error("no published {0} matches that identity")]
    NotFound(&'static str),
    #[error("BSim store deadline exceeded")]
    Deadline,
    #[error("BSim store I/O: {0}")]
    Io(#[from] io::Error),
    #[error("BSim store serialization: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportOutcome {
    Inserted,
    Updated,
    AlreadyPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportResult {
    pub sha256: String,
    pub outcome: ImportOutcome,
}

/// Counts describe the incoming entries, not the whole stored inventory.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MergeReport {
    /// Whether native discovery has already signed its full inventory.
    pub executable_complete: bool,
    pub inserted: usize,
    /// Replacements include a change in provenance without a payload change.
    pub updated: usize,
    pub unchanged: usize,
    pub kept_published: usize,
}

/// A local store. Readers see whole records even across separate processes;
/// local imports serialize publication and never overwrite a binary object.
#[derive(Debug)]
pub struct Store {
    root: PathBuf,
}

enum Original<'a> {
    Bytes(&'a [u8]),
    Staged { file: &'a mut File, len: u64 },
}

impl Original<'_> {
    fn len(&self) -> u64 {
        match self {
            Self::Bytes(bytes) => bytes.len() as u64,
            Self::Staged { len, .. } => *len,
        }
    }

    fn digests(&mut self) -> Result<(String, String), StoreError> {
        let (md5, sha256) = match self {
            Self::Bytes(bytes) => (Md5::digest(*bytes), Sha256::digest(*bytes)),
            Self::Staged { file, len } => {
                file.rewind()?;
                let mut md5 = Md5::new();
                let mut sha256 = Sha256::new();
                let actual = hash_chunks(file.take(*len + 1), |bytes| {
                    md5.update(bytes);
                    sha256.update(bytes);
                })?;
                if actual != *len {
                    return invalid("staged binary changed size");
                }
                (md5.finalize(), sha256.finalize())
            }
        };
        Ok((kuna_base::hex_lower(&md5), kuna_base::hex_lower(&sha256)))
    }

    fn copy_to(&mut self, target: &mut File) -> Result<(), StoreError> {
        match self {
            Self::Bytes(bytes) => target.write_all(bytes)?,
            Self::Staged { file, len } => {
                file.rewind()?;
                if io::copy(&mut file.take(*len), target)? != *len {
                    return invalid("staged binary was truncated");
                }
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct Metadata {
    version: u32,
    settings: u32,
    byte_len: u64,
    name: String,
    architecture: String,
    compiler: String,
    md5: String,
    macho_slice: Option<MachOSlice>,
    complete: bool,
    n_functions: usize,
}

#[derive(Deserialize, PartialEq, Serialize)]
struct Signature {
    source: SignatureSource,
    name: String,
    address: u64,
    features: Vec<u32>,
    has_unimplemented: bool,
    has_bad_data: bool,
}

impl Signature {
    fn new(signature: BsimSignature, source: SignatureSource) -> Self {
        Self {
            source,
            name: signature.name,
            address: signature.address,
            features: signature.features,
            has_unimplemented: signature.has_unimplemented,
            has_bad_data: signature.has_bad_data,
        }
    }
}

// Declaration order is collision precedence, from weakest to strongest.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum SignatureSource {
    Native,
    Project,
    Published,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let root = open_store_root(path.as_ref())?;
        ensure_directory(&root.join("binaries"))?;
        ensure_directory(&root.join("records"))?;
        let store = Self { root };
        for profile in PROFILES {
            ensure_directory(&store.profile_dir(profile))?;
        }
        Ok(store)
    }

    /// Import native discoveries. A repeated identity keeps its original
    /// display name and any existing entries, and may add missing signatures.
    pub fn import(
        &self,
        profile: LshProfile,
        executable: CorpusExecutable,
        original: &[u8],
    ) -> Result<ImportResult, StoreError> {
        self.write(profile, executable, Original::Bytes(original), SignatureSource::Native, None)
            .map(|(import, _)| import)
    }

    /// Import discoveries from an immutable private staging file. For a
    /// universal Mach-O, `macho_slice` is the range the signer validated and
    /// actually loaded; the blob and its digests still cover the whole file.
    pub fn import_file(
        &self,
        profile: LshProfile,
        executable: CorpusExecutable,
        file: &mut File,
        macho_slice: Option<MachOSlice>,
    ) -> Result<ImportResult, StoreError> {
        let meta = file.metadata()?;
        if !meta.is_file() {
            return invalid("staged binary is not a regular file");
        }
        self.write(
            profile, executable, Original::Staged { file, len: meta.len() },
            SignatureSource::Native, macho_slice,
        ).map(|(import, _)| import)
    }

    /// Merge a Ghidra snapshot by address without claiming native discovery
    /// coverage. Saved signatures replace native or older project signatures
    /// at shared addresses, but never explicitly published ones.
    pub fn import_project(
        &self,
        profile: LshProfile,
        executable: CorpusExecutable,
        original: &[u8],
    ) -> Result<(ImportResult, MergeReport), StoreError> {
        self.write(profile, executable, Original::Bytes(original), SignatureSource::Project, None)
    }

    /// Insert or replace one signed function without disturbing other entries
    /// or marking the executable complete.
    pub fn publish(
        &self,
        profile: LshProfile,
        executable: CorpusExecutable,
        original: &[u8],
    ) -> Result<ImportResult, StoreError> {
        self.publish_original(profile, executable, Original::Bytes(original))
    }

    /// Publish an immutable, private staging file without reading it all into memory.
    pub fn publish_file(
        &self,
        profile: LshProfile,
        executable: CorpusExecutable,
        file: &mut File,
    ) -> Result<ImportResult, StoreError> {
        let meta = file.metadata()?;
        if !meta.is_file() {
            return invalid("staged binary is not a regular file");
        }
        self.publish_original(profile, executable, Original::Staged { file, len: meta.len() })
    }

    fn publish_original(
        &self,
        profile: LshProfile,
        executable: CorpusExecutable,
        original: Original<'_>,
    ) -> Result<ImportResult, StoreError> {
        if executable.complete || executable.functions.len() != 1 {
            return invalid("publication requires exactly one signature in an incomplete executable");
        }
        if original.len() == 0 {
            return invalid("original binary is empty");
        }
        self.write(profile, executable, original, SignatureSource::Published, None)
            .map(|(import, _)| import)
    }

    fn write(
        &self,
        profile: LshProfile,
        mut executable: CorpusExecutable,
        mut original: Original<'_>,
        source: SignatureSource,
        macho_slice: Option<MachOSlice>,
    ) -> Result<(ImportResult, MergeReport), StoreError> {
        let native_complete = source == SignatureSource::Native && executable.complete;
        let byte_len = original.len();
        if byte_len > MAX_ARTIFACT_BYTES {
            return invalid("original binary exceeds the 512 MiB artifact limit");
        }
        if !valid_slice_range(macho_slice, byte_len) {
            return invalid("indexed Mach-O slice range is outside the original bytes");
        }
        if let Original::Bytes(bytes) = &original {
            if kuna_analysis::loader::macho_fat::is_fat(bytes) {
                return invalid("a universal Mach-O import requires an indexed slice");
            }
        }
        let settings = with_weights(profile, |w| Ok(w.settings as u32))?;
        if !profile.admits(&executable.architecture) {
            return invalid("executable architecture is incompatible with the weights profile");
        }
        if ![&executable.name, &executable.architecture, &executable.compiler]
            .into_iter()
            .all(|name| valid_name(name))
        {
            return invalid("executable metadata is missing or too long");
        }
        if executable.functions.iter().any(|signature| {
            signature.settings != settings
                || !valid_name(&signature.name)
                || !valid_features(&signature.features)
        }) {
            return invalid("invalid function signature or incompatible settings");
        }
        executable.functions.sort_unstable_by_key(|f| f.address);
        if executable.functions.windows(2).any(|pair| pair[0].address == pair[1].address) {
            return invalid("duplicate function entry address");
        }
        let (md5, sha256) = original.digests()?;
        if executable.md5 != md5 {
            return invalid("executable MD5 does not match the original bytes");
        }
        let _lock = self.import_lock(source)?;
        let dir = self.profile_dir(profile);
        let record_path = dir.join(&sha256);
        let previous = read_meta(&record_path, profile, settings)?;
        let was_present = previous.is_some();
        let was_complete = previous.as_ref().is_some_and(|(meta, _)| meta.complete);
        let mut incoming = executable.functions.into_iter().peekable();
        let mut functions = Vec::new();
        let mut merge = MergeReport { executable_complete: was_complete || native_complete, ..MergeReport::default() };
        let meta = if let Some((meta, reader)) = previous {
            if meta.macho_slice != macho_slice {
                return invalid("another Mach-O slice is already indexed for this profile and whole-file digest");
            }
            if meta.architecture != executable.architecture || meta.compiler != executable.compiler {
                return invalid("the indexed architecture or compiler for this digest has changed");
            }
            if meta.md5 != executable.md5 || meta.byte_len != byte_len {
                return corrupt("the indexed record disagrees with the original bytes");
            }
            // Do not repair a corrupt published object or record silently.
            self.check_binary(&sha256, meta.byte_len)?;
            visit_signatures(reader, &meta, None, |existing| {
                while let Some(new) = incoming.next_if(|new| new.address < existing.address) {
                    functions.push(Signature::new(new, source));
                    merge.inserted += 1;
                }
                let Some(new) = incoming.next_if(|new| new.address == existing.address) else {
                    functions.push(existing);
                    return Ok(());
                };
                let new = Signature::new(new, source);
                let replaces = source > existing.source
                    || (source == existing.source && source != SignatureSource::Native);
                let kept = if replaces && new != existing {
                    merge.updated += 1;
                    new
                } else {
                    if source == SignatureSource::Project && existing.source == SignatureSource::Published {
                        merge.kept_published += 1;
                    } else {
                        merge.unchanged += 1;
                    }
                    existing
                };
                functions.push(kept);
                Ok(())
            })?;
            meta
        } else {
            Metadata {
                version: RECORD_VERSION,
                settings,
                byte_len,
                name: executable.name,
                architecture: executable.architecture,
                compiler: executable.compiler,
                md5: executable.md5,
                macho_slice,
                complete: false,
                n_functions: 0,
            }
        };
        merge.inserted += incoming.len();
        functions.extend(incoming.map(|signature| Signature::new(signature, source)));
        let complete = merge.executable_complete;
        let meta = Metadata { complete, n_functions: functions.len(), ..meta };
        let outcome = if !was_present {
            ImportOutcome::Inserted
        } else if merge.inserted != 0 || merge.updated != 0 || was_complete != complete {
            ImportOutcome::Updated
        } else {
            ImportOutcome::AlreadyPresent
        };
        let result = ImportResult { sha256, outcome };
        let sha256 = &result.sha256;
        if outcome == ImportOutcome::AlreadyPresent {
            verify_binary(&self.binary_path(sha256), sha256, byte_len)?;
            // A previous rename can be visible even if the directory sync failed.
            sync_dir(&dir)?;
            return Ok((result, merge));
        }

        let mut pending = pending_file(&dir)?;
        {
            let mut writer = BufWriter::new(pending.as_file_mut());
            serde_json::to_writer(&mut writer, &meta)?;
            writer.write_all(b"\n")?;
            for signature in &functions {
                serde_json::to_writer(&mut writer, signature)?;
                writer.write_all(b"\n")?;
            }
            writer.flush()?;
        }
        if pending.as_file().metadata()?.len() > MAX_RECORD_BYTES {
            return invalid("the signed record exceeds the store's per-executable size limit");
        }
        pending.as_file().sync_all()?;
        // The record can become visible only after the binary and its directory
        // entry are durable. Nothing reachable over HTTP refers to staging files.
        self.ensure_binary(sha256, &mut original)?;
        sync_dir(&self.root.join("records"))?;
        sync_dir(&self.root)?;
        pending.persist(&record_path).map_err(|e| e.error)?;
        sync_dir(&dir)?;
        Ok((result, merge))
    }

    pub fn profiles(&self) -> Result<ProfilesResponse, StoreError> {
        self.profiles_before(None)
    }

    pub fn profiles_before(&self, deadline: Option<Instant>) -> Result<ProfilesResponse, StoreError> {
        let mut profiles = Vec::with_capacity(PROFILES.len());
        for profile in PROFILES {
            let settings = with_weights(profile, |w| Ok(w.settings as u32))?;
            let mut info = ProfileInfo {
                profile: profile.name().into(),
                settings,
                executables: 0,
                incomplete_executables: 0,
                functions: 0,
            };
            self.scan(profile, settings, deadline, |sha256, meta, _| {
                self.check_binary(sha256, meta.byte_len)?;
                info.executables += 1;
                info.incomplete_executables += usize::from(!meta.complete);
                info.functions = info.functions.checked_add(meta.n_functions)
                    .ok_or_else(|| StoreError::Corrupt("profile function count overflow".into()))?;
                Ok(())
            })?;
            profiles.push(info);
        }
        Ok(ProfilesResponse { profiles })
    }

    pub fn query(&self, request: &QueryRequest) -> Result<QueryResponse, StoreError> {
        self.query_before(request, None)
    }

    pub fn query_before(
        &self,
        request: &QueryRequest,
        deadline: Option<Instant>,
    ) -> Result<QueryResponse, StoreError> {
        let profile = require_profile(&request.profile)?;
        if !valid_features(&request.features) {
            return invalid("feature hashes must be a nonempty, bounded, sorted array");
        }
        if !(0.0..=1.0).contains(&request.min_similarity) {
            return invalid("similarity floor must be between 0 and 1");
        }
        if !(1..=MAX_MATCHES).contains(&request.max_matches) {
            return invalid("match limit is outside the allowed range");
        }
        if let Some(sha256) = request.filters.sha256.as_deref() {
            require_sha256(sha256)?;
        }
        if [&request.filters.executable, &request.filters.architecture, &request.filters.compiler]
            .into_iter()
            .flatten()
            .any(|name| !valid_name(name))
        {
            return invalid("filter value is empty or too long");
        }
        with_weights(profile, |weights| {
            if request.settings != weights.settings as u32 {
                return invalid("signature settings do not match the weights profile");
            }
            let vector = LshCosineVector::from_features(&request.features, &weights.factory, &weights.idf);
            if !vector.length().is_finite() || vector.length() == 0.0 {
                return invalid("query signature has no scoreable weight");
            }
            let mut best = BinaryHeap::<Ranked>::new();
            let mut considered = 0usize;
            let mut incomplete_executables = 0usize;
            let mut truncated = false;
            let mut score_record = |sha256: &str, meta: Metadata, reader: BufReader<File>| {
                if !meta.admits(&request.filters) {
                    return Ok(());
                }
                self.check_binary(sha256, meta.byte_len)?;
                incomplete_executables += usize::from(!meta.complete);
                considered = considered.checked_add(meta.n_functions)
                    .ok_or_else(|| StoreError::Corrupt("query function count overflow".into()))?;
                visit_signatures(reader, &meta, deadline, |function| {
                    let candidate = LshCosineVector::from_features(
                        &function.features,
                        &weights.factory,
                        &weights.idf,
                    );
                    let Some((similarity, significance)) =
                        score_candidate(&weights.factory, &vector, &candidate, request.min_similarity)
                    else {
                        return Ok(());
                    };
                    let key = RankKey {
                        similarity, significance, executable: &meta.name, sha256, address: function.address,
                    };
                    if best.len() == request.max_matches {
                        truncated = true;
                        if best.peek().is_some_and(|worst| key.cmp(worst.key()) != Ordering::Less) {
                            return Ok(());
                        }
                        best.pop();
                    }
                    best.push(Ranked(Match {
                        function: meta.public_function(sha256, &function),
                        similarity,
                        significance,
                    }));
                    Ok(())
                })
            };
            if let Some(sha256) = request.filters.sha256.as_deref() {
                if let Some((meta, reader)) = self.read_named(profile, request.settings, sha256)? {
                    score_record(sha256, meta, reader)?;
                }
            } else {
                self.scan(profile, request.settings, deadline, score_record)?;
            }
            check_deadline(deadline)?;
            Ok(QueryResponse {
                matches: best.into_sorted_vec().into_iter().map(|ranked| ranked.0).collect(),
                considered,
                incomplete_executables,
                min_similarity: request.min_similarity,
                truncated,
            })
        })
    }

    /// Look up an exact published profile, blob digest, and function entry.
    pub fn function(&self, profile: &str, sha256: &str, address: u64) -> Result<StoredFunction, StoreError> {
        self.function_before(profile, sha256, address, None)
    }

    pub fn function_before(
        &self,
        profile: &str,
        sha256: &str,
        address: u64,
        deadline: Option<Instant>,
    ) -> Result<StoredFunction, StoreError> {
        let profile = require_profile(profile)?;
        require_sha256(sha256)?;
        let settings = with_weights(profile, |w| Ok(w.settings as u32))?;
        let (meta, reader) = self.read_named(profile, settings, sha256)?
            .ok_or(StoreError::NotFound("function"))?;
        self.check_binary(sha256, meta.byte_len)?;
        let mut found = None;
        visit_signatures(reader, &meta, deadline, |function| {
            if function.address == address {
                found = Some(meta.public_function(sha256, &function));
            }
            Ok(())
        })?;
        found.ok_or(StoreError::NotFound("function"))
    }

    /// Open a published content-addressed object and return its byte length.
    pub fn open_binary(&self, sha256: &str) -> Result<(File, u64), StoreError> {
        require_sha256(sha256)?;
        for profile in PROFILES {
            let settings = with_weights(profile, |w| Ok(w.settings as u32))?;
            if let Some((meta, _)) = self.read_named(profile, settings, sha256)? {
                self.check_binary(sha256, meta.byte_len)?;
                return Ok((File::open(self.binary_path(sha256))?, meta.byte_len));
            }
        }
        Err(StoreError::NotFound("binary"))
    }

    fn profile_dir(&self, profile: LshProfile) -> PathBuf {
        self.root.join("records").join(profile.name())
    }

    fn binary_path(&self, digest: &str) -> PathBuf {
        self.root.join("binaries").join(digest)
    }

    fn import_lock(&self, source: SignatureSource) -> Result<File, StoreError> {
        let path = self.root.join("import.lock");
        let file = match OpenOptions::new().read(true).write(true).create_new(true).open(&path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                regular_file_len(&path)?;
                OpenOptions::new().read(true).write(true).open(path)?
            }
            Err(e) => return Err(e.into()),
        };
        if source == SignatureSource::Published {
            // Remote publications must not queue behind local imports.
            match file.try_lock() {
                Ok(()) => {}
                Err(fs::TryLockError::WouldBlock) => return Err(StoreError::Deadline),
                Err(fs::TryLockError::Error(error)) => return Err(error.into()),
            }
        } else {
            file.lock()?;
        }
        Ok(file)
    }

    fn ensure_binary(&self, sha256: &str, original: &mut Original<'_>) -> Result<(), StoreError> {
        let path = self.binary_path(sha256);
        let dir = self.root.join("binaries");
        match regular_file_len(&path)? {
            Some(len) => {
                if len != original.len() {
                    return corrupt("the existing binary has an unexpected size");
                }
                verify_binary(&path, sha256, len)?;
            }
            None => {
                let mut pending = pending_file(&dir)?;
                original.copy_to(pending.as_file_mut())?;
                pending.as_file().sync_all()?;
                pending.persist_noclobber(&path).map_err(|e| e.error)?;
            }
        }
        sync_dir(&dir)?;
        Ok(())
    }

    fn check_binary(&self, sha256: &str, expected: u64) -> Result<(), StoreError> {
        if regular_file_len(&self.binary_path(sha256))? != Some(expected) {
            return corrupt(format!("indexed binary {sha256} is missing or has an unexpected size"));
        }
        Ok(())
    }

    fn read_named(
        &self,
        profile: LshProfile,
        settings: u32,
        sha256: &str,
    ) -> Result<Option<(Metadata, BufReader<File>)>, StoreError> {
        read_meta(&self.profile_dir(profile).join(sha256), profile, settings)
    }

    fn scan(
        &self,
        profile: LshProfile,
        settings: u32,
        deadline: Option<Instant>,
        mut visit: impl FnMut(&str, Metadata, BufReader<File>) -> Result<(), StoreError>,
    ) -> Result<(), StoreError> {
        for entry in fs::read_dir(self.profile_dir(profile))? {
            check_deadline(deadline)?;
            let entry = entry?;
            let name = entry.file_name();
            let sha256 = name.to_str().ok_or_else(|| StoreError::Corrupt("non-UTF-8 record name".into()))?;
            if sha256.starts_with(TEMP_PREFIX) {
                continue;
            }
            if !canonical_sha256(sha256) {
                return corrupt("invalid indexed record name");
            }
            let (meta, reader) = read_meta(&entry.path(), profile, settings)?
                .ok_or_else(|| StoreError::Corrupt("indexed record disappeared while reading it".into()))?;
            visit(sha256, meta, reader)?;
        }
        Ok(())
    }
}

impl Metadata {
    fn admits(&self, filters: &QueryFilters) -> bool {
        filters.executable.as_ref().is_none_or(|v| v == &self.name)
            && filters.architecture.as_ref().is_none_or(|v| v == &self.architecture)
            && filters.compiler.as_ref().is_none_or(|v| v == &self.compiler)
    }

    fn public_function(&self, sha256: &str, function: &Signature) -> StoredFunction {
        StoredFunction {
            name: function.name.clone(),
            address: function.address,
            executable: self.name.clone(),
            architecture: self.architecture.clone(),
            compiler: self.compiler.clone(),
            md5: self.md5.clone(),
            sha256: sha256.into(),
            macho_slice: self.macho_slice,
            complete: self.complete,
        }
    }
}

fn read_meta(path: &Path, profile: LshProfile, settings: u32) -> Result<Option<(Metadata, BufReader<File>)>, StoreError> {
    let Some(len) = regular_file_len(path)? else { return Ok(None) };
    if len > MAX_RECORD_BYTES {
        return corrupt("indexed record exceeds the read size limit");
    }
    let mut reader = BufReader::new(File::open(path)?);
    let mut header = Vec::new();
    reader.by_ref().take(MAX_META_BYTES).read_until(b'\n', &mut header)?;
    if header.last() != Some(&b'\n') {
        return corrupt("missing or oversized record metadata");
    }
    let meta: Metadata = serde_json::from_slice(&header)?;
    if meta.version != RECORD_VERSION {
        return corrupt(format!(
            "record version {} is unsupported (expected {RECORD_VERSION}); reimport into a new store",
            meta.version,
        ));
    }
    if meta.settings != settings
        || !profile.admits(&meta.architecture)
        || ![&meta.name, &meta.architecture, &meta.compiler].into_iter().all(|name| valid_name(name))
        || !is_lower_hex(&meta.md5, 32)
        || meta.n_functions as u64 > MAX_RECORD_BYTES
        || meta.byte_len > MAX_ARTIFACT_BYTES
        || !valid_slice_range(meta.macho_slice, meta.byte_len)
    {
        return corrupt("invalid settings or executable metadata");
    }
    Ok(Some((meta, reader)))
}

fn valid_slice_range(slice: Option<MachOSlice>, byte_len: u64) -> bool {
    slice.is_none_or(|slice| {
        slice.offset != 0 && slice.size != 0
            && slice.offset.checked_add(slice.size).is_some_and(|end| end <= byte_len)
    })
}

fn visit_signatures(
    reader: BufReader<File>,
    meta: &Metadata,
    deadline: Option<Instant>,
    mut visit: impl FnMut(Signature) -> Result<(), StoreError>,
) -> Result<(), StoreError> {
    let mut reader = reader.take(MAX_RECORD_BYTES);
    let mut row = Vec::new();
    let mut previous = None;
    for i in 0..meta.n_functions {
        if i % 256 == 0 {
            check_deadline(deadline)?;
        }
        row.clear();
        reader.by_ref().take(MAX_SIGNATURE_BYTES).read_until(b'\n', &mut row)?;
        if row.last() != Some(&b'\n') {
            return corrupt("missing or oversized stored signature");
        }
        let signature: Signature = serde_json::from_slice(&row)?;
        if previous.is_some_and(|address| signature.address <= address)
            || !valid_name(&signature.name)
            || !valid_features(&signature.features)
        {
            return corrupt("invalid stored signature or unordered entry");
        }
        previous = Some(signature.address);
        visit(signature)?;
    }
    if !reader.fill_buf()?.is_empty() {
        return corrupt("unexpected data after stored function inventory");
    }
    check_deadline(deadline)
}

fn verify_binary(path: &Path, expected: &str, len: u64) -> Result<(), StoreError> {
    let mut sha256 = Sha256::new();
    let actual = hash_chunks(File::open(path)?.take(len + 1), |bytes| sha256.update(bytes))?;
    if actual != len {
        return corrupt("the existing binary changed while reading it");
    }
    if kuna_base::hex_lower(&sha256.finalize()) != expected {
        return corrupt("the existing binary does not match its digest");
    }
    Ok(())
}

fn hash_chunks(input: impl Read, mut hash: impl FnMut(&[u8])) -> io::Result<u64> {
    let mut reader = BufReader::with_capacity(64 * 1024, input);
    let mut size = 0;
    loop {
        let chunk = reader.fill_buf()?;
        if chunk.is_empty() {
            return Ok(size);
        }
        let len = chunk.len();
        hash(chunk);
        size += len as u64;
        reader.consume(len);
    }
}

fn open_store_root(path: &Path) -> Result<PathBuf, StoreError> {
    let path = std::path::absolute(path)?;
    let mut base = path.as_path();
    let mut missing = Vec::new();
    while !base.is_dir() {
        missing.push(base);
        base = base.parent().ok_or_else(|| io::Error::new(io::ErrorKind::NotADirectory, "store root"))?;
    }
    // Retry the parent sync even when a prior attempt already made the
    // directory visible. Do not acknowledge creation of an unflushed root.
    if let Some(parent) = base.parent() {
        sync_dir(parent)?;
    }
    for directory in missing.into_iter().rev() {
        ensure_directory(directory)?;
        sync_dir(directory.parent().expect("a missing directory has a parent"))?;
    }
    Ok(path.canonicalize()?)
}

fn ensure_directory(path: &Path) -> Result<(), StoreError> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            if !fs::symlink_metadata(path)?.is_dir() {
                return corrupt(format!("{} is not a real directory", path.display()));
            }
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

fn regular_file_len(path: &Path) -> Result<Option<u64>, StoreError> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_file() => Ok(Some(meta.len())),
        Ok(_) => corrupt(format!("{} is not a regular file", path.display())),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn pending_file(dir: &Path) -> io::Result<NamedTempFile> {
    tempfile::Builder::new().prefix(TEMP_PREFIX).tempfile_in(dir)
}

fn sync_dir(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

fn require_profile(name: &str) -> Result<LshProfile, StoreError> {
    LshProfile::from_name(name).ok_or_else(|| StoreError::Invalid("unknown weights profile".into()))
}

fn require_sha256(digest: &str) -> Result<(), StoreError> {
    if canonical_sha256(digest) {
        Ok(())
    } else {
        invalid("SHA-256 must be 64 lowercase hexadecimal digits")
    }
}

fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= MAX_NAME_BYTES
}

fn valid_features(features: &[u32]) -> bool {
    !features.is_empty() && features.len() <= MAX_QUERY_FEATURES && features.is_sorted()
}

fn invalid<T>(reason: impl Into<String>) -> Result<T, StoreError> {
    Err(StoreError::Invalid(reason.into()))
}

fn corrupt<T>(reason: impl Into<String>) -> Result<T, StoreError> {
    Err(StoreError::Corrupt(reason.into()))
}

fn check_deadline(deadline: Option<Instant>) -> Result<(), StoreError> {
    if deadline.is_some_and(|end| Instant::now() >= end) {
        return Err(StoreError::Deadline);
    }
    Ok(())
}

thread_local! {
    // Kuna's parsed scoring weights own Rc tables. Parse each profile once per
    // worker without changing the weighting library's ownership model.
    static WEIGHTS: RefCell<[Option<LshWeights>; PROFILES.len()]> = const {
        RefCell::new([const { None }; PROFILES.len()])
    };
}

fn with_weights<T>(profile: LshProfile, use_weights: impl FnOnce(&LshWeights) -> Result<T, StoreError>) -> Result<T, StoreError> {
    let index = PROFILES.iter().position(|p| *p == profile).expect("profile table is exhaustive");
    WEIGHTS.with(|cache| {
        if cache.borrow()[index].is_none() {
            cache.borrow_mut()[index] = Some(profile.bundled_weights()
                .map_err(|e| StoreError::Corrupt(format!("bundled profile: {e}")))?);
        }
        let cached = cache.borrow();
        use_weights(cached[index].as_ref().expect("weights initialized above"))
    })
}

struct RankKey<'a> {
    similarity: f64,
    significance: f64,
    executable: &'a str,
    sha256: &'a str,
    address: u64,
}

impl RankKey<'_> {
    fn cmp(&self, other: RankKey<'_>) -> Ordering {
        other.similarity.total_cmp(&self.similarity)
            .then_with(|| other.significance.total_cmp(&self.significance))
            .then_with(|| self.executable.cmp(other.executable))
            .then_with(|| self.address.cmp(&other.address))
            .then_with(|| self.sha256.cmp(other.sha256))
    }
}

struct Ranked(Match);

impl Ranked {
    fn key(&self) -> RankKey<'_> {
        let hit = &self.0;
        RankKey {
            similarity: hit.similarity,
            significance: hit.significance,
            executable: &hit.function.executable,
            sha256: &hit.function.sha256,
            address: hit.function.address,
        }
    }
}

impl PartialEq for Ranked {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Ranked {}

impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(other.key())
    }
}
