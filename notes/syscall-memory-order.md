# System-call memory ordering

The Linux read witness starts with a buffer value of 3, reads 100 through fd17,
and returns the saved value times 5 plus the final buffer. Native code returns 115;
current main emits two reads of the replaced buffer and returns 600. A separate
returned-length witness reads four and then two bytes, preserving both lengths
before reading the final buffer. Native code returns 570; the first memory-effect
implementation could still return 170 by folding the second syscall past that
buffer read.

The x64 rewrite marks only successful recognized ABI rewrites. Writable global
and stack storage receives opaque memory effects during heritage; P6 keeps saved
loads before these calls. Known enabled syscalls also receive a full expression
span check for LOADs and INDIRECT memory reads when folding their results.
Ordinary calls, unrelated userops, x64 argument recovery and syscall mode/image
policies retain their previous decisions.

Permanent regression coverage includes actual Linux pipes at O0/O2, saved pointer
and global expressions, two calls, stores the kernel observes before an overwrite,
and returned byte counts followed by a buffer read. The ARM and AArch64 object fixtures use
a signed 16-bit cast of the second byte count and a precise handler that writes the
buffer and returns the requested byte count;
they also assert separate call statements so correctness does not depend on a C
compiler's operand evaluation order. The focused stage assertions pass 24/24;
current main passes 19/24. Full inherited parity remains 675/675 and stage parity
is 1888/1888, with exactly five added assertions and no existing keys removed.

The extended Linux oracle covers 24 cases across GCC/Clang O0/O2/Os input images,
auto/on/abi output modes and GCC/Clang O0/O2 rebuilt C: 72 complete suites pass.
Six stripped coreutils images (true, false, sleep, env, printf, timeout) retain
byte-identical output across 871 functions. The default x64 loader has 299
functions, with four expected memory-effect changes at 0x12e70, 0x218e0, 0x26a00,
and 0x26e90.

Default speed on the full 299-function x64 loader, with 11 interleaved pairs
and no exclusions: main 49.361480s, candidate 50.411199s, +2.13% (minimum of
each 11 samples). Both used the normal make-binaries recipe and matching
dependency features; the raw run retained early contention samples.

The restored loader varargs setup at 0x26a00 exposes an existing caller-stack
address naming limitation: `&Stack0000000000000008` is emitted without a
declaration. The address is the actual `RSP + 8` computed by the instruction.
Keeping its real store fixes the memory model; producing a valid name for an
unnamed incoming stack address remains an emitter follow-up. A minimal write
syscall reproduces the same limitation while preserving an eight-byte buffer
initialization that main loses:

```sh
printf %s 488d44240848894424f0488d7424f0b801000000bf11000000ba080000000f05c3 | xxd -r -p > /tmp/incoming-stack.bin
kuna decompile /tmp/incoming-stack.bin 0x1000 --raw-image --target x86:LE:64:default:gcc --base 0x1000 --option x64syscall on
```
