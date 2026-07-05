//! The ghidra-mode command loop — a port of `decompiler/cpp/ghidra_process.cc`
//! (`GhidraCapability::readCommand`, `GhidraCommand::doit`, and the seven
//! decomp-capability commands, ghidra_process.cc:86-506).
//!
//! Lifecycle per command (`doit`, ghidra_process.cc:125-160), preserved
//! exactly:
//!
//! ```text
//!   write {0,0,1,6}                        response OPEN — before any work
//!   loadParameters()                       archid as ASCII decimal in a
//!                                          string stream (every command
//!                                          except registerProgram), plus
//!                                          command-specific params
//!   expect burst 3 (command close)
//!   rawAction()
//!   catch DecoderError  -> "Marshaling error: ..."     (warning)
//!   catch JavaError     -> passJavaException, NO sendResult
//!   catch RecovError    -> "Recoverable Error: ..."    (warning)
//!   catch LowlevelError -> "Low-level Error: ..."      (warning)
//!   sendResult()                           optional payload, then ALWAYS
//!                                          the 16/17 warnings frame
//!                                          (possibly empty) — but only
//!                                          while a session is bound
//!   write {0,0,1,7}; flush
//! ```
//!
//! Because the response-open burst is written first, any queries a command
//! issues are nested inside the open command response (the phase-2 engine
//! bridge relies on this).
//!
//! Phase-1 scope (see `docs/ghidra-integration.md`): the protocol side of
//! all seven decomp commands is complete; `decompileAt` answers with the
//! incomplete-function shape (an EMPTY 14/15 payload,
//! ghidra_process.cc:313-334) plus a not-implemented warning, and
//! `flushNative` has no engine caches to clear yet.  `structureGraph` and
//! the four signature commands are deliberately NOT registered: the
//! unknown-command response `{6}{16}"Bad command: <name>"{17}{7}`
//! (ghidra_process.cc:476-484) is the exact graceful-degradation shape the
//! Java side expects (DecompInterface.java:341-347).

use std::io::{Read, Write};

use kuna_base::address::Address;
use kuna_base::error::KunaError;
use kuna_base::marshal::{Decoder, IdRegistry, PackedDecode};
use kuna_base::space::AddrSpaceManager;

use crate::protocol::{
    pass_java_exception, read_string_stream, read_string_stream_optional, read_to_any_burst,
    write_burst, write_string_stream, WireError, WireResult, BURST_COMMAND_CLOSE,
    BURST_COMMAND_OPEN, BURST_MESSAGE_CLOSE, BURST_MESSAGE_OPEN, BURST_RESPONSE_CLOSE,
    BURST_RESPONSE_OPEN, BURST_STRING_CLOSE, BURST_STRING_OPEN,
};
use crate::translate::{build_registry, GhidraTranslate};

/// One registered program: the kuna analog of an `ArchitectureGhidra` slot
/// in the global `archlist` (ghidra_process.cc:76,176-201).
///
/// Phase 1 holds the wire-session state only; the engine (`Architecture`
/// init over the four specs, the query-backed providers) is phase 2.
struct Session {
    /// The four registerProgram XML documents, held verbatim for the
    /// phase-2 engine bridge (C++ parses them in buildSpecFile and clears
    /// them; kuna keeps them so phase 2 can replay them into the engine).
    #[allow(dead_code)]
    pspec: Vec<u8>,
    #[allow(dead_code)]
    cspec: Vec<u8>,
    #[allow(dead_code)]
    tspec: Vec<u8>,
    #[allow(dead_code)]
    corespec: Vec<u8>,
    /// The session's marshaling id tables (the C++ static registration).
    #[allow(dead_code)]
    registry: IdRegistry,
    /// The parsed tspec — endianness, unique base, and the address-space
    /// manager whose indices decode wire \<addr> elements.  `None` when the
    /// tspec failed to parse (recorded as a warning; \<addr> params are
    /// then consumed without decoding — TODO(phase-2): a parse failure
    /// should fail registerProgram once the engine init is real).
    translate: Option<GhidraTranslate>,
    /// Accumulated warnings, shipped on the 16/17 channel by sendResult
    /// (C++ `ArchitectureGhidra::warnings`; `printMessage` appends
    /// `'\n' + message`, ghidra_arch.cc:898-902).
    warnings: String,
    /// setAction "tree"/"notree" (C++ `sendsyntaxtree`, default true).
    send_syntax_tree: bool,
    /// setAction "c"/"noc" (C++ `sendCcode`, default true).
    send_c_code: bool,
    /// setAction "parammeasures"/"noparammeasures" (default false).
    send_param_measures: bool,
    /// setAction "jumpload"/"nojumpload" (C++ `FlowInfo::record_jumploads`
    /// in `ghidra->flowoptions`, default off).
    record_jumploads: bool,
    /// The current root action (C++ `allacts.setCurrent`; default
    /// "decompile").
    current_action: String,
}

