//! Content-addressed reference binaries and Kuna's native BSim signatures.
//!
//! Local imports and optional authenticated HTTP publication share the same
//! content-addressed store.

mod store;

pub use store::{ImportOutcome, ImportResult, MergeReport, Store, StoreError};

use serde::{Deserialize, Serialize};

pub const MAX_QUERY_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_QUERY_FEATURES: usize = 65_536;
pub const MAX_NAME_BYTES: usize = 4096;
pub const MAX_MATCHES: usize = 1_000;
pub const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_ARTIFACT_BYTES: u64 = 512 * 1024 * 1024;
pub const MAX_PUBLISH_METADATA_BYTES: usize = 16 * 1024;
pub const PUBLISH_CONTENT_TYPE: &str = "application/vnd.kuna.bsim.publish";

/// JSON metadata in a publish body, framed by a big-endian u32 byte length
/// and followed by the unmodified executable bytes.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishRequest {
    pub executable: String,
    pub address: Option<u64>,
    pub symbol: Option<String>,
    pub rename: Option<String>,
    pub profile: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PublishResponse {
    pub sha256: String,
    pub address: u64,
    pub name: String,
    pub profile: String,
    pub outcome: ImportOutcome,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProfilesResponse {
    pub profiles: Vec<ProfileInfo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProfileInfo {
    pub profile: String,
    pub settings: u32,
    pub executables: usize,
    pub incomplete_executables: usize,
    pub functions: usize,
}

/// An exact intersection of the supplied filters, within a single profile.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryFilters {
    pub executable: Option<String>,
    pub architecture: Option<String>,
    pub compiler: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryRequest {
    pub profile: String,
    pub settings: u32,
    /// Feature hashes sorted ascending, retaining repeated hashes.
    pub features: Vec<u32>,
    pub min_similarity: f64,
    pub max_matches: usize,
    #[serde(default)]
    pub filters: QueryFilters,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct QueryResponse {
    pub matches: Vec<Match>,
    pub considered: usize,
    pub incomplete_executables: usize,
    pub min_similarity: f64,
    pub truncated: bool,
}

/// Exact byte range of the thin Mach-O selected from a universal original.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MachOSlice {
    pub offset: u64,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StoredFunction {
    pub name: String,
    pub address: u64,
    pub executable: String,
    pub architecture: String,
    pub compiler: String,
    pub md5: String,
    pub sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macho_slice: Option<MachOSlice>,
    pub complete: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Match {
    #[serde(flatten)]
    pub function: StoredFunction,
    pub similarity: f64,
    pub significance: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn canonical_sha256(value: &str) -> bool {
    is_lower_hex(value, 64)
}

fn is_lower_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
