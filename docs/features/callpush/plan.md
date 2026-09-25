# callpush — plan

1. Census every pointer cast on an `&` operand in the castbench main arm by what the object is
   (`addrcensus3.py`); compare the sites with IDA's output at the same addresses. Result: the
   `(unsigned long *)&v` residue is 79% a call's return-address push through an alloca-moved
   stack pointer, not a mistyped object (`analysis.md`).
2. `p4_calls/kuna_callpush.rs`: `RuleCallPush` on STORE, registered in `oppool2` after
   `storevarnode` (group `stackvars`), gated by the live arch flag `drop_call_push`
   (`option callpush on|off`), plumbed through `Architecture`, the ArchSeam (`ArchContext`),
   `options.rs`, the console's live-value table and `phases.toml`.
3. Tests: the fixture `callpush_x86_64.c` (four builds), the two-pass stage test
   `tests/stages/kuna-callpush.xml`, and the CLI test
   `a_calls_own_return_address_push_is_part_of_the_call`.
4. Measure: castbench full both directions, the 444-slice typesweep (on vs off, same build),
   whole-corpus hunks (`hunks2.py`, `sidefx.py`), interleaved min-of-15 speed (`speed.py`).
5. Default: on, if the flip procedure passes (see `record.json`).
