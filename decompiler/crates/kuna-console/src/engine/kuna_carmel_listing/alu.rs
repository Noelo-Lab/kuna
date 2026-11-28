//! CrossView arithmetic, register, and DATA listing forms.

use std::{borrow::Cow, fmt::Write};

use super::{control, operand, Lane, Operand, Operation};

const UNSIGNED: &str = "(unsigned)";
type Text = Cow<'static, str>;
type Scratch = [Option<Text>; 3];

fn unary(sem: u16) -> Option<&'static str> {
    Some(match sem {
        1 => "inc",
        2 => "dec",
        3 => "not",
        4 => "ror",
        5 => "rol",
        8 => "lim",
        9 => "round",
        10 => "neg",
        11 => "abs",
        12 => "exp",
        173 => "lim8",
        _ => return None,
    })
}

fn simple(sem: u16) -> Option<&'static str> {
    Some(match sem {
        6 => "cmp",
        7 => "cmpu",
        13 => "addl",
        14 => "addh",
        15 => "subl",
        16 => "subh",
        83 | 88 => "clearbit",
        84 | 87 => "setbit",
        85 | 90 => "testbit",
        86 | 89 => "changebit",
        91 => "testbf0",
        92 => "testbf1",
        102 => "goabs",
        103 => "callabs",
        107 => "trap",
        118 => "trace",
        119 => "link",
        120 => "leave",
        121 | 123 => "pop",
        122 | 124 => "push",
        172 => "swap",
        176 => "movd2p",
        177 => "movp2d",
        _ => return None,
    })
}

fn minmax(sem: u16) -> Option<(&'static str, bool)> {
    Some(match sem {
        17 | 29 => ("minbtr", false),
        18 | 30 => ("minbtr", true),
        19 | 31 => ("minm", true),
        20 | 32 => ("min", false),
        21 | 33 => ("min", true),
        22 | 34 => ("minm", false),
        23 | 35 => ("maxbtr", false),
        24 | 36 => ("maxbtr", true),
        25 | 37 => ("max", false),
        26 | 38 => ("max", true),
        27 | 39 => ("maxm", false),
        28 | 40 => ("maxm", true),
        129 | 137 => ("minbtr1", false),
        130 | 138 => ("minbtr2", false),
        131 | 141 => ("maxbtr1", false),
        132 | 142 => ("maxbtr2", false),
        133 | 139 => ("minbtr1", true),
        134 | 140 => ("minbtr2", true),
        135 | 143 => ("maxbtr1", true),
        136 | 144 => ("maxbtr2", true),
        _ => return None,
    })
}

fn call<'a>(
    dest: Option<&str>,
    name: &str,
    args: impl IntoIterator<Item = &'a str>,
    separator: &str,
    cast: &str,
) -> String {
    let mut result = String::new();
    if let Some(dest) = dest {
        result.push_str(dest);
        result.push_str(" = ");
    }
    result.push_str(name);
    result.push('(');
    for (i, arg) in args.into_iter().enumerate() {
        if i != 0 {
            result.push_str(separator);
        }
        result.push_str(cast);
        result.push_str(arg);
    }
    result.push(')');
    result
}

fn same_register(a: &str, b: &str) -> bool {
    a == b && !a.starts_with(['*', '('])
}

fn scratch_arithmetic(sem: u16) -> bool {
    matches!(sem, 61 | 63 | 64 | 67 | 69)
}

