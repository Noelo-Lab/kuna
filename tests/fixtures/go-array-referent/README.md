# Go array address regression

The authored ELF covers byte and word array access, byte stores, and array
elements that are pointers. The stage states exact source prototypes. Its
two indirect controls distinguish a loaded array pointer from the pointer to
its first element and from an ordinary pointer-valued load.

Rebuild the data-only ELF from the repository root with Go 1.22.10.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -trimpath \
  -o tests/fixtures/go-array-referent/amd64 tests/fixtures/go-array-referent/arrays.go
chmod a-x tests/fixtures/go-array-referent/amd64
```
