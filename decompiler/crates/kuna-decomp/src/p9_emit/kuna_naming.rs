//! Port of `decompiler/cpp/kuna_naming.{cc,hh}` — kuna's angr-style default
//! naming policy.
//!
//! This is a kuna-owned policy file (NOT upstream Ghidra).  It centralizes the
//! default naming scheme that makes decompiler output read like angr's:
//!
//!   - locals / temps / extraout / unaff  -> `v1`, `v2`, `v3` ...
//!   - arguments                          -> `a0`, `a1` ...   (by signature slot)
//!   - global data                        -> `dat_<addr>`   (lowercase, no leading 0s)
//!   - unnamed functions                  -> `sub_<addr>`
//!   - code labels                        -> `label_<addr>`
//!   - plus a `// rax` / `// stack - 0x10` source-location comment per local.
//!
//! It is a P0 surface-rendering policy: a single master toggle
//! `option namestyle angr|ghidra` (default `angr`) flips the whole scheme.
//!
//! # Where the pieces live
//!
//! The pure address-rendering helpers (`dat_`/`sub_`/`label_`/`a<n>`) were ported
//! into [`crate::database`] as a self-contained DIV-5 boundary (the symbol
//! database is their only consumer there); this module **re-exports** them so the
//! `kuna_naming` surface is complete, and adds the two helpers the database
//! doesn't need: [`kuna_is_generated_name`] (consumed by `coreaction.cc`'s
//! parameter-comment skip) and [`kuna_storage_comment`] (consumed by
//! `printc.cc`'s `emitVarDeclStatement`).
//!
//! `kunaAngrNaming(glb)` is the runtime gate; in the Rust tree the flag lives on
//! `Architecture::name_style_angr`, so [`kuna_angr_naming`] takes the flag value
//! directly (the caller reads `glb->name_style_angr`).
//!
//! # Storage-comment boundary
//!
//! C++ `kunaStorageComment(glb,sym,loc)` reaches into the `Symbol` /
//! `SymbolEntry` / `Funcdata` / `JoinRecord` / `Translate` graph to resolve a
//! symbol's storage location.  Those are owned by other porters (W5/W6).  To stay
//! within this module's boundary, the resolution is split: the caller (the
//! printer, which already holds those objects) gathers the storage facts into a
//! [`StorageView`], and [`kuna_storage_comment`] renders the comment text from
//! it — the rendering rules (register name lowercased, `stack +/- 0x..`, the
//! `rdx:rax` join form, the `tmp` fallback) are transcribed exactly.

use kuna_base::address::sign_extend;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, intb, uintb};

// Re-export the pure address-rendering helpers ported into `database.rs`
// (kuna_naming.cc:55-89).  These are the canonical kuna implementations.
pub use crate::database::{
    kuna_arg_name, kuna_function_name, kuna_global_data_name, kuna_label_name,
};

/// Marshaling element `<namestyle>` (kuna, C++
/// `ELEM_NAMESTYLE = ElementId("namestyle",4020)`; kuna 4000+ range).
pub const ELEM_NAMESTYLE: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("namestyle", 4020);

/// C++ `kunaAngrNaming(const Architecture *glb)` (kuna_naming.cc:30-34).
///
/// "Is angr-style naming active for this Architecture?"  In C++ this is
/// `(glb != 0) && glb->name_style_angr`; the Rust caller passes the flag value
/// (`glb->name_style_angr`) directly — a null `glb` is modeled by passing
/// `false`.
pub fn kuna_angr_naming(name_style_angr: bool) -> bool {
    name_style_angr
}

/// C++ `kunaToLower(string &s)` (kuna_naming.cc:48-53): lowercase a register name
/// in place (angr renders registers lowercase).
///
/// Transcribes the C++ `s[i] = tolower((unsigned char)s[i])` byte-for-byte: the C
/// `tolower` only maps ASCII `A`-`Z`, leaving every other byte (including those
/// >= 0x80) unchanged, so the byte length and all non-ASCII bytes are preserved.
fn kuna_to_lower(s: &mut String) {
    // Register names are ASCII, but operate on the raw bytes to match C exactly
    // (a `to_ascii_lowercase` of a UTF-8 string would touch only ASCII anyway,
    // but going through bytes guarantees byte-identical output for any input).
    let mut bytes = std::mem::take(s).into_bytes();
    for b in bytes.iter_mut() {
        *b = b.to_ascii_lowercase();
    }
    // The mutation only lowercases ASCII, so the buffer stays valid UTF-8.
    *s = String::from_utf8(bytes).expect("ASCII-only lowercasing preserves UTF-8");
}

/// C++ `kunaIsGeneratedName(const string &nm)` (kuna_naming.cc:91-106).
///
/// Recognize a generated default name in either scheme: `param_N` (ghidra) or
/// `vN` / `aN` (angr).  Returns `true` only when the prefix is followed by one or
/// more decimal digits and nothing else.
pub fn kuna_is_generated_name(nm: &str) -> bool {
    let bytes = nm.as_bytes();
    let start: usize;
    if nm.len() >= 6 && &nm[0..6] == "param_" {
        start = 6;
    } else if bytes.len() >= 2
        && (bytes[0] == b'v' || bytes[0] == b'a')
        && bytes[1].is_ascii_digit()
    {
        start = 1;
    } else {
        return false;
    }
    for &b in &bytes[start..] {
        if !b.is_ascii_digit() {
            return false;
        }
    }
    nm.len() > start
}

