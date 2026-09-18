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
| A1 | libctypes → default on → glibc layouts | A | merged f02365cc, 12c31465, eb87d279 | — | #651 #658 #660 | typesweep 889→959 perfect; glibc layouts land real FILE/stat/tm/passwd/group/option/dirent fields | A1c stdio streams (follow-up) |
| A2 | ptrfromuse (off|byte|void, default off) | A | merged 83830e86 | — | #659 | byte: 959→964 perfect, 15 improved/0 worse; void: +100 improved/0 worse; flip withdrawn (subscript-of-global ambiguity) | — |
| A3 | protoorder (types-only redesign, default types) | A | review → land (resume-a wf) | /home/mahaloz/kwt/protoorder | #669 | r5: 959→1089 perfect (+130), improved 1093 / worse 42; 0 call-arity changes over 22 binaries (71,672 x86-64 + 33,145 ARM args identical); speed ≤+1.1%; `lock` stays opt-in (fabricates params) | land |
| B1 | phantom-locals root cause → argclobber | B | merged (PR #652) | — | #652 | review 3 rounds; sound 4-site rule (fmt main 3/2/1→2/2/2, phantom rdx locals gone); opt-in | — |
| B2 | signedness (default auto) | B | merged fe3abc23 | — | #656 | typesweep byte-identical; 377 decl flips / 443 casts dropped / 0 other hunks; +0.6% | — |
| B3 | boolbyte | B | merged 94af9a1b (default OFF) | — | #664 | +28 perfect / 0 worse; every mechanical default-on criterion passes; reviewer held it off for 7 vN-renumbering merge-shape moves | default-on flip PR (phase B) |
| B4 | foldcallretphi / hideshadow / indirectonly / charbyte / mulblob | B | #654 merged; #663 merged (off); hideshadow merged ba79503e (#670); mulblob #671 in review (stage2f); charbyte later | /home/mahaloz/kwt/{mulblob,charbyte} | #654 #663 #670 #671 | | |
| C2 | structsynth | C | merged 2272403d (#655), default OFF | — | #655 | speed +0.08% worst; accuracy −0.151% perfect on 300 slices (factor/shred GMP rows) → opt-in per the rules; precision 0.894; reviewer found a PRE-EXISTING orphan-goto invalid-C bug on main (sort O2 0x3ec0) → B6 | C4 subsumption (stage2g) |
| B5 | GH-657 foldcallret barrier fix | B | merged 39fdc804 | — | #662 | strict fix, no option; 37-binary sweep 126 changed / 0 FLAG; typesweep unmoved | — |
| A5 | formatstring at load time (default static) | A | delta review → land (resume-a wf) | /home/mahaloz/kwt/formatstring | #673 | 959→993 perfect, improved 166 / worse 2; speed −0.4..−4.6% | land |
| B6 | orphan goto (invalid C) | B | merged 70511284 | — | #674 | strict fix | — |
| C4 | structsynth subsumption dedup | C | delta review → land (resume-a wf) | /home/mahaloz/kwt/structdedup | #675 | 514→427 struct names; precision 0.8945 with a 4x size bound | land |
| A1c | stdio streams typed FILE * | A | merged cacce783 | — | #672 | | — |
| B4e | mulblob (128-bit MUL operand blobs) | B | merged c9f5e17d (default ON) | — | #671 | | — |
| B4d | charbyte | B | implementing (resume-a wf) | /home/mahaloz/kwt/charbyte | | | census-gated |
| C7 | structsynth homogeneous-layout decline (+ default-on evaluation) | C | implementing (resume-a wf) | /home/mahaloz/kwt/structarrays | | targets the factor/shred array-as-struct worse rows that hold structsynth off | |
| C3-C9 | struct lane follow-ons | C | structdefs (C5) merged 75a7f6b0 (#661) | — | #661 | | C4 subsumption / C6 all / C7 structarray after #655 lands |

2026-09-18: the first account hit its weekly limit mid-run (agents for #669/#673/#675 died after committing+pushing; #671/#672/#674 had already merged). Resumed on a second account; nothing was lost.

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
