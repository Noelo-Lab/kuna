//! Carmel register selectors and DATA operand spellings.

use std::borrow::Cow;
use std::fmt;

use super::{Operand, Operation};

pub(super) const R: [&str; 8] = ["r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7"];
pub(super) const RN: [&str; 8] = ["rn0", "rn1", "rn2", "rn3", "rn4", "rn5", "rn6", "rn7"];
pub(super) const A: [&str; 6] = ["a0", "a1", "a2", "a3", "a4", "a5"];
pub(super) const ADDRESS_UPDATE_PAIRS: [Option<(u8, u8)>; 8] = [
    Some((0, 1)),
    Some((0, 2)),
    Some((0, 3)),
    None,
    Some((1, 2)),
    Some((1, 3)),
    Some((2, 3)),
    None,
];
pub(super) const ADDRESS_MODES: [&str; 4] = ["lin", "bitr", "reg", "spec"];

const AH: [&str; 6] = ["a0h", "a1h", "a2h", "a3h", "a4h", "a5h"];
const AL: [&str; 6] = ["a0l", "a1l", "a2l", "a3l", "a4l", "a5l"];
const GB: [&str; 6] = ["gb0", "gb1", "gb2", "gb3", "gb4", "gb5"];
const LOOP: [&str; 8] = ["rl0", "rl2", "rm0", "rm2", "rl4", "rl6", "rm4", "rm6"];
const CONFIG: [&str; 6] = ["ecfg", "est", "ccfg1", "ccfg2", "ctbl", "fftv"];
const CONTROL: [Option<&str>; 16] = [
    Some("reserved"),
    Some("reserved"),
    Some("reserved"),
    Some("reserved"),
    Some("lc"),
    Some("pstatus"),
    Some("intbase"),
    Some("user_io"),
    Some("io_space"),
    Some("reserved"),
    Some("pacfg"),
    Some("reserved"),
    Some("reserved"),
    Some("reserved"),
    None,
    None,
];
const CONDITION_1: [&str; 28] = [
    "c1", "nc1", "v1", "nv1", "e1", "ne1", "l1", "nl1", "ce1", "nce1", "ge1", "ge1u", "gt1", "gt1u",
    "eq1", "neq1", "le1", "le1u", "lt1", "lt1u", "i0", "ni0", "i1", "ni1", "i2", "ni2", "i3", "ni3",
];
const CONDITION_2: [&str; 20] = [
    "c2", "nc2", "v2", "nv2", "e2", "ne2", "l2", "nl2", "ce2", "nce2", "ge2", "ge2u", "gt2", "gt2u",
    "eq2", "neq2", "le2", "le2u", "lt2", "lt2u",
];

fn name(bank: &[&'static str], value: u32) -> Option<&'static str> {
    bank.get(value as usize).copied()
}

pub(super) fn condition(value: u32) -> Option<&'static str> {
    let index = value & 31;
    if value < 32 {
        name(&CONDITION_1, index)
    } else if index >= 30 {
        Some("reserved")
    } else {
        name(&CONDITION_2, index)
    }
}

fn loop_address(value: u32) -> Option<&'static str> {
    name(&["reserved", "reserved", "lsa", "lea"], value)
}

fn address_register(value: u32) -> Option<&'static str> {
    match value {
        0..=7 => name(&R, value),
        8..=15 => name(&RN, value - 8),
        16..=23 => name(&LOOP, value - 16),
        24 => Some("rf0"),
        25 => Some("astatus"),
        26 => Some("minmax"),
        27 => Some("sp"),
        28 => Some("rf4"),
        30 => Some("ff12"),
        _ => None,
    }
}

fn accumulator_halves(value: u32) -> Option<&'static str> {
    match value {
        0..=5 => name(&AL, value),
        8..=13 => name(&AH, value - 8),
        _ => None,
    }
}

fn accumulator_flags(value: u32) -> Option<&'static str> {
    match value {
        6 => Some("ff1"),
        7 => Some("ff2"),
        _ => accumulator_halves(value),
    }
}

fn accumulator_bit_register(value: u32) -> Option<&'static str> {
    match value {
        14 => Some("btr"),
        15 => Some("x"),
        _ => accumulator_flags(value),
    }
}

fn wide_register(value: u32) -> Option<&'static str> {
    match value {
        0..=5 => name(&GB, value),
        6 => Some("rmw1"),
        7 => Some("rmw2"),
        8..=13 => name(&A, value - 8),
        14 => Some("op3"),
        16..=29 => accumulator_flags(value - 16),
        30 => Some("x1"),
        31 => Some("x2"),
        32..=63 => address_register(value - 32),
        64..=69 => name(&CONFIG, value - 64),
        70 => Some("btr1"),
        71 => Some("btr2"),
        80..=95 => CONTROL[(value - 80) as usize],
        _ => None,
    }
}

