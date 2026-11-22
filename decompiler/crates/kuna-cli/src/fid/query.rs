//! Read-only, unverified candidate lookup against a Kuna or Ghidra FID database.
use std::path::Path;

use kuna_analysis::fid::kuna_fidapply::{open_any, FidDbStatus, LoadedFid};
use kuna_analysis::fid::hash::FidHashQuad;
use kuna_analysis::fid::{fingerprint_function, fingerprint_raw_function};
use kuna_analysis::listing::CodeUnit;
use kuna_analysis::listing::Listing;
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::engine::{bootstrap_from_object, bootstrap_from_raw, ArmIsa, ConsoleProgram};
use object::Object;
use serde_json::json;

fn usage() {
    eprintln!(
        "usage: kuna fid query <binary> --db <file.fid|file.fidbf|file.fidb> --addr <address> [--addr <address> ...] --json\n\
         \x20                   [--raw-image --target <language-id> --base <address> [--isa arm|thumb]]\n\
         Raw --base is the address of file offset zero; --addr values seed discovery. ARM32 raw input requires an explicit --isa.\n\
         --json emits {{binary, database, address, status, ...}} or {{binary, database, queries, ...}} for multiple addresses; addresses are hex (0x optional); single_candidate is an unverified name"
    );
}

pub(super) fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return 0;
    }
    let args = match parse_args(args) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("kuna fid query: {e}");
            usage();
            return 2;
        }
    };
    let result = match args.raw {
        Some(raw) => query_raw(args.binary, args.db_path, &args.addresses, raw),
        None => query(args.binary, args.db_path, &args.addresses),
    };
    match result {
        Ok(value) => crate::output::emit_with_status(
            &format!("{}\n", serde_json::to_string_pretty(&value).unwrap()),
            0,
        ),
        Err(e) => {
            eprintln!("kuna fid query: {e}");
            1
        }
    }
}

fn parse_address(value: &str) -> Result<u64, String> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u64::from_str_radix(digits, 16).map_err(|_| format!("invalid address {value:?}"))
}

struct QueryArgs<'a> {
    binary: &'a str,
    db_path: &'a str,
    addresses: Vec<u64>,
    raw: Option<RawImage<'a>>,
}

struct RawImage<'a> {
    target: &'a str,
    base: u64,
    isa: Option<ArmIsa>,
}

fn parse_args(args: &[String]) -> Result<QueryArgs<'_>, String> {
    let mut binary = None;
    let mut db_path = None;
    let mut addresses = Vec::new();
    let mut json_output = false;
    let mut raw_image = false;
    let mut target = None;
    let mut base = None;
    let mut isa = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            opt @ ("--db" | "--addr" | "--target" | "--base" | "--isa") => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| format!("missing value for {opt}"))?;
                if opt == "--addr" {
                    addresses.push(parse_address(value)?);
                } else {
                    let slot = match opt {
                        "--db" => &mut db_path,
                        "--target" => &mut target,
                        "--base" => &mut base,
                        _ => &mut isa,
                    };
                    if slot.replace(value.as_str()).is_some() {
                        return Err(format!("duplicate {opt}"));
                    }
                }
            }
            "--raw-image" => raw_image = true,
            "--json" => json_output = true,
            flag if flag.starts_with('-') => return Err(format!("unknown option {flag}")),
            path => {
                if binary.replace(path).is_some() {
                    return Err("expected one binary".into());
                }
            }
        }
        i += 1;
    }
    let binary = binary.ok_or("binary is required")?;
    let db_path = db_path.ok_or("--db is required")?;
    if addresses.is_empty() {
        return Err("--addr is required".into());
    }
    if !json_output {
        return Err("--json is required".into());
    }
    let raw = if raw_image {
        let target = target.filter(|value| !value.trim().is_empty())
            .ok_or("--raw-image requires --target <SLEIGH-language-id>")?;
        let base = parse_address(base.ok_or("--raw-image requires --base <address>")?.trim())?;
        let isa = isa.map(ArmIsa::parse).transpose()?.flatten();
        Some(RawImage { target, base, isa })
    } else {
        if target.is_some() || base.is_some() || isa.is_some() {
            return Err("--target, --base and --isa require --raw-image for FID query".into());
        }
        None
    };
    Ok(QueryArgs { binary, db_path, addresses, raw })
}