fn arithmetic(sem: u16, [mut dest, mut left, mut right]: [&str; 3], narrow: bool, partial: bool) -> String {
    let reverse = matches!(sem, 62 | 65 | 66 | 68);
    if reverse {
        std::mem::swap(&mut left, &mut right);
    }
    let rmw = matches!(dest, "rmw1" | "rmw2");
    let mut same;
    if !narrow {
        // Wide SUB and reversed SUB each allow only their own in-place encoding.
        let select_left = dest == "rmw1" && sem != 62;
        let select_right = dest == "rmw2" && sem != 61 && !partial;
        if select_left || select_right {
            if select_right && !reverse {
                std::mem::swap(&mut left, &mut right);
            }
            (dest, same) = (left, true);
        } else {
            same = same_register(dest, left);
            if !same && matches!(sem, 63 | 64 | 69..=71 | 73 | 161..=163) && same_register(dest, right) {
                std::mem::swap(&mut left, &mut right);
                same = true;
            }
            if sem == 61 && left == right && !dest.is_empty() {
                same = false;
            }
        }
    } else if dest == "rmw1" && !reverse && left.starts_with('*') {
        // A partial scratch write can inherit both the selector and its memory source.
        (dest, same) = (left, true);
    } else if partial {
        same = same_register(dest, left);
    } else if (rmw && matches!(sem, 161..=163)) || (dest == "rmw2" && sem != 61) {
        if dest == "rmw2" && !reverse {
            std::mem::swap(&mut left, &mut right);
        }
        (dest, same) = (left, true);
    } else if rmw && matches!(sem, 69..=71) && (left.starts_with('*') || right.starts_with('*')) {
        if right.starts_with('*') && (dest == "rmw2" || !left.starts_with('*')) {
            std::mem::swap(&mut left, &mut right);
        }
        (dest, same) = (left, true);
    } else {
        same = same_register(dest, left);
    }
    if narrow && !partial {
        if !same && matches!(sem, 67 | 69 | 72 | 73 | 161..=163) && same_register(dest, right) {
            std::mem::swap(&mut left, &mut right);
            same = true;
        }
        if sem == 61 && left == right && !rmw {
            same = false;
        }
    }

    let symbol = match sem {
        61..=68 => '-',
        69..=74 => '+',
        161 => '&',
        162 => '|',
        163 => '^',
        _ => unreachable!(),
    };
    let tail = match sem {
        63 | 65 => Some("borrow1"),
        64 | 66 => Some("borrow2"),
        67 | 68 => Some("borrow"),
        70 => Some("carry1"),
        71 => Some("carry2"),
        72 => Some("carry"),
        _ => None,
    };
    let (mut abs_left, mut abs_right) = (false, sem == 73);
    if sem == 74 {
        if !same && same_register(dest, right) {
            std::mem::swap(&mut left, &mut right);
            same = true;
        }
        (abs_left, abs_right) = (!same, same);
    }
    if narrow && sem == 68 && same {
        return format!("{dest} += {right} + borrow");
    }
    let mut result = String::new();
    if same {
        if !dest.is_empty() {
            result.push_str(dest);
            result.push(' ');
        }
        let _ = write!(result, "{symbol}= ");
    } else {
        let _ = write!(result, "{dest} = ");
        if abs_left {
            let _ = write!(result, "abs({left})");
        } else {
            result.push_str(left);
        }
        let _ = write!(result, " {symbol} ");
    }
    if abs_right {
        let _ = write!(result, "abs({right})");
    } else {
        result.push_str(right);
    }
    if let Some(tail) = tail {
        let join = if same { '+' } else { symbol };
        let _ = write!(result, " {join} {tail}");
    }
    result
}

fn multiply_unsigned(sem: u16) -> u8 {
    match sem {
        44 | 48 | 53 | 57 => 1,
        45 | 49 | 54 | 58 => 2,
        46 | 50 | 59 => 3,
        _ => 0,
    }
}

fn multiply(sem: u16, args: &[&str]) -> Option<String> {
    let [dest, left, right, acc @ ..] = args else {
        return None;
    };
    let (dest, left, right) = (*dest, *left, *right);
    let sem = match sem {
        148..=152 => sem - 97,
        153..=160 => sem - 110,
        _ => sem,
    };
    let unsigned = multiply_unsigned(sem);
    let squaring = left == "op3" && matches!(sem, 43..=46 | 51 | 56..=60);
    let subtract = matches!(sem, 47..=50 | 52);
    let rounded = matches!(sem, 51 | 52 | 60);
    let upper = matches!(sem, 53..=55);
    let accumulator = acc.first().copied();
    let mut result = String::new();
    let special_square = squaring && (sem == 60 || sem == 51 && accumulator.is_some());
    if special_square {
        let _ = write!(result, "{} = round(", if sem == 51 { accumulator? } else { dest });
        if sem == 51 {
            let _ = write!(result, "{dest} + ");
        }
    } else if sem >= 56 {
        if rounded {
            result.push_str("round(");
        }
        let _ = write!(result, "{dest} = ");
    } else if let Some(accumulator) = accumulator {
        let _ = write!(result, "{dest} = ");
        if rounded {
            result.push_str("round(");
        }
        if subtract {
            let _ = write!(result, "{accumulator} - ");
        }
    } else {
        if rounded {
            result.push_str("round(");
        }
        if upper {
            let _ = write!(result, "({dest}>>16)");
        } else {
            result.push_str(dest);
        }
        result.push_str(if subtract { " -= " } else { " += " });
    }

    let cast_right = if unsigned & 1 != 0 { UNSIGNED } else { "" };
    if squaring {
        let _ = write!(result, "sqr({cast_right}{right})");
    } else {
        let cast_left = if unsigned & 2 != 0 { UNSIGNED } else { "" };
        let _ = write!(result, "{cast_left}{left} * {cast_right}{right}");
    }
    if !special_square && sem < 56 && !subtract {
        if let Some(accumulator) = accumulator {
            if upper {
                let _ = write!(result, " + ({accumulator}>>16)");
            } else {
                let _ = write!(result, " + {accumulator}");
            }
        }
    }
    if rounded {
        result.push(')');
    }
    Some(result)
}

