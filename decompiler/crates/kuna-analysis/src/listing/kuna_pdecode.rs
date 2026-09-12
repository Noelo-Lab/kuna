//! (kuna `--jobs`) The discovery walk on N decode lanes: address-interval
//! ownership with bulk-synchronous reconciliation.
//!
//! One rule, enforced by construction: **a lane decodes only the addresses it
//! owns**. The sorted seed list is cut into `32 x lanes` address intervals;
//! every address in the program belongs to exactly one interval; a successor
//! that falls outside a lane's interval is handed to the interval that owns it
//! instead of being decoded locally. Exactly-once decode is preserved with no
//! shared claim set, no shared visited map and no per-address atomic, and the
//! reconciliation is a union over disjoint, address-ordered shards.
//!
//! The per-instruction body is [`super::walk::step`] -- the same one the serial
//! walk drives -- reached through a [`Successors`] sink that routes by owner.
//! There is no second copy of the decode policy to drift.
//!
//! # Why the output is the serial walk's, byte for byte
//!
//! Under the gate ([`admit`]) `decode_one(a)` is a pure function of (address,
//! image bytes, context values): the loaded `.sla` declares no `ContextCommit`,
//! so no decode can write the context database, and the image precondition
//! makes the loader's 512-byte staging window unobservable. The serial walk's
//! output is the least fixpoint of a monotone system over a finite lattice
//! (seeds subset F, admitted CALL targets subset F, F subset I, successors
//! subset I); the lanes evaluate the same equations, each address by exactly one
//! lane, with fair delivery at every barrier, so chaotic iteration converges to
//! the same fixpoint. The two first-writer-wins lines in
//! [`super::walk`] are neutralised: the decoded test has no race because
//! ownership is total, and the function claim inserts a constant record over a
//! map whose seeds were pre-inserted. Nothing downstream sees the schedule:
//! every consumer iterates address-ordered maps.
//!
//! Anything the gate refuses runs the serial walk, which is byte-identical by
//! definition.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::panic::AssertUnwindSafe;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_decodekit::{build_decode_engine_traced, EngineRecipe};
use kuna_sleigh::kuna_ctxsnapshot::{snapshot_context, ContextValueSnapshot};
use kuna_sleigh::kuna_sharedbytes::UnmappedTripwire;
use kuna_sleigh::loadimage::ImageBytes;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::Translate;

use super::context::ContextPainter;
use super::decode::decode_one;
use super::model::{DiscoveredFunction, Insn, InsnLite, Reference};
use super::walk::{
    self, CallbackEvidence, CallbackSink, InsnSink, RefBuckets, StepCtx, Successors, WalkPolicy,
    WalkState,
};
use super::ListingDetail;

/// How many decode lanes the walk should use. Set by `kuna --jobs N` for the
/// duration of the load; absent means the serial walk.
pub const DECODE_JOBS_ENV: &str = "KUNA_DECODE_JOBS";
/// Lower the [`Refusal::TooSmall`] floor (tests set `0`).
pub const DECODE_MIN_BYTES_ENV: &str = "KUNA_DECODE_MIN_BYTES";
/// Override the intervals-per-lane cut (measurement / tests).
pub const DECODE_INTERVALS_ENV: &str = "KUNA_DECODE_INTERVALS";
/// `1` compares the lanes against the serial walk and returns the serial
/// result; `abort` panics on a difference.
pub const DECODE_SELFCHECK_ENV: &str = "KUNA_DECODE_SELFCHECK";
/// `1` prints the lane/interval/round/crossing/decode counts.
pub const DECODE_STATS_ENV: &str = "KUNA_DECODE_STATS";
/// Test-only: panic once inside the numbered lane, to exercise the fallback.
pub const DECODE_FAULT_ENV: &str = "KUNA_DECODE_FAULT";

/// The lane cap. Measured throughput knee on a 40-core box; deliberately
/// distinct from the worker pool's `MAX_AUTO_JOBS`, because a decode lane costs
/// ~48 MB and a pool worker costs a whole program load.
pub const MAX_DECODE_LANES: usize = 32;

/// Intervals per lane. At `32` the largest interval held 1.02% of a 20.2 M
/// instruction program, so the dynamic-claim makespan tail is under a second;
/// `128` bought 0.2 points more.
const INTERVALS_PER_LANE: usize = 32;

/// Executable bytes below which two engine builds would dominate the walk.
const MIN_EXEC_BYTES: u64 = 8 * 1024 * 1024;

/// Addresses the lane-0 kit re-decodes against the parent before anything is
/// spawned.
const PROBE_DECODES: usize = 1024;

/// A runaway guard, not a policy: the round count is bounded by the longest
/// chain of interval-crossing discovered-function edges (measured 2 on a 147 MB
/// x86-64 binary, 8 on a seed-starved one).
const MAX_ROUNDS: usize = 1024;

/// Differences the self-check prints before it stops listing them.
const SELFCHECK_REPORT: usize = 20;

/// Why the walk ran serially. Every variant has a stable stderr spelling; the
/// first group is the gate, the second is a fault after the lanes started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The target has no threads (wasm).
    NoThreads,
    /// No rebuildable engine: not a standalone `Sleigh`, or no `.sla` bytes.
    NoEngine,
    /// The language's constructors carry `globalset`, so a decode at one
    /// address can change how another decodes.
    ContextCommits,
    /// A delay-slot decode fetches past its own instruction.
    DelaySlots,
    /// The loader cannot share its (live, patched) bytes.
    NoSharedBytes,
    /// A per-address decode mode is painted into the context database.
    ContextPaint,
    /// Nothing to walk.
    NoSeeds,
    /// Some executable address is mapped by no segment, which is the one case
    /// in which the loader's staging window is history-dependent.
    UnmappedExec,
    /// The walk is too small to pay for the engine builds.
    TooSmall,
    /// A lane's engine could not be built.
    KitFailed,
    /// A rebuilt engine disagreed with the parent on a sampled decode.
    KitDisagrees,
    /// A lane panicked.
    LaneFault,
    /// A lane fetched bytes at an unmapped address.
    UnmappedFetch,
    /// Two shards claimed one address (a router bug).
    Collision,
    /// The rounds did not converge.
    RoundLimit,
    /// The parent's context database moved during the walk.
    ContextMoved,
}

