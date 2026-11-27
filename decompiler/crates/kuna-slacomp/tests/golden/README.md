# Compiler oracle

`compiler.sha256` pins the SHA-256 digest of each decompressed `.sla` element
stream produced by Ghidra's C++ `sleigh_opt` at revision
`cef869af04c4740a71ad31a55704045b1b0d1644`. Compression bytes are excluded because
flate2 and zlib can compress the same stream differently. The manifest covers
53 specs, including both endiannesses, Toy variants, major processor families,
Hexagon's named sections and crossbuilds, and inherited context assignments
across nested `with` blocks. The named-bitrange fixtures exercise reads and
writes of one-bit, nine-bit and byte-aligned aliases in both endiannesses.
The terminal-pattern fixture covers overlapping patterns resolved by their
intersection, disjunctions and nested specializations.

The Rust test compiles the listed sources and compares directly with these
digests. It needs no prebuilt `.sla` files, Ghidra installation or C++ toolchain.
The other golden text files pin scanner, parser and macro-expansion traces.

The Octeon integer variants were added using the recorded C++ compiler. The
generic MIPS32/MIPS64 entries were regenerated because the conditional Octeon guards
move source line numbers stored in the element stream. That compiler reproduces
the old MIPS digests on the previous sources; the generic constructors retain
their meanings. The Rust compiler matches all six new element streams.

The Octeon save/restore exclusion updates these six entries again: the generic
MIPS streams reflect moved source locations, and the Octeon streams exclude the
colliding constructors. The recorded native compiler reproduces all six prior
digests on the prior sources and supplies all six replacement digests. The
atomic-refusal and generic save/restore tests also run against its output.

The Octeon atomic memory operations update these six entries again. The pinned
native compiler reproduces all six previous digests and matches the Rust
compiler on the updated sources. Comparing its generic MIPS XML outputs before
and after the change finds only constructor `line` attributes changed by the
conditional guards; their instruction semantics are unchanged. The Octeon
streams include the generation-specific atomic and ordering operations.
Removing the `syncio`/`syncioall` names for SYNC selectors 1 and 3 updates
the four Octeon entries only. The pinned native compiler reproduces their four
previous digests on the previous sources and matches the Rust compiler on the
updated ones.

The generation-specific Octeon II/III kernel scratch selectors update the six MIPS entries. The
pinned C++ compiler reproduces the prior hashes on the base and yields the
same six updated element streams as the Rust compiler. The generic MIPS32
and MIPS64 XML outputs differ only in constructor source line numbers.

The SPARC entries include kuna's destination corrections for `fdtoi`, `fqtoi`,
`fstox` and `fqtox`. The first three were regenerated with Ghidra's C++
compiler on the corrected sources, which reproduces the previous hashes on the
unmodified sources; the `fqtox` correction was regenerated with `slacomp`,
which reproduces those C++ digests. The current digests also cover the branch
conditions sampled before a delay slot (`icc`/`xcc` `cs`, `neg` and `vs`, and
the non-annulled register branches). They were regenerated with the recorded
C++ compiler, which reproduces the previous SPARC digests on the previous
sources; `slacomp` produces the same digests.

