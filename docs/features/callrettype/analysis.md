# callrettype: analysis

## The gap

The 2026-09-24 cast census counted 1,994 `(char *)<call>` casts on the 4,815
functions kuna and IDA both emit (IDA: 1,172). 682 of them sit on a call to a
function whose own listing, printed earlier by the same `decompile-all`,
already declares the `char *` return:

```c
char * sub_43ee(unsigned long *a0)             /* coreutils tail -O0, pretty_name */
...
v7 = (char *)sub_e8ca(4,(char *)sub_43ee(a0)); /* recheck @0x5a69 */
```

IDA prints the same call as `v9 = sub_43EE((const char **)a1);`: the result
takes the callee's prototype and needs no conversion.

## Why kuna printed it

Upstream's `TypeOpCall::getOutputLocal` answers only for a LOCKED output. A
recovered prototype is never locked (locking it would commit the output trial
and could make a call gain or lose a result), so the call produces an unknown
of its width, and `ActionSetCasts` converts it to whatever the caller's fold
chose for the variable. `protoorder` already carries a callee's recovered
PARAMETER types to its callers in callee-first order; nothing carried the
return.

## What the vote may and may not do

The stated type is offered as the call output's local type only where the call
already has an output in exactly the storage and width the callee returns in,
so no arity moves and a `void` callee states nothing. The census of the first
builds showed where a stated type is wrong for the caller, and each became a
refusal (spec 04, `callrettype`): the protoorder family refusals, sign
contradictions, a declared or differently-recovered pointee, a pointee read at
another width (unknown bytes included), a result class that disagrees with
another call writing the same value, and a load through the pointer that flows
back into it.

## The tie (found while reading the "more casts" functions)

`du -O0` `map_inode_number` (returns `uintmax_t`) printed, with the vote alone:

```c
long * sub_6b45(struct_12 *a0,unsigned long a1)
...
      v1 = (long *)0xffffffffffffffff;
...
    v1 = (unsigned long)sub_10624((unsigned long *)a0->field_0x8,a1);
```

`ino_map_alloc`'s pointer result lives only in RAX on its way to a field. At
type-inference time it is alone; the merge then ties RAX whole-function
(`mark_output_storage_addr_tied`, because the function joins its returned
values there), the tied variable takes the most specific member type (the
pointer), and the function's return type follows. No inference-time family
sees this: the IR the tie is decided on exists only after propagation (asking
the same predicate at inference time answered "no tie" for grep
`bmexec_trans`, which the merge then tied). Two related effects: a later
inference pass can refuse a vote an earlier pass took while the result keeps
the type (tar `sub_2ba80`, left alone: a conversion, not a wrong type), and a
result the function hands back beside
`return -1` makes its return type a pointer (grep `bmexec_trans` returns
`ptrdiff_t`; dash's token reader returned `(long **)(unsigned long)v4`).

The fix is an audit of the finished function (`kuna_callrettype::contradicted`)
and one more decompile of the caller without the contradicted statements. On
the census corpus it removed every new "pointer variable assigned an integer"
line and every new pointer return type beside a returned non-address number.

Its cost decided its shape. The first audit also withdrew votes a later
inference pass refused while the result kept the type (tar `sub_2ba80`, a
`CONCAT71` shift), and audited every statement: fmt -O2 went +76% (gnulib's
`quotearg_buffer_restyled` decompiled twice for no change in its text) and
bash -O2 +55% (218 second decompiles). The audit now takes only statements
that took (the variable carries the stated type), only the tie
contradictions, and only functions up to 1,000 live p-code ops (a
1,992-op function in dpkg-divert still cost 10% of that run at 2,000).

## Residue

- `(char *)<call>` on callees kuna recovers as `void` whose callers read the
  result (a wrapper ending in `call; leave; ret`): a `void` callee states
  nothing; recovering that return is the callee's prototype question.
- `(long)p - (long)q` for a pointer difference: `castarith` keeps pointer
  arithmetic in pointer terms but not a difference of two pointers; with the
  vote more of those operands are pointers.
- A pointer result stored into an integer-typed field prints `(long)call(...)`;
  C requires that conversion.
