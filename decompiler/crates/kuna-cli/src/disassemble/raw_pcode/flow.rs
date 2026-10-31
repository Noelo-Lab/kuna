//! `raw-pcode --follow`: the lifts direct flow reaches from the start, grouped
//! into blocks with their successors, and every place the walk could not go on.
//!
//! Successors are read off each lift's own p-code by the rule the decompiler's
//! flow follows ([`successors`]), so a predicated instruction falls through and
//! a call is assumed to return. The walk stays inside one window: the caller's
//! `--bytes` or range, else the function extent (64 bytes outside any function).

use super::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

struct Edge {
    kind: &'static str,
    target: Option<u64>,
}

impl Edge {
    fn json(&self) -> Json {
        Json::Object(vec![
            ("kind".into(), Json::Str(self.kind.into())),
            (
                "target".into(),
                self.target
                    .map_or(Json::Null, |t| Json::Number(t.to_string())),
            ),
            (
                "target_hex".into(),
                self.target
                    .map_or(Json::Null, |t| Json::Str(format!("0x{t:x}"))),
            ),
        ])
    }

    fn text(&self) -> String {
        match self.target {
            Some(t) => format!("{} 0x{t:x}", self.kind),
            None => self.kind.to_string(),
        }
    }
}

/// A place the walk could not go on: its reason, the lift's address, and the
/// target or error message that goes with it.
struct Issue {
    reason: &'static str,
    address: u64,
    target: Option<u64>,
    message: Option<String>,
}

impl Issue {
    fn at(reason: &'static str, address: u64) -> Self {
        Self {
            reason,
            address,
            target: None,
            message: None,
        }
    }

    fn json(&self) -> Json {
        let mut members = vec![
            ("reason".into(), Json::Str(self.reason.into())),
            ("address".into(), Json::Number(self.address.to_string())),
            (
                "address_hex".into(),
                Json::Str(format!("0x{:x}", self.address)),
            ),
        ];
        match &self.message {
            Some(message) => members.push(("message".into(), Json::Str(message.clone()))),
            None => {
                members.push((
                    "target".into(),
                    self.target
                        .map_or(Json::Null, |t| Json::Number(t.to_string())),
                ));
                members.push((
                    "target_hex".into(),
                    self.target
                        .map_or(Json::Null, |t| Json::Str(format!("0x{t:x}"))),
                ));
            }
        }
        Json::Object(members)
    }

    fn text(&self) -> String {
        match (&self.message, self.target) {
            (Some(message), _) => format!("{} at 0x{:x}: {message}", self.reason, self.address),
            (None, Some(t)) => format!("{} at 0x{:x} -> 0x{t:x}", self.reason, self.address),
            (None, None) => format!("{} at 0x{:x}", self.reason, self.address),
        }
    }
}

struct Node {
    end: u64,
    row: String,
    edges: Vec<Edge>,
}

/// The successors of one lift, by the rule `FlowInfo::xrefControlFlow` applies
/// (`p2_lift/flow.rs`): a branch into the constant space is p-code-relative and
/// stays inside the lift, the ops after an unconditional transfer that no
/// internal branch skips past are dead, and the lift falls through unless its
/// last live op is a BRANCH, BRANCHIND or RETURN, or an internal branch runs to
/// its end. A predicated instruction (ARM `movle`, x86 `CMOVG`) that skips its
/// own body therefore names the next address twice, as a branch and as its
/// fall-through. `None` for a relative branch that lands outside the lift.
fn successors(ops: &[RawPcodeOp], next: u64) -> Option<Vec<Edge>> {
    let mut edges = Vec::new();
    let (mut deepest, mut to_end, mut last) = (0usize, false, None);
    for (i, op) in ops.iter().enumerate() {
        last = Some(op.opcode);
        match op.opcode {
            "BRANCH" | "CBRANCH" => {
                let dest = op.inputs.first()?;
                if dest.space == "const" {
                    let target = relative_target(i, dest)?;
                    match target.cmp(&ops.len()) {
                        std::cmp::Ordering::Less => deepest = deepest.max(target),
                        std::cmp::Ordering::Equal => to_end = true,
                        std::cmp::Ordering::Greater => return None,
                    }
                } else {
                    edges.push(Edge {
                        kind: "branch",
                        target: Some(dest.offset),
                    });
                }
                if op.opcode == "BRANCH" && i >= deepest {
                    break;
                }
            }
            "BRANCHIND" => {
                edges.push(Edge {
                    kind: "indirect",
                    target: None,
                });
                if i >= deepest {
                    break;
                }
            }
            "RETURN" if i >= deepest => break,
            _ => {}
        }
    }
    if to_end || !matches!(last, Some("BRANCH" | "BRANCHIND" | "RETURN")) {
        edges.push(Edge {
            kind: "fallthrough",
            target: Some(next),
        });
    }
    Some(edges)
}

