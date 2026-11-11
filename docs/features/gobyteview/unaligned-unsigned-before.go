func read_views(a uint16, b uint32) uint64 {
  var frame Wrap // ax | di | edx | esi | stack - 0x11 | stack - 0x17 | stack - 0x18
  
  frame.payload._1_2_ = a
  frame.payload._7_4_ = b
  consume(&frame)
  return ZEXT28(frame.payload._1_2_) ^ ZEXT48(frame.payload._7_4_)
}
