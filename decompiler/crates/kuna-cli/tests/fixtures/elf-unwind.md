# ELF unwind fixture

`ppc-pc-read.elf` is an original synthetic ELF32 big-endian PowerPC image,
authored for this repository under its Apache-2.0 license. It contains no
third-party binary data. Its `bump` saves LR, captures PC with `bcl 20,31`,
reads an inline word, and returns `x * 3 + 7`.

Regenerate with `rustc --edition=2021 elf_unwind_fixture.rs -o generate-elf`,
then `./generate-elf ppc-pc-read.elf`. CLI tests generate both byte orders,
32/64-bit address fields, absolute and relative FDE encodings, and an explicit
inner function entry from the same source.
