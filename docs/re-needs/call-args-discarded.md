---
need_id: call-args-discarded
title: recovered call arguments are discarded at the call site
track: quality
status: closed
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 0
rounds: [1]
first_seen_round: 1
attempts: 1
touches: [decompiler/crates/kuna-decomp/src/p4_params]
scope: small
closed_in_round: 1
closing_pr: 367
---

## Symptom

Filed by round-1 testers on the crackmes corpus. See `.kuna-repipe/rounds/1/gate.json` for the verbatim observations and their replayed transcripts.

## Acceptance

CLOSED in round 1. Shipped: `varargstackargs` and `calleearity` (both default OFF): two ways a recovered argument was dropped

**Closed WITHOUT an acceptance flip.** Filed by hand, so this record carries no `probe_id`/`acceptance_id` and the gate never closed it -- a human did. The design says only the acceptance probe may close a need; this one did not go through that. The durable evidence that the capability works is:
- `tests/stages/kuna-calleearity.xml`
- `tests/stages/kuna-varargstackargs.xml`

Recorded rather than back-filled: a probe retro-fitted now would assert TODAY's behaviour, not the behaviour at closing time, and would look like gate evidence while being nothing of the kind.

## Decision log

- round 1: gated `admitted`, built, merged in PR #367; probe re-run on the merged build confirms the bad behaviour no longer reproduces.
