# hideshadow — whole-corpus `decompile-all` A/B, every hunk classified

444 stripped ELFs: `{O0,O2,O2-noinline}/{coreutils,grep,gzip,diffutils,bzip2,findutils,tar,shadow}/stripped/*`
(the type-campaign slice set). Both arms are the same binary, one invocation each:

    kuna decompile-all <bin> --option hideshadow off
    kuna decompile-all <bin> --option hideshadow on

**434 byte-identical, 10 changed.** No function gained a line, no declaration
moved, no variable was renumbered. Every changed hunk deletes an assignment
that an identical assignment already dominates, with nothing between the two
that writes either side.

| slice | hunk | classification |
|---|---|---|
| O0 coreutils ls / dir / vdir | `(v3 = a2, v4 = a0, v4 = a0, v5 = a1, a1[1])` -> `(v3 = a2, v4 = a0, v5 = a1, a1[1])` | copy removed, value provably identical (adjacent exact duplicate inside one comma expression) |
| O2 coreutils ls / dir / vdir (a) | `v14 = dat_26200;` removed; the identical write is 2 lines above with only `if (dat_26200 <= dat_252e0) goto label_80ec;` between | copy removed, value provably identical (only control flow between; neither `v14` nor `dat_26200` is written) |
| O2 coreutils ls / dir / vdir (b) | `v15 = dat_26311;` removed; the identical write is 4 lines above in the enclosing block, `v13 = v11; v7 = a0;` and the `*a0 != '/'` test between | copy removed, value provably identical (dominating write, no store to either side between) |
| O2 coreutils touch | `dat_19130 = dat_19120; dat_19138 = dat_19128;` removed; the identical pair is immediately above | copy removed, value provably identical (adjacent exact duplicate pair; the `sub_4330` call is above BOTH copies) |
| O2-noinline diffutils diff3 | `v1 = a1;` and `v1 = v14;` removed, each an adjacent exact duplicate | copy removed, value provably identical |
| O2 / O2-noinline shadow usermod | `dat_23568 = v20;` (O2) / `dat_23568 = v12;` (O2-noinline) removed, adjacent exact duplicate | copy removed, value provably identical |

* copy assignment removed and value provably identical: **18 lines / 10 binaries**
* variable count delta: **0** (`varcensus` `declarations` and `single_def_single_use`
  identical in both arms on ls O0, ls O2, touch O2, usermod O2-noinline)
* anything else (a BUG hunk): **0**

Raw unified diffs: `corpus-diffs.txt`.
