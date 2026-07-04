---
case_id: O2-openssh-portable-ssh-ssh_tun_confirm
group_id: openssh-portable::ssh_tun_confirm
status: feature-candidate
tier: L
margin: 190
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: null
root_cause_family: noreturn-propagation-stripped
feature_slug: noreturn-propagate-allpaths
scope: small
angr_ref: angr.analyses.cfg.cfg_base.CFGBase._determine_function_returning
kuna_stage: kuna-analysis S1 (s1_noreturn_propagate / s1_noreturn_disc)
---

## Side-by-side

### angr (stored) — complete, 18 LOC, matches the real ssh.c `ssh_tun_confirm`

```c
// Function: ssh_tun_confirm @ 0x13600
extern unsigned int g_5053bc;

long long ssh_tun_confirm(unsigned long a0, unsigned int a1, unsigned int a2)
{
    long long v1;  // rdi
    long long v2;  // rsi
    long long v3;  // rcx
    long long v4;  // r8
    long long v5;  // r9

    if (!a2)
    {
        sub_466d00("ssh.c", "ssh_tun_confirm", 1862, 0, 2, 0, "Tunnel forwarding failed");
        if (g_5053bc)
            sub_423750(255); /* do not return */
    }
    sub_466d00("ssh.c", "ssh_tun_confirm", 1867, 1, 5, 0, "tunnel forward established, id=%d", a1);
    return sub_413440(v1, v2, a1, v3, v4, v5);
}
```

### kuna (stored, run-time) — 223 LOC, trimmed to the overrun seam (~40 of 223 lines; the elided 180 lines are the swallowed bodies of `ssh_confirm_remote_forward` and `resolve_host`)

```c
// Function: ssh_tun_confirm @ 0x13600
uint8 ssh_tun_confirm(unsigned long a0,uint8 a1,int4 a2)
{
  /* ...30+ locals for the swallowed neighbours' frames elided... */
  if (a2 != 0) {
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
    v38 = (char *)0x13688;
    sub_66d00("ssh.c","ssh_tun_confirm",0x74b,1,5,0);
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x00013440 */
    v3 = sub_13440();
    return v3;
  }
  v38 = (char *)0x13653;
  sub_66d00("ssh.c","ssh_tun_confirm",0x746,0,2,0);
  v4 = "Tunnel forwarding failed";
  if (dat_1053bc == 0) {
    v38 = (char *)0x13688;
    sub_66d00("ssh.c","ssh_tun_confirm",0x74b,1,5,0);
    v3 = sub_13440();
    return v3;
    /* (duplicate dead copy of the same 3 lines elided) */
  }
  v19 = 0xff;
  sub_23750();                       // <-- cleanup_exit(255): NO-RETURN, kuna doesn't know
  v38 = "ssh_tun_confirm";           // <-- from here on this is ssh_confirm_remote_forward's body
  v37 = "ssh.c";
  ...
  if ((int4)v39 != 0x51) {
    v30 = "failure";
    sub_66d00("ssh.c","ssh_confirm_remote_forward",0x6fe,0,5,0);   // neighbour's strings
    ...
  }
  ...
      sub_66d00("ssh.c","resolve_host",0x106,1,7,0);               // and the NEXT one too
      __snprintf_chk(v12,0x20,1,0x20,0xc6c92,v8);
      v2 = getaddrinfo(v20,v12,&v14,&v13);
  ...
label_13754:
  *(void *)&v5[-8] = 0x13759;
  sub_13440();
  if (*(int8 *)&v5[8] != *(int8 *)(v22 + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  v3 = 0;
  return v3;
}
```

### kuna (fresh, current build) — byte-identical to the stored run (223 LOC, 5 gotos, 2 labels, 20 ifs); the gap is NOT fixed by the post-Jun-29 SAILR PRs

Metrics table from the triage dump:

| output | loc | gotos | labels | switches | ifs | loops |
|--------|---|---|---|---|---|---|
| angr(stored) | 18 | 0 | 0 | 0 | 2 | 0 |
| kuna(stored) | 223 | 5 | 2 | 0 | 20 | 0 |
| kuna(fresh) | 223 | 5 | 2 | 0 | 20 | 0 |
| kuna(fresh, `listing on` + `noreturn_disc on`) | 91 | 2 | 0 | 0 | 11 | 0 |

## Analysis

