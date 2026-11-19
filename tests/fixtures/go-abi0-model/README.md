This fixture has a Go caller and an assembly function with ABI0 stack
parameters. The ordinary register-ABI function named `abi0` checks name
disambiguation. Build the Linux AMD64 image with Go 1.18.1, leaving the
symbol table in place and removing DWARF.

```sh
cd tests/fixtures/go-abi0-model
GOOS=linux GOARCH=amd64 CGO_ENABLED=0 GO111MODULE=off \
  go build -trimpath -ldflags='-w -buildid=' -o linux-amd64 .
```
