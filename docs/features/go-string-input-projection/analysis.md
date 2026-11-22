# Go input string projection

Kuna verifies a named AMD64 Go string parameter and recovers its pointer
and length as separate inputs. The [stock reproduction](proposal.md#reproduce)
shows those two parameters and a guarded unsafe byte load. The pinned
Go `main.count` predicate expects an indexed string read. In the older
compiler's output, the pointer captures an iteration index before the
index is incremented. Any replacement has to use that same index value.
[Pinned Go indexing predicate](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_go_decompiler.py#L255-L256)

The first case is a function that returns an indexed byte when the index
fits and returns a fallback byte otherwise. On Go 1.22.10 and 1.27.2,
the load follows the passing edge of an unsigned comparison against the
same input length. The load uses an address derived from the input data
pointer and the unchanged comparison index. A Go string parameter, its
byte length and the same indexed read express those operations directly.

| Control | Decisive evidence |
| --- | --- |
| Explicit pointer and count with the same names | The emitted signature and byte read can be identical. The selected declaration lacks the verified nominal string formal. |
| Empty string with observable backing | Direct pointer observations distinguish two nonnil zero-length backings. An unguarded load can also observe those bytes. |
| Whole string forwarded | Both final call operands come from the same verified entry string. |
| Shorter prefix forwarded | The final pointer matches the entry string and the length is a different value. The original bound still guards the call. |
| Different backing forwarded | The final data operand names a mapped literal. Reusing the incoming string would select a different byte. |

P9 owns the choice of the source declaration and expressions. It needs
P1's nominal input proof and the effective function and call-site
declarations. Host-compiled Go observers compared the proposed byte read
and representative calls with the stock output on both compiler versions.
The [proposal](proposal.md) defines the first supported cases.

The open draft #1474 proposed a separate `gostringargs` option for the same
string-input contract. Its repro, `func Length(s string) uint64`, now prints
`func Length(s_data *byte, s_len int64) uint64` with `return uint64(s_len)`:
`godwarfsig` already names the two words, and this proposal takes the next step
as a refinement of `godwarfsig` rather than a new option.
