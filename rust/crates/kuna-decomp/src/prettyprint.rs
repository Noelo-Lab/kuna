//! Port of `decompiler/cpp/prettyprint.{hh,cc}` (W8, item `w8-s9-prettyprint`)
//! — the Decompiler's source-code emitting layer: the [`Emit`] interface, the
//! trivial [`EmitNoMarkup`] back-end, the XML-markup [`EmitMarkup`] back-end,
//! the [`TokenSplit`] stream element, and the Oppen-style line-breaking engine
//! [`EmitPrettyPrint`] that `PrintLanguage`/`PrintC` drive.
//!
//! ## What this module is responsible for
//!
//! `PrintC` (later wave) turns the AST into a stream of *tag* calls
//! (`tagVariable`, `tagOp`, `print`, `spaces`, `tagLine`, `openParen`, …)
//! against an [`Emit`].  The default emitter `PrintLanguage` installs is an
//! [`EmitPrettyPrint`] whose low-level back-end is an [`EmitNoMarkup`]
//! (`print C`) or an [`EmitMarkup`] (`print C xml`, after `setMarkup(true)`).
//!
//! The **byte-for-byte** C output that the M2/M3 datatests `stringmatch` on is
//! produced by [`EmitPrettyPrint`] + [`EmitNoMarkup`].  The line breaks and
//! indent that the pretty printer inserts are therefore *load-bearing*: the
//! `scan`/`advanceleft`/`print`/`overflow` algorithm, the [`CircularQueue`]
//! indices, the `999999` "won't fit" sentinel, the `< 10` short-break
//! heuristic, and the `indentstack` arithmetic are all transcribed verbatim
//! from `prettyprint.cc`.
//!
//! ## ADR realization / divergences from a literal pointer transcription
//!
//! The C++ `Emit` API takes raw AST pointers (`const Varnode *`,
//! `const PcodeOp *`, `const Datatype *`, `const Symbol *`, `const FlowBlock
//! *`) and the *markup* back-end [`EmitMarkup`] dereferences them lazily inside
//! [`TokenSplit::print_to`] to query `getCreateIndex()` / `getTime()` / `getId()`
//! / `getIndex()` / `getUnsizedId()` / `getName()`.  In this port those AST
//! objects live in generational arenas owned by `Funcdata`/the type factory
//! (ADR 0001); they are not reachable from a buffered token without dragging
//! the whole arena lifetime through the emitter.  The markup association is
//! therefore carried as an already-**resolved** [`MarkupRef`] payload (the same
//! integer references / names the C++ encoder ultimately writes), resolved by
//! the caller (`PrintC`) at tag-call time — exactly the values
//! `EmitMarkup::tag*` would have computed.  The observable encoding is
//! identical; only *when* the pointer is dereferenced moves from inside the
//! emitter to its caller.  The non-markup path (`EmitNoMarkup`,
//! load-bearing for datatests) ignores [`MarkupRef`] entirely, exactly as the
//! C++ `EmitNoMarkup::tag*` ignores its pointer arguments.
//!
//! ## Cross-file marshaling ids
//!
//! `prettyprint.cc` *defines* a block of marshaling ids (`ATTRIB_BLOCKREF` …
//! `ELEM_VARIABLE`); those live here ([`ids`]).  `EmitMarkup` additionally
//! references ids *defined in other translation units* (`ELEM_FUNCTION`,
//! `ELEM_TYPE`, `ELEM_FIELD`, `ELEM_BITFIELD`, `ATTRIB_SYMREF`, …).  Where the
//! owning module has been ported its id is re-exported; the few not yet ported
//! are mirrored locally with their upstream numeric ids and a `// cross-file`
//! note (see [`ids`]) — they will collapse to a re-export when those modules
//! land.  This does not change emitted bytes (the numeric id is the contract).

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Encoder, PackedEncode};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};

/// Marshaling element/attribute ids used by the markup emitter.
///
/// The ids declared in `prettyprint.hh`/`prettyprint.cc` are defined here with
/// their upstream numeric ids.  Ids that `EmitMarkup` borrows from other
/// translation units are mirrored with a `// cross-file` note until those
/// modules are ported (the numeric id is the on-the-wire contract).
pub mod ids {
    use kuna_base::marshal::{AttributeId, ElementId};

    // --- declared in prettyprint.cc (this file owns these) ---
    /// Marshaling attribute "blockref" (prettyprint.cc:21).
    pub const ATTRIB_BLOCKREF: AttributeId = AttributeId::new("blockref", 35);
    /// Marshaling attribute "close" (prettyprint.cc:22).
    pub const ATTRIB_CLOSE: AttributeId = AttributeId::new("close", 36);
    /// Marshaling attribute "color" (prettyprint.cc:23).
    pub const ATTRIB_COLOR: AttributeId = AttributeId::new("color", 37);
    /// Marshaling attribute "indent" (prettyprint.cc:24).
    pub const ATTRIB_INDENT: AttributeId = AttributeId::new("indent", 38);
    /// Marshaling attribute "off" (prettyprint.cc:25).
    pub const ATTRIB_OFF: AttributeId = AttributeId::new("off", 39);
    /// Marshaling attribute "open" (prettyprint.cc:26).
    pub const ATTRIB_OPEN: AttributeId = AttributeId::new("open", 40);
    /// Marshaling attribute "opref" (prettyprint.cc:27).
    pub const ATTRIB_OPREF: AttributeId = AttributeId::new("opref", 41);
    /// Marshaling attribute "varref" (prettyprint.cc:28).
    pub const ATTRIB_VARREF: AttributeId = AttributeId::new("varref", 42);
    /// Marshaling element `<break>` (prettyprint.cc:29).
    pub const ELEM_BREAK: ElementId = ElementId::new("break", 17);
    /// Marshaling element `<clang_document>` (prettyprint.cc:30).
    pub const ELEM_CLANG_DOCUMENT: ElementId = ElementId::new("clang_document", 18);
    /// Marshaling element `<funcname>` (prettyprint.cc:31).
    pub const ELEM_FUNCNAME: ElementId = ElementId::new("funcname", 19);
    /// Marshaling element `<funcproto>` (prettyprint.cc:32).
    pub const ELEM_FUNCPROTO: ElementId = ElementId::new("funcproto", 20);
    /// Marshaling element `<label>` (prettyprint.cc:33).
    pub const ELEM_LABEL: ElementId = ElementId::new("label", 21);
    /// Marshaling element `<return_type>` (prettyprint.cc:34).
    pub const ELEM_RETURN_TYPE: ElementId = ElementId::new("return_type", 22);
    /// Marshaling element `<statement>` (prettyprint.cc:35).
    pub const ELEM_STATEMENT: ElementId = ElementId::new("statement", 23);
    /// Marshaling element `<syntax>` (prettyprint.cc:36).
    pub const ELEM_SYNTAX: ElementId = ElementId::new("syntax", 24);
    /// Marshaling element `<vardecl>` (prettyprint.cc:37).
    pub const ELEM_VARDECL: ElementId = ElementId::new("vardecl", 25);
    /// Marshaling element `<variable>` (prettyprint.cc:38).
    pub const ELEM_VARIABLE: ElementId = ElementId::new("variable", 26);

    // --- referenced by EmitMarkup but defined in other translation units ---
    // cross-file: defined in marshal.cc (ported in kuna-base).
    pub use kuna_base::marshal::{
        ATTRIB_CONTENT, ATTRIB_ID, ATTRIB_NAME, ATTRIB_SPACE, ELEM_VALUE,
    };

    // cross-file: re-export where the owning module is already ported.
    /// Marshaling element `<comment>` (comment.cc:21, id 86), re-exported.
    pub use crate::comment::ELEM_COMMENT;
    /// Marshaling element `<op>` (translate.cc:25, id 27), re-exported.
    pub use kuna_sleigh::translate::ELEM_OP;

    // cross-file: defined in funcdata.cc / type.cc / variable.cc (not yet
    // ported).  Mirrored with the *exact* upstream numeric id so the markup
    // encoding is byte-identical; collapses to a re-export once those land.
    /// Marshaling element `<function>` (funcdata.cc:23, id 116, cross-file).
    pub const ELEM_FUNCTION: ElementId = ElementId::new("function", 116);
    /// Marshaling element `<type>` (type.cc:67, id 60, cross-file).
    pub const ELEM_TYPE: ElementId = ElementId::new("type", 60);
    /// Marshaling element `<field>` (type.cc:56, id 49, cross-file).
    pub const ELEM_FIELD: ElementId = ElementId::new("field", 49);
    /// Marshaling element `<bitfield>` (type.cc:74, id 289, cross-file).
    pub const ELEM_BITFIELD: ElementId = ElementId::new("bitfield", 289);
    /// Marshaling attribute "symref" (variable.cc:25, id 68, cross-file).
    pub const ATTRIB_SYMREF: AttributeId = AttributeId::new("symref", 68);
}

/// An empty string (C++ `Emit::EMPTY_STRING`).
pub const EMPTY_STRING: &str = "";

/// Possible types of syntax highlighting (C++ `Emit::syntax_highlight`).
///
/// Values must match constants in `ClangToken`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SyntaxHighlight {
    /// Keyword in the high-level language.
    KeywordColor = 0,
    /// Comments.
    CommentColor = 1,
    /// Data-type identifiers.
    TypeColor = 2,
    /// Function identifiers.
    FuncnameColor = 3,
    /// Local variable identifiers.
    VarColor = 4,
    /// Constant values.
    ConstColor = 5,
    /// Function parameters.
    ParamColor = 6,
    /// Global variable identifiers.
    GlobalColor = 7,
    /// Un-highlighted.
    NoColor = 8,
    /// Indicates a warning or error state.
    ErrorColor = 9,
    /// A token with special/highlighted meaning.
    SpecialColor = 10,
}

impl SyntaxHighlight {
    /// The integer color value as written by [`EmitMarkup`] (C++ casts the
    /// enum directly to the `color` attribute).
    #[inline]
    fn value(self) -> u64 {
        self as i32 as u64
    }
}

/// Different brace formatting styles (C++ `Emit::brace_style`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BraceStyle {
    /// Opening brace on the same line as if/do/while/for/switch.
    SameLine = 0,
    /// Opening brace is on next line.
    NextLine = 1,
    /// Opening brace is two lines down.
    SkipLine = 2,
}

/// Resolved markup association carried by a token (see module docs).
///
/// The C++ `Emit` API passes raw AST pointers; the markup back-end derefs them
/// lazily to obtain these integer references / names.  In this port the caller
/// (`PrintC`) resolves them up front, so a buffered [`TokenSplit`] needs no AST
/// lifetime.  Every field is `None`/absent unless the corresponding C++ pointer
/// was non-null, matching the `if (ptr != 0)` guards in `EmitMarkup`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MarkupRef {
    /// `op->getTime()` for an associated `PcodeOp` (C++ `ATTRIB_OPREF`).
    pub opref: Option<uintb>,
    /// `vn->getCreateIndex()` for an associated `Varnode` (C++ `ATTRIB_VARREF`).
    pub varref: Option<uintb>,
    /// `bl->getIndex()` for an associated `FlowBlock` (C++ `ATTRIB_BLOCKREF`).
    pub blockref: Option<int4>,
    /// `sym->getId()` for an associated `Symbol` (C++ `ATTRIB_SYMREF`).
    pub symref: Option<uintb>,
    /// `vn->getCreateIndex()` for a return-type's storage `Varnode`.
    pub return_varref: Option<uintb>,
    /// `ct->getUnsizedId()` for an associated `Datatype` (C++ `ATTRIB_ID`).
    pub type_id: Option<uintb>,
    /// `ct->getName()` for an associated `Datatype` (C++ `ATTRIB_NAME`).
    pub type_name: Option<String>,
}

impl MarkupRef {
    /// An association with no resolved members (all C++ pointers were null).
    #[inline]
    pub fn none() -> Self {
        MarkupRef::default()
    }

    /// Convenience: an association carrying only an `opref` (the common case
    /// for `tagOp`/`tagCaseLabel`).
    #[inline]
    pub fn op(opref: Option<uintb>) -> Self {
        MarkupRef { opref, ..MarkupRef::default() }
    }
}

