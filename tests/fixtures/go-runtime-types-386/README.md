# Go 1.27 runtime type region on i386

`go1.27.2-386-stripped` is a Linux i386 ELF built from the adjacent authored
source with Go 1.27.2. Its runtime type region includes array, channel,
function, map, pointer, slice and struct descriptor wrappers. The source
exercises a recursively typed node and a padded record with this mix of
fields. The regression harness reads the image as data.

Rebuild from this directory with the same compiler.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=386 GOTOOLCHAIN=local go build -trimpath \
  -ldflags '-s -w' -o go1.27.2-386-stripped main.go
chmod 0644 go1.27.2-386-stripped
```
