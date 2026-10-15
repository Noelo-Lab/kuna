# Huge-function memory benchmark

Generate a public x86-64 Linux ELF fixture with an adjustable number of conditional
blocks. Assembly keeps the compiler from folding the workload away. Each branch
changes a live accumulator that reaches the return value. No external libraries
or private binaries are required.

The `escaped-locals` shape extends the existing `escaped_locals.c` regression:
distinct stack structs are passed to callbacks, multiplying heritage's guarded
stack ranges by calls. It uses unoptimized C with stack reuse disabled.

```sh
python3 scripts/huge_function_bench.py --out /tmp/kuna-escaped \
  --shape escaped-locals --blocks 100 350 700 \
  --kuna decompiler/target/release/kuna --seconds 10 --memory-mib 512
```

From the repository root:

```sh
python3 scripts/huge_function_bench.py --out /tmp/kuna-huge --blocks 100 1000 10000
```

Requirements: Python 3, an x86-64 GNU assembler/linker exposed through `gcc`, and
`nm`. Run on x86-64 Linux or supply an appropriate cross compiler with `--cc`.
The script never executes the generated fixture.

With built Kuna binaries and processor specs available:

```sh
python3 scripts/huge_function_bench.py --out /tmp/kuna-huge \
  --blocks 100 1000 10000 \
  --kuna decompiler/target/release/kuna --seconds 30 --memory-mib 1024
```

Measurement uses Linux `wait4` without additional utilities. Each fresh
process records elapsed and CPU seconds, peak resident KiB, exit status, function byte
size, timeout status, and its command in `results.json`. Compiler/platform identity
and the Kuna executable hash are stored in `metadata.json`; each result includes
the ELF hash and build command. Decompilation JSON and diagnostics are retained
beside each ELF. Peak RSS measures the process tree's maximum child usage,
not the sum of simultaneously resident processes. Failed or truncated runs are
not successful full decompilations; inspect the diagnostics and emitted warnings.
An external wall deadline kills the process group ten seconds after Kuna's budget.
Use repeated `--option NAME VALUE` arguments to compare phase decisions, for
example `--option listing off`.

Repeat runs in separate output directories before drawing performance conclusions.
Increase block counts gradually while watching machine memory. The cooperative
Kuna memory limit is not a hard operating-system cap. This workload probes scaling;
it does not establish that the private PE failure in issue #1034 has the same cause.

## Initial observations

On repository revision `fe01d629` (release build, GCC 16.2.1, x86-64 Linux), with
a 10-second Kuna budget and 512 MiB memory limit:

| Shape | Blocks | Function bytes | Peak RSS MiB | Outcome |
|---|---:|---:|---:|---|
| branches | 100 | 1,603 | 78 | Decompiled |
| branches | 1,000 | 19,495 | 221 | External deadline at 20 seconds |
| branches | 10,000 | 199,495 | 956 | Cooperative memory limit |
| escaped-locals | 100 | 6,205 | 119–121 | Decompiled |
| escaped-locals | 350 | 21,705 | 515–517 | Cooperative memory limit |
| escaped-locals | 700 | 43,405 | 517–518 | Cooperative memory limit |

These are illustrative measurements, not stable speed baselines. Two escaped-local
runs produced the same outcomes. Peak memory can exceed the cooperative limit;
the 10,000-block example shows why an operating-system cap remains useful.

Disabling `listing` did not avoid the 1,000-block timeout. A GDB sample after five
seconds with `--max-fn-seconds 1` stopped in
`p8_structure::region_structurer::RegionStructurer::structure`, reached through
`ActionBlockStructure::apply`. This identifies a P8 structuring cost and a
watchdog blind spot worth investigating separately. The same 1,000-block fixture
completed in about 5.2 seconds with `--option regionstructure off`, at 224 MiB
peak RSS. Reproduce the comparison with:

```sh
python3 scripts/huge_function_bench.py --out /tmp/kuna-region-off \
  --blocks 1000 --kuna decompiler/target/release/kuna \
  --seconds 10 --memory-mib 512 --option regionstructure off
```

The escaped-stack shape
targets the heritage range-by-call growth described in `escaped_locals.c` and
the existing regression README.
