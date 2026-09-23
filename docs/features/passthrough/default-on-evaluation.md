# passthrough — default-on evaluation

Verdict: **on by default** (round G), on the strength of the parameter arm:
4,107 of 4,346 gained parameters DWARF-confirmed over 574 slices in 25 projects,
**none contradicted**, none lost. The return arm is a judgement with a measured
cost, stated below and in the option's `use_when` rather than measured away.

## Round F (commit 7da4afecf): held off

The argument arm passed every criterion — 454 of 506 gained parameters confirmed
by DWARF over 26 binaries, 0 contradicted, 0 arguments deleted, typesweep +24
perfect with 0 moved off — but the tail-return arm gave 23 functions a return
value DWARF says they do not have, and `type_match` cannot see a return type.
What the note asked for was "a gate on the return arm that refuses a callee
whose own return is a value it did not compute".

## Round G, first pass: the return gate and the whole-corpus check

`recovered_output` (`kuna_protoorder.rs`) states a callee's return only when
`kuna_returnuncomputed::every_return_computes` holds of that callee: every live
RETURN must hand back a value produced in every byte and on every path. gnulib's
`void version_etc_arn` ends its fallthrough on a `__fprintf_chk` and returns
`CONCAT44(<leftover>, <that call's result>)`, which the pair repair's relaxed
question calls computed and this one does not. Contradicted returns over the
444-slice corpus: 212 of 4,267 before, 6 of 4,087 after.

## Round G, second pass: a corpus that is not the corpus we tune on

The 444-slice corpus is eight GNU projects built by one toolchain from one source
idiom. Re-run on **130 slices of 17 disjoint projects** (openssh-portable,
e2fsprogs, dpkg, kmod, dash, cronie, sysvinit, base-passwd, libacl, libbsd,
libselinux, zlib, libexpat, libedit, rsyslog, iproute2, gnutls), the same check
gave a different answer: **159 of 1,691 checkable parameters contradicted
(9.4%)**, against 0.14% on the tuned corpus. Every one was openssh, and every one
was one shape — a callee parameter that exists only because gcc pushed a dead
register for stack alignment at a variadic call (`xcalloc`), or because the callee
reads it as a `va_arg` (`open_safer`).

Three refusals and one detection fix (`dwarf-confirmation.md` has the full
account): the **vararg tail** (`kuna_varargtail.rs`), the **hole** a positional
claim would have to fill (`no_hole_before`), the `xor %eax,%eax` **variadic-call
detection** that had never fired, and `computes_everywhere` answering `false`
rather than `true` when its budget runs out.

| after the tightening | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| parameters, 444-slice corpus | 2,851 | 2,728 | **0** | 123 |
| parameters, 130 disjoint slices | 1,495 | 1,379 | **0** | 116 |
| returns, 444-slice corpus | 4,087 | 4,012 | 6 | 69 |
| returns, 130 disjoint slices | 1,659 | 1,446 | 151 | 62 |

The tightening costs 208 confirmed parameters (5% of the gain) and changes the
typesweep result not at all: still +24 perfect.

## The flip procedure

Measured on the rebased tree (origin/main `9e07ab931`, `calleevote` merged), both
arms of the same build.

| step | result |
|---|---|
| (a) `make test` | PARITY OK, 675/675 — the console decompiles no callee first, so no datatest can reach the rule |
| (b) `make test-stages` | PARITY OK, 1311/1311; `tests/stages/kuna-passthrough.xml` already pins `option passthrough off` in pass 1, so nothing was re-recorded |
| (c) `make test-cli` | 235/235, with one probe re-pinned (below) |
| (d) 444-slice typesweep, new default vs `--option passthrough off` | PERFECT **1,593 → 1,569** reading the flip backwards, i.e. the flip is **+24** (24 onto perfect, 0 off); aggregate 3,853.89 → 3,905.63; 83 improved, **3 worse** |
| (e) interleaved min-of-15, fmt/ls/sort -O2 + bash -O2 | see `record.json`; budget +5% |
| (f) whole-corpus `decompile-all` before/after, 8 binaries | 276 functions change; every changed line is one of the documented effects bar two, both hand-read (a `*a0 = *a0 + 1` that became `a0->field_0x0 = a0->field_0x0 + 1` under a struct pointee, and a `sub_45230(a0,a3); return;` that became `return sub_45230(a0,a3);`) |
| (g) `modes.rs` | the `UNEVALUATED` entry is removed: a default-on option needs no preset override |

The three worse rows are `print_group_list` (groups, id) and `print_stuff` (id):
a `char *` parameter that already matched is retyped `long` by a callee's vote.
They are type votes on an existing parameter, not arity moves — verified against
DWARF — and 83 functions improve against them. The `copy_reg` trio that was worse
before the tightening no longer is.

One `tests/cli` probe pins output the new default moves:

* `protoorder-types-keeps-a-float-pointee-bitwise` expects `double s3(...)`
  instead of `void s3(...)`: `s3` hands back its tail call's result, and the
  fixture's own source says `NI double s3(struct P *dst, long a, long b)`. Its
  clause — the struct pointee and the two integer-bit arguments — is unchanged.

`protoorder-types-never-moves-an-arity` ran with `--option passthrough off` in the
first pass, because the default gave `overrec` a parameter that fixture was
written to deny. The rule was tightened instead, and the probe runs the default
again with no option in its command.

## What ON still costs

The **return** arm. A source-`void` wrapper that tail-calls a value-returning
function compiles to exactly the `jmp` that a wrapper returning what it calls
does, and `rax` holds the callee's result at the RETURN either way. kuna claims
it: right for 5,458 of 5,615 gained returns across both corpora, wrong for 157 —
6 on the GNU corpus, 151 on the disjoint one, where e2fsprogs's one-line bitmap
accessors and openssh's `blf_key` family are written that way. `--option
passthrough off` restores upstream's reading exactly, for the returns as much as
the arguments.

The **parameter** arm's residual cost is the callee's own recovery error: a
callee whose recovered list over-states a parameter its body really reads for
another purpose hands that parameter on. The two refusals cover the shape that
produced every contradicted row measured so far; the next one will be a shape
nobody has measured yet, which is the argument for keeping the option.
