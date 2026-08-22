//! (kuna) The rust-language policy objects: the profile and capability record
//! for Rust output.
//!
//! The type speller lives next door in [`crate::kuna_rusttypes`]; this file owns
//! the surface vocabulary and the capability record that tells the emitters what
//! Rust is willing to be handed.
//!
//! # The capabilities that matter, and why they are capabilities
//!
//! Three of kuna's rendering defaults are **on** and produce text that is not
//! Rust: `truthycond` (DIV-37) renders `x != 0` as `x`, which is not a condition
//! in a language without implicit bool conversion; `braceelide` (DIV-38) drops
//! the braces from a single-statement body, which Rust requires; and
//! `nullprinting` (DIV-35) emits `NULL`. Expressing those as `LangCaps` members
//! rather than telling an operator to flip three options is what makes selecting
//! Rust a single correct choice.
//!
//! `switch_captures_break: false` is the load-bearing one. A C `switch` captures
//! a bare `break`, so `kuna_scope_break` (`p8_structure/kuna_loopbreak_recovery.rs`,
//! DIV-10, default-on) legitimately retags a goto that targets the switch exit as
//! `f_break_goto`. A Rust `match` does **not** capture `break`: the same flag,
//! rendered naively inside a `match` arm inside a loop, breaks the *loop*. The
//! capability is what turns that from a latent miscompile into a gate.
//!
//! # Token precedences, and the one that is a correctness bug
//!
//! Most of the ~46 operator tokens are the right token in every language kuna
//! targets, so `rust_map_token` remaps only the eleven that are not. Two of
//! those are spelling (`~` becomes `!`, `&x` becomes `&mut x`); the rest are
//! **rank**, and rank is where a wrong table is dangerous rather than ugly.
//!
//! C ranks `& ^ |` BELOW `== !=`. Rust ranks them ABOVE. The parenthesizer
//! decides parentheses from precedence alone, so keeping C's numbers emits
//! `a | b == c` for `(a | b) == c` — which Rust reads as `a | (b == c)`. That is
//! a silent wrong answer, not a syntax error, and no amount of testing the
//! *spelling* would find it.
//!
//! Rust also puts every comparison at one non-associative level where C ranks
//! the relational operators above equality. Flattening them is what makes
//! `(a < b) != 0` parenthesise instead of emitting the chained comparison
//! `a < b != 0`, which does not parse. Because those become kuna-owned statics,
//! they carry their own `negate` links (`OpToken::negate`, the C++ field the port
//! left unset because a Rust static cannot reference itself — it can reference
//! its sibling), so `!(a < b)` still folds to `a >= b`.
//!
//! Ghidra's `BOOLEAN_XOR` is the invented C spelling `^^` at precedence 20,
//! between `&&` and `||`. Rust has one `^` for both logical and bitwise xor, so
//! the Rust token is `^` at the bitwise tier.

use crate::kuna_lang::{
    CastForm, CharForm, DeclForm, DoWhileForm, ForLoopForm, GotoForm, InfLoopForm, LabelForm, LangCaps,
    LangForms, LangProfile, MemberForm, ProtoForm, StringEscape, SwitchForm, WhileForm,
};
use crate::printlanguage::{OpToken, TokenType};

/// The Rust operator tokens whose spelling or precedence differs from C.
/// Everything else is inherited from `printc::tokens` unchanged.
pub mod rust_tokens {
    use super::{OpToken, TokenType};

    /// One binary token, spelled and ranked for Rust.
    const fn bin(
        print1: &'static str,
        precedence: i32,
        associative: bool,
        negate: Option<&'static OpToken>,
    ) -> OpToken {
        OpToken {
            print1,
            print2: "",
            stage: 2,
            precedence,
            associative,
            token_type: TokenType::Binary,
            spacing: 1,
            bump: 0,
            negate,
            paren_before_angle: false,
        }
    }

    // Rust ranks `& ^ |` ABOVE the comparisons; C ranks them below. These sit
    // between the shifts (46) and the comparison tier (38), preserving their
    // order relative to each other.
    pub static BITWISE_AND: OpToken = bin("&", 44, true, None);
    pub static BITWISE_XOR: OpToken = bin("^", 43, true, None);
    pub static BITWISE_OR: OpToken = bin("|", 42, true, None);

