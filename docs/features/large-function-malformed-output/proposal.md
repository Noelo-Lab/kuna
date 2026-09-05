# large-function-malformed-output — three defects, one witness, and an acceptance that cannot be met

RE-need `large-function-malformed-output` (round 1/2, challenge `6a3822888a86e4c2c55254fe`).
Witness: `selam.exe` (PE32+ x86-64, MSVC, no PDB), `0x140002d50`, 6144 bytes, 1267 lines of C.

The tester filed one blocker with four complaints. They are **four symptoms of three
independent defects**, none of which is the one the record's hypothesis names, and none of
which produces the string the acceptance probe asks for. All of the numbers below were
measured on `7b39fa06`.

## 1. The measured decomposition

| the tester's complaint | measured today | root cause | shape of the fix |
|---|---|---|---|
| "raw synthetic stack-address writes" | 63 × `*(unsigned long long *)&v67[-8] = 0x1400…;` | `INT 0x29` unbalances the stack pointer by +8 | one gated no-return decision at the P2 flow seam |
| "badly reconstructed BCrypt calls" | `BCryptGenerateSymmetricKey(*(unsigned long long *)&v66[0x50],…)` | same as above, *plus* no Win32 prototypes | same, plus a `libcsigs`-shaped Win32 signature table |
| "impossible conditions such as `v13 != v13 + 6`" | 1 × , and 21 hand-rolled 5-line fill loops | `REP STOSB` is not recovered as a `memset` | extend `memsetrecover` to the string-op form |
| "165 anonymous variables" / "over 1,200 lines" | 159 declarations, 1267 lines | the sum of the three above | — |

The three are independent. Fixing the first does **not** remove the impossible condition —
that question was asked explicitly in the need's decision log, and the answer is measured
below, not argued.

## 2. Defect A — `INT 0x29` (`__fastfail`) grows RSP by 8, and that poisons the whole frame

`0x140002d50` contains eleven `INT 0x29`. x86 SLEIGH lowers `INT imm8` to

```
:INT imm8  is vexMode=0 & byte=0xcd; imm8  { tmp:1 = imm8; intloc:$(SIZE) = swi(tmp); call [intloc]; }
```

— a `call` with **no matching push**. Every other x86 `CALL` pcode does `RSP = RSP - 8` first,
and `x86-64-win.cspec`'s default `__fastcall` carries `extrapop="8" stackshift="8"`, so a real
call is net zero. This one is net **+8**, and kuna prints it:

```c
(*(void *)swi(0x29))(5);
v65 = &v64[8];            // RSP, eight bytes higher than it was
```

Once two paths join with RSP offsets that differ by 8, the frame stops being a constant offset
from the spacebase. Everything downstream inherits it: stack locals stop being locals and
become offsets off a `char *`, and the `CALL` return-address push — normally a dead store into
an unmapped slot — becomes an explicit store through that pointer. That is the tester's
"raw synthetic stack-address writes", verbatim:

```c
*(unsigned int *)&v66[0x30] = 0;                     // an outgoing stack argument
*(unsigned int *)&v66[0x28] = 0x20;                  // another
*(unsigned char **)&v66[0x20] = v48;                 // another
*(unsigned long long *)&v66[-8] = 0x140003e16;       // the CALL's return address
v12 = BCryptGenerateSymmetricKey(*(unsigned long long *)&v66[0x50],&v53,v60,v51);
```

### The mechanism is proven, twice, without touching engine code

`__fastfail` never returns — that is its entire contract — so the honest answer is to cut flow
there. kuna already has that lever on the `--assert` plane, which makes the fix measurable
before it is written:

```bash
kuna decompile selam.exe 0x140002d50 --addr \
  --assert 'flow 0x140003d8f callreturn' ... (× the 11 INT 0x29 sites)
```

| | lines | synthetic ret-addr stores | declarations |
|---|---|---|---|
| today | 1267 | 63 | 159 |
| with the 11 sites cut | **1171** | **0** | **144** |

and the call the tester singled out becomes

```c
v15 = BCryptGenerateSymmetricKey(v52,&v55,v62,v54);
```

The second, independent confirmation: patching the 30 `MOV ECX,5 ; INT 0x29` sites in the image
to `MOV ECX,5 ; NOP ; NOP` gives 1121 lines and 0 synthetic stores. Two different interventions
at the same instruction, the same result.

### Breadth, on this binary alone

24 of 907 functions contain `INT 0x29` — it is the MSVC `/GS` + STL `_STL_VERIFY` idiom, so it
is in essentially every modern MSVC C++ image. Whole-set A/B with the same assertion:

