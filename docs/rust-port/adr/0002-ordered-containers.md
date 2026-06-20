# ADR 0002: Ordered containers only — BTreeMap/BTreeSet with transcribed comparators

## Context

The C++ decompiler's output is a function of container iteration order:
`Funcdata` keeps varnodes in `std::set` with custom comparators, symbol scopes
and many caches are `std::map`, and the printed C ultimately reflects those
orders. Rust's `HashMap`/`HashSet` iterate in random order per process, which
would make the Rust engine non-deterministic and impossible to diff against
the C++ oracle.

## Decision

Use `BTreeMap`/`BTreeSet` with `Ord` keys that transcribe the C++ comparators
**field by field**. That includes `VarnodeCompareLocDef`
(`varnode.cc:34-79`) exactly — the `(f1-1) < (f2-1)` trick that wraps the
free-class flag so "input" sorts before "free", and the final `create_index`
tiebreak — and `SeqNum`'s order for op sets. `HashMap`/`HashSet` are banned
workspace-wide via clippy `disallowed_types = "deny"` wired to
`rust/clippy.toml`. Any allowlist exception requires a verifier-approved
comment at the use site proving the iteration order never escapes (never
reaches output, IR order, or error text).

## Consequences

- Deterministic engine output by construction; golden diffs against the C++
  oracle are meaningful from day one.
- Comparator transcription bugs become ordering diffs in goldens — visible,
  bisectable — instead of latent nondeterminism.
- O(log n) lookups where C++ also paid O(log n) (`std::map`/`std::set` are
  trees); the few C++ `unordered_map`s get BTree semantics, a slowdown we
  accept until a profiled, allowlisted exception proves itself out.
- The `create_index` tiebreak means insertion counters must be ported
  faithfully wherever C++ relies on them.
