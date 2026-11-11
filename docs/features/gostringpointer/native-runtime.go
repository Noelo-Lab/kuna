package main
import "unsafe"
// runtime.badmorestackgsignal
func badmorestackgsignal() {
  write(0x2,unsafe.Pointer((*int8)(unsafe.Pointer(unsafe.StringData("fatal: morestack on gsignal\n\x00")))),28)
  return
}

var called bool
func write(fd uint64, p unsafe.Pointer, n int32) int32 {
 if fd!=2 || n!=28 || string(unsafe.Slice((*byte)(p),n))!="fatal: morestack on gsignal\n" {panic("bytes")};called=true;return n
}
func main(){badmorestackgsignal();if !called {panic("missing call")}}