```
func           #int29          lines      synth        decls
0x140002d50        11   1267->1171     63->0      159->144
0x140004560         2    478->476       0->0      179->179
0x1400017f0         2    172->165       1->0       31->41
0x1400022e0         1    105->94        4->0       21->21
0x14000750e         1     83->74        1->0       12->12
… 19 more …
TOTAL                   2802->2583     85->0      558->493
```

−7.8% lines, −11.6% declarations, and every one of the 85 synthetic stores gone. One function
regresses on declarations (`0x1400017f0`, 31→41): cutting the fastfail block splits variables
that were previously merged across it. That is a real cost and is recorded rather than hidden.

### The fix, and the alternative that was considered and not recommended

**Recommended.** A new `p2_lift/kuna_fastfailnoreturn.rs`, consulted from the one CALLIND seam
in `flow.rs` (`setup_callind_specs`, just before `build_call_specs(op, Address::default(), true)`
at `flow.rs:2167`). If the call's target varnode is defined by a `CALLOTHER` of the `swi` userop
with the constant `0x29`, and the image is a Windows PE, the call is marked no-return, so the
existing `check_for_flow_modification` plants the `artificialHalt(noreturn)` it already knows how
to plant. One module, one gated call site, no new pass type, gated behind an option
(`fastfailnoreturn`) per the transform-tier rule. The Windows gate matters: `int 0x29` is
`__fastfail` only on Windows.

**Rejected for now.** Fixing the *stack model* instead — do not apply `extrapop` to a `call`
whose instruction never pushed — is strictly more general (it would also cover `INT1`, `INT3`,
`INTO`, and `int 0x80`) and needs no Windows gate. It is not recommended as the first increment
because it changes the stack solver for every architecture and every `call` in the corpus, which
is a much larger blast radius than one no-return decision on one Windows opcode, and because
`INT3` genuinely *does* return, so the two fixes are not interchangeable. It belongs in its own
proposal after A ships.

## 3. Defect B — `REP STOSB` is not recovered, which is where the "impossible condition" lives

The condition the tester quoted is **faithfully in the binary**. MSVC emitted it:

```
0x14000385a   LEA RAX,[R12 + 0x6]
0x14000385f   CMP R12,RAX
0x140003862   JZ  0x140003870
0x140003864   MOV ECX,0x6 ; XOR EAX,EAX ; MOV RDI,R12 ; REP STOSB     ; memset(R12,0,6)
```

`first != last` where `last = first + 6`, which the compiler never folded. kuna reproduces it
honestly and then also fails to collapse the fill itself, so one `memset(p,0,6)` costs six lines
and two variables:

```c
if (v13 != (unsigned int *)((long long)v13 + 6)) {
  v19 = 6;  v29 = v13;
  while (v19) { v85 = (unsigned int *)((long long)v29 + 1); *(char *)v29 = 0; v19 -= 1; v29 = v85; }
}
```

There are **21 `REP STOSB` in this function** and 21 such loops in the output — roughly 105 of
the 1267 lines and a large share of the 165 anonymous variables. `memsetrecover` (default on)
does not fire: its summary is "a run of constant-fill stores (incl. inlined/SIMD bzero)", i.e.
the straight-line form, not the string-instruction loop. `builtin_memset` appears 0 times.

Two increments, in this order:

- **B1** — a fold for `INT_NOTEQUAL(V, INT_ADD(V, #c))` / `INT_EQUAL(…)` with `c != 0`, to
  `true` / `false`. Sound without qualification: in modular arithmetic `V + c ≡ V` iff
  `c ≡ 0`. A ~40-line rule in `p3_dataflow`, registered in `universal_sched`. This alone
  removes the exact text the need quotes.
- **B2** — extend `memsetrecover` (or a sibling `stringop` option) to the `REP STOSB` /
  `REP MOVSB` loop shape, collapsing each to one `builtin_memset` / `builtin_memcpy`. This
  subsumes B1 at these sites and is where the line count actually moves.

B2 is the bigger win and the bigger risk; B1 is cheap, independently useful, and closes the
quoted symptom on its own.

**Measured, so the decision log's open question is answered: no, the impossible condition does
not fall out of defect A's fix.** It is present at line 1249 today and at line 1157 with all 11
fastfail sites cut. Different mechanism, different increment.

## 4. Defect C — the Win32 API has no prototypes

Names resolve from the PE import table; signatures do not exist. After defect A is fixed the
arguments are clean values instead of stack blobs, but the *arity* is still wrong:

```c
BCryptDecrypt(v55,0,(int)v105 - (int)v56,&v60);   // takes 10 parameters
BCryptDestroyKey();                                // takes 1
```

