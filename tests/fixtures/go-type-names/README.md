# go-type-names

Three stripped Linux amd64 builds of `main.go` for the `kuna-gotypenames` stage
test: `main.mk` allocates a `main.Point`, `main.main` converts the `*main.Point`
to the `main.Shape` interface, and the runtime packages linked in supply
sub-package types (`internal/bisect.parseError`, `internal/poll.errNetClosing`).

`go1.16.15-amd64` is the oldest name-record encoding the harvest decodes (a
big-endian `u16` length, `itablinks` slice); `go1.17.13-amd64` has the varint
names but still the packed `offset << 1 | embedded` struct fields and the
pointer-form build info; `go1.26.2-amd64` is the descriptor-region layout. All
three are reproducible byte for byte:

```sh
cd tests/fixtures/go-type-names
CGO_ENABLED=0 go1.16.15 build -trimpath -ldflags='-s -w -buildid=' -o go1.16.15-amd64 .
CGO_ENABLED=0 go1.17.13 build -trimpath -ldflags='-s -w -buildid=' -o go1.17.13-amd64 .
CGO_ENABLED=0 go1.26.2  build -trimpath -ldflags='-s -w -buildid=' -o go1.26.2-amd64 .
```