pub(super) fn wide_minmax_omits_source(value: u32) -> bool {
    matches!(value, 64..=119 | 189 | 191 | 200..=207 | 222..)
}

pub(super) fn wide_omits_last_operand(op: &Operation) -> bool {
    let operands = op.operands();
    matches!(op.family, 13 | 26 | 32)
        && operands.last().is_some_and(|last| last.kind == 128 && operand::text(op, operands.len() - 1).is_none())
}

pub(super) fn compact_minmax_omits_source(op: &Operation) -> bool {
    op.family == 46 && op.operands().last().is_some_and(|last| {
        last.kind == 119 && last.value >= 14 || last.kind == 113 && last.value >= 6
    })
}

pub(super) fn compact_unary_omits_source(op: &Operation) -> bool {
    op.family == 30 && matches!(op.semantic, 10 | 11)
        && op.operands().get(1).is_some_and(|last| last.kind == 113 && last.value >= 6)
}

fn compact_stack_operands(op: &Operation) -> ([Option<Text>; 2], usize) {
    let operands = op.operands(); // PUSH has an implicit destination before its source.
    let Some(last) = operands.last() else {
        return ([None, None], 1);
    };
    match (last.kind, last.value) {
        (130, value) => {
            let (first, second) = operand::stack_registers(value);
            let len = if (value & 15) >= 13 || first.is_none() { 1 } else { 2 };
            ([first.map(Cow::Borrowed), second.filter(|_| len == 2).map(Cow::Borrowed)], len)
        }
        (175, 8 | 12) | (174, 16..=19) => ([None, None], 1),
        (114, 6) => ([Some(Cow::Borrowed("ff12")), None], 1),
        _ => ([operand::text(op, operands.len() - 1), None], 1),
    }
}

fn compact_extremum_rejected(op: &Operation) -> bool {
    op.family == 43 && matches!(op.semantic, 19 | 22 | 27 | 28)
        && !matches!(op.operands(), [Operand { kind: 117, .. }, Operand { kind: 105 | 168, .. }])
}

fn compact_round_product_rejected(op: &Operation) -> bool {
    op.family == 40 && op.semantic == 60
        && (op.left & 0x7c000 == 0x44000
            || matches!(op.operands().first(), Some(Operand { kind: 121, value: 6, .. })))
}

fn unbound_compact_rmw(dest: Option<&str>, left: Option<&str>) -> bool {
    dest == Some("rmw1") && left.is_some_and(|source| !source.starts_with('*'))
}

