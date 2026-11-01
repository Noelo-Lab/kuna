# Go DWARF 5 empty entry ranges

`build.py` assembles eight amd64 functions with Go register-ABI metadata and
DWARF 5 location lists. Run it with Python 3, GNU `as`, `ld` and `objcopy` to rebuild
`entry-ranges` (SHA256
`b0f38a89fec7a625d005888c2b10b77352e63d605ac64357eccf2d0bbfabb5a0`).

`identity` mirrors Go 1.27.3's one-byte `RET` identity functions: its input has
a single zero-length range at function entry, with `DW_OP_reg0`. The seven
negative cases change the register, range address, instruction, function extent,
range multiplicity, base address or result location. Only `identity` may acquire
the imported signature.