fn query(binary: &str, db_path: &str, addresses: &[u64]) -> Result<serde_json::Value, String> {
    let bytes = std::fs::read(binary).map_err(|e| format!("cannot read binary: {e}"))?;
    let file = object::File::parse(&*bytes).map_err(|e| format!("invalid binary: {e}"))?;
    if file.kind() == object::ObjectKind::Relocatable {
        return Err("relocatable objects are not supported for lookup".into());
    }
    let prog = bootstrap_from_object(binary, "", &super::spec_roots())
        .map_err(|e| format!("bootstrap: {e}"))?;
    let arch = prog.arch();
    let lang = kuna_decomp::kuna_langfamily::language_id(&arch.archid);
    let (db, format) = load_database(db_path, &prog)?;
    let image =
        ObjectLoadImage::from_bytes(binary, &bytes).map_err(|e| format!("load image: {e}"))?;
    let base_seeds = kuna_analysis::passes::listing_seeds(&file, &bytes);
    let baseline = Listing::build(&file, &image, arch, arch.translate(), &base_seeds);
    let discovered: std::collections::BTreeSet<u64> =
        baseline.functions().map(|(&entry, _)| entry).collect();
    let mut results = Vec::with_capacity(addresses.len());
    for &requested in addresses {
        let thumb = prog.is_arm32() && requested & 1 != 0;
        let addr = if thumb { requested & !1 } else { requested };
        let scope = arch
            .translate()
            .context_scope()
            .ok_or("translator does not support scoped context")?;
        let boundary = instruction_boundary(&baseline, addr, false);
        let is_discovered = discovered.contains(&addr);
        let source = if is_discovered {
            "discovered"
        } else {
            "user_supplied"
        };
        let run = || {
            if is_discovered {
                fid_candidate((binary, db_path), &db, format, lang, &baseline, &prog, addr)
            } else {
                let mut seeds = base_seeds.clone();
                seeds.push(addr);
                seeds.sort_unstable();
                seeds.dedup();
                let listing = Listing::build(&file, &image, arch, arch.translate(), &seeds);
                fid_candidate((binary, db_path), &db, format, lang, &listing, &prog, addr)
            }
        };
        let result = if thumb {
            prog.with_thumb_pointer(addr, run)
                .map_err(|e| e.to_string())
                .and_then(|result| result)
        } else {
            run()
        };
        drop(scope);
        let result = match result {
            Ok(value) => Ok(value),
            Err(reason) if addresses.len() > 1 => Ok(json!({
                "binary": binary, "database": db_path, "address": format!("0x{addr:x}"),
                "status": "error", "reason": reason, "candidates": []
            })),
            Err(reason) => Err(reason),
        };
        results.push(result.map(|mut value| {
            value["requested_address"] = json!(format!("0x{requested:x}"));
            value["entry_source"] = json!(source);
            value["instruction_boundary"] = json!(boundary);
            value["identity_verified"] = json!(false);
            value
        }));
    }

    package_results(binary, db_path, addresses, results, prog.is_arm32())
}

fn query_raw(
    binary: &str,
    db_path: &str,
    addresses: &[u64],
    raw: RawImage<'_>,
) -> Result<serde_json::Value, String> {
    let prog = bootstrap_from_raw(binary, raw.target, raw.base, addresses, raw.isa, &super::spec_roots())
        .map_err(|e| format!("bootstrap: {}", e.explain()))?;
    let (db, format) = load_database(db_path, &prog)?;
    let listing = prog.pending_raw_listing_for_fid().map_err(|e| e.explain().to_owned())?;
    let lang = kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid);
    let results = addresses.iter().map(|&requested| {
        let addr = prog.input_code_offset(requested).map_err(|e| e.explain().to_owned())?;
        let mut value = lookup(
            (binary, db_path), &db, format, lang, prog.output_code_offset(addr),
            fingerprint_raw_function(&listing, prog.arch(), addr),
        );
        value["requested_address"] = json!(format!("0x{requested:x}"));
        // Every requested raw address is a caller-supplied seed, not
        // independent evidence that a function starts here.
        value["entry_source"] = json!("user_supplied");
        value["instruction_boundary"] = json!(instruction_boundary(&listing, addr, true));
        value["identity_verified"] = json!(false);
        Ok(value)
    }).collect();
    package_results(binary, db_path, addresses, results, prog.is_arm32())
}