    // Rust puts every comparison at ONE non-associative level, where C ranks the
    // relational operators (42) above equality (38). Flattening them to 38 is
    // what makes `(a < b) != 0` parenthesise instead of emitting the chained
    // comparison `a < b != 0`, which does not parse. The `negate` links are the
    // pairs `token_negate` resolves by pointer identity for C.
    pub static LESS_THAN: OpToken = bin("<", 38, false, Some(&GREATER_EQUAL));
    pub static LESS_EQUAL: OpToken = bin("<=", 38, false, Some(&GREATER_THAN));
    pub static GREATER_THAN: OpToken = bin(">", 38, false, Some(&LESS_EQUAL));
    pub static GREATER_EQUAL: OpToken = bin(">=", 38, false, Some(&LESS_THAN));

    /// Rust spells bitwise complement `!`, not `~`; same prefix-unary tier.
    pub static BITWISE_NOT: OpToken = OpToken {
        print1: "!",
        print2: "",
        stage: 1,
        precedence: 62,
        associative: false,
        token_type: TokenType::UnaryPrefix,
        spacing: 0,
        bump: 0,
        negate: None,
        paren_before_angle: false,
    };

    /// Rust spells logical xor `^`, which is the same operator as the bitwise
    /// one and sits at the same tier -- not at Ghidra's invented `^^` tier (20).
    pub static BOOLEAN_XOR: OpToken = bin("^", 43, true, None);

    /// Rust has no bare address-of: `&x` is a shared borrow, and a decompiled
    /// address is a raw pointer. `&mut x` is the honest analogue and coerces to
    /// `*mut T`, which is how the type speller spells every recovered pointer.
    pub static ADDRESSOF_MUT: OpToken = OpToken {
        print1: "&mut ",
        print2: "",
        stage: 1,
        precedence: 62,
        associative: false,
        token_type: TokenType::UnaryPrefix,
        spacing: 0,
        bump: 0,
        negate: None,
        paren_before_angle: false,
    };

    /// `expr as T` — postfix-typed rather than C's prefix `(T)expr`. Binds
    /// tighter than the multiplicative operators (54) and looser than unary (62).
    pub static TYPECAST_AS: OpToken = OpToken {
        print1: " as ",
        print2: "",
        stage: 2,
        precedence: 58,
        associative: false,
        token_type: TokenType::Binary,
        spacing: 0,
        bump: 0,
        negate: None,
        // `x as i32 < 5` does not parse; see `OpToken::paren_before_angle`.
        paren_before_angle: true,
    };
}