impl Refusal {
    /// The stderr spelling, stable across releases.
    pub fn reason(self) -> &'static str {
        match self {
            Refusal::NoThreads => "no threads on this target",
            Refusal::NoEngine => "no rebuildable decode engine",
            Refusal::ContextCommits => "language commits context",
            Refusal::DelaySlots => "language has delay slots",
            Refusal::NoSharedBytes => "loader cannot share its bytes",
            Refusal::ContextPaint => "per-address decode context",
            Refusal::NoSeeds => "no seeds",
            Refusal::UnmappedExec => "executable range not fully mapped",
            Refusal::TooSmall => "executable image too small",
            Refusal::KitFailed => "engine rebuild failed",
            Refusal::KitDisagrees => "rebuilt engine disagrees",
            Refusal::LaneFault => "lane fault",
            Refusal::UnmappedFetch => "unmapped fetch",
            Refusal::Collision => "merge collision",
            Refusal::RoundLimit => "round limit",
            Refusal::ContextMoved => "context moved",
        }
    }
}

/// The rebuild instructions every lane's engine is built from, captured once per
/// load and reused across every `Listing` rebuild.
#[derive(Debug)]
struct DecodeSeed {
    recipe: EngineRecipe,
    bytes: Arc<dyn ImageBytes>,
    ctx: ContextValueSnapshot,
}

/// How many lanes the walk may use, and what to build them from.
///
/// Built once where the engine is (kuna-console's commit path) and threaded
/// through [`super::Listing::build_with_meta_planned`], so the callback and ARM
/// re-walks reuse the same recipe, image share and context snapshot instead of
/// re-deriving them.
#[derive(Debug)]
pub struct WalkPlan {
    lanes: usize,
    intervals_per_lane: usize,
    kit: Option<DecodeSeed>,
    declined: Option<Refusal>,
    /// The last line announced, so a plan that drives several rebuilds says it
    /// once.
    announced: Mutex<Option<String>>,
}

impl WalkPlan {
    /// The serial plan: one lane, nothing to build, nothing announced.
    pub fn serial() -> WalkPlan {
        WalkPlan {
            lanes: 1,
            intervals_per_lane: INTERVALS_PER_LANE,
            kit: None,
            declined: None,
            announced: Mutex::new(None),
        }
    }

    /// The plan [`DECODE_JOBS_ENV`] asks for.
    pub fn from_env(arch: &Architecture) -> WalkPlan {
        let lanes = env_usize(DECODE_JOBS_ENV).unwrap_or(1).min(MAX_DECODE_LANES);
        let per_lane = env_usize(DECODE_INTERVALS_ENV)
            .filter(|&n| n > 0)
            .unwrap_or(INTERVALS_PER_LANE);
        WalkPlan::for_lanes(arch, lanes, per_lane)
    }

    /// A plan for exactly `lanes` lanes and `intervals_per_lane` intervals each.
    ///
    /// Everything that can be decided without the walk's own inputs is decided
    /// here: whether the engine can be rebuilt at all, whether the language's
    /// decode can write context, and the image share the lanes read through.
    pub fn for_lanes(arch: &Architecture, lanes: usize, intervals_per_lane: usize) -> WalkPlan {
        let mut plan = WalkPlan::serial();
        if lanes < 2 {
            return plan;
        }
        plan.lanes = lanes;
        plan.intervals_per_lane = intervals_per_lane.max(1);
        if cfg!(target_family = "wasm") {
            plan.declined = Some(Refusal::NoThreads);
            return plan;
        }
        let Some(sleigh) = arch.translate().as_sleigh() else {
            plan.declined = Some(Refusal::NoEngine);
            return plan;
        };
        if sleigh.base().has_context_commits() {
            plan.declined = Some(Refusal::ContextCommits);
            return plan;
        }
        if sleigh.base().max_delay_slot_bytes() != 0 {
            plan.declined = Some(Refusal::DelaySlots);
            return plan;
        }
        let Some(recipe) = arch.decode_recipe() else {
            plan.declined = Some(Refusal::NoEngine);
            return plan;
        };
        let Some(bytes) = arch.translate().loader_rc().borrow().shared_bytes() else {
            plan.declined = Some(Refusal::NoSharedBytes);
            return plan;
        };
        let Some(space) = arch.manage().get_default_code_space().map(Rc::clone) else {
            plan.declined = Some(Refusal::NoEngine);
            return plan;
        };
        let ctx = arch.with_context_db_mut(|db| snapshot_context(db, &space));
        plan.kit = Some(DecodeSeed { recipe, bytes, ctx });
        plan
    }

    /// The requested lane count (`1` = the serial walk).
    pub fn lanes(&self) -> usize {
        self.lanes
    }

    /// Print `line` unless it is the one this plan printed last.
    fn announce(&self, line: String) {
        let mut last = self.announced.lock().unwrap_or_else(|e| e.into_inner());
        if last.as_deref() == Some(line.as_str()) {
            return;
        }
        eprintln!("{line}");
        *last = Some(line);
    }
}

/// Everything the parallel walk reads that the plan does not carry.
pub(super) struct ParallelInputs<'a> {
    pub arch: &'a Architecture,
    pub translate: &'a dyn Translate,
    pub code_space: &'a Rc<AddrSpace>,
    pub exec_ranges: &'a [(u64, u64)],
    pub seeds: &'a [u64],
    pub seed_funcs: &'a BTreeMap<u64, DiscoveredFunction>,
    pub painter: &'a ContextPainter,
    pub local_entries: &'a BTreeMap<u64, u64>,
    pub policy: WalkPolicy,
    pub detail: ListingDetail,
}

/// Run the walk on `plan.lanes()` lanes, or `None` if anything declines.
///
/// Announces the decision on stderr exactly once per plan, so a load that
/// rebuilds the `Listing` (the callback generations, the ARM re-seeds) does not
/// repeat itself.
pub(super) fn try_parallel(plan: &WalkPlan, inputs: &ParallelInputs<'_>) -> Option<WalkState> {
    let started = std::time::Instant::now();
    match admit(plan, inputs) {
        Ok(admitted) => {
            plan.announce(format!(
                "[kuna --jobs] decode: {} lanes, {} intervals",
                admitted.lanes,
                admitted.intervals.len()
            ));
            match run(&admitted, inputs, started) {
                Ok(state) => Some(state),
                Err(why) => {
                    plan.announce(format!("[kuna --jobs] decode: serial ({})", why.reason()));
                    None
                }
            }
        }
        Err(why) => {
            plan.announce(format!("[kuna --jobs] decode: serial ({})", why.reason()));
            None
        }
    }
}

