---
case_id: O0-gnutls-ocsptool-port_to_service
pool: ida
status: feature-candidate
tier: S
margin: 10
fresh_verdict: reproduced unchanged on today's build in BOTH --mode auto/aggressive and --mode reliable; kuna still merges the source's three `return sport;` guards into one short-circuit `if` with a comma side effect and a single trailing `return v2`
option_closing: null
feature_slug: repeatreturn
scope: small
confidence: high
---

## Side-by-side

IDA (stored, GED 0) — four returns, matching the source one-for-one:

```c
char *port_to_service(const char *a1, const char *a2)
{
  if ( (unsigned char)sub_15C46((unsigned int)*a1) != 1 )
    return (char *)a1;
  port = atoi(a1);
  if ( !port )
    return (char *)a1;
  porta = htons(port);
  v5 = getservbyport(porta, a2);
  if ( v5 )
    return v5->s_name;
  fprintf(stderr, "Warning: getservbyport(%s) failed. Using port number as service.\n", a1);
  return (char *)a1;
}
```

kuna (fresh, today's build, `--mode auto` == aggressive; byte-identical under `--mode reliable`):

```c
char * port_to_service(char *a0,unsigned long a1)
{
  int4 v1; // eax
  void *v2; // rax

  v1 = (int4)*a0;
  v2 = a0;
  if ((sub_15c46(v1) == '\x01') && (v1 = atoi(a0), v1)) {
    v1 &= 0xffff;
    v2 = (void *)getservbyport(htons(v1),a1);
    if (v2) // branch-flip
      v2 = *(char **)v2;
    else {
      fprintf(stderr,"Warning: getservbyport(%s) failed. Using port number as service.\n",a0);
    }
  }
  return v2;
}
```

Structure vs stored kuna is UNCHANGED (2 ifs / 0 gotos / 0 labels / one trailing return in both);
only the DIV-34..39 print wave differs (`v1 != 0` -> `v1`, `v1 = v1 & 0xffff` -> `v1 &= 0xffff`,
brace elision, `dat_1f060` -> `stderr`, banner -> `// branch-flip`). Ghidra (stored) is the same
shape and scores the same 10 — this is inherited upstream behaviour, not a kuna regression.

## Source

`~/github/decbench/results/full_run/O0/gnutls/compiled/socket.i:14612` (preprocessor noise stripped):

```c
const char *port_to_service(const char *sport, const char *proto)
{
	unsigned int port;
	struct servent *sr;

	if (!c_isdigit(sport[0]))
		return sport;

	port = atoi(sport);
	if (port == 0)
		return sport;

	port = htons(port);

	sr = getservbyport(port, proto);
	if (sr == NULL) {
		fprintf(stderr, "Warning: getservbyport(%s) failed. Using port number as service.\n", sport);
		return sport;
	}

	return sr->s_name;
}
```

Three guard clauses, four returns — three of which return the SAME value (`sport`).
Source CFG 7 nodes / 6 edges; `degenerate_source: false`, `approximated: false`,
`artifact_suspect: false`. The metric is not lying here, and IDA is not truncating:
its pane is the complete function.

## Correctness bug found here — FIXED (DIV-47)

Separate from (and more serious than) the structural symptom below: the merged exit
this record describes was also **losing a value**. The `fprintf` arm reloads `sport`
at `0x114e8`, and that COPY was marked non-printing, so the emitted C returned the
NULL from the failed `getservbyport` instead of `sport` — proven by compiling the
emitted C and running it (`"99999"` → `(null)` where the binary yields `99999`).
Root cause: `Funcdata::build_copy_pair_range` omitted `Cover::addRefPoint`
(`merge.cc:1121`), so `Merge::checkCopyPair`'s dominance range was a point and the
intervening `getservbyport` write was never seen inside it. Fixed; the restore is
emitted. The structural symptom analysed below (one merged exit vs four returns) is
untouched and still open.

## Analysis

**Structural symptom (one).** kuna renders a function whose source is four guard-clause
returns as ONE merged exit: `v2 = a0;` at entry, the first two guards fused by
`rule_block_or` into a single short-circuit condition carrying a comma side effect
(`(sub_15c46(v1) == '\x01') && (v1 = atoi(a0), v1)`), the third guard demoted to an
assignment diamond, and a single trailing `return v2`. Zero early returns where the
source (and IDA / angr / binja / phoenix, all GED 0) have three.

**Why.** gcc -O0 compiles each `return sport;` to `mov -0x18(%rbp),%rax ; jmp .Lepi`,
so all four returns land on ONE bare epilogue block:

```
   1147a:  jmp 114f5      <- return sport     (guard 1)
   11495:  jmp 114f5      <- return sport     (guard 2)
   114ec:  jmp 114f5      <- return sport     (after fprintf)
   114f2:  mov (%rax),%rax  fallthrough       <- return sr->s_name
   114f5:  leave ; ret
```

The RETURN block at `0x114f5` has `size_in == 4` and its return value is
`MULTIEQUAL(a0, a0, a0, load(sr))` — three IDENTICAL variable slots plus one distinct.
Because the guards share that out-target, `rule_block_or` is free to comma-merge them.

**Root cause — the const-return gate in P8.** kuna already owns the machinery to undo
this (`ActionReturnDup`, the SAILR `ReturnDuplicatorHigh` port). It declines here by
design. `Funcdata::returndup_apply`
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs:2973`) requires

```rust
if !self.returndup_is_const_ret(parent) {
    continue;
}
```

and `returndup_is_const_ret` (same file, :2903) resolves the RETURN's input 1 through
COPY/MULTIEQUAL chains and admits only constants/annotations — a `return <variable>`
share is rejected outright. Its sibling `earlyreturn_const_edges` (:3042) does the
per-edge peel but likewise only for slots that resolve to CONSTANTS; here **zero** slots
are constant, so `earlyreturn` (default-on) also finds nothing to peel. The gate is
deliberate and documented: broad variable-return duplication was measured to regress the
aggregate GED-perfect count by ~976 across 21768 firings, so the const filter was added.
This case is a casualty of that filter.

The splittable precondition is NOT the blocker: `return_split_is_splittable` (:2775)
accepts gcc -O0 `leave; ret` epilogues in this very binary — a whole-binary
`returndup on` vs `off` diff on `stripped/ocsptool` produces 2083 diff lines
(`_DT_INIT`, `sub_94c4`, `sub_978e`, `sub_9e30`, ... all gain `// return-dupe` early
returns). `port_to_service` is untouched in that diff. The ONLY gate that declines is
the const-ret test.

