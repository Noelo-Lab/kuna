//! Port of `decompiler/cpp/ifacedecomp.{hh,cc}` (W9) — the decompiler-specific
//! console commands.
//!
//! This is the `IfaceDecompCommand` family: every command the interactive
//! `decomp_dbg` console and the XML datatest runner (`decomp_test_dbg`) drive
//! against the decompiler engine.  The framework they plug into
//! ([`IfaceStatus`], `IfaceCommand`, [`IfaceCommandAction`], the "base"
//! module commands `quit`/`echo`/`openfile`/...) is ported in `interface.rs`;
//! this module ports the `"decompile"`-module commands and the
//! `IfaceDecompCapability::registerCommands` registration
//! ([`register_decomp_commands`]), plus the console-driver
//! [`execute`]/[`mainloop`] dispatch.
//!
//! # Shape of the port
//!
//! In C++ each `Ifc*` command is a subclass of `IfaceDecompCommand`, holds a
//! back-pointer to the owning `IfaceStatus` (`status`) and to the shared
//! per-module data (`dcp`, an [`IfaceDecompData`]), set by `setData`, and
//! mutates them from a `void execute(istream&)`.  The kuna framework passes
//! `&mut IfaceStatus` to [`IfaceCommandAction::execute`] directly (no stored
//! back-pointer — see `interface.rs`), and the shared [`IfaceDecompData`] lives
//! in the [`IfaceStatus::get_data_mut`] datamap under the module name
//! `"decompile"`.  Each command therefore opens with [`dcp_mut`] to reach the
//! shared data, exactly mirroring `dcp->...` in C++.
//!
//! `module()` returns `"decompile"` for every command (C++
//! `IfaceDecompCommand::getModule`), and `create_data()` returns a fresh
//! [`IfaceDecompData`] (C++ `createData`).  Per `IfaceStatus::register_com`,
//! `create_data()` is invoked exactly once — on the first command registered
//! for the module — so the whole family shares one [`IfaceDecompData`], matching
//! C++ where `registerCom` creates the `IfaceData` only when the module is first
//! seen.
//!
//! # Exact console text
//!
//! The command **token sequences** (`registerCom(... ,"map","address")` etc.),
//! the prefix-expansion they feed, the per-command diagnostic strings
//! (`"No function selected"`, `"Decompiling <name>"`, `"Successfully ..."`,
//! ...), and the [`execute`] exception→prefix grammar (`"Command parsing error:
//! "`, `"Execution error: "`, ...) are byte-faithful to C++: they are what the
//! Python harness (`kuna/run_tests.py`) and the datatest `<stringmatch>`
//! assertions parse.
//!
//! # Documented losses (engine integration not yet exposed by W1–W8)
//!
//! The merged `rust-port` tree delivers the decompiler engine *internals* (lift,
//! flow, SSA, the universalAction pipeline, the print stack) as building blocks,
//! but the **`Architecture`-level integration layer** the engine-touching
//! commands invoke is not yet ported into the kuna-decomp public surface:
//!
//! - `parse_machaddr` / `parse_varnode` (the console address/varnode grammar,
//!   `pcodeparse.cc`), `parse_C` / `parse_type` / `parse_protopieces` (the
//!   C-declaration grammar, `grammar.cc`) — no ported entry points exist.
//! - `Architecture::print` (the owned `PrintLanguage`, used by `print C` /
//!   `docFunction` / `docAllGlobals`), `Architecture::types` (the `TypeFactory`
//!   accessor), `Architecture::loader` (the `LoadImage`), `Architecture::context`
//!   (the `ContextDatabase`) are not exposed as fields/accessors on the merged
//!   `Architecture`.
//! - `Architecture` does not yet implement `ArchOptionContext`, so even
//!   `OptionDatabase::set` cannot run against the real architecture.
//! - The full decompile drive (`allacts.getCurrent()->reset/perform`) and the
//!   loader-backed function load (`followFlow`) are not assembled at the
//!   `Architecture` level.
//!
//! Each command below ports faithfully every part that *is* expressible against
//! the merged API — the registration token set, the argument-parse order and
//! its `IfaceParseError`s, the `dcp->conf`/`dcp->fd` null guards and their
//! exact `IfaceExecutionError` text, and the success/echo text — and routes the
//! remaining engine call through [`engine_unavailable`], whose message names the
//! exact missing C++ entry point.  When the integration layer lands (a later
//! W-item that adds `print`/`types`/`loader`/`ArchOptionContext` to
//! `Architecture`), each `engine_unavailable` site is the single place to wire
//! the real call; the surrounding faithful structure does not change.

