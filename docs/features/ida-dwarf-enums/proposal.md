# [PROPOSAL] DWARF enum-constant naming (`quotearg_style(4,…)` → `quoting_style::shell_escape_always_quoting_style`)

Part of the **kuna → IDA Pro parity** program. Gap #7.

## The problem

`fmt/main`:

```c
// kuna
quotearg_style(4,(char *)file);
// IDA Pro
quotearg_style(quoting_style::shell_escape_always_quoting_style, (char *)file);
```

kuna renders the enum argument as the raw integer `4`; IDA names it from the `quoting_style`
enum. (Lower-priority than naming/prototypes — a single call site here — but a clean DWARF win
that generalizes to every enum-typed argument/variable.)

## Root cause

The binary's `.debug_info` carries `DW_TAG_enumeration_type quoting_style` with its
`DW_TAG_enumerator` members (`literal_quoting_style=0`, …, `shell_escape_always_quoting_style=4`,
…). kuna's DWARF pass (`s1_dwarf/mod.rs`) does not build enum `Datatype`s, and even when a
constant's type resolves to an integer, kuna has no enum member table to decompose it against.
The decompiler already has an **enum render path** (`s9_emit/printc.rs:5699`
`push_enum_constant_ir`, gated on `Datatype::is_enum_type()`), so the missing piece is the
DWARF→enum-type construction + binding the callee arg / variable to that enum type.

## The IDA / Ghidra reference

Both import DWARF `DW_TAG_enumeration_type` as a first-class enum type and, when a constant's
type is that enum, render the member name (decomposing an OR of flags for flag-enums —
`pushEnumConstant`, which kuna already ports).

## Proposed implementation

1. **DWARF pass:** resolve `DW_TAG_enumeration_type` → a kuna enum `Datatype` (name + `{value →
   member}` map + base metatype), registered in the type factory. `build_datatype` already
   walks DWARF type DIEs; add the enum arm.
2. **Binding:** the enum shows up (a) as a `DW_TAG_formal_parameter`/variable type — covered by
   the `ida-dwarf-prototypes` proposal (apply the param's enum type) — or (b) via a callee
   prototype whose arg is the enum. The `quotearg_style` case needs the *callee* prototype
   (also from DWARF if `quotearg_style` is described, else a libc/coreutils type). Once the arg
   varnode carries the enum type, the existing `push_enum_constant_ir` renders the member.

## Dependency

Best landed **after** `ida-dwarf-prototypes` (that proposal supplies the param-type application
this render hangs off). Standalone value is the enum-type construction (reusable everywhere an
enum-typed variable appears), even before full callee-arg binding.

## Default policy

Default-on, no flag. Re-pin only if a DWARF enum datatest changes render.

## Speed / risk

- **Speed:** load-time (enum DIE walk). Negligible decompile impact.
- **Risk:** LOW–MEDIUM. Enum construction is additive; the render path already exists and is
  tested. Risk is limited to mis-decomposing a flag-enum (the ported `pushEnumConstant` already
  handles the OR-of-flags case).

## Testing

`verify_*.rs` with a DWARF enum fixture asserting the member name renders. Gates: all three.

## Effort

**Medium**, and largely reuses the enum render path + the DWARF prototype work. Lower priority
than naming/prototypes/strings.
