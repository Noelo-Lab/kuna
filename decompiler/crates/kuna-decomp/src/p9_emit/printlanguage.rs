//! Port of `decompiler/cpp/printlanguage.{cc,hh}` — the `PrintLanguage` base
//! class: the RPN token-stack driver and the printc-shared scoping/atom
//! machinery shared by every high-level back-end.
//!
//! ## What this module owns vs. what it defers to
//!
//! `PrintLanguage` (printlanguage.hh:138) is a large class that is *mostly*
//! pure-virtual: the per-op-code `op*`/`emit*`/`push*` methods (printlanguage.hh:
//! 314-604) are overridden by `PrintC`/`PrintJava` (separate W8 items).  The
//! concrete base provides three self-contained things this module ports
//! faithfully:
//!
//!   1. **The operator-token model** ([`OpToken`], [`TokenType`]) and the
//!      printing-context enums ([`modifiers`], [`TagType`], [`NamespaceStrategy`])
//!      — the precedence/associativity/spacing data that drives parenthesis
//!      placement.
//!   2. **The RPN stack value types** ([`ReversePolish`], [`NodePending`],
//!      [`Atom`]) and the **parenthesization decision** [`parentheses`] —
//!      *the* print-fidelity-critical pure function: it reads only the token
//!      precedence/associativity/type and the current RPN stage, and decides
//!      whether a sub-expression is wrapped in `(`/`)`.  This is transcribed
//!      token-for-token (including the kuna GH-2786 adjacent-sign fix at
//!      printlanguage.cc:292-294).
//!   3. **The self-contained statics** [`most_natural_base`], [`format_binary`],
//!      [`unicode_needs_escape`], [`unnamed_field`], and the mod/scope-stack and
//!      [`PrintContext::set_integer_format`] helpers — none of which touch the
//!      emitter or IR.
//!
//! ## Boundary: the `Emit`/`EmitMarkup` driver and the IR/PrintC virtuals
//!
//! The token-*emitting* half of the driver — `pushOp`/`pushAtom`/`pushVn`/
//! `pushVnExplicit`/`recurse`/`emitOp`/`emitAtom`/`opBinary`/`opUnary` and the
//! `emitLineComment`/`setCommentDelimeter` plumbing — calls into the `Emit`
//! low-level emitter (`openGroup`/`openParen`/`tagOp`/`spaces`/...), which is
//! `decompiler/cpp/prettyprint.{cc,hh}` (a **separate W8 item**, currently a
//! stub), and reaches the data-flow graph through `Funcdata`-mediated Varnode/
//! PcodeOp accessors and the pure-virtual `push*`/`emit*` methods that `PrintC`
//! supplies (also a separate W8 item).  Those methods are **stub-noted** here
//! (`STUB(printc/prettyprint)`) rather than transcribed against stubs: their
//! bodies are byte-for-byte print logic that only becomes testable once the
//! `Emit` trait and the `PrintC` virtuals exist, and they are wired in the W9
//! m1-bodies closure together with the rest of `printc`.
//!
//! The `syntax_highlight` enum the [`Atom`] carries belongs to the `Emit`
//! driver (`prettyprint.hh:110`); a forward placeholder [`SyntaxHighlight`] is
//! provided here (the cross-wave stub idiom) so the [`Atom`]
//! layout is faithful before `prettyprint` lands.

use kuna_base::address::mostsigbit_set;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4, uintb};

/// Syntax-highlight class for an emitted token.
///
/// STUB(prettyprint): this is `EmitMarkup::syntax_highlight` (prettyprint.hh:
/// 110-122), owned by the (stub) `prettyprint` module.  A forward placeholder is
/// kept here — with the exact discriminants, which "must match constants in
/// ClangToken" — so [`Atom`]'s `highlight` field is laid out faithfully before
/// the real `Emit` driver lands; the W9 closure replaces it with the
/// `prettyprint` definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SyntaxHighlight {
    /// Keyword in the high-level language.
    keyword_color = 0,
    /// Comments.
    comment_color = 1,
    /// Data-type identifiers.
    type_color = 2,
    /// Function identifiers.
    funcname_color = 3,
    /// Local variable identifiers.
    var_color = 4,
    /// Constant values.
    const_color = 5,
    /// Function parameters.
    param_color = 6,
    /// Global variable identifiers.
    global_color = 7,
    /// Un-highlighted.
    no_color = 8,
    /// Indicates a warning or error state.
    error_color = 9,
    /// A token with special/highlighted meaning.
    special_color = 10,
}

