use std::fmt::Write as _;

use super::operand::{self, condition, register_name, sign_extend, A, ADDRESS_UPDATE_PAIRS, R, RN};
use super::{Lane, Operation};

/// None means another renderer owns this semantic; Some(None) means CrossView rejects it.
pub(super) fn render(op: &Operation, pc: u32, lane: Lane) -> Option<Option<String>> {
    let sem = op.semantic;
    Some(match sem {
        104..=106 | 108..=110 | 113 | 114 => flow(op, pc, lane),
        100 | 101 | 164..=169 => repeat(op, pc, lane),
        127 if compact_incomplete_address_update(op) => {
            (lane != Lane::ParallelLeft).then(String::new)
        }
        127 | 128 => address_updates(op),
        82 => clear_accumulators(op, lane),
        125 => bank_exchange(op, lane),
        179..=182 => control_expression(op),
        188 => cliw_text(op),
        145..=147 | 195..=197 => wide_move(op),
        175 if op.family == 33 => wide_move(op),
        119 if !op.narrow() => wide_link(op),
        _ => return None,
    })
}

fn short_relative(op: &Operation) -> bool {
    op.narrow() && matches!(op.semantic, 108..=110) && op.operands().last().is_some_and(|x| x.value == 3)
}

fn relative_target(op: &Operation, pc: u32, lane: Lane, short: bool) -> Option<u32> {
    let displacement = if op.narrow() {
        op.operands().get(usize::from(matches!(op.semantic, 108..=110)))?
    } else {
        op.operands().last()?
    };
    let from_fetch_start = lane == Lane::ParallelRight && (short || matches!(op.semantic, 104 | 105));
    let delta = sign_extend(displacement.value, displacement.width) - i32::from(short);
    Some(pc.wrapping_add_signed(delta).wrapping_sub(u32::from(from_fetch_start)) & 0xff_ffff)
}

pub(super) fn branch_target(op: &Operation, pc: u32, lane: Lane) -> Option<String> {
    let address = relative_target(op, pc, lane, short_relative(op))?;
    Some(format!("CODE:{address:#x}"))
}

fn flow(op: &Operation, pc: u32, lane: Lane) -> Option<String> {
    let name = match op.semantic {
        104 | 108 => "goto",
        105 => "continue",
        106 | 109 => "call",
        110 => "break",
        113 => "return",
        114 => "iret",
        _ => return None,
    };
    let short = short_relative(op);
    if short && lane == Lane::ParallelLeft {
        return None;
    }
    let target = if op.semantic < 110 {
        Some(relative_target(op, pc, lane, short)?)
    } else {
        None
    };
    Some(if op.semantic >= 108 {
        let index = usize::from(!op.narrow());
        let condition = condition(op.operands().get(index)?.value)?;
        match target {
            Some(target) => format!("if ({condition}) {name} CODE:{target:#x}"),
            None => format!("if ({condition}) {name}"),
        }
    } else {
        format!("{name} CODE:{:#x}", target?)
    })
}

fn strip_data_address(text: &str) -> &str {
    text.strip_prefix('*').unwrap_or(text).trim_matches(['(', ')'])
}