/// Is the self-check on, and does a difference abort?
pub(super) fn selfcheck() -> Option<bool> {
    let v = std::env::var(DECODE_SELFCHECK_ENV).ok()?;
    match v.trim() {
        "" | "0" | "off" | "false" => None,
        "abort" => Some(true),
        _ => Some(false),
    }
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.trim().parse::<usize>().ok()
}

// --- the partition -----------------------------------------------------------

/// Ownership map: interval `i` owns `[bounds[i], bounds[i+1])`, and the last
/// owns everything from its low bound up.
#[derive(Debug)]
struct Intervals {
    bounds: Vec<u64>,
}

impl Intervals {
    /// Cut the (sorted, deduped) seed list into at most `k` intervals.
    ///
    /// The cut points are seed addresses, so the seeds -- and with them the bulk
    /// of the program -- spread evenly over the intervals whatever the image
    /// layout is. Duplicate cut points collapse, so the result is strictly
    /// ascending and may be shorter than `k`.
    fn new(seeds: &[u64], k: usize) -> Intervals {
        let m = seeds.len();
        let mut bounds: Vec<u64> = Vec::with_capacity(k + 1);
        bounds.push(0);
        for i in 1..k.max(1) {
            let at = seeds[i * m / k];
            if at > *bounds.last().expect("bounds starts non-empty") {
                bounds.push(at);
            }
        }
        if *bounds.last().expect("bounds starts non-empty") != u64::MAX {
            bounds.push(u64::MAX);
        }
        Intervals { bounds }
    }

    fn len(&self) -> usize {
        self.bounds.len() - 1
    }

    /// Which interval owns `a`.
    fn owner(&self, a: u64) -> usize {
        let i = match self.bounds.binary_search(&a) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        i.min(self.len() - 1)
    }
}

/// A successor handed to the interval that owns it.
#[derive(Debug, Clone, Copy)]
enum Crossing {
    /// A same-path successor (branch target or fall-through).
    Insn(u32, u64),
    /// A claimed function entry.
    Func(u32, u64),
}

impl Crossing {
    fn owner(self) -> u32 {
        match self {
            Crossing::Insn(o, _) | Crossing::Func(o, _) => o,
        }
    }
}

/// The decoded records of one interval: a flat, append-only list plus the
/// membership test [`super::walk::step`] asks.
///
/// Flat rather than a `BTreeMap` because the merge wants one sorted run per
/// interval and the shard wants 112 bytes a record rather than a B-tree node's
/// share of 298.
#[derive(Default)]
struct LaneInsns {
    seen: HashSet<u64>,
    out: Vec<(u64, InsnLite)>,
}

impl InsnSink for LaneInsns {
    fn decoded(&self, vma: u64) -> bool {
        self.seen.contains(&vma)
    }
    fn record(&mut self, insn: InsnLite) {
        self.seen.insert(insn.addr);
        self.out.push((insn.addr, insn));
    }
}

/// One interval's uncapped callback evidence, keyed by target.
///
/// Uncapped deliberately: [`super::walk::CallbackEvidence`]'s cap is
/// order-independent over one evidence set, but capping each shard and merging
/// the survivors is not the same as capping the union once. The reconcile feeds
/// the union through a single `CallbackEvidence`, which is.
#[derive(Default)]
struct LaneCallbacks {
    refs: BTreeMap<u64, u64>,
}

impl CallbackSink for LaneCallbacks {
    fn file(&mut self, source: u64, target: u64) {
        self.refs.entry(target).and_modify(|p| *p = (*p).min(source)).or_insert(source);
    }
}

/// Everything one interval accumulates. Every field is plain data.
struct IntervalState {
    insns: LaneInsns,
    funcs: BTreeMap<u64, DiscoveredFunction>,
    refs: RefBuckets,
    callbacks: LaneCallbacks,
    /// The outer worklist's `visited_funcs`, per interval. Complete, because an
    /// entry belongs to exactly one interval.
    visited: BTreeSet<u64>,
    /// Inbox: function roots delivered for this interval.
    roots: Vec<u64>,
    /// Inbox: instruction addresses delivered for this interval.
    resume: Vec<u64>,
}

impl IntervalState {
    fn new(want_refs: bool) -> IntervalState {
        IntervalState {
            insns: LaneInsns::default(),
            funcs: BTreeMap::new(),
            refs: RefBuckets::new(want_refs),
            callbacks: LaneCallbacks::default(),
            visited: BTreeSet::new(),
            roots: Vec::new(),
            resume: Vec::new(),
        }
    }
}

// A lane hands its whole state back across a thread boundary, so assert that at
// compile time rather than trusting the field list to stay plain data.
const _: fn() = || {
    fn send<T: Send>() {}
    send::<IntervalState>();
};

/// Routes each successor to its owner: in-interval successors stay on the lane's
/// worklists, everything else becomes a [`Crossing`].
struct Router<'a> {
    lo: u64,
    hi: u64,
    last: bool,
    ivs: &'a Intervals,
    out: &'a mut Vec<Crossing>,
    local_insn: Vec<u64>,
    local_func: Vec<u64>,
}

impl Router<'_> {
    #[inline]
    fn owns(&self, a: u64) -> bool {
        a >= self.lo && (a < self.hi || self.last)
    }
}

impl Successors for Router<'_> {
    #[inline]
    fn insn(&mut self, vma: u64) {
        if self.owns(vma) {
            self.local_insn.push(vma);
        } else {
            self.out.push(Crossing::Insn(self.ivs.owner(vma) as u32, vma)); // cast: < K
        }
    }
    #[inline]
    fn func(&mut self, entry: u64) {
        if self.owns(entry) {
            self.local_func.push(entry);
        } else {
            self.out.push(Crossing::Func(self.ivs.owner(entry) as u32, entry)); // cast: < K
        }
    }
}