/// The possible types of operator token (C++ `OpToken::tokentype`,
/// printlanguage.hh:87-94).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Binary operator form (printed between its inputs).
    Binary,
    /// Unary operator form (printed before its input).
    UnaryPrefix,
    /// Function or array operator form.
    Postsurround,
    /// Modifier form (like a cast operation).
    Presurround,
    /// No explicitly printed token.
    Space,
    /// Operation that isn't explicitly printed.
    HiddenFunction,
}

/// A token representing an operator in the high-level language (C++ `OpToken`,
/// printlanguage.hh:84-104).
///
/// The token knows how to print itself and its syntax information: precedence,
/// associativity, spacing, and how it groups its inputs.  The `negate` link (a
/// `OpToken *` in C++, pointing to the complementary token) is carried as a
/// `&'static OpToken` so the static token tables in `printc` can cross-link
/// without an allocation; the only consumer, the (stub) `op_binary`, reads it
/// behind the `negatetoken` modifier.
#[derive(Debug)]
pub struct OpToken {
    /// Printing characters for the token (C++ `print1`).
    pub print1: &'static str,
    /// (terminating) characters for the token (C++ `print2`).
    pub print2: &'static str,
    /// Additional elements consumed from the RPN stack when emitting (C++ `stage`).
    pub stage: int4,
    /// Precedence level (higher binds more tightly) (C++ `precedence`).
    pub precedence: int4,
    /// True if the operator is associative (C++ `associative`).
    pub associative: bool,
    /// The basic token type (C++ `type`).
    pub token_type: TokenType,
    /// Spaces to print around operator (C++ `spacing`).
    pub spacing: int4,
    /// Spaces to indent if we break here (C++ `bump`).
    pub bump: int4,
    /// The token representing the negation of this token (C++ `negate`).
    /// `None` if the operator has no complement.
    pub negate: Option<&'static OpToken>,
    /// (kuna outlang) The operator's result must be parenthesised when it is an
    /// operand of a `<`/`>`-family operator, because the language would otherwise
    /// re-parse the boundary. Rust's `as` is the case this exists for: in
    /// `x as i32 < 5` the `i32 <` opens generic arguments and the expression does
    /// not parse. Precedence cannot express it -- `as` genuinely binds tighter
    /// than `<`, so no ordering of the table both parenthesises here and leaves
    /// `(a + b) as T` correct. `false` for every C token, so the C path is
    /// untouched.
    pub paren_before_angle: bool,
}

/// Context-sensitive modifiers to how tokens get emitted (C++
/// `PrintLanguage::modifiers`, printlanguage.hh:144-161).  These are OR-combined
/// into the active `mods` word; the bit values are load-bearing.
pub mod modifiers {
    use super::uint4;
    /// Force printing of hex.
    pub const FORCE_HEX: uint4 = 1;
    /// Force printing of dec.
    pub const FORCE_DEC: uint4 = 2;
    /// Decide on most aesthetic form.
    pub const BESTFIT: uint4 = 4;
    /// Force scientific notation for floats.
    pub const FORCE_SCINOTE: uint4 = 8;
    /// Force '*' notation for pointers.
    pub const FORCE_POINTER: uint4 = 0x10;
    /// Hide pointer deref for load with other ops.
    pub const PRINT_LOAD_VALUE: uint4 = 0x20;
    /// Hide pointer deref for store with other ops.
    pub const PRINT_STORE_VALUE: uint4 = 0x40;
    /// Do not print branch instruction.
    pub const NO_BRANCH: uint4 = 0x80;
    /// Print only the branch instruction.
    pub const ONLY_BRANCH: uint4 = 0x100;
    /// Statements within condition.
    pub const COMMA_SEPARATE: uint4 = 0x200;
    /// Do not print block structure.
    pub const FLAT: uint4 = 0x400;
    /// Print the false branch (for flat).
    pub const FALSEBRANCH: uint4 = 0x800;
    /// Fall-thru no longer exists.
    pub const NOFALLTHRU: uint4 = 0x1000;
    /// Print the token representing the negation of current token.
    pub const NEGATETOKEN: uint4 = 0x2000;
    /// Do not print the 'this' parameter in argument lists.
    pub const HIDE_THISPARAM: uint4 = 0x4000;
    /// The current block may need to surround itself with additional braces.
    pub const PENDING_BRACE: uint4 = 0x8000;
    /// (kuna) The expression being pushed is consumed as a BOOLEAN (an
    /// if/while/for/ternary condition or a `&&`/`||`/`!` operand), so a
    /// comparison against zero may render in its truthy form (`x` / `!x`)
    /// under `option truthycond` (DIV-36).  Scoped off across any
    /// non-boolean-preserving operator by `op_push_ir`.
    pub const CONDITION_CONTEXT: uint4 = 0x10000;
}

