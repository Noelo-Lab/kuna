## The problem

A protected stub crashes the decompiler instead of producing C. On
`thief_crackme.exe` (crackmes.one `629a286b33c5d45b75903c7a`, MSVC i386 PE), the
116-byte function at `0x41cd08` exits 1:

```
$ kuna decompile thief_crackme.exe 0x41cd08 --addr
void sub_41cd08(void)
{
  /* WARNING: decompilation failed: decompile pipeline reached an un-ported seam
     (LOSS-131): index out of bounds: the len is 1 but the index is 1 */
}
error: decompilation failed for sub_41cd08 ...
note: decomp_dbg stderr:
thread 'main' panicked at crates/kuna-decomp/src/substrate/block.rs:648:22:
index out of bounds: the len is 1 but the index is 1
```

The same panic accounts for 39 functions across 17 images in the RE dataset. A
vendored 5 KiB reproducer is in this PR: `kuna decompile-all
decompiler/crates/kuna-analysis/tests/fixtures/phimarkerrun_pe_i386.exe`.

## The fix

- A block's MULTIEQUALs carry one input per in-edge, so severing an edge has to
  drop that slot from every phi in the target. `branch_remove_internal` walked
  the leading run of markers and stopped at the first op that was not one, which
  is correct only if the markers stay contiguous.
- They do not. `ActionStackPtrFlow::analyze_extra_pop` rewrites a solved phi into
  an `INT_ADD` **in place**, and `op_zero_multi` rewrites a 1-input phi into a
  `COPY` the same way. On the witness the block's op list is `[MULTIEQUAL,
  INT_ADD, MULTIEQUAL, MULTIEQUAL, ...]` and one phi of three got patched; the two
  left behind claimed an in-edge the block no longer had, and the unreachable
  sweep then indexed a phi slot as an edge index and ran off the end.
- The scan now covers the whole op list — which is what `block_remove_internal`
  already does, here and in the C++ it was ported from. Deliberately not a bounds
  guard: the stale slot is removed, so the phi collapses to the value the
  surviving edge carries instead of being wired to a `0xBADDEF` constant.
- Unflagged. A panic that yields a stub instead of C is not a judgement call;
  `docs/history.md` DIV-159 records the divergence from upstream's `break`.

## The tests

`branch_remove_internal_patches_multiequals_behind_a_converted_op` builds the
`[phi, INT_ADD, phi]` layout directly and fails without the fix. The vendored
PE32 is promoted as `tests/cli/block-processing-panics-out.json`.

Swept 975 images with an instrumented build that reports when the new scan
patches a phi the old one skipped: **0 of 803 decbench binaries reach the changed
branch** (60 re-run under a full before/after `decompile-all` are byte-identical),
17 challenge images do. Full A/B on those 17: 30 of 39 crash stubs become real C,
0 new stubs, 0 functions lost, every other function byte-identical. The 9
survivors fail on a different seam.

`make test` PARITY OK 675/675 · `make test-stages` PARITY OK · `make rust-test`
green · `make check-spec` OK · `make test-cli` 123/123 · `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
