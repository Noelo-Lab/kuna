# The function-entry constant escape fires on numbers

## What is wrong

`ActionConstantPtr::isPointer` refuses to look a constant up in the symbol table
when its bit pattern "looks like a single bit or mask" (`bit_transitions < 3`).
That guard is the only thing standing between integer flags, masks and round
sizes and the global scope. kuna's `inferfuncentry` (GH-6930) lifts it whenever
the constant resolves to an exact function entry, because a function placed at a
power-of-two image base is a single-bit value but is still a function.

The lift applied to **every** reader of such a constant. In a position-independent
executable, round buffer sizes land on function entries routinely: `0x2000`,
`0x1000`, `0x100000`. coreutils `tail` at `-O0` is the case that shows the whole
chain:

```c
/* tail.c, dump_remainder */
size_t n = MIN (n_remaining, BUFSIZ);
```

`BUFSIZ` is `0x2000`, and `0x2000` is that image's `_DT_INIT`, so the bound
printed as a code address and dragged the `uintmax_t` counter it bounded with it:

```
long sub_4649(bool a0,char *a1,unsigned int a2,void *a3)
  void *v9;
  v9 = a3;
  if (_DT_INIT < v9)
    v1 = _DT_INIT;
  ...
  v9 = &v9[-v2];
```

`a3` is `uintmax_t n_bytes` and `v9` is `uintmax_t n_remaining`. `protoorder`
then carries `void *` into every caller: `tail_forever`'s `uintmax_t
bytes_to_read` is a `void *` for the same reason.

## Where the evidence actually is

The escape exists for a code address written where a pointer belongs. GH-6930's
own reproducer is a call argument (`bufferevent_setcb(..., evhttp_write_cb,
...)`), and the other honest shapes are a STORE into a function-pointer slot, an
identity comparison with one, and an addition onto a value already typed as a
pointer. An ordering comparison, a copy, a merge, a PIECE or arithmetic are uses
of the constant as a *number*; the entry it lands on is a coincidence of layout.

The reader alone is not enough. An ordering comparison is never a code address,
but the other half of a `MIN` is a plain `COPY` — and a function-pointer table is
plain COPYs too. coreutils `od -O2` selects between `sub_3f20`, `sub_3ff0`,
`sub_40a0` and `sub_4150` that way, and `0x3ff0` is the one of the four with fewer
than three bit transitions, so refusing the escape on COPY alone costs that table
its name and leaves `v4 = (void *)0x3ff0;` among three named siblings.

What separates them is the rest of the function: `dump_remainder` orders `0x2000`
against `n_remaining`, and nothing in `od`'s dispatch ever orders `0x3ff0`. So the
rule is read twice — `p5_types/kuna_inferfuncentry.rs`:

* `reads_as_integer(opcode)` — an ordering comparison, a multiply, a divide, a
  remainder or a shift reads its constant as a number.
* `entry_escape_applies(opcode, value_reads_as_integer)` — the escape is available
  when this reader is not one of those and the same numeric value is not read as
  one anywhere else in the function.

`ActionConstantPtr::apply` already snapshots every constant Varnode once per pass;
the set of numerically-read values is built from that same snapshot, so the extra
cost is one `lone_descend` per constant. With the escape declined the
`bit_transitions` guard does its normal job and the constant stays a literal,
exactly as upstream leaves it.

## What this is not

The two rows that moved the wrong way between the round-B and round-C campaign
measurements were attributed to `ptrfromuse void`. They are not:

* A 444-slice ablation of `--option ptrfromuse off` on `2e28ece4a` moves 204
  functions OFF perfect and **0** onto it, and improves **0** functions: the
  option has no measurable false positive on that corpus.
* `coreutils::O0::tail tail_forever` scores 0.75 in both arms of that ablation.
  Its `bytes_to_read` is `void *` with `ptrfromuse` off, and stays `void *` with
  `protoorder`, `structsynth`, `charbyte`, `boolbyte`, `foldcallretphi`,
  `codescalar`, `formatstring` or `libctypes` off. Only `inferfuncentry off`
  changes it, and the round-B binary already printed `void *` for the callee: what
  round C added was `protoorder` carrying that wrong callee type to the caller.
* `grep::O2-noinline::grep buf_has_encoding_errors` scores 0.2 in both arms too.
  Its `buf` is `unsigned long *`, not `void *`: `skip_easy_bytes` reads a `uword`
  at a time, so that pointee is what the callee's body says, and `protoorder`
  carries it. That is a pointee-granularity question, not this one.
