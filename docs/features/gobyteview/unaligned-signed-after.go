func read_views(a int16, b int32) uint64 {
  var frame Wrap // ax | di | edx | esi | stack - 0x11 | stack - 0x17 | stack - 0x18
  
  *(*uint16)(unsafe.Add(unsafe.Pointer(&frame.payload),1)) = uint16(a)
  *(*uint32)(unsafe.Add(unsafe.Pointer(&frame.payload),7)) = uint32(b)
  consume(&frame)
  return uint64(*(*uint16)(unsafe.Add(unsafe.Pointer(&frame.payload),1))) ^ uint64(*(*uint32)(unsafe.Add(unsafe.Pointer(&frame.payload),7)))
}
