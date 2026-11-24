//! Report Kuna disassembly, lift and C output at fixed entries of a Carmel XML image.
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    path::PathBuf,
    rc::Rc,
    time::Duration,
};

use kuna_base::{address::Address, error::KunaError, xml::{Element, xml_tree}};
use kuna_console::{
    engine::{ConsoleProgram, FlowKind, InsnFlow, bootstrap_program},
    project::{DecompileOptions, FAST_WHOLE_BINARY_FN_BUDGET_SECONDS, FuncResult, decompile_targets_with},
};
use regex::Regex;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const ARCH: &str = "Carmel:BE:24:default:default";
const SLOT: u64 = 3;
const CODE_END: u64 = SLOT * (1 << 24);

fn offset(el: &Element) -> u64 {
    let text = std::str::from_utf8(el.get_attribute_value("offset").unwrap()).unwrap();
    if let Some(hex) = text.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap()
    } else {
        text.parse().unwrap()
    }
}

/// The XML loader pads holes; an address in its padding is not mapped silicon ROM.
fn code_extents(root: &Element) -> BTreeMap<u64, u64> {
    let mut extents = BTreeMap::new();
    for chunk in root.get_children().iter().filter(|e| {
        e.get_name() == "bytechunk" && e.get_attribute_value("space").unwrap() == b"code"
    }) {
        let mut len = 0;
        for run in chunk.get_content().split(|b| b.is_ascii_whitespace()).filter(|r| !r.is_empty()) {
            assert!(run.len() % 2 == 0 && run.iter().all(u8::is_ascii_hexdigit), "invalid CODE hex");
            len += run.len() as u64 / 2;
        }
        let start = offset(chunk);
        let end = start.checked_add(len).expect("CODE extent overflow");
        assert!(len > 0 && end <= CODE_END, "invalid CODE extent {start:#x}..{end:#x}");
        assert!(extents.insert(start, end).is_none(), "duplicate CODE extent at {start:#x}");
    }
    let mut previous_end = 0;
    for (&start, &end) in &extents {
        assert!(previous_end <= start, "overlapping CODE extent at {start:#x}");
        previous_end = end;
    }
    extents
}

#[derive(Default)]
struct Probe {
    state: &'static str,
    len: Option<u64>,
    lifted_len: Option<u64>,
    mnemonic: Option<String>,
    operands: Option<String>,
    error: Option<String>,
    flow: Option<InsnFlow>,
}

impl Probe {
    fn report(&self, at: u64) -> Value {
        json!({
            "byte_address": at, "code_word": at / SLOT, "outcome": self.state,
            "issue_bytes": self.len, "lift_with_delay_bytes": self.lifted_len,
            "mnemonic": self.mnemonic, "operands": self.operands, "error": self.error,
        })
    }
}

struct Image<'a> {
    prog: &'a ConsoleProgram,
    code: &'a BTreeMap<u64, u64>,
}

