//! (kuna `coverage`) Each `--json` record names the bytes its function's blocks
//! cover, a whole-binary run says how much of the image's executable code that
//! is, and a project export's completion line says the same share.

use crate::common;

use object::{Object, ObjectSection, ObjectSymbol, SectionFlags};

fn run(args: &[&str]) -> (String, String) {
    let specs = common::repo_root().join("specs");
    let mut argv = args.to_vec();
    argv.extend_from_slice(&["--sleighpath", specs.to_str().unwrap()]);
    let (stdout, stderr, code) = common::run_kuna(&argv);
    assert_eq!(code, 0, "kuna {args:?}:\n{stdout}\n{stderr}");
    (stdout, stderr)
}

fn json(args: &[&str]) -> serde_json::Value {
    let (stdout, _) = run(args);
    serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("kuna {args:?} printed no JSON ({e}):\n{stdout}"))
}

/// The bytes of the image's executable sections, read with `object` rather than
/// kuna's loader.
fn executable_sections(path: &str) -> Vec<(u64, Vec<u8>)> {
    let data = std::fs::read(path).unwrap();
    let file = object::File::parse(&*data).unwrap();
    file.sections()
        .filter(|s| {
            matches!(s.flags(), SectionFlags::Elf { sh_flags }
                if sh_flags & u64::from(object::elf::SHF_EXECINSTR) != 0)
        })
        .map(|s| (s.address(), s.data().unwrap().to_vec()))
        .collect()
}

/// The symbol table's `[value, value + size)` for `name`.
fn symbol_extent(path: &str, name: &str) -> (u64, u64) {
    let data = std::fs::read(path).unwrap();
    let file = object::File::parse(&*data).unwrap();
    let symbol = file.symbols().find(|s| s.name() == Ok(name)).unwrap();
    (symbol.address(), symbol.address() + symbol.size())
}

fn ranges(record: &serde_json::Value) -> Vec<(u64, u64)> {
    record["ranges"]
        .as_array()
        .unwrap_or_else(|| panic!("no ranges: {record}"))
        .iter()
        .map(|r| (r["start"].as_u64().unwrap(), r["end"].as_u64().unwrap()))
        .collect()
}

/// `ranges`, joined, as a byte set.
fn bytes_of<'a>(ranges: impl Iterator<Item = &'a (u64, u64)>) -> std::collections::BTreeSet<u64> {
    ranges.flat_map(|&(start, end)| start..end).collect()
}

fn count(doc: &serde_json::Value, key: &str) -> u64 {
    doc["coverage"][key].as_u64().unwrap_or_else(|| panic!("coverage.{key}: {}", doc["coverage"]))
}

#[test]
fn a_whole_binary_run_measures_the_executable_bytes_its_functions_cover() {
    let bin = common::fixture("fauxware");
    let doc = json(&["decompile-all", &bin, "--json", "--coverage"]);
    let functions = doc["functions"].as_array().unwrap();
    let code = executable_sections(&bin);
    let executable: u64 = code.iter().map(|(_, bytes)| bytes.len() as u64).sum();
    assert_eq!(executable, 0x18 + 0x80 + 0x338 + 0xe, "fauxware's .init, .plt, .text and .fini");

    let lifted = bytes_of(functions.iter().flat_map(ranges).collect::<Vec<_>>().iter());
    let decompiled = bytes_of(
        functions.iter().filter(|f| f["error"].is_null()).flat_map(ranges).collect::<Vec<_>>().iter(),
    );
    let in_code = |set: &std::collections::BTreeSet<u64>| {
        code.iter().map(|(start, bytes)| (*start..*start + bytes.len() as u64).filter(|a| set.contains(a)).count() as u64).sum::<u64>()
    };
    let unlifted_nul: u64 = code
        .iter()
        .map(|(start, bytes)| {
            bytes.iter().enumerate().filter(|&(i, &b)| b == 0 && !lifted.contains(&(start + i as u64))).count() as u64
        })
        .sum();

    assert_eq!(count(&doc, "executable_bytes"), executable);
    assert_eq!(count(&doc, "lifted_bytes"), in_code(&lifted));
    assert_eq!(count(&doc, "decompiled_bytes"), in_code(&decompiled));
    assert_eq!(count(&doc, "unlifted_nul_bytes"), unlifted_nul);
    assert_eq!(count(&doc, "functions"), functions.len() as u64);
    assert_eq!(
        count(&doc, "decompiled_functions"),
        functions.iter().filter(|f| f["error"].is_null()).count() as u64
    );
    let share = in_code(&lifted) as f64 / (executable - unlifted_nul) as f64;
    assert_eq!(doc["coverage"]["lifted_share"].to_string(), format!("{share:.4}"));

    let plain = json(&["decompile-all", &bin, "--json"]);
    assert!(plain.get("coverage").is_none(), "only --coverage asks for it: {plain}");
}

