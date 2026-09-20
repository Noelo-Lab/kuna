# Type-recovery campaign — progress ledger

Plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md` (session 5b38c0e9). Resumable: every item's
state lives in this table; agents also leave `<worktree>/.scratch/state.md`.

Started 2026-09-16 on main `809712e9`. Stage 1 landed 2026-09-16/17 (#647 #648 #649 #650 #653). Lane A landed 2026-09-17 (#651 #658 #659); B: #652 #654. Campaign baseline 848/10748 perfect → 964 after #659 (with ptrfromuse byte; 959 default) → 986 / mean .2895 on main d96e3408 (Stage-3) → 1,349 / mean .3403 on main d8b9c0b1 (round C) → **1,349 / mean .3405 on main 4c7704e0 (round D, `results.md`)**. Decisions: three equal lanes; Opus subagents; self-merge after
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
| A3 | protoorder (types-only redesign, default types) | A | merged 3433e05d | — | #669 | 1026→1147 perfect on its base; 0 call-arity changes (60-binary corpus); speed ≤+4.1% | — |
| B1 | phantom-locals root cause → argclobber | B | merged (PR #652) | — | #652 | review 3 rounds; sound 4-site rule (fmt main 3/2/1→2/2/2, phantom rdx locals gone); opt-in | — |
| B2 | signedness (default auto) | B | merged fe3abc23 | — | #656 | typesweep byte-identical; 377 decl flips / 443 casts dropped / 0 other hunks; +0.6% | — |
| B3 | boolbyte | B | merged 94af9a1b; default ON merged f1ec42a7 | — | #664 #680 | 959→987 perfect (+28), 430 improved / 0 worse | — |
| B4 | foldcallretphi / hideshadow / indirectonly / charbyte / mulblob | B | #654 merged; #663 merged (off); hideshadow merged ba79503e (#670); mulblob #671 in review (stage2f); charbyte later | /home/mahaloz/kwt/{mulblob,charbyte} | #654 #663 #670 #671 | | |
| C2 | structsynth | C | merged 2272403d (#655); default `param` merged 6e4f6fa5 (#682, user decision) | — | #655 | speed +0.08% worst; accuracy −0.151% perfect on 300 slices (factor/shred GMP rows) → opt-in per the rules; precision 0.894; reviewer found a PRE-EXISTING orphan-goto invalid-C bug on main (sort O2 0x3ec0) → B6 | C4 subsumption (stage2g) |
| B5 | GH-657 foldcallret barrier fix | B | merged 39fdc804 | — | #662 | strict fix, no option; 37-binary sweep 126 changed / 0 FLAG; typesweep unmoved | — |
| A5 | formatstring at load time (default static) | A | merged e76fd044 | — | #673 | 988→1026 perfect (+38), 176 better / 0 worse; speed −4.6..+0.8% | — |
| B6 | orphan goto (invalid C) | B | merged 70511284 | — | #674 | strict fix | — |
| C4 | structsynth subsumption dedup | C | merged bcfe6289 | — | #675 | struct names 515→471 over 12 builds (188→161 on the goal-3 eight); held-out precision .8902→.8908 | — |
| A1c | stdio streams typed FILE * | A | merged cacce783 | — | #672 | | — |
| B4e | mulblob (128-bit MUL operand blobs) | B | merged c9f5e17d (default ON) | — | #671 | | — |
| B4d | charbyte | B | merged d96e3408 (default ON) | — | #683 | 986=986 (metric-neutral); 19 vars unsigned char * → char * | — |
| C7 | structsynth homogeneous-layout decline | C | closed, not merged | /home/mahaloz/kwt/structarrays | #679 | net −91 correct struct pointers under the struct-crediting rule (decbench#93) | dropped |
| C3-C9 | struct lane follow-ons | C | structdefs (C5) merged 75a7f6b0 (#661) | — | #661 | | C4 subsumption / C6 all / C7 structarray after #655 lands |
| B7 | expandload (narrow load through a wider-typed pointer) | B | merged e1139df9 | — | #677 | strict fix; typesweep 959=959 | — |
| B8 | truncarg (keep a narrowed argument's truncation) | B | merged d03560b2 | — | #678 | strict fix; typesweep unmoved | — |
| B4a-on | foldcallretphi default on | B | merged d6c5862f | — | #681 | 986=986; −14 declarations; speed ≤+1.04% | — |
| A1-on | libctypes glibc default | A | stays opt-in, no PR | /home/mahaloz/kwt/libctypes-glibc-on | — | 959=959 +0.55 (9/3); the 5 #660 defect classes reproduce | — |
| A2-on | ptrfromuse void default | A | merged d8b9c0b1 | — | #690 | on the protoorder base: 1105→1309 (+204), 85 improved / 0 worse | — |
| B1-on | argclobber default (argclobberproto) | B | merged 72438114 | — | #689 | 19 functions over 770 ELFs onto the callee's true arity; typesweep 1,349=1,349; fmt::main 1/2/3 → 2/2/2 with no phantom rdx; speed +0.38% | — |
| S3 | final re-measure (Stage 3) | setup | done 2026-09-19 | /home/mahaloz/kwt/_final | results PR | 848→986 perfect, .2645→.2895; 1,449 improved / 8 worse; per-class + rivals + varcensus + structscore + speed | `results.md` |
| B9 | foldcallret short-circuit fix (GH-684) | B | merged a3bf6a15 | — | #687 | strict fix; 8/25,556 functions, `variables[]` identical | — |
| B10 | aliasoverlap (a load past an overlapping store) | B | merged 3831d703 | — | #686 | strict fix; 1/48,235 functions; typesweep rows identical in 444/444 slices | — |
| T1 | --jobs N keeps serial-run struct names | tooling | merged 0304334b | — | #688 | removes the sharded-run structsynth limitation from #682 | — |
| S4 | final re-measure, round C | setup | done 2026-09-20 | /home/mahaloz/kwt/_final-c | results PR | 986→1349 perfect, .2895→.3403; 1,384 improved / 2 worse; 1st of five on perfect% and mean; layout-precision regression attributed to protoorder | `results.md` §Round C |
| C10 | decompile-project takes the callee-first order (layoutvote) | C | merged 3de41f06 | — | #693 | `decompile-all` byte-identical; export agrees with it 2,918/2,918; layout precision .5520 → .8709 — the round-C regression was this join, not protoorder | — |
| A6 | inferfuncentry narrowing (ptrfromuseint) | A | merged 69d252df | — | #694 | 13 functions improved / 0 worse on the 444 slices; `int_u8` +15 TP; 18 of 6,882 functions change | — |
| A7 | charptr (`char *` from use) + loader-table string fix | A | open, review CHANGES | /home/mahaloz/kwt/charptr | #695 | 1,349 → 1,353, 14 improved / 6 down; default off (the flip fails `make test-cli`) | address review, land |
| A8 | libcstructs — 7 more named libc aggregates | A | open, review CHANGES | /home/mahaloz/kwt/libcstructs | #692 | 1,349 → 1,353, mean .3405 → .3419, 91 improved / 1 worse, `ptr_struct` 422 → 496 | address review, land |
| S5 | final re-measure, round D | setup | done 2026-09-20 | /home/mahaloz/kwt/_final-d | results PR | 1,349 perfect / .3405; 13 improved / 0 worse; layout precision repaired .5520 → .8709; decbench#94 counterfactual: the restrict artifact is 98.2% register-only | `results.md` §Round D |

2026-09-18: the first account hit its weekly limit mid-run (agents for #669/#673/#675 died after committing+pushing; #671/#672/#674 had already merged). Resumed on a second account; nothing was lost.

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