/// The op index a p-code-relative branch at `index` lands on: SLEIGH stores the
/// distance in ops, masked to the varnode's size.
fn relative_target(index: usize, dest: &RawPcodeVarnode) -> Option<usize> {
    let bits = dest.size.clamp(1, 8) * 8;
    let distance = if bits == 64 {
        dest.offset as i64
    } else {
        let raw = dest.offset & ((1u64 << bits) - 1);
        if raw >> (bits - 1) == 1 {
            raw as i64 - (1i64 << bits)
        } else {
            raw as i64
        }
    };
    usize::try_from(index as i64 + distance).ok()
}

/// Walk direct flow from the start and render the blocks it reached.
pub(super) fn render(args: &DisArgs) -> Result<Output, String> {
    // A count limits work, not the window, so only a byte bound is the caller's.
    let bounded = args.bytes.is_some() || split_range(args.spec.trim()).is_some();
    with_target(args, bounded, |prog, args, region| walk(args, prog, region))
}

fn walk(args: &DisArgs, prog: &ConsoleProgram, mut region: Region) -> Result<Output, String> {
    require_mapped(args, prog, &region)?;
    let notes = data_notes(prog, &region);
    if region.end.is_none() {
        let extent = prog.function_extent_at(region.start);
        let span = if extent > 0 {
            extent
        } else {
            DEFAULT_WINDOW_BYTES
        };
        region.end = Some(region.start.saturating_add(span));
    }
    let end = region.stop().ok_or("cannot bound control-flow walk")?;
    let inside = |address: u64| address >= region.start && address < end;
    let limit = args.count.unwrap_or(DERIVED_INSTRUCTION_CAP);
    let mut queue = VecDeque::from([region.start]);
    let mut nodes = BTreeMap::<u64, Node>::new();
    let mut issues = Vec::new();
    let mut widest = 0u64;
    let mut failed = BTreeSet::new();
    while let Some(addr) = queue.pop_front() {
        if nodes.contains_key(&addr) || failed.contains(&addr) || !inside(addr) {
            continue;
        }
        if nodes.len() >= limit {
            issues.push(Issue::at("instruction_limit", addr));
            break;
        }
        let lifted = match lift(prog, addr, Some(end)) {
            Ok(lifted) => lifted,
            Err(message) => {
                issues.push(Issue {
                    message: Some(message),
                    ..Issue::at("decode_error", addr)
                });
                failed.insert(addr);
                continue;
            }
        };
        let next = lifted.end();
        let lowest = addr.saturating_sub(widest.saturating_sub(1));
        if nodes.range(lowest..next).any(|(_, node)| node.end > addr) {
            issues.push(Issue::at("overlapping_instruction", addr));
        }
        widest = widest.max(lifted.size);
        let edges = successors(&lifted.ops, next).unwrap_or_else(|| {
            issues.push(Issue::at("unsupported_pcode_flow", addr));
            vec![Edge {
                kind: "unresolved",
                target: None,
            }]
        });
        for edge in &edges {
            match edge.target {
                Some(target) if inside(target) => queue.push_back(target),
                Some(target) => issues.push(Issue {
                    target: Some(target),
                    ..Issue::at("out_of_range", addr)
                }),
                None if edge.kind == "indirect" => issues.push(Issue::at("indirect_branch", addr)),
                None => {}
            }
        }
        let row = if args.json {
            dumps_indent2(&lifted.json())
        } else {
            lifted.text()
        };
        nodes.insert(
            addr,
            Node {
                end: next,
                row,
                edges,
            },
        );
    }

    let mut leaders = BTreeSet::from([region.start]);
    let mut incoming = BTreeMap::<u64, usize>::new();
    for node in nodes.values() {
        for edge in &node.edges {
            if let Some(target) = edge.target.filter(|t| nodes.contains_key(t)) {
                *incoming.entry(target).or_default() += 1;
                if node.edges.len() != 1 || edge.kind != "fallthrough" || target != node.end {
                    leaders.insert(target);
                }
            }
        }
    }
    leaders.extend(
        incoming
            .iter()
            .filter_map(|(&addr, &n)| (n > 1).then_some(addr)),
    );

    let mut seen = BTreeSet::new();
    let (mut blocks, mut text) = (Vec::new(), String::new());
    for &start in nodes.keys() {
        if seen.contains(&start) {
            continue;
        }
        let (mut current, mut rows) = (start, Vec::new());
        let mut last: &[Edge] = &[];
        if !args.json {
            text.push_str(&format!("block 0x{start:x}:\n"));
        }
        while let Some(node) = nodes.get(&current) {
            if !seen.insert(current) {
                break;
            }
            if args.json {
                rows.push(node.row.clone());
            } else {
                text.push_str(&node.row);
            }
            last = &node.edges;
            let [Edge {
                kind: "fallthrough",
                target: Some(next),
            }] = node.edges.as_slice()
            else {
                break;
            };
            if *next != node.end || leaders.contains(next) || !nodes.contains_key(next) {
                break;
            }
            current = *next;
        }
        if args.json {
            let members = vec![
                ("start".into(), Json::Number(start.to_string())),
                ("start_hex".into(), Json::Str(format!("0x{start:x}"))),
                (
                    "successors".into(),
                    Json::Array(last.iter().map(Edge::json).collect()),
                ),
            ];
            blocks.push(object_with_rows(members, "instructions", &rows));
        } else {
            let edges: Vec<String> = last.iter().map(Edge::text).collect();
            let edges = if edges.is_empty() {
                "none".to_string()
            } else {
                edges.join(", ")
            };
            text.push_str(&format!("  successors: {edges}\n"));
        }
    }

    let complete = issues.is_empty();
    let error = issues
        .iter()
        .find_map(|issue| issue.message.as_ref())
        .map(|first| {
            format!("{} instruction(s) in the walk did not lift; the first: {first}", failed.len())
        });
    let text = if args.json {
        let doc = vec![
            ("binary".into(), Json::Str(args.binary.clone())),
            ("kind".into(), Json::Str("flow".into())),
            ("target".into(), target_json(&region)),
            ("start".into(), Json::Number(region.start.to_string())),
            (
                "start_hex".into(),
                Json::Str(format!("0x{:x}", region.start)),
            ),
            ("range_end".into(), Json::Number(end.to_string())),
            ("range_end_hex".into(), Json::Str(format!("0x{end:x}"))),
            ("complete".into(), Json::Bool(complete)),
            (
                "instruction_count".into(),
                Json::Number(nodes.len().to_string()),
            ),
            (
                "notes".into(),
                Json::Array(notes.iter().cloned().map(Json::Str).collect()),
            ),
            (
                "issues".into(),
                Json::Array(issues.iter().map(Issue::json).collect()),
            ),
        ];
        object_with_rows(doc, "blocks", &blocks) + "\n"
    } else {
        let plural = if nodes.len() == 1 {
            "instruction"
        } else {
            "instructions"
        };
        let mut out = format!(
            "# {} {plural} reached from {} (window 0x{:x}..0x{end:x})\n",
            nodes.len(),
            label(&region),
            region.start
        );
        out.push_str(&text);
        if !complete {
            let issues: Vec<String> = issues.iter().map(Issue::text).collect();
            out.push_str(&format!("# incomplete: {}\n", issues.join("; ")));
        }
        out
    };
    Ok(Output { text, notes, error })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn op(opcode: &'static str, inputs: &[(&str, u64, u32)]) -> RawPcodeOp {
        RawPcodeOp {
            opcode,
            output: None,
            inputs: inputs
                .iter()
                .map(|&(space, offset, size)| RawPcodeVarnode {
                    space: space.into(),
                    offset,
                    size,
                })
                .collect(),
        }
    }

    fn edges(ops: &[RawPcodeOp]) -> Vec<(&'static str, Option<u64>)> {
        successors(ops, 0x104)
            .unwrap()
            .iter()
            .map(|e| (e.kind, e.target))
            .collect()
    }

    #[test]
    fn flow_is_read_the_way_the_decompiler_reads_it() {
        let predicated = [
            op("CBRANCH", &[("ram", 0x104, 4), ("unique", 0, 1)]),
            op("COPY", &[]),
        ];
        assert_eq!(
            edges(&predicated),
            [("branch", Some(0x104)), ("fallthrough", Some(0x104))]
        );
        let conditional_return = [op("CBRANCH", &[("ram", 0x104, 4)]), op("RETURN", &[])];
        assert_eq!(edges(&conditional_return), [("branch", Some(0x104))]);
        let jump = [op("BRANCH", &[("ram", 0x200, 4)])];
        assert_eq!(edges(&jump), [("branch", Some(0x200))]);
        let skip_to_end = [
            op("CBRANCH", &[("const", 2, 4)]),
            op("BRANCH", &[("ram", 0x200, 4)]),
        ];
        assert_eq!(
            edges(&skip_to_end),
            [("branch", Some(0x200)), ("fallthrough", Some(0x104))]
        );
        let internal = [
            op("CBRANCH", &[("const", 2, 4)]),
            op("RETURN", &[]),
            op("COPY", &[]),
        ];
        assert_eq!(edges(&internal), [("fallthrough", Some(0x104))]);
        let backward = [op("COPY", &[]), op("CBRANCH", &[("const", 0xffff_ffff, 4)])];
        assert_eq!(edges(&backward), [("fallthrough", Some(0x104))]);
        assert!(successors(&[op("BRANCH", &[("const", 5, 4)])], 0x104).is_none());
        assert_eq!(
            edges(&[op("BRANCHIND", &[("register", 0, 4)])]),
            [("indirect", None)]
        );
        assert_eq!(
            edges(&[op("CALL", &[("ram", 0x300, 4)])]),
            [("fallthrough", Some(0x104))]
        );
        assert_eq!(edges(&[]), [("fallthrough", Some(0x104))]);
    }
}
