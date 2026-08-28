# `variantguard` — naming a Rust enum's variant from the DWARF discriminant

**Status: PROPOSAL. Default-OFF, excluded from every preset, nothing ships.**

The premise is sound and the implementation on `feat/variantguard` is a working
reference. The **memory-object guard direction is known-unsound** and no amount of
positional/CFG reasoning closed it: four independent adversarial rounds each found
a *different shape of the same defect*, and each was found only by a purpose-built
fixture — never by the four gates, and never by the tests written for the previous
round. That pattern, not the feature, is the result.

This document is the record so the next attempt starts from evidence.

---

## 1. The premise, and why it is sound

`dwarfvariants` (DIV-87) imports a Rust tagged enum's `DW_TAG_variant_part` and
installs it as `struct { tag; union payload; }`, recording the exact geometry on a
side table: the discriminant's offset and width, and per variant its source name,
its `DW_AT_discr_value` and its payload struct.

It then refuses to NAME a variant wherever two of them overlay one byte range.
A union member selects itself by **offset**, and the discriminant is never
consulted — so in a tagged enum, where every payload variant begins immediately
after the tag, `Ok.__0` and `Err.__0` sit at the same offset with the same width,
`ScoreUnionFields` ties, and the winner is trial order. Every `Result<T,E>` is that
case, and before the suppression the label was not merely uncertain but
reproducibly false (`Ok` printed on both arms of an if/else and on the consumer's
`Err` arm; `Err` nowhere in the binary).

**The premise of `variantguard` is that control flow supplies the missing
evidence, and that only the block→value mapping is derived.** The compiler already
stated which value selects which variant; a block below `if (r.tag == 1)` is a
block where the discriminant is 1. Neither step guesses, and both seed kinds are
anchored on a Varnode whose *recovered type* is the enum — so no codegen shape can
trigger the pass, which is the premise #333 was withdrawn for.

That premise is not what failed.

## 2. What failed, once, in four costumes

Every defect was one thing:

> **A fact about an enum object's discriminant, applied at a program point where it
> no longer holds.**

### Round 0 — the shape premise (#333, withdrawn before this branch)

Classified an enum from codegen: "two return paths storing different constants at
offset 0". Refuted by `r0-shape-adversarial.rs` — a `#[repr(C)] struct{kind,val}`,
a `(u64,u64)` tuple, a bitmask pair, and a `&'static str` fat pointer whose
"discriminant" is a `.rodata` address were all accepted. **Fix:** take the
discriminant from DWARF, never from shape. This is the premise above, and it has
held ever since: 1 of 14 functions changes on that corpus and it is the true
positive.

### Round 1 — the producer store propagated backwards over whole blocks

A constant `tag = K` store coloured every op in its block *and every block that
reached it*, with no ordering test and no kill. A payload READ executing **before**
the store was labelled with the variant the store was about to write.

```c
v1 = (dst->payload).Err.__0;   // reads whatever the CALLER passed
dst->tag = 1;
```

The decisive form is the **control pair** in
`r1-producer-backwards-clobber-control.rs`: `read_then_clobber` and
`read_then_clobber_ok` are byte-identical but for the clobber constant, and gave
the same `mov 0x8(%rdi),%rax` **opposite** variant names. At most one could be
right; by construction neither was, because the source reads both arms.

**Fix:** a producer store colours **writes only** — a store into the object and the
pointer arithmetic that feeds one and nothing else — read positionally off the
nearest constant tag store above or below it *in its own block*, killed by any
intervening non-constant tag store or call, refused when the two sides disagree.
The whole-block backward analysis was deleted.

### Round 2 — the guard fact had no kill, and beat the producer fact

The guard side coloured every op its edge reached with no kill at all, including
ops after the object had been overwritten *inside* the guarded block; and a
`(true, _) =>` precedence arm let a singleton guard region discard a producer fact
the analysis had computed **correctly**.

```c
(*dst).tag = 1;
(*dst).payload.Ok.__0 = v1 + 1;   // contradicts the line above it
```

and, from `r2-guard-no-kill-call-control-pair.rs`, the guard side's own control
pair: `read_after_call_ok_guard` / `_err_guard`, identical but for which arm the
guard selects, gave the same load opposite names — each naming the variant the
preceding call had just destroyed.

**Fix:** one kill discipline shared by both halves (`object_events`). A guard
proves what the object was AT THE GUARD; a store proves what it becomes; neither
survives an **event** — a store over the tag bytes, a call, or any store the pass
cannot attribute to the object (it may alias it). Within a block the kill is
positional; the fact leaving a block holding an event is the whole set. The
precedence arm was deleted and the two facts are intersected, a disagreement
refusing.

### Round 3 — the hoisted tag read resurrects the killed guard

**This is where it stops.** A `CBRANCH` is its block's last op, but **its condition
Varnode need not be.** rustc hoists the tag read above the clobber and leaves the
branch below it:

```c
v1 = (*dst).tag;                 // read hoisted ABOVE the event
wipe(dst);                       // wipe() sets *dst = Err(99)
if v1 & 1 != 0 { return 0; }     // branch left BELOW it
return (*dst).payload.Ok.__0;    // WRONG: the object is Err(99)
```