/// Register selectors 127 and 128 retain their register-space prefix on the wire.
pub(super) fn register_name(kind: u8, value: u32) -> Option<&'static str> {
    match kind {
        106 | 107 => match value {
            0..=7 => name(&R, value),
            8..=15 => name(&RN, value - 8),
            20 if kind == 107 => Some("x"),
            21 | 28 | 29 if kind == 107 => None,
            16..=29 => accumulator_flags(value - 16),
            30 => Some("op1"),
            31 => Some("op2"),
            _ => None,
        },
        112 => wide_register(value),
        113 | 114 => name(&A, value),
        117 | 120..=122 => match value {
            6 if kind != 117 => Some("rmw1"),
            7 if kind != 117 && kind != 120 => Some("rmw2"),
            _ => accumulator_halves(value),
        },
        118 => match value {
            0..=3 => name(&AL, value),
            4..=7 => name(&AH, value - 4),
            _ => None,
        },
        119 => accumulator_flags(value),
        127 => match value.checked_sub(0x20)? {
            27 => Some("sp"),
            selector => name(&R, selector),
        },
        128 => wide_register(value.checked_sub(0x80)?),
        129 => match value {
            16..=23 => name(&R, value - 16),
            24..=31 => name(&RN, value - 24),
            _ => accumulator_halves(value),
        },
        130 => match value & 15 {
            0..=3 => name(&R, (value & 15) + 4),
            4..=7 => name(&RN, value & 15),
            8..=11 => name(&LOOP, (value & 15) - 4),
            12 => Some("rf4"),
            _ => None,
        },
        131 => address_register(value),
        140 => name(&CONFIG, value),
        141 => match value {
            2 => Some("pacfg"),
            4 => Some("astatus"),
            5 => Some("pstatus"),
            6 => Some("reserved"),
            7 => Some("user_io"),
            _ => None,
        },
        147 => match value {
            0..=31 => address_register(value),
            32..=37 => name(&A, value - 32),
            40..=45 => name(&GB, value - 40),
            48..=63 => accumulator_bit_register(value - 48),
            64..=95 => CONTROL[(value & 15) as usize],
            96..=127 => loop_address(value & 15),
            _ => None,
        },
        154 => name(&GB, value),
        169 => name(&R[..4], value),
        174 => loop_address(value & 15),
        175 => CONTROL.get(value as usize).copied().flatten(),
        178 => match value {
            0..=7 => name(&R, value),
            8..=15 => name(&RN, value - 8),
            _ => None,
        },
        180 => name(&LOOP, value),
        181 => name(&RN, value),
        184 => name(&["x", "op3", "ff1", "ff2"], value),
        195 => match value {
            0..=5 => name(&CONFIG, value),
            8..=13 => name(&GB, value - 8),
            16..=31 => accumulator_bit_register(value - 16),
            _ => None,
        },
        196 => accumulator_bit_register(value),
        202 => name(&R, value),
        206 if value == 0 => Some("sp"),
        _ => None,
    }
}

pub(super) fn stack_registers(value: u32) -> (Option<&'static str>, Option<&'static str>) {
    let first = match value >> 4 {
        0..=3 => name(&R, value >> 4),
        4..=7 => name(&RN, (value >> 4) - 4),
        8..=11 => name(&LOOP, (value >> 4) - 8),
        12 => Some("rf0"),
        13 => Some("astatus"),
        _ => None,
    };
    let second = if value & 15 == 13 {
        Some("astatus")
    } else {
        register_name(130, value)
    };
    (first, second)
}

pub(super) fn sign_extend(value: u32, width: u8) -> i32 {
    let shift = 32 - width;
    ((value << shift) as i32) >> shift
}

struct Hex(i32, usize);

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(value, digits) = *self;
        let sign = if value < 0 { "-" } else { "" };
        write!(f, "{sign}0x{:0digits$x}", value.unsigned_abs())
    }
}

pub(super) fn hex(value: i32, digits: usize) -> String {
    Hex(value, digits).to_string()
}

// A DATA selector consists of the mode in the upper three bits and the R/RN index below it.
fn wide_address(kind: u8, value: u32) -> Option<(u32, bool)> {
    match kind {
        128 if value < 128 => Some((value & 63, value & 64 != 0)),
        125 | 126 if value < 64 => Some((value, kind == 125)),
        _ => None,
    }
}

fn narrow_address(op: &Operation, operand: &Operand, index: usize) -> Option<(u32, bool)> {
    let Operand {
        kind, value, width, ..
    } = *operand;
    let (kind, bank) = match kind {
        104 if matches!(op.family, 36 | 44 | 49) => {
            let long = op.operands().iter().find(|other| other.kind == 108)?;
            (101, u32::from(long.value & 4 == 0) * 4)
        }
        104 => (kind, 4),
        101 if width == 4 => {
            let mask = if index == 1 { 16 } else { 8 };
            (kind, u32::from(op.operands().first()?.value & mask != 0) * 4)
        }
        _ => (kind, 0),
    };
    match kind {
        103 | 105 | 108 | 109 if value < 32 => Some((value, matches!(kind, 108 | 109))),
        110 if value < 32 || (40..48).contains(&value) => Some((value, true)),
        168 if value < 8 => Some((value + 40, false)),
        101 if width == 5 && value < 32 => {
            let bank = if value & 8 == 0 { 4 } else { 0 };
            Some(((value & 4) * 2 + bank + (value & 3), false))
        }
        101 | 104 if value < 16 => Some(((value >> 2) * 8 + bank + (value & 3), false)),
        _ => None,
    }
}

