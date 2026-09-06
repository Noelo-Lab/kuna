//! `--assert` — the one override plane an agent states facts through.
//!
//! # Why this exists
//!
//! Everything kuna knows about a program it *derived*. That is the right default
//! and it is wrong exactly where reverse engineering is hard, and until this
//! module the `kuna` binary had almost no lever to correct it: `rename`,
//! `retype`, `map param`, `map return`, `map address`, `comment instruction` and
//! `parse line extern` all work in the console and none of them was reachable
//! from `kuna` (`docs/re-needs/no-cli-rename-or-prototype-override.md`).  A
//! rename that does not survive the process that made it is not an interface.
//!
//! # The directive
//!
//! One line-oriented vocabulary, keyed by INTENT rather than by phase — an agent
//! should not have to know that renaming is P9 to rename something:
//!
//! ```text
//!   prototype <func> <C declaration>       parse line extern    P4 prototype-source
//!   param [<func>::]<i> <storage> <decl>   map param            P4 prototype-source
//!   return [<func>::]<storage> <decl>      map return           P4 prototype-source
//!   name [<func>::]<symbol> <newname>      rename               P9 naming-policy
//!   type [<func>::]<symbol> <C type>       retype               P5 type-propagation
//!   typedef <C declaration>                parse line           P5
//!   data <addr> <C typedeclaration>        map address          P5 const-pointer
//!   comment [<func>::]<addr> <text>        comment instruction  P9
//!   flow [<func>::]<addr> <flowkind>       override flow        P2 flow-classification
//!   function <start>[-<end>][=<name>]      function bounds      P1  (= --define-function)
//!   readonly <addr>+<size>                 readonly             P1 code-data-partition
//!   volatile <addr>+<size>                 volatile             P1 code-data-partition
//! ```
//!
//! [`Directive`] is the parsed form; the CLI's `assertdecl` module owns the text
//! syntax and the `@FILE` contract, exactly as `funcdecl` does for
//! `--define-function`.  Every directive produces exactly one [`Outcome`], so a
//! caller can report each one's fate machine-readably instead of scraping a
//! console transcript.
//!
//! # Where a directive lands
//!
//! The ordering is not cosmetic; it is what makes the plane work at all.
//!
//! * **Image-scoped** (`readonly`, `volatile`) paints a boolean property over a
//!   memory range.  A range property has to be stated BEFORE the symbols over it
//!   are mapped, because `Scope::addMap` folds the property into each
//!   `SymbolEntry` as it maps it (`database.cc:1156-1158`) and never looks at
//!   the range again; the generated console script therefore emits these before
//!   `read symbols`, and the in-process surface — where the loader's symbols are
//!   already mapped by the time a caller can say anything — re-applies the
//!   property to the symbols the range covers.
//! * **Program-scoped** (`function`, `typedef`, `prototype`, `data`) is applied
//!   by [`apply_program_scoped`] right after the analysis commit, so a declared
//!   fact outranks whatever discovery decided.
//! * **Function-scoped** (`param`, `return`, `comment`, `flow`) is turned into
//!   decompile SEEDS by [`function_seed`] — the facts the drive consumes at flow
//!   time.  `flow` is the sharpest of them: it reclassifies the flow out of one
//!   instruction (`branch`, `call`, `callreturn`, `return`), which is how a
//!   caller corrects a function whose body the flow-follower walked out of — an
//!   indirect `call *%rdx` that never returns, a tail call kuna read as a call,
//!   a jump into a neighbour.
//! * **Symbol-scoped** (`name`, `type`) can only be applied AFTER a decompile:
//!   a local like `v2` does not exist until one has run (`rename v2 buf` before
//!   the first decompile answers `No symbol named: v2`, which is precisely the
//!   bug that makes today's `--kassert p9 naming-policy` inert).  So
//!   [`apply_symbol_scoped`] runs on the first pass's `Funcdata` and the caller
//!   decompiles a second time with the mutated scope carried across — the same
//!   shape the console's `decompile` / `rename` / `decompile` sequence has.
//!
//! A directive that names no function binds to the function being decompiled.
//! That is unambiguous for `kuna decompile`, which selects exactly one; on a
//! whole-binary run it would silently mean "every function that happens to have a
//! `v2`", so it is REJECTED there with a detail telling the caller to qualify it
//! `<func>::<symbol>`.  Rejecting is the point: a directive that is accepted and
//! does nothing is worse than an error.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::Datatype;
use kuna_decomp::fspec::{ParameterPieces, PrototypePieces};
use kuna_decomp::funcdata::Funcdata;

use crate::engine::ConsoleProgram;
use crate::grammar::DataOrg;

/// One parsed assertion.  `raw` is the caller's own text, echoed back in the
/// [`Outcome`] so a report row is greppable against the command line that made
/// it.
#[derive(Clone, Debug, PartialEq)]
pub struct Directive {
    pub raw: String,
    pub body: Body,
}

