//! Port of `decompiler/cpp/analyzesigs.{cc,hh}` — the console commands for
//! signature/feature generation (item `w8-sig-pack`).
//!
//! These are the `signature settings`, `print signatures`, `save signatures`,
//! `saveall signatures`, and `produce signatures` console commands.  Each drives
//! the [`signature`](crate::signature) pipeline over the current (or every)
//! decompiled function.
//!
//! # Boundaries (the W9 console surface)
//!
//! The whole file hangs off the W9 interactive-console subsystem
//! (`IfaceCapability` / `IfaceDecompCommand` / `IfaceStatus` / `IfaceDecompData`
//! — `decompiler/cpp/ifacedecomp.{hh,cc}`), which is **not yet ported**.  The
//! command *registration* (`registerCom`), the `dcp->fd` / `dcp->conf` data
//! pointers, the `status->fileoptr` output streams, the `istream` argument
//! parsing, and `iterateFunctionsAddrOrder` all belong to that wave.
//!
//! So this module ports the **decision bodies** of the commands as standalone,
//! testable functions that take the values the console would otherwise resolve
//! (the parsed settings integer, the `&Funcdata`, a `&mut dyn Encoder` /
//! output buffer), and stubs the console plumbing.  When W9 lands, the
//! `IfaceDecompCommand::execute` wrappers parse their `istream` and call these.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::Encoder;

use crate::funcdata::Funcdata;
use crate::signature::{
    debug_signature, set_settings, GraphSigManager, Hashword,
};

/// The capability name registered by `IfaceAnalyzeSigsCapability`
/// (analyzesigs.cc:27).
pub const CAPABILITY_NAME: &str = "analyzesigs";

/// The console commands this capability registers (C++
/// `IfaceAnalyzeSigsCapability::registerCommands`, analyzesigs.cc:30-38).
///
/// Each entry is the `(command-words, kind)` the C++ `registerCom` would file.
/// Exposed as data (rather than a `register` call) because the `IfaceStatus`
/// registry is the W9 surface; a future W9 `registerCommands` walks this list.
pub const COMMANDS: &[(&str, &str, AnalyzeSigsCommand)] = &[
    ("signature", "settings", AnalyzeSigsCommand::SignatureSettings),
    ("print", "signatures", AnalyzeSigsCommand::PrintSignatures),
    ("save", "signatures", AnalyzeSigsCommand::SaveSignatures),
    ("saveall", "signatures", AnalyzeSigsCommand::SaveAllSignatures),
    ("produce", "signatures", AnalyzeSigsCommand::ProduceSignatures),
];

/// Identity of an analyze-signatures console command (the C++
/// `IfcSignatureSettings` / `IfcPrintSignatures` / ... classes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyzeSigsCommand {
    /// `signature settings <val>` (`IfcSignatureSettings`).
    SignatureSettings,
    /// `print signatures [...]` (`IfcPrintSignatures`).
    PrintSignatures,
    /// `save signatures <file> [...]` (`IfcSaveSignatures`).
    SaveSignatures,
    /// `saveall signatures <file> [...]` (`IfcSaveAllSignatures`).
    SaveAllSignatures,
    /// `produce signatures <file> [...]` (`IfcProduceSignatures`).
    ProduceSignatures,
}

// ===========================================================================
// IfcSignatureSettings (analyzesigs.cc:44-55)
// ===========================================================================

/// Change global settings for signature generation (C++
/// `IfcSignatureSettings::execute`, analyzesigs.cc:44-55).
///
/// `mysetting` is the integer the console parsed from the command stream.  A
/// zero value is rejected (the C++ `throw IfaceParseError("Must specify settings
/// integer")`).  On success the process-global settings are updated and a
/// confirmation line is returned for the caller to print to `status->optr`.
pub fn signature_settings(mysetting: u32) -> KunaResult<String> {
    if mysetting == 0 {
        return Err(KunaError::parse("Must specify settings integer"));
    }
    set_settings(mysetting);
    Ok(format!("Signature settings set to {mysetting:x}"))
}

// ===========================================================================
// IfcPrintSignatures (analyzesigs.cc:63-80)
// ===========================================================================