/// Possible types of [`Atom`] (C++ `PrintLanguage::tagtype`,
/// printlanguage.hh:163-173).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    /// Emit atom as syntax.
    Syntax,
    /// Emit atom as variable.
    VarToken,
    /// Emit atom as function name.
    FuncToken,
    /// Emit atom as operator.
    OpToken,
    /// Emit atom as type.
    TypeToken,
    /// Emit atom as structure field.
    FieldToken,
    /// Emit atom as structure bitfield.
    BitFieldToken,
    /// Emit atom as a case label.
    CaseToken,
    /// For anonymous types.
    BlankToken,
}

/// Strategies for displaying namespace tokens (C++
/// `PrintLanguage::namespace_strategy`, printlanguage.hh:176-180).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceStrategy {
    /// (default) Print just enough namespace info to fully resolve symbol.
    MinimalNamespaces = 0,
    /// Never print namespace information.
    NoNamespaces = 1,
    /// Always print all namespace information.
    AllNamespaces = 2,
}

/// An entry on the reverse polish notation (RPN) stack (C++
/// `PrintLanguage::ReversePolish`, printlanguage.hh:183-190).
///
/// `op` is carried as an opaque index (a `PcodeOp *` in C++; per ADR 0001 the IR
/// is `Funcdata`-owned) and `id`/`id2` are the emitter group ids
/// (`Emit::openGroup`/`openParen` return values, filled by the printc driver).
#[derive(Debug, Clone)]
pub struct ReversePolish {
    /// The operator token (C++ `tok`, an `OpToken *`).
    pub tok: &'static OpToken,
    /// The current stage of printing for the operator (C++ `visited`).
    pub visited: int4,
    /// True if parentheses are required (C++ `paren`).
    pub paren: bool,
    /// The PcodeOp associated with the token (C++ `op`).
    /// STUB(printc): opaque op index; `None` where the C++ holds a null op.
    pub op: Option<usize>,
    /// The id of the token group this belongs to (C++ `id`).
    pub id: int4,
    /// The id of the token group this surrounds (C++ `id2`, `mutable`).
    pub id2: int4,
}

/// A pending data-flow node waiting to be placed on the RPN stack (C++
/// `PrintLanguage::NodePending`, printlanguage.hh:196-204).
#[derive(Debug, Clone)]
pub struct NodePending {
    /// The implied Varnode (C++ `vn`).  STUB(printc): opaque varnode index.
    pub vn: usize,
    /// The single operator consuming the implied Varnode (C++ `op`).
    pub op: usize,
    /// Printing modifications to enforce on the expression (C++ `vnmod`).
    pub vnmod: uint4,
}

impl NodePending {
    /// Construct a pending data-flow node (C++ `NodePending(v,o,m)`).
    pub fn new(vn: usize, op: usize, vnmod: uint4) -> NodePending {
        NodePending { vn, op, vnmod }
    }
}

/// The meta-data a non-`casetoken` [`Atom`] carries beside its name (C++ union
/// `Atom::ptr_second`, printlanguage.hh:216-221).  Carried as a Rust enum (a
/// tagged union) so the right variant is statically tracked.  The pointer
/// members are opaque indices (STUB(printc)); `IntValue` is a real `uintb`.
#[derive(Debug, Clone, Copy)]
pub enum AtomData {
    /// No associated data-flow annotation.
    None,
    /// A Varnode associated with the token (C++ `vn`).
    Vn(usize),
    /// A function associated with the token (C++ `fd`).
    Fd(usize),
    /// A type associated with the token (C++ `ct`).
    Ct(usize),
    /// An integer value associated with a `casetoken` (C++ `intValue`).
    IntValue(uintb),
}

