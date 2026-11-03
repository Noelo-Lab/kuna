# Go integer stores

The emitter now converts an integer STORE value to the pointed-to integer type when both have the same 1-, 2-, 4- or 8-byte width and opposite signedness. `*p = a + b` becomes `*p = int64(a + b)` when `p` points to `int64` and the operands are `uint64`. This retains the stored bits and evaluates the pointer and value once. Noninteger values, unknown carriers and mismatched widths retain their existing output.

The public reproduction is in `public-repro.c`; exact parent and candidate output is retained beside it. The registered native test covers 33 complete Go bodies on Go 1.22.5 and 1.27.3, compared with C at O0/O2 using wrapping signed arithmetic: 21384 function/input comparisons, including exact pointer/value callback counts. The unchanged `nested_store` fixture still has an array-pointer rendering error and is explicitly excluded from native compilation.

The 72-image static comparison covers 37908 distinct functions, with Go paired results disabled and enabled. All 2619 changed Go records differ only by integer conversions around complete assignment right-hand sides and corresponding line mappings. C and Rust records are unchanged; no new decompiler or Go parser errors occur. This is structural coverage, not native equivalence for entire runtimes. Complete changed function pairs, capture commands, hashes and the structural review are retained in `breadth/`.

All four repository checks pass, including 10357 workspace tests and all original 675 assertions. The five new stage assertions bring this parent’s stage total to 3742. Catalog metadata is unchanged.

The speed plan was declared before the integrated build or sampling: 64 balanced paired rounds per case, three warmups per arm, CPU and wall median deltas at most 5%. The 95% upper bounds on mean deltas report uncertainty and are not acceptance thresholds. Every sample and CPU telemetry is retained. The maximum measured median delta is 0.965%; the largest reported upper bound is 4.647%.

| Case | CPU median | Wall median | CPU upper mean bound | Wall upper mean bound |
|---|---:|---:|---:|---:|
| c-store | -0.114% | -0.109% | +0.179% | +0.159% |
| go-store | -0.249% | -0.282% | +0.083% | +0.095% |
| go-import122 | +0.965% | +0.768% | +1.021% | +1.824% |
| go-pair-import127 | -0.101% | -0.116% | -0.029% | -0.020% |
| go-runtime | +0.685% | +0.926% | +0.957% | +0.827% |
| go-pair-runtime | +0.798% | +0.746% | +1.022% | +4.647% |
| rust-large | +0.187% | +0.040% | +0.332% | +2.843% |

`record.json` identifies the binaries and parent tree; the parent’s published commit and actual merge commit have identical trees. `manifest.json` hashes every archived evidence file. Sources, both executables and all 154 compiled processor specs were checked before and after timing.
