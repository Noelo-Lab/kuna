---
case_id: O2-noinline-gnutls-systemkey-getpass_copy
group_id: gnutls::getpass_copy
status: metric-artifact
tier: S
margin: 16
bucket: inherited
fresh_reproduces: true
fresh_ged: 16
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — complete, 15 loc:

```c
// Function: getpass_copy @ 0xe7b0
void getpass_copy(unsigned long a0, unsigned long a1, long long a2)
{
    unsigned long long v4;  // r13
    unsigned long long v5;  // r12
    unsigned long long v6;  // rbx
    unsigned long long v0;  // [bp-0x20]
    unsigned long long v1;  // [bp-0x10]
    unsigned long long v2;  // [bp-0x8]

    v2 = v4;
    v1 = v5;
    v0 = v6;
    sub_40e7c3(a2);
    return;
}
```

kuna (stored, run-time) — complete, 19 loc:

```c
// Function: getpass_copy @ 0xe7b0
void getpass_copy(char *a0,uint8 a1,unsigned long a2)

{
  char *v1; // rax
  uint8 v2; // rax

  v1 = (char *)getpass(a2);
  if (v1 != (char *)0x0) {
    v2 = strlen(v1);
    if (v2 < a1) {
      strcpy(a0,v1);
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x000096b0 */
      gnutls_memset(v1,0,v2);
      return;
    }
    gnutls_memset(v1,0,v2);
  }
  *a0 = '\0';
  return;
}
```

kuna (fresh, current build d12ef72) — byte-identical to the stored kuna output above (0 gotos, 2 ifs, 19 loc).

## Analysis

- Symptom: recorded angr GED=0 vs kuna GED=16; fresh kuna output is unchanged from the
  benchmark run, so nothing merged since 2026-06-29 touches this case. Ghidra=16 and
  ida=16 cluster exactly with kuna; only the angr engine (angr + phoenix, which is the
  same engine with a different structurer) scores 0.
- angr's stored output is TRUNCATED: the body is nothing but three callee-saved register
  spills and a call to `sub_40e7c3(a2)` — and 0x40e7c3 is `getpass_copy+0x13`, i.e. an
  address *inside the function itself* (just past the push-prologue). angr's CFG split
  getpass_copy into a stub + an inner "function" holding the entire real body (the
  getpass/strlen/strcpy/zeroize logic and both branches), so the decompilation scored for
  getpass_copy is a single straight-line basic block.
- Rescore confirms the scoring artifact: `source_nodes: 1` (source file
  `full_run/O0/gnutls/compiled/certtool-common.i`) — the Joern source CFG for
  getpass_copy is degenerate, so GED is meaningless and rewards whichever decompiler
  emits the least structure. angr's truncated 1-block stub gets 0; kuna/ghidra/ida are
  penalized 16 *for correctly recovering the function* (kuna's two nested ifs match the
  real gnutls `getpass_copy` source: getpass -> NULL check -> strlen bound check ->
  strcpy + zeroize, else zeroize, else `pass[0]=0`). Fresh rescore: before=16, after=16,
  delta=0, not approximated.
- No option sweep is warranted: kuna's output is already structurally faithful to the
  source; there is no kuna deficiency for an option to close. Flipping options could only
  "win" by making kuna's output as degenerate as angr's, which is the opposite of a fix.
- Siblings (same artifact, whole 11-case group inherited):
  - O2-gnutls-certtool-getpass_copy: identical pattern — angr emits spills +
    `sub_418e13(a2)` (entry+0x13), kuna fresh identical 2-if body, ghidra/ida=16.
  - O2-noinline-gnutls-gnutls-cli-getpass_copy: identical pattern — angr emits spills +
    `sub_40dbb3(a2)` (entry+0x13), kuna fresh identical 2-if body, ghidra/ida=16
    (that sibling also records binja=0, but ghidra/ida still cluster with kuna).

## Proposed fix

None in kuna — nothing to fix (scope: none). This is a decbench scoring bug to file:

- Guard the scorer against degenerate source CFGs: when `source_nodes <= 1` (Joern failed
  to build a real CFG for the source function, as here for `getpass_copy` in
  `certtool-common.i`), mark the case unscoreable / drop it from margins instead of
  emitting a GED that rewards truncated output.
- Optionally add a truncation sanity check on decompiler output: a decompilation whose
  only call target lies *inside the scored function's own address range*
  (`sub_<entry+0x13>` here) is a split-function stub and should be flagged, not scored.
- The entire gnutls::getpass_copy group (11 sibling cases, margin 16 each) should be
  reclassified as metric artifacts once the guard lands.
