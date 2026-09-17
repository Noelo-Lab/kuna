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
| A3 | protoorder [PROPOSAL] | A | draft #669 open, review loop (stage2d wf) | /home/mahaloz/kwt/protoorder | #669 | | user go applies: flip/land after review |
| B1 | phantom-locals root cause → argclobber | B | merged (PR #652) | — | #652 | review 3 rounds; sound 4-site rule (fmt main 3/2/1→2/2/2, phantom rdx locals gone); opt-in | — |
| B2 | signedness [PROPOSAL] | B | **draft #656 awaiting user go/no-go** (5 review rounds) | /home/mahaloz/kwt/signedness | #656 | DWARF-agreement: upstream 93.4% (O2 71.9%) / auto 93.5% / prefer-signed 98.4% (O2 94.8%); all-signed control 92.8% (O2 59.2%) — the walk earns the slot; metric zero by construction | user decision |
| B3 | boolbyte | B | PR #664 open; review/land re-dispatched (stage2f) | /home/mahaloz/kwt/boolbyte | #664 | census: 866 one-byte GT-_Bool vars committed to char(700)/unsigned char(166) over 444 slices | review → land |
| B4 | foldcallretphi / hideshadow / indirectonly / charbyte / mulblob | B | #654 merged; indirectonly merged 0d79b02d (#663, default OFF: ON arm reproduces an upstream Ghidra fabricated-store defect); hideshadow #670 in review (stage2d); charbyte + mulblob running (stage2f) | /home/mahaloz/kwt/{hideshadow,charbyte,mulblob} | #654 #663 #670 | | |
| C2 | structsynth [PROPOSAL] | C | **draft #655 awaiting user go/no-go** (4 review rounds) | /home/mahaloz/kwt/structsynth | #655 | layout precision 0.894 (704/787 fields), 14/14 exports round-trip, 292 params retyped (0 non-pointers), 1 ordinary FP (ls strmode), type_match −0.05% (4 worse: factor GMP limbs), TRex Fig.6 +109 net | user decision; next C4 ledger subsumption |
| B5 | GH-657 foldcallret barrier fix | B | merged 39fdc804 | — | #662 | strict fix, no option; 37-binary sweep 126 changed / 0 FLAG; typesweep unmoved | — |
| A5 | formatstring at load time (default-on candidate, +8 perfects measured) | A | running (stage2f wf) | /home/mahaloz/kwt/formatstring | | | |
| C3-C9 | struct lane follow-ons | C | structdefs (C5) merged 75a7f6b0 (#661) | — | #661 | | C4 subsumption / C6 all / C7 structarray after #655 lands |

States: pending · running · review · gates · merged · blocked · proposal-waiting · dropped.
