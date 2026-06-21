//! kuna-sleigh: SLEIGH `.sla` reader and instruction-decode runtime.
//!
//! Ports the SLEIGH *consumer* side of the C++ tree (`decompiler/cpp/`):
//!
//! - `slaformat.{hh,cc}` (the compiled `.sla` file format)
//! - `context.{hh,cc}` (context bit-field caching)
//! - `slghsymbol.{hh,cc}` (symbol table as decoded from `.sla`)
//! - `slghpattern.{hh,cc}` (instruction patterns)
//! - `slghpatexpress.{hh,cc}` (pattern expressions)
//! - `semantics.{hh,cc}` (p-code construct templates)
//! - `sleighbase.{hh,cc}` (SleighBase: symbol-table owner)
//! - `sleigh.{hh,cc}` (the decode engine: ParserContext, PcodeCacher, Sleigh)
//!
//! Deliberately NOT ported: the SLEIGH **compiler** (`slgh_compile`,
//! `slghparse.y`, `slghscan.l`, `pcodecompile`, ...). `sleigh_opt` stays C++;
//! this crate only reads the `.sla` artifacts it produces.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).
pub mod translate;
pub mod context;
pub mod globalcontext;
pub mod slghpattern;
pub mod slghpatexpress;
pub mod slghsymbol;
pub mod semantics;
pub mod pcodecompile;
pub mod pcodeparse;
pub mod sleigh;
pub mod sleighbase;
pub mod slaformat;
pub mod loadimage;
pub mod loadimage_xml;
pub mod loadimage_object;
mod elf_plt;
pub mod memstate;
pub mod emulate;
pub mod emulateutil;