impl Session {
    /// Build a session from the four registerProgram documents, parsing the
    /// tspec leniently (a failure becomes a warning, not a command error).
    fn new(pspec: Vec<u8>, cspec: Vec<u8>, tspec: Vec<u8>, corespec: Vec<u8>) -> Session {
        let registry = build_registry();
        let mut warnings = String::new();
        let translate = match GhidraTranslate::decode(&tspec, &registry) {
            Ok(t) => Some(t),
            Err(e) => {
                // printMessage semantics: '\n' + message
                warnings.push('\n');
                warnings.push_str(&format!(
                    "kuna ghidra-mode: could not parse tspec <sleigh> element ({}); \
                     <addr> parameters will not be decoded",
                    e.explain()
                ));
                None
            }
        };
        Session {
            pspec,
            cspec,
            tspec,
            corespec,
            registry,
            translate,
            warnings,
            // ArchitectureGhidra constructor defaults (ghidra_arch.cc:912-926)
            send_syntax_tree: true,
            send_c_code: true,
            send_param_measures: false,
            record_jumploads: false,
            current_action: "decompile".to_string(),
        }
    }

    /// C++ `ArchitectureGhidra::printMessage` (ghidra_arch.cc:898-902).
    fn print_message(&mut self, message: &str) {
        self.warnings.push('\n');
        self.warnings.push_str(message);
    }
}

/// Which registered command is executing (the C++ `commandmap` keys,
/// ghidra_process.cc:496-506, minus structureGraph — see module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandKind {
    RegisterProgram,
    DeregisterProgram,
    FlushNative,
    DecompileAt,
    SetAction,
    SetOptions,
}

/// Per-command scratch state (the C++ `GhidraCommand` subclass members).
struct CommandState {
    kind: CommandKind,
    /// The bound archlist slot (C++ `ghidra` member); `None` until
    /// loadParameters binds it, and unbound again by deregister (C++ nulls
    /// `ghidra` after delete, so its sendResult skips the 16/17 frame).
    slot: Option<usize>,
    /// 0 = keep looping, 1 = terminate (C++ `status`).
    status: i32,
    /// registerProgram result (C++ `RegisterProgram::archid`).
    archid: i32,
    /// deregisterProgram / flushNative result (C++ `res`).
    res: i32,
    /// setAction / setOptions result (C++ `res` bool).
    ok: bool,
    /// registerProgram params.
    specs: Option<[Vec<u8>; 4]>,
    /// setAction params.
    actionstring: Vec<u8>,
    printstring: Vec<u8>,
    /// decompileAt param: the decoded address (None if the tspec is
    /// unavailable) plus its rendered form for messages.
    addr_text: Option<String>,
    /// setOptions param: the raw packed \<optionslist> bytes.
    options_raw: Option<Vec<u8>>,
}

impl CommandState {
    fn new(kind: CommandKind) -> CommandState {
        CommandState {
            kind,
            slot: None,
            status: 0,
            archid: -1,
            res: 0,
            ok: false,
            specs: None,
            actionstring: Vec::new(),
            printstring: Vec::new(),
            addr_text: None,
            options_raw: None,
        }
    }
}

/// The ghidra-mode process: the command loop over the two protocol streams
/// plus the architecture list (C++ `archlist` + `GhidraCapability::
/// readCommand`, ghidra_process.cc:76,464-486).
pub struct GhidraProcess<R: Read, W: Write> {
    sin: R,
    sout: W,
    archlist: Vec<Option<Session>>,
}

impl<R: Read, W: Write> GhidraProcess<R, W> {
    /// Construct over the two protocol streams (stdin/stdout in the real
    /// binary; in-memory buffers in tests).
    pub fn new(sin: R, sout: W) -> Self {
        GhidraProcess {
            sin,
            sout,
            archlist: Vec::new(),
        }
    }

