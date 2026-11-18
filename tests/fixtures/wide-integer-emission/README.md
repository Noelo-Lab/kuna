# Defined integer-emission fixtures

Original C with explicit unsigned wrapping, compiled by LLVM for x86-64,
AArch64, PPC64 and MIPS64. `shift31` stores its original signed input before shifting;
`shift63` covers the maximum fixed 64-bit count. Neither input is a widening
SEXT. `word_wrap` wraps at 32 bits before sign extension. The x86 object also
checks that a shifted call result calls its producer exactly once.

Rebuild with `python3 build.py --compiler /path/to/clang`. The ELF files are
relocatable objects; `provenance.json` binds the source, compiler, commands and
outputs. The CLI regression compiles emitted C with O0/O2 and UBSan over signed
boundary bit patterns. Original x86 code is independently executable on the
host; original AArch64/PPC64 execution is a separate qualification boundary.

MIPS64 uses the soft-float ABI marker for automatic Octeon selection; these
fixtures contain only integer operations. Widened 32-bit and direct 64-bit inputs
are checked separately. AArch64 lowers one extension into unsigned pieces; the
stage assertion records that path separately from the three retained SEXTs.
