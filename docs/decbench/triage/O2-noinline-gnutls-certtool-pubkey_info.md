---
case_id: O2-noinline-gnutls-certtool-pubkey_info
group_id: gnutls::pubkey_info
status: covered-by-option
tier: M
margin: 69
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: 69
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: null
scope: small
angr_ref: angr.analyses.cfg.cfg_base.CFGBase._determine_function_returning (via CFGFast._iteratively_analyze_function_features)
kuna_stage: S1/external-refinement (noreturn_propagate, gated on S1 listing)
closed_by: PR #121 (benchlisting, DIV-15)
ged_after_fix: 6
---

## Side-by-side

angr (stored) — full, 23 loc, 0 gotos, 1 if:

```c
long long pubkey_info(long long a0, struct_0 *a1)
{
    long long v1;  // rax

    v1 = sub_40fbf0(a0, a1);
    if (!v1)
    {
        __fprintf_chk(stderr, 0x1, "find public key error\n");
        sub_40d430(1); /* do not return */
    }
    sub_417770(v1, g_446518, g_446508, g_4465a0, a1->field_24);
    return gnutls_pubkey_deinit(v1);
}
```

kuna (stored, run-time) — 96 loc, 3 gotos, 9 ifs, 3 loops; trimmed to the load-bearing ~45 lines (declarations and the middle of the swallowed loop cut):

```c
void pubkey_info(unsigned long a0,int8 a1)
{
  /* ... 17 local declarations cut ... */
  v4 = sub_fbf0();
  if (v4 != 0) {
    sub_17770(v4,dat_46518,dat_46508,dat_465a0,*(void *)(a1 + 0x24));
    gnutls_pubkey_deinit(v4);
    return;
  }
  v4 = 1;
  __fprintf_chk(dat_464e0,1,"find public key error\n");
  v13 = 1;
  sub_d430();
  /* ===== everything below is the NEXT function's body (certificate-list printer), swallowed ===== */
  v16 = *(void *)(v14 + 0x28);
  v11 = 0;
  v17 = a1;
  v9[0] = (void *)sub_23280(dat_46510,0,v8);
  if (v9[0] == (void *)0x0) {
    v5 = "file";
    if (dat_46510 == 0) {
      v5 = "standard input";
    }
    v6 = "%s";
    do {
label_1417a:
      __fprintf_chk(dat_464e0,1,v6,v5);
      v3 = sub_d430(1);
label_14197:
      v5 = (char *)gnutls_strerror(v3);
      v6 = "import error: %s\n";
    } while( true );
  }
  v3 = gnutls_x509_crt_list_import2(&v11,&v10,v9,dat_465a4,0);
  if (v3 < 0) goto label_14197;
  /* ... branchflip'd DER/PEM warning block cut ... */
  v7 = 0;
  do {
    while (
    if v7 != 0 {__fprintf_chk(dat_46518,1,0x349da)      /* <-- INVALID C: structurer mangles the */
    }v1 = v7 * 8                                        /*     irreducible swallowed-body CFG    */
    if *(int4 *)(v4 + 0x24) != 0 {sub_e170(*(void *)(v11 + v7 * 8),dat_46518,1)
    }v2 = *(void *)(v11 + v1), v13 != 0) {
      v7 = v7 + 1;
      pubkey_info(v2,v4);
      /* ... export/deinit loop tail + duplicated (*dat_464a8)(v11) returns cut ... */
  } while ((int4)v7 < v10);
  (*dat_464a8)(v11);
  return;
}
```

kuna (fresh, current build d12ef72, default options) — **byte-identical to the stored block above** (96 loc / 3 gotos / 9 ifs / 3 loops); gap fully reproduces on main.

kuna (fresh, `--option listing on`) — full; structurally matches angr:

```c
void pubkey_info(unsigned long a0,int8 a1)
{
  int8 v1; // rax

  v1 = sub_fbf0();
  if (v1 != 0) {
    sub_17770(v1,dat_46518,dat_46508,dat_465a0,*(void *)(a1 + 0x24));
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x0000bb90 */
    gnutls_pubkey_deinit(v1);
    return;
  }
  __fprintf_chk(dat_464e0,1,"find public key error\n");
                    /* WARNING: Subroutine does not return */
  sub_d430(1);
}
```

