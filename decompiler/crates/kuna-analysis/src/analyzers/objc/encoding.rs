//! The Objective-C **type-encoding** decoder (design §5 PR-O2).
//!
//! ObjC method/ivar metadata carries a compact, ASCII *type encoding* string
//! (the `@encode(...)` form). A method's `types` field is the return type
//! followed by every argument type, each optionally trailed by a decimal **frame
//! offset** the runtime uses for `objc_msgSend` marshaling. For
//! `-(int)greet:(int)n` the encoding is `i20@0:8i16`:
//!
//! ```text
//!   i 20   → return int, frame size 20
//!   @ 0    → id   self   (arg 0 at frame offset 0)
//!   : 8    → SEL  _cmd   (arg 1 at frame offset 8)
//!   i 16   → int  n      (arg 2 at frame offset 16)
//! ```
//!
//! This is the kuna analog of Ghidra's `ObjectiveC1_TypeEncodings` /
//! `ObjectiveC2_DecompilerMessageAnalyzer` encoding parser (the *naming-tier*
//! subset). It is **not a demangler** — selectors are plain ASCII; this only maps
//! the single-character primitive type codes (+ a pointer/struct fallback) to a
//! kuna [`Datatype`] so an IMP gets a typed `PrototypePieces` (`id self, SEL
//! _cmd, int n`) and an ivar gets its element type.
//!
//! ## Scope (design §3.2 LOSS)
//!
//! Primitive codes (`c i s l q f d B v *`, signed/unsigned), `@` → `id`, `#` →
//! `Class`, `:` → `SEL`, and a `^T` / `{...}` / `[...]` **opaque-pointer / opaque
//! fallback** (we do not reconstruct nested struct/array layout — the same
//! name-level-opaque posture as `dwarf`'s MVP). Protocol qualifiers
//! (`r`/`n`/`N`/`o`/`O`/`R`/`V`) and the GC `!` are skipped. An unparseable or
//! empty encoding yields `None` (the IMP is still *named* `-[Class sel]` — only
//! the typed prototype is dropped), never a failure.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use kuna_decomp::fspec::PrototypePieces;

/// Decode one ObjC method type encoding into a [`PrototypePieces`] named `name`
/// (the `-[Class sel]` label). The encoding is the return type followed by each
/// argument type; the first two arguments of every ObjC method are the implicit
/// `id self` (`@`) and `SEL _cmd` (`:`), which we name accordingly.
///
/// `None` for an empty / unparseable encoding (the IMP keeps its name, only the
/// typed prototype is dropped). `ptr_size` / `word_size` come from
/// `Architecture::data_org`.
pub fn decode_method(
    name: &str,
    encoding: &str,
    tctx: &TypeCtx,
    self_type: Option<&Rc<Datatype>>,
) -> Option<PrototypePieces> {
    let mut p = Parser::new(encoding);

    // Return type first (a null/void return is still `Some(void)`).
    let outtype = Some(p.next_type(tctx)?);

    let mut intypes: Vec<Rc<Datatype>> = Vec::new();
    let mut innames: Vec<String> = Vec::new();
    let mut idx = 0usize;
    while !p.at_end() {
        let Some(ty) = p.next_type(tctx) else { break };
        // The first two ObjC arguments are always the implicit receiver + selector.
        let nm = match idx {
            0 => "self".to_string(),
            1 => "_cmd".to_string(),
            _ => format!("arg{}", idx - 1),
        };
        let ty = match (idx, self_type) {
            (0, Some(st)) => Rc::clone(st),
            _ => ty,
        };
        intypes.push(ty);
        innames.push(nm);
        idx += 1;
    }

    Some(PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes,
        innames,
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
        kuna_library_names: None,
    })
}

/// Split a method encoding into its raw return and argument encodings, frame
/// offsets stripped: `v24@0:8{CGRect=dddd}16` → `("v", ["@", ":", "{CGRect=dddd}"])`.
/// `None` for an empty or unparseable encoding.
pub fn split_method(encoding: &str) -> Option<(String, Vec<String>)> {
    let mut p = Parser::new(encoding);
    let ret = p.next_type_text()?;
    let mut args = Vec::new();
    while !p.at_end() {
        args.push(p.next_type_text()?);
    }
    Some((ret, args))
}

