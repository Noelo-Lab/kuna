//! JSON responses retain the highest-ranked prefix within the wire limit.

use std::io::{self, Write};

use kuna_bsim_server::{QueryResponse, MAX_RESPONSE_BYTES};

pub(super) fn encode(response: &QueryResponse) -> Result<Vec<u8>, serde_json::Error> {
    encode_with_limit(response, MAX_RESPONSE_BYTES)
}

fn encode_with_limit(response: &QueryResponse, limit: usize) -> Result<Vec<u8>, serde_json::Error> {
    const FULL: &[u8] = b"],\"truncated\":false}";
    const PARTIAL: &[u8] = b"],\"truncated\":true}";

    let mut writer = Bounded {
        bytes: Vec::new(),
        limit: limit.saturating_sub(FULL.len()),
    };
    write!(
        writer,
        "{{\"considered\":{},\"incomplete_executables\":{},\"min_similarity\":",
        response.considered, response.incomplete_executables,
    ).map_err(serde_json::Error::io)?;
    serde_json::to_writer(&mut writer, &response.min_similarity)?;
    writer.write_all(b",\"matches\":[").map_err(serde_json::Error::io)?;

    let mut truncated = response.truncated;
    for (i, hit) in response.matches.iter().enumerate() {
        let start = writer.bytes.len();
        let result = (|| {
            if i != 0 {
                writer.write_all(b",").map_err(serde_json::Error::io)?;
            }
            serde_json::to_writer(&mut writer, hit)
        })();
        if let Err(error) = result {
            if error.io_error_kind() != Some(io::ErrorKind::FileTooLarge) {
                return Err(error);
            }
            writer.bytes.truncate(start);
            truncated = true;
            break;
        }
    }
    writer.bytes.extend_from_slice(if truncated { PARTIAL } else { FULL });
    Ok(writer.bytes)
}

struct Bounded {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for Bounded {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() > self.limit - self.bytes.len() {
            return Err(io::ErrorKind::FileTooLarge.into());
        }
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kuna_bsim_server::{Match, StoredFunction};

    use super::*;

    #[test]
    fn size_cap_keeps_the_ranked_prefix_even_for_escaped_names() {
        let hit = |address, name: &str| Match {
            function: StoredFunction {
                address,
                name: name.to_owned(),
                executable: "test".to_owned(),
                architecture: "x86:LE:64:default".to_owned(),
                compiler: "gcc".to_owned(),
                md5: "0".repeat(32),
                sha256: "1".repeat(64),
                macho_slice: None,
                complete: true,
            },
            similarity: 1.0,
            significance: 1.0,
        };
        let mut response = QueryResponse {
            matches: vec![hit(1, "top"), hit(2, &"\0".repeat(4096)), hit(3, "third")],
            considered: 4,
            incomplete_executables: 1,
            min_similarity: 0.7,
            truncated: false,
        };
        let limit = 8 * 1024;
        let wire = encode_with_limit(&response, limit).unwrap();
        assert!(wire.len() <= limit);
        let decoded: QueryResponse = serde_json::from_slice(&wire).unwrap();
        assert_eq!(decoded.matches, response.matches[..1]);
        assert!(decoded.truncated);
        assert_eq!(decoded.considered, response.considered);
        assert_eq!(decoded.incomplete_executables, response.incomplete_executables);
        let wire = encode(&response).unwrap();
        assert_eq!(serde_json::from_slice::<QueryResponse>(&wire).unwrap(), response);
        response.truncated = true;
        let wire = encode(&response).unwrap();
        assert_eq!(serde_json::from_slice::<QueryResponse>(&wire).unwrap(), response);
    }
}