fn repeat(op: &Operation, pc: u32, lane: Lane) -> Option<String> {
    let operands = op.operands();
    let first = operands.first()?;
    match op.semantic {
        101 if first.kind == 167 => match first.value {
            0 if lane == Lane::ParallelLeft => None,
            0 => Some("repeat () single (ds=0)".into()),
            n => Some(format!("repeat ({n:#x}) single (ds=0)")),
        },
        101 => Some(format!("repeat ({}) single (ds=3)", operand::text(op, 0)?)),
        167 if first.value == 0 => match operands.get(1)?.value {
            0 => Some("repeat () single (ds=0)".into()),
            n => Some(format!("repeat ({n:#06x}) single (ds=0)")),
        },
        167 => None,
        168 | 169 => {
            let data_count = op.semantic == 169;
            let value = operands.get(1)?.value;
            let selector = if data_count { 7 } else { 3 };
            if first.value != selector || (data_count && value >= 64) {
                return None;
            }
            let count = if (data_count && value >= 56) || (!data_count && value == 0x8f) {
                None
            } else {
                operand::text(op, 1)
            };
            match count {
                None => Some("repeat () single (ds=0)".into()),
                Some(count) => {
                    let count = if data_count { strip_data_address(&count) } else { &count };
                    Some(format!("repeat ({count}) single (ds={})", first.value & 3))
                }
            }
        }
        sem => {
            let index = usize::from(!op.narrow());
            let count = operands.get(index)?;
            let delay = if op.narrow() { operands.last()?.value } else { first.value };
            let limit = if op.left & 0x800000 == 0 { 0x38 } else { 0x30 };
            if (matches!(sem, 100 | 164) && matches!(count.kind, 157 | 164) && count.value == 0)
                || (sem == 165 && count.value == 0x8f)
                || (sem == 166 && count.value >= limit)
            {
                return None;
            }
            let lea = pc.wrapping_add(operands.get(index + 1)?.value) & 0xff_fffe;
            Some(if sem == 164 {
                let count = count.value;
                format!("repeat ({count:#x}) block (lea={lea:#x}, ds={delay})")
            } else {
                let count = operand::text(op, index)?;
                let count = if sem == 166 { strip_data_address(&count) } else { &count };
                format!("repeat ({count}) block (lea={lea:#x}, ds={delay})")
            })
        }
    }
}

fn address_pair(value: u32) -> Option<(u8, u8)> {
    ADDRESS_UPDATE_PAIRS.get(value as usize).copied().flatten()
}

pub(super) fn compact_incomplete_address_update(op: &Operation) -> bool {
    op.semantic == 127 && op.operands().iter().take(2).any(|x| address_pair(x.value).is_none())
}

pub(super) fn address_updates(op: &Operation) -> Option<String> {
    let operands = op.operands();
    if op.semantic == 127 {
        let (a, b) = address_pair(operands.first()?.value)?;
        let (c, d) = address_pair(operands.get(1)?.value)?;
        let mut text = String::new();
        for (reg, mode) in [a, b, c + 4, d + 4].into_iter().zip(&operands[2..]) {
            let reg = usize::from(reg);
            let suffix = match mode.value {
                0 => continue,
                1 => "+=",
                2 => "++",
                3 => "--",
                _ => return None,
            };
            if !text.is_empty() {
                text.push_str(", ");
            }
            text.push_str(R[reg]);
            text.push_str(suffix);
            if mode.value == 1 {
                text.push_str(RN[reg]);
            }
        }
        return Some(text);
    }
    let [dest, source] = operands else { return None };
    if !op.narrow() || source.kind == 165 {
        let (dest, source, displacement) = if op.narrow() {
            let reg = register_name(dest.kind, dest.value)?;
            (reg, reg, sign_extend(source.value, 8))
        } else {
            // The type-127 listing namespace also admits SP, unlike decode annotations.
            let dest = register_name(127, dest.value)?;
            let source = register_name(127, source.value)?;
            (dest, source, sign_extend(op.extra, 16))
        };
        let sign = if displacement < 0 { "-" } else { "" };
        return Some(format!("{dest} == {source}+{sign}{:#x}", displacement.unsigned_abs()));
    }
    let dest = register_name(dest.kind, dest.value)?;
    Some(match source.kind {
        105 => {
            let index = (source.value & 7) as usize;
            let reg = R[index];
            match source.value >> 3 {
                0 => format!("{dest} == {reg}"),
                1 => format!("{dest} == {reg}+{}", RN[index]),
                2 => format!("{dest} == {reg}+1"),
                _ => format!("{dest} == {reg}-1"),
            }
        }
        _ => format!("{dest} == {}", register_name(source.kind, source.value)?),
    })
}

fn clear_accumulators(op: &Operation, lane: Lane) -> Option<String> {
    let mask = op.operands().first()?.value;
    let mut modes = (0..6).map(|i| (mask >> (10 - 2 * i)) & 3);
    let count = modes.position(|mode| mode == 3).unwrap_or(A.len());
    if count < A.len() && lane == Lane::ParallelLeft {
        return None;
    }
    let mut text = String::new();
    for (mode, name) in [(1, "clr("), (2, "clrr(")] {
        let mut regs = A.into_iter().take(count).enumerate().filter_map(|(i, reg)| {
            ((mask >> (10 - 2 * i)) & 3 == mode).then_some(reg)
        });
        if let Some(reg) = regs.next() {
            if !text.is_empty() {
                text.push_str(", ");
            }
            text.push_str(name);
            text.push_str(reg);
            for reg in regs {
                text.push(',');
                text.push_str(reg);
            }
            text.push(')');
        }
    }
    Some(text)
}

