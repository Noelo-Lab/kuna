# Go output fixtures

`output.c` exercises Go emission from a known C ABI, including width conversions,
short-circuit calls, bottom-tested loops, shared switch cases, pointer indexing,
bit-preserving float conversions and readonly text. `output-c` was built with
GCC 11.4.0 on x86-64 Linux:

```sh
gcc -O2 -g -fno-pie -no-pie -fno-inline -fno-stack-protector \
    -o output-c output.c
```

SHA256: `2ab52230e3235ef5517798c672c476e136122a11526b0d4a5560f04fb4bfa8d4`.

`check.go` compiles with the untouched recovered function bodies and checks their
values and side effects. The callback check supplies its prototype through
`map prototype`: the current DWARF loader imports its function pointer as opaque. `parse.go` checks complete emitted functions with Go's
parser. `output.go` is a native Go source companion; the integration test builds
it with the installed Go toolchain, checks all fourteen named functions for
syntax, and executes its recovered scalar functions. Go 1.20 or newer is needed
for `unsafe.StringData`; local reference measurements use Go 1.22.5.