impl Image<'_> {
    fn covers(&self, at: u64, len: u64) -> bool {
        self.code.range(..=at).next_back().is_some_and(|(_, &end)| at + len <= end)
    }

    fn probe(&self, at: u64) -> Probe {
        let mut out = Probe::default();
        if !self.covers(at, SLOT) {
            out.state = "unmapped";
            return out;
        }
        if at % SLOT != 0 {
            out.state = "invalid_start";
            return out;
        }
        let space = self.prog.arch().manage().get_default_code_space().unwrap();
        let addr = Address::new(Rc::clone(space), at);
        let (mode, next_size, known_template) = self.prog.arch().with_context_db(|db| {
            let get = |name: &[u8]| db.get_variable_value(name, &addr).unwrap();
            (get(b"carmel_issue"), u64::from(get(b"carmel_next_issue_size")), get(b"carmel_cliw_known") != 0)
        });
        let expected_len = match mode {
            0 => {
                out.state = "invalid_start";
                return out;
            }
            1 | 4 | 5 => 2 * SLOT,
            2 | 3 => SLOT,
            _ => panic!("unknown Carmel framing mode {mode} at {at:#x}"),
        };
        if !self.covers(at, expected_len) {
            out.state = "unmapped";
            return out;
        }
        let (len, mnemonic, operands) = match self.prog.disassemble_at(at) {
            Ok(decoded) => decoded,
            Err(error) => {
                out.state = "decode_error";
                out.error = Some(error.to_string());
                return out;
            }
        };
        let cliw = mnemonic.eq_ignore_ascii_case("cliw");
        let raw = mnemonic.eq_ignore_ascii_case("raw24") || mnemonic.eq_ignore_ascii_case("raw48");
        out.len = u64::try_from(len).ok();
        out.mnemonic = Some(mnemonic);
        out.operands = Some(operands);
        if out.len != Some(expected_len) {
            out.state = "inconsistent";
            out.error = Some(format!("assembly length {len} disagrees with painted issue length {expected_len}"));
            return out;
        }
        match self.prog.raw_pcode_at(at) {
            Err(KunaError::Unimpl { explain, .. }) => {
                out.state = match (cliw, known_template) {
                    (true, false) => "raw_cliw",
                    (true, true) => "unsupported_cliw",
                    _ => "unsupported",
                };
                out.error = Some(explain);
            }
            Err(error) => {
                out.state = "lift_error";
                out.error = Some(error.to_string());
            }
            Ok((total, ops)) => {
                out.lifted_len = u64::try_from(total).ok();
                let total = out.lifted_len.unwrap_or_default();
                let flow = self.prog.insn_flow(at);
                let next = (at + expected_len) % CODE_END;
                let delayed = total > expected_len;
                if raw || (cliw && !known_template) {
                    out.state = "inconsistent";
                    out.error = Some("opaque instruction lifted as executable p-code".into());
                } else if total < expected_len || delayed && (
                    !matches!(next_size, 3 | 6) || total != expected_len + next_size
                    || !self.covers(next, next_size) || flow.is_none()
                ) {
                    out.state = "inconsistent";
                    out.error = Some(format!("lift length {total} disagrees with issue {expected_len} and delay {next_size}"));
                } else if ops.iter().any(|op| op.opcode == "CALLOTHER") {
                    out.state = "lift_error";
                    out.error = Some("opaque CALLOTHER effect; no following edge asserted".into());
                } else {
                    let mut pcode_targets = Vec::new();
                    let mut transfers = 0;
                    for op in &ops {
                        match op.opcode {
                            "BRANCH" | "CBRANCH" => {
                                let dst = &op.inputs[0];
                                if dst.space != "const" {
                                    transfers += 1;
                                    if dst.space == "code" {
                                        pcode_targets.push(dst.offset);
                                    }
                                }
                            }
                            "CALL" => {
                                transfers += 1;
                                let dst = &op.inputs[0];
                                if dst.space == "code" {
                                    pcode_targets.push(dst.offset);
                                }
                            }
                            "BRANCHIND" | "CALLIND" | "RETURN" => transfers += 1,
                            _ => {}
                        }
                    }
                    let consistent = match &flow {
                        Some(flow) => (transfers == 1) && flow.targets == pcode_targets && (
                            !matches!(flow.kind, FlowKind::Jump | FlowKind::CondJump | FlowKind::Call)
                            || pcode_targets.len() == 1
                        ),
                        None => transfers == 0,
                    };
                    out.state = if consistent { "decoded" } else { "inconsistent" };
                    if consistent {
                        out.flow = flow;
                    } else {
                        out.error = Some("raw p-code and following-flow transfer disagree".into());
                    }
                }
            }
        }
        out
    }
}

#[derive(Default)]
struct Walk {
    reached: BTreeSet<u64>,
    counts: BTreeMap<&'static str, usize>,
    edges: Vec<Value>,
    incomplete: bool,
}

impl Walk {
    fn count(&mut self, what: &'static str) {
        *self.counts.entry(what).or_default() += 1;
    }

    fn observe(&mut self, at: u64, probe: &Probe) {
        if self.reached.insert(at) {
            self.count(probe.state);
            if matches!(probe.state, "raw_cliw" | "unsupported_cliw") {
                self.count("unresolved_cliw");
            }
        }
        self.incomplete |= probe.state != "decoded";
    }

    fn edge(&mut self, from: u64, kind: &str, to: Option<u64>, status: &str) {
        self.edges.push(json!({
            "from_byte": from, "kind": kind, "to_byte": to,
            "to_code_word": to.filter(|at| at % SLOT == 0).map(|at| at / SLOT),
            "target_outcome": status,
        }));
    }
}

