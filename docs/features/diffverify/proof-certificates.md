# Checking DiffVerify proof certificates

DiffVerify archives each verification condition as SMT-LIB with its digest.
The optional certificate tool asks cvc5 for a Cooperating Proof Calculus
(CPC) proof of each required UNSAT query and checks the proof with Ethos.
Ethos validates the proof against the original SMT-LIB assertions and must
derive `false` at assumption level zero.

```sh
python3 scripts/diffverify/proof_certificate.py rv64-proof.json \
  --signature "$CPC_SIGNATURE" --cvc5 "$CVC5" --ethos "$ETHOS" > certificate.json
python3 scripts/diffverify/proof_certificate.py rv64-proof.json \
  --signature "$CPC_SIGNATURE" --cvc5 "$CVC5" --ethos "$ETHOS" \
  --check-bundle certificate.json
```

Use the `proofs/eo/cpc/Cpc.eo` file and its sibling folders from the
[cvc5 1.4.2 source tree](https://github.com/cvc5/cvc5/tree/cvc5-1.4.2/proofs/eo/cpc)
with [Ethos 0.2.5](https://github.com/cvc5/ethos/releases/tag/ethos-0.2.5).
The bundle binds the original evidence, every checked query, the proof text,
and the transitive CPC signature files to digests. In check mode cvc5 only
rechecks the satisfiable domains. Ethos checks the stored refutations without
asking the proof producer to generate them again.

`CERTIFIED_EQUIVALENCE` carries the original result and its scope. A
certificate derived from `BOUNDED_EQUIVALENCE` covers the same bound. For
induction the bundle includes initiation, preservation and the validated
ranking obligation. For an independent Sail comparison it includes trace
coverage and both comparison obligations. Missing queries, solver
timeouts, unverified proof rules and proof digest mismatches yield
`INCONCLUSIVE`.

The certificate checks the archived formulas. Their correspondence to the
ELF execution still relies on the relevant lifter or ISA model, symbolic
encoders, projections and environmental assumptions. Proof checking
trusts Ethos and the CPC signature, and satisfiability checks trust cvc5.