/// The interface for emitting the Decompiler's formal output: source code.
///
/// Port of the C++ pure-virtual `Emit` base class.  The methods that C++
/// implements non-virtually on the base (whitespace, indent/comment/paren
/// counters, the brace helpers) are provided here as default methods over the
/// [`Emit::state`]/[`Emit::state_mut`] accessors that every back-end exposes.
pub trait Emit {
    /// Borrow the shared base state (indent/paren counters, increment).
    fn state(&self) -> &EmitBase;
    /// Mutably borrow the shared base state.
    fn state_mut(&mut self) -> &mut EmitBase;

    // --- begin/end document/function/block ---

    /// Begin a whole document of output; returns an id for the document.
    fn begin_document(&mut self) -> int4;
    /// End a whole document of output.
    fn end_document(&mut self, id: int4);
    /// Begin a whole declaration and body of a function.
    fn begin_function(&mut self) -> int4;
    /// End a whole declaration and body of a function.
    fn end_function(&mut self, id: int4);
    /// Begin a control-flow element.  `blockref` is `bl->getIndex()`.
    fn begin_block(&mut self, blockref: int4) -> int4;
    /// End a control-flow element.
    fn end_block(&mut self, id: int4);

    // --- line breaks ---

    /// Force a line break at the current indent level.
    fn tag_line(&mut self);
    /// Force a line break with a one-time indent override.
    fn tag_line_indent(&mut self, indent: int4);

    // --- return type / vardecl / statement / funcproto ---

    /// Begin a return-type declaration.  `markup.return_varref` is the storage.
    fn begin_return_type(&mut self, markup: &MarkupRef) -> int4;
    /// End a return-type declaration.
    fn end_return_type(&mut self, id: int4);
    /// Begin a variable declaration.  `markup.symref` is the symbol id.
    fn begin_var_decl(&mut self, markup: &MarkupRef) -> int4;
    /// End a variable declaration.
    fn end_var_decl(&mut self, id: int4);
    /// Begin a source-code statement.  `markup.opref` is the root op's time.
    fn begin_statement(&mut self, markup: &MarkupRef) -> int4;
    /// End a source-code statement.
    fn end_statement(&mut self, id: int4);
    /// Begin a function-prototype declaration.
    fn begin_func_proto(&mut self) -> int4;
    /// End a function-prototype declaration.
    fn end_func_proto(&mut self, id: int4);

    // --- content tokens ---

    /// Emit a variable token.
    fn tag_variable(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef);
    /// Emit an operation token.
    fn tag_op(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef);
    /// Emit a function identifier token.
    fn tag_func_name(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef);
    /// Emit a data-type identifier token.
    fn tag_type(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef);
    /// Emit a field identifier within a structured data-type.  `off` is the
    /// byte offset of the field.
    fn tag_field(&mut self, name: &str, hl: SyntaxHighlight, off: int4, markup: &MarkupRef);
    /// Emit a bitfield identifier within a structured data-type.  `id` is the
    /// field identifier.
    fn tag_bit_field(&mut self, name: &str, hl: SyntaxHighlight, id: int4, markup: &MarkupRef);
    /// Emit a comment string.  `spc`/`off` is the attachment address.  `spc`
    /// mirrors the C++ `const AddrSpace *` (shared, long-lived) as an
    /// [`Rc<AddrSpace>`].
    fn tag_comment(&mut self, name: &str, hl: SyntaxHighlight, spc: &Rc<AddrSpace>, off: uintb);
    /// Emit a code-label identifier.  `spc`/`off` is the labeled code address.
    fn tag_label(&mut self, name: &str, hl: SyntaxHighlight, spc: &Rc<AddrSpace>, off: uintb);
    /// Emit a `case` label constant.  `value` is the raw switch value;
    /// `markup.opref` is the first op of the case block.
    fn tag_case_label(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef, value: uintb);
    /// Emit other (more unusual) syntax (spaces, semicolons, braces, …).
    fn print(&mut self, data: &str, hl: SyntaxHighlight);

    // --- parentheses / groups ---

    /// Emit an open parenthesis, starting a printing unit; returns an id.
    fn open_paren(&mut self, paren: &str, id: int4) -> int4;
    /// Emit a close parenthesis, ending the matching printing unit.
    fn close_paren(&mut self, paren: &str, id: int4);

    /// Start a group of things printed together.  Default: no-op returning 0.
    fn open_group(&mut self) -> int4 {
        0
    }
    /// End a group of things printed together.  Default: no-op.
    fn close_group(&mut self, _id: int4) {}

    /// Reset the emitter to its initial state.
    fn clear(&mut self) {
        let st = self.state_mut();
        st.parenlevel = 0;
        st.indentlevel = 0;
    }

    /// Toggle whether this emits mark-up or not.  Default: no effect.
    fn set_markup(&mut self, _val: bool) {}
    /// Toggle whether this emitter produces packed output.  Default: no effect.
    fn set_packed_output(&mut self, _val: bool) {}

    /// Start a new indent level; returns an id.
    fn start_indent(&mut self) -> int4 {
        let st = self.state_mut();
        st.indentlevel += st.indentincrement;
        0
    }
    /// End an indent level, returning to the previous level.
    fn stop_indent(&mut self, _id: int4) {
        let st = self.state_mut();
        st.indentlevel -= st.indentincrement;
    }
    /// Start a comment block.  Default: no-op returning 0.
    fn start_comment(&mut self) -> int4 {
        0
    }
    /// End a comment block.  Default: no-op.
    fn stop_comment(&mut self, _id: int4) {}

    /// Flush any remaining buffered character data.  Default: no-op.
    fn flush(&mut self) -> KunaResult<()> {
        Ok(())
    }

    /// Provide a maximum line size to the pretty printer.  Default: no effect.
    fn set_max_line_size(&mut self, _mls: int4) -> KunaResult<()> {
        Ok(())
    }
    /// Get the current maximum line size, or -1 if there is none.
    fn get_max_line_size(&self) -> int4 {
        -1
    }
    /// Set the comment fill string used when a comment line is broken.
    fn set_comment_fill(&mut self, _fill: &str) {}

    /// Whether this is an XML markup emitter.
    fn emits_markup(&self) -> bool;

    /// (Re)set the default emitting options.
    fn reset_defaults(&mut self) {
        self.state_mut().reset_defaults_internal();
    }

    // --- non-virtual base helpers (default methods) ---

    /// Get the current parentheses depth (C++ `getParenLevel`).
    fn get_paren_level(&self) -> int4 {
        self.state().parenlevel
    }
    /// Get the number of characters indented per level (C++ `getIndentIncrement`).
    fn get_indent_increment(&self) -> int4 {
        self.state().indentincrement
    }
    /// Set the number of characters indented per level (C++ `setIndentIncrement`).
    fn set_indent_increment(&mut self, val: int4) {
        self.state_mut().indentincrement = val;
    }

    /// Emit a sequence of `num` space characters (C++ `Emit::spaces`).
    ///
    /// `bump` is the extra indent if the spaces force a line break (used only
    /// by [`EmitPrettyPrint`]; the base ignores it, exactly like C++).
    fn spaces(&mut self, num: int4, _bump: int4) {
        // static const string spacearray[] = { "", " ", ... "          " };
        const SPACEARRAY: [&str; 11] = [
            "", " ", "  ", "   ", "    ", "     ", "      ", "       ", "        ", "         ",
            "          ",
        ];
        if num <= 10 {
            self.print(SPACEARRAY[num as usize], SyntaxHighlight::NoColor);
        } else {
            let mut spc = String::new();
            for _ in 0..num {
                spc.push(' ');
            }
            self.print(&spc, SyntaxHighlight::NoColor);
        }
    }

    /// Emit an opening brace with the given format and add an indent level
    /// (C++ `Emit::openBraceIndent`).
    fn open_brace_indent(&mut self, brace: &str, style: BraceStyle) -> int4 {
        if style == BraceStyle::SameLine {
            self.spaces(1, 0);
        } else if style == BraceStyle::SkipLine {
            self.tag_line();
            self.tag_line();
        } else {
            self.tag_line();
        }
        let id = self.start_indent();
        self.print(brace, SyntaxHighlight::NoColor);
        id
    }

    /// Emit an opening brace with the given format; indent is *not* increased
    /// (C++ `Emit::openBrace`).
    fn open_brace(&mut self, brace: &str, style: BraceStyle) {
        if style == BraceStyle::SameLine {
            self.spaces(1, 0);
        } else if style == BraceStyle::SkipLine {
            self.tag_line();
            self.tag_line();
        } else {
            self.tag_line();
        }
        self.print(brace, SyntaxHighlight::NoColor);
    }

    /// Emit a closing brace and remove an indent level (C++ `Emit::closeBraceIndent`).
    fn close_brace_indent(&mut self, brace: &str, id: int4) {
        self.stop_indent(id);
        self.tag_line();
        self.print(brace, SyntaxHighlight::NoColor);
    }

    /// Register a pending brace (C++ `Emit::setPendingPrint` of a `PendingBrace`).
    /// The brace opens lazily at the next [`emit_pending`](Emit::emit_pending)
    /// (the next `tag_line`) unless [`cancel_pending_brace`](Emit::cancel_pending_brace)
    /// is called first — which yields the `else if` syntax.  Resets the fired-
    /// result slot to -1 (a fresh registration).
    fn set_pending_brace(&mut self, style: BraceStyle) {
        let st = self.state_mut();
        st.pend_print = Some(style);
        st.pend_brace_indent = -1;
    }

    /// Is a brace still pending (C++ `Emit::hasPendingPrint`)?  True only while
    /// the brace has neither fired nor been canceled.
    fn has_pending_brace(&self) -> bool {
        self.state().pend_print.is_some()
    }

    /// Cancel the pending brace without firing it (C++ `Emit::cancelPendingPrint`).
    fn cancel_pending_brace(&mut self) {
        self.state_mut().pend_print = None;
    }

    /// The indent id the most recently fired pending brace opened, or -1 if it
    /// never fired (C++ `PendingBrace::getIndentId`).  Read after the else-clause
    /// is emitted to decide whether a brace needs closing.
    fn pending_brace_indent_id(&self) -> int4 {
        self.state().pend_brace_indent
    }

    /// Fire any pending print command (C++ `Emit::emitPending`).  Clears the
    /// pending slot *before* the callback (so the brace's own `tag_line` does not
    /// re-enter), opens the indented brace, and records the resulting indent id
    /// so [`pending_brace_indent_id`](Emit::pending_brace_indent_id) can return it.
    fn emit_pending(&mut self) {
        if let Some(style) = self.state().pend_print {
            // Clear pending before the callback (C++ emitPending order).
            self.state_mut().pend_print = None;
            let id = self.open_brace_indent(OPEN_CURLY_FOR_PENDING, style);
            // Record the fired brace's indent id (C++ PendingBrace::indentId).
            self.state_mut().pend_brace_indent = id;
        }
    }
}

/// The `{` token a fired pending brace prints (C++ `PrintC::OPEN_CURLY`).
const OPEN_CURLY_FOR_PENDING: &str = "{";

/// Shared base state of every emitter (C++ `Emit` data members).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitBase {
    /// Current indent level (in fixed-width characters).
    pub indentlevel: int4,
    /// Current depth of parentheses.
    pub parenlevel: int4,
    /// Change in `indentlevel` per level of nesting.
    pub indentincrement: int4,
    /// The cancelable pending-print command (C++ `Emit::pendPrint`, whose only
    /// concrete subtype is `PendingBrace`).  `Some(style)` while a brace is
    /// registered and not yet fired or canceled.  Fired by
    /// [`Emit::emit_pending`] at the next `tag_line`; if canceled first
    /// ([`Emit::cancel_pending_brace`]) the brace never opens — the `else if`
    /// syntax.
    pub pend_print: Option<BraceStyle>,
    /// Indent id the most recently *fired* pending brace opened, else -1 (C++
    /// `PendingBrace::indentId`, which lives on the caller's stack frame).  Read
    /// by `emit_block_if` to decide whether a brace needs closing.
    pub pend_brace_indent: int4,
}