/// Walk one function's local CFG; record direct calls without traversing into callees.
fn walk(image: &Image<'_>, cache: &mut BTreeMap<u64, Probe>, entry: u64) -> Walk {
    let mut out = Walk::default();
    let mut pending = vec![entry];
    let mut processed = BTreeSet::new();
    while let Some(at) = pending.pop() {
        if !processed.insert(at) {
            continue;
        }
        let current = cache.entry(at).or_insert_with(|| image.probe(at));
        out.observe(at, current);
        if current.state != "decoded" {
            continue;
        }
        let len = current.len.unwrap();
        let total = current.lifted_len.unwrap();
        let flow = current.flow.clone();
        if total > len {
            let slot = (at + len) % CODE_END;
            let delay = cache.entry(slot).or_insert_with(|| image.probe(slot));
            out.observe(slot, delay);
            out.edge(at, "delay", Some(slot), delay.state);
            if delay.state != "decoded" || delay.flow.is_some()
                || delay.len != Some(total - len) || delay.lifted_len != delay.len
            {
                out.count("inconsistent_delay");
                out.incomplete = true;
                continue;
            }
        }
        let mut fallthrough = match flow.as_ref().map(|f| f.kind) {
            None | Some(FlowKind::CondJump) => Some(("fallthrough", (at + total) % CODE_END)),
            _ => None,
        };
        if let Some(flow) = flow {
            let kind = flow.kind.as_str();
            if flow.targets.is_empty() {
                if flow.kind == FlowKind::Return {
                    out.edge(at, kind, None, "dynamic_return");
                } else {
                    out.edge(at, kind, None, "unresolved");
                    out.count("unresolved_indirect");
                    if flow.kind == FlowKind::CallInd {
                        out.count("unresolved_code_calls");
                    }
                    out.incomplete = true;
                }
            }
            for target in flow.targets {
                let target_probe = cache.entry(target).or_insert_with(|| image.probe(target));
                let status = target_probe.state;
                out.edge(at, kind, Some(target), status);
                match status {
                    "invalid_start" => out.count("direct_edges_to_invalid_start"),
                    "unmapped" => out.count("direct_edges_to_absent_rom"),
                    _ => {}
                }
                if flow.kind == FlowKind::Call {
                    if status == "decoded" {
                        // A delayed CALL saves ordinary next PC + two CODE words, not post-delay PC.
                        let skip = if total > len { 2 * SLOT } else { 0 };
                        fallthrough = Some(("return_site", (at + len + skip) % CODE_END));
                    } else {
                        out.count("unresolved_code_calls");
                        out.incomplete = true;
                    }
                } else {
                    pending.push(target);
                }
            }
        }
        if let Some((kind, next)) = fallthrough {
            let probe = cache.entry(next).or_insert_with(|| image.probe(next));
            out.edge(at, kind, Some(next), probe.state);
            pending.push(next);
        }
    }
    out
}

struct CMetrics {
    ignored: Regex,
    identifiers: Regex,
    unique: Regex,
    registers: BTreeSet<String>,
}

