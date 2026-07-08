# kuna → IDA Pro parity roadmap

Tracks kuna's decompiled-output gaps against **IDA Pro 9.2** (reference oracle:
`tests/x86_64/decompiler/fmt`, function `main`; captured via the `decompiler`/idalib CLI).
Started from a `fmt/main` diff after the register↔global render fix.

## Landed (merged to `main`)

| PR | What | IDA gap closed |
|---|---|---|
| #149 | Render register/`unique` members of a global-merged high as the global (no stray `EAX`/`Unique<hex>`) | raw-register variables |
| #151 | Name the program's DWARF data globals (`dat_215120` → `max_width`, `goal_width`, `prefix*`) | unnamed globals (DWARF) |
| #152 | `opflags_for` `INT_EQUAL` regression (from #150) — was erroring functions in `decompile-all` | (regression) |
| #153 | No-return: a conditional jump to a `.cold abort` still returns via fall-through (`quotearg_style` error path restored) | over-aggressive no-return |
| #154 | Reconcile the #149 regmerge test witness with #151's naming | (test) |

## Proposed (this directory — go/no-go)

Ranked by value × tractability. Default policy for all: **default-on, no flag** (clearly-correct
matches to IDA), except where a bring-up `--option` gate is flagged.

| Proposal | Gap | Effort | Value | Notes |
|---|---|---|---|---|
| [`ida-libc-extern-globals`](ida-libc-extern-globals/proposal.md) | `dat_20a098` → `optind`/`stdin`/`stdout`/`optarg` | **Small–Med** | High | `.dynsym`/`.symtab` `STT_OBJECT` reader; reuses #151's install path |
| [`ida-dwarf-prototypes`](ida-dwarf-prototypes/proposal.md) | `undefined16 main(uint4,void*)` → `int main(int argc, char **argv)` | **Med** | High | applies DWARF `DW_TAG_subprogram` return+params; also fixes #2 for DWARF binaries |
| [`ida-string-ref-recovery`](ida-string-ref-recovery/proposal.md) | `bindtextdomain(0x68d8,…)` → `"coreutils"` | **Med** | Med | `char*` typing for **mid-string** readonly-char pointers |
| [`ida-dwarf-enums`](ida-dwarf-enums/proposal.md) | `quotearg_style(4,…)` → `quoting_style::…` | **Med** | Low–Med | DWARF enum types → existing `push_enum_constant` render; best after prototypes |
| [`ida-return-storage-inference`](ida-return-storage-inference/proposal.md) | `undefined16` return + `char[16]` phantom → `int` | **Large** | High | general/stripped return-storage recovery (core dataflow); high blast radius |
| [`ida-inlined-libc-idioms`](ida-inlined-libc-idioms/proposal.md) | inlined `cmpsb`/`df` loop → `strcmp(v19,"-")` | **Large (multi-week)** | Med–High | idiom-recognition pass + `df` folding; staged, per-recognizer gate |

### Suggested order

1. `ida-libc-extern-globals` (finishes the naming story from #151)
2. `ida-dwarf-prototypes` (biggest signature win; subsumes #2 for DWARF)
3. `ida-string-ref-recovery`
4. `ida-dwarf-enums`
5. `ida-return-storage-inference` — approve step-1 investigation first
6. `ida-inlined-libc-idioms` — approve Phase 0 (`df` folding) first

## Where kuna already beats IDA

Char literals: kuna renders `'-'`/`' '`/`'0'` where IDA emits decimal `45`/`32`/`48`.

## Tooling note

IDA reference capture: see the `kuna-ida-declib-setup` steps — corrected `IDA_PATH`
(`/home/mahaloz/ctf/tools/idapro_9.2/idat`), `idalib` activation + `PYTHONPATH`, then
`decompiler decompile main --backend ida`.
