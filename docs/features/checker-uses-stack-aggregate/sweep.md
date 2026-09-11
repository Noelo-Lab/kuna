# Stack-scalar declaration differential sweep

The clean and repaired arms were compiled from the same
`80e7cfbda14eb3c8473552f28c07a2010d6aa202` worktree. The clean binaries were
captured after reverting only the new non-constant predicate; the repaired
binaries were then rebuilt after restoring it. Thus the comparisons contain no
adjacent-commit or fixture skew.

- XML corpus: all 83 upstream datatest files and all 191 stage files.
- Witness binary: all 1,100 functions accepted by `decompile-all` from the
  1,224-entry `phantomgate.exe` inventory, using `funcboundflow off`, eight
  workers, and a 20-second per-function bound.
- Error parity: one function returned an error in both witness arms; there were
  no inventory or error asymmetries.
- Core XML result: byte-identical stdout and stderr, 675/675 assertions.
- Stage XML result: one changed function, the new `stack_aggregate` minimizer;
  its only C delta is the missing declaration. The other 190 stage files are
  byte-identical.
- Witness result: two changed functions, classified below. Both hunks add one
  declaration for an already-used stack local and change no executable
  statement, expression, type use, or control-flow edge.
- Regressions found: zero.

## Changed functions

| Corpus | Binary | Function | Classification |
|---|---|---|---|
| stage | `re-checker-stack-aggregate.xml` | `stack_aggregate` @ `0x100000` | adds `unsigned int local;`; the two partial writes and `sink(&local)` are byte-identical |
| dataset | `phantomgate.exe` | `sub_140001740` @ `0x140001740` | adds `unsigned int v12; // stack - 0x4e8`; all 26 recorded uses are unchanged |
| dataset | `phantomgate.exe` | `sub_1400271c0` @ `0x1400271c0` | adds `unsigned long long v9; // stack - 0x58`; the existing address, partial, and scalar uses are unchanged |

The second witness hit is independent corroboration of the mechanism: it was
not part of the filed acceptance and was discovered only by the whole-binary
differential.
