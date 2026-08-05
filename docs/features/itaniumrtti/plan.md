# itaniumrtti — implementation plan

## 1. Scope — one of two separable halves

`analysis.md` §2 splits the Itanium C++ recovery problem in two and measures both
before choosing. This PR ships **half (a)**, typeinfo/class/inheritance recovery
*plus* the vtables reachable from it. It deliberately does **not** ship half (b),
independent code-pointer-run vtable detection, because on the benchmark corpus that
scan measures relocation residue rather than vtables.

One PR per feature: (b), if it is ever worth doing, is a separate option with its
own witness and its own wrongness measurement.

## 2. Where it runs — load time, analysis tier

The anchor is a **dynamic relocation**, so this is a loader/analyzer-tier fact
established at `load file`, not a decompile pass. It is a new
`crate::pass::AnalysisPass` (`phase P1`, `id "itaniumrtti"`) whose facts are
computed at load and **committed** only when the gate is on
(`engine.rs::analysis_pass_enabled` → `arch.analysis_itaniumrtti`), the same
gate-at-commit shape `rtti` / `cortexmvectors` / `peimportcall` use.

Registration is ELF-only in `passes::passes_for`, so every non-ELF target's pass
list is byte-identical to before the pass existed; the pass *also* self-gates on
`BinaryFormat::Elf` in `run`.

## 3. Module layout — reuse, don't re-port

New file `analyzers/rtti/kuna_itaniumrtti.rs`, a sibling of the MSVC pass inside the
folder that already owns RTTI. Reused rather than duplicated:

| existing | reused for |
|---|---|
| `rtti::models::ImageBytes` | VMA-addressable section view, `read`, `read_cstr` |
| `rtti::refkind::RefKind` | pointer width + endianness, `read_ptr`, `read_u32` |
| `rtti::vftable::TextRanges` | "does this slot point into code" |
| `demangle` (`cpp_demangle`) | the class name, via a new `_ZTS`-wrap helper |

Two small additions to existing files rather than new abstractions:

- `ImageBytes::scan_ptr_slots(rk, targets)` — the **many-targets** form of the
  existing `find_ptrs`. Calling `find_ptrs` once per class is `O(classes x image)`,
  which on a library with hundreds of polymorphic classes is the entire cost of the
  pass; one scan is `O(image)`.
- `demangle::demangle_typeinfo_name` — wraps a bare `<mangled-name>` component back
  into `_ZTS<component>` and demangles that, stripping the rendered
  `typeinfo name for ` prefix. The exact analog of the existing MSVC
  `demangle_rtti_class_name` (`??_R0…@8` wrap). **No new demangler dependency.**

## 4. The algorithm

1. **Discover** typeinfo objects, two anchors unioned:
   - dynamic relocations whose target symbol is one of the three `__cxxabiv1`
     typeinfo vtables, addend `2 x ptr` (flavour *known* from the symbol);
   - defined `_ZTI…` OBJECT symbols (flavour unknown, sniffed structurally) — the
     unstripped / statically linked path.
2. **Decode** in two sweeps: accept a candidate whose name pointer resolves to a
   string that demangles; then, with the accepted set in hand, sniff any unknown
   flavour and read the base list (each base pointer must resolve to an accepted
   typeinfo).
3. **Recover sub-vtables**: one `scan_ptr_slots` for every pointer slot holding an
   accepted typeinfo address, then reject on the two ABI constraints
   (`offset-to-top <= 0`; at least one code slot) and on "the candidate header lies
   inside a typeinfo object".
4. **Walk slots** to the first word that is neither a code pointer nor a
   loader-filled import slot; trailing import slots are dropped.
5. **Attribute + emit**.

## 5. Naming

| address | symbol | kind |
|---|---|---|
| `_ZTI<C>` | `<C>_typeinfo` | Data |
| `_ZTS<C>` | `<C>_typeinfo_name` | Data |
| sub-vtable header | `<C>_vtable` / `<C>_vtable_for_<Base>` | Data |
| first virtual slot | `<C>_vptr` / `<C>_vptr_for_<Base>` | Data |
| slot target | `<C>::vtable_<i>` / `<C>::vtable_for_<Base>_<i>` | Function |

Data uses `_`, functions use `::` — see `analysis.md` §8; the choice was made after
observing the printer render `Shape::vptr` as a bare `vptr`.

Slot arrays are additionally pushed to `AnalysisOutput::readonly`.

## 6. The gate — a NEW option, not an extension of `rtti`

`itaniumrtti = on | off`, default **off**. The alternative considered was widening
the existing PE-only `rtti` option to dispatch on format. Rejected for one reason
that outweighs the "fewer knobs" argument:

**the two paths have different default-ON trajectories.** The MSVC path is a
byte-pattern heuristic (scan for `.?A`, assume `ref − 12` is a structure) and will
plausibly stay opt-in forever. The Itanium path is relocation-anchored and exact,
with a measured 0-change footprint on 15 plain-C binaries — a real default-ON
candidate once a `-frtti` corpus exists to back it. Sharing one flag would chain the
sound recovery to the unsound one permanently.

