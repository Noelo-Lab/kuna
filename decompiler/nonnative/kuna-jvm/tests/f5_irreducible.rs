//! `IrreducibleCFGDeobfuscator` against Vineflower 1.12.0.
//!
//! `tests/data/cfg/IrreducibleDump.java` builds statement graphs the way
//! `DomHelper.graphToStatement` does and runs `processStatement`'s reducibility loop on them;
//! `golden/irreducible.txt.gz` holds each graph spec with the oracle's dumps. The replay rebuilds
//! every graph from its spec with the same construction order (so statement ids match) and
//! compares the dumps text for text.

use std::io::Read;

use kuna_jvm::basic_block::BasicBlock;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::bytecode_version::BytecodeVersion;
use kuna_jvm::code_constants::{GROUP_GENERAL, OPC_NOP};
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::instruction::Instruction;
use kuna_jvm::instruction_sequence::InstructionSequence;
use kuna_jvm::irreducible_cfg_deobfuscator::IrreducibleCFGDeobfuscator;
use kuna_jvm::rt::{reset_class_scope, try_catch, Caught, JHashMap};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::Statement;

enum Spec {
    Block(i32),
    Sequence(Vec<Spec>),
    General(Vec<Spec>),
}

fn parse_node(s: &[u8], pos: &mut usize) -> Spec {
    let c = s[*pos];
    if c == b'[' || c == b'<' {
        *pos += 1;
        let mut children = Vec::new();
        loop {
            children.push(parse_node(s, pos));
            let d = s[*pos];
            *pos += 1;
            if d == b']' || d == b'>' {
                break;
            }
        }
        if c == b'[' {
            Spec::Sequence(children)
        } else {
            Spec::General(children)
        }
    } else {
        let start = *pos;
        while *pos < s.len() && s[*pos].is_ascii_digit() {
            *pos += 1;
        }
        Spec::Block(
            std::str::from_utf8(&s[start..*pos])
                .unwrap()
                .parse()
                .unwrap(),
        )
    }
}

fn build(spec: &Spec, next_offset: &mut i32) -> Statement {
    match spec {
        Spec::Block(size) => {
            let block = BasicBlock::new(
                DecompilerContext::get_counter_container()
                    .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER),
            );
            let seq = InstructionSequence::new();
            for _ in 0..*size {
                seq.add_instruction(Instruction::create(
                    OPC_NOP,
                    false,
                    GROUP_GENERAL,
                    BytecodeVersion::new(52, 0),
                    None,
                    *next_offset,
                    1,
                ));
                *next_offset += 1;
            }
            block.set_seq(seq);
            BasicBlockStatement::new(block).0
        }
        Spec::Sequence(children) | Spec::General(children) => {
            let children: Vec<Statement> = children.iter().map(|c| build(c, next_offset)).collect();
            let stat = match spec {
                Spec::Sequence(_) => SequenceStatement::new(children.clone()).0,
                _ => GeneralStatement::new(children[0], children.clone(), None).0,
            };
            for pair in children.windows(2) {
                pair[0].add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, pair[0], pair[1]));
            }
            stat.set_all_parent();
            stat
        }
    }
}

fn id(s: Option<Statement>) -> String {
    s.map_or_else(|| "-".to_string(), |s| s.id().to_string())
}

