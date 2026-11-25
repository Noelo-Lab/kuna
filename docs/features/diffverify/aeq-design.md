# diffverify — AEQ: laws across functions (design only)

Fission RFC #1101 proposes two research tracks. One is AEQ: tell a queue from a stack, a set
or a ring buffer by the algebraic laws its operations obey (`push(a); push(b); pop() == b`)
rather than by struct offsets. Its CSIC track is implemented in `intervene.rs`
(`csic-results.md`). This note says what AEQ would add to `kuna-verify` and what it would
need. Nothing here is built. Nothing uses Fission's code: only the idea is reused.

## 1. What it would add

`kuna-verify` checks one function at a time, from a random state. It already compares
every byte a function writes outside its frame. So a law check over kuna's side cannot
find a difference that the per-function check would miss *on the same state*. What it can
change is **which states are checked**:

- **Reachable object states.** A random state rarely satisfies an object's invariants
  (`count <= capacity`, `head` and `tail` inside the buffer). Runs from such a state trap,
  spend the budget, or take paths a real program never takes. A sequence of the object's
  own operations from an empty object reaches the states that matter.
- **Faults that show only through a second call.** A store to the wrong field is caught
  today because the bytes differ. A fault that leaves the bytes right but the protocol
  wrong is not, for example an off-by-one in a wrap that only goes wrong on the sixteenth
  `push`. A sequence reaches that call. A single random state almost never does.
- **A summary an agent can read.** "kuna's `pop` breaks LIFO after `push(a); push(b)`" says
  more about a function cluster than one differing byte does.

## 2. The rule: a law is a hypothesis about the machine, never about kuna

A law is checked on kuna's side only after the machine shows it holds. The checker never
assumes that an object is a stack or a queue.

1. **Find a cluster.** From machine runs, find functions that read or write through the
   same pointer argument, at overlapping offsets from it. Call those that write through it
   *mutators* and those that only read *observers*. Do not use kuna's types for this,
   since they are what is being checked.
2. **Build states on the machine.** Start from a zeroed object, or run a function that
   only writes it (a constructor). Then apply random sequences of mutators, at most four
   calls long.
3. **Test candidate laws on the machine.** Try LIFO, FIFO, idempotence (`op(a); op(a)`
   leaves the same state as `op(a)`, as far as the observers can see), commutativity for
   distinct keys, and inverse pairs (`push; pop` leaves the state as it was). A law that
   any machine sequence breaks is *refuted*, and the breaking sequence is kept. A law that
   no sequence breaks is *empirically held*. It is never "proven": the sequences are
   random and bounded. A sequence where the machine traps or returns an error marks a
   precondition (pop on empty, push on full), not a refutation.
4. **Replay on kuna's side.** Run the same sequences with kuna's side of each function,
   state carried from call to call. Compare each call as `check` does today. A `DIFFERS`
   names the sequence, the call, and the byte or value that differs.
5. **Report the law only as context.** "LIFO held on the machine over 64 sequences; kuna's
   side breaks it at call 3" goes in the report. The verdict still comes from comparing a
   concrete call on a state the machine reached. This is the same rule as concolic inputs
   (z3 picks inputs, the run decides) and as `intervene` (a witness, never a cause).

Following this rule, AEQ cannot give a false `DIFFERS` that `check` would not also give on
that state. A wrong law, such as calling a FIFO a LIFO, costs only states that were not
useful.

## 3. What `kuna-verify` would need

| need | today | change |
|---|---|---|
| state carried across calls | `State::new(env, run)` makes a fresh state per run | a run that takes a state and returns it; one machine state and one kuna state, each carried through the sequence |
| several functions at once | `Env::fd` holds one `Funcdata`, and the session hands each one to a callback and drops it | keep each function's compiled `Program`, its `Calls`, and the parts of `Env` that are per function (`extensions`, `tracked`, `booleans`, the frame extents read from `fd`). Swap them in for each call |
| arguments from earlier results | arguments come from the random state | write each argument to its storage (`abi`), whether it is the object pointer, a fresh value, or an earlier call's return value |
| an object in memory | memory outside the frame is derived from the seed, so it is random | an object region whose bytes start as zero or as a constructor leaves them, and which stubs may not touch |
| footprints | `State` logs writes, and `FrameReads` notes frame reads | log reads and writes relative to one pointer argument, to find clusters |
| observational equality | runs compare bytes | run the cluster's observers on both states and compare what they return |
| budgets | per-run op budget | a cap on sequence length and count per law, reported with the result |

## 4. Verdicts it would report

`EMPIRICALLY_HELD(n sequences)`, `REFUTED(sequence)`, `PRECONDITION(sequence, trap)`,
`UNSUPPORTED(reason)` for each law on the machine. Then the existing `AGREES`, `DIFFERS`
and `UNKNOWN` for kuna's side over the sequences. No `PROVED`: a bounded z3 query over a
sequence (C5's machinery) could at most say no counterexample exists within its bound, and
it would be reported as that.

## 5. Why it is not built yet

- **Cost.** The two state changes (§3, first two rows) touch `state.rs`, `check.rs` and the
  session, which are the most shared parts of the crate. Each needs the full verdict
  comparison over the corpus, `ls` and `find` that the crate rules require.
- **Few targets in the corpus.** Datatests decompile single functions. They rarely hold a
  cluster that works on one object. AEQ needs whole binaries: the decbench set, `ls`,
  `find`.
- **Precision first.** The RFC sets cluster precision below 50% as a stop trigger. The
  first experiment should therefore be the clustering alone (§2, step 1) on decbench
  binaries, checked by hand against the source, before any law is tested.
