//! Public compiler-built regressions for automatic selection and imported-call continuation.
use crate::common;
use serde_json::Value;

fn manifest() -> Value {
    serde_json::from_str(include_str!("../../../../tests/fixtures/mips-import-flow/provenance.json")).unwrap()
}

fn run(image: &Value, options: &[&str]) -> Value {
    let path = common::repo_root().join("tests/fixtures/mips-import-flow")
        .join(image["file"].as_str().unwrap());
    let mut args = vec!["decompile-all", path.to_str().unwrap(), "--json"];
    args.extend_from_slice(options);
    let (out, err, rc) = common::run_kuna(&args);
    assert_eq!(rc, 0, "{}: {err}\n{out}", path.display());
    let doc: Value = serde_json::from_str(&out).unwrap();
    for f in doc["functions"].as_array().unwrap() {
        assert!(f["error"].is_null(), "{}: {f}", path.display());
    }
    let bytes = std::fs::read(&path).unwrap();
    let target = options.iter().position(|&o| o == "--target").map(|i| options[i + 1]);
    let image = match target {
        Some(target) => kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_with_target(
            path.to_str().unwrap(), &bytes, target),
        None => kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(path.to_str().unwrap(), &bytes),
    }.unwrap();
    for &(site, end) in &image.mips_linkage().unwrap().transfers {
        let entry = site - 8;
        let code = body(&doc, entry);
        assert!(!code.contains("no-return") && !code.contains("halt_baddata"),
            "resolver is a transfer, not a terminal callee: {entry:#x}: {code}");
        let f = doc["functions"].as_array().unwrap().iter()
            .find(|f| f["address"].as_u64() == Some(entry)).unwrap();
        for mapping in f["line_mappings"].as_array().unwrap() {
            for address in mapping["addresses"].as_array().unwrap() {
                assert!((entry..end).contains(&address.as_u64().unwrap()),
                    "stub escaped its instruction range: {f}");
            }
        }
    }
    doc
}

fn body<'a>(doc: &'a Value, address: u64) -> &'a str {
    let f = doc["functions"].as_array().unwrap().iter()
        .find(|f| f["address"].as_u64() == Some(address)).expect("original function survives discovery");
    assert!(f["error"].is_null(), "{f}");
    f["code"].as_str().unwrap()
}

#[test]
fn returning_and_unknown_imports_preserve_following_effects() {
    for image in manifest()["images"].as_array().unwrap() {
        let file = image["file"].as_str().unwrap();
        if image["source"] != "flow.c" || file.starts_with("flow-octeon3-") || file.contains("-stripped") { continue; }
        let doc = run(image, &[]);
        let code = body(&doc, image["symbols"]["duplicate"]["address"].as_u64().unwrap());
        assert!(code.contains("strlen(") && !code.contains("strlen()"),
            "declared import lost its incoming argument: {}: {code}", image["file"]);
        assert_eq!(code.matches("memcpy(").count(), 2, "{}: {code}", image["file"]);
        assert!((code.contains(" = 0;") || code.contains(" = '\\0';")) && code.contains("return ") && !code.contains("// no-return"),
            "{}: {code}", image["file"]);
        let code = body(&doc, image["symbols"]["unknown_then_store"]["address"].as_u64().unwrap());
        assert!(code.contains("external_transform(") && code.contains("0x5a")
            && code.contains("return ") && !code.contains("// no-return"), "{}: {code}", image["file"]);
        if let Some(address) = image["symbols"]["fail"]["address"].as_u64() {
            let code = body(&doc, address);
            assert!(code.contains("__stack_chk_fail();") && code.contains("no-return")
                && !code.contains("return;"), "{}: {code}", image["file"]);
        }
    }
}

#[test]
fn import_prototypes_and_continuations_survive_single_and_worker_paths() {
    let manifest = manifest();
    for image in manifest["images"].as_array().unwrap().iter().filter(|image|
        image["file"].as_str().unwrap().starts_with("flow-mips32r2-")
            && image["file"].as_str().unwrap().ends_with("local-ssp.so")) {
        let address = image["symbols"]["duplicate"]["address"].as_u64().unwrap();
        let serial = run(image, &["--option", "protoorder", "off"]);
        let workers = run(image, &["--jobs", "2", "--jobs-chunk", "1", "--option", "protoorder", "off"]);
        assert_eq!(body(&serial, address), body(&workers, address));
        let path = common::repo_root().join("tests/fixtures/mips-import-flow")
            .join(image["file"].as_str().unwrap());
        let (code, err, rc) = common::run_kuna(&["decompile", path.to_str().unwrap(),
            &format!("0x{address:x}"), "--addr"]);
        assert_eq!(rc, 0, "{err}");
        assert!(!code.contains("strlen()") && code.matches("memcpy(").count() == 2
            && code.contains("return ") && !code.contains("// no-return"), "{code}");
    }
}

#[test]
fn memcpy_and_printf_calls_keep_their_continuations() {
    let path = common::fixture("floatret_chain_mips_O0");
    let (out, err, rc) = common::run_kuna(&["decompile-all", &path, "--json"]);
    assert_eq!(rc, 0, "{err}");
    let doc: Value = serde_json::from_str(&out).unwrap();
    let copy = body(&doc, 0x400c64);
    assert!(copy.contains("memcpy("), "{copy}");
    for code in [copy, body(&doc, 0x400d60)] {
        for line in code.lines() {
            assert!(!(line.contains("// no-return") && (line.contains("memcpy(") || line.contains("printf("))), "{code}");
        }
    }
    assert!(copy.contains("dat_4120f0") && copy.contains("dat_4120f8"), "source-required stores: {copy}");
    let main = body(&doc, 0x400d60);
    assert!(main.contains("printf(") && !main.contains("0x41a060"),
        "a variadic import must not take the saved gp slot as an argument: {main}");
}

#[test]
fn n32_lazy_stubs_end_at_their_resolver_and_name_their_imports() {
    let manifest = manifest();
    let image = manifest["images"].as_array().unwrap().iter()
        .find(|image| image["file"] == "flow-octeon3-n32.so").unwrap();
    let doc = run(image, &["--target", "MIPS:BE:64:octeon3-32addr"]);
    let names: Vec<&str> = doc["functions"].as_array().unwrap().iter()
        .filter_map(|f| f["name"].as_str()).collect();
    for name in ["strcpy", "external_transform", "memcpy", "strlen"] {
        assert!(names.contains(&name), "{name}: {names:?}");
    }
    assert!(!names.contains(&"_MIPS_STUBS_"), "{names:?}");
}