/// Rust's token mapping: the ~40 tokens whose spelling AND rank are already
/// right pass through; these ten do not.
fn rust_map_token(tok: &'static OpToken) -> &'static OpToken {
    use crate::printc::tokens as c;
    for (from, to) in [
        (&c::BITWISE_NOT, &rust_tokens::BITWISE_NOT),
        (&c::BOOLEAN_XOR, &rust_tokens::BOOLEAN_XOR),
        (&c::BITWISE_AND, &rust_tokens::BITWISE_AND),
        (&c::BITWISE_XOR, &rust_tokens::BITWISE_XOR),
        (&c::BITWISE_OR, &rust_tokens::BITWISE_OR),
        (&c::LESS_THAN, &rust_tokens::LESS_THAN),
        (&c::LESS_EQUAL, &rust_tokens::LESS_EQUAL),
        (&c::GREATER_THAN, &rust_tokens::GREATER_THAN),
        (&c::GREATER_EQUAL, &rust_tokens::GREATER_EQUAL),
        (&c::ADDRESSOF, &rust_tokens::ADDRESSOF_MUT),
        (&c::TYPECAST, &rust_tokens::TYPECAST_AS),
    ] {
        if std::ptr::eq(tok, from) {
            return to;
        }
    }
    tok
}

/// The rust-language profile.
///
/// Keywords with no Rust spelling (`case`, `goto`, `do`, `for`) are the empty
/// string; the matching [`LangCaps`] member forbids the form that would emit
/// them, so an empty spelling can never reach the output.
pub static LANG_RUST: LangProfile = LangProfile {
    name: "rust-language",
    file_ext: "rs",

    kw_open_curly: "{",
    kw_close_curly: "}",
    kw_semicolon: ";",
    kw_colon: ":",
    kw_comma: ",",
    // Rust variadics are `...` only in `extern` blocks; a recovered varargs
    // prototype spells the same.
    kw_dotdotdot: "...",
    kw_void: "()",
    kw_true: "true",
    kw_false: "false",
    kw_if: "if",
    kw_else: "else",
    // No `do`/`for`/`goto`/`case`: see `caps`.
    kw_do: "",
    kw_while: "while",
    kw_for: "",
    kw_goto: "",
    kw_break: "break",
    kw_continue: "continue",
    kw_case: "",
    kw_switch: "match",
    kw_default: "_",
    kw_return: "return",
    // `ADJ` is a Ghidra display macro for a `TypePointerRel`, not a language
    // construct; it renders the same in any output.
    kw_type_pointer_rel: "ADJ",
    null_literal: "core::ptr::null_mut()",
    sanitize_identifiers: true,

    map_token: rust_map_token,
    tok_typecast: &rust_tokens::TYPECAST_AS,

    caps: LangCaps {
        goto: false,
        switch_captures_break: false,
        switch_falls_through: false,
        switch_multi_label: true,
        switch_default_optional: false,
        labeled_loop_break: true,
        labeled_block_break: true,
        do_while: false,
        c_for: false,
        ternary: false,
        comma_expression: false,
        implicit_bool_conditions: false,
        brace_elision: false,
        arrow_member: false,
        paren_conditions: false,
        integer_suffixes: false,
    },
    forms: LangForms {
        proto: ProtoForm::RustFnArrow,
        decl: DeclForm::RustLetColon,
        switch: SwitchForm::RustMatch,
        inf_loop: InfLoopForm::RustLoop,
        while_loop: WhileForm::RustBareWhile,
        do_while: DoWhileForm::RustLoopBreakIf,
        for_loop: ForLoopForm::DegradeToWhile,
        goto: GotoForm::Unrepresentable,
        label: LabelForm::CommentOnly,
        member: MemberForm::RustDerefParen,
        cast: CastForm::PostfixAs,
        char_lit: CharForm::RustByte,
        string_escape: StringEscape::RustEscapes,
    },
    // Decompiled output does not follow Rust naming or use every binding; the
    // attribute keeps a reader's tooling quiet without changing the code.
    fn_attributes: &["#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]"],
};

#[cfg(test)]
mod tests;

// ===========================================================================
// The rust-language emitters.
//
// These are `impl PrintC` blocks rather than a separate printer: the RPN driver,
// the op emitters, the parenthesizer, the cast plumbing and the comment sorter
// are shared verbatim, and only the ~10 *shapes* below differ. Each is reached
// through a `LangForms` match in `printc.rs`, so the C path is a different arm
// and cannot be perturbed by anything written here.
// ===========================================================================

use kuna_base::types::int4;

use crate::architecture::Architecture;
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::prettyprint::{Emit, MarkupRef, SyntaxHighlight};
use crate::printc::{declarator_parts, display_format, to_emit_brace, PrintC};
use crate::printlanguage::modifiers;

impl PrintC {
    /// `unsafe fn name(mut a0: T, ...) -> R`, the Rust counterpart of
    /// `emit_prototype_declaration_c`'s `<ret> name(<params>)`.
    ///
    /// `unsafe` because a decompiled body dereferences raw pointers; an
    /// `unsafe fn`'s body carries them without an inner block, so no extra brace
    /// level appears. `mut` on every parameter because a decompiled body assigns
    /// to its parameter slots freely, and the recovery does not distinguish the
    /// ones that do.
    ///
    /// A unit return is omitted rather than spelled `-> ()`, which is what a Rust
    /// reader expects.
    pub(crate) fn emit_prototype_declaration_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        markup: &MarkupRef,
    ) {
        let display = fd.get_display_name().to_string();
        let rt = self.rt_ctx;
        let ret_type = if fd.get_func_proto().has_store() {
            fd.get_func_proto()
                .get_output_type()
                .map(|t| declarator_parts(t, rt).0)
                .unwrap_or_else(|| "()".to_string())
        } else {
            "()".to_string()
        };

        // The recovered name is often a demangled Rust path, which is not an
        // identifier. Rewrite it and keep the original above, so the mapping back
        // to the symbol is never lost.
        let ident = crate::kuna_rusttypes::sanitize(&display);
        if ident != display {
            self.emit.tag_line();
            self.emit.print(&format!("// {display}"), SyntaxHighlight::CommentColor);
        }
        let display = ident;
        for attr in self.lang().fn_attributes {
            self.emit.tag_line();
            self.emit.print(attr, SyntaxHighlight::CommentColor);
        }
        if !self.lang().fn_attributes.is_empty() {
            self.emit.tag_line();
        }

        let idp = self.emit.begin_func_proto();
        // A C-variadic parameter is only legal on an `unsafe extern "C" fn`;
        // rustc rejects it on a plain `unsafe fn` (a semantic rule, so a
        // token-level parser accepts the shorter form and rustc does not).
        let kw = if fd.get_func_proto().is_dotdotdot() {
            "unsafe extern \"C\" fn"
        } else {
            "unsafe fn"
        };
        self.emit.print(kw, SyntaxHighlight::KeywordColor);
        self.emit.spaces(1, 0);
        let id1g = self.emit.open_group();
        self.emit.tag_func_name(&display, SyntaxHighlight::FuncnameColor, markup);
        let id2 = self.emit.open_paren("(", 0);
        self.emit_prototype_inputs_rust(fd, arch, markup);
        self.emit.close_paren(")", id2);
        self.emit.close_group(id1g);
        if ret_type != "()" {
            self.emit.spaces(1, 0);
            self.emit.print("->", SyntaxHighlight::NoColor);
            self.emit.spaces(1, 0);
            let idret = self.emit.begin_return_type(markup);
            self.emit.tag_type(&ret_type, SyntaxHighlight::TypeColor, markup);
            self.emit.end_return_type(idret);
        }
        self.emit.end_func_proto(idp);
    }

    /// `mut a0: T, mut a1: U` — an empty list stays empty rather than becoming
    /// C's explicit `void`.
    fn emit_prototype_inputs_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        markup: &MarkupRef,
    ) {
        let proto = fd.get_func_proto();
        if !proto.has_store() {
            return;
        }
        let sz = proto.num_params();
        let mut print_comma = false;
        for i in 0..sz {
            let Some(param) = proto.get_param(i) else { continue };
            if print_comma {
                self.emit.print(",", SyntaxHighlight::NoColor);
                self.emit.spaces(1, 0);
            }
            print_comma = true;
            let default_name;
            let mut name = param.get_name();
            if name.is_empty() {
                default_name = if arch.name_style_angr {
                    crate::database::kuna_arg_name(i)
                } else {
                    format!("param_{}", i + 1)
                };
                name = default_name.as_str();
            }
            let ty = match param.get_type() {
                Some(ty) => declarator_parts(ty, self.rt_ctx).0,
                None => "()".to_string(),
            };
            self.emit.print("mut", SyntaxHighlight::KeywordColor);
            self.emit.spaces(1, 0);
            self.emit.tag_variable(name, SyntaxHighlight::VarColor, markup);
            self.emit.print(":", SyntaxHighlight::NoColor);
            self.emit.spaces(1, 0);
            self.emit.tag_type(&ty, SyntaxHighlight::TypeColor, markup);
        }
        if proto.is_dotdotdot() {
            if sz != 0 {
                self.emit.print(",", SyntaxHighlight::NoColor);
                self.emit.spaces(1, 0);
            }
            self.emit.print("...", SyntaxHighlight::NoColor);
        }
    }

    /// `loop { ... }` — where C emits `do { ... } while( true );`.
    pub(crate) fn emit_block_inf_loop_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        self.emit.tag_line();
        self.emit.print("loop", SyntaxHighlight::KeywordColor);
        let id =
            self.emit.open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_loop));
        let body = fd.sblocks_ref().block(blk).get_block(0);
        let id1 = self.emit.begin_block(0);
        self.emit_block(fd, arch, body);
        self.emit.end_block(id1);
        self.emit.close_brace_indent(self.lang().kw_close_curly, id);
        self.flush_eol_warnings();
        self.context.pop_mod();
    }

    /// `loop { body; if !(cond) { break; } }` — Rust has no bottom-tested loop.
    ///
    /// The condition is emitted through the same `ONLY_BRANCH` path C uses for
    /// its `while (cond)` tail, wrapped in a `BOOLEAN_NOT` so the sense is the
    /// *exit* test rather than the continue test.
    pub(crate) fn emit_block_do_while_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        self.emit.tag_line();
        self.emit.print("loop", SyntaxHighlight::KeywordColor);
        let id =
            self.emit.open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_loop));
        let body = fd.sblocks_ref().block(blk).get_block(0);
        self.context.push_mod();
        let id2 = self.emit.begin_block(0);
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, body);
        self.emit.end_block(id2);
        self.context.pop_mod();

        // `if !(cond) { break; }`
        self.emit.tag_line();
        self.emit.tag_op(self.lang().kw_if, SyntaxHighlight::KeywordColor, &MarkupRef::none());
        self.emit.spaces(1, 0);
        self.emit.print("!", SyntaxHighlight::NoColor);
        let idp = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
        self.context.push_mod();
        self.context.set_mod(modifiers::ONLY_BRANCH);
        self.emit_block(fd, arch, body);
        self.context.pop_mod();
        self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, idp);
        let idb =
            self.emit.open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_loop));
        self.emit.tag_line();
        self.emit.print(self.lang().kw_break, SyntaxHighlight::KeywordColor);
        self.emit.print(self.lang().kw_semicolon, SyntaxHighlight::NoColor);
        self.emit.close_brace_indent(self.lang().kw_close_curly, idb);

        self.emit.close_brace_indent(self.lang().kw_close_curly, id);
        self.flush_eol_warnings();
        self.context.pop_mod();
    }

    /// Rust has no statement labels, only loop labels, so a jump target is
    /// recorded as a comment. It is deliberately still emitted: the label names
    /// a place some `goto` marker refers to, and dropping it would leave that
    /// marker pointing at nothing.
    pub(crate) fn emit_label_statement_rust(&mut self, fd: &Funcdata, bl: BlockId) {
        use crate::block::BlockType;
        if self.context.is_set(modifiers::ONLY_BRANCH) {
            return;
        }
        if !fd.sblocks_ref().block(bl).is_unstructured_target() {
            return;
        }
        if fd.sblocks_ref().block(bl).get_type() != BlockType::Copy {
            return;
        }
        let name = self.block_label_name(fd, bl);
        self.emit.tag_line_indent(0);
        self.emit.print(&format!("// {name}:"), SyntaxHighlight::CommentColor);
    }
}