use crate::engine::{bootstrap_from_file, ConsoleProgram, UNBOUNDED_SIZE};
use crate::interface::{
    CommandStream, IfaceCommandAction, IfaceData, IfaceError, IfaceResult, IfaceStatus,
};
use kuna_base::types::{int4, uintb, uintm};
use kuna_decomp::decompile_drive::{
    build_and_follow_flow_with_override, print_c, print_c_types,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::options::OptionDatabase;

/// The module name every decompiler command shares (C++
/// `IfaceDecompCommand::getModule() { return "decompile"; }`).
pub const DECOMPILE_MODULE: &str = "decompile";

// ---------------------------------------------------------------------------
// IfaceDecompData — the shared "decompile" module data (ifacedecomp.hh:44).
// ---------------------------------------------------------------------------

/// C++ `IfaceDecompData` (`ifacedecomp.hh:44`): the data shared by every
/// decompiler command.
///
/// The C++ object also carries a `CallGraph *cgraph` and a
/// `FunctionTestCollection *testCollection`; the call-graph (`callgraph.cc`) and
/// the datatest collection (`testfunction.cc`) are their own W9 items, so those
/// slots are represented here as `bool` "is-allocated" markers — enough to
/// reproduce the null-guard diagnostics (`"No callgraph present"`,
/// `"Callgraph has not been built"`) the commands emit — and wired to the real
/// objects when those items land.
#[derive(Default)]
pub struct IfaceDecompData {
    /// C++ `Funcdata *fd`: the function currently active in the console.
    pub fd: Option<Funcdata>,
    /// C++ `Architecture *conf`: the architecture/program active in the console.
    ///
    /// In the Rust port the leaf [`ConsoleProgram`] owns the `XmlArchitecture`
    /// engine stack (the C++ `XmlArchitecture : Architecture` leaf), reachable via
    /// [`ConsoleProgram::arch_mut`].
    pub conf: Option<ConsoleProgram>,
    /// The SLEIGH spec search roots (C++ `SleighArchitecture::specpaths`, a
    /// process global).  Set by the binary at startup from `-s`/`SLEIGHHOME`; read
    /// by `load file` to resolve the architecture.
    pub spec_roots: Vec<String>,
    /// C++ `CallGraph *cgraph`: present once `callgraph build`/`load` has run.
    /// (The real `CallGraph` is a separate W9 item; this marks allocation so the
    /// `"No callgraph present"` guard is faithful.)
    pub cgraph_allocated: bool,
    /// C++ `FunctionTestCollection *testCollection`: present once `load test
    /// file` has run.  (The datatest runner is a separate W9 item.)
    pub test_collection_present: bool,
    /// Prototypes parsed by `parse line extern ...` (`parse_C`'s `setPrototype`
    /// branch) keyed by function name.
    ///
    /// C++ `Architecture::setPrototype` finds the existing function symbol and
    /// locks the prototype onto its `Funcdata` immediately.  In the kuna console
    /// boundary the `Funcdata` (`dcp.fd`) is built later by `load function`/`load addr`
    /// (`build_and_follow_flow` makes a fresh one), so the pieces are stashed here
    /// when the named function symbol exists and applied at load time.
    /// // STUB(W4 queryFunction/FuncProto restore)
    pub pending_prototypes:
        std::collections::BTreeMap<String, kuna_decomp::fspec::PrototypePieces>,
    /// Flow overrides installed by `override flow <addr> <type>`, keyed by
    /// function name.  C++ keeps these on `dcp->fd->getOverride()` (the Funcdata
    /// is reused); the kuna console rebuilds the IR on `load`/`decompile`, so the
    /// `(address, flow_type)` facts are stashed here and re-seeded onto the fresh
    /// Funcdata's `localoverride` at flow time (the `pending_prototypes`
    /// precedent).
    pub pending_flow_overrides:
        std::collections::BTreeMap<String, Vec<(kuna_base::address::Address, kuna_base::types::uint4)>>,
    /// Prototype overrides installed by `override prototype <addr> <decl>`, keyed
    /// by function name.  C++ keeps these on `dcp->fd->getOverride()` (the Funcdata
    /// is reused); the kuna console rebuilds the IR on `decompile`, so the
    /// `(callpoint, pieces)` facts are stashed here and re-seeded onto the fresh
    /// Funcdata's `localoverride` at flow time (the `pending_flow_overrides`
    /// precedent) — `FlowInfo::build_call_specs` consumes them as
    /// `Override::applyPrototype` (`fspecs.copy(*proto)`).
    pub pending_proto_overrides: std::collections::BTreeMap<
        String,
        Vec<(kuna_base::address::Address, kuna_decomp::fspec::PrototypePieces)>,
    >,
    /// Parameter storage locks installed by `map param <i> <addr> <typedecl>`
    /// (`IfcMapParam`), keyed by function name.  C++ writes these straight onto
    /// the queried Funcdata's live `FuncProto` via `setParam`; the kuna console
    /// rebuilds the IR on `decompile`, so the `(index, name, pieces)` facts are
    /// stashed here and re-seeded onto the fresh Funcdata's prototype at decompile
    /// time (the `pending_prototypes` precedent).  The pieces already carry the
    /// `typelock|namelock` flags the directive set.
    pub pending_param_maps: std::collections::BTreeMap<
        String,
        Vec<(kuna_base::types::int4, String, kuna_decomp::fspec::ParameterPieces)>,
    >,
    /// (kuna) What the last `load function` / `load addr` followed, so the very
    /// next `decompile` can adopt that IR instead of following the same flow a
    /// second time.  See [`PristineFlow`].
    pub pristine_flow: Option<PristineFlow>,
    /// (kuna) How many times a `decompile` adopted the loaded IR instead of
    /// re-following the flow.  Observability only -- nothing reads it to make a
    /// decision; it is what lets a test assert the fast path was actually taken
    /// rather than only that the output did not change.
    pub adopted_flows: u64,
}

/// (kuna) The provenance stamp of the `Funcdata` currently in `dcp.fd`, recorded
/// by `load function` / `load addr` right after their flow follow.
///
/// C++ `IfcDecompile` re-runs the actions on the `Funcdata` `IfcFuncload` built
/// (`clearAnalysis` + `perform`), so upstream follows the flow ONCE.  The kuna
/// console rebuilds the IR instead, because the seeds a decompile is given --
/// `map addr` symbols and DWARF locals, `type varnode` usepoint symbols, `map
/// hash` dynamic symbols, a `parse line` prototype, `map param` storage locks,
/// `override prototype` facts -- are consumed AT FLOW TIME and `load function`
/// applies none of them.  So the rebuild is not gratuitous; it is what makes
/// those console facts take effect.
///
/// It is also pure waste whenever there are no such facts, which is every
/// `kuna decompile <bin> <fn>`: the lift, the block build and the per-jump-table
/// sub-decompilation all run twice.  This stamp is what lets `decompile` prove
/// the waste case: it records the identity the flow was followed under, and the
/// console command counter at that moment, so a `decompile` that runs as the
/// very next command with every seed still empty can adopt the IR verbatim.
///
/// The `command_seq` check is deliberately the whole invalidation story.  Any
/// command in between -- an `option` that changes a flow-time decision, a
/// `kassert`, a `map`/`override`/`parse`, another `load` -- advances the counter
/// and the stamp stops matching, so no command needs its own invalidation hook
/// and none can be forgotten.
pub struct PristineFlow {
    /// `IfaceStatus::command_seq` as of the `load` that followed this flow.
    pub command_seq: u64,
    /// The name the flow was followed under.
    pub name: String,
    /// The entry the flow was followed from.
    pub entry: kuna_base::address::Address,
    /// The declared byte extent the follow was bounded by (0 = unbounded).
    pub size: kuna_base::types::int4,
    /// The flow overrides seeded before the follow.
    pub flow_overrides: Vec<(kuna_base::address::Address, kuna_base::types::uint4)>,
}

impl IfaceData for IfaceDecompData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl IfaceDecompData {
    /// C++ `IfaceDecompData::abortFunction(ostream &s)`.
    ///
    /// Called when a command throws a low-level engine error: clear any analysis
    /// on the current function, drop it, and warn.  Used by [`render_engine_error`]'s
    /// low-level / decoder paths.
    pub fn abort_function(&mut self, out: &mut String) {
        let name = match &self.fd {
            None => return,
            Some(fd) => fd.get_name().to_string(),
        };
        out.push_str("Unable to proceed with function: ");
        out.push_str(&name);
        out.push('\n');
        // C++ also calls conf->clearAnalysis(fd); that engine call is part of
        // the unported Architecture integration layer (see module docs).  The
        // observable console effect — the warning line and dropping `fd` — is
        // reproduced.
        self.fd = None;
    }

    /// C++ `IfaceDecompData::readSymbol(name,res)` (`ifacedecomp.cc`): resolve a
    /// symbol by name, starting in the current function's local scope (or the
    /// global scope when no function is selected).  Returns the matching symbol
    /// ids in that scope.
    ///
    /// The C++ `resolveScopeFromSymbolName` namespace walk handles `a::b` paths;
    /// the console corpus uses bare names, so the local (function) scope is
    /// queried directly with the name.  A namespaced name (containing `::`)
    /// reaches the unported namespace resolver and errs.
    pub fn read_symbol(
        &self,
        name: &str,
    ) -> Result<Vec<kuna_decomp::database::SymbolId>, IfaceError> {
        if name.contains("::") {
            return Err(IfaceError::parse(format!("Bad namespace for symbol: {name}")));
        }
        match &self.fd {
            Some(fd) => match fd.get_scope_local() {
                Some(lm) => Ok(lm.query_by_name(name)),
                None => Ok(Vec::new()),
            },
            None => Err(IfaceError::execution(
                "global symbol scope lookup not yet wired (no function selected)",
            )),
        }
    }

    /// C++ `IfaceDecompData::clearArchitecture()`.
    pub fn clear_architecture(&mut self) {
        self.conf = None;
        self.fd = None;
    }
}

/// Reach the shared [`IfaceDecompData`] from a command (C++ commands read the
/// `dcp` member set by `setData`).
///
/// The module data is registered under [`DECOMPILE_MODULE`] and is always
/// present once any decompiler command has been registered, so a missing entry
/// is an internal wiring bug (never a user-reachable path) and is surfaced as a
/// base [`IfaceError`].
fn dcp_mut(status: &mut IfaceStatus) -> IfaceResult<&mut IfaceDecompData> {
    match status.get_data_mut(DECOMPILE_MODULE) {
        Some(d) => match d.as_any_mut().downcast_mut::<IfaceDecompData>() {
            Some(dcp) => Ok(dcp),
            None => Err(IfaceError::base("decompile module data has wrong type")),
        },
        None => Err(IfaceError::base("decompile module data not registered")),
    }
}

/// The error returned where a command's engine call depends on the unported
/// `Architecture` integration layer (see module docs).
///
/// `entry` names the exact missing C++ entry point so the gap is self-describing
/// in the console; it is an `IfaceExecutionError` (the kind a started-but-failed
/// command throws), which [`execute`] renders under the `"Execution error: "`
/// prefix.
fn engine_unavailable(entry: &str) -> IfaceError {
    IfaceError::execution(format!(
        "engine integration not yet ported: {entry} (Architecture print/types/loader/context \
         + parse_machaddr/parse_C grammars are a later W-item)"
    ))
}

/// C++ `parse_machaddr(istream &s,int4 &defaultsize,const TypeFactory &typegrp,
/// bool ignorecolon)` (`grammar.cc:3099-3178`): read a machine address from the
/// console stream against the program's engine spaces, returning the address and
/// the associated `defaultsize` (the size from an explicit `[space,off,size]`
/// specifier, else the standard size implied by the offset token).
///
/// The supported forms transcribe the C++ grammar: `[space,offset[,size]]`
/// (bracketed, explicit space + optional size), the shortcut form (a leading
/// space-shortcut char then an offset token, e.g. `r0x110320`), and the default
/// code-space form (a leading `0` consumes the default code space).  The join
/// `{...}` form errs (the join-space console syntax is unported).  `ignorecolon`
/// controls whether `:` is a separator in the offset token (false: included,
/// matching the C++ default).
pub(crate) fn parse_machaddr(
    prog: &ConsoleProgram,
    s: &mut CommandStream,
    ignorecolon: bool,
) -> Result<(kuna_base::address::Address, int4), String> {
    use kuna_base::address::Address;
    use std::rc::Rc;
    let manage = prog.arch().manage();
    let mut size: int4 = -1;

    s.skip_ws();
    let tok = s.peek();
    let (space, token) = if tok == Some(b'[') {
        // [space,offset[,size]]
        s.get(); // consume '['
        let base_tok = s.read_to_separator(); // scan base address token
        let b = manage
            .get_space_by_name(&base_tok)
            .ok_or_else(|| "Bad address base".to_string())?;
        s.skip_ws();
        if s.get() != Some(b',') {
            return Err("Missing ',' in address".to_string());
        }
        let offtok = s.read_to_separator(); // the offset portion
        s.skip_ws();
        let mut next = s.get();
        if next == Some(b',') {
            // Optional size specifier (user base, like the C++ `unsetf` then `>>`).
            size = s.read_int();
            s.skip_ws();
            next = s.get();
        }
        if next != Some(b']') {
            return Err("Missing ']' in address".to_string());
        }
        (Rc::clone(b), offtok)
    } else if tok == Some(b'{') {
        return Err("join-space address syntax not yet ported (parse_machaddr '{')".to_string());
    } else {
        // Shortcut or default-code-space form.
        let b = if tok == Some(b'0') {
            // A leading '0' selects the default code space; the whole token is the
            // offset (read below).
            Rc::clone(
                manage
                    .get_default_code_space()
                    .ok_or_else(|| "No default code space".to_string())?,
            )
        } else {
            // The first char is a space shortcut; consume it.
            let sc = s.get().ok_or_else(|| "Missing address".to_string())?;
            let b = manage
                .get_space_by_shortcut(sc)
                .ok_or_else(|| format!("Bad address: {}", sc as char))?;
            Rc::clone(b)
        };
        // Collect the offset token (alnum/_/+ and optionally ':').
        let mut token = String::new();
        loop {
            match s.peek() {
                Some(c)
                    if c.is_ascii_alphanumeric()
                        || c == b'_'
                        || c == b'+'
                        || (!ignorecolon && c == b':') =>
                {
                    token.push(c as char);
                    s.get();
                }
                _ => break,
            }
        }
        (b, token)
    };

    let mut res = Address::new(space, 0);
    let oversize = res
        .read(&token, manage)
        .map_err(|_| "Bad machine address".to_string())?;
    let defaultsize = if size == -1 { oversize } else { size };
    Ok((res, defaultsize))
}

/// C++ `parse_varnode(istream &s,int4 &size,Address &pc,uintm &uq,
/// const TypeFactory &typegrp)` (`grammar.cc:3055-3084`): scan a specific
/// varnode specifier — a storage address, then `(` [`i` | defining-pc] [`:`uniq]
/// `)`.  Returns `(loc, size, pc, uq)`; `pc` is invalid when `i` (an input) or
/// absent, and `uq` is `~0` when no `:uniq` is given.
fn parse_varnode(
    prog: &ConsoleProgram,
    s: &mut CommandStream,
) -> Result<(kuna_base::address::Address, int4, kuna_base::address::Address, uintm), String> {
    use kuna_base::address::Address;
    let (loc, size) = parse_machaddr(prog, s, false)?;
    s.skip_ws();
    if s.get() != Some(b'(') {
        return Err("Missing '('".to_string());
    }
    s.skip_ws();
    let mut pc = Address::new_invalid(); // pc starts out as invalid
    match s.peek() {
        Some(b'i') => {
            s.get(); // consume the 'i' (an input varnode)
        }
        Some(b':') => {} // no pc: fall through to the uniq scan
        Some(_) => {
            // C++ `pc = parse_machaddr(s,discard,typegrp,true)` (ignorecolon).
            let (a, _discard) = parse_machaddr(prog, s, true)?;
            pc = a;
        }
        None => {}
    }
    s.skip_ws();
    let uq: uintm = if s.peek() == Some(b':') {
        s.get(); // consume ':'
        s.skip_ws();
        // C++ `s >> hex >> uq`: extract the leading run of hex digits, stopping
        // at the first non-hex char (e.g. the closing `)`), so no whitespace is
        // required before `)`.
        let mut hex = String::new();
        while let Some(c) = s.peek() {
            if c.is_ascii_hexdigit() {
                hex.push(c as char);
                s.get();
            } else {
                break;
            }
        }
        uintm::from_str_radix(&hex, 16).map_err(|_| "Bad uniq sequence number".to_string())?
    } else {
        !0 // ~((uintm)0)
    };
    s.skip_ws();
    if s.get() != Some(b')') {
        return Err("Missing ')'".to_string());
    }
    Ok((loc, size, pc, uq))
}

/// C++ `IfaceDecompData::readVarnode` (`ifacedecomp.cc:1469-1517`): parse a
/// varnode specifier (via [`parse_varnode`]) and resolve it to a live Varnode in
/// the current function `fd`.
///
/// For a constant Varnode (its storage offset is the value) the p-code sequence
/// number must be present and identify the reading op; the constant input of
/// that op whose address matches `loc` is returned.  The input (`pc` invalid and
/// `uq == ~0`) and defined (`pc` and `uq` both present) arms use the
/// `findVarnodeInput`/`findVarnodeWritten` bank queries.  The mixed loc-scan arm
/// (exactly one of `pc`/`uq` given) needs the `beginLoc`/`endLoc` storage walk,
/// a bank-iterator surface not yet on this console boundary — it errs as documented.
/// `Ok(None)` means "the requested varnode does not exist" (the C++ throw).
fn read_varnode(
    fd: &Funcdata,
    loc: &kuna_base::address::Address,
    defsize: int4,
    pc: &kuna_base::address::Address,
    uq: uintm,
) -> Result<Option<kuna_decomp::context::VarnodeId>, String> {
    use kuna_base::address::SeqNum;
    use kuna_base::space::spacetype;
    let no_uq = uq == !0u32;
    let space = loc.get_space().ok_or_else(|| "Varnode has no space".to_string())?;
    if space.get_type() == spacetype::IPTR_CONSTANT {
        // For a constant the p-code op reading it must be fully specified.
        if pc.is_invalid() || no_uq {
            return Err("Missing p-code sequence number".to_string());
        }
        let seq = SeqNum::new(pc.clone(), uq);
        if let Some(op) = fd.obank().find_op(&seq) {
            let n = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
            for i in 0..n {
                if let Some(tmpvn) = fd.obank().get(op).and_then(|o| o.get_in(i)) {
                    if fd.vbank().get(tmpvn).map(|v| v.get_addr() == loc).unwrap_or(false) {
                        return Ok(Some(tmpvn));
                    }
                }
            }
        }
        Ok(None)
    } else if pc.is_invalid() && no_uq {
        Ok(fd.find_varnode_input(defsize, loc))
    } else if !pc.is_invalid() && !no_uq {
        Ok(fd.find_varnode_written(defsize, loc, pc, Some(uq)))
    } else {
        // The residual `beginLoc(defsize,loc)`..`endLoc` storage walk
        // (exactly one of pc/uq given) — not on this boundary's bank surface.
        Err("kuna rust port: readVarnode loc-scan arm needs the beginLoc/endLoc bank iterator".to_string())
    }
}

/// C++ `s >> hex >> hash` (`IfcMaphash`/`IfcMapunionfacet`): extract one
/// hexadecimal `uint8` dynamic hash from the stream.  An optional `0x`/`0X`
/// prefix is honored (stream `hex` accepts either form); the digit run is the
/// next whitespace-delimited token.  Errs on an empty/unparseable token, matching
/// the C++ failed-extraction signal (the surrounding `execute` then aborts).
fn parse_hex_u64(s: &mut CommandStream) -> Result<u64, String> {
    let tok = s.read_token();
    let t = tok.trim();
    if t.is_empty() {
        return Err("Missing hash value".to_string());
    }
    let digits = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u64::from_str_radix(digits, 16).map_err(|_| "Bad hash value".to_string())
}

/// The boolean property flags `volatile`/`readonly` paint over a range (C++
/// `Varnode::volatil` / `Varnode::readonly`).
mod property_flag {
    pub use kuna_decomp::varnode::varnode_flags::{readonly, volatil};
}

/// Parse an unsigned value with the user-selected base (C++ `s.unsetf(dec|hex|oct)`
/// then `s >> value`): a `0x`/`0X` prefix is hex, a leading `0` is octal,
/// otherwise decimal.  `None` on an empty/unparseable token (the C++ sentinel
/// `0xbadbeef` stays, signalling "missing value").
fn parse_userbase_u64(tok: &str) -> Option<u64> {
    let t = tok.trim();
    if t.is_empty() {
        return None;
    }
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else if t.len() > 1 && t.starts_with('0') {
        u64::from_str_radix(&t[1..], 8).ok()
    } else {
        t.parse::<u64>().ok()
    }
}

/// 32-bit flavor of [`parse_userbase_u64`] (the `set context` value is a `uintm`).
fn parse_userbase_u32(tok: &str) -> Option<u32> {
    parse_userbase_u64(tok).and_then(|v| u32::try_from(v).ok())
}

/// Shared body of `IfcVolatile`/`IfcReadonly` (`ifacedecomp.cc:3006-3042`): parse
/// `<address+size>`, build the inclusive `Range` (open end `off+size`), OR the
/// property over it via `symboltab->setPropertyRange`, and echo the success line.
fn mark_property_range(
    status: &mut IfaceStatus,
    s: &mut CommandStream,
    flag: u32,
    success: &str,
) -> IfaceResult<()> {
    {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
    }
    let dcp = dcp_mut(status)?;
    let prog = dcp.conf.as_mut().expect("conf checked non-None above");
    // C++ Address addr = parse_machaddr(s,size,*dcp->conf->types).
    let (addr, mut size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
    // (kuna) An explicit size may follow the address.  The C++ takes the size
    // only from the bracketed `[space,offset,size]` form, which forces a caller
    // that wants a sized range to also name the address space; `--assert
    // 'readonly 0x404028+8'` lowers to the default-code-space form and states
    // the size here instead.  Additive: the bracketed form has no trailing
    // token, so every existing spelling keeps the size it had.
    s.skip_ws();
    if !s.eof() {
        let tok = s.read_token();
        match parse_userbase_u64(&tok) {
            Some(v) if v >= 1 && v <= int4::MAX as u64 => size = v as int4,
            _ => return Err(IfaceError::parse(format!("Bad size: {tok}"))),
        }
    }
    if size == 0 {
        return Err(IfaceError::execution("Must specify a size"));
    }
    // C++ Range(space, off, off+(size-1)); setPropertyRange => paint [off, off+size).
    let space = addr
        .get_space()
        .cloned()
        .ok_or_else(|| IfaceError::execution("Invalid address space"))?;
    let off = addr.get_offset();
    let addr2 = kuna_base::address::Address::new(space, off.wrapping_add(size as u64));
    prog.arch_mut().symboltab.set_property_range(flag, &addr, &addr2);
    status.out(&format!("{success}\n"));
    Ok(())
}

/// C++ `Architecture::setPrototype(pieces)` (architecture.cc:393) for the
/// console boundary: find the FunctionSymbol named `pieces.name` in the global scope
/// and lock the parsed prototype onto it.  The kuna model stores the locked
/// signature as the symbol's `TypeCode` prototype (built by
/// `TypeFactory::getTypeCode(PrototypePieces)` — the same construction the C++
/// `FuncProto::setPieces` ultimately produces); we retype the FunctionSymbol to
/// that prototype-bearing code type.  A missing symbol is a no-op (the C++ would
/// throw "Unknown function name", but the kuna `parse line extern` path also
/// stashes the pieces for the active-function load, so a name that is only ever
/// the *current* decompile target is still handled there); a non-function symbol
/// is left untouched.
fn apply_prototype_to_symbol(
    status: &mut IfaceStatus,
    pieces: &kuna_decomp::fspec::PrototypePieces,
) -> IfaceResult<()> {
    let dcp = dcp_mut(status)?;
    let prog = match dcp.conf.as_mut() {
        Some(p) => p,
        None => return Ok(()),
    };
    let arch = prog.arch_mut();
    let scope = match arch.symboltab.get_global_scope() {
        Some(s) => s,
        None => return Ok(()),
    };
    // queryFunction(basename) — the function must already be a FunctionSymbol
    // (a `<symbol>` loader record, or a prior `load function`).
    let sid = match arch.symboltab.query_function_by_name(scope, &pieces.name) {
        Some(s) => s,
        None => return Ok(()),
    };
    // getTypeCode(pieces): the prototype-bearing TypeCode the symbol's
    // getPrototype() will return.  A build failure (no proto context) is a
    // no-op — fall back to the stashed-pieces path.
    let type_code = match arch.types().get_type_code_proto(pieces) {
        Ok(tc) => tc,
        Err(_) => return Ok(()),
    };
    let _ = arch.symboltab.retype_symbol(sid, type_code);
    Ok(())
}

/// Shared body of `IfcParseFile`/`IfcParseLine` (`ifacedecomp.cc:347,384`): run
/// `parse_C` against the program's [`Architecture`].  A `ParseError` is reported
/// as the C++ does — `"Error in C syntax: <explain>"` on the output stream, then
/// the `IfaceExecutionError("Bad C syntax")`.
fn run_parse_c(status: &mut IfaceStatus, content: &str) -> IfaceResult<()> {
    use std::cell::RefCell;
    // The factory + data-org from the program; the parse store-writes go through
    // the factory (interior mutability), so an immutable borrow of `prog` suffices.
    let (org, extern_pieces, parse_result) = {
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_ref().expect("conf checked non-None above");
        let arch = prog.arch();
        let (addr_size, word_size) = arch.data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        // setPrototype branch: stash the parsed pieces (applied below, against the
        // mutable `dcp`); the symbol existence check mirrors C++ queryFunction.
        let captured: RefCell<Option<kuna_decomp::fspec::PrototypePieces>> = RefCell::new(None);
        let res = crate::grammar::parse_c(content, arch.types(), org, |pieces| {
            // C++ Architecture::setPrototype resolves the function via queryFunction
            // (which lazily builds the Funcdata from the function symbol) and locks
            // the prototype.  In the kuna console boundary the named function may live
            // only in the binaryimage's symbol records (the readLoaderSymbols →
            // Scope::addFunction markup is a W4 stub, so it is not yet in the
            // symboltab), and `dcp.fd` is built later by `load function`.  So the
            // pieces are captured here and stashed (applied at load time) rather
            // than rejected — letting `parse line extern` take effect and the test
            // proceed to decompile.  // STUB(W4 queryFunction/FuncProto restore)
            *captured.borrow_mut() = Some(pieces);
            Ok(())
        });
        (org, captured.into_inner(), res)
    };
    let _ = org; // org is consumed by parse_c; kept for symmetry with the C++ glb
    match parse_result {
        Ok(()) => {
            if let Some(pieces) = extern_pieces {
                // C++ `Architecture::setPrototype(pieces)` (architecture.cc:393):
                // resolve the FunctionSymbol by name and lock the parsed prototype
                // onto it (`fd->getFuncProto().setPieces(pieces)`).  In kuna the
                // function's data-type IS its `TypeCode`, whose `getPrototype()`
                // carries the locked `FuncProto`; so we build the prototype-bearing
                // TypeCode (`getTypeCode(PrototypePieces)`) and retype the symbol.
                // This makes a *callee*'s declared signature visible to
                // `ActionDefaultParams`' `fc->copy(otherfunc->getFuncProto())` —
                // without it a by-value struct call argument never gets the callee
                // param type and `RulePieceStructure` cannot split its CONCAT into
                // per-field writes.  (Generic over the signature: keyed by the
                // declared name only, exactly as the C++ `queryFunction(basename)`.)
                apply_prototype_to_symbol(status, &pieces)?;
                // Also stash for re-application when THIS function is the one being
                // decompiled (the IR is rebuilt on `decompile`, discarding the
                // symbol-table proto link for the active Funcdata).
                let dcp = dcp_mut(status)?;
                dcp.pending_prototypes.insert(pieces.name.clone(), pieces);
            }
            Ok(())
        }
        Err(e) => {
            status.out(&format!("Error in C syntax: {}\n", e.explain()));
            Err(IfaceError::execution("Bad C syntax"))
        }
    }
}

/// Split a `<func>::<operand>` console token into its two halves, or `None` when
/// the token carries no qualifier.  A C++ name is split at its LAST `::`, the
/// same reading `--assert` uses (`kuna_cli::assertdecl::split_qualifier`).
fn split_qualifier(tok: &str) -> Option<(String, String)> {
    match tok.rsplit_once("::") {
        Some((func, operand)) if !func.is_empty() && !operand.is_empty() => {
            Some((func.to_string(), operand.to_string()))
        }
        _ => None,
    }
}

/// The `<func>` a `map param` / `map return` operand names, without consuming
/// the stream.  `map param`'s first operand is a decimal index and `map
/// return`'s is a machine address, so neither can hold a `::` of its own.
fn peek_qualifier(s: &CommandStream) -> Option<String> {
    let text = s.rest();
    let first = text.split_whitespace().next()?;
    split_qualifier(first).map(|(func, _)| func)
}

/// Merge one declared input slot into the prototype pieces parked for `func`.
///
/// The pieces store is the only channel a callee's signature reaches a CALLER
/// through (`IfcDecompile` re-parks every entry onto its `FunctionSymbol`, where
/// `ActionDefaultParams` reads it), and it describes types — so the explicit
/// storage rides in `input_storage`, the input-side twin of what `map return`
/// already parks in `output_storage`.
///
/// Slots may be declared in any order; a slot no directive named yet is filled
/// with `undefined<addr_size>`, which is what the decompiler says about a value
/// it has not been told anything about.
fn bind_func_param(
    status: &mut IfaceStatus,
    func: &str,
    index: kuna_base::types::int4,
    piece: kuna_decomp::fspec::ParameterPieces,
    pname: &str,
) -> IfaceResult<()> {
    use kuna_decomp::fspec::PrototypePieces;
    if index < 0 {
        return Err(IfaceError::parse("Parameter index must not be negative"));
    }
    let filler = {
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_ref()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        let (addr_size, _) = prog.arch().data_org();
        prog.arch()
            .types()
            .get_base(addr_size, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?
    };
    let dcp = dcp_mut(status)?;
    let entry = dcp
        .pending_prototypes
        .entry(func.to_string())
        .or_insert_with(|| PrototypePieces {
            name: func.to_string(),
            first_var_arg_slot: -1,
            ..Default::default()
        });
    let slot = index as usize;
    while entry.intypes.len() <= slot {
        entry.intypes.push(std::rc::Rc::clone(&filler));
    }
    while entry.innames.len() <= slot {
        entry.innames.push(String::new());
    }
    entry.intypes[slot] = piece.type_.clone().ok_or_else(|| {
        IfaceError::parse("A parameter declaration must name a data-type")
    })?;
    entry.innames[slot] = pname.to_string();
    entry.input_storage.retain(|(i, _)| *i != index);
    entry.input_storage.push((index, piece));
    let pieces = entry.clone();
    retype_symbol_if_complete(status, &pieces)
}

/// Park the locked return storage + type of `func` on its prototype pieces —
/// the cross-function arm of `map return`, and the `output_storage` half of what
/// [`bind_func_param`] does for inputs.
fn bind_func_return(
    status: &mut IfaceStatus,
    func: &str,
    piece: kuna_decomp::fspec::ParameterPieces,
) -> IfaceResult<()> {
    use kuna_decomp::fspec::PrototypePieces;
    let dcp = dcp_mut(status)?;
    let entry = dcp
        .pending_prototypes
        .entry(func.to_string())
        .or_insert_with(|| PrototypePieces {
            name: func.to_string(),
            first_var_arg_slot: -1,
            ..Default::default()
        });
    entry.output_storage = Some(piece);
    let pieces = entry.clone();
    retype_symbol_if_complete(status, &pieces)
}

/// Retype the named `FunctionSymbol` to the prototype-bearing `TypeCode`, but
/// only once the pieces describe a return type.
///
/// `param`/`return` build a signature one slot at a time, so the pieces are
/// incomplete in between — and the storage assignment behind `getTypeCode`
/// dereferences `outtype` unconditionally.  The `PrototypePieces` parked on the
/// symbol is what a caller actually reads (`ActionDefaultParams`); the retype is
/// the extra step that lets a by-value struct argument be split, so skipping it
/// for an input-only declaration costs nothing else.
fn retype_symbol_if_complete(
    status: &mut IfaceStatus,
    pieces: &kuna_decomp::fspec::PrototypePieces,
) -> IfaceResult<()> {
    if pieces.outtype.is_none() {
        return Ok(());
    }
    apply_prototype_to_symbol(status, pieces)
}

/// Parse the `<storage> <C typedeclaration>` tail both `map param` and `map
/// return` end in, into a type-locked [`ParameterPieces`] plus the declared name.
fn parse_storage_and_type(
    status: &mut IfaceStatus,
    s: &mut CommandStream,
    flags: kuna_base::types::uint4,
) -> IfaceResult<(kuna_decomp::fspec::ParameterPieces, String)> {
    use kuna_decomp::fspec::ParameterPieces;
    let dcp = dcp_mut(status)?;
    let prog = dcp
        .conf
        .as_mut()
        .ok_or_else(|| IfaceError::execution("No load image present"))?;
    let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
    s.skip_ws();
    let (addr_size, word_size) = prog.arch().data_org();
    let org = crate::grammar::DataOrg { addr_size, word_size };
    let typetext = s.rest();
    let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
        .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
    Ok((ParameterPieces { addr, type_: Some(ct), flags }, name))
}

/// (kuna) Bind a parsed C prototype to `func`, whatever name the declaration
/// carries — the console spelling of `--assert 'prototype <func> <decl>'`
/// (`map prototype`, registered in [`crate::kuna_console`]).
///
/// `parse line extern <decl>` keys the prototype by the DECLARATION's name, so a
/// declaration that renames the function ("this `sub_1400055e0` is really
/// `sha256`") parks a signature on a fresh unrelated symbol and leaves the
/// selected function with its recovered one.  Here `<func>` says which function
/// the signature describes, which is what the in-process surface
/// (`assertions::apply_prototype`) already did — the two surfaces disagreed
/// about the same directive (`docs/re-needs/text-output-silently-ignores.md`).
///
/// `<func>` is a name or an entry address
/// ([`crate::assertions::resolve_proto_target`]); the pieces are parked through
/// the shared [`crate::assertions::park_prototype`] so both surfaces bind the
/// same operand to the same function.
pub(crate) fn bind_prototype(
    status: &mut IfaceStatus,
    func: &str,
    decl: &str,
) -> IfaceResult<()> {
    use std::cell::RefCell;
    let decl = decl.trim();
    let text = if decl.ends_with(';') {
        format!("extern {decl}")
    } else {
        format!("extern {decl};")
    };
    let (parsed, captured) = {
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_ref()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        let arch = prog.arch();
        let (addr_size, word_size) = arch.data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let captured: RefCell<Option<kuna_decomp::fspec::PrototypePieces>> = RefCell::new(None);
        let parsed = crate::grammar::parse_c(&text, arch.types(), org, |pieces| {
            *captured.borrow_mut() = Some(pieces);
            Ok(())
        });
        (parsed, captured.into_inner())
    };
    if let Err(e) = parsed {
        status.out(&format!("Error in C syntax: {}\n", e.explain()));
        return Err(IfaceError::execution("Bad C syntax"));
    }
    let mut pieces =
        captured.ok_or_else(|| IfaceError::execution("Not a function declaration"))?;
    let dcp = dcp_mut(status)?;
    let prog = dcp
        .conf
        .as_mut()
        .ok_or_else(|| IfaceError::execution("No load image present"))?;
    let target = crate::assertions::resolve_proto_target(prog, func)
        .map_err(IfaceError::execution)?;
    crate::assertions::park_prototype(prog, &target, &mut pieces);
    dcp.pending_prototypes.insert(target.name().to_string(), pieces);
    Ok(())
}

// ===========================================================================
// The "decompile" module commands.
//
// One unit struct per C++ `Ifc*` class.  Every `module()` is "decompile"; the
// first-registered command (`IfcComment`, see register_decomp_commands) carries
// the `create_data()` that builds the shared IfaceDecompData.
// ===========================================================================

/// Define a decompiler console command (ported from `ifacedecomp.cc`).
///
/// In C++ *every* `IfaceDecompCommand::createData()` can build the data, but
/// `registerCom` calls it only once (first module sighting).  We give the
/// builder to a single sentinel command type ([`IfcComment`], the first
/// registered) via the `with_data` arm, and the trait-default `create_data`
/// (`None`) to the rest; the observable result is identical — one
/// [`IfaceDecompData`] per module.
macro_rules! decomp_command {
    // Variant carrying the module-data constructor (the first-registered command).
    ($(#[$m:meta])* $name:ident, with_data, $exec:item) => {
        $(#[$m])*
        pub struct $name;
        impl IfaceCommandAction for $name {
            $exec
            fn module(&self) -> String {
                DECOMPILE_MODULE.to_string()
            }
            fn create_data(&self) -> Option<Box<dyn IfaceData>> {
                Some(Box::new(IfaceDecompData::default()))
            }
        }
    };
    // Plain variant (module data already created by the sentinel).
    ($(#[$m:meta])* $name:ident, $exec:item) => {
        $(#[$m])*
        pub struct $name;
        impl IfaceCommandAction for $name {
            $exec
            fn module(&self) -> String {
                DECOMPILE_MODULE.to_string()
            }
        }
    };
}

// --- Comments (ifacedecomp.cc:292) -----------------------------------------

decomp_command!(
    /// C++ `IfcComment` (`ifacedecomp.cc:292`): a comment line in a script
    /// (`//`/`#`/`%`) — does nothing.  Carries the shared module-data builder.
    IfcComment, with_data,
    fn execute(&self, _status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        // Do nothing
        Ok(())
    }
);

// --- option <name> [p1] [p2] [p3] (ifacedecomp.cc:304) ---------------------

decomp_command!(
    /// C++ `IfcOption`: adjust a decompiler option.
    ///
    /// The argument parse (option name required, up to three params, "Too many
    /// option parameters" on a fourth) is ported faithfully; the
    /// `OptionDatabase::set` call needs `Architecture: ArchOptionContext`, which
    /// the merged tree does not provide (see module docs).
    IfcOption,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        s.skip_ws();
        let optname = s.read_token();
        s.skip_ws();
        if optname.is_empty() {
            return Err(IfaceError::parse("Missing option name"));
        }
        let (mut p1, mut p2, mut p3) = (String::new(), String::new(), String::new());
        if !s.eof() {
            p1 = s.read_token();
            s.skip_ws();
            if !s.eof() {
                p2 = s.read_token();
                s.skip_ws();
                if !s.eof() {
                    p3 = s.read_token();
                    s.skip_ws();
                    if !s.eof() {
                        return Err(IfaceError::parse("Too many option parameters"));
                    }
                }
            }
        }
        // kuna stage-model options (`KUNA_OPTION_NAMES`) are not in the upstream
        // `OptionDatabase` / `ElementId` registry; route them to
        // `Architecture::set_kuna_option`, which validates the value and writes
        // the live flag the consuming action/printer reads.  Upstream options
        // fall through to the byte-identical `OptionDatabase` path below.
        if kuna_decomp::options::KUNA_OPTION_NAMES.contains(&optname.as_str()) {
            let prog = dcp.conf.as_mut().expect("conf checked non-None above");
            let res = prog
                .arch_mut()
                .set_kuna_option(&optname, &p1)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            status.out(&format!("{res}\n"));
            return Ok(());
        }
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let id = prog.registry().find_element(&optname, 0);
        if id == 0 {
            // C++ ElementId::find returns 0 for an unknown name; OptionDatabase::set
            // then throws "Unknown option" (a ParseError-class LowlevelError).
            return Err(IfaceError::execution("Unknown option"));
        }
        // C++ `dcp->conf->options->set(...)`.  The OptionDatabase is a stateless
        // registry of the same option set (`OptionDatabase::new` registers them
        // all); building it fresh avoids aliasing `conf`'s Architecture, which the
        // `set` call borrows mutably (the e2e gate uses the same shape).
        let options = OptionDatabase::new();
        let res = options
            .set(prog.arch_mut(), id, &p1, &p2, &p3)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        status.out(&format!("{res}\n"));
        Ok(())
    }
);

// --- parse file <filename> / parse line ... (ifacedecomp.cc:347, 384) ------

decomp_command!(
    /// C++ `IfcParseFile`: parse a file of C declarations.
    IfcParseFile,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        s.skip_ws();
        let filename = s.read_filename();
        if filename.is_empty() {
            return Err(IfaceError::parse("Missing filename"));
        }
        // C++ opens the file then parse_C(dcp->conf,fs).  "Unable to open file: "
        // on a failed open; "Error in C syntax: ..."/"Bad C syntax" on a parse
        // error.
        let content = std::fs::read_to_string(&filename)
            .map_err(|_| IfaceError::execution(format!("Unable to open file: {filename}")))?;
        run_parse_c(status, &content)
    }
);

decomp_command!(
    /// C++ `IfcParseLine`: parse a line of C syntax (`parse line ...`).
    IfcParseLine,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("No input"));
        }
        // The remainder of the command line is the C declaration to parse.
        let line = s.rest();
        run_parse_c(status, &line)
    }
);

// --- adjust vma <offset> (ifacedecomp.cc:409) ------------------------------

decomp_command!(
    /// C++ `IfcAdjustVma`: shift the load image base address.
    IfcAdjustVma,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        // C++ reads `adjust` with a user-specified base then loader->adjustVma.
        Err(engine_unavailable("LoadImage::adjustVma (Architecture::loader)"))
    }
);

// --- load function <name> / load addr <addr> [name] (ifacedecomp.cc:466,496)

decomp_command!(
    /// C++ `IfcLoadFile` (`consolemain.cc:46`): load an image file (`load file
    /// [<target>] <filename>`).
    ///
    /// The C++ console drives a real binary through BFD; the kuna Rust engine's
    /// only load-image backend is the XML `<binaryimage>` format (the BFD backend
    /// is a later port item).  So `load file <path>` accepts the corpus
    /// `<binaryimage>`/`<decompilertest>` XML the Python tools and datatests feed.
    /// The optional leading `<target>` (a BFD target) is parsed and ignored (the
    /// XML carries its own `arch` attribute).
    IfcLoadFile,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++: s >> filename; if !eof { target=filename; s>>filename; }
        // `read_filename` so a quoted path survives its own spaces; unquoted
        // input still tokenizes on whitespace exactly as C++ does.
        let mut filename = s.read_filename();
        let mut target = String::new();
        s.skip_ws();
        if !s.eof() {
            // Two parameters: the first was the target, the second is the file.
            target = std::mem::take(&mut filename);
            filename = s.read_filename();
        }
        if filename.is_empty() {
            return Err(IfaceError::parse("Missing filename"));
        }
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_some() {
                return Err(IfaceError::execution("Load image already present"));
            }
        }
        // Read the spec roots off the shared data (set by the binary at startup).
        let spec_roots = {
            let dcp = dcp_mut(status)?;
            dcp.spec_roots.clone()
        };
        // capa->buildArchitecture + conf->init(store) (the bootstrap chain).
        match bootstrap_from_file(&filename, &target, &spec_roots) {
            Ok(prog) => {
                // *status->optr << filename << " successfully loaded: " << desc;
                let desc = prog.description().to_string();
                let dcp = dcp_mut(status)?;
                dcp.conf = Some(prog);
                status.out(&format!("{filename} successfully loaded: {desc}\n"));
                Ok(())
            }
            Err(e) => {
                // C++ on init failure: print the error + "Could not create
                // architecture", then leave conf null (NOT a thrown error).
                status.out(&format!("{}\n", e.explain()));
                status.out("Could not create architecture\n");
                Ok(())
            }
        }
    }
);

