//! The display format of one constant, stated by the instruction that reads it.
//!
//! `map convert` and `force varnode` both set the form a constant is printed in
//! (hex, dec, oct, bin, char), and both ask for an identity the C does not
//! show: a dataflow hash, or a varnode with its p-code sequence number. Here
//! the constant is named by the address of the instruction that reads it, with
//! its value where that instruction reads several. The constant gets the same
//! EquateSymbol `force varnode` builds, anchored by its dataflow hash, so the
//! format is carried to the next decompile with the other dynamic Symbols.

use kuna_base::address::Address;
use kuna_base::types::uint4;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::context::VarnodeId;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::varnode::varnode_flags;
use kuna_num::opcodes::OpCode;

/// Why a format that was applied does not show: the next decompile did not
/// find the constant again.
pub const NOT_REPLAYED: &str = "the constant was not found again when the function was decompiled with the format";

/// The console command a request lowers to, which a refusal names.
pub fn console_line(addr: u64, format: &str, value: Option<u64>) -> String {
    let value = value.map(|value| format!(" {value:#x}")).unwrap_or_default();
    format!("force constant {addr:#x} {format}{value}")
}

/// The display-format code of a format word (`Symbol::setDisplayFormat`).
pub fn format_code(word: &str) -> Option<uint4> {
    match word {
        "hex" => Some(1),
        "dec" => Some(2),
        "oct" => Some(3),
        "bin" => Some(4),
        "char" => Some(5),
        _ => None,
    }
}

/// Parse the optional value of a format request: decimal, or hex with `0x`,
/// with an optional leading `-`.
pub fn parse_value(tok: &str) -> Option<u64> {
    let (negative, body) = match tok.strip_prefix('-') {
        Some(body) => (true, body),
        None => (false, tok),
    };
    let magnitude = match body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
        Some(hex) => u64::from_str_radix(hex, 16).ok()?,
        None => body.parse::<u64>().ok()?,
    };
    Some(if negative { magnitude.wrapping_neg() } else { magnitude })
}

/// Is input `slot` of an operation with this opcode a number the C prints,
/// not a space id, a user-op index, a flow target, a truncation amount or an
/// element size?
fn prints_as_number(opc: OpCode, slot: i32) -> bool {
    !matches!(
        (opc, slot),
        (OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER, 0)
            | (
                OpCode::CPUI_BRANCH
                    | OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BRANCHIND
                    | OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_RETURN,
                0,
            )
            | (OpCode::CPUI_SUBPIECE | OpCode::CPUI_PTRSUB | OpCode::CPUI_INDIRECT, 1)
            | (OpCode::CPUI_PTRADD, 2)
            | (OpCode::CPUI_CPOOLREF | OpCode::CPUI_SEGMENTOP | OpCode::CPUI_NEW, _)
    )
}

/// The integer constants the live operations at `at` read and print as
/// numbers, with their values, and the enum one of them prints a name of
/// instead (a format does not reach an enum's names).
fn constants_read(fd: &Funcdata, at: &Address) -> (Vec<(VarnodeId, u64)>, Option<String>) {
    let mut out: Vec<(VarnodeId, u64)> = Vec::new();
    let mut named = None;
    for (_, id) in fd.obank().iter_at(at) {
        let Some(op) = fd.obank().get(id).filter(|op| !op.is_dead()) else { continue };
        let opc = op.code();
        for slot in 0..op.num_input() {
            let Some(vn) = op.get_in(slot).filter(|_| prints_as_number(opc, slot)) else { continue };
            let Some(v) = fd.vbank().get(vn).filter(|v| v.is_constant()) else { continue };
            let integer = matches!(
                v.get_type().get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            );
            if !integer || out.iter().any(|(seen, _)| *seen == vn) {
                continue;
            }
            let shown = fd.vn_type_read_facing(vn, id);
            if shown.is_enum_type() {
                named.get_or_insert_with(|| shown.get_name().to_string());
                continue;
            }
            out.push((vn, v.get_offset()));
        }
    }
    (out, named)
}

