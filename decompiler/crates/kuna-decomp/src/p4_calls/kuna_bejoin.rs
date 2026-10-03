//! (kuna) The order a two-register return value is joined in on an ABI that
//! puts its HIGH word in the first register.
//!
//! PowerPC (r3:r4), MIPS o32 big-endian (v0:v1), SPARC (o0:o1), ARM big-endian
//! (r0:r1) and AVR's gcc ABI (R25:R24) return a value twice a register's width
//! with its most significant half in the first return register, and their
//! output rule says so (`ParamActive::join_pair_order`). Return recovery used
//! to join every pair first register low, so a `long long` came back with its
//! halves swapped.
//!
//! Joining in the ABI's order is only right when the second register really is
//! the low word. A function returning one register often leaves something else
//! in the second: an argument the window hands back (SPARC's `restore` copies
//! `%i1` into `%o1`), a scratch value it also used, a literal `-O0` code wrote
//! and never read (clang's `addiu $3,$zero,0` on MIPS). Joined first register
//! low, such a pair narrows back to the first register everywhere a later pass
//! drops the phantom half (the uncomputed-half repair, a boolean or byte
//! return, C's own truncation in a narrow prototype), which is the right
//! value. Joined in the ABI's order, every one of those keeps the wrong
//! register.
//!
//! So a pair is joined in the ABI's order only when its low word is returned on
//! purpose, and first register low otherwise, exactly as before. The second
//! register is judged at every live RETURN by walking back through moves to
//! where its value was made ([`classify`]):
//!
//! * the two halves of one wider value (an 8-byte load, a product split by
//!   `mflo`/`mfhi`): a wide value, whatever else the function does with it at
//!   that RETURN;
//! * the register's own entry value, reached without an instruction that moves
//!   it (untouched, or carried by a register window): a leftover;
//! * a value also used for anything but the pair's RETURN slots (a store, a
//!   call, a branch, an address, the first register other than through its
//!   sign or a carry), or the first register's own value: scratch;
//! * what a comparison left behind: the first register holds a computed 0 or
//!   1 (`return x >= 0x80000000ULL` on ARM big-endian leaves `0x7fffffff - lo`
//!   in r1 next to the flag in r0), or a literal 0 or 1 where the function
//!   returns both at its RETURNs (a boolean a branch chose: SPARC `addcc;
//!   addxcc; cmp; be` then `restore %g0,1,%o0` leaves the sum in `%o1`), or
//!   the low word is a sum or difference and no carry or borrow out of it
//!   reaches the first register (the halves of a real 64-bit sum are tied by
//!   its carry), or the low word is a sum or difference whose carry or borrow
//!   a branch or the first register tests while the first register is worked
//!   out from comparisons and literals alone, without that carry as one of
//!   them (`return 34` or `return 0` after a range check, `neg 3,3` of a
//!   flag; a 64-bit sum adds the carry itself into its high word);
//! * an `int` truncation: the two registers loaded from adjacent words of one
//!   stack object, the first register from its less significant word -- the
//!   reverse of the order a `long long` comes back in (clang -O0 returns
//!   `(int)t` of a 64-bit temporary as `lwz 4,12(31); lwz 3,16(31)` on
//!   PowerPC, and `t` itself as `lwz 3,12(31); lwz 4,16(31)`). This and the
//!   comparison tests apply only where one register holds a whole `int`,
//!   which AVR's byte registers do not;
//! * a division by a constant: the low half of a product whose high half
//!   reaches the first register shifted down (ARM's `umull r1,r0,r2,r3; lsr
//!   r0,r0,#3` for `a / 10` leaves the product's low half in r1);
//! * the high half of a 64-bit temporary held the reverse way round: a right
//!   shift whose dropped bits reach the first register (ARM's `lsrs r1,r1,#1;
//!   rrx r0,r0` for `(u32)(((u64)a + b) >> 1)` leaves the stale `r1 >> 1`);
//! * an `int` worked out from the high half of a 64-bit temporary, its low
//!   half left behind: the low word is a sum or difference whose carry or
//!   borrow reaches the first register other than as the high word of that
//!   sum itself (`(u32)((x + y) >> 48)` is `addc 4,6,4; adde 3,5,3; srwi
//!   3,3,16` on PowerPC), or, with no carry between them, the first register
//!   is a literal operation -- a shift, a mask, an offset, a negation, a
//!   product -- on a value the low word never reads (gcc -O0 on MIPS masks
//!   `$2` alone in `(u32)((x ^ y) >> 32) & 0xffff`);
//! * the high half of a product beside a first register that reads its low
//!   half: the reverse of a `long long` (clang -O0 on MIPS reloads an
//!   `mfhi` it spilled into `$3`);
//! * a literal zero: `-O0` leaves one behind, so it proves nothing either way;
//! * a callee's clobber or a location never written: nothing;
//! * anything else -- a value or a nonzero literal nothing but the RETURNs read
//!   -- is returned on purpose.
//!
//! A function with an indirect jump flow could not follow has code return
//! recovery does not see (the cases of a switch whose table was not
//! recovered), and an operation given no register inputs (an inline system
//! call) reads the registers set up for it, so a low word nothing visible
//! reads, made where such a reader can read it, is not returned on purpose
//! either ([`hidden_readers`], [`reaches_jump`]).
//!
//! The pair joins in the ABI's order when some RETURN holds a wide value or
//! returns its low word on purpose, and none holds a leftover, scratch, a
//! comparison's leftover, a truncation, a quotient's, a shift's, a reworked
//! temporary's or a product's leftover there. A function whose pair keeps
//! the old join joins its calls' output pairs the old way too
//! ([`joins_first_low`]), so a pair a call hands back and the function
//! returns stays one value.
//!
//! The rule is a prior, not a proof, so it ships behind `option bejoin`, off
//! by default (GH-904): off joins every pair first register low, as before.
//! An `int` that leaves the stale low half of a 64-bit temporary in the second
//! register compiles to the same registers as the `long long` that returns it
//! whole -- `(a + b) >> 32` on PowerPC -O2 is `addc 4,4,6; addze 3,3; add
//! 3,3,5`, the code for `a + b`, and `((u64)a * b) >> 32` on ARM big-endian
//! is the `umull` of `(u64)a * b` -- and reads as that `long long`. The other way
//! round, `(u64)x << 32` leaves the same zero in the second register as a
//! function returning `int`, a `long long` whose low word also feeds a call
//! looks like scratch, a `long long` whose high word is only a carry, or a
//! literal beside a low word whose carry a branch tests, looks like a
//! comparison, one whose high word is a literal operation on a value its low
//! word does not read (`*p | 0xffULL << 32`) looks like a reworked
//! temporary, and one returned beside an unfollowed jump looks like hidden
//! scratch; those keep the old join, which is what they printed before.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// How many Varnodes one walk visits before it gives up (and calls the value
/// scratch, the old join).
const MAX_NODES: usize = 4096;

/// How many operations deep [`literal_value`] looks: SPARC builds a
/// 32-bit constant in two (`sethi` then `or`).
const LITERAL_DEPTH: u32 = 4;

/// The trial indexes `(low, high)` to join the two used output trials of
/// `active` in: the ABI's order when [`low_word_returned`], first trial low
/// otherwise.
pub fn join_order(active: &ParamActive, data: &Funcdata, return_ops: &[OpId]) -> (int4, int4) {
    let abi = active.join_pair_order();
    if abi == (0, 1) || !live(data) || !low_word_returned(active, data, return_ops) {
        (0, 1)
    } else {
        abi
    }
}

/// Is `option bejoin` on: may a pair join in the ABI's order at all? Off,
/// every pair joins first register low, as it did before the option.
pub fn live(data: &Funcdata) -> bool {
    data.get_arch().be_join
}

/// The trial indexes `(low, high)` a call's two output trials in `active`
/// join in: first register low when the option is off or the calling
/// function's own pair joined that way ([`joins_first_low`]), the ABI's order
/// otherwise.
pub fn call_join_order(data: &Funcdata, active: &ParamActive) -> (int4, int4) {
    if !live(data) || data.kuna_pairs_first_low() {
        (0, 1)
    } else {
        active.join_pair_order()
    }
}

/// Did `join_order` pick `order` against the ABI: a pair of used trials the
/// ABI joins first register high, joined first register low? The function's
/// calls then join their own pairs the same way, so a pair a call hands back
/// and the function returns stays one value.
pub fn joins_first_low(active: &ParamActive, order: (int4, int4)) -> bool {
    let used = (0..active.get_num_trials()).take_while(|&i| active.get_trial(i).is_used()).count();
    used == 2 && order == (0, 1) && active.join_pair_order() != (0, 1)
}

/// What one RETURN holds in the second register ([`classify`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LowWord {
    /// The low half of the wide value the first register holds the high half of.
    Wide,
    /// A value or nonzero literal the function returns on purpose.
    Returned,
    /// A literal zero.
    Zero,
    /// A callee's clobber or a location the function never wrote.
    Nothing,
    /// The register's own entry value, left in place or carried by a window.
    Entry,
    /// A value the function also used for something else.
    Scratch,
    /// What a comparison left behind: the first register holds a comparison's
    /// result, or the low word is a sum or difference no carry ties the first
    /// register to, or one whose carry is tested beside a first register
    /// worked out from comparisons and literals.
    Compared,
    /// The less significant word of a stack object in the first register, the
    /// more significant one in the second: an `int` truncation.
    Truncated,
    /// The low half of a product whose high half the first register shifts
    /// down: a division by a constant.
    Quotient,
    /// A right shift of a value whose shifted-out bits the first register
    /// holds: the high half of a 64-bit temporary kept in the reverse pair.
    ShiftedOut,
    /// A sum or difference whose carry or borrow reaches the first register
    /// through more than the additions of its own high word: an `int` worked
    /// out from the high half of a 64-bit temporary, its low half left behind.
    Reworked,
    /// The high half of a product whose low half the first register reads:
    /// the reverse of the order a `long long` product comes back in.
    ProductHigh,
}

/// Is the low word of the pair `active` joins returned on purpose at the live
/// RETURNs of `data` (see the module header)?
fn low_word_returned(active: &ParamActive, data: &Funcdata, return_ops: &[OpId]) -> bool {
    let used = (0..active.get_num_trials()).take_while(|&i| active.get_trial(i).is_used()).count();
    if used != 2 {
        return false;
    }
    let (lo, hi) = active.join_pair_order();
    let (lo_t, hi_t) = (active.get_trial(lo), active.get_trial(hi));
    let live: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| {
            data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0) && !never_reached(data, r)
        })
        .collect();
    let first_literals: BTreeSet<u64> = live
        .iter()
        .filter_map(|&r| data.obank().get(r).and_then(|o| o.get_in(hi_t.get_slot())))
        .filter_map(|h| literal_value(data, h, LITERAL_DEPTH))
        .collect();
    let pair = Pair {
        lo_slot: lo_t.get_slot(),
        hi_slot: hi_t.get_slot(),
        lo_size: lo_t.get_size(),
        own: lo_t.get_address().clone(),
        hi_own: hi_t.get_address().clone(),
        chosen_flag: first_literals.contains(&0) && first_literals.contains(&1),
    };
    let mut returned = false;
    let mut vetoed = false;
    let mut jumps: Option<Vec<BlockId>> = None;
    for retop in live {
        match classify(data, retop, &pair) {
            LowWord::Wide => returned = true,
            LowWord::Returned => {
                let jumps = jumps.get_or_insert_with(|| hidden_readers(data));
                if !jumps.is_empty() && reaches_jump(data, retop, &pair, jumps) {
                    vetoed = true
                } else {
                    returned = true
                }
            }
            LowWord::Entry
            | LowWord::Scratch
            | LowWord::Compared
            | LowWord::Truncated
            | LowWord::Quotient
            | LowWord::ShiftedOut
            | LowWord::Reworked
            | LowWord::ProductHigh => vetoed = true,
            LowWord::Zero | LowWord::Nothing => {}
        }
    }
    returned && !vetoed
}

