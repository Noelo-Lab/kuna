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
//! other `ret`s in it, so the same override can be planted on each.  The test is
//! the continuation itself: a `ret` is a link only when the run leading to it
//! stored the address of the `ret`'s OWN fall-through as a constant.  An ordinary
//! epilogue never pushes the address of the instruction after itself, and a real
//! `call` pushes its own fall-through and is not a `ret`, so neither matches.

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
    /// Falls through to the next address (including a conditional branch and a
    /// real `call`, both of which reach it).
    FallThru,
    /// An indirect branch or an indirect call: the walk cannot follow it.
    Opaque,
}

/// A varnode identity within one decoded instruction, for the literal-tracking
/// map: SLEIGH reaches a `push` of a literal through two `COPY`s
/// (`tmp:4 = imm32; mysave:4 = tmp; *:4 ESP = mysave`), so the value operand of
/// the `STORE` is a unique, never the constant itself.
type VnKey = (i32, uintb, u32);

/// A [`PcodeEmit`] that records, for one decoded instruction, the literals it
/// stores to memory and how it leaves.
#[derive(Default)]
struct ChainScan {
    consts: Vec<uintb>,
    literals: BTreeMap<VnKey, uintb>,
    lastop: Option<OpCode>,
    jump: Option<Address>,
    call: bool,
}

impl ChainScan {
    /// True when the varnode names the constant space.
    fn is_const(v: &VarnodeData) -> bool {
        v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
    }

    fn key(v: &VarnodeData) -> Option<VnKey> {
        v.space.as_ref().map(|s| (s.get_index(), v.offset, v.size))
    }

    /// The literal `v` holds: itself when constant, else the one a `COPY`
    /// earlier in this instruction put there.
    fn literal(&self, v: &VarnodeData) -> Option<uintb> {
        if Self::is_const(v) {
            return Some(v.offset);
        }
        Self::key(v).and_then(|k| self.literals.get(&k).copied())
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
        match opc {
            // STORE(spaceid, pointer, value) — a `push` of a literal.
            OpCode::CPUI_STORE => {
                if let Some(c) = vars.get(2).and_then(|v| self.literal(v)) {
                    self.consts.push(c);
                }
            }
            // The literal hand-off SLEIGH's `push` macros go through.
            OpCode::CPUI_COPY => {
                let (Some(out), Some(src)) = (outvar, vars.first()) else { return };
                let Some(k) = Self::key(out) else { return };
                match self.literal(src) {
                    Some(c) => self.literals.insert(k, c),
                    None => self.literals.remove(&k),
                };
            }
            OpCode::CPUI_BRANCH => {
                self.jump = vars.first().and_then(Self::code_dest);
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => self.call = true,
            _ => {}
        }
    }
}

/// One decoded instruction's chain-relevant shape.
struct Decoded {
    len: int4,
    consts: Vec<uintb>,
    leave: Leave,
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
    // A real `call` stores its own return address; that constant must not be
    // read as a pushed continuation by a later `ret` in the same run.
    let consts = if scan.call { Vec::new() } else { std::mem::take(&mut scan.consts) };
    Some(Decoded { len, consts, leave })
}

/// The `ret`s of `entry`'s body that are RET-call links, in flow order.
///
/// The walk is a straight-line follow from `entry`: it takes an unconditional
/// direct branch, falls through everything else (a conditional branch and a real
/// `call` both reach their fall-through), and stops at an indirect branch, a
/// decode failure, an address it already decoded, or the first `ret` that is not
/// a link.  It carries the constants each instruction stores forward and clears
/// them at every link, so the test at a `ret` asks only about the run that
/// reached it.
fn chain_of(
    trans: &dyn Translate,
    entry: &Address,
    max_sites: usize,
    max_insns: usize,
) -> Vec<Address> {
    let mut sites: Vec<Address> = Vec::new();
    let mut consts: Vec<uintb> = Vec::new();
    let mut seen: BTreeSet<uintb> = BTreeSet::new();
    let mut cur = entry.clone();
    for _ in 0..max_insns {
        if !seen.insert(cur.get_offset()) {
            break;
        }
        let Some(d) = decode_one(trans, &cur) else { break };
        let next = &cur + d.len as i64;
        consts.extend(d.consts);
        match d.leave {
            Leave::Return => {
                if !consts.contains(&next.get_offset()) {
                    break; // a genuine return: the chain ends here
                }
                sites.push(cur.clone());
                if sites.len() >= max_sites {
                    break;
                }
                consts.clear();
                cur = next;
            }
            Leave::Jump(target) => cur = target,
            Leave::FallThru => cur = next,
            Leave::Opaque => break,
        }
    }
    sites
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