impl Default for EmitBase {
    fn default() -> Self {
        EmitBase::new()
    }
}

impl EmitBase {
    /// C++ `Emit::Emit()`: zeroed counters, `resetDefaultsInternal()`.
    pub fn new() -> Self {
        EmitBase {
            indentlevel: 0,
            parenlevel: 0,
            indentincrement: 2,
            pend_print: None,
            pend_brace_indent: -1,
        }
    }
    /// C++ `Emit::resetDefaultsInternal()`.
    fn reset_defaults_internal(&mut self) {
        self.indentincrement = 2;
    }
}

// ===========================================================================
// EmitNoMarkup
// ===========================================================================

/// A trivial emitter that outputs syntax straight to its output buffer.
///
/// Port of C++ `EmitNoMarkup`.  Does neither pretty printing nor markup; dumps
/// any tokens straight to the final output buffer.  Used as the low-level
/// back-end for [`EmitPrettyPrint`] on the byte-exact `print C` path.
///
/// The C++ `ostream *s` becomes an owned `String` sink;
/// [`set_output_stream`](EmitNoMarkup::set_output_stream) resets it, and
/// [`output`](EmitNoMarkup::output)/[`take_output`](EmitNoMarkup::take_output)
/// retrieve it.
#[derive(Debug, Default)]
pub struct EmitNoMarkup {
    base: EmitBase,
    out: String,
}

impl EmitNoMarkup {
    /// C++ `EmitNoMarkup::EmitNoMarkup()`.
    pub fn new() -> Self {
        EmitNoMarkup { base: EmitBase::new(), out: String::new() }
    }
    /// Borrow the accumulated output (C++ would read the bound `ostream`).
    pub fn output(&self) -> &str {
        &self.out
    }
    /// Take ownership of the accumulated output, leaving an empty buffer.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.out)
    }
    /// Reset the output buffer (C++ `setOutputStream`).
    pub fn set_output_stream(&mut self) {
        self.out.clear();
    }
}

impl Emit for EmitNoMarkup {
    fn state(&self) -> &EmitBase {
        &self.base
    }
    fn state_mut(&mut self) -> &mut EmitBase {
        &mut self.base
    }

    fn begin_document(&mut self) -> int4 {
        0
    }
    fn end_document(&mut self, _id: int4) {}
    fn begin_function(&mut self) -> int4 {
        0
    }
    fn end_function(&mut self, _id: int4) {}
    fn begin_block(&mut self, _blockref: int4) -> int4 {
        0
    }
    fn end_block(&mut self, _id: int4) {}

    fn tag_line(&mut self) {
        // In the kuna port the `print C` path drives `EmitNoMarkup` directly
        // where C++ stacks `EmitPrettyPrint` over it (printlanguage.cc:69).
        // EmitPrettyPrint::tagLine fires the pending brace before the break, so
        // the collapsed EmitNoMarkup must absorb that responsibility here for the
        // `else if` syntax to work.  Cleared-before-callback, so no re-entry.
        self.emit_pending();
        self.out.push('\n');
        for _ in 0..self.base.indentlevel {
            self.out.push(' ');
        }
    }
    fn tag_line_indent(&mut self, indent: int4) {
        self.emit_pending();
        self.out.push('\n');
        for _ in 0..indent {
            self.out.push(' ');
        }
    }

    fn begin_return_type(&mut self, _markup: &MarkupRef) -> int4 {
        0
    }
    fn end_return_type(&mut self, _id: int4) {}
    fn begin_var_decl(&mut self, _markup: &MarkupRef) -> int4 {
        0
    }
    fn end_var_decl(&mut self, _id: int4) {}
    fn begin_statement(&mut self, _markup: &MarkupRef) -> int4 {
        0
    }
    fn end_statement(&mut self, _id: int4) {}
    fn begin_func_proto(&mut self) -> int4 {
        0
    }
    fn end_func_proto(&mut self, _id: int4) {}

    fn tag_variable(&mut self, name: &str, _hl: SyntaxHighlight, _markup: &MarkupRef) {
        self.out.push_str(name);
    }
    fn tag_op(&mut self, name: &str, _hl: SyntaxHighlight, _markup: &MarkupRef) {
        self.out.push_str(name);
    }
    fn tag_func_name(&mut self, name: &str, _hl: SyntaxHighlight, _markup: &MarkupRef) {
        self.out.push_str(name);
    }
    fn tag_type(&mut self, name: &str, _hl: SyntaxHighlight, _markup: &MarkupRef) {
        self.out.push_str(name);
    }
    fn tag_field(&mut self, name: &str, _hl: SyntaxHighlight, _off: int4, _markup: &MarkupRef) {
        self.out.push_str(name);
    }
    fn tag_bit_field(&mut self, name: &str, _hl: SyntaxHighlight, _id: int4, _markup: &MarkupRef) {
        self.out.push_str(name);
    }
    fn tag_comment(&mut self, name: &str, _hl: SyntaxHighlight, _spc: &Rc<AddrSpace>, _off: uintb) {
        self.out.push_str(name);
    }
    fn tag_label(&mut self, name: &str, _hl: SyntaxHighlight, _spc: &Rc<AddrSpace>, _off: uintb) {
        self.out.push_str(name);
    }
    fn tag_case_label(
        &mut self,
        name: &str,
        _hl: SyntaxHighlight,
        _markup: &MarkupRef,
        _value: uintb,
    ) {
        self.out.push_str(name);
    }
    fn print(&mut self, data: &str, _hl: SyntaxHighlight) {
        self.out.push_str(data);
    }
    fn open_paren(&mut self, paren: &str, id: int4) -> int4 {
        self.out.push_str(paren);
        self.base.parenlevel += 1;
        id
    }
    fn close_paren(&mut self, paren: &str, _id: int4) {
        self.out.push_str(paren);
        self.base.parenlevel -= 1;
    }
    fn emits_markup(&self) -> bool {
        false
    }
}

// ===========================================================================
// EmitMarkup
// ===========================================================================

/// An emitter that associates XML markup with individual tokens.
///
/// Port of C++ `EmitMarkup`.  Variable and operation tokens are associated with
/// the integer references of their corresponding `Varnode`/`PcodeOp` (carried
/// in a [`MarkupRef`], see module docs).  This is the low-level back-end for
/// [`EmitPrettyPrint`] on the `print C xml` path.
///
/// ## Encoder choice
///
/// C++ holds a persistent `Encoder *` (a `PackedEncode` by default, or an
/// `XmlEncode` after `setPackedOutput(false)`) bound to the stream, and opens
/// elements in `begin*` that stay open until the matching `end*` closes them.
/// The port uses kuna-base's [`PackedEncode`], which is **stateless** — element
/// nesting lives entirely in the length-tagged byte stream, not in encoder
/// state — so each call can construct a fresh `PackedEncode` over the owned
/// buffer and append; `begin*` appends the element-start marker (and
/// attributes), the matching `end*` appends the element-end marker, and the
/// resulting byte stream is identical to one persistent encoder.  This is why
/// the internal `with_encoder` helper re-borrows the buffer per call.
///
/// The unpacked `XmlEncode` path (which *does* carry `depth`/`tag_status`)
/// would need a persistent borrowing encoder and is therefore **not** supported
/// by this back-end (see the port-notes LOSS); `setPackedOutput(false)` is
/// recorded but does not change the encoder.  This matches the only markup path
/// `print C xml` actually exercises (packed).
#[derive(Debug)]
pub struct EmitMarkup {
    base: EmitBase,
    out: Vec<u8>,
    /// `true` selects packed output, `false` would select unpacked XML.
    /// Recorded by `setPackedOutput`; see the type-level doc for why only the
    /// packed encoder is wired here.
    packed: bool,
}

impl Default for EmitMarkup {
    fn default() -> Self {
        EmitMarkup::new()
    }
}

impl EmitMarkup {
    /// C++ `EmitMarkup::EmitMarkup()`.  The upstream default encoder is
    /// `PackedEncode` (`setOutputStream`).
    pub fn new() -> Self {
        EmitMarkup { base: EmitBase::new(), out: Vec::new(), packed: true }
    }

    /// Borrow the accumulated encoded bytes.
    pub fn output(&self) -> &[u8] {
        &self.out
    }
    /// Take ownership of the accumulated encoded bytes.
    pub fn take_output(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.out)
    }

    /// Run `f` with a fresh [`PackedEncode`] over the output buffer.
    ///
    /// `PackedEncode` is stateless (it only holds `&mut Vec<u8>`), so appending
    /// element-start/-end markers across separate calls yields the same byte
    /// stream a single persistent encoder would, exactly as C++.
    fn with_encoder<F: FnOnce(&mut dyn Encoder)>(&mut self, f: F) {
        let _ = self.packed; // see type-level doc; XmlEncode path not wired
        let mut enc = PackedEncode::new(&mut self.out);
        f(&mut enc);
    }
}

impl Emit for EmitMarkup {
    fn state(&self) -> &EmitBase {
        &self.base
    }
    fn state_mut(&mut self) -> &mut EmitBase {
        &mut self.base
    }

