# call-pop-pointer-helper plan

1. Add `callpopret`, default off while developing, at P2 flow classification.
2. Decode at most six callee instructions / 64 raw p-code ops, sharing that
   target decode with `calltrampoline`, and symbolically track the stack pointer
   plus pointer-sized loads from it.
3. Match only a run that discards exactly the pushed return address and ends in
   RETURN through the next untouched word; decline all other control transfers,
   stores and unaccountable stack writes.
4. Reuse the existing CALL-to-BRANCH flow rewrite so normal dataflow recovers the
   popped pointer.
5. Pin the positive idiom and its refusal cases with logic tests, a two-arm stage
   test, the original acceptance, and a vendored minimal ELF acceptance.
6. Sweep every changed whole-binary function in both option arms, time the
   vendored witness, and let ablation plus the speed gate decide the default.
