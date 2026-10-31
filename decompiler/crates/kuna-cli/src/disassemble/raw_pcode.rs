//! `kuna raw-pcode` — each instruction's SLEIGH p-code, before the decompiler
//! changes any of it.
//!
//! The operand, the program load and the window are `kuna disassemble`'s
//! ([`load_target`], [`resolve_region`]), including the windowed load that
//! answers a caller-bounded window without the whole-image discovery walk. Each
//! row is one `Translate::one_instruction` lift ([`ConsoleProgram::raw_pcode_at`])
//! beside the bytes it consumed and its `print_assembly` text. Unlike the
//! listing, a row is never guessed: the walk stops at the first lift that fails
//! or that would run past the window, keeps what it decoded, and exits 1.
//!
//! An odd ARM32 address is a Thumb code pointer, so it is lifted as Thumb at the
//! even address, in a context scoped to this query
//! ([`ConsoleProgram::with_thumb_pointer`]). `--follow` walks direct flow
//! instead of the straight line ([`flow`]).

mod flow;

use super::*;
use kuna_console::engine::{ArmIsa, RawPcodeOp, RawPcodeVarnode};

/// What one query produced: the document for stdout, the notes for stderr, and
/// the failure that cut it short, if one did (exit 1, its decoded part kept).
struct Output {
    text: String,
    notes: Vec<String>,
    error: Option<String>,
}

