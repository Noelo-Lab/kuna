# Rust process exit

`main.rs` is independently authored. `exit_choice` returns 27 on one arm,
terminates with status 0 on another, and stores 31 before terminating with
status 1 on the remaining arm. `returning_names` calls a different namespace's
`exit` and the deliberately similar `std::process::exit_status` symbol, then
stores their result. Both helpers return normally.

Build from the repository root with Rust 1.93.1:

```sh
rustc +1.93.1 --crate-name process_exit_probe --crate-type cdylib --edition 2021 -C opt-level=2 -C debuginfo=0 -C strip=debuginfo -C codegen-units=1 tests/fixtures/rust-process-exit/main.rs -o tests/fixtures/rust-process-exit/linux-amd64
chmod a-x tests/fixtures/rust-process-exit/linux-amd64
```

The 412,104-byte ELF has SHA-256
`bcc23e380abe05a986923406cb1ad5ff03ccb2547e4d3bcfe10fcacd97e94aeb`.
It is supplied only as decompiler input and has never been executed. The actual
standard-library exit symbol is `_ZN3std7process4exit17h644d5313113857bcE`.
The loader unit also scans the same bytes under explicit non-Rust compiler
selections to prove that the additional contract is language-gated.
