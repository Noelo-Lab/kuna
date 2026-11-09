//! Differential CFG tests against Vineflower 1.12.0.
//!
//! `tests/data/cfg/gen.sh` runs `CfgDump.java` on the real Vineflower jar (with the patched,
//! deterministic `BasicBlock.hashCode`) and records, per method, the parsed instructions and the
//! graph after construction (`built`), after goto removal (`pre`) and right before
//! `DomHelper.parseGraph` (`final`), plus the final reverse post-order and immediate dominators.
//! The replay test rebuilds each `FullInstructionSequence` from the dump, runs the same steps as
//! `MethodProcessor.codeToJava` and compares the dumps text for text.

use std::io::Read;

use kuna_jvm::basic_block::BasicBlock;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::bytecode_version::BytecodeVersion;
use kuna_jvm::control_flow_graph::ControlFlowGraph;
use kuna_jvm::dead_code_helper::DeadCodeHelper;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::exception_deobfuscator::ExceptionDeobfuscator;
use kuna_jvm::exception_handler::ExceptionHandler;
use kuna_jvm::exception_table::ExceptionTable;
use kuna_jvm::full_instruction_sequence::FullInstructionSequence;
use kuna_jvm::generic_dominator_engine::GenericDominatorEngine;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::i_graph::IGraph;
use kuna_jvm::instruction::Instruction;
use kuna_jvm::rt::{reset_class_scope, try_catch, JHashMap, JHashSet};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;

const OPC_JSR: i32 = 168;
const OPC_RET: i32 = 169;
const OPC_JSR_W: i32 = 201;

struct Insn {
    start_offset: i32,
    length: i32,
    opcode: i32,
    group: i32,
    wide: bool,
    operands: Option<Vec<i32>>,
}

#[derive(Default)]
struct Case {
    header: String,
    class: String,
    name: String,
    descriptor: String,
    version: (i32, i32),
    jsr: bool,
    pool: bool,
    insns: Vec<Insn>,
    handlers: Vec<ExceptionHandler>,
    stages: Vec<(String, String)>,
    tail: String,
    error: Option<(String, String, String)>,
}

impl Case {
    fn has_jsr_instructions(&self) -> bool {
        self.insns
            .iter()
            .any(|i| i.opcode == OPC_JSR || i.opcode == OPC_RET || i.opcode == OPC_JSR_W)
    }
}

fn read_golden(name: &str) -> String {
    let path = format!(
        "{}/tests/data/cfg/golden/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    if name.ends_with(".gz") {
        let mut text = String::new();
        flate2::read::GzDecoder::new(&bytes[..])
            .read_to_string(&mut text)
            .expect("gzip");
        text
    } else {
        String::from_utf8(bytes).expect("utf8")
    }
}

fn list(field: &str) -> Vec<i32> {
    if field == "-" {
        Vec::new()
    } else {
        field.split(',').map(|v| v.parse().expect("int")).collect()
    }
}

fn parse(text: &str) -> (String, Vec<Case>) {
    let mut lines = text.lines();
    let options = lines.next().expect("header").to_string();
    let mut cases = Vec::new();
    let mut case: Option<Case> = None;
    let mut stage: Option<(String, String)> = None;

    for line in lines {
        if let Some(rest) = line.strip_prefix("=method ") {
            let f: Vec<&str> = rest.split(' ').collect();
            let (major, minor) = f[3]
                .strip_prefix("v=")
                .expect("v=")
                .split_once('.')
                .expect(".");
            case = Some(Case {
                header: line.to_string(),
                class: f[0].to_string(),
                name: f[1].to_string(),
                descriptor: f[2].to_string(),
                version: (major.parse().unwrap(), minor.parse().unwrap()),
                jsr: f[4] == "jsr=1",
                pool: f[5] == "pool=1",
                ..Case::default()
            });
            continue;
        }
        let c = case.as_mut().expect("line outside a method");
        if line == "=end" {
            if let Some(s) = stage.take() {
                c.stages.push(s);
            }
            cases.push(case.take().unwrap());
        } else if let Some(name) = line.strip_prefix('@') {
            if let Some(s) = stage.take() {
                c.stages.push(s);
            }
            stage = Some((name.to_string(), String::new()));
        } else if let Some(rest) = line.strip_prefix("i ") {
            let f: Vec<&str> = rest.split(' ').collect();
            c.insns.push(Insn {
                start_offset: f[1].parse().unwrap(),
                length: f[2].parse().unwrap(),
                opcode: f[3].parse().unwrap(),
                group: f[4].parse().unwrap(),
                wide: f[5] == "1",
                operands: (f[6] != "-").then(|| list(f[6])),
            });
        } else if let Some(rest) = line.strip_prefix("x ") {
            let f: Vec<&str> = rest.split(' ').collect();
            c.handlers.push(ExceptionHandler::new(
                f[0].parse().unwrap(),
                f[1].parse().unwrap(),
                f[2].parse().unwrap(),
                (f[3] != "-").then(|| f[3].to_string()),
            ));
        } else if let Some(rest) = line.strip_prefix("!error ") {
            let (st, rest) = rest.split_once(' ').unwrap();
            let (class, msg) = rest.split_once(' ').unwrap_or((rest, ""));
            c.error = Some((st.to_string(), class.to_string(), msg.to_string()));
        } else if line.starts_with("c ")
            || line.starts_with("e ")
            || line.starts_with("rpo ")
            || line.starts_with("idom")
        {
            if let Some(s) = stage.take() {
                c.stages.push(s);
            }
            c.tail.push_str(line);
            c.tail.push('\n');
        } else {
            let s = stage.as_mut().expect("graph line outside a stage");
            s.1.push_str(line);
            s.1.push('\n');
        }
    }
    (options, cases)
}

