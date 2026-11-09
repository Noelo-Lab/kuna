//! Write kuna's pass trace for class files; the command line is the Java tracer's
//! (`tools/vftrace/run.sh`, which forwards to the corpus oracle's `VfDriver`):
//!
//! ```text
//! cargo run -p kuna-jvm --example jvmtrace -- --trace OUT [--filter S] [--initial-only]
//!     [--fixture] [--opt k=v]... [--lib PATH]... [--inputs-from FILE] INPUT... [-o OUTDIR]
//! ```
//!
//! `--fixture` starts from the corpus fixture's base option map (`DecompilerTestFixture.setUp`),
//! `--opt` pairs overlay it in order. The default runs the whole decompiler with tracing on
//! (decompiled classes go to `-o` when given). `--initial-only` instead runs
//! `MethodProcessor.codeToJava` up to `DomHelper.parseGraph` on every method of every input
//! class and records the "Initial" step, for comparing the parts of the pipeline that are ported.
//! `--hash`, `--shift`, `--javadoc`, `--plugins` and `--save` are accepted and ignored.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::rc::Rc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::control_flow_graph::ControlFlowGraph;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::dead_code_helper::DeadCodeHelper;
use kuna_jvm::decompile_record::DecompileRecord;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dom_helper::DomHelper;
use kuna_jvm::exception_deobfuscator::ExceptionDeobfuscator;
use kuna_jvm::fernflower::Fernflower;
use kuna_jvm::i_fernflower_logger::PrintStreamLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::kuna_trace;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::try_catch;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_processor::VarProcessor;

const FIXTURE_BASE: [(&str, &str); 9] = [
    ("log-level", "warn"),
    ("decompile-generics", "1"),
    ("remove-synthetic", "1"),
    ("remove-bridge", "1"),
    ("keep-literals", "1"),
    ("__unit_test_mode__", "1"),
    ("new-line-separator", "1"),
    ("error-message", ""),
    ("include-runtime", ""),
];

struct Args {
    trace: PathBuf,
    filter: Option<String>,
    initial_only: bool,
    options: Vec<(String, String)>,
    libs: Vec<PathBuf>,
    inputs: Vec<PathBuf>,
    out: Option<PathBuf>,
}

fn usage(msg: &str) -> ! {
    eprintln!("error: {msg}");
    eprintln!(
        "usage: jvmtrace --trace OUT [--filter S] [--initial-only] [--fixture] [--opt k=v]... \
         [--lib PATH]... [--inputs-from FILE] INPUT... [-o OUTDIR]"
    );
    std::process::exit(64);
}

fn parse_args() -> Args {
    let mut trace = None;
    let mut filter = None;
    let mut initial_only = false;
    let mut fixture = false;
    let mut pairs = Vec::new();
    let mut libs = Vec::new();
    let mut inputs = Vec::new();
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut value = |what: &str| {
            args.next()
                .unwrap_or_else(|| usage(&format!("{a} needs {what}")))
        };
        match a.as_str() {
            "--trace" => trace = Some(PathBuf::from(value("a file"))),
            "--filter" => filter = Some(value("a substring")),
            "--initial-only" => initial_only = true,
            "--fixture" => fixture = true,
            "--opt" => pairs.push(value("k=v")),
            "--lib" => libs.push(PathBuf::from(value("a path"))),
            "--inputs-from" => {
                let list = value("a file");
                let text = std::fs::read_to_string(&list)
                    .unwrap_or_else(|e| usage(&format!("{list}: {e}")));
                inputs.extend(text.lines().filter(|l| !l.is_empty()).map(PathBuf::from));
            }
            "-o" => out = Some(PathBuf::from(value("a directory"))),
            "--hash" | "--shift" | "--javadoc" | "--save" => {
                value("a value");
            }
            "--plugins" | "--print-options" => {}
            _ if a.starts_with("--opt=") => pairs.push(a["--opt=".len()..].to_string()),
            _ if a.starts_with("--") => usage(&format!("unknown flag {a}")),
            _ => inputs.push(PathBuf::from(a)),
        }
    }
    let trace = trace.unwrap_or_else(|| usage("missing --trace OUT"));
    if inputs.is_empty() {
        usage("no inputs");
    }
    let mut options: Vec<(String, String)> = Vec::new();
    if fixture {
        options.extend(
            FIXTURE_BASE
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
    }
    for kv in pairs {
        match kv.split_once('=') {
            Some((k, v)) if !k.is_empty() => options.push((k.to_string(), v.to_string())),
            _ => usage(&format!("--opt expects k=v, got {kv}")),
        }
    }
    Args {
        trace,
        filter,
        initial_only,
        options,
        libs,
        inputs,
        out,
    }
}

