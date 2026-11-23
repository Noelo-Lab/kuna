# One frame STORE16 becomes two unrelated C objects

Observed on main `24eb881e5a8513da7baf18e2d1860fc9eeb040ff`, optimized Kuna SHA256
`23d845f0f135a26b4648e1bb8ec37360a803daba56ccb9a4b73a69f2db718fa7`.
The authored Linux AMD64 SysV fixture uses one sixteen-byte load, one
sixteen-byte store into its own frame, and an observer reading both eight-byte
halves. It does not rely on language detection, DWARF, a Go ABI guess or an
upstream specimen. The runnable reproduction is [reproduce.py](reproduce.py).

```sh
python3 docs/features/native-frame-store16/reproduce.py \
  --kuna "$PWD/decompiler/target/release/kuna" \
  --output /tmp/kuna-native-frame-store16 --expect-stock
```

Requires Linux x86-64, Python, GNU binutils and a GCC-compatible C compiler.
The output directory must not exist. The script assembles an authored ELF,
uses normal `raw-pcode` and `decompile` commands, then compiles and disassembles
C objects. It never executes the fixture or generated objects. Built Kuna
specs must be available normally or through `KUNA_SPECS`.

Stock output includes:

```c
unsigned long v1; // stack - 0x18 | xmm0_qa
unsigned long v2; // stack - 0x10 | xmm0_qb

v1 = *(unsigned long *)src;
v2 = *(unsigned long *)&src[8];
return observe16((unsigned char *)&v1);
```

Raw p-code reports a single LOAD16 and STORE16. Final variable metadata instead
reports two eight-byte stack objects. Passing `&v1` does not expose a sixteen-byte
C object containing `v2`. When the emitted body is compiled against an inline
observer with the exact machine reads, GCC 11.4 warns that `v2` is unused and
produces `mov (%rdi),%rax; xor (%rsp),%rax`. A reference with an actual
sixteen-byte byte array produces `mov (%rdi),%rax; xor 8(%rdi),%rax`.
Compilation is static evidence of the lost storage relationship, not a native
execution or equivalence claim. The script saves both C files, compiler output,
disassembly, raw p-code and hashes in `proof.json`.

## Useful controls already inspected

An explicit zero STORE16 already produces one `char[16]`, `memset` and the
correct observer address. Two independent constant STORE8 operations can also
be recovered by the existing array logic. Those successes must remain intact;
they do not justify treating any adjacent declarations as one object.

A nonconstant copy retains the split when the observer's declared pointer
changes from `unsigned char *` to `unsigned long long *`. Two separate zero
STORE16 operations produce two arrays while an observer32 receives only the
first: this needs a larger union proof and is outside the first candidate.
A conditional zero/nonzero STORE16 preserves branch-dependent values but can
split the storage into sixteen byte locals. An unknown XMM15 store stays
unknown. No zero value may be inferred from storage provenance.

## Existing support and the missing contract

- `p2_lift/flow.rs` records original memory operations with
  `kuna_localstackviews::record_native`. The identity includes the original
  generational operation, source point and width.
- `RuleStoreVarnode` in `p3_dataflow/ruleaction_4.rs` calls
  `kuna_localstackviews::record_store` before lowering a fixed frame STORE.
  This provides an early place to retain its complete physical extent.
- `kuna_localstackviews` already checks frame ownership, incoming formals,
  wrapping, contained accesses, locked layouts and early-analysis availability.
  Its automatic backing path requires typed publications or complete aggregate
  contracts and admits raw scalar widths only through eight bytes. A raw
  sixteen-byte store is not sufficient under the current policy.
- `kuna_stackviews` can map shared backing and placed views once the full extent
  is admitted. It does not prove that two adjacent local ranges belong together.
- `PrintC::emit_wide_copy` already prints supported mapped wide zero/copy values.
  `kuna_stackassign` has a declared-layout memmove-safe path. These emitters do
  not establish automatic frame backing or permit arbitrary mutable rereads.

The proposed missing contract is one original physical STORE16 extent retained
through splitting and allocation, not a new vector zero recognizer. See
[proposal.md](proposal.md) for admission, refusal and transaction boundaries.

The observer in this authored fixture is fully known. That is controlled witness
evidence, not proof that a general pointer declaration supplies a callee memory
contract. The proposal explicitly gates escaped-object admission on a complete
bounded consumer proof; that integration is unresolved and may require a
separately reviewed prerequisite. Explicit composite stack assertions can already
claim a wider interval under their own wrap/overlap guards; this automatic proof
must preserve that user authority.
