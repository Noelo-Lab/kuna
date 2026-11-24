//! Compare canonical Carmel fetches against expected text and issue boundaries.

use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::PathBuf,
};

use kuna_console::{
    disasm::{Row, hex},
    engine::{Bootstrap, ConsoleProgram, language_database, reuse_specs},
};
use kuna_decomp::kuna_loadoptions::LoadOptions;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const TARGET: &str = "Carmel:BE:24:default:default";
const SLOT: usize = 3;
const FETCH: usize = 2 * SLOT;
const MAX_WORD: u64 = 0xff_ffff;
const CROSSVIEW_SHA256: &str = "33f5b2a7c64d829f1d5244c675777ccda18c37add46941c3aea3f8a127971a3c";
const STATUSES: [&str; 5] = ["match", "mismatch", "native-error", "python-error", "both-error"];
const COMPARISONS: [&str; 4] = ["match", "mismatch", "reference-error", "kuna-error"];

fn packet_bytes(hex: &str) -> Option<[u8; FETCH]> {
    if hex.len() != 2 * FETCH || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let mut bytes = [0; FETCH];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).ok()?;
    }
    Some(bytes)
}

fn word(value: &Value) -> Option<u64> {
    value.as_u64().filter(|&n| n <= MAX_WORD)
}

/// A reference failure is distinct from malformed corpus data. In particular,
/// unknown text is a successful observation and is compared verbatim.
fn reference(value: &Value) -> Result<Option<&[Value]>, &'static str> {
    match (value.get("rows"), value.get("error")) {
        (Some(rows), None) => {
            let rows = rows.as_array().ok_or("rows must be an array")?;
            if !(1..=2).contains(&rows.len()) {
                return Err("expected one or two rows per fetch");
            }
            for row in rows {
                if row.as_object().is_none_or(|obj| obj.len() != 3)
                    || word(&row["address"]).is_none()
                    || !row["text"].is_string()
                    || row["slots"].as_array().is_none_or(|slots| {
                        !(1..=2).contains(&slots.len()) || slots.iter().any(|slot| word(slot).is_none())
                    })
                {
                    return Err("invalid row: expected a 24-bit address, 24-bit slots, and text");
                }
            }
            Ok(Some(rows))
        }
        (None, Some(error)) if error["type"].is_string() && error["message"].is_string() => Ok(None),
        _ => Err("expected either rows or an error with string type and message"),
    }
}

fn disassemble(prog: &ConsoleProgram, pc: u64) -> Result<Vec<Value>, String> {
    let mut addr = prog.input_code_offset(pc).map_err(|err| err.to_string())?;
    if addr % SLOT as u64 != 0 {
        return Err(format!("CODE word {pc:#08x} maps to unaligned byte offset {addr:#x}"));
    }
    let end = addr + FETCH as u64;
    let mut rows = Vec::with_capacity(2);
    while addr < end {
        let address = addr / SLOT as u64;
        let (length, mnemonic, operands) = prog.disassemble_at(addr)
            .map_err(|err| format!("CODE word {address:#08x}: {err}"))?;
        if !matches!(length, 3 | 6) || addr + length as u64 > end {
            return Err(format!("CODE word {address:#08x}: consumed {length} bytes outside fetch framing"));
        }
        let bytes = prog.read_bytes(addr, length as usize)
            .ok_or_else(|| format!("CODE word {address:#08x}: could not read {length} consumed bytes"))?;
        let row = Row { addr, size: length as u64, bytes, mnemonic, operands };
        let slots: Vec<_> = row.bytes.chunks_exact(SLOT)
            .map(|s| u32::from_be_bytes([0, s[0], s[1], s[2]]))
            .collect();
        // The standard listing text joins mnemonic and operands with one
        // separator. Do not adjust punctuation, whitespace, case or numbers.
        rows.push(json!({"address": address, "slots": slots, "text": row.text()}));
        addr += length as u64;
    }
    Ok(rows)
}

