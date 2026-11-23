# Go mixed-register result evidence

A Go AMD64 ABIInternal declaration returning `(int64, float64)` has two
independent result registers. The integer reaches RAX and the float reaches the
low eight bytes of XMM0. The current general 16-byte return class instead
joins RAX and RBX. Explicitly declaring a 16-byte struct on that model makes
its second field read the denominator's bits in RBX. The real float operation
is eliminated because the model has no use for it.

The pinned stripped Go 1.27.1 `ratio` illustrates the failure. Both return
paths set RAX and XMM0, including the path returning two zero values. The
caller at `0x4a6b51` reads XMM0 and RAX separately. Current Kuna emits only
`int64`. The same behavior is reproducible on fresh named AMD64 Go 1.22.10
and 1.27.2 fixtures. Their complete DWARF declares both result formals and
uses present, empty result location expressions. Both fixture callers
consume the two lanes independently before overwriting either one. [Pinned result assertions](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_go_compare.py#L118-L130)

On both authored builds, the explicit 16-byte stock result passes all 625
integer-value checks and fails 599 float-bit checks. A diagnostic compiler
spec uses RAX and XMM0 instead for that one size class. Both float failure
counts drop to zero and one `CALL` supplies both ordered eight-byte
projections. The observers share the same binding for the emitted all-zero
carrier constructor. The diagnostic spec would alter unrelated integer
pairs if deployed globally. It establishes the storage requirement for the
named mixed declaration.

`grade` uses the separate string header in RAX/RBX and a float in XMM0.
`bounds` uses XMM0 and XMM1. Their consumers prove three distinct cases.
The named two-float pair has a separate proposal and candidate. The stripped
pins need their own result acquisition and source-order evidence. Those
acquisition steps remain open when the named mixed case is complete.
