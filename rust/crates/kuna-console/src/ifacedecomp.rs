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
use kuna_base::types::int4;
use kuna_decomp::decompile_drive::{
    build_and_follow_flow, build_and_follow_flow_with_override, print_c,
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
    /// seam the `Funcdata` (`dcp.fd`) is built later by `load function`/`load addr`
    /// (`build_and_follow_flow` makes a fresh one), so the pieces are stashed here
    /// when the named function symbol exists and applied at load time.
    /// // SEAM(W4 queryFunction/FuncProto restore)
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
fn parse_machaddr(
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
    let (addr, size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
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
            // the prototype.  In the kuna console seam the named function may live
            // only in the binaryimage's symbol records (the readLoaderSymbols →
            // Scope::addFunction markup is a W4 seam, so it is not yet in the
            // symboltab), and `dcp.fd` is built later by `load function`.  So the
            // pieces are captured here and stashed (applied at load time) rather
            // than rejected — letting `parse line extern` take effect and the test
            // proceed to decompile.  // SEAM(W4 queryFunction/FuncProto restore)
            *captured.borrow_mut() = Some(pieces);
            Ok(())
        });
        (org, captured.into_inner(), res)
    };
    let _ = org; // org is consumed by parse_c; kept for symmetry with the C++ glb
    match parse_result {
        Ok(()) => {
            if let Some(pieces) = extern_pieces {
                // Stash for application at load time (the FuncProto restore seam).
                let dcp = dcp_mut(status)?;
                dcp.pending_prototypes.insert(pieces.name.clone(), pieces);
            }
            Ok(())
        }
        Err(e) => {
            // C++: *status->optr << "Error in C syntax: " << err.explain << endl;
            //      throw IfaceExecutionError("Bad C syntax");
            status.out(&format!("Error in C syntax: {}\n", e.explain()));
            Err(IfaceError::execution("Bad C syntax"))
        }
    }
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
        // C++: string res = dcp->conf->options->set(ElementId::find(optname,0),p1,p2,p3);
        //      *status->optr << res << endl;
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
        let filename = s.read_token();
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
        let mut filename = s.read_token();
        s.skip_ws();
        if !s.eof() {
            // Two parameters: the first was the target, the second is the file.
            filename = s.read_token();
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
        match bootstrap_from_file(&filename, &spec_roots) {
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
        let funcname = s.read_token();
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No image loaded"));
        }
        // C++: resolveScopeFromSymbolName + queryFunction; then if !hasNoCode,
        //      dcp->followFlow(*status->optr,0).  The kuna seam resolves the entry
        // from the binaryimage's own symbol records (the readLoaderSymbols seam).
        let flow_overrides = dcp.pending_flow_overrides.get(&funcname).cloned().unwrap_or_default();
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let entry = match prog.lookup_symbol(&funcname) {
            Some(addr) => addr,
            None => return Err(IfaceError::execution(format!("Unknown function name: {funcname}"))),
        };
        // Build the Funcdata + follow flow (C++ Funcdata + followFlow), seeding any
        // `override flow` facts stashed for this function before flow follows.
        let fd = build_and_follow_flow_with_override(
            prog.arch_mut(),
            &funcname,
            entry,
            UNBOUNDED_SIZE,
            &flow_overrides,
        )
        .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        dcp.fd = Some(fd);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcAddrrangeLoad`: create a function at an address (`load addr
    /// <addr> [name]`).
    IfcAddrrangeLoad,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No binary loaded"));
        }
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        // C++ Address offset = parse_machaddr(s,size,*dcp->conf->types) — the full
        // console address grammar over the engine spaces.
        let (offset, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        let name = s.read_token(); // optional
        // C++ nameFunction picks a default name if none was given.
        let name = if name.is_empty() {
            prog.arch().name_function(&offset)
        } else {
            name
        };
        // C++ addFunction(offset,name); followFlow(size).  The symbol-table
        // addFunction is a later seam; build the Funcdata + follow flow directly.
        let fd = build_and_follow_flow(prog.arch_mut(), &name, offset, UNBOUNDED_SIZE)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
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
        // at `load file` (the readLoaderSymbols seam runs eagerly there), so the
        // symbols are already available; this is a faithful no-op success (the
        // symbol-table `Scope::addFunction` markup is a later W4 item).
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
        // C++ Address addr = parse_machaddr(...); ct = parse_type(s,name,glb).
        let fd_present = dcp_mut(status)?.fd.is_some();
        if fd_present {
            // C++ fd-local form (ifacedecomp.cc:561-563):
            //   sym = dcp->fd->getScopeLocal()->addSymbol(name,ct,addr,Address())->getSymbol();
            //   sym->getScope()->setAttribute(sym, Varnode::namelock|Varnode::typelock);
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
        // C++ ct = parse_type(s,name,dcp->conf): parse the required type + name.
        let (addr_size, word_size) = prog.arch().data_org();
        let org = crate::grammar::DataOrg { addr_size, word_size };
        let typetext = s.rest();
        let (ct, name) = crate::grammar::parse_type(&typetext, prog.arch().types(), org)
            .map_err(|e| IfaceError::parse(e.explain().to_string()))?;
        // Global branch: flags = namelock|typelock | getProperty(addr);
        //   scope = findCreateScopeFromSymbolName(name); sym = scope->addSymbol(...);
        //   setAttribute(flags); if scope has parent: addRange.
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
        let (sym, _eref) = arch
            .symboltab
            .add_symbol_mapped(scope, &basename, ct, &addr, &invalid)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        arch.symboltab.set_attribute(sym, flags);
        Ok(())
    }
);

