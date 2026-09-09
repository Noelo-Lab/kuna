---
need_id: neopack-payload-requires-external
title: NEOpack payload requires external decompressor execution
track: loader
status: open
severity: major
probe_id: p-c9020a5d7fe1
acceptance_id: a-74d47ac7b3b6
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Obtain an analyzable payload from this packed PE.

> **NEOpack payload requires external decompressor execution** (major, `5ab77f5433c5d40ad448c1c1`)
> unpack exits 1: no UPX PackHeader found. Help offers UPX and raw LZMA only. Executing the loader's decompressor in Unicorn recovered four sections. Already-filed hit: block-processing-panics-out occurred on the first reconstructed image with overly broad executable flags; not re-filed.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "no UPX PackHeader found"
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "(?i)unpacked"
    ],
    "stdout_absent": [
      "no UPX PackHeader found"
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5433c5d40ad448c1c1` (round 12, tester t-r12-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `missing-capability|?|exit_code,stdout_is_json` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **overturned** (was inconclusive). REFUTER RAN (captain, in-tick, 2026-09-09 06:3xZ). NO HYPOTHESIS WAS EVER FILED ('_none offered_'), so what is overturned here is the NEED'S FRAMING AND ITS ACCEPTANCE, both of which are measurably wrong. The symptom itself reproduces exactly.

WHERE THE BINARY ACTUALLY IS: the arena target is a DIRECTORY, .kuna-repipe/arena/12/5ab77f5433c5d40ad448c1c1/target/CryptoME.zip.__x/, and the PE is CryptoME.exe inside it (PE32 GUI i386, 139776 bytes). Pointing kuna at the target path itself gives 'could not read ... No such file or directory', which is not the defect.

SYMPTOM REPRODUCED: kuna unpack <CryptoME.exe> --json -> rc=1, 'no UPX PackHeader found'.

THE PACKER IS NEOLITE, NOT AN LZMA STREAM. Section table read straight out of the PE:
  .text    va=0x001000 vs=0x00f000 raw=0x000600 rs=0x008e00
  .rdata   va=0x010000 vs=0x004000
  .data    va=0x014000 vs=0x044000
  .rsrc    va=0x058000 vs=0x09f000
  .NEOpack va=0x0f7000 vs=0x002000 raw=0x020800 rs=0x001a00
  .NEOdata va=0x0f9000 vs=0x001000 raw=0x022200 rs=0x000000
entrypoint 0xf7001, i.e. INSIDE .NEOpack -- the loader stub. NEOLite carries a proprietary LZ, not LZMA, which is why the tester could only recover the four sections by EXECUTING the stub's decompressor under Unicorn rather than by decoding a stream. kuna's existing --raw-lzma escape hatch therefore does not reach this file, and 'four sections recovered' is the count of the four real sections above, not of a payload blob.

THE ACCEPTANCE IS TRIVIALLY SATISFIABLE AND WOULD SHIP NOTHING. It asserts only exit_code == 0 and stdout_is_json, with NO clause on what was recovered. A one-line change that makes unpack report 'not packed' with rc 0 turns it green. Compare its three CLOSED siblings, every one of which has a content clause:
  headerless-lzma-payload-unpacked (#508)  json unpacked_size eq <n>
  upx-lzma-compression-blocks (#498)       json count gt 0
  unpack-proceed-without-upx (#488)        -o {{TMP}}/out.bin, in-repo fixture
This need is the only one of the four with no content clause. BEFORE IT IS EVER DISPATCHED, ITS ACCEPTANCE NEEDS A CLAUSE ON THE RECOVERED IMAGE -- at minimum count gt 0 plus an -o output, ideally the .text virtual size 0x f000 or a byte in the recovered image.

THE REPRO PROBE DOES NOT DISCRIMINATE THE DEFECT EITHER, AND THE ACCEPTANCE COLLIDES WITH THE SIBLINGS. Measured: kuna unpack exits 1 with the identical 'no UPX PackHeader found' message on an ORDINARY ELF .so (/tmp/ctl.so) and on an ordinary unpacked PE (round 12's vm.exe). So the repro clause matches every non-UPX binary in existence, packed or not. That also means the naive route to the acceptance -- make unpack exit 0 when it finds no PackHeader -- is not merely empty, it would flip the rc-1 clause of every sibling unpack probe at once.

RE-SCOPE IT. Filed as scope small / track tooling / touches kuna-cli. The evidence says otherwise: recovering this image needs either a NEOLite-specific decompressor or generic stub emulation, and the tester's own successful route was a Unicorn emulator. That is PROPOSAL-sized and it is engine work, not CLI work. The honest small deliverable, which is worth doing and is a different need, is RECOGNITION: unpack sees .NEOpack/.NEOdata and an entrypoint inside .NEOpack, and says 'NEOLite-packed; kuna cannot decompress this format' instead of 'this file is not [UPX]' -- naming the packer is what an agent needs to stop guessing, and it is a message change with no engine risk.
- round 12 captain: acceptance_id corrected a-ecb1adba078b -> a-74d47ac7b3b6. The id is derived from cmd+expect; an earlier tightening rewrote the assertion without recomputing it, so the front-matter label no longer named the assertion that runs. Body unchanged -- this is a relabel only. It matters at promotion (the opportunity selector is the acceptance_id) and at dedup (cluster.py trusts a stored id over recomputing, so a re-filing of the same probe would have opened a duplicate need instead of adding an instance).
- round 12 BUILDER: closed by a static NEOLite decompressor (`kuna-analysis/src/neolite.rs`), NOT by recognition and NOT by emulation. The refuter's read of the FILE is exact and reproduced; its scope verdict is overturned by measurement. NEOLite compresses each section IN PLACE and leaves the original section table untouched, so every virtual address and virtual size in the packed file still describes the original image and nothing has to be inferred by executing anything. The codec was read out of the stub -- an LZX derivative: 58 position slots, three repeated offsets, `256 + slot*8 + len_slot` main symbols, DEFLATE's length base/extra tables behind a 28-symbol tree, DEFLATE-style code-length transmission delta-coded mod 16 (tables at 0x4f7096 / 0x4f70bb / 0x4f70e0 in the stub) -- and a reference decoder decoded .text, .rdata and .data on its first run, with the recovered OEP landing on an MSVC SEH prologue. Witness: rc 1 -> rc 0, 139,776 -> 447,488 bytes, entry 0xf7001 -> 0x365a, imports 0xf7fac -> 0x12bd4, `kuna functions` 0 -> 387 with names.
- round 12 BUILDER: the refuter's OTHER finding is upheld and acted on. Recognition is a POSITIVE test (a `.NEOpack` section that OWNS the entry point) asked BEFORE `upx::unpack`, so the UPX arm is never reached differently and `no UPX PackHeader found` is byte-identical on every non-NEOLite file -- measured over 169 dataset MZ images, of which exactly one (the witness) carries `.NEOpack`. And the promoted probe carries five content clauses (packer, entry_hex, count, unpacked_size, import_directory), so the empty exit-0 the refuter warned about fails it.
