# Verification evidence

`verification_evidence.py` emits structured evidence from the existing
Tier C JSON lines, the three-way function oracle, and symbolic reports.
Each record carries the outcome, assurance, backends, coverage, bounds
and the digest of the source report. It preserves archived witnesses
and points to the exact proof queries by digest.

```sh
python3 scripts/diffverify/verification_evidence.py tier-c.jsonl --kind tier-c
python3 scripts/diffverify/verification_evidence.py comparison.json --kind cross-backend
python3 scripts/diffverify/verification_evidence.py proof.json --kind symbolic \
  --verified-certificate certificate.json
```

The Tier C outcomes map to `SAMPLED_AGREEMENT`, `COUNTEREXAMPLE`,
`DECLARED_DIVERGENCE` and `INCONCLUSIVE`. An independent execution
agreement applies to its recorded case and observations. A confirmed
comparison mismatch remains a counterexample when the case also has
unsupported effects. Model and bounded proofs carry the scope recorded
by the symbolic encoders.

Pass the successful output of `proof_certificate.py` with
`--verified-certificate`. Its source digest and checked obligations must
match the symbolic report. Certification keeps the proof's original
domain and bounds. [Proof certificates](proof-certificates.md)
