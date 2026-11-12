//! FLIRT .sig candidate lookup. References are reported but not resolved.
use std::io::Read;

use lancelot_flirt::{FlirtSignature, SigElement, Symbol};
use object::{Object, ObjectSection, SectionKind};
use serde_json::{json, Value};

const MAX_SIG: usize = 64 * 1024 * 1024;
const MAX_WINDOW: usize = 1024 * 1024;

pub(super) fn query(
    binary: &str,
    database: &str,
    input: &[u8],
    addresses: &[u64],
    file: &object::File<'_>,
) -> Result<Value, String> {
    let sig = unpack(input)?;
    let signatures = std::panic::catch_unwind(|| lancelot_flirt::sig::parse(&sig))
        .map_err(|_| "invalid or unsupported FLIRT signature".to_string())?
        .map_err(|e| format!("invalid or unsupported FLIRT signature: {e}"))?;
    let results = addresses
        .iter()
        .map(|&address| query_one(binary, database, address, file, &signatures))
        .collect();
    super::query::package_results(binary, database, addresses, results)
}

fn query_one(
    binary: &str,
    database: &str,
    address: u64,
    file: &object::File<'_>,
    signatures: &[FlirtSignature],
) -> Result<Value, String> {
    let mut window = None;
    let mut window_limited = false;
    for section in file.sections() {
        if section.kind() != SectionKind::Text || address < section.address() {
            continue;
        }
        let offset = address - section.address();
        let Ok(offset) = usize::try_from(offset) else {
            continue;
        };
        let data = section
            .data()
            .map_err(|e| format!("cannot read executable section: {e}"))?;
        if let Some(tail) = data.get(offset..) {
            window_limited = tail.len() > MAX_WINDOW;
            window = Some(&tail[..tail.len().min(MAX_WINDOW)]);
            break;
        }
    }
    let bytes = window.ok_or("address is not in a file-backed executable section")?;
    let mut candidates = Vec::new();
    for sig in signatures {
        if !matches(sig, bytes) {
            continue;
        }
        let refs: Vec<Value> = sig
            .names
            .iter()
            .filter_map(|symbol| match symbol {
                Symbol::Reference(n) => Some(json!({"offset": n.offset, "name": n.name})),
                _ => None,
            })
            .collect();
        for name in &sig.names {
            if let Symbol::Public(name) = name {
                if name.offset == 0 {
                    candidates.push(json!({"name": name.name, "reference_constraints": refs,
                        "reference_validation": "not_checked", "pattern_length": sig.byte_sig.0.len(),
                        "crc_length": sig.size_of_bytes_crc16, "module_size": sig.size_of_function}));
                }
            }
        }
    }
    candidates.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    let names: std::collections::BTreeSet<_> = candidates
        .iter()
        .filter_map(|c| c["name"].as_str())
        .collect();
    let status = if window_limited {
        "incomplete"
    } else {
        match names.len() {
            0 => "no_match",
            1 => "unique",
            _ => "ambiguous",
        }
    };
    Ok(
        json!({"binary": binary, "database": database, "database_format": "flirt_sig",
        "address": format!("0x{address:x}"), "status": status,
        "match_basis": "pattern_crc_tail", "architecture_validation": "not_checked",
        "reference_validation": "not_checked", "byte_window_limited": window_limited,
        "candidates": candidates}),
    )
}

fn matches(sig: &FlirtSignature, bytes: &[u8]) -> bool {
    if bytes.len() < sig.byte_sig.0.len() {
        return false;
    }
    sig.byte_sig
        .0
        .iter()
        .zip(bytes)
        .all(|(expected, byte)| match expected {
            SigElement::Byte(value) => value == byte,
            SigElement::Wildcard => true,
        })
        && sig.match_crc16(bytes)
        && sig.match_tail_bytes(bytes)
        && sig.match_footer(bytes)
}

fn unpack(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() > MAX_SIG {
        return Err("FLIRT signature exceeds size limit".into());
    }
    if input.len() < 37 || &input[..6] != b"IDASGN" {
        return Err("invalid FLIRT header".into());
    }
    let extra = match input[6] {
        5 => 0,
        6 | 7 => 4,
        8 | 9 => 6,
        10 => 8,
        _ => return Err("unsupported FLIRT version".into()),
    };
    let header_len = 37 + extra + input[34] as usize;
    if input.len() < header_len {
        return Err("truncated FLIRT header".into());
    }
    if input[16] & 0x10 == 0 {
        return Ok(input.to_vec());
    }
    let mut decoder =
        flate2::read::ZlibDecoder::new(&input[header_len..]).take((MAX_SIG + 1) as u64);
    let mut bytes = Vec::new();
    decoder
        .read_to_end(&mut bytes)
        .map_err(|e| format!("invalid compressed FLIRT signature: {e}"))?;
    if bytes.len() > MAX_SIG {
        return Err("decompressed FLIRT signature exceeds size limit".into());
    }
    let mut output = input[..header_len].to_vec();
    output[16] &= !0x10;
    output.extend(bytes);
    Ok(output)
}