#[test]
fn coverage_is_refused_on_a_narrowed_run() {
    let bin = common::fixture("fauxware");
    let (_, stderr, code) = common::run_kuna(&["decompile-all", &bin, "--json", "--coverage", "--functions", "main"]);
    assert_eq!(code, 2, "{stderr}");
    assert!(stderr.contains("--coverage measures the whole binary"), "{stderr}");

    let specs = common::repo_root().join("specs");
    let specs = specs.to_str().unwrap();
    let narrowed = ["decompile-all", &bin, "--json", "--coverage", "--min-size", "100", "--sleighpath", specs];
    let (_, stderr, code) = common::run_kuna(&narrowed);
    assert_eq!(code, 1, "{stderr}");
    assert!(stderr.contains("the triage filters kept"), "{stderr}");

    let kept_all = json(&["decompile-all", &bin, "--json", "--coverage", "--min-size", "0"]);
    assert!(kept_all.get("coverage").is_some(), "a filter that drops nothing still measures: {kept_all}");
}

#[test]
fn a_record_names_the_bytes_its_blocks_cover() {
    let bin = common::fixture("fauxware");
    let doc = json(&["decompile", &bin, "main", "--json"]);
    let main = &doc["functions"][0];
    assert_eq!(ranges(main), vec![symbol_extent(&bin, "main")], "{main}");
    assert_eq!(main["ranges"][0]["start_hex"], "0x40071d");
    assert!(doc.get("coverage").is_none(), "one function is not a measure of the binary: {doc}");
}

#[test]
fn ranges_can_be_omitted() {
    let bin = common::fixture("fauxware");
    let doc = json(&["decompile", &bin, "main", "--json", "--omit", "ranges"]);
    assert!(doc["functions"][0].get("ranges").is_none(), "{doc}");
    assert_eq!(doc["omitted"], serde_json::json!(["ranges"]));
}

#[test]
fn a_jobs_run_reports_what_a_serial_run_does() {
    let bin = common::fixture("fauxware");
    let serial = json(&["decompile-all", &bin, "--json", "--coverage", "--option", "protoorder", "off"]);
    let pooled = json(&["decompile-all", &bin, "--json", "--coverage", "--option", "protoorder", "off", "--jobs", "2"]);
    assert_eq!(pooled["coverage"], serial["coverage"]);
    let by_name = |doc: &serde_json::Value| -> Vec<(String, Vec<(u64, u64)>)> {
        doc["functions"].as_array().unwrap().iter().map(|f| (f["name"].as_str().unwrap().to_owned(), ranges(f))).collect()
    };
    assert_eq!(by_name(&pooled), by_name(&serial));
}

#[test]
fn a_project_export_says_how_much_code_it_covers() {
    let bin = common::fixture("fauxware");
    let doc = json(&["decompile-all", &bin, "--json", "--coverage"]);
    let out = common::scratch_file("coverage-project", "kuna");
    let (stdout, _) = run(&["decompile-project", &bin, "-o", out.to_str().unwrap()]);
    let total = count(&doc, "executable_bytes") - count(&doc, "unlifted_nul_bytes");
    assert!(stdout.contains(&format!(" decompiled of {total} bytes\n")), "{stdout}");
    assert!(stdout.contains("; code: ") && stdout.contains("% lifted, "), "{stdout}");
}

#[test]
fn a_range_ending_in_a_branch_takes_in_its_delay_slot() {
    // MIPS `main` ends with `jr $ra` at 0x201a4 and its delay slot at 0x201a8.
    let bin = common::fixture("adrnext_mips");
    let doc = json(&["decompile", &bin, "main", "--json"]);
    assert_eq!(ranges(&doc["functions"][0]), vec![symbol_extent(&bin, "main")], "{doc}");
}

#[test]
fn a_thumb_function_is_covered_at_its_even_address() {
    let bin = common::fixture("arm_funcmode_le32");
    let doc = json(&["decompile", &bin, "thumb_helper", "--json"]);
    let (start, end) = symbol_extent(&bin, "thumb_helper");
    assert_eq!(ranges(&doc["functions"][0]), vec![(start & !1, end & !1)], "{doc}");
}

#[test]
fn a_streamed_export_reports_what_a_buffered_one_does_and_a_narrowed_one_nothing() {
    let bin = common::fixture("fauxware");
    let export = |extra: &[&str]| {
        let out = common::scratch_file("coverage-export", "kuna");
        let mut args = vec!["decompile-project", bin.as_str(), "-o", out.to_str().unwrap()];
        args.extend_from_slice(extra);
        run(&args).0
    };
    let clause = |stdout: &str| stdout.find("; code: ").map(|at| stdout[at..].to_owned());
    let buffered = export(&[]);
    assert!(clause(&buffered).is_some(), "{buffered}");
    assert_eq!(clause(&export(&["--stream"])), clause(&buffered));
    let narrowed = export(&["--functions", "main"]);
    assert_eq!(clause(&narrowed), None, "{narrowed}");
}
