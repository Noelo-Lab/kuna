# Proposed rustfmtio recovery

Recover direct formatting through the verified `&Stdout` Write implementation
on x86-64 little-endian ELF from explicitly supported Rust producer builds.
Add one named option, `rustfmtio`, with metadata in the knowledge plane and
rendering in P9. The option remains unshipped until correctness and speed
acceptance is complete. A default-on proposal requires unchanged original
assertions and the existing five-percent speed budget.

## Receiver and result contracts

Classify only candidates whose full method identity and every alias agree.
Bind each candidate to a reviewed machine-code profile that establishes whether
the original first argument is a reference cell or a promoted inner pointer.
Validate code extent, internal control flow, relevant external identities and
protected data; unknown bodies, conflicting names and altered reference roles
must refuse recovery. A symbol name or pointer width alone is insufficient.
Profile matching belongs in the analysis tier, with compact facts consumed by
the emitter, and must avoid indexing unrelated symbols.

For the reference-cell form, borrow the original `&Stdout` cell. For the promoted
form, construct a temporary Stdout from the same inner pointer using only the
pinned one-word, no-Drop layout. Do not call `stdout()` to replace that pointer.
Use a scoped `std::io::Write` import and the existing verified format-template,
primitive and string bindings. Preserve the original descriptor setup and IR.

Keep the original eight-byte integer result type. Map success to zero; consume
an error into `*mut ()`, then expose its address and cast to the original signed
or unsigned integer type. Do not collapse failure to a boolean, drop the Error,
or transmute a pointer-containing value directly into an integer. Reject
unsupported result shapes, writers, callbacks and producer/layout combinations.
Whole-function recompilation and general Rust ownership reconstruction are
outside this feature's claim.

## Acceptance

1. Add native examples for literal, empty, signed/unsigned, repeated/flagged,
   string and mixed formatting, with dynamic and unknown-writer controls.
   Include both observed receiver ABIs and unsupported producer builds.
2. Independently verify every matched code profile and its references. Mutate
   names, aliases, instructions, branch destinations, protected data and result
   shapes; unsupported candidates must stay explicit calls.
3. Execute every changed emitted expression unchanged with explicit typed
   bindings. Compare native stdout, real broken-pipe payloads and exactly-once
   custom-error ownership, including boundary integers and varied UTF-8 inputs.
4. Review every changed complete function and source mapping across the image
   corpus. Require C/Go identity, Rust option-ablation identity and no newly
   failing compiler diagnostic multiset. Keep compilation limits explicit.
5. Add the owning spec prose, option/catalog wiring and two-pass stage testcase.
   Run all repository checks and measure a declared parent/off/default speed
   comparison before deciding the default.

This is one feature PR. It will be implemented after its formatting dependencies
land and their tested source identities are incorporated. Other I/O writers,
derived Debug callbacks, general Result types and broader Go/Rust parity remain
separate changes.
