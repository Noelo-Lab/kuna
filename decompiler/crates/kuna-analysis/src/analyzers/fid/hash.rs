//! The FID hashing core — FNV-1a64 + the operand-masked `FidHashQuad`.
//!
//! A **byte-exact** port of Ghidra's FID hasher; a one-bit divergence makes every
//! hash miss silently (a fingerprint database lookup just fails to find a match,
//! with no crash). The constants and the per-instruction loop order are therefore
//! reproduced verbatim — see the cited upstream sources.
//!
//! # Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - `Ghidra/Framework/Generic/src/main/java/generic/hash/FNV1a64MessageDigest.java`
//!   — the FNV-1a64 digest ([`Fnv1a64`]).
//! - `Ghidra/Framework/Generic/src/main/java/generic/hash/AbstractMessageDigest.java`
//!   — `update(int)` is 4 bytes **big-endian** (`>>24` first); the FNV digest does
//!   not override it, so it inherits this default ([`Fnv1a64::update_i32`]).
//! - `Ghidra/Features/FunctionID/src/main/java/ghidra/feature/fid/hash/MessageDigestFidHasher.java`
//!   — `MessageDigestFidHasher.hash` ([`FidHasher::hash`], the per-instruction
//!   loop, the 4-way scalar rule, the masked-opcode feed).
//! - `Ghidra/Features/FunctionID/src/main/java/ghidra/feature/fid/hash/FidHashQuadImpl.java`
//!   — the result record ([`FidHashQuad`]).
//!
//! # The critical interleave invariant (§4.2 of `docs/fid-design.md`)
//!
//! Both digests (`full` and `specific`) run the **same instruction stream in the
//! same order**; the only per-operand difference is `full_u` vs `spec_u`; and the
//! masked-byte `update_bytes` for an instruction comes **after** all of that
//! instruction's operand `update_i32` writes. Get the interleave wrong and FNV
//! diverges even with identical inputs.
//!
//! All integer arithmetic uses `wrapping_*` because Java's `int`/`long`
//! two's-complement overflow is observable in the resulting hash.

use crate::fid::skipper::InstructionSkipper;

// --- §4.1 FNV-1a64 -----------------------------------------------------------

/// `FNV1a64MessageDigest.FNV_64_OFFSET_BASIS`.
const FNV_64_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
/// `FNV1a64MessageDigest.FNV_64_PRIME` (`0x100000001b3`).
const FNV_64_PRIME: u64 = 1_099_511_628_211;

/// The FNV-1a64 message digest, a line-faithful port of Ghidra's
/// `FNV1a64MessageDigest`.
///
/// `update_byte` XORs the byte (as an unsigned byte, `& 0xff`) into the running
/// hash then multiplies by the FNV prime — both with wrapping `u64` arithmetic so
/// Java `long` overflow is reproduced. `digest_long` returns the hash and **resets
/// to the offset basis** (`digestLong` calls `init()`), so one `Fnv1a64` can be
/// reused across functions.
#[derive(Debug, Clone)]
pub struct Fnv1a64 {
    hashvalue: u64,
}

impl Default for Fnv1a64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Fnv1a64 {
    /// `new FNV1a64MessageDigest()` — seeds the hash to the offset basis.
    pub fn new() -> Self {
        Self { hashvalue: FNV_64_OFFSET_BASIS }
    }

    /// `update(byte input)`: `hashvalue ^= input & 0xff; hashvalue *= FNV_64_PRIME;`.
    #[inline]
    pub fn update_byte(&mut self, b: u8) {
        self.hashvalue ^= u64::from(b);
        self.hashvalue = self.hashvalue.wrapping_mul(FNV_64_PRIME);
    }

    /// `update(byte[] input, int offset, int len)` over a whole slice.
    #[inline]
    pub fn update_bytes(&mut self, buf: &[u8]) {
        for &b in buf {
            self.update_byte(b);
        }
    }