fn compare(reference: Option<&[Value]>, kuna: Option<&[Value]>) -> &'static str {
    match (reference, kuna) {
        (None, _) => "reference-error",
        (Some(_), None) => "kuna-error",
        (Some(expected), Some(actual)) if expected == actual => "match",
        _ => "mismatch",
    }
}

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

#[test]
#[ignore = "requires an input JSONL file in KUNA_CARMEL_DISASM_CORPUS"]
fn sweep() {
    let input = PathBuf::from(std::env::var_os("KUNA_CARMEL_DISASM_CORPUS")
        .expect("set KUNA_CARMEL_DISASM_CORPUS to a Carmel JSONL input file"));
    let mut reader = BufReader::new(File::open(&input)
        .unwrap_or_else(|err| panic!("Carmel corpus {}: {err}", input.display())));

    reuse_specs();
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let languages = language_database(&[specs]).expect("scan Kuna language specifications");
    let options = LoadOptions::default();
    let load = Bootstrap::new(&languages).target(TARGET).options(&options);
    let path = super::common::scratch_file("carmel-disasm-parity", "bin");
    let mut open = OpenOptions::new();
    open.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        open.mode(0o600);
    }
    let mut writer = open.open(&path).expect("create private raw fetch scratch file");
    let scratch = Scratch(path);
    let path = scratch.0.to_str().expect("raw scratch path must be UTF-8");
    // Check the harness separately; a missing SLA is not a corpus-wide
    // instruction mismatch. Subsequent images still keep their original PCs.
    writer.write_all(&[0x2b, 0xec, 0, 0x2b, 0xec, 0]).expect("write Carmel bootstrap probe");
    load.raw(path, 0, &[0]).expect("bootstrap canonical Carmel CODE; ensure carmel.sla is compiled");

    let mut bytes = Vec::new();
    let mut digest = Sha256::new();
    let mut line = 0;
    let mut footer = None;
    let mut corpus = BTreeMap::from(STATUSES.map(|status| (status, 0u64)));
    let mut sources = BTreeMap::new();
    let mut native_comparisons = BTreeMap::from(COMPARISONS.map(|status| (status, 0u64)));
    let mut python_comparisons = native_comparisons.clone();
    let (mut attempted, mut native_unknown, mut three_way) = (0u64, 0u64, 0u64);
    let (mut bootstrap_errors, mut decode_errors) = (0u64, 0u64);
    let mut details = BTreeMap::new();
    let mut logged = 0;
    loop {
        bytes.clear();
        line += 1;
        let invalid = |message: &str| -> ! { panic!("Carmel corpus {}:{line}: {message}", input.display()) };
        if reader.read_until(b'\n', &mut bytes).unwrap_or_else(|err| invalid(&err.to_string())) == 0 {
            break;
        }
        digest.update(&bytes);
        let record: Value = serde_json::from_slice(&bytes).unwrap_or_else(|err| invalid(&err.to_string()));
        let kind = record["type"].as_str();
        if line == 1 {
            if kind != Some("header") {
                invalid("first record must be a header");
            }
            if record["schema"].as_str() != Some("carmel-kuna-disasm-v1")
                || record["oracle"]["sha256"].as_str() != Some(CROSSVIEW_SHA256)
                || record["oracle"]["isolate_packets"].as_bool() != Some(true)
            {
                invalid("expected the pinned CrossView executable with isolated packets");
            }
            continue;
        }
        if footer.is_some() {
            invalid("record after summary");
        }
        match kind {
            Some("summary") => {
                footer = Some((line, record));
                continue;
            }
            Some("fetch") => {}
            _ => invalid("expected fetch or final summary record"),
        }
        let pc = word(&record["pc"]).filter(|pc| pc % 2 == 0)
            .unwrap_or_else(|| invalid("pc must be an even 24-bit CODE word address"));
        let packet = record["packet"].as_str().and_then(packet_bytes)
            .unwrap_or_else(|| invalid("packet must be 12 hex digits in canonical big-endian order"));
        let source = record["source"].as_str().filter(|s| !s.is_empty())
            .unwrap_or_else(|| invalid("missing fetch source"));
        let native = reference(&record["native"]).unwrap_or_else(|err| invalid(&format!("native: {err}")));
        let python = reference(&record["python"]).unwrap_or_else(|err| invalid(&format!("python: {err}")));
        let status = match (native, python) {
            (None, None) => "both-error",
            (None, Some(_)) => "native-error",
            (Some(_), None) => "python-error",
            (Some(n), Some(p)) if n == p => "match",
            _ => "mismatch",
        };
        if record["status"].as_str() != Some(status) {
            invalid(&format!("status must be {status} for the recorded observations"));
        }
        *corpus.get_mut(status).unwrap() += 1;
        if let Some(count) = sources.get_mut(source) {
            *count += 1;
        } else {
            sources.insert(source.to_owned(), 1u64);
        }
        attempted += 1;
        native_unknown += u64::from(native.is_some_and(|rows| rows.iter().any(|row| row["text"] == "???;")));

        writer.rewind().and_then(|()| writer.write_all(&packet))
            .unwrap_or_else(|err| invalid(&format!("writing raw scratch image: {err}")));
        let kuna = match load.raw(path, pc, &[pc]) {
            Ok(prog) => disassemble(&prog, pc).map_err(|error| {
                decode_errors += 1;
                ("decode", error)
            }),
            Err(error) => {
                bootstrap_errors += 1;
                Err(("bootstrap", error.to_string()))
            }
        };
        let against_native = compare(native, kuna.as_deref().ok());
        let against_python = compare(python, kuna.as_deref().ok());
        *native_comparisons.get_mut(against_native).unwrap() += 1;
        *python_comparisons.get_mut(against_python).unwrap() += 1;
        if against_native == "match" && against_python == "match" {
            three_way += 1;
        } else if logged < 24 {
            let n = details.entry((against_native, against_python)).or_insert(0);
            if *n < 4 {
                *n += 1;
                logged += 1;
                let kuna = match kuna {
                    Ok(rows) => json!({"rows": rows}),
                    Err((stage, error)) => json!({"stage": stage, "error": error}),
                };
                eprintln!("CARMEL_DISASM_PARITY_DIFF {}", json!({
                    "line": line, "pc": pc, "packet": record["packet"], "source": source,
                    "kuna_vs_native": against_native, "kuna_vs_python": against_python,
                    "native": record["native"], "python": record["python"], "kuna": kuna,
                }));
            }
        }
    }

    let (summary_line, footer) = footer.expect("Carmel corpus must start with a header and end with a summary");
    assert!(attempted > 0, "Carmel corpus contains no fetch records");
    let native_comparable = attempted - corpus["native-error"] - corpus["both-error"];
    for (key, expected) in [
        ("attempted", json!(attempted)),
        ("counts", json!(corpus)),
        ("sources", json!(sources)),
        ("native_comparable", json!(native_comparable)),
        ("native_unknown", json!(native_unknown)),
    ] {
        if let Some(actual) = footer.get(key) {
            assert_eq!(actual, &expected, "Carmel corpus {}:{summary_line}: inconsistent summary {key}", input.display());
        }
    }
    let sha256 = hex(digest.finalize().as_slice());
    println!("CARMEL_DISASM_PARITY_REPORT {}", json!({
        "schema": 1, "arch": TARGET, "corpus_sha256": sha256, "attempted": attempted,
        "sources": sources, "native_comparable": native_comparable, "native_unknown": native_unknown,
        "reference_status": corpus, "kuna_vs_native": native_comparisons, "kuna_vs_python": python_comparisons,
        "kuna_errors": {"bootstrap": bootstrap_errors, "decode": decode_errors},
        "three_way_matches": three_way, "details_logged": logged,
    }));
    assert_eq!(three_way, attempted, "three-way Carmel disassembly parity was not established; CrossView is authoritative");
}