**Owning phase: P8 (structure / goto-quality).**

**Options swept, none close it** (all on today's build, `--mode reliable`, addr 0x1144a):
`returndup on` (also already on via aggressive), `condfold on`, `condfold wide`,
`earlyreturn off`, `gotoreduce off`, `iteregion off` — output byte-identical in every
case. `--mode aggressive` and `--mode reliable` are byte-identical too, so neither a
code fix nor a mode default has closed this.

**Secondary (not the symptom, not GED-visible):** the prototype and types are also worse
than IDA's — `unsigned long a1` for `const char *proto`, `void *v2` for `char *`, and
`*(char **)v2` instead of `v5->s_name` (no `struct servent` recovery). That is P4/P5 and
should be triaged separately if wanted; it does not affect this case's score.

## Proposed fix

**Mechanism — `repeatreturn`: peel the REPEATED arms of a shared variable-return epilogue.**
Add a predicate next to `returndup_is_const_ret` in
`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs`:

> a splittable shared RETURN block also qualifies when its value MULTIEQUAL has at least
> two in-edge slots resolving to the SAME non-free varnode that is defined OUTSIDE the
> block (a repeated arm).

Then OR it into the `returndup_apply` gate and let the existing loop split every in-edge
but one via the existing `node_split`. On this function that peels the three `a0` edges,
leaving the `load(sr)` edge on the original block — exactly IDA's four-return shape.

The repetition is the evidence the const gate was standing in for. At -O0 the only way
several distinct predecessors can converge on one identical value at the epilogue is that
the source wrote `return <that value>;` several times; at -O2 the same shape is compiler
cross-jumping, which kuna already reverts for non-return tails (`crossjumprevert`). The
shape the ~976-regression measurement punished — one DISTINCT variable per in-edge
(`v = f(); ... v = g(); return v;`) — has no repeated arm and stays merged under this
narrowing, so the narrowing is strictly disjoint from the regressing population.

Owning files:
- `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs` (new predicate + the
  `returndup_apply` gate; reuses `return_split_is_splittable` + `node_split` verbatim)
- `decompiler/crates/kuna-decomp/src/p8_structure/kuna_returndup.rs` (option plumbing, or
  a sibling `kuna_repeatreturn.rs` action if it wants its own ablation switch — preferred,
  so the decbench delta is separable from `returndup`'s)
- `decompiler/crates/kuna-decomp/phases.toml` + `src/p0_knowledge/options.rs` (new option
  row, default OFF pending the ablation) + the hard-coded catalog counts

Risks:
- **Aggregate default.** History says this area is aggregate-sensitive; the option must
  ship default-OFF and earn a default flip from a decbench ablation (the same evidence bar
  DIV-23/DIV-25 met for `earlyreturn`/`switchreturn`). Do not assume the win generalizes
  from this family.
- **Output churn.** Any function with a repeated-arm shared epilogue re-renders, so the
  datatest corpus will diff; expect per-test `<com>option ... off</com>` opt-outs rather
  than a baseline re-pin.
- **Growth.** Reuse the existing `MAX_RETURNDUP_INEDGES`/`MAX_RETURNDUP_SPLITS` caps; a
  wide repeated-arm merge (e.g. an error-handling funnel where many paths `return -1`)
  would otherwise fan out. Note `returndup`'s existing const path already duplicates that
  same funnel when the value is constant, so the caps are proven adequate.

**Leverage.** The whole 9-case `gnutls::port_to_service` family is this one gap
(3 x margin 10 at O0, 6 x margin 5 at O2/O2-noinline = 60 total), and in EVERY one Ghidra
scores identically to kuna while angr/binja/phoenix score 0 — the signature of an
inherited upstream merge that SAILR's return duplicator undoes. Spot-checked
`O2-gnutls-ocsptool-port_to_service` (0xf700): same class, kuna nests the guards and ends
with one shared `return a0`.