    /// `AbstractMessageDigest.update(int input)` — 4 bytes **big-endian**
    /// (`>>24` byte first). The FNV digest does not override `update(int)`, so it
    /// inherits this. The `i32` is reinterpreted as `u32` for the byte extraction
    /// (Java masks each shift with `& 0xff`).
    #[inline]
    pub fn update_i32(&mut self, v: i32) {
        let v = v as u32;
        self.update_byte((v >> 24) as u8);
        self.update_byte((v >> 16) as u8);
        self.update_byte((v >> 8) as u8);
        self.update_byte(v as u8);
    }

    /// `digestLong()`: return the hash and reset to the offset basis.
    #[inline]
    pub fn digest_long(&mut self) -> u64 {
        let result = self.hashvalue;
        self.hashvalue = FNV_64_OFFSET_BASIS;
        result
    }

    /// `reset()`: re-seed to the offset basis.
    #[inline]
    pub fn reset(&mut self) {
        self.hashvalue = FNV_64_OFFSET_BASIS;
    }
}

// --- §4.2 the quad + hasher --------------------------------------------------

/// `MessageDigestFidHasher.shortCodeUnitLimit` — the minimum code-unit count for a
/// hashable function (FID's `SHORT_HASH_CODE_UNIT_LENGTH`). Extents with fewer
/// than this many code units are rejected (`hash` returns `None`).
pub const SHORT_HASH_CODE_UNIT_LENGTH: i32 = 4;

/// `Byte.MAX_VALUE` — the cap on `specific_addl_size`.
const BYTE_MAX_VALUE: i32 = 127;

/// `Short.MAX_VALUE` — the code-unit-index break guard.
const SHORT_MAX_VALUE: i32 = 32767;

/// The placeholder a scalar contributes to a hash when its *value* is suppressed
/// (relocation-bound, whole-operand address, or too-large non-whole-operand
/// scalar). The full hash **never** sees a real scalar value — only this
/// placeholder — which is what makes the full hash position/relocation
/// independent. Verbatim `0xfeeddead` from `MessageDigestFidHasher`.
const FEEDDEAD: i32 = 0xfeed_deadu32 as i32;

/// `FidHashQuadImpl` — the four-field function fingerprint (a faithful port; the
/// field types match Ghidra's `short`/`long`/`byte`/`long`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FidHashQuad {
    /// `codeUnitSize` (Ghidra `short fullCount`): the post-skip code-unit count
    /// minus the call count.
    pub code_unit_size: i16,
    /// `fullHash`: the operand-masked FNV-1a64 digest (the position/relocation-
    /// independent fingerprint used for the database index).
    pub full_hash: u64,
    /// `specificCodeUnitSize` (Ghidra `byte additionalSpecificCount`):
    /// `min(specificCount, 127)`.
    pub specific_addl_size: u8,
    /// `specificHash`: the FNV-1a64 digest that also folds in usable scalar
    /// values (the disambiguating tiebreak field).
    pub specific_hash: u64,
}

/// A single instruction's contribution to the hash, the pure-core input to
/// [`FidHasher::hash`]. This is the byte-exact projection of what
/// `MessageDigestFidHasher.hash` reads off a Ghidra `Instruction` — assembled (in
/// PR4) from [`kuna_sleigh::sleigh::InsnMask`] + the Listing's flow class + the
/// instruction's address range, but defined here so the hashing core is unit-
/// testable independent of the Listing/SLEIGH.
#[derive(Debug, Clone)]
pub struct InsnFingerprint {
    /// The instruction's start VMA (`getMinAddress`).
    pub min_addr: u64,
    /// The instruction's end VMA, inclusive (`getMaxAddress` = `min + len - 1`).
    pub max_addr: u64,
    /// The raw instruction bytes (`getBytes`), length `len`.
    pub bytes: Vec<u8>,
    /// `instructMask` — `fixed_mask[i]` pins the fixed opcode/encoding bits;
    /// `applyMask` ANDs each byte with this. Length must equal `bytes.len()`.
    pub fixed_mask: Vec<u8>,
    /// `getFlowType().isCall()`.
    pub is_call: bool,
    /// The classified operands, in operand order (`getNumOperands`).
    pub operands: Vec<OperandFingerprint>,
}

