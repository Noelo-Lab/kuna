# AArch64 Go stack-pop regression

`recursive.S` is authored assembly for a scalar recursive Fibonacci function.
It saves the input at `SP+16` across the first recursive call and saves that
call's result at `SP+24` across the second. Each call uses X0 for its input
and result; BL/RET leaves the caller's SP unchanged.

Reproduce the stage's raw bytes without linking or executing the object:

```sh
clang --target=aarch64-linux-gnu -c recursive.S -o recursive.o
llvm-objcopy-14 -O binary --only-section=.text recursive.o recursive.text
readelf -r recursive.o
```

The object has no relocations. Its 60 text bytes are embedded in
`tests/stages/ghangr-go-aarch64-stackpop.xml`. The explicit scalar prototype
isolates stack-slot preservation from return-width inference. With the old
Go `abi-internal` extrapop of eight, the first call's result assignment is
lost and the second call reads an uninitialized stack value instead of the
saved input. Extrapop zero preserves both values. Stackshift remains eight.
