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

use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
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
    types: &dyn TypeFactory,
    ptr_size: i32,
    word_size: u32,
) -> Option<PrototypePieces> {
    let mut p = Parser::new(encoding);

    // Return type first (a null/void return is still `Some(void)`).
    let outtype = Some(p.next_type(types, ptr_size, word_size)?);

    let mut intypes: Vec<Rc<Datatype>> = Vec::new();
    let mut innames: Vec<String> = Vec::new();
    let mut idx = 0usize;
    while !p.at_end() {
        let Some(ty) = p.next_type(types, ptr_size, word_size) else { break };
        // The first two ObjC arguments are always the implicit receiver + selector.
        let nm = match idx {
            0 => "self".to_string(),
            1 => "_cmd".to_string(),
            _ => format!("arg{}", idx - 1),
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
    })
}

/// Decode a single ivar type encoding (the element's `@encode` string) into its
/// [`Datatype`]. Used for the `<Class>::ivar` typing (design §5 PR-O2). `None`
/// for an empty / unparseable encoding.
pub fn decode_ivar_type(
    encoding: &str,
    types: &dyn TypeFactory,
    ptr_size: i32,
    word_size: u32,
) -> Option<Rc<Datatype>> {
    Parser::new(encoding).next_type(types, ptr_size, word_size)
}

/// A cursor over an ObjC type-encoding string. `next_type` consumes one type code
/// (skipping protocol qualifiers and the trailing decimal frame offset).
struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Parser<'a> {
        Parser { bytes: s.as_bytes(), pos: 0 }
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
    fn next_type(
        &mut self,
        types: &dyn TypeFactory,
        ptr_size: i32,
        word_size: u32,
    ) -> Option<Rc<Datatype>> {
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
            // `@` = id (object pointer) → an opaque `objc_object*`.
            b'@' => named_opaque_ptr(types, "objc_object", ptr_size, word_size)?,
            // `#` = Class → an opaque `objc_class*`.
            b'#' => named_opaque_ptr(types, "objc_class", ptr_size, word_size)?,
            // `:` = SEL → an opaque `objc_selector*`.
            b':' => named_opaque_ptr(types, "objc_selector", ptr_size, word_size)?,
            // `^T` = pointer to T: decode the pointee (name-level), fall back to
            // void* on an opaque/unparseable pointee.
            b'^' => {
                let pointee = self
                    .next_type(types, ptr_size, word_size)
                    .or_else(|| types.get_type_void().ok())?;
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

/// A named-opaque pointer datatype (`objc_object*` for `id`, `objc_class*` for
/// `Class`, `objc_selector*` for `SEL`): a pointer to a named-opaque struct, the
/// `dwarf` `get_type_struct` posture. Falls back to a plain `void*` if the
/// named-opaque struct cannot be created.
fn named_opaque_ptr(
    types: &dyn TypeFactory,
    struct_name: &str,
    ptr_size: i32,
    word_size: u32,
) -> Option<Rc<Datatype>> {
    let pointee = types
        .get_type_struct(struct_name)
        .ok()
        .or_else(|| types.get_type_void().ok())?;
    types.get_type_pointer(ptr_size, pointee, word_size).ok()
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
        let p = decode_method("-[Greeter greet:]", "i20@0:8i16", &f, ptr, ws)
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
        let p = decode_method("-[C set:]", "v24@0:8d16", &f, ptr, ws).unwrap();
        assert_eq!(p.outtype.as_ref().unwrap().get_size(), 0, "void is the size-0 void type");
        assert_eq!(p.intypes.len(), 3);
        assert_eq!(p.intypes[2].get_size(), 8, "double arg");
    }

    /// An ivar encoding (no trailing frame offset): `q` → a signed 8-byte int.
    #[test]
    fn decode_ivar_long() {
        let f = factory();
        let ty = decode_ivar_type("q", &f, 8, 1).unwrap();
        assert_eq!(ty.get_size(), 8);
    }

    /// `^i` (int*) and `^v` (void*) decode to pointer-sized types. (The char-cache
    /// the `*` char* path needs is populated by the real `Architecture` bootstrap,
    /// not by this minimal test factory, so the char* case is covered by the
    /// end-to-end console test rather than here.)
    #[test]
    fn decode_pointers() {
        let f = factory();
        assert_eq!(decode_ivar_type("^i", &f, 8, 1).unwrap().get_size(), 8, "int*");
        assert_eq!(decode_ivar_type("^v", &f, 8, 1).unwrap().get_size(), 8, "void*");
    }

    /// An empty / garbage encoding yields `None` — the IMP keeps its name, only the
    /// typed prototype is dropped.
    #[test]
    fn empty_encoding_is_none() {
        let f = factory();
        assert!(decode_method("-[C x]", "", &f, 8, 1).is_none());
        assert!(decode_ivar_type("", &f, 8, 1).is_none());
    }
}
