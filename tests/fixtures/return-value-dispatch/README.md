# Return-value dispatch regressions

Original MIPS N32/N64 assembly, including freestanding Linux callers. No vendor
code is included. `narrow` visits a JR dispatcher three times; its delay-slot
update returns unsigned int 3, including the last visit through a narrowing
SUBPIECE. `foreign-copy` copies a0 to v0 in the delay slot, adds 7 to a0 and
re-enters to copy the updated value. Its caller checks 73 becomes 80.

The raw `.bin` contains only `bounce`. `build.py` accepts the same explicit GNU
tool paths and external artifact directory as `../indirect-dispatch/build.py`;
`provenance.json` binds sources, linked images, extracted bytes and tool hashes.
Native CLI replay checks 10,000 unsigned values at O0/O2 with UBSan. The original
linked callers are independently executable with QEMU Octeon68XX.
