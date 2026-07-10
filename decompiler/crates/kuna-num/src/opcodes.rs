//! Port of `decompiler/cpp/opcodes.hh` + `opcodes.cc` (W1, item
//! `w1-num-pcode-semantics`): the p-code operation enumeration, its name
//! tables, and the opcode marshaling protocol.
//!
//! UB-1 (`docs/rust-port/upstream-bugs.md`): the upstream `opcode_name[]`
//! table is one entry short (74 entries for `CPUI_MAX` = 75) — `get_opname`
//! reads past the array for `CPUI_SPULL` (74) and returns the stale name
//! `"EXTRACT"` for `CPUI_ZPULL` (71).  The port does NOT replicate the bug:
//! [`OPCODE_NAME`] covers the full enum with the canonical names `"ZPULL"`
//! and `"SPULL"` (the names `tests/golden/vectors/opbehavior.csv` pins), a
//! compile-time assertion checks `OPCODE_NAME.len() == CPUI_MAX`, and the
//! sorted-search table [`OPCODE_INDICES`] is regenerated for the corrected
//! names (75 slots; upstream's 74-entry table both lacks `SPULL` and places
//! 71 at the stale `"EXTRACT"` sort position).  For every name the C++
//! sorted search resolves correctly, [`get_opcode`] returns the same opcode;
//! the differences are exactly the UB-1 surface (`"EXTRACT"` no longer
//! resolves, `"ZPULL"`/`"SPULL"` now do).
//!
//! This module also layers the opcode read/write protocol onto the
//! `kuna-base` marshaling traits ([`OpcodeDecoder`] / [`OpcodeEncoder`]):
//! C++ declares `readOpcode`/`writeOpcode` as `Decoder`/`Encoder` virtuals
//! (marshal.hh), but `OpCode` lives in this crate, which depends on
//! `kuna-base` — so the per-format implementations (XML: opcode name string;
//! packed: positive signed integer) are expressed here as extension traits
//! over the concrete `XmlDecode`/`PackedDecode`/`XmlEncode`/`PackedEncode`
//! types (see the deliberate-API note in `kuna_base::marshal`).

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, Encoder, PackedDecode, PackedEncode, XmlDecode, XmlEncode,
};

