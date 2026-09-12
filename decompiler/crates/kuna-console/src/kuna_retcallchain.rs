//! (kuna `override flow`) The RET-call chain a flow-call override starts.
//!
//! A `push <continuation>; push <target>; ret` run is a call written without a
//! `call` instruction: the `ret` pops `<target>` and jumps to it, and the callee
//! returns to `<continuation>`, which is the instruction right after the `ret`.
//! Packers and crackmes build whole function bodies out of these, one link per
//! callee, and each link is a separate `ret` as far as the flow follower is
//! concerned.
//!
//! `override flow <addr> call` reclassifies ONE of them.  The follower then
//! resumes at the continuation exactly as it should — and runs straight into the
//! next link's `ret`, which is still a return, so the whole tail of the function
//! is dead and decompiles to `return;`.  The caller sees one recovered call and
//! no evidence that the rest of the chain exists.
//!
//! [`kuna_chain_sites`] walks that chain from the overridden site and reports the
//! other `ret`s in it, so the same override can be planted on each.
//! [`kuna_entry_chain_sites`] exposes the same recognition for an unasserted
//! function entry, allowing the decompile drive to seed the complete chain by
//! default. A `ret` is a link only when raw-p-code provenance proves its
//! destination was loaded from a slot written by this run and the adjacent slot
//! contains the `ret`'s OWN fall-through. An ordinary epilogue, a discarded
//! fall-through push, and an unrelated-memory store cannot satisfy that pair; a
//! LOAD/STORE pair must also name the same p-code memory space. A real `call` or
//! unsupported `CALLOTHER` clears provenance before the walk continues.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

/// How the decoded instruction leaves: what the walk does next.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Leave {
    /// Ends in `CPUI_RETURN` — a chain-link candidate.
    Return,
    /// An unconditional direct branch: the walk follows it.
    Jump(Address),
    /// Falls through to the next address.
    FallThru,
    /// An indirect branch or an indirect call: the walk cannot follow it.
    Opaque,
}

/// A raw-p-code varnode identity. Space indices are stable within one
/// translator, and the offset/size pair distinguishes register slices and
/// unique temporaries.
type VnKey = (i32, uintb, u32);

#[derive(Clone)]
struct RawOp {
    opcode: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

/// A [`PcodeEmit`] that records one decoded instruction whole.
#[derive(Default)]
struct ChainScan {
    ops: Vec<RawOp>,
    lastop: Option<OpCode>,
    jump: Option<Address>,
    call: bool,
    conditional: bool,
}

impl ChainScan {
    /// True when the varnode names the constant space.
    fn is_const(v: &VarnodeData) -> bool {
        v.space
            .as_ref()
            .is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
    }

    /// The code address a branch destination varnode names, or `None` for a
    /// p-code-relative (constant-space) internal branch.
    fn code_dest(v: &VarnodeData) -> Option<Address> {
        match &v.space {
            Some(s) if s.get_type() != spacetype::IPTR_CONSTANT => {
                Some(Address::new(std::rc::Rc::clone(s), v.offset))
            }
            _ => None,
        }
    }

    fn leave(&self) -> Leave {
        // A single fall-through walk cannot prove that setup after a
        // conditional dominates a later RETURN. Stop rather than infer across
        // a path that may bypass the stores.
        if self.conditional {
            return Leave::Opaque;
        }
        match self.lastop {
            Some(OpCode::CPUI_RETURN) => Leave::Return,
            Some(OpCode::CPUI_BRANCHIND) => Leave::Opaque,
            Some(OpCode::CPUI_BRANCH) => match &self.jump {
                Some(a) => Leave::Jump(a.clone()),
                None => Leave::FallThru,
            },
            _ => Leave::FallThru,
        }
    }
}

impl PcodeEmit for ChainScan {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.lastop = Some(opc);
        self.ops.push(RawOp {
            opcode: opc,
            out: outvar.cloned(),
            ins: vars.to_vec(),
        });
        match opc {
            OpCode::CPUI_BRANCH => {
                self.jump = vars.first().and_then(Self::code_dest);
            }
            OpCode::CPUI_CBRANCH => self.conditional = true,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => self.call = true,
            _ => {}
        }
    }
}