/// The blocks holding code return recovery cannot see read registers: an
/// indirect jump flow could not follow, now a call followed by an artificial
/// RETURN (`Treating indirect jump as call`), whose cases are not in the
/// function; and a user-defined operation given no register inputs, which
/// may read any of them (PowerPC's `sc` lifts to `syscall()`, which reads the
/// arguments set up in r3 and r4).
fn hidden_readers(data: &Funcdata) -> Vec<BlockId> {
    let jumps = data.obank().iter_code(OpCode::CPUI_RETURN).filter_map(|r| {
        let op = data.obank().get(r)?;
        let artificial = !op.is_dead()
            && op.get_halt_type() == 0
            && op.get_in(0).and_then(|x| data.vbank().get(x)).is_some_and(|x| x.is_constant());
        let bl = op.get_parent().filter(|_| artificial)?;
        let ops = data.bb_ops(bl);
        let at = ops.iter().position(|&o| o == r)?.checked_sub(1)?;
        data.obank().get(ops[at]).filter(|prev| prev.code() == OpCode::CPUI_CALLIND).map(|_| bl)
    });
    let opaque = data.obank().iter_code(OpCode::CPUI_CALLOTHER).filter_map(|c| {
        let op = data.obank().get(c).filter(|op| !op.is_dead())?;
        let blind = (1..op.num_input()).all(|i| op.get_in(i).and_then(|x| data.vbank().get(x)).is_some_and(|x| x.is_constant()));
        op.get_parent().filter(|_| blind)
    });
    jumps.chain(opaque).collect()
}

/// Could one of the hidden readers in `jumps` ([`hidden_readers`]) read the
/// low word `retop` returns: is one of the values it was made from an input,
/// a literal, or made in a block that dominates a reader? gcc copies an argument
/// into `$3` before a switch for the case that passes it on (`move v1,a1`),
/// and the default path returns with the copy still there; a value made
/// after the switch, on the path that returns it, is out of the cases' reach.
fn reaches_jump(data: &Funcdata, retop: OpId, pair: &Pair, jumps: &[BlockId]) -> bool {
    let Some(low) = data.obank().get(retop).and_then(|o| o.get_in(pair.lo_slot)) else { return true };
    let ends = ends_of(data, low, &pair.own, pair.lo_size);
    ends.values.iter().any(|&(v, _)| {
        match data.vbank().get(v).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)).and_then(|op| op.get_parent()) {
            None => true,
            Some(made) => jumps.iter().any(|&j| data.bblocks_ref().dominates(made, Some(j))),
        }
    })
}

/// Where the two trials sit in a RETURN, and the low word's register.
struct Pair {
    lo_slot: int4,
    hi_slot: int4,
    lo_size: int4,
    own: Address,
    /// The high word's register.
    hi_own: Address,
    /// The first register holds the literal 0 at some live RETURN and 1 at
    /// another: a boolean a branch chose.
    chosen_flag: bool,
}

/// Classify the second register `retop` returns.
fn classify(data: &Funcdata, retop: OpId, pair: &Pair) -> LowWord {
    let Some(o) = data.obank().get(retop) else { return LowWord::Scratch };
    let (Some(low), Some(high)) = (o.get_in(pair.lo_slot), o.get_in(pair.hi_slot)) else {
        return LowWord::Scratch;
    };
    let int_sized = pair.lo_size >= int_size(data);
    if int_sized && computed_flag(data, high) {
        return LowWord::Compared;
    }
    if halves_of_one_value(data, low, high, pair.lo_size) {
        return LowWord::Wide;
    }
    let ends = ends_of(data, low, &pair.own, pair.lo_size);
    if ends.overflow {
        return LowWord::Scratch;
    }
    if ends.values.iter().any(|&(v, off)| !only_returned(data, v, off, pair)) {
        return LowWord::Scratch;
    }
    if ends.entry {
        return LowWord::Entry;
    }
    if ends.values.is_empty() {
        return LowWord::Nothing;
    }
    let word = |(v, off): (VarnodeId, int4)| {
        literal_value(data, v, LITERAL_DEPTH).map(|k| k.checked_shr(8 * off as u32).unwrap_or(0) & ones(pair.lo_size))
    };
    if ends.values.iter().all(|&e| word(e) == Some(0)) {
        return LowWord::Zero;
    }
    if ends.values.iter().any(|&(v, off)| off == 0 && divided_down(data, v, high, pair.lo_size)) {
        return LowWord::Quotient;
    }
    if ends.values.iter().any(|&(v, off)| off == 0 && shifts_into_first(data, v, high)) {
        return LowWord::ShiftedOut;
    }
    if int_sized && pair.chosen_flag && matches!(literal_value(data, high, LITERAL_DEPTH), Some(0 | 1)) {
        return LowWord::Compared;
    }
    if int_sized && ends.values.iter().any(|&(v, off)| off == 0 && sum_or_difference(data, v)) && carry_free(data, high, low, &ends)
    {
        return LowWord::Compared;
    }
    if int_sized
        && ends.values.iter().any(|&(v, off)| {
            off == 0 && sum_or_difference(data, v) && {
                let carries = carries_of(data, v);
                tests_carry(data, &carries, high) && of_flags(data, high, &carries)
            }
        })
    {
        return LowWord::Compared;
    }
    if int_sized && reworked(data, high, low, pair) {
        return LowWord::Reworked;
    }
    if int_sized
        && ends.values.iter().any(|&(v, off)| {
            product_high_half(data, v, off, pair.lo_size).is_some_and(|p| reads_low_half(data, high, p, pair.lo_size))
        })
    {
        return LowWord::ProductHigh;
    }
    if int_sized && loads_low_word_first(data, high, low, pair) {
        return LowWord::Truncated;
    }
    LowWord::Returned
}

/// Is `low` a right shift by a literal `c` of a value whose low `c` bits --
/// the bits the shift drops -- reach the first register `high`? That is a
/// 64-bit temporary held the reverse way round, its high half in the second
/// register: ARM big-endian `(u32)(((u64)a + b) >> 1)` is `adds r0,r1,r0; adc
/// r1,r2,#0; lsrs r1,r1,#1; rrx r0,r0`, which moves bit 0 of `r1` into `r0`
/// and leaves the stale `r1 >> 1` behind. A `long long` shifted right moves
/// the bit the other way, from the first register into the second, so its
/// low word is never the bare shift. Reading the value shifted right by `c`
/// or more reads none of the dropped bits. The walk follows a stack reload
/// to what the function stored there (-O0 keeps the result in a local). A
/// shift that leaves only the top bit or the sign (`x >> 31`) is a value of
/// its own, which a saturated `long long` puts in its low word beside the
/// high word it came from.
fn shifts_into_first(data: &Funcdata, low: VarnodeId, high: VarnodeId) -> bool {
    let def = |vn: VarnodeId| data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let shift_by = |op: &crate::op::PcodeOp| op.get_in(1).and_then(|k| literal_value(data, k, LITERAL_DEPTH));
    let Some(op) = def(low).filter(|op| matches!(op.code(), OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT)) else {
        return false;
    };
    let (Some(src), Some(c)) = (op.get_in(0), shift_by(op)) else { return false };
    let bits = data.vbank().get(src).map_or(0, |v| 8 * v.get_size() as u64);
    if c == 0 || c + 1 >= bits {
        return false;
    }
    let source = same_value_key(data, src);
    let mut stored: Option<Vec<((u64, i64), VarnodeId)>> = None;
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![high];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > MAX_NODES {
            continue;
        }
        let Some(op) = def(cur) else { continue };
        if op.code() == OpCode::CPUI_LOAD {
            if let Some(slot) = stack_load(data, cur) {
                let stores = stored.get_or_insert_with(|| stack_stores(data));
                work.extend(stores.iter().filter(|(at, _)| *at == slot).map(|&(_, value)| value));
            }
            continue;
        }
        let inputs = match op.code() {
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => 0,
            OpCode::CPUI_INDIRECT => 1,
            _ => op.num_input(),
        };
        for i in 0..inputs {
            let Some(x) = op.get_in(i) else { continue };
            if same_value_key(data, x) != source {
                work.push(x);
                continue;
            }
            let above = i == 0
                && matches!(op.code(), OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT)
                && shift_by(op).is_some_and(|k| k >= c);
            if !above {
                return true;
            }
        }
    }
    false
}

/// Does the first register, on some path to the RETURN, hold an `int`
/// worked out from the high half of a 64-bit temporary whose low half the
/// second register holds: the high word of the low word's sum or difference
/// worked over ([`carried_into`]), or, with no carry between them, a literal
/// operation on a value the low word never reads ([`works_over`])? The two
/// registers are followed through moves and through choices made in one
/// block together, input by input, so a loop's 64-bit accumulator pairs its
/// sum with the sum's own high word. A high word whose path the low word's
/// choice does not follow (choices made in different blocks, as an
/// operator's cases in an expression evaluator) is judged against every
/// sum that choice can return: a sum whose own high word is among them is
/// a 64-bit value, and the low word read whole keeps a literal operation's
/// source in reach.
fn reworked(data: &Funcdata, high: VarnodeId, low: VarnodeId, pair: &Pair) -> bool {
    let unmove = |vn: VarnodeId| {
        let mut cur = vn;
        for _ in 0..8 {
            match moved(data, cur) {
                Some(x) => cur = x,
                None => break,
            }
        }
        cur
    };
    let phi = |vn: VarnodeId| {
        data.vbank()
            .get(vn)
            .and_then(|v| v.get_def())
            .and_then(|d| data.obank().get(d))
            .filter(|op| op.code() == OpCode::CPUI_MULTIEQUAL)
    };
    let mut seen: BTreeSet<(VarnodeId, VarnodeId)> = BTreeSet::new();
    let mut mixed: Vec<(VarnodeId, VarnodeId)> = Vec::new();
    let mut work = vec![(high, low)];
    while let Some((h, l)) = work.pop() {
        let (h, l) = (unmove(h), unmove(l));
        if !seen.insert((h, l)) || seen.len() > MAX_NODES {
            continue;
        }
        let inputs = |op: &crate::op::PcodeOp| (0..op.num_input()).filter_map(|i| op.get_in(i)).collect::<Vec<_>>();
        match (phi(h), phi(l)) {
            (Some(a), Some(b)) if a.get_parent() == b.get_parent() && a.num_input() == b.num_input() => {
                work.extend(inputs(a).into_iter().zip(inputs(b)));
            }
            (Some(a), _) => work.extend(inputs(a).into_iter().map(|x| (x, l))),
            (None, Some(_)) => mixed.push((h, l)),
            (None, None) => {
                let ends = ends_of(data, l, &pair.own, pair.lo_size);
                let mut carried = false;
                for &(v, off) in &ends.values {
                    let sum = unmove(v);
                    if off != 0 || !sum_or_difference(data, sum) {
                        continue;
                    }
                    match carried_into(data, sum, h) {
                        Some(true) => return true,
                        Some(false) => carried = true,
                        None => {}
                    }
                }
                let computed = ends.values.iter().any(|&(v, _)| literal_value(data, v, LITERAL_DEPTH).is_none());
                if !carried && computed && works_over(data, h, l) {
                    return true;
                }
            }
        }
    }
    let mut verdicts: std::collections::BTreeMap<VarnodeId, (bool, bool)> = std::collections::BTreeMap::new();
    for &(h, l) in &mixed {
        let mut carried = false;
        for sum in low_sums(data, l) {
            match carried_into(data, sum, h) {
                Some(true) => verdicts.entry(sum).or_default().0 = true,
                Some(false) => {
                    verdicts.entry(sum).or_default().1 = true;
                    carried = true;
                }
                None => {}
            }
        }
        let ends = ends_of(data, l, &pair.own, pair.lo_size);
        let computed = ends.values.iter().any(|&(v, _)| literal_value(data, v, LITERAL_DEPTH).is_none());
        if !carried && computed && works_over(data, h, l) {
            return true;
        }
    }
    verdicts.values().any(|&(reworks, clean)| reworks && !clean)
}

