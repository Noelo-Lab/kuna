# Learned Go return width across project reopen

A serial batch can learn that a call returns a full word and preserve a caller's
low-byte projection in Go output. A fresh single-function process can lose that
fact even when it reads the same image and explicit user prototype.

## Reproduce

Use Go 1.27.2 and a Kuna build with the AMD64 processor specs installed. Set
`KUNA` or `GO_TOOL` to the absolute executable path if it is outside `PATH`.
The script builds an authored image and reads it as decompiler data.

```bash
set -euo pipefail
work=$(mktemp -d)
cd "$work"
cat > words.go <<'GO'
package main

var out bool

//go:noinline
func produce(seed uint64) uint64 { return seed ^ 0x1234567810111111 }

//go:noinline
func consume(skip bool, seed uint64) bool {
    if skip { return false }
    return byte(produce(seed)) != 0
}

//go:noinline
func unrelated(x uint32) uint32 { return x ^ 0x1234 }

func main() { out = consume(false, uint64(unrelated(7))) }
GO
GO111MODULE=off GOTOOLCHAIN=local GOPROXY=off GOSUMDB=off GOOS=linux GOARCH=amd64 \
    CGO_ENABLED=0 "${GO_TOOL:-go}" build -p=2 -trimpath -o words words.go
chmod a-x words
printf '%s\n' 'prototype main.consume bool consume(bool skip,uint8 seed)' > facts.kuna
common=(--option godwarfsig off --assert @facts.kuna --assert-strict)
selected=(--functions main.produce,main.consume,main.unrelated)
KUNA_PROTOORDER_TRACE=1 "${KUNA:-kuna}" decompile-project words "${selected[@]}" "${common[@]}" \
    --option protoorder types -o ordered > ordered.stdout 2> ordered.trace
"${KUNA:-kuna}" decompile-project words "${selected[@]}" "${common[@]}" \
    --option protoorder off -o unordered > unordered.stdout
cmp ordered/words.c unordered/words.c
cmp ordered/words.h unordered/words.h
rg -F '[protoorder] state main.produce' ordered.trace
"${KUNA:-kuna}" decompile-all words "${selected[@]}" "${common[@]}" \
    --option protoorder types --language go > batch.go
"${KUNA:-kuna}" decompile words main.consume "${common[@]}" --language go > fresh.go
rg -Fx '  return byte(produce(seed)) != 0x0' batch.go
rg -Fx '  return produce(seed) != 0x0' fresh.go
printf 'Artifacts in %s\n' "$work"
```

On main `ed7a24ed0` the two C artifacts compare equal and the script prints the
following lines.

```text
[protoorder] state main.produce @0x47db00 params=1 trimmed=0
  return byte(produce(seed)) != 0x0
  return produce(seed) != 0x0
```

The call at `0x47db40` is followed by `TEST AL,AL` and `SETNE AL`. Seed
`0x1234567810111011` makes the authored producer return `0x100`, so the original
low-byte test returns false. The fresh output's full-word comparison is true.
The producer has no user prototype in the reproduction.

`--option godwarfsig off` stands in for an image without DWARF. With the
default options this unstripped image already prints
`return byte(int8(produce(seed))) != 0x0` in a fresh run, because `godwarfsig`
states the producer's `uint64` result. A `-ldflags='-s -w'` build of the same
source reproduces the fresh `return produce(a1) != 0x0` with default options.

`decompile-project` already chooses the serial callee-first driver for an
explicit `protoorder types` selection. The ordered C project trace confirms
that the producer participated, while its emitted C and header carry no
distinguishing analysis record. The Go batch and fresh CLI outputs expose the
difference. The console accepts `load file` and `read symbols`. Both `save`
and `restore` produce `ERROR: Invalid command`, and neither writes a
snapshot. [Project exporter](../../../decompiler/crates/kuna-cli/src/decompile_project.rs)
[Console registration](../../../decompiler/crates/kuna-console/src/ifacedecomp.rs)

The proposed first round trip preserves the learned primitive result through
the existing C export and freshly loaded console. [Proposal](proposal.md)
