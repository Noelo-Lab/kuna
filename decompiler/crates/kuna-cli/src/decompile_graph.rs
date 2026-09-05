//! `kuna decompile-graph` — whole-program JSON: every function with its C, its
//! assembly, and the call edges between them.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use kuna_analysis::listing::xrefs::XrefKind;
use kuna_console::engine::{ConsoleProgram, EntryProvenance, FunctionEntry};
use kuna_console::project::{decompile_targets, FuncResult};
use object::{Object, ObjectSegment};

use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::jsonfmt::{dumps_indent2, Json};

struct ExportArgs {
    binary: String,
    version: String,
    output: Option<String>,
    max_fn_seconds: Option<u64>,
    options: Vec<(String, String)>,
    mode: Option<String>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
}

pub fn run(argv: &[String]) -> i32 {
    let args = match parse_args(argv) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("error: {error}");
            usage();
            return 2;
        }
    };
    match export(&args) {
        Ok(text) => {
            if let Some(path) = &args.output {
                if let Err(error) = std::fs::write(path, &text) {
                    eprintln!("error: cannot write {path}: {error}");
                    return 1;
                }
                0
            } else {
                crate::output::emit_with_status(&text, 0)
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            1
        }
    }
}

fn export(args: &ExportArgs) -> Result<String, String> {
    let options =
        mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
    let load = Args {
        binary: args.binary.clone(),
        json: false,
        names: None,
        addrs: Vec::new(),
        no_vars: false,
        max_fn_seconds: args.max_fn_seconds.unwrap_or(120),
        options,
        func_decls: Vec::new(),
        assertions: Vec::new(),
        assert_strict: false,
        slice: args.slice.clone(),
        target: args.target.clone(),
        sleighpath: args.sleighpath.clone(),
    };
    let mut prog = load_program(&load, DriverDefaults::Decompile)?;
    if load.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget = Some(std::time::Duration::from_secs(load.max_fn_seconds));
    }
    let entries = prog.function_entries_canonical();
    let body_entries: Vec<FunctionEntry> = entries
        .iter()
        .filter(|entry| {
            function_kind(&prog, entry) == "normal" && prog.entry_bytes_mapped(&entry.addr)
        })
        .cloned()
        .collect();
    let results = decompile_targets(&mut prog, body_entries, false, true, false);
    for result in &results {
        if let Some(error) = &result.error {
            eprintln!(
                "warning: could not decompile {} @ 0x{:x}: {error}",
                result.name, result.address
            );
        }
    }
    let result_by_address: BTreeMap<u64, &FuncResult> = results
        .iter()
        .map(|result| (result.address, result))
        .collect();

    let bytes = std::fs::read(&args.binary).map_err(|error| format!("{}: {error}", args.binary))?;
    let file = object::File::parse(&*bytes)
        .map_err(|error| format!("could not parse {}: {error}", args.binary))?;
    let seeds: Vec<u64> = entries
        .iter()
        .map(|entry| entry.addr.get_offset())
        .collect();
    let xrefs =
        kuna_analysis::listing::xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds);
    let functions = Json::Array(
        entries
            .iter()
            .map(|entry| {
                let address = entry.addr.get_offset();
                let kind = function_kind(&prog, entry);
                let result = result_by_address.get(&address).copied();
                Json::Object(vec![
                    ("address".into(), number(address)),
                    ("name".into(), Json::Str(entry.name.clone())),
                    ("parameters".into(), parameters(result)),
                    (
                        "signature".into(),
                        result
                            .and_then(|result| result.proto.as_ref())
                            .map_or(Json::Null, |value| {
                                Json::Str(value.trim().trim_end_matches(';').to_string())
                            }),
                    ),
                    (
                        "assembly".into(),
                        if kind == "normal" {
                            assembly(&prog, entry).map_or(Json::Null, Json::Str)
                        } else {
                            Json::Null
                        },
                    ),
                    (
                        "codeC".into(),
                        result
                            .and_then(|result| result.code.as_ref())
                            .map_or(Json::Null, |value| Json::Str(value.clone())),
                    ),
                    ("kind".into(), Json::Str(kind.into())),
                    (
                        "hasIndirectCalls".into(),
                        Json::Bool(xrefs.has_indirect_calls(address)),
                    ),
                    (
                        "isEntryPoint".into(),
                        Json::Bool(is_entry_point(&file, entry)),
                    ),
                ])
            })
            .collect(),
    );
    let edges = edges_json(&entries, &xrefs);
    let binary_path = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?;
    let edge_count = match &edges {
        Json::Array(values) => values.len(),
        _ => 0,
    };
    let document = Json::Object(vec![
        ("schemaVersion".into(), number(2)),
        (
            "binary".into(),
            Json::Object(vec![
                (
                    "name".into(),
                    Json::Str(
                        binary_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                    ),
                ),
                ("version".into(), Json::Str(args.version.clone())),
                (
                    "sourcePath".into(),
                    Json::Str(binary_path.to_string_lossy().into_owned()),
                ),
                (
                    "analysisImageBase".into(),
                    analysis_image_base(&file).map_or(Json::Null, number),
                ),
                ("sha256".into(), Json::Null),
                ("functionCount".into(), number(entries.len() as u64)),
                ("edgeCount".into(), number(edge_count as u64)),
            ]),
        ),
        ("functions".into(), functions),
        ("edges".into(), edges),
    ]);
    Ok(format!("{}\n", dumps_indent2(&document)))
}