/// Does a carry or borrow out of the sum or difference `sum` reach the first
/// register `high`, and if so, other than as the high word of that sum
/// itself? `None` when no flag reaches it, or reaches it only through what
/// the walk cannot read ([`Off::Unknown`]). The high word of a 64-bit sum adds
/// the carry to the sum of the operands' high words (`adde`, `adc`, `addx`,
/// MIPS's `sltu` then `addu`) and a difference subtracts the borrow
/// (`subfe`, `sbc`, `subx`); a first register that goes on to shift, mask,
/// multiply or negate that word, or add a literal the low word does not,
/// holds an `int` worked out from the high half of the temporary:
/// `(u32)((x + y) >> 48)` on PowerPC is `addc 4,6,4; adde 3,5,3; srwi
/// 3,3,16`, which leaves the low sum in r4. Each value the first register is
/// a move or a choice of is judged on its own; one no flag reaches says
/// nothing.
fn carried_into(data: &Funcdata, sum: VarnodeId, high: VarnodeId) -> Option<bool> {
    let signs = carry_signs(data, sum);
    if signs.is_empty() {
        return None;
    }
    let tainted = descendants(data, signs.iter().map(|&(f, _)| f));
    let literal = data
        .vbank()
        .get(sum)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .is_some_and(|op| (0..op.num_input()).any(|i| op.get_in(i).and_then(|x| literal_value(data, x, LITERAL_DEPTH)).is_some()));
    let mut reached = None;
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![high];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > MAX_NODES {
            continue;
        }
        if let Some(x) = moved(data, cur) {
            work.push(x);
            continue;
        }
        let op = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
        if let Some(phi) = op.filter(|op| op.code() == OpCode::CPUI_MULTIEQUAL) {
            work.extend((0..phi.num_input()).filter_map(|i| phi.get_in(i)));
            continue;
        }
        if !tainted.contains(&cur) {
            continue;
        }
        let terms = Terms { data, signs: &signs, tainted: &tainted, literal };
        match terms.walk(cur, 1, 12, &mut BTreeSet::new()) {
            Err(Off::Reworked) => return Some(true),
            Ok(true) => reached = Some(false),
            _ => {}
        }
    }
    reached
}

/// Is the first register `high` a literal operation -- a shift, a mask, an
/// offset, a negation, a product, an `or` or `xor` with a literal -- on a
/// value `low` never reads, nor anything that value only moves bits out of
/// ([`sources`])? The halves of a 64-bit value computed together
/// share their sources (`(v & 0xff) << 32 | v >> 8` reads `v` in both, a
/// 64-bit `x >> 1` moves the high word's bit into the low word); an `int`
/// worked out from the high half of a temporary leaves the low half beside it
/// untouched by what it did: gcc -O0 on MIPS computes `(u32)((x ^ y) >> 32) &
/// 0xffff` in `$2:$3` and masks `$2` alone, ARM's `umull r1,r0` leaves the low
/// half of `(u32)((x * k) >> 32) & 0xff` in r1. The value the first register
/// is made from is followed through moves, choices and the reload of a stack
/// slot stored once (-O0 keeps a result in a local; a slot stored more than
/// once is an accumulator whose earlier values were not returned).
fn works_over(data: &Funcdata, high: VarnodeId, low: VarnodeId) -> bool {
    let def = |x: VarnodeId| data.vbank().get(x).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let sums = low_sums(data, low);
    let mut stored: Option<Vec<((u64, i64), VarnodeId)>> = None;
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![high];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > 64 {
            continue;
        }
        if let Some(x) = moved(data, cur) {
            work.push(x);
            continue;
        }
        let Some(op) = def(cur) else { continue };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => work.extend((0..op.num_input()).filter_map(|i| op.get_in(i))),
            OpCode::CPUI_LOAD => {
                if let Some(slot) = stack_load(data, cur) {
                    let stores = stored.get_or_insert_with(|| stack_stores(data));
                    let values: Vec<VarnodeId> = stores.iter().filter(|(at, _)| *at == slot).map(|&(_, value)| value).collect();
                    if let [value] = values[..] {
                        work.push(value);
                    }
                }
            }
            _ => {
                if sums.iter().any(|&sum| carried_into(data, sum, cur) == Some(false)) {
                    continue;
                }
                if let Some(worked) = worked_operand(data, op) {
                    if !parallel(data, op, low) && !sources(data, worked).into_iter().any(|x| reads(data, low, x)) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// The values `vn` is worked out from by literal operations alone: `vn`
/// itself and, through moves, literal operations ([`worked_operand`]),
/// rotates by literals (PowerPC's `srwi` is `rlwinm`, a rotate then a mask)
/// and stack reloads, what those read.
fn sources(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let def = |x: VarnodeId| data.vbank().get(x).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let literal = |op: &crate::op::PcodeOp, i: int4| op.get_in(i).and_then(|x| literal_value(data, x, LITERAL_DEPTH)).is_some();
    let shifted = |x: VarnodeId| {
        def(x)
            .filter(|op| {
                matches!(op.code(), OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT) && literal(op, 1)
            })
            .and_then(|op| op.get_in(0))
            .map(|x| moved_from(data, x))
    };
    let mut stored: Option<Vec<((u64, i64), VarnodeId)>> = None;
    let mut found: Vec<VarnodeId> = Vec::new();
    let mut work = vec![vn];
    while let Some(cur) = work.pop() {
        if found.contains(&cur) || found.len() > 64 {
            continue;
        }
        found.push(cur);
        if let Some(x) = moved(data, cur) {
            work.push(x);
            continue;
        }
        let Some(op) = def(cur) else { continue };
        if let Some(x) = worked_operand(data, op) {
            work.push(x);
            continue;
        }
        match op.code() {
            OpCode::CPUI_INT_SRIGHT if literal(op, 1) => work.extend(op.get_in(0)),
            OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD => {
                let (a, b) = (op.get_in(0).and_then(shifted), op.get_in(1).and_then(shifted));
                if a.is_some() && a == b {
                    work.extend(a);
                }
            }
            OpCode::CPUI_LOAD => {
                if let Some(slot) = stack_load(data, cur) {
                    let stores = stored.get_or_insert_with(|| stack_stores(data));
                    work.extend(stores.iter().filter(|(at, _)| *at == slot).map(|&(_, value)| value));
                }
            }
            _ => {}
        }
    }
    found
}

/// Is `low`, through moves, the same bitwise operation `op` applies to the
/// first register -- a complement, or an `and`, `or` or `xor` with a literal
/// -- the two halves of one 64-bit `~x` or `x & K` get alike?
fn parallel(data: &Funcdata, op: &crate::op::PcodeOp, low: VarnodeId) -> bool {
    let mut cur = low;
    for _ in 0..8 {
        match moved(data, cur) {
            Some(x) => cur = x,
            None => break,
        }
    }
    let Some(other) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return false;
    };
    let literal = |o: &crate::op::PcodeOp| (0..o.num_input()).any(|i| o.get_in(i).and_then(|x| literal_value(data, x, LITERAL_DEPTH)).is_some());
    other.code() == op.code()
        && match op.code() {
            OpCode::CPUI_INT_NEGATE => true,
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => literal(other),
            _ => false,
        }
}

/// The sums and differences `low` holds, through moves, choices and stack
/// reloads: at -O0 a `long long`'s two words reach the RETURN from two
/// locals stored on every path (`i64 tabs64(i64 a)` stores `a` and `-a`),
/// and the high word stored beside a negation is that negation's own.
fn low_sums(data: &Funcdata, low: VarnodeId) -> Vec<VarnodeId> {
    let mut stored: Option<Vec<((u64, i64), VarnodeId)>> = None;
    let mut sums = Vec::new();
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![low];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > 64 {
            continue;
        }
        if let Some(x) = moved(data, cur) {
            work.push(x);
            continue;
        }
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { continue };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => work.extend((0..op.num_input()).filter_map(|i| op.get_in(i))),
            OpCode::CPUI_LOAD => {
                if let Some(slot) = stack_load(data, cur) {
                    let stores = stored.get_or_insert_with(|| stack_stores(data));
                    work.extend(stores.iter().filter(|(at, _)| *at == slot).map(|&(_, value)| value));
                }
            }
            _ if sum_or_difference(data, cur) => sums.push(cur),
            _ => {}
        }
    }
    sums
}

/// The value `op` applies a literal operation to: a shift by a literal
/// (short of a sign fill), a mask, an offset, a product, an `or` or `xor`
/// with a literal, a subtraction from or of one, or a negation.
fn worked_operand(data: &Funcdata, op: &crate::op::PcodeOp) -> Option<VarnodeId> {
    let literal = |i: int4| op.get_in(i).and_then(|x| literal_value(data, x, LITERAL_DEPTH));
    let size = op.get_out().and_then(|x| data.vbank().get(x)).map_or(4, |x| x.get_size());
    let bits = 8 * size as u64;
    let other = |k: Option<u64>| match (literal(0), literal(1)) {
        (None, Some(c)) if k.is_none_or(|k| c != k) => op.get_in(0),
        (Some(c), None) if k.is_none_or(|k| c != k) => op.get_in(1),
        _ => None,
    };
    match op.code() {
        OpCode::CPUI_INT_AND => other(Some(ones(size))),
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD => other(Some(0)),
        OpCode::CPUI_INT_SUB => match (literal(0), literal(1)) {
            (Some(_), None) => op.get_in(1),
            (None, Some(c)) if c != 0 => op.get_in(0),
            _ => None,
        },
        OpCode::CPUI_INT_MULT => other(Some(1)),
        OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT => {
            literal(1).filter(|&c| c > 0 && c < bits).and_then(|_| op.get_in(0)).filter(|_| literal(0).is_none())
        }
        OpCode::CPUI_INT_SRIGHT => {
            literal(1).filter(|&c| c > 0 && c + 1 < bits).and_then(|_| op.get_in(0)).filter(|_| literal(0).is_none())
        }
        OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_NEGATE => op.get_in(0).filter(|_| literal(0).is_none()),
        _ => None,
    }
}

/// Is `target` among the values `from` is computed from, following stack
/// reloads to what was stored there? `false` past [`MAX_NODES`].
fn reads(data: &Funcdata, from: VarnodeId, target: VarnodeId) -> bool {
    let goal = moved_from(data, target);
    let mut stored: Option<Vec<((u64, i64), VarnodeId)>> = None;
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![from];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_NODES {
            return false;
        }
        if cur == target || moved_from(data, cur) == goal {
            return true;
        }
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { continue };
        let inputs = match op.code() {
            OpCode::CPUI_LOAD => {
                if let Some(slot) = stack_load(data, cur) {
                    let stores = stored.get_or_insert_with(|| stack_stores(data));
                    work.extend(stores.iter().filter(|(at, _)| *at == slot).map(|&(_, value)| value));
                }
                continue;
            }
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => 0,
            OpCode::CPUI_INDIRECT => 1,
            _ => op.num_input(),
        };
        work.extend((0..inputs).filter_map(|i| op.get_in(i)));
    }
    false
}

/// What `vn` is a move of: the input of a copy, an indirect, an `or`, `xor`
/// or addition with zero (MIPS's `move` is `or $3,$5,$zero`), or an `or` or
/// `and` of a value with itself (PowerPC's `mr 3,5` is `or 3,5,5`).
fn moved(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    let op = data.obank().get(data.vbank().get(vn)?.get_def()?)?;
    let zero = |i: int4| op.get_in(i).is_some_and(|x| data.vbank().get(x).is_some_and(|x| x.is_constant() && x.get_offset() == 0));
    match op.code() {
        OpCode::CPUI_COPY => op.get_in(0),
        OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => op.get_in(0),
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD if zero(1) => op.get_in(0),
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD if zero(0) => op.get_in(1),
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND if op.get_in(0) == op.get_in(1) => op.get_in(0),
        _ => None,
    }
}

/// The terms of the high word of a sum or difference whose flags are
/// `signs` ([`carry_signs`]); `tainted` holds every value a flag reaches.
struct Terms<'a> {
    data: &'a Funcdata,
    signs: &'a [(VarnodeId, i8)],
    tainted: &'a BTreeSet<VarnodeId>,
    /// The low word adds a literal, so the high word may add that 64-bit
    /// literal's high half (PowerPC's `addme` adds all ones for `x - 1`, and
    /// SQLite adds `0x7fffffff` to the high word of `x + LARGEST_INT64`).
    literal: bool,
}

/// Why a value a flag reaches is not a sum's own high word ([`Terms::walk`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Off {
    /// The flag reaches it through an operation the high word of a 64-bit
    /// sum does not apply -- a shift, a mask, a product, a literal added, a
    /// flag of the wrong sign.
    Reworked,
    /// The flag reaches it through something the walk does not read: a piece
    /// of a wider value, a flag whose meaning is not known, a walk too deep.
    Unknown,
}