/// A single non-operator token emitted by the decompiler (C++
/// `PrintLanguage::Atom`, printlanguage.hh:211-259).
///
/// Plays the role of a variable token on the RPN stack.  The C++ `Atom` is a
/// family of constructors selecting which union member of `ptr_second` is set;
/// here the constructors set the [`AtomData`] variant explicitly.  `name` is a
/// `const string &` in C++ (a borrow of a long-lived string); carried here as an
/// owned `String` for the value-type RPN stack.
#[derive(Debug, Clone)]
pub struct Atom {
    /// The actual printed characters of the token (C++ `name`).
    pub name: String,
    /// The type of Atom (C++ `type`).
    pub tag: TagType,
    /// The highlighting to use when emitting (C++ `highlight`).
    pub highlight: SyntaxHighlight,
    /// A p-code operation associated with the token (C++ `op`).  STUB(printc).
    pub op: Option<usize>,
    /// Other meta-data associated with the token (C++ union `ptr_second`).
    pub data: AtomData,
    /// The offset (within the parent structure) for a field token (C++ `offset`).
    pub offset: int4,
}

impl Atom {
    /// Construct a token with no associated data-flow annotations
    /// (C++ `Atom(nm,t,hl)`, printlanguage.hh:225-226).
    pub fn syntax(name: impl Into<String>, tag: TagType, hl: SyntaxHighlight) -> Atom {
        Atom { name: name.into(), tag, highlight: hl, op: None, data: AtomData::None, offset: 0 }
    }

    /// Construct a token for a data-type name (C++ `Atom(nm,t,hl,c)`,
    /// printlanguage.hh:229-230).
    pub fn typed(name: impl Into<String>, tag: TagType, hl: SyntaxHighlight, ct: usize) -> Atom {
        Atom { name: name.into(), tag, highlight: hl, op: None, data: AtomData::Ct(ct), offset: 0 }
    }

    /// Construct a token for a field name (C++ `Atom(nm,t,hl,c,off,o)`,
    /// printlanguage.hh:233-234).
    pub fn field(
        name: impl Into<String>,
        tag: TagType,
        hl: SyntaxHighlight,
        ct: usize,
        off: int4,
        op: usize,
    ) -> Atom {
        Atom {
            name: name.into(),
            tag,
            highlight: hl,
            op: Some(op),
            data: AtomData::Ct(ct),
            offset: off,
        }
    }

    /// Construct a token with an associated PcodeOp (C++ `Atom(nm,t,hl,o)`,
    /// printlanguage.hh:237-238).
    pub fn with_op(name: impl Into<String>, tag: TagType, hl: SyntaxHighlight, op: usize) -> Atom {
        Atom {
            name: name.into(),
            tag,
            highlight: hl,
            op: Some(op),
            data: AtomData::None,
            offset: 0,
        }
    }

    /// Construct a token with an associated PcodeOp and Varnode
    /// (C++ `Atom(nm,t,hl,o,v)`, printlanguage.hh:241-242).
    pub fn with_op_vn(
        name: impl Into<String>,
        tag: TagType,
        hl: SyntaxHighlight,
        op: usize,
        vn: usize,
    ) -> Atom {
        Atom {
            name: name.into(),
            tag,
            highlight: hl,
            op: Some(op),
            data: AtomData::Vn(vn),
            offset: 0,
        }
    }

    /// Construct a token for a function name (C++ `Atom(nm,t,hl,o,f)`,
    /// printlanguage.hh:245-246).
    pub fn func(
        name: impl Into<String>,
        tag: TagType,
        hl: SyntaxHighlight,
        op: usize,
        fd: usize,
    ) -> Atom {
        Atom {
            name: name.into(),
            tag,
            highlight: hl,
            op: Some(op),
            data: AtomData::Fd(fd),
            offset: 0,
        }
    }

    /// Construct a token with an associated PcodeOp, Varnode, and constant value
    /// (C++ `Atom(nm,t,hl,o,v,intValue)`, printlanguage.hh:249-258).  A
    /// `casetoken` stores `intValue`; any other tag stores the Varnode.
    pub fn with_value(
        name: impl Into<String>,
        tag: TagType,
        hl: SyntaxHighlight,
        op: usize,
        vn: usize,
        int_value: uintb,
    ) -> Atom {
        let data =
            if tag == TagType::CaseToken { AtomData::IntValue(int_value) } else { AtomData::Vn(vn) };
        Atom { name: name.into(), tag, highlight: hl, op: Some(op), data, offset: 0 }
    }
}

