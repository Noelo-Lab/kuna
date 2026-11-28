//! Translation validation of explicitly scoped x86-64 code.

mod byte_file;
mod execute;
mod formal_rv64;
mod induction;
mod memory;
mod obligation;
mod raw_function;
mod solver;
mod terms;

use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use std::time::{Duration, Instant};

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;
use object::{Object, ObjectSymbol, SymbolKind};
use serde_json::{json, Value};

use crate::concolic::{Emitter, Expr};
use crate::state::{Env, ExplicitState, Region};
use crate::{abi, calls, oracle, program, session};

use byte_file::ByteFile;
use memory::{Memory, Window};
use obligation::{Obligation, Verdict};

pub use formal_rv64::export as rv64_terms;

pub struct Options<'a> {
    pub inputs: &'a [String],
    pub bindings: &'a [(String, u64)],
    pub ranges: &'a [(String, u64, u64)],
    pub memory: &'a [MemoryRange],
    pub stop: Option<u64>,
    pub loop_bound: Option<u8>,
    pub induct: bool,
    pub indirect: &'a std::collections::BTreeMap<u64, Vec<u64>>,
    pub output: &'a str,
    pub output_bytes: u32,
    pub step_limit: u64,
    pub timeout: Duration,
    pub solver: &'a Path,
    pub opcode_mutant: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemoryMode {
    Shared,
    PrivateStack,
    Image,
}

impl MemoryMode {
    fn name(self) -> &'static str {
        match self {
            Self::Shared => "symbolic",
            Self::PrivateStack => "private-stack",
            Self::Image => "image",
        }
    }
}

pub struct MemoryRange {
    pub address: u64,
    pub size: u32,
    pub mode: MemoryMode,
}

struct Encoding {
    proof: Proof,
    mutation: Option<Value>,
    ir_blocks: usize,
    ir_phis: usize,
}

enum Proof {
    Observations {
        obligation: Obligation,
        memory_inputs: Vec<(u64, String)>,
    },
    Induction(induction::Induction),
}

/// Prove equality of the declared return projection over the selected interval.
pub fn prove(
    path: &str,
    function: &str,
    decompile_options: &[(String, String)],
    spec_roots: &[String],
    opts: &Options<'_>,
) -> Result<Value, String> {
    let image = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    let mut regions = oracle::elf::regions(&image)?;
    regions.sort_unstable_by_key(|r| r.address);
    if regions.windows(2).any(|w| w[0].end() > w[1].address) {
        return Err("overlapping ELF pages are outside the proof model".into());
    }
    let source = oracle::sha256(&image);
    let mut result = None;
    let mut regions = Some(regions);
    let commands = session::binary_script(path, &[function.to_owned()], decompile_options);
    session::drive(commands, None, spec_roots, |prog, fd| {
        if let Some(regions) = regions.take() {
            let entry = fd.get_address().get_offset();
            let end = opts.stop.map_or_else(|| function_end(&image, entry), Ok);
            result = Some(end.map(|end| verify_block(prog, fd, opts, regions, entry, end)));
        }
    })?;
    let mut result = result.ok_or("no decompiled function is available")??;
    result["provenance"]["elf_sha256"] = json!(source);
    result["decompile_options"] = json!(decompile_options);
    Ok(result)
}

fn function_end(image: &[u8], entry: u64) -> Result<u64, String> {
    let file = object::File::parse(image).map_err(|err| format!("ELF function extent: {err}"))?;
    let sizes: std::collections::BTreeSet<_> = file
        .symbols()
        .filter(|sym| {
            sym.address() == entry
                && sym.kind() == SymbolKind::Text
                && sym.is_definition()
                && sym.size() != 0
        })
        .map(|sym| sym.size())
        .collect();
    if sizes.len() != 1 {
        return Err(
            "whole-function proof needs one nonempty ELF symbol extent at the selected entry"
                .into(),
        );
    }
    entry
        .checked_add(*sizes.first().expect("one symbol size"))
        .ok_or_else(|| "ELF function extent wraps".into())
}

