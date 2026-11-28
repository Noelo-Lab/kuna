//! d8 lowers javac's string concatenation to a `StringBuilder` and, in a
//! release build, drops the `move-result` of each `append` (the call returns
//! the builder it was called on), so the chain becomes a builder in a
//! register and one statement per `append`. `StringBuilder.append` returns
//! `this`, so the register still holds the call's result: a builder created,
//! appended to and turned into its string within straight-line code on one
//! source line, read by nothing else and not named by debug info, gets its
//! `move-result`s back (each `append` writes the register again), and the
//! chain is one expression as javac wrote it. (A builder the source appends
//! to in statements of its own looks the same, but its statements are on
//! lines of their own.)

use std::collections::BTreeSet;

use crate::insn::Insn;
use crate::switchmap::Regs;
use crate::Dex;

const SB: &[u8] = b"Ljava/lang/StringBuilder;";

/// Inserts the dropped `move-result-object`s of every such chain. `entered`
/// holds the addresses branches and handlers reach, `tries` the protected
/// ranges, `named(reg, start, end)` whether debug info names the register
/// within the range and `line(addr)` the source line of an address.
pub fn rechain(dex: &Dex, insns: &mut Vec<Insn>, entered: &BTreeSet<u32>, tries: &[(u32, u32)], regs: Regs, named: &dyn Fn(u32, u32, u32) -> bool, line: &dyn Fn(u32) -> Option<u32>) {
    let mut k = 0;
    while k < insns.len() {
        if let Some(appends) = chain(dex, insns, k, entered, tries, regs, named, line) {
            for &j in appends.iter().rev() {
                let mut res = insns[j].clone();
                res.op = 0x0c;
                res.a = insns[j].args()[0];
                res.addr = insns[j].next();
                res.size = 0;
                insns.insert(j + 1, res);
            }
            k += appends.len();
        }
        k += 1;
    }
}

fn method_is(dex: &Dex, x: &Insn, name: &[u8], ret: &[u8]) -> bool {
    let Ok(m) = dex.method(x.idx) else { return false };
    let desc = dex.proto_desc(m.proto).unwrap_or_default();
    dex.type_desc(m.class).is_ok_and(|c| c == SB) && dex.string(m.name).is_ok_and(|n| n == name) && desc.ends_with(ret)
}

/// The `append`s without a result of the chain a `new-instance` at `k` starts.
#[allow(clippy::too_many_arguments)]
fn chain(dex: &Dex, insns: &[Insn], k: usize, entered: &BTreeSet<u32>, tries: &[(u32, u32)], regs: Regs, named: &dyn Fn(u32, u32, u32) -> bool, line: &dyn Fn(u32) -> Option<u32>) -> Option<Vec<usize>> {
    let x = &insns[k];
    if x.op != 0x22 || dex.type_desc(x.idx).ok()? != SB {
        return None;
    }
    let r = x.a;
    let range = |a: u32| tries.iter().position(|&(s, e)| s <= a && a < e);
    let item = range(x.addr);
    let mut j = k + 1;
    loop {
        let y = insns.get(j)?;
        if entered.contains(&y.addr) || range(y.addr) != item {
            return None;
        }
        if matches!(y.op, 0x70 | 0x76) && y.args().first() == Some(&r) {
            if !method_is(dex, y, b"<init>", b")V") {
                return None;
            }
            break;
        }
        let (reads, writes) = regs(y);
        if reads.contains(&r) || writes.contains(&r) || matches!(y.op, 0x0e..=0x11 | 0x27..=0x2c | 0x32..=0x3d) {
            return None;
        }
        j += 1;
    }
    let mut appends = Vec::new();
    j += 1;
    loop {
        let y = insns.get(j)?;
        if entered.contains(&y.addr) || range(y.addr) != item {
            return None;
        }
        let receiver = matches!(y.op, 0x6e | 0x74) && y.args().first() == Some(&r);
        let result = insns.get(j + 1).is_some_and(|n| matches!(n.op, 0x0a..=0x0c));
        if receiver && method_is(dex, y, b"toString", b")Ljava/lang/String;") && result {
            break;
        }
        if receiver && method_is(dex, y, b"append", b")Ljava/lang/StringBuilder;") && !result {
            appends.push(j);
            j += 1;
            continue;
        }
        let (reads, writes) = regs(y);
        if reads.contains(&r) || writes.contains(&r) || matches!(y.op, 0x0e..=0x11 | 0x27..=0x2c | 0x32..=0x3d) {
            return None;
        }
        j += 1;
    }
    let first = line(x.addr);
    if appends.is_empty() || named(r, x.addr, insns[j].next()) || insns[k..=j].iter().any(|y| line(y.addr) != first) {
        return None;
    }
    Some(appends)
}
