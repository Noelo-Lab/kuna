# Preserve mapped global array bases

An explicitly declared array base can disappear when additive simplification
combines it with a negative element displacement before constant-pointer
recovery. The numeric address remains correct, but subsequent recovery sees an
address before the array rather than its declared base. It may retain a numeric
address or select a neighboring object's type.

`globalarrayanchor` defers those additions while constant-pointer recovery can
still recognize the exact base. It changes no p-code operands and creates no
symbols. Recovery later expresses the displacement in the index:

```
base + index * element_size - element_size
    → array[index - 1]
```

The query must find a declared array at the exact constant address in the
LOAD/STORE's byte-addressed space. The negative displacement must be a whole
number of elements and no greater than the array's declared size. Every variable
coefficient must be an element-size multiple. No nearby-symbol search is made.
Sharing, pointer-typed variable terms, ambiguous pointer spaces, incompatible
widths and constants already declined by pointer recovery all reject the guard.

The guard is active only with type recovery and a scheduled constant-pointer
pass, before its fourth application. This prevents holding an addition forever
when pointer recovery cannot use the base. Ordinary arithmetic, unmapped
addresses and unrelated constant-only expressions still fold. Same-width copies
and casts may connect the address tree; unused pure readers do not count as
sharing, but auto-live and indirect-source readers do.

The catalog default is OFF. `aggressive` enables the option, and size-selected
`auto` consequently uses it on small inputs. `reliable` retains the catalog
default. An explicit `--option globalarrayanchor off` after mode selection
restores normal folding.

SPARC32 and AArch64 stages distinguish disabled and enabled LOAD/STORE output,
including multiple indices, byte arrays, different element widths and nearby
objects. Direct IR tests exercise recovery limits and sharing. Independently
compiled O0/O2 controls provide output comparisons across twelve architectures;
only cases with an exact base reaching the engine are positive trigger coverage.
