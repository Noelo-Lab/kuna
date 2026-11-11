func read_views(a uint16, b uint32) uint64 {
  var frame Wrap // ax | di | edx | esi | stack - 0x11 | stack - 0x17 | stack - 0x18
  
  *(*uint16)(unsafe.Add(unsafe.Pointer(&frame.payload),1)) = a
  *(*uint32)(unsafe.Add(unsafe.Pointer(&frame.payload),7)) = b
  consume(&frame)
  return ZEXT28(*(*uint16)(unsafe.Add(unsafe.Pointer(&frame.payload),1))) ^ ZEXT48(*(*uint32)(unsafe.Add(unsafe.Pointer(&frame.payload),7)))
}
