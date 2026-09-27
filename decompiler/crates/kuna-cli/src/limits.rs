//! The engine budgets a function can blow before anyone decompiles it:
//! `maxinstruction` (the instructions one function's flow may follow) and
//! `jumptablemax` (the entries one jump table may have).
//!
//! Both are measured off the reference walk the call graph already ran, never by
//! decompiling: each function's own descent is re-counted on the successor graph
//! a measured walk keeps (`xrefs::build_measured`), and each switch carries the
//! case count its range check states. `functions --summary` reports the result
//! as `summary.limits`, and `functions --reachable-from` as a top-level `limits`
//! (see `docs/cli.md`).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use kuna_analysis::listing::xrefs::SwitchTable;
use kuna_console::engine::{ConsoleProgram, FunctionEntry};

use crate::decompile_all::CallGraph;
use crate::jsonfmt::Json;

/// The live budgets and the selected functions over either.
pub(crate) struct Limits {
    maxinstruction: u64,
    jumptablemax: u64,
    over: Vec<OverLimit>,
}

/// One function past a budget: its own body is longer than `maxinstruction`, or
/// it dispatches through a switch longer than `jumptablemax` (listed).
struct OverLimit {
    entry: FunctionEntry,
    /// Counted up to `maxinstruction + 1`.
    instructions: usize,
    switches: Vec<SwitchTable>,
}

/// Measure `selected` against the live `maxinstruction` / `jumptablemax`.
///
/// A switch belongs to the function whose extent contains its dispatch, and to
/// the entry whose walk read it only when no extent does: a lower entry's
/// descent (a gcc `.cold` fragment jumping back into its parent) can reach the
/// dispatch before its own function is walked.
pub(crate) fn measure(prog: &ConsoleProgram, graph: &CallGraph, selected: &[FunctionEntry]) -> Limits {
    let maxinstruction = u64::from(prog.arch().max_instructions);
    let jumptablemax = u64::from(prog.arch().max_jumptable_size);
    let mut switches: BTreeMap<u64, Vec<SwitchTable>> = BTreeMap::new();
    for sw in graph.switch_tables().iter().filter(|sw| sw.truncated) {
        let owner = graph
            .owner_of(sw.dispatch)
            .or_else(|| graph.is_entry(sw.function).then_some(sw.function));
        if let Some(owner) = owner {
            switches.entry(owner).or_default().push(sw.clone());
        }
    }
    let addrs: Vec<u64> = selected.iter().map(|e| e.addr.get_offset()).collect();
    let cap = usize::try_from(maxinstruction).map_or(usize::MAX, |m| m.saturating_add(1));
    let counts = graph.instruction_counts(&addrs, cap);
    let over = selected
        .iter()
        .zip(counts)
        .filter_map(|(e, instructions)| {
            let switches = switches.remove(&e.addr.get_offset()).unwrap_or_default();
            (instructions as u64 > maxinstruction || !switches.is_empty()).then(|| OverLimit {
                entry: e.clone(),
                instructions,
                switches,
            })
        })
        .collect();
    Limits { maxinstruction, jumptablemax, over }
}

pub(crate) fn to_json(limits: &Limits, display_address: &dyn Fn(u64) -> u64) -> Json {
    let hex = |a: u64| {
        let a = display_address(a);
        (Json::Number(a.to_string()), Json::Str(format!("0x{a:x}")))
    };
    let over = limits
        .over
        .iter()
        .map(|o| {
            let (address, address_hex) = hex(o.entry.addr.get_offset());
            let switches = o
                .switches
                .iter()
                .map(|sw| {
                    let (address, address_hex) = hex(sw.dispatch);
                    Json::Object(vec![
                        ("address".into(), address),
                        ("address_hex".into(), address_hex),
                        (
                            "cases".into(),
                            sw.cases.map_or(Json::Null, |n| Json::Number(n.to_string())),
                        ),
                        ("read".into(), Json::Number(sw.read.to_string())),
                    ])
                })
                .collect();
            Json::Object(vec![
                ("name".into(), Json::Str(o.entry.name.clone())),
                ("address".into(), address),
                ("address_hex".into(), address_hex),
                ("size".into(), Json::Number(o.entry.size.to_string())),
                ("instructions".into(), Json::Number(o.instructions.to_string())),
                (
                    "over_maxinstruction".into(),
                    Json::Bool(o.instructions as u64 > limits.maxinstruction),
                ),
                ("switches_over_jumptablemax".into(), Json::Array(switches)),
            ])
        })
        .collect();
    Json::Object(vec![
        ("maxinstruction".into(), Json::Number(limits.maxinstruction.to_string())),
        ("jumptablemax".into(), Json::Number(limits.jumptablemax.to_string())),
        ("over".into(), Json::Array(over)),
    ])
}

pub(crate) fn write_text(out: &mut String, limits: &Limits, display_address: &dyn Fn(u64) -> u64) {
    let _ = writeln!(
        out,
        "limits\tmaxinstruction {}\tjumptablemax {}",
        limits.maxinstruction, limits.jumptablemax
    );
    for o in &limits.over {
        let address = display_address(o.entry.addr.get_offset());
        let _ = writeln!(out, "  0x{address:x}\t{} instructions\t{}", o.instructions, o.entry.name);
        for sw in &o.switches {
            let at = display_address(sw.dispatch);
            let cases = sw.cases.map_or_else(|| format!(">={}", sw.read), |n| n.to_string());
            let _ = writeln!(out, "    switch 0x{at:x}\t{cases} cases\t{} read", sw.read);
        }
    }
}
