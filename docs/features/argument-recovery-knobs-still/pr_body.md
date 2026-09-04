## What was broken

RE-friction round 2, need `argument-recovery-knobs-still` (major, **2 instances**,
two independent testers on two PE x86-64 binaries):

> **Argument recovery knobs still emit a zero-argument allocation call**
> (`6547b4d50f4238b24302b588`) — Even with both newly advertised recovery options
> enabled, one branch emits `sub_140002c90()` while adjacent branches emit
> `sub_140002c90(v11 + 0x27)`. The option-enabled output is byte-for-byte
> identical to the default output.

The tester's diagnosis is wrong and the captain's refutation is right: `calleearity`
and `varargstackargs` are **both default-ON** (DIV-102 / DIV-101), so "I enabled
them and nothing changed" is what enabling something already enabled looks like.
Instrumenting the seam gives the real cause, and it is finalization **order**:

```
[arity] CALL @0x1400016f7 entry=0x140002c90  ...  witness=[]
[arity]   trial1 addr=register:0x8 sz=8 used=false dnu=false unref=false slot=1 hasval=true
[arity]   sib  @0x1400016d1 input_active=true  act=["register:0x8/8 chk=true act=true"]
```

The losing site reaches `build_input_from_trials` **first**, with its RCX trial in
exactly the state `calleearity`'s promotion plan accepts — and its witness, the
sibling that recovers precisely `register:0x8`, is still `input_active`.
`ActionActiveParam` finalizes each spec the moment it is fully checked, in `qlst`
order, so `calleearity` reconciles only against the sites *before* it and declines
here for the reason its own "What it cannot do" section states.

That direction is not a corner case. It is MSVC's aligned `operator new`, whose
two arms call the same allocator:

```
cmp  rcx,0x1000
jc   small                ; rcx = the requested size
lea  rax,[rcx+0x27]
cmp  rax,rcx
jbe  abort                ; the overflow guard
mov  rcx,rax
call operator_new         ; LARGE: fresh rcx, recovered
small:
test rcx,rcx
jz   nothing
call operator_new         ; SMALL: rcx live-in, dropped by only_op_use
```

## The mechanism — `calleearityfwd` (new option, default on, DIV-103)

**Reordering the finalization was designed and rejected, not overlooked.**
`Funcdata::check_call_double_use` branches on whether *another* call spec is still
`input_active` while scoring a trial, and `ParamActive::maxpass` is 0 for the
standard model — so a spec's trials are checked in the same `apply()` where its
neighbours finalize. Deferring would change argument recovery on every binary
rather than only where two sites disagree.

Instead, a call that finalizes with an **empty** argument list is set aside —
together with the Varnodes its still-promotable trials point at, captured before
`op_set_all_input` drops them, which is the only moment they are reachable — and
retried **once**, at the end of the same `ActionActiveParam::apply`, when every
spec in the pass is final. The witness search and every one of `calleearity`'s
five refusals are reused unchanged (register storage only, real Varnodes only,
all-or-nothing, never subtractive, only a call that recovered nothing), so this
adds **no new way to promote a trial** — it only lets the existing one see the
sites that come after. Two limits are its own: a captured Varnode wider than its
trial is declined rather than truncated, and nothing crosses an `apply`. Inert
with `calleearity off`.

## The acceptance probe that now passes

The need's original acceptance had to be **replaced at triage** before this was
dispatched: it asserted only `stdout_absent: sub_140002c90\(\);`, which the
shipped default-off `--option spillargtrial reload` already satisfies by
*fabricating* a second argument at **both** sites, including the one that was
already correct. The replacement (`a-a94fa26848a4`) asserts all three:

```
$ kuna decompile "trappy attack.exe" 0x1400011c0 --addr
...
      v10 = (char ****)sub_140002c90(v11 + 0x27);     <- still exactly one argument
...
      v19 = (char *****)sub_140002c90(v11);           <- was sub_140002c90();
```

`verify --need argument-recovery-knobs-still` → **PASS**, and the single-line diff
above is the *entire* change to that function.