/// Run one interval to local quiescence, appending its crossings to `out`.
fn run_interval(
    iv: usize,
    ivs: &Intervals,
    st: &mut IntervalState,
    ctx: &StepCtx<'_>,
    out: &mut Vec<Crossing>,
    decodes: &mut usize,
) {
    let mut router = Router {
        lo: ivs.bounds[iv],
        hi: ivs.bounds[iv + 1],
        last: iv + 1 == ivs.len(),
        ivs,
        out,
        local_insn: Vec::new(),
        local_func: Vec::new(),
    };
    loop {
        if let Some(entry) = st.roots.pop().or_else(|| router.local_func.pop()) {
            if !st.visited.insert(entry) {
                continue; // already walked this function
            }
            router.local_insn.push(entry);
        } else if let Some(vma) = st.resume.pop() {
            router.local_insn.push(vma);
        } else if router.local_insn.is_empty() {
            break;
        }
        while let Some(vma) = router.local_insn.pop() {
            if walk::step(
                ctx,
                vma,
                &mut st.insns,
                &mut st.funcs,
                &mut st.refs,
                &mut st.callbacks,
                &mut router,
            ) {
                *decodes += 1;
            }
        }
    }
}

// --- the gate ----------------------------------------------------------------

/// An admitted plan: the lanes, the interval cut and the claim order.
pub(super) struct Admitted<'a> {
    lanes: usize,
    seed: &'a DecodeSeed,
    intervals: Intervals,
    /// Round-1 claim order: interval indices by seed count descending (LPT).
    order: Vec<usize>,
}

/// Evaluate the whole gate. A refusal runs the serial walk.
fn admit<'a>(plan: &'a WalkPlan, inputs: &ParallelInputs<'_>) -> Result<Admitted<'a>, Refusal> {
    if let Some(why) = plan.declined {
        return Err(why);
    }
    let seed = plan.kit.as_ref().ok_or(Refusal::NoEngine)?;
    // The decode-mode paint is a per-address context the snapshot cannot carry.
    // Belt and braces: every language that paints one also commits context.
    if !inputs.painter.is_empty() {
        return Err(Refusal::ContextPaint);
    }
    if inputs.seeds.is_empty() {
        return Err(Refusal::NoSeeds);
    }
    let exec_bytes: u64 = inputs.exec_ranges.iter().map(|&(lo, hi)| hi.saturating_sub(lo)).sum();
    let floor = env_usize(DECODE_MIN_BYTES_ENV).map(|n| n as u64).unwrap_or(MIN_EXEC_BYTES);
    if exec_bytes < floor {
        return Err(Refusal::TooSmall);
    }
    // The image precondition. `fill_span` reports failure only when the FIRST
    // byte of a span is unmapped, and that is the one case in which a lane's own
    // staging window can answer differently from the parent's: a window that
    // happens to cover the address serves zeroes where a fresh fill errors, and
    // on x86 `00 00` is a valid instruction. Every fetch the walk makes starts
    // at an `in_exec` address, so with every executable address mapped the
    // window is unobservable.
    for &(lo, hi) in inputs.exec_ranges {
        if hi > lo && !seed.bytes.mapped_covers(lo, hi) {
            return Err(Refusal::UnmappedExec);
        }
    }

    let intervals = Intervals::new(inputs.seeds, plan.lanes * plan.intervals_per_lane);
    let mut counts = vec![0usize; intervals.len()];
    for &s in inputs.seeds {
        counts[intervals.owner(s)] += 1;
    }
    let mut order: Vec<usize> = (0..intervals.len()).filter(|&i| counts[i] > 0).collect();
    order.sort_by(|&a, &b| counts[b].cmp(&counts[a]).then(a.cmp(&b)));

    Ok(Admitted { lanes: plan.lanes, seed, intervals, order })
}

// --- the lanes ---------------------------------------------------------------

/// A lane's engine, its own code space and its unmapped-fetch tripwire.
struct Kit {
    engine: Sleigh,
    space: Rc<AddrSpace>,
    tripwire: UnmappedTripwire,
}

/// A barrier that can be cancelled.
///
/// `std::sync::Barrier` would hang the surviving lanes when one panics and never
/// arrives, which is the opposite of a fallback. A poisoned gate wakes every
/// waiter and refuses every future arrival.
struct Gate {
    state: Mutex<GateState>,
    signal: Condvar,
    parties: usize,
}

struct GateState {
    count: usize,
    generation: u64,
    poisoned: bool,
}

impl Gate {
    fn new(parties: usize) -> Gate {
        Gate {
            state: Mutex::new(GateState { count: 0, generation: 0, poisoned: false }),
            signal: Condvar::new(),
            parties,
        }
    }

    /// Wait for every lane. `false` means the gate was poisoned: abandon the walk.
    fn wait(&self) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if state.poisoned {
            return false;
        }
        let generation = state.generation;
        state.count += 1;
        if state.count == self.parties {
            state.count = 0;
            state.generation = state.generation.wrapping_add(1);
            self.signal.notify_all();
            return true;
        }
        while state.generation == generation && !state.poisoned {
            state = self.signal.wait(state).unwrap_or_else(|e| e.into_inner());
        }
        !state.poisoned
    }

    fn poison(&self) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.poisoned = true;
        self.signal.notify_all();
    }
}

/// What the round loop agreed on at the last barrier.
#[derive(Default)]
struct Round {
    active: Vec<usize>,
    delivered: usize,
}