impl Terms<'_> {
    /// Is `vn`, taken with `sign`, that high word: additions and subtractions
    /// (through moves, truncations and choices) of values no flag reaches and
    /// of no literal but zero (any literal where the low word adds one too,
    /// as long as it is not added on top of a word that already holds the
    /// carry: `adde` of a register holding 7 is `x + 0x700000005`, `addze`
    /// then `addi 3,3,7` is `(u32)((x + 5) >> 32) + 7`; a loop's accumulator
    /// carries an earlier carry and may add its literal, as coreutils dd's
    /// 64-bit decrement does), with at least one flag entering with its sign?
    /// (SPARC's `smul` truncates a 64-bit product, so `* 7` of the high word
    /// is a product the walk reads.)
    /// `Ok(false)` for a term no flag reaches, `Err` for a flag reaching it
    /// any other way ([`Off`]). A choice met again on the way round a loop (a
    /// 64-bit accumulator) adds nothing.
    fn walk(&self, vn: VarnodeId, sign: i8, depth: u32, open: &mut BTreeSet<(VarnodeId, i8)>) -> Result<bool, Off> {
        let data = self.data;
        let def = |x: VarnodeId| data.vbank().get(x).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
        let (mut cur, mut s) = (vn, sign);
        for _ in 0..8 {
            if let Some(&(_, want)) = self.signs.iter().find(|&&(f, _)| f == cur) {
                return match want {
                    0 => Err(Off::Unknown),
                    w if w == s => Ok(true),
                    _ => Err(Off::Reworked),
                };
            }
            if let Some(x) = moved(data, cur) {
                cur = x;
                continue;
            }
            let at_zero = |op: &crate::op::PcodeOp| {
                op.get_in(1).and_then(|k| data.vbank().get(k)).is_some_and(|k| k.is_constant() && k.get_offset() == 0)
            };
            match def(cur).map(|op| (op.code(), op)) {
                Some((OpCode::CPUI_INT_ZEXT, op)) => cur = op.get_in(0).ok_or(Off::Unknown)?,
                Some((OpCode::CPUI_SUBPIECE, op)) if at_zero(op) => cur = op.get_in(0).ok_or(Off::Unknown)?,
                Some((OpCode::CPUI_BOOL_NEGATE, op)) => {
                    cur = op.get_in(0).ok_or(Off::Unknown)?;
                    s = -s;
                }
                _ => break,
            }
        }
        if !self.tainted.contains(&cur) {
            return match literal_value(data, cur, LITERAL_DEPTH) {
                None | Some(0) => Ok(false),
                Some(_) if self.literal => Ok(false),
                Some(_) => Err(Off::Reworked),
            };
        }
        let op = def(cur).filter(|_| depth > 0).ok_or(Off::Unknown)?;
        if op.code() == OpCode::CPUI_MULTIEQUAL && !open.insert((cur, s)) {
            return Ok(false);
        }
        let mut term = |i: int4, s: i8| match op.get_in(i) {
            Some(x) => self.walk(x, s, depth - 1, open),
            None => Err(Off::Unknown),
        };
        let both = |results: Vec<Result<bool, Off>>| {
            if results.contains(&Err(Off::Reworked)) {
                Err(Off::Reworked)
            } else if results.contains(&Err(Off::Unknown)) {
                Err(Off::Unknown)
            } else {
                Ok(results.contains(&Ok(true)))
            }
        };
        let flag_itself = |i: int4| {
            let mut x = op.get_in(i);
            for _ in 0..8 {
                let Some(v) = x else { return false };
                if self.signs.iter().any(|&(f, _)| f == v) {
                    return true;
                }
                x = moved(data, v).or_else(|| {
                    def(v).filter(|d| matches!(d.code(), OpCode::CPUI_INT_ZEXT | OpCode::CPUI_BOOL_NEGATE)).and_then(|d| d.get_in(0))
                });
            }
            false
        };
        let nonzero = |i: int4| op.get_in(i).and_then(|x| literal_value(data, x, LITERAL_DEPTH)).is_some_and(|k| k != 0);
        let looped = |i: int4| {
            let mut x = op.get_in(i);
            for _ in 0..8 {
                let Some(v) = x else { return false };
                if def(v).is_some_and(|d| d.code() == OpCode::CPUI_MULTIEQUAL) {
                    return true;
                }
                x = moved(data, v);
            }
            false
        };
        let on_top = |carried: Result<bool, Off>, at: int4, other: int4| {
            carried == Ok(true) && nonzero(other) && !flag_itself(at) && !looped(at)
        };
        match op.code() {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let down = if op.code() == OpCode::CPUI_INT_SUB { -s } else { s };
                let (a, b) = (term(0, s), term(1, down));
                if on_top(a, 0, 1) || on_top(b, 1, 0) {
                    Err(Off::Reworked)
                } else {
                    both(vec![a, b])
                }
            }
            OpCode::CPUI_INT_2COMP => term(0, -s),
            OpCode::CPUI_MULTIEQUAL => both((0..op.num_input()).map(|i| term(i, s)).collect()),
            OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM => Err(Off::Reworked),
            _ => Err(Off::Unknown),
        }
    }
}

/// The carries and borrows out of `sum`, each with the sign it enters the
/// high word of the 64-bit value with: `1` for a carry or a "no borrow"
/// (`subfc`'s and ARM's `subs` carry, `b <= a`), `-1` for a borrow (`a <
/// b`, MIPS's `a < a - b`, or `b != 0` for `0 - b`) or a "no carry", `0` for a flag whose
/// meaning is not one of those. A flag is a comparison of nothing but the
/// sum's operands and itself, read through copies and the pieces of a
/// register pair heritage splits and joins (SPARC -O0 reloads a `long long`
/// with `ldd` and adds its halves out of `%i2:%i3`).
fn carry_signs(data: &Funcdata, sum: VarnodeId) -> Vec<(VarnodeId, i8)> {
    let def = |x: VarnodeId| data.vbank().get(x).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let Some(op) = def(sum) else { return Vec::new() };
    let (a, b) = match op.code() {
        OpCode::CPUI_INT_2COMP => (None, op.get_in(0)),
        _ => (op.get_in(0), op.get_in(1)),
    };
    let key = |x: Option<VarnodeId>| x.and_then(|x| value_key(data, x));
    let (ka, kb, ks) = (key(a), key(b), key(Some(sum)));
    let same = |x: VarnodeId, k: &Option<Value>| k.is_some() && value_key(data, x) == *k;
    let zero = |k: &Option<Value>| matches!(k, Some(Value::Literal(0, _)));
    let operand = |x: VarnodeId| same(x, &ka) || same(x, &kb);
    let negation = op.code() == OpCode::CPUI_INT_2COMP || (op.code() == OpCode::CPUI_INT_SUB && zero(&ka));
    let mut flags: Vec<VarnodeId> = Vec::new();
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work: Vec<VarnodeId> = [a, b, Some(sum)]
        .into_iter()
        .flatten()
        .chain([&ka, &kb, &ks].into_iter().filter_map(|k| match k {
            Some(Value::At(base, _, _)) => Some(*base),
            _ => None,
        }))
        .collect();
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > 256 {
            continue;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        if v.is_constant() {
            continue;
        }
        for d in v.descend_iter() {
            let Some(next) = data.obank().get(d).filter(|o| !o.is_dead()) else { continue };
            let Some(out) = next.get_out() else { continue };
            match next.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE | OpCode::CPUI_PIECE | OpCode::CPUI_INT_ZEXT => work.push(out),
                OpCode::CPUI_INDIRECT if next.get_in(0) == Some(cur) => work.push(out),
                OpCode::CPUI_INT_CARRY
                | OpCode::CPUI_INT_SCARRY
                | OpCode::CPUI_INT_SBORROW
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL => {
                    let mine = (0..next.num_input())
                        .all(|i| next.get_in(i).is_some_and(|x| operand(x) || same(x, &ks)));
                    if mine && !flags.contains(&out) {
                        flags.push(out);
                    }
                }
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL if negation => {
                    let (x, y) = (next.get_in(0), next.get_in(1));
                    let tests = |v: Option<VarnodeId>| v.is_some_and(|v| same(v, &kb) || same(v, &ks));
                    let nil = |v: Option<VarnodeId>| v.and_then(|v| value_key(data, v)).is_some_and(|k| matches!(k, Value::Literal(0, _)));
                    if ((tests(x) && nil(y)) || (nil(x) && tests(y))) && !flags.contains(&out) {
                        flags.push(out);
                    }
                }
                _ => {}
            }
        }
    }
    flags
        .into_iter()
        .map(|f| {
            let Some(flag) = def(f) else { return (f, 0) };
            let (Some(x), Some(y)) = (flag.get_in(0), flag.get_in(1)) else { return (f, 0) };
            let sign = match (op.code(), flag.code()) {
                (OpCode::CPUI_INT_ADD, OpCode::CPUI_INT_CARRY) if operand(x) && operand(y) => 1,
                (OpCode::CPUI_INT_ADD, OpCode::CPUI_INT_LESS) if same(x, &ks) && operand(y) => 1,
                (OpCode::CPUI_INT_ADD, OpCode::CPUI_INT_LESSEQUAL) if operand(x) && same(y, &ks) => -1,
                (OpCode::CPUI_INT_SUB, OpCode::CPUI_INT_LESS) if same(x, &ka) && (same(y, &kb) || same(y, &ks)) => -1,
                (OpCode::CPUI_INT_SUB, OpCode::CPUI_INT_LESSEQUAL)
                    if (same(x, &kb) && same(y, &ka)) || (same(x, &ks) && same(y, &ka)) =>
                {
                    1
                }
                (OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_2COMP, OpCode::CPUI_INT_LESS)
                    if (zero(&ka) || a.is_none()) && matches!(value_key(data, x), Some(Value::Literal(0, _)))
                        && (same(y, &kb) || same(y, &ks)) =>
                {
                    -1
                }
                (_, OpCode::CPUI_INT_NOTEQUAL) if negation => -1,
                (_, OpCode::CPUI_INT_EQUAL) if negation => 1,
                _ => 0,
            };
            (f, sign)
        })
        .collect()
}

/// What a Varnode holds, for telling two apart as values ([`value_key`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    /// `size` bytes `offset` bytes above the least significant byte of a
    /// Varnode.
    At(VarnodeId, int4, int4),
    /// A `size`-byte literal.
    Literal(u64, int4),
}

/// The value `vn` holds, read through copies, truncations, zero extensions
/// and the pieces of a register pair heritage splits and joins, down to the
/// Varnode or literal it was made as.
fn value_key(data: &Funcdata, vn: VarnodeId) -> Option<Value> {
    let size = data.vbank().get(vn)?.get_size();
    let (mut cur, mut off) = (vn, 0);
    for _ in 0..16 {
        let v = data.vbank().get(cur)?;
        if v.is_constant() {
            let k = v.get_offset().checked_shr(8 * off as u32).unwrap_or(0) & ones(size);
            return Some(Value::Literal(k, size));
        }
        let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { break };
        let in_size = |i: int4| op.get_in(i).and_then(|x| data.vbank().get(x)).map_or(0, |x| x.get_size());
        match op.code() {
            OpCode::CPUI_COPY => cur = op.get_in(0)?,
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => cur = op.get_in(0)?,
            OpCode::CPUI_SUBPIECE => {
                let k = op.get_in(1).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant())?.get_offset();
                off += k as int4;
                cur = op.get_in(0)?;
            }
            OpCode::CPUI_PIECE => {
                let lo = in_size(1);
                if off + size <= lo {
                    cur = op.get_in(1)?;
                } else if off >= lo {
                    off -= lo;
                    cur = op.get_in(0)?;
                } else {
                    break;
                }
            }
            OpCode::CPUI_INT_ZEXT if off + size <= in_size(0) => cur = op.get_in(0)?,
            _ => break,
        }
    }
    Some(Value::At(cur, off, size))
}

/// Every Varnode computed from one of `roots`, as far as [`MAX_NODES`] go.
fn descendants(data: &Funcdata, roots: impl Iterator<Item = VarnodeId>) -> BTreeSet<VarnodeId> {
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work: Vec<VarnodeId> = roots.collect();
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > MAX_NODES {
            continue;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        for d in v.descend_iter() {
            if let Some(out) = data.obank().get(d).filter(|op| !op.is_dead()).and_then(|op| op.get_out()) {
                work.push(out);
            }
        }
    }
    seen
}

