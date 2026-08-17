//! ghidra-sim v1: a mock-Java [`AnswerSource`] backed by kuna's OWN analysis
//! of a real vendored ELF.
//!
//! The sim plays the Java host's role with real program facts:
//!
//! | query | answer source |
//! |---|---|
//! | getPcode | the oracle's real Sleigh (`one_instruction` re-encoded as the wire `<inst>`) |
//! | getBytes | `ConsoleProgram::read_bytes` (the same `ObjectLoadImage` the CLI reads) |
//! | getCodeLabel | `ConsoleProgram::function_named_at` (the committed symbol set) |
//! | getRegister / getRegisterName | the real Sleigh's register table |
//! | getUserOpName | the real Sleigh's userop table |
//! | getComments | empty `<commentdb>` (always written, like Java) |
//! | getTrackedRegisters | empty `<tracked_pointset>` (always written, like Java) |
//! | getStringData | program bytes NUL-scanned + the biased size header |
//! | isNameUsed | constant `f` |
//! | **getMappedSymbols / getExternalRef** | **EMPTY — the Phase-2 reality this harness pins.** |
//!
//! ### PHASE-3 SEAM
//! `getMappedSymbols`/`getExternalRef` answer empty in v1, which is exactly
//! what makes today's `sub_ADDR`/`dat_ADDR`/raw-register output measurable.
//! Phase 3 plugs a `<mapsym>` encoder into [`SimOracle::respond`] at the
//! marked arms (building `<doc><mapsym><function|symbol…><addr><rangelist>`
//! from `ConsoleProgram::function_named_at`/`global_data_symbols`), after
//! which the pinned `getMappedSymbols == 0` / placeholder-rate numbers in
//! `ghidra_sim_e2e.rs` FLIP.  `getDataType`/`getNamespacePath` are the same
//! kind of seam (they need the Phase-3 `<type>` encoder / namespace model).
//!
//! The tspec is GENERATED from the loaded Sleigh's `AddrSpaceManager`
//! ([`generate_tspec`]), so the sim's packed space indices and the process's
//! tspec-derived indices agree **by construction** — the deepest interop
//! invariant of the packed protocol.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaError;
use kuna_base::marshal::{
    Decoder, ElementId, Encoder, PackedDecode, PackedEncode, ATTRIB_ID, ATTRIB_INDEX, ATTRIB_NAME,
    ATTRIB_OFFSET, ATTRIB_SIZE, ATTRIB_TYPE, ELEM_VOID,
};
use kuna_base::space::{spacetype, AddrSpaceManager, RegisterLookup};
use kuna_num::opcodes::{OpCode, OpcodeEncoder};
use kuna_num::pcoderaw::{VarnodeData, ELEM_SPACEID};
use kuna_sleigh::translate::{PcodeEmit, Translate, ATTRIB_CODE, ELEM_OP};

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_decomp::comment::ELEM_COMMENTDB;
use kuna_decomp::decompile_drive::print_c;
use kuna_decomp::options::{OptionDatabase, KUNA_OPTION_NAMES};
use kuna_ghidra::ids::{
    ATTRIB_MAXSIZE, ELEM_COMMAND_GETBYTES, ELEM_COMMAND_GETCODELABEL, ELEM_COMMAND_GETCOMMENTS,
    ELEM_COMMAND_GETCPOOLREF, ELEM_COMMAND_GETDATATYPE, ELEM_COMMAND_GETEXTERNALREF,
    ELEM_COMMAND_GETMAPPEDSYMBOLS, ELEM_COMMAND_GETNAMESPACEPATH, ELEM_COMMAND_GETPCODE,
    ELEM_COMMAND_GETREGISTER, ELEM_COMMAND_GETREGISTERNAME, ELEM_COMMAND_GETSTRINGDATA,
    ELEM_COMMAND_GETTRACKEDREGISTERS, ELEM_COMMAND_GETUSEROPNAME, ELEM_COMMAND_ISNAMEUSED,
    ELEM_UNIMPL,
};
use kuna_ghidra::protocol::{nibble_expand, string_data_size_header};
use kuna_sleigh::globalcontext::ELEM_TRACKED_POINTSET;