fn push_move_operand(text: &mut String, op: &Operation, index: usize) -> Option<()> {
    let operand = op.operands().get(index)?;
    let value = operand.value;
    if operand.kind != 157 {
        text.push_str(&operand::text(op, index)?);
        return Some(());
    }
    match op.semantic {
        146 | 147 => write!(text, "(io){value:#08x}").unwrap(),
        197 => write!(text, "{value:#08x}").unwrap(),
        145 => {
            let value = sign_extend(value, 16);
            let sign = if value < 0 { "-" } else { "" };
            write!(text, "{sign}{:#06x}", value.unsigned_abs()).unwrap();
        }
        _ => write!(text, "{:#06x}", value & 0xffff).unwrap(),
    }
    Some(())
}

fn wide_move(op: &Operation) -> Option<String> {
    let mut text = String::new();
    push_move_operand(&mut text, op, 0)?;
    text.push_str(if op.semantic == 175 { " = (unsigned)" } else { " = " });
    push_move_operand(&mut text, op, 1)?;
    Some(text)
}

fn wide_link(op: &Operation) -> Option<String> {
    let dest = operand::text(op, 0)?;
    let count = sign_extend(op.operands().get(1)?.value, 24);
    (R.contains(&dest.as_ref()) && (-0x8000..0).contains(&count))
        .then(|| format!("link({dest}, -{:#06x})", count.unsigned_abs()))
}

const BANKXCHG_LOOP_REGS: [&str; 4] = ["rm0", "rm2", "rm4", "rm6"];
const BANKXCHG_WIDE_ACC_BITS: [u8; 6] = [16, 17, 12, 13, 18, 19];

fn bank_exchange(op: &Operation, lane: Lane) -> Option<String> {
    let mut text = String::from("bankxchg(");
    let mut push_reg = |reg: &str| {
        if !text.ends_with('(') {
            text.push(',');
        }
        text.push_str(reg);
    };
    if op.narrow() {
        let [loops, registers, accumulator] = op.operands() else { return None };
        for (mask, bank) in [(registers.value, R.as_slice()), (loops.value, BANKXCHG_LOOP_REGS.as_slice())] {
            for (i, reg) in bank.iter().enumerate().rev() {
                if mask & (1 << i) != 0 {
                    push_reg(reg);
                }
            }
        }
        if accumulator.value != 0 {
            push_reg(if lane == Lane::ParallelRight { "a3" } else { "a2" });
            push_reg("est");
        }
    } else {
        // Wide spelling sorts register names, unlike the compact mask order.
        let mask = op.operands().first()?.value;
        let regs = BANKXCHG_WIDE_ACC_BITS.into_iter().zip(A)
            .chain([(15, "ecfg"), (14, "est")])
            .chain((0..8).zip(R))
            .chain((8..12).zip(BANKXCHG_LOOP_REGS));
        for (bit, reg) in regs {
            if mask & (1 << bit) != 0 {
                push_reg(reg);
            }
        }
    }
    text.push(')');
    Some(text)
}

fn cliw_banks(mode: u32) -> [usize; 4] {
    match mode {
        4..=7 | 13 => [0, 0, 4, 4],
        8..=11 | 14 => [4, 4, 0, 0],
        15 => [0, 0, 0, 0],
        _ => [0, 4, 4, 0],
    }
}

// A prefix may consume a field; an ordinary absent field still occupies its position.
fn cliw_argument(
    value: u32,
    bank: usize,
    enabled: bool,
    long_address: &mut bool,
    long_stack: &mut bool,
) -> Option<[&'static str; 5]> {
    let (parts, elide) = match value {
        28.. => {
            *long_address = true;
            return None;
        }
        26 => {
            *long_stack = true;
            return None;
        }
        25 => {
            let stack = std::mem::take(long_stack);
            let reg = if stack { "sp32" } else if *long_address { "(long*)sp" } else { "sp" };
            ([reg, "", "", "", ""], stack)
        }
        24 | 27 => ([""; 5], false),
        _ => {
            let index = bank + (value & 3) as usize;
            let (suffix, offset) = match value >> 2 {
                0 => ("", ""),
                1 => ("+=", RN[index]),
                2 => ("++", ""),
                3 => ("--", ""),
                4 => ("+", if bank == 0 { "rf0" } else { "rf4" }),
                _ => ("+", RN[index]),
            };
            let long = std::mem::take(long_address);
            let (prefix, end) = if !long {
                ("", "")
            } else if matches!(value >> 2, 1 | 4 | 5) {
                ("(long*)(", ")")
            } else {
                ("(long*)", "")
            };
            ([prefix, R[index], suffix, offset, end], long)
        }
    };
    if enabled { Some(parts) } else if elide { None } else { Some([""; 5]) }
}

