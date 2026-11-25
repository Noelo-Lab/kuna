# One-string source projection

The authored `projection.go` covers a fit-checked one-byte read on a Go string,
and calls with the unchanged pair, a shorter length and a different backing.
The other functions expose the data word or use an independent pointer and count.

Build the data-only fixture from the repository root with Go 1.22.10.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -trimpath \
  -o tests/fixtures/go-string-projection/amd64 \
  tests/fixtures/go-string-projection/projection.go
chmod a-x tests/fixtures/go-string-projection/amd64
```

The fixture keeps Go DWARF and its ordinary Go build identifier. The stage
reads it as data. The expected explicit call-site address is verified with
`go tool objdump -s 'main.Forward$' tests/fixtures/go-string-projection/amd64`.