use super::{resp_bytes, resp_empty, resp_string, AnswerSource, QUERY_COMMAND_IDS};

/// `<inst>` — the getPcode response root (ELEM_INST, kuna-decomp
/// pcodeinject.rs; the numeric id is the wire contract).
pub const ELEM_INST: ElementId = ElementId::new("inst", 98);

/// The repository root (the tests run from the crate directory).
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root resolves")
}

/// A recording `PcodeEmit` capturing each op `one_instruction` produces.
#[derive(Default)]
pub struct RecordEmit {
    pub ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
}
impl PcodeEmit for RecordEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

/// Per-query traffic log (the behavioral-fingerprint pins).
#[derive(Debug, Default)]
pub struct QueryLog {
    /// Total queries by command element id.
    pub counts: BTreeMap<u32, u64>,
    /// Every getPcode address, in wire order (repeat asks included).
    pub pcode_addrs: Vec<u64>,
    /// Distinct addresses the sim decoded successfully.
    pub decoded_insts: BTreeSet<u64>,
}

/// The real-ELF mock-Java answer source (see the module docs).
pub struct SimOracle {
    pub prog: ConsoleProgram,
    /// The oracle Sleigh's address-space manager — shared space identities
    /// for every packed encode/decode the sim performs.
    pub manager: Rc<AddrSpaceManager>,
    pub big_endian: bool,
    pub unique_base: u64,
    /// The Sleigh's userop table, served index-by-index to the probe loop.
    pub user_ops: Vec<String>,
    /// Every register name of the language (the badness-scanner harvest).
    pub register_names: BTreeSet<String>,
    pub log: QueryLog,
    /// PHASE-3 SEAM (flushNative semantics): a label override consulted before
    /// the program lookup, letting a test change a "Java-side" symbol answer
    /// between decompiles to prove cache clearing.
    pub label_overrides: BTreeMap<u64, String>,
}