impl CMetrics {
    fn new(prog: &ConsoleProgram) -> Self {
        let mut registers = BTreeMap::new();
        prog.arch().translate().get_all_registers(&mut registers);
        Self {
            ignored: Regex::new(r#"(?s)/\*.*?\*/|//[^\n]*|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'"#).unwrap(),
            identifiers: Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b").unwrap(),
            unique: Regex::new(r"^Unique[0-9a-fA-F]+$").unwrap(),
            registers: registers.into_values().collect(),
        }
    }

    fn report(&self, result: &FuncResult, incomplete: bool) -> Value {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        let mut truncated = incomplete || result.warnings.iter().any(|warning| matches!(
            warning.kind().as_str(),
            "bad-instruction" | "unimplemented-instruction" | "instruction-limit" | "out-of-bounds" | "unmapped-flow"
        ));
        if let Some(c) = &result.code {
            let cleaned = self.ignored.replace_all(c, " ");
            for token in self.identifiers.find_iter(&cleaned).map(|m| m.as_str()) {
                let register = ["unaff_", "in_", "extraout_"].iter()
                    .find_map(|prefix| token.strip_prefix(*prefix)).unwrap_or(token);
                let metric = match token {
                    "for" | "while" => "loops",
                    "if" => "if_statements",
                    "switch" => "switch_statements",
                    "goto" => "gotos",
                    "halt_baddata" | "halt_unimplemented" | "halt_missing" => {
                        truncated = true;
                        "decode_halts"
                    }
                    _ if self.registers.contains(register) => "raw_register_mentions",
                    _ if self.unique.is_match(token) => "raw_unique_mentions",
                    _ => continue,
                };
                *counts.entry(metric).or_default() += 1;
            }
        }
        let status = match &result.error {
            Some(error) if error == "timed out" || error.contains("per-function decompile budget exceeded") => "timed_out",
            Some(_) => "failed",
            None if result.code.is_none() => "failed",
            None if truncated => "truncated",
            None => "complete",
        };
        json!({ "outcome": status, "error": result.error, "counts": counts })
    }
}

#[test]
#[ignore = "requires a Carmel XML image in KUNA_CARMEL_WILD_IMAGE"]
fn sweep() {
    let file = std::env::var_os("KUNA_CARMEL_WILD_IMAGE").expect("set KUNA_CARMEL_WILD_IMAGE to a Carmel XML image");
    let xml = std::fs::read(file).expect("read KUNA_CARMEL_WILD_IMAGE");
    let mut digest = String::with_capacity(64);
    for byte in Sha256::digest(&xml) {
        write!(&mut digest, "{byte:02x}").expect("format SHA-256 into String");
    }
    let document = xml_tree(&xml).expect("parse Carmel XML");
    let root = document.get_root();
    assert_eq!(root.get_name(), "binaryimage");
    assert_eq!(root.get_attribute_value("arch").unwrap(), ARCH.as_bytes());
    let code = code_extents(root);
    let mut anchors = BTreeMap::new();
    for symbol in root.get_children().iter().filter(|el| el.get_name() == "symbol") {
        assert_eq!(symbol.get_attribute_value("space").unwrap(), b"code");
        let at = offset(symbol);
        assert_eq!(at % SLOT, 0, "fixed entry is not a CODE word boundary");
        let name = format!("FUN_{:06x}", at / SLOT);
        assert_eq!(symbol.get_attribute_value("name").unwrap(), name.as_bytes());
        assert!(anchors.insert(at, name).is_none(), "duplicate fixed entry at {at:#x}");
    }
    assert!(!anchors.is_empty(), "Carmel XML image has no fixed entries");
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut prog = bootstrap_program(Rc::clone(root), ARCH, &[specs.to_str().unwrap().into()])
        .expect("bootstrap Carmel XML against compiled Carmel specification");
    let targets = prog.function_entries_canonical();
    assert_eq!(targets.len(), anchors.len(), "loader did not retain the fixed-entry denominator");
    for entry in &targets {
        assert_eq!(anchors.get(&entry.addr.get_offset()), Some(&entry.name));
    }
    let metrics = CMetrics::new(&prog);
    let mut cache = BTreeMap::new();
    let walks: Vec<_> = {
        let image = Image { prog: &prog, code: &code };
        targets.iter().map(|entry| walk(&image, &mut cache, entry.addr.get_offset())).collect()
    };
    prog.arch_mut().kuna_fn_budget = Some(Duration::from_secs(FAST_WHOLE_BINARY_FN_BUDGET_SECONDS));
    let opts = DecompileOptions { single_target: targets.len() == 1, ..Default::default() };
    let results = decompile_targets_with(&mut prog, targets, &opts);
    assert_eq!(walks.len(), results.len());
    let mut outcomes = BTreeMap::from([("complete", 0), ("truncated", 0), ("failed", 0), ("timed_out", 0)]);
    let mut entries = Vec::new();
    for (result, walk) in results.iter().zip(walks) {
        let c = metrics.report(result, walk.incomplete);
        *outcomes.get_mut(c["outcome"].as_str().unwrap()).unwrap() += 1;
        let issues: Vec<_> = walk.reached.iter().map(|&at| cache[&at].report(at)).collect();
        entries.push(json!({
            "name": result.name, "byte_address": result.byte_address, "code_word": result.byte_address / SLOT,
            "native_witness": "not run", "counts": walk.counts, "issues": issues, "edges": walk.edges, "c": c,
        }));
    }
    println!("CARMEL_WILD_REPORT {}", json!({
        "schema": 1, "arch": ARCH, "xml_sha256": digest, "fixed_entries": anchors.len(),
        "native_witness": "not run; unmapped ROM targets have no native witness",
        "c_outcomes": outcomes, "entries": entries,
    }));
}
