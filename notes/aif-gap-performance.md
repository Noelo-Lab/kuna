# AIF gap analysis performance

A small selected function can incur whole-image discovery cost. With aggressive
analysis, a 16-MiB ARM ELF containing 4,096 defined eight-byte functions and NOP
gaps retained about 1 GiB in the speculative decoder. Repeated unsuccessful
subroutine checks can also revisit thousands of cached instructions per candidate.

## Reproduce

This input is generated entirely from the source below. It needs Python 3 and GNU
ARM binutils. Run from a checkout with built kuna binaries and processor specs;
the output directory is outside the checkout.

```sh
mkdir -p ../tmp/aif-repro
python3 - <<'PY' | arm-linux-gnueabi-as -march=armv7-a -o ../tmp/aif-repro/gap.o
print('.text\n.arm')
for i in range(4096):
    print(f'.global f{i}\n.type f{i}, %function\nf{i}:\nmov r0, #7\nbx lr\n'
          f'.size f{i}, .-f{i}\n.word 0xe320f000, 0xe320f000')
print('.fill (16777216-4096*16)/4, 4, 0xe320f000')
PY
arm-linux-gnueabi-ld -Ttext=0x80000000 -e f0 \
  -o ../tmp/aif-repro/gap.elf ../tmp/aif-repro/gap.o
for listing in on off; do
  /usr/bin/time -f 'elapsed=%e maxrss=%M KiB' \
    decompiler/target/release/kuna decompile ../tmp/aif-repro/gap.elf f0 \
    --mode aggressive --option fast_funcdisc off --option listing "$listing"
done
```

Both modes must still emit `return 7;`. Repeat each mode three times and compare
median elapsed time and maximum peak RSS. Disable only `aif` in a third control
to distinguish the gap walk from the remaining Listing cost.

## Preserved behavior

The cursor still visits the same candidates in the same order. It retires only
cached results below that cursor, keeps the first decode of forward addresses,
and uses the original next-instruction boundary for subroutine validation.
Executable-range boundaries constrain cursor movement separately.

Validation reuses a visited-address hash set and work stack. An already-decoded
fixed-stride fall-through prefix represents its visited addresses, with deferred
branches and information flags replayed in their original order. The prefix
counts toward the unchanged 4,000-step limit and is included in accepted bodies.
It cannot skip a new decode or a processor-context update. This removes repeated
visitation work without changing discovery thresholds or imposing a cutoff.

The combined SLEIGH operation reuses the p-code parse for assembly only without
context commits or delay slots; otherwise it performs the original post-lift
assembly parse. Its eight-entry constructor-decision cache keys the complete
input buffer, context words, table and operand offset. Operand evaluation,
context updates and p-code generation still run at each address.

## Regression coverage

- Padding retention, activation at 20 functions, first-decode order, sparse
  executable ranges, forward/backward branches, loops and escapes.
- Overlapping validation prefixes, deferred branches, call information, both
  validation policies, the instruction limit and address overflow.
- Combined versus sequential decoding on focused ARM, Thumb, x86, SPARC and MIPS
  cases and all 16 existing golden lift fixtures, including errors and context.

The memory claim is a workload budget, not an absolute cap for arbitrary graphs:
forward cached results remain available until retirement is safe. Listing and
operand-reference costs are separate from this change.