decomp_command!(
    /// C++ `IfcFuncload`: make a named function current (`load function
    /// <name>`), following its flow if it has code.
    IfcFuncload,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // (kuna `symbolnamebound`) Canonicalize what the user typed to the
        // spelling the symbol table and the listing use, so a name the binary
        // spells past the scope bound both RESOLVES and renders as one name.
        // Idempotent, and a no-op for every real name.
        let funcname =
            kuna_decomp::kuna_symbolnamebound::bound_scope_path(&s.read_token(), "::").into_owned();
        let command_seq = status.command_seq;
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No image loaded"));
        }
        // The kuna path resolves the entry from the binaryimage's own symbol
        // records (the readLoaderSymbols hook), where C++ uses
        // resolveScopeFromSymbolName + queryFunction + followFlow.
        let mut flow_overrides =
            dcp.pending_flow_overrides.get(&funcname).cloned().unwrap_or_default();
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        // (kuna) Safety commit: if a session loads a function without an explicit
        // `read symbols`, commit the stashed analysis facts now (default gates;
        // no-op once committed). The CLI always emits `read symbols` first, where
        // the per-pass `--option` gates apply; this only covers a hand session.
        prog.commit_pending_analysis()
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let selector = crate::engine::EntrySelector::parse(&funcname);
        let selected = prog
            .resolve_entry(&selector)
            .map_err(|error| IfaceError::execution(error.to_string()))?;
        let entry = selected.addr;
        let resolved_name = if matches!(selector, crate::engine::EntrySelector::Name(_)) {
            funcname
        } else {
            selected.name
        };
        // (kuna, Ghidra-gap) Apply the analysis's `call error(nonzero,…)` no-return facts
        // as CALL_RETURN flow overrides — the SAME prune `decompile-all` does — so the
        // single-function console decompile does NOT overrun past a no-returning
        // `call error` into the following function. `error_noreturn_callsites` is empty
        // unless the Listing + `noreturn_error` are on (so a listing-less session is
        // unchanged). The whole binary's list is passed; only sites this function's flow
        // visits are applied.
        if let Some(space) = entry.get_space() {
            for &off in &prog.arch().error_noreturn_callsites {
                flow_overrides.push((
                    kuna_base::address::Address::new(std::rc::Rc::clone(space), off),
                    kuna_decomp::overrides::flow_type::CALL_RETURN,
                ));
            }
        }
        // Build the Funcdata + follow flow (C++ Funcdata + followFlow), seeding any
        // `override flow` facts stashed for this function before flow follows.
        // A `function bounds` declaration for this entry bounds the follow;
        // without one the extent is the natural (unbounded) one.
        let declared = prog.declared_extent(entry.get_offset());
        let entry_stamp = entry.clone();
        let fd = build_and_follow_flow_with_override(
            prog.arch_mut(),
            &resolved_name,
            entry,
            declared,
            &flow_overrides,
        )
        .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // (kuna) Stamp the follow so an immediately-following `decompile` can adopt
        // this IR rather than following the same flow again (see `PristineFlow`).
        dcp.pristine_flow = Some(PristineFlow {
            command_seq,
            name: resolved_name,
            entry: entry_stamp,
            size: declared,
            flow_overrides,
        });
        dcp.fd = Some(fd);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcAddrrangeLoad`: create a function at an address (`load addr
    /// <addr> [name]`).
    IfcAddrrangeLoad,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let command_seq = status.command_seq;
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No binary loaded"));
        }
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        // (kuna) Safety commit (see IfcFuncload): commit stashed analysis facts if
        // a session reaches `load addr` without an explicit `read symbols`.
        prog.commit_pending_analysis()
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // C++ Address offset = parse_machaddr(s,size,*dcp->conf->types) — the full
        // console address grammar over the engine spaces.
        let (requested, parsed_size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        let in_default_space = requested
            .get_space()
            .zip(prog.arch().manage().get_default_code_space())
            .is_some_and(|(requested, default)| std::rc::Rc::ptr_eq(requested, default));
        let selected = if in_default_space {
            prog.resolve_entry(&crate::engine::EntrySelector::Numeric(requested.get_offset()))
        } else {
            prog.resolve_address(&requested)
        }
        .map_err(|error| IfaceError::execution(error.to_string()))?;
        let offset = selected.addr;
        s.skip_ws();
        let name = s.read_token(); // optional
        // No explicit name: prefer the FunctionSymbol already installed here,
        // resolved across scopes so a demangled C++ entry reports its qualified
        // `Class::method` form.  Jumping straight to C++ `nameFunction` printed a
        // `sub_<addr>` header for `--addr` on an UNSTRIPPED binary where the by-name
        // path printed the real name (DIV-59); it stays the unknown-address fallback.
        let name = if !name.is_empty() {
            name
        } else {
            prog.arch()
                .symboltab
                .function_display_name_across_scopes(&offset)
                .unwrap_or_else(|| prog.arch().name_function(&offset))
        };
        // (kuna, Ghidra-gap) error(nonzero) no-return prune, mirroring decompile-all and
        // IfcFuncload: apply the analysis's `call error(nonzero,…)` facts as CALL_RETURN
        // flow overrides so `load addr` does NOT overrun past a no-returning `call error`
        // into the next function. Empty unless the Listing + noreturn_error are on.
        let mut flow_overrides: Vec<(kuna_base::address::Address, kuna_base::types::uint4)> =
            Vec::new();
        if let Some(space) = offset.get_space() {
            for &off in &prog.arch().error_noreturn_callsites {
                flow_overrides.push((
                    kuna_base::address::Address::new(std::rc::Rc::clone(space), off),
                    kuna_decomp::overrides::flow_type::CALL_RETURN,
                ));
            }
        }
        // The symbol-table addFunction is a later boundary; build the Funcdata
        // + follow flow directly (C++ addFunction + followFlow).
        // `load addr [ram,<start>,<size>]` states the extent inline (C++
        // `followFlow(offset, offset+size)`); an extent already declared for this
        // entry by `map function` applies when the command carries none.  The
        // parsed size is the address's own byte width when no `,<size>` was
        // given, so only a size that exceeds it is a real bound.
        let inline_size =
            if parsed_size > requested.get_addr_size() { parsed_size } else { UNBOUNDED_SIZE };
        let declared =
            if inline_size > 0 { inline_size } else { prog.declared_extent(offset.get_offset()) };
        if inline_size > 0 {
            prog.declare_extent(offset.get_offset(), inline_size);
        }
        let entry_stamp = offset.clone();
        let name_stamp = name.clone();
        let fd =
            build_and_follow_flow_with_override(prog.arch_mut(), &name, offset, declared, &flow_overrides)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // (kuna) Same follow stamp as `load function` (see `PristineFlow`).
        dcp.pristine_flow = Some(PristineFlow {
            command_seq,
            name: name_stamp,
            entry: entry_stamp,
            size: declared,
            flow_overrides,
        });
        dcp.fd = Some(fd);
        Ok(())
    }
);

// --- read symbols / clear architecture (ifacedecomp.cc:529, 518) -----------