fn edges(lst: Vec<StatEdge>) -> String {
    if lst.is_empty() {
        return "-".to_string();
    }
    lst.iter()
        .map(|e| {
            format!(
                "{}:{}>{}@{}",
                e.get_type(),
                e.get_source().id(),
                e.get_destination().id(),
                id(e.with(|d| d.closure))
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn dump(out: &mut String, s: Statement, indent: &str) {
    out.push_str(&format!(
        "d {indent}{} {} p={} c={} f={}",
        s.type_() as i32,
        s.id(),
        id(s.get_parent()),
        i32::from(s.is_copied()),
        id(s.get_first())
    ));
    if let Some(bb) = s.try_as_basic_block() {
        let block = bb.get_block();
        let seq = block.get_seq();
        let instrs = if seq.is_empty() {
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
        out.push_str(&format!(" b={} i={instrs}", block.get_id()));
    }
    out.push_str(&format!(
        " S={} P={}\n",
        edges(s.get_all_successor_edges()),
        edges(s.get_all_predecessor_edges())
    ));
    let stats: Vec<Statement> = s.get_stats().borrow().to_vec();
    for st in stats {
        dump(out, st, &format!("{indent} "));
    }
}

/// `IrreducibleDump.run`, minus the exception text.
fn run(nodes: &str, edge_spec: &str, out: &mut String) -> Result<(), Caught> {
    let bytes = nodes.as_bytes();
    let mut specs = Vec::new();
    let mut pos = 0;
    loop {
        specs.push(parse_node(bytes, &mut pos));
        if pos >= bytes.len() {
            break;
        }
        pos += 1;
    }

    try_catch(|| {
        let mut next_offset = 0;
        let stats: Vec<Statement> = specs.iter().map(|s| build(s, &mut next_offset)).collect();
        let first = stats[0];
        let exit = DummyExitStatement::new().0;
        let general = GeneralStatement::new(first, stats.clone(), None).0;
        general.set_all_parent();

        let mut first_succ: JHashMap<i32, i32> = JHashMap::new();
        let mut stranger: Option<Statement> = None;
        for e in edge_spec.split(',').filter(|e| !e.is_empty()) {
            let (rest, closure_spec) = match e.split_once('@') {
                Some((r, c)) => (r, Some(c)),
                None => (e, None),
            };
            let (rest, type_spec) = match rest.split_once(':') {
                Some((r, t)) => (r, Some(t)),
                None => (rest, None),
            };
            let (from, to_spec) = rest.split_once('>').unwrap();
            let from: i32 = from.parse().unwrap();
            let src = stats[from as usize];
            let dst = match to_spec {
                "x" => exit,
                "s" => *stranger.get_or_insert_with(|| build(&Spec::Block(1), &mut next_offset)),
                n => stats[n.parse::<usize>().unwrap()],
            };
            if Some(dst) != stranger && first_succ.get_cloned(&from).is_none() {
                first_succ.put(
                    from,
                    if to_spec == "x" {
                        -1
                    } else {
                        to_spec.parse().unwrap()
                    },
                );
            }
            let closure = closure_spec.map(|c| {
                if c == "g" {
                    general
                } else {
                    stats[c.parse::<usize>().unwrap()]
                }
            });
            let edge = match type_spec {
                None if dst == first => {
                    StatEdge::new_closure(StatEdge::TYPE_CONTINUE, src, general, Some(general))
                }
                None if dst == exit => {
                    StatEdge::new_closure(StatEdge::TYPE_BREAK, src, exit, Some(general))
                }
                None => StatEdge::new_closure(StatEdge::TYPE_REGULAR, src, dst, None),
                Some("e") => {
                    StatEdge::new_exceptions(src, dst, Some(vec!["java/lang/Exception".into()]))
                }
                Some(t) => {
                    let type_ = match t {
                        "r" => StatEdge::TYPE_REGULAR,
                        "b" => StatEdge::TYPE_BREAK,
                        "c" => StatEdge::TYPE_CONTINUE,
                        "f" => StatEdge::TYPE_FINALLYEXIT,
                        _ => panic!("edge type {t}"),
                    };
                    StatEdge::new_closure(type_, src, dst, closure)
                }
            };
            src.add_successor(edge);
        }
        for (i, &stat) in stats.iter().enumerate() {
            let Some(to) = first_succ.get_cloned(&(i as i32)) else {
                continue;
            };
            if stat.is_basic_block() || to == 0 {
                continue;
            }
            let last = stat.get_stats().borrow().get_last();
            let dst = if to < 0 { exit } else { stats[to as usize] };
            last.add_successor(StatEdge::new_closure(
                StatEdge::TYPE_BREAK,
                last,
                dst,
                Some(stat),
            ));
        }

        dump(out, general, "");
        dump(out, exit, "");
        for _ in 0..5 {
            let irreducible = IrreducibleCFGDeobfuscator::is_statement_irreducible(general);
            out.push_str(&format!("irr {}\n", i32::from(irreducible)));
            if !irreducible {
                break;
            }
            let split = IrreducibleCFGDeobfuscator::split_irreducible_node(general);
            out.push_str(&format!("split {}\n", i32::from(split)));
            if !split {
                break;
            }
            dump(out, general, "");
            dump(out, exit, "");
        }
    })
}

struct Case {
    header: String,
    nodes: String,
    edges: String,
    body: String,
    error: Option<(String, String)>,
}

fn cases() -> Vec<Case> {
    let path = format!(
        "{}/tests/data/cfg/golden/irreducible.txt.gz",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut text = String::new();
    flate2::read::GzDecoder::new(&bytes[..])
        .read_to_string(&mut text)
        .expect("gzip");
    let mut lines = text.lines();
    assert!(lines.next().unwrap().starts_with("#irreducible"));
    let mut cases = Vec::new();
    let mut case: Option<Case> = None;
    for line in lines {
        if let Some(rest) = line.strip_prefix("=case ") {
            let f: Vec<&str> = rest.split(' ').collect();
            case = Some(Case {
                header: line.to_string(),
                nodes: f[1].to_string(),
                edges: f.get(2).unwrap_or(&"").to_string(),
                body: String::new(),
                error: None,
            });
        } else if line == "=end" {
            cases.push(case.take().expect("=end outside a case"));
        } else if let Some(rest) = line.strip_prefix("!error ") {
            let (class, msg) = rest.split_once(' ').unwrap_or((rest, ""));
            case.as_mut().unwrap().error = Some((class.to_string(), msg.to_string()));
        } else {
            let c = case.as_mut().expect("line outside a case");
            c.body.push_str(line);
            c.body.push('\n');
        }
    }
    cases
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

#[test]
fn irreducible_splitting_matches_vineflower() {
    let context = crate::common::install_default_context();
    let all = cases();
    let mut failures = Vec::new();
    let (mut irreducible, mut splits, mut errors) = (0, 0, 0);
    for case in &all {
        reset_class_scope();
        context.context().survive_class_scope_reset([1; 4]);
        let mut out = String::new();
        let result = run(&case.nodes, &case.edges, &mut out);
        let verdict = match (&case.error, result) {
            (None, Ok(())) if out == case.body => Ok(()),
            (None, Ok(())) => Err(first_difference(&case.body, &out)),
            (None, Err(e)) => Err(format!("kuna threw {}", e.message)),
            (Some(e), Ok(())) => Err(format!("oracle threw {} {}, kuna did not", e.0, e.1)),
            (Some(e), Err(caught)) => {
                let same_class = caught.is(&e.0) || caught.message.contains(&e.0);
                let checked = e.0 == "RuntimeException";
                if out != case.body {
                    Err(first_difference(&case.body, &out))
                } else if !same_class || (checked && !caught.message.ends_with(&e.1)) {
                    Err(format!(
                        "oracle threw {} {}, kuna {}",
                        e.0, e.1, caught.message
                    ))
                } else {
                    errors += 1;
                    Ok(())
                }
            }
        };
        if let Err(msg) = verdict {
            failures.push(format!("{}\n  {msg}", case.header));
        }
        irreducible += usize::from(case.body.contains("irr 1"));
        splits += case.body.matches("split 1").count();
    }
    reset_class_scope();
    assert!(
        failures.is_empty(),
        "{} of {} graphs differ from Vineflower; first ones:\n{}",
        failures.len(),
        all.len(),
        failures
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        all.len() >= 400 && irreducible >= 200 && splits >= 500 && errors >= 5,
        "{} graphs, {irreducible} irreducible, {splits} splits, {errors} errors",
        all.len()
    );
}
