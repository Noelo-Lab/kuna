# Go 1.26 runtime type links on i386

The adjacent authored source is built with Go 1.26.0. Its recursive node
and padded record sit in the type graph reached from the four-byte
typelinks entries. The stage harness opens the ELF as data.

```sh
CGO_ENABLED=0 GOOS=linux GOARCH=386 GOTOOLCHAIN=local go build -trimpath \
  -ldflags '-s -w' -o go1.26.0-386-stripped main.go
chmod 0644 go1.26.0-386-stripped
```
