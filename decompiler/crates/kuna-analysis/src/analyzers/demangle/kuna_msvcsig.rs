//! (kuna `msvcsig`) The MSVC arm of [`super::kuna_cppsig`]: read everything an
//! MSVC-mangled name declares, and apply it to the defined functions AND the PE
//! imports that carry one.
//!
//! An MSVC name is a full declaration, which an Itanium name is not: it states
//! the access specifier (so whether there is a `this`), `static`, the calling
//! convention, the return type and every parameter type. What it does not state
//! is the layout of a class, and that is the one thing this arm never invents.
//!
//! ```text
//!  ?IsValid@Arr@@QAEHXZ                 public: int __thiscall Arr::IsValid(void)
//!  ??0Arr@@QAE@ABV0@@Z                  public: __thiscall Arr::Arr(class Arr const &)
//!  ?MakeStack@@YA?AVArr@@V1@AAV1@1@Z    class Arr __cdecl MakeStack(class Arr,class Arr &,class Arr &)
//! ```
//!
//! ## What the declaration decides
//!
//! * **The convention.** On 32-bit x86 the stated `__cdecl`/`__stdcall`/
//!   `__thiscall`/`__fastcall` is the name of the prototype model the pieces are
//!   laid out for, so a `__thiscall` member takes `this` in ECX and a `__cdecl`
//!   function leaves its arguments for the caller to pop. Every other platform
//!   has one convention and keeps the default model.
//! * **`this`.** A member that is not `static` takes one, as argument 0.
//! * **The return.** A stated return type is applied (Itanium states none). A
//!   class, struct or union returned BY VALUE comes back through a hidden pointer
//!   the caller passes after `this`, and the callee returns that pointer — but
//!   only sometimes. MSVC returns a trivially copyable aggregate of 1, 2, 4 or 8
//!   bytes from a free or `static` function in EDX:EAX, and neither the size nor
//!   the triviality is in the name. Two cases are certain: a non-static member
//!   always returns a class through the pointer, and so does every function when
//!   the class has a non-trivial copy constructor, destructor or vftable. The
//!   image proves the latter when it defines or imports one of them (a `??0`
//!   constructor that takes arguments, a `??1` destructor, a `??_7` vftable or a
//!   `??_G`/`??_E` deleting destructor — a trivial one is never emitted). With
//!   neither, the return is undecided and the declaration is refused.
//! * **The parameters.** A primitive, pointer, reference or function pointer
//!   has a known width. A class, struct, union or enum passed BY VALUE does not:
//!   MSVC copies it into the outgoing argument area at its own (unstated) size,
//!   so its slot, and the position of everything after it, is unknown. On an
//!   import declared `__cdecl` the parameters before it are applied and the rest
//!   of the list is left open, exactly as for a variadic function: the caller
//!   pops, so the open tail claims nothing about the stack. Anywhere else the
//!   declaration is refused — a callee-pops convention would claim a stack
//!   adjustment that depends on the missing size, and a defined function would
//!   lose the parameters its own body recovers.
//!
//! Applied to the IAT slot and to every `FF 25` thunk that jumps through it, as
//! the Win32 and libc tables are.

use std::collections::HashSet;
use std::rc::Rc;

use object::read::{Object, ObjectSymbol};
use object::{Architecture, BinaryFormat};

use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use super::demangle_raw;
use super::kuna_cppsig::{
    is_identifier, last_token, primitive_type, split_params, split_top_level, strip_qualifier_word,
    strip_template_args,
};
use crate::pass::AnalysisCtx;

/// An MSVC calling convention this arm can name a prototype model for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Convention {
    Cdecl,
    Stdcall,
    Thiscall,
    Fastcall,
}

impl Convention {
    /// The prototype model the x86 Windows compiler spec registers for it.
    pub(super) fn model(self) -> &'static str {
        match self {
            Convention::Cdecl => "__cdecl",
            Convention::Stdcall => "__stdcall",
            Convention::Thiscall => "__thiscall",
            Convention::Fastcall => "__fastcall",
        }
    }
}

/// Every convention keyword the demangler prints; only the first four map to a
/// model, and a declaration naming another is refused.
const CONVENTIONS: &[(&str, Option<Convention>)] = &[
    ("__cdecl", Some(Convention::Cdecl)),
    ("__stdcall", Some(Convention::Stdcall)),
    ("__thiscall", Some(Convention::Thiscall)),
    ("__fastcall", Some(Convention::Fastcall)),
    ("__vectorcall", None),
    ("__clrcall", None),
    ("__pascal", None),
    ("__eabi", None),
    ("__swift_1", None),
    ("__swift_2", None),
    ("__regcall", None),
];

