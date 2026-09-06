//! `kuna decompile-graph` — the whole program as one JSON document: every
//! discovered function with its C, its assembly and its recovered signature,
//! plus the call edges between them.
//!
//! ```text
//!   kuna decompile-graph <binary> [-o|--output FILE] [--label TEXT]
//!                        [--functions a,b,..] [--addr 0xVMA].. [--max-fn-seconds N]
//!                        [--mode auto|reliable|aggressive|fast]
//!                        [--define-function S[-E][=N]|@FILE].. [--option N V]..
//!                        [--slice ARCH] [--target T] [--sleighpath D]
//! ```
//!
//! One in-process load, like `decompile-all` and `decompile-project` — and the
//! same policies, read from the same helpers rather than restated here:
//!
//! * which entries exist, and which of them have bodies worth decompiling
//!   ([`resolve_targets`], i.e. `function_entries_executable`);
//! * what a function *is* ([`Classifier`], the per-function `kind` the browser
//!   inventory already labels with);
//! * what calls what ([`CallGraph`], the edge model `--reachable-from` walks and
//!   `kuna xrefs` answers with);
//! * how a body disassembles ([`crate::disassemble::function_listing`]).
//!
//! `address` is the only key: a name can repeat inside one document, because a
//! PLT thunk, an import slot and the callable they name are three addresses
//! under one name.
//!
//! The document is observational: it adds no analysis facts, mutates no IR and
//! changes no existing C rendering, so it has neither an option row nor a DIV
//! entry. Field-by-field schema: `docs/cli.md`.

use std::collections::{BTreeMap, BTreeSet};

use kuna_console::classify::Classifier;
use kuna_console::engine::{ConsoleProgram, EntryProvenance, FunctionEntry};
use kuna_console::project::{decompile_targets, FuncResult};
use object::{Object, ObjectSegment};

use crate::decompile_all::{
    load_program, parse_args, resolve_targets, Args, CallGraph, DriverDefaults,
};
use crate::jsonfmt::{dumps_indent2, Json};

/// The document shape. Bumped whenever a field is added, removed or changes
/// meaning, so a consumer can refuse a document it does not understand.
const SCHEMA_VERSION: u64 = 4;

/// `kuna decompile-graph` entry point.
///
/// A wrapper parse for the two flags only this surface has, then the shared
/// `decompile-all` parser for everything else — the same split
/// `decompile-project` uses, so the load/decompile flags cannot drift between
/// the whole-binary surfaces.
pub fn run(argv: &[String]) -> i32 {
    let mut output: Option<String> = None;
    let mut label = String::new();
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            flag @ ("-o" | "--output" | "--label") => {
                if i + 1 >= argv.len() {
                    eprintln!("error: {flag} requires a value");
                    usage();
                    return 2;
                }
                if flag == "--label" {
                    label = argv[i + 1].clone();
                } else {
                    output = Some(argv[i + 1].clone());
                }
                i += 1;
            }
            "-h" | "--help" => {
                usage();
                return 0;
            }
            "--json" | "--no-vars" => {
                eprintln!("error: {} is not a decompile-graph option", argv[i]);
                usage();
                return 2;
            }
            other => rest.push(other.to_string()),
        }
        i += 1;
    }
    let args = match parse_args(&rest, "decompile-graph") {
        Ok(args) => args,
        Err(error) => {
            eprintln!("error: {error}");
            usage();
            return 2;
        }
    };
    match export(&args, &label) {
        Ok(text) => match &output {
            Some(path) => match std::fs::write(path, &text) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("error: cannot write {path}: {error}");
                    1
                }
            },
            None => crate::output::emit_with_status(&text, 0),
        },
        Err(error) => {
            eprintln!("error: {error}");
            1
        }
    }
}