impl SimOracle {
    /// Bootstrap the oracle over a vendored ELF, exactly as `kuna decompile`
    /// does in-process: `load file` → the CLI's default options (`listing on` +
    /// the size-resolved mode preset) → `read symbols`.
    ///
    /// Returns `None` (with the canonical skip message the CI canary greps
    /// for) when the `.sla` specs are not built.
    pub fn bootstrap(binary: &Path) -> Option<SimOracle> {
        let root = repo_root();
        let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
        let bin = binary.to_str()?.to_string();
        let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "ghidra_sim: skipping (bootstrap failed, build `.sla` with `make specs`): {}",
                    e.explain()
                );
                return None;
            }
        };
        let size = std::fs::metadata(&bin).map(|m| m.len()).unwrap_or(0);
        apply_cli_default_options(&mut prog, size);
        prog.commit_pending_analysis()
            .expect("read symbols (analysis commit) must succeed");

        let translate = prog.arch().translate();
        let manager = translate.manager_rc();
        let sleigh = translate.as_sleigh().expect("oracle engine is a Sleigh");
        let big_endian = sleigh.is_big_endian();
        let unique_base = sleigh.get_unique_base() as u64;
        let mut user_ops = Vec::new();
        sleigh.get_user_op_names(&mut user_ops);
        #[allow(clippy::mutable_key_type)] // AddrSpace interior mutability does not affect Ord
        let mut reglist: BTreeMap<VarnodeData, String> = BTreeMap::new();
        sleigh.get_all_registers(&mut reglist);
        let register_names: BTreeSet<String> = reglist.into_values().collect();

        Some(SimOracle {
            prog,
            manager,
            big_endian,
            unique_base,
            user_ops,
            register_names,
            log: QueryLog::default(),
            label_overrides: BTreeMap::new(),
        })
    }

    /// The primary label the "Java side" knows at an address (a function name
    /// from the committed symbol set; the override map wins).  Public so the
    /// harness asserts the name echo against exactly what the sim SERVED —
    /// including a test-injected `label_overrides` entry — rather than against
    /// the underlying program lookup.
    pub fn code_label(&self, offset: u64) -> Option<String> {
        if let Some(over) = self.label_overrides.get(&offset) {
            return Some(over.clone());
        }
        self.prog.function_named_at(offset)
    }

    /// Re-encode one really-translated instruction as the wire `<inst>`
    /// document (`<inst offset=len><addr/><op>…</inst>` — the exact shape
    /// `GhidraTranslate::one_instruction` decodes).
    fn inst_doc(&self, addr: &Address, len: i32, emit: &RecordEmit) -> Vec<u8> {
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&ELEM_INST);
            e.write_signed_integer(&ATTRIB_OFFSET, len as i64);
            addr.encode(&mut e).expect("pc addr encodes");
            for (opc, out, ins) in &emit.ops {
                e.open_element(&ELEM_OP);
                e.write_signed_integer(&ATTRIB_SIZE, ins.len() as i64);
                e.write_opcode(&ATTRIB_CODE, *opc);
                match out {
                    Some(v) => encode_varnode(&mut e, v),
                    None => {
                        e.open_element(&ELEM_VOID);
                        e.close_element(&ELEM_VOID);
                    }
                }
                for (slot, v) in ins.iter().enumerate() {
                    // LOAD/STORE input 0 is the encoded space-id constant; the
                    // wire form is `<spaceid name=…>` (PcodeOpRaw::decode maps
                    // it back to the constant-space index form).
                    let is_spaceid = slot == 0
                        && matches!(opc, OpCode::CPUI_LOAD | OpCode::CPUI_STORE);
                    if is_spaceid {
                        let spc = v
                            .get_space_from_const(&self.manager)
                            .expect("spaceid constant resolves");
                        e.open_element(&ELEM_SPACEID);
                        e.write_space(&ATTRIB_NAME, &spc);
                        e.close_element(&ELEM_SPACEID);
                    } else {
                        encode_varnode(&mut e, v);
                    }
                }
                e.close_element(&ELEM_OP);
            }
            e.close_element(&ELEM_INST);
        }
        doc
    }

    /// getPcode: really translate the instruction at `addr` with the oracle
    /// Sleigh and answer the re-encoded `<inst>`; `<unimpl>` on an unimplemented
    /// instruction; empty (the caller's BadDataError) when decode fails.
    fn answer_pcode(&mut self, dec: &mut PackedDecode) -> Vec<u8> {
        let addr = Address::decode(dec).expect("getPcode carries an <addr>");
        self.log.pcode_addrs.push(addr.get_offset());
        let mut emit = RecordEmit::default();
        let result = {
            let sleigh = self
                .prog
                .arch()
                .translate()
                .as_sleigh()
                .expect("oracle engine is a Sleigh");
            sleigh.one_instruction(&mut emit, &addr)
        };
        match result {
            Ok(len) => {
                self.log.decoded_insts.insert(addr.get_offset());
                resp_string(&self.inst_doc(&addr, len, &emit))
            }
            Err(KunaError::Unimpl {
                instruction_length, ..
            }) => {
                let mut doc = Vec::new();
                {
                    let mut e = PackedEncode::new(&mut doc);
                    e.open_element(&ELEM_UNIMPL);
                    e.write_signed_integer(&ATTRIB_OFFSET, instruction_length as i64);
                    e.close_element(&ELEM_UNIMPL);
                }
                resp_string(&doc)
            }
            Err(_) => resp_empty(),
        }
    }

    /// getBytes: the oracle load image (the same bytes the CLI engine reads).
    fn answer_bytes(&mut self, dec: &mut PackedDecode) -> Vec<u8> {
        let (addr, size) = Address::decode_sized(dec).expect("getBytes carries a sized <addr>");
        match self.prog.read_bytes(addr.get_offset(), size as usize) {
            Some(bytes) => resp_bytes(&nibble_expand(&bytes)),
            None => resp_empty(), // DataUnavail — same signal the CLI loader raises
        }
    }

    /// getStringData: NUL-scan program bytes (the Java side would charset-decode;
    /// the vendored fixtures are ASCII/UTF-8) and frame with the biased
    /// 2-byte size header + truncation flag + nibble-doubled payload.
    fn answer_string_data(&mut self, dec: &mut PackedDecode) -> Vec<u8> {
        let max_bytes = dec
            .read_signed_integer_id(&ATTRIB_MAXSIZE)
            .expect("getStringData maxsize") as usize;
        let _type_name = dec.read_string_id(&ATTRIB_TYPE).expect("type name");
        let _type_id = dec.read_unsigned_integer_id(&ATTRIB_ID).expect("type id");
        let addr = Address::decode(dec).expect("getStringData <addr>");
        let mut collected: Vec<u8> = Vec::new();
        let mut truncated = true;
        for i in 0..max_bytes.max(1) as u64 {
            match self.prog.read_bytes(addr.get_offset() + i, 1) {
                Some(b) if b[0] != 0 => collected.push(b[0]),
                Some(_) => {
                    truncated = false;
                    break;
                }
                None => break, // ran off the image: serve what we have
            }
        }
        collected.push(0); // upstream ships the NUL terminator, doubled too
        let mut raw = Vec::new();
        raw.extend_from_slice(&string_data_size_header(collected.len() as u32));
        raw.push(u8::from(truncated));
        raw.extend_from_slice(&nibble_expand(&collected));
        resp_bytes(&raw)
    }
}