fn function_kind(prog: &ConsoleProgram, entry: &FunctionEntry) -> &'static str {
    let mapped = prog.entry_bytes_mapped(&entry.addr);
    if mapped && prog.lone_jump_target(entry.addr.get_offset()).is_some() {
        "thunk"
    } else if entry.provenance == EntryProvenance::UndefinedExternal {
        "external"
    } else if !mapped {
        "import"
    } else {
        "normal"
    }
}

fn assembly(prog: &ConsoleProgram, entry: &FunctionEntry) -> Option<String> {
    if entry.size == 0 {
        return None;
    }
    let mut out = String::new();
    let mut address = entry.addr.get_offset();
    let end = address.checked_add(entry.size)?;
    let mut mnemonic = String::new();
    let mut body = String::new();
    while address < end {
        let length = prog
            .disassemble_at_into(address, &mut mnemonic, &mut body)
            .ok()?;
        if length <= 0 {
            return None;
        }
        let _ = writeln!(out, "{address:08x}  {mnemonic} {body}");
        address = address.checked_add(length as u64)?;
    }
    Some(out.trim_end().to_string())
}

fn parameters(result: Option<&FuncResult>) -> Json {
    let mut variables: Vec<_> = result
        .into_iter()
        .flat_map(|result| result.variables.iter())
        .filter(|variable| variable.is_param)
        .collect();
    variables.sort_by_key(|variable| variable.arg_index.unwrap_or(usize::MAX));
    Json::Array(
        variables
            .into_iter()
            .enumerate()
            .map(|(ordinal, variable)| {
                Json::Object(vec![
                    ("ordinal".into(), number(ordinal as u64)),
                    ("name".into(), Json::Str(variable.name.clone())),
                    ("type".into(), Json::Str(variable.type_name.clone())),
                ])
            })
            .collect(),
    )
}

