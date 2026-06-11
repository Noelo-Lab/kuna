//! Port of `decompiler/cpp/context.hh` + `context.cc` (W2, item
//! `w2-sleigh-context`): objects describing the context around the parsing
//! of an instruction by the SLEIGH engine.
//!
//! Scope note (what lives here now vs. later):
//!
//! - [`Token`] and [`FixedHandle`] are ported here: they depend only on
//!   W1 types and are needed across the sleigh symbol/decode waves.
//! - `ConstructState`, `ContextSet`, `ParserContext`, `ParserWalker` and
//!   `ParserWalkerChange` are **deferred to the sleigh decode-engine item**:
//!   they are built around `Constructor` / `TripleSymbol` /
//!   `OperandSymbol` (`slghsymbol.{hh,cc}`) and `Translate`
//!   (`translate.{hh,cc}`), none of which are ported yet (see this crate's
//!   `lib.rs`: `sleigh.{hh,cc}` is listed as "the decode engine:
//!   ParserContext, PcodeCacher, Sleigh").  Porting them now would force
//!   placeholder types into modules owned by later items.
//! - C++ `SleighError` (defined in `context.hh`) is already represented by
//!   `kuna_base::error::KunaError::Sleigh` (constructor helper
//!   `KunaError::sleigh`), per ADR 0004.
//! - The context *database* machinery the parser consults (`ContextCache`,
//!   `ContextDatabase`, ...) is declared in `globalcontext.hh` upstream and
//!   is ported in [`crate::globalcontext`].
//!
//! Representation notes:
//!
//! - Names are byte strings (`Vec<u8>`), following the workspace marshal
//!   convention: `Token` names arrive from `.sla` decode via
//!   `Decoder::read_string`, which returns bytes.
//! - C++ `AddrSpace *` members (possibly null) are `Option<Rc<AddrSpace>>`,
//!   following the kuna-base `Address`/`VarnodeData` convention.

use std::rc::Rc;

use kuna_base::space::AddrSpace;

/// \brief A multiple-byte sized chunk of pattern in the instruction byte
/// stream
#[derive(Debug, Clone)]
pub struct Token {
    /// Name of the token
    name: Vec<u8>,
    /// Number of bytes in token
    size: i32,
    /// Index of \b this token, for resolving offsets
    index: i32,
    /// Set to \b true if encodings within \b this token are big endian
    bigendian: bool,
}

impl Token {
    /// Constructor
    pub fn new(nm: &[u8], sz: i32, be: bool, ind: i32) -> Token {
        Token { name: nm.to_vec(), size: sz, bigendian: be, index: ind }
    }

    /// Get the size in bytes
    pub fn get_size(&self) -> i32 {
        self.size
    }

    /// Return \b true if encodings within \b this are big endian
    pub fn is_big_endian(&self) -> bool {
        self.bigendian
    }

    /// Get the index associated with \b this token
    pub fn get_index(&self) -> i32 {
        self.index
    }

    /// Get the name of the token
    pub fn get_name(&self) -> &[u8] {
        &self.name
    }
}

/// \brief A resolved version of (or pointer to) a SLEIGH defined Varnode
///
/// For a static Varnode, this is the triple (address space, offset, size)
/// for the Varnode.  For a dynamic Varnode, this also encodes the pointer
/// Varnode containing the dynamic offset and a temporary storage location
/// for the dereferenced value.
///
/// (C++ leaves a default-constructed FixedHandle uninitialized; the Rust
/// `Default` zeroes every field, with the null space pointers as `None`.)
#[derive(Debug, Clone, Default)]
pub struct FixedHandle {
    /// The address space of the Varnode
    pub space: Option<Rc<AddrSpace>>,
    /// Number of bytes in the Varnode
    pub size: u32,
    /// Null \e or the space where the dynamic offset is stored
    pub offset_space: Option<Rc<AddrSpace>>,
    /// The offset for the static Varnode \e or the offset for the pointer
    pub offset_offset: u64,
    /// Size of pointer
    pub offset_size: u32,
    /// Address space for temporary location for value
    pub temp_space: Option<Rc<AddrSpace>>,
    /// Offset of the temporary location
    pub temp_offset: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_token_accessors() {
        let tok = Token::new(b"instr", 4, true, 2);
        assert_eq!(tok.get_name(), b"instr");
        assert_eq!(tok.get_size(), 4);
        assert!(tok.is_big_endian());
        assert_eq!(tok.get_index(), 2);
        let tok2 = Token::new(b"tok16", 2, false, 0);
        assert!(!tok2.is_big_endian());
        assert_eq!(tok2.get_size(), 2);
    }

    #[test]
    fn test_context_fixedhandle_default() {
        let hand = FixedHandle::default();
        assert!(hand.space.is_none());
        assert!(hand.offset_space.is_none());
        assert!(hand.temp_space.is_none());
        assert_eq!(hand.size, 0);
        assert_eq!(hand.offset_offset, 0);
        assert_eq!(hand.offset_size, 0);
        assert_eq!(hand.temp_offset, 0);
    }
}