impl AnswerSource for SimOracle {
    fn respond(&mut self, doc: &[u8]) -> Vec<u8> {
        let manager = Rc::clone(&self.manager);
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(doc).expect("query doc ingests");
        let el = dec.open_element().expect("query has a command element");
        assert!(
            QUERY_COMMAND_IDS.contains(&el),
            "process issued a non-query element id {el} as a callback query"
        );
        *self.log.counts.entry(el).or_insert(0) += 1;

        if el == ELEM_COMMAND_GETUSEROPNAME.get_id() {
            let index = dec
                .read_signed_integer_id(&ATTRIB_INDEX)
                .expect("getUserOpName index");
            let name = usize::try_from(index)
                .ok()
                .and_then(|i| self.user_ops.get(i))
                .cloned()
                .unwrap_or_default();
            resp_string(name.as_bytes())
        } else if el == ELEM_COMMAND_GETREGISTER.get_id() {
            let name =
                String::from_utf8_lossy(&dec.read_string_id(&ATTRIB_NAME).expect("register name"))
                    .into_owned();
            let lookup = {
                let sleigh = self
                    .prog
                    .arch()
                    .translate()
                    .as_sleigh()
                    .expect("oracle engine is a Sleigh");
                RegisterLookup::get_register(sleigh, &name)
            };
            match lookup {
                Ok(v) => {
                    let spc = v.space.expect("register storage has a space");
                    let mut doc = Vec::new();
                    {
                        let mut e = PackedEncode::new(&mut doc);
                        Address::new(spc, v.offset)
                            .encode_sized(&mut e, v.size as i32)
                            .expect("register addr encodes");
                    }
                    resp_string(&doc)
                }
                Err(_) => resp_empty(),
            }
        } else if el == ELEM_COMMAND_GETREGISTERNAME.get_id() {
            let (addr, size) =
                Address::decode_sized(&mut dec).expect("getRegisterName sized <addr>");
            let name = {
                let sleigh = self
                    .prog
                    .arch()
                    .translate()
                    .as_sleigh()
                    .expect("oracle engine is a Sleigh");
                match addr.get_space() {
                    Some(spc) => sleigh.get_register_name(spc, addr.get_offset(), size),
                    None => String::new(),
                }
            };
            resp_string(name.as_bytes())
        } else if el == ELEM_COMMAND_GETPCODE.get_id() {
            self.answer_pcode(&mut dec)
        } else if el == ELEM_COMMAND_GETBYTES.get_id() {
            self.answer_bytes(&mut dec)
        } else if el == ELEM_COMMAND_GETCODELABEL.get_id() {
            let addr = Address::decode(&mut dec).expect("getCodeLabel <addr>");
            let label = self.code_label(addr.get_offset()).unwrap_or_default();
            resp_string(label.as_bytes())
        } else if el == ELEM_COMMAND_GETCOMMENTS.get_id() {
            // Always written, possibly empty (like Java).  A Phase-3 sim can
            // fill this from the analysis CommentFacts via the ported
            // CommentDatabaseInternal encode (p9_emit/comment.rs).
            let mut doc = Vec::new();
            {
                let mut e = PackedEncode::new(&mut doc);
                e.open_element(&ELEM_COMMENTDB);
                e.close_element(&ELEM_COMMENTDB);
            }
            resp_string(&doc)
        } else if el == ELEM_COMMAND_GETTRACKEDREGISTERS.get_id() {
            // Always written, possibly empty.  A Phase-3 sim can serve the real
            // pspec tracked set via `encode_tracked` (kuna-sleigh/globalcontext.rs).
            let mut doc = Vec::new();
            {
                let mut e = PackedEncode::new(&mut doc);
                e.open_element(&ELEM_TRACKED_POINTSET);
                e.close_element(&ELEM_TRACKED_POINTSET);
            }
            resp_string(&doc)
        } else if el == ELEM_COMMAND_GETSTRINGDATA.get_id() {
            self.answer_string_data(&mut dec)
        } else if el == ELEM_COMMAND_ISNAMEUSED.get_id() {
            resp_string(b"f")
        } else if el == ELEM_COMMAND_GETMAPPEDSYMBOLS.get_id()
            || el == ELEM_COMMAND_GETEXTERNALREF.get_id()
        {
            // ===================== PHASE-3 SEAM =====================
            // Answer EMPTY: the Phase-2 engine never issues these, and the
            // pinned `getMappedSymbols == 0` query-traffic number documents
            // that.  Phase 3 plugs the `<mapsym>` encoder in HERE (build
            // `<doc><mapsym><function|symbol…><addr><rangelist>` from
            // `self.prog.function_named_at` / `self.prog.global_data_symbols`
            // + `self.label_overrides`), and the harness pins flip.
            // ========================================================
            resp_empty()
        } else if el == ELEM_COMMAND_GETDATATYPE.get_id()
            || el == ELEM_COMMAND_GETNAMESPACEPATH.get_id()
            || el == ELEM_COMMAND_GETCPOOLREF.get_id()
        {
            // PHASE-3 SEAM: needs the `<type>` encoder / namespace model /
            // cpool records — empty is "not found", the lenient answer.
            resp_empty()
        } else {
            // The four inject variants (241/242/243/252): the wire cspec's
            // fixups are compiled engine-side, so these never fire today.
            resp_empty()
        }
    }
}