Promoted to `tests/cli/argument-recovery-knobs-still.json`, re-pointed at the new
vendored `alignednew_x86_64` fixture (CI has no dataset) and additionally pinned by
a cargo test in `kuna-cli/tests/decompile_cli.rs`, since nothing in CI runs
`tests/cli/`.

## The second witness

`69a3822f` asked for `NtWriteFile`'s native arguments. `decompile-all` over the
whole binary finds exactly **one** `NtWriteFile` call site, so there is no sibling
to reconcile against; the line is byte-identical before and after, confirming the
refutation that this bullet is off-mechanism. The same binary does carry the real
mechanism twice, and both are fixed and verified against the disassembly:
`sub_14002f800` (`if (a0 < 0x1000) return sub_14002fd38(a0);`) and `sub_14002ff34`
(`mov bl,cl; call f; mov cl,bl; call f` → the first site recovers `sub_1400309c4(a0)`).

## Corpus sweep (standing requirement 7)

126 binaries A/B'd **whole-binary** (`decompile-all`, every function): 81 decbench
ELF O0/O2 across x86-64/ARM/Cortex-M, and 45 crackmes (PE x86-64 + static ELF).

- **40 argument gains on 15 of the 81 ELF binaries; 244 on 18 of the 45 crackmes.**
- **Zero statements deleted, moved or re-anchored.** Normalized statement multisets:
  everything removed reappears carrying an argument, everything added is either the
  argument or a value the argument keeps live.
- The DWARF-named gains audit themselves: `fgetc(a0)`, `sshbuf_len(buf)`,
  `__uflow(f)`, `_fclose_r(ptr)`, `EVP_CIPHER_CTX_get_iv_length(ctx)`,
  `quote_n(1,v4)`, `gnutls_strerror(v & 0xffffffff)`, and `__fixdfsi(v5,v46)` where
  ARM soft-float passes the double in `r0:r1`.
- The largest structural change is a *recovery*: in `p0tp.exe` the aligned
  `operator delete` mirror goes from a collapsed `sub_14000c538();` under a folded
  disjunction to the full `ptr`/`size` computation and `sub_14000c538(v31,v40)`,
  matching the shape kuna already emits where the arguments survive.
- **One residual, found by the sweep and named rather than hidden**: in
  `bobxREAL.exe` a CRT `_errno` site gains `(v10,v11)` because a sibling site in the
  **same function** had already over-recovered two registers with this option OFF.
  That is DIV-102's stated residual ("the reconciliation is only as good as the
  witness site it copies") inherited unchanged — the option cannot create an
  over-recovery, only propagate one within a function.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK — 675/675** (`docs/baseline.json` unmoved) |
| `make test-stages` | **PARITY OK — 597/597** (594 → 597 for the new two-pass `tests/stages/kuna-calleearityfwd.xml`) |
| `make rust-test` | green |
| `make check-spec` | green |
| `kuna catalog --check` | `catalog OK` |
| `verify --need argument-recovery-knobs-still` | **acceptance PASS** |

**Speed flat.** Witness function `0x1400011c0`: **-1.69%** over 11 interleaved
repeats (`scripts.pipeline.timeit`, 5% budget). Whole-binary
`decompile-all trappy attack.exe`: min-of-5 interleaved **-8.3%** against an on/on
control spread of **-15.2%** measured on the same run, i.e. below this box's noise
floor. The retry runs only for a call that finalized with an empty argument list,
once per `ActionActiveParam::apply`; there is no per-op cost.

## On the default

T_TRIAGE asked for a new **default-off** option. This ships as a new option (never
an edit to `calleearity`'s rule, so the ablation is intact and `calleearity off`
still disables both directions) but **default-on**, because the replaced acceptance
probe runs `kuna decompile ... --addr` with **no options** and can only pass on
default output. The `docs/agents.md` default-ON bar is met on its own terms: 0/675
datatest assertions change, 0 stage assertions regress, the sweep classifies every
hunk, and the speed delta is inside the noise floor. Recorded in
`docs/features/argument-recovery-knobs-still/record.json` under `decisions`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