    fn begin_document(&mut self) -> int4 {
        self.with_encoder(|e| e.open_element(&ids::ELEM_CLANG_DOCUMENT));
        0
    }
    fn end_document(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&ids::ELEM_CLANG_DOCUMENT));
    }
    fn begin_function(&mut self) -> int4 {
        self.with_encoder(|e| e.open_element(&ids::ELEM_FUNCTION));
        0
    }
    fn end_function(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&ids::ELEM_FUNCTION));
    }
    fn begin_block(&mut self, blockref: int4) -> int4 {
        self.with_encoder(|e| {
            e.open_element(&crate::block::ELEM_BLOCK);
            e.write_signed_integer(&ids::ATTRIB_BLOCKREF, blockref as i64);
        });
        0
    }
    fn end_block(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&crate::block::ELEM_BLOCK));
    }

    fn tag_line(&mut self) {
        // emitPending() (C++ EmitMarkup::tagLine): fire a registered PendingBrace
        // before the break.  Cleared-before-callback, so no re-entry.
        self.emit_pending();
        let indent = self.base.indentlevel;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_BREAK);
            e.write_signed_integer(&ids::ATTRIB_INDENT, indent as i64);
            e.close_element(&ids::ELEM_BREAK);
        });
    }
    fn tag_line_indent(&mut self, indent: int4) {
        self.emit_pending();
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_BREAK);
            e.write_signed_integer(&ids::ATTRIB_INDENT, indent as i64);
            e.close_element(&ids::ELEM_BREAK);
        });
    }

    fn begin_return_type(&mut self, markup: &MarkupRef) -> int4 {
        let vr = markup.return_varref;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_RETURN_TYPE);
            if let Some(v) = vr {
                e.write_unsigned_integer(&ids::ATTRIB_VARREF, v);
            }
        });
        0
    }
    fn end_return_type(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&ids::ELEM_RETURN_TYPE));
    }
    fn begin_var_decl(&mut self, markup: &MarkupRef) -> int4 {
        let sr = markup.symref;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_VARDECL);
            if let Some(s) = sr {
                e.write_unsigned_integer(&ids::ATTRIB_SYMREF, s);
            }
        });
        0
    }
    fn end_var_decl(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&ids::ELEM_VARDECL));
    }
    fn begin_statement(&mut self, markup: &MarkupRef) -> int4 {
        let op = markup.opref;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_STATEMENT);
            if let Some(o) = op {
                e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
            }
        });
        0
    }
    fn end_statement(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&ids::ELEM_STATEMENT));
    }
    fn begin_func_proto(&mut self) -> int4 {
        self.with_encoder(|e| e.open_element(&ids::ELEM_FUNCPROTO));
        0
    }
    fn end_func_proto(&mut self, _id: int4) {
        self.with_encoder(|e| e.close_element(&ids::ELEM_FUNCPROTO));
    }

    fn tag_variable(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        let (vr, op) = (markup.varref, markup.opref);
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_VARIABLE);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            if let Some(v) = vr {
                e.write_unsigned_integer(&ids::ATTRIB_VARREF, v);
            }
            if let Some(o) = op {
                e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_VARIABLE);
        });
    }
    fn tag_op(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        let op = markup.opref;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_OP);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            if let Some(o) = op {
                e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_OP);
        });
    }
    fn tag_func_name(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        let op = markup.opref;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_FUNCNAME);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            if let Some(o) = op {
                e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_FUNCNAME);
        });
    }
    fn tag_type(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        let tid = markup.type_id;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_TYPE);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            if let Some(id) = tid {
                if id != 0 {
                    e.write_unsigned_integer(&ids::ATTRIB_ID, id);
                }
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_TYPE);
        });
    }
    fn tag_field(&mut self, name: &str, hl: SyntaxHighlight, off: int4, markup: &MarkupRef) {
        let (tname, tid, op) = (markup.type_name.clone(), markup.type_id, markup.opref);
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_FIELD);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            if let Some(nm) = tname {
                e.write_string(&ids::ATTRIB_NAME, nm.as_bytes());
                if let Some(id) = tid {
                    if id != 0 {
                        e.write_unsigned_integer(&ids::ATTRIB_ID, id);
                    }
                }
                e.write_signed_integer(&ids::ATTRIB_OFF, off as i64);
                if let Some(o) = op {
                    e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
                }
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_FIELD);
        });
    }
    fn tag_bit_field(&mut self, name: &str, hl: SyntaxHighlight, id: int4, markup: &MarkupRef) {
        let (tname, tid, op) = (markup.type_name.clone(), markup.type_id, markup.opref);
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_BITFIELD);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            if let Some(nm) = tname {
                e.write_string(&ids::ATTRIB_NAME, nm.as_bytes());
            }
            if let Some(t) = tid {
                if t != 0 {
                    e.write_unsigned_integer(&ids::ATTRIB_ID, t);
                }
            }
            e.write_signed_integer(&ids::ATTRIB_OFF, id as i64);
            if let Some(o) = op {
                e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_BITFIELD);
        });
    }
    fn tag_comment(&mut self, name: &str, hl: SyntaxHighlight, spc: &Rc<AddrSpace>, off: uintb) {
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_COMMENT);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            e.write_space(&ids::ATTRIB_SPACE, spc);
            e.write_unsigned_integer(&ids::ATTRIB_OFF, off);
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_COMMENT);
        });
    }
    fn tag_label(&mut self, name: &str, hl: SyntaxHighlight, spc: &Rc<AddrSpace>, off: uintb) {
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_LABEL);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            e.write_space(&ids::ATTRIB_SPACE, spc);
            e.write_unsigned_integer(&ids::ATTRIB_OFF, off);
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_LABEL);
        });
    }
    fn tag_case_label(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef, value: uintb) {
        let op = markup.opref;
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_VALUE);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            e.write_unsigned_integer(&ids::ATTRIB_OFF, value);
            if let Some(o) = op {
                e.write_unsigned_integer(&ids::ATTRIB_OPREF, o);
            }
            e.write_string(&ids::ATTRIB_CONTENT, name.as_bytes());
            e.close_element(&ids::ELEM_VALUE);
        });
    }
    fn print(&mut self, data: &str, hl: SyntaxHighlight) {
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_SYNTAX);
            if hl != SyntaxHighlight::NoColor {
                e.write_unsigned_integer(&ids::ATTRIB_COLOR, hl.value());
            }
            e.write_string(&ids::ATTRIB_CONTENT, data.as_bytes());
            e.close_element(&ids::ELEM_SYNTAX);
        });
    }
    fn open_paren(&mut self, paren: &str, id: int4) -> int4 {
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_SYNTAX);
            e.write_signed_integer(&ids::ATTRIB_OPEN, id as i64);
            e.write_string(&ids::ATTRIB_CONTENT, paren.as_bytes());
            e.close_element(&ids::ELEM_SYNTAX);
        });
        self.base.parenlevel += 1;
        0
    }
    fn close_paren(&mut self, paren: &str, id: int4) {
        self.with_encoder(|e| {
            e.open_element(&ids::ELEM_SYNTAX);
            e.write_signed_integer(&ids::ATTRIB_CLOSE, id as i64);
            e.write_string(&ids::ATTRIB_CONTENT, paren.as_bytes());
            e.close_element(&ids::ELEM_SYNTAX);
        });
        self.base.parenlevel -= 1;
    }
    fn set_packed_output(&mut self, val: bool) {
        self.packed = val;
    }
    fn emits_markup(&self) -> bool {
        true
    }
}

// ===========================================================================
// circularqueue
// ===========================================================================

/// A circular buffer (C++ template `circularqueue<_type>`).
///
/// Acts as a stack (`push`/`pop`) or a queue (`push`/`popbottom`); the size can
/// be grown with [`expand`](CircularQueue::expand).  Objects are looked up by an
/// integer reference into the backing array.  The empty/full bookkeeping
/// (`left == (right+1)%max`) is transcribed exactly because [`EmitPrettyPrint`]
/// reads/writes `ref(i)` by these raw indices.
#[derive(Debug, Clone)]
pub struct CircularQueue<T> {
    cache: Vec<T>,
    /// Index of the leftmost (bottom) object.
    left: int4,
    /// Index of the rightmost (top) object.
    right: int4,
    /// Size of the backing array.
    max: int4,
}

impl<T: Default + Clone> CircularQueue<T> {
    /// C++ `circularqueue(int4 sz)`: empty queue (`left=1`, `right=0`).
    pub fn new(sz: int4) -> Self {
        let mut cache = Vec::with_capacity(sz as usize);
        for _ in 0..sz {
            cache.push(T::default());
        }
        CircularQueue { cache, left: 1, right: 0, max: sz }
    }

    /// C++ `getMax()`.
    pub fn get_max(&self) -> int4 {
        self.max
    }

    /// C++ `setMax(int4 sz)`: reallocates (always empties the queue).
    pub fn set_max(&mut self, sz: int4) {
        if self.max != sz {
            self.max = sz;
            self.cache = Vec::with_capacity(sz as usize);
            for _ in 0..sz {
                self.cache.push(T::default());
            }
        }
        self.left = 1;
        self.right = 0;
    }

    /// C++ `expand(int4 amount)`: grow, preserving contents; the leftmost
    /// element ends up at reference 0 (the caller must adjust references).
    pub fn expand(&mut self, amount: int4) {
        let newmax = self.max + amount;
        let mut newcache: Vec<T> = Vec::with_capacity(newmax as usize);
        for _ in 0..newmax {
            newcache.push(T::default());
        }
        let mut i = self.left;
        let mut j = 0;
        // Assume there is at least one element in queue
        while i != self.right {
            newcache[j as usize] = self.cache[i as usize].clone();
            j += 1;
            i = (i + 1) % self.max;
        }
        newcache[j as usize] = self.cache[i as usize].clone(); // Copy rightmost
        self.left = 0;
        self.right = j;
        self.cache = newcache;
        self.max += amount;
    }

    /// C++ `clear()`.
    pub fn clear(&mut self) {
        self.left = 1;
        self.right = 0;
    }

    /// C++ `empty()`.
    pub fn empty(&self) -> bool {
        self.left == (self.right + 1) % self.max
    }

    /// C++ `topref()`.
    pub fn topref(&self) -> int4 {
        self.right
    }

    /// C++ `bottomref()`.
    pub fn bottomref(&self) -> int4 {
        self.left
    }

    /// C++ `ref(int4 r)` (shared).
    pub fn get_ref(&self, r: int4) -> &T {
        &self.cache[r as usize]
    }
    /// C++ `ref(int4 r)` (mutable).
    pub fn get_ref_mut(&mut self, r: int4) -> &mut T {
        &mut self.cache[r as usize]
    }

    /// C++ `top()` (shared).
    pub fn top(&self) -> &T {
        &self.cache[self.right as usize]
    }
    /// C++ `top()` (mutable).
    pub fn top_mut(&mut self) -> &mut T {
        &mut self.cache[self.right as usize]
    }
    /// C++ `bottom()` (shared).
    pub fn bottom(&self) -> &T {
        &self.cache[self.left as usize]
    }

    /// C++ `push()`: advance `right` and return a reference to the new slot.
    /// Returns the index (the C++ returns `_type&`; callers in this port want
    /// the reference index for [`get_ref_mut`](CircularQueue::get_ref_mut)).
    pub fn push(&mut self) -> int4 {
        self.right = (self.right + 1) % self.max;
        self.right
    }

    /// C++ `pop()`: pop the last (top) object; returns its old index.
    pub fn pop(&mut self) -> int4 {
        let tmp = self.right;
        self.right = (self.right + self.max - 1) % self.max;
        tmp
    }

    /// C++ `popbottom()`: dequeue the first (bottom) object; returns its old index.
    pub fn popbottom(&mut self) -> int4 {
        let tmp = self.left;
        self.left = (self.left + 1) % self.max;
        tmp
    }
}

// ===========================================================================
// TokenSplit
// ===========================================================================

/// General class of a token (C++ `TokenSplit::printclass`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintClass {
    /// A token that starts a printing group.
    Begin,
    /// A token that ends a printing group.
    End,
    /// A token representing actual content.
    TokenString,
    /// White space (where line breaks can be inserted).
    TokenBreak,
    /// Start of a new nesting level.
    BeginIndent,
    /// End of a nesting level.
    EndIndent,
    /// Start of a comment block.
    BeginComment,
    /// End of a comment block.
    EndComment,
    /// Mark-up that doesn't affect pretty printing.
    Ignore,
}

/// The exhaustive list of possible token types (C++ `TokenSplit::tag_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    /// Start of a document.
    DocuB,
    /// End of a document.
    DocuE,
    /// Start of a function body.
    FuncB,
    /// End of a function body.
    FuncE,
    /// Start of a control-flow section.
    BlocB,
    /// End of a control-flow section.
    BlocE,
    /// Start of a return type declaration.
    RtypB,
    /// End of a return type declaration.
    RtypE,
    /// Start of a variable declaration.
    VardB,
    /// End of a variable declaration.
    VardE,
    /// Start of a statement.
    StatB,
    /// End of a statement.
    StatE,
    /// Start of a function prototype.
    ProtB,
    /// End of a function prototype.
    ProtE,
    /// A variable identifier.
    VariT,
    /// An operator.
    OpT,
    /// A function identifier.
    FnamT,
    /// A data-type identifier.
    TypeT,
    /// A field name for a structured data-type.
    FieldT,
    /// A bitfield name in a structured data-type.
    BitfieldT,
    /// Part of a comment block.
    CommT,
    /// A code label.
    LabelT,
    /// A case label.
    CaseT,
    /// Other unspecified syntax.
    SyntT,
    /// Open parenthesis.
    OparT,
    /// Close parenthesis.
    CparT,
    /// Start of an arbitrary (invisible) grouping.
    OinvT,
    /// End of an arbitrary (invisible) grouping.
    CinvT,
    /// White space.
    SpacT,
    /// Required line break.
    BumpT,
    /// Required line break with one-time indent level.
    LineT,
}

/// Address-space payload (C++ `ptr_second.spc`) for comment/label tokens.
///
/// The C++ `const AddrSpace *spc` becomes a shared [`Rc<AddrSpace>`] (the
/// spaces are long-lived and owned by the architecture); cloning the `Rc` is
/// the cheap reference-count bump the C++ pointer copy is.  The accompanying
/// offset lives in [`TokenSplit::off`] exactly as in C++ (`tagComment` sets
/// both `ptr_second.spc` and the `off` member).
#[derive(Debug, Clone)]
struct AddrPayload {
    spc: Rc<AddrSpace>,
}

/// A token/command object in the pretty-printing stream (C++ `TokenSplit`).
///
/// The pretty-printing algorithm works on a stream of these: content tokens
/// plus begin/end and open/close delimiters that bracket the groups of tokens
/// printed as a unit.  Every instance has a [`PrintClass`] and a [`TagType`].
///
/// The markup associations the C++ object stored as raw AST pointers
/// (`ptr_second`, `op`) are carried as a resolved [`MarkupRef`] (see module
/// docs); `EmitNoMarkup` ignores them, `EmitMarkup` writes the same encoding.
#[derive(Debug, Clone)]
pub struct TokenSplit {
    tagtype: TagType,
    delimtype: PrintClass,
    tok: String,
    hl: SyntaxHighlight,
    markup: MarkupRef,
    addr: Option<AddrPayload>,
    off: uintb,
    indentbump: int4,
    numspaces: int4,
    size: int4,
    count: int4,
}