/// `MethodProcessor.codeToJava` from `expandData` through `DomHelper.parseGraph`.
fn initial_statement(cl: StructClass, mt: StructMethod) -> RootStatement {
    mt.expand_data(cl);
    let seq = mt.get_instruction_sequence().expect("expanded method");
    let graph = ControlFlowGraph::new(seq);
    DecompilerContext::get_counter_container()
        .set_counter(CounterContainer::VAR_COUNTER, mt.get_local_variables());
    DeadCodeHelper::remove_dead_blocks(graph);
    if mt.get_bytecode_version().has_jsr()
        || DecompilerContext::get_option(IFernflowerPreferences::FORCE_JSR_INLINE)
    {
        graph.inline_jsr(cl, mt);
    }
    DeadCodeHelper::connect_dummy_exit_block(graph);
    DeadCodeHelper::remove_gotos(graph);
    ExceptionDeobfuscator::remove_circular_ranges(graph);
    ExceptionDeobfuscator::restore_pop_ranges(graph);
    if DecompilerContext::get_option(IFernflowerPreferences::REMOVE_EMPTY_RANGES) {
        ExceptionDeobfuscator::remove_empty_ranges(graph);
    }
    if DecompilerContext::get_option(IFernflowerPreferences::ENSURE_SYNCHRONIZED_MONITOR) {
        DeadCodeHelper::extend_synchronized_range_to_monitorexit(graph);
    }
    if DecompilerContext::get_option(IFernflowerPreferences::INCORPORATE_RETURNS) {
        DeadCodeHelper::incorporate_value_returns(graph);
    }
    ExceptionDeobfuscator::insert_empty_exception_handler_blocks(graph);
    DeadCodeHelper::merge_basic_blocks(graph);
    if ExceptionDeobfuscator::has_obfuscated_exceptions(graph) {
        if !ExceptionDeobfuscator::handle_multiple_entry_exception_ranges(graph) {
            graph.add_comment("$VF: Could not handle exception ranges with multiple entries");
            graph.with_mut(|d| d.add_error_comment = true);
        }
        ExceptionDeobfuscator::insert_dummy_exception_handler_blocks(
            graph,
            mt.get_bytecode_version(),
        );
    }
    DomHelper::parse_graph(graph, mt, 0)
}

fn trace_initial(inputs: &[PathBuf]) -> (usize, usize) {
    let (mut traced, mut failed) = (0, 0);
    for path in inputs {
        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                failed += 1;
                continue;
            }
        };
        let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
        DecompilerContext::set_current_class(Some(cl));
        let methods: Vec<StructMethod> = cl.get_methods().iter().copied().collect();
        for mt in methods {
            if !mt.contains_code() {
                continue;
            }
            DecompilerContext::start_method(VarProcessor::new(mt, &mt.method_descriptor()));
            match try_catch(|| initial_statement(cl, mt)) {
                Ok(root) => {
                    DecompileRecord::new(mt).add("Initial", Some(root));
                    traced += 1;
                }
                Err(e) => {
                    eprintln!(
                        "{}.{}{}: {}",
                        cl.qualified_name(),
                        mt.get_name(),
                        mt.get_descriptor(),
                        e.message
                    );
                    failed += 1;
                }
            }
        }
        DecompilerContext::set_current_class(None);
    }
    (traced, failed)
}

fn write_class(out: &Path, path: &str, content: &str) {
    let dest = out.join(path);
    if let Some(parent) = dest.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&dest, content) {
        eprintln!("{}: {e}", dest.display());
    }
}

fn main() -> ExitCode {
    let args = parse_args();
    let mut fernflower = Fernflower::new(&args.options, Rc::new(PrintStreamLogger::stderr()));
    for lib in &args.libs {
        fernflower.add_library_file(lib);
    }
    for input in &args.inputs {
        fernflower.add_source_file(input);
    }
    if let Err(e) = kuna_trace::start(&args.trace, args.filter.as_deref()) {
        eprintln!("{}: {e}", args.trace.display());
        return ExitCode::from(1);
    }
    let started = std::time::Instant::now();
    let mut status = ExitCode::SUCCESS;
    if args.initial_only {
        let (traced, failed) = trace_initial(&args.inputs);
        eprintln!("jvmtrace: Initial traced for {traced} methods, {failed} failed");
    } else {
        let out = args.out.clone();
        let result = try_catch(|| {
            fernflower.decompile_context(&mut |class| {
                if let Some(out) = &out {
                    write_class(out, &class.path, &class.content);
                }
            })
        });
        if let Err(e) = result {
            eprintln!("jvmtrace: decompilation stopped: {}", e.message);
            status = ExitCode::from(2);
        }
    }
    match kuna_trace::finish() {
        Some(s) => eprintln!(
            "jvmtrace: {} methods, {} steps, {} bytes, {} ms -> {}",
            s.methods,
            s.steps,
            s.bytes,
            started.elapsed().as_millis(),
            s.path.display()
        ),
        None => eprintln!("jvmtrace: tracing was not active"),
    }
    status
}
