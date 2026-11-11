// main.pair
func pair(a0 int64) [16]byte {
  var v1 [16]byte // rbx:rax
  
  *(*uint64)(unsafe.Add(unsafe.Pointer(&v1),8)) = uint64(a0 + 1)
  *(*uint64)(unsafe.Add(unsafe.Pointer(&v1),0)) = uint64(a0)
  return v1
}

// main.combine
func combine(x int64) int64 {
  var v1 [16]byte // rbx:rax
  
  v1 = pair(x)
  return *(*int64)(unsafe.Add(unsafe.Pointer(&v1),0)) * 3 + *(*int64)(unsafe.Add(unsafe.Pointer(&v1),8))
}