/// A lane's own counters, folded into the stats line.
#[derive(Default, Clone, Copy)]
struct LaneStats {
    decodes: usize,
    crossings: usize,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// Build the lanes, run the rounds, and reconcile the shards.
fn run(
    admitted: &Admitted<'_>,
    inputs: &ParallelInputs<'_>,
    started: std::time::Instant,
) -> Result<WalkState, Refusal> {
    let ivs = &admitted.intervals;
    let lanes = admitted.lanes;

    // One kit on this thread first: if the rebuilt engine disagrees with the
    // parent anywhere in a 1,024-address sample, decline before any thread is
    // spawned. The probe's kit becomes lane 0's, so the check is free.
    let kit0 = build_kit(admitted.seed)?;
    probe(&kit0, inputs)?;

    let states: Vec<Mutex<IntervalState>> =
        (0..ivs.len()).map(|_| Mutex::new(IntervalState::new(inputs.detail.refs))).collect();
    for &s in inputs.seeds {
        lock(&states[ivs.owner(s)]).roots.push(s);
    }

    let gate = Gate::new(lanes);
    let cursor = AtomicUsize::new(0);
    let sort_cursor = AtomicUsize::new(0);
    let delivered = AtomicUsize::new(0);
    let round = Mutex::new(Round { active: admitted.order.clone(), delivered: usize::MAX });
    let rounds = AtomicUsize::new(0);
    let total_decodes = AtomicUsize::new(0);
    let total_crossings = AtomicUsize::new(0);
    let fault_lane = env_usize(DECODE_FAULT_ENV);
    let fault_armed = AtomicBool::new(fault_lane.is_some());

    // The lane closure may capture only what can cross a thread boundary: the
    // `Architecture`, the parent's translator and its `Rc<AddrSpace>` stay on
    // this thread, and each lane builds its own engine and space instead.
    let seed = admitted.seed;
    let exec_ranges = inputs.exec_ranges;
    let local_entries = inputs.local_entries;
    let policy = inputs.policy;
    let detail = inputs.detail;

    let body = |me: usize, kit: Option<Kit>| -> Result<bool, Refusal> {
        let kit = match kit {
            Some(k) => k,
            None => build_kit(seed)?,
        };
        let ctx = StepCtx {
            translate: &kit.engine,
            code_space: &kit.space,
            exec_ranges,
            local_entries,
            policy,
            detail,
        };
        if !gate.wait() {
            return Err(Refusal::LaneFault);
        }

        let mut out: Vec<Crossing> = Vec::new();
        let mut stats = LaneStats::default();
        let mut active = lock(&round).active.clone();
        let mut seen_rounds = 0usize;
        loop {
            if fault_lane == Some(me) && fault_armed.swap(false, Ordering::SeqCst) {
                panic!("KUNA_DECODE_FAULT: lane {me} injected fault");
            }
            // Production: claim intervals from the cursor and run each to local
            // quiescence. The cursor hands each interval to exactly one lane per
            // round, so the lock is uncontended here; it is the delivery lock.
            loop {
                let at = cursor.fetch_add(1, Ordering::Relaxed);
                let Some(&iv) = active.get(at) else { break };
                let mut st = lock(&states[iv]);
                run_interval(iv, ivs, &mut st, &ctx, &mut out, &mut stats.decodes);
            }
            if !gate.wait() {
                return Err(Refusal::LaneFault);
            }

            // Delivery: bucket this lane's crossings into the owners' inboxes.
            // Sorted by owner so each inbox is locked once, not once a crossing.
            stats.crossings += out.len();
            out.sort_unstable_by_key(|c| c.owner());
            let mut at = 0usize;
            while at < out.len() {
                let owner = out[at].owner();
                let mut st = lock(&states[owner as usize]); // cast: an interval index
                while at < out.len() && out[at].owner() == owner {
                    match out[at] {
                        Crossing::Insn(_, vma) => st.resume.push(vma),
                        Crossing::Func(_, entry) => st.roots.push(entry),
                    }
                    at += 1;
                }
            }
            delivered.fetch_add(out.len(), Ordering::Relaxed);
            out.clear();
            if !gate.wait() {
                return Err(Refusal::LaneFault);
            }

            // Bookkeeping, by one lane, between two barriers: nothing else is
            // touching an interval here.
            if me == 0 {
                let moved = delivered.swap(0, Ordering::Relaxed);
                let mut r = lock(&round);
                r.delivered = moved;
                r.active.clear();
                if moved > 0 {
                    let mut next: Vec<(usize, usize)> = Vec::new();
                    for (i, s) in states.iter().enumerate() {
                        let st = lock(s);
                        let pending = st.roots.len() + st.resume.len();
                        if pending > 0 {
                            next.push((i, pending));
                        }
                    }
                    next.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                    r.active = next.into_iter().map(|(i, _)| i).collect();
                }
                cursor.store(0, Ordering::Relaxed);
                rounds.fetch_add(1, Ordering::Relaxed);
            }
            if !gate.wait() {
                return Err(Refusal::LaneFault);
            }

            let r = lock(&round);
            if r.delivered == 0 {
                break;
            }
            active = r.active.clone();
            drop(r);
            seen_rounds += 1;
            if seen_rounds > MAX_ROUNDS {
                return Err(Refusal::RoundLimit);
            }
        }

        // Sort the shards while the lanes are still up: the merge wants one
        // ascending run per interval, and 512 small sorts beat one big one.
        loop {
            let at = sort_cursor.fetch_add(1, Ordering::Relaxed);
            if at >= states.len() {
                break;
            }
            lock(&states[at]).insns.out.sort_unstable_by_key(|&(vma, _)| vma);
        }

        total_decodes.fetch_add(stats.decodes, Ordering::Relaxed);
        total_crossings.fetch_add(stats.crossings, Ordering::Relaxed);
        Ok(kit.tripwire.get())
    };

    // Every lane body is caught: a lane that dies must poison the gate, or the
    // survivors block at a barrier nobody will ever reach.
    let guarded = |me: usize, kit: Option<Kit>| -> Result<bool, Refusal> {
        match std::panic::catch_unwind(AssertUnwindSafe(|| body(me, kit))) {
            Ok(Ok(tripped)) => Ok(tripped),
            Ok(Err(why)) => {
                gate.poison();
                Err(why)
            }
            Err(_) => {
                gate.poison();
                Err(Refusal::LaneFault)
            }
        }
    };

    let outcome = std::thread::scope(|scope| {
        let guarded = &guarded;
        let mut handles = Vec::with_capacity(lanes - 1);
        for me in 1..lanes {
            handles.push(scope.spawn(move || guarded(me, None)));
        }
        // This thread is lane 0, and already holds the probe's kit.
        let mut tripped = false;
        let mut failure: Option<Refusal> = None;
        match guarded(0, Some(kit0)) {
            Ok(t) => tripped |= t,
            Err(why) => failure = Some(why),
        }
        for handle in handles {
            match handle.join() {
                Ok(Ok(t)) => tripped |= t,
                Ok(Err(why)) => failure = failure.or(Some(why)),
                Err(_) => failure = failure.or(Some(Refusal::LaneFault)),
            }
        }
        if let Some(why) = failure {
            Err(why)
        } else if tripped {
            Err(Refusal::UnmappedFetch)
        } else {
            Ok(())
        }
    });
    outcome?;

    // Nothing under the gate may write the parent's context database.
    let after = inputs.arch.with_context_db_mut(|db| snapshot_context(db, inputs.code_space));
    if after != admitted.seed.ctx {
        return Err(Refusal::ContextMoved);
    }

    let walk_time = started.elapsed();
    let merge_started = std::time::Instant::now();
    let shards: Vec<IntervalState> =
        states.into_iter().map(|m| m.into_inner().unwrap_or_else(|e| e.into_inner())).collect();
    let state = reconcile(shards, inputs.seeds, inputs.seed_funcs)?;

    if env_usize(DECODE_STATS_ENV).is_some_and(|n| n > 0) {
        eprintln!(
            "[kuna --jobs] decode stats: lanes={} intervals={} rounds={} crossings={} \
             decodes={} walk={:.2}s merge={:.2}s",
            lanes,
            ivs.len(),
            rounds.load(Ordering::Relaxed),
            total_crossings.load(Ordering::Relaxed),
            total_decodes.load(Ordering::Relaxed),
            walk_time.as_secs_f64(),
            merge_started.elapsed().as_secs_f64(),
        );
    }
    Ok(state)
}

fn build_kit(seed: &DecodeSeed) -> Result<Kit, Refusal> {
    let (engine, space, tripwire) =
        build_decode_engine_traced(&seed.recipe, Arc::clone(&seed.bytes), &seed.ctx)
            .map_err(|_| Refusal::KitFailed)?;
    Ok(Kit { engine, space, tripwire })
}

/// Re-decode a sample of the seeds on `kit` and compare against the parent.
///
/// A rebuilt engine that is not decode-equivalent produces silently different
/// bytes, not an error, so this runs before anything is spawned.
fn probe(kit: &Kit, inputs: &ParallelInputs<'_>) -> Result<(), Refusal> {
    let seeds = inputs.seeds;
    let stride = (seeds.len() / PROBE_DECODES).max(1);
    for &vma in seeds.iter().step_by(stride).take(PROBE_DECODES) {
        let want = decode_one(
            inputs.translate,
            vma,
            inputs.code_space,
            inputs.detail.assembly,
            inputs.policy.stack_callbacks,
        );
        let got = decode_one(
            &kit.engine,
            vma,
            &kit.space,
            inputs.detail.assembly,
            inputs.policy.stack_callbacks,
        );
        match (want, got) {
            (Err(_), Err(_)) => {}
            (Ok(a), Ok(b)) => {
                if a.len != b.len
                    || a.mnemonic != b.mnemonic
                    || a.operands != b.operands
                    || a.stored_scalar_values != b.stored_scalar_values
                {
                    return Err(Refusal::KitDisagrees);
                }
                let (ca, cb) = (
                    super::classify::classify(&a.ops, vma, a.len),
                    super::classify::classify(&b.ops, vma, b.len),
                );
                if ca.flow != cb.flow || ca.flows != cb.flows || ca.fall_through != cb.fall_through
                {
                    return Err(Refusal::KitDisagrees);
                }
            }
            _ => return Err(Refusal::KitDisagrees),
        }
    }
    if kit.tripwire.get() {
        return Err(Refusal::UnmappedFetch);
    }
    Ok(())
}

// --- the merge ---------------------------------------------------------------

/// Union the shards back into one [`WalkState`].
///
/// Every step consumes ascending keys of disjoint domains, so the result does
/// not depend on the order the lanes ran in.
fn reconcile(
    shards: Vec<IntervalState>,
    seeds: &[u64],
    seed_funcs: &BTreeMap<u64, DiscoveredFunction>,
) -> Result<WalkState, Refusal> {
    let mut insns: BTreeMap<u64, Insn> = BTreeMap::new();
    let mut funcs: BTreeMap<u64, DiscoveredFunction> = BTreeMap::new();
    let mut refs_to: BTreeMap<u64, Vec<Reference>> = BTreeMap::new();
    let mut refs_from: BTreeMap<u64, Vec<Reference>> = BTreeMap::new();
    let mut callbacks: BTreeMap<u64, u64> = BTreeMap::new();

    // The seed pass first, so a seeded entry keeps its name/from_symbol even
    // when a CALL also targeted it -- exactly the serial walk's pre-population.
    for &entry in seeds {
        let df = seed_funcs.get(&entry).cloned().unwrap_or_else(|| walk::discovered(entry));
        funcs.entry(entry).or_insert(df);
    }

    let mut previous: Option<u64> = None;
    for shard in shards {
        for (vma, lite) in shard.insns.out {
            // The shards are disjoint by construction, so this can only fire on
            // a router or owner() bug -- in which case the parallel result is
            // discarded rather than silently missing an instruction.
            if previous.is_some_and(|p| vma <= p) {
                return Err(Refusal::Collision);
            }
            previous = Some(vma);
            insns.insert(vma, Insn::from(lite));
        }
        for (entry, record) in shard.funcs {
            funcs.entry(entry).or_insert(record);
        }
        let (to, from) = shard.refs.into_parts();
        for (key, bucket) in to {
            refs_to.entry(key).or_default().extend(bucket);
        }
        for (key, bucket) in from {
            refs_from.entry(key).or_default().extend(bucket);
        }
        for (target, source) in shard.callbacks.refs {
            callbacks.entry(target).and_modify(|p| *p = (*p).min(source)).or_insert(source);
        }
    }

    // One capped pass over the union, which is what the serial walk ran.
    let mut evidence = CallbackEvidence::default();
    for (target, source) in callbacks {
        evidence.file(source, target);
    }

    Ok(WalkState {
        insns,
        refs_to,
        refs_from,
        funcs,
        stack_callback_refs: evidence.into_refs(),
    })
}

// --- the self-check ----------------------------------------------------------

/// Compare a parallel [`WalkState`] against the serial one field by field,
/// printing the first [`SELFCHECK_REPORT`] differences. Returns the count.
pub(super) fn compare(parallel: &WalkState, serial: &WalkState) -> usize {
    let mut differences = 0usize;
    let mut report = |line: String| {
        differences += 1;
        if differences <= SELFCHECK_REPORT {
            eprintln!("[kuna --jobs] selfcheck: {line}");
        }
    };

    let mut keys: BTreeSet<u64> = parallel.insns.keys().copied().collect();
    keys.extend(serial.insns.keys().copied());
    for vma in keys {
        match (parallel.insns.get(&vma), serial.insns.get(&vma)) {
            (Some(a), Some(b)) => {
                if a.len != b.len
                    || a.fall_through != b.fall_through
                    || a.flow != b.flow
                    || a.flows != b.flows
                    || a.mnemonic != b.mnemonic
                    || a.operands != b.operands
                {
                    report(format!("insn {vma:#x} differs"));
                }
            }
            (Some(_), None) => report(format!("insn {vma:#x} only in the lanes")),
            (None, Some(_)) => report(format!("insn {vma:#x} only in the serial walk")),
            (None, None) => {}
        }
    }

    let mut entries: BTreeSet<u64> = parallel.funcs.keys().copied().collect();
    entries.extend(serial.funcs.keys().copied());
    for entry in entries {
        if parallel.funcs.get(&entry) != serial.funcs.get(&entry) {
            report(format!("function {entry:#x} differs"));
        }
    }

    for (name, a, b) in [
        ("refs_to", &parallel.refs_to, &serial.refs_to),
        ("refs_from", &parallel.refs_from, &serial.refs_from),
    ] {
        let by_source = name == "refs_to";
        let mut left = a.clone();
        let mut right = b.clone();
        super::finalize_refs(&mut left, by_source);
        super::finalize_refs(&mut right, by_source);
        let mut buckets: BTreeSet<u64> = left.keys().copied().collect();
        buckets.extend(right.keys().copied());
        for key in buckets {
            let (l, r) = (left.get(&key), right.get(&key));
            let same = match (l, r) {
                (Some(l), Some(r)) => {
                    l.len() == r.len()
                        && l.iter()
                            .zip(r.iter())
                            .all(|(x, y)| x.from == y.from && x.to == y.to && x.kind == y.kind)
                }
                (None, None) => true,
                _ => false,
            };
            if !same {
                report(format!("{name}[{key:#x}] differs"));
            }
        }
    }

    if parallel.stack_callback_refs != serial.stack_callback_refs {
        report("stack callback evidence differs".to_string());
    }

    if differences == 0 {
        eprintln!("[kuna --jobs] selfcheck: 0 differences over the whole walk");
    } else {
        eprintln!("[kuna --jobs] selfcheck: {differences} differences");
    }
    differences
}

#[cfg(test)]
mod tests {
    use super::super::walk::{FuncSink, RefSink};
    use super::*;