/// The directive vocabulary (see the module docs for the text syntax).
#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    /// `function <start>[-<end>][=<name>]` — the `--define-function` alias.
    Function { start: u64, end: Option<u64>, name: Option<String> },
    /// `typedef <C declaration>` — intern a type so later directives can name it.
    Typedef { decl: String },
    /// `prototype <func> <C declaration>` — the function's signature.
    Prototype { func: String, decl: String },
    /// `data <addr> <C typedeclaration>` — a named, typed global.
    Data { addr: u64, decl: String },
    /// `param [<func>::]<i> <storage> <C typedeclaration>` — a locked input.
    Param { func: Option<String>, index: int4, storage: String, decl: String },
    /// `return [<func>::]<storage> <C typedeclaration>` — a locked return.
    Return { func: Option<String>, storage: String, decl: String },
    /// `comment [<func>::]<addr> <text>` — a comment at an instruction.
    Comment { func: Option<String>, addr: u64, text: String },
    /// `flow [<func>::]<addr> branch|call|callreturn|return` — the flow out of
    /// this instruction is not what kuna decided it was.
    Flow { func: Option<String>, addr: u64, kind: String },
    /// `name [<func>::]<symbol> <newname>` — rename a local.
    Name { func: Option<String>, symbol: String, newname: String },
    /// `type [<func>::]<symbol> <C type>` — retype a local.
    Type { func: Option<String>, symbol: String, decl: String },
    /// `readonly <addr>+<size>` — the bytes in this range never change at run
    /// time, so a load from it is its initialiser.
    Readonly { addr: u64, size: int4 },
    /// `volatile <addr>+<size>` — device memory: every access is a real access
    /// and two reads of one address are two reads.
    Volatile { addr: u64, size: int4 },
}

/// What became of one directive.  `status` is `applied` or `rejected`; a
/// rejection always carries a `detail` naming the reason.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub directive: String,
    pub kind: &'static str,
    pub phase: &'static str,
    pub subphase: &'static str,
    pub status: &'static str,
    pub detail: Option<String>,
}

impl Body {
    /// `(kind, phase, sub-phase)` — the phase-model coordinates a directive
    /// writes at, reported so an agent can correlate a rejection with
    /// `kuna catalog` / `docs/phases.md`.
    pub fn coords(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Body::Function { .. } => ("function", "P1", "function-boundary"),
            Body::Typedef { .. } => ("typedef", "P5", "type-propagation"),
            Body::Prototype { .. } => ("prototype", "P4", "prototype-source"),
            Body::Data { .. } => ("data", "P5", "const-pointer"),
            Body::Param { .. } => ("param", "P4", "prototype-source"),
            Body::Return { .. } => ("return", "P4", "prototype-source"),
            Body::Comment { .. } => ("comment", "P9", "external-refinement"),
            Body::Flow { .. } => ("flow", "P2", "flow-classification"),
            Body::Name { .. } => ("name", "P9", "naming-policy"),
            Body::Type { .. } => ("type", "P5", "type-propagation"),
            Body::Readonly { .. } => ("readonly", "P1", "code-data-partition"),
            Body::Volatile { .. } => ("volatile", "P1", "code-data-partition"),
        }
    }

    /// The function this directive names, when it names one.
    fn qualifier(&self) -> Option<&str> {
        match self {
            Body::Prototype { func, .. } => Some(func.as_str()),
            Body::Param { func, .. }
            | Body::Return { func, .. }
            | Body::Comment { func, .. }
            | Body::Flow { func, .. }
            | Body::Name { func, .. }
            | Body::Type { func, .. } => func.as_deref(),
            _ => None,
        }
    }

    /// Is this directive applied per-function (rather than once per program)?
    fn is_function_scoped(&self) -> bool {
        matches!(
            self,
            Body::Param { .. }
                | Body::Return { .. }
                | Body::Comment { .. }
                | Body::Flow { .. }
                | Body::Name { .. }
                | Body::Type { .. }
        )
    }

    /// The function a `param`/`return` directive names, when it names one.
    ///
    /// These two are the only function-scoped directives that mean something for
    /// a function this run does not decompile: they describe a signature, and a
    /// CALLER needs its callee's signature.  `comment`, `flow`, `name` and `type`
    /// describe the inside of one function body and have no cross-function
    /// reading at all.
    pub(crate) fn cross_function_prototype(&self) -> Option<String> {
        match self {
            Body::Param { func, .. } | Body::Return { func, .. } => func.clone(),
            _ => None,
        }
    }

    /// Is this directive one that can only be applied to an already-decompiled
    /// function (because the symbol it names does not exist before)?
    fn is_symbol_scoped(&self) -> bool {
        matches!(self, Body::Name { .. } | Body::Type { .. })
    }
}

impl Directive {
    fn applied(&self) -> Outcome {
        let (kind, phase, subphase) = self.body.coords();
        Outcome {
            directive: self.raw.clone(),
            kind,
            phase,
            subphase,
            status: "applied",
            detail: None,
        }
    }

