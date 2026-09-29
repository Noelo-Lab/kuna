//! SLEIGH instruction decoding, p-code execution and shared compiler machinery.
//!
//! The runtime reads compiled `.sla` files, resolves instruction patterns and
//! emits p-code. Pattern builders, symbols and semantic templates also support
//! `kuna-slacomp`, which owns the SLEIGH source parser and compiler driver.
//! P-code expression construction is shared with the runtime snippet parser.
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
pub mod kuna_ctxsnapshot;
pub mod kuna_sharedbytes;
pub mod loadimage_xml;
pub mod memstate;
pub mod emulate;
pub mod emulateutil;
