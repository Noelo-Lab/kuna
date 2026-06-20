//! kuna-base: foundation layer of the kuna Rust port.
//!
//! Ports the dependency-free bottom of the C++ tree (`decompiler/cpp/`):
//!
//! - `types.h` (fixed-width integer typedefs and masks)
//! - `error.hh` (error type hierarchy; see ADR 0004)
//! - `xml.y`/`xml.cc` (hand-ported XML parser, replacing the bison grammar)
//! - `marshal.{hh,cc}` (Encoder/Decoder: XML and Packed formats)
//! - `opcodes.{hh,cc}` (the `OpCode` enum and name tables)
//! - `space.{hh,cc}` (address spaces)
//! - `address.{hh,cc}` (Address, Range, SeqNum)
//! - `pcoderaw.{hh,cc}` (raw varnode/op data prior to Funcdata ownership)
//! - `globalcontext.{hh,cc}` (context database)
//! - `partmap.hh` / `rangemap.hh` (interval container templates)
//! - `compression.{hh,cc}` (zlib stream wrapper, via `flate2`)
//! - `crc32.{hh,cc}` (CRC32 table)
//! - `filemanage.{hh,cc}` (path discovery helpers)
//! - `translate.{hh,cc}` / `loadimage.{hh,cc}` (Translate/LoadImage as traits)
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`), which
//! denies `std::collections::HashMap`/`HashSet` per ADR 0002.

pub mod error;
pub mod types;
pub mod partmap;
pub mod rangemap;
pub mod xml;
pub mod marshal;
pub mod space;
pub mod address;
pub mod cfmt;
pub mod crc32;
pub mod compression;
pub mod filemanage;
