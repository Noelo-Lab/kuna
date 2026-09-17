# indirectonly — the counterexample that keeps the option off

`sources in counterexample/`

## 1. What goes wrong

`merge_test_adjacent`'s illegal-input refusal is lifted for an input that is only
read through INDIRECTs. That is sound when the illegal input is the **destination**
of the copy — the machine really does store into the frame slot, and the merge only
moves where the value is computed. It is not sound in the other direction: when the
machine *loads* an escaped frame slot into a register and then mutates the register,
the merge makes the emitted C mutate the slot, so the C claims a store the machine
never performs. If the enclosing object's address reaches a later call, that call
reads a value that never existed.

## 2. Repro

`counterexample/walk.S` is the shape, reduced to 40 lines: an array object is built
on the frame, three calls fill it through its address, the element count is then
**loaded into r14d** and counted down in the register, and after the loop the
object's address goes to a fourth call that reads the count.

```
gcc -shared -fPIC -o walk.so counterexample/walk.S
gcc -O2 -c counterexample/lib.c -o lib.o
gcc -o prog counterexample/main.c walk.so lib.o -Wl,-rpath,$PWD && ./prog
```

Ground truth:

```
sink 0x3
sink 0x2
sink 0x1
a_done sees n=3
```

kuna, default (`indirectonly` off) — the count is a register local and the slot is
only read:

```c
void walk(unsigned long *a0)
{
  long v1; // stack - 0x38
  int v2; // r14d
  int v3; // stack - 0x30

  a_init(&v1,3);
  a_push(&v1,*a0);
  a_push(&v1,a0[1]);
  a_push(&v1,a0[2]);
  v2 = v3 + -1;
  ...
  a_done(&v1);
}
```

kuna, `--option indirectonly on` — r14d is merged into the slot, so the C
decrements memory that the machine never writes and hands it to `a_done`:

```c
void walk(unsigned long *a0)
{
  long v1; // stack - 0x38
  int v2; // stack - 0x30

  a_init(&v1,3);
  a_push(&v1,*a0);
  a_push(&v1,a0[1]);
  a_push(&v1,a0[2]);
  v2 -= 1;
  ...
  a_done(&v1);
}
```

## 3. The two bodies disagree when compiled

`counterexample/roundtrip.c` transcribes both emitted bodies against the same
`lib.c`, so the only difference is the merge:

```
gcc -O1 -o roundtrip counterexample/roundtrip.c counterexample/lib.c counterexample/sinkdef.c && ./roundtrip
-- walk_off (option off / origin-main output) --
a_done sees n=3     <- matches the binary
-- walk_on  (option on) --
a_done sees n=-1    <- wrong
```

## 4. Why the cover machinery does not catch it

`analysis.md` used to argue that a call which may read or write the slot attaches a
`CPUI_INDIRECT` whose cover overlaps the register, so `merge_test_required`'s
cover-intersection test rejects the merge. That argument is false, and the raw
p-code says why (`print raw` on `walk`, option off, abridged):

```
0x1150:12e:  s0x...ffd0:4(0x1150:12e) = s0x...ffd0:4(i) [] i0x1150:18(free)   ; INDIRECT at a_init
0x115b:12f:  s0x...ffd0:4(0x115b:12f) = s0x...ffd0:4(0x1150:12e) [] ...        ; INDIRECT at a_push
0x1167:130:  s0x...ffd0:4(0x1167:130) = s0x...ffd0:4(0x115b:12f) [] ...        ; INDIRECT at a_push
0x1173:131:  s0x...ffd0:4(0x1173:131) = s0x...ffd0:4(0x1167:130) [] ...        ; INDIRECT at a_push
0x117d:33:   R14D(0x117d:33) = s0x...ffd0:4(0x1173:131) + #0xffffffff:4        ; the load, the LAST read
...
0x118e:12a:  s0x...ffc8(0x118e:12a) = s0x...ffc8(0x1183:12c) [] i0x118e:45     ; INDIRECT at sink -- for -0x38 only
0x118e:45:   call fsink(free)(u0x...(0x118a:41))
0x11a0:5f:   call fa_done(free)(RSP(0x113d:157))                               ; NO INDIRECT at all
```

A CPUI_INDIRECT is attached only where the storage is still **live in the SSA**.
`stack-0x30`'s last read is the load at `0x117d`, so heritage stops guarding it:
there is no INDIRECT for it at `call sink` and none at `call a_done`, its cover ends
at `0x117d`, and the cover-intersection test has nothing to intersect. The slot at
`-0x38` is still live (it is dereferenced in the loop) and does get an INDIRECT at
`sink` — which is exactly why only the dead-after-read half is vulnerable.

There is also no p-code representation of a callee *reading* an escaped slot at all:
INDIRECT models a possible write. So the information the merge would need is not in
the IR at this point, and no local test can recover it.

## 5. This is upstream behaviour, not a porting error

Stock **Ghidra 12.1.2** (`analyzeHeadless`, default analysis, `DecompInterface`) on
the same `walk.so`:

```c
void walk(undefined8 *param_1)

{
  long local_38;
  int local_30;

  a_init(&local_38,3);
  a_push(&local_38,*param_1);
  a_push(&local_38,param_1[1]);
  a_push(&local_38,param_1[2]);
  local_30 = local_30 + -1;
  if (-1 < local_30) {
    do {
      sink(*(undefined8 *)(local_38 + (long)local_30 * 8));
      local_30 = local_30 + -1;
    } while (local_30 != -1);
  }
  a_done(&local_38);
  return;
}
```

Identical fabricated store. `markIndirectOnly`/`checkIndirectUse` and
`mergeTestAdjacent` are ported opcode-for-opcode, so kuna with the option **on**
reproduces Ghidra exactly; kuna with the option **off** — the shipped default — is
the arm that does not fabricate the store.

## 6. What is pinned

* `tests/stages/kuna-indirectonly-escape.xml` — the same shape as a bytechunk
  (`counterexample/stage-escape.S` is the source). Pass 1 runs the **default**, so
  flipping the shipped default back to on turns the test red; pass 2 asks for the
  option and pins what it costs.
* `tests/stages/kuna-indirectonly.xml` — the sound direction, unchanged.