/// The struct a by-value encoding names (`{CGRect={CGPoint=dd}{CGSize=dd}}`),
/// built once per name with its leaf members as fields — nested geometry
/// flattened (`x, y, width, height`), since a field of a field is what the
/// printer renders worst — so a method returning one has a type the model can
/// place in registers. `None` for an unnamed or malformed layout.
pub fn struct_type(encoding: &str, tctx: &TypeCtx) -> Option<Rc<Datatype>> {
    let name = struct_name(encoding)?;
    if let Some(t) = tctx.structs.borrow().get(name) {
        return Some(Rc::clone(t));
    }
    let types = tctx.types;
    let shell = match types.find_by_name(name).ok().flatten() {
        Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT && !t.is_incomplete() => {
            tctx.structs.borrow_mut().insert(name.to_string(), Rc::clone(&t));
            return Some(t);
        }
        Some(_) => return None,
        None => types.kuna_declare_record(name, false).ok()?,
    };
    let members = struct_members(encoding)?;
    let names = member_names(name);
    let mut fields = Vec::new();
    let mut offset = 0i32;
    for (k, code) in members.iter().enumerate() {
        let ty = Parser::new(&code.to_string()).next_type(tctx)?;
        let size = ty.get_size();
        if size <= 0 {
            return None;
        }
        let align = size.min(tctx.ptr_size);
        offset = (offset + align - 1) / align * align;
        let field = names.get(k).map(|n| n.to_string()).unwrap_or_else(|| format!("field_{k}"));
        fields.push(TypeField::new(offset, offset, field, ty));
        offset += size;
    }
    let size = (offset + tctx.ptr_size - 1) / tctx.ptr_size * tctx.ptr_size;
    let done = types.set_fields_struct_raw(&shell, fields, Vec::new(), size, tctx.ptr_size, 0).ok()?;
    tctx.structs.borrow_mut().insert(name.to_string(), Rc::clone(&done));
    Some(done)
}

/// The name of a struct encoding (`CGRect` of `{CGRect=…}`), `None` when it
/// has none (`{?=dd}`).
pub fn struct_name(encoding: &str) -> Option<&str> {
    let body = encoding.strip_prefix('{')?.strip_suffix('}')?;
    let (name, _) = body.split_once('=')?;
    (!name.is_empty() && name != "?").then_some(name)
}

/// The leaf member codes of a struct encoding, nested structs flattened:
/// `{CGRect={CGPoint=dd}{CGSize=dd}}` → `dddd`. `None` for a malformed layout.
pub fn struct_members(encoding: &str) -> Option<Vec<char>> {
    let body = encoding.strip_prefix('{')?.strip_suffix('}')?;
    let (_, members) = body.split_once('=')?;
    let mut out = Vec::new();
    let (mut depth, mut start) = (0usize, 0usize);
    for (i, c) in members.char_indices() {
        match c {
            '{' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    out.extend(struct_members(&members[start..=i])?);
                }
            }
            c if depth == 0 => out.push(c),
            _ => {}
        }
    }
    (depth == 0 && !out.is_empty()).then_some(out)
}

/// The leaf member names of the CoreGraphics / Foundation geometry structs,
/// whose encodings carry none.
pub fn member_names(name: &str) -> &'static [&'static str] {
    match name {
        "CGPoint" => &["x", "y"],
        "CGSize" => &["width", "height"],
        "CGVector" => &["dx", "dy"],
        "CGRect" => &["x", "y", "width", "height"],
        "_NSRange" | "NSRange" => &["location", "length"],
        "UIOffset" => &["horizontal", "vertical"],
        "UIEdgeInsets" => &["top", "left", "bottom", "right"],
        "NSDirectionalEdgeInsets" => &["top", "leading", "bottom", "trailing"],
        "CGAffineTransform" => &["a", "b", "c", "d", "tx", "ty"],
        _ => &[],
    }
}

/// Decode a single ivar type encoding (the element's `@encode` string) into its
/// [`Datatype`]. Used for the `<Class>::ivar` typing (design §5 PR-O2). `None`
/// for an empty / unparseable encoding.
pub fn decode_ivar_type(encoding: &str, tctx: &TypeCtx) -> Option<Rc<Datatype>> {
    Parser::new(encoding).next_type(tctx)
}

