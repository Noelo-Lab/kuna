package main
import ("encoding/binary";"unsafe")
func write_pair(ctx uint64) {
  var v1 *uint64 // rcx | stack - 0x8
  var v2 [16]byte // rbx:rax
  
  v1 = *(**uint64)(unsafe.Pointer(uintptr(ctx + 0x8)))
  v2 = pair(*(*uint64)(unsafe.Pointer(uintptr(ctx))))
  *v1 = *(*uint64)(unsafe.Add(unsafe.Pointer(&v2),0))
  *(*uint64)(unsafe.Add(unsafe.Pointer(v1),1 * 8)) = *(*uint64)(unsafe.Add(unsafe.Pointer(&v2),8))
  return
}

var calls int
var context [2]uint64
var storage [4]uint64
func pair(x uint64) [16]byte {
  calls++
  var result [16]byte
  binary.LittleEndian.PutUint64(result[:8],x)
  binary.LittleEndian.PutUint64(result[8:],x+1)
  return result
}
func main() {
  for _,x:=range []uint64{0,1,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffef,0xffffffffffffffff} {
    storage=[4]uint64{0x1234,0,0,0x5678}
    context=[2]uint64{x,uint64(uintptr(unsafe.Pointer(&storage[1])))}
    before:=calls
    write_pair(uint64(uintptr(unsafe.Pointer(&context))))
    if storage[1]!=x || storage[2]!=x+1 || storage[0]!=0x1234 || storage[3]!=0x5678 || calls!=before+1 {panic("stored result words")}
  }
}
