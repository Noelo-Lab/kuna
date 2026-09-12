# Mergecheck shape-C findings

Base: `342eaa03f0b71f7c312de004017eb8262fe0f63c`.

Mergecheck reports seven shape-C replacements. They are intentional catalog
prose updates, not sparse reindexing or generated-baseline drift. The exact
old/new line coordinates and durable rationale are:

| # | Base line | Candidate line | Replacement | Why it is truthful and required |
|---|---|---|---|---|
| 1 | `decompiler/crates/kuna-decomp/phases.toml:2922` | `decompiler/crates/kuna-decomp/phases.toml:2922` | `calleeretpreserves.summary`: the body-only DIV-133 explanation -> that complete original explanation plus the locked-void ABI-output fallback and exact `/GS` call-site proof | The implementation now has two positive-evidence arms. The original x86 cspec, RAX/EAX/RCX, Heritage, get-PC, body-validity, and fast-fail `intloc = swi(...)` detail remains intact; the appended text states that an incomplete probe retains positive pre-edge direct-write and STORE-space facts, that every recovered ABI-return write or STORE into an ABI output processor space vetoes the exact arm, and that only the logical 8-byte ABI slice survives within a 16-byte XMM heritage range while killed flanks remain killed. |
| 2 | `decompiler/crates/kuna-decomp/phases.toml:2923` | `decompiler/crates/kuna-decomp/phases.toml:2923` | `calleeretpreserves.use_when`: the existing integer-return symptoms and off/bisect guidance -> those same details plus the declared-void XMM0 cookie-check symptom and exact-arm guardrails | The option now fixes the floating-return shape. The prior diagnostic guidance is retained while documenting ABI-output-only eligibility, one-restart multi-site marking, the override/direct-write/output-space-STORE vetoes, and the expected wide-range discriminator: low XMM0_Qa may survive while upper XMM0_Qb remains undefined and killed. |
| 3 | `decompiler/crates/kuna-decomp/phases.toml:2926` | `decompiler/crates/kuna-decomp/phases.toml:2926` | `calleeretpreserves.inspiration`: only `main-returns-invented-cookie` with the exact vm.exe witness -> that provenance plus the exact PolyMLP hash/function/address witness | Both dataset needs exercise the same option through distinct evidence arms. The original vm.exe identity and upstream code references remain present verbatim. |
| 4 | `decompiler/crates/kuna-decomp/phases.toml:2929` | `decompiler/crates/kuna-decomp/phases.toml:2929` | `calleeretpreserves.symptoms`: six existing symptoms -> the same six plus the XMM6-to-XMM0 disappearance and discarded declared-double helper result | These are the two directly observed PolyMLP symptoms. Every prior symptom remains an unchanged prefix. |
| 5 | `docs/options.md:2083` | `docs/options.md:2085` | generated `Symptoms` line -> generated line containing the two additional symptoms | This is the generated mirror of finding 4; leaving the old line would fail catalog freshness. |
| 6 | `docs/options.md:2084` | `docs/options.md:2086` | generated `What it does` line -> generated line matching the extended summary | This is the generated mirror of finding 1 and retains all prior catalog detail before the extension. |
| 7 | `docs/options.md:2085` | `docs/options.md:2087` | generated `When to flip` line -> generated line matching the extended `use_when` guidance | This is the generated mirror of finding 2 and retains the existing off/bisect guidance. |

The two symptom-index rows at `docs/options.md:628-629` are additive, not
shape-C replacements. `docs/history.md:308-330` is likewise an additive
DIV-133 extension entry. The `msvcstackguard` surface note is byte-identical to
the base, including its distinction from the glibc matcher. Mergecheck reports
no source-code or counter replacement findings, and the 675-case parity
baseline at `docs/baseline.json` is byte-identical to the base. The stage
baseline is intentionally additive: current main's entry-return-dispatch case
and this feature's eight assertions are both retained, bringing the corpus to
811/811, including the production incomplete-probe write veto and the 16-byte
XMM low-survives/upper-killed split. Focused predicate coverage also pins an
incomplete summary with no direct ranges and a retained output-space STORE.
Mergecheck re-derived 191 settables, tiers
`(61, 71, 59)`, 279 corpus files, and next ElementId 4165. The only replay
conflicts were the corpus-count comment/assert and the stage baseline footer;
both were regenerated/derived from the combined tree rather than taking either
parent. The authoritative need index remains byte-identical to the base.
