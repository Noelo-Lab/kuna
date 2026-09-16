# Type-recovery campaign — progress ledger

Plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md` (session 5b38c0e9). Resumable: every item's
state lives in this table; agents also leave `<worktree>/.scratch/state.md`.

Started 2026-09-16 on main `809712e9`. Decisions: three equal lanes; Opus subagents; self-merge after
reviewer + green gates; `[PROPOSAL]` items wait for the user; port TRex/RecStruct from source with attribution.

| id | item | lane | state | worktree / branch | PR | last gate result | next step |
|---|---|---|---|---|---|---|---|
| S0.1 | make binaries (main) | setup | done 18:54 | — | — | rc=0 | baseline copy at /home/mahaloz/kwt/_baseline/ |
| S0.2 | reclaim orphan worktrees | setup | done (116 dirs, ~43G) | — | — | — | .claude/worktrees left alone |
| S0.3 | campaign state dir | setup | done | — | — | — | — |
| S0.4 | canonical typesweep baseline | setup | running (bg) | /home/mahaloz/kwt/_baseline | — | log typesweep.log | out docs/decbench/typecampaign/baseline/ |
| S0.5 | census on fresh binary | setup | running (bg) | — | — | census0.log | /home/mahaloz/kwt/_baseline/census0.json |
| F1 | fix(cli) bad option value | found. | running (wf_4fdfa3a8-dc9) | /home/mahaloz/kwt/optvalue feat/optvalue | | | impl→review→land |
| F2 | bytehonest | found. | running (wf_4fdfa3a8-dc9) | /home/mahaloz/kwt/bytehonest feat/bytehonest | | | impl→review→land |
| F3 | measurement tooling | found. | running (wf_4fdfa3a8-dc9) | /home/mahaloz/kwt/typetools feat/typetools | | | impl→review→land |
| F4 | elfmain | found. | running (wf_4fdfa3a8-dc9) | /home/mahaloz/kwt/elfmain feat/elfmain | | | impl→review→land |
| F5 | impliedrefs/termdup | found. | running (wf_4fdfa3a8-dc9) | /home/mahaloz/kwt/impliedrefs feat/impliedrefs | | | impl→review→land |
| A1 | libctypes | A | running (stage2a wf) | /home/mahaloz/kwt/libctypes feat/libctypes | | | impl→review→land |
| A2 | ptrfromuse | A | pending | | | | after A1 |
| A3 | protoorder [PROPOSAL] | A | pending | | | | after A2 |
| B1 | phantom-locals root cause | B | running (stage2a wf) | /home/mahaloz/kwt/phantomargs feat/phantomargs | | | investigate→(PR) |
| B2 | signedness [PROPOSAL] | B | pending | | | | |
| B3 | boolbyte | B | pending | | | | |
| B4 | hideshadow/indirectonly/foldcallret/charbyte/mulblob | B | pending | | | | |
| C2 | structsynth [PROPOSAL] | C | pending | | | | after F3 |
| C3-C9 | struct lane follow-ons | C | pending | | | | after C2 go |

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
