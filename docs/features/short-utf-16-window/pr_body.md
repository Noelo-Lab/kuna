## What was broken

The round-10 `ObfuscationFiesta.exe` witness declared
`data 0x1400074b0 wchar_t window_class[3]` and a `FindWindowW` prototype. Both
assertions reported `applied`, but the decompiler still emitted `v128 = "I";`
and later cast that narrow local to `wchar_t *`.

## Mechanism

Operand-reference analysis had already planted an exact-address `char[2]`
mapping after reading the low byte and high-byte NUL of short UTF-16 `ID`. The
later explicit declaration added a second overlapping `wchar_t[3]` symbol, so
ordinary smallest-container lookup kept selecting the inferred narrow object.

Caller-declared data now replaces an existing non-function mapping at the same
exact address. It retypes, resizes, and renames that symbol in place before the
assertion applies its type and name locks. Both the in-process assertion plane
and the `map address` console lowering use this seam. No global string encoding
heuristic, length minimum, phase-9 emission rule, or `printc` behavior changes.

## Evidence

Canonical acceptance `a-abec8c6e76cf` for probe `p-47b3ce664b23` passes on a
checked-in PE32+ fixture. The fixture deliberately reproduces the initial
`char[2]` mapping and includes adjacent UTF-16 `OLLYDBG` and ASCII `ASCII`
controls. Independent database, in-process assertion, stage, and CLI tests pin
the same precedence rule.

The exact dataset witness now emits `v128 = L"ID";` and
`FindWindowW(v128,NULL);`; both strict assertions remain reported as applied.
Without a data assertion, candidate output is byte-identical to main. An
explicit `char[2]` declaration stays narrow, and the neighboring wide and ASCII
literals retain their original spelling.

The durable need closes in round 12, attempt 1, on PR #593. The authoritative
index preserves all 197 active and 1 rejected records, including the PR
#590/#591/#592 closures. Its only semantic changes are this target record and
the aggregate transition from 138 closed / 58 open to 139 closed / 57 open;
the single blocked record is unchanged. A sparse reindex was deliberately not
used.

## Gates

- `make test`: 675/675, parity OK
- `make test-stages`: 801/801, parity OK
- `make test-cli`: 146/146
- full Rust workspace suite: green on the reviewed implementation head
- focused post-rebase database and assertion-plane tests: green
- strict spec, catalog, acceptance, and merge guards: green

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
