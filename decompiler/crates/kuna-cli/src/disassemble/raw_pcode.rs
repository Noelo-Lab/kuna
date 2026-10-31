//! Instruction-level SLEIGH output before the decompiler changes it.

mod flow;

use super::*;
use kuna_console::engine::RawPcodeVarnode;
use serde_json::{json, Value};

pub fn run(argv: &[String]) -> i32 {
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("usage: kuna raw-pcode <binary> <name|0xaddr|0xstart-0xend> [--addr] [--count N] [--bytes N] [--json] [--follow] [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]");
        return 0;
    }
    let args = match parse_args(argv) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };
    if args.view.is_some() {
        eprintln!("error: --as does not apply to raw-pcode");
        return 2;
    }
    let result = if args.follow {
        flow::render(&args)
    } else {
        render_raw(&args)
    };
    match result {
        Ok((text, status)) => crate::output::emit_with_status(&text, status),
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn varnode(v: &RawPcodeVarnode) -> Value {
    json!({"space": v.space, "offset": v.offset, "offset_hex": format!("0x{:x}", v.offset), "size": v.size})
}

fn render_raw(args: &DisArgs) -> Result<(String, i32), String> {
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
    let mut failure: Option<(u64, String)> = None;
    while instructions.len() < limit && region.stop().is_none_or(|end| addr < end) {
        let decoded = (|| -> Result<_, String> {
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
            let bytes = prog
                .read_bytes(addr, size as usize)
                .ok_or_else(|| format!("cannot read instruction bytes at 0x{addr:x}"))?;
            Ok((size, ops, next, mnemonic, operands, bytes))
        })();
        let (size, ops, next, mnemonic, operands, bytes) = match decoded {
            Ok(decoded) => decoded,
            Err(message) => {
                failure = Some((addr, message));
                break;
            }
        };
        let bytes = hex(&bytes);
        text.push_str(&format!("0x{addr:x}: {bytes}  {mnemonic} {operands}\n"));
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
        instructions.push(json!({"address": addr, "address_hex": format!("0x{addr:x}"), "size": size, "bytes": bytes, "mnemonic": mnemonic, "operands": operands, "ops": json_ops}));
        addr = next;
    }
    let stop_reason = if failure.is_some() {
        "decode_error"
    } else if instructions.len() == limit && region.stop().is_none_or(|end| addr < end) {
        "instruction_limit"
    } else if region.reaches_past_image()
        && region.mapped_end.is_some_and(|end| addr >= end)
        && args.count.is_none_or(|count| instructions.len() < count)
    {
        "mapped_end"
    } else {
        "range_end"
    };
    let truncated = stop_reason != "range_end";
    let status = if failure.is_some() { 1 } else { 0 };

    if args.json {
        let error = failure.as_ref().map(|(address, message)| {
            json!({
                "address": address, "address_hex": format!("0x{address:x}"), "message": message
            })
        });
        Ok((
            format!("{}\n", serde_json::to_string_pretty(&json!({
            "binary": args.binary, "start": region.start, "start_hex": format!("0x{:x}", region.start), "end": addr, "end_hex": format!("0x{addr:x}"), "truncated": truncated,
            "stop_reason": stop_reason, "error": error, "instructions": instructions
        })).map_err(|e| e.to_string())?),
            status,
        ))
    } else {
        if let Some((address, message)) = &failure {
            text.push_str(&format!("# stopped at 0x{address:x}: {message}\n"));
        } else if truncated {
            text.push_str(&format!("# truncated at 0x{addr:x} ({stop_reason})\n"));
        }
        Ok((text, status))
    }
}
