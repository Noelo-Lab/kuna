# ADR 0006: Stage registry — stages.toml as single source of truth, build.rs codegen

## Context

kuna's stage model lives in `kuna_stages.cc` as four hand-maintained static
tables: 42 stage groups, 37 substages, ~60 surfaces, and 22 settable
assertions. The Python control surface (`kuna.catalog`) consumes the `stage
catalog` JSON those tables emit, and `tests/stages/kuna-catalog.xml`
stringmatches the **raw** emitter output byte-for-byte. In C++ the tables and
the emitter sit in one file; in Rust, hand-written static arrays would drift
from documentation just as easily.

## Decision

The four tables move to `rust/crates/kuna-decomp/stages.toml` as the single
source of truth. A `build.rs` codegen step reads it and generates: the static
tables, typed `OptionValues` (each settable becomes a typed field with its
enum of allowed values), and the live-value `match` that maps a settable name
to its current value. The catalog JSON emitter is **hand-written** to be
byte-compatible with `kunaEmitSettableJson` — same key order, same spacing,
same escaping — because `tests/stages/kuna-catalog.xml` stringmatches the raw
output. No serde for that output path (serde's formatting choices are not
contractual and would silently break the stringmatch).

## Consequences

- One edit point: adding a group/substage/settable is a TOML row, and the
  tables, typed options, and catalog all update together — codegen makes
  drift a build failure instead of a doc bug. ADR 0007's rule that a new
  `PassRegistration` group REQUIRES a stages.toml row hangs off this.
- The hand-written emitter is verified by the existing datatest; any future
  format change must be made deliberately against that golden.
- `build.rs` adds a TOML parse dependency at build time only; runtime stays
  static data.
- Typed `OptionValues` turn invalid option values into compile-time/parse-time
  errors instead of string comparisons scattered through the engine.