/// Calculate and print signatures for a function (C++
/// `IfcPrintSignatures::execute`, analyzesigs.cc:63-80).
///
/// The C++ checks `dcp->fd != 0` and `dcp->fd->isProcComplete()` (the W9 data
/// pointer + the completeness gate), builds a `GraphSigManager`, configures it
/// from the stream (the optional `maxiter`), generates, and prints one feature
/// per line to `status->fileoptr`.
///
/// This port takes the resolved `&Funcdata` and the already-parsed
/// `maxiter_override`, enforces the completeness gate, and returns the printed
/// block of feature lines (`Signature::print`, signature.cc:62-68 — `*origin =
/// 0x........`).  The `printOrigin` of a `VarnodeSignature`/`BlockSignature`
/// needs the W8 print surface (`Varnode::printRaw`/`FlowBlock::printHeader`); a
/// generic feature prints its bare hash, so the printed text is built from
/// [`GraphSigManager::get_signature_vector`] (the sorted hash list) — the same
/// hashes, without the per-feature origin string (STUB(W8)).
pub fn print_signatures(
    fd: &Funcdata,
    maxiter_override: Option<i32>,
) -> KunaResult<String> {
    if !fd.is_proc_complete() {
        return Err(KunaError::lowlevel("Function has not been fully analyzed"));
    }
    let mut smanage = GraphSigManager::new()?;
    smanage.initialize_from_stream(maxiter_override);
    smanage.set_current_function(fd)?;
    smanage.generate(fd);

    let mut out = String::new();
    out.push_str(&format!("Signatures for {}\n", fd.get_name()));
    // Signature::print is `*<origin> = 0x........`; the origin string needs the
    // W8 print surface, so emit the hash list (sorted) one per line.
    for h in smanage.get_signature_vector() {
        out.push_str(&format!("* = 0x{h:08x}\n"));
    }
    Ok(out)
}

// ===========================================================================
// IfcSaveSignatures (analyzesigs.cc:88-117)
// ===========================================================================

/// Calculate signatures for a function and encode them (C++
/// `IfcSaveSignatures::execute`, analyzesigs.cc:88-117).
///
/// The C++ opens the named file, builds an `XmlEncode` over it, and writes the
/// full feature description (`smanage.encode`).  This port takes the resolved
/// `&Funcdata`, the parsed `maxiter_override`, and the destination encoder (the
/// file-open / filename parse is the W9 console surface), running the same
/// generate + `encode` path.  The `<varsig>`/`<blocksig>` encodes need the W8
/// marshal surface and STUB inside [`GraphSigManager::encode`].
pub fn save_signatures(
    fd: &Funcdata,
    maxiter_override: Option<i32>,
    encoder: &mut dyn Encoder,
) -> KunaResult<()> {
    if !fd.is_proc_complete() {
        return Err(KunaError::lowlevel("Function has not been fully analyzed"));
    }
    let mut smanage = GraphSigManager::new()?;
    smanage.initialize_from_stream(maxiter_override);
    smanage.set_current_function(fd)?;
    smanage.generate(fd);
    smanage.encode(encoder)
}

// ===========================================================================
// IfcProduceSignatures::iterationCallback (analyzesigs.cc:214-241)
// ===========================================================================

/// Compute the overall combined hash for one function (the per-function body of
/// C++ `IfcProduceSignatures::iterationCallback`, analyzesigs.cc:233-238).
///
/// The C++ re-decompiles the function (the W9 action-database drive), generates,
/// then writes `<name> = 0x<16-hex>` to the output file.  This port takes the
/// already-decompiled `&Funcdata` and the parsed `maxiter_override`, generates,
/// and returns the formatted line (the re-decompilation + file write are the W9
/// surface).
pub fn produce_signature_line(
    fd: &Funcdata,
    maxiter_override: Option<i32>,
) -> KunaResult<String> {
    let mut smanage = GraphSigManager::new()?;
    smanage.initialize_from_stream(maxiter_override);
    smanage.set_current_function(fd)?;
    smanage.generate(fd);
    let finalsig: Hashword = smanage.get_overall_hash();
    Ok(format!("{} = 0x{:016x}", fd.get_name(), finalsig))
}

// ===========================================================================
// IfcSaveAllSignatures::iterationCallback (analyzesigs.cc:165-204)
// ===========================================================================

/// Encode the per-function record body of `saveall signatures` (the encode tail
/// of C++ `IfcSaveAllSignatures::iterationCallback`, analyzesigs.cc:184-201).
///
/// The C++ writes a binary header (space index, offset, sig count, name length,
/// name) then the `XmlEncode` of the manager.  The header is built from
/// `fd->getAddress()` + `fd->getName()`; the binary framing and the
/// per-architecture function iteration (`iterateFunctionsAddrOrder`, the action
/// re-drive, the `status->fileoptr` binary stream) are the W9 console surface
/// and STUB.  This port runs the generate + `encode` core (the part that is not
/// console plumbing) into the given encoder, returning the number of features.
pub fn saveall_signature_record(
    fd: &Funcdata,
    maxiter_override: Option<i32>,
    encoder: &mut dyn Encoder,
) -> KunaResult<i32> {
    let mut smanage = GraphSigManager::new()?;
    smanage.initialize_from_stream(maxiter_override);
    smanage.set_current_function(fd)?;
    smanage.generate(fd);
    let numsigs = smanage.num_signatures();
    if numsigs != 0 {
        smanage.encode(encoder)?;
    }
    Ok(numsigs)
}