    fn rejected(&self, detail: impl Into<String>) -> Outcome {
        let (kind, phase, subphase) = self.body.coords();
        Outcome {
            directive: self.raw.clone(),
            kind,
            phase,
            subphase,
            status: "rejected",
            detail: Some(detail.into()),
        }
    }
}

/// Does this directive bind to the function named `name`?
///
/// A qualified directive binds only to the function it names.  An unqualified
/// one binds to the single function under decompile, which is why `single` (the
/// run selected exactly one function) is required for it: see the module docs.
fn binds_to(body: &Body, name: &str, single: bool) -> bool {
    match body.qualifier() {
        Some(func) => func == name,
        None => single,
    }
}

/// The console-side data organisation of the loaded program.
fn data_org(prog: &ConsoleProgram) -> DataOrg {
    let (addr_size, word_size) = prog.arch().data_org();
    DataOrg { addr_size, word_size }
}

/// Build an [`Address`] in the program's default code space.
fn code_addr(prog: &ConsoleProgram, vma: u64) -> Option<Address> {
    let space = prog.arch().manage().get_default_code_space().cloned()?;
    Some(Address::new(space, vma))
}

/// Parse a storage token (`%RDI`, `[stack,-0x18,8]`, `s0x10`) through the
/// console's own machine-address grammar, so `--assert` and the console spell
/// storage the same way.
fn parse_storage(prog: &ConsoleProgram, tok: &str) -> Result<Address, String> {
    let mut s = crate::interface::CommandStream::new(tok);
    crate::ifacedecomp::parse_machaddr(prog, &mut s, false).map(|(addr, _size)| addr)
}

/// Apply the program-scoped directives (`function`, `typedef`, `prototype`,
/// `data`) and record an [`Outcome`] for each.
///
/// Called right after the analysis commit — a caller-declared fact outranks
/// discovery — and before any function is selected.  The function- and
/// symbol-scoped directives are left for the decompile loop and are given a
/// placeholder outcome that the loop overwrites; one that is never reached keeps
/// it, which is how "you asked about a function this run did not decompile"
/// becomes a report row instead of silence.
pub fn apply_program_scoped(prog: &mut ConsoleProgram) {
    let directives = prog.assertions().to_vec();
    for (i, directive) in directives.iter().enumerate() {
        // A QUALIFIED `param`/`return` is a statement about the named function's
        // prototype and holds whether or not this run decompiles it, so it is
        // applied here rather than waiting for a selection that may never name
        // it (`docs/re-needs/qualified-parameter-assertions-modify.md`).
        if let Some(func) = directive.body.cross_function_prototype() {
            let outcome = match apply_cross_function(prog, &func, &directive.body) {
                Ok(()) => directive.applied(),
                Err(detail) => directive.rejected(detail),
            };
            prog.set_assertion_outcome(i, outcome);
            continue;
        }
        if directive.body.is_function_scoped() {
            let outcome = directive.rejected(
                "no decompiled function matched this directive (name it as \
                 <func>::<operand>, or select the function)",
            );
            prog.set_assertion_outcome(i, outcome);
            continue;
        }
        let outcome = match apply_one_program_scoped(prog, &directive.body) {
            Ok(()) => directive.applied(),
            Err(detail) => directive.rejected(detail),
        };
        prog.set_assertion_outcome(i, outcome);
    }
}

fn apply_one_program_scoped(prog: &mut ConsoleProgram, body: &Body) -> Result<(), String> {
    match body {
        Body::Function { start, end, name } => {
            let addr = code_addr(prog, *start)
                .ok_or_else(|| "the loaded program has no default code space".to_string())?;
            let size = end.map(|e| e - *start).unwrap_or(0) as int4;
            prog.declare_function(addr, name.as_deref(), size)
                .map(|_| ())
                .map_err(|e| e.explain().to_string())
        }
        Body::Typedef { decl } => {
            let org = data_org(prog);
            let text = with_semicolon(decl);
            crate::grammar::parse_c(&text, prog.arch().types(), org, |_| Ok(()))
                .map_err(|e| e.explain().to_string())
        }
        Body::Prototype { func, decl } => apply_prototype(prog, func, decl),
        Body::Data { addr, decl } => apply_data(prog, *addr, decl),
        Body::Readonly { addr, size } => {
            paint_property(prog, *addr, *size, kuna_decomp::varnode::varnode_flags::readonly)
        }
        Body::Volatile { addr, size } => {
            paint_property(prog, *addr, *size, kuna_decomp::varnode::varnode_flags::volatil)
        }
        // Handled by the decompile loop.
        _ => Err("internal: not a program-scoped directive".into()),
    }
}

