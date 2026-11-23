# `--types` fixture

`curlish.so` is `curlish.c` built on x86-64 Linux with clang 22 and lld:

```bash
clang -O2 -c curlish.c -o curlish.o && ld.lld -shared -o curlish.so curlish.o
```

SHA-256 `7478204e368315d7f2ca2f48555d290074c29d168c9b8d1ff28ce19671583730`, 3088 bytes.
That clang enables `-fcf-protection` by default; add it to rebuild with one
that does not (clang 21 with it emits the same `fetch` and the same IBT PLT).

The headers are what `tests/types_header.rs` loads over it: `curl.h` (the
library), `app.h` (the caller), `bad.h` (one line that does not parse),
`macro.h` (needs a preprocessor) and `dir/` (a directory of headers).