impl PrintC {
    /// `let mut name: T;` — where C emits `T name;`.
    ///
    /// The array count moves *into* the type (`[T; N]`) rather than trailing the
    /// identifier: `decl_type` is already the Rust prefix spelling of the element
    /// type, so the count is wrapped around it here. This is the same inversion
    /// that makes `TypeSpeller::declarator` return an empty `back`.
    pub(crate) fn emit_var_decl_rust(
        &mut self,
        name: &str,
        decl_type: &str,
        array_count: Option<i32>,
        markup: &MarkupRef,
    ) {
        self.emit.print("let mut", SyntaxHighlight::KeywordColor);
        self.emit.spaces(1, 0);
        self.emit.tag_variable(name, SyntaxHighlight::VarColor, markup);
        self.emit.print(":", SyntaxHighlight::NoColor);
        self.emit.spaces(1, 0);
        match array_count {
            Some(n) => {
                let ty = format!("[{decl_type}; {n}]");
                self.emit.tag_type(&ty, SyntaxHighlight::TypeColor, markup);
            }
            None => self.emit.tag_type(decl_type, SyntaxHighlight::TypeColor, markup),
        }
    }
}

impl PrintC {
    /// A jump the target language cannot express.
    ///
    /// The structurer manufactures `goto`s as its escape hatch, and Rust has no
    /// form for one that is neither a `break` nor a `continue`. Rather than emit
    /// a `goto` into a document called Rust -- which is what the reference
    /// implementation does -- kuna emits a comment naming the construct plus a
    /// **diverging** `panic!`.
    ///
    /// Diverging because `!` coerces to any type, so a jump in expression or
    /// value position still type-checks and the document still parses. Loud
    /// because a bare comment is silently wrong: a reader would see code that
    /// looks like a translation and is not. And greppable, because
    /// `panic!("kuna: unstructured goto` counted over a whole-binary render IS
    /// the quality number for this back-end -- exactly what `gotoreduce`,
    /// `taildup`, `ifelseflatten` and `crossjumprevert` reduce.
    pub(crate) fn emit_unrepresentable_goto(&mut self, fd: &Funcdata, target: BlockId) {
        let name = self.block_label_name(fd, target);
        self.emit.print(
            &format!("/* kuna: unstructured goto to {name} */"),
            SyntaxHighlight::CommentColor,
        );
        self.emit.spaces(1, 0);
        self.emit.print(
            &format!("panic!(\"kuna: unstructured goto to {name}\")"),
            SyntaxHighlight::NoColor,
        );
    }
}