/// The `"("` token (C++ `PrintLanguage::OPEN_PAREN`).
pub const OPEN_PAREN: &str = "(";
/// The `")"` token (C++ `PrintLanguage::CLOSE_PAREN`).
pub const CLOSE_PAREN: &str = ")";

/// Comment-type bits this base touches (C++ `Comment::*`, comment.hh).  Only the
/// bits `resetDefaultsInternal` ORs together are needed here.  STUB(comment): the
/// full set lives in the (stub) `comment` module.
pub mod comment_type {
    use super::uint4;
    /// `Comment::warning` — an in-body warning comment.
    pub const WARNING: uint4 = 2;
    /// `Comment::user2` — a user instruction comment.
    pub const USER2: uint4 = 4;
    /// `Comment::header` — a function-header comment.
    pub const HEADER: uint4 = 8;
    /// `Comment::warningheader` — a warning in the function header.
    pub const WARNINGHEADER: uint4 = 16;
}

/// The printing-context state shared by every back-end: the printing-modification
/// stack, the symbol-scope stack, and the active mods word.
///
/// This is the slice of `PrintLanguage`'s member state (printlanguage.hh:262-279)
/// that is *not* coupled to the `Emit` driver or the IR — the mod/scope stacks
/// and the comment/namespace defaults.  The RPN/nodepend stacks and the `emit`
/// pointer (`revpol`/`nodepend`/`pending`/`emit`) live with the printc driver,
/// since every method that touches them also drives `Emit` or the IR.
///
/// Scopes are carried as opaque indices (a `const Scope *` in C++; the symbol
/// table is W4).  STUB(printc).
#[derive(Debug, Clone)]
pub struct PrintContext {
    /// Printing modification stack (C++ `modstack`).
    modstack: Vec<uint4>,
    /// The symbol scope stack (C++ `scopestack`).  STUB(printc): opaque scope ids.
    scopestack: Vec<usize>,
    /// The current symbol scope (C++ `curscope`).  `None` when the stack is empty.
    curscope: Option<usize>,
    /// Currently active printing modifications (C++ `mods`).
    mods: uint4,
    /// Number of characters a comment line should be indented (C++ `line_commentindent`).
    line_commentindent: int4,
    /// Type of instruction comments to display (C++ `instr_comment_type`).
    instr_comment_type: uint4,
    /// Type of header comments to display (C++ `head_comment_type`).
    head_comment_type: uint4,
    /// How namespace tokens are displayed (C++ `namespc_strategy`).
    namespc_strategy: NamespaceStrategy,
}

impl Default for PrintContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PrintContext {
    /// Construct with the C++ `resetDefaultsInternal` defaults
    /// (printlanguage.cc:582-590), the same values the `PrintLanguage`
    /// constructor establishes.
    pub fn new() -> PrintContext {
        let mut ctx = PrintContext {
            modstack: Vec::new(),
            scopestack: Vec::new(),
            curscope: None,
            mods: 0,
            line_commentindent: 0,
            instr_comment_type: 0,
            head_comment_type: 0,
            namespc_strategy: NamespaceStrategy::MinimalNamespaces,
        };
        ctx.reset_defaults_internal();
        ctx
    }

    /// C++ `PrintLanguage::resetDefaultsInternal` (printlanguage.cc:582-590).
    pub fn reset_defaults_internal(&mut self) {
        self.mods = 0;
        self.head_comment_type = comment_type::HEADER | comment_type::WARNINGHEADER;
        self.line_commentindent = 20;
        self.namespc_strategy = NamespaceStrategy::MinimalNamespaces;
        self.instr_comment_type = comment_type::USER2 | comment_type::WARNING;
    }

    // -- Routines consistent across languages (printlanguage.hh:285-291) -----

    /// C++ `isSet(m)` — is the given printing modification active?
    #[inline]
    pub fn is_set(&self, m: uint4) -> bool {
        (self.mods & m) != 0
    }

    /// C++ `pushScope(sc)` — push a new symbol scope.
    pub fn push_scope(&mut self, sc: usize) {
        self.scopestack.push(sc);
        self.curscope = Some(sc);
    }

    /// C++ `popScope` (printlanguage.cc:113-121) — pop to the previous scope.
    pub fn pop_scope(&mut self) {
        self.scopestack.pop();
        self.curscope = self.scopestack.last().copied();
    }

    /// C++ `pushMod` — push current printing modifications to the stack.
    pub fn push_mod(&mut self) {
        self.modstack.push(self.mods);
    }