/// \brief The op-code defining a specific p-code operation (PcodeOp)
///
/// These break up into categories:
///   - Branching operations
///   - Load and Store
///   - Comparison operations
///   - Arithmetic operations
///   - Logical operations
///   - Extension and truncation operations
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpCode {
    /// Copy one operand to another
    CPUI_COPY = 1,
    /// Load from a pointer into a specified address space
    CPUI_LOAD = 2,
    /// Store at a pointer into a specified address space
    CPUI_STORE = 3,

    /// Always branch
    CPUI_BRANCH = 4,
    /// Conditional branch
    CPUI_CBRANCH = 5,
    /// Indirect branch (jumptable)
    CPUI_BRANCHIND = 6,

    /// Call to an absolute address
    CPUI_CALL = 7,
    /// Call through an indirect address
    CPUI_CALLIND = 8,
    /// User-defined operation
    CPUI_CALLOTHER = 9,
    /// Return from subroutine
    CPUI_RETURN = 10,

    // Integer/bit operations
    /// Integer comparison, equality (==)
    CPUI_INT_EQUAL = 11,
    /// Integer comparison, in-equality (!=)
    CPUI_INT_NOTEQUAL = 12,
    /// Integer comparison, signed less-than (<)
    CPUI_INT_SLESS = 13,
    /// Integer comparison, signed less-than-or-equal (<=)
    CPUI_INT_SLESSEQUAL = 14,
    /// Integer comparison, unsigned less-than (<)
    /// (This also indicates a borrow on unsigned substraction)
    CPUI_INT_LESS = 15,
    /// Integer comparison, unsigned less-than-or-equal (<=)
    CPUI_INT_LESSEQUAL = 16,
    /// Zero extension
    CPUI_INT_ZEXT = 17,
    /// Sign extension
    CPUI_INT_SEXT = 18,
    /// Addition, signed or unsigned (+)
    CPUI_INT_ADD = 19,
    /// Subtraction, signed or unsigned (-)
    CPUI_INT_SUB = 20,
    /// Test for unsigned carry
    CPUI_INT_CARRY = 21,
    /// Test for signed carry
    CPUI_INT_SCARRY = 22,
    /// Test for signed borrow
    CPUI_INT_SBORROW = 23,
    /// Twos complement
    CPUI_INT_2COMP = 24,
    /// Logical/bitwise negation (~)
    CPUI_INT_NEGATE = 25,
    /// Logical/bitwise exclusive-or (^)
    CPUI_INT_XOR = 26,
    /// Logical/bitwise and (&)
    CPUI_INT_AND = 27,
    /// Logical/bitwise or (|)
    CPUI_INT_OR = 28,
    /// Left shift (<<)
    CPUI_INT_LEFT = 29,
    /// Right shift, logical (>>)
    CPUI_INT_RIGHT = 30,
    /// Right shift, arithmetic (>>)
    CPUI_INT_SRIGHT = 31,
    /// Integer multiplication, signed and unsigned (*)
    CPUI_INT_MULT = 32,
    /// Integer division, unsigned (/)
    CPUI_INT_DIV = 33,
    /// Integer division, signed (/)
    CPUI_INT_SDIV = 34,
    /// Remainder/modulo, unsigned (%)
    CPUI_INT_REM = 35,
    /// Remainder/modulo, signed (%)
    CPUI_INT_SREM = 36,

    /// Boolean negate (!)
    CPUI_BOOL_NEGATE = 37,
    /// Boolean exclusive-or (^^)
    CPUI_BOOL_XOR = 38,
    /// Boolean and (&&)
    CPUI_BOOL_AND = 39,
    /// Boolean or (||)
    CPUI_BOOL_OR = 40,

    // Floating point operations
    /// Floating-point comparison, equality (==)
    CPUI_FLOAT_EQUAL = 41,
    /// Floating-point comparison, in-equality (!=)
    CPUI_FLOAT_NOTEQUAL = 42,
    /// Floating-point comparison, less-than (<)
    CPUI_FLOAT_LESS = 43,
    /// Floating-point comparison, less-than-or-equal (<=)
    CPUI_FLOAT_LESSEQUAL = 44,
    // Slot 45 is currently unused
    /// Not-a-number test (NaN)
    CPUI_FLOAT_NAN = 46,

    /// Floating-point addition (+)
    CPUI_FLOAT_ADD = 47,
    /// Floating-point division (/)
    CPUI_FLOAT_DIV = 48,
    /// Floating-point multiplication (*)
    CPUI_FLOAT_MULT = 49,
    /// Floating-point subtraction (-)
    CPUI_FLOAT_SUB = 50,
    /// Floating-point negation (-)
    CPUI_FLOAT_NEG = 51,
    /// Floating-point absolute value (abs)
    CPUI_FLOAT_ABS = 52,
    /// Floating-point square root (sqrt)
    CPUI_FLOAT_SQRT = 53,

    /// Convert an integer to a floating-point
    CPUI_FLOAT_INT2FLOAT = 54,
    /// Convert between different floating-point sizes
    CPUI_FLOAT_FLOAT2FLOAT = 55,
    /// Round towards zero
    CPUI_FLOAT_TRUNC = 56,
    /// Round towards +infinity
    CPUI_FLOAT_CEIL = 57,
    /// Round towards -infinity
    CPUI_FLOAT_FLOOR = 58,
    /// Round towards nearest
    CPUI_FLOAT_ROUND = 59,

    // Internal opcodes for simplification. Not
    // typically generated in a direct translation.

    // Data-flow operations
    /// Phi-node operator
    CPUI_MULTIEQUAL = 60,
    /// Copy with an indirect effect
    CPUI_INDIRECT = 61,
    /// Concatenate
    CPUI_PIECE = 62,
    /// Truncate
    CPUI_SUBPIECE = 63,

    /// Cast from one data-type to another
    CPUI_CAST = 64,
    /// Index into an array ([])
    CPUI_PTRADD = 65,
    /// Drill down to a sub-field  (->)
    CPUI_PTRSUB = 66,
    /// Look-up a \e segmented address
    CPUI_SEGMENTOP = 67,
    /// Recover a value from the \e constant \e pool
    CPUI_CPOOLREF = 68,
    /// Allocate a new object (new)
    CPUI_NEW = 69,
    /// Insert a bit-range
    CPUI_INSERT = 70,
    /// Extract an unsigned bit-range
    CPUI_ZPULL = 71,
    /// Count the 1-bits
    CPUI_POPCOUNT = 72,
    /// Count the leading 0-bits
    CPUI_LZCOUNT = 73,
    /// Extract a signed bit-range
    CPUI_SPULL = 74,

    /// Value indicating the end of the op-code values
    CPUI_MAX = 75,
}

