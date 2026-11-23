# Named reference Option conditions

`named_some` and `named_none` consume a complete `Option<&u64>` only in a
null-niche predicate. Distinct callbacks record the selected branch. The
payload-consuming, returned-Option, raw-pointer and raw-integer functions
are controls for the boundary of a condition-only recovery.

The image was compiled with rustc 1.90.0 and full DWARF:

```sh
rustc --edition=2021 -C opt-level=1 -C debuginfo=2 -C panic=abort \
  -C relocation-model=static -C link-dead-code=yes -C link-arg=-nostartfiles \
  author.rs -o option-amd64
chmod 444 option-amd64
```

The image is input data for the decompiler. Native checks compile fresh
observers containing emitted functions; they do not run this image.