fn edges_json(entries: &[FunctionEntry], xrefs: &kuna_analysis::listing::xrefs::XrefIndex) -> Json {
    let known: BTreeSet<u64> = entries
        .iter()
        .map(|entry| entry.addr.get_offset())
        .collect();
    let mut edges = Vec::new();
    for caller in &known {
        let mut references: Vec<_> = xrefs
            .refs_from_function(*caller)
            .into_iter()
            .filter(|reference| matches!(reference.kind, XrefKind::Call | XrefKind::Jump))
            .collect();
        references.sort_by(|left, right| {
            left.from
                .cmp(&right.from)
                .then(left.to.cmp(&right.to))
                .then(left.kind.cmp(&right.kind))
        });
        let mut targets = BTreeSet::new();
        for reference in references {
            targets.insert((reference.from, reference.to));
        }
        let mut seen_callees = BTreeSet::new();
        for callee in targets
            .into_iter()
            .filter_map(|(_, callee)| seen_callees.insert(callee).then_some(callee))
            .enumerate()
        {
            let (order, callee) = callee;
            edges.push(Json::Object(vec![
                ("callerAddress".into(), number(*caller)),
                ("calleeAddress".into(), number(callee)),
                ("calleeModule".into(), Json::Null),
                ("calleeOrder".into(), number(order as u64)),
            ]));
        }
    }
    Json::Array(edges)
}

fn is_entry_point(file: &object::File, entry: &FunctionEntry) -> bool {
    entry.addr.get_offset() == file.entry()
        || matches!(
            entry.name.as_str(),
            "main" | "WinMain" | "DllMain" | "entry" | "_start"
        )
}

fn number(value: u64) -> Json {
    Json::Number(value.to_string())
}

/// The static image base in the same address space as the exported function
/// VMAs. PE has an explicit optional-header ImageBase; other linked formats use
/// the lowest loadable segment VMA. Relocatable inputs have no static image
/// base, so their synthetic loader addresses deliberately yield `null`.
fn analysis_image_base(file: &object::File) -> Option<u64> {
    let pe_base = file.relative_address_base();
    if pe_base != 0 {
        return Some(pe_base);
    }
    file.segments()
        .filter(|segment| segment.size() != 0)
        .map(|segment| segment.address())
        .min()
}

fn parse_args(argv: &[String]) -> Result<ExportArgs, String> {
    let mut binary = None;
    let mut version = String::new();
    let mut output = None;
    let mut max_fn_seconds = None;
    let mut options = Vec::new();
    let mut mode = None;
    let mut slice = None;
    let mut target = None;
    let mut sleighpath = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                output = Some(argv.get(i).ok_or("--output requires a value")?.clone());
            }
            "--max-fn-seconds" => {
                i += 1;
                max_fn_seconds = Some(
                    argv.get(i)
                        .ok_or("--max-fn-seconds requires a value")?
                        .parse()
                        .map_err(|_| "invalid --max-fn-seconds value")?,
                );
            }
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            "--mode" => {
                i += 1;
                mode = Some(argv.get(i).ok_or("--mode requires a value")?.clone());
            }
            "--slice" => {
                i += 1;
                slice = Some(argv.get(i).ok_or("--slice requires a value")?.clone());
            }
            "--target" => {
                i += 1;
                target = Some(argv.get(i).ok_or("--target requires a value")?.clone());
            }
            "--sleighpath" => {
                i += 1;
                sleighpath = Some(argv.get(i).ok_or("--sleighpath requires a value")?.clone());
            }
            "-h" | "--help" => {
                usage();
                std::process::exit(0);
            }
            value if value.starts_with('-') => return Err(format!("unknown option {value}")),
            value if binary.is_none() => binary = Some(value.to_string()),
            value if version.is_empty() => version = value.to_string(),
            value => return Err(format!("unexpected argument {value:?}")),
        }
        i += 1;
    }
    Ok(ExportArgs {
        binary: binary.ok_or("decompile-graph requires <binary>")?,
        version,
        output,
        max_fn_seconds,
        options,
        mode,
        slice,
        target,
        sleighpath,
    })
}

fn usage() {
    eprintln!("usage: kuna decompile-graph <binary> [version] [-o|--output FILE] [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] [--option N V]... [--slice ARCH] [--target T] [--sleighpath D]");
}