fn cliw_text(op: &Operation) -> Option<String> {
    let [mode, high, p0, p0_used, p1, p1_used, _, p2, p3, low] = op.operands() else {
        return None;
    };
    let index = (high.value << 10) | low.value;
    let mut text = format!("cliwexe {index:#x}(");
    let (mut long_address, mut long_stack) = (false, false);
    let mut separators = 0;
    for ((p, used), bank) in [(p0, p0_used.value != 0), (p1, p1_used.value != 0), (p2, true), (p3, true)]
        .into_iter().zip(cliw_banks(mode.value))
    {
        if let Some(parts) = cliw_argument(p.value, bank, used, &mut long_address, &mut long_stack) {
            if parts.iter().any(|part| !part.is_empty()) {
                for _ in 0..separators {
                    text.push_str(", ");
                }
                for part in parts {
                    text.push_str(part);
                }
                separators = 1;
            } else {
                separators += 1;
            }
        }
    }
    text.push(')');
    Some(text)
}

const CE_INPUTS: [&str; 14] = ["v1", "v2", "c1", "c2", "l1", "l2", "m1", "m2", "z1", "z2", "e1", "e2", "ce1", "ce2"];

// CrossView uses these spellings instead of direct Shannon decomposition.
fn ce_pattern(truth: u8) -> Option<&'static str> {
    Some(match truth {
        0x00 => "a&~a", 0x01 => "~(a|b|c)", 0x03 => "~(a|b)", 0x0f => "~a",
        0x14 => "(a^b)&~c", 0x15 => "~(a&b|c)", 0x17 => "~a&(b^c)|~b&~c",
        0x19 => "~a&b&c|~b&~c", 0x1a => "a&~b&~c|~a&c", 0x1e => "a^(b|c)",
        0x1f => "~a|~b&~c", 0x25 => "a&~b&c|~a&~c", 0x28 => "(a^b)&c",
        0x2a => "(a&~b|~a)&c", 0x2d => "a^(b|~c)", 0x2f => "~a|~b&c",
        0x35 => "a&~b|~a&~c", 0x3a => "a&~b|~a&c", 0x3c => "a^b", 0x3f => "~(a&b)",
        0x41 => "~(a^b)&~c", 0x45 => "a&b&~c|~a&~c", 0x4a => "a&b&~c|~a&c",
        0x4b => "a^(~b|c)", 0x51 => "(a|~a&~b)&~c", 0x52 => "~a&~b&c|a&~c",
        0x53 => "~a&~b|a&~c", 0x54 => "(a|b)&~c", 0x56 => "~a&(b^c)|a&~c",
        0x57 => "~a&~(b&c)|a&~c", 0x58 => "~a&b&c|a&~c", 0x59 => "~a&~(b^c)|a&~c",
        0x5a => "a&(b|~b)^c", 0x5b => "~a&(~b|c)|a&~c", 0x5c => "~a&b|a&~c",
        0x5d => "~a&(b|~c)|a&~c", 0x5e => "~a&(b|c)|a&~c", 0x5f => "~a&(b|~b)|a&~c",
        0x65 => "a&(b^c)|~a&~c", 0x69 => "~a^b^c", 0x6a => "a&b^c",
        0x6e => "~a&b|(b^c)", 0x6f => "~a|(b^c)", 0x75 => "a&~(b&c)|~a&~c",
        0x78 => "a^b&c", 0x7a => "a&~(b&c)|~a&c", 0x7f => "~a|~(b&c)",
        0x85 => "a&b&c|~a&~c", 0x87 => "~a^b&c", 0x8a => "a&b&c|~a&c",
        0x8f => "~a|b&c", 0x95 => "~(a&b^c)", 0x96 => "a^b^c",
        0x9a => "a&~(b^c)|~a&c", 0x9f => "~a|~(b^c)", 0xa1 => "~a&~b&~c|a&c",
        0xa2 => "~a&~b&c|a&c", 0xa3 => "~a&~b|a&c", 0xa4 => "~a&b&~c|a&c",
        0xa5 => "~(a&(b|~b)^c)", 0xa6 => "~a&b^c", 0xa7 => "~a&~(b&c)|a&c",
        0xa8 => "(a|b)&c", 0xa9 => "~a&~b^c", 0xab => "~a&~b|c",
        0xac => "~a&b|a&c", 0xad => "~a&(b|~c)|a&c", 0xae => "~a&b|c",
        0xaf => "~a&(b|~b)|c", 0xb4 => "a^b&~c", 0xb5 => "a&(~b|c)|~a&~c",
        0xba => "a&(~b|c)|~a&c", 0xbe => "a^b|c", 0xbf => "~a|~b|c",
        0xc3 => "~(a^b)", 0xc5 => "a&b|~a&~c", 0xca => "a&b|~a&c",
        0xd2 => "a^~b&c", 0xd5 => "a&(b|~c)|~a&~c", 0xda => "a&(b|~c)|~a&c",
        0xe1 => "a^~b&~c", 0xe5 => "a&(b|c)|~a&~c", 0xea => "a&b|c",
        0xeb => "~a^b|c", 0xf0 => "a", 0xf6 => "a|b^c", 0xf8 => "a|b&c",
        0xfc => "a|b", 0xfe => "a|b|c", 0xff => "a|~a",
        _ => return None,
    })
}

