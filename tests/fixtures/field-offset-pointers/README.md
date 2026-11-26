# Pointer parameters read only at large field offsets

`fields.c` declares three records whose fields sit below, at and well past
0x1000, the lowest value x86-64 and AArch64 treat as a possible address.
`near_leaf` reads its record at 0xff8 and 0xffc, `page_leaf` at 0x1000 and
0x1004, and `conn_leaf` at 0x48a0, 0x48a4 and 0x48a8; none of them reads offset
0. Four functions index a table with their first parameter, which is an index,
not a pointer: `get_data` the exported `.data` table, `put_bss` the exported
`.bss` table, `get_code` a `static const` table placed in `.text` (as firmware
that links read-only data into code has it), and `get_local` a `static` `.bss`
table.

| Image | Kind | Layout |
| --- | --- | --- |
| `fields_x86_64.so` | x86-64 shared object | `near_leaf` at 0x1000, so `page_leaf`'s field offset is also a function entry; `code_table` at 0x10e0 in `.text` |
| `fields_x86_64_stripped.so` | the same, stripped | only the dynamic symbols remain: `code_table` and `local_table` are unnamed |
| `fields_x86_64` | x86-64 non-PIE executable | code at 0x401000, data at 0x403000 |
| `fields_aarch64.so` | AArch64 shared object | code at 0x380, `code_table` at 0x450, data at 0x2000 |
| `code_only_x86_64.so` | x86-64 shared object from `code_only.c` | no data section; `update` at 0x1000 reads its record at 0x1000 and 0x1004 |

Built in this directory with Ubuntu's GCC 15.2.0 (`gcc-15` 15.2.0-16ubuntu1,
`gcc-15-aarch64-linux-gnu` 15.2.0-16ubuntu1cross1) and GNU binutils 2.46
(`binutils-x86-64-linux-gnu`, `binutils-aarch64-linux-gnu` 2.46-3ubuntu2); the
commands reproduce the committed files byte for byte. The assembler warns that
it ignores the changed attributes of `.text` for `code_table`.

```sh
gcc -O2 -fPIC -shared -nostdlib -fno-ident -Wl,--build-id=none -o fields_x86_64.so fields.c
x86_64-linux-gnu-strip --strip-all -o fields_x86_64_stripped.so fields_x86_64.so
gcc -O2 -fno-pie -no-pie -nostdlib -fno-ident -Wl,--build-id=none -Wl,-e,near_leaf -o fields_x86_64 fields.c
aarch64-linux-gnu-gcc -O2 -fPIC -shared -nostdlib -fno-ident -Wl,--build-id=none \
    -Wl,-z,max-page-size=0x1000 -Wl,-z,common-page-size=0x1000 -o fields_aarch64.so fields.c
gcc -O2 -fPIC -shared -nostdlib -fno-ident -Wl,--build-id=none -o code_only_x86_64.so code_only.c
```

| File | SHA-256 |
| --- | --- |
| `fields.c` | `e70b1516205a5abcf838e8b369ac88c1569e9b5c85c6159a363f590a577dd8b9` |
| `code_only.c` | `19efcae4d3280cbb6dd1331eda420dc44c4a28a1d3dd99745a7e48db6b41d21e` |
| `fields_x86_64.so` | `9439acada457beebc0e06185d7e45e05088548f064fedc67a0f481964221d1c8` |
| `fields_x86_64_stripped.so` | `777d03010d6b8e3df86d6b2d3647bd1b98348bf08e8645667997535eb809ec22` |
| `fields_x86_64` | `90e6995f9916089a24b0bfc4cb4aac01008df2fb24b3947b3250c3418cee10a0` |
| `fields_aarch64.so` | `8eb947f117104cae06ee3bd26179f96e08928ac1e39880c5e55b54c076e1a7a2` |
| `code_only_x86_64.so` | `1b3f03963a5f9c1c17b511084af55a4dd97d0b1c26dac37de6c7f3540f03e821` |

`tests/stages/kuna-ptrfromuse-faroffset.xml` loads all five.