Rescore (Joern, one run): recorded 69.0 → before (fresh, defaults) **69.0** → after (`listing on`) **6.0** (delta −63; source_nodes=3, no degeneracy, no approximation). The residual 6 exactly equals ghidra's 6 on this function — i.e. with `listing on` kuna lands on its ancestor's inherited rendering (early-return true-arm vs angr's fall-through ordering), and the kuna-specific gap is fully closed.

## Analysis

**Symptom.** In the stripped binary, `pubkey_info` ends its error path with `call sub_d430(1)` — certtool's internal `app_exit` wrapper (`if (val && tempfile) remove(); exit(val);`, verified by decompiling 0xd430 directly, where kuna itself correctly marks the tail `exit()` noreturn). kuna does not know `sub_d430` is noreturn, so flow falls through the call into the next function in the image (a certificate-list printer at ~0x13f5x): the output balloons 23→96 lines, grows 3 gotos/3 loops/9 ifs, a spurious recursive `pubkey_info(v2,v4)` call, and the swallowed body's irreducible CFG even drives the structurer into emitting INVALID C (`while ( if v7 != 0 {...}` ). angr marks `sub_40d430(1); /* do not return */` and bounds the function correctly, scoring GED 0.

**Root cause.** Stripped-binary internal noreturn wrapper — the classic noreturn-propagation-stripped family. kuna's name-list passes cannot reach it: `noreturn_known` (S1) and `noreturn_extern`/`noreturn_externmatch` (S2 flow seams) all key on the callee's *name* matching the vendored noreturn list; `sub_d430` is a stripped internal function with no name. The pass that does cover it — `noreturn_propagate` (S1 external-refinement, the kuna analog of angr's CFGFast call-graph noreturn propagation, default-ON since DIV-14) — is **a structural no-op unless the Listing tier is built**, and `listing` (S1 code-data-partition, the program-wide recursive-descent disassembly) is default-OFF for decode-cost reasons. So the default `kuna decompile-all` benchmark run never propagates `exit` → `app_exit`, while angr's `CFGBase._determine_function_returning` (iterated by `CFGFast._iteratively_analyze_function_features`) always runs as part of CFG recovery.

**Why angr wins.** angr's noreturn analysis is not optional: CFGFast concludes `sub_40d430` is non-returning (its only exits reach `exit`), the caller's block after the call is never treated as fall-through, and the function is bounded exactly as in the source (3-node source CFG, GED 0). ida gets the same via FLIRT/its own noreturn propagation; ghidra's "Non-Returning Functions – Discovered" analyzer gets it too (ghidra=6 is purely rendering).

**Option coverage.** `--option listing on` alone closes the gap (69 → 6): it builds the Listing, which lets the already-default-on `noreturn_propagate` conclude `sub_d430` noreturn (last real instruction is a call to already-noreturn `exit`, no return path) and plant the halt. No other flag needed; `noreturn_disc` (the >=3-call-site evidence analyzer) is not required here.

**Siblings.** None (group is a singleton).

## Proposed fix

No new pass is needed — the mechanism (`noreturn_propagate` over the Listing) already exists and demonstrably closes the case. The gap is purely that the benchmark/default path never builds the Listing. Options, in increasing ambition:

1. **decbench-side (immediate, zero engine risk):** pass `--option listing on` in the kuna decbench backend (`decbench/decompilers/raw/kuna_raw.py` → `kuna decompile-all` invocation). For `decompile-all` the program-wide disassembly cost is paid once per binary and amortizes across all functions.
2. **kuna-side (small, option-gated per pipeline rule 1):** default `listing on` for the `decompile-all` (load-once) path only, keeping single-function `kuna decompile` at zero decode cost. Parity is safe by construction — the Listing is real-ELF-path only, so all 675 datatests + 158 stage tests are byte-identical (already the recorded catalog guarantee for `listing`/`noreturn_propagate`). The <=5% speed budget must be measured on a large binary (certtool-sized: one extra recursive-descent disassembly per load); if it exceeds budget on decompile-all, ship as the decbench flag (1) instead.

Owning code: `decompiler/crates/kuna-decomp` S1 external-refinement (`noreturn_propagate` consumer) + the Listing builder (S1 code-data-partition); no S2–S9 work. Risks: `noreturn_propagate` REMOVES CODE — a wrong noreturn conclusion on some other function could drop live code; mitigated by it being fixpoint-structural (no evidence threshold shortcuts past a return path) and already default-on-by-policy since DIV-14.
