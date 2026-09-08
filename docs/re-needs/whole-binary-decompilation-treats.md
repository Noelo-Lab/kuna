---
need_id: whole-binary-decompilation-treats
title: Whole-binary decompilation treats PE import pointer slots as function bodies
track: loader
status: open
severity: major
probe_id: p-ac6929b867fc
acceptance_id: a-a28f3cfabf9d
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f6333c5d40ad448ca40]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/loader]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Decompile actual code while keeping known import address table slots as data.

> **Whole-binary decompilation treats PE import pointer slots as function bodies** (major, `5ab77f6333c5d40ad448ca40`)
> Only IAT addresses 0x44f264 and 0x44f268 are returned. LoadLibraryA gets an invented body containing out(*v1,a1); GetProcAddress fails loading bytes. kuna read shows import-name RVAs in these slots.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/BUG.cRYPTO.kEYGENME.zip.__x/BUG.exe",
    "binary_sha256": "54d08ffbaba9daebe6f337c37472cfacfcc9b1c2d303c16c8ec65e73ab90f5bf",
    "binary_size": 99469,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "stdout_matches": [
      "\"address_hex\": \"0x44f264\"",
      "\"address_hex\": \"0x44f268\""
    ],
    "json": [
      {
        "path": "functions[*].code",
        "op": "matches",
        "value": "out\\("
      }
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_headercode_i386.exe",
    "binary_sha256": "43d2d88fd87b1756b1a0ae6feac7ee8701f40a91b30be8171f689aeea7f88382",
    "binary_size": 1024,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_headercode_i386.exe"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--no-vars"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_absent": [
      "\"address_hex\": \"0x40200[04]\""
    ],
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
      },
      {
        "path": "functions[*].address_hex",
        "op": "eq",
        "value": "0x400154"
      },
      {
        "path": "functions[*].code",
        "op": "contains",
        "value": "GetProcAddress(LoadLibraryA("
      }
    ]
  },
  "notes": "Vendored twin of the reported keygenme: two nameless sections both 0xc00000e0 (MEM_EXECUTE clear), AddressOfEntryPoint 0x154 in the header page. Built by tests/fixtures/pe_headercode_i386.py; unpatched it answers count 0 as the dataset image does, which stays the witness in Reproduction. REPLACES the filed `count > 2`, unreachable there -- see the decision log."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Executable section flags override known import-slot classification during batch selection.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f6333c5d40ad448ca40` (round 4, tester t-r4-5ab77f63)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 4 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED -- the symptom stands, the mechanism does not. Reproduced on sha c5f4073a: decompile-all --json returns exactly 2 functions, 0x44f264 LoadLibraryA (size 4, body out(*v1,a1)) and 0x44f268 GetProcAddress (size 3480), and kuna functions --json returns the same 2 -- so this is not a decompile-all batch-selection decision at all, it is the whole function list. THE FILED CAUSE IS FALSIFIABLE TWO WAYS. (1) Nothing consults section flags on this path: loadimage_object.rs:486-489 pushes EVERY ImportSym from fmt.resolve_imports straight into funcsyms as a FuncSym, and pe_iat.rs:214 emits one ImportSym per IAT slot at the slot VA (out.push(ImportSym{addr: slot_va, name})). There is no import-slot classification for an exec flag to override -- the slot is filed as a function symbol by construction. (2) The image has no executable section to do the overriding: 2 nameless sections, both characteristics 0xc00000e0, i.e. CNT_CODE|INITIALIZED|UNINITIALIZED plus R plus W and NO MEM_EXECUTE (0x20000000 clear). The import directory RVA 0x4f20c lands inside the second one (0x437000-0x450000), the same section as everything else, so there is not even an .idata to classify. The veneer half of pe_iat is correctly exec-gated (pe_iat.rs:257) and simply finds nothing here, which is why only the raw slots survive. ACCEPTANCE IS GAMEABLE, FIX IT AT TRIAGE: the reason the list is ONLY those two is that discovery found zero real functions on this packed image -- entry RVA 0x154 resolves to 0x400154, inside the PE headers, below the first section at 0x401000 (the sibling need pe-header-entry-mapped is the same image trait). So a builder who merely suppresses IAT slots makes decompile-all emit an EMPTY function list, and the filed acceptance (stdout_absent on 0x44f26[48]) PASSES on nothing at all. The tester's actual ask was to decompile real code while keeping the slots as data, so the acceptance must also require at least one non-import function body. TRACK NOTE: touches is kuna-analysis (loader), not kuna-decomp.

### 2026-09-08T07:00Z — captain, round 9 B_DRAIN: track quality → loader, and this need is NOT dispatchable alone

Applying this need's own round-4 refuter, which ended `TRACK NOTE: touches is kuna-analysis
(loader), not kuna-decomp`. The mechanism it measured — `loadimage_object.rs:486-489` pushes
every `ImportSym` straight in as a `FuncSym`, `pe_iat.rs:214` emits one per IAT slot VA — is
loader-tier and adds no `settableTable` row, so the five option leases `TRACK_RESOURCES`
gives `quality` were blocking it for nothing. Rationale and the measurement that makes a
lease-free track safe here (39 non-quality closes, zero option-plane edits) are recorded once
on the anchor, `bulk-decompilation-decodes-pe`; the same **stop-and-report-`blocked`-if-it-
needs-an-option** requirement applies to this need.

**Anchor: `bulk-decompilation-decodes-pe`.** Same defect, second image. Dispatch this need
only as that anchor's sibling cluster lease — never as its own builder — or two builders write
the same fix.

**Its acceptance is gameable and must be strengthened before it is dispatched.** The refuter
established that discovery finds *zero* real functions on this image (entry RVA 0x154 →
0x400154, inside the PE headers, below the first section at 0x401000 — the same image trait as
`pe-header-entry-mapped`), so the whole function list is the two IAT slots. A builder that
merely suppresses import slots makes `decompile-all` emit an **empty** list, and the filed
`stdout_absent` clause on 0x44f264/0x44f268 then passes on nothing at all. The acceptance
needs a second clause requiring at least one non-import function body before this closes.
- round 10 REFUTER: hypothesis **overturned**. Captain re-measured the acceptance probe on main 7e77fb8c (post-#509) on 2026-09-08: the DEFECT half now passes -- stdout_absent for "address_hex": "0x44f26[48]" is OK, the two bogus IAT-slot functions are gone. What still fails is the SHAPE clause json[0] count > 2: kuna decompile-all on BUG.exe now returns count=0, functions=[]. So this need has changed character -- it is no longer 'IAT slots decoded as function bodies' (that was closed as a side effect of #509 bulk-decompilation-decodes-pe), it is now 'zero functions discovered in this PE at all'. A builder must NOT re-fix the import-slot filter; the work is PE function discovery on a 99469-byte MinGW-ish keygenme where the entry/exports path finds nothing. Not dispatched in round 10's fanout because it shares the kuna-analysis/src/loader path with sectionless-elf-import-relocations and select.py has no lease for that prefix.

### 2026-09-08 — builder b-r10-whole-binary-dec: the acceptance retargeted, and why

The filed clause `count > 2` cannot be met on the witness, and no amount of engine work
would change that. Read the image's own dispatch table: the entry at 0x400154 does
`xchg esp,[0x44f250]` then `popad`, which loads `ebx = 0x44f254`, a six-word table holding
0x4001e8 / 0x4001dc / 0x4001de (helpers reached only through `call [ebx+N]`), 0x4098a3 (the
original entry, in the section whose `SizeOfRawData` is 0 -- zeroes in the file until the
stub unpacks it) and the two IAT slots the acceptance forbids. Statically there is exactly
ONE function body in this image, and `count > 2` only ever read as "more than the two IAT
slots" because the slots were being counted as functions; #509 stopped counting them.

So the acceptance now runs on `pe_headercode_i386.exe`, a vendored twin built to the
reported image's exact traits (both sections 0xc00000e0 with MEM_EXECUTE clear,
`AddressOfEntryPoint` 0x154 in the header page, the import directory in the second
section), and asserts the tester's ask directly instead of by proxy: the IAT slots absent,
the declared entry present, and a real body that names both imports. It is un-gameable in
the way the round-9 captain note asked for -- an empty function list fails `count == 1` and
fails the address clause. Same precedent as the anchor need `bulk-decompilation-decodes-pe`,
whose acceptance was likewise retargeted at a vendored twin with the dataset image left as
the witness in Reproduction.

Measured on the witness itself (`BUG.exe`, sha 54d08ffb):

| | `kuna functions` | `kuna decompile-all` |
|---|---|---|
| before | 2 (both IAT slots) | 0 |
| after | 3 (+ `sub_400154`) | 1, the unpacker stub decompiled |

The root cause is also not the filed one and not the round-10 refuter's framing of "PE
function discovery finds nothing": `entry::collect_entries` keeps a candidate only if it
lands in an executable section, and this image defeats that twice -- its entry is in the
header page, which is no section, and both of its sections carry `MEM_EXECUTE` clear, so
`executable_sections` is empty. Patching `MEM_EXECUTE` into both section headers of a copy
still yields only the two IAT slots, because the entry is in no section either way. The fix
exempts the declared entry, and only where the image maps it in the header page.

A second dataset image has the identical layout and was fixed with it: JinKu.keygenme.exe
(challenge 5ab77f6333c5d40ad448ca3e), entry RVA 0x154, two nameless sections both
0xc00000e0, `decompile-all` 0 -> 1.
