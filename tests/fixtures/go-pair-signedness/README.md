# Go pair result signedness

Build this Linux AMD64 fixture with Go 1.22.10.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 GOTOOLCHAIN=local \
  go build -trimpath -o linux-amd64 pair.go
chmod a-x linux-amd64
```

The stage harness reads the ELF as data. Retype changes signedness without
reordering either result from Mixed. MergeRetype takes that pair on one
branch and receives an already correctly typed pair from Reorder on the other.