/// `readonly` / `volatile` — the in-process twin of the console's `readonly` and
/// `volatile` commands (`IfcReadonly`/`IfcVolatile`): OR one boolean Varnode
/// property over `[addr, addr+size)`.
///
/// The paint alone is not enough here.  `Scope::addMap` copies the range
/// property into a `SymbolEntry` as it maps it (`database.cc:1156-1158`) and the
/// per-function global snapshot then reads the SYMBOL's flags, so a property
/// stated after the loader's symbols are mapped — which is every in-process run,
/// since `bootstrap_from_object` reads them — is invisible at exactly the
/// addresses a caller is most likely to name.  Re-applying it to the symbols the
/// range covers is what makes `--assert 'readonly 0x404028+8'` and the console's
/// pre-`read symbols` ordering mean the same thing.
fn paint_property(
    prog: &mut ConsoleProgram,
    vma: u64,
    size: int4,
    flag: uint4,
) -> Result<(), String> {
    if size <= 0 {
        return Err("a range needs a size of at least one byte".into());
    }
    let first = code_addr(prog, vma)
        .ok_or_else(|| "the loaded program has no default code space".to_string())?;
    let space = first
        .get_space()
        .cloned()
        .ok_or_else(|| "the loaded program has no default code space".to_string())?;
    let end = vma
        .checked_add(size as u64)
        .ok_or_else(|| "the range wraps past the end of the address space".to_string())?;
    let last_open = Address::new(Rc::clone(&space), end);
    prog.arch_mut().symboltab.set_property_range(flag, &first, &last_open);
    repaint_covered_symbols(prog, &space, vma, end, flag);
    Ok(())
}

/// OR `flag` onto every global symbol whose storage overlaps `[first, end)`.
///
/// Walks the global scope's map by repeatedly asking for the next overlapping
/// entry and stepping past it, so the cost is one lookup per covered symbol
/// rather than one per byte.
fn repaint_covered_symbols(
    prog: &mut ConsoleProgram,
    space: &Rc<kuna_base::space::AddrSpace>,
    first: u64,
    end: u64,
    flag: uint4,
) {
    let Some(scope) = prog.arch().symboltab.get_global_scope() else {
        return;
    };
    let mut at = first;
    while at < end {
        let addr = Address::new(Rc::clone(space), at);
        let remaining = (end - at).min(int4::MAX as u64) as int4;
        let Some(eref) = prog.arch().symboltab.find_overlap(scope, &addr, remaining) else {
            break;
        };
        let (sym, past) = {
            let entry = prog.arch().symboltab.entry(scope, eref);
            (entry.symbol, entry.get_last().wrapping_add(1))
        };
        prog.arch_mut().symboltab.set_attribute(sym, flag);
        // A zero-width or backwards entry would spin; always make progress.
        at = past.max(at.wrapping_add(1));
    }
}

/// C declarations are parsed as a statement; accept the caller's text with or
/// without the terminating `;`.
fn with_semicolon(decl: &str) -> String {
    let decl = decl.trim();
    if decl.ends_with(';') {
        decl.to_string()
    } else {
        format!("{decl};")
    }
}

/// Which function a directive's `<func>` operand names, once the loaded program
/// has been consulted.
///
/// `<func>` is a NAME first — that is what every existing directive meant — and
/// an ENTRY ADDRESS second.  An agent working on a stripped or import-heavy
/// binary has the address long before it has a name it trusts, and the address
/// form used to be accepted and then discarded: nothing is called
/// `0x140003ddf`, so the by-name park landed on no symbol at all and the call
/// site kept its recovered signature
/// (`docs/re-needs/accepted-sqrt-prototype-still.md`).
///
/// Address is also the key the READ side already uses
/// (`ArchContext::callee_proto_pieces` looks a call spec's entry address up),
/// which is why the address form reaches a callee the name form cannot: a
/// PE import thunk and the IAT slot it jumps to are two FunctionSymbols with
/// the SAME name, and the by-name query answers with the slot while every call
/// in the program goes to the thunk.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ProtoTarget {
    /// Bind by name.  An unresolved name is still legal — it parks in the
    /// pending store, which is what lets `map prototype main …` precede the
    /// symbols it talks about.
    Named(String),
    /// Bind by entry address: the address to park at, and the resolved
    /// function's own display name (the pending store and the printer are both
    /// keyed by name, so the directive must not rename the function to its VMA).
    At(Address, String),
}

impl ProtoTarget {
    /// The name this prototype describes — the operand itself for the name
    /// form, the resolved function's display name for the address form.
    pub(crate) fn name(&self) -> &str {
        match self {
            ProtoTarget::Named(n) => n,
            ProtoTarget::At(_, n) => n,
        }
    }
}