`libcsigs` already seeds ~200 libc signatures on top of `libproto`'s 27. The same shape applied
to the common Win32 surface (`bcrypt`, `kernel32`, `advapi32`) is a self-contained analysis-tier
increment with no engine risk. It is listed for completeness and sequencing, not proposed for
this branch.

## 5. The acceptance probe asks for something this binary cannot honestly give

`a-f6e8100daa0b` requires the literal `std::string` in the output of `0x140002d50`. It cannot be
satisfied soundly, and the reason is not effort:

- **No debug info.** No PDB, no symbol table, no export names.
- **No `basic_string` RTTI.** The image carries 30+ MSVC RTTI type descriptors
  (`.?AVruntime_error@std@@`, `.?AV?$basic_ostream@DU?$char_traits@D@std@@@std@@`, …) and
  `basic_string` is not among them, because `std::string` is not polymorphic and therefore has
  no RTTI. `--option rtti on` cannot reach it.
- **This function constructs no `std::string`.** The MSVC SSO signature — `_Myres = 0xf` at
  `+0x18`, `_Mysize = 0` at `+0x10`, and the `CMP [x+0x18],0xf ; CMOVA` accessor — appears
  nowhere in `0x140002d50..0x140004550`; the sole `0xf` in the range is a displacement. The
  nearest string object lives in the *caller* at `0x140004a95`. So even a complete MSVC STL
  layout-recovery feature, the thing the need's title imagines and the thing
  `docs/features/c-string-objects-become/proposal.md` deferred, **would not fire here** and
  would not flip this probe.
- What the function actually takes is `a1 = {const char *, size_t}` — a `std::string_view`,
  built by the caller from an SSO string at `0x140004b4b..0x140004b6e`. Naming a recovered
  `{pointer, length}` pair `std::string_view` on that shape alone is a guess, and it would
  satisfy the probe's regex while being exactly the "spells one type `std::string` without
  removing any of the malformed C" outcome the decision log already warned about.

### The acceptance to use instead — both clauses verified in both directions today

```
stdout_absent:  \*\(unsigned long long \*\)&v\d+\[-8\] = 0x1        # 63 today,  0 with the fix
stdout_matches: BCryptGenerateSymmetricKey\(v\d+,                   #  0 today,  1 with the fix
exit_code eq 0
```

Both are name-agnostic (no SSA index is pinned), both were measured on the unmodified binary
via the `--assert flow … callreturn` build, and together they assert the two complaints defect A
actually closes. The decision log's candidate clause
`(\w+) != \(unsigned int \*\)\(\(long long\)\1 \+ 6\)` is correct and still present today — it
belongs on the **B** increment, not on A, and adding it to A's acceptance would make A
unclosable.

## 6. Plan, cost and risk

| # | increment | shape | risk |
|---|---|---|---|
| A | `fastfailnoreturn` — `int 0x29` is no-return on a Windows PE | 1 module + 1 call site in `flow.rs`, 1 `[[settable]]` row, 1 stages XML | low; gated, Windows-only, REMOVES CODE so it needs the two-pass stage test and a corpus sweep |
| B1 | fold `V != V + #c` | 1 rule in `p3_dataflow` | low; sound over modular arithmetic |
| B2 | `REP STOSB`/`MOVSB` → `builtin_memset`/`memcpy` | 1 module in the `memsetrecover` family | medium; a fill loop that is *not* a pure memset must not be collapsed |
| C | Win32 signature table | analysis-tier data + wiring, `libcsigs` shape | low; data-only |

**Speed.** Not measured — the `--assert` build proves the output, not the cost. A cuts flow and
therefore decodes strictly less, so it should not be able to lose; B2 adds a pattern match per
loop. Both need `scripts.pipeline.timeit` at implementation time and neither claim should be
taken on faith before that.

**Sequencing.** A first — it is the only one with two independent proofs and it is what unblocks
the call-argument recovery that C then completes. A and B1 are one PR each and could be done in
one session. B2 and C are separate.

**Why this is a proposal and not a PR.** Closing the need as written needs C++ type recovery that
would not fire on the witness (§5); closing what the tester actually reported needs three
independent changes in three phases. Either way it is past the one-focused-change bar. The
acceptance re-cut in §5 is the decision the captain has to make before A can be landed as a
closing PR — as written, A can be measured but not merged, because the merge gate requires the
acceptance to pass.

## 7. What was not done

- No engine code was written. Everything above is measured through `--assert flow … callreturn`
  and a byte-patched copy of the image, both of which reproduce the proposed fix's effect on the
  unmodified binary through mechanisms already in the tree.
- Defect C's breadth was not measured beyond this binary.
- Whether upstream Ghidra survives the `INT` stack unbalance was not checked; the fix proposed
  here does not depend on the answer.
