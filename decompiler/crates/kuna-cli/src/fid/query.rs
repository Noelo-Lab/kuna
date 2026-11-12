//! Read-only candidate lookup against a user-supplied kuna FID database.
use kuna_analysis::fid::db::{FidDatabase, FidDb};
use kuna_analysis::fid::fingerprint_function;
use kuna_analysis::listing::CodeUnit;
use kuna_analysis::listing::Listing;
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use object::Object;
use serde_json::json;

pub(super) fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: kuna fid query <binary> --db <file.fid> --addr <address> [--addr <address> ...] --json\n--json emits {{binary, database, address, status, ...}} or {{binary, database, queries, ...}} for multiple addresses; single_candidate is an unverified name");
        return 0;
    }
    match query(args) {
        Ok(value) => {
            println!("{}", serde_json::to_string_pretty(&value).unwrap());
            0
        }
        Err(e) => {
            eprintln!("kuna fid query: {e}");
            1
        }
    }
}

fn query(args: &[String]) -> Result<serde_json::Value, String> {
    let mut binary = None;
    let mut db_path = None;
    let mut addresses = Vec::new();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--db" | "--addr" => {
                let opt = args[i].as_str();
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| format!("missing value for {opt}"))?;
                if opt == "--db" {
                    if db_path.replace(value.as_str()).is_some() {
                        return Err("duplicate --db".into());
                    }
                } else {
                    let parsed = if let Some(hex) = value.strip_prefix("0x") {
                        u64::from_str_radix(hex, 16)
                    } else {
                        value.parse::<u64>()
                    }
                    .map_err(|_| "invalid address")?;
                    addresses.push(parsed);
                }
            }
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
    let db_bytes = std::fs::read(db_path).map_err(|e| format!("cannot read database: {e}"))?;
    let bytes = std::fs::read(binary).map_err(|e| format!("cannot read binary: {e}"))?;
    let file = object::File::parse(&*bytes).map_err(|e| format!("invalid binary: {e}"))?;
    if file.kind() == object::ObjectKind::Relocatable {
        return Err("relocatable objects are not supported for lookup".into());
    }
    let db = FidDb::load(&db_bytes).map_err(|e| format!("invalid database: {e}"))?;
    let prog = bootstrap_from_object(binary, "", &super::spec_roots())
        .map_err(|e| format!("bootstrap: {e}"))?;
    let arch = prog.arch();
    let lang = arch
        .archid
        .rsplit_once(':')
        .map(|(lang, _)| lang)
        .unwrap_or(&arch.archid);
    if db.lang_id() != lang {
        return Err(format!(
            "database language {} differs from binary language {lang}",
            db.lang_id()
        ));
    }
    let image =
        ObjectLoadImage::from_bytes(binary, &bytes).map_err(|e| format!("load image: {e}"))?;
    let base_seeds = kuna_analysis::passes::listing_seeds(&file, &bytes);
    let baseline = Listing::build(&file, &image, arch, arch.translate(), &base_seeds);
    let discovered: std::collections::BTreeSet<u64> =
        baseline.functions().map(|(&entry, _)| entry).collect();
    let mut results = Vec::with_capacity(addresses.len());
    for &requested in &addresses {
        let thumb = prog.is_arm32() && requested & 1 != 0;
        let addr = if thumb { requested & !1 } else { requested };
        let scope = arch
            .translate()
            .context_scope()
            .ok_or("translator does not support scoped context")?;
        let boundary = match baseline.code_unit_at(addr) {
            CodeUnit::Instruction(start) if start == addr => "instruction_start",
            CodeUnit::Instruction(_) => "inside_instruction",
            CodeUnit::Undefined => "undecoded",
            CodeUnit::Data(..) => "non_executable",
        };
        let is_discovered = discovered.contains(&addr);
        let source = if is_discovered {
            "discovered"
        } else {
            "user_supplied"
        };
        let run = || {
            if is_discovered {
                fid_candidate(binary, db_path, &db, lang, &baseline, &prog, addr)
            } else {
                let mut seeds = base_seeds.clone();
                seeds.push(addr);
                seeds.sort_unstable();
                seeds.dedup();
                let listing = Listing::build(&file, &image, arch, arch.translate(), &seeds);
                fid_candidate(binary, db_path, &db, lang, &listing, &prog, addr)
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

    package_results(binary, db_path, &addresses, results, prog.is_arm32())
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
    binary: &str,
    db_path: &str,
    db: &FidDb,
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
        for insn in kuna_analysis::fid::extent::calculate_extent(listing, addr) {
            let matches_listing =
                prog.disassemble_at(insn.addr)
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
    let quad = match fingerprint_function(listing, prog.arch(), addr) {
        Ok(quad) => quad,
        Err(reason) => {
            return Ok(
                json!({"binary": binary, "database": db_path, "address": format!("0x{addr:x}"), "status": "unavailable", "reason": reason, "candidates": []}),
            )
        }
    };
    let mut candidates: Vec<_> = db
        .find_by_full_hash(quad.full_hash)
        .iter()
        .map(|r| {
            json!({
                "name": r.name, "specific_hash": format!("0x{:016x}", r.specific_hash),
                "specific_hash_matches": r.specific_hash == quad.specific_hash,
                "code_unit_size": r.code_unit_size,
            })
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
    let status = match names.len() {
        0 => "no_match",
        1 => "single_candidate",
        _ => "ambiguous",
    };
    Ok(
        json!({"binary": binary, "database": db_path, "database_format": "kuna_fid", "language": lang,
        "address": format!("0x{addr:x}"), "status": status, "match_basis": "full_hash",
        "full_hash": format!("0x{:016x}", quad.full_hash), "specific_hash": format!("0x{:016x}", quad.specific_hash),
        "code_unit_size": quad.code_unit_size, "candidates": candidates}),
    )
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
        let db = FidDb::from_records("ARM:LE:32:v8", "default", vec![]);
        let query = || {
            fid_candidate(
                path,
                "test.fid",
                &db,
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
