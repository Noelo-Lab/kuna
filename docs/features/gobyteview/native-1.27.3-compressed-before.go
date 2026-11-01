// main.pair
func pair(a0 int64) [16]byte {
  var v1 [16]byte // rbx:rax
  
  v1._8_8_ = uint64(a0 + 1)
  v1._0_8_ = uint64(a0)
  return v1._0_16_
}
