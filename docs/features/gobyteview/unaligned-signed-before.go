func read_views(a int16, b int32) uint64 {
  var frame Wrap // ax | di | edx | esi | stack - 0x11 | stack - 0x17 | stack - 0x18
  
  frame.payload._1_2_ = uint16(a)
  frame.payload._7_4_ = uint32(b)
  consume(&frame)
  return uint64(frame.payload._1_2_) ^ uint64(frame.payload._7_4_)
}
