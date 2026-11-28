//! Export the Kuna terms that an independent Sail trace can be compared with.

use std::fmt::Write as _;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;
use serde_json::{json, Value};

use crate::concolic::{Emitter, Expr};
use crate::state::{Env, ExplicitState};
use crate::{abi, calls, oracle, program, session};

use super::byte_file::ByteFile;
use super::execute::{self, Instruction, RawState, RAW_BRANCH_DEPTH};
use super::memory::Memory;

pub fn export(
    binary: &str,
    function: &str,
    spec_roots: &[String],
    mutant: Option<usize>,
) -> Result<Value, String> {
    let image = std::fs::read(binary).map_err(|err| format!("{binary}: {err}"))?;
    let mut regions = oracle::elf::riscv_regions(&image)?;
    regions.sort_unstable_by_key(|r| r.address);
    if regions.windows(2).any(|w| w[0].end() > w[1].address) {
        return Err("overlapping ELF pages are outside the RV64 model".into());
    }
    let mut state = Some(ExplicitState {
        regions,
        ..ExplicitState::default()
    });
    let mut result = None;
    let commands = session::binary_script(binary, &[function.to_owned()], &[]);
    session::drive(commands, None, spec_roots, |prog, fd| {
        if let Some(state) = state.take() {
            result = Some(encode(prog, fd, &image, state, mutant));
        }
    })?;
    let mut report = result.ok_or("no RV64 function was decompiled")??;
    report["provenance"]["elf_sha256"] = json!(oracle::sha256(&image));
    Ok(report)
}

fn encode(
    prog: &ConsoleProgram,
    fd: &Funcdata,
    image: &[u8],
    explicit: ExplicitState,
    mutant: Option<usize>,
) -> Result<Value, String> {
    let entry = fd.get_address().get_offset();
    let end = super::function_end(image, entry)?;
    if end - entry != 8 || entry % 4 != 0 {
        return Err(
            "the RV64 model needs one 32-bit instruction followed by a 32-bit return".into(),
        );
    }
    explicit
        .check(entry, 8, b'x')
        .map_err(|halt| halt.render())?;
    let word = |at| u32::from_le_bytes(std::array::from_fn(|i| explicit.byte(at + i as u64)));
    let opcode = word(entry);
    if word(entry + 4) != 0x0000_8067 {
        return Err("the RV64 leaf must end with jalr x0,0(ra)".into());
    }
    let sources = source_registers(opcode)?;
    let mut env = Env::new(prog, 0).map_err(|halt| halt.render())?;
    env.explicit = Some(explicit);
    let decoded = crate::machine::lift(&env, entry).map_err(|halt| halt.render())?;
    if decoded.len != 4 {
        return Err("Kuna did not decode a single 32-bit RV64 instruction".into());
    }
    // Validate that the ELF bytes of the return also match the decompiler's loader.
    let ret = crate::machine::lift(&env, entry + 4).map_err(|halt| halt.render())?;
    if ret.len != 4 {
        return Err("Kuna did not decode a single 32-bit RV64 return".into());
    }
    let mut declarations = String::from("(set-logic QF_BV)\n");
    let mut inputs = ByteFile::default();
    let zero = oracle::register(&env, "zero")?;
    inputs.write(zero.off, 8, &execute::constant(0, 8)?)?;
    for &index in &sources {
        let name = format!("a{}", index - 10);
        let reg = oracle::register(&env, &name)?;
        if reg.size != 8 {
            return Err(format!("unexpected RV64 width for {name}"));
        }
        let mut names = Vec::new();
        for byte in 0..8 {
            let byte_name = format!("kuna_x{index}_b{byte}");
            let _ = writeln!(declarations, "(declare-const {byte_name} (_ BitVec 8))");
            inputs.write(
                reg.off + byte,
                1,
                &Rc::new(Expr::Var(Rc::from(byte_name.as_str()))),
            )?;
            names.push(byte_name);
        }
        names.reverse();
        let _ = writeln!(
            declarations,
            "(define-fun kuna_x{index} () (_ BitVec 64) (concat {}))",
            names.join(" ")
        );
    }
    let output = oracle::register(&env, "a0")?;
    let claim = abi::claim(prog, fd);
    if claim.float
        || !claim
            .storage
            .as_ref()
            .is_some_and(|reg| output.covers(reg) && reg.size == 8)
    {
        return Err("final IR does not return the full RV64 a0 value".into());
    }
    let memory = Memory::default();
    let mut steps = 0;
    let mut machine = Instruction::with_ops(&env, entry, entry + 4, &decoded.ops, &mut steps, 128);
    let raw = machine
        .run(RawState::new(&inputs, &memory)?, 0, RAW_BRANCH_DEPTH)?
        .observe(&output)?;
    let mut sites = calls::sites(fd, &env);
    let mut graph = program::compile(prog, fd, &sites);
    let mutation = oracle::apply_opcode_mutant(&mut graph, &mut sites, &env, mutant)?;
    let options = super::Options {
        inputs: &[],
        bindings: &[],
        ranges: &[],
        memory: &[],
        stop: Some(entry + 4),
        loop_bound: None,
        induct: false,
        indirect: &Default::default(),
        output: "a0",
        output_bytes: 8,
        step_limit: 128,
        timeout: Duration::ZERO,
        solver: Path::new("z3"),
        opcode_mutant: mutant,
    };
    let ir = execute::ir_block(&graph, &inputs, &memory, &env, 8, &options, None)?;
    let mut emitter = Emitter::default();
    let raw_return = emitter.name(&raw.value);
    let final_return = emitter.name(&ir.value);
    let raw_defined = emitter.name(&raw.defined);
    let ir_defined = emitter.name(&ir.defined);
    declarations.push_str(emitter.definitions());
    Ok(json!({
        "version":1,
        "function":fd.get_name(),
        "pc":format!("{entry:#x}"),
        "opcode":format!("{opcode:08x}"),
        "input_registers":sources,
        "output_register":10,
        "result_size":8,
        "symbols_smt2":declarations,
        "raw_value":raw_return,
        "ir_value":final_return,
        "raw_defined":raw_defined,
        "ir_defined":ir_defined,
        "mutation":mutation,
        "provenance":{
            "language":prog.arch().archid,
            "sla_sha256":prog.arch().decode_recipe().map(|recipe| oracle::sha256(recipe.sla.as_ref())),
        }
    }))
}

fn source_registers(opcode: u32) -> Result<Vec<u32>, String> {
    let rd = (opcode >> 7) & 31;
    let rs1 = (opcode >> 15) & 31;
    let rs2 = (opcode >> 20) & 31;
    let mode = (opcode >> 12) & 7;
    let high = opcode >> 25;
    let mut sources = match opcode & 127 {
        0x13 if matches!(mode, 0 | 2 | 3 | 4 | 6 | 7) => vec![rs1],
        0x13 if (mode == 1 && high >> 1 == 0) || (mode == 5 && matches!(high >> 1, 0 | 16)) => {
            vec![rs1]
        }
        0x33 if high == 0 || (high == 32 && matches!(mode, 0 | 5)) => vec![rs1, rs2],
        _ => return Err("opcode is outside the modeled RV64I ALU subset".into()),
    };
    if rd != 10
        || sources
            .iter()
            .any(|source| *source != 0 && !(10..=17).contains(source))
    {
        return Err("RV64 leaf must write a0 and read a0-a7 or x0".into());
    }
    sources.retain(|source| *source != 0);
    sources.sort_unstable();
    sources.dedup();
    Ok(sources)
}
