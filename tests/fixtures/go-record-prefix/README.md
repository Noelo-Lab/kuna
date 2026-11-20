# Leading padding in a recovered Go record

The authored x86-64 fixture has a 24-byte DWARF record with named fields
at byte offsets 8 and 16. Four functions read eight bytes from the base,
from byte 4 inside the leading padding, and from each named field.
The stage selects Go emission for that recovered layout.

Regenerate the static test input from this directory.

```bash
cc -O2 -gdwarf-4 -fdebug-prefix-map="$PWD"=. -fno-pie -no-pie \
  -nostdlib -Wl,-e,before_a0 -o loads loads.c
chmod 644 loads
```