/// Does a constant of `size` bytes holding `held` answer to the stated `value`?
fn answers(held: u64, size: i32, value: u64) -> bool {
    let mask = if size >= 8 { u64::MAX } else { (1u64 << (size.max(1) * 8)) - 1 };
    held & mask == value & mask && (value & !mask == 0 || value | mask == u64::MAX)
}

/// The constants at `at` a request names: all of them when they share one
/// value, else the ones holding `value`.
fn selected(fd: &Funcdata, at: &Address, value: Option<u64>) -> Result<Vec<VarnodeId>, String> {
    let (read, named) = constants_read(fd, at);
    if read.is_empty() {
        return Err(match named {
            Some(name) => format!("the constant read at {:#x} prints as a name of {name}, not as a number", at.get_offset()),
            None => format!("no constant is read at {:#x}", at.get_offset()),
        });
    }
    let size_of = |vn: VarnodeId| fd.vbank().get(vn).map_or(0, |v| v.get_size());
    let chosen: Vec<VarnodeId> = match value {
        Some(value) => read.iter().filter(|(vn, held)| answers(*held, size_of(*vn), value)).map(|(vn, _)| *vn).collect(),
        None => read.iter().map(|(vn, _)| *vn).collect(),
    };
    let mut values: Vec<u64> = read.iter().map(|(_, held)| *held).collect();
    values.sort_unstable();
    values.dedup();
    let listed = values.iter().map(|v| format!("{v:#x}")).collect::<Vec<_>>().join(", ");
    if chosen.is_empty() {
        return Err(format!("no constant read at {:#x} holds that value (it reads {listed})", at.get_offset()));
    }
    if value.is_none() && values.len() > 1 {
        return Err(format!(
            "{:#x} reads several constants ({listed}); name one by its value",
            at.get_offset()
        ));
    }
    Ok(chosen)
}

/// Print the constant the instruction at `at` reads in `format`.
pub fn apply(
    fd: &mut Funcdata,
    arch: &Architecture,
    at: &Address,
    format: &str,
    value: Option<u64>,
    maxduplicates: u32,
) -> Result<(), String> {
    let code = format_code(format).ok_or_else(|| format!("Unrecognized integer format: {format}"))?;
    let base1 = arch
        .types()
        .get_base(1, type_metatype::TYPE_UNKNOWN)
        .map_err(|e| e.explain().to_string())?;
    let chosen = selected(fd, at, value)?;
    if format == "char" {
        let mut read = chosen.iter().filter_map(|&vn| fd.vbank().get(vn)).map(|v| (v.get_size(), v.get_offset()));
        if let Some((_, held)) = read.find(|&(size, held)| size > 4 && held != held as i32 as i64 as u64) {
            return Err(format!("a character literal cannot hold {held:#x}"));
        }
    }
    for vn in chosen {
        fd.build_dynamic_symbol(vn, maxduplicates, base1.clone()).map_err(|e| e.explain().to_string())?;
        let sym = fd.vn_high_equate_symbol(vn).ok_or_else(|| "Unable to create symbol".to_string())?;
        let scope = fd.get_scope_local_mut().ok_or_else(|| "Function has no local scope".to_string())?;
        scope.set_display_format(sym, code);
        scope.set_attribute(sym, varnode_flags::typelock);
    }
    Ok(())
}

/// Is a constant the request names bound to a Symbol of that format in `fd`?
pub fn holds(fd: &Funcdata, at: &Address, format: &str, value: Option<u64>) -> bool {
    let (Some(code), Some(lm)) = (format_code(format), fd.get_scope_local()) else { return false };
    constants_read(fd, at).0.into_iter().any(|(vn, held)| {
        value.is_none_or(|value| answers(held, fd.vbank().get(vn).map_or(0, |v| v.get_size()), value))
            && fd.vn_high_equate_symbol(vn).is_some_and(|sym| lm.database().symbol(sym).dispflags & 7 == code)
    })
}
