# Byte-array view review

The baseline is package-type commit `72757bccf5e212f0a3238bd22c47cd2800665a47`;
its tree equals merged main `0a449781c78ced2bc8009fad2e2f6acf01de88a9`.
The working changes were preserved byte for byte while advancing to that merge.

The final sixteen-image comparison has 2,841 changed Go functions, 13,399 typed
integer views and 9,241 whole-array covers. Every change reverses to its exact
original path and access width, or removes a whole-array cover. All remaining
non-whitespace tokens, including comments and literals, compare identically.
The verifier rejects independent arithmetic, offset, width and comment mutations.
This is a spelling check; native execution separately checks behavior and types.

The native pair writes offsets zero and eight of its sixteen-byte carrier and
returns the entire carrier. Compressed and plain binaries built with Go 1.22.5
and 1.27.3 produce bodies that execute unchanged under both compilers. Six
inputs include negative values and both signed extremes, checking wraparound
and both return words. This proves the recovered carrier, not source tuples.

The mapped `Wrap.payload` witness accesses two bytes at offset one and four
bytes at offset seven. Both unsigned and signed parameter declarations execute
25 input pairs, including the sign bits and all-one bit patterns. The emitted
uint16/uint32 store views match their assignment conversions. The reads preserve
zero extension, and ten other bytes remain unchanged.

Real-image review includes `runtime.runExitHooks` in the Go 1.22 native image:
the eight-byte view at `exitHooks+8` is int64, agreeing with its assignment.
`runtime.gcStart` has uint32 views for the unsigned conversions at `work+0x194`
and `work+0x198`. `runtime.mallocinit` retains the signed two-byte read at
`class_to_size+4` and its surrounding uint16 conversion. These are width and
signedness choices, not changes to the recovered memory layout.

The signedness correction changes 135 Go functions relative to the initial
implementation. Reverse-view token comparison finds no other token changes.
It also declines thirteen formerly emitted integer views when the rendered
value is a pointer, including `userArenaState+32` and `runtime.trace+16`.
Both the facing IR type and the rendered Go type must qualify; an integer
facing type alone cannot authorize a pointer-valued view. These accesses retain
the baseline artificial member and remain a separate output gap.

The largest affected function is `cmpbody`, with 109 whole-array cover removals;
its native Go 1.27 address is `0x407b20`. AES hash bodies exercise 97 whole-array
covers and two typed integer views. Their raw diffs retain all computations and
control flow. The speed comparison includes this largest cover case, the native
pair, default C, and a Rust control.

The 1,769 explicit-conversion assignments in the final whole-image outputs have
matching integer view and conversion types. This scan cannot type-check every
external reference in every runtime function; executable witnesses and the
production planner's eligibility checks provide the additional evidence.
