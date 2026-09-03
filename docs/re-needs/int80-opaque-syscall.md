---
need_id: int80-opaque-syscall
title: 32-bit Linux int 0x80 renders as swi(0x80)
track: quality
status: closed
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 0
rounds: [1]
first_seen_round: 1
attempts: 1
touches: [decompiler/crates/kuna-decomp/src/p2_lift/kuna_linuxsyscall.rs]
scope: small
closed_in_round: 1
closing_pr: 367
---

## Symptom

Filed by round-1 testers on the crackmes corpus. See `.kuna-repipe/rounds/1/gate.json` for the verbatim observations and their replayed transcripts.

## Acceptance

CLOSED in round 1. Shipped: `linuxsyscall` (default OFF -- pass `--option linuxsyscall on`): names the syscall EAX selects and takes the ABI registers as arguments

## Decision log

- round 1: gated `admitted`, built, merged in PR #367; probe re-run on the merged build confirms the bad behaviour no longer reproduces.
