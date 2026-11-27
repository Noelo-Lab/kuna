//! (kuna) `switchflow` — a jump table whose base or range check lies past
//! where [`super::kuna_switchtable`]'s straight walk back from the dispatch
//! stops.
//!
//! # The gap
//!
//! The dispatch slice steps back one decoded instruction at a time and stops
//! at the first one that does not fall into the next, and the range-check
//! search stops sixteen instructions up. Three shapes every x86-64 compiler
//! emits sit behind one or the other:
//!
//! ```text
//! lea  jt(%rip),%r12            ; hoisted out of the loop
//! loop: ...
//!       jne  1f
//!       call branch_b
//!       jmp  2f                 ; <- the slice stops here
//! 1:    call branch_a
//! 2:    cmp  $7,%r13d; ja default; movslq (%r12,%rax,4),%rax; add %r12,%rax; jmp *%rax
//! ```
//!
//! the `if/else` merge above (clang `-O2`, MSVC `/O2`); MSVC's one `LEA
//! RDI,[__ImageBase]` shared by two switches, the second dispatched after the
//! first one's case bodies end in `jmp`; and `if (op < 8) { ...; switch (op) }`,
//! whose check is forty instructions above the jump. Each lost the whole
//! switch, and every call its case bodies make.
//!
//! # The rule
//!
//! Each widening runs only where the straight read finds no table.
//!
//! * **Crossing.** Where the slice reaches the head of a block nothing is
//!   known to fall into, and the walk has decoded exactly one branch into that
//!   head (a conditional branch's taken side, a `jmp`, a switch case), the
//!   slice goes on from that branch, but only when the instructions gathered
//!   so far read no table as they are and would read one once every register
//!   they still want held a constant ([`missing_base`]), and that read's index
//!   has the checked range check below or a mask. The base is then the one
//!   thing missing, so a jump through a pointer, a virtual call or a returned
//!   function pointer is followed no further than before. A function entry is
//!   not crossed.
//! * **A checked range check.** A read that crossed, and any check past the
//!   sixteen-instruction window (up to [`MAX_FAR_GUARD_INSNS`] above the
//!   jump), counts only when no decoded branch enters the run between the
//!   check and the jump and the comparison reads the register the table is
//!   indexed by ([`Selector`]), or one that register was copied from just
//!   above the comparison (MSVC's `mov r10d,ecx; cmp ecx,8; jae default; ...
//!   mov ecx,[rdx+r10*4+rvas]`, [`copied_above`]). Its count is exact: `jae`
//!   after `cmp $c` admits `c` cases, `ja` `c + 1`.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::kuna_fasthash::FxHashMap;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{
    falls_through, fold_slice, is_constant, key_of, keys_overlap, leaves_registers_unknown, previous, survives, Key,
    Val, MAX_GUARD_INSNS,
};
use super::xrefs::{decode, FullCapture, FullOp};

/// How far above a dispatch a range check of its own selector may sit.
pub(super) const MAX_FAR_GUARD_INSNS: usize = 128;

/// What the walk knows about the branches into one address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Entry {
    /// No decoded branch targets it.
    Unentered,
    /// Exactly this decoded instruction branches to it.
    One(u64),
    /// Several do, or it is a function entry.
    Several,
}

/// The decoded branches into each address, as the walk files them.
#[derive(Default)]
pub(super) struct Entries(FxHashMap<u64, u64>);

const SEVERAL: u64 = u64::MAX;

impl Entries {
    pub(super) fn record(&mut self, to: u64, from: u64) {
        self.0.entry(to).and_modify(|f| if *f != from { *f = SEVERAL }).or_insert(from);
    }

    pub(super) fn at(&self, to: u64) -> Entry {
        match self.0.get(&to) {
            None => Entry::Unentered,
            Some(&SEVERAL) => Entry::Several,
            Some(&from) => Entry::One(from),
        }
    }
}

/// Does the slice gathered so far (`gathered`, last instruction first) reach
/// the computed jump at `from` as a table read only once every key in `need`
/// holds a constant? A slice that reads a table already, its index left
/// unknown, is not missing its base. The answer carries the addend of the
/// table-address sum that read would index by.
pub(super) fn missing_base(
    gathered: &[(u64, Vec<FullOp>)],
    need: &[Key],
    from: u64,
    data: Option<&Rc<AddrSpace>>,
    pool: &PoolImage,
    preserved: &[(u64, u64)],
) -> Option<Option<(u64, usize, Key)>> {
    let table = |vals: Vec<(Key, Val)>| {
        let (dispatch, call, index) = fold_slice(gathered.iter().rev(), vals, from, data, pool, preserved);
        let read = !call
            && match dispatch {
                Some(Val::Case { .. } | Val::Relative { .. }) => true,
                Some(Val::Entry { width, .. }) => width == pool.ptr_width(),
                _ => false,
            };
        read.then_some(index)
    };
    if table(Vec::new()).is_some() {
        return None;
    }
    table(need.iter().map(|&k| (k, Val::Const(0))).collect())
}