fn ids(blocks: &[BasicBlock]) -> String {
    if blocks.is_empty() {
        return "-".to_string();
    }
    blocks
        .iter()
        .map(|b| b.get_id().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn ints(values: &[i32]) -> String {
    if values.is_empty() {
        return "-".to_string();
    }
    values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// The same text `CfgDump.dumpGraph` prints.
fn dump_graph(graph: ControlFlowGraph) -> String {
    let mut out = String::new();
    let blocks: Vec<BasicBlock> = graph.get_blocks().borrow().iter().copied().collect();
    for block in blocks {
        let seq = block.get_seq();
        let s = if seq.is_empty() {
            "-".to_string()
        } else {
            (0..seq.length())
                .map(|i| {
                    let instr = seq.get_instr(i);
                    instr.with(|d| format!("{}@{}", d.opcode, d.start_offset))
                })
                .collect::<Vec<_>>()
                .join(",")
        };
        out.push_str(&format!(
            "b {} s={} o={} S={} P={} E={} Q={} m={}\n",
            block.get_id(),
            s,
            ints(&block.get_instr_old_offsets().to_vec()),
            ids(&block.get_succs().to_vec()),
            ids(&block.get_preds().to_vec()),
            ids(&block.get_succ_exceptions().to_vec()),
            ids(&block.get_pred_exceptions().to_vec()),
            block.get_mark()
        ));
    }
    let last = graph.get_last();
    out.push_str(&format!(
        "f {} l {} L={} LS={} n={}\n",
        graph.get_first().get_id(),
        last.get_id(),
        ids(&last.get_preds().to_vec()),
        ids(&last.get_succs().to_vec()),
        graph.with(|d| d.last_id)
    ));
    for range in graph.get_exceptions().iter() {
        let types = match range.get_exception_types() {
            None => "null".to_string(),
            Some(t) if t.is_empty() => "-".to_string(),
            Some(t) => t.join("|"),
        };
        out.push_str(&format!(
            "r {} B={} T={}\n",
            range.get_handler().get_id(),
            ids(&range.get_protected_range().to_vec()),
            types
        ));
    }
    out.push_str(&format!(
        "F={}\n",
        ids(&graph.get_finally_exits().borrow().to_vec())
    ));
    out
}

struct CfgGraph(ControlFlowGraph);

impl IGraph<BasicBlock> for CfgGraph {
    fn get_reverse_post_order_list(&self) -> Vec<BasicBlock> {
        self.0.get_reverse_post_order()
    }

    fn get_roots(&self) -> JHashSet<BasicBlock> {
        JHashSet::from_collection(vec![self.0.get_first()])
    }
}

/// Checks the oracle's immediate dominators through the public `is_dominator`.
fn check_dominators(graph: ControlFlowGraph, idom_line: &str) -> Result<(), String> {
    let mut idom: JHashMap<i32, i32> = JHashMap::new();
    for pair in idom_line.split(' ').skip(1) {
        let (b, d) = pair.split_once('=').unwrap();
        idom.put(b.parse().unwrap(), d.parse().unwrap());
    }
    let rpo = graph.get_reverse_post_order();
    let by_id: JHashMap<i32, BasicBlock> =
        JHashMap::from_entries(rpo.iter().map(|b| (b.get_id(), *b)));
    let mut engine = GenericDominatorEngine::new(Box::new(CfgGraph(graph)));
    engine.initialize();

    let exhaustive = rpo.len() <= 120;
    for &block in &rpo {
        let mut chain = vec![block.get_id()];
        let mut cur = block.get_id();
        loop {
            let next = idom
                .get_cloned(&cur)
                .ok_or_else(|| format!("no idom for {cur}"))?;
            if next == cur {
                break;
            }
            chain.push(next);
            cur = next;
        }
        for &d in &chain {
            if !engine.is_dominator(block, by_id.get_cloned(&d).unwrap()) {
                return Err(format!("{} should be dominated by {}", block.get_id(), d));
            }
        }
        if exhaustive {
            for &other in &rpo {
                if !chain.contains(&other.get_id()) && engine.is_dominator(block, other) {
                    return Err(format!(
                        "{} should not be dominated by {}",
                        block.get_id(),
                        other.get_id()
                    ));
                }
            }
        }
    }
    Ok(())
}

fn build_sequence(case: &Case) -> FullInstructionSequence {
    let version = BytecodeVersion::new(case.version.0, case.version.1);
    let mut instructions = Vec::new();
    let mut offset_to_index: JHashMap<i32, i32> = JHashMap::new();
    for insn in &case.insns {
        let instr = Instruction::create(
            insn.opcode,
            insn.wide,
            insn.group,
            version,
            insn.operands.clone(),
            insn.start_offset,
            insn.length,
        );
        offset_to_index.put(insn.start_offset, instructions.len() as i32);
        instructions.push(instr);
    }
    let seq = FullInstructionSequence::new(
        instructions,
        offset_to_index,
        ExceptionTable::new(case.handlers.clone()),
    );
    for i in 0..seq.length() {
        seq.get_instr(i).init_instruction(seq);
    }
    seq
}

#[derive(Default)]
struct Outcome {
    stages: Vec<(String, String)>,
    tail: String,
    error: Option<(String, String)>,
}

/// How the jsr inlining step of `codeToJava` runs.
enum JsrPass {
    /// The class predates Java 7 but has no jsr/ret: `inlineJsr` then only resets the markers
    /// and removes empty blocks (its type walk removes nothing but return-address stores).
    Reduced,
    /// The real `inlineJsr`, which needs the parsed class.
    Real(StructClass, StructMethod),
}

/// `MethodProcessor.codeToJava` up to `DomHelper.parseGraph`.
fn run_pipeline(
    case: &Case,
    seq: impl FnOnce() -> FullInstructionSequence,
    jsr: Option<JsrPass>,
) -> Outcome {
    let mut out = Outcome::default();

    let graph = match try_catch(|| ControlFlowGraph::new(seq())) {
        Ok(graph) => graph,
        Err(e) => {
            out.error = Some(("built".into(), e.message));
            return out;
        }
    };
    out.stages.push(("built".into(), dump_graph(graph)));

    let pre = try_catch(|| {
        DeadCodeHelper::remove_dead_blocks(graph);
        match jsr {
            None => {}
            Some(JsrPass::Reduced) => {
                graph.remove_markers();
                DeadCodeHelper::remove_empty_blocks(graph);
            }
            Some(JsrPass::Real(cl, mt)) => graph.inline_jsr(cl, mt),
        }
        DeadCodeHelper::connect_dummy_exit_block(graph);
        DeadCodeHelper::remove_gotos(graph);
    });
    if let Err(e) = pre {
        out.error = Some(("pre".into(), e.message));
        return out;
    }
    out.stages.push(("pre".into(), dump_graph(graph)));

    let fin = try_catch(|| {
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
                BytecodeVersion::new(case.version.0, case.version.1),
            );
        }
    });
    if let Err(e) = fin {
        out.error = Some(("final".into(), e.message));
        return out;
    }
    out.stages.push(("final".into(), dump_graph(graph)));

    if let Some(comments) = graph.with(|d| d.comment_lines.clone()) {
        for c in comments.to_vec() {
            out.tail.push_str(&format!("c {c}\n"));
        }
    }
    let error_comment = graph.with(|d| d.add_error_comment);
    out.tail
        .push_str(&format!("e {}\n", i32::from(error_comment)));

    let idom_line = case
        .tail
        .lines()
        .find(|l| l.starts_with("idom"))
        .unwrap_or("idom");
    let dom = try_catch(|| {
        let rpo = ids(&graph.get_reverse_post_order());
        (rpo, check_dominators(graph, idom_line))
    });
    match dom {
        Err(e) => out.error = Some(("dom".into(), e.message)),
        Ok((rpo, checked)) => {
            out.tail.push_str(&format!("rpo {rpo}\n"));
            if let Err(msg) = checked {
                out.error = Some(("dom".into(), msg));
            }
        }
    }
    out
}

fn expected_tail(case: &Case) -> String {
    case.tail
        .lines()
        .filter(|l| !l.starts_with("idom"))
        .map(|l| format!("{l}\n"))
        .collect()
}

fn first_difference(expected: &str, actual: &str) -> String {
    for (n, (e, a)) in expected.lines().zip(actual.lines()).enumerate() {
        if e != a {
            return format!("line {}:\n  oracle: {e}\n  kuna:   {a}", n + 1);
        }
    }
    format!(
        "length differs: oracle {} lines, kuna {} lines",
        expected.lines().count(),
        actual.lines().count()
    )
}

fn compare(case: &Case, out: &Outcome) -> Result<(), String> {
    for (name, expected) in &case.stages {
        let actual = out.stages.iter().find(|(n, _)| n == name);
        match actual {
            None => return Err(format!("stage {name}: kuna stopped early: {:?}", out.error)),
            Some((_, actual)) if actual != expected => {
                return Err(format!(
                    "stage {name}: {}",
                    first_difference(expected, actual)
                ))
            }
            _ => {}
        }
    }
    match (&case.error, &out.error) {
        (None, None) => {
            let expected = expected_tail(case);
            if expected != out.tail {
                return Err(format!("tail: {}", first_difference(&expected, &out.tail)));
            }
        }
        (None, Some(e)) => return Err(format!("kuna threw at {}: {}", e.0, e.1)),
        (Some(e), None) => {
            return Err(format!(
                "oracle threw at {} ({} {}), kuna did not",
                e.0, e.1, e.2
            ))
        }
        (Some(e), Some(a)) => {
            if e.0 != a.0 {
                return Err(format!("oracle threw at {}, kuna at {}: {}", e.0, a.0, a.1));
            }
            let checked = e.1 == "RuntimeException" || e.1 == "IllegalStateException";
            if checked && !a.1.ends_with(&e.2) {
                return Err(format!(
                    "exception message: oracle {:?}, kuna {:?}",
                    e.2, a.1
                ));
            }
        }
    }
    Ok(())
}

fn check_options(header: &str) {
    for (key, name) in [
        (
            IFernflowerPreferences::REMOVE_EMPTY_RANGES,
            "remove-empty-try-catch",
        ),
        (
            IFernflowerPreferences::ENSURE_SYNCHRONIZED_MONITOR,
            "ensure-synchronized-monitors",
        ),
        (
            IFernflowerPreferences::INCORPORATE_RETURNS,
            "incorporate-returns",
        ),
        (IFernflowerPreferences::OLD_TRY_DEDUP, "old-try-dedup"),
        (IFernflowerPreferences::FORCE_JSR_INLINE, "force-jsr-inline"),
    ] {
        let value = if DecompilerContext::get_option(key) {
            "1"
        } else {
            "0"
        };
        assert!(
            header.contains(&format!("{name}={value}")),
            "option {name} is {value} here, oracle header: {header}"
        );
    }
}

/// Replays one golden file; returns (replayed, emulated-jsr, skipped) counts.
fn replay(file: &str) -> (usize, usize, usize) {
    let (options, cases) = parse(&read_golden(file));
    check_options(&options);
    let (mut replayed, mut emulated, mut skipped) = (0, 0, 0);
    let mut failures = Vec::new();
    for case in &cases {
        if case.pool || (case.jsr && case.has_jsr_instructions()) {
            skipped += 1;
            continue;
        }
        reset_class_scope();
        let jsr = case.jsr.then_some(JsrPass::Reduced);
        let out = run_pipeline(case, || build_sequence(case), jsr);
        if let Err(msg) = compare(case, &out) {
            failures.push(format!("{}\n  {msg}", case.header));
        }
        if case.jsr {
            emulated += 1;
        } else {
            replayed += 1;
        }
    }
    reset_class_scope();
    assert!(
        failures.is_empty(),
        "{file}: {} of {} methods differ from Vineflower; first ones:\n{}",
        failures.len(),
        replayed + emulated,
        failures
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    eprintln!("{file}: {replayed} replayed, {emulated} with the jsr pass reduced, {skipped} need the class parser");
    (replayed, emulated, skipped)
}

#[test]
fn cfg_matches_vineflower_on_test_cases() {
    let (replayed, _, skipped) = replay("cases.cfg");
    assert!(
        replayed >= 100 && skipped < 40,
        "{replayed} replayed, {skipped} skipped"
    );
}

#[test]
fn cfg_matches_vineflower_on_jdk_methods() {
    let (replayed, _, _) = replay("jdk.cfg.gz");
    assert!(replayed >= 300, "{replayed} replayed");
}

#[test]
fn cfg_matches_vineflower_on_vineflower_test_corpus() {
    let (replayed, emulated, _) = replay("vineflower.cfg.gz");
    assert!(
        replayed + emulated >= 500,
        "{replayed} + {emulated} replayed"
    );
}

#[test]
fn golden_parser_reads_every_case() {
    let (_, cases) = parse(&read_golden("cases.cfg"));
    assert!(cases
        .iter()
        .all(|c| c.stages.len() == 3 || c.error.is_some()));
    assert!(cases
        .iter()
        .any(|c| c.class == "CfgCases" && c.name == "nestedTry" && !c.descriptor.is_empty()));
}

fn method_key(class: &str, name: &str, descriptor: &str, version: BytecodeVersion) -> String {
    format!(
        "{class} {name} {descriptor} v={}.{}",
        version.major, version.minor
    )
}

/// The committed class files through the real parser and the real `inlineJsr`, against the same
/// oracle dumps; this also covers the methods the replay above has to skip.
#[test]
#[ignore = "needs class-file parser (integration)"]
fn cfg_matches_vineflower_from_class_files() {
    let (options, cases) = parse(&read_golden("cases.cfg"));
    check_options(&options);
    let mut by_key: JHashMap<String, usize> = JHashMap::new();
    for (i, c) in cases.iter().enumerate() {
        by_key.put(
            method_key(
                &c.class,
                &c.name,
                &c.descriptor,
                BytecodeVersion::new(c.version.0, c.version.1),
            ),
            i,
        );
    }

    let root = format!("{}/tests/data/cfg/classes", env!("CARGO_MANIFEST_DIR"));
    let mut failures = Vec::new();
    let mut checked = 0;
    for set in ["r8", "r21", "jasm", "legacy"] {
        let mut files: Vec<_> = std::fs::read_dir(format!("{root}/{set}"))
            .expect("class dir")
            .map(|e| e.expect("entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "class"))
            .collect();
        files.sort();
        for path in files {
            let bytes = std::fs::read(&path).expect("class file");
            let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
            DecompilerContext::set_current_class(Some(cl));
            let class = cl.with(|d| d.qualified_name.clone());
            let methods: Vec<StructMethod> = cl.get_methods().borrow().iter().copied().collect();
            for mt in methods {
                if !mt.contains_code() {
                    continue;
                }
                let version = mt.get_bytecode_version();
                let key = method_key(&class, &mt.get_name(), &mt.get_descriptor(), version);
                let case = &cases[by_key
                    .get_cloned(&key)
                    .unwrap_or_else(|| panic!("no oracle dump for {key}"))];
                reset_class_scope();
                mt.release_resources();
                mt.expand_data(cl);
                let jsr = (version.has_jsr()
                    || DecompilerContext::get_option(IFernflowerPreferences::FORCE_JSR_INLINE))
                .then_some(JsrPass::Real(cl, mt));
                let out = run_pipeline(
                    case,
                    || mt.get_instruction_sequence().expect("expanded method"),
                    jsr,
                );
                if let Err(msg) = compare(case, &out) {
                    failures.push(format!("{}\n  {msg}", case.header));
                }
                checked += 1;
            }
            DecompilerContext::set_current_class(None);
        }
    }
    reset_class_scope();
    assert!(
        failures.is_empty(),
        "{} of {checked} methods differ from Vineflower; first ones:\n{}",
        failures.len(),
        failures
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert_eq!(checked, cases.len(), "every dumped method is checked");
}
