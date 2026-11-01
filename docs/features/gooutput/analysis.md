# Missing Go output selection

A native Go 1.22.5 noinline add function decompiles as:

```c
int main.add(int a0,int a1)
{
  return a0 + a1;
}
```

Requesting `--language go` fails with `unknown output language "go" (expected
auto, or one of: c-language, rust-language)`. The source, binary and captured
outputs are in the local research bundle. The function itself is recovered;
the output-language registry has no Go entry.

Pinned angr's TestGoTypeSpelling and TestGoCodegenHandlers run locally: five
tests pass, covering scalar/pointer/array/function spelling, handler coverage,
register fallback, precedence and field-gap fallback. The reference fixture
fauxware is 8,776 bytes, SHA256
c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b.

Kuna already centralizes language profiles and several statement-form dispatch
points. The new backend can reuse those without inventing a second IR or C
printer. Important gaps to audit include Rust-specific fallback assumptions
behind false language capabilities, such as rendering a non-ternary selection
as a Rust if expression, and bottom-tested loop continue behavior. These must
be handled explicitly for Go.

The initial feature changes explicit Go output only. Automatic language policy,
Go multi-result ABI recovery, metadata facets and runtime folds remain separate
features and are not claimed as complete by syntax support.

The reference Go 1.22.5 basics class also passes locally (eight functions,
including its prototype, multi-result, runtime-cleanup and ownership checks).
Its binary is 1,471,746 bytes, SHA256
b0c5144b05517c764da886a30b5eeca5709b6b655e93b55ae4967042943db50f.
Kuna's current C output for the same eight functions is captured separately;
this syntax feature alone will not satisfy all of those semantic expectations.
