## The need

`docs/re-needs/cli-mode-read-raw.md` (major, 1 instance, round 2, challenge `653d88600f4238b24302b0ec`):

> **No CLI mode to read raw bytes from data addresses**
> The new disassemble command returned raw instruction bytes, but there is no analogous data/hexdump mode; strings discarded non-printable constants.

The tester wanted the encoded globals at `0x100003f30` — the `00..0f`, repeated `0x10`
and repeated `0x20` vectors an initializer indexes. kuna answered with
`ADD byte ptr [RCX],AL` / `OR CL,byte ptr [RBX]`: a correct decode of `00 01 02 03 ..`
and a lie about the program. They left for `xxd -g1 -s 0x3f30 -l 96`.

The companion need `disassembling-non-executable-rdata` is the same gap seen from the
other side — `disassemble` decoding a PE `.rdata` range into `SBB`/`POP`/`PUSH`/`JNC`
with nothing saying the range is non-executable.

## The mechanism

`kuna disassemble` becomes one command with **two renderings of one walk**, selected by
`--as code|data|auto`:

- **`auto`** (the default) asks the loader which section holds the start address. A
  section carrying `DATA` without `CODE` (`.rdata`, `.rodata`, `__TEXT,__const`) holds
  bytes, so it is shown as bytes — and the reason goes on **stderr**, so `--json` stdout
  stays one document.
- **`code`** and **`data`** override it in either direction. A packer puts real code in
  `.data`; a compiler puts real data in `__TEXT`.
- **`kuna read`** is the same command with `--as data` as its default — the spelling to
  reach for when what you want is the bytes.

Two exceptions keep the inference honest: a target that resolved to a **discovered
function entry** is code wherever it was linked and is never reclassified, and an address
in **no section the loader published** (the XML `<binaryimage>` corpus, a raw blob) is
silence rather than evidence and keeps the instruction listing.

The byte view is `xxd -g1` with kuna's address column, and its `--json` carries the span
as one contiguous hex string alongside the per-row cut:

```
$ kuna read ./crackme0x04 0x100003f30 --addr --bytes 96
# 96 bytes at 0x100003f30 (0x100003f30..0x100003f90)
0x100003f30   00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  |................|
0x100003f40   10 10 10 10 10 10 10 10 10 10 10 10 10 10 10 10  |................|
0x100003f50   20 20 20 20 20 20 20 20 20 20 20 20 20 20 20 20  |                |
0x100003f60   25 73 00 43 72 61 63 6b 6d 65 20 4c 65 76 65 6c  |%s.Crackme Level|
```

Both views share one envelope (`binary, kind, target, start, end, count, bytes,
truncated, notes`), so a consumer reads the same keys whichever answered; `kind` says
which did.

**Nothing about the decode changed.** No `phases.toml` row, no `options.rs`
registration, no catalog counter, no stages XML, no DIV row: a rendering choice inside a
read-only query surface cannot change emitted C.

## The acceptance probe

`a-ecb7f35d39aa` fails on `cb357826` and passes here:

```
$ kuna disassemble .../crackme0x04 0x100003f30-0x100003f40 --addr --json
{ "kind": "data", ..., "hex": "000102030405060708090a0b0c0d0e0f", ... }
```

`scripts.repipe.verify --need cli-mode-read-raw` reports **PASS / transition `closed`**.

Promoted to `tests/cli/cli-mode-read-raw.json`, re-pointed from the dataset crackme
(CI has no dataset) at the in-repo Mach-O fixture `macho_imports`, whose
`__TEXT,__cstring` at `0x1000005ee` is the same shape: data linked inside an executable
segment. `make test-cli` is 15/15.

The companion need's acceptance (`a-c42440103c7d`, a stderr warning on a non-executable
range) also passes as a side effect — one surface answered both, as its triage note
predicted. Its record is left for the captain to transition.

## Gates

| Gate | Result |
|---|---|
| `make test` | PARITY OK — 675/675 assertions |
| `make test-stages` | PARITY OK — 597/597 assertions |
| `make rust-test` | green — 5,296 passed / 1 failed / 38 ignored. The single failure is `verify_w10_proto_unlock::w10_proto_unlock_const_return_collapses_no_tied_roundtrip`, the known repipe-harness artifact: the loop exports `KUNA_DECOMP_TEST` pointing at a **Rust** `decomp_test_dbg`, so that test compares Rust against Rust. Re-run with it unset: 4/4 pass. |
| `make check-spec` | OK |
| `make test-cli` | 15/15 |
| `kuna catalog --check` | catalog OK |
| `kuna disassemble` suite | 96/96 (`cargo test -p kuna-cli --test disassemble_cli`) |

**Sweep for the wrongness this could introduce** — a *code* target flipped to the byte
view: every discovered entry (capped at 40/image) of all 73 binary fixtures under
`kuna-analysis/tests/fixtures` + `tests/hang-repro`, through `disassemble --addr --count 1
--json`. **1,299 entries answered `kind: "code"`; 0 flipped.** The 17 non-answers are the
pre-existing unmapped-entry exit-1 path (an ET_REL undefined extern at a synthetic address,
a `0x0` entry), which fails before view selection is reached.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