impl PrintC {
    /// `while cond { ... }`, and `loop { ...; if cond { break; } ... }` for the
    /// overflow form.
    ///
    /// The overflow shape is not a Rust-specific invention: kuna already renders
    /// a loop whose head is too complex to sit inside `while (...)` as
    /// `while( true ) { cond-body; if (cond) break; body }` (`f_whiledo_overflow`,
    /// set at fold time by the structurer). That IS the Rust shape -- only
    /// `while( true )` becomes `loop`. Nothing about the polarity is re-decided
    /// here, which matters: the loop-continue edge's sense was settled when the
    /// loop was folded, and flipping it at print time would emit a loop that
    /// breaks where it should continue.
    ///
    /// A C-style `for` header is deliberately not reached: `analyze_for_loops` is
    /// forced off for a language without one, so the initializer and increment
    /// stay where the CFG put them. Moving them back at print time would let a
    /// `continue` in the body skip the increment.
    pub(crate) fn emit_block_while_do_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        let cond_block = fd.sblocks_ref().block(blk).get_block(0);
        let indent;
        if fd.sblocks_ref().block(blk).has_overflow_syntax() {
            self.emit.tag_line();
            self.emit.print("loop", SyntaxHighlight::KeywordColor);
            indent = self
                .emit
                .open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_loop));
            self.context.push_mod();
            self.context.set_mod(modifiers::NO_BRANCH);
            self.emit_block(fd, arch, cond_block);
            self.context.pop_mod();
            self.emit.tag_line();
            self.emit.tag_op(self.lang().kw_if, SyntaxHighlight::KeywordColor, &MarkupRef::none());
            self.emit.spaces(1, 0);
            self.context.push_mod();
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, cond_block);
            self.context.pop_mod();
            let idb = self
                .emit
                .open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_ifelse));
            self.emit.tag_line();
            self.emit.print(self.lang().kw_break, SyntaxHighlight::KeywordColor);
            self.emit.print(self.lang().kw_semicolon, SyntaxHighlight::NoColor);
            self.emit.close_brace_indent(self.lang().kw_close_curly, idb);
        } else {
            self.emit_comment_block_tree(fd, cond_block);
            self.emit.tag_line();
            self.emit.tag_op(self.lang().kw_while, SyntaxHighlight::KeywordColor, &MarkupRef::none());
            self.emit.spaces(1, 0);
            // (kuna outlang) C renders a condition that carries statements as a
            // comma expression -- `while (v = f(), v != 0)`. Rust has no comma
            // operator, but its block expression is exactly the same thing with
            // braces: `while { v = f(); v != 0 } { ... }`. The two emissions of
            // the condition block below are the ones C already performs for its
            // overflow form -- statements under NO_BRANCH, the test under
            // ONLY_BRANCH -- so nothing about the condition is re-derived here.
            let block_cond = self.cond_block_emits_statements(fd, cond_block);
            let id1 = if block_cond {
                let id = self
                    .emit
                    .open_brace_indent(self.lang().kw_open_curly, crate::prettyprint::BraceStyle::SameLine);
                self.context.push_mod();
                self.context.set_mod(modifiers::NO_BRANCH);
                self.emit_block(fd, arch, cond_block);
                self.context.pop_mod();
                self.emit.tag_line();
                Some(id)
            } else {
                None
            };
            let grp = self.emit.open_group();
            self.context.push_mod();
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, cond_block);
            self.context.pop_mod();
            self.emit.close_group(grp);
            if let Some(id) = id1 {
                self.emit.close_brace_indent(self.lang().kw_close_curly, id);
            }
            indent = self
                .emit
                .open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_loop));
            self.flush_eol_warnings();
        }
        self.context.set_mod(modifiers::NO_BRANCH);
        let id2 = self.emit.begin_block(0);
        self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(1));
        self.emit.end_block(id2);
        self.emit.close_brace_indent(self.lang().kw_close_curly, indent);
        self.context.pop_mod();
    }
}