    /// Tear down into the underlying streams (test access to the written
    /// response bytes).
    pub fn into_inner(self) -> (R, W) {
        (self.sin, self.sout)
    }

    /// Run the process loop until a command terminates it (C++ `main`'s
    /// `while(status == 0) status = readCommand(...)`,
    /// ghidra_process.cc:532-535).  Returns the terminating status.
    pub fn run(&mut self) -> WireResult<i32> {
        loop {
            let status = self.read_command()?;
            if status != 0 {
                return Ok(status);
            }
        }
    }

    /// Read and execute one command (C++ `GhidraCapability::readCommand`,
    /// ghidra_process.cc:464-486).  Returns the command's meta-status
    /// (0 = continue, 1 = terminate).
    pub fn read_command(&mut self) -> WireResult<i32> {
        // Align ourselves: scan to the next command-open burst, skipping
        // anything else (including the dangling params + close burst of a
        // rejected command).
        loop {
            if read_to_any_burst(&mut self.sin)? == BURST_COMMAND_OPEN {
                break;
            }
        }
        let name_bytes = read_string_stream(&mut self.sin)?;
        let name = String::from_utf8_lossy(&name_bytes).into_owned();
        let kind = match name.as_str() {
            "registerProgram" => CommandKind::RegisterProgram,
            "deregisterProgram" => CommandKind::DeregisterProgram,
            "flushNative" => CommandKind::FlushNative,
            "decompileAt" => CommandKind::DecompileAt,
            "setAction" => CommandKind::SetAction,
            "setOptions" => CommandKind::SetOptions,
            // structureGraph, generateSignatures, debugSignatures,
            // getSignatureSettings, setSignatureSettings are deliberately
            // unregistered in phase 1: this response — with NO payload
            // burst and NO command-close read — is the exact
            // graceful-degradation contract the Java side expects
            // (ghidra_process.cc:476-484; DecompInterface.java:341-347).
            _ => {
                write_burst(&mut self.sout, BURST_RESPONSE_OPEN)?;
                write_burst(&mut self.sout, BURST_MESSAGE_OPEN)?;
                self.sout
                    .write_all(format!("Bad command: {name}").as_bytes())
                    .map_err(WireError::Io)?;
                write_burst(&mut self.sout, BURST_MESSAGE_CLOSE)?;
                write_burst(&mut self.sout, BURST_RESPONSE_CLOSE)?;
                self.sout.flush().map_err(WireError::Io)?;
                return Ok(0);
            }
        };
        self.doit(kind)
    }

    /// The canonical command lifecycle (C++ `GhidraCommand::doit`,
    /// ghidra_process.cc:125-160).
    fn doit(&mut self, kind: CommandKind) -> WireResult<i32> {
        let mut cmd = CommandState::new(kind);
        // Command response header — BEFORE any work, so queries nest inside
        write_burst(&mut self.sout, BURST_RESPONSE_OPEN)?;
        let result = self.run_command(&mut cmd);
        match result {
            Ok(()) => {}
            // Pipe/IO failures propagate to the process loop (C++ exit(1))
            Err(e @ (WireError::PipeClosed | WireError::Io(_))) => return Err(e),
            // catch(JavaError): pass the exception, abort sending results
            Err(WireError::Kuna(KunaError::Java { type_name, explain })) => {
                pass_java_exception(&mut self.sout, &type_name, &explain)?;
                // C++ relies on cin.tie(&cout) flushing before the next
                // blocking read; Rust must flush explicitly.
                self.sout.flush().map_err(WireError::Io)?;
                return Ok(cmd.status);
            }
            // catch(DecoderError) / catch(RecovError) / catch(LowlevelError):
            // classify into a warning (KunaError::Decoder is the standalone
            // C++ DecoderError; Recov its subclass family; every other
            // variant derives LowlevelError — see kuna-base error.rs docs)
            Err(WireError::Kuna(err)) => {
                let errmsg = match &err {
                    KunaError::Decoder { .. } => format!("Marshaling error: {}", err.explain()),
                    KunaError::Recov { .. } => format!("Recoverable Error: {}", err.explain()),
                    _ => format!("Low-level Error: {}", err.explain()),
                };
                self.print_message(cmd.slot, &errmsg);
            }
        }
        self.send_result(&cmd)?;
        write_burst(&mut self.sout, BURST_RESPONSE_CLOSE)?;
        self.sout.flush().map_err(WireError::Io)?;
        Ok(cmd.status)
    }