decomp_command!(
    /// C++ `IfcReadSymbols`: read symbols from the load image.
    IfcReadSymbols,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        // C++ `dcp->conf->readLoaderSymbols("::")`.  The kuna XML engine reads the
        // binaryimage's `<symbol>` records into the program's name→address table
        // at `load file` (the readLoaderSymbols hook runs eagerly there), so the
        // symbols are already available.
        //
        // (kuna) This is also the gated-commit point for the kuna_analysis passes:
        // `bootstrap_from_object` STASHES the per-pass facts at load (no longer
        // commits them eagerly) so they can be committed here, AFTER the per-pass
        // `--option <id> on|off` flags have been applied — the CLI `build_script`
        // emits the `option` lines BEFORE `read symbols`. A disabled pass's facts
        // are dropped; the default (all-on except addrtable) commit is identical to
        // the old eager behavior. A no-op on the XML datatest path (nothing
        // stashed), so the 675/158 parity oracles are structurally untouched.
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        prog.commit_pending_analysis()
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcCleararch`: clear the current architecture/program.
    IfcCleararch,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        dcp.clear_architecture();
        Ok(())
    }
);

// --- map ... (ifacedecomp.cc:550-799) --------------------------------------

decomp_command!(
    /// C++ `IfcMapaddress`: `map address <addr> <typedeclaration>`.
    IfcMapaddress,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        let fd_present = dcp_mut(status)?.fd.is_some();
        if fd_present {
            // C++ fd-local form (ifacedecomp.cc:561-563).
            use kuna_decomp::varnode::varnode_flags;
            let dcp = dcp_mut(status)?;
            let prog = dcp.conf.as_mut().expect("conf checked non-None above");
            let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
            s.skip_ws();
            let (addr_size, word_size) = prog.arch().data_org();
            let org = crate::grammar::DataOrg { addr_size, word_size };
            let typetext = s.rest();
            let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
                .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
            let invalid = kuna_base::address::Address::new_invalid();
            let fd = dcp.fd.as_mut().expect("fd checked Some above");
            let scope_local = fd.get_scope_local_mut().ok_or_else(|| {
                IfaceError::execution("Function has no local scope (no stack space)")
            })?;
            let sym = scope_local
                .add_symbol(&name, ct, &addr, &invalid)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            scope_local.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
            return Ok(());
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        // Parse the required type + name (C++ parse_type).
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let typetext = s.rest();
        let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
        // Global branch: build the flags, resolve/create the scope, add the mapped
        // symbol, set the locks, and (for a namespace scope) register its range.
        use kuna_decomp::varnode::varnode_flags;
        let inherit = prog.arch().symboltab.get_property(&addr);
        let flags = varnode_flags::namelock | varnode_flags::typelock | inherit;
        let num_spaces = prog.arch().manage().num_spaces() as int4;
        let arch = prog.arch_mut();
        let (scope, basename) = arch
            .symboltab
            .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let invalid = kuna_base::address::Address::new_invalid();
        let (sym, eref) = arch
            .symboltab
            .add_symbol_mapped(scope, &basename, ct, &addr, &invalid)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        arch.symboltab.set_attribute(sym, flags);
        // C++ ifacedecomp.cc:573-576: if this is a (global) namespace scope (it has
        // a parent), register the symbol's whole-map address range on that scope so
        // address->symbol resolution descends into the namespace.
        if arch.symboltab.scope_has_parent(scope) {
            let entry = arch.symboltab.entry(scope, eref);
            let spc = entry.addr.get_space().expect("mapped entry has a space").clone();
            let first = entry.get_first();
            let last = entry.get_last();
            arch.symboltab.add_range(scope, spc, first, last);
        }
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMaphash`: `map hash <addr> <hash> <typedeclaration>`.
    IfcMaphash,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++ ifacedecomp.cc:588-605.
        {
            let dcp = dcp_mut(status)?;
            if dcp.fd.is_none() {
                return Err(IfaceError::execution("No function loaded"));
            }
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        use kuna_decomp::varnode::varnode_flags;
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        // C++ `s >> hex >> hash`: a hexadecimal dynamic-hash value.
        let hash = parse_hex_u64(s).map_err(IfaceError::parse)?;
        s.skip_ws();
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let typetext = s.rest();
        let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        let scope_local = fd.get_scope_local_mut().ok_or_else(|| {
            IfaceError::execution("Function has no local scope (no stack space)")
        })?;
        let sym = scope_local
            .add_dynamic_symbol(&name, ct, &addr, hash)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        scope_local.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMapParam`: `map param <i> <addr> <typedeclaration>`
    /// (ifacedecomp.cc:613).  Lock the storage and data-type of the `i`-th input
    /// parameter on the current function's prototype.
    IfcMapParam,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces};
        // (kuna) `map param <func>::<i> <storage> <decl>` declares a slot of the
        // prototype of the function it NAMES, which need not be the one under
        // decompile.  Without it a caller has no way to say what a CALLEE takes,
        // and the qualifier the `--assert` grammar documents was dropped on the
        // way here, so the directive silently retyped the caller instead
        // (`docs/re-needs/qualified-parameter-assertions-modify.md`).
        if peek_qualifier(s).is_some() {
            let tok = s.read_token();
            let (func, index_tok) =
                split_qualifier(&tok).expect("peek_qualifier saw a qualifier");
            let index: kuna_base::types::int4 = index_tok.parse().map_err(|_| {
                IfaceError::parse("Parameter index must be a decimal number")
            })?;
            s.skip_ws();
            let flags = parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK;
            let (piece, pname) = parse_storage_and_type(status, s, flags)?;
            return bind_func_param(status, &func, index, piece, &pname);
        }
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        // C++ `s >> dec >> i`: the parameter position.
        let i = s.read_int();
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_mut()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        // C++: piece.addr = parse_machaddr(s,size,*dcp->conf->types).
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        // C++: piece.type = parse_type(s,name,dcp->conf).
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let typetext = s.rest();
        let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
        // C++: piece.flags = ParameterPieces::typelock | ParameterPieces::namelock.
        let piece = ParameterPieces {
            addr,
            type_: Some(ct),
            flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
        };
        // The C++ `FuncProto::store` is always present (set by `setScope` at
        // Funcdata construction); the merged-tree load path leaves it null, so
        // attach the stand-alone internal store before writing (as IfcMapReturn).
        let void_type = prog
            .arch()
            .types()
            .get_type_void()
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // C++: dcp->fd->getFuncProto().setParam(i, name, piece).
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        fd.get_func_proto_mut().attach_internal_store(void_type);
        fd.get_func_proto_mut().set_param(i, &name, &piece);
        // Stash the lock keyed by function name so the `decompile` IR rebuild can
        // re-seed it on the fresh prototype (C++ keeps it on the reused Funcdata;
        // the kuna console discards `dcp.fd` on `decompile`).  Precedent:
        // `pending_prototypes` / `pending_flow_overrides`.
        let fname = fd.get_name().to_string();
        dcp.pending_param_maps
            .entry(fname)
            .or_default()
            .push((i, name, piece));
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMapReturn`: `map return <addr> <typedeclaration>`
    /// (ifacedecomp.cc:635-648).  Set a locked return-value storage on the
    /// current function's prototype.
    IfcMapReturn,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces};
        // (kuna) `map return <func>::<storage> <decl>` — the cross-function arm,
        // as for `map param` above.  The storage grammar has no `::` of its own,
        // so the qualifier is unambiguous.
        if let Some(func) = peek_qualifier(s) {
            let text = s.rest();
            let mut parts = text.trim_start().splitn(2, char::is_whitespace);
            let first = parts.next().unwrap_or("");
            let tail = parts.next().unwrap_or("");
            let storage = split_qualifier(first).map(|(_, op)| op).unwrap_or_default();
            let dequalified = format!("{storage} {tail}");
            let mut stream = CommandStream::new(&dequalified);
            let (piece, _) =
                parse_storage_and_type(status, &mut stream, parameter_pieces_flags::TYPELOCK)?;
            return bind_func_return(status, &func, piece);
        }
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_mut()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        // C++: piece.addr = parse_machaddr(s,size,*dcp->conf->types).
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        // C++: piece.type = parse_type(s,name,dcp->conf).
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let typetext = s.rest();
        let (ct, _name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
        let piece = ParameterPieces {
            addr,
            type_: Some(ct),
            flags: parameter_pieces_flags::TYPELOCK,
        };
        // The C++ `FuncProto::store` is always present (set by `setScope` at
        // Funcdata construction); the merged-tree load path leaves it null, so
        // attach the stand-alone internal store before writing the output (C++
        // `store->setOutput(piece)`).  Idempotent.
        let void_type = prog
            .arch()
            .types()
            .get_type_void()
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // C++: dcp->fd->getFuncProto().setOutput(piece).
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        let fname = fd.get_name().to_string();
        fd.get_func_proto_mut().attach_internal_store(void_type);
        fd.get_func_proto_mut().set_output(&piece);
        // (kuna) C++ keeps this locked output live on the callee `Funcdata`, so a
        // caller's `ActionDefaultParams` (`fc->copy(otherfunc->getFuncProto())`)
        // picks up the custom (possibly stack-relative) return storage.  The
        // merged tree rebuilds callee prototypes from `PrototypePieces`, so park an
        // *output-only* pieces (no declared inputs → input recovery stays
        // model-driven, matching C++ `map return`) carrying the explicit output
        // storage.  Merge into any existing pending proto for this function.
        let entry = dcp
            .pending_prototypes
            .entry(fname.clone())
            .or_insert_with(|| kuna_decomp::fspec::PrototypePieces {
                name: fname.clone(),
                first_var_arg_slot: -1,
                ..Default::default()
            });
        entry.output_storage = Some(piece);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMapfunction`: `map function <addr> [name] [nocode]`.
    IfcMapfunction,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No binary loaded"));
            }
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        let mut name = s.read_token(); // optional
        if name.is_empty() {
            name = prog.arch().name_function(&addr);
        }
        let type_code = prog
            .arch()
            .types()
            .get_type_code()
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let min_size = prog.arch().min_funcsymbol_size;
        let num_spaces = prog.arch().manage().num_spaces() as int4;
        let arch = prog.arch_mut();
        let (scope, basename) = arch
            .symboltab
            .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        arch.symboltab
            .add_function(scope, &addr, &basename, min_size, type_code)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // C++ `dcp->fd = scope->addFunction(addr,name)->getFunction()`: make the
        // newly-mapped function the current function so the override commands
        // (`override flow|prototype`, which require `dcp->fd != 0`) can attach to
        // it.  The C++ `getFunction()` lazily builds the Funcdata WITHOUT following
        // flow; the kuna boundary builds the same un-followed Funcdata (the real flow
        // follow runs at `load function`/`decompile`).
        let fd = prog
            .arch()
            .new_funcdata(&name, addr.clone(), UNBOUNDED_SIZE)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // Make the function loadable by `load function <name>` (the console boundary).
        prog.register_symbol(&name, addr);
        // C++ reads an optional trailing "nocode" keyword (setNoCode on fd).
        s.skip_ws();
        let nocode = s.read_token();
        let dcp = dcp_mut(status)?;
        dcp.fd = Some(fd);
        if nocode == "nocode" {
            if let Some(fd) = dcp.fd.as_mut() {
                fd.set_no_code(true);
            }
        }
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMapexternalref`: `map externalref <addr> <ref> [name]`.
    IfcMapexternalref,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("parse_machaddr + Scope::addExternalRef"))
    }
);

decomp_command!(
    /// C++ `IfcMaplabel`: `map label <name> <address>`.
    IfcMaplabel,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let name = s.read_token();
        if name.is_empty() {
            return Err(IfaceError::parse("Need label name and address"));
        }
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        use kuna_decomp::varnode::varnode_flags;
        let fd_present = dcp_mut(status)?.fd.is_some();
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        let lab_type = prog
            .arch()
            .types()
            .get_base(1, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        if fd_present {
            // C++ fd-local form.
            let fd = dcp.fd.as_mut().expect("fd checked Some above");
            let scope_local = fd.get_scope_local_mut().ok_or_else(|| {
                IfaceError::execution("Function has no local scope (no stack space)")
            })?;
            let sym = scope_local
                .add_code_label(&addr, &name, lab_type)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            scope_local.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
            return Ok(());
        }
        let arch = prog.arch_mut();
        let gscope = arch
            .symboltab
            .get_global_scope()
            .ok_or_else(|| IfaceError::execution("No global scope"))?;
        let sym = arch
            .symboltab
            .add_code_label(gscope, &addr, &name, lab_type)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        arch.symboltab.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMapconvert`: `map convert <format> <value> <addr> <hash>`
    /// (ifacedecomp.cc:735).  Add an EquateSymbol forcing the display format of
    /// the constant `value` (parsed as hex) at the dynamic location.
    IfcMapconvert,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        use kuna_decomp::database::symbol_dispflags;
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        // C++ `s >> name`: the format token.
        let name = s.read_token();
        let format = match name.as_str() {
            "hex" => symbol_dispflags::FORCE_HEX,
            "dec" => symbol_dispflags::FORCE_DEC,
            "bin" => symbol_dispflags::FORCE_BIN,
            "oct" => symbol_dispflags::FORCE_OCT,
            "char" => symbol_dispflags::FORCE_CHAR,
            _ => return Err(IfaceError::parse("Bad convert format")),
        };
        // C++ `s >> ws >> hex >> value`: the constant value, always hexadecimal.
        let value = parse_hex_u64(s).map_err(IfaceError::parse)?;
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_mut()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        // C++ `parse_machaddr(s,size,*dcp->conf->types)`: the pc address of the hash.
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        // C++ `s >> hex >> hash`: the dynamic-hash value.
        let hash = parse_hex_u64(s).map_err(IfaceError::parse)?;
        // C++ EquateSymbol type is getBase(1,TYPE_UNKNOWN).
        let base1 = prog
            .arch()
            .types()
            .get_base(1, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        let scope_local = fd.get_scope_local_mut().ok_or_else(|| {
            IfaceError::execution("Function has no local scope (no stack space)")
        })?;
        scope_local
            .add_equate_symbol("", format, value, &addr, hash, base1)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMapunionfacet`: `map unionfacet <union> <field> <addr> <hash>`.
    IfcMapunionfacet,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++ ifacedecomp.cc:774-799.
        use kuna_decomp::dtype::type_metatype;
        use kuna_decomp::varnode::varnode_flags;
        {
            let dcp = dcp_mut(status)?;
            if dcp.fd.is_none() {
                return Err(IfaceError::execution("No function loaded"));
            }
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        s.skip_ws();
        let union_name = s.read_token();
        let ct = prog
            .arch()
            .types()
            .find_by_name(&union_name)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?
            .filter(|t| t.get_metatype() == type_metatype::TYPE_UNION)
            .ok_or_else(|| IfaceError::parse(format!("Bad union data-type: {union_name}")))?;
        s.skip_ws();
        let field_num = s.read_int();
        if field_num < -1 || field_num >= ct.num_depend() {
            return Err(IfaceError::parse("Bad field index"));
        }
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        let hash = parse_hex_u64(s).map_err(IfaceError::parse)?;
        // C++ builds the symbol name "unionfacet<n>_<hexoff>".
        let sym_name = format!("unionfacet{}_{:x}", field_num + 1, addr.get_offset());
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        let scope_local = fd.get_scope_local_mut().ok_or_else(|| {
            IfaceError::execution("Function has no local scope (no stack space)")
        })?;
        let sym = scope_local
            .add_union_facet_symbol(&sym_name, ct, field_num, &addr, hash)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        scope_local.set_attribute(sym, varnode_flags::typelock | varnode_flags::namelock);
        Ok(())
    }
);

// --- disassemble [addr1 addr2] (ifacedecomp.cc:806) ------------------------

/// C++ `IfaceAssemblyEmit` (`ifacedecomp.hh:71`): the [`AssemblyEmit`] sink that
/// `IfcPrintdisasm` feeds, formatting one `<addr>: <mnem><pad><body>` line per
/// instruction into the bulk-output buffer.
///
/// The mnemonic is left-justified to a fixed field width (C++ `mnemonicpad=10`)
/// before the operand body.  The one deliberate deviation from the verbatim C++
/// (which pads unconditionally) is that the pad is suppressed for an empty body:
/// the kuna `<stringmatch>` matcher uses the `regex` crate whose `$` rejects a
/// line with trailing whitespace, so an operand-less instruction (e.g. `ARHL`)
/// must render with no trailing spaces.
struct IfaceAssemblyEmit<'a> {
    out: &'a mut String,
}

impl kuna_sleigh::translate::AssemblyEmit for IfaceAssemblyEmit<'_> {
    fn dump(&mut self, addr: &kuna_base::address::Address, mnem: &str, body: &str) {
        // C++ `addr.printRaw(*s)` -> "0x...." (hex width = 2 * address size).
        let _ = addr.print_raw(self.out);
        self.out.push_str(": ");
        self.out.push_str(mnem);
        if !body.is_empty() {
            let mut w = mnem.chars().count();
            while w < 10 {
                self.out.push(' ');
                w += 1;
            }
            self.out.push_str(body);
        }
        self.out.push('\n');
    }
}

decomp_command!(
    /// C++ `IfcPrintdisasm`: disassemble a range (or the current function if no
    /// addresses are given).
    IfcPrintdisasm,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        s.skip_ws();
        // Collect the listing into a buffer while holding the immutable program
        // borrow, then write it to the bulk-output stream (the redirected
        // `fileoptr` the datatest matcher reads — NOT the discarded `optr`).
        let mut out = String::new();
        {
            let dcp = dcp_mut(status)?;
            // C++: no args -> the current function's whole body; else two machine
            // addresses delimiting the range.
            let (mut addr, mut size): (kuna_base::address::Address, int4) = if s.eof() {
                match &dcp.fd {
                    None => return Err(IfaceError::execution("No function selected")),
                    Some(fd) => {
                        out.push_str(&format!("Assembly listing for {}\n", fd.get_name()));
                        (fd.get_address().clone(), fd.get_size())
                    }
                }
            } else {
                let prog = match &dcp.conf {
                    None => return Err(IfaceError::execution("No load image present")),
                    Some(p) => p,
                };
                let (a1, _) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
                s.skip_ws();
                let (a2, _) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
                let sz = a2.get_offset().wrapping_sub(a1.get_offset()) as int4;
                (a1, sz)
            };
            let prog = dcp
                .conf
                .as_ref()
                .ok_or_else(|| IfaceError::execution("No load image present"))?;
            let trans = prog.arch().translate();
            let mut emit = IfaceAssemblyEmit { out: &mut out };
            // C++ loop: print one instruction, advance by its length, until the
            // requested byte count is exhausted.
            while size > 0 {
                let consumed = trans
                    .print_assembly(&mut emit, &addr)
                    .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
                if consumed <= 0 {
                    break;
                }
                addr = &addr + i64::from(consumed);
                size -= consumed;
            }
        }
        status.file_out(&out);
        Ok(())
    }
);

// --- dump / binary (ifacedecomp.cc:843, 860) -------------------------------