/// The register holding the table index, followed up from the table-address
/// sum through copies, extensions and scaling by a constant, as the range
/// check search steps back over each instruction. A call ends it unless the
/// calling convention preserves the register.
pub(super) struct Selector {
    site: Option<(u64, usize)>,
    key: Option<Key>,
}

impl Selector {
    /// `index` is the addend of the table-address sum the fold could not value:
    /// (instruction, op, varnode).
    pub(super) fn new(index: Option<(u64, usize, Key)>) -> Self {
        Selector { site: index.map(|(at, op, _)| (at, op)), key: index.map(|(_, _, key)| key) }
    }

    /// Steps back over the instruction at `at`, whose p-code is `ops`.
    pub(super) fn step(&mut self, at: u64, ops: &[FullOp], data: Option<&Rc<AddrSpace>>, preserved: &[(u64, u64)]) {
        let ops = match self.site {
            Some((site, upto)) if site == at => {
                self.site = None;
                &ops[..upto.min(ops.len())]
            }
            Some(_) => return,
            None => ops,
        };
        let Some(mut key) = self.key else { return };
        for op in ops.iter().rev() {
            if leaves_registers_unknown(op.opcode) && !survives(key, preserved) {
                self.key = None;
                return;
            }
            let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
            if !keys_overlap(out, key) {
                continue;
            }
            let input = |i: usize| op.ins.get(i).and_then(|vn| key_of(vn, data));
            let constant = |i: usize| op.ins.get(i).is_some_and(is_constant);
            if out != key {
                if op.opcode == OpCode::CPUI_INT_ZEXT && input(0) == Some(key) {
                    continue;
                }
                self.key = None;
                return;
            }
            let next = match op.opcode {
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => input(0),
                OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_LEFT if constant(1) => input(0),
                OpCode::CPUI_INT_MULT if constant(0) => input(1),
                _ => None,
            };
            let Some(next) = next else {
                self.key = None;
                return;
            };
            key = next;
        }
        self.key = Some(key);
    }

    /// The register holding the index where the search stands, once it has
    /// stepped back past the sum.
    pub(super) fn key(&self) -> Option<Key> {
        self.key.filter(|_| self.site.is_none())
    }
}

/// Is `key` a copy of a register in `compared` made in the straight run just
/// above the comparison at `cmp`, with neither written in between? The copy
/// may be a zero or sign extension, or a chain of them.
#[allow(clippy::too_many_arguments)]
pub(super) fn copied_above(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &dyn Fn(u64) -> bool,
    entered: &dyn Fn(u64) -> Entry,
    cmp: u64,
    mut key: Key,
    compared: &[Key],
) -> bool {
    let mut cap = FullCapture::default();
    let mut ends_at = cmp;
    for _ in 0..MAX_GUARD_INSNS {
        if entered(ends_at) != Entry::Unentered {
            return false;
        }
        let Some(cur) = previous(decoded, ends_at) else { return false };
        let Some(len) = decode(translate, cur, code_space, &mut cap) else { return false };
        if cur.wrapping_add(u64::from(len)) != ends_at || !falls_through(cap.ops()) {
            return false;
        }
        for op in cap.ops().iter().rev() {
            if leaves_registers_unknown(op.opcode) {
                return false;
            }
            let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
            if compared.iter().any(|c| keys_overlap(*c, out)) {
                return false;
            }
            if !keys_overlap(out, key) {
                continue;
            }
            let input = op.ins.first().and_then(|vn| key_of(vn, data));
            if out != key {
                if op.opcode == OpCode::CPUI_INT_ZEXT && input == Some(key) {
                    continue;
                }
                return false;
            }
            let copy = matches!(op.opcode, OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT);
            let Some(src) = input.filter(|_| copy) else { return false };
            if compared.iter().any(|c| keys_overlap(*c, src)) {
                return true;
            }
            key = src;
        }
        ends_at = cur;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_branch_into_an_address_is_one_and_two_are_several() {
        let mut e = Entries::default();
        e.record(0x30, 0x10);
        e.record(0x30, 0x10);
        e.record(0x40, 0x10);
        e.record(0x40, 0x20);
        assert_eq!(e.at(0x30), Entry::One(0x10));
        assert_eq!(e.at(0x40), Entry::Several);
        assert_eq!(e.at(0x50), Entry::Unentered);
    }
}
