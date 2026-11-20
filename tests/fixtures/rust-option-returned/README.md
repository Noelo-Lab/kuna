# Returned Rust Option fixture

`inspect_returned` tests the tag of the complete `Option<u64>` returned by
`provide_tagged`. The Rust program supplies its own entry and was built
with rustc 1.90.0 for static decompilation. It uses a branch on the low
return word and writes a different callback marker for each outcome.

```sh
rustc --edition=2021 -C opt-level=1 -C debuginfo=2 -C panic=abort \
  -C relocation-model=static -C link-dead-code=yes -C link-arg=-nostartfiles \
  option.rs -o option-amd64
chmod a-x option-amd64
```