/// One operand's contribution: its value-bit mask (for the relocation range
/// check) and the classified op-objects (`getOpObjects(ii)`).
#[derive(Debug, Clone)]
pub struct OperandFingerprint {
    /// `getOperandValueMask(ii).getBytes()` — the operand's value-bit mask, one
    /// byte per instruction byte. Used to find the operand's byte range for the
    /// relocation check (`hasRelocation`). An empty mask means
    /// `getOperandValueMask(ii) == null` ⇒ the operand is skipped entirely.
    pub value_mask: Vec<u8>,
    /// The resolved op-objects (`Scalar` / `Register` / `Address`).
    pub objects: Vec<OpObject>,
}

/// A classified operand object — the hashing core's mirror of
/// [`kuna_sleigh::sleigh::OpObject`], plus the Ghidra `OperandType` flags the
/// 4-way scalar rule reads (`OperandType.isScalar` / `OperandType.isAddress`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpObject {
    /// A scalar/immediate. `signed` is `scalar.getSignedValue()`;
    /// `whole_operand` is `OperandType.isScalar(operandType)` (the scalar is the
    /// whole operand, not a sub-field); `is_address` is
    /// `OperandType.isAddress(operandType)`.
    Scalar { signed: i64, whole_operand: bool, is_address: bool },
    /// A register at the given register-space offset (`reg.getOffset()`).
    Register { offset: u64 },
    /// A code/data address object (hashed as a value-suppressed placeholder).
    Address,
}

/// A relocation oracle: does any relocation fall inside `[min_addr, max_addr]`
/// (inclusive)? The pure hashing core consumes this as a trait so it is testable
/// without a real relocation table; PR4 supplies the `elf_reloc.rs`-backed
/// adapter. A target with no relocations (a statically-linked or `.o`-without-
/// `.rela` body) uses [`NoRelocations`].
pub trait RelocationQuery {
    /// `relocationTable.getRelocations(range).hasNext()` — is there any relocation
    /// in the inclusive address range `[min_addr, max_addr]`?
    fn has_relocation_in(&self, min_addr: u64, max_addr: u64) -> bool;
}

/// A relocation oracle that never reports a relocation (the common case for a
/// fully-resolved static body, and the default in tests).
#[derive(Debug, Clone, Copy, Default)]
pub struct NoRelocations;

impl RelocationQuery for NoRelocations {
    fn has_relocation_in(&self, _min_addr: u64, _max_addr: u64) -> bool {
        false
    }
}

/// `MessageDigestFidHasher.hasRelocation`: clip `[min, max]` to the operand's
/// non-zero mask-byte range, then ask the relocation oracle whether any
/// relocation lands in the clipped range.
///
/// The leading zero mask bytes advance `min` (`minAddress.addWrap(1)`); the
/// trailing zero mask bytes retreat `max` (`maxAddress.subtract(1)`). If after
/// clipping `min <= max`, query the range. This pins the relocation check to the
/// operand's *value* bytes, not the whole instruction.
fn has_relocation(
    value_mask: &[u8],
    mut min_addr: u64,
    mut max_addr: u64,
    relocs: &dyn RelocationQuery,
) -> bool {
    // Advance min past leading zero mask bytes (`addWrap` — wrapping add).
    for &b in value_mask {
        if b != 0 {
            break;
        }
        min_addr = min_addr.wrapping_add(1);
    }
    // Retreat max past trailing zero mask bytes (`subtract`).
    for &b in value_mask.iter().rev() {
        if b != 0 {
            break;
        }
        max_addr = max_addr.wrapping_sub(1);
    }
    // Ghidra compares `minAddress.compareTo(maxAddress) <= 0` (signed within the
    // space, but FID addresses are non-negative offsets — unsigned `<=` matches).
    if min_addr <= max_addr {
        return relocs.has_relocation_in(min_addr, max_addr);
    }
    false
}

/// The FID hasher: the operand-masked FNV-1a64 fingerprinter. Holds the two
/// digests (reused across calls, reset at the start of each `hash`) and the
/// per-architecture instruction skippers.
///
/// `kuna-analysis` wiring (PR4) builds this with the x86 skipper for x86 targets
/// and an empty skipper set elsewhere (non-x86 never skips). The pure core only
/// needs the skipper set + an extent of [`InsnFingerprint`]s.
pub struct FidHasher {
    /// `MessageDigestFidHasher.shortCodeUnitLimit`.
    short_code_unit_limit: i32,
    full_digest: Fnv1a64,
    specific_digest: Fnv1a64,
    skippers: Vec<Box<dyn InstructionSkipper>>,
}