/// Load once, decompile the selected bodies, walk the program once for edges,
/// and render.
///
/// Nodes are the whole inventory and bodies are only what the whole-binary
/// target policy admits, so a `--functions` / `--addr` narrowing buys a cheap
/// whole-program graph with just the named bodies in it, and never a body for an
/// address that is not executable content — the row's `kind` says so, and an
/// explicit selection does not buy an exception.
///
/// (kuna outlang) C-only, for the reason `decompile-project` is: honouring
/// `--language rust` would put Rust in a field called `codeC`. The auto policy
/// is off for this command too (`parse_args_with_filters`), so a rustc-built
/// binary does not trip the refusal on its own.
fn export(args: &Args, label: &str) -> Result<String, String> {
    let binary_path = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?;
    let mut prog = load_program(args, DriverDefaults::Decompile)?;
    if prog.arch().print().get_name() != "c-language" {
        return Err(format!(
            "the graph document is C-only (got {}); use `kuna decompile` or \
             `kuna decompile-all --json` for other output languages",
            prog.arch().print().get_name()
        ));
    }
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }

    let entries = prog.function_entries_canonical();
    // What a row IS never depends on what this run was asked to decompile, so
    // the executable set comes from the policy, not from the results.
    let executable: BTreeSet<u64> =
        prog.function_entries_executable().iter().map(|e| e.addr.get_offset()).collect();
    let mut targets = resolve_targets(&prog, args)?;
    targets.retain(|entry| {
        let selected = executable.contains(&entry.addr.get_offset());
        if !selected {
            eprintln!(
                "warning: {} @ 0x{:x} is not executable content; exported without a body",
                entry.name,
                entry.addr.get_offset()
            );
        }
        selected
    });
    let results = decompile_targets(
        &mut prog,
        targets,
        /* no_vars= */ false,
        /* want_proto= */ true,
        /* want_provenance= */ false,
    );
    for result in &results {
        if let Some(error) = &result.error {
            eprintln!(
                "warning: could not decompile {} @ 0x{:x}: {error}",
                result.name, result.address
            );
        }
    }
    let by_address: BTreeMap<u64, &FuncResult> =
        results.iter().map(|result| (result.address, result)).collect();

    let bytes = kuna_analysis::loader::elf_shdr::read_image(&args.binary)
        .map_err(|error| format!("{}: {error}", args.binary))?;
    let file = object::File::parse(&*bytes)
        .map_err(|error| format!("could not parse {}: {error}", args.binary))?;
    let graph = CallGraph::build_from(&prog, &file);
    let classifier =
        Classifier::from_object(&prog, Some(&file), entries.iter().map(|e| e.addr.get_offset()));
    let known: BTreeSet<u64> = entries.iter().map(|e| e.addr.get_offset()).collect();

    // Through the inventory, because an ARM ELF stores the Thumb mode bit in
    // `e_entry` (`0x100d7` for a `_start` reported at `0x100d6`); and through
    // `image_entry_vma`, because a Mach-O `LC_MAIN` states its entry as a
    // `__TEXT`-relative file offset, which roots the walk at no function at all.
    let image_entry: Option<u64> = kuna_analysis::analyzers::entry::image_entry_vma(&file, &bytes)
        .map(|vma| prog.find_entry_at(vma).map_or(vma, |e| e.addr.get_offset()));

    let functions = Json::Array(
        entries
            .iter()
            .map(|entry| {
                function_json(
                    &prog,
                    &classifier,
                    &graph,
                    entry,
                    by_address.get(&entry.addr.get_offset()).copied(),
                    executable.contains(&entry.addr.get_offset()),
                    image_entry,
                )
            })
            .collect(),
    );
    let edges = edges_json(&graph, &known);
    let edge_count = match &edges {
        Json::Array(values) => values.len(),
        _ => 0,
    };

    let document = Json::Object(vec![
        ("schemaVersion".into(), number(SCHEMA_VERSION)),
        (
            "binary".into(),
            Json::Object(vec![
                (
                    "name".into(),
                    Json::Str(
                        binary_path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                    ),
                ),
                ("label".into(), Json::Str(label.to_string())),
                ("sourcePath".into(), Json::Str(binary_path.to_string_lossy().into_owned())),
                ("analysisImageBase".into(), analysis_image_base(&file).map_or(Json::Null, number)),
                ("functionCount".into(), number(entries.len() as u64)),
                ("edgeCount".into(), number(edge_count as u64)),
            ]),
        ),
        ("functions".into(), functions),
        ("edges".into(), edges),
    ]);
    Ok(format!("{}\n", dumps_indent2(&document)))
}

