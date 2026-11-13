//! Port of `decompiler/cpp/kuna_dynamichashmax.{cc,hh}` — the dynamic-hash
//! collision budget (kuna GH-8467, S6 alias-facets / dynamic-symbol
//! attachment).
//!
//! When a HighVariable needs a storage-independent identity (a \e dynamic
//! Symbol keyed on a hash of its local data-flow),
//! [`DynamicHash::unique_hash`](crate::dynamic::DynamicHash::unique_hash)
//! cycles through hash methods and keeps the first whose same-address collision
//! list has size `<= maxduplicates` (hard-coded 8 upstream, `dynamic.cc:433`
//! and `:494`).  On dense, highly-repetitive code -- a fully unrolled AArch64
//! NEON byte-search loop -- more than 8 structurally identical Varnodes share
//! the same address with colliding hashes, so every method's collision list
//! overflows 8, the champion stays empty, and `uniqueHash` returns `hash == 0`;
//! `buildDynamicSymbol` then throws `RecovError("Unable to find unique hash for
//! varnode")` and the function aborts (GH-8467).
//!
//! kuna's own analysis uses [`MAX_DUPLICATES`] (16) -- large enough to admit a
//! unique hash for these dense cases while still fitting the existing 3-bit
//! total/position encoding (16 is the exact ceiling of that range).  The wire
//! path (`funcdata_encode.rs`) keeps upstream's 8, because Java recomputes the
//! same hash with that budget.

/// (kuna GH-8467) The same-address collision budget `DynamicHash::unique_hash`
/// is given for kuna's own analysis (upstream hard-codes 8).
pub const MAX_DUPLICATES: u32 = 16;