decomp_command!(
    /// C++ `IfcMaphash`: `map hash <addr> <hash> <typedeclaration>`.
    IfcMaphash,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        Err(engine_unavailable("parse_machaddr + parse_type + Scope::addDynamicSymbol"))
    }
);

decomp_command!(
    /// C++ `IfcMapParam`: `map param #i <addr> <typedeclaration>`.
    IfcMapParam,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        Err(engine_unavailable("parse_machaddr + parse_type + FuncProto::setParam"))
    }
);

decomp_command!(
    /// C++ `IfcMapReturn`: `map return <addr> <typedeclaration>`.
    IfcMapReturn,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        Err(engine_unavailable("parse_machaddr + parse_type + FuncProto::setOutput"))
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
        // C++ Address addr = parse_machaddr(s,size,*dcp->conf->types).
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        s.skip_ws();
        let mut name = s.read_token(); // optional
        if name.is_empty() {
            name = prog.arch().name_function(&addr);
        }
        // C++ scope = symboltab->findCreateScopeFromSymbolName(name,"::",basename,0);
        //      dcp->fd = scope->addFunction(addr,name)->getFunction();
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
        // flow; the kuna seam builds the same un-followed Funcdata (the real flow
        // follow runs at `load function`/`decompile`).
        let fd = prog
            .arch()
            .new_funcdata(&name, addr.clone(), UNBOUNDED_SIZE)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        // Make the function loadable by `load function <name>` (the console seam).
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
        // LabSymbol::buildType -> getBase(1, TYPE_UNKNOWN).
        let lab_type = prog
            .arch()
            .types()
            .get_base(1, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)
            .map_err(|e| IfaceError::execution(e.explain().to_string()))?;
        if fd_present {
            // C++ fd-local form: scope = dcp->fd->getScopeLocal();
            //   sym = scope->addCodeLabel(addr,name);
            //   scope->setAttribute(sym, namelock|typelock).
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
    /// C++ `IfcMapconvert`: `map convert <format> <value> <addr> <hash>`.
    IfcMapconvert,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        let name = s.read_token();
        // C++ matches hex|dec|bin|oct|char, else throws "Bad convert format".
        match name.as_str() {
            "hex" | "dec" | "bin" | "oct" | "char" => {}
            _ => return Err(IfaceError::parse("Bad convert format")),
        }
        Err(engine_unavailable("parse_machaddr + Scope::addEquateSymbol"))
    }
);

decomp_command!(
    /// C++ `IfcMapunionfacet`: `map unionfacet <union> <field> <addr> <hash>`.
    IfcMapunionfacet,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function loaded"));
        }
        s.skip_ws();
        let _union_name = s.read_token();
        // C++ then looks the union up in conf->types (unported) and validates it
        // is a TYPE_UNION ("Bad union data-type: <name>") before parsing the
        // field index / address / hash.
        Err(engine_unavailable("TypeFactory::findByName + Scope::addUnionFacetSymbol"))
    }
);

// --- disassemble [addr1 addr2] (ifacedecomp.cc:806) ------------------------