impl Default for TokenSplit {
    /// C++ `TokenSplit()` is a trivial constructor leaving members
    /// uninitialized; the slot is always assigned via one of the `begin*`/
    /// `tag*` methods before use.  The Rust default gives benign zeros.
    fn default() -> Self {
        TokenSplit {
            tagtype: TagType::SyntT,
            delimtype: PrintClass::Ignore,
            tok: String::new(),
            hl: SyntaxHighlight::NoColor,
            markup: MarkupRef::default(),
            addr: None,
            off: 0,
            indentbump: 0,
            numspaces: 0,
            size: 0,
            count: 0,
        }
    }
}

/// Static counter for uniquely assigning begin/end pair ids
/// (C++ `TokenSplit::countbase`).
///
/// C++ uses a process-wide `static int4 countbase`.  The port keeps the same
/// monotonic behavior with a thread-local cell (the emitter is single-threaded
/// per `PrintLanguage`); the absolute id values are never compared across
/// emitters, only matched begin/end within one stream.
fn next_countbase() -> int4 {
    thread_local! {
        static COUNTBASE: std::cell::Cell<int4> = const { std::cell::Cell::new(0) };
    }
    COUNTBASE.with(|c| {
        let v = c.get();
        c.set(v + 1);
        v
    })
}

impl TokenSplit {
    /// C++ `beginDocument()`.
    pub fn begin_document(&mut self) -> int4 {
        self.tagtype = TagType::DocuB;
        self.delimtype = PrintClass::Begin;
        self.size = 0;
        self.count = next_countbase();
        self.count
    }
    /// C++ `endDocument(int4 id)`.
    pub fn end_document(&mut self, id: int4) {
        self.tagtype = TagType::DocuE;
        self.delimtype = PrintClass::End;
        self.size = 0;
        self.count = id;
    }
    /// C++ `beginFunction(const Funcdata *f)`.
    pub fn begin_function(&mut self) -> int4 {
        self.tagtype = TagType::FuncB;
        self.delimtype = PrintClass::Begin;
        self.size = 0;
        self.count = next_countbase();
        self.count
    }
    /// C++ `endFunction(int4 id)`.
    pub fn end_function(&mut self, id: int4) {
        self.tagtype = TagType::FuncE;
        self.delimtype = PrintClass::End;
        self.size = 0;
        self.count = id;
    }
    /// C++ `beginBlock(const FlowBlock *b)`.  `blockref` is `b->getIndex()`.
    pub fn begin_block(&mut self, blockref: int4) -> int4 {
        self.tagtype = TagType::BlocB;
        self.delimtype = PrintClass::Ignore;
        self.markup.blockref = Some(blockref);
        self.count = next_countbase();
        self.count
    }
    /// C++ `endBlock(int4 id)`.
    pub fn end_block(&mut self, id: int4) {
        self.tagtype = TagType::BlocE;
        self.delimtype = PrintClass::Ignore;
        self.count = id;
    }
    /// C++ `beginReturnType(const Varnode *v)`.
    pub fn begin_return_type(&mut self, markup: &MarkupRef) -> int4 {
        self.tagtype = TagType::RtypB;
        self.delimtype = PrintClass::Begin;
        self.markup.return_varref = markup.return_varref;
        self.count = next_countbase();
        self.count
    }
    /// C++ `endReturnType(int4 id)`.
    pub fn end_return_type(&mut self, id: int4) {
        self.tagtype = TagType::RtypE;
        self.delimtype = PrintClass::End;
        self.count = id;
    }
    /// C++ `beginVarDecl(const Symbol *sym)`.
    pub fn begin_var_decl(&mut self, markup: &MarkupRef) -> int4 {
        self.tagtype = TagType::VardB;
        self.delimtype = PrintClass::Begin;
        self.markup.symref = markup.symref;
        self.count = next_countbase();
        self.count
    }
    /// C++ `endVarDecl(int4 id)`.
    pub fn end_var_decl(&mut self, id: int4) {
        self.tagtype = TagType::VardE;
        self.delimtype = PrintClass::End;
        self.count = id;
    }
    /// C++ `beginStatement(const PcodeOp *o)`.
    pub fn begin_statement(&mut self, markup: &MarkupRef) -> int4 {
        self.tagtype = TagType::StatB;
        self.delimtype = PrintClass::Begin;
        self.markup.opref = markup.opref;
        self.count = next_countbase();
        self.count
    }
    /// C++ `endStatement(int4 id)`.
    pub fn end_statement(&mut self, id: int4) {
        self.tagtype = TagType::StatE;
        self.delimtype = PrintClass::End;
        self.count = id;
    }
    /// C++ `beginFuncProto()`.
    pub fn begin_func_proto(&mut self) -> int4 {
        self.tagtype = TagType::ProtB;
        self.delimtype = PrintClass::Begin;
        self.count = next_countbase();
        self.count
    }
    /// C++ `endFuncProto(int4 id)`.
    pub fn end_func_proto(&mut self, id: int4) {
        self.tagtype = TagType::ProtE;
        self.delimtype = PrintClass::End;
        self.count = id;
    }
    /// C++ `tagVariable(name,h,v,o)`.
    pub fn tag_variable(&mut self, name: &str, h: SyntaxHighlight, markup: &MarkupRef) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::VariT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
        self.markup.varref = markup.varref;
        self.markup.opref = markup.opref;
    }
    /// C++ `tagOp(name,h,o)`.
    pub fn tag_op(&mut self, name: &str, h: SyntaxHighlight, markup: &MarkupRef) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::OpT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
        self.markup.opref = markup.opref;
    }
    /// C++ `tagFuncName(name,h,f,o)`.
    pub fn tag_func_name(&mut self, name: &str, h: SyntaxHighlight, markup: &MarkupRef) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::FnamT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
        self.markup.opref = markup.opref;
    }
    /// C++ `tagType(name,h,ct)`.
    pub fn tag_type(&mut self, name: &str, h: SyntaxHighlight, markup: &MarkupRef) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::TypeT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
        self.markup.type_id = markup.type_id;
    }
    /// C++ `tagField(name,h,ct,o,inOp)`.
    pub fn tag_field(&mut self, name: &str, h: SyntaxHighlight, o: int4, markup: &MarkupRef) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::FieldT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
        self.markup.type_id = markup.type_id;
        self.markup.type_name = markup.type_name.clone();
        self.off = o as uintb;
        self.markup.opref = markup.opref;
    }
    /// C++ `tagBitField(name,h,ct,id,inOp)`.
    pub fn tag_bit_field(&mut self, name: &str, h: SyntaxHighlight, id: int4, markup: &MarkupRef) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::BitfieldT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
        self.markup.type_id = markup.type_id;
        self.markup.type_name = markup.type_name.clone();
        self.off = id as uintb;
        self.markup.opref = markup.opref;
    }
    /// C++ `tagComment(name,h,s,o)`.
    pub fn tag_comment(&mut self, name: &str, h: SyntaxHighlight, spc: &Rc<AddrSpace>, o: uintb) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.addr = Some(AddrPayload { spc: Rc::clone(spc) });
        self.off = o;
        self.tagtype = TagType::CommT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
    }
    /// C++ `tagLabel(name,h,s,o)`.
    pub fn tag_label(&mut self, name: &str, h: SyntaxHighlight, spc: &Rc<AddrSpace>, o: uintb) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.addr = Some(AddrPayload { spc: Rc::clone(spc) });
        self.off = o;
        self.tagtype = TagType::LabelT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
    }
    /// C++ `tagCaseLabel(name,h,inOp,intValue)`.
    pub fn tag_case_label(&mut self, name: &str, h: SyntaxHighlight, markup: &MarkupRef, value: uintb) {
        self.tok = name.to_string();
        self.size = self.tok.len() as int4;
        self.markup.opref = markup.opref;
        self.off = value;
        self.tagtype = TagType::CaseT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
    }
    /// C++ `print(const string &data,h)`.
    pub fn print_tok(&mut self, data: &str, h: SyntaxHighlight) {
        self.tok = data.to_string();
        self.size = self.tok.len() as int4;
        self.tagtype = TagType::SyntT;
        self.delimtype = PrintClass::TokenString;
        self.hl = h;
    }
    /// C++ `openParen(paren,id)`.
    pub fn open_paren(&mut self, paren: &str, id: int4) {
        self.tok = paren.to_string();
        self.size = 1;
        self.tagtype = TagType::OparT;
        self.delimtype = PrintClass::TokenString;
        self.count = id;
    }
    /// C++ `closeParen(paren,id)`.
    pub fn close_paren(&mut self, paren: &str, id: int4) {
        self.tok = paren.to_string();
        self.size = 1;
        self.tagtype = TagType::CparT;
        self.delimtype = PrintClass::TokenString;
        self.count = id;
    }
    /// C++ `openGroup()`.
    pub fn open_group(&mut self) -> int4 {
        self.tagtype = TagType::OinvT;
        self.delimtype = PrintClass::Begin;
        self.count = next_countbase();
        self.count
    }
    /// C++ `closeGroup(int4 id)`.
    pub fn close_group(&mut self, id: int4) {
        self.tagtype = TagType::CinvT;
        self.delimtype = PrintClass::End;
        self.count = id;
    }
    /// C++ `startIndent(int4 bump)`.
    pub fn start_indent(&mut self, bump: int4) -> int4 {
        self.tagtype = TagType::BumpT;
        self.delimtype = PrintClass::BeginIndent;
        self.indentbump = bump;
        self.size = 0;
        self.count = next_countbase();
        self.count
    }
    /// C++ `stopIndent(int4 id)`.
    pub fn stop_indent(&mut self, id: int4) {
        self.tagtype = TagType::BumpT;
        self.delimtype = PrintClass::EndIndent;
        self.size = 0;
        self.count = id;
    }
    /// C++ `startComment()`.
    pub fn start_comment(&mut self) -> int4 {
        self.tagtype = TagType::OinvT;
        self.delimtype = PrintClass::BeginComment;
        self.count = next_countbase();
        self.count
    }
    /// C++ `stopComment(int4 id)`.
    pub fn stop_comment(&mut self, id: int4) {
        self.tagtype = TagType::CinvT;
        self.delimtype = PrintClass::EndComment;
        self.count = id;
    }
    /// C++ `spaces(int4 num,int4 bump)`.
    pub fn spaces(&mut self, num: int4, bump: int4) {
        self.tagtype = TagType::SpacT;
        self.delimtype = PrintClass::TokenBreak;
        self.numspaces = num;
        self.indentbump = bump;
    }
    /// C++ `tagLine()`.
    pub fn tag_line(&mut self) {
        self.tagtype = TagType::BumpT;
        self.delimtype = PrintClass::TokenBreak;
        self.numspaces = 999999;
        self.indentbump = 0;
    }
    /// C++ `tagLine(int4 indent)`.
    pub fn tag_line_indent(&mut self, indent: int4) {
        self.tagtype = TagType::LineT;
        self.delimtype = PrintClass::TokenBreak;
        self.numspaces = 999999;
        self.indentbump = indent;
    }

    /// C++ `getIndentBump()`.
    pub fn get_indent_bump(&self) -> int4 {
        self.indentbump
    }
    /// C++ `getNumSpaces()`.
    pub fn get_num_spaces(&self) -> int4 {
        self.numspaces
    }
    /// C++ `getSize()`.
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// C++ `setSize(int4 sz)`.
    pub fn set_size(&mut self, sz: int4) {
        self.size = sz;
    }
    /// C++ `getClass()`.
    pub fn get_class(&self) -> PrintClass {
        self.delimtype
    }
    /// C++ `getTag()`.
    pub fn get_tag(&self) -> TagType {
        self.tagtype
    }

    /// Send this token to a low-level emitter (C++ `TokenSplit::print(Emit *)`).
    ///
    /// `oinv_t`/`cinv_t` (invisible group delimiters) emit nothing; `line_t`/
    /// `bump_t` must never reach a low-level emitter (the pretty printer turns
    /// them into `tagLine` calls itself) and so throw, exactly as C++.
    pub fn print_to(&self, emit: &mut dyn Emit) -> KunaResult<()> {
        match self.tagtype {
            TagType::DocuB => {
                emit.begin_document();
            }
            TagType::DocuE => emit.end_document(self.count),
            TagType::FuncB => {
                emit.begin_function();
            }
            TagType::FuncE => emit.end_function(self.count),
            TagType::BlocB => {
                emit.begin_block(self.markup.blockref.unwrap_or(0));
            }
            TagType::BlocE => emit.end_block(self.count),
            TagType::RtypB => {
                emit.begin_return_type(&self.markup);
            }
            TagType::RtypE => emit.end_return_type(self.count),
            TagType::VardB => {
                emit.begin_var_decl(&self.markup);
            }
            TagType::VardE => emit.end_var_decl(self.count),
            TagType::StatB => {
                emit.begin_statement(&self.markup);
            }
            TagType::StatE => emit.end_statement(self.count),
            TagType::ProtB => {
                emit.begin_func_proto();
            }
            TagType::ProtE => emit.end_func_proto(self.count),
            TagType::VariT => emit.tag_variable(&self.tok, self.hl, &self.markup),
            TagType::OpT => emit.tag_op(&self.tok, self.hl, &self.markup),
            TagType::FnamT => emit.tag_func_name(&self.tok, self.hl, &self.markup),
            TagType::TypeT => emit.tag_type(&self.tok, self.hl, &self.markup),
            TagType::FieldT => emit.tag_field(&self.tok, self.hl, self.off as int4, &self.markup),
            TagType::BitfieldT => {
                emit.tag_bit_field(&self.tok, self.hl, self.off as int4, &self.markup)
            }
            TagType::CommT => {
                let addr = self.addr.as_ref().expect("comment token missing addr payload");
                emit.tag_comment(&self.tok, self.hl, &addr.spc, self.off);
            }
            TagType::LabelT => {
                let addr = self.addr.as_ref().expect("label token missing addr payload");
                emit.tag_label(&self.tok, self.hl, &addr.spc, self.off);
            }
            TagType::CaseT => emit.tag_case_label(&self.tok, self.hl, &self.markup, self.off),
            TagType::SyntT => emit.print(&self.tok, self.hl),
            TagType::OparT => {
                emit.open_paren(&self.tok, self.count);
            }
            TagType::CparT => emit.close_paren(&self.tok, self.count),
            TagType::OinvT => {} // Invisible open
            TagType::CinvT => {} // Invisible close
            TagType::SpacT => emit.spaces(self.numspaces, 0),
            TagType::LineT | TagType::BumpT => {
                return Err(KunaError::lowlevel("Should never get called"));
            }
        }
        Ok(())
    }

    /// C++ `getCount()` (only under `PRETTY_DEBUG`).
    pub fn get_count(&self) -> int4 {
        self.count
    }
}

