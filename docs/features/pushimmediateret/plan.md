# pushimmediateret design

1. Extend the existing bounded raw-p-code RET provenance with a distinct
   one-store immediate-tail result. Reject the adjacent-store pair owned by
   `entryretdispatch`.
2. Add the default-off development option `pushimmediateret` in P2. At the
   shared decompile boundary, seed a BRANCH flow fact for the proven RET unless
   the caller supplied any explicit fact at that site; treat explicit RETURN as
   a clean veto.
3. Add an i386 fixture whose positive is `call unpacker; push encrypted_oep;
   ret`, with no function symbol at the encrypted target. Include ordinary,
   argument-push, stack-adjust, stack-overwrite, computed-target, conditional,
   and two-push dispatch negatives.
4. Add focused provenance, CLI, and two-pass stage tests. Replace both weak
   dataset acceptances with one promoted in-repo predicate that matches the
   actual indirect transfer statement inside `functions[0].code`, so a warning
   containing the address cannot pass.
5. Measure option off/on, run the default-on 675-assertion ablation and a
   whole-fixture off/on sweep, then decide the shipped default. Update catalog
   metadata/counters, the P2 spec, history if default-on, and both need records.

Scope is small: one bounded P2 flow-classification decision, one new option
identity module, and reuse of the existing raw-p-code provenance and flow-
override seam. It adds no pass type, dataflow machinery, structuring work, or
target-function discovery.
