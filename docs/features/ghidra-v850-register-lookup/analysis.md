# Ghidra-safe V850 register lookup

Issue: [#428](https://github.com/Noelo-Lab/kuna/issues/428)

## Failure

The V850 SLEIGH specification represents `jmp [reg]` as `CALLIND`. With
`v850indirectbranch` enabled, Kuna examines indirect control-flow ops and uses
the input register name to distinguish a jump through a general register from a
call through `PC`.

The live implementation in `ArchFlowEnv::is_v850_indirect_jmp` obtained that
name by calling `EngineTranslate::as_sleigh()` and unwrapping the result. This
worked in the CLI because its translator is a standalone `Sleigh`. Ghidra mode
installs `GhidraTranslate`, whose `as_sleigh()` correctly returns `None`, so the
same shared flow pass aborted the native decompiler process:

```text
v850 indirect-branch predicate: standalone Sleigh engine
```

The option currently defaults off and has no element in Ghidra's upstream
`<optionslist>` vocabulary. That makes the fault latent in the stock GUI, but it
is still an invalid dependency in a per-op predicate. A future way to transmit
Kuna options would make the crash user-reachable immediately.

## Translator boundary

`Architecture` owns `Box<dyn EngineTranslate>` so flow recovery can run with
either translator:

```text
FlowInfo::xref_control_flow
  -> ArchFlowEnv::is_v850_indirect_jmp
     -> EngineTranslate / RegisterLookup::get_register_name
        -> Sleigh: local register table
        -> GhidraTranslate: getRegisterName callback and cache
```

`EngineTranslate` extends `Translate`, and `Translate` extends
`RegisterLookup`. Register names are therefore already part of the common
interface. `as_sleigh()` is an escape hatch for operations that inherently need
standalone SLEIGH state, such as instruction masks or local specification
compilation. Reading a register name is not one of those operations.

The fix calls `arch.translate().get_register_name(space, offset, size)`
directly. The standalone path still reads the same register table. In Ghidra
mode, `GhidraTranslate` delegates to `GhidraRegisterLookup`, which serves a
cached name or sends the existing `command_getregistername` callback to the Java
host. An empty name retains the existing “not a named hardware register” result.

Returning `false` whenever `as_sleigh()` is unavailable would avoid the panic,
but it would silently disable the V850 correction in Ghidra mode. Using the
common interface preserves the option's behavior for both back ends.

## Regression test

`v850_register_lookup_e2e.rs` drives the real `GhidraProcess` against the
in-tree `ghidra_sim` oracle:

1. Register the ARM fixture through the normal four-document wire handshake.
2. Enable `v850indirectbranch` after registration through a test-only process
   seam. This is necessary because Kuna-owned options have no upstream wire id.
3. Decompile `fmt_arm/main` through `decompileAt`.
4. Require at least one `getRegisterName` callback and a non-empty function
   response with no warning.

The architecture of the fixture is not the trigger. Before the fix, enabling
the gate made the shared predicate unwrap `as_sleigh()` while walking the first
operation, before the predicate had established a V850 jump shape. The test
therefore reproduces the same backend-boundary failure with a small fixture
already covered by the Ghidra simulation harness.

On the unpatched implementation the test panics at
`decompile_drive.rs` with the message above. With the common lookup it passes
and records the host register-name callback, proving that the Ghidra translator
rather than a standalone SLEIGH engine supplied the name.

## Scope

This is a strict crash fix. It adds no option and does not alter the option's
classification rule. Standalone decompilation resolves the same name from the
same SLEIGH register table as before. The only changed case is a non-SLEIGH
translator reaching an already abstract register-name operation.

A real Ghidra smoke run remains useful for the ordinary Ghidra integration, but
the stock Java client cannot currently enable `v850indirectbranch`. The
simulation test is the executable coverage for the exact option-gated path.

## Validation

The focused regression was run before and after the implementation change. On
the old code it reproduced the `as_sleigh()` panic quoted above; on the fixed
code it passed and observed the `getRegisterName` callback. The complete
`kuna-ghidra` release suite also passed, including the normally ignored
sort/grep breadth test invoked by `make test-ghidra`.

The repository parity and specification gates passed:

- `make test`: 675/675, `PARITY OK`.
- `make test-stages`: 635/635, `PARITY OK`.
- `make check-spec`: green.
- `make test-ghidra`: green, including this regression.

`make rust-test` ran the complete workspace suite green, including the new
regression.
