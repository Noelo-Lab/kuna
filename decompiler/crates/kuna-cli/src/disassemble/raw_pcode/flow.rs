//! Conservative function-local traversal of SLEIGH's unmodified operations.

use super::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

struct Edge {
    kind: &'static str,
    target: Option<u64>,
}

struct Node {
    end: u64,
    row: Value,
    text: String,
    edges: Vec<Edge>,
}

fn issue(issues: &mut Vec<Value>, reason: &str, address: u64, target: Option<u64>) {
    issues.push(json!({"reason": reason, "address": address,
        "address_hex": format!("0x{address:x}"), "target": target,
        "target_hex": target.map(|v| format!("0x{v:x}"))}));
}

fn edge_json(edge: &Edge) -> Value {
    json!({"kind": edge.kind, "target": edge.target,
        "target_hex": edge.target.map(|v| format!("0x{v:x}"))})
}

pub(super) fn render(args: &DisArgs) -> Result<(String, i32), String> {
    let options =
        mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
    let prog = load_program(&load_args(args, options), DriverDefaults::Inventory)?;
    let (args, thumb) = pointer_args(&prog, args)?;
    if let Some(vma) = thumb {
        return prog
            .with_thumb_pointer(vma, || render_loaded(&args, &prog))
            .map_err(|e| e.to_string())?;
    }
    render_loaded(&args, &prog)
}

