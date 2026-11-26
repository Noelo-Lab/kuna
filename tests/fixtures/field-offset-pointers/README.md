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

`absolute.c` holds the controls for the limits of that reading: `read_reg`
reads an absolute address past 1 MiB (0x40020000, where a microcontroller maps
its peripherals) at an offset in its parameter, `letter_rank` indexes the
first table in `.rodata` from a character (`letters[c - 'a']`, whose base
`&letters - 0x61` lies just below the section), and `get_ext` indexes an
extern table, which a relocatable object places in a slot outside every
section.

| Image | Kind | Layout |
| --- | --- | --- |
| `fields_x86_64.so` | x86-64 shared object | `near_leaf` at 0x1000, so `page_leaf`'s field offset is also a function entry; `code_table` at 0x10e0 in `.text` |
| `fields_x86_64_stripped.so` | the same, stripped | only the dynamic symbols remain: `code_table` and `local_table` are unnamed |
| `fields_x86_64` | x86-64 non-PIE executable | code at 0x401000, data at 0x403000 |
| `fields_aarch64.so` | AArch64 shared object | code at 0x380, `code_table` at 0x450, data at 0x2000 |
| `code_only_x86_64.so` | x86-64 shared object from `code_only.c` | no data section; `update` at 0x1000 reads its record at 0x1000 and 0x1004 |
| `absolute_x86_64.so` | x86-64 shared object from `absolute.c` | `read_reg` at 0x1000, `letter_rank` at 0x1010, `letters` at 0x2000 |
| `absolute_x86_64.o` | x86-64 relocatable object from `absolute.c` | `get_ext` at 0x30; kuna lays the object out from 0x400000 |

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
gcc -O2 -fPIC -shared -nostdlib -fno-ident -Wl,--build-id=none -o absolute_x86_64.so absolute.c
gcc -O2 -fno-ident -c -o absolute_x86_64.o absolute.c
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
| `absolute.c` | `1b0a87673fb69373b3c73f42778dbb97ca9f2903dbfb2f80c4a44190c4cfb8f9` |
| `absolute_x86_64.so` | `0191513bafdf81646eff7b72dfdc26753abea72df570209346973aadfedca15e` |
| `absolute_x86_64.o` | `9967f37b816cb8278fe4ff69f6f3299d234e91383a0f64be8ebc4fe65f747df1` |

`tests/stages/kuna-ptrfromuse-faroffset.xml` loads all seven.
