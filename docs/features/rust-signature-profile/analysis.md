# Rust compiler records and signature profiles

A Rust binary can carry an exact compiler version and still match signature banks
labeled for a different release. In the pinned 2025 coreutils `fmt` image,
the compiler record names `1.89.0-nightly`. The compatible signature bank pair
is labeled `1.88.0`. The 2023 O3 fixture matches the `1.71.0` O3 pair, and the
2025 O0 and O3 fixtures match their respective optimization profiles.

Kuna's FLIRT matcher can already compare patterns against machine code. A useful
report must also account for missing banks, tied scores, executable section
eligibility, and the provenance of the signature bytes. Treat the compiler record
and matching result as separate observations so that a profile score never
becomes a compiler or ABI assertion.

The comparison cases come from [angr's Rust version detection tests](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_rust_decompiler.py) and use [the pinned binary corpus](https://github.com/angr/binaries/tree/76ad9e296dde8f82b8ede93ce06a578cd1933639/tests/x86_64/rust/coreutils) with [the pinned signature corpus](https://github.com/angr/flirt_signatures/tree/eaf9234448ced5800497d8ec2c565b186c9f65e4/x86_64/linux/rust).