impl OpCode {
    /// Convert a raw integer to the matching OpCode (the Rust face of C++
    /// `(OpCode)val` casts on decoded values).
    ///
    /// Returns `None` for any value that is not an actual operation: out of
    /// range, the unused name-table slots 0 (`"BLANK"`) and 45 (`"UNUSED1"`),
    /// and the `CPUI_MAX` sentinel.  C++ happily materializes `(OpCode)0` /
    /// `(OpCode)45` (in-range but nameless enum values that no encoder ever
    /// produces); Rust cannot represent them, so they map to `None` and the
    /// decode protocol rejects them as a bad opcode.
    pub fn from_i32(val: i32) -> Option<OpCode> {
        Some(match val {
            1 => OpCode::CPUI_COPY,
            2 => OpCode::CPUI_LOAD,
            3 => OpCode::CPUI_STORE,
            4 => OpCode::CPUI_BRANCH,
            5 => OpCode::CPUI_CBRANCH,
            6 => OpCode::CPUI_BRANCHIND,
            7 => OpCode::CPUI_CALL,
            8 => OpCode::CPUI_CALLIND,
            9 => OpCode::CPUI_CALLOTHER,
            10 => OpCode::CPUI_RETURN,
            11 => OpCode::CPUI_INT_EQUAL,
            12 => OpCode::CPUI_INT_NOTEQUAL,
            13 => OpCode::CPUI_INT_SLESS,
            14 => OpCode::CPUI_INT_SLESSEQUAL,
            15 => OpCode::CPUI_INT_LESS,
            16 => OpCode::CPUI_INT_LESSEQUAL,
            17 => OpCode::CPUI_INT_ZEXT,
            18 => OpCode::CPUI_INT_SEXT,
            19 => OpCode::CPUI_INT_ADD,
            20 => OpCode::CPUI_INT_SUB,
            21 => OpCode::CPUI_INT_CARRY,
            22 => OpCode::CPUI_INT_SCARRY,
            23 => OpCode::CPUI_INT_SBORROW,
            24 => OpCode::CPUI_INT_2COMP,
            25 => OpCode::CPUI_INT_NEGATE,
            26 => OpCode::CPUI_INT_XOR,
            27 => OpCode::CPUI_INT_AND,
            28 => OpCode::CPUI_INT_OR,
            29 => OpCode::CPUI_INT_LEFT,
            30 => OpCode::CPUI_INT_RIGHT,
            31 => OpCode::CPUI_INT_SRIGHT,
            32 => OpCode::CPUI_INT_MULT,
            33 => OpCode::CPUI_INT_DIV,
            34 => OpCode::CPUI_INT_SDIV,
            35 => OpCode::CPUI_INT_REM,
            36 => OpCode::CPUI_INT_SREM,
            37 => OpCode::CPUI_BOOL_NEGATE,
            38 => OpCode::CPUI_BOOL_XOR,
            39 => OpCode::CPUI_BOOL_AND,
            40 => OpCode::CPUI_BOOL_OR,
            41 => OpCode::CPUI_FLOAT_EQUAL,
            42 => OpCode::CPUI_FLOAT_NOTEQUAL,
            43 => OpCode::CPUI_FLOAT_LESS,
            44 => OpCode::CPUI_FLOAT_LESSEQUAL,
            46 => OpCode::CPUI_FLOAT_NAN,
            47 => OpCode::CPUI_FLOAT_ADD,
            48 => OpCode::CPUI_FLOAT_DIV,
            49 => OpCode::CPUI_FLOAT_MULT,
            50 => OpCode::CPUI_FLOAT_SUB,
            51 => OpCode::CPUI_FLOAT_NEG,
            52 => OpCode::CPUI_FLOAT_ABS,
            53 => OpCode::CPUI_FLOAT_SQRT,
            54 => OpCode::CPUI_FLOAT_INT2FLOAT,
            55 => OpCode::CPUI_FLOAT_FLOAT2FLOAT,
            56 => OpCode::CPUI_FLOAT_TRUNC,
            57 => OpCode::CPUI_FLOAT_CEIL,
            58 => OpCode::CPUI_FLOAT_FLOOR,
            59 => OpCode::CPUI_FLOAT_ROUND,
            60 => OpCode::CPUI_MULTIEQUAL,
            61 => OpCode::CPUI_INDIRECT,
            62 => OpCode::CPUI_PIECE,
            63 => OpCode::CPUI_SUBPIECE,
            64 => OpCode::CPUI_CAST,
            65 => OpCode::CPUI_PTRADD,
            66 => OpCode::CPUI_PTRSUB,
            67 => OpCode::CPUI_SEGMENTOP,
            68 => OpCode::CPUI_CPOOLREF,
            69 => OpCode::CPUI_NEW,
            70 => OpCode::CPUI_INSERT,
            71 => OpCode::CPUI_ZPULL,
            72 => OpCode::CPUI_POPCOUNT,
            73 => OpCode::CPUI_LZCOUNT,
            74 => OpCode::CPUI_SPULL,
            _ => return None,
        })
    }
}

