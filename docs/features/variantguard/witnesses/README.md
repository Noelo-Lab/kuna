# `variantguard` witness corpus

Every fixture that refuted a revision of `variantguard`, kept so the next attempt
starts from evidence instead of re-deriving it. The full account is
[`../analysis.md`](../analysis.md).

**Build line** (all `.rs`; `rustc 1.90.0`):

```sh
rustc -C opt-level=1 -C debuginfo=2 -C relocation-model=static \
      -C panic=abort -C target-feature=+crt-static -C link-arg=-nostartfiles \
      -o <name> <name>.rs
```

`r0-c-*.c` are built with `gcc -g -O2` and `clang -g -O2`; the `.o` form of
`r0-c-tagged-struct.c` is linked into the Rust image with an added
`-C link-arg=<obj>.o`.

| File | Round | The defect it caught |
|---|---|---|
| `r0-shape-adversarial.rs` | 0 | The **premise** check that #333 failed: `#[repr(C)] struct{kind,val}`, a `(u64,u64)` tuple, a bitmask pair, a `&'static str` fat pointer, a niche `Option<&str>`, a 3-variant enum. All must be INERT. Also holds `p_split` (the true positive) and `p_either` (a payload read under no guard, which must stay unnamed). |
| `r0-c-tagged-struct.c`, `r0-c-standalone.c` | 0 | The same, in C, at gcc `-O2` and clang `-O2`, including a hand-rolled `struct { enum kind; union value; }`. |
| `r1-producer-backwards-clobber-control.rs` | 1 | **The producer store propagated backwards over whole blocks.** `read_then_clobber` / `read_then_clobber_ok` are byte-identical but for the clobber constant, and gave the SAME instruction opposite variant names. The decisive control-pair form. |
| `r1-producer-backwards-read-then-store.rs` | 1 | A read kept live across the clobber: output named `Err`, then tested for `Ok` and returned unchanged, three lines apart. |
| `r1-producer-backwards-unknown-path.rs` | 1 | One forward path stores, the other returns or diverges — probes for the `UNKNOWN` apparatus (since deleted). |
| `r1-proof-channel-two-objects.rs` | 1 | Per-root attribution and aliasing: two `Result`s in one function, a guard on one and a read of the other, a loop re-tagging across its back edge. **Held.** |
| `r2-guard-no-kill-guarded-write.rs` | 2 | **The guard fact had no kill, and beat the producer fact by precedence.** A write that builds `Err` inside an `Ok`-guarded block was named `Ok`, one line below its own `tag = 1`. Plus the mirror, an aliasing clobber, and a split-tag join. |
| `r2-guard-no-kill-stale-read.rs` | 2 | A read under a stale guard, with and without a call doing the clobbering. |
| `r2-guard-no-kill-call-control-pair.rs` | 2 | The guard side's control pair: `read_after_call_ok_guard` / `_err_guard`, identical but for which arm the guard selects, gave the same load opposite names — each naming the variant the preceding call destroyed. |
| `r2-guard-overwrite-in-arm.rs` | 2 | The clobber inside the *guarded* block rather than the join block. |
| `r2-guard-overwrite-and-call.rs` | 2 | Guard, then overwrite; guard, then a call that mutates through the same pointer. |
| `r2-producer-positional-probes.rs` | 2 | Attacks aimed at the round-2 producer rule: two objects interleaved in one block, an address expression feeding both a read and a write, `tag=0; payload; tag=1` in a straight line, and a write/call/tag sequence. **The producer rule held on all four.** |
| `r3-hoisted-tag-read-resurrects-guard.rs` | 3 | **The refutation this proposal ends on.** The tag is READ above a clobbering call and the BRANCH on it is below: the block-level kill raises the fact to the whole set and the same block's stale edge constraint immediately resurrects it. `hoist_xblock` puts the clobber in a different block so a positional kill cannot see it at all. |
| `breadth-std-g-rust.rs` | — | The std-linked `-g` breadth witness: `HashMap`, `BTreeMap`, `Box<dyn Error>`, `Result`, `enum Tree { Leaf, Node }`, `enum Expr { Lit, Bin, Neg }`. 161 variant parts, 608 functions. |
