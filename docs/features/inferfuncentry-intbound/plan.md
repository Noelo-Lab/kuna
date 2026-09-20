# Plan

Strict fix, no new option. `inferfuncentry` keeps its meaning (on = the GH-6930
escape exists, off = upstream); what changes is which readers the escape is
offered to.

1. `p5_types/kuna_inferfuncentry.rs`: `reads_as_integer(opcode)` (ordering
   comparison, multiply, divide, remainder, shift) and
   `entry_escape_applies(opcode, value_reads_as_integer)`.
2. `p9_emit/coreaction_render.rs`: `ActionConstantPtr::apply` builds the set of
   numerically-read constant values once per pass from the snapshot it already
   takes, and `is_pointer` consults the predicate at the one place the
   `bit_transitions < 3` rejection is skipped. No other call site exists.
3. `tests/stages/kuna-inferfuncentry-intbound.xml`: `handler` at `0x100000`
   (`1 << 20`), `bound` computing `MIN (n, 0x100000)`, `reg` registering
   `handler` as a callback, `dispatch` selecting between `handler` and `setcb`.
   Five assertions: the bounded value keeps its integer type, its bound prints as
   a number, the callback argument still prints the name, a COPY of the same
   address in a body that never orders it still prints the name, and
   `option inferfuncentry off` still returns the argument to a literal.
4. Three unit tests on the predicates.
5. `docs/spec/05-types.md`: the escape's paragraph says what the escape is
   offered to and why.

## Acceptance

* `make test` 675/675, `make test-stages` PARITY OK, `make rust-test`,
  `make check-spec` (+ `--strict`), `make test-cli`, `kuna catalog --check`,
  `counters --check`, `docs/options.md` byte-fresh.
* 444-slice typesweep (coreutils grep gzip diffutils bzip2 findutils tar shadow,
  O0/O2/O2-noinline, pinned metric): perfect at least main's, no row worse.
* Whole-binary `decompile-all` before/after over 16 builds, every hunk
  classified; no `STATEMENT` or `OTHER` hunk left unread.
* Speed within +5%, interleaved min-of-15.
