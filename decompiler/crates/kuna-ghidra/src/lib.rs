//! kuna-ghidra: the ghidra-mode process front-end — the native side of the
//! Ghidra GUI <-> decompiler stdin/stdout protocol.
//!
//! Upstream Ghidra's GUI talks to a separate native executable (installed
//! as `decompile`, built from the CORE+DECCORE+GHIDRA source groups —
//! `decompiler/cpp/ghidra_process.cc`, `ghidra_arch.cc`,
//! `ghidra_translate.cc`, ...) over a burst-framed binary protocol on
//! stdin/stdout: Ghidra sends *commands* (registerProgram, decompileAt,
//! setOptions, ...) carrying packed/XML documents, and while executing a
//! command the decompiler sends *queries* back (getPcode, getBytes,
//! getMappedSymbols, ... — 19 of them) that Ghidra answers from the open
//! program.  No `.sla` is ever loaded: every instruction's p-code, every
//! byte, symbol, type and comment arrives over the wire.
//!
//! This crate is the kuna port of that front-end, layered as:
//!
//! - [`protocol`] — the burst framing (`ghidra_arch.cc:50-248`), generic
//!   over `Read`/`Write`.
//! - [`ids`] — the protocol ElementIds (upstream numbers, wire-compatible).
//! - [`client`] — the 19 typed decompiler->Ghidra queries
//!   (`ghidra_arch.cc:445-896`).
//! - [`translate`] — the tspec `<sleigh>` decode into a real
//!   `AddrSpaceManager` (`ghidra_translate.cc:161-176`), which binds the
//!   packed protocol's address-space indices.
//! - [`process`] — the command loop (`ghidra_process.cc`), the `kuna_ghidra`
//!   binary's engine.
//!
//! ## Phase plan (see `docs/ghidra-integration.md`)
//!
//! - **Phase 1 (this crate today): wire-protocol complete, engine
//!   stubbed.**  The full command lifecycle, framing, tspec parse and query
//!   client are implemented and byte-exact; `decompileAt` answers the
//!   incomplete-function shape (empty payload + a not-implemented warning)
//!   so the Ghidra GUI degrades cleanly.  Deliberately no kuna-decomp
//!   dependency.
//! - **Phase 2: the engine bridge** — an `ArchitectureGhidra` analog
//!   backing kuna's loader/translator/scope/type/comment/string/cpool/
//!   inject/context seams with the query client, plus `Funcdata::encode`
//!   and the marked-up C output for the decompileAt response.
//! - **Phase 3: lazy scope/types** (the ScopeGhidra cache/holes model).
//! - **Phase 4: GUI parity** (structureGraph, signature commands, ...).
//!
//! The Ghidra-side extension that spawns this binary lives outside the
//! cargo workspace; `docs/ghidra-integration.md` documents the pairing,
//! the targeted Ghidra release, and the deliberate divergences (setOptions
//! tolerance, malformed-archid handling).

pub mod client;
pub mod ids;
pub mod process;
pub mod protocol;
pub mod provider;
pub mod translate;