impl PrintC {
    /// `match v { A | B => { ... } _ => {} }`.
    ///
    /// Three things differ from C's `switch`, and each is a property of `match`
    /// rather than a stylistic choice:
    ///
    /// * **Arms do not fall out**, so the explicit `break;` C emits for a case
    ///   that reaches the switch exit is dropped.
    /// * **A `match` on an integer must be exhaustive**, so a `_` arm is emitted
    ///   unconditionally -- synthesised as empty when the recovered switch had no
    ///   `default`.
    /// * **A wildcard must come last**, or the arms after it are unreachable.
    ///   `finalize_one_switch` sorts cases by label, so the default lands wherever
    ///   its label puts it; it is hoisted here. Reordering is safe because the
    ///   remaining patterns are disjoint integer literals.
    ///
    /// Multi-label arms are free: `emit_switch_case` already enumerates one
    /// `case N:` per jump-table index targeting the same block, so the same list
    /// joins with ` | `. Note that the ordinary source shape `case A: case B:
    /// body` produces ONE `CaseOrder` with two indices -- it is not a fall-through
    /// chain, so the common case never reaches the marker below.
    ///
    /// A genuine fall-through (a case with code that flows into the next) has no
    /// `match` form without duplicating the tail, which is a block-graph edit and
    /// not a printing decision. Those arms carry the same loud marker a residual
    /// goto does.
    pub(crate) fn emit_block_match_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        let switch_block = fd.sblocks_ref().block(blk).get_block(0);

        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, switch_block);
        self.context.pop_mod();
        self.emit.tag_line();
        // The `match v` header: the BRANCHIND arm emits the keyword and, for a
        // language without parenthesised conditions, the bare scrutinee.
        self.context.push_mod();
        self.context.set_mod(modifiers::ONLY_BRANCH | modifiers::COMMA_SEPARATE);
        self.emit_block(fd, arch, switch_block);
        self.context.pop_mod();
        let brace_id = self
            .emit
            .open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_switch));
        self.flush_eol_warnings();

        let ncase = fd.sblocks_ref().block(blk).switch_caseblocks().len();
        let mut default_case: Option<usize> = None;
        for i in 0..ncase {
            if fd.sblocks_ref().block(blk).switch_caseblocks()[i].isdefault {
                default_case = Some(i);
                continue;
            }
            self.emit_match_arm_rust(fd, arch, blk, i);
        }
        match default_case {
            Some(i) => self.emit_match_arm_rust(fd, arch, blk, i),
            None => {
                // `match` on an integer is never exhaustive without a wildcard;
                // falling out of the C switch is "continue past it", which is an
                // empty arm.
                self.emit.tag_line();
                self.emit.print(self.lang().kw_default, SyntaxHighlight::KeywordColor);
                self.emit.spaces(1, 0);
                self.emit.print("=>", SyntaxHighlight::NoColor);
                self.emit.spaces(1, 0);
                self.emit.print("{}", SyntaxHighlight::NoColor);
            }
        }

        self.emit.tag_line();
        self.emit.close_brace_indent(self.lang().kw_close_curly, brace_id);
        self.context.pop_mod();
    }

    /// One `A | B => { ... }` arm.
    fn emit_match_arm_rust(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
        casenum: usize,
    ) {
        let case = fd.sblocks_ref().block(blk).switch_caseblocks()[casenum].clone();
        let firstop = self.case_first_op(fd, case.block);

        self.emit.tag_line();
        if case.isdefault {
            self.emit.print(self.lang().kw_default, SyntaxHighlight::KeywordColor);
        } else {
            let jt_index = fd.sblocks_ref().block(blk).switch_jt_index();
            let nlabels = match (jt_index, case.basicblock) {
                (Some(j), Some(bb)) => {
                    fd.get_jump_table(j as int4).num_indices_by_block(fd, bb).unwrap_or(1).max(1)
                }
                _ => 1,
            };
            let signed = jt_index
                .map(|j| fd.get_jump_table(j as int4).kuna_has_signed_labels())
                .unwrap_or(false);
            let sz = self.switch_var_size(fd, blk);
            for i in 0..nlabels {
                if i != 0 {
                    self.emit.spaces(1, 0);
                    self.emit.print("|", SyntaxHighlight::NoColor);
                    self.emit.spaces(1, 0);
                }
                let val = match (jt_index, case.basicblock) {
                    (Some(j), Some(bb)) => {
                        let ind =
                            fd.get_jump_table(j as int4).get_index_by_block(fd, bb, i).unwrap_or(0);
                        fd.get_jump_table(j as int4).get_label_by_index(ind)
                    }
                    _ => case.label,
                };
                match firstop.or_else(|| self.any_op(fd, case.block)) {
                    Some(op) => {
                        if signed {
                            self.push_constant_ir_fmt_sign(
                                val,
                                sz,
                                op,
                                display_format::NONE,
                                true,
                            );
                        } else {
                            self.push_constant_ir(val, sz, op);
                        }
                        self.recurse();
                    }
                    // No op to hang the constant's markup on. C emits `case :`
                    // here, which is merely odd; a `match` arm with no pattern
                    // does not parse, so the value is printed directly.
                    None => self.emit.print(
                        &format!("{}", val as i64),
                        SyntaxHighlight::ConstColor,
                    ),
                }
            }
        }
        self.emit.spaces(1, 0);
        self.emit.print("=>", SyntaxHighlight::NoColor);
        let id = self
            .emit
            .open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_switch));
        let id2 = self.emit.begin_block(0);
        if case.gototype != 0 {
            // An unstructured case body: the same marker a residual goto gets.
            self.emit.tag_line();
            self.emit_goto_statement(fd, blk, case.block, case.gototype);
        } else {
            self.emit_block(fd, arch, case.block);
            // A genuine C fall-through: `grab_case_basic` records the case this
            // one falls INTO in `chain` (it is `-1` otherwise). Note this is NOT
            // the ordinary `case A: case B: body` shape -- that is one CaseOrder
            // with two jump-table indices, already rendered as `A | B =>` above.
            // `isexit` is the wrong test: an arm that returns has no out-edge and
            // so is not an exit, but it plainly does not fall through either.
            if case.chain != -1 {
                self.emit.tag_line();
                self.emit.print(
                    "/* kuna: C switch fall-through has no match form; the next arm's body is not duplicated */",
                    SyntaxHighlight::CommentColor,
                );
            }
        }
        self.emit.end_block(id2);
        self.emit.close_brace_indent(self.lang().kw_close_curly, id);
    }
}

impl PrintC {
    /// Whether a loop's condition block would emit statements as well as its
    /// test -- the case C renders with a comma expression.
    ///
    /// Counts printed ops the way `if_body_elides` does: not-printed ops,
    /// branches and implied-output ops never reach the page.
    fn cond_block_emits_statements(&self, fd: &Funcdata, cond: BlockId) -> bool {
        use crate::block::BlockType;
        if fd.sblocks_ref().block(cond).get_type() != BlockType::Copy {
            // A condition that is itself a list or a folded condition carries
            // statements by construction.
            return true;
        }
        let Some(under) = fd.sblocks_ref().block(cond).get_copy() else {
            return false;
        };
        let mut cur = fd.bb_op_head(under);
        while let Some(inst) = cur {
            cur = fd.bb_op_next(inst);
            let Some(o) = fd.obank().get(inst) else { continue };
            if o.not_printed() || o.is_branch() {
                continue;
            }
            if let Some(out) = o.get_out() {
                if fd.vbank().get(out).map(|v| v.is_implied()).unwrap_or(false) {
                    continue;
                }
            }
            return true;
        }
        false
    }
}