/// \brief Names of operations associated with their opcode number
///
/// Some of the names have been replaced with special placeholder
/// ops for the sleigh compiler and interpreter these are as follows:
///  -  MULTIEQUAL = BUILD
///  -  INDIRECT   = DELAY_SLOT
///  -  PTRADD     = LABEL
///  -  PTRSUB     = CROSSBUILD
///
/// UB-1 fix (see module docs): unlike the upstream 74-entry table, this
/// table has all `CPUI_MAX` (75) entries — slot 71 is the canonical
/// `"ZPULL"` (upstream: stale `"EXTRACT"`) and slot 74 `"SPULL"` exists
/// (upstream: out-of-bounds read).
#[rustfmt::skip]
static OPCODE_NAME: [&str; 75] = [
    "BLANK", "COPY", "LOAD", "STORE",
    "BRANCH", "CBRANCH", "BRANCHIND", "CALL",
    "CALLIND", "CALLOTHER", "RETURN", "INT_EQUAL",
    "INT_NOTEQUAL", "INT_SLESS", "INT_SLESSEQUAL", "INT_LESS",
    "INT_LESSEQUAL", "INT_ZEXT", "INT_SEXT", "INT_ADD",
    "INT_SUB", "INT_CARRY", "INT_SCARRY", "INT_SBORROW",
    "INT_2COMP", "INT_NEGATE", "INT_XOR", "INT_AND",
    "INT_OR", "INT_LEFT", "INT_RIGHT", "INT_SRIGHT",
    "INT_MULT", "INT_DIV", "INT_SDIV", "INT_REM",
    "INT_SREM", "BOOL_NEGATE", "BOOL_XOR", "BOOL_AND",
    "BOOL_OR", "FLOAT_EQUAL", "FLOAT_NOTEQUAL", "FLOAT_LESS",
    "FLOAT_LESSEQUAL", "UNUSED1", "FLOAT_NAN", "FLOAT_ADD",
    "FLOAT_DIV", "FLOAT_MULT", "FLOAT_SUB", "FLOAT_NEG",
    "FLOAT_ABS", "FLOAT_SQRT", "INT2FLOAT", "FLOAT2FLOAT",
    "TRUNC", "CEIL", "FLOOR", "ROUND",
    "BUILD", "DELAY_SLOT", "PIECE", "SUBPIECE", "CAST",
    "LABEL", "CROSSBUILD", "SEGMENTOP", "CPOOLREF", "NEW",
    "INSERT", "ZPULL", "POPCOUNT", "LZCOUNT", "SPULL",
];