decomp_command!(
    /// C++ `IfcDump`: hex-dump a memory range (`dump <addr+size>`).
    IfcDump,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("parse_machaddr + LoadImage::load"))
    }
);

decomp_command!(
    /// C++ `IfcDumpbinary`: dump bytes to a file (`binary <addr+size>
    /// <filename>`).
    IfcDumpbinary,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("parse_machaddr + LoadImage::load (binary dump)"))
    }
);

// --- decompile (ifacedecomp.cc:889) ----------------------------------------

decomp_command!(
    /// C++ `IfcDecompile`: decompile the current function.
    ///
    /// The "No function selected" guard, the "No code for <name>" early return,
    /// the "Clearing old decompilation" notice, and the "Decompiling <name>"
    /// line are ported faithfully; the `allacts.getCurrent()->reset/perform`
    /// drive needs the unported `Architecture::allacts` integration (the
    /// per-function action wiring), so the trailing
    /// "Decompilation complete"/"Break at ..." text is produced by the engine
    /// drive once it lands.
    IfcDecompile,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let command_seq = status.command_seq;
        // Read the per-function values + take the program out so the engine work
        // borrows neither `status` nor `dcp` while the console output is written.
        let (name, has_no_code, proc_started, entry, size, mut mapped_symbols, usepoint_symbols, dynamic_symbols, pending_proto, all_pending_protos, mut prog) = {
            let dcp = dcp_mut(status)?;
            let (name, has_no_code, proc_started, entry, size, mapped_symbols, usepoint_symbols, dynamic_symbols) = match &dcp.fd {
                None => return Err(IfaceError::execution("No function selected")),
                Some(fd) => (
                    fd.get_name().to_string(),
                    fd.has_no_code(),
                    fd.is_proc_started(),
                    fd.get_address().clone(),
                    fd.get_size(),
                    // The console-mapped `map addr` symbols (carried across the
                    // IR rebuild below, which discards the current Funcdata).
                    fd.mapped_symbol_specs(),
                    // The console-added usepoint-scoped `type varnode %REG(pc)`
                    // symbols (e.g. retstruct's `tmp`), carried WITH their use
                    // address so `linkSymbol`'s usepoint query still binds them.
                    fd.usepoint_symbol_specs(),
                    // The console-added `map hash` dynamic symbols (likewise carried
                    // across so `ActionDynamicSymbols` can name the matched temps).
                    fd.dynamic_symbol_specs(),
                ),
            };
            // The `parse line extern <decl>` prototype stashed for this function
            // (C++ Architecture::setPrototype applies it to the queried Funcdata;
            // here the IR is rebuilt on `decompile`, so it is re-applied below).
            let pending_proto = dcp.pending_prototypes.get(&name).cloned();
            // All parsed prototypes (callees included).  C++ `setPrototype` locks each
            // declared function's `FuncProto` on its (lazily-built) Funcdata; here every
            // declared prototype is re-parked on its global FunctionSymbol below so a
            // caller's `ActionDefaultParams` can `fc->copy(otherfunc->getFuncProto())`.
            let all_pending_protos: Vec<kuna_decomp::fspec::PrototypePieces> =
                dcp.pending_prototypes.values().cloned().collect();
            match dcp.conf.take() {
                None => return Err(IfaceError::execution("No load image present")),
                Some(prog) => {
                    (name, has_no_code, proc_started, entry, size, mapped_symbols, usepoint_symbols, dynamic_symbols, pending_proto, all_pending_protos, prog)
                }
            }
        };
        // (kuna DWARF subtask 3) Append the DWARF stack LOCALS parked on this
        // function (by entry VMA) to the `mapped_symbols` re-seeded into the rebuilt
        // `Funcdata`'s `ScopeLocal` — each as a `typelock|namelock` stack symbol via
        // the same `seed_mapped_symbols` path a hand-typed `map addr` uses. So a
        // `-g` binary's `FILE *file` renders by its DWARF name+type instead of
        // `local_18`. Empty (no-op) for a function with no DWARF locals or the XML
        // datatest path (which parks none). A function-local `map addr` the user set
        // by hand still wins: `seed_mapped_symbols` skips an address already mapped
        // (`add_symbol`'s overlap arm), and the hand-mapped symbols come first.
        mapped_symbols.extend(prog.dwarf_locals_for(entry.get_offset()));
        // Re-park every parsed callee prototype on its global FunctionSymbol so the
        // pipeline's `ActionDefaultParams` copies the locked callee proto into the
        // call site (C++ `coreaction.cc:2385` `fc->copy(otherfunc->getFuncProto())`).
        // Functions without a declared prototype (or whose symbol is absent) are
        // unaffected — the default-model recovery still applies there.
        let all_pending_protos_empty = all_pending_protos.is_empty();
        for pieces in all_pending_protos {
            prog.arch_mut().set_function_prototype_pieces(&pieces.name.clone(), pieces);
        }
        // The `override flow` facts stashed for this function (re-seeded on the
        // rebuilt IR, like `pending_proto`/`mapped_symbols`).
        let mut flow_overrides = dcp_mut(status)?
            .pending_flow_overrides
            .get(&name)
            .cloned()
            .unwrap_or_default();
        // (kuna, Ghidra-gap) Apply the analysis's `call error(nonzero,…)` no-return facts
        // as CALL_RETURN flow overrides on the rebuilt IR — the SAME prune `decompile-all`
        // does — so the single-function console decompile does NOT overrun a no-returning
        // `call error` into the following function. `error_noreturn_callsites` is empty
        // unless the Listing + noreturn_error are on, so a listing-less session (datatest
        // path) is byte-identical. Only sites this function's flow visits are applied.
        if let Some(space) = entry.get_space() {
            for &off in &prog.arch().error_noreturn_callsites {
                flow_overrides.push((
                    kuna_base::address::Address::new(std::rc::Rc::clone(space), off),
                    kuna_decomp::overrides::flow_type::CALL_RETURN,
                ));
            }
        }
        // The `map param <i> <addr> <decl>` storage locks stashed for this
        // function (re-seeded on the rebuilt IR, like `pending_proto`).
        let mapped_params = dcp_mut(status)?
            .pending_param_maps
            .get(&name)
            .cloned()
            .unwrap_or_default();
        // The `override prototype` facts stashed for this function (re-seeded on the
        // rebuilt IR), consumed at flow time as `Override::applyPrototype`.  The
        // shared decompile step may discover further per-call-site printf/scanf
        // overrides on top of these; those come back as `step.discovered`.
        let proto_overrides = dcp_mut(status)?
            .pending_proto_overrides
            .get(&name)
            .cloned()
            .unwrap_or_default();
        if has_no_code {
            dcp_mut(status)?.conf = Some(prog);
            status.out(&format!("No code for {name}\n"));
            return Ok(());
        }
        if proc_started {
            status.out("Clearing old decompilation\n");
            // C++: dcp->conf->clearAnalysis(dcp->fd).  The kuna decompile drive
            // rebuilds the Funcdata from scratch below, so the prior IR is
            // discarded the same way (no per-Funcdata clearAnalysis surface yet).
        }
        status.out(&format!("Decompiling {name}\n"));
        // The entry this drive targets, kept for the failure arm below (`entry`
        // itself is consumed by the shared decompile step).
        let stamp_addr = entry.clone();
        // C++: allacts.getCurrent()->reset(*fd); res = perform(*fd); then the
        // "Decompilation complete"/"Break at .." reporting.  The kuna decompile
        // drive (decompile_drive::decompile_func) installs the `decompile` root,
        // resets it, and runs the 252-pass perform loop to completion.
        //
        // (kuna) Adopt the IR `load function` / `load addr` already followed, when
        // this `decompile` is provably the same flow follow repeated.  Upstream
        // never repeats it (`IfcDecompile` re-runs the actions on `IfcFuncload`'s
        // Funcdata); kuna rebuilds because the seeds below are consumed at flow
        // time and the load applies none of them — so the rebuild is required
        // exactly when some seed is non-empty, and pure waste when none is.  Every
        // seed the drive would apply before/at flow time is checked, plus the
        // command counter (see `PristineFlow`), so anything at all between the two
        // commands falls back to the rebuild.
        let prefollowed = {
            let dcp = dcp_mut(status)?;
            let seeds_empty = mapped_symbols.is_empty()
                && usepoint_symbols.is_empty()
                && dynamic_symbols.is_empty()
                && pending_proto.is_none()
                && all_pending_protos_empty
                && mapped_params.is_empty()
                && proto_overrides.is_empty();
            let stamp_matches = matches!(&dcp.pristine_flow, Some(p)
                if p.command_seq + 1 == command_seq
                    && p.name == name
                    && p.entry == entry
                    && p.size == size
                    && p.flow_overrides == flow_overrides);
            // The IR must also have been followed under the SAME architecture
            // configuration the drive will run with, because a `Funcdata` snapshots
            // the per-function flags into its ArchSeam handle when it is BUILT (see
            // `docs/spec/00-overview.md` §0.5) -- a flag flipped afterwards is
            // invisible to it.  Three things move between the load and the drive:
            //
            //  * `formatstring` turns read-only propagation on around the drive so
            //    the printf format constant can be READ (on ARM it is a PC-relative
            //    literal-pool load that only folds with `fillin_read_only`).  The
            //    loaded IR snapshotted the flag OFF, so adopting it renders
            //    `printf((char *)(dat_52c + 0x51c), ...)` -- the format string never
            //    resolves and the varargs never get typed.
            //  * the watchdog arms its per-function deadline inside the drive
            //    (`kuna_fn_budget` is `None` on every console path).
            //  * ghidra mode stages name/dynamic/prototype-model recommendations that
            //    the drive takes; a follow that ran while they were still parked is
            //    not the same follow.
            let same_config = !prog.arch().analysis_formatstring
                && prog.arch().kuna_fn_budget.is_none()
                && prog.arch().kuna_pending_name_recs.is_empty()
                && prog.arch().kuna_pending_dyn_recs.is_empty()
                && prog.arch().kuna_pending_proto_model.is_none();
            if seeds_empty && stamp_matches && same_config {
                dcp.adopted_flows += 1;
                dcp.fd.take()
            } else {
                None
            }
        };
        // The recipe that rebuilds what was just handed over, for the failure arm
        // below.  A drive that aborts (the GH-6904 LOSS-131 stub path) leaves the
        // console expected to still hold an un-decompiled `dcp.fd` for a following
        // `print C`; adopting the IR moves it into the drive, which consumes it, so
        // the error arm re-follows the SAME name/entry/size/overrides `load
        // function` used.  Only reached when a decompile fails, i.e. never on a
        // healthy run and never on the parity corpora.
        let adopted_recipe = prefollowed.as_ref().and_then(|_| {
            dcp_mut(status).ok().and_then(|dcp| dcp.pristine_flow.take())
        });
        dcp_mut(status)?.pristine_flow = None;
        // (DIV-66) Routed through the SHARED step so this console command and the
        // whole-binary loop (`project::decompile_targets`) run the identical
        // pipeline — including the FormatStringAnalyzer half-B override /
        // re-decompile loop, which used to live only here.
        let step = crate::decompile_step::decompile_one_prefollowed(
            prog.arch_mut(),
            &name,
            entry,
            size,
            &crate::decompile_step::DecompileSeed {
                mapped_symbols: &mapped_symbols,
                usepoint_symbols: &usepoint_symbols,
                dynamic_symbols: &dynamic_symbols,
                pending_proto: pending_proto.as_ref(),
                flow_overrides: &flow_overrides,
                mapped_params: &mapped_params,
            },
            &proto_overrides,
            prefollowed,
        );
        let result = step.result;
        if !step.discovered.is_empty() {
            status.out("Re-decompiling with format-string varargs typing\n");
        }
        // Persist the discovered call-site overrides for any later re-decompile of
        // this function (the `pending_proto_overrides` precedent).
        for (callpoint, pieces) in step.discovered {
            dcp_mut(status)?
                .pending_proto_overrides
                .entry(name.clone())
                .or_default()
                .push((callpoint, pieces));
        }
        // Restore the program (and the fresh Funcdata on success) regardless.
        if result.is_err() {
            // Re-follow the adopted IR so the console is left holding the same
            // un-decompiled `dcp.fd` the rebuild path would have left (see
            // `adopted_recipe`).  `None` whenever nothing was adopted.
            if let Some(recipe) = &adopted_recipe {
                if let Ok(fd) = build_and_follow_flow_with_override(
                    prog.arch_mut(),
                    &recipe.name,
                    recipe.entry.clone(),
                    recipe.size,
                    &recipe.flow_overrides,
                ) {
                    dcp_mut(status)?.fd = Some(fd);
                }
            }
        }
        let dcp = dcp_mut(status)?;
        dcp.conf = Some(prog);
        match result {
            Ok(fd) => {
                // (kuna `--assert`) Every flow override the follower REFUSED, named
                // in the console's own command spelling so a front-end driving
                // `decomp_dbg` can pair each line with the `override flow` it sent.
                // The refusal is reported here and not at `override flow` time
                // because nothing can be applied until flow has followed.
                let refusals: Vec<String> = fd
                    .kuna_rejected_flow_overrides()
                    .iter()
                    .map(|(addr, type_, reason)| {
                        format!(
                            "Rejected override flow {:#x} {}: {reason}\n",
                            addr.get_offset(),
                            kuna_decomp::overrides::Override::type_to_string(*type_)
                        )
                    })
                    .collect();
                dcp.fd = Some(fd);
                for line in &refusals {
                    status.out(line);
                }
                // C++ res>=0 path: "Decompilation complete".
                status.out("Decompilation complete\n");
                Ok(())
            }
            Err(e) => {
                let msg = e.explain().to_string();
                // (kuna GH-6904) A *recoverable* per-function abort — the pipeline
                // hit a documented un-ported stub (LOSS-131) and unwound, discarding
                // this function's half-built IR — degrades gracefully instead of
                // poisoning the whole console session.  This mirrors the C++
                // `IfcProduceC::iterationCallback` catch (`ifacedecomp.cc:2402`):
                // print "Skipping <name>: <err>" and continue, so a subsequent
                // `print C` still renders (the prior, un-decompiled `dcp.fd`) and a
                // datatest's `<stringmatch>` rules are evaluated rather than the
                // whole file being marked an execution error.  Only the LOSS-131
                // stub-abort is swallowed; a genuine fatal `IfaceExecutionError`
                // (e.g. "No function selected") still propagates.  The whole-corpus
                // (675/675) is inert here: no datatest function aborts, so this arm
                // is never reached for them.
                // Stamp the reason on the retained (un-decompiled) Funcdata: it
                // has no structured blocks, so a following `print C` would
                // otherwise emit the generic "structuring declined" shell and hide
                // the abort.  Guarded on the address so a stale, unrelated function
                // is never mislabeled.  Stamped for EVERY per-function abort, not
                // only the swallowed one: a `decompile` that raised leaves the same
                // shell behind, and a front-end that reads only the C (`kuna
                // decompile`'s text surface) cannot otherwise tell it from a
                // genuinely empty function.
                if let Some(fd) = dcp.fd.as_mut() {
                    if fd.get_address() == &stamp_addr {
                        fd.set_kuna_pipeline_failure(&msg);
                    }
                }
                if msg.contains("LOSS-131") {
                    status.out(&format!("Skipping {name}: {msg}\n"));
                    Ok(())
                } else {
                    Err(IfaceError::execution(msg))
                }
            }
        }
    }
);

// --- print C ... (ifacedecomp.cc:923-987) ----------------------------------

decomp_command!(
    /// C++ `IfcPrintCFlat`: `print C flat`.
    IfcPrintCFlat,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("PrintLanguage::docFunction (flat) (Architecture::print)"))
    }
);

decomp_command!(
    /// C++ `IfcPrintCGlobals`: `print C globals`.
    IfcPrintCGlobals,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("PrintLanguage::docAllGlobals (Architecture::print)"))
    }
);

decomp_command!(
    /// C++ `IfcPrintCTypes`: `print C types` — render every user-defined
    /// data-type as C definitions (`PrintLanguage::docTypeDefinitions`,
    /// `ifacedecomp.cc:960`).  Output goes to the bulk stream (`fileoptr`),
    /// mirroring `print C` (`IfcPrintCStruct` below): render with `dcp`
    /// borrowed, then write to the status.  Returns nothing (an empty string)
    /// when no user types are interned — empty is not an error.
    IfcPrintCTypes,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let text = {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
            let prog = dcp.conf.as_mut().expect("conf checked non-None above");
            print_c_types(prog.arch_mut())
        };
        status.file_out(&text);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcPrintCXml`: `print C xml`.
    IfcPrintCXml,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("PrintLanguage::docFunction (xml markup) (Architecture::print)"))
    }
);

decomp_command!(
    /// C++ `IfcPrintCStruct`: `print C` — the headline command of the datatests
    /// (231 `<com>print C</com>` uses).
    IfcPrintCStruct,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        // The kuna print drive (decompile_drive::print_c) renders the function
        // through the owned PrintC.  Output goes to the bulk stream (fileoptr),
        // which the Python tools capture via `openfile write`.  Render the C with
        // `dcp` borrowed, then drop the borrow before writing to the status.
        let c = {
            let dcp = dcp_mut(status)?;
            if dcp.fd.is_none() {
                return Err(IfaceError::execution("No function selected"));
            }
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
            let fd = dcp.fd.take().expect("fd checked non-None above");
            let text = {
                let prog = dcp.conf.as_mut().expect("conf checked non-None above");
                print_c(prog.arch_mut(), &fd)
            };
            dcp.fd = Some(fd);
            text
        };
        status.file_out(&c);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcPrintLanguage`: `print language <langname>`.
    IfcPrintLanguage,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("No print language specified"));
        }
        Err(engine_unavailable("Architecture::setPrintLanguage + docFunction"))
    }
);

// --- print raw (ifacedecomp.cc:1018) ---------------------------------------

decomp_command!(
    /// C++ `IfcPrintRaw`: `print raw` — dump the function's raw p-code.
    IfcPrintRaw,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        // C++: dcp->fd->printRaw(*status->fileoptr).  Render the SSA listing
        // against the real Architecture (register-name resolution + default
        // size) and write it to the bulk stream, mirroring `print C`.
        let raw = {
            let dcp = dcp_mut(status)?;
            if dcp.fd.is_none() {
                return Err(IfaceError::execution("No function selected"));
            }
            let fd = dcp.fd.take().expect("fd checked non-None above");
            let res = {
                let prog = dcp
                    .conf
                    .as_ref()
                    .ok_or_else(|| IfaceError::execution("No load image present"))?;
                kuna_decomp::funcdata_printraw::print_raw(prog.arch(), &fd)
            };
            dcp.fd = Some(fd);
            res.map_err(IfaceError::execution)?
        };
        status.file_out(&raw);
        Ok(())
    }
);