impl FidHasher {
    /// Build a hasher with the given skipper set and the standard
    /// [`SHORT_HASH_CODE_UNIT_LENGTH`] limit.
    pub fn new(skippers: Vec<Box<dyn InstructionSkipper>>) -> Self {
        Self {
            short_code_unit_limit: SHORT_HASH_CODE_UNIT_LENGTH,
            full_digest: Fnv1a64::new(),
            specific_digest: Fnv1a64::new(),
            skippers,
        }
    }

    /// `MessageDigestFidHasher.hash(Function func)` — port the per-instruction
    /// loop **line-for-line**.
    ///
    /// `extent` is the function body's code units in ascending address order
    /// (`FunctionExtentGenerator.calculateExtent`; see [`crate::fid::extent`]).
    /// `relocs` is the relocation oracle for the 4-way scalar rule. Returns `None`
    /// when the extent has fewer than `short_code_unit_limit` usable code units.
    pub fn hash(
        &mut self,
        extent: &[InsnFingerprint],
        relocs: &dyn RelocationQuery,
    ) -> Option<FidHashQuad> {
        // `if (extent.size() < shortCodeUnitLimit) return null;`
        if (extent.len() as i32) < self.short_code_unit_limit {
            return None;
        }

        self.full_digest.reset();
        self.specific_digest.reset();

        let mut specific_count: i32 = 0;
        let mut call_count: i32 = 0;
        let mut code_unit_index: i32 = -1;

        for insn in extent {
            // `++codeUnitIndex;`
            code_unit_index += 1;
            // `if (codeUnitIndex >= Short.MAX_VALUE - 1) break;`
            if code_unit_index >= SHORT_MAX_VALUE - 1 {
                break;
            }

            let min_address = insn.min_addr;
            let max_address = insn.max_addr;
            // `int amountToFetch = codeUnit.getLength();`  /  `actualNumberRead`.
            // The fingerprint already carries exactly `len` bytes.
            let mut buffer = insn.bytes.clone();
            let actual_number_read = buffer.len();

            // --- Skippers (BEFORE the call count / operand hashing). On a skip,
            // un-count this code unit and continue (which also skips the final
            // masked-byte update for this instruction).
            let mut skip = false;
            for skipper in &self.skippers {
                if skipper.should_skip(&buffer, actual_number_read) {
                    skip = true;
                    // `codeUnitIndex -= 1;  // Don't count this in the score`
                    code_unit_index -= 1;
                    break;
                }
            }
            if skip {
                continue;
            }

            // `if (instruction.getFlowType().isCall()) callCount += 1;`
            if insn.is_call {
                call_count += 1;
            }

            // --- Operand sub-hashing: the specific hash folds in usable operand
            // scalars; the full hash masks every operand value out.
            for (ii, operand) in insn.operands.iter().enumerate() {
                // `Mask operandMask = prototype.getOperandValueMask(ii);`
                // `if (operandMask == null) continue;`  (empty mask == null)
                if operand.value_mask.is_empty() {
                    continue;
                }
                // `int specificUpdate = (ii + 1) * 7777;`
                // `int fullUpdate = specificUpdate;`
                let ii1 = (ii as i32).wrapping_add(1);
                let mut specific_update: i32 = ii1.wrapping_mul(7777);
                let mut full_update: i32 = specific_update;

                for obj in &operand.objects {
                    match *obj {
                        OpObject::Scalar { signed, whole_operand, is_address } => {
                            // `long val = scalar.getSignedValue();`
                            let mut val: i64 = signed;
                            // 4-way rule, in order:
                            if has_relocation(
                                &operand.value_mask,
                                min_address,
                                max_address,
                                relocs,
                            ) {
                                // (1) relocation-bound → placeholder.
                                val = i64::from(FEEDDEAD);
                            } else if whole_operand {
                                // `OperandType.isScalar(operandType)`
                                if is_address {
                                    // (2) whole-operand address → placeholder.
                                    val = i64::from(FEEDDEAD);
                                } else {
                                    // (4a) small/usable whole-operand scalar.
                                    specific_count += 1;
                                }
                            } else if val >= 256 || val <= -256 {
                                // (3) large non-whole-operand scalar → placeholder.
                                val = i64::from(FEEDDEAD);
                            } else {
                                // (4b) small/usable non-whole-operand scalar.
                                specific_count += 1;
                            }
                            // `specificUpdate = specificUpdate + ((int) val + 1234567) * 67999;`
                            // The `(int) val` truncation of the 64-bit value is
                            // observable — reproduce it (`as i32`).
                            let mixed = (val as i32)
                                .wrapping_add(1234567)
                                .wrapping_mul(67999);
                            specific_update = specific_update.wrapping_add(mixed);
                            // `fullUpdate += 0xfeeddead;`  (the full hash never
                            // sees a scalar value).
                            full_update = full_update.wrapping_add(FEEDDEAD);
                        }
                        OpObject::Register { offset } => {
                            // `int val = reg.getOffset();` — Java `int`, so the
                            // 64-bit register offset truncates to 32 bits.
                            // `val = (val + 7654321) * 98777;`
                            let val = (offset as i32)
                                .wrapping_add(7654321)
                                .wrapping_mul(98777);
                            // Registers are structural — into BOTH hashes.
                            full_update = full_update.wrapping_add(val);
                            specific_update = specific_update.wrapping_add(val);
                        }
                        OpObject::Address => {
                            // Address object: placeholder value into both
                            // (the `obj instanceof Address` arm).
                            // `specificUpdate = specificUpdate + (0xfeeddead + 1234567) * 67999;`
                            let mixed = FEEDDEAD
                                .wrapping_add(1234567)
                                .wrapping_mul(67999);
                            specific_update = specific_update.wrapping_add(mixed);
                            // `fullUpdate += 0xfeeddead;`
                            full_update = full_update.wrapping_add(FEEDDEAD);
                        }
                    }
                }
                // `fullDigest.update(fullUpdate); specificDigest.update(specificUpdate);`
                self.full_digest.update_i32(full_update);
                self.specific_digest.update_i32(specific_update);
            }

            // --- Mask the opcode bytes: `instructMask.applyMask(buffer, buffer)`.
            // An empty/absent mask is Ghidra's NullPointerException arm: fill the
            // whole code unit with 0xa5 (it cannot give reliable mask info).
            if insn.fixed_mask.len() >= actual_number_read && !insn.fixed_mask.is_empty()
            {
                for i in 0..actual_number_read {
                    buffer[i] &= insn.fixed_mask[i];
                }
            } else {
                for byte in buffer.iter_mut().take(actual_number_read) {
                    *byte = 0xa5;
                }
            }

            // --- Feed the masked bytes to BOTH digests (AFTER the operand
            // updates above — the critical interleave invariant).
            self.full_digest.update_bytes(&buffer[..actual_number_read]);
            self.specific_digest.update_bytes(&buffer[..actual_number_read]);
        }

        // `++codeUnitIndex;` — now a length, not the last index.
        code_unit_index += 1;

        // `if (codeUnitIndex < shortCodeUnitLimit) return null;`
        if code_unit_index < self.short_code_unit_limit {
            return None;
        }

        // `short fullCount = (short) (codeUnitIndex - callCount);`
        let code_unit_size = (code_unit_index - call_count) as i16;
        // `byte additionalSpecificCount = (byte) Math.min(specificCount, Byte.MAX_VALUE);`
        let specific_addl_size = specific_count.min(BYTE_MAX_VALUE) as u8;

        Some(FidHashQuad {
            code_unit_size,
            full_hash: self.full_digest.digest_long(),
            specific_addl_size,
            specific_hash: self.specific_digest.digest_long(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- FNV-1a64 golden vectors (the canonical FNV test vectors) -------------
    //
    // The reference values are from the FNV-1a 64-bit test suite (Landon Curt
    // Noll's `test_fnv.c`). `fnv1a64(s)` runs the byte stream through `Fnv1a64`.
    fn fnv1a64(bytes: &[u8]) -> u64 {
        let mut d = Fnv1a64::new();
        d.update_bytes(bytes);
        d.digest_long()
    }

    #[test]
    fn fnv_offset_basis_is_empty_string() {
        // The empty input digests to the offset basis (no bytes mixed).
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
    }

    #[test]
    fn fnv_known_vectors() {
        // Canonical FNV-1a64 vectors.
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a64(b"b"), 0xaf63_df4c_8601_f1a5);
        assert_eq!(fnv1a64(b"c"), 0xaf63_de4c_8601_eff2);
        assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
        assert_eq!(fnv1a64(b"hello"), 0xa430_d846_80aa_bd0b);
    }

    #[test]
    fn fnv_digest_long_resets_to_basis() {
        let mut d = Fnv1a64::new();
        d.update_bytes(b"hello");
        let first = d.digest_long();
        assert_eq!(first, 0xa430_d846_80aa_bd0b);
        // After digesting, the digest is back to the basis: a fresh "hello"
        // hashes identically.
        d.update_bytes(b"hello");
        assert_eq!(d.digest_long(), 0xa430_d846_80aa_bd0b);
    }

    #[test]
    fn update_i32_is_big_endian() {
        // update(int) feeds bytes >>24,>>16,>>8,>>0 — verify against the
        // equivalent explicit byte stream.
        let mut a = Fnv1a64::new();
        a.update_i32(0x0102_0304);
        let via_int = a.digest_long();

        let mut b = Fnv1a64::new();
        b.update_bytes(&[0x01, 0x02, 0x03, 0x04]);
        let via_bytes = b.digest_long();

        assert_eq!(via_int, via_bytes);
    }

    #[test]
    fn update_i32_negative_value() {
        // 0xfeeddead as a signed i32 is negative; the byte extraction reinterprets
        // as u32 (Java masks each shift with & 0xff). Big-endian: de ad ... wait,
        // 0xfeeddead → bytes fe ed de ad.
        let mut a = Fnv1a64::new();
        a.update_i32(FEEDDEAD);
        let via_int = a.digest_long();
        let mut b = Fnv1a64::new();
        b.update_bytes(&[0xfe, 0xed, 0xde, 0xad]);
        assert_eq!(via_int, b.digest_long());
    }

    // --- A hand-computed FidHashQuad on a tiny 4-instruction sequence ---------
    //
    // This drives the full `FidHasher::hash` over a synthetic extent and asserts
    // the exact quad against an independent reference computation (below). It is
    // the self-consistency / hand-computed fidelity gate; the Ghidra-validated
    // gate is a documented follow-up (see the increment log).

    /// An independent, straight-line reimplementation of the hasher loop used
    /// only to derive the expected quad in the test (a second pair of eyes on the
    /// algorithm — if both diverge from Ghidra they diverge together, but a
    /// transcription slip in the production `hash` shows up as a mismatch here).
    fn reference_quad(extent: &[InsnFingerprint]) -> Option<FidHashQuad> {
        if (extent.len() as i32) < SHORT_HASH_CODE_UNIT_LENGTH {
            return None;
        }
        let mut full = Fnv1a64::new();
        let mut spec = Fnv1a64::new();
        let mut specific_count: i32 = 0;
        let mut call_count: i32 = 0;
        let mut idx: i32 = -1;
        let no_reloc = NoRelocations;
        for insn in extent {
            idx += 1;
            let mut buf = insn.bytes.clone();
            let n = buf.len();
            if insn.is_call {
                call_count += 1;
            }
            for (ii, op) in insn.operands.iter().enumerate() {
                if op.value_mask.is_empty() {
                    continue;
                }
                let base = ((ii as i32) + 1).wrapping_mul(7777);
                let mut su = base;
                let mut fu = base;
                for obj in &op.objects {
                    match *obj {
                        OpObject::Scalar { signed, whole_operand, is_address } => {
                            let mut val = signed;
                            let _ = no_reloc;
                            if whole_operand {
                                if is_address {
                                    val = i64::from(FEEDDEAD);
                                } else {
                                    specific_count += 1;
                                }
                            } else if val >= 256 || val <= -256 {
                                val = i64::from(FEEDDEAD);
                            } else {
                                specific_count += 1;
                            }
                            su = su.wrapping_add(
                                (val as i32).wrapping_add(1234567).wrapping_mul(67999),
                            );
                            fu = fu.wrapping_add(FEEDDEAD);
                        }
                        OpObject::Register { offset } => {
                            let v = (offset as i32)
                                .wrapping_add(7654321)
                                .wrapping_mul(98777);
                            fu = fu.wrapping_add(v);
                            su = su.wrapping_add(v);
                        }
                        OpObject::Address => {
                            su = su.wrapping_add(
                                FEEDDEAD.wrapping_add(1234567).wrapping_mul(67999),
                            );
                            fu = fu.wrapping_add(FEEDDEAD);
                        }
                    }
                }
                full.update_i32(fu);
                spec.update_i32(su);
            }
            for i in 0..n {
                buf[i] &= insn.fixed_mask[i];
            }
            full.update_bytes(&buf[..n]);
            spec.update_bytes(&buf[..n]);
        }
        idx += 1;
        if idx < SHORT_HASH_CODE_UNIT_LENGTH {
            return None;
        }
        Some(FidHashQuad {
            code_unit_size: (idx - call_count) as i16,
            full_hash: full.digest_long(),
            specific_addl_size: specific_count.min(BYTE_MAX_VALUE) as u8,
            specific_hash: spec.digest_long(),
        })
    }

    /// A small but representative 5-instruction synthetic extent exercising:
    /// a register operand, a small usable scalar, a large suppressed scalar,
    /// a whole-operand address, and a plain no-operand instruction.
    fn sample_extent() -> Vec<InsnFingerprint> {
        let insn = |min: u64,
                    bytes: &[u8],
                    mask: &[u8],
                    is_call: bool,
                    operands: Vec<OperandFingerprint>| {
            InsnFingerprint {
                min_addr: min,
                max_addr: min + bytes.len() as u64 - 1,
                bytes: bytes.to_vec(),
                fixed_mask: mask.to_vec(),
                is_call,
                operands,
            }
        };
        vec![
            // push rbp  (no operand value mask)
            insn(0x1000, &[0x55], &[0xff], false, vec![]),
            // mov rbp, rsp  (register operands)
            insn(
                0x1001,
                &[0x48, 0x89, 0xe5],
                &[0xff, 0xff, 0xc0],
                false,
                vec![
                    OperandFingerprint {
                        value_mask: vec![0x00, 0x00, 0x38],
                        objects: vec![OpObject::Register { offset: 0x20 }],
                    },
                    OperandFingerprint {
                        value_mask: vec![0x00, 0x00, 0x07],
                        objects: vec![OpObject::Register { offset: 0x28 }],
                    },
                ],
            ),
            // add eax, 0x10  (small usable scalar)
            insn(
                0x1004,
                &[0x83, 0xc0, 0x10],
                &[0xff, 0xff, 0x00],
                false,
                vec![OperandFingerprint {
                    value_mask: vec![0x00, 0x00, 0xff],
                    objects: vec![OpObject::Scalar {
                        signed: 0x10,
                        whole_operand: false,
                        is_address: false,
                    }],
                }],
            ),
            // mov eax, 0x12345678  (large suppressed scalar — whole operand, but
            // not an address, so it counts; value still folded as-is since whole
            // operand path keeps the value)
            insn(
                0x1007,
                &[0xb8, 0x78, 0x56, 0x34, 0x12],
                &[0xff, 0x00, 0x00, 0x00, 0x00],
                false,
                vec![OperandFingerprint {
                    value_mask: vec![0x00, 0xff, 0xff, 0xff, 0xff],
                    objects: vec![OpObject::Scalar {
                        signed: 0x1234_5678,
                        whole_operand: true,
                        is_address: false,
                    }],
                }],
            ),
            // call rel32 (whole-operand address scalar → suppressed; is_call)
            insn(
                0x100c,
                &[0xe8, 0x00, 0x00, 0x00, 0x00],
                &[0xff, 0x00, 0x00, 0x00, 0x00],
                true,
                vec![OperandFingerprint {
                    value_mask: vec![0x00, 0xff, 0xff, 0xff, 0xff],
                    objects: vec![OpObject::Scalar {
                        signed: 0x0,
                        whole_operand: true,
                        is_address: true,
                    }],
                }],
            ),
        ]
    }

    #[test]
    fn hand_computed_quad_matches_reference() {
        let extent = sample_extent();
        let expected = reference_quad(&extent).expect("extent >= 4 code units");
        let mut hasher = FidHasher::new(vec![]);
        let got = hasher.hash(&extent, &NoRelocations).expect("hashable");
        assert_eq!(got, expected, "production hash diverged from reference");
    }

    #[test]
    fn quad_field_values_are_as_expected() {
        // 5 code units, 1 call → code_unit_size = 5 - 1 = 4.
        // Usable scalars: the small `0x10` (counts) and the whole-operand
        // `0x12345678` (counts) → specific_count = 2; the call's address scalar
        // is suppressed (whole-operand address) → does not count.
        let extent = sample_extent();
        let mut hasher = FidHasher::new(vec![]);
        let q = hasher.hash(&extent, &NoRelocations).unwrap();
        assert_eq!(q.code_unit_size, 4);
        assert_eq!(q.specific_addl_size, 2);
        // Full and specific hashes differ (scalars folded only into specific).
        assert_ne!(q.full_hash, q.specific_hash);
    }

    #[test]
    fn hash_is_deterministic_across_runs() {
        // Self-consistency: the same extent hashes identically twice (the digest
        // reset between runs is correct).
        let extent = sample_extent();
        let mut hasher = FidHasher::new(vec![]);
        let q1 = hasher.hash(&extent, &NoRelocations).unwrap();
        let q2 = hasher.hash(&extent, &NoRelocations).unwrap();
        assert_eq!(q1, q2);
        // And a fresh hasher agrees.
        let mut hasher2 = FidHasher::new(vec![]);
        let q3 = hasher2.hash(&extent, &NoRelocations).unwrap();
        assert_eq!(q1, q3);
    }

    #[test]
    fn extent_below_four_returns_none() {
        let extent = sample_extent();
        let short = &extent[..3];
        let mut hasher = FidHasher::new(vec![]);
        assert!(hasher.hash(short, &NoRelocations).is_none());
    }

    #[test]
    fn relocation_suppresses_scalar_in_specific_count() {
        // A scalar that would otherwise count is suppressed when a relocation
        // covers its bytes → specific_count drops, and the specific hash changes.
        struct AlwaysReloc;
        impl RelocationQuery for AlwaysReloc {
            fn has_relocation_in(&self, _min: u64, _max: u64) -> bool {
                true
            }
        }
        // A single instruction with a small whole-operand scalar, repeated to
        // reach >= 4 code units (distinct addresses so the loop is well-formed).
        let mk = |a: u64| InsnFingerprint {
            min_addr: a,
            max_addr: a + 3,
            bytes: vec![0xb8, 0x10, 0x00, 0x00],
            fixed_mask: vec![0xff, 0x00, 0x00, 0x00],
            is_call: false,
            operands: vec![OperandFingerprint {
                value_mask: vec![0x00, 0xff, 0xff, 0xff],
                objects: vec![OpObject::Scalar {
                    signed: 0x10,
                    whole_operand: true,
                    is_address: false,
                }],
            }],
        };
        let extent: Vec<_> = (0..4).map(|i| mk(0x2000 + i * 4)).collect();

        let mut h1 = FidHasher::new(vec![]);
        let no_reloc = h1.hash(&extent, &NoRelocations).unwrap();
        let mut h2 = FidHasher::new(vec![]);
        let with_reloc = h2.hash(&extent, &AlwaysReloc).unwrap();

        // Without relocations all four scalars count; with relocations none do.
        assert_eq!(no_reloc.specific_addl_size, 4);
        assert_eq!(with_reloc.specific_addl_size, 0);
        // Suppressing the value changes the specific hash; the full hash (never
        // sees the value) is identical either way.
        assert_ne!(no_reloc.specific_hash, with_reloc.specific_hash);
        assert_eq!(no_reloc.full_hash, with_reloc.full_hash);
    }
}