fn scratch_operands(op: &Operation, pc: u32, lane: Lane) -> Scratch {
    let sem = op.semantic;
    let operands = op.operands();
    if compact_extremum_rejected(op) || compact_round_product_rejected(op) {
        return [None, None, None];
    }
    match sem {
        104..=106 | 108..=110 | 113 | 114 => {
            if sem >= 108 {
                let Some(condition) = operands.first().and_then(|cond| operand::condition(cond.value)) else {
                    return [None, None, None];
                };
                let target = if sem < 113 { control::branch_target(op, pc, lane).map(Cow::Owned) } else { None };
                return [Some(Cow::Borrowed(condition)), target, None];
            }
            return [control::branch_target(op, pc, lane).map(Cow::Owned), None, None];
        }
        100 => {
            let Some(count) = operand::text(op, 0) else {
                return [None, None, None];
            };
            if count == "0x0" && operands[0].kind == 164 {
                return [None, None, None];
            }
            // Keep the internal endpoint, not the displayed address of the last fetch.
            let end = pc.wrapping_add(operands[1].value).wrapping_add(2) & 0xff_ffff;
            return [Some(count), Some(Cow::Owned(operand::hex(end as i32, 0))), None];
        }
        101 => return [operand::text(op, 0).filter(|count| count != "0x0"), None, None],
        125 => {
            let mask = operands[1].value;
            return [7, 6, 5].map(|i| Some(Cow::Borrowed(if mask & (1 << i) != 0 { operand::R[i] } else { "none" })));
        }
        127 => {
            let pair = |index: usize| operand::ADDRESS_UPDATE_PAIRS[operands[index].value as usize];
            let Some((first, second)) = pair(0) else {
                return [None, None, None];
            };
            // The two pairs write plain register names, independently of the update modes.
            return [
                Some(Cow::Borrowed(operand::R[usize::from(first)])),
                Some(Cow::Borrowed(operand::R[usize::from(second)])),
                pair(1).map(|(third, _)| Cow::Borrowed(operand::R[usize::from(third) + 4])),
            ];
        }
        128 => {
            if let Some(mut text) = control::address_updates(op) {
                if let Some(separator) = text.find(" == ") {
                    let source = text.split_off(separator + " == ".len());
                    text.truncate(separator);
                    return [Some(Cow::Owned(text)), Some(Cow::Owned(source)), None];
                }
            }
            return [None, None, None];
        }
        _ => {}
    }

    let (mut args, len) = if matches!(sem, 121..=124) {
        let ([first, second], len) = compact_stack_operands(op);
        ([first, second, None], len)
    } else {
        (std::array::from_fn(|i| (i < operands.len()).then(|| operand::text(op, i)).flatten()), operands.len())
    };
    if op.family == 46 && operands.last().is_some_and(|last| last.kind == 201)
        && (args[0].is_none() || args[1].is_none())
    {
        // Indexed extrema validate the pair before either scratch write.
        return [None, None, None];
    }
    if matches!(op.family, 48 | 50) && len == 3 && !scratch_arithmetic(sem)
        && unbound_compact_rmw(args[0].as_deref(), args[1].as_deref())
    {
        return [None, None, None];
    }
    let count = len.min(args.len());
    if let Some(invalid) = args[..count].iter().position(Option::is_none) {
        // A reserved selector leaves all subsequent scratch buffers untouched.
        args[invalid..].fill(None);
    } else if (count..len).all(|index| operand::text(op, index).is_some()) {
        let unsigned = if sem == 99 { 2 } else { multiply_unsigned(sem) };
        let unsigned_extremum = minmax(sem).is_some_and(|(_, unsigned)| unsigned);
        for (index, arg) in args[..count].iter_mut().enumerate() {
            let text = arg.as_deref().unwrap_or_default();
            if text != "op3" && (sem == 41 && index == 1
                || unsigned_extremum && index >= len.saturating_sub(2)
                || matches!(index, 1 | 2) && unsigned & (1 << (2 - index)) != 0)
            {
                *arg = Some(Cow::Owned(format!("{UNSIGNED}{text}")));
            }
        }
    }
    args
}

pub(super) fn suppress_predicate(op: &Operation, _lane: Lane) -> bool {
    let operands = op.operands();
    if op.family == 50 && scratch_arithmetic(op.semantic) {
        let [dest, left, right] = std::array::from_fn(|i| operand::text(op, i));
        if dest.is_none() || left.is_none() || right.is_none()
            || op.narrow() && unbound_compact_rmw(dest.as_deref(), left.as_deref())
        {
            return true;
        }
    }
    if op.narrow() {
        compact_minmax_omits_source(op) || compact_unary_omits_source(op)
            || matches!(op.family, 9 | 10) && operands.last().is_some_and(|last| last.kind == 130 && last.value & 15 >= 13)
    } else {
        op.family == 29 && operands.get(2).is_some_and(|source| wide_minmax_omits_source(source.value))
            || wide_omits_last_operand(op)
    }
}

