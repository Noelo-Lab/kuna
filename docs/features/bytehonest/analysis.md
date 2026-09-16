# bytehonest — analysis

## 1. The symptom

`kuna decompile-all --json` describes a recovered variable with a C type name.
For a one-byte value the type system never committed to, that name is `char`:

```
$ kuna decompile-all O2/coreutils/stripped/fmt --addr 0x40f0 --json
"variables": [ { "name": "param_1", "type": "char", "size": 1, "kind": "arg" } ]
```

The ground truth for that function is
`void close_stdout_set_ignore_EPIPE(_Bool ignore)`. kuna did not recover `char`:
it recovered *one byte it could not classify*, and the printer picked `char`
because `char` is the one-byte C type that reads as a value. The JSON surface
then repeats the printer's choice as if it were a recovery result.

## 2. Where the spelling comes from

`infra/decompile_drive.rs (extract_variables)` builds the `variables` array in
three sections: parameters off the `FuncProto` (1), the surviving `ScopeLocal`
stack symbols (2), and — under `framelayout` — the frame slots an earlier
`restructure_varnode` pass recovered (3). Sections 1 and 2 called
`printc::type_to_c_string` directly, i.e. exactly what the `.c` text says.

Under the default `realtypes` the residual `TYPE_UNKNOWN` carrier of size 1
(kuna's `xunknown1`, Ghidra's `undefined1`) renders as `char`; with `realtypes`
off the same row exports the internal spelling `xunknown1`, which is not a type
name any consumer knows. Section 3 already avoided both, through
`frame_slot_type_name`, which reports an uncommitted slot by width
(`undefined<N>`) — so the same fact was already reported two different ways by
two sections of the same array.

## 3. Why the width is the better answer

- It is what kuna actually knows. `undefined1` states the width and declines the
  element type; `char` asserts one.
- Every other decompiler reports it that way: Ghidra as `undefined1`, IDA as
  `_BYTE`, and kuna's own section 3 as `undefined<N>`.
- On the decbench `type_match` metric, an uncommitted width matches a ground-truth
  `char` **and** a `_Bool` (the `_UNCOMMITTED_TYPES`/`_SIZE_SCALARS` rule), where a
  committed `char` matches only `char`. So the honest answer also scores better —
  but the direction of the argument is that the surface should not claim more than
  the recovery established; the metric agreement is a consequence, not the reason.
- The C text is untouched. `char a0` is still the right *rendering*: it must be
  legal C, and `undefined1` is not a C type. The two surfaces answer two different
  questions, and this is the one place they are deliberately allowed to differ.

## 4. Scope — why not the section-3 rule

`frame_slot_type_name` also rewrites an ARRAY of unknown, turning `char[264]`
into `undefined264`. That is right for a frame slot (nobody established the
element type of a 264-byte spill area) and wrong for the variables sections: a
264-byte local really is an array, and `undefined264` would drop the array-ness a
consumer (and the metric) needs, while matching nothing. The predicate here is
therefore strictly narrower: `metatype == TYPE_UNKNOWN && size == 1`, read off the
`Rc<Datatype>` rather than off its printed spelling, and never an array.

## 5. Consequence to state

`kuna-console/src/project.rs` (`decompile-project`) reads `extract_variables`
too, so the exported `.c`/`.h` project's variable inventory reports the same
`undefined1` as `decompile-all --json`. The emitted `.c` file is unchanged.