fn load_database(db_path: &str, prog: &ConsoleProgram) -> Result<(LoadedFid, &'static str), String> {
    let (report, db) = open_any(Path::new(db_path), prog.arch());
    let format = match &db {
        LoadedFid::Kuna(_) => "kuna_fid",
        LoadedFid::Ghidra(_) => "ghidra_fid",
        LoadedFid::None => {
            let reason = report.reason.unwrap_or_default();
            return Err(match report.status {
                FidDbStatus::Missing | FidDbStatus::Unreadable => format!("cannot read database: {reason}"),
                FidDbStatus::Invalid => format!("invalid database: {reason}"),
                _ => reason,
            });
        }
    };
    Ok((db, format))
}

fn instruction_boundary(listing: &Listing, addr: u64, raw: bool) -> &'static str {
    match listing.code_unit_at(addr) {
        // A raw Listing can contain a decode that used zero padding at EOF.
        CodeUnit::Instruction(start) if raw && !listing.instruction_at(start).is_some_and(|insn| {
            start.checked_add(u64::from(insn.len)).is_some_and(|end| {
                listing.exec_ranges().iter().any(|&(lo, hi)| lo <= start && end <= hi)
            })
        }) => "undecoded",
        CodeUnit::Instruction(start) if start == addr => "instruction_start",
        CodeUnit::Instruction(_) => "inside_instruction",
        CodeUnit::Undefined => "undecoded",
        CodeUnit::Data(..) => "non_executable",
    }
}

pub(super) fn package_results(
    binary: &str,
    database: &str,
    addresses: &[u64],
    results: Vec<Result<serde_json::Value, String>>,
    arm32: bool,
) -> Result<serde_json::Value, String> {
    if results.len() == 1 {
        return results.into_iter().next().expect("one result");
    }
    let queries: Vec<_> = addresses
        .iter()
        .zip(results)
        .map(|(&requested, result)| {
            let address = if arm32 { requested & !1 } else { requested };
            result.unwrap_or_else(|error| {
                json!({"binary": binary, "database": database,
            "address": format!("0x{address:x}"), "requested_address": format!("0x{requested:x}"),
            "identity_verified": false, "status": "error", "reason": error, "candidates": []})
            })
        })
        .collect();
    Ok(json!({"binary": binary, "database": database, "queries": queries}))
}

fn fid_candidate(
    (binary, db_path): (&str, &str),
    db: &LoadedFid,
    database_format: &'static str,
    lang: &str,
    listing: &Listing,
    prog: &ConsoleProgram,
    addr: u64,
) -> Result<serde_json::Value, String> {
    if !listing.functions().any(|(&entry, _)| entry == addr) {
        return Err(format!(
            "address 0x{addr:x} could not be decoded as a function entry"
        ));
    }
    if prog.is_arm32() {
        let end = listing.next_function_after(addr).map(|f| f.entry);
        for (vma, insn) in listing.instructions_in_range(addr, end) {
            let matches_listing =
                prog.disassemble_at(*vma)
                    .is_ok_and(|(len, mnemonic, operands)| {
                        u32::try_from(len).ok() == Some(insn.len)
                            && mnemonic == insn.mnemonic.as_str()
                            && operands == insn.operands
                    });
            if !matches_listing {
                return Ok(
                    json!({"binary": binary, "database": db_path, "address": format!("0x{addr:x}"), "status": "unavailable", "reason": "decode_context_mismatch", "candidates": []}),
                );
            }
        }
    }
    Ok(lookup(
        (binary, db_path), db, database_format, lang, addr,
        fingerprint_function(listing, prog.arch(), addr),
    ))
}