pub(super) fn render(op: &Operation, pc: u32, lane: Lane, previous: Option<&Operation>) -> Option<String> {
    let sem = op.semantic;
    let narrow = op.narrow();
    let operands = op.operands();
    let mnemonic = match sem {
        111 => "return",
        112 => "iret",
        115 => "eni",
        116 => "disi",
        117 => "nop",
        178 => "reset",
        194 => "update_sr1",
        _ => "",
    };
    if !mnemonic.is_empty() {
        return Some(mnemonic.to_owned());
    }
    if narrow && matches!(sem, 121..=124) {
        let (args, len) = compact_stack_operands(op);
        if args[..len].iter().any(Option::is_none)
            || len == 1 && lane == Lane::ParallelLeft && operands.last()?.kind == 130
        {
            return None;
        }
        return Some(call(None, simple(sem)?, args[..len].iter().flatten().map(AsRef::as_ref), ", ", ""));
    }
    if narrow {
        if compact_extremum_rejected(op) || compact_round_product_rejected(op) {
            return None;
        }
        if sem == 119 && operands.get(1)?.value & 0x80 == 0
            || matches!(sem, 87..=89) && matches!(operands.first()?.value, 2 | 4)
        {
            return None;
        }
        if matches!(sem, 83..=86) {
            let target = operands.first()?;
            if target.kind == 140 && target.value >= if sem == 85 { 6 } else { 2 } {
                return None;
            }
        }
        if sem == 98 && matches!(operands, [Operand { kind: 113, value: 6.., .. }, Operand { kind: 165, .. }]) {
            return None;
        }
    } else {
        if matches!(sem, 65 | 66) || op.family == 38 && matches!(sem, 31 | 34 | 39 | 40)
            || matches!(sem, 41 | 42) && operands.first()?.value == 143
        {
            return None;
        }
        if matches!(sem, 176 | 177) && op.payload & 1 != 0 {
            return None;
        }
    }

    // Wide multiply with a separate accumulator has the most ALU operands.
    let mut rendered: [Option<Text>; 4] = Default::default();
    let (start, end) = match sem {
        102 | 103 if narrow => (0, 1),
        102 | 103 => (1, operands.len()),
        121..=124 if matches!(operands.first()?.kind, 185 | 193) => (1, operands.len()),
        _ => (0, operands.len()),
    };
    let mut len = end - start;
    for (index, arg) in rendered.get_mut(..len)?.iter_mut().enumerate() {
        *arg = operand::text(op, start + index);
    }
    if sem == 126 && !narrow {
        rendered[..len].rotate_left(1);
    }
    if narrow && matches!(op.family, 48 | 50) && len == 3 {
        if operands[0].kind == 120 && operands[0].value == 7 && operands[1].kind == 106 {
            rendered[0] = Some(Cow::Borrowed("rmw2"));
        }
        if unbound_compact_rmw(rendered[0].as_deref(), rendered[1].as_deref()) {
            if !scratch_arithmetic(sem) {
                return None;
            }
            rendered[0] = None;
        }
    }
    if narrow && op.family == 44 && rendered[0].as_deref() == Some("rmw2") {
        return None;
    }
    if narrow && compact_unary_omits_source(op) {
        if lane == Lane::ParallelLeft {
            return None;
        }
        let dest = rendered[0].as_deref().filter(|s| !s.is_empty())?;
        return Some(call(None, unary(sem)?, [dest], ", ", ""));
    }
    if narrow && matches!(sem, 3 | 10 | 11)
        && matches!(operands.get(1), Some(Operand { kind: 113, value: 7, .. }))
    {
        if let Some(dest) = rendered[0].as_deref().filter(|dest| dest.starts_with('*')) {
            if lane == Lane::ParallelLeft {
                return None;
            }
            return Some(call(None, unary(sem)?, [dest], ", ", ""));
        }
    }
    if !narrow && op.family == 29 && wide_minmax_omits_source(operands.get(2)?.value) {
        len = 2;
    }
    if !narrow && matches!(sem, 83..=86) {
        // CrossView uses five bits only in the unconditional extended form.
        let mask = if op.left >> 22 == 3 { 31 } else { 15 };
        rendered[1] = Some(Cow::Owned(operand::hex((operands.get(1)?.value & mask) as i32, 0)));
    }
    if !narrow && matches!(sem, 176 | 177) {
        rendered[len] = Some(Cow::Owned(operand::hex(op.extra as i32, 0)));
        len += 1;
    }
    if !narrow && matches!(op.family, 13 | 26 | 32)
        && operands.last()?.kind == 128 && rendered[len - 1].is_none()
    {
        len -= 1;
        if let Some(name) = unary(sem) {
            return if matches!(sem, 3 | 10 | 11) && rendered[..len].iter().all(Option::is_some) {
                Some(call(None, name, rendered[..len].iter().flatten().map(AsRef::as_ref), ", ", ""))
            } else {
                None
            };
        }
    }
    if narrow && compact_minmax_omits_source(op) {
        if lane == Lane::ParallelLeft {
            return None;
        }
        len -= 1;
    }
    if op.family == 50 && scratch_arithmetic(sem) && rendered[..len].iter().any(Option::is_none) {
        if lane == Lane::ParallelLeft {
            return None;
        }
        let mut dest = rendered[0].take();
        let mut left = rendered[1].take();
        let mut right = rendered[2].take();
        if let Some(previous) = previous {
            let prior_lane = if lane == Lane::ParallelRight { Lane::ParallelLeft } else { Lane::Sequential };
            let [p_dest, p_left, p_right] = scratch_operands(previous, pc.wrapping_sub(1), prior_lane);
            if dest.is_none() {
                (dest, left, right) = (p_dest, p_left, p_right);
            } else if left.is_none() {
                (left, right) = (p_left, p_right);
            } else if right.is_none() {
                right = p_right;
            }
        }
        if let Some(dest) = dest.as_deref() {
            let right = if left.is_some() { right.as_deref().unwrap_or_default() } else { "" };
            return Some(arithmetic(sem, [dest, left.as_deref().unwrap_or_default(), right], narrow, true));
        }
        let prefix = if lane == Lane::ParallelRight { " " } else { "" };
        let symbol = if sem == 69 { '+' } else { '-' };
        let borrow = match sem {
            63 => " + borrow1",
            64 => " + borrow2",
            67 => " + borrow",
            _ => "",
        };
        return Some(format!("{prefix}{symbol}= {borrow}"));
    }

    let mut args = [""; 4];
    for (arg, value) in args[..len].iter_mut().zip(&rendered[..len]) {
        *arg = value.as_deref()?;
    }
    let args = &mut args[..len];
    if let Some(name) = simple(sem) {
        return Some(call(None, name, args.iter().copied(), ", ", ""));
    }
    if let Some(name) = unary(sem) {
        let [dest, source] = *args else { return None };
        if matches!(sem, 1 | 2 | 4 | 5) {
            if same_register(dest, source) {
                return Some(call(None, name, [dest], ", ", ""));
            }
            if matches!(dest, "rmw1" | "rmw2") {
                return Some(call(None, name, [source], ", ", ""));
            }
        }
        return Some(call(Some(dest), name, [source], ", ", ""));
    }
    if let Some((name, unsigned)) = minmax(sem) {
        if !narrow && op.family == 29 && operands[2].value < 56 {
            let address = args[2].strip_prefix('*').unwrap_or(args[2]);
            args[2] = address.strip_prefix('(').and_then(|s| s.strip_suffix(')')).unwrap_or(address);
        }
        let dest = (len == 3).then_some(args[0]);
        return Some(call(dest, name, args[len - 2..].iter().copied(), ", ", if unsigned { UNSIGNED } else { "" }));
    }
    if matches!(sem, 43..=60 | 148..=160) {
        return multiply(sem, args);
    }
    if matches!(sem, 61..=74 | 161..=163) {
        return Some(arithmetic(sem, (&*args).try_into().ok()?, narrow, false));
    }
    if matches!(sem, 41 | 42) {
        let [dest, source, shift] = *args else { return None };
        let cast = if sem == 41 { UNSIGNED } else { "" };
        // Compact shifts use explicit assignment when all three ports alias.
        let explicit = narrow && op.family == 47 && same_register(dest, shift);
        if (!narrow && dest == "rmw1" || same_register(dest, source)) && !explicit {
            return Some(format!("{cast}{source} <<= {shift}"));
        }
        return Some(format!("{dest} = {cast}{source} << {shift}"));
    }
    match sem {
        75 | 76 | 94 | 95 | 126 => {
            let name = match sem {
                75 => "add2",
                76 => "sub2",
                94 => "extracts",
                95 => "extractz",
                _ => "load",
            };
            let (dest, rest) = args.split_first()?;
            Some(call(Some(dest), name, rest.iter().copied(), ", ", ""))
        }
        93 => Some(call(None, "insert", args.iter().copied(), ",", "")),
        96 | 97 => Some(format!("{} /= {}", args.first()?, args.get(1)?)),
        170 | 171 => Some(format!("btr{} = {} / {}", sem - 169, args.first()?, args.get(1)?)),
        98 | 99 | 175 if len == 2 => {
            let cast = if sem == 98 { "" } else { UNSIGNED };
            Some(format!("{} = {cast}{}", args[0], args[1]))
        }
        _ => None,
    }
}
