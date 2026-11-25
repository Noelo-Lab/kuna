//! Intel HEX framing, checksums, and extended-address state.
//!
//! Address units are the producer's. This reader does not infer a memory bank,
//! put bytes into an image, or assume how far a payload advances an address.

use kuna_base::error::{KunaError, KunaResult};

/// A validated Intel HEX record.
#[derive(Debug, Eq, PartialEq)]
pub enum Record<'a> {
    /// Data address with the most recent extension already applied.
    Data { address: u32, bytes: &'a [u8] },
    /// Type 02 (segment shifted left four) or type 04 (upper sixteen bits).
    AddressBase(u32),
    /// Type 03 start address, with CS and IP preserved separately.
    StartSegment { cs: u16, ip: u16 },
    /// Type 05 start linear address.
    StartLinear(u32),
    End,
}

/// Recognize the first nonwhitespace prefix without validating the records.
pub fn recognizes(bytes: &[u8]) -> bool {
    bytes.trim_ascii_start().starts_with(b":")
}

/// Decode and validate through EOF. Payloads borrow reusable scratch space and
/// are live only during the callback. On error, previous callbacks stand.
pub fn read(bytes: &[u8], mut consume: impl FnMut(usize, Record<'_>) -> KunaResult<()>) -> KunaResult<()> {
    let mut scratch = [0; 260]; // 255 data bytes + the five record fields
    let mut base = 0;
    let mut started = false;
    let mut ended = false;
    let mut last_line = 0;
    for (index, line) in bytes.split(|&byte| byte == b'\n').enumerate() {
        last_line = index + 1;
        let line = line.trim_ascii();
        if line.is_empty() {
            continue;
        }
        let error = |reason| error(last_line, reason);
        if ended {
            return Err(error("record after EOF"));
        }
        let (kind, offset, payload) = decode(line, &mut scratch).map_err(error)?;
        let record = match (kind, offset, payload) {
            (0, _, _) => Record::Data { address: base + u32::from(offset), bytes: payload },
            (1, 0, []) => {
                ended = true;
                Record::End
            }
            (2 | 4, 0, &[hi, lo]) => {
                base = u32::from(u16::from_be_bytes([hi, lo])) << if kind == 2 { 4 } else { 16 };
                Record::AddressBase(base)
            }
            (3 | 5, 0, &[a, b, c, d]) => {
                if started {
                    return Err(error("duplicate start address record"));
                }
                started = true;
                if kind == 3 {
                    Record::StartSegment { cs: u16::from_be_bytes([a, b]), ip: u16::from_be_bytes([c, d]) }
                } else {
                    Record::StartLinear(u32::from_be_bytes([a, b, c, d]))
                }
            }
            (1..=5, _, _) => return Err(error("malformed control record")),
            _ => return Err(error("unsupported record type")),
        };
        consume(last_line, record)?;
    }
    if !ended {
        return Err(error(last_line, "missing EOF record"));
    }
    Ok(())
}

fn error(line: usize, reason: &str) -> KunaError {
    KunaError::lowlevel(format!("Intel HEX line {line}: {reason}"))
}

fn hex_pair(pair: &[u8]) -> Result<u8, &'static str> {
    let digit = |byte| match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err("invalid hexadecimal digit"),
    };
    Ok(digit(pair[0])? << 4 | digit(pair[1])?)
}

fn decode<'a>(line: &[u8], scratch: &'a mut [u8; 260]) -> Result<(u8, u16, &'a [u8]), &'static str> {
    let encoded = line.strip_prefix(b":").ok_or("expected ':' record prefix")?;
    let count = hex_pair(encoded.get(..2).ok_or("truncated record")?)?;
    let size = usize::from(count) + 5;
    if encoded.len() != size * 2 {
        return Err("record byte count does not match its length");
    }
    let record = &mut scratch[..size];
    record[0] = count;
    for (out, pair) in record[1..].iter_mut().zip(encoded[2..].chunks_exact(2)) {
        *out = hex_pair(pair)?;
    }
    if record.iter().copied().fold(0u8, u8::wrapping_add) != 0 {
        return Err("checksum mismatch");
    }
    Ok((record[3], u16::from_be_bytes([record[1], record[2]]), &record[4..size - 1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_extensions_do_not_depend_on_payload_width() {
        let input = b":020000040001F9\r\n:06FFFE0011223344556698\r\n:0400000500020000F5\r\n:00000001FF\r\n";
        let mut addresses = Vec::new();
        read(input, |_, record| {
            match record {
                Record::AddressBase(address) | Record::StartLinear(address) => addresses.push(address),
                Record::Data { address, bytes } => {
                    assert_eq!(bytes, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
                    addresses.push(address);
                }
                Record::End => {}
                record => panic!("unexpected {record:?}"),
            }
            Ok(())
        }).unwrap();
        assert_eq!(addresses, [0x10000, 0x1fffe, 0x20000]);
    }

    #[test]
    fn keeps_segment_address_fields_and_replaces_the_base() {
        let input = b":020000040001F9\n:020000021234B6\n:0100020055A8\n:0400000312345678E5\n:00000001FF\n";
        let mut addresses = Vec::new();
        read(input, |_, record| {
            match record {
                Record::AddressBase(base) => addresses.push(base),
                Record::Data { address, bytes } => {
                    assert_eq!(bytes, [0x55]);
                    addresses.push(address);
                }
                Record::StartSegment { cs, ip } => {
                    addresses.extend([u32::from(cs), u32::from(ip)]);
                }
                record => assert_eq!(record, Record::End),
            }
            Ok(())
        }).unwrap();
        assert_eq!(addresses, [0x10000, 0x12340, 0x12342, 0x1234, 0x5678]);
    }

    #[test]
    fn rejects_corruption_and_trailing_records() {
        let cases: &[(&[u8], &str)] = &[
            (b":0400000500020000F4\n:00000001FF", "checksum mismatch"),
            (b":020000040001F9", "missing EOF"),
            (b":00000001FF\n:020000040001F9", "record after EOF"),
        ];
        for &(input, message) in cases {
            let err = read(input, |_, _| Ok(())).unwrap_err();
            assert!(err.explain().contains(message), "{}", err.explain());
        }
    }
}