    /// C++ `popMod` — pop to the previous printing modifications.
    pub fn pop_mod(&mut self) {
        self.mods = self.modstack.pop().expect("popMod on empty modstack");
    }

    /// C++ `setMod(m)` — activate the given printing modification.
    pub fn set_mod(&mut self, m: uint4) {
        self.mods |= m;
    }

    /// C++ `unsetMod(m)` — deactivate the given printing modification.
    pub fn unset_mod(&mut self, m: uint4) {
        self.mods &= !m;
    }

    /// The currently active mods word (C++ `mods`).
    #[inline]
    pub fn mods(&self) -> uint4 {
        self.mods
    }

    /// Set the active mods word directly (used by the printc driver's
    /// `recurse`/`clear`, which save and restore `mods`).
    #[inline]
    pub fn set_mods(&mut self, m: uint4) {
        self.mods = m;
    }

    /// The current symbol scope (C++ `curscope`).
    #[inline]
    pub fn cur_scope(&self) -> Option<usize> {
        self.curscope
    }

    /// The instruction-comment type (C++ `getInstructionComment`).
    #[inline]
    pub fn instruction_comment(&self) -> uint4 {
        self.instr_comment_type
    }

    /// Set the instruction-comment type (C++ `setInstructionComment`).
    #[inline]
    pub fn set_instruction_comment(&mut self, val: uint4) {
        self.instr_comment_type = val;
    }

    /// The header-comment type (C++ `getHeaderComment`).
    #[inline]
    pub fn header_comment(&self) -> uint4 {
        self.head_comment_type
    }

    /// Set the header-comment type (C++ `setHeaderComment`).
    #[inline]
    pub fn set_header_comment(&mut self, val: uint4) {
        self.head_comment_type = val;
    }

    /// Set how namespace tokens are displayed (C++ `setNamespaceStrategy`).
    #[inline]
    pub fn set_namespace_strategy(&mut self, strat: NamespaceStrategy) {
        self.namespc_strategy = strat;
    }

    /// How namespace tokens are displayed (C++ `namespc_strategy`).
    #[inline]
    pub fn namespace_strategy(&self) -> NamespaceStrategy {
        self.namespc_strategy
    }

    /// The comment-line indent (C++ `line_commentindent`).
    #[inline]
    pub fn line_comment_indent(&self) -> int4 {
        self.line_commentindent
    }

    /// C++ `setLineCommentIndent` (printlanguage.cc:84-90), minus the
    /// `emit->getMaxLineSize()` upper-bound check (that bound lives on the
    /// `Emit` driver, prettyprint).  `max_line_size` is supplied by the caller
    /// that owns the emitter.
    pub fn set_line_comment_indent(&mut self, val: int4, max_line_size: int4) -> KunaResult<()> {
        if val < 0 || val >= max_line_size {
            return Err(KunaError::lowlevel("Bad comment indent value"));
        }
        self.line_commentindent = val;
        Ok(())
    }

    /// C++ `setIntegerFormat` (printlanguage.cc:705-719) — set the default
    /// integer display format, toggling the `force_hex`/`force_dec` mods.
    pub fn set_integer_format(&mut self, nm: &str) -> KunaResult<()> {
        // Prefix match on the first 3 chars, as the C++ does.
        let m = if nm.starts_with("hex") {
            modifiers::FORCE_HEX
        } else if nm.starts_with("dec") {
            modifiers::FORCE_DEC
        } else if nm.starts_with("best") {
            0
        } else {
            return Err(KunaError::lowlevel(format!("Unknown integer format option: {nm}")));
        };
        // Turn off any pre-existing force, then set the new one.
        self.mods &= !(modifiers::FORCE_HEX | modifiers::FORCE_DEC);
        self.mods |= m;
        Ok(())
    }

    /// C++ `setFlat(val)` (printlanguage.cc:669-676) — toggle the `flat` mod.
    pub fn set_flat(&mut self, val: bool) {
        if val {
            self.mods |= modifiers::FLAT;
        } else {
            self.mods &= !modifiers::FLAT;
        }
    }
}

