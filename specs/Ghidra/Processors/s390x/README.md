# IBM z/Architecture (s390x)

SLEIGH processor module for 64-bit Linux on IBM Z. Language id
`s390x:BE:64:default`, selected for an `EM_S390` ELF of the 64-bit class.

## What is modelled

- The general instructions of the z/Architecture through the z15 facilities:
  RR, RRE, RRF, RX, RXY, RS, RSY, RI, RIL, RIE, RIS, RRS, SI, SIY, SIL and SS
  formats.
- Binary floating point on `f0`-`f15`: short and long, and extended values in
  the register pairs `fN`:`fN+2`.
- The s390x ELF ABI in `s390x.cspec`.

Not yet modelled: the vector facility (every `E7` opcode), hexadecimal and
decimal floating point, packed decimal, and the privileged instructions. An
instruction outside the module does not decode.

## How the files are produced

`gen_sinc.py` holds the opcode tables and writes the four `s390x_*.sinc`
files, which are committed:

```sh
python3 gen_sinc.py data/languages
slacomp data/languages/s390x.slaspec
```

## The condition code

The 2-bit condition code is one-hot in `CC0`..`CC3`: `CCn` is set exactly when
the condition code is `n`. `CCA` ("2 or 3") and `CCB` ("1 or 3") repeat that
state for the two-code masks; every instruction that sets the condition code
sets all six. A branch mask is a boolean over the flags, chosen so the branch
after a compare, a test or a logical add is one relation.

## Approximations

- `MVC`/`CLC`/`NC`/`OC`/`XC` of 1, 2, 4 or 8 bytes are one access of that
  width. An overlapping `MVC` of those lengths (the byte-propagation idiom) is
  therefore read as a plain copy. Other lengths are the user operations `mvc`,
  `clc`, `storage_and`, `storage_or` and `storage_xor`.
- `SRST`, `CLST` and `MVST` are the user operations `search_string`,
  `compare_string` and `move_string` and always complete; `CLST` does not
  advance its two address registers.
- `EX` and `EXRL` are the user operation `execute(register, target)`.
- A floating-point to fixed conversion sets the condition code from its
  operand; condition code 3 is set for a NaN but not for a value out of range,
  and an out-of-range value is not saturated.
- Multiply-and-add and multiply-and-subtract (`MADBR`, `MAEBR`, ...) are a
  multiply followed by an add, so the intermediate product is rounded.
- `POPCNT` without the z15 mask is the user operation `popcount_bytes`.
