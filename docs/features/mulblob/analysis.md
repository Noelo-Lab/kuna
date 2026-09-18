# mulblob — the 16-byte operand blobs of an unsigned wide multiply

## The observation

`coreutils fmt -O2`, `xstrtoumax` (`sub_74d0` in the stripped image) declares 28
locals for a function whose source has about eight. Twenty of them look like
this:

```
  char v10 [16];
  char v11 [16];
  ...
      v8._8_8_ = 0;
      v8._0_8_ = v29;
      v18._8_8_ = 0;
      v18._0_8_ = v27;
      v29 = SUB168(v8._0_16_ * v18._0_16_,0);
      if (SUB168(v8._0_16_ * v18._0_16_,8)) {
```

The function has ten `MUL RCX` instructions (0x767f, 0x7722, 0x7727, 0x774f,
0x7754, 0x7783, 0x7813, 0x7833, 0x7853, 0x7873). Each contributes two of these
16-byte locals, whose entire content is one operand. Binary-wide on `fmt -O2`
that is 23 of 298 non-thunk declarations, 7.7%.

Upstream Ghidra prints the same thing, as `undefined1 auVar1 [16]`, so this is a
quality divergence rather than a fidelity gap.

## Root cause

`MUL rm64` lowers in SLEIGH (`specs/Ghidra/Processors/x86/data/languages/ia.sinc:4128`)
to

```
tmp:16 = zext(RAX) * zext(rm64); RDX = tmp(8); RAX = tmp(0); multflags(RDX);
```

`print raw` on the function confirms the shape after the mainloop:

```
0x0000767f:ac1: u0x000a7308(0x0000767f:ac1) = #0x0
0x0000767f:ac2: u0x000a7300(0x0000767f:ac2) = RBX(0x00007609:731)
0x0000767f:30b: u0x000a7300:16(0x0000767f:30b) = CONCAT88(u0x000a7308,u0x000a7300)
0x0000767f:30c: u0x000a7400:16(0x0000767f:30c) = CONCAT88(u0x000a7408,u0x000a7400)
0x0000767f:30d: u0x000a7600:16 = u0x000a7300:16 * u0x000a7400:16
0x0000767f:30e: RDX = SUB168(u0x000a7600:16,#0x8:4)
0x0000767f:30f: RAX = SUB168(u0x000a7600:16,#0x0:4)
```

The two CONCAT88 ops carry the *original* lift-time sequence numbers (`30b`,
`30c`) while the constant and the operand COPY feeding them carry much later
ones (`ac1`..`ac4`): the ops that were lifted as `INT_ZEXT` were rewritten in
place into `CPUI_PIECE`.

The rewriter is `RulePieceStructure`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs`, C++
`ruleaction.cc:7488`). It asks `determine_datatype` for the extension output's
*structured* type; that resolves to the Varnode's own data-type when no Symbol
covers it, and a 16-byte Varnode has no primitive type, so
`TypeFactory::get_base(16, TYPE_UNKNOWN)` hands back the width fallback
`undefined1[16]`. `Datatype::is_piece_structured` is `metatype <= TYPE_ARRAY`
(`substrate/dtype.rs:1152`), so an array qualifies. The rule therefore converts
the `INT_ZEXT` into a `PIECE` of a zero constant and the operand
(`convert_zext_to_piece`), marks the output a partial root, and P6's
`base_explicit` gives a proto-partial root its own declaration
(`p6_variables/coreaction_cleanup.rs`), with `arraycoverwidth` rendering the two
halves as `._0_8_` / `._8_8_`.

The signed sibling is untouched by all of this. `IMUL rm64` lowers through
`sext` (`ia.sinc:3604`), `CPUI_INT_SEXT` is not in `RulePieceStructure`'s
`get_op_list`, and kuna has always printed `SUB168(SEXT816(x) * SEXT816(y),8)`
for it — the exact rendering the unsigned form should have.

## The options considered

(a) **Decline the structuring for this shape** (shipped). Nothing is rewritten:
the extension stays an `INT_ZEXT`, its output stays an implied varnode, and the
printer renders `SUB168(ZEXT816(x) * ZEXT816(y),8)` — a form the emitter
already produces for `IMUL`. No op is created, moved or deleted, so both halves
of the product remain the same SUBPIECEs of the same product.

(b) **Port upstream's `coreaction.cc:3113-3125` checks into `base_explicit`'s
addr-tied ZEXT/PIECE arms.** Rejected as a fix for *this*: those arms are about
addr-tied Varnodes, and these operands live in the unique space. Upstream prints
the same blobs, so there is no upstream behaviour to inherit here.

(c) **Print the product as `(unsigned __int128)x * y`.** Rejected: kuna has no
128-bit integer type, and inventing one at the printer would make a rendering
that no type in the model backs.

## Scope of the decline

The same rewrite is load-bearing wherever a concatenation really does build a
structure (a struct assembled in registers, a stack composite written in
pieces). `declines_zext` therefore requires all of:

* the output is in the unique space, and is not addr-tied, mapped, persistent or
  already a proto-partial;
* it is wider than 8 bytes and no wider than 16;
* its data-type is exactly the width fallback: an array of that many one-byte
  `undefined` elements;
* no local Symbol covers its address (which is what would have given
  `determine_datatype` a real structured type);
* every reader is a same-width integer arithmetic op whose own output is read
  only through `SUBPIECE`.
