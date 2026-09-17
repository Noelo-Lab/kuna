# Type-recovery campaign — progress ledger

Plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md` (session 5b38c0e9). Resumable: every item's
state lives in this table; agents also leave `<worktree>/.scratch/state.md`.

Started 2026-09-16 on main `809712e9`. Stage 1 landed 2026-09-16/17 (#647 #648 #649 #650); typetools #653 pending. Decisions: three equal lanes; Opus subagents; self-merge after
reviewer + green gates; `[PROPOSAL]` items wait for the user; port TRex/RecStruct from source with attribution.

| id | item | lane | state | worktree / branch | PR | last gate result | next step |
|---|---|---|---|---|---|---|---|
| S0.1 | make binaries (main) | setup | done 18:54 | — | — | rc=0 | baseline copy at /home/mahaloz/kwt/_baseline/ |
| S0.2 | reclaim orphan worktrees | setup | done (116 dirs, ~43G) | — | — | — | .claude/worktrees left alone |
| S0.3 | campaign state dir | setup | done | — | — | — | — |
| S0.4 | canonical typesweep baseline | setup | done | /home/mahaloz/kwt/_baseline | — | 848/10748 perfect (7.89%), mean .2645 | baseline.md |
| S0.5 | census on fresh binary | setup | done | — | — | census0.json | superseded by varcensus/structscore (F3) |
| F1 | fix(cli) bad option value | found. | merged 98a559a0 | — | #647 | all green, review 1 round | — |
| F2 | bytehonest (default on) | found. | merged 32b25184 | — | #650 | all green, review 1 round; typesweep improved>>worse | — |
| F3 | measurement tooling | found. | review r4 → land (stage2b wf) | /home/mahaloz/kwt/typetools feat/typetools | #653 | cheap gates green; rust-test structurally untouched | re-review, land |
| F4 | elfmain (default on) | found. | merged 37e0d31e | — | #649 | all green, review 3 rounds | — |
| F5 | impliedrefs/termdup | found. | merged f5cea6bc | — | #648 | all green, review 1 round | — |
| A1 | libctypes | A | review r4 → land (stage2c wf) | /home/mahaloz/kwt/libctypes feat/libctypes | #651 | gates green locally; typesweep tar+e2fsprogs +9 perfect / 0 worse | rebase, delta review, land; then A1-flip (default on) |
| A2 | ptrfromuse | A | queued (stage2c wf, after A1 lands) | /home/mahaloz/kwt/ptrfromuse | | | |
| A3 | protoorder [PROPOSAL] | A | pending | | | | after A2 |
| B1 | phantom-locals root cause → argclobber | B | merged (PR #652) | — | #652 | review 3 rounds; sound 4-site rule (fmt main 3/2/1→2/2/2, phantom rdx locals gone); opt-in | — |
| B2 | signedness [PROPOSAL] | B | running (stage2b wf) | /home/mahaloz/kwt/signedness feat/signedness | | | draft PR → user go/no-go |
| B3 | boolbyte | B | pending | | | | |
| B4 | hideshadow/indirectonly/foldcallret/charbyte/mulblob | B | foldcallret running (stage2b wf) | /home/mahaloz/kwt/foldcallret feat/foldcallret | | | others after |
| C2 | structsynth [PROPOSAL] | C | running (stage2b wf) | /home/mahaloz/kwt/structsynth feat/structsynth | | | draft PR → user go/no-go |
| C3-C9 | struct lane follow-ons | C | pending | | | | after C2 go |

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