/// Resolve a `<func>` operand against the loaded program (see [`ProtoTarget`]).
///
/// An explicitly `0x`-prefixed operand that no function starts at is an ERROR
/// rather than a silent park: `0x…` is not a C identifier, so such a directive
/// is provably inert and the caller deserves to hear it.  A bare hex token
/// (`140003ddf`) is ambiguous with a real identifier (`abc` is both), so it
/// takes the address path only when it resolves and no function of that name
/// exists, and never errors.
pub(crate) fn resolve_proto_target(
    prog: &ConsoleProgram,
    func: &str,
) -> Result<ProtoTarget, String> {
    let named = match prog.arch().symboltab.get_global_scope() {
        Some(g) => prog.arch().symboltab.query_function_by_name(g, func).is_some(),
        None => false,
    };
    if named {
        return Ok(ProtoTarget::Named(func.to_string()));
    }
    let hex = func.strip_prefix("0x").or_else(|| func.strip_prefix("0X"));
    let explicit = hex.is_some();
    let digits = hex.unwrap_or(func);
    let vma = if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_hexdigit()) {
        u64::from_str_radix(digits, 16).ok()
    } else {
        None
    };
    if let Some(vma) = vma {
        if let Some(addr) = code_addr(prog, vma) {
            if let Some(name) = prog.arch().symboltab.function_display_name_across_scopes(&addr) {
                return Ok(ProtoTarget::At(addr, name));
            }
        }
        if explicit {
            return Err(format!("no function starts at {func}"));
        }
    }
    Ok(ProtoTarget::Named(func.to_string()))
}

/// Park `pieces` on the target's `FunctionSymbol`, which is where a CALLER's
/// `ActionDefaultParams` reads a declared callee signature back from
/// (`ArchContext::callee_proto_pieces`).
///
/// `pieces.name` is set to the target's name first, so a directive that
/// renames — `prototype sub_1400055e0 void *sha256(…)` — still describes
/// `sub_1400055e0`, and an address form describes the function that lives
/// there rather than a function called `0x140003ddf`.
pub(crate) fn park_proto_pieces(
    prog: &mut ConsoleProgram,
    target: &ProtoTarget,
    pieces: &mut PrototypePieces,
) {
    pieces.name = target.name().to_string();
    match target {
        ProtoTarget::Named(name) => {
            prog.arch_mut().set_function_prototype_pieces(name, pieces.clone())
        }
        ProtoTarget::At(addr, _) => {
            prog.arch_mut().set_function_prototype_pieces_at(addr, pieces.clone())
        }
    }
}

/// The two things a full `prototype` directive does: park the pieces for the
/// callers, and lock the signature onto the symbol itself.
pub(crate) fn park_prototype(
    prog: &mut ConsoleProgram,
    target: &ProtoTarget,
    pieces: &mut PrototypePieces,
) {
    park_proto_pieces(prog, target, pieces);
    lock_prototype_on_symbol(prog, target, pieces);
}

/// `prototype <func> <C declaration>` — the in-process twin of `parse line
/// extern <decl>` (`IfcParseLine`'s `setPrototype` branch).
///
/// The parsed pieces are parked in two places, because two different consumers
/// read them: on the function's own `FunctionSymbol` (where a CALLER's
/// `ActionDefaultParams` finds them, so a call to this function renders typed),
/// and in the program's pending-prototype store (where the function's OWN
/// decompile finds them, since the drive rebuilds the `Funcdata` and the
/// symbol-table link does not survive that).
///
/// The `<func>` operand is authoritative over the name inside the declaration:
/// it says *which* function this signature describes, so `prototype sub_1400
/// int handler(char *)` binds to `sub_1400`.  It may be a name or an entry
/// address ([`resolve_proto_target`]).
fn apply_prototype(prog: &mut ConsoleProgram, func: &str, decl: &str) -> Result<(), String> {
    use std::cell::RefCell;
    let org = data_org(prog);
    let text = format!("extern {}", with_semicolon(decl));
    let captured: RefCell<Option<PrototypePieces>> = RefCell::new(None);
    crate::grammar::parse_c(&text, prog.arch().types(), org, |pieces| {
        *captured.borrow_mut() = Some(pieces);
        Ok(())
    })
    .map_err(|e| e.explain().to_string())?;
    let mut pieces = captured
        .into_inner()
        .ok_or_else(|| "not a function declaration".to_string())?;
    let target = resolve_proto_target(prog, func)?;
    park_prototype(prog, &target, &mut pieces);
    prog.set_pending_prototype(target.name(), pieces);
    Ok(())
}

/// Lock the parsed prototype onto the target's `FunctionSymbol` by retyping it
/// to the prototype-bearing `TypeCode` (C++ `Architecture::setPrototype`).  A
/// missing symbol is a no-op: the pending store above still carries the fact for
/// the function's own decompile.
pub(crate) fn lock_prototype_on_symbol(
    prog: &mut ConsoleProgram,
    target: &ProtoTarget,
    pieces: &PrototypePieces,
) {
    let arch = prog.arch_mut();
    let sid = match target {
        ProtoTarget::Named(name) => {
            let Some(scope) = arch.symboltab.get_global_scope() else {
                return;
            };
            match arch.symboltab.query_function_by_name(scope, name) {
                Some(sid) => sid,
                None => return,
            }
        }
        ProtoTarget::At(addr, _) => match arch.symboltab.find_function_across_scopes(addr) {
            Some((sid, _)) => sid,
            None => return,
        },
    };
    if let Ok(tc) = arch.types().get_type_code_proto(pieces) {
        let _ = arch.symboltab.retype_symbol(sid, tc);
    }
}