/// The operator spellings that may follow `operator` directly.
const OPERATORS: &[&str] = &[
    "=", "==", "!=", "<", ">", "<=", ">=", "<=>", "+", "-", "*", "/", "%", "^", "&", "|", "~",
    "!", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "<<", ">>", "<<=", ">>=", "&&", "||",
    "++", "--", ",", "->*", "->", "()", "[]",
];

/// What the declaration says about the return value.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Ret {
    /// A constructor, destructor or conversion operator: no return type printed.
    Unstated,
    /// A class, struct or union returned by value, by its qualified name.
    ByValue(String),
    /// An enum returned by value: in EAX at a width the name does not give.
    Enum,
    /// Any other return type, as demangled text.
    Type(String),
}

/// One parsed MSVC declaration.
#[derive(Debug)]
pub(super) struct MsvcDecl {
    pub(super) qualified: String,
    pub(super) scope: String,
    /// The bare class name a `this` points at.
    pub(super) class: String,
    pub(super) convention: Convention,
    pub(super) this: bool,
    pub(super) ret: Ret,
    pub(super) params: Vec<String>,
    pub(super) varargs: bool,
}

/// Parse the demangled text of an MSVC function name, or `None` for a data
/// symbol, a special name (`` `vftable' ``, an adjustor thunk), a function
/// template, or any shape the parse cannot account for.
pub(super) fn parse(dem: &str) -> Option<MsvcDecl> {
    if dem.contains('`') || dem.contains('"') {
        return None;
    }
    let body = strip_trailing_qualifiers(dem);
    if !body.ends_with(')') {
        return None;
    }
    let open = matching_open(body)?;
    let params_text = &body[open + 1..body.len() - 1];
    let head = &body[..open];
    let (convention, cstart, cend) = find_convention(head)?;
    let prefix = head[..cstart].trim();
    let name = head[cend..].trim();

    let (member, is_static, ret_text) = split_prefix(prefix)?;
    let (scope, last) = split_name(name)?;
    let class = match scope.is_empty() {
        true => String::new(),
        false => {
            let comps = split_top_level(&scope);
            strip_template_args(comps[comps.len() - 1])
        }
    };
    let this = member && !is_static;
    if this && !is_identifier(&class) {
        return None;
    }

    let ret = if ret_text.is_empty() {
        let bare = strip_template_args(&last);
        let ctor_dtor = bare == class || bare.strip_prefix('~') == Some(class.as_str());
        if !(ctor_dtor || last.starts_with("operator ")) {
            return None;
        }
        Ret::Unstated
    } else if let Some(cls) = by_value_aggregate(ret_text) {
        Ret::ByValue(cls)
    } else if by_value_enum(ret_text) {
        Ret::Enum
    } else {
        Ret::Type(ret_text.to_string())
    };

    let mut params = Vec::new();
    let mut varargs = false;
    for p in split_params(params_text) {
        let p = p.trim();
        if p.is_empty() || p == "void" {
            continue;
        }
        if varargs {
            return None;
        }
        if p == "..." {
            varargs = true;
            continue;
        }
        params.push(p.to_string());
    }

    Some(MsvcDecl {
        qualified: name.to_string(),
        scope,
        class,
        convention,
        this,
        ret,
        params,
        varargs,
    })
}

/// Drop the cv/ref qualifiers that may follow a member's parameter list.
fn strip_trailing_qualifiers(s: &str) -> &str {
    let mut t = s.trim_end();
    loop {
        let before = t.len();
        for word in ["const", "volatile", "__ptr64", "__restrict", "__unaligned"] {
            if let Some(r) = strip_qualifier_word(t, word) {
                t = r.trim_end();
            }
        }
        if let Some(r) = t.strip_suffix("&&").or_else(|| t.strip_suffix('&')) {
            t = r.trim_end();
        }
        if t.len() == before {
            return t;
        }
    }
}

