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
use kuna_base::address::{ATTRIB_FIRST, ATTRIB_LAST, ELEM_RANGELIST};
use kuna_base::marshal::{
    Decoder, ElementId, Encoder, PackedDecode, PackedEncode, ATTRIB_ID, ATTRIB_INDEX,
    ATTRIB_METATYPE, ATTRIB_MODEL, ATTRIB_NAME, ATTRIB_NAMELOCK, ATTRIB_OFFSET, ATTRIB_READONLY,
    ATTRIB_SIZE, ATTRIB_TYPE, ATTRIB_TYPELOCK, ATTRIB_VAL, ELEM_SYMBOL, ELEM_VOID,
};
use kuna_base::space::{spacetype, AddrSpaceManager, RegisterLookup};
use kuna_num::opcodes::{OpCode, OpcodeEncoder};
use kuna_num::pcoderaw::{VarnodeData, ELEM_SPACEID};
use kuna_sleigh::translate::{PcodeEmit, Translate, ATTRIB_CODE, ELEM_OP};

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_decomp::comment::ELEM_COMMENTDB;
use kuna_decomp::decompile_drive::print_c;
use kuna_decomp::dtype::{type_metatype, Datatype, DatatypeKind, ELEM_TYPEREF};
use kuna_decomp::fspec::PrototypePieces;
use kuna_decomp::funcdata_encode::ELEM_FUNCTION;
use kuna_decomp::inject_sleigh::SleighInjectEngine;
use kuna_decomp::options::{OptionDatabase, KUNA_OPTION_NAMES};
use kuna_decomp::pcodeinject::{
    InjectContext, CALLFIXUP_TYPE, CALLOTHERFIXUP_TYPE, ELEM_CONTEXT,
};
use kuna_decomp::prettyprint::ids::{ELEM_FIELD, ELEM_TYPE};
use kuna_decomp::remote_provider::{
    ATTRIB_CAT, ATTRIB_DOTDOTDOT, ATTRIB_EXTRAPOP, ATTRIB_LOCK, ATTRIB_MAIN, ATTRIB_NORETURN,
    ATTRIB_VOIDLOCK, ELEM_HASH, ELEM_HOLE, ELEM_LOCALDB, ELEM_MAPSYM, ELEM_PARENT,
    ELEM_PROTOTYPE, ELEM_RETURNSYM, ELEM_SCOPE, ELEM_SYMBOLLIST,
};
use kuna_ghidra::ids::{
    ATTRIB_MAXSIZE, ELEM_COMMAND_GETBYTES, ELEM_COMMAND_GETCALLFIXUP,
    ELEM_COMMAND_GETCALLOTHERFIXUP, ELEM_COMMAND_GETCODELABEL, ELEM_COMMAND_GETCOMMENTS,
    ELEM_COMMAND_GETCPOOLREF, ELEM_COMMAND_GETDATATYPE, ELEM_COMMAND_GETEXTERNALREF,
    ELEM_COMMAND_GETMAPPEDSYMBOLS, ELEM_COMMAND_GETNAMESPACEPATH, ELEM_COMMAND_GETPCODE,
    ELEM_COMMAND_GETREGISTER, ELEM_COMMAND_GETREGISTERNAME, ELEM_COMMAND_GETSTRINGDATA,
    ELEM_COMMAND_GETTRACKEDREGISTERS, ELEM_COMMAND_GETUSEROPNAME, ELEM_COMMAND_ISNAMEUSED,
    ELEM_UNIMPL,
};
use kuna_ghidra::protocol::{nibble_expand, string_data_size_header};
use kuna_sleigh::globalcontext::ELEM_TRACKED_POINTSET;