/// Encode one recorded varnode as a sized `<addr>` (the wire varnode form).
fn encode_varnode(e: &mut PackedEncode, v: &VarnodeData) {
    let spc = v.space.as_ref().expect("recorded varnode has a space");
    Address::new(Rc::clone(spc), v.offset)
        .encode_sized(e, v.size as i32)
        .expect("varnode encodes");
}

/// Generate the registerProgram tspec from the oracle Sleigh's space manager —
/// the `SleighLanguage.encodeTranslator` shape (`<sleigh>` → `<spaces>` →
/// one `<space|space_unique|space_other>` per language space).
///
/// The emitted `index` attributes are the oracle manager's REAL indices, so
/// the process's tspec-derived manager assigns identical numbers (sparse
/// `insert_space`); the engine-created special spaces (fspec/iop/join/stack)
/// are appended by `Architecture` init in the same order on both sides.
pub fn generate_tspec(manager: &AddrSpaceManager, big_endian: bool, unique_base: u64) -> Vec<u8> {
    let default_name = manager
        .get_default_code_space()
        .expect("default code space")
        .get_name()
        .to_string();
    let mut spaces = String::new();
    for i in 0..manager.num_spaces() {
        let Some(spc) = manager.get_space(i) else {
            continue;
        };
        let elem = match spc.get_type() {
            spacetype::IPTR_CONSTANT
            | spacetype::IPTR_FSPEC
            | spacetype::IPTR_IOP
            | spacetype::IPTR_JOIN
            | spacetype::IPTR_SPACEBASE => continue, // engine-created, never in a tspec
            spacetype::IPTR_INTERNAL => "space_unique",
            spacetype::IPTR_PROCESSOR if spc.get_name() == "OTHER" => "space_other",
            spacetype::IPTR_PROCESSOR if spc.is_overlay() => continue,
            spacetype::IPTR_PROCESSOR => "space",
        };
        spaces.push_str(&format!(
            "<{elem} name=\"{}\" index=\"{}\" size=\"{}\" bigendian=\"{}\" delay=\"{}\" physical=\"{}\"",
            spc.get_name(),
            spc.get_index(),
            spc.get_addr_size(),
            spc.is_big_endian(),
            spc.get_delay(),
            spc.has_physical(),
        ));
        if spc.get_word_size() != 1 {
            spaces.push_str(&format!(" wordsize=\"{}\"", spc.get_word_size()));
        }
        spaces.push_str("/>");
    }
    format!(
        "<sleigh bigendian=\"{big_endian}\" uniqbase=\"0x{unique_base:x}\">\
         <spaces defaultspace=\"{default_name}\">{spaces}</spaces></sleigh>"
    )
    .into_bytes()
}

