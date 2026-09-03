---
need_id: getprocaddress-result-discarded
title: a GetProcAddress result is discarded and the pointer left uninitialised
track: quality
status: open
severity: major
hypothesis_status: inconclusive
credibility: 0.8
instances: 1
rounds: [1]
first_seen_round: 1
attempts: 0
touches: [decompiler/crates/kuna-decomp]
scope: small
---

## Symptom

Filed by a round-1 tester; gated `admitted`. Verbatim observation and replayed transcript in `.kuna-repipe/rounds/1/gate.json`.

## Decision log

- round 1: gated `admitted`, NOT built (round 1 closed 12 of 23). Carried into round 2.