use super::{resp_bytes, resp_empty, resp_exception, resp_string, AnswerSource, QUERY_COMMAND_IDS};

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
    /// Every getRegister name asked for, in wire order (repeats included).
    pub register_probes: Vec<String>,
    /// The subset of [`QueryLog::register_probes`] the language does not
    /// define — each one is a thrown `No Register Defined` in real Ghidra and
    /// an `Unexpected Exception` ERROR record in its log.
    pub register_probe_failures: Vec<String>,
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
    /// Committed global data symbols keyed by start VMA: `(name, byte size)`.
    /// The `<mapsym><symbol>` answer source.
    pub data_symbols: BTreeMap<u64, (String, i64)>,
    /// Declared (locked) callee prototypes keyed by entry offset — the
    /// analysis-committed libproto/DWARF signatures the CLI path consumes; the
    /// `<mapsym><function><prototype>` answer source.
    pub callee_pieces: BTreeMap<u64, PrototypePieces>,
    /// Read-only image ranges (READONLY|CODE sections) — the `<hole>`
    /// mutability source (the CLI paints the same const-ness from ELF flags).
    pub readonly_ranges: Vec<(u64, u64)>,
    /// PHASE-3 SEAM (host context): tracked-register values the "Java side"
    /// reports on TOP of the pspec defaults (a user 'Set Register Value'),
    /// served by the getTrackedRegisters answer — lets a test prove a
    /// host-side context fact changes the output.
    pub tracked_overrides: Vec<kuna_sleigh::globalcontext::TrackedContext>,
    /// PHASE-4 SEAM (rename/retype persistence): host-committed non-parameter
    /// locals served in the function's `<localdb>` (keyed by entry offset) —
    /// the shape Java's `LocalSymbolMap.grabFromFunction` produces after
    /// `HighFunctionDBUtil.updateDBVariable` wrote a user rename/retype to
    /// the database.  Lets a test prove the GUI rename SURVIVES the
    /// event-driven re-decompile.
    pub local_var_overrides: BTreeMap<u64, Vec<HostLocalVar>>,
    /// Force the getPcodeInject answer down one of the host's two failure
    /// paths instead of lifting the payload (`None` = answer for real).
    pub inject_fault: Option<InjectFault>,
}

/// The two ways `DecompileCallback.getPcodeInject` declines to answer, which
/// reach the engine as different wire shapes and must not be conflated.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InjectFault {
    /// The name is not in the host's `PcodeInjectLibrary`: a thrown
    /// `NotFoundException` (DecompileCallback.java:296-299).
    NotFound,
    /// No instruction at `baseAddr`, or the payload lifted to nothing: the
    /// callback returns without encoding, so Java writes an EMPTY response
    /// (DecompileCallback.java:311-316 and :331-333).
    NoPcode,
}

/// One host-committed local for [`SimOracle::local_var_overrides`].
#[derive(Clone)]
pub struct HostLocalVar {
    /// The (user-chosen) name; served namelocked.
    pub name: String,
    /// Storage space name.
    pub space: String,
    /// Storage offset.
    pub offset: u64,
    /// Storage size in bytes.
    pub size: i64,
    /// First-use code offset (the uselimit rangelist start); `None` = an
    /// address-tied (whole-function) local, served with an empty rangelist.
    pub first_use: Option<u64>,
    /// A DYNAMIC (hash) entry instead of `<addr>` storage — what Java writes
    /// for every `requiresDynamicStorage` variable (unique-space
    /// representatives, `splitOutMergeGroup` products).  0 = mapped storage.
    pub hash: u64,
    /// Served typelocked?  Java's `grabFromFunction` sends `typelock=false`
    /// for an `Undefined`-typed DB local — the shape a plain GUI RENAME
    /// produces (`updateDBVariable(name, null)`), which kuna must keep as a
    /// name RECOMMENDATION; `true` is the retype shape (a real type), which
    /// survives as a seeded Symbol.
    pub typelock: bool,
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

        let data_symbols: BTreeMap<u64, (String, i64)> = prog
            .global_data_symbols()
            .into_iter()
            .map(|(name, vma, size)| (vma, (name, size.max(1))))
            .collect();
        let callee_pieces: BTreeMap<u64, PrototypePieces> = prog
            .arch()
            .symboltab
            .build_callee_proto_pieces()
            .into_iter()
            .map(|(_, off, pieces)| (off, pieces))
            .collect();
        use kuna_sleigh::loadimage::section_flags;
        let readonly_ranges: Vec<(u64, u64)> = prog
            .sections()
            .into_iter()
            .filter(|(_, _, flags)| {
                (flags & (section_flags::READONLY | section_flags::CODE)) != 0
                    && (flags & (section_flags::UNALLOC | section_flags::NOLOAD)) == 0
            })
            .map(|(vma, size, _)| (vma, vma + size.saturating_sub(1)))
            .collect();