/// C++ `PrintLanguage::parentheses` (printlanguage.cc:270-327).
///
/// The token at the top of the RPN stack (`top`) is being emitted; this decides
/// whether its input expression — ending with operator token `op2` — needs to be
/// surrounded by parentheses to convey the proper data-flow meaning.  This is a
/// **pure** function of the two tokens' type/precedence/associativity and the
/// top token's current `visited` stage (and, for `hiddenfunction`, the
/// previous token on the stack), so it is transcribed exactly here including the
/// kuna GH-2786 adjacent-sign fix.
///
/// `prev_token` is `revpol[size-2].tok` (the token below `top`), needed only for
/// the `HiddenFunction` arm; pass `None` when the stack has only one entry.
pub fn parentheses(top: &ReversePolish, op2: &OpToken, prev_token: Option<&OpToken>) -> bool {
    let top_token = top.tok;
    let stage = top.visited;
    // (kuna outlang) See `OpToken::paren_before_angle`.
    if op2.paren_before_angle
        && top_token.token_type == TokenType::Binary
        && (top_token.print1.starts_with('<') || top_token.print1.starts_with('>'))
    {
        return true;
    }
    match top_token.token_type {
        TokenType::Space | TokenType::Binary => {
            if top_token.precedence > op2.precedence {
                return true;
            }
            if top_token.precedence < op2.precedence {
                return false;
            }
            // C++ `topToken == op2` — pointer identity of the static singletons.
            if top_token.associative && std::ptr::eq(top_token, op2) {
                return false;
            }
            // If operators are adjacent, the operator printed first must be
            // evaluated first.  Here op2 must be evaluated first, so check if it
            // is printed first (in the first stage of binary).
            if op2.token_type == TokenType::Postsurround && stage == 0 {
                return false;
            }
            true
        }
        TokenType::UnaryPrefix => {
            if top_token.precedence > op2.precedence {
                return true;
            }
            if top_token.precedence < op2.precedence {
                return false;
            }
            // (kuna) GH-2786: adjacent identical sign tokens would merge into the
            // --/++ operator (printlanguage.cc:292-294).
            if op2.token_type == TokenType::UnaryPrefix
                && top_token.print1 == op2.print1
                && (top_token.print1 == "-" || top_token.print1 == "+")
            {
                return true;
            }
            if op2.token_type == TokenType::UnaryPrefix
                || op2.token_type == TokenType::Presurround
            {
                return false;
            }
            true
        }
        TokenType::Postsurround => {
            if stage == 1 {
                return false; // Inside the surround
            }
            if top_token.precedence > op2.precedence {
                return true;
            }
            if top_token.precedence < op2.precedence {
                return false;
            }
            // Equal precedences: this postsurround comes after, so op2 being
            // first doesn't need parens.
            if op2.token_type == TokenType::Postsurround || op2.token_type == TokenType::Binary {
                return false;
            }
            true
        }
        TokenType::Presurround => {
            if stage == 0 {
                return false; // Inside the surround
            }
            if top_token.precedence > op2.precedence {
                return true;
            }
            if top_token.precedence < op2.precedence {
                return false;
            }
            if op2.token_type == TokenType::UnaryPrefix
                || op2.token_type == TokenType::Presurround
            {
                return false;
            }
            true
        }
        TokenType::HiddenFunction => {
            // If there is an unresolved previous token.
            if stage == 0 {
                if let Some(prev) = prev_token {
                    // New token is printed next to the previous token.
                    if prev.token_type != TokenType::Binary
                        && prev.token_type != TokenType::UnaryPrefix
                    {
                        return false;
                    }
                    if prev.precedence < op2.precedence {
                        return false;
                    }
                    // If precedence is equal, make sure we don't treat two tokens
                    // as associative, i.e. we should have parentheses.
                }
            }
            true
        }
    }
}

