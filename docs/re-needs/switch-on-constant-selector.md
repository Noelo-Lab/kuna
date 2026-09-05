---
need_id: switch-on-constant-selector
title: a recovered switch dispatches on a constant, every case dead
track: quality
status: closed
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 0
rounds: [1]
first_seen_round: 1
attempts: 1
touches: [decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitch.rs]
scope: small
closed_in_round: 1
closing_pr: 367
---

## Symptom

Filed by round-1 testers on the crackmes corpus. See `.kuna-repipe/rounds/1/gate.json` for the verbatim observations and their replayed transcripts.

## Acceptance

CLOSED in round 1. Shipped: `switchselector` (default OFF -- pass `--option switchselector on`): declines a lowered-switch install whose selector cannot be re-read, leaving the compiler if/else-if chain over the real parameter

## Decision log

- round 1: gated `admitted`, built, merged in PR #367; probe re-run on the merged build confirms the bad behaviour no longer reproduces.