/// The type-mapping context a method or ivar encoding is decoded against: the
/// architecture's [`TypeFactory`] plus its pointer size / word size
/// (`data_org`), and a cache of the named opaque pointer types the decoder
/// hands out (`objc_object *`, `objc_selector *`, `NSString *`). The factory
/// finds a type by name with a linear scan of every type it holds, and an
/// image with thousands of classes asks for `objc_object *` once per method
/// argument, so the cache is what keeps the walk linear in the metadata.
pub struct TypeCtx<'a> {
    pub types: &'a dyn TypeFactory,
    pub ptr_size: i32,
    pub word_size: u32,
    opaque: RefCell<HashMap<String, Rc<Datatype>>>,
    /// The by-value structs [`struct_type`] built, by name.
    structs: RefCell<HashMap<String, Rc<Datatype>>>,
}

impl<'a> TypeCtx<'a> {
    pub fn new(types: &'a dyn TypeFactory, ptr_size: i32, word_size: u32) -> TypeCtx<'a> {
        TypeCtx {
            types,
            ptr_size,
            word_size,
            opaque: RefCell::new(HashMap::new()),
            structs: RefCell::new(HashMap::new()),
        }
    }

    /// A pointer to the named struct (`objc_object*` for `id`, `objc_class*` for
    /// `Class`, `objc_selector*` for `SEL`, `Foo*` for `@"Foo"`): the struct of
    /// that name when one exists, else a declared record so a pointer built
    /// before the class's layout lands (`kuna_objcivars`) reads the completion
    /// afterwards; a plain `void*` if the record cannot be created.
    pub(super) fn opaque_ptr(&self, struct_name: &str) -> Option<Rc<Datatype>> {
        if let Some(t) = self.opaque.borrow().get(struct_name) {
            return Some(Rc::clone(t));
        }
        let pointee = self
            .types
            .find_by_name(struct_name)
            .ok()
            .flatten()
            .filter(|t| t.get_metatype() == type_metatype::TYPE_STRUCT)
            .or_else(|| self.types.kuna_declare_record(struct_name, false).ok())
            .or_else(|| self.types.get_type_void().ok())?;
        let ptr = self.types.get_type_pointer(self.ptr_size, pointee, self.word_size).ok()?;
        self.opaque.borrow_mut().insert(struct_name.to_string(), Rc::clone(&ptr));
        Some(ptr)
    }
}

/// A cursor over an ObjC type-encoding string. `next_type` consumes one type code
/// (skipping protocol qualifiers and the trailing decimal frame offset).
struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    /// Pointer nesting (`^^^…`) entered so far; a hostile depth is refused.
    depth: usize,
}

