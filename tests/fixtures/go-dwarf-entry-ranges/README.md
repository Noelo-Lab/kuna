# Go DWARF 5 empty entry ranges

`build.py` assembles eight amd64 functions with Go register-ABI metadata and
DWARF 5 location lists. Run it with Python 3, GNU `as`, `ld` and `objcopy` to rebuild
`entry-ranges` (SHA256
`b0f38a89fec7a625d005888c2b10b77352e63d605ac64357eccf2d0bbfabb5a0`).
The identical uncompressed debug sections are retained in `entry-ranges-plain`
(SHA256 `4634398d909f8a704c9a9c5b7dbb281435d8615362364559b8608d45a2136179`).
That copy checks that rejected Go contracts cannot acquire C-style prototypes
with result formals mistaken for inputs, with either setting of `cppproto`.

`identity` mirrors Go 1.27.3's one-byte `RET` identity functions: its input has
a single zero-length range at function entry, with `DW_OP_reg0`. The seven
negative cases change the register, range address, instruction, function extent,
range multiplicity, base address or result location. Only `identity` may acquire
the imported signature.