/// Apply a `param <func>::<i> <storage> <decl>` / `return <func>::<storage>
/// <decl>` to the prototype of the function it NAMES — the in-process twin of
/// the cross-function arm of the console's `map param` / `map return`.
///
/// The parked [`PrototypePieces`] is the one channel a callee's signature
/// reaches a CALLER through (`ActionDefaultParams` rebuilds each call site's
/// prototype from the pieces on the callee's `FunctionSymbol`), and it describes
/// types only — so the explicit storage rides in `input_storage` /
/// `output_storage`, which `FuncProto::set_pieces` re-applies after the
/// model-driven assignment.  Parking it also covers the case where the named
/// function IS the one under decompile: `function_seed` seeds `pending_proto`
/// from the same store.
fn apply_cross_function(
    prog: &mut ConsoleProgram,
    func: &str,
    body: &Body,
) -> Result<(), String> {
    use kuna_decomp::fspec::parameter_pieces_flags;
    let org = data_org(prog);
    let target = resolve_proto_target(prog, func)?;
    let mut pieces = prog.pending_prototype(target.name()).cloned().unwrap_or(PrototypePieces {
        name: target.name().to_string(),
        first_var_arg_slot: -1,
        ..Default::default()
    });
    match body {
        Body::Param { index, storage, decl, .. } => {
            if *index < 0 {
                return Err("a parameter index must not be negative".into());
            }
            let addr = parse_storage(prog, storage)?;
            let (ct, pname) = crate::grammar::parse_type(decl, prog.arch().types(), org)
                .map_err(|e| e.explain().to_string())?;
            // A slot no directive has named yet is `undefined<addr_size>` — what
            // the decompiler says about a value it was told nothing about — so
            // slots may be declared in any order.
            let (addr_size, _) = prog.arch().data_org();
            let filler = prog
                .arch()
                .types()
                .get_base(addr_size, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)
                .map_err(|e| e.explain().to_string())?;
            let slot = *index as usize;
            while pieces.intypes.len() <= slot {
                pieces.intypes.push(Rc::clone(&filler));
            }
            while pieces.innames.len() <= slot {
                pieces.innames.push(String::new());
            }
            pieces.intypes[slot] = ct.clone();
            pieces.innames[slot] = pname;
            let piece = ParameterPieces {
                addr,
                type_: Some(ct),
                flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
            };
            pieces.input_storage.retain(|(i, _)| *i != *index);
            pieces.input_storage.push((*index, piece));
        }
        Body::Return { storage, decl, .. } => {
            let addr = parse_storage(prog, storage)?;
            let (ct, _) = crate::grammar::parse_type(decl, prog.arch().types(), org)
                .map_err(|e| e.explain().to_string())?;
            pieces.output_storage = Some(ParameterPieces {
                addr,
                type_: Some(ct),
                flags: parameter_pieces_flags::TYPELOCK,
            });
        }
        _ => return Err("internal: not a cross-function prototype directive".into()),
    }
    park_proto_pieces(prog, &target, &mut pieces);
    // The symbol retype (what lets a by-value struct argument be split) needs a
    // return type; `param` alone never declares one, and the storage assignment
    // behind `getTypeCode` dereferences `outtype` unconditionally.
    if pieces.outtype.is_some() {
        lock_prototype_on_symbol(prog, &target, &pieces);
    }
    prog.set_pending_prototype(target.name(), pieces);
    Ok(())
}

/// `data <addr> <C typedeclaration>` — the in-process twin of the global branch
/// of `map address` (`IfcMapaddress`): a named, typed, name+type-locked global
/// mapped at `addr`, so a load through that address renders by name.
fn apply_data(prog: &mut ConsoleProgram, vma: u64, decl: &str) -> Result<(), String> {
    use kuna_decomp::varnode::varnode_flags;
    let org = data_org(prog);
    let addr = code_addr(prog, vma)
        .ok_or_else(|| "the loaded program has no default code space".to_string())?;
    let (ct, name) = crate::grammar::parse_type(decl, prog.arch().types(), org)
        .map_err(|e| e.explain().to_string())?;
    if name.is_empty() {
        return Err("a data declaration must name the symbol".into());
    }
    let inherit = prog.arch().symboltab.get_property(&addr);
    let flags = varnode_flags::namelock | varnode_flags::typelock | inherit;
    let num_spaces = prog.arch().manage().num_spaces() as int4;
    let arch = prog.arch_mut();
    let (scope, basename) = arch
        .symboltab
        .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)
        .map_err(|e| e.explain().to_string())?;
    let invalid = Address::new_invalid();
    let (sym, _entry) = arch
        .symboltab
        .add_symbol_mapped(scope, &basename, ct, &addr, &invalid)
        .map_err(|e| e.explain().to_string())?;
    arch.symboltab.set_attribute(sym, flags);
    Ok(())
}

