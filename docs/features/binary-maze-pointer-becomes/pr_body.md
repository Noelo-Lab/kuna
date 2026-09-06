## The problem

A pointer into a binary blob prints as an empty string literal, and the address
it displaced disappears from the C entirely. Build the two-line reduction and
decompile it:

```
$ cat > blob.c <<'C'
#include <string.h>
const unsigned char maze[64] = { 0,0,0,0,0,0,0,0, 0,0x77,0xdf,0x77,0xff,0xfd,0xff,0x7f };
const char merged[] = "\0Report bugs to: %s\n";
unsigned long probe(void) { return strlen((const char *)maze) + strlen(merged); }
int main(void) { return (int)probe(); }
C
$ gcc -O0 -no-pie -fno-builtin -o blob blob.c && strip blob
$ kuna decompile ./blob 0x401136 --addr
long sub_401136(void)
{
  ...
  v1 = strlen("");
  return v1 + strlen("");
}
```

Both calls read `strlen("")`, and the two are not the same thing: `maze` is 64
bytes of packed data whose first row happens to be zeroes, `merged` is a real
empty string. The blob's address is gone.

The witness is crackmes.one `60be2ad433c5d410b8842c95` (`Sabloom Text 6.exe`),
where `sub_402020` walks a 585-byte packed maze at `0x403550`: kuna emitted
`v16 = ""; v21 = "";` and then indexed past the terminator, and `0x403550`
appeared nowhere in the function while `0x403511` and `0x403799` around it
survived.

## The fix

- `pushPtrCharConstant`'s accept test is `StringManager::isString`, which says
  yes for any read-only location whose first byte is a NUL — `checkCharacters`
  walks only to the first terminator, so for a zero-length string it validates
  zero characters and can reject nothing.
- New `emptystrconst` option (P9, default on) declines the literal when the
  escape walk emitted no characters, and the constant falls through to the
  ordinary casted-hex print.
- Emptiness alone is not the tell: `setlocale(6,"")` is idiomatic, and a linker
  that merges string constants stores a program's only `""` as the tail NUL of
  another literal. So the rule also reads sixteen bytes at the address and
  declines only when they positively contradict text: skip the terminator run,
  walk the next run to its NUL, and reject on a byte no C string holds (outside
  printable ASCII and `\t`/`\n`/`\r`). It is a falsification test, so padding, an
  unreadable window, and a run still spelling text all keep the literal.
- What this does not repair: the constant is `char *` because it shares a merged
  live range with a genuine `char *`. That typing is the reason the probe runs at
  all and is left alone.

## The tests

`tests/stages/kuna-emptystrconst.xml` is the two-pass case: with the option off
the blob pointer and a genuine `""` both print `use("")`; on, only the blob's
becomes `use((char *)0x100100)`. `tests/cli/binary-maze-pointer-becomes.json` is
the promoted probe over the new `emptystrconst_x86_64` fixture and pins both
directions. `make test` is 675/675 byte-identical. A `decompile-all` sweep over
58 images found 327 empty literals: 312 kept, 15 declined, every changed line a
1:1 `""` to `(type *)0xADDR` substitution.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
