# Go compiler-check folding

Match Go compiler panic helpers inside a terminal conditional successor. Remove
only that edge, leaving ordinary SSA cleanup to discard unused check operands.
Use the existing P7 fullloop slot so call and global-store effects are already
explicit. Require the Go source-language fact and `gobounds on`. Exact names
exclude explicit panic and type assertions; any store, volatile access, other
call, or branch with two matching successors vetoes the edit.

The regression covers option off/on, a C-language control, an arm with an
observable store, and a straight-line panic wrapper. A compiled stripped Go
fixture exercises the detection path on amd64 and arm64, including explicit
panic and a store preceding an indexed load.

This is one feature toward Go/Rust parity. Go source emission, multi-value
presentation, typed slices/interfaces, runtime builtin reconstruction, closures,
channels, maps, and the Rust enum/macro/pattern passes remain separate work.