/// The resolved storage facts [`kuna_storage_comment`] renders, gathered by the
/// caller from the `Symbol`/`SymbolEntry`/`Funcdata`/`JoinRecord`/`Translate`
/// graph (boundary: see module docs).
///
/// This mirrors exactly the data the C++ `kunaStorageComment` reads off `glb`
/// and `sym`: the storage address+size after the dynamic-symbol fall-through,
/// the register name covering that storage (empty if none), the stack space and
/// (if a JOIN) the constituent pieces.
pub struct StorageView<'a> {
    /// The storage [`AddrSpace`] (`addr.getSpace()`), or `None` if the symbol has
    /// no representative storage at all (the C++ "no representative storage" path
    /// that yields `tmp`).
    pub space: Option<&'a AddrSpace>,
    /// The storage offset (`addr.getOffset()`).
    pub offset: uintb,
    /// The address size in bytes (`addr.getAddrSize()`) — used for the stack
    /// sign-extension.
    pub addr_size: int4,
    /// The word size of the storage space (`spc->getWordSize()`).
    pub word_size: int4,
    /// The register name covering `(space, offset, size)`
    /// (`glb->translate->getRegisterName(...)`), or empty if none.
    pub register_name: String,
    /// `true` if `space` is the stack space (`spc == glb->getStackSpace()`).
    pub is_stack: bool,
    /// For a JOIN-space storage, the rendered piece names already resolved by the
    /// caller (each is the lowercased register name, or `spacename_<hexoffset>`
    /// when the piece has no register).  Empty/`None` for non-JOIN storage or a
    /// join record with no pieces.
    pub join_pieces: Option<Vec<String>>,
}

/// C++ `kunaStorageComment(const Architecture *glb,const Symbol *sym,string &out)`
/// (kuna_naming.cc:108-180).
///
/// Build a `// rax` / `// stack - 0x10` style storage annotation for a symbol's
/// storage.  Returns `Some(text)` when a comment was produced, `None` when none
/// is (the C++ `false` cases: null `glb`/`sym` or `numEntries()==0`, modeled by
/// the caller passing `view = None`).
///
/// The dynamic-symbol fall-through (resolve a representative Varnode through the
/// function) and the `numEntries()/getFirstWholeMap()` reads happen on the caller
/// side (they touch W5/W6 types); [`StorageView`] carries the post-resolution
/// facts.  A `space == None` view models the "no representative storage at all"
/// branch that yields `tmp`.
pub fn kuna_storage_comment(view: &StorageView) -> String {
    let spc = match view.space {
        Some(s) => s,
        None => return "tmp".to_string(),
    };

    // Register-backed storage -> the register name, lowercased ("rax").
    if !view.register_name.is_empty() {
        let mut reg = view.register_name.clone();
        kuna_to_lower(&mut reg);
        return reg;
    }

    // Stack-backed storage -> a frame-relative signed offset ("stack - 0x10").
    if view.is_stack {
        let mut soff: intb =
            AddrSpace::byte_to_address(view.offset, view.word_size as u32) as intb;
        soff = sign_extend(soff, view.addr_size * 8 - 1);
        if soff < 0 {
            return format!("stack - 0x{:x}", soff.unsigned_abs());
        } else {
            return format!("stack + 0x{:x}", soff as uintb);
        }
    }

    // Split value -> its constituent register pieces ("rdx:rax").
    if spc.get_type() == spacetype::IPTR_JOIN {
        if let Some(pieces) = &view.join_pieces {
            if !pieces.is_empty() {
                return pieces.join(":");
            }
        }
    }

    // A decompiler-internal temporary (unique space) with no machine register or
    // stack home: annotate it honestly so every local still carries a location.
    "tmp".to_string()
}

/// (kuna) Master naming-scheme toggle: `namestyle angr|ghidra`
/// (C++ `OptionNameStyle`, kuna_naming.cc:16-28).
///
/// "angr" (default) renders angr-style names (vN/aN/dat_/sub_/label_) plus a
/// storage-location comment on each local; "ghidra" restores upstream Ghidra
/// naming (iVar/param_/`<type>Ram`/func_/code_).
///
/// Follows the kuna-option idiom (`OptionLowerSwitch::apply`): parse + validate
/// here, return the resolved flag plus the confirmation message; the caller
/// writes the flag into `Architecture::name_style_angr` (W4/W9 dispatch).
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionNameStyle;

impl OptionNameStyle {
    /// The option name (C++ `name = "namestyle"`).
    pub const NAME: &'static str = "namestyle";

    /// C++ `OptionNameStyle::apply`.
    ///
    /// Returns `Ok((val, message))` on a valid value; an invalid value yields the
    /// exact upstream validation message as a `ParseError`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = if p1 == "angr" {
            true
        } else if p1 == "ghidra" {
            false
        } else {
            return Err(KunaError::parse("namestyle must be \"angr\" or \"ghidra\""));
        };
        // glb->name_style_angr = val;  -- left to the caller (STUB(W4/W9)).
        Ok((val, format!("Naming scheme set to {p1}")))
    }
}

#[cfg(test)]
#[path = "kuna_naming/tests.rs"]
mod tests;
