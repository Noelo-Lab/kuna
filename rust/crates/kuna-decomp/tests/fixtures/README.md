# Test fixtures

## `list_action_decompile_oracle.txt`

Raw `list action` console output of the **main checkout's** C++ `decomp_dbg`,
for the derived **decompile** root action tree.  It is the B0 oracle for
`tests/universalaction_listing.rs` (the `rport/w8-fw-universalaction` keystone):
the Rust `SchedNode` decompile dump must equal this byte-for-byte after removing
the documented unported-pass allowlist (`universalaction::UNPORTED_ALLOWLIST`)
and renumbering.

Regenerate from the repo root:

```sh
printf 'int main(){return 0;}\n' | gcc -O0 -x c - -o /tmp/t.out
printf 'load file /tmp/t.out\nread symbols\nload function main\nlist action\nquit\n' \
    | SLEIGHHOME="$(pwd)/specs" ./decompiler/cpp/decomp_dbg 2>/dev/null \
    | awk '/^\[decomp\]> list action$/{f=1;next} /^\[decomp\]> quit$/{f=0} f' \
    > rust/crates/kuna-decomp/tests/fixtures/list_action_decompile_oracle.txt
```

`load function main` sets `decompile` as the current root, so `list action`
prints the filtered decompile tree.  The `awk` window strips the console
prompt/echo lines, leaving the pure `Action::print` dump (numbered lines plus the
blank separator each nested group/pool leaves beneath it).  Requires the SLEIGH
`.sla` to be built (`make specs`) and `decomp_dbg` linked (`make binaries`).
