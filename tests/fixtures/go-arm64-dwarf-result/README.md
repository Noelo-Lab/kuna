# Go AArch64 signed result width

`scalarwidth.go` contains two pairs with the same instruction bytes. The Go
DWARF formals distinguish the 32-bit and 64-bit results. The Linux/AArch64
image uses Go 1.27.2's default zlib-compressed DWARF. The stage checks its
result widths with recovery enabled, disabled, and with an explicit prototype.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -trimpath -o arm64.elf scalarwidth.go
```