**Symptom.** Boundary overrun past an internal no-return wrapper. The real
`ssh_tun_confirm` (ssh.c:1862) is ~8 source lines; on the error path it calls
`cleanup_exit(255)` — `sub_23750` in the stripped binary. kuna does not know
0x23750 is no-return, so it decompiles straight through the call and swallows the
next two functions (`ssh_confirm_remote_forward`, `resolve_host` — their
`__func__` strings appear verbatim in kuna's output), producing 223 LOC / 5 gotos
where angr, ghidra, and ida all emit the exact 3-branch source shape (GED 0).
Fresh kuna (current main) reproduces byte-identically.

**Not an artifact.** `artifact_suspect=false`; angr's stored block is complete
(it renders the full source function including both branches and the tail call,
with an explicit `/* do not return */` on `sub_423750`), and ghidra/ida score 0
independently of the angr engine. No rescore needed (tier L).

**Root cause (kuna).** `cleanup_exit` disassembles to:

```
23750: push rbp; ...                    ; entry
23764: call 24b60                      ; cleanup handler (returns)
23773: je 2377e / 2377c: jne 2378a     ; conditional
2377e: call 26100
23783: mov edi,ebp
23785: call c9a0 <_exit@plt>           ; no-return; NOT the address-last insn
2378a: call bbb0 <unlink@plt>          ; cold tail laid out AFTER the _exit call
2378f: jmp 2377e                       ; backward jump = the address-last real insn
23791: nop padding
```

Every CFG path funnels into `call _exit@plt` and the body contains no RET — the
function is no-return in fact. Both kuna Listing-tier passes miss it:

- `s1_noreturn_propagate` (`function_is_no_return`, rule 1) requires the
  **address-last real instruction** to be a call/jmp to a terminal callee. Here
  the address-last real instruction is the in-body backward `jmp 2377e` (gcc put
  the cold `unlink` tail *after* the `call _exit`), so rule 1 fails and the pass
  gives up — even though rules 2–4 (no RET, no computed jump, no escaping branch)
  all pass.
- `s1_noreturn_disc` needs either >=3 call sites with invalid fall-through
  (`cleanup_exit`'s call sites are mostly conditional with live code after — no
  evidence) or `last_act_is_terminal_call`, whose `call_site_has_no_fallthrough`
  predicate returns false because 0x2378a (the byte after `call _exit@plt`) is a
  *valid decoded instruction* (the jne target), not data/another function's entry.

**Why the name-based options don't cover it.** `noreturn_known` /
`noreturn_extern` / `noreturn_externmatch` (all default-on) are name-list seams;
`cleanup_exit` is an internal, project-specific wrapper and the binary is
stripped — there is no name to match (and even unstripped, `cleanup_exit` is not
on the vendored list). The option sweep confirms: `--option listing on --option
noreturn_disc on` (with default-on `noreturn_propagate` active) shrinks 223→91
LOC — it *does* discover `sub_62a40` (`sshfatal`) via call-site evidence and cuts
the `resolve_host` tail — but the `cleanup_exit` overrun into
`ssh_confirm_remote_forward` persists. No existing option closes the gap.

**Why angr wins.** angr's `CFGBase._determine_function_returning`
(cfg_base.py:1067 in the decbench venv angr 9.2.213; same shape in angr-dev) is a
pure CFG-closure fixpoint: "at least one return site => returning; calls a known
non-returning function and has no other exits => non-returning", iterated over
the callgraph via `_updated_nonreturning_functions` with no address-order or
evidence-count requirement. `_exit` is known no-return, `cleanup_exit` has no
other exit, done. Ghidra-proper reaches the same conclusion through its function
body analysis, hence ghidra=0 too.

**Siblings.** `O2-noinline-openssh-portable-ssh-ssh_tun_confirm`: identical
symptom — kuna 213 LOC / 5 gotos vs angr 13 LOC, margin 182, same
`ssh_confirm_remote_forward`/`resolve_host` swallow past `cleanup_exit` (fresh
reproduces). One fix covers the whole group, and likely every other
openssh-portable case whose error path calls `cleanup_exit`/`sshfatal`.

## Proposed fix

**Mechanism.** Generalize rule 1 of `function_is_no_return` in
`/home/mahaloz/github/kuna/decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs`
from "the address-last real instruction is a terminal transfer" to a CFG-closure
form that tolerates cold-tail layout: keep rules 2–4 exactly (no RET, no
computed jump, every static jump target in-body), and replace rule 1 with

1a. the body contains at least one reachable call/jmp to a terminal callee, AND
1b. no instruction can fall off the end of the body: the address-last real
    instruction is a terminal transfer OR has no fall-through (an unconditional
    in-body `jmp`, as in `cleanup_exit`).

Soundness: with no RET (2), no computed jump (3), no escaping static jump (4),
and no fall-through past the body end (1b), the only exits are calls to
already-no-return callees (1a) — the function cannot return. This stays strictly
more conservative than angr (which also concludes pure infinite loops). The
emission path is unchanged: the same `NoReturnFact` → `commit_analysis_output` →
`set_function_no_return` seam; the caller-side dead-code drop is inherited from
the `flow.rs` artificial-halt path. Implementation must also verify the terminal
seed contains the `_exit@plt` stub address (the Known facts do reach the Listing
— `noreturn_disc` seeded from them successfully for `sub_62a40`'s promotion).

**Owning files.**
- `/home/mahaloz/github/kuna/decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (`function_is_no_return`, rule 1)
- optionally the same relaxation in `s1_noreturn_disc/mod.rs::last_act_is_terminal_call` (not required if propagate is fixed)
- cross-crate e2e: extend `verify_noreturn_propagate.rs` with a cold-tail fixture (wrapper whose `call exit` is followed by a reachable cold block + backward jmp)

**Risks / default-on viability.** The change lives inside an already-shipped,
already-default-on option (`noreturn_propagate`, DIV-14) that is inert unless
`listing on` — so all three parity gates (675 datatests, stages, rust-test) are
byte-identical by construction and default-run speed is untouched (0/675 + <=5%
policy trivially met). The real exposure is that decbench ran kuna with defaults,
where `listing` is off: to collect this win in the benchmark either the decbench
backend must pass `--option listing on` (plus `noreturn_disc on`) or `listing`
needs a default-on cost evaluation — that decision should ride with the PR as a
measured decompile-speed number per pipeline standing requirement 4. False
positives are bounded by rules 2–4 remaining intact; a mislabeled no-return
would drop live caller code, so the cold-tail fixture plus a re-run of the
openssh group is the acceptance test.