/// Calculate signatures for a function and encode the full description with the
/// `debug` variant (the `debugSignature`-style sorted full encode), the body
/// behind a future `IfcSaveSignatures` debug mode.  Kept as a thin wrapper over
/// [`debug_signature`](crate::signature::debug_signature) so callers have a
/// single entry point that mirrors the C++ free function.
pub fn save_debug_signatures(fd: &Funcdata, encoder: &mut dyn Encoder) -> KunaResult<()> {
    debug_signature(fd, encoder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use crate::context::{ArchContext, BlockId, TypeOp};
    use crate::signature::{set_settings, sig_mods, SETTINGS_TEST_LOCK};
    use kuna_num::opcodes::OpCode;

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn add_op(fd: &mut Funcdata, bl: BlockId, opc: OpCode, ninput: i32, pc: Address) -> OpId {
        let op = fd.new_op(ninput, pc);
        fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
        fd.op_insert(op, bl, None);
        op
    }

    use crate::context::OpId;

    /// Build a small complete function (input + add + return) and mark it
    /// processing-complete so the print gate passes.
    fn build_complete_fd() -> Funcdata {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);

        let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let p = fd.set_input_varnode(p).unwrap();
        let c = fd.new_constant(4, 5);
        let add = add_op(&mut fd, bl, OpCode::CPUI_INT_ADD, 2, Address::new(Rc::clone(&rs), 0x1000));
        let out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), add).unwrap();
        fd.op_set_input(add, p, 0).unwrap();
        fd.op_set_input(add, c, 1).unwrap();
        let ret = add_op(&mut fd, bl, OpCode::CPUI_RETURN, 2, Address::new(Rc::clone(&rs), 0x1004));
        let iop = fd.new_constant(8, 0);
        fd.op_set_input(ret, iop, 0).unwrap();
        fd.op_set_input(ret, out, 1).unwrap();
        fd.structure_reset();
        // Mark processing-complete so the print/save gates pass (the C++
        // isProcComplete() guard).
        fd.set_flag_raw(crate::funcdata::funcdata_flags::processing_complete);
        fd
    }

    #[test]
    fn settings_rejects_zero_and_accepts_nonzero() {
        let _guard = SETTINGS_TEST_LOCK.lock().unwrap();
        assert!(signature_settings(0).is_err());
        let msg = signature_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1).unwrap();
        assert!(msg.starts_with("Signature settings set to"), "got: {msg}");
        // The setting is now live.
        assert_eq!(get_settings_for_test(), (sig_mods::SIG_DONOTUSE_CONST << 2) | 1);
    }

    fn get_settings_for_test() -> u32 {
        crate::signature::get_settings()
    }

    #[test]
    fn print_signatures_requires_complete_function() {
        let _guard = SETTINGS_TEST_LOCK.lock().unwrap();
        set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);
        // An incomplete (default) function fails the gate.
        let incomplete = build_fd();
        assert!(print_signatures(&incomplete, None).is_err());
    }

    #[test]
    fn print_signatures_emits_feature_lines() {
        let _guard = SETTINGS_TEST_LOCK.lock().unwrap();
        set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);
        let fd = build_complete_fd();
        let out = print_signatures(&fd, None).unwrap();
        assert!(out.contains("Signatures for func"), "got: {out}");
        assert!(out.contains("* = 0x"), "expected feature lines: {out}");
    }

    #[test]
    fn produce_signature_line_formats_overall_hash() {
        let _guard = SETTINGS_TEST_LOCK.lock().unwrap();
        set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);
        let fd = build_complete_fd();
        let line = produce_signature_line(&fd, None).unwrap();
        // "<name> = 0x" + 16 hex digits.
        assert!(line.starts_with("func = 0x"), "got: {line}");
        let hex = line.trim_start_matches("func = 0x");
        assert_eq!(hex.len(), 16, "expected 16 hex digits, got: {line}");
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn commands_table_lists_all_five() {
        assert_eq!(COMMANDS.len(), 5);
        assert_eq!(CAPABILITY_NAME, "analyzesigs");
        assert!(COMMANDS.iter().any(|(_, _, k)| *k == AnalyzeSigsCommand::SignatureSettings));
        assert!(COMMANDS.iter().any(|(_, _, k)| *k == AnalyzeSigsCommand::ProduceSignatures));
    }
}