/// One decoded instruction's chain-relevant shape.
struct Decoded {
    len: int4,
    ops: Vec<RawOp>,
    leave: Leave,
    call: bool,
}

/// Decode the instruction at `at`, or `None` when it does not decode.
fn decode_one(trans: &dyn Translate, at: &Address) -> Option<Decoded> {
    let mut scan = ChainScan::default();
    // Advisory probe over bytes nothing has vouched for: contain a decode `Err`
    // AND any translator panic to `None`, the way `lone_jump_target` does.
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        trans.one_instruction(&mut scan, at)
    }));
    let len = match decoded {
        Ok(Ok(len)) => len,
        _ => return None,
    };
    if len <= 0 {
        return None;
    }
    let leave = scan.leave();
    Some(Decoded {
        len,
        ops: scan.ops,
        leave,
        call: scan.call,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RelativeAddress {
    base: VnKey,
    offset: i128,
}

/// The address-space identifier carried in input 0 of CPUI_LOAD/CPUI_STORE.
/// Two numerically equal affine pointers in different spaces are not aliases.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MemorySpaceId(uintb);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MemoryLocation {
    space: MemorySpaceId,
    address: RelativeAddress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackedValue {
    Constant(uintb),
    Relative(RelativeAddress),
    /// A LOAD from a slot written during this same straight-line run.
    FromStore(MemoryLocation, u32),
}

#[derive(Clone, Copy, Debug)]
struct StoreFact {
    location: MemoryLocation,
    size: u32,
    value: Option<TrackedValue>,
}

/// Minimal, conservative raw-p-code provenance for the stack-directed idiom.
///
/// The recognizer does not need general constant propagation. It needs to prove
/// two facts: RETURN's destination was loaded from a slot this run wrote, and
/// the immediately following slot contains this RET's own fall-through. An
/// affine register expression is enough to follow x86 PUSH/ADD/RET p-code while
/// refusing unknown pointer arithmetic, cross-space aliases, unrelated-memory
/// stores, and unsupported userops with unknown side effects.
#[derive(Default)]
struct ReturnProvenance {
    values: BTreeMap<VnKey, TrackedValue>,
    invalid: BTreeSet<VnKey>,
    stores: Vec<StoreFact>,
}

impl ReturnProvenance {
    fn key(v: &VarnodeData) -> Option<VnKey> {
        v.space.as_ref().map(|s| (s.get_index(), v.offset, v.size))
    }

    fn is_register(v: &VarnodeData) -> bool {
        v.space.as_ref().is_some_and(|s| {
            s.get_type() == spacetype::IPTR_PROCESSOR && s.get_name() == "register"
        })
    }

    fn memory_space(v: &VarnodeData) -> Option<MemorySpaceId> {
        ChainScan::is_const(v).then_some(MemorySpaceId(v.offset))
    }

    fn keys_overlap(a: VnKey, b: VnKey) -> bool {
        if a.0 != b.0 {
            return false;
        }
        let a_start = u128::from(a.1);
        let b_start = u128::from(b.1);
        let a_end = a_start + u128::from(a.2);
        let b_end = b_start + u128::from(b.2);
        a_start < b_end && b_start < a_end
    }

    fn value(&self, v: &VarnodeData) -> Option<TrackedValue> {
        if ChainScan::is_const(v) {
            return Some(TrackedValue::Constant(v.offset));
        }
        let key = Self::key(v)?;
        self.values.get(&key).copied().or_else(|| {
            (Self::is_register(v)
                && !self
                    .invalid
                    .iter()
                    .any(|invalid| Self::keys_overlap(*invalid, key)))
            .then_some(TrackedValue::Relative(RelativeAddress {
                base: key,
                offset: 0,
            }))
        })
    }

    fn set(&mut self, out: Option<&VarnodeData>, value: Option<TrackedValue>) {
        let Some(out) = out else {
            return;
        };
        let Some(key) = Self::key(out) else {
            return;
        };
        if Self::is_register(out) {
            self.values
                .retain(|other, _| !Self::keys_overlap(*other, key));
            self.stores.retain(|fact| {
                if !Self::keys_overlap(fact.location.address.base, key) {
                    return true;
                }
                matches!(
                    value,
                    Some(TrackedValue::Relative(address))
                        if address.base == fact.location.address.base
                )
            });
            // Keep the written slice as an alias barrier even when its exact
            // value is known. Exact lookups use `values` first; wider or
            // narrower aliases cannot silently become fresh affine bases.
            self.invalid.insert(key);
        } else {
            self.values.remove(&key);
        }
        match value {
            Some(value) => {
                self.values.insert(key, value);
            }
            None => {
                self.values.remove(&key);
            }
        }
    }

    fn signed_constant(v: &VarnodeData) -> Option<i128> {
        if !ChainScan::is_const(v) || v.size == 0 || v.size > 8 {
            return None;
        }
        let bits = v.size * 8;
        if bits == 64 {
            return Some(i128::from(v.offset as i64));
        }
        let mask = (1_u64 << bits) - 1;
        let raw = v.offset & mask;
        let sign = 1_u64 << (bits - 1);
        if raw & sign == 0 {
            Some(i128::from(raw))
        } else {
            Some(i128::from(raw) - (1_i128 << bits))
        }
    }

    fn add_constant(value: TrackedValue, amount: i128, subtract: bool) -> Option<TrackedValue> {
        let TrackedValue::Relative(mut address) = value else {
            return None;
        };
        address.offset = if subtract {
            address.offset.checked_sub(amount)?
        } else {
            address.offset.checked_add(amount)?
        };
        Some(TrackedValue::Relative(address))
    }

    fn binary_relative(&self, opcode: OpCode, ins: &[VarnodeData]) -> Option<TrackedValue> {
        let left_vn = ins.first()?;
        let right_vn = ins.get(1)?;
        let left = self.value(left_vn);
        let right = self.value(right_vn);
        match opcode {
            OpCode::CPUI_INT_ADD => {
                if let (Some(value @ TrackedValue::Relative(_)), Some(amount)) =
                    (left, Self::signed_constant(right_vn))
                {
                    Self::add_constant(value, amount, false)
                } else if let (Some(amount), Some(value @ TrackedValue::Relative(_))) =
                    (Self::signed_constant(left_vn), right)
                {
                    Self::add_constant(value, amount, false)
                } else {
                    None
                }
            }
            OpCode::CPUI_INT_SUB => {
                let value = left?;
                let amount = Self::signed_constant(right_vn)?;
                Self::add_constant(value, amount, true)
            }
            _ => None,
        }
    }

    fn overlaps(a: &StoreFact, location: MemoryLocation, size: u32) -> bool {
        if a.location.space != location.space || a.location.address.base != location.address.base {
            return false;
        }
        let a_end = a.location.address.offset + i128::from(a.size);
        let b_end = location.address.offset + i128::from(size);
        a.location.address.offset < b_end && location.address.offset < a_end
    }

    fn store(&mut self, space_id: &VarnodeData, pointer: &VarnodeData, value: &VarnodeData) {
        let Some(space) = Self::memory_space(space_id) else {
            self.stores.clear();
            return;
        };
        let Some(TrackedValue::Relative(address)) = self.value(pointer) else {
            // An unknown pointer may alias the stack. Forget rather than infer.
            self.stores.clear();
            return;
        };
        let location = MemoryLocation { space, address };
        let size = value.size;
        // Distinct unproven register bases may alias. Preserve facts only when
        // the new pointer is in another memory space, or is proven relative to
        // their same base in this space.
        self.stores.retain(|fact| {
            fact.location.space != space
                || (fact.location.address.base == address.base
                    && !Self::overlaps(fact, location, size))
        });
        self.stores.push(StoreFact {
            location,
            size,
            value: self.value(value),
        });
    }

    fn load(
        &self,
        space_id: &VarnodeData,
        pointer: &VarnodeData,
        size: u32,
    ) -> Option<TrackedValue> {
        let space = Self::memory_space(space_id)?;
        let TrackedValue::Relative(address) = self.value(pointer)? else {
            return None;
        };
        let location = MemoryLocation { space, address };
        self.stores
            .iter()
            .rev()
            .find(|fact| fact.location == location && fact.size == size)
            .map(|_| TrackedValue::FromStore(location, size))
    }

    fn is_own_fallthrough_pair(&self, returned: &VarnodeData, fallthrough: uintb) -> bool {
        let Some(TrackedValue::FromStore(popped, size)) = self.value(returned) else {
            return false;
        };
        let Some(continuation_offset) = popped.address.offset.checked_add(i128::from(size)) else {
            return false;
        };
        // RETURN must have popped this slot: x86 RET advances the same register
        // that addressed its LOAD by exactly the loaded word before emitting
        // CPUI_RETURN. This rules out a coincidental load through another base.
        let popped_stack = TrackedValue::Relative(RelativeAddress {
            base: popped.address.base,
            offset: continuation_offset,
        });
        if self.values.get(&popped.address.base) != Some(&popped_stack) {
            return false;
        }
        self.stores.iter().any(|fact| {
            fact.location.space == popped.space
                && fact.location.address.base == popped.address.base
                && fact.location.address.offset == continuation_offset
                && fact.size == size
                && fact.value == Some(TrackedValue::Constant(fallthrough))
        })
    }

    /// Apply one instruction's p-code and return whether its RETURN consumes a
    /// store paired with its own fall-through continuation.
    fn apply(&mut self, ops: &[RawOp], fallthrough: uintb) -> bool {
        // A RETURN source must be loaded by this instruction, not be a stale
        // value retained from an earlier load in the straight-line run.
        self.values
            .retain(|_, value| !matches!(value, TrackedValue::FromStore(_, _)));
        let mut matched = false;
        for op in ops {
            match op.opcode {
                OpCode::CPUI_COPY => {
                    let value = op.ins.first().and_then(|v| self.value(v));
                    self.set(op.out.as_ref(), value);
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                    let value = self.binary_relative(op.opcode, &op.ins);
                    self.set(op.out.as_ref(), value);
                }
                OpCode::CPUI_LOAD => {
                    let value = match (op.out.as_ref(), op.ins.first(), op.ins.get(1)) {
                        (Some(out), Some(space), Some(pointer)) => {
                            self.load(space, pointer, out.size)
                        }
                        _ => None,
                    };
                    self.set(op.out.as_ref(), value);
                }
                OpCode::CPUI_STORE => {
                    if let (Some(space), Some(pointer), Some(value)) =
                        (op.ins.first(), op.ins.get(1), op.ins.get(2))
                    {
                        self.store(space, pointer, value);
                    } else {
                        self.stores.clear();
                    }
                }
                // An unsupported userop can read or write arbitrary state even
                // when it has no output. Treat it as a full provenance barrier.
                OpCode::CPUI_CALLOTHER => self.clear(),
                OpCode::CPUI_RETURN => {
                    matched = op.ins.first().is_some_and(|returned| {
                        self.is_own_fallthrough_pair(returned, fallthrough)
                    });
                }
                _ => self.set(op.out.as_ref(), None),
            }
        }
        matched
    }

    fn clear(&mut self) {
        self.values.clear();
        self.invalid.clear();
        self.stores.clear();
    }
}

/// The `ret`s of `entry`'s body that are RET-call links, in flow order.
///
/// The walk is a straight-line follow from `entry`: it takes an unconditional
/// direct branch and falls through ordinary instructions, but stops at any
/// conditional or indirect control flow because the scanned path cannot prove
/// dominance. It also stops at a decode failure, an address it already decoded,
/// or the first `ret` that is not a link. It carries only enough provenance to
/// prove RETURN loaded a slot this run wrote and that the adjacent continuation
/// slot holds the RET's own fall-through. Provenance is cleared at every link
/// and real call, so the test at a `ret` asks only about the run that reached it.
fn chain_of(
    trans: &dyn Translate,
    entry: &Address,
    max_sites: usize,
    max_insns: usize,
) -> Vec<Address> {
    let mut sites: Vec<Address> = Vec::new();
    let mut provenance = ReturnProvenance::default();
    let mut seen: BTreeSet<uintb> = BTreeSet::new();
    let mut cur = entry.clone();
    for _ in 0..max_insns {
        if !seen.insert(cur.get_offset()) {
            break;
        }
        let Some(d) = decode_one(trans, &cur) else {
            break;
        };
        let next = &cur + d.len as i64;
        let return_is_link = if d.call {
            // Neither the call's own implicit return-address store nor state a
            // real callee may mutate can establish provenance for a later RET.
            provenance.clear();
            false
        } else {
            provenance.apply(&d.ops, next.get_offset())
        };
        match d.leave {
            Leave::Return => {
                if !return_is_link {
                    break; // a genuine return: the chain ends here
                }
                sites.push(cur.clone());
                if sites.len() >= max_sites {
                    break;
                }
                provenance.clear();
                cur = next;
            }
            Leave::Jump(target) => cur = target,
            Leave::FallThru => cur = next,
            Leave::Opaque => break,
        }
    }
    sites
}

/// The RET-call links reached in flow order from a function entry.
///
/// Empty unless the first `RETURN` reached from `entry` loads a slot written by
/// this run and its adjacent continuation slot contains that instruction's own
/// fall-through address. The same bounded walk and strict provenance predicate
/// back both automatic entry recognition and explicit `flow <addr> call` chain
/// extension.
pub fn kuna_entry_chain_sites(
    trans: &dyn Translate,
    entry: &Address,
    max_sites: usize,
    max_insns: usize,
) -> Vec<Address> {
    if max_sites == 0 {
        return Vec::new();
    }
    chain_of(trans, entry, max_sites, max_insns)
}

/// The OTHER links of the RET-call chain the override at `at` names, in flow
/// order, given the entry of the function it sits in.
///
/// Empty unless `at` is itself a link — which is every ordinary use of
/// `override flow`, since an ordinary `ret` has no pushed continuation.
pub fn kuna_chain_sites(
    trans: &dyn Translate,
    entry: &Address,
    at: &Address,
    max_sites: usize,
    max_insns: usize,
) -> Vec<Address> {
    if max_sites == 0 {
        return Vec::new();
    }
    let sites = chain_of(trans, entry, max_sites, max_insns);
    if !sites.iter().any(|s| s == at) {
        return Vec::new();
    }
    sites.into_iter().filter(|s| s != at).collect()
}

/// How many links [`kuna_chain_sites`] reports, and how far it decodes looking
/// for them.  A chain is one call per link, so the site cap is a function-sized
/// call count and the instruction cap a function-sized body.
pub const CHAIN_MAX_SITES: usize = 256;
/// See [`CHAIN_MAX_SITES`].
pub const CHAIN_MAX_INSNS: usize = 4096;

/// The chained sites, spelled the way the console spells an address, for the
/// line `override flow` prints when it extended itself.
pub fn kuna_render_sites(sites: &[Address]) -> String {
    sites
        .iter()
        .map(|a| {
            let mut buf = String::new();
            let _ = a.print_raw(&mut buf);
            buf
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::space::{spacetype, AddrSpace, ConstantSpace, UniqueSpace};

    use super::*;

    fn vn(space: &Rc<AddrSpace>, offset: uintb, size: u32) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::clone(space)),
            offset,
            size,
        }
    }

    struct RetOps {
        space: VarnodeData,
        pointer: VarnodeData,
        loaded: VarnodeData,
        plus_word: VarnodeData,
        userop: VarnodeData,
    }

    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>, Rc<AddrSpace>) {
        let constant = Rc::new(ConstantSpace::new());
        let register = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            4,
            1,
            1,
            0,
            1,
            1,
        ));
        let unique = Rc::new(UniqueSpace::new(2, 0, false));
        (constant, register, unique)
    }

    /// Prepare the two stores immediately before an i386 RET-call link. The
    /// target is at base-8 and its continuation at base-4.
    fn prepared(
        target_space: uintb,
        continuation_space: uintb,
        fallthrough: uintb,
    ) -> (ReturnProvenance, RetOps) {
        let (constant, register, unique) = spaces();
        let pointer = vn(&register, 0, 4);
        let base = ReturnProvenance::key(&pointer).unwrap();
        let target_space = vn(&constant, target_space, 8);
        let continuation_space = vn(&constant, continuation_space, 8);
        let target = vn(&constant, 0x1122_3344, 4);
        let continuation = vn(&constant, fallthrough, 4);
        let mut provenance = ReturnProvenance::default();

        provenance.set(
            Some(&pointer),
            Some(TrackedValue::Relative(RelativeAddress { base, offset: -4 })),
        );
        provenance.store(&continuation_space, &pointer, &continuation);
        provenance.set(
            Some(&pointer),
            Some(TrackedValue::Relative(RelativeAddress { base, offset: -8 })),
        );
        provenance.store(&target_space, &pointer, &target);

        (
            provenance,
            RetOps {
                space: target_space,
                pointer,
                loaded: vn(&unique, 0x100, 4),
                plus_word: vn(&constant, 4, 4),
                userop: vn(&constant, 7, 4),
            },
        )
    }

    fn ret_ops(parts: &RetOps, with_userop: bool) -> Vec<RawOp> {
        let mut ops = Vec::new();
        if with_userop {
            ops.push(RawOp {
                opcode: OpCode::CPUI_CALLOTHER,
                out: None,
                ins: vec![parts.userop.clone()],
            });
        }
        ops.extend([
            RawOp {
                opcode: OpCode::CPUI_LOAD,
                out: Some(parts.loaded.clone()),
                ins: vec![parts.space.clone(), parts.pointer.clone()],
            },
            RawOp {
                opcode: OpCode::CPUI_INT_ADD,
                out: Some(parts.pointer.clone()),
                ins: vec![parts.pointer.clone(), parts.plus_word.clone()],
            },
            RawOp {
                opcode: OpCode::CPUI_RETURN,
                out: None,
                ins: vec![parts.loaded.clone()],
            },
        ]);
        ops
    }

    #[test]
    fn load_and_continuation_must_share_the_exact_memory_space() {
        let fallthrough = 0x804_900c;
        let (mut same, same_ops) = prepared(3, 3, fallthrough);
        assert!(same.apply(&ret_ops(&same_ops, false), fallthrough));

        let (mut crossed, crossed_ops) = prepared(3, 4, fallthrough);
        assert!(!crossed.apply(&ret_ops(&crossed_ops, false), fallthrough));
    }

    #[test]
    fn callother_between_setup_and_return_clears_all_provenance() {
        let fallthrough = 0x804_900c;
        let (mut provenance, parts) = prepared(3, 3, fallthrough);
        assert!(!provenance.apply(&ret_ops(&parts, true), fallthrough));
        assert!(provenance.stores.is_empty());
    }
}
