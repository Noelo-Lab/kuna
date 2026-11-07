## Before / after — `zero_unsigned` in `array_constant_x86_64`

Real captured kuna output on the function this feature was built for, with `option rustarrayzero` flipped. Ported from **angr** — Wide-store parity audit exposed invalid scalar zero assigned to a recovered integer byte array; source-native memory is the oracle..

| metric | `rustarrayzero off` *(default)* | `rustarrayzero on` |
|---|---|---|
| gotos | 0 | 0 |
| labels | 0 | 0 |
| loc | 6 | 6 |
| switches | 0 | 0 |
| speed | 114.59160363301635 ms | 115.96163595095277 ms — Δ 1.1955782749353938% (budget 5%) |

<details open><summary><b>kuna — before (<code>option rustarrayzero off</code>)</b></summary>

```c
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_unsigned(mut p: *mut [u8; 16])
{
  *p = 0;
  return;
}
```

</details>

<details open><summary><b>kuna — after (<code>option rustarrayzero on</code>)</b></summary>

```c
#[allow(non_snake_case, unused_mut, unused_variables, unused_parens, dead_code)]
unsafe fn zero_unsigned(mut p: *mut [u8; 16])
{
  *p = [0; 16];
  return;
}
```

</details>

