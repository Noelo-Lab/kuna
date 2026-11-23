# Preserve explicit two-word Go record inputs

A Go function that already decompiles correctly as two scalars loses both values when you give Kuna its real by-value record signature.

## Reproduce

Use Go 1.22.10 or 1.27.2 with Kuna's processor specs available. The generated ELF is an input to the decompiler.

```sh
d=$(mktemp -d)
cat > "$d/main.go" <<'GO'
package main
type Point struct { X, Y int64 }
var point = Point{3, -7}
var result int64
//go:noinline
func Manhattan(p Point) int64 {
    if p.X < 0 { p.X = -p.X }
    if p.Y < 0 { p.Y = -p.Y }
    return p.X + p.Y
}
func main() { result = Manhattan(point) }
GO
CGO_ENABLED=0 GOOS=linux GOARCH="${GOARCH:-amd64}" go build -trimpath -ldflags=-w -o "$d/point" "$d/main.go"
"${KUNA:-kuna}" decompile "$d/point" main.Manhattan --language go --json \
  --assert 'typedef struct Point { int8 X; int8 Y; } Point;' \
  --assert 'prototype main.Manhattan int8 Manhattan(Point p)' |
  jq -r '.functions[0].code'
```

The emitted signature has `p Point`, but the body computes from uninitialized register locals. Inspecting the same declaration with `--raw` shows the incoming machine values still present. Decompiling `main.main` with the same assertions shows its call taking an uninitialized stack record. The fix must connect that formal and that call operand to the same two physical values.

## First implementation

- Admit ordinary little-endian AMD64 and AArch64 Go ABIInternal only, with a complete explicit input declaration that names one 16-byte record with exactly two direct 64-bit integer fields at byte offsets zero and eight. The record size and field layout must account for every byte.
- Extend the shared P4 prototype input allocator so the caller and callee agree before either locks the contract. Allocate the fields from the next two available integer positions in the active ABIInternal model. Earlier scalar or explicit assignments keep their reserved locations. Use the existing join representation with the first field in the low word.
- Treat the assignment as a transaction. If both fields fit, reserve both slots and publish one joined parameter. If either fails, restore the allocation state and place the whole argument in its ABI stack slot. The next scalar can still use a remaining register. Verify the actual fallback stack location on each architecture. Honor an existing explicit aggregate storage location and an applicable call-site prototype before computing any new assignment.
- Keep one typed input and use the existing callee projections and caller join machinery. Model the known ABI fact directly. Automatic nominal DWARF record acquisition, other aggregate layouts, returns, and other calling conventions have separate evidence requirements.

## Acceptance

| Fixture | Required observation |
| --- | --- |
| Named record, two fields, first input | Both fields reach the callee and the caller from the original GPR pair. The emitted body passes the arithmetic observer on both Go generations and architectures. |
| Same-width scalar without record evidence | A plain 16-byte scalar or raw carrier does not enter the record classifier. Existing two-scalar function and call behavior remains byte-identical. |
| Record after an integer and a float | The integer advances the GPR cursor while the float uses its own bank. The record takes the next two GPRs and the next scalar starts after both. Use a noncommutative field expression to expose swapped pieces. |
| One GPR remains before the record | The whole record uses its verified stack slot. A subsequent scalar still uses that final GPR. Also cover a genuine stack record after complete exhaustion. |
| Explicit input or site storage | A stated physical input or site prototype retains authority, even when the allocator could construct a join. |
| Unsupported declaration | Mixed float/integer fields, narrower fields, gaps, nested records, arrays, partial types and Go ABI0 take the existing guarded path. |

Use actual Go-built callers and callees as static input. Compare physical assignments in raw p-code, check the joined operand and projections in final IR, and compile the emitted arithmetic into a separate observer. Add a single-pass stage witness and measure optimized affected and scalar-control requests before publication.
