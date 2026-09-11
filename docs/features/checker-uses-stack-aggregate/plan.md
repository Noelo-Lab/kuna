# Plan

1. Instrument the P9 declaration walk to distinguish the whole-storage-sibling
   hypothesis from the unrelated aggregate-symbol hypothesis.
2. Correct the existing scalar-piece suppression predicate at its narrowest
   false-positive condition, without changing type recovery or statement output.
3. Add a helper-level unit test, a self-contained stage test, and an in-repo
   CLI minimizer that preserves the acceptance probe's anchored backreference.
4. Compare exact pre-fix and fixed builds across the complete XML corpus and
   every decompilable function in the witness binary; classify every changed
   function before committing.