// ===========================================================================
// EmitPrettyPrint
// ===========================================================================

/// Low-level back-end selection for [`EmitPrettyPrint`].
///
/// C++ holds `Emit *lowlevel` (an `EmitNoMarkup` or `EmitMarkup`) and
/// `delete`s/recreates it in `setMarkup`.  The port holds the concrete enum so
/// the output buffer is reachable for tests; `set_markup` swaps the variant.
#[derive(Debug)]
pub enum LowLevel {
    /// No-markup back-end (`print C`).
    NoMarkup(EmitNoMarkup),
    /// XML-markup back-end (`print C xml`).
    Markup(EmitMarkup),
}

impl LowLevel {
    fn as_emit_mut(&mut self) -> &mut dyn Emit {
        match self {
            LowLevel::NoMarkup(e) => e,
            LowLevel::Markup(e) => e,
        }
    }
    fn as_emit(&self) -> &dyn Emit {
        match self {
            LowLevel::NoMarkup(e) => e,
            LowLevel::Markup(e) => e,
        }
    }
    /// The accumulated text output (only meaningful for the no-markup back-end).
    pub fn output(&self) -> &str {
        match self {
            LowLevel::NoMarkup(e) => e.output(),
            LowLevel::Markup(_) => "",
        }
    }
}

/// A generic source-code pretty printer (C++ `EmitPrettyPrint`).
///
/// Based on the standard Derek C. Oppen pretty-printing algorithm.  Buffers a
/// stream of [`TokenSplit`]s in a [`CircularQueue`], assigns each a size, tracks
/// open-group / whitespace references in a second queue (`scanqueue`), and
/// decides where line breaks go so that lines stay within the maximum line size
/// (see [`get_max_line_size`](EmitPrettyPrint::get_max_line_size)) while
/// minimizing breaks in important groups.  This emitter wraps a lower-level
/// [`LowLevel`] back-end that does the final emitting (and optional markup).
///
/// The whole `scan`/`advanceleft`/`print`/`overflow` machinery is transcribed
/// verbatim because the inserted breaks are part of the byte-exact C output.
pub struct EmitPrettyPrint {
    base: EmitBase,
    lowlevel: LowLevel,
    /// Space available for currently active nesting levels.
    indentstack: Vec<int4>,
    /// Space remaining in current line.
    spaceremain: int4,
    /// Maximum number of characters allowed in a line.
    maxlinesize: int4,
    /// # of characters committed from the current line.
    leftotal: int4,
    /// # of characters yet to be committed from the current line.
    rightotal: int4,
    /// `true` if break needed before next token.
    needbreak: bool,
    /// `true` if in the middle of a comment.
    commentmode: bool,
    /// Used to fill comments if line breaks are forced.
    commentfill: String,
    /// References to current open and whitespace tokens.
    scanqueue: CircularQueue<int4>,
    /// The full stream of tokens.
    tokqueue: CircularQueue<TokenSplit>,
}

impl Default for EmitPrettyPrint {
    fn default() -> Self {
        EmitPrettyPrint::new()
    }
}

impl EmitPrettyPrint {
    /// C++ `EmitPrettyPrint::EmitPrettyPrint()`.
    ///
    /// Queues are sized `3*100`; the low-level back-end defaults to
    /// `EmitNoMarkup` ("Do not emit xml by default"); `resetDefaultsPrettyPrint`
    /// sets `maxlinesize` to 100.
    pub fn new() -> Self {
        let mut pp = EmitPrettyPrint {
            base: EmitBase::new(),
            lowlevel: LowLevel::NoMarkup(EmitNoMarkup::new()),
            indentstack: Vec::new(),
            spaceremain: 0,
            maxlinesize: 0,
            leftotal: 0,
            rightotal: 0,
            needbreak: false,
            commentmode: false,
            commentfill: String::new(),
            scanqueue: CircularQueue::new(3 * 100),
            tokqueue: CircularQueue::new(3 * 100),
        };
        // spaceremain = maxlinesize; (maxlinesize still 0 here in C++ field
        // order — it is set by resetDefaultsPrettyPrint below)
        pp.spaceremain = pp.maxlinesize;
        pp.needbreak = false;
        pp.commentmode = false;
        pp.reset_defaults_pretty_print();
        pp
    }

    /// Borrow the accumulated text output (no-markup back-end).
    pub fn output(&self) -> &str {
        self.lowlevel.output()
    }
    /// Borrow the low-level back-end.
    pub fn lowlevel(&self) -> &LowLevel {
        &self.lowlevel
    }
    /// Mutably borrow the low-level back-end.
    pub fn lowlevel_mut(&mut self) -> &mut LowLevel {
        &mut self.lowlevel
    }

    /// C++ `resetDefaultsPrettyPrint()`: `setMaxLineSize(100)`.
    fn reset_defaults_pretty_print(&mut self) {
        let _ = self.set_max_line_size(100);
    }

    /// C++ `EmitPrettyPrint::expand()`.
    ///
    /// Grow the token buffer (`+200`) and adjust the scanqueue references, which
    /// were relative to the old `tokqueue` left index.
    fn expand_buffers(&mut self) {
        let max = self.tokqueue.get_max();
        let left = self.tokqueue.bottomref();
        self.tokqueue.expand(200);
        // Expanding puts the leftmost element at reference 0, so adjust refs.
        for i in 0..max {
            let v = *self.scanqueue.get_ref(i);
            *self.scanqueue.get_ref_mut(i) = (v + max - left).rem_euclid(max);
        }
        self.scanqueue.expand(200);
    }

    /// C++ `EmitPrettyPrint::overflow()`.
    ///
    /// (Permanently) adjust the indent levels to guarantee at least half a line
    /// of space, and issue a line break.
    fn overflow(&mut self) {
        let half = self.maxlinesize / 2;
        for i in (0..self.indentstack.len()).rev() {
            if self.indentstack[i] < half {
                self.indentstack[i] = half;
            } else {
                break;
            }
        }
        let newspaceremain = if !self.indentstack.is_empty() {
            *self.indentstack.last().unwrap()
        } else {
            self.maxlinesize
        };
        if newspaceremain == self.spaceremain {
            return; // Line breaking doesn't give us any additional space
        }
        if self.commentmode && (newspaceremain == self.spaceremain + self.commentfill.len() as int4)
        {
            return; // Line breaking doesn't give us any additional space
        }
        self.spaceremain = newspaceremain;
        self.lowlevel
            .as_emit_mut()
            .tag_line_indent(self.maxlinesize - self.spaceremain);
        if self.commentmode && !self.commentfill.is_empty() {
            let fill = self.commentfill.clone();
            self.lowlevel
                .as_emit_mut()
                .print(&fill, SyntaxHighlight::CommentColor);
            self.spaceremain -= self.commentfill.len() as int4;
        }
    }

    /// C++ `EmitPrettyPrint::print(const TokenSplit &tok)`.
    ///
    /// Output the given token to the low-level emitter, adjusting the
    /// `indentstack` and `spaceremain` per the token's class.  The token is
    /// passed by reference index into [`tokqueue`] to avoid cloning; only the
    /// fields read here are touched.
    fn print_token(&mut self, tok_ref: int4) -> KunaResult<()> {
        // C++ `int4 val = 0;` — declared once, assigned in the indent/break
        // arms.  Each arm assigns before reading, so no `mut` is needed.
        let val;
        let class = self.tokqueue.get_ref(tok_ref).get_class();
        match class {
            PrintClass::Ignore => {
                // Markup or other that doesn't use space
                let tok = self.tokqueue.get_ref(tok_ref).clone();
                tok.print_to(self.lowlevel.as_emit_mut())?;
            }
            PrintClass::BeginIndent => {
                val = *self.indentstack.last().expect("begin_indent with empty indentstack")
                    - self.tokqueue.get_ref(tok_ref).get_indent_bump();
                self.indentstack.push(val);
            }
            PrintClass::BeginComment => {
                self.commentmode = true;
                // fallthru, treat as a group begin
                let tok = self.tokqueue.get_ref(tok_ref).clone();
                tok.print_to(self.lowlevel.as_emit_mut())?;
                self.indentstack.push(self.spaceremain);
            }
            PrintClass::Begin => {
                let tok = self.tokqueue.get_ref(tok_ref).clone();
                tok.print_to(self.lowlevel.as_emit_mut())?;
                self.indentstack.push(self.spaceremain);
            }
            PrintClass::EndIndent => {
                if self.indentstack.is_empty() {
                    return Err(KunaError::lowlevel("indent error"));
                }
                self.indentstack.pop();
            }
            PrintClass::EndComment => {
                self.commentmode = false;
                // fallthru, treat as a group end
                let tok = self.tokqueue.get_ref(tok_ref).clone();
                tok.print_to(self.lowlevel.as_emit_mut())?;
                self.indentstack.pop();
            }
            PrintClass::End => {
                let tok = self.tokqueue.get_ref(tok_ref).clone();
                tok.print_to(self.lowlevel.as_emit_mut())?;
                self.indentstack.pop();
            }
            PrintClass::TokenString => {
                if self.tokqueue.get_ref(tok_ref).get_size() > self.spaceremain {
                    self.overflow();
                }
                let tok = self.tokqueue.get_ref(tok_ref).clone();
                tok.print_to(self.lowlevel.as_emit_mut())?;
                self.spaceremain -= self.tokqueue.get_ref(tok_ref).get_size();
            }
            PrintClass::TokenBreak => {
                let tok = self.tokqueue.get_ref(tok_ref);
                if tok.get_size() > self.spaceremain {
                    if tok.get_tag() == TagType::LineT {
                        // Absolute indent
                        self.spaceremain = self.maxlinesize - tok.get_indent_bump();
                    } else {
                        // relative indent
                        val = *self.indentstack.last().expect("token_break with empty indentstack")
                            - tok.get_indent_bump();
                        // If creating a line break doesn't save that much
                        // don't do the line break
                        if (tok.get_num_spaces() <= self.spaceremain) && (val - self.spaceremain < 10)
                        {
                            let num = tok.get_num_spaces();
                            self.lowlevel.as_emit_mut().spaces(num, 0);
                            self.spaceremain -= num;
                            return Ok(());
                        }
                        *self.indentstack.last_mut().unwrap() = val;
                        self.spaceremain = val;
                    }
                    self.lowlevel
                        .as_emit_mut()
                        .tag_line_indent(self.maxlinesize - self.spaceremain);
                    if self.commentmode && !self.commentfill.is_empty() {
                        let fill = self.commentfill.clone();
                        self.lowlevel
                            .as_emit_mut()
                            .print(&fill, SyntaxHighlight::CommentColor);
                        self.spaceremain -= self.commentfill.len() as int4;
                    }
                } else {
                    let num = tok.get_num_spaces();
                    self.lowlevel.as_emit_mut().spaces(num, 0);
                    self.spaceremain -= num;
                }
            }
        }
        Ok(())
    }