// UB-1 (docs/rust-port/upstream-bugs.md): the upstream table is one entry
// short of the enum; this compile-time assertion pins the Rust table to the
// full opcode count so the bug cannot be reintroduced.
const _: () = assert!(OPCODE_NAME.len() == OpCode::CPUI_MAX as usize);

/// The alphabetic sort-order permutation of [`OPCODE_NAME`] driving the
/// binary search in [`get_opcode`] (C++ `opcode_indices`).
///
/// Regenerated for the corrected 75-entry name table (UB-1): upstream's
/// 74-entry table places 71 at the stale `"EXTRACT"` position and lacks 74;
/// here `"SPULL"` (74) sorts between `"SEGMENTOP"` and `"STORE"`, and
/// `"ZPULL"` (71) sorts last.  Slot 0 is `"BLANK"` (excluded from the
/// search), so slots 1..=74 cover every named operation.  Sortedness and
/// permutation-ness are unit-tested below.
#[rustfmt::skip]
static OPCODE_INDICES: [i32; 75] = [
     0, 39, 37, 40, 38,  4,  6, 60,  7,  8,  9, 64,  5, 57,  1, 68,
    66, 61, 55, 52, 47, 48, 41, 43, 44, 49, 46, 51, 42, 53, 50, 58,
    70, 54, 24, 19, 27, 21, 33, 11, 29, 15, 16, 32, 25, 12, 28, 35,
    30, 23, 22, 34, 18, 13, 14, 36, 31, 20, 26, 17, 65,  2, 73, 69,
    62, 72, 10, 59, 67, 74,  3, 63, 56, 45, 71,
];

/// Convert an OpCode to the name as a string.
/// \param opc is an OpCode value
/// \return the name of the operation as a string
pub fn get_opname(opc: OpCode) -> &'static str {
    // C++ indexes the (one-short) table blindly; with the UB-1 fix every
    // real operation 1..=74 is in bounds.  `CPUI_MAX` itself would panic,
    // surfacing the C++ out-of-bounds read as a caught invariant violation.
    OPCODE_NAME[opc as usize] // cast: enum discriminant (1..=74 for real ops) as index
}

/// Convert a name string to the matching OpCode.
/// \param nm is the name of an operation
/// \return the corresponding OpCode value, or `None` if the name isn't an op
///         (C++ returns `(OpCode)0`)
pub fn get_opcode(nm: &str) -> Option<OpCode> {
    let mut min: i32 = 1; // Don't include BLANK
    let mut max: i32 = OpCode::CPUI_MAX as i32 - 1;

    while min <= max {
        // Binary search
        let cur = (min + max) / 2;
        let ind = OPCODE_INDICES[cur as usize]; // Get opcode in cur's sort slot
        // C++ compares `const char*` against std::string: byte-lexicographic,
        // identical to Rust's str ordering.
        let name = OPCODE_NAME[ind as usize]; // cast above: cur in 1..=74, in bounds
        if name < nm {
            min = cur + 1; // Everything equal or below cur is less
        } else if name > nm {
            max = cur - 1; // Everything equal or above cur is greater
        } else {
            // Found the match.  `"UNUSED1"` (slot 45) names no enum value:
            // C++ would return the invalid in-range value (OpCode)45 here;
            // from_i32 maps it to None ("name isn't an op").
            return OpCode::from_i32(ind);
        }
    }
    None // Name isn't an op
}

