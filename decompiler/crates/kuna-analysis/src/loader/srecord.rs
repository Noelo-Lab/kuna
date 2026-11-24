//! Wire validation for Motorola S-record streams.
//!
//! Addresses are left in producer units; this layer does not construct an image.

use kuna_base::error::{KunaError, KunaResult};

/// A validated record, in producer address units. S0 and S5/S6 records are
/// validated but omitted from the stream.
#[derive(Debug, Eq, PartialEq)]
pub enum Record<'a> {
    /// A payload at the given unscaled address and the encoded address width
    /// in bytes (2, 3, or 4).
    Data {
        address: u32,
        bytes: &'a [u8],
        address_size: u8,
    },
    /// A stream terminator with an entry address.
    End { address: u32, address_size: u8 },
}

fn error(line: usize, reason: &str) -> KunaError {
    KunaError::lowlevel(format!("Motorola S-record line {line}: {reason}"))
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn hex_byte(pair: &[u8]) -> Result<u8, &'static str> {
    let hi = hex_nibble(pair[0]).ok_or("invalid hexadecimal digit")?;
    let lo = hex_nibble(pair[1]).ok_or("invalid hexadecimal digit")?;
    Ok(hi << 4 | lo)
}

fn decode<'a>(
    text: &[u8],
    scratch: &'a mut [u8; 256],
) -> Result<&'a [u8], &'static str> {
    let count = hex_byte(text.get(..2).ok_or("truncated record")?)?;
    let size = usize::from(count) + 1;
    if text.len() < size * 2 {
        return Err("truncated record");
    }
    if text.len() != size * 2 {
        return Err("record byte count does not match its length");
    }
    scratch[0] = count;
    for (out, pair) in scratch[1..size].iter_mut().zip(text[2..].chunks_exact(2)) {
        *out = hex_byte(pair)?;
    }
    Ok(&scratch[..size])
}

/// Recognize the first nonwhitespace prefix, without validating the file.
pub fn recognizes(bytes: &[u8]) -> bool {
    matches!(bytes.trim_ascii_start(), [b'S', b'0'..=b'9', ..])
}

/// Decode and validate records up to EOF. Payloads borrow reusable scratch
/// storage and are only live during the callback. A late error does not
/// roll back earlier callbacks.
pub fn read(
    bytes: &[u8],
    mut consume: impl FnMut(usize, Record<'_>) -> KunaResult<()>,
) -> KunaResult<()> {
    let mut scratch = [0; 256];
    let mut state = State::default();
    let mut ended = false;
    let mut last_line = 0;
    for (index, line) in bytes.split(|&byte| byte == b'\n').enumerate() {
        last_line = index + 1;
        let line = line.trim_ascii();
        if line.is_empty() {
            continue;
        }
        if ended {
            return Err(error(last_line, "record after EOF"));
        }
        if let Some(record) = state.record(line, &mut scratch).map_err(|err| error(last_line, err))? {
            ended = matches!(record, Record::End { .. });
            consume(last_line, record)?;
        }
    }
    if !ended {
        return Err(error(last_line, "missing EOF record"));
    }
    Ok(())
}

#[derive(Default)]
struct State {
    data_count: usize,
    saw_header: bool,
    saw_count: bool,
}

impl State {
    fn record<'a>(
        &mut self,
        line: &[u8],
        scratch: &'a mut [u8; 256],
    ) -> Result<Option<Record<'a>>, &'static str> {
        let [b'S', kind, text @ ..] = line else {
            return Err("expected S-record prefix");
        };
        let record = decode(text, scratch)?;
        let kind = *kind;
        let width = match kind {
            b'0' | b'1' | b'5' | b'9' => 2,
            b'2' | b'6' | b'8' => 3,
            b'3' | b'7' => 4,
            _ => return Err("unsupported S-record type"),
        };
        if record.len() < width + 2 {
            return Err("truncated S-record address");
        }
        if record.iter().copied().fold(0u8, u8::wrapping_add) != 0xff {
            return Err("checksum mismatch");
        }
        let address = record[1..1 + width]
            .iter()
            .fold(0u32, |value, &byte| (value << 8) | u32::from(byte));
        let bytes = &record[1 + width..record.len() - 1];
        match (kind, bytes) {
            (b'0', _) => {
                if address != 0 || self.saw_header || self.data_count != 0 || self.saw_count {
                    return Err("invalid S0 header");
                }
                self.saw_header = true;
                Ok(None)
            }
            (b'1'..=b'3', _) => {
                if self.saw_count {
                    return Err("data record after record count");
                }
                self.data_count += 1;
                Ok(Some(Record::Data {
                    address,
                    bytes,
                    address_size: width as u8,
                }))
            }
            (b'5' | b'6', []) => {
                if self.saw_count || u64::from(address) != self.data_count as u64 {
                    return Err("invalid S-record data count");
                }
                self.saw_count = true;
                Ok(None)
            }
            (b'7'..=b'9', []) => Ok(Some(Record::End { address, address_size: width as u8 })),
            _ => Err("malformed S-record control record"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srec_retains_four_byte_data_and_entry_addresses_and_validates_count() {
        let input = b"S0040000619A\nS3071020304009034C\nS5030001FB\nS705102030405A\n";
        let mut records = Vec::new();
        read(input, |_, record| {
            match record {
                Record::Data { address, bytes, address_size: 4 } => {
                    assert_eq!(bytes, [9, 3]);
                    records.push(address);
                }
                Record::End { address, address_size: 4 } => records.push(address),
                record => panic!("unexpected {record:?}"),
            }
            Ok(())
        }).unwrap();
        assert_eq!(records, [0x10203040, 0x10203040]);
        let wrong_count = b"S3071020304009034C\nS5030002FA\nS705102030405A\n";
        assert!(read(wrong_count, |_, _| Ok(())).unwrap_err()
            .explain().contains("invalid S-record data count"));
    }

    #[test]
    fn rejects_corruption_and_trailing_records() {
        let cases: &[(&[u8], &str)] = &[
            (b"S3071020304009034D\nS705102030405A", "checksum mismatch"),
            (b"S3071020304009034C", "missing EOF"),
            (b"S705102030405A\nS3071020304009034C", "record after EOF"),
        ];
        for &(input, message) in cases {
            let error = read(input, |_, _| Ok(())).unwrap_err();
            assert!(error.explain().contains(message), "{}", error.explain());
        }
    }
}