decomp_command!(
    /// C++ `IfcPrintdisasm`: disassemble a range (or the current function if no
    /// addresses are given).
    IfcPrintdisasm,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        s.skip_ws();
        if s.eof() {
            if dcp.fd.is_none() {
                return Err(IfaceError::execution("No function selected"));
            }
            // C++ prints "Assembly listing for <name>" then walks printAssembly.
            return Err(engine_unavailable("Translate::printAssembly (function listing)"));
        }
        // C++ parses two machine addresses then walks printAssembly.
        Err(engine_unavailable("parse_machaddr + Translate::printAssembly"))
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
        // Read the per-function values + take the program out so the engine work
        // borrows neither `status` nor `dcp` while the console output is written.
        let (name, has_no_code, proc_started, entry, size, mapped_symbols, pending_proto, mut prog) = {
            let dcp = dcp_mut(status)?;
            let (name, has_no_code, proc_started, entry, size, mapped_symbols) = match &dcp.fd {
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
                ),
            };
            // The `parse line extern <decl>` prototype stashed for this function
            // (C++ Architecture::setPrototype applies it to the queried Funcdata;
            // here the IR is rebuilt on `decompile`, so it is re-applied below).
            let pending_proto = dcp.pending_prototypes.get(&name).cloned();
            match dcp.conf.take() {
                None => return Err(IfaceError::execution("No load image present")),
                Some(prog) => {
                    (name, has_no_code, proc_started, entry, size, mapped_symbols, pending_proto, prog)
                }
            }
        };
        // The `override flow` facts stashed for this function (re-seeded on the
        // rebuilt IR, like `pending_proto`/`mapped_symbols`).
        let flow_overrides = dcp_mut(status)?
            .pending_flow_overrides
            .get(&name)
            .cloned()
            .unwrap_or_default();
        if has_no_code {
            // Restore the program before the early return.
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
        // C++: allacts.getCurrent()->reset(*fd); res = perform(*fd); then the
        // "Decompilation complete"/"Break at .." reporting.  The kuna decompile
        // drive (decompile_drive::decompile_func) installs the `decompile` root,
        // resets it, and runs the 252-pass perform loop to completion.
        let result = kuna_decomp::decompile_drive::decompile_func_full_with_override(
            prog.arch_mut(),
            &name,
            entry,
            size,
            &mapped_symbols,
            pending_proto.as_ref(),
            &flow_overrides,
        );
        // Restore the program (and the fresh Funcdata on success) regardless.
        let dcp = dcp_mut(status)?;
        dcp.conf = Some(prog);
        match result {
            Ok(fd) => {
                dcp.fd = Some(fd);
                // C++ res>=0 path: "Decompilation complete".
                status.out("Decompilation complete\n");
                Ok(())
            }
            Err(e) => Err(IfaceError::execution(e.explain().to_string())),
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
    /// C++ `IfcPrintCTypes`: `print C types`.
    IfcPrintCTypes,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("PrintLanguage::docTypeDefinitions (Architecture::print/types)"))
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
        // C++: dcp->conf->print->setOutputStream(status->fileoptr);
        //      dcp->conf->print->docFunction(dcp->fd);
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
        let dcp = dcp_mut(status)?;
        if dcp.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        Err(engine_unavailable("Funcdata::printRaw"))
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
        // C++ track.back().loc = dcp->conf->translate->getRegister(name).
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
    /// C++ `IfcRename`: `rename <oldname> <newname>`.
    IfcRename,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
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
        Err(engine_unavailable("IfaceDecompData::readSymbol + Scope::renameSymbol"))
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
    /// C++ `IfcRetype`: `retype <symbolname> <typedeclaration>`.
    IfcRetype,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        s.skip_ws();
        let name = s.read_token();
        if name.is_empty() {
            return Err(IfaceError::parse("Must specify name of symbol"));
        }
        // C++ then parse_type(s,newname,conf) before resolving the symbol.
        Err(engine_unavailable("parse_type + Scope::retypeSymbol"))
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
    /// C++ `IfcTypeVarnode`: `type varnode <varnode> <typedeclaration>`.
    IfcTypeVarnode,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        Err(engine_unavailable("parse_type + IfaceDecompData::readVarnode"))
    }
);

// --- force varnode / datatype / goto (ifacedecomp.cc:1769-1831) ------------

decomp_command!(
    /// C++ `IfcForceFormat`: `force varnode <varnode> <format>`.
    IfcForceFormat,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        // C++ readVarnode then validates constant/integer ("Can only force ..").
        Err(engine_unavailable("IfaceDecompData::readVarnode + Scope::setDisplayFormat"))
    }
);