/// The index of the `(` matching the `)` that ends `s`, counting parentheses
/// only (a template argument or an operator symbol cannot unbalance them).
fn matching_open(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in s.char_indices().rev() {
        match c {
            ')' => depth += 1,
            '(' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// The first convention keyword standing outside every bracket, with its byte
/// range. One inside a template argument or a function-pointer return type is
/// preceded by an open bracket and skipped.
fn find_convention(head: &str) -> Option<(Convention, usize, usize)> {
    let b = head.as_bytes();
    let (mut angle, mut paren) = (0i32, 0i32);
    for (i, &c) in b.iter().enumerate() {
        match c {
            b'<' => angle += 1,
            b'>' => angle -= 1,
            b'(' => paren += 1,
            b')' => paren -= 1,
            b'_' if angle == 0 && paren == 0 && (i == 0 || b[i - 1] == b' ') => {
                for (word, conv) in CONVENTIONS {
                    let end = i + word.len();
                    if head[i..].starts_with(word) && b.get(end) == Some(&b' ') {
                        return conv.map(|c| (c, i, end));
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Split `public: static int` into (member, static, return text), or `None` for
/// a `[thunk]:` adjustor.
fn split_prefix(prefix: &str) -> Option<(bool, bool, &str)> {
    if prefix.starts_with('[') {
        return None;
    }
    let mut p = prefix;
    let mut member = false;
    for access in ["public:", "protected:", "private:"] {
        if let Some(r) = p.strip_prefix(access) {
            member = true;
            p = r.trim_start();
            break;
        }
    }
    let mut is_static = false;
    loop {
        let (word, rest) = p.split_once(' ').unwrap_or((p, ""));
        match word {
            "static" => is_static = true,
            "virtual" => {}
            _ => break,
        }
        p = rest.trim_start();
    }
    Some((member, is_static, p.trim()))
}

/// Split a qualified name into its scope and last component, accounting for an
/// operator name whose symbol would otherwise read as a bracket.
fn split_name(name: &str) -> Option<(String, String)> {
    let op = if name.starts_with("operator") {
        Some(0)
    } else {
        name.rfind("::operator").map(|i| i + 2)
    };
    let (scope, last) = match op {
        Some(at) if !name[at + "operator".len()..].starts_with(is_ident_char) => {
            let last = &name[at..];
            let symbol = &last["operator".len()..];
            let known = OPERATORS.contains(&symbol)
                || (symbol.starts_with(' ') && !symbol.contains(['(', ')']));
            if !known {
                return None;
            }
            let scope = if at == 0 { "" } else { &name[..at - 2] };
            (scope, last)
        }
        _ => {
            let comps = split_top_level(name);
            let last = comps[comps.len() - 1];
            let base = last.strip_prefix('~').unwrap_or(last);
            if last.contains('<') {
                // Only a class template's own constructor or destructor repeats
                // the class's arguments; any other `<` is a function template.
                let owner = match comps.len() {
                    n if n >= 2 => strip_template_args(comps[n - 2]),
                    _ => return None,
                };
                if strip_template_args(base) != owner {
                    return None;
                }
            } else if !is_identifier(base) {
                return None;
            }
            let cut = name.len() - last.len();
            (if cut >= 2 { &name[..cut - 2] } else { "" }, last)
        }
    };
    if !scope.is_empty() {
        if !balanced(scope) {
            return None;
        }
        for comp in split_top_level(scope) {
            if !is_identifier(&strip_template_args(comp)) {
                return None;
            }
        }
    }
    Some((scope.to_string(), last.to_string()))
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Every `<`/`(` closed, none closed early.
fn balanced(s: &str) -> bool {
    let (mut angle, mut paren) = (0i32, 0i32);
    for c in s.chars() {
        match c {
            '<' => angle += 1,
            '>' => angle -= 1,
            '(' => paren += 1,
            ')' => paren -= 1,
            _ => {}
        }
        if angle < 0 || paren < 0 {
            return false;
        }
    }
    angle == 0 && paren == 0
}

/// Peel trailing cv qualifiers, returning the rest and whether a `*`/`&`
/// declarator remains.
fn peel_cv(text: &str) -> &str {
    let mut t = text.trim();
    loop {
        let before = t.len();
        for word in ["const", "volatile", "__ptr64", "__restrict", "__unaligned"] {
            if let Some(r) = strip_qualifier_word(t, word) {
                t = r.trim_end();
            }
        }
        for word in ["const ", "volatile "] {
            if let Some(r) = t.strip_prefix(word) {
                t = r.trim_start();
            }
        }
        if t.len() == before {
            return t;
        }
    }
}

/// The qualified name of a class, struct or union spelled BY VALUE, or `None`
/// for a pointer, a reference or any other type.
fn by_value_aggregate(text: &str) -> Option<String> {
    let t = peel_cv(text);
    if t.ends_with(['*', '&']) || t.contains('(') {
        return None;
    }
    ["class ", "struct ", "union "]
        .iter()
        .find_map(|k| t.strip_prefix(k))
        .map(|r| r.trim().to_string())
}

/// An enum spelled by value.
fn by_value_enum(text: &str) -> bool {
    let t = peel_cv(text);
    t.starts_with("enum ") && !t.ends_with(['*', '&']) && !t.contains('(')
}

/// What one demangled type spelling is, read before any type is built so a
/// refused declaration leaves nothing behind in the type factory.
enum Shape<'a> {
    /// A function pointer.
    Code,
    /// A primitive, or a pointee named by its bare class name, under
    /// `indirection` pointer levels.
    Named { base: &'a str, primitive: bool, indirection: usize },
    /// A class, struct, union or enum by value: a width the name does not give.
    UnknownSize,
    Refused,
}

/// Classify one demangled parameter (or return) type.
fn shape(text: &str) -> Shape<'_> {
    let t = text.trim();
    if t.contains("::*") {
        return Shape::Refused;
    }
    if is_function_pointer(t) {
        return Shape::Code;
    }
    if t.contains(['(', '[']) {
        return Shape::Refused;
    }
    let mut rest = t;
    let mut indirection = 0usize;
    loop {
        let before = rest.len();
        rest = peel_cv(rest);
        if let Some(r) = rest.strip_suffix("&&").or_else(|| rest.strip_suffix(['*', '&'])) {
            indirection += 1;
            rest = r;
        }
        if rest.len() == before {
            break;
        }
    }
    let base = peel_cv(rest);
    if PRIMITIVES.contains(&base) {
        return Shape::Named { base, primitive: true, indirection };
    }
    if indirection == 0 {
        return Shape::UnknownSize;
    }
    let bare = ["class ", "struct ", "union ", "enum "]
        .iter()
        .find_map(|k| base.strip_prefix(k))
        .unwrap_or(base)
        .trim();
    if !is_identifier(&bare_class_name(bare)) {
        return Shape::Refused;
    }
    Shape::Named { base: bare, primitive: false, indirection }
}

/// The innermost component of a qualified class name without its template
/// arguments: what the placeholder structure is called.
fn bare_class_name(qualified: &str) -> String {
    let comps = split_top_level(qualified);
    strip_template_args(comps[comps.len() - 1])
}

/// Build the type a [`Shape::Code`] or [`Shape::Named`] describes.
fn build_type(
    shape: &Shape,
    types: &dyn TypeFactory,
    ptr: int4,
    word_size: uint4,
) -> Option<Rc<Datatype>> {
    let (mut ty, indirection) = match shape {
        Shape::Code => (types.get_type_code().ok()?, 1),
        Shape::Named { base, primitive: true, indirection } => {
            (msvc_primitive(base, types)?, *indirection)
        }
        Shape::Named { base, primitive: false, indirection } => {
            (types.get_type_struct(&bare_class_name(base)).ok()?, *indirection)
        }
        Shape::UnknownSize | Shape::Refused => return None,
    };
    for _ in 0..indirection {
        ty = types.get_type_pointer(ptr, ty, word_size).ok()?;
    }
    Some(ty)
}

/// `R (__cdecl *)(A)` and its reference and `const` forms.
fn is_function_pointer(t: &str) -> bool {
    if !t.ends_with(')') {
        return false;
    }
    let Some(args) = matching_open(t) else { return false };
    let decl = t[..args].trim_end();
    if !decl.ends_with(')') {
        return false;
    }
    let Some(open) = matching_open(decl) else { return false };
    let mut inner = decl[open + 1..decl.len() - 1].trim();
    for (word, _) in CONVENTIONS {
        if let Some(r) = inner.strip_prefix(word) {
            inner = r.trim_start();
        }
    }
    matches!(peel_cv(inner), "*" | "&" | "*&")
}

/// The primitive spellings the MSVC demangler prints.
const PRIMITIVES: &[&str] = &[
    "void", "bool", "char", "signed char", "unsigned char", "wchar_t", "char8_t", "char16_t",
    "char32_t", "short", "unsigned short", "int", "unsigned int", "long", "unsigned long",
    "long long", "unsigned long long", "__int8", "unsigned __int8", "__int16", "unsigned __int16",
    "__int32", "unsigned __int32", "__int64", "unsigned __int64", "int64_t", "uint64_t",
    "__int128", "unsigned __int128", "float", "double", "long double",
];

/// A [`PRIMITIVES`] spelling at the width MSVC gives it: `long` is 4 bytes and
/// `long double` 8 on every Windows target, and `wchar_t` is 2.
fn msvc_primitive(text: &str, types: &dyn TypeFactory) -> Option<Rc<Datatype>> {
    let (size, meta) = match text {
        "long" => (4, type_metatype::TYPE_INT),
        "unsigned long" => (4, type_metatype::TYPE_UINT),
        "__int64" | "int64_t" => (8, type_metatype::TYPE_INT),
        "unsigned __int64" | "uint64_t" => (8, type_metatype::TYPE_UINT),
        "__int8" => return types.get_type_char(1).ok(),
        "unsigned __int8" => (1, type_metatype::TYPE_UINT),
        "__int16" => (2, type_metatype::TYPE_INT),
        "unsigned __int16" => (2, type_metatype::TYPE_UINT),
        "__int32" => (4, type_metatype::TYPE_INT),
        "unsigned __int32" => (4, type_metatype::TYPE_UINT),
        "long double" => (8, type_metatype::TYPE_FLOAT),
        "wchar_t" => return types.get_type_char(2).ok(),
        _ => return primitive_type(text, types).flatten(),
    };
    types.get_base(size, meta).ok()
}

/// The scopes the image proves are classes MSVC returns through a hidden
/// pointer from any function: each has a non-trivial copy constructor or
/// destructor or a vftable that the image defines or imports (a trivial special
/// member is never emitted, so its symbol cannot exist).
pub(super) fn nontrivial_classes<'a>(names: impl Iterator<Item = &'a str>) -> HashSet<String> {
    let mut out = HashSet::new();
    for raw in names {
        let special = ["??0", "??1", "??_7", "??_G", "??_E"];
        if !special.iter().any(|p| raw.starts_with(p)) {
            continue;
        }
        let Some(dem) = demangle_raw(raw) else { continue };
        let class = if raw.starts_with("??_7") {
            dem.find("::`vftable'")
                .map(|i| dem[..i].trim().trim_start_matches("const ").trim().to_string())
        } else if raw.starts_with("??_") {
            dem.find("::`").map(|i| last_token(dem[..i].trim()))
        } else {
            parse(&dem)
                .filter(|d| raw.starts_with("??1") || !d.params.is_empty() || d.varargs)
                .map(|d| d.scope)
        };
        if let Some(c) = class.filter(|c| !c.is_empty()) {
            out.insert(c);
        }
    }
    out
}

/// The built prototype and the model its storage was laid out for.
pub(super) struct MsvcSig {
    pub(super) pieces: PrototypePieces,
    pub(super) model: Option<&'static str>,
}

/// Build the prototype `decl` declares, or `None` where it cannot be laid out
/// soundly (see the module header). Every refusal is decided before the first
/// type is built.
pub(super) fn build(
    decl: &MsvcDecl,
    nontrivial: &HashSet<String>,
    import: bool,
    x86_32: bool,
    types: &dyn TypeFactory,
    ptr: int4,
    word_size: uint4,
) -> Option<MsvcSig> {
    let ret = match &decl.ret {
        Ret::Unstated | Ret::Enum => None,
        Ret::ByValue(cls) => {
            let hidden = Shape::Named { base: cls, primitive: false, indirection: 1 };
            if !(decl.this || nontrivial.contains(cls)) || !is_identifier(&bare_class_name(cls)) {
                return None;
            }
            Some((hidden, true))
        }
        Ret::Type(text) => match shape(text) {
            s @ (Shape::Code | Shape::Named { .. }) => Some((s, false)),
            _ => return None,
        },
    };
    let mut params: Vec<Shape> = Vec::with_capacity(decl.params.len());
    let mut open = false;
    for p in &decl.params {
        match shape(p) {
            s @ (Shape::Code | Shape::Named { .. }) => params.push(s),
            Shape::UnknownSize if import && x86_32 && decl.convention == Convention::Cdecl => {
                open = true;
                break;
            }
            _ => return None,
        }
    }

    let mut intypes: Vec<Rc<Datatype>> = Vec::with_capacity(params.len() + 2);
    let mut innames: Vec<String> = Vec::with_capacity(params.len() + 2);
    if decl.this {
        let cls = types.get_type_struct(&decl.class).ok()?;
        intypes.push(types.get_type_pointer(ptr, cls, word_size).ok()?);
        innames.push("this".to_string());
    }
    let outtype = match ret {
        None => None,
        Some((s, hidden)) => {
            let ty = build_type(&s, types, ptr, word_size)?;
            if hidden {
                intypes.push(Rc::clone(&ty));
                innames.push(String::new());
            }
            Some(ty)
        }
    };
    for s in &params {
        intypes.push(build_type(s, types, ptr, word_size)?);
        innames.push(String::new());
    }
    let first_var_arg_slot = if open || decl.varargs { intypes.len() as int4 } else { -1 };
    Some(MsvcSig {
        pieces: PrototypePieces {
            name: decl.qualified.clone(),
            outtype,
            intypes,
            innames,
            first_var_arg_slot,
            output_storage: None,
            input_storage: Vec::new(),
        },
        model: x86_32.then(|| decl.convention.model()),
    })
}

/// A 32-bit x86 PE or COFF object: the one target whose MSVC conventions differ.
fn is_x86_32_msvc(file: &object::File) -> bool {
    file.architecture() == Architecture::I386
        && matches!(file.format(), BinaryFormat::Pe | BinaryFormat::Coff)
}

/// The MSVC-mangled imports of a PE, at every address the import resolver names
/// (the IAT slot and each thunk that jumps through it). Empty, without walking
/// the code for thunks, when no import is MSVC-mangled.
pub(super) fn msvc_imports(ctx: &AnalysisCtx) -> Vec<(u64, String)> {
    if ctx.file.format() != BinaryFormat::Pe {
        return Vec::new();
    }
    let any = ctx
        .file
        .imports()
        .is_ok_and(|imports| imports.iter().any(|i| i.name().starts_with(b"?")));
    if !any {
        return Vec::new();
    }
    crate::protos::resolved_import_addrs(ctx.file, ctx.bytes)
        .into_iter()
        .filter(|(name, _)| name.starts_with('?'))
        .map(|(name, addr)| (addr, name))
        .collect()
}

/// Every MSVC-mangled name the image carries, defined, imported or exported:
/// the evidence [`nontrivial_classes`] reads.
fn image_msvc_names(file: &object::File) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if let Ok(n) = sym.name() {
            if n.starts_with('?') {
                names.push(n.to_string());
            }
        }
    }
    if let Ok(imports) = file.imports() {
        for i in imports {
            if i.name().starts_with(b"?") {
                names.push(String::from_utf8_lossy(i.name()).into_owned());
            }
        }
    }
    if let Ok(exports) = file.exports() {
        for e in exports {
            if e.name().starts_with(b"?") {
                names.push(String::from_utf8_lossy(e.name()).into_owned());
            }
        }
    }
    names
}

/// Read every MSVC declaration among the defined function symbols in `defined`
/// and the imports in `imports` into `out`, in that order.
#[allow(clippy::type_complexity)]
pub(super) fn collect(
    ctx: &AnalysisCtx,
    defined: &[(u64, String)],
    imports: &[(u64, String)],
    out: &mut Vec<(u64, PrototypePieces, Option<&'static str>)>,
) {
    let any_defined = defined.iter().any(|(_, raw)| raw.starts_with('?'));
    if !any_defined && imports.is_empty() {
        return;
    }
    let names = image_msvc_names(ctx.file);
    let nontrivial = nontrivial_classes(names.iter().map(String::as_str));
    let x86_32 = is_x86_32_msvc(ctx.file);
    let types = ctx.arch.types();
    let (_addr_size, word_size) = ctx.arch.data_org();
    let ptr = types.get_size_of_pointer();
    let sources = defined
        .iter()
        .filter(|(_, raw)| raw.starts_with('?'))
        .map(|(a, raw)| (*a, raw, false))
        .chain(imports.iter().map(|(a, raw)| (*a, raw, true)));
    for (addr, raw, import) in sources {
        let Some(decl) = demangle_raw(raw).and_then(|dem| parse(&dem)) else { continue };
        if let Some(sig) = build(&decl, &nontrivial, import, x86_32, types, ptr, word_size) {
            out.push((addr, sig.pieces, sig.model));
        }
    }
}

#[cfg(test)]
mod tests;
