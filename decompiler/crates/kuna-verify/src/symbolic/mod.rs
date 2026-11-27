//! Translation validation of an explicitly scoped x86-64 straight-line block.

mod byte_file;
mod execute;
mod obligation;
mod solver;
mod terms;

use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use std::time::{Duration, Instant};

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;
use serde_json::{json, Value};

use crate::concolic::{Emitter, Expr};
use crate::state::{Env, ExplicitState, Region};
use crate::{abi, calls, oracle, program, session};

use byte_file::ByteFile;
use obligation::{Obligation, Verdict};

pub struct Options<'a> {
    pub inputs: &'a [String],
    pub bindings: &'a [(String, u64)],
    pub stop: u64,
    pub output: &'a str,
    pub output_bytes: u32,
    pub step_limit: u64,
    pub timeout: Duration,
    pub solver: &'a Path,
    pub opcode_mutant: Option<usize>,
}

struct Encoding {
    obligation: Obligation,
    mutation: Option<Value>,
    ir_blocks: usize,
    ir_phis: usize,
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
            result = Some(verify_block(prog, fd, opts, regions));
        }
    })?;
    let mut result = result.ok_or("no decompiled function is available")?;
    result["provenance"]["elf_sha256"] = json!(source);
    result["decompile_options"] = json!(decompile_options);
    Ok(result)
}

fn verify_block(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    opts: &Options<'_>,
    regions: Vec<Region>,
) -> Value {
    let entry = fd.get_address().get_offset();
    let mut result = json!({
        "version": 1, "function": fd.get_name(),
        "scope": {
            "architecture": "x86-64", "abi": "sysv-amd64",
            "raw_interval": { "start": format!("{entry:#x}"), "end_exclusive": format!("{:#x}", opts.stop) },
            "observation": { "register": opts.output, "bytes": opts.output_bytes, "arithmetic_definedness": true },
            "symbolic_inputs": opts.inputs,
            "symbolic_flags": oracle::SYMBOLIC_FLAGS.map(|(name, _)| name),
            "bindings": opts.bindings.iter().map(|(r, v)| (r, format!("{v:#x}"))).collect::<Vec<_>>(),
            "max_ops_per_side": opts.step_limit,
            "max_raw_local_branch_depth": execute::RAW_BRANCH_DEPTH,
            "max_ir_path_depth": execute::IR_BRANCH_DEPTH,
            "solver_wall_ms": opts.timeout.as_millis() as u64,
        },
        "provenance": {
            "language": prog.arch().archid,
            "sla_sha256": prog.arch().decode_recipe().map(|recipe| oracle::sha256(recipe.sla.as_ref())),
            "solver": opts.solver,
        },
        "limitations": [
            "raw-and-final-ir-share-sleigh-and-pcode-semantics",
            "return-instruction-and-effects-outside-the-scope-are-not-proven",
        ],
    });
    let Encoding {
        obligation,
        mutation,
        ir_blocks,
        ir_phis,
    } = match encode(prog, fd, opts, entry, regions) {
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
    let domain = obligation.domain_query();
    let difference = obligation.difference_query();
    result["obligation"] = json!({
        "domain_sha256": oracle::sha256(domain.as_bytes()),
        "difference_sha256": oracle::sha256(difference.as_bytes()),
        "domain_smt2": domain, "difference_smt2": difference,
    });
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
    match obligation.solve(&mut solver, deadline) {
        Verdict::Equivalent => result["classification"] = json!("MODEL_EQUIVALENCE"),
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
            result["witness"] = json!({ "registers": registers, "independent_replay": "pending" });
        }
    }
    result
}

fn encode(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    opts: &Options<'_>,
    entry: u64,
    regions: Vec<Region>,
) -> Result<Encoding, String> {
    let mut env = Env::new(prog, 0).map_err(|halt| halt.render())?;
    let explicit = ExplicitState {
        regions,
        ..ExplicitState::default()
    };
    explicit
        .check(opts.stop, 1, b'x')
        .map_err(|halt| halt.render())?;
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
    let raw = execute::raw_block(
        &env,
        &inputs,
        entry,
        opts.stop,
        &projection,
        opts.step_limit,
    )?;
    let ir = execute::ir_block(
        &graph,
        &inputs,
        env.register.get_index(),
        projection.size,
        opts.stop,
        opts.step_limit,
    )?;
    let mut emitter = Emitter::default();
    let mismatch = terms::observed_terms(&mut emitter, raw, ir)?;
    declarations.push_str(emitter.definitions());
    Ok(Encoding {
        obligation: Obligation {
            domain: declarations,
            mismatch,
            input_bytes: names,
        },
        mutation,
        ir_blocks: graph.blocks.len(),
        ir_phis: graph.blocks.iter().map(|block| block.phis.len()).sum(),
    })
}