/// `kuna raw-pcode` entry point.
pub fn run(argv: &[String]) -> i32 {
    if argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        usage();
        return 0;
    }
    let args = match parse_args_for(argv, "raw-pcode") {
        Ok(args) if args.view.is_some() => {
            eprintln!("error: --as does not apply to raw-pcode");
            return 2;
        }
        Ok(args) => args,
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };
    let rendered = if args.follow {
        flow::render(&args)
    } else {
        render(&args)
    };
    match rendered {
        Ok(out) => {
            for note in &out.notes {
                eprintln!("note: {note}");
            }
            if let Some(error) = &out.error {
                eprintln!("error: {error}");
            }
            crate::output::emit_with_status(&out.text, i32::from(out.error.is_some()))
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Load and resolve the target as `disassemble` does, then run `walk` over it:
/// in Thumb mode at the even address when the operand is an odd ARM32 pointer.
fn with_target<R>(
    args: &DisArgs,
    bounded: bool,
    walk: impl FnOnce(&ConsoleProgram, &DisArgs, Region) -> Result<R, String>,
) -> Result<R, String> {
    let (prog, region) = load_target(args, ViewRequest::Code, bounded)?;
    let Some(vma) = thumb_pointer(&prog, args)? else {
        return walk(&prog, args, region);
    };
    let args = DisArgs {
        spec: format!("0x{vma:x}"),
        ..args.clone()
    };
    let region = resolve_region(&prog, &args)?;
    prog.with_thumb_pointer(vma, || walk(&prog, &args, region))
        .map_err(|e| e.to_string())?
}

/// The even address an odd ARM32 code pointer names, when the operand is one.
/// A range is read literally, and `--isa arm` contradicts the pointer.
fn thumb_pointer(prog: &ConsoleProgram, args: &DisArgs) -> Result<Option<u64>, String> {
    let spec = args.spec.trim();
    if !prog.is_arm32()
        || split_range(spec).is_some()
        || !(args.by_address || looks_like_addr(spec))
    {
        return Ok(None);
    }
    let addr = parse_addr(spec)?;
    if addr & 1 == 0 {
        return Ok(None);
    }
    if args.isa == Some(ArmIsa::Arm) {
        return Err("an odd ARM code pointer requests Thumb, conflicting with --isa arm".into());
    }
    Ok(Some(addr & !1))
}

/// The notes a start in a non-executable data section earns: the lift is what
/// was asked for, but `disassemble` would have shown those bytes as data.
fn data_notes(prog: &ConsoleProgram, region: &Region) -> Vec<String> {
    let section = section_flags_at(prog, region.start);
    if decide_view(ViewRequest::Auto, region.from_entry, section) == View::Code {
        return Vec::new();
    }
    vec![format!(
        "0x{:x} is in a non-executable data section; its bytes are lifted as \
         instructions anyway -- `kuna read` shows them as bytes",
        region.start
    )]
}

/// One instruction as SLEIGH lifts it.
struct Lift {
    addr: u64,
    size: u64,
    bytes: Vec<u8>,
    mnemonic: String,
    operands: String,
    ops: Vec<RawPcodeOp>,
}

/// Lift the instruction at `addr`, refusing one that does not decode or that
/// would run past `stop`.
fn lift(prog: &ConsoleProgram, addr: u64, stop: Option<u64>) -> Result<Lift, String> {
    let (size, ops) = prog
        .raw_pcode_at(addr)
        .map_err(|e| format!("cannot lift instruction at 0x{addr:x}: {e}"))?;
    let size = u64::try_from(size)
        .ok()
        .filter(|&n| n > 0)
        .ok_or_else(|| format!("invalid instruction length at 0x{addr:x}"))?;
    let next = addr
        .checked_add(size)
        .ok_or("instruction address overflow")?;
    if stop.is_some_and(|end| next > end) {
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
    Ok(Lift {
        addr,
        size,
        bytes,
        mnemonic,
        operands,
        ops,
    })
}

impl Lift {
    fn end(&self) -> u64 {
        self.addr + self.size
    }

    /// `0xADDR: BYTES  MNEMONIC operands`, then one indented line per op.
    fn text(&self) -> String {
        let mut out = format!("0x{:x}: {}  {}", self.addr, hex(&self.bytes), self.mnemonic);
        if !self.operands.is_empty() {
            out.push(' ');
            out.push_str(&self.operands);
        }
        out.push('\n');
        for op in &self.ops {
            out.push_str("  ");
            if let Some(output) = &op.output {
                out.push_str(&spelled(output));
                out.push_str(" = ");
            }
            out.push_str(op.opcode);
            let inputs: Vec<String> = op.inputs.iter().map(spelled).collect();
            if !inputs.is_empty() {
                out.push(' ');
                out.push_str(&inputs.join(", "));
            }
            out.push('\n');
        }
        out
    }

    fn json(&self) -> Json {
        let ops = self
            .ops
            .iter()
            .map(|op| {
                Json::Object(vec![
                    ("opcode".into(), Json::Str(op.opcode.to_string())),
                    (
                        "output".into(),
                        op.output.as_ref().map_or(Json::Null, varnode_json),
                    ),
                    (
                        "inputs".into(),
                        Json::Array(op.inputs.iter().map(varnode_json).collect()),
                    ),
                ])
            })
            .collect();
        Json::Object(vec![
            ("address".into(), Json::Number(self.addr.to_string())),
            (
                "address_hex".into(),
                Json::Str(format!("0x{:x}", self.addr)),
            ),
            ("size".into(), Json::Number(self.size.to_string())),
            ("bytes".into(), Json::Str(hex(&self.bytes))),
            ("mnemonic".into(), Json::Str(self.mnemonic.clone())),
            ("operands".into(), Json::Str(self.operands.clone())),
            ("ops".into(), Json::Array(ops)),
        ])
    }
}

/// The `(space,offset,size)` spelling Ghidra's raw p-code listing uses.
fn spelled(v: &RawPcodeVarnode) -> String {
    format!("({},{:#x},{})", v.space, v.offset, v.size)
}

fn varnode_json(v: &RawPcodeVarnode) -> Json {
    Json::Object(vec![
        ("space".into(), Json::Str(v.space.clone())),
        ("offset".into(), Json::Number(v.offset.to_string())),
        ("offset_hex".into(), Json::Str(format!("0x{:x}", v.offset))),
        ("size".into(), Json::Number(v.size.to_string())),
    ])
}

/// `members` rendered as [`dumps_indent2`] renders them with one more member,
/// `key`, holding `rows` — each already rendered on its own. A lift is about
/// ten times its own text as a tree of values, so a long listing keeps only the
/// text.
fn object_with_rows(mut members: Vec<(String, Json)>, key: &str, rows: &[String]) -> String {
    members.push((key.to_string(), Json::Array(Vec::new())));
    let mut out = dumps_indent2(&Json::Object(members));
    out.truncate(out.len() - "[]\n}".len());
    out.push('[');
    for (i, row) in rows.iter().enumerate() {
        out.push_str(if i == 0 { "\n    " } else { ",\n    " });
        out.push_str(&row.replace('\n', "\n    "));
    }
    if !rows.is_empty() {
        out.push_str("\n  ");
    }
    out.push_str("]\n}");
    out
}

/// The straight-line query: lift forward from the start until the window, the
/// count, the derived cap or a failed lift ends it.
fn render(args: &DisArgs) -> Result<Output, String> {
    with_target(
        args,
        window_is_caller_bounded(args),
        |prog, args, region| {
            require_mapped(args, prog, &region)?;
            let notes = data_notes(prog, &region);
            let limit = args.count.unwrap_or(if region.derived {
                DERIVED_INSTRUCTION_CAP
            } else {
                usize::MAX
            });
            let stop = region.stop();
            let (mut addr, mut listed) = (region.start, 0usize);
            let (mut text, mut rows) = (String::new(), Vec::new());
            let mut failure = None;
            while listed < limit && stop.is_none_or(|end| addr < end) {
                let lifted = match lift(prog, addr, stop) {
                    Ok(lifted) => lifted,
                    Err(message) => {
                        failure = Some(message);
                        break;
                    }
                };
                if args.json {
                    rows.push(dumps_indent2(&lifted.json()));
                } else {
                    text.push_str(&lifted.text());
                }
                listed += 1;
                addr = lifted.end();
            }
            let stop_reason = if failure.is_some() {
                "decode_error"
            } else if listed == limit && stop.is_none_or(|end| addr < end) {
                "instruction_limit"
            } else if region.reaches_past_image()
                && region.mapped_end.is_some_and(|end| addr >= end)
                && args.count.is_none_or(|count| listed < count)
            {
                "mapped_end"
            } else {
                "range_end"
            };
            let truncated = stop_reason != "range_end";
            let text = if args.json {
                let error = failure.as_ref().map_or(Json::Null, |message| {
                    Json::Object(vec![
                        ("address".into(), Json::Number(addr.to_string())),
                        ("address_hex".into(), Json::Str(format!("0x{addr:x}"))),
                        ("message".into(), Json::Str(message.clone())),
                    ])
                });
                let mut doc = envelope(args, &region, addr, listed, truncated, &notes, "pcode");
                doc.push(("stop_reason".into(), Json::Str(stop_reason.into())));
                doc.push(("error".into(), error));
                object_with_rows(doc, "instructions", &rows) + "\n"
            } else {
                let plural = if listed == 1 {
                    "instruction"
                } else {
                    "instructions"
                };
                let mut out = format!(
                    "# {listed} {plural} at {} (0x{:x}..0x{addr:x}, {} bytes)\n",
                    label(&region),
                    region.start,
                    addr - region.start
                );
                out.push_str(&text);
                if let Some(message) = &failure {
                    out.push_str(&format!("# stopped at 0x{addr:x}: {message}\n"));
                } else if truncated {
                    out.push_str(&format!("# truncated at 0x{addr:x} ({stop_reason})\n"));
                }
                out
            };
            Ok(Output {
                text,
                notes,
                error: failure,
            })
        },
    )
}

fn usage() {
    eprintln!(
        "usage: kuna raw-pcode <binary> <name|0xaddr|0xstart-0xend> [--addr] \\\n\
         \x20                [--count N] [--bytes N] [--follow] [--json] \\\n\
         \x20                [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                [--option N V].. [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Each instruction's SLEIGH p-code with its bytes, before any decompiler pass.\n\
         The target and its extent are `kuna disassemble`'s. The walk stops at the\n\
         first instruction that does not lift and exits 1, keeping what it lifted.\n\
         An odd ARM32 address lifts as Thumb at the even address.\n\
         \n\
         --follow walks direct branches and fall-through from the start instead of\n\
         the straight line, and groups the lifts into blocks with their successors.\n\
         \n\
         --json emits {{binary,kind,target,start,end,count,bytes,truncated,notes,\n\
         stop_reason,error}} plus instructions:[{{address,address_hex,size,bytes,\n\
         mnemonic,operands,ops:[{{opcode,output,inputs}}]}}]; with --follow,\n\
         {{binary,kind,target,start,range_end,complete,instruction_count,notes,issues}}\n\
         plus blocks:[{{start,successors,instructions}}]."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumb_pointer_context_is_restored_after_success_and_error() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../kuna-analysis/tests/fixtures/arm_interwork_le32");
        let args = parse_args(&[fixture.to_str().unwrap().into(), "0x2000000".into()]).unwrap();
        let prog = load_program(&load_args(&args, Vec::new()), DriverDefaults::Inventory).unwrap();
        let original = prog.disassemble_at(0x2000000).unwrap();
        assert_eq!(original.0, 4);
        let thumb = prog
            .with_thumb_pointer(0x2000000, || prog.disassemble_at(0x2000000))
            .unwrap()
            .unwrap();
        assert_eq!(thumb.0, 2);
        assert_eq!(prog.disassemble_at(0x2000000).unwrap(), original);
        let result = prog
            .with_thumb_pointer(0x2000000, || -> Result<(), &str> { Err("stop") })
            .unwrap();
        assert_eq!(result, Err("stop"));
        assert_eq!(prog.disassemble_at(0x2000000).unwrap(), original);
    }

    #[test]
    fn rows_rendered_one_by_one_splice_into_the_document_dumps_would_render() {
        let row = |n: u64| {
            Json::Object(vec![
                ("address".into(), Json::Number(n.to_string())),
                (
                    "ops".into(),
                    Json::Array(vec![Json::Object(vec![("x".into(), Json::Null)])]),
                ),
            ])
        };
        let members = || vec![("binary".to_string(), Json::Str("a.out".into()))];
        for count in [0, 1, 3] {
            let rows: Vec<Json> = (0..count).map(row).collect();
            let rendered: Vec<String> = rows.iter().map(dumps_indent2).collect();
            let mut whole = members();
            whole.push(("instructions".into(), Json::Array(rows)));
            assert_eq!(
                object_with_rows(members(), "instructions", &rendered),
                dumps_indent2(&Json::Object(whole))
            );
        }
    }
}