In the same run kuna renders `wipe` as `(*dst).tag = 1; (*dst).payload.Err.__0 =
99;` — the analysis has the right answer and prints the wrong one three lines away.

The mechanism, exactly: `compute_regions` correctly raises the event-holding
block's out-fact to the whole set, and then **immediately re-applies that same
block's edge constraint**, which was derived from the stale pre-clobber read. The
killed fact is resurrected on the edge. `Regions::guard_at`'s positional kill never
sees it, because the read is in a *successor* whose entry region is already the
resurrected singleton. `hoist_xblock` puts the clobber in a different block, so an
intra-block positional kill cannot see it at all.

Witness: `r3-hoisted-tag-read-resurrects-guard.rs`.

**No fix attempted.** The edge constraint would have to be invalidated by
reasoning about where its *condition* was computed relative to the event — i.e.
reaching definitions over the object — and the aliasing half needs an escape
analysis. That is a different piece of work.

## 3. What IS verified sound

Two things survived every attack aimed at them, across rounds 2 and 3, and are the
recommended starting point for any retry.

**The producer half** (`producer_writes`): writes only, intra-block, positional,
killed by a tag store / a call / an unattributable store, refusing on
disagreement. `r2-producer-positional-probes.rs` attacked it with two objects
interleaved in one block, an address expression feeding both a read and a write,
`tag=0; payload; tag=1` in a straight line, and a write/call/tag sequence. It held
on all four, and on both reviewers' independent probes.

**The value-object guard.** A value object (`ObjKind::Value`) has no events at all,
because an SSA value cannot be clobbered — that is what SSA means, and it is why
the round-3 resurrection has no analogue there. It held under every probe both
reviewers built, including a `MULTIEQUAL` joining two variants, a value
round-tripped through memory, and an address-taken local handed to a callee (which
is correctly reclassified as `Mem` and therefore falls under the unsound half).

This is also where the feature's real value sits: `use16` — `match r16(x) { Ok(v)
=> v, Err(e) => e + 100 }` — is a value object, and it is the headline recovery.

> **Recommendation for a future attempt: value-object guard + producer writes only.
> Omit the memory-object guard entirely until there is an escape analysis.**

## 4. The measured cost, honestly

On the std-linked `-g` witness (`breadth-std-g-rust.rs`, 161 variant parts, 608
functions), with everything enabled:

| revision | functions changed | variant labels |
|---|---|---|
| round 2 (producer fixed, guard unkilled) | 5 | 17 |
| round 3 (kill discipline) | 4 | 11 |

Every one of the 6 labels the kill discipline cost is a **read below a CALL** — the
`Tree` drop glue, `depth`'s `Node.field_0x8`, one `Bin.__1` each in `eval` and the
`Expr` drop glue. Those are refusals, not regressions.

An independent reviewer, measuring the same shape on their own std-linked `-g`
binary, recorded **5 functions / 9 labels → 3 / 7**, with both lost functions being
drop glue and a recursive tree walk, and **2 of 2 lost labels a read below a call**.
The two measurements agree in kind and in direction.

Dropping the memory-object guard as recommended in §3 would cost the remaining
drop-glue and tree-walk recoveries and keep `use16`-shaped ones. On this witness
that is most of what is left; the feature's breadth was always small, and §6 says
why that is not the interesting part.

## 5. Two methodology findings, worth more than the feature

**(a) A per-invariant adversarial fixture is the only thing that found any of
this.** Every round's tests passed. Every round was wrong. The gates never fired:
both parity corpora are symbol-less `bytechunk`s with no DWARF, so a DWARF-gated
change is structurally unreachable from them — `675/675 PARITY OK` is true and
nearly vacuous here. Each defect was found by a fixture built specifically to
attack one claimed invariant, and never by the tests written for the previous
round, which were themselves non-vacuous (ablating the option failed 12 of 16).
Tests written against a claim you already believe do not test the claim.

**(b) A reviewer reported that `kuna decompile` (the CLI) masks the defect while
the console / `bootstrap_from_object` path shows it — I could not reproduce that,
and record the negative.** On the round-3 witness both paths print the identical
wrong `(*dst).payload.Ok.__0`, and on the committed clobber fixture the only
difference between the two paths is the output **language** — the CLI's language
detection renders a Rust binary as Rust (`(*dst).payload.Ok`) while the console
prints C (`(dst->payload).Ok`). That is easy to misread as a naming difference when
scanning for `payload.Ok`. The general hazard is real and worth keeping in mind —
there genuinely are two load paths, and only the console path is exercised by the
cross-crate tests — but the specific claim did not reproduce for me.

## 6. What this proposal leaves behind

* A working reference implementation on `feat/variantguard`
  (`p5_types/kuna_variantguard.rs`), default-off and preset-excluded.
* The full witness corpus, [`witnesses/`](witnesses/), one file per defect.
* A committed fixture, `variantguard_clobber_x86_64` (+ vendored source), holding
  eleven refusals including two control pairs.
* 16 cross-crate tests and 22 stage assertions, all two-pass, all surviving the
  default-off flip.
* Everything DIV-87 already shipped is untouched: the layout, the recovered
  signatures, the side table, and the suppression rule that keeps `dwarfvariants`
  sound on its own.

`docs/history.md` carries **no DIV row**, because nothing ships and there is no
divergence to record.