fn has_displacement(selector: u32) -> bool {
    matches!(selector >> 3, 4 | 6)
}

fn address_text(selector: u32, long: bool, extra: u32, displacement_slot: usize) -> String {
    let prefix = if long { "*(long*)" } else { "*" };
    let index = (selector & 7) as usize;
    let base = R[index];
    match selector >> 3 {
        0 => format!("{prefix}{base}"),
        1 => format!("{prefix}({base}+={})", RN[index]),
        2 => format!("{prefix}{base}++"),
        3 => format!("{prefix}{base}--"),
        4 | 6 => {
            let operator = if selector >> 3 == 4 { "+" } else { "+=" };
            if displacement_slot < 2 {
                let displacement = sign_extend(extra >> (8 * displacement_slot), 8);
                format!("{prefix}({base}{operator}{})", Hex(displacement, 2))
            } else {
                format!("{prefix}({base}{operator}{{8}})")
            }
        }
        5 => format!("{prefix}({base}+{})", RN[index]),
        _ => format!("{prefix}{}", Hex((extra & 0xffff) as i32, 4)),
    }
}

/// Render an encoded operand without substituting text for an unassigned selector.
pub(super) fn text(op: &Operation, index: usize) -> Option<Cow<'static, str>> {
    let operands = op.operands();
    let operand = operands.get(index)?;
    let Operand {
        kind, value, width, ..
    } = *operand;
    let semantic = op.semantic;
    if kind == 146 {
        return condition(value).map(Cow::Borrowed);
    }
    if !op.narrow() {
        // These operands use the physical low halfword, not the family-specific extra field.
        let extra = op.left & 0xffff;
        if let Some((selector, long)) = wide_address(kind, value) {
            let slot = if has_displacement(selector) {
                operands[..index]
                    .iter()
                    .filter_map(|prior| wide_address(prior.kind, prior.value))
                    .filter(|&(prior, _)| has_displacement(prior))
                    .count()
            } else {
                0
            };
            return Some(Cow::Owned(address_text(selector, long, extra, slot)));
        }
        return match (kind, value) {
            (112, 15) | (125 | 126, 79) => {
                let literal = match semantic {
                    93 | 94 | 95 => {
                        let separator = if semantic == 93 { "," } else { ", " };
                        format!(
                            "{}{separator}{}",
                            Hex((extra >> 8) as i32, 0),
                            Hex((extra & 255) as i32, 0)
                        )
                    }
                    41 | 42 => hex(sign_extend(extra, 7), 0),
                    _ => hex(sign_extend(extra, 16), 4),
                };
                Some(Cow::Owned(literal))
            }
            (112 | 125 | 126, _) => wide_register(value & 63).map(Cow::Borrowed),
            (128, 0x8f) => Some(Cow::Owned(hex(sign_extend(extra, 16), 4))),
            (151, _) => Some(Cow::Owned(format!("(ext){}", Hex(value as i32, 3)))),
            (172, _) => name(&ADDRESS_MODES, value).map(Cow::Borrowed),
            _ => register_name(kind, value).map(Cow::Borrowed),
        };
    }
    if op.family == 37
        && (kind == 175 && matches!(value, 8 | 10 | 12) || kind == 113 && value >= 6)
    {
        return None;
    }
    if let Some((selector, long)) = narrow_address(op, operand, index) {
        return Some(Cow::Owned(address_text(selector, long, 0, 0)));
    }
    match kind {
        113 if value == 6 && semantic == 98 => Some(Cow::Borrowed("btr")),
        114 if value == 7 && matches!(semantic, 93..=95) => Some(Cow::Borrowed("ff12")),
        147 if semantic == 99 && matches!(value, 72 | 76 | 88 | 92) => None,
        159 if semantic == 126 => name(&ADDRESS_MODES, value).map(Cow::Borrowed),
        162 => Some(Cow::Owned(hex((value & 15) as i32, 0))),
        163 if matches!(semantic, 41 | 42) => Some(Cow::Owned(hex(sign_extend(value, width), 0))),
        163 => Some(Cow::Owned(hex((value & 31) as i32, 0))),
        164 if matches!(semantic, 41 | 42) => Some(Cow::Owned(hex(sign_extend(value, width), 0))),
        164 => Some(Cow::Owned(hex(value as i32, 0))),
        165 => {
            let immediate = if matches!(semantic, 99 | 107) {
                value as i32
            } else {
                sign_extend(value, width.min(8))
            };
            Some(Cow::Owned(hex(immediate, 0)))
        }
        201 => {
            let index = (value & 7) as usize;
            let register = R[index];
            if value & 8 != 0 {
                Some(Cow::Owned(format!("({register}+={})", RN[index])))
            } else {
                Some(Cow::Borrowed(register))
            }
        }
        _ => register_name(kind, value).map(Cow::Borrowed),
    }
}
