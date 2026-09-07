---
need_id: unpack-proceed-without-upx
title: Unpack cannot proceed without a UPX PackHeader
track: tooling
status: open
severity: blocker
probe_id: p-c9020a5d7fe1
acceptance_id: a-0afde75500f7
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [5ab77f5433c5d40ad448c1cd]
rounds: [6]
first_seen_round: 6
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/upx]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover the payload, allowing analyst-supplied decoder parameters when packing metadata is unavailable.

> **Unpack cannot proceed without a UPX PackHeader** (blocker, `5ab77f5433c5d40ad448c1cd`)
> Unpack exits 1 with no UPX PackHeader found. Disassembly exposes a decoder at 0x40e550 reading 0x40a000, writing 0x401000, and transferring to 0x401d91. Help and catalog expose no unpack parameter overrides. Exact packer identity remains unverified.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}",
    "--json",
    "-o",
    "{{TMP}}/out.bin"
  ],
  "cwd": "{{TMP}}",
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe",
    "binary_sha256": "4004f1b83d3e89ce35908d39c0e4ae06c0aad92fe6bdf8e7711b09b2baba4689",
    "binary_size": 22016,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe",
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "no UPX PackHeader found"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}",
    "--json",
    "-o",
    "{{TMP}}/out.bin"
  ],
  "cwd": "{{TMP}}",
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe",
    "binary_sha256": "4004f1b83d3e89ce35908d39c0e4ae06c0aad92fe6bdf8e7711b09b2baba4689",
    "binary_size": 22016,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe",
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "no UPX PackHeader found"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly a modified UPX-family loader with removed metadata; decoder support is unverified.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/EndOfWorld.zip.__x/EndOfWorld.exe` — Reference unavailable: server exited with status 1 before registering. No comparative decompilation was obtained.

## Instances

- `5ab77f5433c5d40ad448c1cd` (round 6, tester t-r6-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 6 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED, by direct inspection of the packed file (not by re-running the probe). The filed guess was 'a modified UPX-family loader with removed metadata'. The metadata is NOT removed: the PackHeader sits at file offset 0x3e0 of EndOfWorld.exe, intact and self-consistent -- magic UPX!, version 12, format 9 = win32/pe, method 2, level 4, stored checksum 82 == computed 82 (sum(p[4..31])%251). u_file_size 0xc000, u_len 0xc6a0, c_len 0x454e. This is a stock UPX 1.25 PE32 (section table renamed to .text/.data with .text rawsz=0, .data raw holding the packed stream; the tester's decoder at 0x40e550 reading 0x40a000 into 0x401000 is exactly that layout). kuna fails for TWO structural reasons, both in decompiler/crates/kuna-analysis/src/upx/mod.rs, and NEITHER is the filed one. (1) parse_pack_header scans only the tail window 2*4096+2*36+1 = 8265 bytes, i.e. from 0x35b7 to EOF on this 0x5600-byte file, because it is written for the ELF layout where the PackHeader is the last thing in the file. A PE keeps it in the header padding at 0x3e0 -- OUTSIDE the window -- so the scan finds nothing and returns NotPacked. (2) Even if it were found, unpack() gates on is_elf_format(format) (mod.rs:263, the list 12|20|22|23|25|30|39|42|45|132|133|137|140) and format 9 is win32/pe, so it would return 'only the ELF formats are implemented'. Also note the ELF-shaped parse would still not survive on a PE: the le32 read immediately after the header, which the ELF layout defines as the overlay offset, is 0xf2fddffc here (adjacent loader bytes, not a field), so the >= image.len() check would report Corrupt. CONSEQUENCES FOR TRIAGE, please act on these. (a) The user-facing diagnostic is actively wrong and should be fixed whatever else happens: kuna says 'this file is not one', of a file that is unambiguously UPX-packed. (b) The need's metadata is wrong: touches says decompiler/crates/kuna-cli, but nothing in kuna-cli is implicated -- the code is kuna-analysis/src/upx/{mod,elf}.rs. (c) scope: small is WRONG. The acceptance demands 'unpack exits 0', which cannot be reached by any parameter override or scan widening; it requires implementing the win32/pe target end to end (PE section reconstruction plus import/reloc rebuild, UPX's p_w32pe), a new format alongside the ELF one. Recommend rescoping to large/proposal, or splitting a small honest-diagnostic need ('recognise a non-ELF UPX image and name its format') off the big one. The SYMPTOM stands in full -- a blocker for this challenge either way.
- round 6 T_TRIAGE: TRIAGE: touches kuna-cli -> kuna-analysis/src/upx (parse_pack_header's last-8265-bytes scan and unpack()'s is_elf_format gate, upx/mod.rs:263). scope small -> LARGE: the acceptance demands `unpack` exit 0 on a win32/pe image, which is PE section reconstruction plus import/reloc rebuild end to end. Expect this one to stop at a [PROPOSAL] draft; that is the correct outcome, not a failure.