/// C++ `PrintLanguage::unicodeNeedsEscape` (printlanguage.cc:418-494).
///
/// Separate unicode characters that can be clearly emitted in a source-code
/// string from characters better represented with an escape sequence.  Pure
/// function of the codepoint; transcribed range-for-range.
pub fn unicode_needs_escape(codepoint: int4) -> bool {
    if codepoint < 0x20 {
        // C0 Control characters
        return true;
    }
    if codepoint < 0x7F {
        // Printable ASCII
        match codepoint {
            92 | 0x22 | 0x27 => return true, // back-slash, '"', '\''
            _ => {}
        }
        return false;
    }
    if codepoint < 0x100 {
        if codepoint > 0xa0 {
            // Printable codepoints A1-FF
            return false;
        }
        return true; // Delete + C1 Control characters
    }
    if codepoint >= 0x2fa20 {
        // Up to last currently defined language
        return true;
    }
    if codepoint < 0x2000 {
        if (0x180b..=0x180e).contains(&codepoint) {
            return true; // Mongolian separators
        }
        if codepoint == 0x61c {
            return true; // arabic letter mark
        }
        if codepoint == 0x1680 {
            return true; // ogham space mark
        }
        return false;
    }
    if codepoint < 0x3000 {
        if codepoint < 0x2010 {
            return true; // white space and separators
        }
        if (0x2028..=0x202f).contains(&codepoint) {
            return true; // white space and separators
        }
        if codepoint == 0x205f || codepoint == 0x2060 {
            return true; // white space and word joiner
        }
        if (0x2066..=0x206f).contains(&codepoint) {
            return true; // bidirectional markers
        }
        return false;
    }
    if codepoint < 0xe000 {
        if codepoint == 0x3000 {
            return true; // ideographic space
        }
        if codepoint >= 0xd7fc {
            // D7FC-D7FF unassigned; D800-DFFF surrogates (illegal).
            return true;
        }
        return false;
    }
    if codepoint < 0xf900 {
        return true; // private use
    }
    if (0xfe00..=0xfe0f).contains(&codepoint) {
        return true; // variation selectors
    }
    if codepoint == 0xfeff {
        return true; // zero width non-breaking space
    }
    if (0xfff0..=0xffff).contains(&codepoint) {
        if codepoint == 0xfffc || codepoint == 0xfffd {
            return false;
        }
        return true; // interlinear specials
    }
    false
}

/// C++ `PrintLanguage::unnamedField` (printlanguage.cc:726-732).
///
/// An artificial field name `_<off>_<size>_` for a value extracted from a
/// structured data-type when the natural name is unavailable.
pub fn unnamed_field(off: int4, size: int4) -> String {
    format!("_{off}_{size}_")
}

/// C++ `PrintLanguage::mostNaturalBase` (printlanguage.cc:738-795).
///
/// Count `'0'`/`'9'` digits base 10 and `'0'`/`'f'` digits base 16; the higher
/// count is the preferred base.  Returns 10 for decimal, 16 for hexadecimal.
/// Transcribed exactly (`uintb` arithmetic, the `countdec` switch tie-breaks).
pub fn most_natural_base(val: uintb) -> int4 {
    let mut countdec: int4 = 0; // Count 0's and 9's

    let mut tmp: uintb = val;
    if tmp == 0 {
        return 10;
    }
    let setdig = tmp % 10;
    if setdig == 0 || setdig == 9 {
        countdec += 1;
        tmp /= 10;
        while tmp != 0 {
            let dig = tmp % 10;
            if dig == setdig {
                countdec += 1;
            } else {
                break;
            }
            tmp /= 10;
        }
    }
    match countdec {
        0 => return 16,
        1 => {
            if tmp > 1 || setdig == 9 {
                return 16;
            }
        }
        2 => {
            if tmp > 10 {
                return 16;
            }
        }
        3 | 4 => {
            if tmp > 100 {
                return 16;
            }
        }
        _ => {
            if tmp > 1000 {
                return 16;
            }
        }
    }

    let mut counthex: int4 = 0; // Count 0's and f's

    tmp = val;
    let setdig = tmp & 0xf;
    if setdig == 0 || setdig == 0xf {
        counthex += 1;
        tmp >>= 4;
        while tmp != 0 {
            let dig = tmp & 0xf;
            if dig == setdig {
                counthex += 1;
            } else {
                break;
            }
            tmp >>= 4;
        }
    }

    if countdec > counthex {
        10
    } else {
        16
    }
}

/// C++ `PrintLanguage::formatBinary` (printlanguage.cc:800-825).
///
/// Append a string of `'0'`/`'1'` characters representing `val`, padded to the
/// next power-of-two width above its most significant set bit (8/16/32/64 bits).
pub fn format_binary(s: &mut String, val: uintb) {
    let pos = mostsigbit_set(val);
    let pos = if pos < 0 {
        s.push('0');
        return;
    } else if pos <= 7 {
        7
    } else if pos <= 15 {
        15
    } else if pos <= 31 {
        31
    } else {
        63
    };
    let mut mask: uintb = 1;
    mask <<= pos;
    while mask != 0 {
        if (mask & val) != 0 {
            s.push('1');
        } else {
            s.push('0');
        }
        mask >>= 1;
    }
}

#[cfg(test)]
mod tests;