/// The product `end` (a returned value's `off`-th byte, [`ends_of`]) is the
/// high half of, through a stack reload of it: an INT_MULT at least twice
/// `lo_size` wide read at `lo_size` bytes or above (MIPS's `mfhi`, spilled
/// and reloaded at -O0).
fn product_high_half(data: &Funcdata, end: VarnodeId, off: int4, lo_size: int4) -> Option<VarnodeId> {
    let product = |vn: VarnodeId, k: u64| {
        let v = data.vbank().get(vn)?;
        let op = data.obank().get(v.get_def()?)?;
        (op.code() == OpCode::CPUI_INT_MULT && v.get_size() >= 2 * lo_size && k >= lo_size as u64).then_some(vn)
    };
    if let Some(p) = product(end, off.max(0) as u64) {
        return Some(p);
    }
    let slot = stack_load(data, end).filter(|_| off == 0)?;
    let mut found: Option<VarnodeId> = None;
    for (at, value) in stack_stores(data) {
        if at != slot {
            continue;
        }
        let (whole, k) = piece_of(data, value)?;
        let p = product(whole, k)?;
        if found.is_some_and(|q| q != p) {
            return None;
        }
        found = Some(p);
    }
    found
}

/// Does the first register `high` read the low half of `product`: does a
/// value it is computed from, following stack reloads to what was stored,
/// take `product` other than as its high half?
fn reads_low_half(data: &Funcdata, high: VarnodeId, product: VarnodeId, lo_size: int4) -> bool {
    let bits = 8 * lo_size as u64;
    let mut stored: Option<Vec<((u64, i64), VarnodeId)>> = None;
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![high];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) || seen.len() > MAX_NODES {
            continue;
        }
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { continue };
        if op.code() == OpCode::CPUI_LOAD {
            if let Some(slot) = stack_load(data, cur) {
                let stores = stored.get_or_insert_with(|| stack_stores(data));
                work.extend(stores.iter().filter(|(at, _)| *at == slot).map(|&(_, value)| value));
            }
            continue;
        }
        let amount = op.get_in(1).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant()).map(|x| x.get_offset());
        let inputs = match op.code() {
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => 0,
            OpCode::CPUI_INDIRECT => 1,
            _ => op.num_input(),
        };
        for i in 0..inputs {
            let Some(x) = op.get_in(i) else { continue };
            if moved_from(data, x) != product {
                work.push(x);
                continue;
            }
            let high_half = i == 0
                && match op.code() {
                    OpCode::CPUI_SUBPIECE => amount.is_some_and(|k| k >= lo_size as u64),
                    OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => amount.is_some_and(|c| c >= bits),
                    _ => false,
                };
            if !high_half {
                return true;
            }
        }
    }
    false
}

/// Every live STORE to a stack address ([`stack_offset`]): its space index
/// and offset, and the value it stores.
fn stack_stores(data: &Funcdata) -> Vec<((u64, i64), VarnodeId)> {
    data.obank()
        .iter_code(OpCode::CPUI_STORE)
        .filter_map(|st| {
            let op = data.obank().get(st).filter(|op| !op.is_dead())?;
            let space = data.vbank().get(op.get_in(0)?)?.get_offset();
            Some(((space, stack_offset(data, op.get_in(1)?)?), op.get_in(2)?))
        })
        .collect()
}

/// What `vn` is a copy of, for telling two Varnodes apart as values: the
/// Varnode before any moves, or the whole and offset of a truncation of one.
fn same_value_key(data: &Funcdata, vn: VarnodeId) -> (VarnodeId, Option<u64>) {
    let at = moved_from(data, vn);
    let piece = data.vbank().get(at).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)).and_then(|op| {
        (op.code() == OpCode::CPUI_SUBPIECE)
            .then(|| Some((moved_from(data, op.get_in(0)?), data.vbank().get(op.get_in(1)?)?.get_offset())))
            .flatten()
    });
    match piece {
        Some((whole, k)) => (whole, Some(k)),
        None => (at, None),
    }
}

/// Is `product` a multiply at least twice the low word wide whose high half
/// reaches the first register `high` through a right shift by a literal on
/// the way: a division by a constant, which leaves the product's low half in
/// the second register (ARM's `umull r1,r0,r2,r3; lsr r0,r0,#3` for `a / 10`,
/// `smull r1,r0,r2,r3; add r0,r0,r0,lsr #31` for `a / 3`, and the remainder
/// built from such a quotient)? A `long long` product returns its high half
/// as it is or with more terms added, never shifted down.
fn divided_down(data: &Funcdata, product: VarnodeId, high: VarnodeId, lo_size: int4) -> bool {
    let wide_product = data.vbank().get(product).is_some_and(|v| {
        v.get_size() >= 2 * lo_size
            && v.get_def().and_then(|d| data.obank().get(d)).is_some_and(|op| op.code() == OpCode::CPUI_INT_MULT)
    });
    if !wide_product {
        return false;
    }
    let bits = 8 * lo_size as u64;
    let mut seen: BTreeSet<(VarnodeId, bool)> = BTreeSet::new();
    let mut work = vec![(high, false, 0u32)];
    while let Some((cur, shifted, depth)) = work.pop() {
        if depth > 16 || !seen.insert((cur, shifted)) || seen.len() > MAX_NODES {
            continue;
        }
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            continue;
        };
        let amount = op.get_in(1).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant()).map(|x| x.get_offset());
        let inputs = match op.code() {
            OpCode::CPUI_SUBPIECE if op.get_in(0) == Some(product) => {
                if shifted && amount.is_some_and(|k| 8 * k >= bits) {
                    return true;
                }
                continue;
            }
            OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT if op.get_in(0) == Some(product) => {
                if amount.is_some_and(|c| c >= bits && (shifted || c > bits)) {
                    return true;
                }
                continue;
            }
            OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                if amount.is_some_and(|c| c > 0) {
                    work.extend(op.get_in(0).map(|x| (x, true, depth + 1)));
                }
                continue;
            }
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => 0,
            OpCode::CPUI_INDIRECT | OpCode::CPUI_INT_LEFT | OpCode::CPUI_SUBPIECE => 1,
            OpCode::CPUI_COPY
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR => op.num_input(),
            _ => 0,
        };
        work.extend((0..inputs).filter_map(|i| op.get_in(i)).map(|x| (x, shifted, depth + 1)));
    }
    false
}

/// Are `high` and `low` loaded from adjacent `lo_size`-byte words of one
/// stack object, `high` from its less significant word? A `long long` comes
/// back from a stack temporary with its more significant word in the first
/// register, so this is an `int` truncation of the temporary. Two words that
/// are each where a register's own argument was stored and is reloaded from
/// (clang -O0 `((u64)a << 32) | b` reloads `a` and `b` from their homes, `b`
/// below `a`) are two values, not one object.
fn loads_low_word_first(data: &Funcdata, high: VarnodeId, low: VarnodeId, pair: &Pair) -> bool {
    let (Some((space, h)), Some((space2, l))) = (stack_load(data, high), stack_load(data, low)) else {
        return false;
    };
    let manage = data.get_arch().manage();
    let big_endian = space < manage.num_spaces() as u64 && manage.get_space(space as i32).is_some_and(|s| s.is_big_endian());
    let size = pair.lo_size as i64;
    let reversed = space == space2 && if big_endian { h == l + size } else { h + size == l };
    reversed && !(argument_home(data, space, h, &pair.hi_own) && argument_home(data, space, l, &pair.own))
}

/// Is the stack word at `offset` written only with the entry value of the
/// register at `reg` (through copies and a register window's moves), and at
/// least once?
fn argument_home(data: &Funcdata, space: u64, offset: i64, reg: &Address) -> bool {
    let mut stores = 0;
    for st in data.obank().iter_code(OpCode::CPUI_STORE) {
        let Some(op) = data.obank().get(st) else { continue };
        if op.is_dead() {
            continue;
        }
        let (Some(sp), Some(at), Some(value)) = (op.get_in(0), op.get_in(1), op.get_in(2)) else { return false };
        if data.vbank().get(sp).map(|v| v.get_offset()) != Some(space) || stack_offset(data, at) != Some(offset) {
            continue;
        }
        if !entry_value_of(data, value, reg) {
            return false;
        }
        stores += 1;
    }
    stores > 0
}

/// Is `vn`, through copies and choices of nothing else, the entry value of
/// the register at `reg`?
fn entry_value_of(data: &Funcdata, vn: VarnodeId, reg: &Address) -> bool {
    entry_value_within(data, vn, reg, 4)
}

/// [`entry_value_of`], following at most `choices` nested choices (on
/// 511bd082d a SPARC -O0 argument reaches its home through a choice of its
/// entry value and the indirect effects `save` puts on it).
fn entry_value_within(data: &Funcdata, vn: VarnodeId, reg: &Address, choices: u32) -> bool {
    let mut cur = vn;
    for _ in 0..64 {
        let Some(v) = data.vbank().get(cur) else { return false };
        let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else {
            return v.is_input() && v.get_addr() == reg;
        };
        match (op.code(), op.get_in(0)) {
            (OpCode::CPUI_COPY, Some(x)) => cur = x,
            (OpCode::CPUI_INDIRECT, Some(x)) if !op.is_indirect_creation() => cur = x,
            (OpCode::CPUI_MULTIEQUAL, _) if choices > 0 => {
                return (0..op.num_input())
                    .filter_map(|i| op.get_in(i))
                    .filter(|&x| x != cur)
                    .all(|x| entry_value_within(data, x, reg, choices - 1));
            }
            _ => return false,
        }
    }
    false
}

/// The space index and stack offset `vn` is loaded from, through copies: a
/// LOAD from [`stack_offset`].
fn stack_load(data: &Funcdata, vn: VarnodeId) -> Option<(u64, i64)> {
    let mut cur = vn;
    for _ in 0..64 {
        let op = data.obank().get(data.vbank().get(cur)?.get_def()?)?;
        match op.code() {
            OpCode::CPUI_COPY => cur = op.get_in(0)?,
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => cur = op.get_in(0)?,
            OpCode::CPUI_LOAD => {
                let space = data.vbank().get(op.get_in(0)?)?.get_offset();
                return Some((space, stack_offset(data, op.get_in(1)?)?));
            }
            _ => return None,
        }
    }
    None
}

/// The offset from the function's entry stack pointer of the address `vn`,
/// when it is that pointer plus literals, possibly through a frame pointer
/// (PowerPC's `mr 31,1` is `or 31,1,1`) or a choice whose inputs all hold
/// the same address, through the indirect effects a SPARC `save` puts on
/// `%sp` (one per register it spills).
fn stack_offset(data: &Funcdata, vn: VarnodeId) -> Option<i64> {
    stack_offset_within(data, vn, 4)
}

/// [`stack_offset`], following at most `choices` nested choices.
fn stack_offset_within(data: &Funcdata, vn: VarnodeId, choices: u32) -> Option<i64> {
    let mut at = vn;
    let mut offset: i64 = 0;
    for _ in 0..64 {
        let v = data.vbank().get(at)?;
        let Some(def) = v.get_def() else {
            let sp = data.get_arch().manage().get_stack_space()?.get_spacebase(0).ok()?;
            let a = v.get_addr();
            let is_sp = v.is_input()
                && a.get_space().zip(sp.space.as_ref()).is_some_and(|(x, y)| Rc::ptr_eq(x, y))
                && a.get_offset() == sp.offset
                && v.get_size() as u32 == sp.size;
            return is_sp.then_some(offset);
        };
        let op = data.obank().get(def)?;
        let literal = |i: int4| {
            op.get_in(i).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant()).map(|x| signed(x.get_offset(), x.get_size()))
        };
        match op.code() {
            OpCode::CPUI_COPY => at = op.get_in(0)?,
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => at = op.get_in(0)?,
            OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND if op.get_in(0) == op.get_in(1) => at = op.get_in(0)?,
            OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR if literal(1) == Some(0) => at = op.get_in(0)?,
            OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR if literal(0) == Some(0) => at = op.get_in(1)?,
            OpCode::CPUI_INT_ADD => match (literal(0), literal(1)) {
                (None, Some(k)) => {
                    offset = offset.wrapping_add(k);
                    at = op.get_in(0)?;
                }
                (Some(k), None) => {
                    offset = offset.wrapping_add(k);
                    at = op.get_in(1)?;
                }
                _ => return None,
            },
            OpCode::CPUI_INT_SUB => {
                offset = offset.wrapping_sub(literal(1)?);
                at = op.get_in(0)?;
            }
            OpCode::CPUI_MULTIEQUAL if choices > 0 => {
                let mut agreed: Option<i64> = None;
                for i in 0..op.num_input() {
                    let x = op.get_in(i)?;
                    if x == at {
                        continue;
                    }
                    let o = stack_offset_within(data, x, choices - 1)?;
                    if agreed.is_some_and(|a| a != o) {
                        return None;
                    }
                    agreed = Some(o);
                }
                return agreed.map(|a| offset.wrapping_add(a));
            }
            _ => return None,
        }
    }
    None
}

