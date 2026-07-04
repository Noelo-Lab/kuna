---
case_id: O2-openssh-portable-ssh-sk-helper-mktemp_proto
group_id: openssh-portable::mktemp_proto
status: metric-artifact
tier: M
margin: 81
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: 81
option_closing: listing
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: kuna-analysis Listing tier (noreturn_propagate consumer, pre-S1 flow)
---
## Side-by-side

angr (stored) — the ENTIRE block, 9 lines. Obviously truncated: no getenv/snprintf branches,
and `sub_427cb3` = 0x400000 + 0x27cb3 = this function's own entry (0x27ca0) + 0x13 — a bogus
tail-call into its own body:

```c
// Function: mktemp_proto @ 0x27ca0
void mktemp_proto(unsigned long a0, unsigned long a1)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_427cb3();
    return;
}
```

kuna (stored, run-time = kuna 0.1.0) — decompiles PAST the internal fatal wrapper
`sub_1f2a0` (= sshfatal: `("misc.c","mktemp_proto",0x702,...)`) and swallows the NEXT
function in misc.c (parse_ipqos: the ipqos name table walk with strcasecmp + strtol <= 0xff).
Trimmed to the load-bearing 25 of 40 lines:

```c
// Function: mktemp_proto @ 0x27ca0
uint8 mktemp_proto(char *a0,uint8 a1)
{
  ...
  v2 = getenv("TMPDIR");
  if ((((v2 != 0) && (v1 = __snprintf_chk(a0,a1,1,0xffffffffffffffff,"%s/ssh-XXXXXXXXXXXX",v2), 1 <= v1)) && ((uint8)(int8)v1 < a1)) || ((v1 = snprintf(a0,a1,"/tmp/ssh-XXXXXXXXXXXX"), 0 <= v1 && ((uint8)(int8)v1 < a1)))) {
    return 0;
  }
  v7 = "misc.c";
  sub_1f2a0("misc.c","mktemp_proto",0x702,1,1,0);   /* sshfatal — noreturn, not known */
  v9 = *(void *)(v8 + 0x28);
  if (v7 != (char *)0x0) {                          /* everything below is parse_ipqos */
    v4 = (int8 *)0x671d0;
    ...
    do {
      if (strcasecmp(v7,v6) == 0) {
        return (uint8)*(uint4 *)(v3 * 0x10 + 0x671c8);
      }
      v6 = (char *)*v4;
      v4 = &v4[2];
      v3 = (uint8)((int4)v3 + 1);
    } while (v6 != (char *)0x0);
    v3 = strtol(v7,&v5,0);
    if (((*v7 != '\0') && (*v5 == '\0')) && (v3 <= 0xff)) {
      return v3;
    }
  }
  return 0xffffffff;
}
```

kuna (fresh, current main d12ef72) — byte-identical to the stored block above (40 loc,
4 ifs, 1 loop). The gap is NOT already fixed by the post-Jun-29 SAILR PRs.

EXTRA — kuna fresh with `--option listing on` (default-on noreturn_propagate becomes active;
adding `noreturn_disc on` gives the identical result). Structurally matches the misc.c
source function exactly (two guarded returns + trailing noreturn fatal):

```c
// Function: mktemp_proto @ 0x27ca0
unsigned long mktemp_proto(char *a0,uint8 a1)
{
  int4 v1; // eax
  int8 v2; // rax

  v2 = getenv("TMPDIR");
  if (((v2 != 0) && (v1 = __snprintf_chk(a0,a1,1,0xffffffffffffffff,"%s/ssh-XXXXXXXXXXXX",v2), 1 <= v1)) && ((uint8)(int8)v1 < a1)) {
    return 0;
  }
  v1 = snprintf(a0,a1,"/tmp/ssh-XXXXXXXXXXXX");
  if ((0 <= v1) && ((uint8)(int8)v1 < a1)) {
    return 0;
  }
                    /* WARNING: Subroutine does not return */
  sub_1f2a0("misc.c","mktemp_proto",0x702,1,1,0);
}
```

## Analysis

