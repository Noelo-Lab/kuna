# FID prototype fixture

A `.fid` record can carry its function's prototype, parameter names and the
types they need. `lib.o` is `lib.c` built with debug information; `lib.fid` is
the database `kuna fid build` writes from it (format version 2); `prog` is
`main.c` linked with a `lib.c` built without debug information, then stripped.
`decompiler/crates/kuna-cli/tests/fid_proto_cli.rs` rebuilds `lib.fid` from
`lib.o` and decompiles `prog` against it.

## Build

```bash
clang -O2 -g -fdebug-prefix-map=$PWD=. -fno-pic -c lib.c -o lib.o
clang -O2 -fno-pic -c lib.c -o lib_nodebug.o
clang -O2 -fno-pic -fno-stack-protector -c main.c -o main.o
ld.lld -e check -o prog main.o lib_nodebug.o && llvm-strip --strip-all prog
kuna fid build lib.o -o lib.fid --lang x86:LE:64:default --cspec gcc
```

`parse_header` is at `0x201230` in `prog`, `check` at `0x201200`.
