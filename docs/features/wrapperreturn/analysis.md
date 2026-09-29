# ARM wrapper return evidence

A consumed result was declared void across a non-tail ARM call wrapper. Recovery
now requires a caller's real use, a known producer, and preserved return storage
on every exit. Unused results, explicit void, writes, indirect calls and unanchored
cycles do not establish a return. Retries are limited to demanded functions and
then their affected callers; isolated function and parallel runs stay conservative.

The changed functions are `wrapper.o:wrapper` (forwards the producer result) and
`wrapper.o:consumer` (retains its computed result and passes the recovered input).
The genuine void wrapper remains void. The synthetic CLI regression also covers
three forwarding levels and all refusal cases above.

The seven-file corpus contains 47 functions. All synthetic input source is in
`fixtures/`; `fmtlf_armhf` and its C source are already in the repository's
`kuna-analysis` tests. No third-party binary was added. Replay after `make` using
Python 3, `arm-linux-gnueabi-as`, and the GNU ARM soft/hard-float cross-compilers:

```sh
python3 docs/features/wrapperreturn/corpus.py --output ../tmp/wrapperreturn-corpus
```

`corpus.diff` contains every changed function, with trailing whitespace trimmed.
The replay directory retains the raw outputs. The other functions are byte-identical.
Compiler versions can affect synthetic instruction selection; the Rust regressions
use fixed instruction bytes and construct their ELF metadata directly.

Both runtime settings pass the original 675 assertions without a baseline change.
The new raw stage checks conservative behavior when the frontend cannot supply
the required evidence; the synthetic ELF CLI tests exercise the positive path.
The option stays off by default and is absent from automatic presets. Broader
architecture/corpus and speed evidence is required before preset promotion.

Declared non-void producers are covered separately from inferred producers;
declared void still wins. A p-code regression requires every byte of a split
register return to come from the same call, in the right order, with a finite
walk. The combined ARM VFP check additionally requires caller evidence before
relaxing the call-clobber heuristic; an unused double result stays conservative.