// --- list action / override / prototypes (ifacedecomp.cc:1029-1079) --------

decomp_command!(
    /// C++ `IfcListaction`: `list action`.
    IfcListaction,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Decompile action not loaded"));
        }
        Err(engine_unavailable("ActionDatabase::getCurrent()->print"))
    }
);

decomp_command!(
    /// C++ `IfcListOverride`: `list override`.
    IfcListOverride,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        let name = match &dcp.fd {
            None => return Err(IfaceError::execution("No function selected")),
            Some(fd) => fd.get_name().to_string(),
        };
        status.out(&format!("Function: {name}\n"));
        Err(engine_unavailable("Override::printRaw"))
    }
);

decomp_command!(
    /// C++ `IfcListprototypes`: `list prototypes`.
    IfcListprototypes,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("Architecture::protoModels (prototype list)"))
    }
);

// --- set context / set track (ifacedecomp.cc:1087, 1131) -------------------

decomp_command!(
    /// C++ `IfcSetcontextrange`: `set context <name> <value> [start end]`.
    IfcSetcontextrange,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        let name = s.read_token();
        s.skip_ws();
        if name.is_empty() {
            return Err(IfaceError::parse("Missing context variable name"));
        }
        // C++: s.unsetf(...); uintm value=0xbadbeef; s>>value (user base);
        //      "Missing context value" if unchanged.
        let valtok = s.read_token();
        let value = match parse_userbase_u32(&valtok) {
            Some(v) => v,
            None => return Err(IfaceError::parse("Missing context value")),
        };
        s.skip_ws();
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        if s.eof() {
            // No range indicates a default value: context->setVariableDefault.
            prog.arch().with_context_db_mut(|db| db.set_variable_default(name.as_bytes(), value))
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            return Ok(());
        }
        // Otherwise parse the [begin,end) range.
        let (addr1, _s1) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        let (addr2, _s2) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        if addr1.is_invalid() || addr2.is_invalid() {
            return Err(IfaceError::parse("Invalid address range"));
        }
        if addr2 <= addr1 {
            return Err(IfaceError::parse("Bad address range"));
        }
        prog.arch()
            .with_context_db_mut(|db| db.set_variable_region(name.as_bytes(), &addr1, &addr2, value))
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcSettrackedrange`: `set track <name> <value> [start end]`.
    IfcSettrackedrange,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        let name = s.read_token();
        s.skip_ws();
        if name.is_empty() {
            return Err(IfaceError::parse("Missing tracked register name"));
        }
        // C++: s.unsetf(...); uintb value=0xbadbeef; s>>value (user base).
        let valtok = s.read_token();
        let value = match parse_userbase_u64(&valtok) {
            Some(v) => v,
            None => return Err(IfaceError::parse("Missing context value")),
        };
        s.skip_ws();
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let loc = prog
            .arch()
            .get_register_varnode(name.as_bytes())
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        if s.eof() {
            // No range: append to the default tracked set.
            prog.arch().with_context_db_mut(|db| {
                let track = db.get_tracked_default();
                track.push(kuna_sleigh::globalcontext::TrackedContext { loc, val: value });
            });
            return Ok(());
        }
        let (addr1, _s1) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        let (addr2, _s2) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        if addr1.is_invalid() || addr2.is_invalid() {
            return Err(IfaceError::parse("Invalid address range"));
        }
        if addr2 <= addr1 {
            return Err(IfaceError::parse("Bad address range"));
        }
        prog.arch().with_context_db_mut(|db| {
            // C++ createSet(addr1,addr2); track = def (copy default as base); push.
            let def = db.get_tracked_default().clone();
            let track = db.create_set(&addr1, &addr2);
            *track = def;
            track.push(kuna_sleigh::globalcontext::TrackedContext { loc, val: value });
        });
        Ok(())
    }
);

// --- break action / break start (ifacedecomp.cc:1182, 1208) ----------------

decomp_command!(
    /// C++ `IfcBreakaction`: `break action <actionname>`.
    IfcBreakaction,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let specify = s.read_token();
        s.skip_ws();
        let dcp = dcp_mut(status)?;
        if specify.is_empty() {
            return Err(IfaceError::execution("No action/rule specified"));
        }
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Decompile action not loaded"));
        }
        Err(engine_unavailable("ActionDatabase::getCurrent()->setBreakPoint(break_action)"))
    }
);

decomp_command!(
    /// C++ `IfcBreakstart`: `break start <actionname>`.
    IfcBreakstart,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let specify = s.read_token();
        s.skip_ws();
        let dcp = dcp_mut(status)?;
        if specify.is_empty() {
            return Err(IfaceError::execution("No action/rule specified"));
        }
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Decompile action not loaded"));
        }
        Err(engine_unavailable("ActionDatabase::getCurrent()->setBreakPoint(break_start)"))
    }
);

// --- print tree varnode / block (ifacedecomp.cc:1231, 1245) ----------------

decomp_command!(
    /// C++ `IfcPrintTree`: `print tree varnode`.
    IfcPrintTree,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata::printVarnodeTree"))
    }
);

decomp_command!(
    /// C++ `IfcPrintBlocktree`: `print tree block`.
    IfcPrintBlocktree,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata::printBlockTree"))
    }
);

// --- print spaces (ifacedecomp.cc:1259) ------------------------------------

decomp_command!(
    /// C++ `IfcPrintSpaces`: `print spaces`.
    IfcPrintSpaces,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("AddrSpaceManager::getSpace (space listing)"))
    }
);

// --- print high (ifacedecomp.cc:1296) --------------------------------------

decomp_command!(
    /// C++ `IfcPrintHigh`: `print high <name>`.
    IfcPrintHigh,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata::findHigh + HighVariable::printInfo"))
    }
);

// --- print parammeasures (ifacedecomp.cc:1316) -----------------------------

decomp_command!(
    /// C++ `IfcPrintParamMeasures`: `print parammeasures`.
    IfcPrintParamMeasures,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("ParamIDAnalysis::savePretty (paramid.cc)"))
    }
);

// --- rename / remove / retype / isolate (ifacedecomp.cc:1332-1443) ---------

decomp_command!(
    /// C++ `IfcRename`: `rename <oldname> <newname>` (ifacedecomp.cc:1332).
    IfcRename,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        use kuna_decomp::database::symbol_category;
        use kuna_decomp::varnode::varnode_flags;
        s.skip_ws();
        let oldname = s.read_token();
        s.skip_ws();
        let newname = s.read_token();
        s.skip_ws();
        if oldname.is_empty() {
            return Err(IfaceError::parse("Missing old symbol name"));
        }
        if newname.is_empty() {
            return Err(IfaceError::parse("Missing new name"));
        }
        let dcp = dcp_mut(status)?;
        let sym_list = dcp.read_symbol(&oldname)?;
        if sym_list.is_empty() {
            return Err(IfaceError::execution(format!("No symbol named: {oldname}")));
        }
        if sym_list.len() > 1 {
            return Err(IfaceError::execution(format!("More than one symbol named: {oldname}")));
        }
        let sym = sym_list[0];
        let fd = dcp.fd.as_mut().expect("read_symbol succeeded => fd present");
        let lm = fd
            .get_scope_local_mut()
            .ok_or_else(|| IfaceError::execution("Function has no local scope"))?;
        // C++: if (sym->getCategory() == function_parameter)
        //        dcp->fd->getFuncProto().setInputLock(true);
        if lm.symbol_category(sym) == symbol_category::FUNCTION_PARAMETER {
            fd.get_func_proto_mut().set_input_lock(true);
            let lm = fd.get_scope_local_mut().expect("local scope present");
            lm.rename_symbol(sym, &newname)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
        } else {
            lm.rename_symbol(sym, &newname)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
        }
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcRemove`: `remove <symbolname>`.
    IfcRemove,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        s.skip_ws();
        let name = s.read_token();
        if name.is_empty() {
            return Err(IfaceError::parse("Missing symbol name"));
        }
        Err(engine_unavailable("IfaceDecompData::readSymbol + Scope::removeSymbol"))
    }
);

decomp_command!(
    /// C++ `IfcRetype`: `retype <symbolname> <typedeclaration>`
    /// (ifacedecomp.cc:1390).  Change the data-type (and optionally the name) of
    /// a symbol resolved by name in the current function's scope.
    IfcRetype,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        use kuna_decomp::database::symbol_category;
        use kuna_decomp::varnode::varnode_flags;
        s.skip_ws();
        let name = s.read_token();
        if name.is_empty() {
            return Err(IfaceError::parse("Must specify name of symbol"));
        }
        // C++: ct = parse_type(s,newname,dcp->conf).
        let (ct, newname) = {
            let dcp = dcp_mut(status)?;
            let prog = dcp
                .conf
                .as_ref()
                .ok_or_else(|| IfaceError::execution("No load image present"))?;
            s.skip_ws();
            let (addr_size, word_size) = prog.arch().data_org();
            let org = crate::grammar::DataOrg { addr_size, word_size };
            let typetext = s.rest();
            crate::grammar::parse_type(&typetext, prog.arch().types(), org)
                .map_err(|e| IfaceError::parse(e.explain().to_string()))?
        };
        let dcp = dcp_mut(status)?;
        let sym_list = dcp.read_symbol(&name)?;
        if sym_list.is_empty() {
            return Err(IfaceError::execution(format!("No symbol named: {name}")));
        }
        if sym_list.len() > 1 {
            return Err(IfaceError::execution(format!("More than one symbol named : {name}")));
        }
        let sym = sym_list[0];
        let fd = dcp.fd.as_mut().expect("read_symbol succeeded => fd present");
        // C++: if (sym->getCategory()==function_parameter)
        //        dcp->fd->getFuncProto().setInputLock(true);
        let is_param = fd
            .get_scope_local()
            .map(|lm| lm.symbol_category(sym) == symbol_category::FUNCTION_PARAMETER)
            .unwrap_or(false);
        if is_param {
            fd.get_func_proto_mut().set_input_lock(true);
        }
        let lm = fd
            .get_scope_local_mut()
            .ok_or_else(|| IfaceError::execution("Function has no local scope"))?;
        lm.retype_symbol(sym, ct).map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        lm.set_attribute(sym, varnode_flags::typelock);
        if !newname.is_empty() && newname != name {
            lm.rename_symbol(sym, &newname)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            lm.set_attribute(sym, varnode_flags::namelock);
        }
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcIsolate`: `isolate <name>`.
    IfcIsolate,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        s.skip_ws();
        let symbol_name = s.read_token();
        if symbol_name.is_empty() {
            return Err(IfaceError::parse("Missing symbol name"));
        }
        Err(engine_unavailable("IfaceDecompData::readSymbol + Symbol::setIsolated"))
    }
);

// --- print varnode / cover ... (ifacedecomp.cc:1540-1693) ------------------

decomp_command!(
    /// C++ `IfcPrintVarnode`: `print varnode <varnode>`.
    IfcPrintVarnode,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        // C++ reads the varnode (which throws "No function selected" if fd==0).
        Err(engine_unavailable("IfaceDecompData::readVarnode (parse_varnode)"))
    }
);

decomp_command!(
    /// C++ `IfcPrintCover`: `print cover high <name>`.
    IfcPrintCover,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("HighVariable::printCover"))
    }
);

decomp_command!(
    /// C++ `IfcVarnodehighCover`: `print cover varnodehigh <varnode>`.
    IfcVarnodehighCover,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("IfaceDecompData::readVarnode + HighVariable cover"))
    }
);

decomp_command!(
    /// C++ `IfcVarnodeCover`: `print cover varnode <varnode>`.
    IfcVarnodeCover,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("IfaceDecompData::readVarnode + Cover::print"))
    }
);

decomp_command!(
    /// C++ `IfcPrintExtrapop`: `print extrapop [<varname>]`.
    IfcPrintExtrapop,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata extrapop reporting"))
    }
);

// --- name varnode / type varnode (ifacedecomp.cc:1695, 1734) ---------------

decomp_command!(
    /// C++ `IfcNameVarnode`: `name varnode <varnode> <name>`.
    IfcNameVarnode,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("IfaceDecompData::readVarnode + Funcdata::nameRecommend"))
    }
);

decomp_command!(
    /// C++ `IfcTypeVarnode`: `type varnode <varnode> <typedeclaration>`
    /// (ifacedecomp.cc:1734-1762).  Type-lock a specific varnode's storage to a
    /// data-type by adding an isolated, type-locked Symbol to the function's
    /// local scope.
    IfcTypeVarnode,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        use kuna_decomp::varnode::varnode_flags;
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_mut()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        // C++: Address loc(parse_varnode(s,size,pc,uq,*dcp->conf->types)).
        let (loc, size, pc, _uq) = parse_varnode(prog, s).map_err(IfaceError::parse)?;
        s.skip_ws();
        // C++: ct = parse_type(s,name,dcp->conf).
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let typetext = s.rest();
        let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;

        // C++: dcp->conf->clearAnalysis(dcp->fd) — clear analysis so the varnode
        // assignment takes effect on the next decompile (Funcdata::clear).
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        fd.clear();

        // The W4 scope hierarchy is not exposed across this boundary, so a varnode
        // with no natural sub-scope binds straight to the function-local scope
        // (the C++ fallback arm, which is taken for register storage like %EAX).
        let _ = (size, &pc);
        let scope_local = fd.get_scope_local_mut().ok_or_else(|| {
            IfaceError::execution("Function has no local scope (no stack space)")
        })?;
        let sym = scope_local
            .add_symbol(&name, ct, &loc, &pc)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        scope_local.set_attribute(sym, varnode_flags::typelock);
        scope_local.set_symbol_isolated(sym, true);
        if !name.is_empty() {
            scope_local.set_attribute(sym, varnode_flags::namelock);
        }
        let scope_name = scope_local.full_name();
        let sym_name = name; // the console echoes sym->getName()
        // C++ writes to status->fileoptr (the bulk output stream).
        status.file_out(&format!(
            "Successfully added {sym_name} to scope {scope_name}\n"
        ));
        Ok(())
    }
);

// --- force varnode / datatype / goto (ifacedecomp.cc:1769-1831) ------------

decomp_command!(
    /// C++ `IfcForceFormat`: `force varnode <varnode> <format>` (ifacedecomp.cc:
    /// 1769-1788).  Mark a constant Varnode in the current function so it prints in
    /// one of hex/dec/oct/bin/char: read the varnode, build a dynamic (equate)
    /// Symbol over it, then force the integer display format and type-lock it.
    IfcForceFormat,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        use kuna_decomp::dtype::type_metatype;
        use kuna_decomp::varnode::varnode_flags;
        // C++ readVarnode begins with "if (fd==0) throw No function selected".
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_ref()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        // C++: Varnode *vn = dcp->readVarnode(s) — parse + resolve the varnode.
        let (loc, defsize, pc, uq) = parse_varnode(prog, s).map_err(IfaceError::parse)?;
        // The DynamicHash collision budget (glb->dynamic_hash_maxdup_high) and the
        // EquateSymbol's getBase(1,TYPE_UNKNOWN) type are the two merged-tree boundaries
        // build_dynamic_symbol takes as parameters; resolve both from the arch.
        let maxduplicates: u32 = if prog.arch().dynamic_hash_maxdup_high { 16 } else { 8 };
        let base1 = prog
            .arch()
            .types()
            .get_base(1, type_metatype::TYPE_UNKNOWN)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let fd = dcp.fd.as_ref().expect("fd checked Some above");
        let vn = read_varnode(fd, &loc, defsize, &pc, uq)
            .map_err(IfaceError::execution)?
            .ok_or_else(|| IfaceError::execution("Requested varnode does not exist"))?;
        let v = fd
            .vbank()
            .get(vn)
            .ok_or_else(|| IfaceError::execution("Requested varnode does not exist"))?;
        if !v.is_constant() {
            return Err(IfaceError::execution("Can only force format on a constant"));
        }
        let mt = v.get_type().get_metatype();
        if mt != type_metatype::TYPE_INT
            && mt != type_metatype::TYPE_UINT
            && mt != type_metatype::TYPE_UNKNOWN
        {
            return Err(IfaceError::execution(
                "Can only force format on integer type constant",
            ));
        }
        let fd = dcp.fd.as_mut().expect("fd checked Some above");
        fd.build_dynamic_symbol(vn, maxduplicates, base1)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        let sym = fd
            .vn_high_equate_symbol(vn)
            .ok_or_else(|| IfaceError::execution("Unable to create symbol"))?;
        s.skip_ws();
        let format_string = s.read_token();
        let format: kuna_base::types::uint4 = match format_string.as_str() {
            "hex" => 1,
            "dec" => 2,
            "oct" => 3,
            "bin" => 4,
            "char" => 5,
            _ => {
                return Err(IfaceError::execution(format!(
                    "Unrecognized integer format: {format_string}"
                )))
            }
        };
        let scope_local = fd
            .get_scope_local_mut()
            .ok_or_else(|| IfaceError::execution("Function has no local scope"))?;
        scope_local.set_display_format(sym, format);
        scope_local.set_attribute(sym, varnode_flags::typelock);
        status.out("Successfully forced format display\n");
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcForceDatatypeFormat`: `force datatype <datatype> <format>`
    /// (ifacedecomp.cc:1794).  Force the integer display format of a named type.
    IfcForceDatatypeFormat,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        s.skip_ws();
        let type_name = s.read_token();
        s.skip_ws();
        let format_string = s.read_token();
        // C++ `Datatype::encodeIntegerFormat`: hex|dec|oct|bin|char -> 1..5.
        let format: kuna_base::types::uint4 = match format_string.as_str() {
            "hex" => 1,
            "dec" => 2,
            "oct" => 3,
            "bin" => 4,
            "char" => 5,
            _ => {
                return Err(IfaceError::execution(format!(
                    "Unrecognized integer format: {format_string}"
                )))
            }
        };
        let dcp = dcp_mut(status)?;
        let prog = dcp
            .conf
            .as_mut()
            .ok_or_else(|| IfaceError::execution("No load image present"))?;
        let types = prog.arch().types();
        // C++ `dt = dcp->conf->types->findByName(typeName)`.
        let dt = types
            .find_by_name(&type_name)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?
            .ok_or_else(|| IfaceError::execution(format!("Unknown data-type: {type_name}")))?;
        // C++ `dcp->conf->types->setDisplayFormat(dt, format)`.
        types
            .set_display_format(&dt, format)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        status.out("Successfully forced data-type display\n");
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcForcegoto`: `force goto <branchaddr> <targetaddr>`.
    IfcForcegoto,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("parse_machaddr + Override::insertForceGoto"))
    }
);

// --- override prototype / jumptable / flow (ifacedecomp.cc:1840-1953) ------

/// A console-side [`FuncProtoOverride`](kuna_decomp::overrides::FuncProtoOverride)
/// holding the parsed [`PrototypePieces`] for `override prototype <addr> <decl>`.
///
/// C++ wraps the pieces in a full `FuncProto` (`setInternal`/`setPieces`) and
/// stores it in the function's `Override`.  The W4 `applyPrototype` consume
/// (`FlowInfo::queryCall`) is still stubbed (LOSS-031 neighborhood), so this wrapper
/// only needs to round-trip the pieces; `encode`/`print_raw` (debug-only surfaces,
/// not exercised by the datatest corpus) are faithful stubs.
struct PiecesProtoOverride {
    pieces: kuna_decomp::fspec::PrototypePieces,
}

impl kuna_decomp::overrides::FuncProtoOverride for PiecesProtoOverride {
    fn set_override(&mut self, _val: bool) {
        // C++ FuncProto::setOverride sets a flag consumed by the (stubbed)
        // applyPrototype; the pieces carry no such flag, so this is a no-op until
        // the W4 FuncProto-backed override lands.
    }
    fn encode(&self, _encoder: &mut dyn kuna_base::marshal::Encoder) -> kuna_base::error::KunaResult<()> {
        // STUB(W4): FuncProto::encode of an override is a debug/save surface absent
        // from the datatest corpus.
        Err(kuna_base::error::KunaError::lowlevel(
            "kuna rust port: prototype-override encode needs the W4 FuncProto::encode",
        ))
    }
    fn print_raw(&self, s: &mut String) {
        // C++ FuncProto::printRaw uses the literal name "func"; render the pieces'
        // model name + arity for a faithful-enough debug line.
        s.push_str("func(");
        s.push_str(&self.pieces.intypes.len().to_string());
        s.push(')');
    }
    fn pieces(&self) -> Option<&kuna_decomp::fspec::PrototypePieces> {
        Some(&self.pieces)
    }
}

decomp_command!(
    /// C++ `IfcProtooverride`: `override prototype <addr> <declaration>`.
    ///
    /// Parse the call-point address and the prototype declaration, find the call
    /// site at that address, build a prototype override, and install it on the
    /// function's `Override` (C++ `dcp->fd->getOverride().insertProtoOverride`).
    IfcProtooverride,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_ref().expect("conf present when fd present");
        let (callpoint, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        let fd = dcp.fd.as_ref().expect("fd present");
        let mut found = false;
        for i in 0..fd.num_calls() {
            let op = fd.get_call_specs(i).get_op();
            if let Some(o) = fd.obank().get(op) {
                if o.get_addr() == &callpoint {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return Err(IfaceError::execution("No call is made at this address"));
        }
        // C++ parse_protopieces(pieces,s,dcp->conf) — the remainder of the line.
        s.skip_ws();
        let decl = s.rest().trim().to_string();
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let pieces = crate::grammar::parse_protopieces(&decl, prog.arch().types(), org)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // C++ builds a FuncProto (setInternal + setPieces) and
        // insertProtoOverride(callpoint, newproto).  The W4 `applyPrototype` consume
        // (FlowInfo::queryCall) is still stubbed (LOSS-031 neighborhood), so the
        // override is stored but not yet applied at flow time; the command succeeds
        // (the script proceeds) exactly as C++.
        // The kuna console rebuilds the IR on `decompile`, dropping this fd's
        // Override; stash the (callpoint, pieces) by function name so the next
        // `decompile` re-seeds it onto the fresh Funcdata (the `flow`/`proto`
        // override precedent).
        let funcname = dcp.fd.as_ref().expect("fd present").get_display_name().to_string();
        dcp.pending_proto_overrides
            .entry(funcname)
            .or_default()
            .push((callpoint.clone(), pieces.clone()));
        let ov: Box<dyn kuna_decomp::overrides::FuncProtoOverride> =
            Box::new(PiecesProtoOverride { pieces });
        dcp.fd
            .as_mut()
            .expect("fd present")
            .get_override_mut()
            .insert_proto_override(callpoint, ov);
        status.out("Successfully added override\n");
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcJumpOverride`: `override jumptable ...`.
    IfcJumpOverride,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("parse_machaddr + Funcdata::installJumpTable + setOverride"))
    }
);

