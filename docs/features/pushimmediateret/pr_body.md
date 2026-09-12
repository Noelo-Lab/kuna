## The problem

CryptoME's i386 unpacking entry executes `call 0x4f7990; push 0x40365a;
ret`. Kuna kept the unpacker call but treated the final RET as an ordinary
return, so dead-store removal silently erased the transfer to the original
entry point. Two RE-friction records describe the same defect:
`push-immediate-ret-transfer` and `entry-point-push-return`.

## The fix

- The bounded raw-p-code stack provenance shared with `entryretdispatch` now
  distinguishes a one-store immediate RET tail from its two-store call form.
- A proven one-store form seeds P2 BRANCH, never CALL, so there is no
  fall-through. Calls, stack changes or overwrites, computed targets,
  conditional/indirect flow, and opaque userops decline.
- Explicit flow facts remain authoritative, including a RETURN veto.
- The destination is not synthesized as a function. CryptoME's bytes there are
  encrypted until the unpacker runs.
- The new `pushimmediateret` option ships on by default (DIV-170); `option
  pushimmediateret off` restores the former bare-return output.

The mechanism and proof boundary are recorded in
[`analysis.md`](analysis.md) and [`plan.md`](plan.md).

## Evidence

- Default-on ablation: **675/675 PARITY OK**, no baseline re-pin.
- Stage gate: **813/813 PARITY OK** after adding the two-pass witness.
- Exact CryptoME speed: 121.24 ms off vs 110.71 ms on (11-sample medians,
  **-8.69%**, within the 5% regression budget).
- All ordinary/adversarial negatives are byte-identical; the two-push form
  remains owned by `entryretdispatch`.
- The shared acceptance for both duplicate needs passes three repetitions and
  matches a complete unpacker-call plus switch/case construct. Address digits
  in a warning cannot satisfy it.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/)