        Some(SimOracle {
            prog,
            manager,
            big_endian,
            unique_base,
            user_ops,
            register_names,
            log: QueryLog::default(),
            label_overrides: BTreeMap::new(),
            data_symbols,
            callee_pieces,
            readonly_ranges,
            tracked_overrides: Vec::new(),
            local_var_overrides: BTreeMap::new(),
            inject_fault: None,
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

    /// getCallOtherFixup / getCallFixup: the `DecompileCallback.getPcodeInject`
    /// role — decode the `<context>` the engine sent, lift the named payload
    /// against it with the oracle's OWN (locally compiled) inject library, and
    /// answer the `<inst>`.
    ///
    /// The host's three answers are distinct and the sim keeps them so: an
    /// unregistered name THROWS a `NotFoundException` frame, a missing
    /// instruction or an empty lift answers EMPTY, and only a real lift answers
    /// a document.  Under the upstream contract an empty response IS a failure,
    /// so answering everything empty would make a working engine look broken.
    fn answer_inject(&self, el: u32, dec: &mut PackedDecode) -> Vec<u8> {
        let name = dec.read_string_id(&ATTRIB_NAME).expect("inject name");
        let ptype = if el == ELEM_COMMAND_GETCALLFIXUP.get_id() {
            CALLFIXUP_TYPE
        } else {
            CALLOTHERFIXUP_TYPE
        };
        let mut ctx = decode_inject_context(dec);
        let baseaddr = ctx.baseaddr.clone().expect("<context> carries a baseaddr");
        let not_found = || {
            resp_exception(
                "ghidra.util.exception.NotFoundException",
                &format!(
                    "No p-code injection with name: {}",
                    String::from_utf8_lossy(&name)
                ),
            )
        };
        match self.inject_fault {
            Some(InjectFault::NotFound) => return not_found(),
            Some(InjectFault::NoPcode) => return resp_empty(),
            None => {}
        }

        let arch = self.prog.arch();
        let id = arch.pcodeinjectlib.base.get_payload_id(ptype, &name);
        if id < 0 {
            return not_found();
        }
        let sleigh = arch
            .translate()
            .as_sleigh()
            .expect("oracle engine is a Sleigh");
        // Java re-derives nextAddr from the real instruction's default
        // fall-through, and answers empty when there is no instruction there
        // (DecompileCallback.java:311-320).
        let mut probe = RecordEmit::default();
        let len = match sleigh.one_instruction(&mut probe, &baseaddr) {
            Ok(len) => len,
            Err(_) => return resp_empty(),
        };
        ctx.nextaddr = Some(&baseaddr + len as i64);

        let payload = arch.pcodeinjectlib.get_payload(id);
        let Some(tpl) = arch.pcodeinjectlib.get_tpl(id) else {
            return resp_empty();
        };
        let engine = SleighInjectEngine::new(
            Rc::clone(arch.manage().get_constant_space().expect("const space")),
            Rc::clone(arch.manage().get_unique_space().expect("unique space")),
            Rc::clone(arch.manage().get_default_code_space().expect("code space")),
        );
        let mut emit = RecordEmit::default();
        if engine.emit_payload(payload, tpl, &mut ctx, &mut emit).is_err() {
            return resp_empty();
        }
        resp_string(&self.inst_doc(&baseaddr, len, &emit))
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

    // ------------------------------------------------------------------
    // Phase-3 answers: <doc><mapsym> / <hole> (the DecompileCallback shapes)
    // ------------------------------------------------------------------

    /// getMappedSymbols/getExternalRef: the Java `DecompileCallback.
    /// getMappedSymbols` dispatch over the oracle's committed program facts —
    /// a `<function>` answer at a function entry, a `<symbol>` answer inside a
    /// committed data symbol, else a one-byte `<hole>` whose mutability comes
    /// from the section flags (exactly the facts the CLI path consumes).
    fn answer_mapped_symbols(&mut self, dec: &mut PackedDecode) -> Vec<u8> {
        let addr = Address::decode(dec).expect("getMappedSymbols <addr>");
        let offset = addr.get_offset();
        if let Some(name) = self.code_label(offset) {
            let noreturn = self
                .prog
                .arch()
                .symboltab
                .function_is_no_return_across_scopes(&addr);
            let pieces = self.callee_pieces.get(&offset).cloned();
            return resp_string(&self.function_doc(&addr, &name, noreturn, pieces.as_ref()));
        }
        if let Some((start, (name, size))) = self
            .data_symbols
            .range(..=offset)
            .next_back()
            .map(|(s, v)| (*s, v.clone()))
        {
            if offset < start.wrapping_add(size as u64) {
                let sym_addr = Address::new(
                    Rc::clone(addr.get_space().expect("ram address has a space")),
                    start,
                );
                return resp_string(&self.symbol_doc(&sym_addr, &name, size as i32));
            }
        }
        resp_string(&self.hole_doc(&addr))
    }

    /// `<doc id=0><mapsym><function …>…</function><addr/><rangelist/></mapsym></doc>`
    /// (Java `HighFunctionSymbol.encode` → `HighFunction.encode`).
    fn function_doc(
        &self,
        entry: &Address,
        name: &str,
        noreturn: bool,
        pieces: Option<&PrototypePieces>,
    ) -> Vec<u8> {
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&kuna_ghidra::ids::ELEM_DOC);
            e.write_unsigned_integer(&ATTRIB_ID, 0); // global namespace
            e.open_element(&ELEM_MAPSYM);
            e.open_element(&ELEM_FUNCTION);
            // A stable fake host-database id (never in the internal 0x40… range).
            e.write_unsigned_integer(&ATTRIB_ID, entry.get_offset() | 0x1_0000_0000);
            e.write_string(&ATTRIB_NAME, name.as_bytes());
            e.write_signed_integer(&ATTRIB_SIZE, 1);
            if noreturn {
                e.write_bool(&ATTRIB_NORETURN, true);
            }
            entry.encode(&mut e).expect("entry addr encodes");
            let locals = self
                .local_var_overrides
                .get(&entry.get_offset())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            if pieces.is_some() || !locals.is_empty() {
                self.encode_localdb(&mut e, name, pieces, locals);
            }
            if let Some(p) = pieces {
                self.encode_prototype(&mut e, p);
            }
            e.close_element(&ELEM_FUNCTION);
            // The mapping SymbolEntry: <addr size=1/> + empty <rangelist/>.
            entry.encode_sized(&mut e, 1).expect("mapping addr encodes");
            e.open_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_MAPSYM);
            e.close_element(&kuna_ghidra::ids::ELEM_DOC);
        }
        doc
    }