/// The deepest `^` nesting decoded; a real encoding is two or three deep.
const MAX_POINTER_DEPTH: usize = 32;

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Parser<'a> {
        Parser { bytes: s.as_bytes(), pos: 0, depth: 0 }
    }

    /// At (or past) the end of the encoding.
    fn at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Skip the ObjC method-encoding protocol qualifiers + GC markers that may
    /// prefix a type code (`r`const `n`in `N`inout `o`out `O`bycopy `R`byref
    /// `V`oneway, `!` GC-invisible).
    fn skip_qualifiers(&mut self) {
        while let Some(c) = self.peek() {
            if matches!(c, b'r' | b'n' | b'N' | b'o' | b'O' | b'R' | b'V' | b'!') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// The `"Name"` an ivar encoding appends to `@` (`@"NSString"`, also
    /// `@"<Proto>"` for a protocol-qualified `id`, which names no class),
    /// consumed when present.
    fn quoted_class_name(&mut self) -> Option<String> {
        if self.peek()? != b'"' {
            return None;
        }
        let rest = &self.bytes[self.pos + 1..];
        let end = rest.iter().position(|&b| b == b'"')?;
        let name = std::str::from_utf8(&rest[..end]).ok()?;
        self.pos += end + 2;
        let class = name.split('<').next().unwrap_or("");
        (!class.is_empty() && class.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$'))
            .then(|| class.to_string())
    }

    /// Skip the decimal frame-offset digits trailing a type code in a *method*
    /// encoding (`i16` → skip `16`). An ivar/standalone encoding has no trailing
    /// number, so this is a no-op there.
    fn skip_frame_offset(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Consume one type code and map it to a [`Datatype`]. Returns `None` at the
    /// end of the string or on an unparseable code.
    fn next_type(&mut self, tctx: &TypeCtx) -> Option<Rc<Datatype>> {
        let (types, ptr_size, word_size) = (tctx.types, tctx.ptr_size, tctx.word_size);
        self.skip_qualifiers();
        let c = self.peek()?;
        self.pos += 1;

        let ty = match c {
            // --- primitive scalar codes (Apple `Type Encodings` table) ---
            b'c' => types.get_type_char(1).ok()?, // char / BOOL (signed)
            b'C' => types.get_base(1, type_metatype::TYPE_UINT).ok()?, // unsigned char
            b's' => types.get_base(2, type_metatype::TYPE_INT).ok()?, // short
            b'S' => types.get_base(2, type_metatype::TYPE_UINT).ok()?, // unsigned short
            b'i' => types.get_base(4, type_metatype::TYPE_INT).ok()?, // int
            b'I' => types.get_base(4, type_metatype::TYPE_UINT).ok()?, // unsigned int
            b'l' => types.get_base(4, type_metatype::TYPE_INT).ok()?, // long (32-bit ABI int)
            b'L' => types.get_base(4, type_metatype::TYPE_UINT).ok()?,
            b'q' => types.get_base(8, type_metatype::TYPE_INT).ok()?, // long long / 64-bit long
            b'Q' => types.get_base(8, type_metatype::TYPE_UINT).ok()?,
            b'f' => types.get_base(4, type_metatype::TYPE_FLOAT).ok()?, // float
            b'd' => types.get_base(8, type_metatype::TYPE_FLOAT).ok()?, // double
            b'B' => types.get_base(1, type_metatype::TYPE_BOOL).ok()?, // C99 _Bool
            b'v' => types.get_type_void().ok()?,                       // void
            // `*` = char* (a C string).
            b'*' => {
                let ch = types.get_type_char(1).ok()?;
                types.get_type_pointer(ptr_size, ch, word_size).ok()?
            }
            // `@` = id (object pointer) → an opaque `objc_object*`; an ivar's
            // `@"NSString"` names the class, so the pointee is that opaque type;
            // `@?` is a block, modelled as `void*`.
            b'@' => {
                if self.peek() == Some(b'?') {
                    self.pos += 1;
                    let v = types.get_type_void().ok()?;
                    types.get_type_pointer(ptr_size, v, word_size).ok()?
                } else {
                    match self.quoted_class_name() {
                        Some(class) => tctx.opaque_ptr(&class)?,
                        None => tctx.opaque_ptr("objc_object")?,
                    }
                }
            }
            // `#` = Class → an opaque `objc_class*`.
            b'#' => tctx.opaque_ptr("objc_class")?,
            // `:` = SEL → an opaque `objc_selector*`.
            b':' => tctx.opaque_ptr("objc_selector")?,
            // `^T` = pointer to T: decode the pointee (name-level), fall back to
            // void* on an opaque/unparseable pointee.
            b'^' => {
                self.depth += 1;
                if self.depth > MAX_POINTER_DEPTH {
                    return None;
                }
                let pointee = self.next_type(tctx);
                if self.depth > MAX_POINTER_DEPTH {
                    return None;
                }
                self.depth -= 1;
                let pointee = pointee.or_else(|| types.get_type_void().ok())?;
                types.get_type_pointer(ptr_size, pointee, word_size).ok()?
            }
            // `{name=...}` struct, `(name=...)` union, `[N T]` array, `b<n>`
            // bitfield: name-level-opaque — skip the body and yield a void* / void
            // placeholder (the dwarf recursion-cap posture). Reconstructing the
            // nested layout is the deferred field-level scope.
            b'{' => {
                self.skip_balanced(b'{', b'}');
                // A struct value passed/returned by value is opaque here; model it
                // as void (size unknown without field reconstruction).
                types.get_type_void().ok()?
            }
            b'(' => {
                self.skip_balanced(b'(', b')');
                types.get_type_void().ok()?
            }
            b'[' => {
                self.skip_balanced(b'[', b']');
                types.get_type_void().ok()?
            }
            b'b' => {
                // bitfield `b<width>`: consume the width digits, model as uint.
                self.skip_frame_offset();
                types.get_base(4, type_metatype::TYPE_UINT).ok()?
            }
            // `?` = unknown (e.g. a function pointer / block); model as void*.
            b'?' => {
                let v = types.get_type_void().ok()?;
                types.get_type_pointer(ptr_size, v, word_size).ok()?
            }
            // Anything else: unparseable — stop.
            _ => return None,
        };

        // Consume the trailing decimal frame offset a *method* encoding appends.
        self.skip_frame_offset();
        Some(ty)
    }

    /// Advance over one type encoding without building a type, returning its
    /// text with the frame offset stripped (`{CGRect=dddd}` for a struct by
    /// value, `@"UIView"` for a class pointer). `None` at the end or on an
    /// unknown code.
    fn next_type_text(&mut self) -> Option<String> {
        let text = self.type_text()?;
        self.skip_frame_offset();
        Some(text)
    }

    /// One type encoding's text, the frame offset left in place.
    fn type_text(&mut self) -> Option<String> {
        self.skip_qualifiers();
        let start = self.pos;
        let c = self.peek()?;
        self.pos += 1;
        match c {
            b'^' => {
                self.type_text()?;
            }
            b'{' => self.skip_balanced(b'{', b'}'),
            b'(' => self.skip_balanced(b'(', b')'),
            b'[' => self.skip_balanced(b'[', b']'),
            b'b' => self.skip_frame_offset(),
            b'@' => {
                if self.quoted_class_name().is_none() && self.peek() == Some(b'?') {
                    self.pos += 1;
                }
            }
            b'c' | b'C' | b's' | b'S' | b'i' | b'I' | b'l' | b'L' | b'q' | b'Q' | b'f' | b'd' | b'B' | b'v'
            | b'*' | b'#' | b':' | b'?' => {}
            _ => return None,
        }
        Some(std::str::from_utf8(&self.bytes[start..self.pos]).ok()?.to_string())
    }

    /// Skip a balanced `open`…`close` run (for `{...}` / `(...)` / `[...]`),
    /// honoring nesting. Leaves `pos` just past the closing delimiter.
    fn skip_balanced(&mut self, open: u8, close: u8) {
        let mut depth = 1usize;
        while let Some(c) = self.peek() {
            self.pos += 1;
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_decomp::dtype::TypeFactoryImpl;

    /// A configured [`TypeFactory`] (the dtype-test recipe + the core-type cache):
    /// default alignment map, max base size 8, a 64-bit `setup_sizes`, and the
    /// cached core types (`char`/`void`/`code`) — enough for every
    /// `get_base`/`get_type_char`/`get_type_void`/`get_type_pointer` path. The
    /// encoding decoder is arch-neutral, so an 8-byte-pointer factory suffices
    /// (pointer width is passed explicitly via `ptr_size`).
    fn factory() -> TypeFactoryImpl {
        let f = TypeFactoryImpl::new();
        f.set_default_alignment_map();
        f.set_max_basetype_size(8);
        // 64-bit sizes (sp=8 ⇒ int=4, long=8, ptr=8) + the core-type cache so
        // get_type_char / get_type_void resolve.
        f.setup_sizes(Some(8), 8, 4);
        let _ = f.cache_core_types();
        f
    }

    /// `-(int)greet:(int)n` → `i20@0:8i16`: return `int`, then `id self`,
    /// `SEL _cmd`, `int n` (the implicit receiver/selector + the one real arg).
    #[test]
    fn decode_greet_method() {
        let f = factory();
        let (ptr, ws): (i32, u32) = (8, 1);
        let p = decode_method("-[Greeter greet:]", "i20@0:8i16", &TypeCtx::new(&f, ptr, ws), None)
            .expect("the greet: encoding decodes");
        assert_eq!(p.name, "-[Greeter greet:]");
        assert_eq!(p.intypes.len(), 3, "self + _cmd + n");
        assert_eq!(p.innames, vec!["self", "_cmd", "arg1"]);
        // Return is a 4-byte int.
        assert_eq!(p.outtype.as_ref().unwrap().get_size(), 4);
        // self/_cmd are pointer-sized (id/SEL); n is a 4-byte int.
        assert_eq!(p.intypes[0].get_size(), ptr, "id self is a pointer");
        assert_eq!(p.intypes[1].get_size(), ptr, "SEL _cmd is a pointer");
        assert_eq!(p.intypes[2].get_size(), 4, "int n");
    }

    /// `-(void)set:(double)` style `v` return + a `d` double arg; and the
    /// frame-offset digits are consumed (not mis-parsed as a type).
    #[test]
    fn decode_void_return_double_arg() {
        let f = factory();
        let (ptr, ws): (i32, u32) = (8, 1);
        let p = decode_method("-[C set:]", "v24@0:8d16", &TypeCtx::new(&f, ptr, ws), None).unwrap();
        assert_eq!(p.outtype.as_ref().unwrap().get_size(), 0, "void is the size-0 void type");
        assert_eq!(p.intypes.len(), 3);
        assert_eq!(p.intypes[2].get_size(), 8, "double arg");
    }

    /// An ivar encoding (no trailing frame offset): `q` → a signed 8-byte int.
    #[test]
    fn decode_ivar_long() {
        let f = factory();
        let ty = decode_ivar_type("q", &TypeCtx::new(&f, 8, 1)).unwrap();
        assert_eq!(ty.get_size(), 8);
    }

    /// `^i` (int*) and `^v` (void*) decode to pointer-sized types. (The char-cache
    /// the `*` char* path needs is populated by the real `Architecture` bootstrap,
    /// not by this minimal test factory, so the char* case is covered by the
    /// end-to-end console test rather than here.)
    #[test]
    fn decode_pointers() {
        let f = factory();
        assert_eq!(decode_ivar_type("^i", &TypeCtx::new(&f, 8, 1)).unwrap().get_size(), 8, "int*");
        assert_eq!(decode_ivar_type("^v", &TypeCtx::new(&f, 8, 1)).unwrap().get_size(), 8, "void*");
    }

    /// An empty / garbage encoding yields `None` — the IMP keeps its name, only the
    /// typed prototype is dropped.
    #[test]
    fn empty_encoding_is_none() {
        let f = factory();
        assert!(decode_method("-[C x]", "", &TypeCtx::new(&f, 8, 1), None).is_none());
        assert!(decode_ivar_type("", &TypeCtx::new(&f, 8, 1)).is_none());
    }

    /// An ivar's `@"NSString"` names its class: the pointee is the opaque struct
    /// `NSString`, and a protocol-qualified `@"<Proto>"` falls back to `objc_object`.
    #[test]
    fn decode_ivar_quoted_class() {
        let f = factory();
        let tctx = TypeCtx::new(&f, 8, 1);
        let t = decode_ivar_type("@\"NSString\"", &tctx).unwrap();
        assert_eq!(t.get_metatype(), type_metatype::TYPE_PTR);
        assert_eq!(t.get_ptr_to().unwrap().get_name(), "NSString");
        let t = decode_ivar_type("@\"<Greeter>\"", &tctx).unwrap();
        assert_eq!(t.get_ptr_to().unwrap().get_name(), "objc_object");
    }

    /// A block (`@?`) is one pointer, and pointer nesting is bounded.
    #[test]
    fn blocks_are_one_type_and_nesting_is_bounded() {
        let f = factory();
        let tctx = TypeCtx::new(&f, 8, 1);
        let p = decode_method("-[C a:]", "v24@0:8@?16", &tctx, None).unwrap();
        assert_eq!(p.intypes.len(), 3, "the block is one argument, not `@` then `?`");
        let deep = format!("{}i", "^".repeat(MAX_POINTER_DEPTH + 1));
        assert!(decode_ivar_type(&deep, &tctx).is_none());
        assert!(decode_ivar_type("^^i", &tctx).is_some());
    }

    #[test]
    fn split_method_keeps_struct_and_class_text() {
        let (ret, args) = split_method("v40@0:8{CGRect=dddd}16@\"UIView\"32^v40@?48").unwrap();
        assert_eq!(ret, "v");
        assert_eq!(args, ["@", ":", "{CGRect=dddd}", "@\"UIView\"", "^v", "@?"]);
        assert!(split_method("").is_none());
        assert!(split_method("v@:!").is_none());
    }

    #[test]
    fn struct_type_builds_nested_geometry() {
        let f = factory();
        let tctx = TypeCtx::new(&f, 8, 1);
        let rect = struct_type("{CGRect={CGPoint=dd}{CGSize=dd}}", &tctx).unwrap();
        assert_eq!((rect.get_name(), rect.get_size()), ("CGRect", 32));
        assert_eq!(struct_members("{CGRect={CGPoint=dd}{CGSize=dd}}"), Some(vec!['d'; 4]));
        assert_eq!(struct_members("{CGRect=dd"), None);
        let range = struct_type("{_NSRange=QQ}", &tctx).unwrap();
        assert_eq!(range.get_size(), 16);
        assert!(Rc::ptr_eq(&rect, &struct_type("{CGRect={CGPoint=dd}{CGSize=dd}}", &tctx).unwrap()));
        assert!(struct_type("{?=dd}", &tctx).is_none());
    }
}
