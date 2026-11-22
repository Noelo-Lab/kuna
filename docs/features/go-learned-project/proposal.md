# Restore a learned primitive Go result from C project export

## Entry points

Add an opt-in `decompile-project --save-session PATH` sidecar to serial C
export on Linux AMD64 Go inputs with explicit `protoorder types`. Add console `restore PATH` after `load file` for a fresh
matching image. The user can then select Go output and render the same
consumer. Keep the existing C project artifact formats and default entry
points. The writer collects the final facts after the ordered pass and strict
assertion checks have completed. A streamed or multi-worker project cannot
supply the first version's single result table, so a request to save that
session reports why the snapshot is unavailable. [Existing export driver](../../../decompiler/crates/kuna-cli/src/decompile_project.rs)

## Format and authority

The sidecar is bounded UTF-8 JSON with schema `kuna-project-scalar-session/1`.
Bind it to the complete source image digest and the resolved ELF target,
including language, compiler, endianness, load mapping, compiled processor
spec and processor/compiler configuration digests. Record effective analysis
settings needed to rebuild the same facts, including the Go DWARF prototype
setting and the ordered run's result policy. The console keeps its separately
chosen display language.

The first fact variant is a confirmed primitive integer result from
`RecoveredTypes.output`. Store the resolved producer entry, register-space
storage, byte width and signedness, with the shared P4 provenance. Bounds and
enum values are validated before applying a record. Unsupported inferred facts
such as aggregate, pointer and paired results are omitted when exporting.
Unknown field tags, fact kinds or versions in an input sidecar reject the
restore as a whole. Recreate the recorded result in the same
`seed_protoorder_types` channel. A stored result makes no input, arity or
prototype-lock claim. The existing result-conversion check still requires
matching low storage, width and type at the actual call. [Learned results](../../../decompiler/crates/kuna-decomp/src/p4_calls/kuna_protoorder.rs)
[Result conversion guards](../../../decompiler/crates/kuna-decomp/src/p9_emit/kuna_callresult.rs)

The initial user value is an applied explicit function prototype at a
resolved entry using fixed-width primitive integers and Boolean types.
Validate the resolved types and widths through the existing declaration
parser. Variadic, pointer, composite or unresolved prototypes and other
directive kinds are outside this format. If the input contains one, a requested snapshot reports that
directive and leaves the existing file intact. Rejected directives also
prevent snapshot creation. The record carries parsed data for permitted
directives and never interprets an imported value as a console command or
file inclusion.

## Restore order

1. Decode and validate the complete sidecar, rejecting duplicate entries,
   invalid storage and mismatched image or spec identity without changing
   the current console.
2. Rebuild the program in temporary state from the verified image bytes and
   recorded analysis options. Apply the supported explicit prototype through the existing
   declaration path. An already edited console is outside this first reopen
   pairing and reports that restriction before changing the loaded program.
3. Install the unlocked learned result only where explicit function and site
   authority allow it. Switch the console to the new state after validation.
   Later explicit declarations take precedence and invalidate imported learned
   facts before another decompile. First use can discard the imported facts
   conservatively after a semantic edit.

Write the requested sidecar atomically. Retain the existing default export,
including byte-identical C and header artifacts, when the sidecar is requested
successfully. The schema carries only typed analysis facts and the permitted user declaration.

## Acceptance

Fresh console restore must reproduce the batch's low-byte comparison on the
first Go render without adding a producer declaration. It must preserve the
unrelated function and honor a later explicit producer or call-site prototype.
Check a same-length changed image, mismatched specs, unsupported directives,
a newer schema, duplicate records and malformed input. A failed restore must
leave the existing loaded state usable.

This proposal covers the authored AMD64 Go scalar case through C export and
console restore. Interactive console save, Rust global declarations,
language-owned composite types and standalone recovery without a restored
fact remain outside the initial format.

## Open questions from review

This adds a large new surface for one fact kind: a `decompile-project
--save-session` flag, a console `restore` command and a versioned JSON
schema. It is limited to serial C export with an explicit `protoorder
types`, which is not the default (`cycles` is). A single-function `kuna
decompile` already bootstraps its direct callees' recovered parameter types
on AArch64 targets with `<prefersplit>` (`docs/cli.md`). Would extending
that bootstrap to a direct callee's result width fix the fresh view with
no sidecar?
