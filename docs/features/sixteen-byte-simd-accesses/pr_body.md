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

- exact dataset acceptance `a-9d336b0f3009`: PASS; 0 malformed and 7 corrected
  casts in otherwise byte-identical 630-line output
- focused C declarator tests: 4/4 implementation-head tests PASS; exact corpus
  assertions added for hosted final-head CI; strict C11 shape check PASS
- datatests 675/675; stages 824/824; CLI 158/158; spec lenient/strict and
  catalog checks PASS
- corpus differential: 253 binaries / 2,949 functions; 247 outputs identical,
  six outputs contain only the enumerated declarator corrections
- complete release workspace: local run stopped at the disk-safety threshold;
  hosted full CI remains required before merge
