# Plan

1. Keep the existing `call_output_foldable` predicate for direct call outputs.
2. Before multiplier analysis duplicates a multi-use implied expression, walk
   its printable operand tree and materialize the root if it reaches a foldable
   call.
3. Cover both the cross-block witness and a same-block two-sink minimizer.
4. Sweep the in-repo and RE corpora, classifying every changed function and
   rejecting any hunk that adds or removes a distinct callee.