    /// loadParameters + the command-close burst + rawAction (the `try`
    /// body of doit).
    fn run_command(&mut self, cmd: &mut CommandState) -> WireResult<()> {
        self.load_parameters(cmd)?;
        let t = read_to_any_burst(&mut self.sin)?;
        if t != BURST_COMMAND_CLOSE {
            return Err(WireError::Kuna(KunaError::java(
                "alignment",
                "Missing end of command",
            )));
        }
        self.raw_action(cmd)
    }

    // -- loadParameters -----------------------------------------------------

    /// The base `GhidraCommand::loadParameters` (ghidra_process.cc:86-103):
    /// the architecture id as ASCII decimal in a string stream, validated
    /// against the archlist, then `clearWarnings`.
    fn bind_session(&mut self, start_msg: &str, end_msg: &str) -> WireResult<usize> {
        let t = read_to_any_burst(&mut self.sin)?;
        if t != BURST_STRING_OPEN {
            return Err(WireError::Kuna(KunaError::java("alignment", start_msg)));
        }
        let (payload, code) = crate::protocol::read_id_payload(&mut self.sin)?;
        if code != BURST_STRING_CLOSE {
            return Err(WireError::Kuna(KunaError::java("alignment", end_msg)));
        }
        let id = parse_arch_id(&payload);
        if id >= 0 && (id as usize) < self.archlist.len() && self.archlist[id as usize].is_some() {
            let slot = id as usize;
            // ghidra->clearWarnings()
            if let Some(session) = self.archlist[slot].as_mut() {
                session.warnings.clear();
            }
            return Ok(slot);
        }
        Err(WireError::Kuna(KunaError::java(
            "decompiler",
            "No architecture registered with decompiler",
        )))
    }

    fn load_parameters(&mut self, cmd: &mut CommandState) -> WireResult<()> {
        match cmd.kind {
            // RegisterProgram::loadParameters (ghidra_process.cc:162-173):
            // four consecutive string streams, no arch id
            CommandKind::RegisterProgram => {
                let pspec = read_string_stream(&mut self.sin)?;
                let cspec = read_string_stream(&mut self.sin)?;
                let tspec = read_string_stream(&mut self.sin)?;
                let corespec = read_string_stream(&mut self.sin)?;
                cmd.specs = Some([pspec, cspec, tspec, corespec]);
                Ok(())
            }
            // DeregisterProgram::loadParameters (ghidra_process.cc:212-229)
            CommandKind::DeregisterProgram => {
                let slot = self.bind_session(
                    "Expecting deregister id start",
                    "Expecting deregister id end",
                )?;
                cmd.slot = Some(slot);
                Ok(())
            }
            CommandKind::FlushNative => {
                let slot = self.bind_session("Expecting arch id start", "Expecting arch id end")?;
                cmd.slot = Some(slot);
                Ok(())
            }
            // DecompileAt::loadParameters (ghidra_process.cc:284-291): base,
            // then the packed <addr> ingested and decoded
            CommandKind::DecompileAt => {
                let slot = self.bind_session("Expecting arch id start", "Expecting arch id end")?;
                cmd.slot = Some(slot);
                let raw = read_string_stream_optional(&mut self.sin)?;
                let session = self.archlist[slot].as_ref().expect("bound session");
                match (&session.translate, raw) {
                    (Some(tr), Some(bytes)) => {
                        let mut decoder = PackedDecode::new(&tr.manager);
                        decoder.ingest_stream(&bytes).map_err(WireError::Kuna)?;
                        let addr = Address::decode(&mut decoder).map_err(WireError::Kuna)?;
                        cmd.addr_text = Some(render_address(&addr));
                    }
                    (None, Some(_bytes)) => {
                        // tspec unavailable: the <addr> was consumed but
                        // cannot be decoded (lenient skip; the session
                        // already carries the tspec warning).
                        // TODO(phase-2): hard-fail once engine init is real.
                        cmd.addr_text = None;
                    }
                    (_, None) => {
                        // Missing payload: C++ Address::decode on the empty
                        // decoder raises DecoderError -> "Marshaling error"
                        return Err(WireError::Kuna(KunaError::decoder(
                            "Expecting <addr> element",
                        )));
                    }
                }
                Ok(())
            }
            // SetAction::loadParameters (ghidra_process.cc:368-376)
            CommandKind::SetAction => {
                let slot = self.bind_session("Expecting arch id start", "Expecting arch id end")?;
                cmd.slot = Some(slot);
                cmd.actionstring = read_string_stream(&mut self.sin)?;
                cmd.printstring = read_string_stream(&mut self.sin)?;
                Ok(())
            }
            // SetOptions::loadParameters (ghidra_process.cc:418-426): base,
            // then the packed <optionslist> string stream
            CommandKind::SetOptions => {
                let slot = self.bind_session("Expecting arch id start", "Expecting arch id end")?;
                cmd.slot = Some(slot);
                cmd.options_raw = read_string_stream_optional(&mut self.sin)?;
                Ok(())
            }
        }
    }