fn verify_block(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    opts: &Options<'_>,
    regions: Vec<Region>,
    entry: u64,
    end: u64,
) -> Value {
    let whole = opts.stop.is_none();
    let mut result = json!({
        "version": 1, "function": fd.get_name(),
        "scope": {
            "architecture": "x86-64", "abi": "sysv-amd64",
            "raw_interval": { "start": format!("{entry:#x}"), "end_exclusive": format!("{end:#x}") },
            "control_flow": match (whole, opts.induct, opts.loop_bound) {
                (true, true, _) => "inductive-function",
                (true, false, Some(_)) => "bounded-function",
                (true, false, None) => "acyclic-function",
                (false, _, _) => "straight-line-interval",
            },
            "raw_indirect_targets": opts.indirect.iter().map(|(pc, targets)| (
                format!("{pc:#x}"), targets.iter().map(|target| format!("{target:#x}")).collect::<Vec<_>>(),
            )).collect::<std::collections::BTreeMap<_, _>>(),
            "observation": { "register": opts.output, "bytes": opts.output_bytes, "arithmetic_definedness": true },
            "symbolic_inputs": opts.inputs,
            "symbolic_flags": oracle::SYMBOLIC_FLAGS.map(|(name, _)| name),
            "bindings": opts.bindings.iter().map(|(r, v)| (r, format!("{v:#x}"))).collect::<Vec<_>>(),
            "ranges": opts.ranges.iter().map(|(r, a, b)| (r, format!("{a:#x}"), format!("{b:#x}"))).collect::<Vec<_>>(),
            "memory": opts.memory.iter().map(|m| json!({
                "start": format!("{:#x}", m.address), "bytes": m.size, "mode": m.mode.name(),
            })).collect::<Vec<_>>(),
            "stack_entry": (whole || !opts.memory.is_empty()).then(|| format!("{:#x}", oracle::SP)),
            "max_ops_per_side": opts.step_limit,
            "max_visits_per_raw_instruction_and_ir_block": opts.loop_bound,
            "max_recursion_depth": 0,
            "max_raw_local_branch_depth": execute::RAW_BRANCH_DEPTH,
            "max_ir_path_depth": execute::IR_BRANCH_DEPTH,
            "max_machine_path_depth": raw_function::FUNCTION_BRANCH_DEPTH,
            "solver_wall_ms": opts.timeout.as_millis() as u64,
        },
        "provenance": {
            "language": prog.arch().archid,
            "sla_sha256": prog.arch().decode_recipe().map(|recipe| oracle::sha256(recipe.sla.as_ref())),
            "solver": opts.solver,
        },
        "limitations": [
            "raw-and-final-ir-share-sleigh-and-pcode-semantics",
            if whole { "one-byte-near-returns-reference-a-fixed-unmodified-sentinel-slot" }
            else { "return-instruction-and-effects-outside-the-scope-are-not-proven" },
        ],
    });
    let Encoding {
        proof,
        mutation,
        ir_blocks,
        ir_phis,
    } = match encode(prog, fd, opts, entry..end, regions) {
        Ok(encoded) => encoded,
        Err(reason) => {
            result["classification"] = json!("INCONCLUSIVE");
            result["phase"] = json!("encoding");
            result["reason"] = json!(reason);
            return result;
        }
    };
    result["mutation"] = json!(mutation);
    result["scope"]["ir_blocks"] = json!(ir_blocks);
    result["scope"]["ir_phis"] = json!(ir_phis);
    let deadline = Instant::now() + opts.timeout;
    let mut version = Command::new(opts.solver);
    version.arg("-version");
    let version = solver::run_text(
        &mut version,
        "",
        deadline.min(Instant::now() + Duration::from_millis(500)),
    );
    result["provenance"]["solver_version"] = json!(version.ok().map(|text| text.trim().to_owned()));
    let mut solver = Command::new(opts.solver);
    solver.args(["-in", "-smt2"]);
    let (obligation, memory_inputs) = match proof {
        Proof::Observations {
            obligation,
            memory_inputs,
        } => (obligation, memory_inputs),
        Proof::Induction(induction) => {
            let report = induction.solve(&mut solver, deadline);
            result["classification"] = report["classification"].clone();
            if let Some(phase) = report.get("phase") {
                result["phase"] = phase.clone();
                result["reason"] = report["reason"].clone();
            }
            result["induction"] = report;
            return result;
        }
    };
    let domain = obligation.domain_query();
    let difference = obligation.difference_query();
    result["obligation"] = json!({
        "domain_sha256": oracle::sha256(domain.as_bytes()),
        "difference_sha256": oracle::sha256(difference.as_bytes()),
        "domain_smt2": domain, "difference_smt2": difference,
    });
    if let Some(coverage) = obligation.coverage_query() {
        result["obligation"]["coverage_sha256"] = json!(oracle::sha256(coverage.as_bytes()));
        result["obligation"]["coverage_smt2"] = json!(coverage);
    }
    if let (Some(completion), Some(bounded)) = (
        obligation.completion_query(),
        obligation.bounded_domain_query(),
    ) {
        result["obligation"]["completion_sha256"] = json!(oracle::sha256(completion.as_bytes()));
        result["obligation"]["completion_smt2"] = json!(completion);
        result["obligation"]["bounded_domain_sha256"] = json!(oracle::sha256(bounded.as_bytes()));
        result["obligation"]["bounded_domain_smt2"] = json!(bounded);
    }
    match obligation.solve(&mut solver, deadline) {
        Verdict::Equivalent => result["classification"] = json!("MODEL_EQUIVALENCE"),
        Verdict::BoundedEquivalent => {
            result["classification"] = json!("BOUNDED_EQUIVALENCE");
            result["scope"]["proven_domain"] =
                json!("inputs-completing-on-both-sides-within-bound");
        }
        Verdict::InfeasibleDomain => {
            result["classification"] = json!("INCONCLUSIVE");
            result["phase"] = json!("initial-domain");
            result["reason"] = json!("unsatisfiable-precondition");
        }
        Verdict::Unresolved { phase, reason } => {
            result["classification"] = json!("INCONCLUSIVE");
            result["phase"] = json!(phase);
            result["reason"] = json!(reason);
        }
        Verdict::Counterexample(bytes) => {
            let names = &obligation.input_bytes;
            let mut registers = opts
                .inputs
                .iter()
                .zip(names.chunks_exact(8))
                .map(|(register, symbols)| {
                    let mut encoded = [0u8; 8];
                    for (dest, symbol) in encoded.iter_mut().zip(symbols) {
                        *dest = bytes[symbol];
                    }
                    (
                        register.as_str(),
                        format!("{:#x}", u64::from_le_bytes(encoded)),
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>();
            let mut flags = 2u64;
            for ((_, bit), symbol) in oracle::SYMBOLIC_FLAGS
                .into_iter()
                .zip(&names[opts.inputs.len() * 8..])
            {
                flags |= u64::from(bytes[symbol]) << bit;
            }
            registers.insert("RFLAGS", format!("{flags:#x}"));
            result["classification"] = json!("COUNTEREXAMPLE");
            let memory = memory_inputs
                .iter()
                .map(|(address, symbol)| {
                    (format!("{address:#x}"), format!("{:#04x}", bytes[symbol]))
                })
                .collect::<std::collections::BTreeMap<_, _>>();
            result["witness"] = json!({ "registers": registers, "memory": memory, "independent_replay": "pending" });
        }
    }
    result
}

fn encode(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    opts: &Options<'_>,
    extent: std::ops::Range<u64>,
    regions: Vec<Region>,
) -> Result<Encoding, String> {
    let mut env = Env::new(prog, 0).map_err(|halt| halt.render())?;
    let explicit = ExplicitState {
        regions,
        ..ExplicitState::default()
    };
    if let Some(stop) = opts.stop {
        explicit
            .check(stop, 1, b'x')
            .map_err(|halt| halt.render())?;
    }
    env.explicit = Some(explicit);
    let mut declarations = String::from("(set-logic QF_BV)\n");
    let mut inputs = ByteFile::default();
    let mut names = Vec::new();
    let mut offsets = std::collections::BTreeMap::new();
    for reg in opts.inputs {
        if !oracle::GPR.contains(&reg.as_str()) || reg == "RSP" {
            return Err(format!("unsupported symbolic input register {reg}"));
        }
        let physical = oracle::register(&env, reg)?;
        if physical.size != 8 || offsets.insert(reg.as_str(), names.len()).is_some() {
            return Err(format!(
                "invalid or duplicate symbolic input register {reg}"
            ));
        }
        for byte in 0..8 {
            let name = format!("b{}", names.len());
            let _ = writeln!(declarations, "(declare-const {name} (_ BitVec 8))");
            inputs.write(
                physical.off + byte,
                1,
                &Rc::new(Expr::Var(Rc::from(name.as_str()))),
            )?;
            names.push(name);
        }
    }
    for (reg, _) in oracle::SYMBOLIC_FLAGS {
        let physical = oracle::register(&env, reg)?;
        if physical.size != 1 {
            return Err(format!("invalid symbolic x86 flag storage for {reg}"));
        }
        let name = format!("b{}", names.len());
        let _ = writeln!(
            declarations,
            "(declare-const {name} (_ BitVec 8))\n(assert (bvule {name} #x01))"
        );
        inputs.write(
            physical.off,
            1,
            &Rc::new(Expr::Var(Rc::from(name.as_str()))),
        )?;
        names.push(name);
    }
    for (reg, value) in opts.bindings {
        let start = *offsets
            .get(reg.as_str())
            .ok_or_else(|| format!("binding refers to undeclared register {reg}"))?;
        for (name, byte) in names[start..start + 8].iter().zip(value.to_le_bytes()) {
            let _ = writeln!(declarations, "(assert (= {name} (_ bv{byte} 8)))");
        }
    }
    for (reg, low, high) in opts.ranges {
        let start = *offsets
            .get(reg.as_str())
            .ok_or_else(|| format!("range refers to undeclared register {reg}"))?;
        let word = names[start..start + 8]
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        let _ = writeln!(declarations, "(assert (and (bvule (_ bv{low} 64) (concat {word})) (bvule (concat {word}) (_ bv{high} 64))))");
    }
    let memory = prepare_memory(&env, opts.memory, &mut declarations, &mut names)?;
    let rsp = oracle::register(&env, "RSP")?;
    if opts.stop.is_none() || memory.0.enabled() {
        env.sp0 = oracle::SP;
        inputs.write(
            rsp.off,
            rsp.size,
            &Rc::new(Expr::Const(u128::from(env.sp0), 64)),
        )?;
    }
    let physical = oracle::register(&env, opts.output)?;
    if !matches!(opts.output, "RAX" | "RDX") || !matches!(opts.output_bytes, 1 | 2 | 4 | 8) {
        return Err("use an integer return projection of RAX or RDX, 1, 2, 4 or 8 bytes".into());
    }
    let projection = crate::storage::Storage::new(&physical.space, physical.off, opts.output_bytes);
    let claim = abi::claim(prog, fd);
    if claim.float
        || !claim
            .storage
            .as_ref()
            .is_some_and(|out| projection.covers(out) && out.size == projection.size)
    {
        return Err("the final IR return storage differs from the selected projection".into());
    }
    let mut sites = calls::sites(fd, &env);
    let mut graph = program::compile(prog, fd, &sites);
    let mutation = oracle::apply_opcode_mutant(&mut graph, &mut sites, &env, opts.opcode_mutant)?;
    let ir_blocks = graph.blocks.len();
    let ir_phis = graph.blocks.iter().map(|block| block.phis.len()).sum();
    if opts.induct {
        if opts.stop.is_some()
            || opts.loop_bound.is_some()
            || opts
                .memory
                .iter()
                .any(|window| window.mode == MemoryMode::PrivateStack)
        {
            return Err(
                "induction requires whole-function mode without a visit bound or private stack windows".into(),
            );
        }
        let induction = induction::Function {
            env: &env,
            extent,
            program: &graph,
            inputs: &inputs,
            memory: &memory.0,
            output: &projection,
            stack_pointer: &rsp,
            options: opts,
        }
        .infer(declarations)?;
        return Ok(Encoding {
            proof: Proof::Induction(induction),
            mutation,
            ir_blocks,
            ir_phis,
        });
    }
    let raw = if let Some(stop) = opts.stop {
        execute::raw_block(
            &env,
            &inputs,
            &memory.0,
            extent.start,
            stop,
            &projection,
            opts.step_limit,
        )?
    } else {
        raw_function::RawFunction {
            env: &env,
            extent,
            output: &projection,
            stack_pointer: &rsp,
            indirect: opts.indirect,
            step_limit: opts.step_limit,
            loop_bound: opts.loop_bound,
            boundary: None,
        }
        .run(execute::RawState::new(&inputs, &memory.0)?)?
    };
    let ir = execute::ir_block(
        &graph,
        &inputs,
        &memory.0,
        &env,
        projection.size,
        opts,
        None,
    )?;
    let mut emitter = Emitter::default();
    let observations = terms::observed_terms(&mut emitter, raw, ir, opts.loop_bound.is_some())?;
    declarations.push_str(emitter.definitions());
    Ok(Encoding {
        proof: Proof::Observations {
            obligation: Obligation {
                domain: declarations,
                mismatch: observations.mismatch,
                coverage_gap: observations.coverage_gap,
                completion_gap: observations.completion_gap,
                input_bytes: names,
            },
            memory_inputs: memory.1,
        },
        mutation,
        ir_blocks,
        ir_phis,
    })
}

fn prepare_memory(
    env: &Env,
    ranges: &[MemoryRange],
    declarations: &mut String,
    names: &mut Vec<String>,
) -> Result<(Memory, Vec<(u64, String)>), String> {
    use std::collections::BTreeMap;
    if ranges.iter().map(|r| u64::from(r.size)).sum::<u64>() > 256 {
        return Err("symbolic memory windows may contain at most 256 bytes in total".into());
    }
    let image = &env.explicit.as_ref().ok_or("no ELF memory map")?.regions;
    if !ranges.is_empty()
        && image.iter().any(|r| {
            let collides = |base, end| r.address < end && base < r.end();
            collides(oracle::HEAP, oracle::HEAP + 2 * oracle::PAGE)
                || collides(oracle::STACK, oracle::CALLER + oracle::PAGE)
        })
    {
        return Err("ELF overlaps the symbolic stack or heap".into());
    }
    let mut windows = Vec::new();
    let mut bytes = BTreeMap::new();
    let mut symbols = Vec::new();
    for range in ranges {
        let end = range
            .address
            .checked_add(u64::from(range.size))
            .filter(|&end| end > range.address && end <= 1 << 47)
            .ok_or("invalid symbolic memory window")?;
        let elf = image
            .iter()
            .find(|r| r.address <= range.address && end <= r.end());
        let synthetic_shared = (range.address >= oracle::HEAP
            && end <= oracle::HEAP + 2 * oracle::PAGE)
            || (range.address >= oracle::CALLER && end <= oracle::CALLER + oracle::PAGE);
        let allowed = match range.mode {
            MemoryMode::Shared => {
                synthetic_shared || elf.is_some_and(|r| r.permissions.contains('w'))
            }
            MemoryMode::PrivateStack => range.address >= oracle::STACK && end <= oracle::SP,
            MemoryMode::Image => {
                elf.is_some_and(|r| r.permissions.contains('r') && !r.permissions.contains('w'))
            }
        };
        if !allowed {
            return Err(format!(
                "{} memory window at {:#x} is outside its declared backing",
                range.mode.name(),
                range.address
            ));
        }
        windows.push(Window {
            start: range.address,
            size: range.size,
            writable: range.mode != MemoryMode::Image,
            observed: range.mode == MemoryMode::Shared,
            private_stack: range.mode == MemoryMode::PrivateStack,
        });
        for address in range.address..end {
            let term = if range.mode == MemoryMode::Image {
                let value = elf
                    .ok_or("no symbolic image bytes at this address")?
                    .initial_byte(address);
                Rc::new(Expr::Const(u128::from(value), 8))
            } else {
                let name = format!("b{}", names.len());
                let _ = writeln!(declarations, "(declare-const {name} (_ BitVec 8))");
                let term = Rc::new(Expr::Var(Rc::from(name.as_str())));
                names.push(name.clone());
                symbols.push((address, name));
                term
            };
            if bytes.insert(address, term).is_some() {
                return Err("symbolic memory windows overlap".into());
            }
        }
    }
    Ok((Memory::new(windows, bytes), symbols))
}
