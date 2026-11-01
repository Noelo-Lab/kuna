# Dynamic Rust print arguments

At angr commit `6afa0c79a83d9f677a46b0275cdb63bf16c1979e`, TestRustFormatMacrosTemplateLayout expects print_one and reorder in fmtdemo-1.93.1 to render println! with recovered values and argument indices. Kuna exposes the two runtime words and stack descriptor array instead.

Rust 1.93.0/1.93.1 stores bounded literal pieces and placeholders in a readonly template; the other word points to pairs of value pointer and formatter callback. The final IR preserves descriptor cells as stack INDIRECT effects associated with the print call. Their input zero is the pre-call cell, whereas their output can describe a modified value and must not supply the callback identity.

Display, Debug and scientific formatter names can collapse when trait qualifiers are removed. Even the core::fmt::num::imp module contains both Display and exponent traits. A bare or normalized fmt name is insufficient evidence. Preserve exact raw-symbol trait/type identity before normalization and reject ambiguous aliases. Generic reference formatters also require pointee type recovery and remain outside this feature.

Primary sources: Rust tags 1.93.0 and 1.93.1 library/core/src/fmt/{mod,num}.rs; angr/rust/optimization_passes/macro/fmt_template.py and tests/analyses/decompiler/test_rust_decompiler.py at the pinned revision. The real fixture is angr/binaries commit 76ad9e296dde8f82b8ede93ce06a578cd1933639, tests/x86_64/rust/fmt_arguments/fmtdemo-1.93.1. The pinned angr integration test was executed locally in an isolated environment and passed all eight expectations in 28.433 seconds. This confirms the reference target; it does not imply that Kuna meets those expectations.