    // -- rawAction ------------------------------------------------------------

    fn raw_action(&mut self, cmd: &mut CommandState) -> WireResult<()> {
        match cmd.kind {
            // RegisterProgram::rawAction (ghidra_process.cc:176-201): find a
            // free slot (the C++ loop keeps the LAST open slot), build the
            // session, archid = slot index
            CommandKind::RegisterProgram => {
                let [pspec, cspec, tspec, corespec] =
                    cmd.specs.take().expect("registerProgram params");
                let session = Session::new(pspec, cspec, tspec, corespec);
                let mut open: Option<usize> = None;
                for (i, s) in self.archlist.iter().enumerate() {
                    if s.is_none() {
                        open = Some(i); // C++ keeps scanning: last open slot
                    }
                }
                let slot = match open {
                    Some(i) => {
                        self.archlist[i] = Some(session);
                        i
                    }
                    None => {
                        self.archlist.push(Some(session));
                        self.archlist.len() - 1
                    }
                };
                cmd.slot = Some(slot);
                cmd.archid = slot as i32;
                Ok(())
            }
            // DeregisterProgram::rawAction (ghidra_process.cc:231-251):
            // free the slot, res=1, status=1 terminates the process loop.
            // C++ nulls `ghidra` after the delete, so the base sendResult's
            // 16/17 frame is skipped — mirror by unbinding the slot.
            CommandKind::DeregisterProgram => {
                if let Some(slot) = cmd.slot.take() {
                    cmd.res = 1;
                    self.archlist[slot] = None;
                    cmd.status = 1;
                } else {
                    cmd.res = 0;
                }
                Ok(())
            }
            // FlushNative::rawAction (ghidra_process.cc:262-273): phase 1
            // has no engine caches (global scope / non-core types /
            // comments / strings / cpool are all phase 2) — nothing to
            // flush.  TODO(phase-2): clear the provider caches here.
            CommandKind::FlushNative => {
                cmd.res = 0;
                Ok(())
            }
            // DecompileAt::rawAction (ghidra_process.cc:293-335): phase 1
            // emits the incomplete-function shape — an EMPTY 14/15 payload
            // (exactly what upstream sends when !fd->isProcComplete()) —
            // plus a warning on the 16/17 channel, which renders as a clean
            // error in the Ghidra GUI instead of a hang/desync.
            CommandKind::DecompileAt => {
                write_burst(&mut self.sout, BURST_STRING_OPEN)?;
                write_burst(&mut self.sout, BURST_STRING_CLOSE)?;
                let at = match &cmd.addr_text {
                    Some(t) => t.clone(),
                    None => "(address not decoded)".to_string(),
                };
                self.print_message(
                    cmd.slot,
                    &format!(
                        "kuna ghidra-mode: engine bridge not yet implemented (phase 1); \
                         function at {at} not decompiled"
                    ),
                );
                Ok(())
            }
            // SetAction::rawAction (ghidra_process.cc:378-406)
            CommandKind::SetAction => {
                let slot = cmd.slot.expect("bound session");
                let actionstring = String::from_utf8_lossy(&cmd.actionstring).into_owned();
                let printstring = String::from_utf8_lossy(&cmd.printstring).into_owned();
                let session = self.archlist[slot].as_mut().expect("bound session");
                if !actionstring.is_empty() {
                    // allacts.setCurrent: the registered root actions
                    // (ghidra_process.hh:190-196 plus the "universal" root
                    // the ActionDatabase always defines)
                    match actionstring.as_str() {
                        "decompile" | "normalize" | "jumptable" | "paramid" | "register"
                        | "firstpass" | "universal" => {
                            session.current_action = actionstring;
                        }
                        _ => {
                            // C++ setCurrent -> deriveAction -> getGroup(name), which
                            // throws "Action group does not exist: <name>" for an
                            // unregistered root (action.cc:1005-1013,1145-1158); match
                            // its wording so the 16/17 warning the GUI shows is faithful.
                            return Err(WireError::Kuna(KunaError::lowlevel(format!(
                                "Action group does not exist: {actionstring}"
                            ))));
                        }
                    }
                }
                if !printstring.is_empty() {
                    match printstring.as_str() {
                        "tree" => session.send_syntax_tree = true,
                        "notree" => session.send_syntax_tree = false,
                        "c" => session.send_c_code = true,
                        "noc" => session.send_c_code = false,
                        "parammeasures" => session.send_param_measures = true,
                        "noparammeasures" => session.send_param_measures = false,
                        "jumpload" => session.record_jumploads = true,
                        "nojumpload" => session.record_jumploads = false,
                        _ => {
                            return Err(WireError::Kuna(KunaError::lowlevel(format!(
                                "Unknown print action: {printstring}"
                            ))))
                        }
                    }
                }
                cmd.ok = true;
                Ok(())
            }
            // SetOptions::rawAction (ghidra_process.cc:435-445).
            // DELIBERATE DIVERGENCE (docs/ghidra-integration.md): upstream
            // decodes the <optionslist> and throws on any unknown option
            // (OptionDatabase::set -> ParseError -> response 'f' -> Java
            // IOException "Did not accept decompiler options", killing the
            // program open).  Phase-1 kuna has no option database yet, and
            // for drop-in robustness across Ghidra versions it TOLERATES
            // the whole list: the packed stream is consumed (and counted
            // when decodable) and the answer is always 't'.
            CommandKind::SetOptions => {
                let slot = cmd.slot.expect("bound session");
                let counted = cmd.options_raw.as_ref().and_then(|raw| {
                    let session = self.archlist[slot].as_ref().expect("bound session");
                    count_option_elements(raw, session.translate.as_ref().map(|t| &t.manager))
                });
                let msg = match counted {
                    Some(n) => format!(
                        "kuna ghidra-mode: setOptions: {n} option element(s) recorded \
                         but not applied (phase 1)"
                    ),
                    None => "kuna ghidra-mode: setOptions accepted but not applied (phase 1)"
                        .to_string(),
                };
                self.print_message(cmd.slot, &msg);
                cmd.ok = true;
                Ok(())
            }
        }
    }