/// Apply the options `kuna decompile` would inject for this binary size:
/// `listing on` plus the `auto`-resolved mode preset (aggressive under
/// 500 KiB) — the CLI-default ground truth the differential pins against.
pub fn apply_cli_default_options(prog: &mut ConsoleProgram, size_bytes: u64) {
    let mode = kuna_decomp::modes::resolve_mode_for_size(None, size_bytes)
        .expect("auto mode resolves");
    let mut options: Vec<(&str, &str)> = vec![("listing", "on")];
    if let Some(overrides) = kuna_decomp::modes::mode_overrides(mode) {
        options.extend(overrides.iter().copied());
    }
    for (name, value) in options {
        if KUNA_OPTION_NAMES.contains(&name) {
            prog.arch_mut()
                .set_kuna_option(name, value)
                .unwrap_or_else(|e| panic!("option {name}: {}", e.explain()));
            continue;
        }
        let id = prog.registry().find_element(name, 0);
        if id == 0 {
            continue; // a load-time env gate — inert for the in-process oracle
        }
        let db = OptionDatabase::new();
        db.set(prog.arch_mut(), id, value, "", "")
            .unwrap_or_else(|e| panic!("option {name}: {}", e.explain()));
    }
}

/// Decompile `name`@`addr` through the in-process CLI path — the SAME shared
/// per-function step the whole-binary surfaces run (`decompile_step::
/// decompile_one`, DIV-66), seeded exactly as `kuna_console::project` seeds it:
/// the DWARF stack-local re-seed plus the analysis's `call error(nonzero,…)`
/// CALL_RETURN flow prunes (project.rs builds the same list from
/// `arch.error_noreturn_callsites`; empty unless Listing + `noreturn_error`
/// found such sites).  Then `print_c`.  This is the differential ground truth.
pub fn decompile_cli(prog: &mut ConsoleProgram, name: &str, addr: &Address) -> String {
    let mapped = prog.dwarf_locals_for(addr.get_offset());
    let flow_ovr: Vec<(Address, u32)> = match addr.get_space() {
        Some(space) if !prog.arch().error_noreturn_callsites.is_empty() => prog
            .arch()
            .error_noreturn_callsites
            .iter()
            .map(|&off| {
                (
                    Address::new(Rc::clone(space), off),
                    kuna_decomp::overrides::flow_type::CALL_RETURN,
                )
            })
            .collect(),
        _ => Vec::new(),
    };
    let fd = kuna_console::decompile_step::decompile_one(
        prog.arch_mut(),
        name,
        addr.clone(),
        0,
        &kuna_console::decompile_step::DecompileSeed::plain(&mapped, &flow_ovr),
        &[],
    )
    .result
    .unwrap_or_else(|e| panic!("CLI-path decompile of {name} failed: {}", e.explain()));
    print_c(prog.arch_mut(), &fd)
}