/// `value`, a `size`-byte literal, sign-extended.
fn signed(value: u64, size: int4) -> i64 {
    let bits = 8 * size.clamp(1, 8) as u32;
    ((value << (64 - bits)) as i64) >> (64 - bits)
}

/// The size of the target's `int` (4 when the type factory is not built yet).
fn int_size(data: &Funcdata) -> int4 {
    data.get_arch().types().map(|t| t.get_size_of_int()).filter(|&s| s > 0).unwrap_or(4)
}

/// Is `vn` 0 or 1 by construction -- a comparison, a carry or borrow, a sign
/// bit, a leading-zero count shifted down to its top bit (PowerPC's `cntlzw;
/// srwi 5` for `== 0`), or 0s and 1s chosen between -- rather than one
/// literal? A value that
/// is always 0 (the sign of a small positive `int`, a zero extension) is the
/// high word of a zero- or sign-extended value, not a flag.
fn computed_flag(data: &Funcdata, vn: VarnodeId) -> bool {
    nonzero_bits(data, vn, 12, &mut Vec::new()) == 1 && literal_value(data, vn, LITERAL_DEPTH).is_none()
}

/// The bits of `vn` that can be nonzero, as far as `depth` operations show
/// (C++ `Varnode::nzm`, worked out here because return recovery runs before
/// the masks settle); every bit for anything it cannot follow.
fn nonzero_bits(data: &Funcdata, vn: VarnodeId, depth: u32, path: &mut Vec<VarnodeId>) -> u64 {
    let Some(v) = data.vbank().get(vn) else { return u64::MAX };
    let all = ones(v.get_size());
    if v.is_constant() {
        return v.get_offset() & all;
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { return all };
    if depth == 0 || path.contains(&vn) {
        return all;
    }
    path.push(vn);
    let mut arg = |i: int4| op.get_in(i).map_or(u64::MAX, |x| nonzero_bits(data, x, depth - 1, path));
    let shift = |i: int4| op.get_in(i).and_then(|x| literal_value(data, x, LITERAL_DEPTH)).and_then(|c| u32::try_from(c).ok());
    let size_of = |i: int4| op.get_in(i).and_then(|x| data.vbank().get(x)).map_or(8, |x| x.get_size());
    let r = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => arg(0),
        OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => arg(0),
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).fold(0, |m, i| m | arg(i)),
        OpCode::CPUI_INT_SEXT => {
            let m = arg(0);
            if m >> (8 * size_of(0) - 1) & 1 == 1 {
                all
            } else {
                m
            }
        }
        OpCode::CPUI_INT_AND | OpCode::CPUI_BOOL_AND => arg(0) & arg(1),
        OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR => arg(0) | arg(1),
        OpCode::CPUI_INT_ADD => {
            let (a, b) = (arg(0), arg(1));
            if a & b == 0 {
                a | b
            } else {
                all
            }
        }
        OpCode::CPUI_INT_MULT => {
            if arg(0) == 0 || arg(1) == 0 {
                0
            } else {
                all
            }
        }
        OpCode::CPUI_INT_LEFT => match shift(1) {
            Some(c) => arg(0).checked_shl(c).unwrap_or(0),
            None => all,
        },
        OpCode::CPUI_INT_RIGHT => match shift(1) {
            Some(c) => arg(0).checked_shr(c).unwrap_or(0),
            None => all,
        },
        OpCode::CPUI_INT_SRIGHT => match shift(1) {
            Some(c) => {
                let m = arg(0);
                if m >> (8 * v.get_size() - 1) & 1 == 1 {
                    all
                } else {
                    m.checked_shr(c).unwrap_or(0)
                }
            }
            None => all,
        },
        OpCode::CPUI_SUBPIECE => match shift(1) {
            Some(k) => k.checked_mul(8).and_then(|b| arg(0).checked_shr(b)).unwrap_or(0),
            None => all,
        },
        OpCode::CPUI_PIECE => {
            let lo_bits = 8 * size_of(1) as u32;
            arg(0).checked_shl(lo_bits).unwrap_or(0) | arg(1)
        }
        OpCode::CPUI_LZCOUNT | OpCode::CPUI_POPCOUNT => (8 * size_of(0) as u64 + 1).next_power_of_two() - 1,
        code if is_flag_op(code) => 1,
        _ => all,
    };
    path.pop();
    r & all
}

/// Does `code` produce a comparison's result, a carry or a borrow?
fn is_flag_op(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN
    )
}

/// Is `vn` a sum or a difference of something other than literals, and not
/// a move spelled as adding or subtracting zero (SPARC's `restore
/// %i0,%g0,%o1`)? `0 - x` is a negation, not a move.
fn sum_or_difference(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)).is_some_and(|op| {
        let zero = |i: int4| op.get_in(i).is_some_and(|x| literal_value(data, x, LITERAL_DEPTH) == Some(0));
        let moved = match op.code() {
            OpCode::CPUI_INT_ADD => zero(0) || zero(1),
            OpCode::CPUI_INT_SUB => zero(1),
            OpCode::CPUI_INT_2COMP => false,
            _ => return false,
        };
        !moved
            && (0..op.num_input())
                .filter_map(|i| op.get_in(i))
                .any(|x| data.vbank().get(x).is_some_and(|x| !x.is_constant()))
    })
}

/// Is the first register `high` built with no carry or borrow out of the low
/// word: is it more than one literal, while nothing a comparison, a carry or a
/// borrow produced -- and none of the low word's own values -- reaches it?
/// The halves of a sum or difference are tied by its carry; a first register
/// they are not tied by holds something else, and the sum was computed for its
/// flags.
fn carry_free(data: &Funcdata, high: VarnodeId, low: VarnodeId, ends: &Ends) -> bool {
    if literal_value(data, high, LITERAL_DEPTH).is_some() {
        return false;
    }
    let lows: BTreeSet<VarnodeId> = ends.values.iter().map(|&(v, _)| v).chain(std::iter::once(low)).collect();
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![high];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_NODES || lows.contains(&cur) {
            return false;
        }
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            continue;
        };
        if is_flag_op(op.code()) {
            return false;
        }
        let inputs = if op.code() == OpCode::CPUI_INDIRECT { 1 } else { op.num_input() };
        work.extend((0..inputs).filter_map(|i| op.get_in(i)));
    }
    true
}

/// Is `vn` a literal, or worked out from comparisons and literals alone --
/// a flag negated, shifted, offset or masked (`neg 3,3` after PowerPC's
/// `cntlzw; srwi 5`, `flag - 1`, `flag ^ -1`, `flag << 3`), or a choice
/// between such values -- so that it holds one of a few values a comparison
/// picks? A choice's input that its edge's branch has just tested equal to a
/// literal is that literal (ARM's `adcs r0,r0,#0; movne r0,#22` returns the
/// `adcs` result only when it is 0). One of `carries` is not such a flag but
/// the carry of the sum beside it, which a 64-bit sum adds into its high word.
fn of_flags(data: &Funcdata, vn: VarnodeId, carries: &[VarnodeId]) -> bool {
    fn walk(
        data: &Funcdata,
        vn: VarnodeId,
        depth: u32,
        carries: &[VarnodeId],
        done: &mut BTreeSet<VarnodeId>,
        path: &mut Vec<VarnodeId>,
    ) -> bool {
        if done.contains(&vn) {
            return true;
        }
        if depth == 0 || path.contains(&vn) || done.len() > MAX_NODES || carries.contains(&moved_from(data, vn)) {
            return false;
        }
        let Some(op) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            return data.vbank().get(vn).is_some_and(|v| v.is_constant());
        };
        let leaf = literal_value(data, vn, LITERAL_DEPTH).is_some()
            || nonzero_bits(data, vn, 4, &mut Vec::new()).count_ones() <= 1;
        let ok = leaf || {
            let inputs = match op.code() {
                OpCode::CPUI_INDIRECT if op.is_indirect_creation() => 0,
                OpCode::CPUI_INDIRECT => 1,
                OpCode::CPUI_COPY
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_INT_2COMP
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_BOOL_NEGATE
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_RIGHT
                | OpCode::CPUI_INT_SRIGHT => op.num_input(),
                _ => 0,
            };
            inputs > 0 && {
                path.push(vn);
                let phi = op.code() == OpCode::CPUI_MULTIEQUAL;
                let all = (0..inputs).all(|i| {
                    op.get_in(i).is_some_and(|x| {
                        (phi && pinned(data, op.get_parent(), i, x)) || walk(data, x, depth - 1, carries, done, path)
                    })
                });
                path.pop();
                all
            }
        };
        if ok {
            done.insert(vn);
        }
        ok
    }
    walk(data, vn, 8, carries, &mut BTreeSet::new(), &mut Vec::new())
}

/// `vn` before any copies, extensions and indirects that only move it.
fn moved_from(data: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut cur = vn;
    for _ in 0..64 {
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { break };
        match (op.code(), op.get_in(0)) {
            (OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT, Some(x)) => cur = x,
            (OpCode::CPUI_INDIRECT, Some(x)) if !op.is_indirect_creation() => cur = x,
            _ => break,
        }
    }
    cur
}

/// Is input `slot` of a choice in `block`, `vn`, pinned to a literal by its
/// edge: does the branch ending the predecessor that edge leaves send control
/// along it only when `vn` (through moves) equals a literal?
fn pinned(data: &Funcdata, block: Option<BlockId>, slot: int4, vn: VarnodeId) -> bool {
    let Some(bl) = block else { return false };
    let block = data.bblocks_ref().block(bl);
    if slot >= block.size_in() {
        return false;
    }
    let (from, edge) = (block.get_in(slot), block.get_in_rev_index(slot));
    let Some(cbranch) = data.bb_op_tail(from).and_then(|t| data.obank().get(t)) else { return false };
    if cbranch.code() != OpCode::CPUI_CBRANCH {
        return false;
    }
    let Some(cond) = cbranch.get_in(1) else { return false };
    let mut facts = Vec::new();
    equal_along(data, cond, (edge == 1) != cbranch.is_boolean_flip(), 8, &mut facts);
    let at = moved_from(data, vn);
    facts.iter().any(|&f| moved_from(data, f) == at)
}

/// The values the condition `cond` pins to a literal when it is `holds`:
/// `x == k` true, `x != k` false, through boolean negations and comparisons
/// of a 0-or-1 value with 0 or 1.
fn equal_along(data: &Funcdata, cond: VarnodeId, holds: bool, depth: u32, facts: &mut Vec<VarnodeId>) {
    let Some(op) = data.vbank().get(cond).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { return };
    if depth == 0 {
        return;
    }
    match op.code() {
        OpCode::CPUI_COPY => {
            if let Some(x) = op.get_in(0) {
                equal_along(data, x, holds, depth - 1, facts);
            }
        }
        OpCode::CPUI_BOOL_NEGATE => {
            if let Some(x) = op.get_in(0) {
                equal_along(data, x, !holds, depth - 1, facts);
            }
        }
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let equal = op.code() == OpCode::CPUI_INT_EQUAL;
            let (Some(a), Some(b)) = (op.get_in(0), op.get_in(1)) else { return };
            let (x, k) = match (literal_value(data, a, LITERAL_DEPTH), literal_value(data, b, LITERAL_DEPTH)) {
                (None, Some(k)) => (a, k),
                (Some(k), None) => (b, k),
                _ => return,
            };
            if holds == equal {
                facts.push(x);
            }
            if k <= 1 && nonzero_bits(data, x, 4, &mut Vec::new()) <= 1 {
                equal_along(data, x, (holds == equal) == (k == 1), depth - 1, facts);
            }
        }
        _ => {}
    }
}

