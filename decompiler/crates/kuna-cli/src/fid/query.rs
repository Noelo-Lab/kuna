//! Read-only candidate lookup against a user-supplied kuna FID database.
use kuna_analysis::fid::db::{FidDatabase, FidDb};
use kuna_analysis::fid::fingerprint_function;
use kuna_analysis::listing::Listing;
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::engine::bootstrap_from_object;
use object::Object;
use serde_json::json;

pub(super) fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: kuna fid query <binary> --db <file.fid|file.sig> --addr <address> [--addr <address> ...] --json\n--json emits {{binary, database, address, status, ...}} or {{binary, database, queries, ...}} for multiple addresses");
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
    if db_bytes.starts_with(b"IDASGN") {
        return super::flirt::query(binary, db_path, &db_bytes, &addresses, &file);
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
    let mut results = Vec::with_capacity(addresses.len());
    let mut shared_listing = None;
    for &addr in &addresses {
        if base_seeds.binary_search(&addr).is_ok() {
            let listing = shared_listing.get_or_insert_with(|| {
                Listing::build(&file, &image, arch, arch.translate(), &base_seeds)
            });
            results.push(fid_candidate(
                binary, db_path, &db, lang, listing, arch, addr,
            ));
        } else {
            let mut seeds = base_seeds.clone();
            seeds.push(addr);
            seeds.sort_unstable();
            seeds.dedup();
            let listing = Listing::build(&file, &image, arch, arch.translate(), &seeds);
            results.push(fid_candidate(
                binary, db_path, &db, lang, &listing, arch, addr,
            ));
        }
    }
    package_results(binary, db_path, &addresses, results)
}

pub(super) fn package_results(
    binary: &str,
    database: &str,
    addresses: &[u64],
    results: Vec<Result<serde_json::Value, String>>,
) -> Result<serde_json::Value, String> {
    if results.len() == 1 {
        return results.into_iter().next().expect("one result");
    }
    let queries: Vec<_> = addresses.iter().zip(results).map(|(&address, result)| {
        result.unwrap_or_else(|error| json!({"binary": binary, "database": database,
            "address": format!("0x{address:x}"), "status": "error", "reason": error, "candidates": []}))
    }).collect();
    Ok(json!({"binary": binary, "database": database, "queries": queries}))
}

fn fid_candidate(
    binary: &str,
    db_path: &str,
    db: &FidDb,
    lang: &str,
    listing: &Listing,
    arch: &kuna_decomp::architecture::Architecture,
    addr: u64,
) -> Result<serde_json::Value, String> {
    if !listing.functions().any(|(&entry, _)| entry == addr) {
        return Err(format!(
            "address 0x{addr:x} could not be decoded as a function entry"
        ));
    }
    let quad = match fingerprint_function(listing, arch, addr) {
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
        1 => "unique",
        _ => "ambiguous",
    };
    Ok(
        json!({"binary": binary, "database": db_path, "database_format": "kuna_fid", "language": lang,
        "address": format!("0x{addr:x}"), "status": status, "match_basis": "full_hash",
        "full_hash": format!("0x{:016x}", quad.full_hash), "specific_hash": format!("0x{:016x}", quad.specific_hash),
        "code_unit_size": quad.code_unit_size, "candidates": candidates}),
    )
}
