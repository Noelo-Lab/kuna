# Type-recovery campaign — progress ledger

Plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md` (session 5b38c0e9). Resumable: every item's
state lives in this table; agents also leave `<worktree>/.scratch/state.md`.

Started 2026-09-16 on main `809712e9`. Stage 1 landed 2026-09-16/17 (#647 #648 #649 #650 #653). Lane A landed 2026-09-17 (#651 #658 #659); B: #652 #654. Campaign baseline 848/10748 perfect → 964 after #659 (with ptrfromuse byte; 959 default). Decisions: three equal lanes; Opus subagents; self-merge after
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
| F3 | measurement tooling (mine --metric, typescore, varcensus, structscore) | found. | merged ff56f941 | — | #653 | 4 review rounds | — |
| F4 | elfmain (default on) | found. | merged 37e0d31e | — | #649 | all green, review 3 rounds | — |
| F5 | impliedrefs/termdup | found. | merged f5cea6bc | — | #648 | all green, review 1 round | — |
| A1 | libctypes → default on | A | merged f02365cc + 12c31465 | — | #651, #658 | typesweep 444 slices: perfect 889→959 (+70), 0 off-perfect, 1 worse; PTRSUB( 0; off byte-identical on 16 binaries | A1b glibc layout + A1c stdio symbols (stage2d) |
| A2 | ptrfromuse (off|byte|void, default off) | A | merged 83830e86 | — | #659 | byte: 959→964 perfect, 15 improved/0 worse; void: +100 improved/0 worse; flip withdrawn (subscript-of-global ambiguity) | — |
| A3 | protoorder [PROPOSAL] | A | running (stage2d wf) | /home/mahaloz/kwt/protoorder feat/protoorder | | | draft PR → user go/no-go |
| B1 | phantom-locals root cause → argclobber | B | merged (PR #652) | — | #652 | review 3 rounds; sound 4-site rule (fmt main 3/2/1→2/2/2, phantom rdx locals gone); opt-in | — |
| B2 | signedness [PROPOSAL] | B | draft PR open, review loop (stage2b wf) | /home/mahaloz/kwt/signedness feat/signedness | #656 | | user go/no-go |
| B3 | boolbyte | B | pending | | | | |
| B4 | foldcallretphi / hideshadow / indirectonly / charbyte / mulblob | B | foldcallretphi merged fc7c7a22 (#654); hideshadow running (stage2d wf) | /home/mahaloz/kwt/hideshadow | #654 | | indirectonly, charbyte next |
| C2 | structsynth [PROPOSAL] | C | draft PR open, review loop (stage2b wf) | /home/mahaloz/kwt/structsynth feat/structsynth | #655 | | user go/no-go |
| C3-C9 | struct lane follow-ons | C | pending | | | | after C2 go |

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