/// One function row, keyed by `address`: `name` is not unique in the document,
/// since a thunk, the pointer slot it forwards through and the callable they
/// stand for are three rows under one name.
///
/// `codeC` is `null` with a non-null `error` when the decompile failed, and
/// `null` with a `null` error when no body was attempted — a bodyless `kind`, or
/// a target list that did not select this entry. `assembly` follows the
/// *attempt*, not the outcome: a function whose C failed still has bytes, and
/// the listing is what is left to look at — an entry whose extent could not be
/// measured lists its first instruction rather than nothing.
fn function_json(
    prog: &ConsoleProgram,
    classifier: &Classifier,
    graph: &CallGraph,
    entry: &FunctionEntry,
    result: Option<&FuncResult>,
    executable: bool,
    image_entry: Option<u64>,
) -> Json {
    let address = entry.addr.get_offset();
    let kind = function_kind(prog, classifier, entry, executable);
    let code = result.and_then(|r| r.code.as_ref());
    Json::Object(vec![
        ("address".into(), number(address)),
        ("name".into(), Json::Str(entry.name.clone())),
        ("size".into(), number(entry.size)),
        ("kind".into(), Json::Str(kind.into())),
        ("parameters".into(), parameters(result)),
        (
            "signature".into(),
            result.and_then(|r| r.proto.as_ref()).map_or(Json::Null, |value| {
                Json::Str(value.trim().trim_end_matches(';').to_string())
            }),
        ),
        (
            "assembly".into(),
            match result {
                Some(_) => crate::disassemble::function_listing(
                    prog,
                    address,
                    address.saturating_add(entry.size.max(1)),
                )
                .map_or(Json::Null, Json::Str),
                None => Json::Null,
            },
        ),
        ("codeC".into(), code.map_or(Json::Null, |value| Json::Str(value.clone()))),
        (
            "error".into(),
            result
                .and_then(|r| r.error.as_ref())
                .map_or(Json::Null, |value| Json::Str(value.clone())),
        ),
        ("hasIndirectCalls".into(), Json::Bool(graph.has_indirect_calls(address))),
        ("forwardsTo".into(), forwards_to(prog, graph, address).map_or(Json::Null, number)),
        ("isEntryPoint".into(), Json::Bool(image_entry == Some(address))),
    ])
}

/// What the row is, in this document's five-value vocabulary. The first three
/// are the rows with no body: this surface never decompiles an address that is
/// not executable content, not even a named one, and each of those is named
/// rather than lifted out of whatever bytes happen to be there.
///
/// * `external` — a loader-defined undefined symbol: the definition is in
///   another module and there are no bytes here at all.
/// * `import` — a pointer slot the program calls through: a PE `.idata` entry,
///   a Mach-O `__got` / stub slot. It carries the imported name so a call to it
///   renders.
/// * `data` — any other named address that is not code: a Mach header symbol,
///   an Objective-C class object. It reached the callable inventory as a symbol,
///   not as a function.
/// * `thunk` — a body that only forwards: a PLT/stub-section entry, an imported
///   name, or a lone jump ([`Classifier`]).
/// * `normal` — a body of its own.
fn function_kind(
    prog: &ConsoleProgram,
    classifier: &Classifier,
    entry: &FunctionEntry,
    executable: bool,
) -> &'static str {
    if !prog.entry_bytes_mapped(&entry.addr)
        && entry.provenance == EntryProvenance::UndefinedExternal
    {
        return "external";
    }
    let shape = classifier.kind(prog, &entry.name, entry.addr.get_offset());
    match (executable, shape) {
        (false, "plt") => "import",
        (false, _) => "data",
        (true, "plt" | "thunk") => "thunk",
        (true, _) => "normal",
    }
}