    // -- sendResult -----------------------------------------------------------

    /// The per-command payload plus the base `GhidraCommand::sendResult`
    /// warnings frame (ghidra_process.cc:108-116,203-210,253-260,275-282,
    /// 408-416,447-455).  The 16/17 frame is written only while a session
    /// is bound (the C++ `ghidra != nullptr` check).
    fn send_result(&mut self, cmd: &CommandState) -> WireResult<()> {
        match cmd.kind {
            CommandKind::RegisterProgram => {
                write_string_stream(&mut self.sout, cmd.archid.to_string().as_bytes())?;
            }
            CommandKind::DeregisterProgram | CommandKind::FlushNative => {
                write_string_stream(&mut self.sout, cmd.res.to_string().as_bytes())?;
            }
            CommandKind::SetAction | CommandKind::SetOptions => {
                write_string_stream(&mut self.sout, if cmd.ok { b"t" } else { b"f" })?;
            }
            // DecompileAt writes its payload inside rawAction (or none at
            // all when rawAction was aborted) — nothing here
            CommandKind::DecompileAt => {}
        }
        if let Some(slot) = cmd.slot {
            if let Some(session) = self.archlist[slot].as_ref() {
                write_burst(&mut self.sout, BURST_MESSAGE_OPEN)?;
                self.sout
                    .write_all(session.warnings.as_bytes())
                    .map_err(WireError::Io)?;
                write_burst(&mut self.sout, BURST_MESSAGE_CLOSE)?;
            }
        }
        Ok(())
    }