decomp_command!(
    /// C++ `IfcFlowOverride`: `override flow <addr> branch|call|callreturn|return`.
    IfcFlowOverride,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++: if (dcp->fd==0) throw "No function selected".
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_ref().expect("conf present when fd present");
        // C++ Address addr( parse_machaddr(s,discard,*dcp->conf->types) ).
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        let token = s.read_token();
        if token.is_empty() {
            return Err(IfaceError::parse("Missing override type"));
        }
        // C++ type = Override::stringToType(token); if (type==NONE) "Bad override type".
        let type_ = kuna_decomp::overrides::Override::string_to_type(token.as_bytes());
        if type_ == kuna_decomp::overrides::flow_type::NONE {
            return Err(IfaceError::parse("Bad override type"));
        }
        // C++ dcp->fd->getOverride().insertFlowOverride(addr,type).
        let fname = dcp.fd.as_ref().expect("fd present").get_name().to_string();
        dcp.fd
            .as_mut()
            .expect("fd present")
            .get_override_mut()
            .insert_flow_override(addr.clone(), type_);
        // Stash by function name so the override survives the IR rebuild on
        // `load function`/`decompile` (the kuna console rebuilds the Funcdata).
        dcp.pending_flow_overrides.entry(fname).or_default().push((addr, type_));
        status.out("Successfully added override\n");
        Ok(())
    }
);

// --- deadcode delay (ifacedecomp.cc:1962) ----------------------------------

decomp_command!(
    /// C++ `IfcDeadcodedelay`: `deadcode delay <space> <delay>`.
    IfcDeadcodedelay,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        let _name = s.read_token();
        // C++ resolves the space ("Bad space: <name>") and reads the delay int
        // ("Need delay integer") before applying it.
        Err(engine_unavailable("Architecture::getSpaceByName + setDeadcodeDelay"))
    }
);

// --- global add / remove / spaces / registers (ifacedecomp.cc:1993-2046) ---

/// The `[first, last]` (inclusive) global-scope range a `global add`/`global
/// remove` names, parsed the way C++ `IfcGlobalAdd` does: `parse_machaddr`, then
/// `Range(space, first, first+size-1)`.
fn parse_global_range(
    status: &mut IfaceStatus,
    s: &mut CommandStream,
) -> IfaceResult<(std::rc::Rc<kuna_base::space::AddrSpace>, uintb, uintb)> {
    {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No image loaded"));
        }
    }
    let dcp = dcp_mut(status)?;
    let prog = dcp.conf.as_mut().expect("conf checked non-None above");
    let (addr, size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
    if size <= 0 {
        return Err(IfaceError::execution("Must specify a size"));
    }
    let space = addr
        .get_space()
        .cloned()
        .ok_or_else(|| IfaceError::execution("Invalid address space"))?;
    let first = addr.get_offset();
    Ok((space, first, first.wrapping_add(size as uintb).wrapping_sub(1)))
}

decomp_command!(
    /// C++ `IfcGlobalAdd`: `global add <addr+size>` —
    /// `symboltab->addRange(getGlobalScope(), space, first, last)`.
    ///
    /// Note for a caller reaching for this: on every stock cspec the `<global>`
    /// tag already claims the whole default data space (`<range space="ram"/>`),
    /// so on a normal ELF the range is global before this runs and adding it
    /// changes nothing.  It is the undo of `global remove`, and the lever for a
    /// space the cspec does NOT claim.
    IfcGlobalAdd,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let (space, first, last) = parse_global_range(status, s)?;
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let scope = prog
            .arch()
            .symboltab
            .get_global_scope()
            .ok_or_else(|| IfaceError::execution("No global scope"))?;
        prog.arch_mut().symboltab.add_range(scope, space, first, last);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcGlobalRemove`: `global remove <addr+size>` —
    /// `symboltab->removeRange(getGlobalScope(), space, first, last)`.
    IfcGlobalRemove,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let (space, first, last) = parse_global_range(status, s)?;
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let scope = prog
            .arch()
            .symboltab
            .get_global_scope()
            .ok_or_else(|| IfaceError::execution("No global scope"))?;
        prog.arch_mut().symboltab.remove_range(scope, space, first, last);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcGlobalify`: `global spaces`.
    IfcGlobalify,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("Architecture::globalify (whole-space globals)"))
    }
);

decomp_command!(
    /// C++ `IfcGlobalRegisters`: `global registers`.
    IfcGlobalRegisters,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("Architecture register-global mapping"))
    }
);

// --- graph dataflow / controlflow / dom (ifacedecomp.cc:2509-2588) ---------

decomp_command!(
    /// C++ `IfcGraphDataflow`: `graph dataflow <filename>`.
    IfcGraphDataflow,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("dump_dataflow_graph (graph.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcGraphControlflow`: `graph controlflow <filename>`.
    IfcGraphControlflow,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("dump_controlflow_graph (graph.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcGraphDom`: `graph dom <filename>`.
    IfcGraphDom,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("dump_dom_graph (graph.cc)"))
    }
);

// --- produce C / prototypes (ifacedecomp.cc:2360, 2412) --------------------

decomp_command!(
    /// C++ `IfcProduceC`: `produce C <filename>`.
    IfcProduceC,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        s.skip_ws();
        let name = s.read_token();
        if name.is_empty() {
            return Err(IfaceError::parse("Need file name to write to"));
        }
        Err(engine_unavailable("iterateFunctionsAddrOrder + PrintLanguage::docFunction"))
    }
);

decomp_command!(
    /// C++ `IfcProducePrototypes`: `produce prototypes`.
    IfcProducePrototypes,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image"));
        }
        if !dcp.cgraph_allocated {
            return Err(IfaceError::execution("Callgraph has not been built"));
        }
        Err(engine_unavailable("iterateFunctionsLeafOrder (prototype distinguishing)"))
    }
);

// --- print inputs / inputs all (ifacedecomp.cc:2240, 2253) -----------------

decomp_command!(
    /// C++ `IfcPrintInputs`: `print inputs`.
    IfcPrintInputs,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("IfcPrintInputs::print (function-input report)"))
    }
);

decomp_command!(
    /// C++ `IfcPrintInputsAll`: `print inputs all`.
    IfcPrintInputsAll,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("iterateFunctionsAddrOrder (inputs report)"))
    }
);

// --- prototype lock / unlock (ifacedecomp.cc:2286, 2301) -------------------

decomp_command!(
    /// C++ `IfcLockPrototype`: `prototype lock`.
    IfcLockPrototype,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("FuncProto::setInputLock/setOutputLock"))
    }
);

decomp_command!(
    /// C++ `IfcUnlockPrototype`: `prototype unlock`.
    IfcUnlockPrototype,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("FuncProto::setInputLock/setOutputLock (clear)"))
    }
);

// --- print localrange / map (ifacedecomp.cc:2316, 2330) --------------------

decomp_command!(
    /// C++ `IfcPrintLocalrange`: `print localrange`.
    IfcPrintLocalrange,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata::printLocalRange"))
    }
);

decomp_command!(
    /// C++ `IfcPrintMap`: `print map [<name>]`.
    IfcPrintMap,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _name = s.read_token();
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image"));
        }
        Err(engine_unavailable("Scope::printBounds/printEntries"))
    }
);

// --- comment instruction (ifacedecomp.cc:2589) -----------------------------

decomp_command!(
    /// C++ `IfcCommentInstr`: `comment instruction <addr> <text>`.
    IfcCommentInstr,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("Decompile action not loaded"));
            }
            if dcp.fd.is_none() {
                return Err(IfaceError::execution("No function selected"));
            }
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_ref().expect("conf checked non-None above");
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        // C++ skips ws then reads char-by-char to EOL as the comment body.
        s.skip_ws();
        let comment = s.rest();
        let func_addr = dcp.fd.as_ref().expect("fd checked non-None above").get_address().clone();
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let arch = prog.arch_mut();
        let ctype = arch.print().instruction_comment_flags();
        arch.commentdb.add_comment(ctype, &func_addr, &addr, &comment);
        Ok(())
    }
);

// --- count pcode / actionstats / reset actionstats (ifacedecomp.cc) --------

decomp_command!(
    /// C++ `IfcCountPcode`: `count pcode`.
    IfcCountPcode,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata op-count walk"))
    }
);

decomp_command!(
    /// C++ `IfcPrintActionstats`: `print actionstats`.
    IfcPrintActionstats,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Image not loaded"));
        }
        Err(engine_unavailable("ActionDatabase::getCurrent()->printStatistics"))
    }
);

decomp_command!(
    /// C++ `IfcResetActionstats`: `reset actionstats`.
    IfcResetActionstats,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Image not loaded"));
        }
        Err(engine_unavailable("ActionDatabase::getCurrent()->resetStats"))
    }
);

// --- duplicate hash (ifacedecomp.cc:2679) ----------------------------------

decomp_command!(
    /// C++ `IfcDuplicateHash`: `duplicate hash`.
    IfcDuplicateHash,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image"));
        }
        Err(engine_unavailable("IfcDuplicateHash::check (DynamicHash walk)"))
    }
);

// --- callgraph build / build quick / dump / load / list --------------------

decomp_command!(
    /// C++ `IfcCallGraphBuild`: `callgraph build`.
    IfcCallGraphBuild,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Image not loaded"));
        }
        Err(engine_unavailable("CallGraph build (callgraph.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcCallGraphBuildQuick`: `callgraph build quick`.
    IfcCallGraphBuildQuick,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Image not loaded"));
        }
        Err(engine_unavailable("CallGraph build (quick) (callgraph.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcCallGraphDump`: `callgraph dump <filename>`.
    IfcCallGraphDump,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if !dcp.cgraph_allocated {
            return Err(IfaceError::execution("No callgraph present"));
        }
        Err(engine_unavailable("CallGraph::encode (callgraph.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcCallGraphLoad`: `callgraph load <filename>`.
    IfcCallGraphLoad,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Decompile action not loaded"));
        }
        if dcp.cgraph_allocated {
            return Err(IfaceError::execution("Callgraph already loaded"));
        }
        Err(engine_unavailable("CallGraph::decoder (callgraph.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcCallGraphList`: `callgraph list`.
    IfcCallGraphList,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if !dcp.cgraph_allocated {
            return Err(IfaceError::execution("No callgraph present"));
        }
        Err(engine_unavailable("CallGraph leaf walk (callgraph.cc)"))
    }
);

// --- fixup call / callother / apply (ifacedecomp.cc) -----------------------

decomp_command!(
    /// C++ `IfcCallFixup`: `fixup call ...`.
    IfcCallFixup,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("PcodeInjectLibrary::manualCallFixup"))
    }
);

decomp_command!(
    /// C++ `IfcCallOtherFixup`: `fixup callother ...`.
    IfcCallOtherFixup,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("PcodeInjectLibrary::manualCallOtherFixup"))
    }
);

decomp_command!(
    /// C++ `IfcFixupApply`: `fixup apply <fixup> <function>`.
    ///
    /// Resolve the call-fixup by name (`getPayloadId(CALLFIXUP_TYPE,fixup)`) and the
    /// function symbol by name, then set the fixup as the function's inject id (C++
    /// `fd->getFuncProto().setInjectId(injectid)`).  The cspec `<callfixup>` elements
    /// are decoded into `pcodeinjectlib` at bootstrap (`Architecture::decode_call_fixups`).
    IfcFixupApply,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("Missing fixup name"));
        }
        let fixup_name = s.read_token();
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("Missing function name"));
        }
        let func_name = s.read_token();

        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let injectid = prog
            .arch()
            .pcodeinjectlib
            .base
            .get_payload_id(kuna_decomp::pcodeinject::CALLFIXUP_TYPE, fixup_name.as_bytes());
        if injectid < 0 {
            return Err(IfaceError::execution(format!("Unknown fixup: {fixup_name}")));
        }
        // C++ resolveScopeFromSymbolName + queryFunction; "Unknown function name" if
        // no function symbol matches.  query_global_function folds both into the
        // single resolution the loader-symbol table backs.
        let sid = prog
            .arch()
            .query_global_function(&func_name)
            .map_err(|_| IfaceError::execution(format!("Unknown function name: {func_name}")))?;
        // C++ fd->getFuncProto().setInjectId(injectid).
        prog.arch_mut().symboltab.set_function_inject_id(sid, injectid);
        status.out("Successfully applied callfixup\n");
        Ok(())
    }
);

// --- volatile / readonly (ifacedecomp.cc) ----------------------------------

decomp_command!(
    /// C++ `IfcVolatile`: `volatile [space,offset,size]`.
    IfcVolatile,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        mark_property_range(status, s, property_flag::volatil, "Successfully marked range as volatile")
    }
);

decomp_command!(
    /// C++ `IfcReadonly`: `readonly [space,offset,size]`.
    IfcReadonly,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        mark_property_range(status, s, property_flag::readonly, "Successfully marked range as readonly")
    }
);

// --- pointer setting / prefersplit (ifacedecomp.cc) ------------------------