The x86 entries include the width-specific x87 integer-conversion operations,
the control-word writes of `FXRSTOR` and `XRSTOR`/`XRSTORS`, and the signed
BOUND operand loads and bounds-check user-ops.
They were regenerated with the recorded C++ compiler after checking that it
reproduces both unmodified x86 hashes; kuna’s compiler produces the same changed
element streams.
The combined digests were produced with `slacomp`, which reproduces the
pinned C++ digests for each of those changes applied alone.
The three x86 digests were regenerated with `slacomp` again for the UINTR
group (`uintr.sinc`), the RIP-relative `PREFETCHIT0/1` constructors and the
VSIB-operand `VGATHERPF` constructors; `slacomp` reproduces all three
previous digests on the previous sources.
The three x86 digests were regenerated with `slacomp` again when the VMX
instructions gained their status flags (and VMREAD/VMWRITE their REX.W-free
64-bit decode); `slacomp` reproduces all three previous digests on the
previous sources.
`x86-16.slaspec` (16-bit x86 with the real-mode code segment in the decode
context) was added with a digest from the recorded C++ compiler, which
`slacomp` matches. Its `ia.sinc` hook replaces three blank lines, so the
`x86.slaspec` and `x86-64.slaspec` digests are unchanged, source lines
included.
Its digest was regenerated with the recorded C++ compiler, which `slacomp`
matches, when 16-bit x86 gained the 8087-emulator decode: the x87 ESC
constructors moved into an `x87` table there. Their `ia.sinc` lines changed
in place (each quoted mnemonic and `X87_Dn` macro expands back to the old
text outside 16-bit x86, and the table's `with` block is under `@ifdef IA16`),
so the `x86.slaspec` and `x86-64.slaspec` digests are again unchanged.
The `x86-64.slaspec` digest was regenerated with `slacomp` when the decode began to
ignore a REX prefix that comes before a legacy prefix, as the CPU does. `slacomp`
reproduces all three previous x86 digests on the previous sources. The recorded C++
compiler also reproduces them, and it gives the same new `x86-64.slaspec` digest on
the updated sources. The constructor
replaces two blank lines under `@ifdef IA64`, so the `x86.slaspec` and
`x86-16.slaspec` digests are unchanged, source lines included.

The AArch64 entry includes kuna's rounding-mode corrections for the
`FCVT{A,M,N,P}{S,U}`, `FRINT*` and `FRINT32/64*` constructors, their four
rounding user ops, and the `FRINT32/64*` zeroing of the register bits above
the result. Its digest was regenerated with the recorded C++ compiler,
which reproduces the previous AArch64 digest on the unmodified sources;
`slacomp` produces the same digest.
It also includes the pass-through copy (`Rd = Rd`) the `ShowPAC=0` forms
of the pointer-authentication instructions that write a general register
now lift to. The digest was regenerated the same way: the recorded C++
compiler reproduces the previous digest on the previous sources and
`slacomp` matches it on the updated ones.
It also includes the memory-tagging constructors under `ShowMemTag=1`, whose
tag operation is now a user op producing the tagged pointer (`AddPtrTag`,
`ExcludePtrTag`, `LoadPtrTag`, `StorePtrTag`), without the alignment-fault
branch. That digest was regenerated with `slacomp`, which reproduces the
previous AArch64 digest on the previous sources.
It also includes the `W`-destination sign-extending loads (`ldrsb`, `ldrsh`,
`ldursb`, `ldursh`, `ldtrsb`, `ldtrsh`), which now sign-extend to 32 bits and
zero bits 63:32. The digest was regenerated with `slacomp`, which reproduces
the previous AArch64 digest on the previous sources and reports the same
compiler warnings on both.

The RISC-V entry includes the Zcmp/Zcmt constructors (`riscv.zc.sinc`), the
`Zcmp` context bit and the `c.fsdsp` guard on it. Its digest was regenerated
with `slacomp`, which reproduces the previous RISC-V digest on the unmodified
sources.

The ARMv8 little-endian entry includes the Thumb `stmia` base writeback that
applies when the base is in the register list. Its digest was regenerated with
`slacomp`, which reproduces the previous ARM digest on the previous sources.

The ARMv8 little-endian entry includes the carry-out the Thumb-2 `ands`,
`bics`, `movs` and `mvns` constructors now set from their shifter operand. Its
digest was regenerated with `slacomp`, which reproduces the previous ARM
digest on the previous sources.

The ARM entry includes the carry of the VFP compares (`FloatVectorCompare` in
`ARMneon.sinc`), set for unordered operands as `!tNG`, the negation of the
macro's own `op1 f< op2`. The edit replaces one line and adds no temporary, so
no source line and no unique offset moves. Its digest was regenerated with
`slacomp`, which reproduces the previous ARM digest on the unmodified sources.

`xml_ops.xml` is the same pinned C++ compiler's `-y` output for
`snips/xml_ops.slaspec`. The CLI test compares XML output byte-for-byte in both
single-file and recursive modes, including the `BUILD` and `INT_ADD` opcode
names.

## Regeneration

Regenerate only for a reviewed compiler or upstream-spec change. Build
`sleigh_opt` from the recorded Ghidra revision using its C++ directory's
`make sleigh_opt` target. From the kuna repository root, run the following with
that executable's absolute path. Do not generate expectations with kuna's own
compiler.

```bash
oracle=/absolute/path/to/sleigh_opt
python3 - "$oracle" > /tmp/compiler.sha256 <<'PY'
import hashlib
from pathlib import Path
import subprocess
import sys
import tempfile
import zlib

manifest = Path("decompiler/crates/kuna-slacomp/tests/golden/compiler.sha256")
with tempfile.TemporaryDirectory() as scratch:
    output = Path(scratch) / "compiled.sla"
    for line in manifest.read_text().splitlines():
        if line.startswith("#"):
            print(line)
            continue
        _, source = line.split("  ", 1)
        subprocess.run(
            [sys.argv[1], str(Path(source).resolve()), str(output)],
            check=True,
            stdout=subprocess.DEVNULL,
        )
        encoded = output.read_bytes()
        assert encoded.startswith(b"sla\x04")
        digest = hashlib.sha256(zlib.decompress(encoded[4:])).hexdigest()
        print(f"{digest}  {source}")
PY
diff -u decompiler/crates/kuna-slacomp/tests/golden/compiler.sha256 /tmp/compiler.sha256
```

Review the differences before replacing the manifest. Record a changed oracle
revision in both this file and the manifest header.

Regenerate the XML fixture with the same compiler:

```bash
"$oracle" -y decompiler/crates/kuna-slacomp/tests/golden/snips/xml_ops.slaspec /tmp/xml_ops.sla
diff -u decompiler/crates/kuna-slacomp/tests/golden/xml_ops.xml /tmp/xml_ops.sla
```
