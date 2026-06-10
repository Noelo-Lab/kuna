# Rust port verifier protocol

This is the review protocol for every `verify` item in
`docs/rust-port/checklist.json`. It is normative: a wave gate does not pass
until every port item in the wave has a verdict file produced under this
protocol.

## Structural separation

**Programmer != verifier, structurally.** The agent (or person) who ported an
item never writes its verdict. The verifier receives ONLY:

1. the C++ source in scope (the item's `modules` at the recorded
   `cpp_blob_sha`),
2. the Rust diff for the item,
3. the gate output (the item's `gate` command, run fresh),
4. the checklist entry itself (title, notes, declared losses).

The verifier does **not** receive the porter's reasoning, chat transcript, or
self-assessment. If the diff doesn't stand on its own against the C++ source,
that is itself a finding.

## The mandatory hunt list

Faithful-port bugs cluster in known places. Every review MUST explicitly hunt
each of these and say so in the verdict (a checked-off list, with anchors for
anything suspicious):

- **Signedness.** The C++ builds with `-Wno-sign-compare`: signed/unsigned
  comparisons that C++ silently allows must have been resolved deliberately in
  Rust. Hunt every comparison whose C++ operands were `int4` vs `uint4`/
  `uintb`, and every `int4` loop index against a container size.
- **Integer widths.** `int4`/`uint4`/`intb`/`uintb`/`uintm` mappings; implicit
  C++ integer promotions (e.g. `int4 * int4` overflowing into `intb`) that Rust
  makes explicit — verify the port chose the C++ result, not the "fixed" one.
- **Wrapping.** C++ unsigned arithmetic wraps silently; Rust panics in debug.
  Every arithmetic op on `uintb`-lineage values needs `wrapping_*`/`Wrapping`
  or a proof it cannot wrap. A `wrapping_add` that should be `checked_add` is
  equally a finding.
- **Comparator totality.** Every ported `operator<`/comparator must be a total
  order (no `partial_cmp().unwrap()` on floats, no inconsistent tie-breaks);
  BTree/sort misbehavior from a non-total comparator is silent output
  corruption.
- **Iteration-order provenance, per loop.** For each loop over a container,
  state where the C++ order came from (std::map key order, list insertion
  order, vector index) and how the Rust side reproduces it. Any `HashMap`/
  `HashSet` in ported logic is an automatic finding.
- **Off-by-one / do-while / reverse iterators.** C++ `do {} while` has no Rust
  analogue (`loop`+break ports must keep the at-least-once semantics);
  `rbegin()/rend()` and `--end()` idioms; `lower_bound`/`upper_bound`
  boundary inclusion.
- **Erase-while-iterating.** Upstream constantly erases from lists/maps during
  traversal with carefully sequenced iterator increments (`it = c.erase(it)` vs
  `c.erase(it++)`). The Rust translation (collect-then-remove, retain, index
  rewind) must provably visit the same elements in the same order.
- **Exception -> Result partial-state parity.** Where C++ throws
  (`LowlevelError` etc.) mid-mutation, the surrounding state is left partially
  modified and callers sometimes rely on that (restart machinery, recover
  paths). Verify the `?`-propagation points leave the same partial state, or
  that a cleanup the port added matches an upstream catch site.

## Mechanical pass

Run, and attach output to the verdict:

- `cargo run -p port-audit -- --item <item-id>` (blob-sha drift + per-item
  checks),
- grep the item's Rust code for `todo!`, `unimplemented!`, `HashMap`,
  `HashSet`, `sort_unstable`, and bare `as` casts (each bare `as` either
  carries a `// cast:` justification comment or is a finding),
- `cargo clippy -p <crate> -- -D warnings` clean.

## Adversarial pass

The verifier writes **>= 3 new tests** targeting the spots the hunt list
flagged as most fragile for this item (boundary inputs, empty containers,
wrap-prone values, order-sensitive traversals). These tests land with the
verdict (in the item's crate, marked with the item id) regardless of verdict;
a REJECT must include at least one failing test or a concrete divergence
trace against the C++ oracle.

## Verdict file

One file per item: `docs/rust-port/reviews/<item>.md`, where `<item>` is the
**port** item id. Format:

```
# <item-id>
verdict: ACCEPT | ACCEPT-WITH-LOSSES | REJECT
verifier: <agent/person>
date: YYYY-MM-DD
gate: <command run> -> <result>
hunt list: <one line per hunt-list entry: clean, or finding ref>
findings:
  - F1 (<severity>): <description>
       cpp: decompiler/cpp/<file>:<line>
       rust: rust/<crate>/<file>:<line>
  ...
adversarial tests: <list of test names added>
losses: <LOSS-ids cited, for ACCEPT-WITH-LOSSES>
```

Every finding carries both anchors (cpp + rust). `ACCEPT-WITH-LOSSES` requires
each accepted divergence to be a cited `losses.md` entry (append a new one if
needed — the verifier may append, never edit). A REJECT sends the item back to
`todo` with `attempts` incremented and findings as the work order.

## Attempt budget

**Max 3 attempts** (port -> REJECT cycles) per item. On the third REJECT the
item's status becomes `blocked` and it is flagged for a human decision in
`status.md`; no further automated attempts.
