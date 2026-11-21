# FID prototype layout fixture

Records whose types the C parser could rebuild wrongly.
`decompiler/crates/kuna-cli/tests/fid_proto_cli.rs` builds a `.fid` from these
objects and checks what the match does with it.

- `lib.c` / `lib.o` (with debug information) / `prog` (`main.c` linked with a
  `lib.c` built without debug information, then stripped).
  - `hdr_score` takes a `struct hdr *` whose first member is a one-byte packed
    enum. The record spells the enum's width, so `len` and `id` keep their
    offsets.
  - `make_bits` returns, and `take_bits` takes, a `struct bits` with bitfields
    by value. The record leaves `bits` opaque, so the match drops both
    prototypes instead of inventing a hidden return buffer.
- `a.c` / `a.o` and `b.c` / `b.o` both define `struct st`, differently, and
  both have a `get` with the same code. The type is disputed, and the database
  is the same whichever object comes first.

## Build

```bash
F="-O2 -ffreestanding -fno-stack-protector -fno-pic"
gcc $F -g -fdebug-prefix-map=$PWD=. -c lib.c -o lib.o
gcc $F -c lib.c -o lib_nodebug.o
gcc $F -c main.c -o main.o
gcc -nostdlib -static -no-pie -Wl,-e,entry -o prog main.o lib_nodebug.o && strip --strip-all prog
gcc $F -g -fdebug-prefix-map=$PWD=. -c a.c -o a.o
gcc $F -g -fdebug-prefix-map=$PWD=. -c b.c -o b.o
```

These were built with gcc 11.4 (Ubuntu 22.04).
