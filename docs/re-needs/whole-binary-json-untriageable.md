---
need_id: whole-binary-json-untriageable
title: whole-binary JSON is too large to triage from
track: tooling
status: closed
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 1
rounds: [1]
first_seen_round: 1
attempts: 1
touches: [decompiler/crates/kuna-cli/src/decompile_all.rs, docs/cli.md]
scope: small
closed_in_round: 1
closing_pr: 367
---

## Symptom

Filed by round-1 testers on the crackmes corpus. See `.kuna-repipe/rounds/1/gate.json` for the verbatim observations and their replayed transcripts.

## Acceptance

CLOSED in round 1. Shipped: `kuna functions --summary --filter RE --reachable-from F --min-size N --sort size --limit N` -- on a 211 KB PE the summary is 2.8 KB against 174 KB unfiltered

## Decision log

- round 1: gated `admitted`, built, merged in PR #367; probe re-run on the merged build confirms the bad behaviour no longer reproduces.