    /// `<localdb lock main><scope><parent id=0/><rangelist/><symbollist>` with
    /// one cat-0 `<mapsym><symbol>` per declared parameter (Java
    /// `LocalSymbolMap.encodeLocalDb`) plus one plain `<mapsym>` per
    /// host-committed local override ([`HostLocalVar`]).
    fn encode_localdb(
        &self,
        e: &mut PackedEncode,
        fname: &str,
        pieces: Option<&PrototypePieces>,
        locals: &[HostLocalVar],
    ) {
        let ram = self
            .manager
            .get_default_code_space()
            .expect("default code space")
            .clone();
        e.open_element(&ELEM_LOCALDB);
        e.write_bool(&ATTRIB_LOCK, false);
        e.write_space(&ATTRIB_MAIN, &ram);
        e.open_element(&ELEM_SCOPE);
        e.write_string(&ATTRIB_NAME, fname.as_bytes());
        e.open_element(&ELEM_PARENT);
        e.write_unsigned_integer(&ATTRIB_ID, 0);
        e.close_element(&ELEM_PARENT);
        e.open_element(&ELEM_RANGELIST);
        e.close_element(&ELEM_RANGELIST);
        e.open_element(&ELEM_SYMBOLLIST);
        for l in locals {
            let spc = self
                .manager
                .get_space_by_name(&l.space)
                .unwrap_or_else(|| panic!("override space {}", l.space))
                .clone();
            e.open_element(&ELEM_MAPSYM);
            if l.hash != 0 {
                e.write_string(&kuna_base::marshal::ATTRIB_TYPE, b"dynamic");
            }
            e.open_element(&ELEM_SYMBOL);
            // A stable fake host-database id (never internal-range).
            e.write_unsigned_integer(&ATTRIB_ID, l.offset | 0x2_0000_0000);
            e.write_string(&ATTRIB_NAME, l.name.as_bytes());
            e.write_bool(&ATTRIB_TYPELOCK, l.typelock);
            e.write_bool(&ATTRIB_NAMELOCK, true);
            e.write_signed_integer(&ATTRIB_CAT, -1);
            // A sized base type (metatype uint) — what grabFromFunction sends
            // for a typed DB local.
            e.open_element(&kuna_decomp::prettyprint::ids::ELEM_TYPE);
            e.write_string(&ATTRIB_NAME, b"");
            e.write_string(&kuna_base::marshal::ATTRIB_METATYPE, b"uint");
            e.write_signed_integer(&ATTRIB_SIZE, l.size);
            e.close_element(&kuna_decomp::prettyprint::ids::ELEM_TYPE);
            e.close_element(&ELEM_SYMBOL);
            // Storage entry: a DYNAMIC <hash val> or <addr space offset size/>,
            // then the uselimit.
            if l.hash != 0 {
                e.open_element(&ELEM_HASH);
                e.write_unsigned_integer(&ATTRIB_VAL, l.hash);
                e.close_element(&ELEM_HASH);
            } else {
                e.open_element(&kuna_base::address::ELEM_ADDR);
                spc.encode_attributes_sized(e, l.offset, l.size as i32)
                    .expect("override storage encodes");
                e.close_element(&kuna_base::address::ELEM_ADDR);
            }
            e.open_element(&ELEM_RANGELIST);
            if let Some(first) = l.first_use {
                e.open_element(&kuna_base::address::ELEM_RANGE);
                e.write_space(&kuna_base::marshal::ATTRIB_SPACE, &ram);
                e.write_unsigned_integer(&kuna_base::address::ATTRIB_FIRST, first);
                e.write_unsigned_integer(&kuna_base::address::ATTRIB_LAST, first);
                e.close_element(&kuna_base::address::ELEM_RANGE);
            }
            e.close_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_MAPSYM);
        }
        let Some(pieces) = pieces else {
            e.close_element(&ELEM_SYMBOLLIST);
            e.close_element(&ELEM_SCOPE);
            e.close_element(&ELEM_LOCALDB);
            return;
        };
        for (i, ct) in pieces.intypes.iter().enumerate() {
            e.open_element(&ELEM_MAPSYM);
            e.open_element(&ELEM_SYMBOL);
            let pname = pieces
                .innames
                .get(i)
                .cloned()
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| format!("param_{}", i + 1));
            e.write_string(&ATTRIB_NAME, pname.as_bytes());
            e.write_bool(&ATTRIB_TYPELOCK, true);
            e.write_bool(&ATTRIB_NAMELOCK, true);
            e.write_signed_integer(&ATTRIB_CAT, 0);
            e.write_unsigned_integer(&ATTRIB_INDEX, i as u64);
            self.encode_wire_type(e, ct);
            e.close_element(&ELEM_SYMBOL);
            // Storage entry: a dynamic <hash> (the decoder needs no static
            // storage for a parameter) + empty <rangelist/>.
            e.open_element(&ELEM_HASH);
            e.write_unsigned_integer(&ATTRIB_VAL, 1);
            e.close_element(&ELEM_HASH);
            e.open_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_MAPSYM);
        }
        e.close_element(&ELEM_SYMBOLLIST);
        e.close_element(&ELEM_SCOPE);
        e.close_element(&ELEM_LOCALDB);
    }

    /// `<prototype extrapop="unknown" model="default" …><returnsym>` (Java
    /// `FunctionPrototype.encodePrototype`, params via `<localdb>`).
    fn encode_prototype(&self, e: &mut PackedEncode, pieces: &PrototypePieces) {
        e.open_element(&ELEM_PROTOTYPE);
        e.write_string(&ATTRIB_EXTRAPOP, b"unknown");
        e.write_string(&ATTRIB_MODEL, b"default");
        if pieces.first_var_arg_slot >= 0 {
            e.write_bool(&ATTRIB_DOTDOTDOT, true);
        }
        if pieces.intypes.is_empty() {
            e.write_bool(&ATTRIB_VOIDLOCK, true);
        }
        e.open_element(&ELEM_RETURNSYM);
        e.write_bool(&ATTRIB_TYPELOCK, true);
        // Blank <addr/>: the decompiler's model assigns return storage.
        e.open_element(&kuna_base::address::ELEM_ADDR);
        e.close_element(&kuna_base::address::ELEM_ADDR);
        match &pieces.outtype {
            Some(ct) => self.encode_wire_type(e, ct),
            None => {
                e.open_element(&ELEM_VOID);
                e.close_element(&ELEM_VOID);
            }
        }
        e.close_element(&ELEM_RETURNSYM);
        e.close_element(&ELEM_PROTOTYPE);
    }

    /// `<doc id=0><mapsym><symbol …><type…/></symbol><addr size/><rangelist/>`
    /// — a committed data symbol (Java `DecompileCallback.encodeData` /
    /// `HighCodeSymbol`).
    fn symbol_doc(&self, start: &Address, name: &str, size: i32) -> Vec<u8> {
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&kuna_ghidra::ids::ELEM_DOC);
            e.write_unsigned_integer(&ATTRIB_ID, 0);
            e.open_element(&ELEM_MAPSYM);
            e.open_element(&ELEM_SYMBOL);
            e.write_unsigned_integer(&ATTRIB_ID, start.get_offset() | 0x2_0000_0000);
            e.write_string(&ATTRIB_NAME, name.as_bytes());
            e.write_bool(&ATTRIB_TYPELOCK, true);
            e.write_bool(&ATTRIB_NAMELOCK, true);
            if self.in_readonly(start.get_offset()) {
                e.write_bool(&ATTRIB_READONLY, true);
            }
            e.write_signed_integer(&ATTRIB_CAT, -1);
            // The committed data symbols carry no recovered type: undefinedN
            // (an unknown base of the symbol's byte size, the Ghidra shape).
            e.open_element(&ELEM_TYPE);
            e.write_string(&ATTRIB_NAME, b"");
            e.write_string(&ATTRIB_METATYPE, b"unknown");
            e.write_signed_integer(&ATTRIB_SIZE, size.clamp(1, 8) as i64);
            e.close_element(&ELEM_TYPE);
            e.close_element(&ELEM_SYMBOL);
            start
                .encode_sized(&mut e, size.max(1))
                .expect("data addr encodes");
            e.open_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_RANGELIST);
            e.close_element(&ELEM_MAPSYM);
            e.close_element(&kuna_ghidra::ids::ELEM_DOC);
        }
        doc
    }

    /// A one-byte `<hole [readonly] space first last/>` (Java
    /// `DecompileCallback.encodeHole`'s single-address arm).
    fn hole_doc(&self, addr: &Address) -> Vec<u8> {
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&ELEM_HOLE);
            if self.in_readonly(addr.get_offset()) {
                e.write_bool(&ATTRIB_READONLY, true);
            }
            let spc = addr.get_space().expect("hole address has a space");
            e.write_space(&kuna_base::marshal::ATTRIB_SPACE, spc);
            e.write_unsigned_integer(&ATTRIB_FIRST, addr.get_offset());
            e.write_unsigned_integer(&ATTRIB_LAST, addr.get_offset());
            e.close_element(&ELEM_HOLE);
        }
        doc
    }

    fn in_readonly(&self, offset: u64) -> bool {
        self.readonly_ranges
            .iter()
            .any(|(first, last)| offset >= *first && offset <= *last)
    }

    /// Encode a data-type the way Java's `encodeTypeRef`/`encodeType` does:
    /// named types travel as `<typeref name id/>` (core types resolve locally
    /// on the kuna side — same hash ids — and unknown named types trigger a
    /// getDataType query answered by [`Self::answer_data_type`]); anonymous
    /// pointers/arrays inline recursively; everything else inlines as a base.
    fn encode_wire_type(&self, e: &mut PackedEncode, dt: &Rc<Datatype>) {
        use type_metatype::*;
        match dt.get_metatype() {
            TYPE_VOID => {
                e.open_element(&ELEM_VOID);
                e.close_element(&ELEM_VOID);
            }
            TYPE_PTR => {
                if let DatatypeKind::Pointer { ptrto, wordsize, .. } = &dt.kind {
                    e.open_element(&ELEM_TYPE);
                    e.write_string(&ATTRIB_NAME, b"");
                    e.write_string(&ATTRIB_METATYPE, b"ptr");
                    e.write_signed_integer(&ATTRIB_SIZE, dt.get_size() as i64);
                    if *wordsize != 1 {
                        e.write_unsigned_integer(
                            &kuna_base::marshal::ATTRIB_WORDSIZE,
                            *wordsize as u64,
                        );
                    }
                    self.encode_wire_type(e, ptrto);
                    e.close_element(&ELEM_TYPE);
                } else {
                    self.encode_base_inline(e, dt);
                }
            }
            TYPE_ARRAY => {
                if let DatatypeKind::Array { arrayof, arraysize } = &dt.kind {
                    e.open_element(&ELEM_TYPE);
                    e.write_string(&ATTRIB_NAME, b"");
                    e.write_string(&ATTRIB_METATYPE, b"array");
                    e.write_signed_integer(&ATTRIB_SIZE, dt.get_size() as i64);
                    e.write_signed_integer(
                        &kuna_decomp::dtype::ATTRIB_ARRAYSIZE,
                        *arraysize as i64,
                    );
                    self.encode_wire_type(e, arrayof);
                    e.close_element(&ELEM_TYPE);
                } else {
                    self.encode_base_inline(e, dt);
                }
            }
            _ if !dt.get_name().is_empty() && dt.get_id() != 0 => {
                e.open_element(&ELEM_TYPEREF);
                e.write_string(&ATTRIB_NAME, dt.get_name().as_bytes());
                e.write_unsigned_integer(&ATTRIB_ID, dt.get_id());
                e.close_element(&ELEM_TYPEREF);
            }
            _ => self.encode_base_inline(e, dt),
        }
    }

    /// An inline `<type>` for a base/unknown shape.
    fn encode_base_inline(&self, e: &mut PackedEncode, dt: &Rc<Datatype>) {
        let meta = kuna_decomp::dtype::metatype2string(dt.get_metatype())
            .unwrap_or_else(|_| "unknown".to_string());
        e.open_element(&ELEM_TYPE);
        e.write_string(&ATTRIB_NAME, dt.get_name().as_bytes());
        e.write_string(&ATTRIB_METATYPE, meta.as_bytes());
        e.write_signed_integer(&ATTRIB_SIZE, dt.get_size().max(1) as i64);
        if dt.get_id() != 0 && !dt.get_name().is_empty() {
            e.write_unsigned_integer(&ATTRIB_ID, dt.get_id());
        }
        e.close_element(&ELEM_TYPE);
    }

    /// getDataType: look the (name, id) up in the oracle's own factory and
    /// encode the full definition — the sim half of the
    /// `TypeFactoryGhidra::findById` miss path.
    fn answer_data_type(&mut self, dec: &mut PackedDecode) -> Vec<u8> {
        let name =
            String::from_utf8_lossy(&dec.read_string_id(&ATTRIB_NAME).expect("type name"))
                .into_owned();
        let _id = dec.read_signed_integer_id(&ATTRIB_ID).unwrap_or(0);
        let found = self
            .prog
            .arch()
            .types()
            .find_by_name(&name)
            .ok()
            .flatten();
        match found {
            Some(dt) => {
                let mut doc = Vec::new();
                {
                    let mut e = PackedEncode::new(&mut doc);
                    self.encode_full_type(&mut e, &dt);
                }
                resp_string(&doc)
            }
            None => resp_empty(),
        }
    }

    /// A full (non-typeref) `<type>` definition for a getDataType answer:
    /// composites carry their fields; everything else the base shape.
    fn encode_full_type(&self, e: &mut PackedEncode, dt: &Rc<Datatype>) {
        use type_metatype::*;
        match (&dt.kind, dt.get_metatype()) {
            (DatatypeKind::Struct { field, .. }, TYPE_STRUCT)
            | (DatatypeKind::Union { field }, _) => {
                let meta = if dt.get_metatype() == TYPE_STRUCT {
                    "struct"
                } else {
                    "union"
                };
                e.open_element(&ELEM_TYPE);
                e.write_string(&ATTRIB_NAME, dt.get_name().as_bytes());
                e.write_unsigned_integer(&ATTRIB_ID, dt.get_id());
                e.write_string(&ATTRIB_METATYPE, meta.as_bytes());
                e.write_signed_integer(&ATTRIB_SIZE, dt.get_size() as i64);
                e.write_signed_integer(
                    &kuna_decomp::dtype::ATTRIB_ALIGNMENT,
                    dt.alignment.max(1) as i64,
                );
                for f in field {
                    e.open_element(&ELEM_FIELD);
                    e.write_string(&ATTRIB_NAME, f.name.as_bytes());
                    e.write_signed_integer(&ATTRIB_OFFSET, f.offset as i64);
                    e.write_signed_integer(&ATTRIB_ID, f.ident as i64);
                    self.encode_wire_type(e, &f.field_type);
                    e.close_element(&ELEM_FIELD);
                }
                e.close_element(&ELEM_TYPE);
            }
            _ => self.encode_base_inline(e, dt),
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
            self.log.register_probes.push(name.clone());
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
                // Java THROWS on an undefined name rather than answering an
                // empty response (DecompileCallback.java:756-762), and the
                // sim answering leniently is what hid GH-388.
                Err(_) => {
                    self.log.register_probe_failures.push(name.clone());
                    resp_exception(
                        "java.lang.RuntimeException",
                        &format!("No Register Defined: {name}"),
                    )
                }
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
            // Phase 3: the REAL tracked set for the address — the program
            // context's values (pspec defaults) plus any test-injected
            // `tracked_overrides` (a user 'Set Register Value'), exactly the
            // DecompileCallback shape (always written, possibly empty).
            let addr = Address::decode(&mut dec).expect("getTrackedRegisters <addr>");
            let mut set = kuna_sleigh::globalcontext::TrackedSet::new();
            self.prog
                .arch()
                .translate()
                .with_context_db_dyn(&mut |db| {
                    set = db.get_tracked_set(&addr).clone();
                });
            for over in &self.tracked_overrides {
                match set.iter_mut().find(|t| t.loc == over.loc) {
                    Some(t) => t.val = over.val,
                    None => set.push(over.clone()),
                }
            }
            let mut doc = Vec::new();
            {
                let mut e = PackedEncode::new(&mut doc);
                if set.is_empty() {
                    e.open_element(&ELEM_TRACKED_POINTSET);
                    e.close_element(&ELEM_TRACKED_POINTSET);
                } else {
                    kuna_sleigh::globalcontext::encode_tracked(&mut e, &addr, &set)
                        .expect("tracked set encodes");
                }
            }
            resp_string(&doc)
        } else if el == ELEM_COMMAND_GETSTRINGDATA.get_id() {
            self.answer_string_data(&mut dec)
        } else if el == ELEM_COMMAND_ISNAMEUSED.get_id() {
            resp_string(b"f")
        } else if el == ELEM_COMMAND_GETMAPPEDSYMBOLS.get_id()
            || el == ELEM_COMMAND_GETEXTERNALREF.get_id()
        {
            // Phase 3: real `<doc><mapsym>` / `<hole>` answers from the
            // oracle's committed program facts (functions + locked prototypes
            // + noreturn, data symbols, section mutability) — the DecompileCallback
            // role.  `label_overrides` rides through `code_label`, which is what
            // the flushNative cache-clearing test flips.
            self.answer_mapped_symbols(&mut dec)
        } else if el == ELEM_COMMAND_GETDATATYPE.get_id() {
            // Phase 3: the TypeFactoryGhidra findById miss path — answer the
            // full definition from the oracle's own factory.
            self.answer_data_type(&mut dec)
        } else if el == ELEM_COMMAND_GETNAMESPACEPATH.get_id()
            || el == ELEM_COMMAND_GETCPOOLREF.get_id()
        {
            // Namespace scopes / cpool records: not modeled by the sim (the
            // oracle commits everything into the global scope) — empty is the
            // lenient "not found" answer.
            resp_empty()
        } else if el == ELEM_COMMAND_GETCALLOTHERFIXUP.get_id()
            || el == ELEM_COMMAND_GETCALLFIXUP.get_id()
        {
            self.answer_inject(el, &mut dec)
        } else {
            // getCallMechanism (242) / getPcodeExecutable (252): neither is
            // reachable from the live engine, so empty stays the answer.
            resp_empty()
        }
    }
}