/// Where a forwarding entry sends control: the destination of a direct lone
/// jump, or the fixed pointer slot an indirect one (`jmp [slot]`) reads.
///
/// The slot half needs the jump to name it as a decode-time constant, which is
/// what an x86 `jmp [rip+disp]` / `FF 25` stub does. An AArch64 stub computes it
/// (`adrp x16, page; ldr x16, [x16]; br x16`), so a Mach-O `__stubs` entry is
/// `kind = thunk` with no `forwardsTo` — the import slot is still its own row,
/// reached by name.
fn forwards_to(prog: &ConsoleProgram, graph: &CallGraph, address: u64) -> Option<u64> {
    match prog.lone_jump_target(address) {
        Some(Some(target)) => Some(target),
        Some(None) => graph.veneer_slot(address),
        None => None,
    }
}

/// The recovered parameters, in ABI order.
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

/// The edge list: every caller in address order, each one's callees in
/// reference order with a contiguous zero-based `calleeOrder`.
///
/// `kind` is the `kuna xrefs` vocabulary, so the document's edges and that
/// command's rows cannot disagree: `call` a call site, `jump` a tail call or a
/// branch into a neighbouring entry, `data` an address handed to someone else to
/// call (the edge that gives `main` a caller).
///
/// Both endpoints are always rows in `functions` — [`CallGraph::callees_of`]
/// resolves a target onto the inventory and drops what lands nowhere — so a
/// consumer can build the graph without a containment test of its own. `known`
/// is the row set this document actually rendered, which is the snapshot taken
/// before the decompile loop ran; filtering against it is what keeps the
/// guarantee if a pass names a function while decompiling.
fn edges_json(graph: &CallGraph, known: &BTreeSet<u64>) -> Json {
    let mut edges = Vec::new();
    for &caller in known {
        for (order, (callee, kind)) in graph
            .callees_of(caller)
            .into_iter()
            .filter(|(callee, _)| known.contains(callee))
            .enumerate()
        {
            edges.push(Json::Object(vec![
                ("callerAddress".into(), number(caller)),
                ("calleeAddress".into(), number(callee)),
                ("kind".into(), Json::Str(kind.as_str().into())),
                ("calleeOrder".into(), number(order as u64)),
            ]));
        }
    }
    Json::Array(edges)
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
    file.segments().filter(|segment| segment.size() != 0).map(|segment| segment.address()).min()
}

fn usage() {
    eprintln!(
        "usage: kuna decompile-graph <binary> [-o|--output FILE] [--label TEXT] \\\n\
         \x20                   [--functions a,b,..] [--addr 0xVMA].. [--max-fn-seconds N] \\\n\
         \x20                   [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                   [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                   [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         The whole program as one JSON document (schema 4): every discovered\n\
         function with its recovered signature, parameters, C and assembly, plus\n\
         the call edges between them.  Written to stdout, or to -o FILE.\n\
         C only: the document has no other output language.  `address` is the\n\
         key, not `name` -- a thunk and the import it forwards to share a name.\n\
         --label TEXT is copied verbatim into `binary.label`, for a consumer that\n\
         wants to stamp the document with its own version.\n\
         Every function is a node; --functions/--addr narrow which of them are\n\
         decompiled, not which appear.  Unfiltered fast runs default to 10 seconds\n\
         per function, other runs to 120; a function over budget becomes its own\n\
         `error` record and the run still exits 0."
    );
}
