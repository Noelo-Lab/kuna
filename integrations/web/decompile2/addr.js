// addr.js — address helpers shared by the study view's renderers. Addresses
// travel as lowercase `0x…` strings and are compared/offset as BigInt, never as
// JS numbers (a 64-bit address does not fit a double).

/** Normalize a number, BigInt or hex string to lowercase `0x…`; null if unusable. */
export function addrHex(value) {
  try {
    if (typeof value === 'bigint') return '0x' + value.toString(16);
    if (typeof value === 'number') return Number.isSafeInteger(value) && value >= 0 ? '0x' + value.toString(16) : null;
    if (typeof value === 'string' && /^(0x)?[0-9a-f]+$/i.test(value.trim())) {
      const body = value.trim().replace(/^0x/i, '');
      return '0x' + BigInt('0x' + body).toString(16);
    }
  } catch (_) { /* fall through */ }
  return null;
}

/** `0x11ab` → 4523n (null stays null). */
export function addrBig(hex) {
  return hex == null ? null : BigInt(hex);
}

/** `0x11ab` → `11ab`. */
export function bare(hex) {
  return hex ? hex.replace(/^0x/, '') : '';
}

/** A signed offset as `+0x13` / `-0x4`. */
export function signedHex(delta) {
  const d = BigInt(delta);
  return (d < 0n ? '-0x' + (-d).toString(16) : '+0x' + d.toString(16));
}
