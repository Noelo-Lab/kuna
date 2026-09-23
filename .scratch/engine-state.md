# castcensus lane `engine` — state (DONE)
Report: .scratch/engine.md (committed+pushed on campaign/castcensus).
Branch holds env-gated scratch instrumentation (KUNA_CASTCENSUS=1) in
p9_emit/cast.rs (`pub mod census`), coreaction_casts.rs (:1432/:1542 bumps, arm tag at :313,
demotion bumps), printc.rs (6 print-site bumps), kuna-cli/src/main.rs (census::dump()),
lib.rs (`mod p9_emit` -> `pub mod p9_emit`). NOT FOR MERGE.
Artifacts: .scratch/{castshapes.py,buckets.py,census_*.txt,abl/,fresh/,typesweep-ptrfromuse-byte/}
Headline: PTRARITH 48.6% of O0 casts; 74% of its inward casts are a `void *` base + const
offset; `--option ptrfromuse byte` = -14.5% casts O0 / -9.9% O2 at zero type_match cost
(typesweep 241->241 perfect, +1.10 aggregate, 0 worsened).
