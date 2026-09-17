# hideshadow — plan

1. Port `ActionHideShadow::apply` (`coreaction.cc:4976-4995`) into
   `p6_variables/kuna_hideshadow.rs` as `hide_shadow_copies`, driving the
   already-ported `Merge::hide_shadows` over the `with_covermerge` bridge and
   deduping HighVariables through the bank mark surface. The action stays where
   it is in `universal_sched`, so `list_action_decompile_oracle.txt` does not
   move.
2. Gate it on `hideshadow on|off` (`Architecture::hide_shadow`, the ArchSeam
   copy in `substrate/context.rs`). OFF returns before the walk, which is
   byte-for-byte what the stub did.
3. Make the Cover read faithful: `bank_update_cover` before the containment
   test, standing in for the C++ `getCover()` lazy rebuild.
4. Two-pass stage test over the full binary -> decompile path, plus a guard
   function whose two copies are on exclusive branches and must survive ON.
5. Whole-corpus `decompile-all` A/B over the campaign's 444 slices, every hunk
   classified.
6. `varcensus` on the changed binaries; `timeit` interleaved; datatest ablation
   with the default flipped; then decide the shipped default.
