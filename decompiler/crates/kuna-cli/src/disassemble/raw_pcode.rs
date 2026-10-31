//! Instruction-level SLEIGH output before the decompiler changes it.

use super::*;
use kuna_console::engine::RawPcodeVarnode;
use serde_json::{json, Value};

pub fn run(argv: &[String]) -> i32 {
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("usage: kuna raw-pcode <binary> <name|0xaddr|0xstart-0xend> [--addr] [--count N] [--bytes N] [--json] [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]");
        return 0;
    }
    let args = match parse_args(argv) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };
    if args.follow || args.view.is_some() {
        eprintln!("error: --follow and --as do not apply to raw-pcode");
        return 2;
    }
    match render_raw(&args) {
        Ok(text) => crate::output::emit_with_status(&text, 0),
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn varnode(v: &RawPcodeVarnode) -> Value {
    json!({"space": v.space, "offset": v.offset, "offset_hex": format!("0x{:x}", v.offset), "size": v.size})
}

fn render_raw(args: &DisArgs) -> Result<String, String> {
    let options =
        mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
    let prog = load_program(&load_args(args, options), DriverDefaults::Inventory)?;
    let region = resolve_region(&prog, args)?;
    if !prog.vma_bytes_mapped(region.start) {
        return Err(format!("no bytes mapped at 0x{:x}", region.start));
    }
    let limit = args.count.unwrap_or(if region.derived {
        DERIVED_INSTRUCTION_CAP
    } else {
        usize::MAX
    });
    let mut addr = region.start;
    let mut instructions = Vec::new();
    let mut text = String::new();
    while instructions.len() < limit && region.stop().is_none_or(|end| addr < end) {
        let (size, ops) = prog
            .raw_pcode_at(addr)
            .map_err(|e| format!("cannot lift instruction at 0x{addr:x}: {e}"))?;
        if size <= 0 {
            return Err(format!("invalid instruction length at 0x{addr:x}"));
        }
        let next = addr
            .checked_add(size as u64)
            .ok_or("instruction address overflow")?;
        if region.stop().is_some_and(|end| next > end) {
            return Err(format!(
                "instruction at 0x{addr:x} crosses the requested or mapped boundary"
            ));
        }
        let (_, mnemonic, operands) = prog
            .disassemble_at(addr)
            .map_err(|e| format!("cannot disassemble 0x{addr:x}: {e}"))?;
        text.push_str(&format!("0x{addr:x}: {mnemonic} {operands}\n"));
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
            json_ops.push(json!({"opcode": op.opcode, "output": output, "inputs": inputs}));
        }
        instructions.push(json!({"address": addr, "address_hex": format!("0x{addr:x}"), "size": size, "mnemonic": mnemonic, "operands": operands, "ops": json_ops}));
        addr = next;
    }
    let truncated = (instructions.len() == limit && region.stop().is_none_or(|end| addr < end))
        || (region.reaches_past_image()
            && region.mapped_end.is_some_and(|end| addr >= end)
            && args.count.is_none_or(|count| instructions.len() < count));
    if args.json {
        Ok(format!("{}\n", serde_json::to_string_pretty(&json!({"binary": args.binary, "start": region.start, "end": addr, "truncated": truncated, "instructions": instructions})).map_err(|e| e.to_string())?))
    } else {
        if truncated {
            text.push_str(&format!("# truncated at 0x{addr:x}\n"));
        }
        Ok(text)
    }
}