/// The function-scoped facts a decompile of `name` must be seeded with.
///
/// `param` and `return` are consumed at flow time (they are the prototype the
/// function is decompiled against), so they cannot be applied afterwards;
/// `flow` is consumed at flow time too, and even earlier — the follower reads it
/// while it is still deciding which bytes belong to the function; `comment` is
/// program state and is written here too, since it is keyed by the owning
/// function's entry.
#[derive(Default)]
pub struct FunctionSeed {
    /// `map param` storage locks, in the drive's `mapped_params` shape.
    pub mapped_params: Vec<(int4, String, ParameterPieces)>,
    /// The prototype this function is decompiled against (`prototype`, plus the
    /// output storage a `return` directive locks).
    pub pending_proto: Option<PrototypePieces>,
    /// `override flow` facts, in the drive's `flow_overrides` shape: the flow
    /// type forced at each named instruction.
    pub flow_overrides: Vec<(Address, uint4)>,
}

/// Build the [`FunctionSeed`] for the function `name` entered at `entry`, and
/// record an outcome for every directive that bound to it.
///
/// `single` says the run selected exactly one function, which is what lets an
/// unqualified directive bind (see the module docs).
pub fn function_seed(
    prog: &mut ConsoleProgram,
    name: &str,
    entry: &Address,
    single: bool,
) -> FunctionSeed {
    let directives = prog.assertions().to_vec();
    let mut seed = FunctionSeed {
        pending_proto: prog.pending_prototype(name).cloned(),
        ..Default::default()
    };
    for (i, directive) in directives.iter().enumerate() {
        if directive.body.is_symbol_scoped() || !directive.body.is_function_scoped() {
            continue;
        }
        // Already applied program-scoped, onto the prototype of the function it
        // names; `pending_proto` above picks it back up when that function is the
        // one being decompiled.
        if directive.body.cross_function_prototype().is_some() {
            continue;
        }
        if !binds_to(&directive.body, name, single) {
            continue;
        }
        let outcome = match seed_one(prog, &directive.body, name, entry, &mut seed) {
            Ok(()) => directive.applied(),
            Err(detail) => directive.rejected(detail),
        };
        prog.set_assertion_outcome(i, outcome);
    }
    seed
}

fn seed_one(
    prog: &mut ConsoleProgram,
    body: &Body,
    name: &str,
    entry: &Address,
    seed: &mut FunctionSeed,
) -> Result<(), String> {
    use kuna_decomp::fspec::parameter_pieces_flags;
    let org = data_org(prog);
    match body {
        Body::Param { index, storage, decl, .. } => {
            let addr = parse_storage(prog, storage)?;
            let (ct, pname) = crate::grammar::parse_type(decl, prog.arch().types(), org)
                .map_err(|e| e.explain().to_string())?;
            let piece = ParameterPieces {
                addr,
                type_: Some(ct),
                flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
            };
            seed.mapped_params.push((*index, pname, piece));
            Ok(())
        }
        Body::Return { storage, decl, .. } => {
            let addr = parse_storage(prog, storage)?;
            let (ct, _) = crate::grammar::parse_type(decl, prog.arch().types(), org)
                .map_err(|e| e.explain().to_string())?;
            let piece = ParameterPieces {
                addr,
                type_: Some(ct),
                flags: parameter_pieces_flags::TYPELOCK,
            };
            // The output-only pieces `map return` parks: explicit storage, and
            // the declared type as the return type (without which the model's
            // `assignMap` has no output to work with — see
            // `FuncProto::seed_locked_from_pieces`).
            let proto = seed.pending_proto.get_or_insert_with(|| PrototypePieces {
                name: name.to_string(),
                first_var_arg_slot: -1,
                ..Default::default()
            });
            proto.outtype = piece.type_.clone();
            proto.output_storage = Some(piece);
            Ok(())
        }
        Body::Comment { addr, text, .. } => {
            let at = code_addr(prog, *addr)
                .ok_or_else(|| "the loaded program has no default code space".to_string())?;
            let arch = prog.arch_mut();
            let ctype = arch.print().instruction_comment_flags();
            arch.commentdb.add_comment(ctype, entry, &at, text);
            Ok(())
        }
        Body::Flow { addr, kind, .. } => {
            let type_ = kuna_decomp::overrides::Override::string_to_type(kind.as_bytes());
            if type_ == kuna_decomp::overrides::flow_type::NONE {
                return Err(format!(
                    "Bad override type: {kind} (want branch, call, callreturn or return)"
                ));
            }
            let at = code_addr(prog, *addr)
                .ok_or_else(|| "the loaded program has no default code space".to_string())?;
            seed.flow_overrides.push((at, type_));
            Ok(())
        }
        _ => Err("internal: not a function-scoped directive".into()),
    }
}

/// Are there symbol-scoped directives bound to `name`?  The second decompile
/// pass exists only for these, so every run without one keeps its current cost.
pub fn has_symbol_scoped(prog: &ConsoleProgram, name: &str, single: bool) -> bool {
    prog.assertions()
        .iter()
        .any(|d| d.body.is_symbol_scoped() && binds_to(&d.body, name, single))
}

