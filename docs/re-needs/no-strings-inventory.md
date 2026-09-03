---
need_id: no-strings-inventory
title: kuna cannot list or search the strings in a binary
track: tooling
status: closed
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 0
rounds: [1]
first_seen_round: 1
attempts: 1
touches: [decompiler/crates/kuna-cli/src/strings.rs, docs/cli.md]
scope: small
closed_in_round: 1
closing_pr: 367
---

## Symptom

Filed by round-1 testers on the crackmes corpus. See `.kuna-repipe/rounds/1/gate.json` for the verbatim observations and their replayed transcripts.

## Acceptance

CLOSED in round 1. Shipped: `kuna strings <bin> [--json] [--min-length N] [--filter RE] [--encoding ascii|utf16|all] [--section S]` -- each row carries the VMA, the section, and the OWNING FUNCTION, which strings(1) cannot give

## Decision log

- round 1: gated `admitted`, built, merged in PR #367; probe re-run on the merged build confirms the bad behaviour no longer reproduces.
