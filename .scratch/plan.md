# The cast plan — ranked, from the three census reports

Source reports (read the section your lane cites): `/home/mahaloz/kwt/castcensus/.scratch/measure.md`
(the counter, the shared-set tables, the regression), `engine.md` (rule-by-rule origin in the engine, with
file:line anchors), `ida.md` (what IDA does instead, and where copying IDA would be wrong).

## The user's rule (verbatim, 2026-09-23)

> "One thing I am noticing across many samples is how often casting occurs in Kuna, compared to other
> decompiler, specifically IDA Pro. We should work do have less casting BECAUSE our typing system is better
> and requires less explicit casting because we already know what it is. Look at IDA Pro. Note: that does
> not mean just hide casts, it means meaningfully remove them when possible."

Operationally:
- A cast may disappear only because (a) the type became right, (b) the arithmetic is expressed in the
  pointer's own terms, or (c) C's own conversion rules already perform exactly that conversion on exactly
  those types. Never because output was suppressed. `nocastprinting` is not a lever.
- **A removed cast must not change the value the program computes.** Every lane proves this with compiled
  round-trip tests (gcc, and clang where it differs), not by argument.
- **Never weaken a type to lose a cast.** IDA avoids `(long)p` by typing 52% of parameters as a 64-bit
  integer; kuna types 60% as pointers. Retyping `void *a0` as `long a0` is hiding, and it would cost the
  type_match lead.
- IDA's value-truncating casts (`(unsigned int)&v7`, `(char)"Paul Eggert"`, varargs forced to
  `unsigned int`) are bugs. Do not copy them.

## The instrument (every cast lane uses it, both directions)

`/home/mahaloz/kwt/castbench/castbench.py` (the census counter wrapped as one tool):

```bash
export SLEIGHHOME=<wt>/specs
export KUNA_SPECS=<wt>/specs
B=/home/mahaloz/kwt/castbench
python3 $B/castbench.py run   --kuna <wt>/decompiler/target/release/kuna --out <wt>/.scratch/cb-on  [--option NAME VALUE]
python3 $B/castbench.py table --out <wt>/.scratch/cb-on --base $B/main-dbe854ba3     # vs IDA, vs main
python3 $B/castbench.py diff  --out <wt>/.scratch/cb-on --base $B/main-dbe854ba3     # per function, both directions
```

`$B/main-dbe854ba3/` is current main's arm (commit dbe854ba3), already run. If origin/main moves under you, build
it and re-run the base arm rather than comparing across a moved main. `--set small` (fmt/ls/sort/du ×
O0,O2) is the fast loop; report the `full` set (45 binaries × O0/O2/O2-noinline, about 4,800 shared
functions) in the PR.

Report in every PR: shared-set casts and casts per 1,000 lines for main and the branch, the ratio to IDA,
functions with fewer casts and functions with MORE casts, and the reason for every "more" function in the
top 25. The gap is two-sided: kuna already casts less than IDA on 36% of functions. A change that regresses
those is not a win.

The cast count is not the decbench metric. Every lane also runs the 444-slice typesweep (the brief's
recipe, metric pinned per final-c/pindb.py). A cast lever must not lose perfect functions, and every worse
row must be read.

## Baseline (main 796939d54, 4,815 shared functions)

| | casts | /kloc | vs IDA |
|---|---:|---:|---:|
| IDA | 37,821 | 155.4 | 1.00 |
| kuna at the campaign baseline | 40,529 | 211.3 | 1.07 |
| kuna main | 44,821 | 235.2 | 1.19 |

## Ranked levers

### 1. Keep pointer arithmetic in pointer terms (lane `castarith`)
**3,771 casts at the `*(T *)((long)p + …)` shape, 2,944 of them the simple `p + K` form; 36% of all of
kuna's casts; IDA's share of this idiom is 1.1%.**
Today: `*(unsigned int *)((long)a0 + 0xac)` (2 casts) where IDA prints `*((unsigned int *)a0 + 0x2b)`
(1 cast), and `a0[0x2b]` when the pointee width equals the access width (0 casts). Of the constant-offset
sites, 99.8% have `sizeof(T) | K`. Origin (engine.md §4.1): `get_input_cast` default arm for `CPUI_INT_ADD`
(`coreaction_casts.rs:750`) → `cast_standard` `cast.rs:815`, plus `cast_output` for the INT_ADD result.
PTRSUB→INT_ADD demotion fires 0 times on this corpus, so these ops were INT_ADD all along, because a
`void *` has no field at K to build a PTRSUB against.
- Lower an `INT_ADD(p, K)` whose pointer operand `p` is pointer-typed and whose result is used as `T *`
  into pointer arithmetic on `(T *)p` scaled by `sizeof(T)` when it divides K. The result is
  `*((T *)p + K/sizeof T)`, or `p[K/sizeof T]` when p is already `T *`. Choose the rendering with the
  `arraynotation` convention in mind, and state the choice. Non-dividing K keeps today's form (0.2%).
- Expressions that are not dereferenced (`(T *)((long)p + 0x10)` passed as an argument) follow the same rule.
- Implementation seam: the lane's choice. Candidates are a P5 rewrite of the INT_ADD into PTRADD over a
  CAST, before ActionSetCasts runs, or a P9 rewrite in the kuna layer next to `cast_fixup_ptradd`. The
  value must be identical. Prove it with compiled round-trip tests over widths 1/2/4/8, signed and
  unsigned, negative K, K not divisible, a base that is itself a cast, and a pointer compared or
  subtracted afterwards.