/// Get the complementary OpCode.
///
/// Every comparison operation has a complementary form that produces
/// the opposite output on the same inputs. Set \b reorder to true if
/// the complimentary operation involves reordering the input parameters.
/// \param opc is the OpCode to complement
/// \param reorder is set to \b true if the inputs need to be reordered
/// \return the complementary OpCode or CPUI_MAX if not given a comparison operation
pub fn get_booleanflip(opc: OpCode, reorder: &mut bool) -> OpCode {
    match opc {
        OpCode::CPUI_INT_EQUAL => {
            *reorder = false;
            OpCode::CPUI_INT_NOTEQUAL
        }
        OpCode::CPUI_INT_NOTEQUAL => {
            *reorder = false;
            OpCode::CPUI_INT_EQUAL
        }
        OpCode::CPUI_INT_SLESS => {
            *reorder = true;
            OpCode::CPUI_INT_SLESSEQUAL
        }
        OpCode::CPUI_INT_SLESSEQUAL => {
            *reorder = true;
            OpCode::CPUI_INT_SLESS
        }
        OpCode::CPUI_INT_LESS => {
            *reorder = true;
            OpCode::CPUI_INT_LESSEQUAL
        }
        OpCode::CPUI_INT_LESSEQUAL => {
            *reorder = true;
            OpCode::CPUI_INT_LESS
        }
        OpCode::CPUI_BOOL_NEGATE => {
            *reorder = false;
            OpCode::CPUI_COPY
        }
        OpCode::CPUI_FLOAT_EQUAL => {
            *reorder = false;
            OpCode::CPUI_FLOAT_NOTEQUAL
        }
        OpCode::CPUI_FLOAT_NOTEQUAL => {
            *reorder = false;
            OpCode::CPUI_FLOAT_EQUAL
        }
        OpCode::CPUI_FLOAT_LESS => {
            *reorder = true;
            OpCode::CPUI_FLOAT_LESSEQUAL
        }
        OpCode::CPUI_FLOAT_LESSEQUAL => {
            *reorder = true;
            OpCode::CPUI_FLOAT_LESS
        }
        _ => OpCode::CPUI_MAX,
    }
}

// ---------------------------------------------------------------------------
// Opcode marshaling protocol (C++ Decoder::readOpcode / Encoder::writeOpcode)
// ---------------------------------------------------------------------------

/// The C++ `Decoder::readOpcode` virtuals, layered onto the concrete
/// `kuna-base` decoders (see module docs for why this is an extension trait
/// rather than part of `Decoder` itself).
pub trait OpcodeDecoder: Decoder {
    /// \brief Read the current attribute as a p-code OpCode
    /// (C++ `readOpcode(void)`).
    fn read_opcode(&mut self) -> KunaResult<OpCode>;

    /// \brief Find the specific attribute in the current element and return
    /// it as an OpCode (C++ `readOpcode(AttributeId &)`).
    fn read_opcode_id(&mut self, attrib_id: &AttributeId) -> KunaResult<OpCode>;
}

/// The C++ `Encoder::writeOpcode` virtual, layered onto the concrete
/// `kuna-base` encoders.
pub trait OpcodeEncoder: Encoder {
    /// \brief Write a p-code operation opcode into the encoding, associating
    /// it with the given annotation (C++ `writeOpcode`).
    fn write_opcode(&mut self, attrib_id: &AttributeId, opc: OpCode);
}