    /// Route a message to the bound session's warning accumulator; without
    /// a bound session the message is dropped (the C++ base sendResult
    /// skips the 16/17 frame entirely when `ghidra` is null).
    ///
    /// TODO(phase-2): C++ `RegisterProgram::rawAction` assigns `ghidra` to
    /// the freshly-`new`'d `ArchitectureGhidra` *before* `init`, so an
    /// `init` that throws on bad specs still ships its error on the 16/17
    /// channel — and Java's registerProgram treats a non-empty
    /// nativeMessage as registration failure (`DecompInterface.java:291-294`).
    /// Phase 1 never hits this (`Session::new` is infallible; every pre-bind
    /// error is a `JavaError`), but once phase 2 makes registerProgram's
    /// engine init fallible, the in-flight session needs a warning sink here
    /// (bind the slot first, or buffer warnings on `CommandState`) or a
    /// failed registration would answer with a bare archid and no warning,
    /// and Java would think it succeeded.
    fn print_message(&mut self, slot: Option<usize>, message: &str) {
        if let Some(slot) = slot {
            if let Some(session) = self.archlist[slot].as_mut() {
                session.print_message(message);
            }
        }
    }
}

/// Parse the ASCII-decimal architecture id (C++ `sin >> dec >> id`,
/// ghidra_process.cc:92): skip whitespace, optional sign, decimal digits,
/// stopping at the first non-digit.
///
/// DELIBERATE DIVERGENCE: on extraction failure C++11 stores 0 AND sets
/// failbit, after which every read fails and the process exits.  kuna
/// returns -1, which the caller turns into the "No architecture registered
/// with decompiler" JavaError — the client sees a clean exception and the
/// process stays alive (docs/ghidra-integration.md).
fn parse_arch_id(payload: &[u8]) -> i32 {
    let mut i = 0usize;
    while i < payload.len() && (payload[i] as char).is_ascii_whitespace() {
        i += 1;
    }
    let mut negative = false;
    if i < payload.len() && (payload[i] == b'+' || payload[i] == b'-') {
        negative = payload[i] == b'-';
        i += 1;
    }
    let mut val: i64 = 0;
    let mut any = false;
    while i < payload.len() && payload[i].is_ascii_digit() {
        any = true;
        val = val
            .saturating_mul(10)
            .saturating_add((payload[i] - b'0') as i64);
        i += 1;
    }
    if !any {
        return -1;
    }
    if negative {
        val = -val;
    }
    val.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// Render an address for warning messages: the space shortcut plus the C++
/// `printRaw` form (the shape of upstream's decompileAt/getBytes messages).
fn render_address(addr: &Address) -> String {
    let mut s = String::new();
    if let Some(spc) = addr.get_space() {
        s.push(spc.get_shortcut());
    }
    if addr.print_raw(&mut s).is_err() {
        s.push_str("invalid_addr");
    }
    s
}

/// Best-effort count of the child elements of the packed \<optionslist>
/// (option elements hold only strings/ints, so an empty manager decodes
/// them fine).  `None` when the stream doesn't decode.
fn count_option_elements(raw: &[u8], manager: Option<&AddrSpaceManager>) -> Option<usize> {
    let empty;
    let mgr = match manager {
        Some(m) => m,
        None => {
            empty = AddrSpaceManager::new();
            &empty
        }
    };
    let mut decoder = PackedDecode::new(mgr);
    decoder.ingest_stream(raw).ok()?;
    let root = decoder.open_element().ok()?;
    if root == 0 {
        return None;
    }
    let mut n = 0usize;
    while decoder.peek_element().ok()? != 0 {
        decoder.skip_element().ok()?;
        n += 1;
    }
    decoder.close_element(root).ok()?;
    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arch_id() {
        assert_eq!(parse_arch_id(b"0"), 0);
        assert_eq!(parse_arch_id(b"17"), 17);
        assert_eq!(parse_arch_id(b" 3"), 3);
        assert_eq!(parse_arch_id(b"5xyz"), 5); // stops at first non-digit
        assert_eq!(parse_arch_id(b"-2"), -2);
        assert_eq!(parse_arch_id(b""), -1); // kuna divergence: -1, not 0
        assert_eq!(parse_arch_id(b"abc"), -1);
    }
}
