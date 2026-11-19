# Go AArch64 signed result width

`scalarwidth.go` contains two pairs with the same instruction bytes. The Go
DWARF formals distinguish the 32-bit and 64-bit results. Both Linux/AArch64
images use Go 1.27.2. The stage cases check the result widths under
recovery, inference, and an explicit prototype.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -trimpath -o arm64.elf scalarwidth.go
CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -trimpath -ldflags=-compressdwarf=false -o arm64-plain.elf scalarwidth.go
```
