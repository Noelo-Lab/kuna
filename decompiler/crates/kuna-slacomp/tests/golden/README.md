# Compiler oracle

`compiler.sha256` pins the SHA-256 digest of each decompressed `.sla` element
stream produced by Ghidra's C++ `sleigh_opt` at revision
`cef869af04c4740a71ad31a55704045b1b0d1644`. Compression bytes are excluded because
flate2 and zlib can compress the same stream differently. The manifest covers
48 specs, including both endiannesses, Toy variants, major processor families,
Hexagon's named sections and crossbuilds, and inherited context assignments
across nested `with` blocks. The named-bitrange fixtures exercise reads and
writes of one-bit, nine-bit and byte-aligned aliases in both endiannesses.
The terminal-pattern fixture covers overlapping patterns resolved by their
intersection, disjunctions and nested specializations.

The Rust test compiles the listed sources and compares directly with these
digests. It needs no prebuilt `.sla` files, Ghidra installation or C++ toolchain.
The other golden text files pin scanner, parser and macro-expansion traces.

The SPARC entries include kuna's destination corrections for `fdtoi`, `fqtoi`,
`fstox` and `fqtox`. The first three were regenerated with Ghidra's C++
compiler on the corrected sources, which reproduces the previous hashes on the
unmodified sources; the `fqtox` correction was regenerated with `slacomp`,
which reproduces those C++ digests.

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
