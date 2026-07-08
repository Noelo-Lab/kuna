# [PROPOSAL] String-reference recovery for readonly-char pointers (`bindtextdomain(0x68d8,…)` → `"coreutils"`)

Part of the **kuna → IDA Pro parity** program. Gap #3.

## The problem

`fmt/main`:

```c
// kuna
bindtextdomain(0x68d8,"/usr/local/share/locale");   // arg1 raw, arg2 fine
textdomain(0x68d8);
error(1,*__errno_location(),0x6f0c,v8);               // 0x6f0c should be "%s"
// IDA Pro
bindtextdomain("coreutils","/usr/local/share/locale");
textdomain("coreutils");
error(1, *v30, "%s", v29);
```

Within the *same* `bindtextdomain` call, `arg2` resolves to a string but `arg1` (`0x68d8`)
renders as a raw integer — so this is **not** a callee-prototype / arg-type problem.

**Root cause (confirmed).** `0x68d8` points at the bytes `63 6f 72 65 75 74 69 6c 73 00` =
`"coreutils"`, which is a **substring** of `"GNU coreutils"` starting at `0x68d4`. kuna types
(and string-marks) a constant pointer only when it targets the **start** of a detected string;
a pointer into the *middle* of a larger readonly string never gets a `TYPE_PTR`-to-char, so
`PrintC::pushConstant` renders it as an integer instead of taking the
`push_ptr_char_constant_ir` string arm (`s9_emit/printc.rs:5760`). `"/usr/local/share/locale"`
is a standalone string start, so it resolves. `0x6f0c` (`"%s"`) is the same class.

## The IDA / Ghidra reference

Both resolve a constant that points **anywhere** into readonly character data by reading from
that offset to the terminating NUL — no requirement that the offset be a pre-detected string
start. Ghidra: `TypeFactory`/`StringManager` readonly-char reference on any pointer into a
`readonly` char range. IDA: any offset into a string segment renders as the tail string.

## Proposed implementation

Two layers, either/both:

1. **Type layer (preferred, general):** when a constant's value falls inside a readonly
   (`Varnode::readonly` / section-flag) range whose bytes at that offset are printable char
   data ending in NUL, give the constant a `char *` (`TYPE_PTR`→`TYPE_INT1` char-print) type —
   regardless of whether the offset is a marked string start. This is the point where
   `dcgettext`'s already-typed arg gets its string; extend the same typing to any
   readonly-char pointer constant.
2. **Print layer (fallback):** in `push_ptr_char_constant_ir`, if the pointer is untyped but
   lands in a readonly-char range, read to NUL and emit the literal (the C++ `pushPtrCharConstant`
   already does the readonly-string read; the gap is the *entry* condition being gated on the
   char-ptr type, which layer 1 supplies).

## Default policy

Default-on, no flag. Any baseline that renders a mid-string pointer as an int would flip to the
string literal — re-pin with justification (matches IDA).

## Speed / risk

- **Speed:** negligible (one readonly-range membership test + NUL scan per constant, only for
  constants in a readonly data range).
- **Risk:** MEDIUM. Must not over-eagerly stringify a numeric constant that merely *happens* to
  land in a readonly range but isn't a genuine char pointer (e.g. a jump-table base, a packed
  bitmask). Gate strictly: the target bytes must be printable-char + NUL-terminated within a
  bounded length, AND (ideally) the constant must be used in a pointer context. Ghidra's own
  heuristic is a good reference for the guard.

## Testing

`verify_*.rs` on `regglobal_fmt_x86_64`: assert `main` renders `bindtextdomain("coreutils",…)`
and `error(…,"%s",…)`. Add a negative fixture (a numeric constant in `.rodata` that must NOT
stringify). Gates: all three.

## Effort

**Medium.** The print machinery exists; the work is the readonly-char pointer typing + a
careful guard against false positives.