    fn ivs(seeds: &[u64], k: usize) -> Intervals {
        Intervals::new(seeds, k)
    }

    #[test]
    fn owner_is_total_and_agrees_with_the_bounds() {
        let seeds = [0x1000u64, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000, 0x8000];
        for k in [1usize, 2, 3, 4, 8, 16, 64] {
            let iv = ivs(&seeds, k);
            assert_eq!(iv.bounds[0], 0, "k={k}: the first bound covers address 0");
            assert_eq!(*iv.bounds.last().unwrap(), u64::MAX, "k={k}: the map is total");
            assert!(iv.bounds.windows(2).all(|w| w[0] < w[1]), "k={k}: bounds ascend strictly");
            // Every bound, one below it, one above it, plus the extremes.
            let mut probes: Vec<u64> = vec![0, 1, u64::MAX - 1, u64::MAX];
            for &b in &iv.bounds {
                probes.extend([b.saturating_sub(1), b, b.saturating_add(1)]);
            }
            probes.extend(seeds.iter().copied());
            for a in probes {
                let owner = iv.owner(a);
                assert!(owner < iv.len(), "k={k}: owner({a:#x}) = {owner} is out of range");
                assert!(
                    a >= iv.bounds[owner],
                    "k={k}: owner({a:#x}) = {owner} starts above the address"
                );
                assert!(
                    a < iv.bounds[owner + 1] || owner + 1 == iv.len(),
                    "k={k}: owner({a:#x}) = {owner} ends below the address"
                );
            }
        }
    }

