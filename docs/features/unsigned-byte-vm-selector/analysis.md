# loweredswitchlabels — preserve comparison-proven label signedness

## Concrete divergence

The exact `KataVM_L1` function at `0x12d0` declares its dispatch selector as
`unsigned char` and compares `AL` with opcode `0x8b`. Kuna recovered the
compiler's comparison cascade as a switch but printed `case -0x75`. Integer
promotion makes that label unreachable from the unsigned-byte selector.

The generic numeric printer was behaving as instructed. The lowered-switch
installer marked the entire table signed whenever any case constant had the
selector-width sign bit, so the bit pattern `0x8b` became `-117` without any
signed comparison or signed selector evidence.

## Owning phase and repair

This is P2 `switch-model`. Detection already requires at least one range split
to distinguish a compiler-lowered switch from a hand-written equality chain.
Those range opcodes carry the missing interpretation:

- `INT_SLESS` and `INT_SLESSEQUAL` prove signed labels;
- `INT_LESS` and `INT_LESSEQUAL` prove unsigned labels; and
- equality case constants provide no signedness evidence.

Transparent selector identity is not enough: a `ZEXT` path proves the
underlying selector unsigned, while `SEXT` proves it signed. Detection now
carries that widening fact and the effective selector width through
canonicalization. It safely projects widened constants back to selector-width
bit patterns, then requires the extension, the range opcode, every range node,
and every comparison path to agree. Thus `ZEXT(u8)` followed by `SLESS` cannot
promote a table whose `0x8b` label would become `-0x75`; the converse
`SEXT`/unsigned-range form and mixed ZEXT/SEXT paths also decline.

A shrinking zero-offset `SUBPIECE` is not a transparent selector identity in
strict mode. The restart record stores only an address and width, so peeling
`SUBPIECE(wide, 0)` to `wide` while retaining byte-sized case normalization
would install a different expression: nonzero upper bits could select a
different `BRANCHIND` target. Detection still canonicalizes to the wide source
for structural ownership, marks that path ambiguous, and declines the complete
cascade. Same-width `SUBPIECE` remains transparent. Option off deliberately
retains the historical shrinking peel.

The proven bit is stored in the address-only side record that survives the
pipeline restart, then installed directly on the synthesized `JumpTable`. Every
new proof and decline is gated by `loweredswitchlabels`; option off performs the
original structural recovery and historical high-case-bit guess.

## Controls and blast radius

Unit controls cover signed byte `0x8b` as -117, signed int `0xffffffff` as -1,
unsigned u32 `0xffffffff`, matching ZEXT/unsigned and SEXT/signed widened
selectors, widened-constant normalization, and rejection of ZEXT/signed,
SEXT/unsigned, mixed-extension-path, mixed-range evidence, and a wide live source
projected through a low-byte `SUBPIECE`. The truncation control also proves the
ambiguous outer head remains in the structural map, so its inner three-case/range
subtree is not recovered independently. Option-off controls prove extension
conflicts, mixed range cascades, and the historical wide-selector truncation peel
still recover with the legacy heuristic. The exact fixture's
two-pass stage test proves `loweredswitchlabels off` reproduces `case -0x75`
while the default renders `case 0x8b`; both retain the switch and low labels.
With `loweredswitch off`, the switch disappears and the original chain retains
`v17 != 0x8b`, proving the repair is metadata on the recovery path.

The P9 numeric printer is untouched. Signed getopt-style switches therefore
retain negative labels, while unsigned high-bit cases no longer borrow that
format solely from their bit pattern.
