# w6-s5-type-3
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> in-scope GREEN (dtype lib 1015 pass; verify_w5_dtype_expand 9 pass; verify_w6_s5_type_3 5 pass). One out-of-scope RED: `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` ("Could not find .sla file for x86:LE:64:default") — a pre-existing build-artifact gap (the worktree has no compiled `.sla`; gitignored), touches SLEIGH spec loading, NOT dtype.rs. Documented as the known gate gap (see LOSS-084 surface note). clippy -p kuna-decomp -- -D warnings: clean.

## Scope
C++ in scope: `decompiler/cpp/type.cc` lines ~3565-end (`TypeFactory`) plus the
type-3 `TypePointerRel` overrides the factory drives (compare 3072-3090,
compareDependency 3092-3103, downChain 3120-3136, isPtrsubMatching 3138-3147) and
the `Datatype::hashName`/`hashSize` helpers (693-720). Rust diff: `rust/crates/
kuna-decomp/src/dtype.rs` (+2038/-208), porting `TypeFactoryImpl` (the interning
container: `tree`/`nametree`/`typecache*`/`charcache`, `findAdd`/`insert`/
`findByIdLocal`/`findNoName`, `cacheCoreTypes`, every atomic/pointer/composite
getter, `downChain`/`getExactPiece`/`concretize`/`resizeInteger`/`resizePointer`)
and the type-3 `TypePointerRel` methods that were SEAM(W6) stubs after type-1/2.

## Mandatory hunt list
- **Signedness.** clean — `compareDependency`/`compare` tiers all transcribe the
  C++ signed/unsigned operand widths faithfully (uint4 wordsize, sub_metatype Ord,
  int4 offset). The ONE signedness divergence is `hashName`'s unsigned `u8` byte
  read vs C++ signed-`char` `(uint8)nm[i]` — F2, minor, ASCII-only (LOSS-085b).
- **Integer widths.** F1 (minor): `TypePointerRel::isPtrsubMatching` C++ stores its
  running offset in `int4 iOff` (type.cc:3143/3145), truncating the int8 sum to 32
  bits before the bound check; the port keeps it `int8`. Value-identical unless the
  PTRSUB offset exceeds 2^32 against an int4-sized parent (LOSS-085a). Elsewhere
  clean: the inherited `TypePointer::isPtrsubMatching` uses `int8` throughout in
  both (only `typesize` is int4 in C++, widened to int8 in Rust — value-preserving);
  `downChain`'s `int4 ptrtoSize`/`size*8-1`/`signOff % ptrtoSize` all match.
- **Wrapping.** clean — `hashName` rotate-left-8 + `wrapping_add`; `hashSize`
  `(size as i64 as u64).wrapping_mul(0x98251033aecbabaf)` (C++ `uint8 sizeHash =
  size;` sign-extend-then-wrap); `compare(_dependency)` size tails use
  `op.get_size().wrapping_sub(self.size)` matching C++ `(op.getSize()-size)`;
  `downChain` rel_off `((off + offset) as uint8 & calc_mask(size)) as int8` matches
  the C++ uintb mask-and-truncate. No checked/wrapping confusion found.
- **Comparator totality.** clean — `TreeKey::Ord` = (compareDependency, then
  getId()) exactly mirrors `DatatypeCompare` (type.hh:360-366). The `unwrap_or(0)`
  on the fallible `compareDependency` is a TOTAL fallback (ties -> id tie-break),
  and is provably never hit for factory-interned types: the only `Err` path is
  `TypeCode::compareDependency` with TWO bound `FuncProto`s, and every code type
  the factory interns has `proto: None` (`get_type_code_impl`/`make_type_code_named`),
  for which `compare_code_basic(None,None)=Ok(0)`. F2 verifies the PointerRel
  formal/ephemeral `stripped` tie-break is antisymmetric.
- **Iteration-order provenance.** clean — every container is `BTreeSet`/`BTreeMap`/
  sorted-`Vec` (no HashMap/HashSet, grep-confirmed). `cacheCoreTypes` iterates the
  `tree` (C++ `DatatypeCompare` order = TreeKey::Ord order, deterministic for the
  atomic core types it reads since they have no ptrto). `nametree` is a `Vec` kept
  sorted by `(name,id)` via binary-search insertion = C++ `DatatypeNameCompare`.
  `enum` namemap is `BTreeMap` (std::map key order). NOTE: `compareDependency`
  orders ptrto/parent by RAW-POINTER address (`Rc::as_ptr`), exactly reproducing
  C++'s `ptrto < tp->ptrto` raw-pointer comparison — inherently per-run/per-factory
  but internally consistent, and identical to the C++ property (the tree is rebuilt
  each run in both). Not a determinism regression.
