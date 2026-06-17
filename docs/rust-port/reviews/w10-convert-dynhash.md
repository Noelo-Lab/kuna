# Convert (convert.xml, 17) — no-op-gated; calc_hash is FAITHFUL, real gate is constant-sizing

Wave wnmcmkjpf (2026-06-17) declined. The triage hypothesis (calc_hash constant-arm
fidelity bug) was DISPROVEN by the live-engine port — another instance of meta-lesson #1
(read-only passes get the mechanism wrong; the port attempt is the real test).

## calc_hash is faithful (proven)

Offline CRC replay seeded from live-engine traces (the constant root + opedge list): char
#17 const `0x61` self-hashes to EXACTLY the canonical `0x20c20f516a` at size 8 — Rust
matches. The hash algorithm is correct.

## The real root: CALL-arg constant SIZE (not the hash)

The 16 CALL-argument constants require a **size-4 fold** to match the canonical stored hash
(`size=4, fold=4, slot=1, tcode=7=CPUI_CALL` reproduces `0x721eecc7` for `dec 100`). Rust
feeds them at **size 8** → low-32 diverges (`0x813f5bc5` vs `0x721eecc7`). **Same algorithm,
wrong INPUT: the constant Varnode size.** All 17 convert assertions fail because the equated
constant's hash doesn't match the upstream-authored stored hash, so `find_varnode` → None and
the display-format equate never binds.

## The two-part gate (Convert B1 + B2)

- **B1 (constant sizing):** make CALL-argument constants adopt the committed callee parameter
  size (4 bytes for `recv_signed(int4)`/`recv_unsigned(uint4)`) so the dynamic-hash
  constant-fold uses size 4 and matches the canonical hashes. This is a **call-site
  argument / type-propagation** fix — NOT dynamic.rs. (Locus: where CALL inputs get their
  type/size from the callee proto — likely coreaction_protos/coreaction_infertypes; the
  latter is currently owned by struct-return-v2, so scope precisely after it lands.)
- **B2 (copy-elim honoring the dynamic-hash pin):** the early ActionDynamicMapping must pin
  the equated constant's COPY so it survives merge/copy-propagation to the late
  ActionDynamicSymbols + render. C++ does this via `Varnode::mapped` (varnode.cc:448
  setSymbolEntry) + copy-elimination honoring `isMapped()`. The Rust copy-elim does not honor
  the mapped flag for these const-input COPYs.

## Shelved prerequisite (branch rport/w10-convert-dynhash @ 5685b39, +0 alone)

Faithful, regression-free, metric-neutral — the foundation for the eventual B1+B2 wave:
1. `DynamicSymbolSpec` carries category+dispflags+equate_value across the kuna-console
   Funcdata rebuild so an EquateSymbol from `map convert` re-seeds as category=equate with its
   forced format (database.rs scope_dynamic_symbol_specs + funcdata.rs seed_dynamic_symbols) —
   verified attempt_dynamic_mapping then sees cat=1/EQUATE and fires the equate arm.
2. Both equate-mapping arms set `Varnode::mapped`, mirroring C++ setSymbolEntry.

**Convert QUEUED as a B1+B2 wave (+17) building on 5685b39 — after struct-return-v2 frees
coreaction_infertypes (B1 may need it). Not a dynamic.rs quick win after all.**
