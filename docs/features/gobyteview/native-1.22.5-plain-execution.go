package main
import("encoding/binary";"unsafe")
// main.pair
func pair(a0 int64) [16]byte {
  var v1 [16]byte // rbx:rax
  
  *(*uint64)(unsafe.Add(unsafe.Pointer(&v1),8)) = uint64(a0 + 1)
  *(*uint64)(unsafe.Add(unsafe.Pointer(&v1),0)) = uint64(a0)
  return v1
}

func main() {
 for _,x:=range []int64{0,1,-1,-17,0x7fffffffffffffff,-0x8000000000000000} {
  result:=pair(x)
  if binary.LittleEndian.Uint64(result[:8])!=uint64(x) || binary.LittleEndian.Uint64(result[8:])!=uint64(x)+1 {panic("result words")}
 }
}
