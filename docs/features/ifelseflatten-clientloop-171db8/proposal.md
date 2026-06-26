# [PROPOSAL] angr-ifelseflatten-clientloop-171db8 — back the ET_REL synthetic extern area with zero-filled readable memory

**Status:** draft proposal — needs human go/no-go before an implementation worker is spent.

**Opportunity:** `test_ifelseflatten_clientloop :: client_request_tun_fwd`
**Binary:** `clientloop.o` (x86_64 ELF **ET_REL**), selector `client_request_tun_fwd` @ `0x405170`.

## The problem

The opportunity is **mislabeled**. It is not an if-else flattening / structuring gap. **kuna
produces zero output** for `client_request_tun_fwd`: it hard-fails in the **loader** with
`Unable to load 512 bytes at r0x0040cb60`, before any S2–S9 decompiler pass runs. angr decompiles
the function fully. Full root-cause in [`analysis.md`](analysis.md) and side-by-side in
[`angr-vs-kuna.txt`](angr-vs-kuna.txt).

**Mechanism.** The function reads an **undefined external data global**:
`cmpl $0x1, options+0x1373` (`options` is ELF `SHN_UNDEF`/`STT_NOTYPE`). kuna's ET_REL loader
(`kuna-analysis/src/s1_loader/elf_reloc.rs:282`) gives each undefined extern only a **16-byte
synthetic slot with no readable backing segment**. The slots work as named *call targets*
(functions resolved by name, no bytes read), but a *data* read at `options+0x1373` (≈`0x40cb60`)
lands in unbacked memory → `loadimage_object.rs:597` "initial address not mapped" → abort.

## angr reference

angr's CLE loader backs its **extern object** with a **zero-filled, readable** region, so reads
of undefined-extern globals return 0 and decompilation proceeds. This is the existing
`relocobjects` (DIV-7) loader feature's missing complement: `relocobjects` binds extern *call
targets*; it does not back extern *data* reads.

## Why this is out of scope for one decompiler Action/Rule (Hard rule 7)

The fix lives in the **loader** (`kuna-analysis` crate, stage **S1 / code-data-partition**), not in
a `kuna_<slug>.rs` decompiler Action/Rule (S2–S9). It requires new memory-backing infrastructure
(a synthetic zero-filled segment), it touches a different tier than the assigned structuring
feature, and there is no kuna output to restructure until it lands. A decider subagent ratified
this as `scope: large`, `outcome: proposal`.

## Proposed implementation plan (for the approved implementation worker)

1. **New loader option** (env-var bridge, mirroring `relocobjects` / `i386_pie_plt`):
   `externdatazerofill` (working name). Default decision deferred to ablation (likely default-ON as
   a pure capability — like `relocobjects` — since linked images never take the ET_REL path).
2. **Back the extern area** in `elf_reloc::layout_relocatable`: after `extern_cursor`/`extern_order`
   are finalized, emit one zero-filled, **readable** `Segment` (and a read-only `SectionInfo`) that
   spans `[extern_base, extern_cursor)` rounded up to a page — so any read inside the synthetic
   extern region returns 0 instead of aborting. Keep the slot stride large enough (or size the
   backing generously, e.g. page-granular) that an in-struct offset like `options+0x1373` stays
   inside the backed region.
3. **Gate** the new backing behind the option (`KUNA_*` env var read at load time, like
   `KUNA_RELOC_OBJECTS`); off ⇒ byte-identical to today (still hard-fails, preserving parity).
4. **Verify**: re-run `compare --entry test_ifelseflatten_clientloop`; confirm kuna now produces
   output. Add a firing stage test once there is output to assert on.
5. **Re-assess the *structural* delta** (the original "ifelseflatten" question) only after output
   exists — it is a **separate** follow-up feature if a real gap remains.

## Speed / risk assessment

- **Risk: low/contained.** The change only affects the ET_REL (`.o`) path; linked ET_EXEC/ET_DYN
  images keep the PT_LOAD loader and are byte-identical. Gated off ⇒ no behavior change ⇒
  `make test` / `docs/baseline.json` parity preserved.
- **Speed: negligible.** One extra synthetic segment at load time; no per-function cost. Must still
  be measured per Hard rule 6 in the implementation worker.
- **Subtlety:** sizing the extern backing. A 16-byte-per-symbol stride is too small for struct
  globals read at large offsets (`options+0x1373`); the backing must cover realistic in-object
  offsets (page-granular backing, or widen the per-extern reservation). The implementation worker
  should pick the bound and document it.

## Proposed option name

`externdatazerofill` (alt: `relocexterndata`). LLM-discoverable `settableTable` row, `stage="S1"`,
`source_decompiler="angr"`, `inspiration="test_ifelseflatten_clientloop; CLE extern object zero-fill; client_request_tun_fwd"`,
`change_kind="structure-recovery"`.

---
🤖 Generated with [Claude Code](https://claude.com/claude-code)