/// The carries and borrows out of the sum or difference `sum`: comparisons
/// of nothing but its operands and itself, each possibly through a copy or a
/// truncation (`addcc`'s carry of `op1:4` and `op2:4`, PowerPC's `addc` into
/// CA, MIPS's `sltu` of the sum against an operand). A sign or zero test of
/// the sum alone is not one: it compares against a literal the sum does not
/// add.
fn carries_of(data: &Funcdata, sum: VarnodeId) -> Vec<VarnodeId> {
    let Some(op) = data.vbank().get(sum).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return Vec::new();
    };
    let root = |x: VarnodeId| flag_root(data, x);
    let operands: Vec<VarnodeId> =
        (0..op.num_input()).filter_map(|i| op.get_in(i)).chain(std::iter::once(sum)).map(root).collect();
    let constants: Vec<u64> = operands
        .iter()
        .filter_map(|&x| data.vbank().get(x))
        .filter(|x| x.is_constant())
        .map(|x| x.get_offset())
        .collect();
    let is_operand = |x: VarnodeId| {
        let size = data.vbank().get(x).map_or(8, |x| x.get_size());
        let r = root(x);
        operands.contains(&r)
            || data.vbank().get(r).is_some_and(|r| {
                r.is_constant() && constants.iter().any(|&k| k & ones(size) == r.get_offset() & ones(size))
            })
    };
    let mut aliases: Vec<VarnodeId> =
        operands.iter().copied().filter(|&x| data.vbank().get(x).is_some_and(|v| !v.is_constant())).collect();
    let mut i = 0;
    while i < aliases.len() && aliases.len() < 64 {
        if let Some(v) = data.vbank().get(aliases[i]) {
            for d in v.descend_iter() {
                let Some(next) = data.obank().get(d) else { continue };
                let moves = matches!(next.code(), OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE);
                if moves && next.get_out().is_some_and(|o| root(o) == root(aliases[i])) {
                    aliases.extend(next.get_out().filter(|o| !aliases.contains(o)));
                }
            }
        }
        i += 1;
    }
    let mut flags: Vec<VarnodeId> = Vec::new();
    for &x in &aliases {
        let Some(v) = data.vbank().get(x) else { continue };
        for d in v.descend_iter() {
            let Some(f) = data.obank().get(d) else { continue };
            let carry = matches!(
                f.code(),
                OpCode::CPUI_INT_CARRY
                    | OpCode::CPUI_INT_SCARRY
                    | OpCode::CPUI_INT_SBORROW
                    | OpCode::CPUI_INT_LESS
                    | OpCode::CPUI_INT_LESSEQUAL
                    | OpCode::CPUI_INT_SLESS
                    | OpCode::CPUI_INT_SLESSEQUAL
            );
            if !f.is_dead() && carry && (0..f.num_input()).all(|i| f.get_in(i).is_some_and(is_operand)) {
                flags.extend(f.get_out().filter(|o| !flags.contains(o)));
            }
        }
    }
    flags
}

/// `vn` before copies and truncations to its own low bytes, the value a
/// flag compares (`addcc`'s carry of `op1:4` and `op2:4`).
fn flag_root(data: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut cur = vn;
    for _ in 0..4 {
        let Some(d) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            break;
        };
        let at_zero = d.get_in(1).and_then(|k| data.vbank().get(k)).is_some_and(|k| k.is_constant() && k.get_offset() == 0);
        match (d.code(), d.get_in(0)) {
            (OpCode::CPUI_COPY, Some(x)) => cur = x,
            (OpCode::CPUI_SUBPIECE, Some(x)) if at_zero => cur = x,
            _ => break,
        }
    }
    cur
}

/// Does one of `carries` reach a conditional branch or the first register
/// `high`: was the carry or borrow tested?
fn tests_carry(data: &Funcdata, carries: &[VarnodeId], high: VarnodeId) -> bool {
    let mut flags = carries.to_vec();
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    while let Some(cur) = flags.pop() {
        if cur == high {
            return true;
        }
        if !seen.insert(cur) || seen.len() > MAX_NODES {
            continue;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        for d in v.descend_iter() {
            let Some(next) = data.obank().get(d) else { continue };
            if next.is_dead() {
                continue;
            }
            if next.code() == OpCode::CPUI_CBRANCH {
                return true;
            }
            flags.extend(next.get_out());
        }
    }
    false
}

/// The value `vn` is, through copies, the byte `k` piece of: `SUBPIECE(w,
/// k)`, or `SUBPIECE(w >> 8 * n, k)` as the piece `k + n` of `w` (MIPS's
/// `mfhi`).
fn piece_of(data: &Funcdata, vn: VarnodeId) -> Option<(VarnodeId, u64)> {
    let mut cur = vn;
    for _ in 0..8 {
        let op = data.obank().get(data.vbank().get(cur)?.get_def()?)?;
        match op.code() {
            OpCode::CPUI_COPY => cur = op.get_in(0)?,
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => cur = op.get_in(0)?,
            OpCode::CPUI_SUBPIECE => {
                let at = data.vbank().get(op.get_in(1)?)?;
                if !at.is_constant() {
                    return None;
                }
                let whole = op.get_in(0)?;
                let shifted = data.vbank().get(whole)?.get_def().and_then(|d| data.obank().get(d)).and_then(|sh| {
                    let by = data.vbank().get(sh.get_in(1)?)?;
                    (matches!(sh.code(), OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT)
                        && by.is_constant()
                        && by.get_offset() % 8 == 0)
                        .then(|| (sh.get_in(0), by.get_offset() / 8))
                });
                return match shifted {
                    Some((Some(w), bytes)) => Some((w, at.get_offset() + bytes)),
                    _ => Some((whole, at.get_offset())),
                };
            }
            _ => return None,
        }
    }
    None
}

/// Are `low` and `high`, through copies, the halves of one value at least two
/// registers wide: `SUBPIECE(w, 0)` and `SUBPIECE(w, lo_size)`, the second
/// possibly spelled `SUBPIECE(w >> 8 * lo_size, 0)` (MIPS `mfhi`)? `w` has to
/// be made wide by an operation (a load, a product, an extension): a register
/// pair heritage reassembles or merges (SPARC's `%i0:%i1`, which `ldd` and
/// `std` use elsewhere in the function) holds two values, not one.
fn halves_of_one_value(data: &Funcdata, low: VarnodeId, high: VarnodeId, lo_size: int4) -> bool {
    let made_wide = |w: VarnodeId| {
        data.vbank().get(w).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)).is_some_and(|op| {
            !matches!(
                op.code(),
                OpCode::CPUI_PIECE | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT
            )
        })
    };
    match (piece_of(data, low), piece_of(data, high)) {
        (Some((w, 0)), Some((w2, k))) => w == w2 && k == lo_size as u64 && made_wide(w),
        _ => false,
    }
}

/// Where a returned value was made ([`ends_of`]).
#[derive(Default)]
struct Ends {
    /// Values and literals the walk stopped at, each with the byte offset of
    /// the low word inside it.
    values: Vec<(VarnodeId, int4)>,
    /// The register's own entry value, reached with no instruction moving it.
    entry: bool,
    /// The walk ran out of budget.
    overflow: bool,
}