/// Apply the symbol-scoped directives (`name`, `type`) to the first pass's
/// `Funcdata`, and return whether any of them took.
///
/// The caller re-decompiles when this returns `true`, carrying the mutated local
/// scope across as `mapped_symbols` — the console's `decompile` / `rename` /
/// `decompile` sequence, which is the only order in which these can work: the
/// local a directive names does not exist until a decompile has produced it.
///
/// Directives are applied in the order the caller gave them, so
/// `type v2 char[16]` followed by `name v2 credbuf` means what it reads like
/// (the reverse order would leave the second directive naming a symbol the first
/// had already renamed).
pub fn apply_symbol_scoped(
    prog: &mut ConsoleProgram,
    fd: &mut Funcdata,
    name: &str,
    single: bool,
) -> bool {
    let directives = prog.assertions().to_vec();
    let mut applied_any = false;
    for (i, directive) in directives.iter().enumerate() {
        if !directive.body.is_symbol_scoped() || !binds_to(&directive.body, name, single) {
            continue;
        }
        let outcome = match apply_one_symbol_scoped(prog, fd, &directive.body) {
            Ok(()) => {
                applied_any = true;
                directive.applied()
            }
            Err(detail) => directive.rejected(detail),
        };
        prog.set_assertion_outcome(i, outcome);
    }
    applied_any
}

fn apply_one_symbol_scoped(
    prog: &ConsoleProgram,
    fd: &mut Funcdata,
    body: &Body,
) -> Result<(), String> {
    use kuna_decomp::database::symbol_category;
    use kuna_decomp::varnode::varnode_flags;
    let (symbol, retype) = match body {
        Body::Name { symbol, .. } => (symbol.as_str(), None),
        Body::Type { symbol, decl, .. } => {
            let org = data_org(prog);
            let parsed = crate::grammar::parse_type(decl, prog.arch().types(), org)
                .map_err(|e| e.explain().to_string())?;
            (symbol.as_str(), Some(parsed))
        }
        _ => return Err("internal: not a symbol-scoped directive".into()),
    };
    let found = fd
        .get_scope_local()
        .map(|lm| lm.query_by_name(symbol))
        .unwrap_or_default();
    match found.len() {
        0 => return Err(format!("No symbol named: {symbol}")),
        1 => {}
        n => return Err(format!("More than one symbol named: {symbol} ({n})")),
    }
    let sym = found[0];
    // A parameter's storage is model-derived; locking its name or type locks the
    // input side of the prototype too (C++ `IfcRename`/`IfcRetype`).
    let is_param = fd
        .get_scope_local()
        .map(|lm| lm.symbol_category(sym) == symbol_category::FUNCTION_PARAMETER)
        .unwrap_or(false);
    if is_param {
        fd.get_func_proto_mut().set_input_lock(true);
    }
    let lm = fd
        .get_scope_local_mut()
        .ok_or_else(|| "Function has no local scope".to_string())?;
    match (body, retype) {
        (Body::Name { newname, .. }, _) => {
            lm.rename_symbol(sym, newname).map_err(|e| e.explain().to_string())?;
            lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
        }
        (Body::Type { .. }, Some((ct, newname))) => {
            lm.retype_symbol(sym, ct).map_err(|e| e.explain().to_string())?;
            lm.set_attribute(sym, varnode_flags::typelock);
            if !newname.is_empty() && newname != symbol {
                lm.rename_symbol(sym, &newname).map_err(|e| e.explain().to_string())?;
                lm.set_attribute(sym, varnode_flags::namelock);
            }
        }
        _ => unreachable!("symbol-scoped directive kinds are exhausted above"),
    }
    Ok(())
}

/// Does any directive assert a read-only range?
///
/// Painting a range read-only does nothing on its own: folding a read-only load
/// into its value is gated by the program-wide `readonly` option, which is
/// default-off.  So a `readonly` directive turns that option on for the run --
/// otherwise this plane's own failure mode (a directive that is accepted and
/// changes nothing) is exactly what it would ship.  Every surface applies it
/// BEFORE the caller's own options, so an explicit `--option readonly off` still
/// wins; asserting the range is a statement about the range, not an override of
/// a switch the caller set by hand.
pub fn implies_readonly_propagation(directives: &[Directive]) -> bool {
    directives.iter().any(|d| matches!(d.body, Body::Readonly { .. }))
}

/// The outcome of a directive nothing claimed -- the load never reached the
/// surface that applies it.  Reported rather than dropped, because a silently
/// ignored assertion is the failure mode this plane exists to end.
pub fn unclaimed(directive: &Directive) -> Outcome {
    directive.rejected("not reached by this run")
}

/// The mutated local scope of the first pass, in the shape the second decompile
/// re-seeds it from (`IfcDecompile` carries the same specs across its own IR
/// rebuild).
pub fn carried_symbols(fd: &Funcdata) -> Vec<(String, Rc<Datatype>, Address, uint4)> {
    fd.mapped_symbol_specs()
}
