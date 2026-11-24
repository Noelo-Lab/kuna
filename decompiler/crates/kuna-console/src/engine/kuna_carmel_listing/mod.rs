//! CrossView-style text for a framed Carmel issue. SLEIGH still controls
//! instruction starts, widths, and executable p-code.

mod alu;
mod control;
mod decode;
mod operand;

use decode::{Operand, Operation};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lane {
    Sequential,
    ParallelLeft,
    ParallelRight,
}

fn operation_text(op: &Operation, pc: u32, lane: Lane, previous: Option<&Operation>) -> Option<String> {
    let sem = op.semantic;
    let operands = op.operands();
    if !op.narrow() && op.left >> 22 == 3 {
        match op.family {
            5 | 19 | 21 => return None,
            13 if sem == 12 => return None,
            37 if matches!(sem, 20 | 21 | 25 | 26 | 137..=144) => return None,
            22 if !matches!(operands[0].value, 0x40..=0x7f | 0x88..=0x8e) => return None,
            28 if matches!(operands.last()?.value, 0xc0 | 0xc2..=0xc7) => return None,
            _ => {}
        }
    }
    let mut text = match control::render(op, pc, lane) {
        Some(text) => text?,
        None => alu::render(op, pc, lane, previous)?,
    };
    if alu::suppress_predicate(op, lane) {
        return Some(text);
    }
    if !op.narrow() {
        if op.left & 0x800000 == 0 && !matches!(sem, 108..=110 | 164..=169 | 188) {
            text.insert_str(0, if op.left & 0x400000 == 0 { "ifexe(ce1) " } else { "ifexe(ce2) " });
        }
    } else if op.left & 1 != 0
        && !matches!(sem, 100 | 101 | 107..=110 | 113 | 114)
        && !(sem == 82 && (0..12).step_by(2).any(|bit| operands[0].value >> bit & 3 == 3))
        && !control::compact_incomplete_address_update(op)
    {
        text.insert_str(0, "ifexe ");
    }
    Some(text)
}

fn end_statement(text: &mut String, op: &Operation) {
    let sem = op.semantic;
    if matches!(sem, 100 | 101 | 164..=169 | 188) {
        return;
    }
    if matches!(sem, 104 | 106 | 108..=110 | 113 | 114) {
        let ops = op.operands();
        let delay = if op.narrow() { ops.last() } else { ops.first() }.map_or(0, |op| op.value);
        if delay != 0 && !(op.narrow() && matches!(sem, 108..=110) && delay == 3) {
            return;
        }
    }
    text.push(';');
}

fn fetch_word(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])
}

pub(super) fn print(fetch: &[u8; 6], pc: u32, size: i32, mnemonic: &mut String, body: &mut String) {
    let left = fetch_word(fetch);
    let right = fetch_word(&fetch[3..]);
    let text = (|| {
        if size == 3 {
            let (raw, previous) = if pc & 1 == 0 { (left, None) } else { (right, decode::compact(left)) };
            let op = decode::compact(raw)?;
            let mut text = operation_text(&op, pc, Lane::Sequential, previous.as_ref())?;
            end_statement(&mut text, &op);
            return Some(text);
        }
        if right & 0x800000 != 0 {
            let op = decode::wide(left, right)?;
            let mut text = operation_text(&op, pc, Lane::Sequential, None)?;
            end_statement(&mut text, &op);
            return Some(text);
        }
        let first = decode::compact(left)?;
        let second = decode::compact(right)?;
        let mut head = operation_text(&first, pc, Lane::ParallelLeft, None)?;
        let tail = operation_text(&second, pc + 1, Lane::ParallelRight, Some(&first))?;
        head.push_str(if head.is_empty() { "|| " } else { " || " });
        head.push_str(&tail);
        end_statement(&mut head, &second);
        Some(head)
    })();
    let Some(mut text) = text else {
        mnemonic.push_str("???;");
        return;
    };
    if let Some(at) = text.find(' ') {
        body.push_str(&text[at + 1..]);
        text.truncate(at);
    }
    mnemonic.push_str(&text);
}