    /// C++ `EmitPrettyPrint::advanceleft()`.
    ///
    /// Emit groups of tokens that have been fully committed (non-negative size)
    /// from the bottom of [`tokqueue`], updating [`leftotal`].
    fn advanceleft(&mut self) -> KunaResult<()> {
        let mut l = self.tokqueue.bottom().get_size();
        while l >= 0 {
            let bottom_ref = self.tokqueue.bottomref();
            let class = self.tokqueue.get_ref(bottom_ref).get_class();
            self.print_token(bottom_ref)?;
            match class {
                PrintClass::TokenBreak => {
                    self.leftotal += self.tokqueue.get_ref(bottom_ref).get_num_spaces();
                }
                PrintClass::TokenString => {
                    self.leftotal += l;
                }
                _ => {}
            }
            self.tokqueue.popbottom();
            if self.tokqueue.empty() {
                break;
            }
            l = self.tokqueue.bottom().get_size();
        }
        Ok(())
    }

    /// C++ `EmitPrettyPrint::scan()`.
    ///
    /// The heart of the algorithm.  Process the just-pushed top token: assign it
    /// a size, update the open/break scanqueue and the running totals, and force
    /// line breaks (via [`advanceleft`](EmitPrettyPrint::advanceleft)) when the
    /// line would overflow.
    fn scan(&mut self) -> KunaResult<()> {
        if self.tokqueue.empty() {
            // If we managed to overflow queue, expand it
            self.expand_buffers();
        }
        // Delay creating reference until after the possible expansion
        let top_ref = self.tokqueue.topref();
        let class = self.tokqueue.get_ref(top_ref).get_class();
        match class {
            PrintClass::BeginComment | PrintClass::Begin => {
                if self.scanqueue.empty() {
                    self.leftotal = 1;
                    self.rightotal = 1;
                }
                self.tokqueue.get_ref_mut(top_ref).set_size(-self.rightotal);
                let slot = self.scanqueue.push();
                *self.scanqueue.get_ref_mut(slot) = self.tokqueue.topref();
            }
            PrintClass::EndComment | PrintClass::End => {
                self.tokqueue.get_ref_mut(top_ref).set_size(0);
                if !self.scanqueue.empty() {
                    let ref_idx = self.scanqueue.pop();
                    let ref_tok_idx = *self.scanqueue.get_ref(ref_idx);
                    let newsize = self.tokqueue.get_ref(ref_tok_idx).get_size() + self.rightotal;
                    self.tokqueue.get_ref_mut(ref_tok_idx).set_size(newsize);
                    if (self.tokqueue.get_ref(ref_tok_idx).get_class() == PrintClass::TokenBreak)
                        && (!self.scanqueue.empty())
                    {
                        let ref2_idx = self.scanqueue.pop();
                        let ref2_tok_idx = *self.scanqueue.get_ref(ref2_idx);
                        let newsize2 =
                            self.tokqueue.get_ref(ref2_tok_idx).get_size() + self.rightotal;
                        self.tokqueue.get_ref_mut(ref2_tok_idx).set_size(newsize2);
                    }
                    if self.scanqueue.empty() {
                        self.advanceleft()?;
                    }
                }
            }
            PrintClass::TokenBreak => {
                if self.scanqueue.empty() {
                    self.leftotal = 1;
                    self.rightotal = 1;
                } else {
                    let ref_idx = self.scanqueue.topref();
                    let ref_tok_idx = *self.scanqueue.get_ref(ref_idx);
                    if self.tokqueue.get_ref(ref_tok_idx).get_class() == PrintClass::TokenBreak {
                        self.scanqueue.pop();
                        let newsize =
                            self.tokqueue.get_ref(ref_tok_idx).get_size() + self.rightotal;
                        self.tokqueue.get_ref_mut(ref_tok_idx).set_size(newsize);
                    }
                }
                self.tokqueue.get_ref_mut(top_ref).set_size(-self.rightotal);
                let slot = self.scanqueue.push();
                *self.scanqueue.get_ref_mut(slot) = self.tokqueue.topref();
                self.rightotal += self.tokqueue.get_ref(top_ref).get_num_spaces();
            }
            PrintClass::BeginIndent | PrintClass::EndIndent | PrintClass::Ignore => {
                self.tokqueue.get_ref_mut(top_ref).set_size(0);
            }
            PrintClass::TokenString => {
                if !self.scanqueue.empty() {
                    self.rightotal += self.tokqueue.get_ref(top_ref).get_size();
                    while self.rightotal - self.leftotal > self.spaceremain {
                        let ref_idx = self.scanqueue.popbottom();
                        let ref_tok_idx = *self.scanqueue.get_ref(ref_idx);
                        self.tokqueue.get_ref_mut(ref_tok_idx).set_size(999999);
                        self.advanceleft()?;
                        if self.scanqueue.empty() {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// C++ `EmitPrettyPrint::checkstart()`.
    fn checkstart(&mut self) -> KunaResult<()> {
        if self.needbreak {
            let slot = self.tokqueue.push();
            self.tokqueue.get_ref_mut(slot).spaces(0, 0);
            self.scan()?;
        }
        self.needbreak = false;
        Ok(())
    }

    /// C++ `EmitPrettyPrint::checkstring()`.
    fn checkstring(&mut self) -> KunaResult<()> {
        if self.needbreak {
            let slot = self.tokqueue.push();
            self.tokqueue.get_ref_mut(slot).spaces(0, 0);
            self.scan()?;
        }
        self.needbreak = true;
        Ok(())
    }

    /// C++ `EmitPrettyPrint::checkend()`.
    fn checkend(&mut self) -> KunaResult<()> {
        if !self.needbreak {
            let slot = self.tokqueue.push();
            self.tokqueue
                .get_ref_mut(slot)
                .print_tok(EMPTY_STRING, SyntaxHighlight::NoColor); // Add a blank string
            self.scan()?;
        }
        self.needbreak = true;
        Ok(())
    }

    /// C++ `EmitPrettyPrint::checkbreak()`.
    fn checkbreak(&mut self) -> KunaResult<()> {
        if !self.needbreak {
            let slot = self.tokqueue.push();
            self.tokqueue
                .get_ref_mut(slot)
                .print_tok(EMPTY_STRING, SyntaxHighlight::NoColor); // Add a blank string
            self.scan()?;
        }
        self.needbreak = false;
        Ok(())
    }

    // --- the fallible mirrors of the trait methods ---
    //
    // Every C++ `EmitPrettyPrint` tag method follows the same shape:
    //   check*(); push token; build token; scan();
    // `scan()` can throw (`LowlevelError`), so these are fallible.  The `Emit`
    // trait methods (below) wrap these, panicking on the internal-invariant
    // errors the way a C++ uncaught `LowlevelError` would abort.

    /// Fallible C++ `beginDocument()`.
    pub fn try_begin_document(&mut self) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_document();
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endDocument(int4 id)`.
    pub fn try_end_document(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_document(id);
        self.scan()
    }
    /// Fallible C++ `beginFunction(const Funcdata *fd)`.
    pub fn try_begin_function(&mut self) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_function();
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endFunction(int4 id)`.
    pub fn try_end_function(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_function(id);
        self.scan()
    }
    /// Fallible C++ `beginBlock(const FlowBlock *bl)`.
    pub fn try_begin_block(&mut self, blockref: int4) -> KunaResult<int4> {
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_block(blockref);
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endBlock(int4 id)`.
    pub fn try_end_block(&mut self, id: int4) -> KunaResult<()> {
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_block(id);
        self.scan()
    }
    /// Fallible C++ `tagLine()`.
    pub fn try_tag_line(&mut self) -> KunaResult<()> {
        // emitPending() (C++ EmitPrettyPrint::tagLine): fire a registered
        // PendingBrace before the line break.  `emit_pending` clears the pending
        // slot before opening the brace, so the brace's own tag_line does not
        // re-enter here.
        Emit::emit_pending(self);
        self.checkbreak()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_line();
        self.scan()
    }
    /// Fallible C++ `tagLine(int4 indent)`.
    pub fn try_tag_line_indent(&mut self, indent: int4) -> KunaResult<()> {
        self.checkbreak()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_line_indent(indent);
        self.scan()
    }
    /// Fallible C++ `beginReturnType(const Varnode *vn)`.
    pub fn try_begin_return_type(&mut self, markup: &MarkupRef) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_return_type(markup);
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endReturnType(int4 id)`.
    pub fn try_end_return_type(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_return_type(id);
        self.scan()
    }
    /// Fallible C++ `beginVarDecl(const Symbol *sym)`.
    pub fn try_begin_var_decl(&mut self, markup: &MarkupRef) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_var_decl(markup);
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endVarDecl(int4 id)`.
    pub fn try_end_var_decl(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_var_decl(id);
        self.scan()
    }
    /// Fallible C++ `beginStatement(const PcodeOp *op)`.
    pub fn try_begin_statement(&mut self, markup: &MarkupRef) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_statement(markup);
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endStatement(int4 id)`.
    pub fn try_end_statement(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_statement(id);
        self.scan()
    }
    /// Fallible C++ `beginFuncProto()`.
    pub fn try_begin_func_proto(&mut self) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).begin_func_proto();
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `endFuncProto(int4 id)`.
    pub fn try_end_func_proto(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).end_func_proto(id);
        self.scan()
    }
    /// Fallible C++ `tagVariable(...)`.
    pub fn try_tag_variable(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        markup: &MarkupRef,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_variable(name, hl, markup);
        self.scan()
    }
    /// Fallible C++ `tagOp(...)`.
    pub fn try_tag_op(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_op(name, hl, markup);
        self.scan()
    }
    /// Fallible C++ `tagFuncName(...)`.
    pub fn try_tag_func_name(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        markup: &MarkupRef,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_func_name(name, hl, markup);
        self.scan()
    }
    /// Fallible C++ `tagType(...)`.
    pub fn try_tag_type(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        markup: &MarkupRef,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_type(name, hl, markup);
        self.scan()
    }
    /// Fallible C++ `tagField(...)`.
    pub fn try_tag_field(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        o: int4,
        markup: &MarkupRef,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_field(name, hl, o, markup);
        self.scan()
    }
    /// Fallible C++ `tagBitField(...)`.
    pub fn try_tag_bit_field(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        id: int4,
        markup: &MarkupRef,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_bit_field(name, hl, id, markup);
        self.scan()
    }
    /// Fallible C++ `tagComment(...)`.
    pub fn try_tag_comment(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        spc: &Rc<AddrSpace>,
        off: uintb,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_comment(name, hl, spc, off);
        self.scan()
    }
    /// Fallible C++ `tagLabel(...)`.
    pub fn try_tag_label(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        spc: &Rc<AddrSpace>,
        off: uintb,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_label(name, hl, spc, off);
        self.scan()
    }
    /// Fallible C++ `tagCaseLabel(...)`.
    pub fn try_tag_case_label(
        &mut self,
        name: &str,
        hl: SyntaxHighlight,
        markup: &MarkupRef,
        value: uintb,
    ) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).tag_case_label(name, hl, markup, value);
        self.scan()
    }
    /// Fallible C++ `print(const string &data,hl)`.
    pub fn try_print(&mut self, data: &str, hl: SyntaxHighlight) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).print_tok(data, hl);
        self.scan()
    }
    /// Fallible C++ `openParen(const string &paren,int4 id)`.
    pub fn try_open_paren(&mut self, paren: &str, _id: int4) -> KunaResult<int4> {
        let id = self.try_open_group()?; // Open paren automatically opens group
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).open_paren(paren, id);
        self.scan()?;
        self.needbreak = true;
        Ok(id)
    }
    /// Fallible C++ `closeParen(const string &paren,int4 id)`.
    pub fn try_close_paren(&mut self, paren: &str, id: int4) -> KunaResult<()> {
        self.checkstring()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).close_paren(paren, id);
        self.scan()?;
        self.try_close_group(id)
    }
    /// Fallible C++ `openGroup()`.
    pub fn try_open_group(&mut self) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).open_group();
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `closeGroup(int4 id)`.
    pub fn try_close_group(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).close_group(id);
        self.scan()
    }
    /// Fallible C++ `startComment()`.
    pub fn try_start_comment(&mut self) -> KunaResult<int4> {
        self.checkstart()?;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).start_comment();
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `stopComment(int4 id)`.
    pub fn try_stop_comment(&mut self, id: int4) -> KunaResult<()> {
        self.checkend()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).stop_comment(id);
        self.scan()
    }
    /// Fallible C++ `spaces(int4 num,int4 bump)`.
    pub fn try_spaces(&mut self, num: int4, bump: int4) -> KunaResult<()> {
        self.checkbreak()?;
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).spaces(num, bump);
        self.scan()
    }
    /// Fallible C++ `startIndent()`.
    pub fn try_start_indent(&mut self) -> KunaResult<int4> {
        let bump = self.base.indentincrement;
        let slot = self.tokqueue.push();
        let id = self.tokqueue.get_ref_mut(slot).start_indent(bump);
        self.scan()?;
        Ok(id)
    }
    /// Fallible C++ `stopIndent(int4 id)`.
    pub fn try_stop_indent(&mut self, id: int4) -> KunaResult<()> {
        let slot = self.tokqueue.push();
        self.tokqueue.get_ref_mut(slot).stop_indent(id);
        self.scan()
    }

    /// C++ `EmitPrettyPrint::clear()`.
    pub fn clear_pretty(&mut self) {
        // Emit::clear()
        self.base.parenlevel = 0;
        self.base.indentlevel = 0;
        match &mut self.lowlevel {
            LowLevel::NoMarkup(e) => e.clear(),
            LowLevel::Markup(e) => e.clear(),
        }
        self.indentstack.clear();
        self.scanqueue.clear();
        self.tokqueue.clear();
        self.leftotal = 1;
        self.rightotal = 1;
        self.needbreak = false;
        self.commentmode = false;
        self.spaceremain = self.maxlinesize;
    }

    /// C++ `EmitPrettyPrint::flush()`.
    pub fn try_flush(&mut self) -> KunaResult<()> {
        while !self.tokqueue.empty() {
            let bottom_ref = self.tokqueue.bottomref();
            if self.tokqueue.get_ref(bottom_ref).get_size() < 0 {
                return Err(KunaError::lowlevel(
                    "Cannot flush pretty printer. Missing group end",
                ));
            }
            self.print_token(bottom_ref)?;
            self.tokqueue.popbottom();
        }
        self.needbreak = false;
        match &mut self.lowlevel {
            LowLevel::NoMarkup(e) => e.flush()?,
            LowLevel::Markup(e) => e.flush()?,
        }
        Ok(())
    }

    /// C++ `EmitPrettyPrint::setMarkup(bool val)`.
    pub fn set_markup_pretty(&mut self, val: bool) {
        // C++ preserves the bound ostream across the swap; the port preserves
        // nothing (a fresh buffer) because each back-end owns its own sink.
        self.lowlevel = if val {
            LowLevel::Markup(EmitMarkup::new())
        } else {
            LowLevel::NoMarkup(EmitNoMarkup::new())
        };
    }

    /// C++ `EmitPrettyPrint::setMaxLineSize(int4 val)`.
    pub fn set_max_line_size_pretty(&mut self, val: int4) -> KunaResult<()> {
        // C++ `if ((val<20)||(val>10000))` — the equivalent range check.
        if !(20..=10000).contains(&val) {
            return Err(KunaError::lowlevel("Bad maximum line size"));
        }
        self.maxlinesize = val;
        self.scanqueue.set_max(3 * val);
        self.tokqueue.set_max(3 * val);
        self.spaceremain = self.maxlinesize;
        self.clear_pretty();
        Ok(())
    }

    /// C++ `EmitPrettyPrint::resetDefaults()`.
    pub fn reset_defaults_pretty(&mut self) {
        match &mut self.lowlevel {
            LowLevel::NoMarkup(e) => e.reset_defaults(),
            LowLevel::Markup(e) => e.reset_defaults(),
        }
        self.base.reset_defaults_internal();
        self.reset_defaults_pretty_print();
    }
}

