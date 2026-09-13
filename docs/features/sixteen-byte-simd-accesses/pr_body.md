## Summary

- fix C declarator precedence for pointers nested with arrays/postfixes
- correct seven invalid SIMD casts from `char *[16]` to `char (*)[16]`
- replace the unit that pinned the known inversion with exact mirror and nested
  declarator controls

## Why no option

This is an invalid-C correctness defect: the old spelling changes a pointer to
an array into an array of pointers.  Keeping the incorrect type behind an option
would not provide a useful analysis tradeoff, so the fix is unconditional and
allocates no phase or XML identifier.

## Validation

- exact dataset acceptance `a-9d336b0f3009`: pending final-head rerun
- focused C declarator tests: pending final-head rerun
- strict C syntax/type validation: pending final-head rerun
- datatests / stages / CLI / full workspace: pending final-head rerun
- corpus declarator differential: pending final-head classification
