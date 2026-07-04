---
case_id: O2-noinline-gnutls-systemkey-figure_key_type
group_id: gnutls::figure_key_type
status: metric-artifact
tier: M
margin: 70
bucket: inherited
fresh_reproduces: true
fresh_ged: 70
option_closing: null
root_cause_family: angr-truncation-artifact
feature_slug: null
scope: none
angr_ref: null
kuna_stage: null
---
## Side-by-side

angr (stored) — complete, 12 lines:

```c
// Function: figure_key_type @ 0xde40
void figure_key_type(unsigned long a0)
{
    unsigned long long v3;  // r13
    unsigned long long v4;  // r12
    unsigned long long v0;  // [bp-0x10]
    unsigned long long v1;  // [bp-0x8]

    v1 = v3;
    v0 = v4;
    sub_40de58();
    return;
}
```

kuna (stored == fresh, byte-identical; 45 lines, shown in full):

```c
// Function: figure_key_type @ 0xde40
unsigned long figure_key_type(unsigned long a0)

{
  unsigned long v1; // r13
  
  v1 = 1;
  if (strcasecmp(a0,0x2869a) != 0) {
    v1 = 6;
    if (strcasecmp(a0,"rsa-pss") != 0) {
      v1 = 7;
      if (strcasecmp(a0,"ed25519") != 0) {
        if (strcasecmp(a0,"eddsa") != 0) {
          v1 = 0xc;
          if (strcasecmp(a0,"ed448") != 0) {
            v1 = 5;
            if (strcasecmp(a0,"x25519") != 0) {
              v1 = 0xb;
              if (strcasecmp(a0,0x286c1) != 0) {
                v1 = 2;
                if (strcasecmp(a0,0x286c8) != 0) {
                  v1 = 4;
                  if (strcasecmp(a0,"ecdsa") != 0) {
                    if (strcasecmp(a0,0x286cc) != 0) {
                      v1 = 8;
                      if (strcasecmp(a0,"gost01") != 0) {
                        v1 = 9;
                        if (strcasecmp(a0,"gost12-256") != 0) {
                          v1 = 10;
                          if (strcasecmp(a0,"gost12-512") != 0) {
                            v1 = 0;
                            __fprintf_chk(dat_364c0,1,"unknown key type: %s\n",a0);
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  return v1;
}
```

Structural metrics: angr(stored) 12 loc / 0 ifs; kuna(stored) and kuna(fresh) identical at 45 loc / 13 ifs / 0 gotos / 0 labels.

## Analysis

- Symptom: angr scores GED 0, kuna (and ghidra=70, ida=70) score 70. Fresh kuna output is byte-identical to the stored run, and a rescore confirms GED_after=70 (kuna commit d12ef72), so the recorded number reproduces — but the number is meaningless.
- Artifact, confirmed two ways:
  1. angr's stored block is obviously TRUNCATED. The real gnutls `figure_key_type` is a 13-deep `strcasecmp` if-chain (rsa/rsa-pss/ed25519/eddsa/ed448/x25519/.../gost12-512 with an "unknown key type" fprintf tail) — exactly what kuna, ghidra, and ida all recovered. angr instead emitted only the register-save prologue and a call to `sub_40de58` = entry(0x40de40)+0x18, i.e. angr split the function at the end of its prologue and shipped a 1-block stub. phoenix=0 is the same angr engine (different structurer), not independent confirmation.
  2. The mandatory rescore reports `source_nodes: 1` — the Joern-parsed source CFG (mapped to `O0/gnutls/compiled/certtool-cfg.i`) is degenerate. A 1-node source graph matches angr's 1-block stub "perfectly" (GED 0) and charges every correct decompilation its full graph size (kuna's ~13-branch CFG → 70). Note the source mapping itself looks suspect: `figure_key_type` lives in gnutls's certtool-common code, not `certtool-cfg`; either the benchmark's function→source mapping or Joern's parse of the `.i` produced the degenerate graph.
- Why no kuna option is relevant: kuna's output is already the desirable answer — full body, 0 gotos, 0 labels, clean nesting, matching ghidra/ida structurally. There is no gap to close; an option sweep was skipped as moot.
- Siblings (dumped, no options):
  - `O2-gnutls-systemkey-figure_key_type` @ 0xe160: identical pattern — angr 12-line stub calling `sub_40e178` (entry+0x18), kuna full 13-if chain (45 loc). Same artifact.
  - `O2-noinline-gnutls-ocsptool-figure_key_type` @ 0xdb50: identical pattern — angr 12-line stub calling `sub_40db68` (entry+0x18), kuna full 13-if chain; binja=76 clusters with kuna/ghidra/ida too. Same artifact.
  - The entire `gnutls::figure_key_type` group (all 4 cases) is this one artifact and should be excluded/annotated as a group.

## Proposed fix

No kuna change. Decbench-side bug to file, two parts:

1. Scoring guard: treat `source_nodes == 1` as degenerate and mark the case unscored (or scored as N/A) instead of emitting a GED that rewards truncated decompiler output. This single guard neutralizes the whole `gnutls::figure_key_type` group.
2. Function→source mapping check for gnutls: `figure_key_type` was scored against a CFG extracted from `certtool-cfg.i` that Joern parsed to 1 node; verify the mapping points at the real defining translation unit and that Joern parsed the `.i` body at all.

Optionally, an upstream note for the angr backend: its CFG splits `figure_key_type` after the prologue (body attributed to `sub_<entry+0x18>`), so its stored output is not a valid decompilation of the function — any metric other than the degenerate one would have penalized it.