Secondary: the two are mutually exclusive by format anyway (MSVC RTTI only exists in
PE, Itanium only in ELF), so a shared flag buys no user simplification; and widening
a shipped option silently changes what it means on ELF inputs.

Full `settableTable` row in `phases.toml` (every field incl. `tier` + `symptoms`)
plus registration in `p0_knowledge/options.rs`, the `Architecture` flag, the
`analysis_pass_enabled` arm, and the count bumps — all counts read off a fresh green
build, never computed.

**DIV-64.** The *option* diverges from nothing — its default is `off` and
`--mode reliable` is byte-identical. But §9 puts it in `AGGRESSIVE_OVERRIDES`,
which `auto` selects below 500 KiB, so the preset makes it the default rendering on
ELF C++ with RTTI — a default-output change that **no parity oracle can see**, which
is exactly why DIV-43 exists for the symmetric removal of `dwarf_lines` from the
same preset. Without preset membership this would correctly carry no DIV row.

## 7. Testing

**No stage XML.** The XML `<binaryimage>` datatest bootstrap never produces an
`AnalysisOutput` and so cannot reach the analyzer tier at all — the same reason
`cppsig`, `cppproto`, `fdeinterior` and `cppcallnames` all use Rust integration
tests. `decompiler/crates/kuna-console/tests/verify_itaniumrtti.rs` is the two-pass
e2e gate (7 assertions), precedent `verify_fdeinterior.rs` / `verify_cppcallnames.rs`.

**New fixture** `itaniumrtti_x86_64.so` — `g++ -O0 -fPIC -shared -fvisibility=hidden
-fvisibility-inlines-hidden`, then `strip --strip-all`. Hidden visibility is
load-bearing: without it every implicit class method is emitted WEAK **and
exported**, so `.dynsym` alone would name them and the test would prove nothing.
Hidden, the only defined dynamic symbols are the three `probe_*` entry points. The
classes cover all three `__cxxabiv1` flavours including a `__vmi_class_type_info`
with a base at +16 (so the vtable object carries a real thunk block), **plus the two
naming hazards an adversarial probe found**: two template instantiations whose
name-only demangling collides (`Vec<int>` / `Vec<double>`), and an
anonymous-namespace class whose ABI type-name string carries the TU-local `*`
marker.

Plus 8 hermetic unit tests in the module over a synthetic image (flavour mapping,
si-inheritance decode, base-pointer rejection, defining-base attribution, secondary
sub-vtable naming, `offset-to-top` sign extension per pointer width, data-label
composition, and class-name sanitization keeping two template instantiations
distinct), and one in the demangler for the `_ZTS` wrap incl. the TU-local `*`
marker.

## 8. Measurement plan

- **Sweep**: the pre-PR default path (`--mode aggressive --option itaniumrtti off`)
  vs the post-PR default path (`--mode aggressive`) over the C fixture corpus (must
  be 0) and the C++ corpus, every hunk classified; plus the above-threshold binaries
  (leveldb `-O0`/`-O2`, `mcount_x86_64`) at the option level.
- **Speed**: interleaved off/on, **minimum of 15** pairs. `kuna functions` (load +
  commit only) is the tightest isolation because the pass is load-time; also
  `decompile-all` end-to-end, on both a C++ RTTI binary and a plain-C control.
- **Benchmark**: report the leveldb result honestly. It is expected to be zero and
  the reason (`-fno-rtti`) is structural, not a tuning failure.

## 9. Reachability — preset membership is part of the feature, not an extra

Shipping the option default-off is only half the decision. `auto` resolves to
`aggressive` below 500 KiB, so an option absent from `AGGRESSIVE_OVERRIDES` never
runs on the default path — `decompile-all`, the `kuna` CLI, the web front-end and the
benchmark all miss it. This feature was first written in exactly that state and
measured **0 recovered names** under `--mode aggressive`.

`itaniumrtti` therefore joins the preset. It is the same class the preset's own
doc-comment already names as safe to blanket-enable — the format-gated no-ops
(`rtti`/`pdb` = PE, `objc`/`macho-arm64e` = Mach-O, `sparcstructret` = SPARC) — with
a strictly narrower firing condition: ELF **and** a `.rela.dyn` carrying
`__cxxabiv1` typeinfo relocations.

Because mode presets are not a datatest surface, a membership change alters default
rendering where no parity oracle can see it. That is precisely why DIV-43 exists for
the symmetric case (removing `dwarf_lines` from the same preset), so this takes a DIV
row too: **DIV-64**.

The invariant behind all of this was only *sampled* by a test, never enforced, which
is why the omission was silent. `modes.rs::aggressive_carries_every_default_off_option`
now enforces it — and immediately found four further default-off options in the same
trap (`cortexmvectors`, `ptrentry`, `tailcallentry`, `paramcopyhoist`), listed as
`UNEVALUATED` rather than flipped, since each needs its own sweep and PR.

The sweep baseline changes with the preset: the honest comparison is no longer
"default vs `--option on`" but the **pre-PR default path vs the post-PR default
path**, i.e. `--mode aggressive --option itaniumrtti off` vs `--mode aggressive`.