impl Emit for EmitPrettyPrint {
    fn state(&self) -> &EmitBase {
        &self.base
    }
    fn state_mut(&mut self) -> &mut EmitBase {
        &mut self.base
    }

    fn begin_document(&mut self) -> int4 {
        self.try_begin_document().expect("EmitPrettyPrint::beginDocument")
    }
    fn end_document(&mut self, id: int4) {
        self.try_end_document(id).expect("EmitPrettyPrint::endDocument")
    }
    fn begin_function(&mut self) -> int4 {
        self.try_begin_function().expect("EmitPrettyPrint::beginFunction")
    }
    fn end_function(&mut self, id: int4) {
        self.try_end_function(id).expect("EmitPrettyPrint::endFunction")
    }
    fn begin_block(&mut self, blockref: int4) -> int4 {
        self.try_begin_block(blockref).expect("EmitPrettyPrint::beginBlock")
    }
    fn end_block(&mut self, id: int4) {
        self.try_end_block(id).expect("EmitPrettyPrint::endBlock")
    }
    fn tag_line(&mut self) {
        self.try_tag_line().expect("EmitPrettyPrint::tagLine")
    }
    fn tag_line_indent(&mut self, indent: int4) {
        self.try_tag_line_indent(indent).expect("EmitPrettyPrint::tagLine")
    }
    fn begin_return_type(&mut self, markup: &MarkupRef) -> int4 {
        self.try_begin_return_type(markup).expect("EmitPrettyPrint::beginReturnType")
    }
    fn end_return_type(&mut self, id: int4) {
        self.try_end_return_type(id).expect("EmitPrettyPrint::endReturnType")
    }
    fn begin_var_decl(&mut self, markup: &MarkupRef) -> int4 {
        self.try_begin_var_decl(markup).expect("EmitPrettyPrint::beginVarDecl")
    }
    fn end_var_decl(&mut self, id: int4) {
        self.try_end_var_decl(id).expect("EmitPrettyPrint::endVarDecl")
    }
    fn begin_statement(&mut self, markup: &MarkupRef) -> int4 {
        self.try_begin_statement(markup).expect("EmitPrettyPrint::beginStatement")
    }
    fn end_statement(&mut self, id: int4) {
        self.try_end_statement(id).expect("EmitPrettyPrint::endStatement")
    }
    fn begin_func_proto(&mut self) -> int4 {
        self.try_begin_func_proto().expect("EmitPrettyPrint::beginFuncProto")
    }
    fn end_func_proto(&mut self, id: int4) {
        self.try_end_func_proto(id).expect("EmitPrettyPrint::endFuncProto")
    }
    fn tag_variable(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        self.try_tag_variable(name, hl, markup).expect("EmitPrettyPrint::tagVariable")
    }
    fn tag_op(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        self.try_tag_op(name, hl, markup).expect("EmitPrettyPrint::tagOp")
    }
    fn tag_func_name(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        self.try_tag_func_name(name, hl, markup).expect("EmitPrettyPrint::tagFuncName")
    }
    fn tag_type(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef) {
        self.try_tag_type(name, hl, markup).expect("EmitPrettyPrint::tagType")
    }
    fn tag_field(&mut self, name: &str, hl: SyntaxHighlight, off: int4, markup: &MarkupRef) {
        self.try_tag_field(name, hl, off, markup).expect("EmitPrettyPrint::tagField")
    }
    fn tag_bit_field(&mut self, name: &str, hl: SyntaxHighlight, id: int4, markup: &MarkupRef) {
        self.try_tag_bit_field(name, hl, id, markup).expect("EmitPrettyPrint::tagBitField")
    }
    fn tag_comment(&mut self, name: &str, hl: SyntaxHighlight, spc: &Rc<AddrSpace>, off: uintb) {
        self.try_tag_comment(name, hl, spc, off).expect("EmitPrettyPrint::tagComment")
    }
    fn tag_label(&mut self, name: &str, hl: SyntaxHighlight, spc: &Rc<AddrSpace>, off: uintb) {
        self.try_tag_label(name, hl, spc, off).expect("EmitPrettyPrint::tagLabel")
    }
    fn tag_case_label(&mut self, name: &str, hl: SyntaxHighlight, markup: &MarkupRef, value: uintb) {
        self.try_tag_case_label(name, hl, markup, value).expect("EmitPrettyPrint::tagCaseLabel")
    }
    fn print(&mut self, data: &str, hl: SyntaxHighlight) {
        self.try_print(data, hl).expect("EmitPrettyPrint::print")
    }
    fn open_paren(&mut self, paren: &str, id: int4) -> int4 {
        self.try_open_paren(paren, id).expect("EmitPrettyPrint::openParen")
    }
    fn close_paren(&mut self, paren: &str, id: int4) {
        self.try_close_paren(paren, id).expect("EmitPrettyPrint::closeParen")
    }
    fn open_group(&mut self) -> int4 {
        self.try_open_group().expect("EmitPrettyPrint::openGroup")
    }
    fn close_group(&mut self, id: int4) {
        self.try_close_group(id).expect("EmitPrettyPrint::closeGroup")
    }
    fn clear(&mut self) {
        self.clear_pretty();
    }
    fn set_packed_output(&mut self, val: bool) {
        self.lowlevel.as_emit_mut().set_packed_output(val);
    }
    fn spaces(&mut self, num: int4, bump: int4) {
        self.try_spaces(num, bump).expect("EmitPrettyPrint::spaces")
    }
    fn start_indent(&mut self) -> int4 {
        self.try_start_indent().expect("EmitPrettyPrint::startIndent")
    }
    fn stop_indent(&mut self, id: int4) {
        self.try_stop_indent(id).expect("EmitPrettyPrint::stopIndent")
    }
    fn start_comment(&mut self) -> int4 {
        self.try_start_comment().expect("EmitPrettyPrint::startComment")
    }
    fn stop_comment(&mut self, id: int4) {
        self.try_stop_comment(id).expect("EmitPrettyPrint::stopComment")
    }
    fn flush(&mut self) -> KunaResult<()> {
        self.try_flush()
    }
    fn set_max_line_size(&mut self, val: int4) -> KunaResult<()> {
        self.set_max_line_size_pretty(val)
    }
    fn get_max_line_size(&self) -> int4 {
        self.maxlinesize
    }
    fn set_comment_fill(&mut self, fill: &str) {
        self.commentfill = fill.to_string();
    }
    fn emits_markup(&self) -> bool {
        self.lowlevel.as_emit().emits_markup()
    }
    fn reset_defaults(&mut self) {
        self.reset_defaults_pretty();
    }
    fn set_markup(&mut self, val: bool) {
        self.set_markup_pretty(val);
    }
}

#[cfg(test)]
mod tests;