fn lookup(
    (binary, db_path): (&str, &str),
    db: &LoadedFid,
    database_format: &'static str,
    lang: &str,
    addr: u64,
    fingerprint: Result<FidHashQuad, &'static str>,
) -> serde_json::Value {
    let quad = match fingerprint {
        Ok(quad) => quad,
        Err(reason) => return json!({
            "binary": binary, "database": db_path, "address": format!("0x{addr:x}"),
            "status": "unavailable", "reason": reason, "candidates": []
        }),
    };
    let mut candidates: Vec<_> = db
        .find_by_full_hash(quad.full_hash)
        .iter()
        .map(|r| {
            let mut candidate = json!({
                "name": r.name.as_ref(), "specific_hash": format!("0x{:016x}", r.specific_hash),
                "specific_hash_matches": r.specific_hash == quad.specific_hash,
                "code_unit_size": r.code_unit_size,
                "code_unit_size_matches": r.code_unit_size == quad.code_unit_size,
            });
            if matches!(db, LoadedFid::Ghidra(_)) {
                candidate["ghidra_flags"] = json!(r.flags);
            }
            candidate
        })
        .collect();
    candidates.sort_by(|a, b| {
        a["name"].as_str().cmp(&b["name"].as_str()).then_with(|| {
            a["specific_hash"]
                .as_str()
                .cmp(&b["specific_hash"].as_str())
        })
    });
    let names: std::collections::BTreeSet<_> = candidates
        .iter()
        .filter_map(|c| c["name"].as_str())
        .collect();
    let specific_evidence = if candidates.is_empty() {
        "no_candidates"
    } else if candidates.iter().any(|candidate| candidate["specific_hash_matches"] == true) {
        "matching_candidate"
    } else {
        "no_candidate_matches"
    };
    let status = match names.len() {
        0 => "no_match",
        1 => "single_candidate",
        _ => "ambiguous",
    };
    json!({"binary": binary, "database": db_path, "database_format": database_format, "language": lang,
        "address": format!("0x{addr:x}"), "status": status, "match_basis": "full_hash",
        "full_hash": format!("0x{:016x}", quad.full_hash), "specific_hash": format!("0x{:016x}", quad.specific_hash),
        "code_unit_size": quad.code_unit_size, "specific_hash_evidence": specific_evidence, "candidates": candidates})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflicting_arm_decode_context_is_unavailable_and_scoped() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .unwrap();
        let binary = root.join("decompiler/crates/kuna-analysis/tests/fixtures/arm_interwork_le32");
        let path = binary.to_str().unwrap();
        let bytes = std::fs::read(&binary).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let image = ObjectLoadImage::from_bytes(path, &bytes).unwrap();
        let roots = vec![root.join("specs").to_str().unwrap().to_string()];
        let prog = bootstrap_from_object(path, "", &roots).unwrap();
        let arch = prog.arch();
        let seeds = kuna_analysis::passes::listing_seeds(&file, &bytes);
        let listing = Listing::build(&file, &image, arch, arch.translate(), &seeds);
        let db = LoadedFid::Kuna(kuna_analysis::fid::db::FidDb::from_records(
            "ARM:LE:32:v8",
            "default",
            vec![],
        ));
        let query = || {
            fid_candidate(
                (path, "test.fid"),
                &db,
                "kuna_fid",
                "ARM:LE:32:v8",
                &listing,
                &prog,
                0x2000080,
            )
            .unwrap()
        };
        let before = query();
        assert_eq!(before["reason"], "too_short");
        let conflict = prog.with_thumb_pointer(0x2000080, query).unwrap();
        assert_eq!(conflict["status"], "unavailable");
        assert_eq!(conflict["reason"], "decode_context_mismatch");
        assert_eq!(query(), before);
    }
}
