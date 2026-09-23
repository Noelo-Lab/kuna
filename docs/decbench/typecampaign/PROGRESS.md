# Type-recovery campaign — progress ledger

Plan: `~/.claude/plans/today-we-are-taking-quirky-rabin.md` (session 5b38c0e9). Resumable: every item's
state lives in this table; agents also leave `<worktree>/.scratch/state.md`.

Started 2026-09-16 on main `809712e9`. Stage 1 landed 2026-09-16/17 (#647 #648 #649 #650 #653). Lane A landed 2026-09-17 (#651 #658 #659); B: #652 #654. Campaign baseline 848/10748 perfect → 964 after #659 (with ptrfromuse byte; 959 default) → 986 / mean .2895 on main d96e3408 (Stage-3) → 1,349 / mean .3403 on main d8b9c0b1 (round C) → 1,349 / mean .3405 on main 4c7704e0 (round D) → 1,353 / mean .3415 on main 2da61985 (round E) → **1,522 / mean .3553 on main 810b7dc8 (round F, `results.md`)**. Decisions: three equal lanes; Opus subagents; self-merge after
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
| A7 | charptr (`char *` from use) + loader-table string fix | A | merged 68c59945; fix merged ce008ce2 (default off) | — | #695 #704 | default output unmoved; after #704 the option arm is 1,353 → 1,353, 2 up / 4 down (−0.57) | — |
| A8 | libcstructs — 7 more named libc aggregates; width gate + fit rule follow-up | A | merged eaa19ebb; follow-up merged 2da61985 | — | #692 #706 | the whole round-E gain: 1,349 → 1,353, +10.70 aggregate, 71 improved / 1 worse (#692); #706 −0.36, 0 / 1 (grep `kwsprep`); `ptr_struct` 422 → 474 | — |
| S5 | final re-measure, round D | setup | done 2026-09-20 | /home/mahaloz/kwt/_final-d | results PR | 1,349 perfect / .3405; 13 improved / 0 worse; layout precision repaired .5520 → .8709; decbench#94 counterfactual: the restrict artifact is 98.2% register-only | `results.md` §Round D |
| C11 | structsynth `nest` — a record pointer loaded from a field (+ structscore header fix) | C | merged a26b99db (nest opt-in, default stays `param`) | — | #705 | typesweep 1,349 = 1,349; +2 nested fields on 10 builds; `--jobs 8` tar O2 +63% → not flipped; half of "nesting F1 = 0" was the instrument | request-relative `--jobs` recipe, then re-evaluate |
| A7-on | charptr default on | A | dropped (no PR) | branch feat/charptr-on | — | flip fails `make test-cli` 213/219 and the typesweep (−0.57, 2 up / 4 down); found the recursion-park lever: +36 (`self`) / +46 (`all`) perfect, +163 / +188 `ptr_char` TP | recursion park as its own option |
| S6a | O2 census (why 3.09% perfect at -O2) | setup | done (investigation) | campaign/o2census | — | the O2 gap is the ceiling: 85.8% of O2 functions hold an unscoreable GT variable; P-tier conversion 43.8% (O0 43.5%); inlined-callee GT upstream fix: O2 74 → 153 | `final-e/o2census/` |
| S6 | final re-measure, round E | setup | done 2026-09-22 | /home/mahaloz/kwt/_final-e | results PR | 1,353 perfect / .3415; 70 improved / 1 worse; attribution by bisect: all #692; TRex 4.129 / 1.915; layout recall .0929 → .0859 (named `obstack *` leaves the join) | `results.md` §Round E |
| C12 | structsynth `locals` — records for call-returned pointers | C | merged 40184673 (default `locals`) | — | #709 | typesweep exactly neutral on the final tree (`param` → `locals` 1,522 = 1,522, 0 up / 0 down); +8 declarations over 2,918 functions; TRex O0 and decbench#93 crediting both rise | — |
| A9 | protoorder `cycles` — recursive components state their recovered types | A | merged 3a42d864 (default `cycles`) | — | #712 | `types` → `cycles` on the final tree: 1,472 → 1,522 perfect, 171 up / 1 worse (`ginstall::install_file_in_file`); argument match 49.2% → 50.0% | — |
| B11 | `slotptr` — a framelayout filler slot typed from the pointer stored into it | B | merged 656e6c70 (default on) | — | #710 | the round's biggest lever: off → on 1,399 → 1,522 perfect, 425 up / 5 down; `ptr_char` +653 stack TP; C output byte-identical | — |
| A10 | `passthrough` — a register handed untouched to a callee becomes a parameter | A | merged 810b7dc8 (default **off**) | — | #708 | option on: 1,522 → 1,546 perfect, 128 up / 6 down, and the only round-F lever that moves O2 (74 → 81) and O2-noinline (383 → 400); 454/506 gained parameters confirmed by DWARF, 0 contradicted | flip needs a check on the 52 DWARF does not confirm |
| C13 | `calleevote` — a parameter takes the type every caller passes | C | open, review CHANGES (round 4) | /home/mahaloz/kwt/calleevote (feat/calleevote) | #711 | on `3a42d864`: 1,522 → 1,575 perfect, 264 up / 3 down; credited 1,771 → 2,128; kmod -O2-noinline +11.5% | address review, land |
| S7 | final re-measure, round F | setup | done 2026-09-23 | /home/mahaloz/kwt/_final-f | results PR | 1,522 perfect / .3553; 581 improved / 6 worse; `ptr_char` 28.9% → 34.6%; stack storage 53.4% → 57.6%; TRex O0 4.129 → 4.420; instruments byte-identical E → F | `results.md` §Round F |

2026-09-18: the first account hit its weekly limit mid-run (agents for #669/#673/#675 died after committing+pushing; #671/#672/#674 had already merged). Resumed on a second account; nothing was lost.

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
