# w4-fw-options
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier, round 3)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (250 lib + every verify pack green; this item's `verify_w4_fw_options` = 16/16, including the round-2 F3 REJECT-evidence tests which now PASS and 4 new round-3 adversarial tests)

## Scope and what changed since round 2

C++: `decompiler/cpp/options.cc` (1107 lines), `decompiler/cpp/options.hh` (354 lines).
Rust: `rust/crates/kuna-decomp/src/options.rs`, `src/options/tests.rs`.
Branch: `worktree-wf_a6e29a1b-137-2` @ `f8c688f` (round-3 repair) + the verifier
test commit `34a3325`.

The round-3 repair (`f8c688f`) touched ONLY `parse_int_auto`'s `!any` branch
(31 src lines, the exact F3/LOSS-045 fix) plus the in-module tests. The body of
the port — all ~40 upstream `ArchOption` subclasses, registration order, element
ids, the `OptionDatabase` dispatch, the `decodeOne`/`decode` path — is unchanged
from round 2 and was re-confirmed faithful this round.

## F1 (round-1 major, integer-WIDTH saturation) — RESOLVED, re-verified

Recorded `LOSS-040 CLOSURE` (round 2). Re-ground-truthed this round against a
fresh g++ 11 + clang++ -std=c++11 `istringstream >> int/unsigned` oracle: target-
width saturation is faithful (`"3000000000">>int=INT_MAX`, `"0x100000000">>uint=
UINT_MAX`, `"-2147483648">>int=INT_MIN` no-fail, `"-2147483649">>int=INT_MIN`
fail, `"-4294967295">>uint=1` no-fail, `"-0x100000000">>uint=UINT_MAX` fail).

## F3 (round-2 major, num_get failbit-with-zero) — RESOLVED, verified faithful

The round-3 fix captures `empty_field = i >= bytes.len()` right after the
leading-whitespace skip (options.rs:400) and, in the no-digit-extracted branch
(:441-452), returns `None` only when `empty_field` (empty/whitespace-only ->
target unchanged -> caller's sentinel survives -> reject) and
`Some(T::from_u64(0,false))` otherwise (non-empty no-leading-digit -> failbit-
with-zero store -> caller accepts). This is exactly LOSS-045's restoration
criteria.

Independently ground-truthed against a g++ 11 + clang++ -std=c++11 oracle — every
case matches:

| input | C++ `>> int` (val<-=-1) | port | match |
|---|---|---|---|
| `"abc"` | 0, fail | 0 | yes |
| `"-"` | 0, fail | 0 | yes |
| `"  abc"` | 0, fail | 0 | yes |
| `"0xZ"` | 0, fail | 0 | yes |
| `".5"` | 0, fail | 0 | yes |
| `"+"` | 0, fail | 0 | yes |
| `""` | -1 (unchanged), fail | -1 | yes |
| `"   "` | -1 (unchanged), fail | -1 | yes |

The three round-2 REJECT-evidence tests
(`{maxlinewidth,maxinstruction,extrapop}_nonnumeric_input_accepts_as_zero_like_cpp`)
now PASS; `maxlinewidth_empty_input_errors_in_both` confirms the empty case is
still faithful. Recorded as `LOSS-045 CLOSURE` (RESOLVED) in losses.md.

## Hunt list (independently re-derived)

- **Signedness**: clean. `maxinstruction` `newMax<0` (int4, options.cc:993),
  `extrapop` `expop==-300` (int4, :269), `jumptablemax` `val==0` (uint4, :883),
  `maxlinewidth`/`indentincrement`/`commentindent` `val==-1` (int4, :520/:537/:555)
  — every sentinel transcribed with the C++ type and comparison; the
  `width_overflow = val > T::max_magnitude(neg)` compare is `u64 > u64` (total).
  No signed/unsigned comparison silently flipped.
- **Integer widths**: F1 RESOLVED. The `as` narrows in `from_u64`/`max_magnitude`
  (options.rs:477-515) are guarded by the width-saturation so they only run on
  in-range magnitudes; each carries a `// justified:` comment. The digit-byte
  widenings (:418-420, `(c-b'0') as u64` etc.) are lossless (value <= 15).
- **Wrapping**: `overflowing_mul`/`overflowing_add` in the digit loop and
  `wrapping_neg` in the sign application reproduce the C++ unsigned negate-modulo
  quirk; verified against the oracle (`"-1">>uint=UINT_MAX` no-fail,
  `"-4294967295">>uint=1` no-fail). A `wrapping_add` that should be `checked_add`:
  n/a (the overflow flag drives saturation, not a panic).
- **Comparator totality**: n/a. The only ordering is a `uint4`-keyed `BTreeMap`
  (point `get` + `insert` + drop); no ported `operator<`, no float compare.
- **Iteration-order provenance, per loop**: (1) C++ dtor `for(optionmap.begin()..)`
  delete-all is order-immaterial -> Rust `Box` drop; (2) `optionmap.find(id)` is a
  point lookup -> `BTreeMap::get`; (3) `OptionDatabase::new` registration walks a
  fixed source-order list mirroring options.cc:119-155 -> deterministic; (4)
  `parse_int_auto` forward byte index; (5) `register_option_elements` over the
  fixed `UPSTREAM_OPTION_ELEMENTS` slice. No `HashMap`/`HashSet` anywhere.
- **Off-by-one / do-while / reverse iterators**: `decodeOne` nested-if param walk
  is a 1:1 transcription (options.cc:207-232 -> options.rs:1955-1986), including
  the quirk that a first child that is neither `<param1>` nor absent leaves the
  sub-element unclosed and `p1` empty; `decode` `while peekElement()!=0` matches.
  No do-while, no reverse iterators, no `lower_bound`/`upper_bound`.
- **Erase-while-iterating**: none in options.cc.
- **Exception -> Result partial-state parity**: re-checked every multi-write body.
  `OptionAliasBlock` (writes field only in valid match arms, error arm throws
  before write -> field unchanged on error: FAITHFUL), `OptionNanIgnore` (same
  shape: FAITHFUL), the comment-toggle options (`on_or_off(p2)?` then
  `encode_comment_type(p1)?` then the set, same order as C++, no write before a
  throw: FAITHFUL), `OptionExtraPop`/`OptionCurrentAction` (throw/seam ordering
  matches). **Only `OptionSplitDatatypes` diverges — F2 below.**
- **Aliasing**: n/a (no shared mutable reference retained across calls; `glb` is a
  borrowed `&mut dyn` passed per `apply`).

## Findings

  - F2 (minor): `OptionSplitDatatypes::apply` partial-state divergence on a
       malformed p2/p3. C++ writes `glb->split_datatype_config = getOptionBit(p1)`
       (options.cc:1047) BEFORE `getOptionBit(p2)`/`(p3)` can throw, so a malformed
       p2 leaves the Architecture at `getOptionBit(p1)` (= `option_struct`); the
       Rust accumulates into a local `config` and writes glb only after all three
       parse, so on the same input glb stays at `old_config` (0 by default).
       Confirmed by `splitdatatype_bad_p2_partial_state_divergence_documented`
       (port leaves 0; C++ would leave 1). The thrown error text is identical;
       only the surviving field differs. Narrow trigger (malformed multi-param
       `splitdatatype`), error-path-only, and the field is overwritten by the next
       valid command — so it cannot change a successful decompilation. This is the
       lone upstream multi-write body that writes incrementally before a possible
       throw (the other three write after the fallible parse). Minor; accepted as
       LOSS-047.
       cpp: decompiler/cpp/options.cc:1046-1049
       rust: rust/crates/kuna-decomp/src/options.rs:1768-1772

## Mechanical pass

- `todo!`/`unimplemented!`/`unreachable!`: none in `options.rs`/`options/tests.rs`.
- `HashMap`/`HashSet`: none. Dispatch is `BTreeMap<uint4,_>` (mirrors `map<uint4,_>`).
- `sort_unstable`: none.
- Bare `as` casts: lines 477/479/486/488/509/515 carry `// justified:` comments;
  418-420 are single-ASCII-hex-digit-byte `as u64` widenings (value <= 15,
  lossless). No unjustified cast.
- `cargo clippy -p kuna-decomp`: clean (0 warnings/errors).
- Item gate `cargo test -p kuna-decomp`: PASS — 250 lib tests + every verify pack;
  `verify_w4_fw_options` 16/16.

## Adversarial tests

`rust/crates/kuna-decomp/tests/verify_w4_fw_options.rs` (committed @ `34a3325`),
16 tests, public `options` API only. Carried from round 2 (now all green): the F1
boundary guards and the three F3 non-numeric-accepts-as-zero tests (the round-2
REJECT evidence, which the round-3 fix turned green) + `maxlinewidth_empty_input_errors_in_both`.
New this round (4):
  - `extrapop_minus_300_and_empty_throw_zero_and_abc_accept` — the `-300` sentinel
    interplay: `"-300"` (digits parse to the sentinel -> throw), `""` (empty ->
    sentinel survives -> throw), `"0"` (-> accept, store 0), `"abc"` (failbit-with-
    zero -> accept, store 0). Proves the F3 fix did not disturb the sentinel.
  - `maxinstruction_octal_and_hex_base_edges_match_cpp` — base auto-detection:
    `"08"=>0`, `"019"=>1`, `"0x10"=>16`, `"0xG"=>0`, `"0x"=>0`, `"0177"=>127`,
    `"00"=>0`, each matched to the `>> int` oracle.
  - `jumptablemax_zero_and_nonnumeric_both_reject_via_zero_sentinel` — the uint4
    `val==0` sentinel collides with both the num_get-unchanged 0 and the failbit-
    with-zero 0, so `"abc"`/`""`/`"0"`/`"00"`/`"0x0"` all reject in both C++ and
    the port (and `"5"` is stored).
  - `splitdatatype_bad_p2_partial_state_divergence_documented` — F2 evidence:
    `splitdatatype struct garbage` errors identically but leaves the port's
    `split_datatype_config` at 0 (C++ leaves `option_struct`).
All oracle values produced with standalone g++ 11 + clang++ `-std=c++11`
`istringstream >> int/unsigned` harnesses (`unsetf(dec|hex|oct)`).

## Disposition

ACCEPT-WITH-LOSSES. F1 (round-1 major) and F3 (round-2 major) — both num_get
integer-parse divergences — are resolved and independently verified faithful
against the C++11 oracle across the full edge set. The full hunt list is clean
except the one remaining minor: F2, a partial-state-on-error-path divergence
unique to `OptionSplitDatatypes`, error-path-only and overwritten by the next
valid command, recorded as LOSS-047. No blocker/major stands, so the item is
accepted.

losses: LOSS-047 (F2, new this round); LOSS-040 CLOSURE (F1, round 2) and
LOSS-045 CLOSURE (F3, round 3) record the resolved divergences.
