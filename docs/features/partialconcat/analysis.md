The game client/server port conversion helper copies EDI into EAX, rotates AX
by eight bits, then returns. Its output preserves the upper word and swaps the
lower two bytes. Kuna emitted an undeclared CONCAT22 helper even though the
operation has an exact scalar C expression. A narrow return would hide the
preserved bits the assembly actually returns.

P9 can express the existing PIECE value with unsigned casts, masks and OR.
The lower mask is essential: a uint16_t expression is promoted to int in C,
so the left shift can produce bits outside the low word. The source prefix
may appear directly as SUBPIECE or as a zero-offset truncation after cast
insertion replaces the extraction with a right shift.

An off/on sweep of 880 application and helper functions across the 16 module
binaries changed exactly the eight port conversion helpers. Crackmes and cIMG
functions were unchanged. Generated 32- and 64-bit C compiled with UBSan and
matched the assembly over every low word under seven prefixes and one million
additional random inputs: 2,917,504 comparisons passed.