/// XML protocol: the opcode name string, resolved with `get_opcode`
/// (marshal.cc `XmlDecode::readOpcode`).
fn opcode_from_name_bytes(nm: &[u8]) -> KunaResult<OpCode> {
    // The table names are pure ASCII, so a non-UTF-8 attribute value can
    // never match one; rejecting it takes the same "Bad encoded OpCode"
    // path the C++ byte comparison would end in.
    let opc = std::str::from_utf8(nm).ok().and_then(get_opcode);
    match opc {
        Some(opc) => Ok(opc),
        None => Err(KunaError::decoder("Bad encoded OpCode")),
    }
}

impl OpcodeDecoder for XmlDecode<'_> {
    fn read_opcode(&mut self) -> KunaResult<OpCode> {
        let nm = self.read_string()?;
        opcode_from_name_bytes(&nm)
    }

    fn read_opcode_id(&mut self, attrib_id: &AttributeId) -> KunaResult<OpCode> {
        // C++ handles ATTRIB_CONTENT vs findMatchingAttribute exactly as
        // readString(AttributeId) does.
        let nm = self.read_string_id(attrib_id)?;
        opcode_from_name_bytes(&nm)
    }
}

/// Packed protocol: a positive signed integer holding the raw enum value
/// (marshal.cc `PackedDecode::readOpcode`).
fn opcode_from_packed_integer(raw: i64) -> KunaResult<OpCode> {
    let val = raw as i32; // cast: C++ `(int4)readSignedInteger()` truncation
    if val < 0 || val >= OpCode::CPUI_MAX as i32 {
        return Err(KunaError::decoder("Bad encoded OpCode"));
    }
    // C++ returns the in-range but invalid enum values 0 and 45 unchecked;
    // Rust cannot represent them (see OpCode::from_i32) and rejects them
    // with the same error a genuinely out-of-range value gets.
    OpCode::from_i32(val).ok_or_else(|| KunaError::decoder("Bad encoded OpCode"))
}

impl OpcodeDecoder for PackedDecode<'_> {
    fn read_opcode(&mut self) -> KunaResult<OpCode> {
        opcode_from_packed_integer(self.read_signed_integer()?)
    }

    fn read_opcode_id(&mut self, attrib_id: &AttributeId) -> KunaResult<OpCode> {
        // C++: findMatchingAttribute + readOpcode + curPos restore — the
        // same sequencing readSignedInteger(AttributeId) performs.
        opcode_from_packed_integer(self.read_signed_integer_id(attrib_id)?)
    }
}

impl OpcodeEncoder for XmlEncode<'_> {
    fn write_opcode(&mut self, attrib_id: &AttributeId, opc: OpCode) {
        // C++ writes get_opname(opc) without xml_escape; opcode names contain
        // no escapable characters, so writeString's escaping is the identity
        // and the emitted bytes are identical.
        self.write_string(attrib_id, get_opname(opc).as_bytes());
    }
}

