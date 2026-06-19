# WS3 — compiler-side p-code actions (`SleighPcode` + `MacroBuilder`)

Ports the two compiler-only classes that `slgh_compile.{cc,hh}` layer on top of the
already-ported `PcodeCompile` trait (`kuna-sleigh/src/pcodecompile.rs`):

| C++ class      | C++ anchor                              | Rust |
|----------------|-----------------------------------------|------|
| `SleighPcode`  | `slgh_compile.hh:282-292`, cc:1930-1958 | `pcodecompile_actions::SleighPcode` |
| `MacroBuilder` | `slgh_compile.hh:256-275`, cc:1785-1928 | `pcodecompile_actions::MacroBuilder` |

Module: `rust/crates/kuna-slacomp/src/pcodecompile_actions.rs` (WS3-owned, file-disjoint).

## The ownership split — the `CompilerHost` freeze interface (for WS4)

In C++ both classes hold a back-pointer to `SleighCompile` (`compiler` / `slgh`) and
call back into it.  The Rust port abstracts that back-pointer behind a trait the WS4
driver implements, so WS3 stays file-disjoint:

```rust
pub trait CompilerHost {
    fn get_unique_addr(&mut self) -> u32;          // SleighCompile::getUniqueAddr (cc:2465)
    fn get_unique_space(&self) -> Rc<AddrSpace>;   // getUniqueSpace
    fn get_constant_space(&self) -> Rc<AddrSpace>; // getConstantSpace
    fn get_location(&self, symbol_name: &[u8]) -> Option<Location>; // getLocation (cc:3119)
    fn add_symbol(&mut self, sym: PcodeCompileSymbol);              // addSymbol  (cc:1954)
    fn report_error(&mut self, loc: Option<&Location>, msg: &str);  // reportError (cc:1942)
    fn report_warning(&mut self, loc: Option<&Location>, msg: &str);// reportWarning (cc:1948)
}
```

**WS4 action:** `SleighCompile` `impl CompilerHost`.  `get_unique_addr` is exactly
`getUniqueBase(); setUniqueBase(base + SleighBase::MAX_UNIQUE_SIZE); return base;`
(cc:2465-2470). `get_location` is keyed by symbol *name* in the port (matching the
already-ported `PcodeCompile::get_location` convention), not a `SleighSymbol *`.

### Method-set frozen for WS3's classes

- `SleighPcode`: `new`, `allocate_temp(host)`, `get_location(host, name)`,
  `report_error(host, loc, msg)`, `report_warning(host, loc, msg)`,
  `add_symbol(host, sym)` — all thin routers to `CompilerHost` (cc:1930-1958).
- `MacroBuilder<'a>`: `new(host, outvec, labelbase)`, `set_macro_op(macroop)`,
  `has_error()`, private `report_error`/`transfer_op`, and the `PcodeBuilder` impl
  (`dump`/`append_build`/`delay_slot`/`set_label`/`append_cross_build` + the four
  `labelbase`/`labelcount` accessors).  `build(...)` is the inherited trait dispatch
  (`semantics.rs`).

## Faithful-port notes

- `setMacroOp` (cc:1809): inputs `1..numInput()` of the MACROBUILD op become the
  `HandleTpl` params (input 0 is the macro index); `free()` → `params.clear()`.
- `transferOp` (cc:1833): the C++ passes `params` explicitly but always as
  `this->params`; the port reads `self.params`.  The truncation path (`plus >= 0`
  after `VarnodeTpl::transfer`) inserts a `CPUI_SUBPIECE` writing a fresh
  `getUniqueAddr()` temp, reading the substituted handle, with a constant-space
  `plus` input of size 4, then rewrites the original input to the subpiece output.
  C++ `delete`s the clone on failure; in Rust `transfer_op` consumes the owned clone
  (no leak / no double-push).
- `dump` (cc:1886): clone op, fix relative (label) operands by `+labelbase`, then
  `transferOp`.  `set_label` (cc:1912): clone the label op, adjust the label index by
  `+labelbase`, push (no parameter substitution).
- `appendBuild`/`delaySlot`/`appendCrossBuild` all forward to `dump` (header inlines).
- The `PcodeBuilder::build` directive dispatch (BUILD=MULTIEQUAL, DELAY_SLOT=INDIRECT,
  LABELBUILD=PTRADD, CROSSBUILD=PTRSUB) is the already-ported trait method.

## Golden verification

Instrumented /tmp copy `decompiler/cpp/**` (vendored tree byte-untouched).  Dumper
`/tmp/kunacpp/macrodump.cc` (regenerable):

1. `cp -r decompiler/cpp /tmp/kunacpp`
2. make `predefinedSymbols()` public in `/tmp/kunacpp/slgh_compile.hh` and rename the
   binary's `int main` → `int slgh_main` (so the dumper provides `main`).
3. compile the `CORE + SLEIGH + SLACOMP` object set (no libbfd needed), link the
   dumper, run → `tests/golden/macrobuilder_expand.txt`.

The dumper builds a representative macro body (`ConstructTpl` of `OpTpl`s referencing
macro handle-parameters) and a MACROBUILD invocation supplying two handle args, runs
the **real** C++ `MacroBuilder::build`, and dumps the op-template list before and
after expansion.  It exercises **every** MacroBuilder path: plain handle
substitution, `setLabel` index `+labelbase`, `dump` relative-operand `+labelbase`,
and the truncation → `CPUI_SUBPIECE` insertion (`getUniqueAddr`/`getUniqueSpace`/
`getConstantSpace`).

Rust test `tests/macrobuilder_golden.rs` builds the identical templates, runs the
Rust `MacroBuilder`, and asserts a **byte-identical** dump.  Plus three in-module unit
tests: `SleighPcode` hook routing, `setMacroOp` param collection, and the illegal-
bitrange truncation error.

No upstream `TEST()` covers these classes, so none to port 1:1.
