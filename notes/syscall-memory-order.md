# System-call memory ordering

The Linux read witness starts with a buffer value of 3, reads 100 through fd17,
and returns the saved value times 5 plus the final buffer. Native code returns 115;
unfixed code emits two reads of the replaced buffer and returns 600. A separate
returned-length witness reads four and then two bytes, preserving both lengths
before reading the final buffer. Native code returns 570; the first memory-effect
implementation could still return 170 by folding the second syscall past that
buffer read.

The x64 rewrite marks only successful recognized ABI rewrites. The new memory
effects additionally require the exact GCC AMD64 target, ordinary SysV function
and evaluation models, and the real eight-byte RSP stack. Within that domain,
writable global and stack storage receives opaque memory effects during
heritage, and P6 keeps saved loads before these calls. Windows and MSABI models
retain their prior register-only rendering. Both memory effects and ordering
checks use the same admission; the rewrite marker alone is insufficient.

Known enabled syscalls also receive a full expression span check for LOADs and
INDIRECT memory reads when folding their results. CALLOTHER already carries
call flags; the missing protection was the intervening memory-read check through
the final expression consumer. Ordinary calls, unrelated userops, x64 argument
recovery and syscall mode/image policies retain their previous decisions.

Permanent regression coverage includes actual Linux pipes at O0/O2, saved pointer
and global expressions, two calls, stores the kernel observes before an overwrite,
and returned byte counts followed by a buffer read. The ARM and AArch64 object
fixtures use a signed 16-bit cast of the second byte count and a precise handler
that writes the buffer and returns the requested byte count;
they also assert separate call statements so correctness does not depend on a C
compiler's operand evaluation order. The focused stage assertions pass 24/24;
unfixed code passes 19/24. Full inherited parity remains 675/675. The stage
baseline adds exactly five assertions to the inherited 1900, removing no keys.

The extended Linux oracle covers 24 cases across GCC/Clang O0/O2/Os input images,
auto/on/abi output modes and GCC/Clang O0/O2 rebuilt C. Windows, effective MSABI,
explicit MSABI prototypes, and locked SysV prototypes with MSABI evaluation
have output-preservation controls across default, aggressive, on, abi and off.
Enabled excluded output must also compile with GCC and Clang.

The physical caller-pointer test uses assembly to pin the caller's stack
pointer. Actual Linux write syscalls must send that pointer plus the instruction's
offset through a pipe, both from original assembly and from emitted C. It covers
incoming offsets 8 and 40, ordinary and project exports, and GCC/Clang O0/O2.
The emitter's CFA expression and noinline function preserve the caller address;
a newly allocated local would produce a different payload.

The loader varargs setup at 0x26a00 retains the incoming-stack pointer's store.
The supported SysV emitter renders that actual address using its CFA expression,
so the restored output compiles. A minimal write syscall demonstrates the same
eight-byte buffer initialization that unfixed code loses:

```sh
printf %s 488d44240848894424f0488d7424f0b801000000bf11000000ba080000000f05c3 | xxd -r -p > /tmp/incoming-stack.bin
kuna decompile /tmp/incoming-stack.bin 0x1000 --raw-image --target x86:LE:64:default:gcc --base 0x1000 --option x64syscall on
```
