//! (kuna `swiftdemangle`) Swift's Punycode variant (RFC 3492 with `_` as
//! the delimiter and `A`–`J` standing in for the digits `0`–`9`).

const BASE: u32 = 36;
const TMIN: u32 = 1;
const TMAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 128;

fn digit(c: u8) -> Option<u32> {
    match c {
        b'a'..=b'z' => Some((c - b'a') as u32),
        b'A'..=b'J' => Some((c - b'A') as u32 + 26),
        _ => None,
    }
}

fn adapt(mut delta: u32, num_points: u32, first_time: bool) -> u32 {
    delta /= if first_time { DAMP } else { 2 };
    delta += delta / num_points;
    let mut k = 0;
    while delta > ((BASE - TMIN) * TMAX) / 2 {
        delta /= BASE - TMIN;
        k += BASE;
    }
    k + (((BASE - TMIN + 1) * delta) / (delta + SKEW))
}

/// Decode one Swift-punycoded identifier; `None` for malformed input or
/// a code point that is not a valid scalar.
pub fn decode_swift(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut out: Vec<char> = Vec::new();
    let delim = bytes.iter().rposition(|&b| b == b'_');
    let mut pos = 0;
    if let Some(d) = delim {
        for &b in &bytes[..d] {
            if !b.is_ascii() {
                return None;
            }
            out.push(b as char);
        }
        pos = d + 1;
    }
    let mut n = INITIAL_N;
    let mut i: u32 = 0;
    let mut bias = INITIAL_BIAS;
    while pos < bytes.len() {
        let old_i = i;
        let mut w: u32 = 1;
        let mut k = BASE;
        loop {
            let d = digit(*bytes.get(pos)?)?;
            pos += 1;
            i = i.checked_add(d.checked_mul(w)?)?;
            let t = if k <= bias {
                TMIN
            } else if k >= bias + TMAX {
                TMAX
            } else {
                k - bias
            };
            if d < t {
                break;
            }
            w = w.checked_mul(BASE - t)?;
            k += BASE;
        }
        let len = out.len() as u32 + 1;
        bias = adapt(i - old_i, len, old_i == 0);
        n = n.checked_add(i / len)?;
        i %= len;
        // Non-identifier ASCII characters travel in the (otherwise invalid)
        // surrogate range as `0xD800 + byte`.
        let c = match n {
            0xD800..=0xD87F => char::from_u32(n - 0xD800)?,
            _ => char::from_u32(n)?,
        };
        out.insert(i as usize, c);
        i += 1;
    }
    Some(out.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_the_abi_document_example() {
        assert_eq!(decode_swift("vergenza_JFa").as_deref(), Some("vergüenza"));
    }
}
