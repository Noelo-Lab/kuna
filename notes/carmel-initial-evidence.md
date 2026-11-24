# Carmel: initial lift evidence

## Compiler suite

The [original C and assembler fixtures](../tests/fixtures/carmel/README.md)
are independent of the generated SLEIGH. Both TASKING C89 levels linked with
no run-time library. The native simulator matched 18 bounded cases at each
level, including six `addone`/`call_addone` calls, multiple control paths,
DATA aliasing and volatile stores, stack-spilled arguments, pointer calls,
and fixed-point dot/FIR output. FIR used independent DATA buses (`0x4110` and
`0xc120`); both streams in single-port memory produced native stop `0x501`.
The `-O0` nested loop and FIR needed 397 and 478 cycles respectively to reach
the return sentinel. These are native state witnesses, not Kuna lift coverage.

| Source fixture | `-O0` Kuna | `-O2` Kuna | First-slice boundary |
|---|---|---|---|
| `addone` | truncated | complete | Unoptimized `LINK` frame unlifted |
| `call_addone` | truncated | truncated | `LINK` / wide relative CALL |
| `call_stack` | truncated | truncated | Stack argument PUSH / wide CALL |
| `call_indirect` | truncated | truncated | Saved-register stack and conditional/indirect flow |
| `call_pointer` | truncated | truncated | Saved-register stack and indirect call |
| `scalar` | truncated | truncated | General logic and shifts |
| `nested` | truncated | truncated | Comparisons, conditional branches and hardware repeat |
| `bump_field` | truncated | truncated | Offset DATA accesses and frame forms |
| `volatile_pair` | truncated | truncated | Volatile DATA loads and frame forms |
| `stack_args` | truncated | truncated | `LINK`, stack DATA reads and general add |
| `dot_q7` | truncated | truncated | Wider MAC, DATA loads and repeat |
| `fir_q7` | truncated | truncated | Wider MAC, DATA loads and repeated/conditional flow |

The score is 1 complete of 24 fixed source/optimization pairs, 23 truncated,
0 failed. Compiler-discovered internal `_from_q7` is not part of this count.
There is no complete pointer, call or DSP C result to showcase. The completed
scalar has one recovered 16-bit argument, a return, and no gotos. Its expression
retains the machine's optional saturation at `0x7fff`; `ECFG` currently appears
as a register-derived local instead of a typed input. All bounded defined inputs
agree with the original C; signed overflow at `0x7fff` is not a C witness.

```c
short sub_2e(short a0)
{
  unsigned short v1; // ecfg
  return (a0 + 1) - (unsigned short)(a0 == 0x7fff && v1 & 4);
}
```

The raw odd-slot CLI case in the plan reaches `INC A1L` and then `RETURN`.
The 16-bit TASKING scalar result is in `A0L`, so SSA removes the `A1L` write
from C as dead. The raw p-code test pins the actual `A1L` write and absence of
an `A0L` write; the stage test separately puts a live-return increment in the
odd slot so the emitted C exercises that path.

## Independent native reference

The p-code fixtures verify old-state reads (`21ac2031aa20`), widened INC
status and wrap, signed DATA stores, CODE-word returns, delayed branches,
call return-PC placement, and the signed CLIW MAC at both ECFG scales.
Unlifted effects are not no-ops. A single-issue state match does not make
adjacent DATA reads/writes cycle-faithful. The lift publishes lane flags, not
the pipeline's standing condition-execution feedback in `EST[13:12]`; no
predicated flow or PowerPlug execution is claimed. The Python reference
remains unfinished: sealed
`54c095800007c6a96829691a45b425f51a25da97` matched 4,017/4,096 draws;
the 27 targeted repairs on `eb76f5f20c` were not a fresh holdout. No new
raw-encoding confidence or original PE32 instruction-coverage claim follows.

The focused native differential, clock, control, CLIW and PowerPlug witnesses
passed (eight tests across the two targeted runs). The broader five-module
native run reached 102 completed tests before its deadline; it has no final
result and is not a confidence or cycle gate.

## Wild corpus

The image and entry manifest was frozen before the constructors. The field
images are Linksys WRTU54G-TM 4.16.363, VINETIC 2CPE 0.17.64 and Sigrand/HT503,
VINETIC 4VIP 16.15.56, and sealed 2CPE 0.17.200. The SDK images are the
TASKING linked examples `demo`, `filter`, `graphs`, `spectra`, and sealed
`queens`. The recovered 3.0r6 archive added no unique linked example: its
three members exactly matched 3.1r2. No verified M-GOLD, SignalMaster/CAREB,
or MoNET binary was found. Original binaries and XML are private.

| Split | Images | Fixed anchors | Supported / unsupported at reached issues | Complete / truncated C |
|---|---:|---:|---:|---:|
| Field development | 4 | 18 | 0 / 18 | 0 / 18 |
| Field holdout | 1 | 6 | 0 / 6 | 0 / 6 |
| SDK development | 4 | 4 | 0 / 4 | 0 / 4 |
| SDK holdout | 1 | 1 | 0 / 1 | 0 / 1 |

All 29 fixed starts agreed with the independent framing cross-check, not a
native disassembly oracle. Every walk hit an unsupported issue at its start;
there were no failed or timed-out images, no comparable wild direct edges,
and no complete wild C or structural-quality score. In particular, zero
recorded unresolved ROM/CLIW edges does not mean every target is resolved.
The two holdout images ran once, without tuning against them.

Run the same private sweep with an XML image produced by the reference
repository's `tools/carmel_wild_export.py`:

```sh
KUNA_CARMEL_WILD_IMAGE=/absolute/private/image.xml \
  cargo test --manifest-path decompiler/Cargo.toml --release -p kuna-console \
  --test integration verify_carmel_wild::sweep -- --ignored --exact --nocapture
```

The release sweep binary SHA-256 was
`a6b40068a7ad74df646b87827d3149b4b8426a509a340c0ae26943c5caada83d`;
the compiled spec hash was
`28f1b7194c3e80e1d960613f18ffb41bdd5a8a1904a7a4c62003210645715b57`.
Development image runs took 1.58–1.88 s including process startup; the two
holdouts took 2.76 s and 3.57 s. The compiled `addone` CLI cold-process
median was 1.76 s (seven runs). The x86-64 `/usr/bin/true` entry report was
byte-identical to the installed baseline, and the vendored datatest corpus
kept all 675 passing assertions.

A separately bounded wild witness used VINETIC 2CPE 0.17.64 `pcm_ch`
at CODE word `0x89a`: TASKING and Kuna p-code agreed on `A2L=0x8000`,
the preserved accumulator high halves and guards, `EST=0x10`, and the next
issue at word `0x89c`. It is inside a vector-backed function but is not
an additional function anchor. The three byte-identical 2CPE V2.2 copies
do not count as independent state witnesses.