impl OpcodeEncoder for PackedEncode<'_> {
    fn write_opcode(&mut self, attrib_id: &AttributeId, opc: OpCode) {
        // C++: writeHeader(ATTRIBUTE, id) + writeInteger(SIGNEDINT_POSITIVE,
        // opc) — byte-identical to writeSignedInteger for a non-negative
        // value.
        self.write_signed_integer(attrib_id, opc as i64); // cast: enum discriminant, non-negative
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every actual operation, in enum order (no BLANK/UNUSED1/CPUI_MAX).
    fn all_ops() -> Vec<OpCode> {
        (1..OpCode::CPUI_MAX as i32).filter_map(OpCode::from_i32).collect()
    }

    #[test]
    fn test_opcodes_table_is_sorted_permutation() {
        // OPCODE_INDICES must be a permutation of 0..CPUI_MAX ...
        let mut seen = [false; 75];
        for &ind in OPCODE_INDICES.iter() {
            assert!(!seen[ind as usize], "duplicate sort slot {ind}");
            seen[ind as usize] = true;
        }
        assert!(seen.iter().all(|&b| b));
        // ... in strictly ascending (byte-lexicographic) name order.
        for w in OPCODE_INDICES.windows(2) {
            assert!(
                OPCODE_NAME[w[0] as usize] < OPCODE_NAME[w[1] as usize],
                "sort order violated at {} vs {}",
                OPCODE_NAME[w[0] as usize],
                OPCODE_NAME[w[1] as usize]
            );
        }
    }

    #[test]
    fn test_opcodes_name_roundtrip_all_ops() {
        // get_opcode(get_opname(op)) == op for every real operation —
        // these are exactly the names the C++ sorted search resolves
        // correctly (UB-1 aside), plus ZPULL/SPULL which it does not.
        for op in all_ops() {
            assert_eq!(get_opcode(get_opname(op)), Some(op), "roundtrip of {op:?}");
        }
        assert_eq!(all_ops().len(), 73); // 74 slots minus unused 45
    }

    #[test]
    fn test_opcodes_ub1_names_fixed() {
        // UB-1: upstream returns "EXTRACT" for ZPULL and reads out of
        // bounds for SPULL; the canonical names are pinned here and the
        // stale name no longer resolves.
        assert_eq!(get_opname(OpCode::CPUI_ZPULL), "ZPULL");
        assert_eq!(get_opname(OpCode::CPUI_SPULL), "SPULL");
        assert_eq!(get_opcode("EXTRACT"), None);
        assert_eq!(get_opcode("ZPULL"), Some(OpCode::CPUI_ZPULL));
        assert_eq!(get_opcode("SPULL"), Some(OpCode::CPUI_SPULL));
    }

    #[test]
    fn test_opcodes_lookup_misses() {
        assert_eq!(get_opcode(""), None);
        assert_eq!(get_opcode("BLANK"), None); // excluded by min=1
        assert_eq!(get_opcode("UNUSED1"), None); // names no enum value
        assert_eq!(get_opcode("INT_AD"), None);
        assert_eq!(get_opcode("INT_ADDX"), None);
        // Greater than every table name: the C++ search would probe the
        // out-of-bounds sorted slot 74 here (part of the UB-1 surface).
        assert_eq!(get_opcode("ZZZZ"), None);
        assert_eq!(get_opcode("AAAA"), None);
    }

    #[test]
    fn test_opcodes_booleanflip_pairs() {
        let cases = [
            (OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL, false),
            (OpCode::CPUI_INT_NOTEQUAL, OpCode::CPUI_INT_EQUAL, false),
            (OpCode::CPUI_INT_SLESS, OpCode::CPUI_INT_SLESSEQUAL, true),
            (OpCode::CPUI_INT_SLESSEQUAL, OpCode::CPUI_INT_SLESS, true),
            (OpCode::CPUI_INT_LESS, OpCode::CPUI_INT_LESSEQUAL, true),
            (OpCode::CPUI_INT_LESSEQUAL, OpCode::CPUI_INT_LESS, true),
            (OpCode::CPUI_BOOL_NEGATE, OpCode::CPUI_COPY, false),
            (OpCode::CPUI_FLOAT_EQUAL, OpCode::CPUI_FLOAT_NOTEQUAL, false),
            (OpCode::CPUI_FLOAT_NOTEQUAL, OpCode::CPUI_FLOAT_EQUAL, false),
            (OpCode::CPUI_FLOAT_LESS, OpCode::CPUI_FLOAT_LESSEQUAL, true),
            (OpCode::CPUI_FLOAT_LESSEQUAL, OpCode::CPUI_FLOAT_LESS, true),
        ];
        for (opc, flip, expect_reorder) in cases {
            let mut reorder = !expect_reorder; // must be overwritten
            assert_eq!(get_booleanflip(opc, &mut reorder), flip);
            assert_eq!(reorder, expect_reorder, "reorder flag for {opc:?}");
        }
        // Non-comparison ops return CPUI_MAX and leave reorder untouched.
        let mut reorder = true;
        assert_eq!(get_booleanflip(OpCode::CPUI_INT_ADD, &mut reorder), OpCode::CPUI_MAX);
        assert!(reorder);
    }
}