fn render_loaded(args: &DisArgs, prog: &ConsoleProgram) -> Result<(String, i32), String> {
    let mut region = resolve_region(prog, args)?;
    if !prog.vma_bytes_mapped(region.start) {
        return Err(format!("no bytes mapped at 0x{:x}", region.start));
    }
    // A count limits work, not the function's address space.
    if region.end.is_none() {
        let extent = prog.function_extent_at(region.start);
        region.end = Some(region.start.saturating_add(if extent > 0 {
            extent
        } else {
            DEFAULT_WINDOW_BYTES
        }));
    }
    let end = region.stop().ok_or("cannot bound control-flow walk")?;
    let inside = |address: u64| address >= region.start && address < end;
    let limit = args.count.unwrap_or(DERIVED_INSTRUCTION_CAP);
    let mut queue = VecDeque::from([region.start]);
    let mut nodes = BTreeMap::<u64, Node>::new();
    let mut issues = Vec::new();
    let mut failed = false;
    while let Some(addr) = queue.pop_front() {
        if nodes.contains_key(&addr) {
            continue;
        }
        if !inside(addr) {
            continue;
        }
        if nodes.len() >= limit {
            issue(&mut issues, "instruction_limit", addr, None);
            break;
        }
        let lifted = (|| -> Result<_, String> {
            let (size, ops) = prog.raw_pcode_at(addr).map_err(|e| e.to_string())?;
            if size <= 0 {
                return Err("invalid instruction length".into());
            }
            let next = addr
                .checked_add(size as u64)
                .ok_or("instruction address overflow")?;
            if next > end {
                return Err("instruction crosses the range or mapped boundary".into());
            }
            let (_, mnemonic, operands) = prog.disassemble_at(addr).map_err(|e| e.to_string())?;
            let bytes = prog
                .read_bytes(addr, size as usize)
                .ok_or("cannot read instruction bytes")?;
            Ok((size, ops, next, mnemonic, operands, bytes))
        })();
        let (size, ops, next, mnemonic, operands, bytes) = match lifted {
            Ok(value) => value,
            Err(message) => {
                issues.push(json!({"reason":"decode_error", "address":addr,
                    "address_hex":format!("0x{addr:x}"), "message":message}));
                failed = true;
                continue;
            }
        };
        if nodes
            .iter()
            .any(|(&start, node)| addr < node.end && next > start)
        {
            issue(&mut issues, "overlapping_instruction", addr, None);
        }
        let mut edges = Vec::new();
        let flow_ops: Vec<_> = ops
            .iter()
            .enumerate()
            .filter(|(_, op)| matches!(op.opcode, "BRANCH" | "CBRANCH" | "BRANCHIND" | "RETURN"))
            .collect();
        let unsupported = flow_ops.iter().any(|(i, op)| {
            *i + 1 != ops.len()
                || (matches!(op.opcode, "BRANCH" | "CBRANCH")
                    && op.inputs.first().is_none_or(|v| v.space == "const"))
        }) || flow_ops.len() > 1;
        if unsupported {
            issue(&mut issues, "unsupported_pcode_flow", addr, None);
            edges.push(Edge {
                kind: "unresolved",
                target: None,
            });
        } else if let Some((_, op)) = flow_ops.last() {
            match op.opcode {
                "RETURN" => {}
                "BRANCHIND" => {
                    issue(&mut issues, "indirect_branch", addr, None);
                    edges.push(Edge {
                        kind: "indirect",
                        target: None,
                    });
                }
                "BRANCH" | "CBRANCH" => {
                    let target = op.inputs[0].offset;
                    edges.push(Edge {
                        kind: "branch",
                        target: Some(target),
                    });
                    if op.opcode == "CBRANCH" {
                        edges.push(Edge {
                            kind: "fallthrough",
                            target: Some(next),
                        });
                    }
                }
                _ => unreachable!(),
            }
        } else {
            edges.push(Edge {
                kind: "fallthrough",
                target: Some(next),
            });
        }
        for edge in &edges {
            if let Some(target) = edge.target {
                if inside(target) {
                    queue.push_back(target);
                } else {
                    issue(&mut issues, "out_of_range", addr, Some(target));
                }
            }
        }
        let bytes = hex(&bytes);
        let mut text = format!("0x{addr:x}: {bytes}  {mnemonic} {operands}\n");
        let mut json_ops = Vec::new();
        for op in ops {
            let output = op.output.as_ref().map(varnode);
            let inputs: Vec<_> = op.inputs.iter().map(varnode).collect();
            let display = |v: &RawPcodeVarnode| format!("({},{:#x},{})", v.space, v.offset, v.size);
            let prefix = op
                .output
                .as_ref()
                .map(|v| format!("{} = ", display(v)))
                .unwrap_or_default();
            text.push_str(&format!(
                "  {prefix}{} {}\n",
                op.opcode,
                op.inputs.iter().map(display).collect::<Vec<_>>().join(", ")
            ));
            json_ops.push(json!({"opcode":op.opcode,"output":output,"inputs":inputs}));
        }
        nodes.insert(
            addr,
            Node {
                end: next,
                text,
                edges,
                row: json!({"address":addr,"address_hex":format!("0x{addr:x}"),"size":size,
                "bytes":bytes,"mnemonic":mnemonic,"operands":operands,"ops":json_ops}),
            },
        );
    }
    let mut leaders = BTreeSet::from([region.start]);
    let mut incoming = BTreeMap::<u64, usize>::new();
    for node in nodes.values() {
        for edge in &node.edges {
            if let Some(target) = edge.target.filter(|v| nodes.contains_key(v)) {
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
    let mut blocks = Vec::new();
    let mut text = String::new();
    for &start in nodes.keys() {
        if seen.contains(&start) {
            continue;
        }
        let mut current = start;
        let mut rows = Vec::new();
        let mut last_edges = Vec::new();
        text.push_str(&format!("block 0x{start:x}:\n"));
        while let Some(node) = nodes.get(&current) {
            if !seen.insert(current) {
                break;
            }
            rows.push(node.row.clone());
            text.push_str(&node.text);
            last_edges = node.edges.iter().map(edge_json).collect();
            if node.edges.len() != 1 || node.edges[0].kind != "fallthrough" {
                break;
            }
            let Some(next) = node.edges[0].target else {
                break;
            };
            if next != node.end
                || leaders.contains(&next)
                || seen.contains(&next)
                || !nodes.contains_key(&next)
            {
                break;
            }
            current = next;
        }
        text.push_str(&format!(
            "  successors: {}\n",
            serde_json::to_string(&last_edges).map_err(|e| e.to_string())?
        ));
        blocks.push(json!({"start":start,"start_hex":format!("0x{start:x}"),"instructions":rows,"successors":last_edges}));
    }
    let complete = issues.is_empty();
    let status = i32::from(failed);
    if args.json {
        Ok((
            format!(
                "{}\n",
                serde_json::to_string_pretty(&json!({"binary":args.binary,"mode":"flow",
            "start":region.start,"start_hex":format!("0x{:x}",region.start),
            "range_end":end,"range_end_hex":format!("0x{end:x}"),"complete":complete,
            "instruction_count":nodes.len(),"issues":issues,"blocks":blocks}))
                .map_err(|e| e.to_string())?
            ),
            status,
        ))
    } else {
        if !complete {
            text.push_str(&format!(
                "# incomplete: {}\n",
                serde_json::to_string(&issues).map_err(|e| e.to_string())?
            ));
        }
        Ok((text, status))
    }
}