- **Off-by-one / do-while / reverse iterators.** clean — `getExactPiece`'s C++
  `do {...} while(ct != null)` is faithfully a Rust `loop { ...; match sub { Some
  => continue with next, None => break } }` (body runs >=1, same op order:
  size-beyond-end check, exact-match return, save-last, getSubType, loop-or-break).
  `getFieldIter` binary search untouched here. `findByIdLocal` id==0 `lower_bound`
  first-match = Rust `filter(name).min_by((name,id))` (smallest id with that name).
- **Erase-while-iterating.** clean — no in-place container traversal-erase. The
  `restampCoreFlag`/`reinternReplace`/`calcTruncate` "mutate interned object in
  place" paths are modeled as tree remove(old key)+insert(new key) where the mutated
  field (coretype flag / truncate) is NOT in compareDependency, so old/new keys
  compare equal -> same tree slot. Removal is unambiguous (id-unique). Default
  config (sizeOfAltPointer==0) makes calcTruncate a no-op, so no spurious churn.
- **Exception -> Result partial-state parity.** clean — `insert` returns Err on a
  shared id BEFORE any nametree mutation (C++ throws after printing, deletes the
  newtype; the partial state — tree already holds the original — matches). `findAdd`
  on a redefinition mismatch returns Err without inserting (C++ same). The SEAM(W6)
  `Err`s (TypeCode proto recursion, TypeSpacebase getSubType) are read-only bails.

## Findings
- F1 (minor): `TypePointerRel::isPtrsubMatching` int4-`iOff` truncation elided —
  port computes the offset sum in int8 and never truncates to 32 bits.
     cpp: decompiler/cpp/type.cc:3143-3146
     rust: rust/crates/kuna-decomp/src/dtype.rs (is_ptrsub_matching, PointerRel arm)
     Severity: minor — value-identical unless a >= 2^32 PTRSUB offset is bounded by
     an int4-sized parent type (out of realistic domain). LOSS-085a.
- F2 (minor): `Datatype::hashName` reads name bytes as unsigned `u8`, not the C++
  signed-`char` `(uint8)nm[i]` (which sign-extends bytes >= 0x80).
     cpp: decompiler/cpp/type.cc:697-702
     rust: rust/crates/kuna-decomp/src/dtype.rs (hash_name)
     Severity: minor — only observable for non-ASCII type-name bytes, which do not
     occur for C-identifier type names; porter doc-comment acknowledges the scope.
     LOSS-085b.
- F3 (nit, no anchor needed): several added widening/index `as` casts
  (`as int8`/`as usize`/`as uint4`) lack the protocol's `// cast:` justification
  comment. All are value-preserving widenings or container indices matching C++
  implicit promotions; no correctness impact. Not gating.

## Adversarial tests
Added `rust/crates/kuna-decomp/tests/verify_w6_s5_type_3.rs` (committed on the
branch, 5 tests, all green):
- `f1_ptrrel_isptrsubmatching_bounds_realistic_domain` — bound inclusivity
  (lower/upper/zero/negative) in the agreeing domain.
- `f1_ptrrel_isptrsubmatching_int4_truncation_divergence_documented` — pins the
  Rust int8 result at the 2^32 boundary (where C++ int4 would truncate), forcing a
  future restoration to notice F1.
- `f2_ptrrel_compare_formal_ephemeral_totality` — formal(None) < ephemeral(Some)
  tie-break, antisymmetry, both-Some / both-None ties.
- `f3_ptrrel_compare_dependency_field_precedence` — submeta/ptrto-identity/offset/
  parent-identity/wordsize/size tier order with shared-`Rc` interning identity.
- `f4_hash_name_size_properties` — hashName header bits + determinism; hashSize
  involution across sizes incl. negative (sign-extend) size.

## losses
LOSS-085 (this item's two minor width/charset divergences, F1+F2). The type-2
SEAM surface this item leaves intact (`TypeCode::compareDependency` proto recursion,
`TypeSpacebase::getSubType`) is already covered by the prior LOSS-082.