**Symptom (kuna side).** In the stripped binary the internal fatal wrapper `sshfatal`
(`sub_1f2a0`) has no name and no noreturn fact, so kuna's flow follows the fall-through
after the call and merges the next misc.c function (`parse_ipqos`) into `mktemp_proto`:
40 loc / 4 ifs / 1 loop vs the real function's 2 ifs. Classic
noreturn-propagation-stripped boundary overrun.

**Why the SCORE is an artifact anyway.** The rescore
(`scripts.decbench.rescore`, kuna commit d12ef72) reports **source_nodes = 1** — a
degenerate source CFG — and the source file it diffed against is
`O0/openssh-portable/compiled/auth-krb5.i`, the WRONG translation unit: `mktemp_proto` is
*defined* in misc.c; auth-krb5.i only sees the misc.h *declaration*, which Joern parses as
a 1-node CFG. Against a 1-node source graph GED degenerates to |dNodes|+|dEdges| of the
decompiled output itself, so the metric rewards the SMALLEST output regardless of
correctness: angr's stored block is a truncated 9-line stub whose one call target is
`entry+0x13` rebased at 0x400000 (a broken single-block decompilation, not a structuring
win) → GED 0; phoenix (same angr engine) → 0; every decompiler that recovered the real
body is penalized proportionally to its size: ghidra 38, ida 39, kuna 81 (kuna largest
because it also swallowed parse_ipqos). Rescore fresh GED: before 81.0 / after 81.0
(no options), confirming reproduction and that the number cannot be fixed from kuna's side
— a perfect decompilation still scores far above angr's degenerate 0.

**Why existing default-on kuna options don't fire.** `noreturn_known` /
`noreturn_extern` / `noreturn_externmatch` are all NAME-driven (vendored
exit/abort/__stack_chk_fail list); `sshfatal` is an internal, stripped-name function, so
no name matches. `noreturn_propagate` (the call-graph propagation, kuna analog of angr's
CFGFast no-return propagation) IS default-on but is a documented no-op unless the Listing
is built, and `listing` is default-off (whole-program recursive-descent disassembly; kept
off for speed + byte-identical parity). Flipping `listing on` alone closes the structural
gap here — propagation seeds from the PLT-named `exit` family (dynamic symbols survive
stripping) and concludes sshfatal noreturn through the wrapper chain; `noreturn_disc`
(the Ghidra FindNoReturnFunctionsAnalyzer analog, call-site evidence tally) is not even
needed for this case.

**Siblings.** All 13 siblings are the same function in other openssh binaries.
Spot-checked two stored dumps: `O2-openssh-portable-scp-mktemp_proto` (angr = identical
9-line stub calling `sub_4269e3` = entry+0x13; kuna = identical 40-line overrun past
`sub_1d990`) and `O2-noinline-openssh-portable-ssh-keyscan-mktemp_proto` (angr stub
`sub_438ea3` = entry+0x13; kuna overrun past `sub_a370`). Same recorded numbers
(0 vs 81, ghidra 38, ida 39) — the whole group is this one artifact plus this one covered
kuna symptom; a group-level decbench fix re-scores all 13.

## Proposed fix

**Primary (decbench-side bug to file).** The source-CFG extractor resolved `mktemp_proto`
to `auth-krb5.i` (a declaration → 1-node CFG) instead of the defining TU `misc.i`/misc.c.
Fix the definition-vs-declaration lookup (prefer a TU where the function has a body /
max-node CFG across TUs), and/or invalidate scoring when `source_nodes == 1` (the runner
already knows a degenerate source CFG makes the score meaningless — this case shows it
also silently rewards truncated decompiler output, since angr's GED-0 block is a broken
one-block stub). No kuna feature work is warranted off this margin.

**Secondary (kuna, already covered — no new code).** The genuine kuna-vs-ghidra
structural defect (boundary overrun past a stripped internal noreturn wrapper) is fully
covered by existing options: `--option listing on` (activates the default-on
`noreturn_propagate` consumer in the kuna-analysis Listing tier;
`decompiler/crates/kuna-decomp` flow honors the resulting noreturn fact pre-S1).
Default-on viability for `listing` is a separate campaign question — it is a
whole-program disassembly kept off for speed and parity (0/675 gate is currently
byte-identical with it off), so it stays opt-in; benchmark runs on stripped binaries
should consider adding `listing on` to the kuna invocation profile instead.