decomp_command!(
    /// C++ `IfcForceDatatypeFormat`: `force datatype <datatype> <format>`.
    IfcForceDatatypeFormat,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        let _dcp = dcp_mut(status)?;
        s.skip_ws();
        let _type_name = s.read_token();
        // C++ looks the type up in conf->types ("Unknown data-type: <name>").
        Err(engine_unavailable("TypeFactory::findByName + setDisplayFormat"))
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
/// (`FlowInfo::queryCall`) is still seamed (LOSS-031 neighborhood), so this wrapper
/// only needs to round-trip the pieces; `encode`/`print_raw` (debug-only surfaces,
/// not exercised by the datatest corpus) are faithful stubs.
struct PiecesProtoOverride {
    pieces: kuna_decomp::fspec::PrototypePieces,
}

impl kuna_decomp::overrides::FuncProtoOverride for PiecesProtoOverride {
    fn set_override(&mut self, _val: bool) {
        // C++ FuncProto::setOverride sets a flag consumed by the (seamed)
        // applyPrototype; the pieces carry no such flag, so this is a no-op until
        // the W4 FuncProto-backed override lands.
    }
    fn encode(&self, _encoder: &mut dyn kuna_base::marshal::Encoder) -> kuna_base::error::KunaResult<()> {
        // SEAM(W4): FuncProto::encode of an override is a debug/save surface absent
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
}

decomp_command!(
    /// C++ `IfcProtooverride`: `override prototype <addr> <declaration>`.
    ///
    /// Parse the call-point address and the prototype declaration, find the call
    /// site at that address, build a prototype override, and install it on the
    /// function's `Override` (C++ `dcp->fd->getOverride().insertProtoOverride`).
    IfcProtooverride,
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        // C++: if (dcp->fd==0) throw "No function selected".
        if dcp_mut(status)?.fd.is_none() {
            return Err(IfaceError::execution("No function selected"));
        }
        let dcp = dcp_mut(status)?;
        let prog = dcp.conf.as_ref().expect("conf present when fd present");
        // C++ Address callpoint( parse_machaddr(s,discard,*dcp->conf->types) ).
        let (callpoint, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        // C++ for(i..numCalls) if (getCallSpecs(i)->getOp()->getAddr()==callpoint) break;
        //      if (i==numCalls) throw "No call is made at this address".
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
        // (FlowInfo::queryCall) is still seamed (LOSS-031 neighborhood), so the
        // override is stored but not yet applied at flow time; the command succeeds
        // (the script proceeds) exactly as C++.
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

decomp_command!(
    /// C++ `IfcGlobalAdd`: `global add <addr+size>`.
    IfcGlobalAdd,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No image loaded"));
        }
        Err(engine_unavailable("parse_machaddr + ScopeGlobal range add"))
    }
);

decomp_command!(
    /// C++ `IfcGlobalRemove`: `global remove <addr+size>`.
    IfcGlobalRemove,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No image loaded"));
        }
        Err(engine_unavailable("parse_machaddr + ScopeGlobal range remove"))
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
        // C++ Address addr = parse_machaddr(s,size,*dcp->conf->types).
        let (addr, _size) = parse_machaddr(prog, s, false).map_err(IfaceError::parse)?;
        // C++ skips ws then reads char-by-char to EOL as the comment body.
        s.skip_ws();
        let comment = s.rest();
        // uint4 type = dcp->conf->print->getInstructionComment();
        let func_addr = dcp.fd.as_ref().expect("fd checked non-None above").get_address().clone();
        let prog = dcp.conf.as_mut().expect("conf checked non-None above");
        let arch = prog.arch_mut();
        let ctype = arch.print().instruction_comment_flags();
        // dcp->conf->commentdb->addComment(type, fd->getAddress(), addr, comment).
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
        // C++ injectid = pcodeinjectlib->getPayloadId(CALLFIXUP_TYPE, fixupName);
        //      if (injectid < 0) throw "Unknown fixup: ".
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
    /// C++ `IfcPointerSetting`: `pointer setting <name> <basetype> ...`.
    IfcPointerSetting,
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let dcp = dcp_mut(status)?;
        if dcp.conf.is_none() {
            return Err(IfaceError::execution("No load image present"));
        }
        Err(engine_unavailable("TypeFactory pointer-setting (offset/space)"))
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
/// Faithful transcription of the nested loop: drain the current input stream
/// (writing the prompt and running each command via [`execute`]), then break on
/// `done`, break if there is no script to pop, else `popScript` and continue.
/// The C++ `optr->flush()` is a no-op in the buffer-backed [`IfaceStatus`] (the
/// binary drains `optr`), so it is elided.
pub fn mainloop(status: &mut IfaceStatus) {
    loop {
        while !status.is_stream_finished() {
            status.write_prompt();
            // C++ status->optr->flush(); — no-op against the in-memory buffer.
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
