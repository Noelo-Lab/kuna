---
case_id: O2-noinline-openssh-portable-ssh-sk-helper-sshbuf_set_max_size
group_id: openssh-portable::sshbuf_set_max_size
status: metric-artifact
tier: M
margin: 79
bucket: inherited
fresh_reproduces: true
fresh_ged: 79
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — complete, 15 LOC, zero branches:

```c
// Function: sshbuf_set_max_size @ 0xc1c0
void sshbuf_set_max_size(unsigned long a0, unsigned long a1)
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
    sub_40c1d0();
    return;
}
```

kuna (stored == fresh, byte-identical; 46 LOC, 9 ifs — the real function):

```c
// Function: sshbuf_set_max_size @ 0xc1c0
int4 sshbuf_set_max_size(int8 *a0,uint8 a1)

{
  uint8 v1;
  int4 v2;
  int8 v3; // rax
  uint8 v4;

  v2 = sub_b4e0();
  if ((v2 == 0) && (a0[4] != a1)) {
    if (((int4)a0[6] != 0) || (2 <= *(uint4 *)&a0[7])) {
      v2 = -0x31;
    }
    else {
      if (0x8000001 <= a1) {
        return -9;
      }
      sub_b680(a0,a1 < (uint8)a0[3]);
      if (a1 < (uint8)a0[5]) {
        v1 = a0[3];
        if (a1 <= v1) {
          return -9;
        }
        v4 = v1 + 0xff & 0xffffffffffffff00;
        if (v1 <= 0xff) {
          v4 = 0x100;
        }
        if (a1 < v4) {
          v4 = a1;
        }
        v3 = sub_3ba50(*a0,a0[5],v4,1);
        if (v3 == 0) {
          return -2;
        }
        *a0 = v3;
        a0[1] = v3;
        a0[5] = v4;
        if (a1 < v4) {
          return -9;
        }
      }
      a0[4] = a1;
      v2 = 0;
    }
  }
  return v2;
}
```

(kuna fresh output is byte-identical to stored — not re-fenced. Metrics: angr 15 LOC / 0 ifs; kuna stored and fresh both 46 LOC / 9 ifs / 0 gotos.)

## Analysis

Symptom: angr scored GED 0, kuna 79, on a function where kuna's output is the complete,
correctly structured `sshbuf_set_max_size` (matches the OpenSSH sshbuf.c source: the
SSHBUF_SIZE_MAX check, sshbuf_maybe_pack, the ROUNDUP resize via recallocarray, the
tail assignments). angr's stored block is TRUNCATED: it contains only the 16-byte
prologue (endbr64 + push r13/r12/rbx spills, rendered as `v2 = v4; ...`) and then a
call to `sub_40c1d0` — which is 0x400000 + 0xc1d0, i.e. the address immediately after
the prologue of the very function being scored (0xc1c0 + 0x10). angr's CFG split the
function body off as a separate "function" and decompiled only the prologue stub.
phoenix=0 is the same engine on the same broken function boundary, not independent
confirmation; ghidra (79) and ida (75) cluster with kuna (79) on the full body.

Rescore confirmation (ran once): `source_nodes: 1` — the source CFG is DEGENERATE, so
GED collapses to |dNodes|+|dEdges| of the decompiled graph, i.e. "smallest output
wins". Worse, the rescore reveals the scorer resolved the source function to
`/home/mahaloz/github/decbench/results/full_run/O0/openssh-portable/compiled/auth-krb5.i`
— the wrong translation unit entirely (`sshbuf_set_max_size` lives in sshbuf.c/.i), so
Joern produced a 1-node placeholder graph. Two compounding decbench-side defects:
(1) wrong source-file match giving a degenerate source CFG, (2) angr's truncated
prologue-stub output being maximally rewarded by that degenerate graph. Fresh kuna
(commit d12ef72) reproduces GED 79 exactly — correctly, since its full 9-if function
graph is "far" from a 1-node graph. No kuna option sweep is meaningful: any option
that changed output could only "win" here by emitting less code.

Note the case JSON said artifact_suspect=false / bucket=inherited, but the evidence
(angr 0-if stub vs 3 native decompilers agreeing on the full body, plus
source_nodes==1) overrides both flags.

Siblings: all 9 siblings are the same statically-linked sshbuf.c function in the other
openssh-portable binaries. Spot-checked two:
- O2-noinline-openssh-portable-scp-sshbuf_set_max_size: identical — angr 15-LOC
  prologue stub calling `sub_41a240` (= 0x1a230+0x10), kuna full 46-LOC/9-if body.
- O2-noinline-openssh-portable-sftp-sshbuf_set_max_size: identical — angr stub calling
  `sub_41b2b0` (= 0x1b2a0+0x10), kuna full body.
The entire 10-case group is one artifact; no kuna work applies.

## Proposed fix

No kuna change. Decbench-side bugs to file:
1. Source-function resolution matched `sshbuf_set_max_size` to `auth-krb5.i` (an O0
   compiled/preprocessed file that does not define it), yielding a 1-node Joern CFG;
   the scorer should reject/flag cases with source_nodes==1 instead of scoring them
   (a degenerate source CFG makes GED a pure output-size penalty).
2. Guard against truncated decompiler output: a scored function whose decompilation
   is a straight-line prologue ending in a call to `entry+0x10` of itself (angr
   function-boundary split) should be flagged, not scored 0.
Recommend excluding the whole openssh-portable::sshbuf_set_max_size group (10 cases,
margin 79 each) from kuna-vs-angr feature mining.