decomp_command!(
    /// C++ `IfcPointerSetting`: `pointer setting <name> <basetype> offset <val>`
    /// (relative pointer) or `pointer setting <name> <basetype> space <spc>`
    /// (space-attributed pointer).  Ported from `ifacedecomp.cc:3051-3099`.
    IfcPointerSetting,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        {
            let dcp = dcp_mut(status)?;
            if dcp.conf.is_none() {
                return Err(IfaceError::execution("No load image present"));
            }
        }
        // C++ parse: name, base-type, then the setting keyword, each guarded on
        // eof for the "Missing ..." parse errors.
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("Missing name"));
        }
        let type_name = s.read_token();
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("Missing base-type"));
        }
        let base_type = s.read_token();
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("Missing setting"));
        }
        let setting = s.read_token();
        use kuna_decomp::dtype::type_metatype;
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        if setting == "offset" {
            // s.unsetf(dec|hex|oct); s >> off; if (off <= 0) throw "Missing offset".
            s.skip_ws();
            let off_tok = s.read_token();
            let off_val = parse_userbase_u64(&off_tok);
            let off = match off_val {
                Some(v) if v >= 1 && v <= int4::MAX as u64 => v as int4,
                _ => return Err(IfaceError::parse("Missing offset")),
            };
            // bt = types->findByName(baseType); must be a TYPE_STRUCT.
            let bt = prog
                .arch()
                .types()
                .find_by_name(&base_type)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            let bt = match bt {
                Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT => t,
                _ => return Err(IfaceError::parse("Base-type must be a structure")),
            };
            let ptrto = prog
                .arch()
                .types()
                .get_ptr_to_from_parent(std::rc::Rc::clone(&bt), off)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
            // spc = conf->getDefaultDataSpace(); then getTypePointerRel(6-arg).
            let spc = prog
                .arch()
                .manage()
                .get_default_data_space()
                .cloned()
                .ok_or_else(|| IfaceError::execution("No default data space"))?;
            let addr_size = spc.get_addr_size() as int4;
            let word_size = spc.get_word_size() as int4;
            prog.arch()
                .types()
                .get_type_pointer_rel_full(addr_size, bt, ptrto, word_size, off, &type_name)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        } else if setting == "space" {
            // s >> spaceName; if empty throw "Missing name of address space".
            s.skip_ws();
            let space_name = s.read_token();
            if space_name.is_empty() {
                return Err(IfaceError::parse("Missing name of address space"));
            }
            // ptrTo = types->findByName(baseType); throw if unknown.
            let ptr_to = prog
                .arch()
                .types()
                .find_by_name(&base_type)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?
                .ok_or_else(|| {
                    IfaceError::parse(format!("Unknown base data-type: {base_type}"))
                })?;
            // spc = conf->getSpaceByName(spaceName); throw if unknown.
            let spc = prog
                .arch()
                .manage()
                .get_space_by_name(&space_name)
                .cloned()
                .ok_or_else(|| IfaceError::parse(format!("Unknown space: {space_name}")))?;
            prog.arch()
                .types()
                .get_type_pointer_with_space(ptr_to, spc, &type_name)
                .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        } else {
            return Err(IfaceError::parse(format!("Unknown pointer setting: {setting}")));
        }
        status.out(&format!("Successfully created pointer: {type_name}\n"));
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcPreferSplit`: `prefersplit <addr+size> <splitsize>`.
    IfcPreferSplit,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("parse_machaddr + Architecture::splitrecords"))
    }
);

// --- structure blocks / analyze range (ifacedecomp.cc) ---------------------

decomp_command!(
    /// C++ `IfcStructureBlocks`: `structure blocks <infile> <outfile>`.
    IfcStructureBlocks,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("BlockGraph structuring (blockaction.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcAnalyzeRange`: `analyze range <varnode>`.
    IfcAnalyzeRange,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("ValueSetSolver range analysis"))
    }
);

// --- load test file / list test commands / execute test command ------------

decomp_command!(
    /// C++ `IfcLoadTestFile`: `load test file <filename>`.
    IfcLoadTestFile,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("FunctionTestCollection::loadTest (testfunction.cc)"))
    }
);

decomp_command!(
    /// C++ `IfcListTestCommands`: `list test commands`.
    IfcListTestCommands,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if !dcp.test_collection_present {
            return Err(IfaceError::execution("No test file is loaded"));
        }
        Err(engine_unavailable("FunctionTestCollection command listing"))
    }
);

decomp_command!(
    /// C++ `IfcExecuteTestCommand`: `execute test command <i>`.
    IfcExecuteTestCommand,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if !dcp.test_collection_present {
            return Err(IfaceError::execution("No test file is loaded"));
        }
        Err(engine_unavailable("FunctionTestCollection command run"))
    }
);

// --- continue (ifacedecomp.cc:2475) ----------------------------------------

decomp_command!(
    /// C++ `IfcContinue`: `continue` — resume a broken decompilation.
    IfcContinue,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("Decompile action not loaded"));
        }
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        // C++ then checks the action status (status_start -> "Decompilation has
        // not been started", status_end -> "Decompilation is already complete")
        // before perform().
        Err(engine_unavailable("ActionDatabase::getCurrent()->perform (continue)"))
    }
);

// ===========================================================================
// Registration — IfaceDecompCapability::registerCommands (ifacedecomp.cc:34).
//
// The token sequences are byte-identical to C++; this is the prefix-expansion
// surface the datatests rely on.  The "base" module commands (quit, history,
// openfile, closefile, echo) are ported in interface.rs and registered here
// exactly where C++ registers them.
// ===========================================================================

/// C++ `IfaceDecompCapability::registerCommands(IfaceStatus *status)`
/// (`ifacedecomp.cc:34`): register every decompiler console command, plus the
/// "base" module commands the console shares.
///
/// `OPACTION_DEBUG`/`CPUI_RULECOMPILE`/`TYPEPROP_DEBUG`-gated commands are
/// compiled out of the kuna build (those macros are never defined here), exactly
/// as in C++, so they are not registered.
pub fn register_decomp_commands(status: &mut IfaceStatus) {
    use crate::interface::{
        IfcClosefile, IfcEcho, IfcHistory, IfcOpenfile, IfcOpenfileAppend, IfcQuit,
    };

    // The base module commands (interface.cc) — registered first, exactly as
    // ifacedecomp.cc:37-45.  IfcComment carries the "decompile" module-data
    // builder; "//" is the first decompiler command registered, so the shared
    // IfaceDecompData is created here.
    status.register_com(Box::new(IfcComment), &["//"]);
    status.register_com(Box::new(IfcComment), &["#"]);
    status.register_com(Box::new(IfcComment), &["%"]);
    status.register_com(Box::new(IfcQuit), &["quit"]);
    status.register_com(Box::new(IfcHistory), &["history"]);
    status.register_com(Box::new(IfcOpenfile), &["openfile", "write"]);
    status.register_com(Box::new(IfcOpenfileAppend), &["openfile", "append"]);
    status.register_com(Box::new(IfcClosefile), &["closefile"]);
    status.register_com(Box::new(IfcEcho), &["echo"]);

    // The decompiler module commands (ifacedecomp.cc:47-142).  `source` is a
    // console-only command (consolemain.cc) registered there in C++; the kuna
    // console binary registers it where it owns the source-script reader.
    status.register_com(Box::new(IfcOption), &["option"]);
    status.register_com(Box::new(IfcParseFile), &["parse", "file"]);
    status.register_com(Box::new(IfcParseLine), &["parse", "line"]);
    status.register_com(Box::new(IfcAdjustVma), &["adjust", "vma"]);
    status.register_com(Box::new(IfcFuncload), &["load", "function"]);
    status.register_com(Box::new(IfcAddrrangeLoad), &["load", "addr"]);
    status.register_com(Box::new(IfcReadSymbols), &["read", "symbols"]);
    status.register_com(Box::new(IfcCleararch), &["clear", "architecture"]);
    status.register_com(Box::new(IfcMapaddress), &["map", "address"]);
    status.register_com(Box::new(IfcMaphash), &["map", "hash"]);
    status.register_com(Box::new(IfcMapParam), &["map", "param"]);
    status.register_com(Box::new(IfcMapReturn), &["map", "return"]);
    status.register_com(Box::new(IfcMapfunction), &["map", "function"]);
    status.register_com(Box::new(IfcMapexternalref), &["map", "externalref"]);
    status.register_com(Box::new(IfcMaplabel), &["map", "label"]);
    status.register_com(Box::new(IfcMapconvert), &["map", "convert"]);
    status.register_com(Box::new(IfcMapunionfacet), &["map", "unionfacet"]);
    status.register_com(Box::new(IfcPrintdisasm), &["disassemble"]);
    status.register_com(Box::new(IfcDecompile), &["decompile"]);
    status.register_com(Box::new(IfcDump), &["dump"]);
    status.register_com(Box::new(IfcDumpbinary), &["binary"]);
    status.register_com(Box::new(IfcForcegoto), &["force", "goto"]);
    status.register_com(Box::new(IfcForceFormat), &["force", "varnode"]);
    status.register_com(Box::new(IfcForceDatatypeFormat), &["force", "datatype"]);
    status.register_com(Box::new(IfcProtooverride), &["override", "prototype"]);
    status.register_com(Box::new(IfcJumpOverride), &["override", "jumptable"]);
    status.register_com(Box::new(IfcFlowOverride), &["override", "flow"]);
    status.register_com(Box::new(IfcDeadcodedelay), &["deadcode", "delay"]);
    status.register_com(Box::new(IfcGlobalAdd), &["global", "add"]);
    status.register_com(Box::new(IfcGlobalRemove), &["global", "remove"]);
    status.register_com(Box::new(IfcGlobalify), &["global", "spaces"]);
    status.register_com(Box::new(IfcGlobalRegisters), &["global", "registers"]);
    status.register_com(Box::new(IfcGraphDataflow), &["graph", "dataflow"]);
    status.register_com(Box::new(IfcGraphControlflow), &["graph", "controlflow"]);
    status.register_com(Box::new(IfcGraphDom), &["graph", "dom"]);
    status.register_com(Box::new(IfcPrintLanguage), &["print", "language"]);
    status.register_com(Box::new(IfcPrintCStruct), &["print", "C"]);
    status.register_com(Box::new(IfcPrintCFlat), &["print", "C", "flat"]);
    status.register_com(Box::new(IfcPrintCGlobals), &["print", "C", "globals"]);
    status.register_com(Box::new(IfcPrintCTypes), &["print", "C", "types"]);
    status.register_com(Box::new(IfcPrintCXml), &["print", "C", "xml"]);
    status.register_com(Box::new(IfcPrintParamMeasures), &["print", "parammeasures"]);
    status.register_com(Box::new(IfcProduceC), &["produce", "C"]);
    status.register_com(Box::new(IfcProducePrototypes), &["produce", "prototypes"]);
    status.register_com(Box::new(IfcPrintRaw), &["print", "raw"]);
    status.register_com(Box::new(IfcPrintInputs), &["print", "inputs"]);
    status.register_com(Box::new(IfcPrintInputsAll), &["print", "inputs", "all"]);
    status.register_com(Box::new(IfcListaction), &["list", "action"]);
    status.register_com(Box::new(IfcListOverride), &["list", "override"]);
    status.register_com(Box::new(IfcListprototypes), &["list", "prototypes"]);
    status.register_com(Box::new(IfcSetcontextrange), &["set", "context"]);
    status.register_com(Box::new(IfcSettrackedrange), &["set", "track"]);
    status.register_com(Box::new(IfcBreakstart), &["break", "start"]);
    status.register_com(Box::new(IfcBreakaction), &["break", "action"]);
    status.register_com(Box::new(IfcPrintSpaces), &["print", "spaces"]);
    status.register_com(Box::new(IfcPrintHigh), &["print", "high"]);
    status.register_com(Box::new(IfcPrintTree), &["print", "tree", "varnode"]);
    status.register_com(Box::new(IfcPrintBlocktree), &["print", "tree", "block"]);
    status.register_com(Box::new(IfcPrintLocalrange), &["print", "localrange"]);
    status.register_com(Box::new(IfcPrintMap), &["print", "map"]);
    status.register_com(Box::new(IfcPrintVarnode), &["print", "varnode"]);
    status.register_com(Box::new(IfcPrintCover), &["print", "cover", "high"]);
    status.register_com(Box::new(IfcVarnodeCover), &["print", "cover", "varnode"]);
    status.register_com(Box::new(IfcVarnodehighCover), &["print", "cover", "varnodehigh"]);
    status.register_com(Box::new(IfcPrintExtrapop), &["print", "extrapop"]);
    status.register_com(Box::new(IfcPrintActionstats), &["print", "actionstats"]);
    status.register_com(Box::new(IfcResetActionstats), &["reset", "actionstats"]);
    status.register_com(Box::new(IfcCountPcode), &["count", "pcode"]);
    status.register_com(Box::new(IfcTypeVarnode), &["type", "varnode"]);
    status.register_com(Box::new(IfcNameVarnode), &["name", "varnode"]);
    status.register_com(Box::new(IfcRename), &["rename"]);
    status.register_com(Box::new(IfcRetype), &["retype"]);
    status.register_com(Box::new(IfcRemove), &["remove"]);
    status.register_com(Box::new(IfcIsolate), &["isolate"]);
    status.register_com(Box::new(IfcLockPrototype), &["prototype", "lock"]);
    status.register_com(Box::new(IfcUnlockPrototype), &["prototype", "unlock"]);
    status.register_com(Box::new(IfcCommentInstr), &["comment", "instruction"]);
    status.register_com(Box::new(IfcDuplicateHash), &["duplicate", "hash"]);
    status.register_com(Box::new(IfcCallGraphBuild), &["callgraph", "build"]);
    status.register_com(Box::new(IfcCallGraphBuildQuick), &["callgraph", "build", "quick"]);
    status.register_com(Box::new(IfcCallGraphDump), &["callgraph", "dump"]);
    status.register_com(Box::new(IfcCallGraphLoad), &["callgraph", "load"]);
    status.register_com(Box::new(IfcCallGraphList), &["callgraph", "list"]);
    status.register_com(Box::new(IfcCallFixup), &["fixup", "call"]);
    status.register_com(Box::new(IfcCallOtherFixup), &["fixup", "callother"]);
    status.register_com(Box::new(IfcFixupApply), &["fixup", "apply"]);
    status.register_com(Box::new(IfcVolatile), &["volatile"]);
    status.register_com(Box::new(IfcReadonly), &["readonly"]);
    status.register_com(Box::new(IfcPointerSetting), &["pointer", "setting"]);
    status.register_com(Box::new(IfcPreferSplit), &["prefersplit"]);
    status.register_com(Box::new(IfcStructureBlocks), &["structure", "blocks"]);
    status.register_com(Box::new(IfcAnalyzeRange), &["analyze", "range"]);
    status.register_com(Box::new(IfcLoadTestFile), &["load", "test", "file"]);
    status.register_com(Box::new(IfcListTestCommands), &["list", "test", "commands"]);
    status.register_com(Box::new(IfcExecuteTestCommand), &["execute", "test", "command"]);
    status.register_com(Box::new(IfcContinue), &["continue"]);
}

/// Register the console-only commands C++ `consolemain.cc` adds on top of
/// [`register_decomp_commands`] (the extra `main()` registrations:
/// `load file`/`addpath`/`save`/`restore`).
///
/// Only `load file` is wired in the kuna port (the engine-backed image load);
/// `addpath`/`save`/`restore` reach the spec-path globals / `Architecture::encode`
/// / `restoreXml` marshaling, which are later port items, so they are not
/// registered here (an unregistered token surfaces "ERROR: Invalid command",
/// matching a console where the command was never added).
pub fn register_console_commands(status: &mut IfaceStatus) {
    status.register_com(Box::new(IfcLoadFile), &["load", "file"]);
}

// ===========================================================================
// execute / mainloop (ifacedecomp.cc, the console driver).
// ===========================================================================

/// C++ free function `execute(IfaceStatus *status,IfaceDecompData *dcp)`
/// (`ifacedecomp.cc`): run one command line, mapping any thrown exception to its
/// console prefix.
///
/// The exception→prefix grammar is byte-faithful and load-bearing for the
/// harness:
///   - `IfaceParseError`     → `"Command parsing error: "`
///   - `IfaceExecutionError` → `"Execution error: "`
///   - `IfaceError` (base)   → `"ERROR: "`
///   - `ParseError`          → `"Parse ERROR: "`
///   - `RecovError`          → `"Function ERROR: "`
///   - `LowlevelError`       → `"Low-level ERROR: "` (+ `abortFunction`)
///   - `DecoderError`        → `"Decoding ERROR: "`  (+ `abortFunction`)
///
/// In the kuna port a command's `execute` returns an [`IfaceError`] (the
/// interface hierarchy); engine errors (`KunaError`/`LowlevelError` family) are
/// converted to an [`IfaceError`] at the (unported) engine call boundary, so the
/// three [`IfaceError`] kinds are the arms reachable today.  The remaining arms
/// are transcribed in [`render_engine_error`] for when the engine integration
/// lands and real `KunaError`s flow out of the command bodies; the catch
/// placement (which frame catches which) is preserved (ADR 0004).
///
/// Returns after writing the diagnostic and calling [`IfaceStatus::evaluate_error`].
pub fn execute(status: &mut IfaceStatus) {
    match status.run_command() {
        Ok(_) => return,
        Err(err) => {
            // The IfaceError hierarchy: ifaceParse / ifaceExecution / base.
            if err.is_parse() {
                status.out(&format!("Command parsing error: {err}\n"));
            } else if err.is_execution() {
                status.out(&format!("Execution error: {err}\n"));
            } else {
                status.out(&format!("ERROR: {err}\n"));
            }
        }
    }
    status.evaluate_error();
}

/// Render an engine-layer error (`KunaError`, the `LowlevelError` hierarchy)
/// under the exact console prefix C++ `execute` assigns its class, and run the
/// `abortFunction` side effect for the two arms that have it.
///
/// Not yet reachable from [`execute`] (no command body lets a `KunaError`
/// escape, because the engine calls are routed through `engine_unavailable` as
/// an `IfaceExecutionError`); transcribed now so the catch grammar is complete
/// and ready to wire when the engine integration lands.  Mirrors the
/// `ParseError`/`RecovError`/`LowlevelError`/`DecoderError` catch arms of C++
/// `execute`.
pub fn render_engine_error(
    err: &kuna_base::error::KunaError,
    dcp: &mut IfaceDecompData,
    out: &mut String,
) {
    use kuna_base::error::KunaError;
    match err {
        KunaError::Parse { explain } => {
            out.push_str("Parse ERROR: ");
            out.push_str(explain);
            out.push('\n');
        }
        KunaError::Recov { explain } => {
            out.push_str("Function ERROR: ");
            out.push_str(explain);
            out.push('\n');
        }
        KunaError::Decoder { explain } => {
            out.push_str("Decoding ERROR: ");
            out.push_str(explain);
            out.push('\n');
            dcp.abort_function(out);
        }
        // The remaining KunaError variants are all part of the C++
        // `LowlevelError` hierarchy (RecovError aside, handled above), which the
        // `catch(LowlevelError &)` frame catches.
        other => {
            out.push_str("Low-level ERROR: ");
            out.push_str(other.explain());
            out.push('\n');
            dcp.abort_function(out);
        }
    }
}

/// C++ free function `mainloop(IfaceStatus *status)` (`ifacedecomp.cc`): execute
/// commands as they become available.
///
/// Drives the nested loop: drain the current input stream
/// (writing the prompt and running each command via [`execute`]), then break on
/// `done`, break if there is no script to pop, else `popScript` and continue.
/// The C++ `optr->flush()` is a no-op in the buffer-backed [`IfaceStatus`] (the
/// binary drains `optr`), so it is elided.
pub fn mainloop(status: &mut IfaceStatus) {
    loop {
        while !status.is_stream_finished() {
            status.write_prompt();
            // C++ optr->flush() — no-op against the in-memory buffer.
            execute(status);
        }
        if status.done {
            break;
        }
        if status.num_input_stream_size() == 0 {
            break;
        }
        status.pop_script();
    }
}

#[cfg(test)]
mod tests;