/// Walk back from `low` through moves -- copies, phis, indirects, and the
/// pieces of a wider register heritage splits and joins (SPARC's `ldd` writes
/// `%i0:%i1` as one, a byte store splits `%i1` into bytes) -- to where its
/// `lo_size` bytes were made, following where those bytes sit inside each
/// value on the way. A copy that a register
/// window makes ([`moves_register_window`]) does not count as the function
/// moving the value; any other does, so the entry value of `own` reached
/// through one is a value the function put back on purpose (ARM's `mov r4,r1;
/// bl ext; mov r1,r4`).
fn ends_of(data: &Funcdata, low: VarnodeId, own: &Address, lo_size: int4) -> Ends {
    let mut ends = Ends::default();
    let mut seen: BTreeSet<(VarnodeId, int4, int4, bool)> = BTreeSet::new();
    let mut work = vec![(low, 0, lo_size, false)];
    while let Some((cur, off, len, moved)) = work.pop() {
        if !seen.insert((cur, off, len, moved)) {
            continue;
        }
        if seen.len() > MAX_NODES {
            ends.overflow = true;
            break;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        if v.is_constant() {
            ends.values.push((cur, off));
            continue;
        }
        let Some(def) = v.get_def() else {
            if !v.is_input() {
                continue;
            }
            if !moved && within(slice_address(v.get_addr(), v.get_size(), off, len), len, own, lo_size) {
                ends.entry = true;
            } else {
                ends.values.push((cur, off));
            }
            continue;
        };
        let Some(op) = data.obank().get(def) else { continue };
        let input = |i: int4| op.get_in(i).and_then(|x| data.vbank().get(x).map(|v| (x, v)));
        match op.code() {
            OpCode::CPUI_COPY if !input(0).is_some_and(|(_, x)| x.is_constant()) => {
                work.extend(op.get_in(0).map(|x| (x, off, len, moved || !moves_register_window(data, def))));
            }
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => {}
            OpCode::CPUI_INDIRECT => work.extend(op.get_in(0).map(|x| (x, off, len, moved))),
            OpCode::CPUI_MULTIEQUAL => {
                work.extend((0..op.num_input()).filter_map(|i| op.get_in(i)).map(|x| (x, off, len, moved)));
            }
            OpCode::CPUI_PIECE => match (input(0), input(1)) {
                (Some((hi, _)), Some((lo_id, lo))) => {
                    let split = lo.get_size();
                    if off < split {
                        work.push((lo_id, off, len.min(split - off), moved));
                    }
                    if off + len > split {
                        let from = off.max(split);
                        work.push((hi, from - split, off + len - from, moved));
                    }
                }
                _ => ends.values.push((cur, off)),
            },
            OpCode::CPUI_SUBPIECE => match (input(0), input(1)) {
                (Some((whole, _)), Some((_, k))) if k.is_constant() => {
                    work.push((whole, off + k.get_offset() as int4, len, moved))
                }
                _ => ends.values.push((cur, off)),
            },
            _ => ends.values.push((cur, off)),
        }
    }
    ends
}

/// Do the `len` bytes at `at` lie inside the `size`-byte register at `own`?
fn within(at: Option<Address>, len: int4, own: &Address, size: int4) -> bool {
    at.is_some_and(|a| {
        a.get_space().zip(own.get_space()).is_some_and(|(x, y)| Rc::ptr_eq(x, y))
            && a.get_offset() >= own.get_offset()
            && a.get_offset() + len as u64 <= own.get_offset() + size as u64
    })
}

/// Where the `size` bytes `off` bytes above the least significant byte of the
/// `whole`-byte value at `addr` are stored.
fn slice_address(addr: &Address, whole: int4, off: int4, size: int4) -> Option<Address> {
    let space = addr.get_space()?;
    let rel = if space.is_big_endian() { whole - off - size } else { off };
    (rel >= 0).then(|| Address::new(Rc::clone(space), space.wrap_offset(addr.get_offset().wrapping_add(rel as u64))))
}

/// Is every use of `value` -- and of everything computed from it -- one of the
/// pair's RETURN slots: the low word itself, or the first register through the
/// value's sign (`sra 31`, the high half of its extension), a comparison (a
/// carry or borrow out of it), or the other half of a wider value? Reaching the
/// first register any other way, or unchanged, makes it scratch -- a literal
/// SPARC multiplies by is extended into the product, not returned -- and so
/// does a call, a store, a branch, an address, or the stack pointer (clang -O0
/// restores `%sp` through `%i1` after a variable-length array).
fn only_returned(data: &Funcdata, value: VarnodeId, off: int4, pair: &Pair) -> bool {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Via {
        Same,
        Extended,
        Changed,
        Excused,
    }
    let bump = |via: Via| if via == Via::Excused { Via::Excused } else { Via::Changed };
    let stack_pointer = data.get_arch().manage().get_stack_space().and_then(|s| s.get_spacebase(0).ok());
    let sets_stack_pointer = |out: VarnodeId| {
        let (Some(v), Some(sp)) = (data.vbank().get(out), stack_pointer.as_ref()) else { return false };
        let a = v.get_addr();
        a.get_space().zip(sp.space.as_ref()).is_some_and(|(x, y)| Rc::ptr_eq(x, y))
            && a.get_offset() < sp.offset + sp.size as u64
            && sp.offset < a.get_offset() + v.get_size() as u64
    };
    let constant = |op: &crate::op::PcodeOp, i: int4| {
        op.get_in(i).and_then(|k| data.vbank().get(k)).filter(|k| k.is_constant()).map(|k| k.get_offset())
    };
    let lo = pair.lo_size as u64;
    let mut seen: BTreeSet<(VarnodeId, Via, int4)> = BTreeSet::new();
    let mut work = vec![(value, Via::Same, off)];
    while let Some((cur, via, off)) = work.pop() {
        if !seen.insert((cur, via, off)) {
            continue;
        }
        if seen.len() > MAX_NODES {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        let bits = 8 * v.get_size() as u64;
        let whole = matches!(via, Via::Same | Via::Extended);
        let low_at = off as u64;
        for d in v.descend_iter() {
            let Some(op) = data.obank().get(d) else { continue };
            if op.is_dead() {
                continue;
            }
            let (next, next_off) = match op.code() {
                OpCode::CPUI_RETURN => {
                    for i in 0..op.num_input() {
                        if op.get_in(i) != Some(cur) || i == pair.lo_slot {
                            continue;
                        }
                        if i != pair.hi_slot || via != Via::Excused {
                            return false;
                        }
                    }
                    continue;
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if cur == value && v.get_def().is_none() && undecided_input(data, d) => {
                    continue
                }
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_STORE
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND => return false,
                OpCode::CPUI_LOAD => {
                    if op.get_in(1) == Some(cur) {
                        return false;
                    }
                    (bump(via), 0)
                }
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => (via, off),
                OpCode::CPUI_INDIRECT => {
                    if op.get_in(0) != Some(cur) {
                        continue;
                    }
                    (via, off)
                }
                OpCode::CPUI_SUBPIECE if whole && bits > 8 * lo => match constant(op, 1) {
                    Some(k) if k == low_at => (Via::Same, 0),
                    Some(k) if k >= low_at + lo => (Via::Excused, 0),
                    _ => (bump(via), 0),
                },
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT
                    if whole && bits > 8 * lo && op.get_in(0) == Some(cur) =>
                {
                    match constant(op, 1) {
                        Some(c) if c >= 8 * (low_at + lo) => (Via::Excused, 0),
                        _ => (bump(via), 0),
                    }
                }
                OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT if via == Via::Same && off == 0 => (Via::Extended, 0),
                OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL
                | OpCode::CPUI_INT_CARRY
                | OpCode::CPUI_INT_SCARRY
                | OpCode::CPUI_INT_SBORROW => (Via::Excused, 0),
                OpCode::CPUI_INT_SRIGHT
                    if matches!(via, Via::Same | Via::Excused)
                        && op.get_in(0) == Some(cur)
                        && constant(op, 1).is_some_and(|c| c + 1 == bits) =>
                {
                    (Via::Excused, 0)
                }
                _ => (bump(via), 0),
            };
            if let Some(out) = op.get_out() {
                if sets_stack_pointer(out) {
                    return false;
                }
                work.push((out, next, next_off));
            }
        }
    }
    true
}

/// Are the inputs of `call` still trials -- registers a callee without a known
/// prototype might read -- rather than its settled arguments? The function's
/// own input passing through such a call is not yet an argument of it.
fn undecided_input(data: &Funcdata, call: OpId) -> bool {
    data.get_call_specs_index(call).is_some_and(|i| data.get_call_specs(i).is_input_active())
}

/// Every bit of a `size`-byte value set.
fn ones(size: int4) -> u64 {
    if size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    }
}

/// The value of `vn` when it is a constant or integer arithmetic and logic on
/// constants alone, within `depth` operations, truncated to its size.
fn literal_value(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    let mask = |x: u64, sz: int4| if sz >= 8 { x } else { x & ((1u64 << (8 * sz)) - 1) };
    if v.is_constant() {
        return Some(mask(v.get_offset(), size));
    }
    if depth == 0 {
        return None;
    }
    let op = data.obank().get(v.get_def()?)?;
    let arg = |i: int4| op.get_in(i).and_then(|x| literal_value(data, x, depth - 1));
    let arg_size = |i: int4| op.get_in(i).and_then(|x| data.vbank().get(x)).map_or(0, |x| x.get_size());
    let r = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => arg(0)?,
        OpCode::CPUI_INT_SEXT => {
            let (x, sz) = (arg(0)?, arg_size(0));
            if sz < 8 && x >> (8 * sz - 1) & 1 == 1 {
                x | !((1u64 << (8 * sz)) - 1)
            } else {
                x
            }
        }
        OpCode::CPUI_INT_ADD => arg(0)?.wrapping_add(arg(1)?),
        OpCode::CPUI_INT_SUB => arg(0)?.wrapping_sub(arg(1)?),
        OpCode::CPUI_INT_OR => arg(0)? | arg(1)?,
        OpCode::CPUI_INT_XOR => arg(0)? ^ arg(1)?,
        OpCode::CPUI_INT_AND => arg(0)? & arg(1)?,
        OpCode::CPUI_INT_LEFT => arg(0)?.checked_shl(arg(1)?.try_into().ok()?).unwrap_or(0),
        OpCode::CPUI_INT_RIGHT => arg(0)?.checked_shr(arg(1)?.try_into().ok()?).unwrap_or(0),
        OpCode::CPUI_INT_NEGATE => !arg(0)?,
        OpCode::CPUI_INT_2COMP => arg(0)?.wrapping_neg(),
        OpCode::CPUI_PIECE => {
            let lo_bits = 8 * arg_size(1) as u32;
            arg(0)?.checked_shl(lo_bits).unwrap_or(0) | arg(1)?
        }
        OpCode::CPUI_SUBPIECE => arg(0)?.checked_shr(8 * arg(1)? as u32).unwrap_or(0),
        _ => return None,
    };
    Some(mask(r, size))
}

/// Is `copy` one register's move in a register-window instruction: a COPY of
/// one register into another, at a machine instruction that copies every
/// general-purpose register the prototype model passes arguments in, either
/// out of them (SPARC's `save` moves `%o0`-`%o5` into `%i0`-`%i5`) or back
/// into them (`restore`)? The register copied may be a heritage temporary
/// reassembling a register the function also reads in parts. `restore`'s own
/// destination write (`restore %g0,1,%o1`) copies a temporary, so it is not the
/// window's. A compiler's move copies one register (`mov r1,r4`) or a pair
/// (AVR's `movw`), so it never answers `true`; neither does a model with fewer
/// than three argument registers, where "every" says too little.
fn moves_register_window(data: &Funcdata, copy: OpId) -> bool {
    fn is_register(a: &Address) -> bool {
        a.get_space().is_some_and(|sp| sp.get_type() == spacetype::IPTR_PROCESSOR)
    }
    fn reassembled(data: &Funcdata, src: &crate::varnode::Varnode, depth: u32) -> bool {
        depth > 0
            && src.get_def().and_then(|d| data.obank().get(d)).is_some_and(|piece| {
                piece.code() == OpCode::CPUI_PIECE
                    && (0..piece.num_input()).all(|i| {
                        piece
                            .get_in(i)
                            .and_then(|x| data.vbank().get(x))
                            .is_some_and(|x| is_register(x.get_addr()) || reassembled(data, x, depth - 1))
                    })
            })
    }
    let Some(op) = data.obank().get(copy) else { return false };
    let one_register = op.code() == OpCode::CPUI_COPY
        && match (op.get_out().and_then(|x| data.vbank().get(x)), op.get_in(0).and_then(|x| data.vbank().get(x))) {
            (Some(out), Some(src)) => {
                is_register(out.get_addr())
                    && out.get_addr() != src.get_addr()
                    && (is_register(src.get_addr()) || reassembled(data, src, 4))
            }
            _ => false,
        };
    if !one_register {
        return false;
    }
    let proto = data.get_func_proto();
    if !proto.has_model() {
        return false;
    }
    let Some(input) = proto.model().input_opt() else { return false };
    let at = op.get_addr().clone();
    let mut into: Vec<(Address, i32)> = Vec::new();
    let mut from: Vec<(Address, i32)> = Vec::new();
    for (_, id) in data.obank().iter_at(&at) {
        let Some(op) = data.obank().get(id) else { continue };
        if op.is_dead() || op.code() != OpCode::CPUI_COPY {
            continue;
        }
        let (Some(out), Some(src)) =
            (op.get_out().and_then(|x| data.vbank().get(x)), op.get_in(0).and_then(|x| data.vbank().get(x)))
        else {
            continue;
        };
        if src.get_addr() == out.get_addr() {
            continue;
        }
        if is_register(out.get_addr()) {
            into.push((out.get_addr().clone(), out.get_size()));
        }
        if is_register(src.get_addr()) {
            from.push((src.get_addr().clone(), src.get_size()));
        }
    }
    let registers: Vec<&crate::fspec::ParamEntry> = input
        .get_entry()
        .iter()
        .filter(|e| {
            e.get_type() == crate::dtype::type_class::TYPECLASS_GENERAL
                && e.get_space().get_type() == spacetype::IPTR_PROCESSOR
        })
        .collect();
    let covers = |moved: &[(Address, i32)]| {
        registers.iter().all(|e| {
            moved.iter().any(|(a, size)| {
                *size == e.get_size()
                    && a.get_offset() == e.get_base()
                    && a.get_space().is_some_and(|sp| Rc::ptr_eq(sp, e.get_space()))
            })
        })
    };
    registers.len() >= 3 && (covers(&into) || covers(&from))
}

/// Is `retop` reached only along branch edges whose conditions literals
/// already decide the other way? SPARC's `call` keeps such a RETURN for a
/// `restore` in its delay slot (`didrestore = 0; ...; if (didrestore == 0)
/// goto next; return [o7]`), and return recovery settles the prototype before
/// the rule pool folds it away, so its `%o1` is whatever the call was passed.
fn never_reached(data: &Funcdata, retop: OpId) -> bool {
    let Some(bl) = data.obank().get(retop).and_then(|o| o.get_parent()) else { return false };
    let block = data.bblocks_ref().block(bl);
    if block.size_in() == 0 {
        return false;
    }
    (0..block.size_in()).all(|k| {
        let (from, edge) = (block.get_in(k), block.get_in_rev_index(k));
        let Some(cbranch) = data.bb_op_tail(from).and_then(|t| data.obank().get(t)) else { return false };
        if cbranch.code() != OpCode::CPUI_CBRANCH {
            return false;
        }
        let Some(val) = cbranch.get_in(1).and_then(|c| decided(data, c, 8)) else { return false };
        let taken = if (val != 0) != cbranch.is_boolean_flip() { 1 } else { 0 };
        taken != edge
    })
}

/// The value of a condition built from literals by copies, `==`, `!=` and
/// `!`, or `None` when anything else (an INDIRECT a call may change, an input)
/// feeds it.
fn decided(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return Some(v.get_offset());
    }
    if depth == 0 {
        return None;
    }
    let op = data.obank().get(v.get_def()?)?;
    let arg = |i: i32| op.get_in(i).and_then(|x| decided(data, x, depth - 1));
    match op.code() {
        OpCode::CPUI_COPY => arg(0),
        OpCode::CPUI_BOOL_NEGATE => arg(0).map(|x| (x == 0) as u64),
        OpCode::CPUI_INT_EQUAL => Some((arg(0)? == arg(1)?) as u64),
        OpCode::CPUI_INT_NOTEQUAL => Some((arg(0)? != arg(1)?) as u64),
        _ => None,
    }
}

/// Does `whole` sit in two of the model's output registers, the FIRST of them
/// holding its most significant half -- a pair return recovery joined in the
/// ABI's order ([`join_order`])? `false` for one register, storage the model
/// does not list, or a pair joined first register low.
pub(crate) fn first_register_holds_high(data: &Funcdata, whole: VarnodeId) -> bool {
    live(data)
        && pair_halves(data, whole)
        .is_some_and(|(hi, hi_size, lo, lo_size)| data.get_func_proto().output_holds_high_first(&hi, hi_size, &lo, lo_size))
}

/// The storage of `whole`'s most and least significant halves -- two pieces of
/// a join, or the two halves of one register -- with their sizes.
fn pair_halves(data: &Funcdata, whole: VarnodeId) -> Option<(Address, i32, Address, i32)> {
    use crate::kuna_returnuncomputed::{slot_storage, storage_pieces};
    let pieces = storage_pieces(data, whole)?;
    match pieces.as_slice() {
        [(hs, ho, hz), (ls, lo, lz)] => {
            Some((Address::new(Rc::clone(hs), *ho), *hz, Address::new(Rc::clone(ls), *lo), *lz))
        }
        [_] => {
            let size = data.vbank().get(whole).map(|v| v.get_size()).unwrap_or(0);
            if size < 2 || size % 2 != 0 {
                return None;
            }
            let half = size / 2;
            Some((slot_storage(data, whole, half, half)?, half, slot_storage(data, whole, 0, half)?, half))
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "kuna_bejoin/tests.rs"]
mod tests;