const CE_NIBBLES: [&str; 16] = [
    "", "~b&~c", "~b&c", "~b", "b&~c", "(b|~b)&~c", "(b^c)", "~(b&c)",
    "b&c", "~(b^c)", "(b|~b)&c", "(~b|c)", "b", "(b|~c)", "(b|c)", "",
];

fn push_ce_pattern(text: &mut String, pattern: &str, inputs: [&str; 3]) {
    for ch in pattern.chars() {
        match ch {
            'a'..='c' => text.push_str(inputs[(ch as u8 - b'a') as usize]),
            _ => text.push(ch),
        }
    }
}

fn truth_expression(text: &mut String, inputs: [&str; 3], truth: u8) {
    if let Some(pattern) = ce_pattern(truth) {
        push_ce_pattern(text, pattern, inputs);
        return;
    }
    let hi = usize::from(truth >> 4);
    let lo = usize::from(truth & 15);
    if hi == lo {
        push_ce_pattern(text, "(a|~a)&", inputs);
        push_ce_pattern(text, CE_NIBBLES[hi], inputs);
        return;
    }
    for (negated, nibble) in [(false, hi), (true, lo)] {
        if nibble == 0 {
            continue;
        }
        if negated {
            if hi != 0 {
                text.push('|');
            }
            text.push('~');
        }
        text.push_str(inputs[0]);
        if nibble != 15 {
            text.push('&');
            push_ce_pattern(text, CE_NIBBLES[nibble], inputs);
        }
    }
}

fn control_expression(op: &Operation) -> Option<String> {
    let [mode, a, b, c, truth] = op.operands() else { return None };
    let inputs = [*CE_INPUTS.get(a.value as usize)?, *CE_INPUTS.get(b.value as usize)?, *CE_INPUTS.get(c.value as usize)?];
    let assign = if mode.value == 1 { "==" } else { "=" };
    let full = matches!(mode.value, 0 | 2);
    let mut text = String::new();
    if full {
        text.push_str("full(");
    }
    match op.semantic {
        179 => write!(text, "ce1 {assign} ").unwrap(),
        180 => write!(text, "ce2 {assign} ").unwrap(),
        181 => write!(text, "ce1 {assign} (ce2 {assign} ").unwrap(),
        _ => write!(text, "ce2 {assign} ~(ce1 {assign} ").unwrap(),
    }
    truth_expression(&mut text, inputs, truth.value as u8);
    if op.semantic >= 181 {
        text.push(')');
    }
    if full {
        text.push(')');
    }
    Some(text)
}