    #[test]
    fn more_intervals_than_seeds_collapses_instead_of_splitting_nothing() {
        let iv = ivs(&[0x400u64, 0x800], 64);
        assert_eq!(iv.bounds, vec![0, 0x400, 0x800, u64::MAX]);
        assert_eq!(iv.owner(0), 0);
        assert_eq!(iv.owner(0x3ff), 0);
        assert_eq!(iv.owner(0x400), 1);
        assert_eq!(iv.owner(0x800), 2);
        assert_eq!(iv.owner(u64::MAX), 2);
    }

    #[test]
    fn a_single_interval_owns_everything() {
        let iv = ivs(&[7u64], 1);
        assert_eq!(iv.len(), 1);
        for a in [0u64, 7, 0x1000, u64::MAX] {
            assert_eq!(iv.owner(a), 0);
        }
    }

    #[test]
    fn the_router_keeps_what_it_owns_and_hands_over_the_rest() {
        let iv = ivs(&[0x1000u64, 0x2000, 0x3000, 0x4000], 4);
        let mid = 1usize;
        let mut out = Vec::new();
        let mut router = Router {
            lo: iv.bounds[mid],
            hi: iv.bounds[mid + 1],
            last: mid + 1 == iv.len(),
            ivs: &iv,
            out: &mut out,
            local_insn: Vec::new(),
            local_func: Vec::new(),
        };
        let inside = iv.bounds[mid];
        router.insn(inside);
        router.func(inside);
        router.insn(0);
        router.func(u64::MAX);
        assert_eq!(router.local_insn, vec![inside]);
        assert_eq!(router.local_func, vec![inside]);
        assert_eq!(out.len(), 2, "both out-of-interval successors crossed");
        assert!(matches!(out[0], Crossing::Insn(0, 0)));
        assert!(matches!(out[1], Crossing::Func(o, u64::MAX) if o as usize == iv.len() - 1));
    }

    fn seed_record(entry: u64, name: &str) -> DiscoveredFunction {
        DiscoveredFunction {
            entry,
            name: Some(name.to_string()),
            from_symbol: true,
            has_no_return: false,
            call_fixup: None,
        }
    }

