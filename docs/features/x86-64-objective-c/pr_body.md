## Problem

Kuna moved valid odd x86-64 Objective-C IMP addresses down by one byte; on the vendored witness it bound the selector to padding and produced a one-byte function instead of the method body. The original dataset witness likewise moved `-[CrackMeManager checkCode:]` from metadata IMP `0x100001d2b` to `0x100001d2a`:

```bash
kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/macho_objc_odd_imp '-[Greeter greet:]' --json
```

```json
{"functions":[{"name":"-[Greeter greet:]","address":4294968896,"address_hex":"0x100000640","size":1,"code":"void -[Greeter greet:](void)\n{\n  return;\n}"}]}
```

## Fix

Thread the architecture reported by the parsed object into both absolute and relative Objective-C method-list readers. Clear an odd IMP's low bit only for 32-bit `object::Architecture::Arm`, where it represents Thumb state; preserve it for x86-64 and every other architecture.

The regression coverage proves both directions: a vendored x86-64 Mach-O keeps its odd IMP and decompiles the full method, while an ARM32 unit test still normalizes an odd Thumb IMP.

## Verification

- The promoted CLI acceptance resolves `-[Greeter greet:]` at exactly `0x100000641`.
- The original dataset witness resolves `-[CrackMeManager checkCode:]` at exactly `0x100001d2b` with its full body.
- Objective-C analyzer tests, the full `kuna-analysis` suite, datatests, stage tests, CLI tests, specification/catalog checks, and the Rust workspace suite pass.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/)
