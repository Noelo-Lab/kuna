# Plan — `dedupvardecls` (collapse identical duplicate local declarations)

## Feature
An S9 emit-time filter: when enabled, `emit_local_var_decls` skips any local
declaration whose **fully-rendered signature** (final declarator type + name + array
adornment + storage comment) is byte-identical to one already emitted. This is the
scalar analogue of the existing composite-symbol collapse (`printc.rs:2020-2034`),
approximating Ghidra's once-per-Symbol `ScopeLocal` walk.

## Scope: SMALL (decider-confirmed)
- 1 new module (`s9_emit/kuna_dedupvardecls.rs`).
- 3 ported-core anchor files: `printc.rs` (gated skip), `architecture.rs` (flag + seam +
  dispatch), `options.rs` (`KUNA_OPTION_NAMES`). At the gate ceiling, not over.
- No S7 / region code. No new pass type. Modelable as one gated early-return.

## Mechanism
- New `bool` arch flag `dedup_var_decls`, default-off, copied into the ArchSeam next to
  `name_style_angr` (the printer reads the seam `arch`).
- New module owns `OptionDedupVarDecls::apply` (on/off parse) and a `DeclDedup` helper
  tracking seen rendered-declaration signatures.
- `emit_local_var_decls`: build a `DeclDedup` when `arch.dedup_var_decls`; in the emit
  loop, after computing `decl_type` / `array_count` / `comment`, skip the entry if its
  signature was already seen. Off ⇒ byte-identical output.

## Dedup key (decider-recommended)
Full rendered signature `(decl_type, name, array_adornment, storage_comment)`. By-name
would wrongly merge distinct same-named locals differing in type/storage; the full
signature suppresses a line ONLY when emitted bytes are character-identical — provably
lossless (and removes strictly-invalid C re-declarations).

## Plumbing
- `stages.toml`: `[[settable]]` row `dedupvardecls` (S9, naming-policy / local-decl-walk),
  `source_decompiler="angr"`, `change_kind="presentation-default"`, `live_field`.
- `kuna_stages/tests.rs`: bump settable count 37→38; with_live 20→21.
- ElementId 4091 (next free after 4090 `hideextensions`).

## Test
`tests/stages/ghangr-x8664-cvs-863633.xml`: a self-contained x86-64 chunk that produces a
duplicated scalar local; pass 1 (`option dedupvardecls off`) asserts the duplicate
appears ≥2×; pass 2 (default/on) asserts exactly 1.

## Default decision
Implement default-OFF. Run the 675-assertion ablation + speed gate (Phase 5). If
ablation is 0/675 and speed within budget → flip default-ON + DIV entry. Else opt-in.

## Decider verdict (verbatim recorded in record.json `decisions`).