/// Decode the `<context>` element of a getPcodeInject query — the mirror of
/// `InjectContextGhidra::encode` and of Java's `InjectContext.decode`:
/// baseaddr, calladdr, then the optional sized operand lists.  `nextaddr` is
/// not on the wire; the caller re-derives it.
fn decode_inject_context(dec: &mut PackedDecode) -> InjectContext {
    let cel = dec.open_element_id(&ELEM_CONTEXT).expect("<context>");
    let baseaddr = Address::decode(dec).expect("context baseaddr");
    let calladdr = Address::decode(dec).expect("context calladdr");
    let mut ctx = InjectContext {
        baseaddr: Some(baseaddr),
        nextaddr: None,
        calladdr: calladdr.get_space().map(|_| calladdr.clone()),
        inputlist: Vec::new(),
        output: Vec::new(),
    };
    loop {
        let sub = dec.peek_element().expect("context child peek");
        let is_out = sub == kuna_base::marshal::ELEM_OUTPUT.get_id();
        if !is_out && sub != kuna_base::marshal::ELEM_INPUT.get_id() {
            break;
        }
        let opened = dec.open_element().expect("context operand list");
        while dec.peek_element().expect("operand peek") != 0 {
            let (a, sz) = Address::decode_sized(dec).expect("context <addr>");
            let vn = VarnodeData {
                space: a.get_space().cloned(),
                offset: a.get_offset(),
                size: sz as u32,
            };
            if is_out {
                ctx.output.push(vn);
            } else {
                ctx.inputlist.push(vn);
            }
        }
        dec.close_element(opened).expect("close operand list");
    }
    dec.close_element(cel).expect("close <context>");
    ctx
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