- **Do NOT flip `ptrfromuse` to `byte`.** engine.md measured `byte` as metric-free on 18 slices that
  exclude coreutils. The `void` default was chosen because it moved **204** functions onto perfect over
  the 444 slices: qsort comparators and hash callbacks whose declared parameters are `void const *`. The
  `void *` stays, and the lowering stops laundering it through `long`.
- Option-gated (a Ghidra divergence: upstream prints the `(long)` form, and the datatests may pin it).
  Default per the flip procedure.

### 2. Give the `void *` bases a record (lane `structdeep`, already in flight)
**1,817 `void *`-based sites. A record turns 2 casts into 0 (`a0->field_0xac`) and improves the type.**
ls O0 `sub_5297(void *a0, struct_4 *a1)`: every access through `a1` is cast-free and every access through
`a0` costs two. Which base gets a record decides it. structdeep's "more bases" census must say, for the
void-base sites in the cast census, why structsynth declined each base, and report the cast delta beside
layout precision and typesweep. Precision stays the gate: a wrong record is worse than a cast.

### 3. Name every referenced static address (lane `castglobals`)
**1,118 casts on constant addresses (IDA: essentially none); grew from 572 to 1,118 during the campaign.**
`sub_e4ca((struct_2 *)0x2b080)` where IDA prints `&stru_2B080`. kuna mints `dat_<addr>` two lines away for
another global. Origin: `push_constant_ir`'s TYPE_PTR arm (`printc.rs:7622-7628`), where a
pointer-typed constant never became a symbol reference. The symbol has to come from a tier that can
create it (memory: every stock cspec claims all of ram as global scope, so `global add` is a measured
no-op). Mint a typed global symbol for a constant address that is used as a pointer into a loaded
section, and print it as `&dat_2b080` / `dat_1234`. `*(unsigned long *)0x1234` becomes `dat_1234` with
`unsigned long dat_1234`. Refuse: an address outside every loaded section, a constant that is also used
as a number, and conflicting access widths without a record. The C must still compile (the global is
declared in `decompile-project` output). Option-gated. Check `inferfuncentry` (#694), which treats
BUFSIZ 0x2000 as a function, and the `dat_` naming paths.

### 4. Drop casts that C's own conversion rules already perform (lane `castimplied`)
**About 750 strictly removable sites.** The subsets: an extension feeding a call argument whose declared
parameter type is the extension's output type (154 at O0). An extension or cast at the statement top
assigned to a variable declared with exactly that type (238). `return` of the declared return type (57).
The inner cast of a stacked pair the outer one subsumes (up to 410, but only where the inner cast is a
pure widening the outer repeats; `(int)(unsigned char)x` is not removable). `memchr(v10,(int)v4,…)` where
memchr's parameter is `int` and `v4` is `char`: the cast is the conversion C performs. Origin (engine.md
§4.2): `is_extension_cast_implied` (`cast.rs:610`, a verbatim upstream port) returns false for a `None`
reader, for CALL/RETURN/COPY readers, and on any metatype mismatch.
- Build it as a kuna predicate in a new `p9_emit/kuna_<slug>.rs`, next to the `kuna_typeround`
  `SignPlan::drop_cast` precedent (`printc.rs:5933`). Leave the ported `CastStrategyC` and its pinned
  tests (`verify_w10_cast_strategy_adversarial.rs`, `p9_emit/cast/tests.rs`) untouched.
- Hard requirements: the prototype must be trusted (a declared or measured-libc signature, or a locked
  recovered one; never a trial), never varargs positions, never `kuna_truncarg`'s deliberate argument
  casts (#678), never `boolbyte`'s `(bool)(unsigned char)` (#680), and never a narrowing, sign-flip, float,
  or pointer↔integer conversion. Every dropped cast has a round-trip test showing the same value.

### 5. Type a local by its ordering uses (lane `castsign`)
**About 596 casts: `if (0 <= (long)v15)` with `unsigned long v15`.** kuna declares unsigned and the program
compares signed. The opposite bias (IDA's) costs 587 in the other direction. Type the local signed when
every ordering use is signed and no use needs it unsigned (an unsigned compare, a logical shift right,
division, or widening by ZEXT that is actually read). Owner: `signedness` / `kuna_typeround`. Worth zero
on type_match, so the gate is the cast count, the typesweep not moving, and round trips. Option or a new
value of `signedness`.

### Later (not in this round)
- Split partially written variables instead of `*(T *)&v = x` (1,035 gap vs IDA; needs real variable
  splitting; never adopt IDA's `LOBYTE` macros, which are not C).
- `(char *)<call>`: 2,048 vs IDA 1,187, from unknown libc return types. This belongs to the
  libcwiden/libctypes line (`libctypes=glibc` already removes 1.5%).
- Stack slots read at another width (`*(long *)&v`: +1,139 vs base).

## Order and interactions

- castarith and structdeep overlap on the void-base sites. Whichever lands second re-measures on top of
  the first; neither claims the other's casts.
- castimplied and castsign both touch `p9_emit` / `kuna_typeround`. castsign is chained after
  castimplied.
- Every lane rebases onto the current origin/main before measuring its final numbers, and re-runs the
  castbench base arm if main moved.