    fn lite(addr: u64, len: u32) -> InsnLite {
        InsnLite {
            addr,
            len,
            fall_through: Some(addr + u64::from(len)),
            flow: Default::default(),
            flows: Vec::new(),
            mnemonic: String::new(),
            operands: String::new(),
        }
    }

    #[test]
    fn a_seed_record_beats_a_discovered_one_and_duplicates_collapse() {
        let mut a = IntervalState::new(true);
        a.funcs.claim(0x1000);
        a.funcs.claim(0x2000);
        let mut b = IntervalState::new(true);
        b.funcs.claim(0x1000); // the same entry, discovered from another lane
        b.funcs.claim(0x3000);

        let seeds = [0x1000u64];
        let mut seed_funcs = BTreeMap::new();
        seed_funcs.insert(0x1000, seed_record(0x1000, "main"));

        let st = reconcile(vec![a, b], &seeds, &seed_funcs).expect("no collision");
        assert_eq!(st.funcs.len(), 3);
        assert_eq!(
            st.funcs[&0x1000].name.as_deref(),
            Some("main"),
            "the seed record wins over a discovered one, whichever lane found it"
        );
        assert!(!st.funcs[&0x2000].from_symbol);
        assert!(!st.funcs[&0x3000].from_symbol);
    }

    #[test]
    fn the_merge_refuses_two_shards_that_claim_one_address() {
        let mut a = IntervalState::new(true);
        a.insns.record(lite(0x1000, 4));
        let mut b = IntervalState::new(true);
        b.insns.record(lite(0x1000, 4));
        let merged = reconcile(vec![a, b], &[], &BTreeMap::new());
        assert!(
            matches!(merged, Err(Refusal::Collision)),
            "two shards claiming one address must discard the parallel result"
        );
    }

    #[test]
    fn merged_references_equal_a_pre_merged_walk() {
        use super::super::model::RefKind;
        let edges = [
            (0x1000u64, 0x2000u64, RefKind::Call),
            (0x1004, 0x2000, RefKind::Code),
            (0x2000, 0x2004, RefKind::Code),
            (0x1000, 0x2000, RefKind::Call), // a duplicate the finalize must drop
            (0x3000, 0x2000, RefKind::Call),
        ];
        // One walk that filed everything...
        let mut single = RefBuckets::new(true);
        for &(from, to, kind) in &edges {
            single.file(from, to, kind);
        }
        let (mut want_to, mut want_from) = single.into_parts();
        super::super::finalize_refs(&mut want_to, true);
        super::super::finalize_refs(&mut want_from, false);

        // ...against three shards that each filed a slice of it.
        let mut shards: Vec<IntervalState> =
            (0..3).map(|_| IntervalState::new(true)).collect();
        for (i, &(from, to, kind)) in edges.iter().enumerate() {
            shards[i % 3].refs.file(from, to, kind);
        }
        let st = reconcile(shards, &[], &BTreeMap::new()).expect("no collision");
        let (mut got_to, mut got_from) = (st.refs_to, st.refs_from);
        super::super::finalize_refs(&mut got_to, true);
        super::super::finalize_refs(&mut got_from, false);

        let flat = |m: &BTreeMap<u64, Vec<Reference>>| {
            m.iter()
                .map(|(k, v)| (*k, v.iter().map(|r| (r.from, r.to, r.kind)).collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        };
        assert_eq!(flat(&got_to), flat(&want_to));
        assert_eq!(flat(&got_from), flat(&want_from));
    }

    #[test]
    fn callback_evidence_survives_the_split_and_the_cap() {
        let cap = super::super::kuna_callbackentry::MAX_CALLBACK_EVIDENCE;
        let count = cap + 64;
        // One walk that saw every site.
        let mut want = CallbackEvidence::default();
        for i in 0..count as u64 {
            want.file(0x9000 + i, 0x1000 + i);
            want.file(0x8000 + i, 0x1000 + i); // a lower source for the same target
        }
        // Four shards, each with a slice, merged then capped once.
        let mut shards: Vec<IntervalState> =
            (0..4).map(|_| IntervalState::new(true)).collect();
        for i in 0..count as u64 {
            shards[(i % 4) as usize].callbacks.file(0x9000 + i, 0x1000 + i);
            shards[((i + 1) % 4) as usize].callbacks.file(0x8000 + i, 0x1000 + i);
        }
        let st = reconcile(shards, &[], &BTreeMap::new()).expect("no collision");
        assert_eq!(st.stack_callback_refs.len(), cap, "the union is capped once");
        assert_eq!(st.stack_callback_refs, want.into_refs());
    }

    #[test]
    fn a_poisoned_gate_wakes_every_waiter() {
        let gate = Arc::new(Gate::new(3));
        let woke = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let gate = Arc::clone(&gate);
            let woke = Arc::clone(&woke);
            handles.push(std::thread::spawn(move || {
                let ok = gate.wait();
                woke.fetch_add(1, Ordering::SeqCst);
                ok
            }));
        }
        // The third party never arrives; it panicked and poisoned instead.
        while lock(&gate.state).count < 2 {
            std::thread::yield_now();
        }
        gate.poison();
        for handle in handles {
            assert!(!handle.join().expect("the waiter returned"), "a poisoned wait must fail");
        }
        assert_eq!(woke.load(Ordering::SeqCst), 2);
        assert!(!gate.wait(), "a poisoned gate refuses every later arrival");
    }

    #[test]
    fn every_refusal_has_a_distinct_spelling() {
        let all = [
            Refusal::NoThreads,
            Refusal::NoEngine,
            Refusal::ContextCommits,
            Refusal::DelaySlots,
            Refusal::NoSharedBytes,
            Refusal::ContextPaint,
            Refusal::NoSeeds,
            Refusal::UnmappedExec,
            Refusal::TooSmall,
            Refusal::KitFailed,
            Refusal::KitDisagrees,
            Refusal::LaneFault,
            Refusal::UnmappedFetch,
            Refusal::Collision,
            Refusal::RoundLimit,
            Refusal::ContextMoved,
        ];
        let spellings: BTreeSet<&str> = all.iter().map(|r| r.reason()).collect();
        assert_eq!(spellings.len(), all.len(), "a reason must name exactly one refusal");
    }
}
