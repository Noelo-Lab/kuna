package main
import("encoding/binary";"unsafe")
func read_views(a uint16, b uint32) uint64 {
  var frame Wrap // ax | di | edx | esi | stack - 0x11 | stack - 0x17 | stack - 0x18
  
  *(*uint16)(unsafe.Add(unsafe.Pointer(&frame.payload),1)) = a
  *(*uint32)(unsafe.Add(unsafe.Pointer(&frame.payload),7)) = b
  consume(&frame)
  return ZEXT28(*(*uint16)(unsafe.Add(unsafe.Pointer(&frame.payload),1))) ^ ZEXT48(*(*uint32)(unsafe.Add(unsafe.Pointer(&frame.payload),7)))
}

type Wrap struct { payload [16]byte }
var wantA uint16
var wantB uint32
func consume(p *Wrap) {
  if binary.LittleEndian.Uint16(p.payload[1:]) != wantA ||
     binary.LittleEndian.Uint32(p.payload[7:]) != wantB { panic("stored words") }
  for _, i := range []int{0,3,4,5,6,11,12,13,14,15} {
    if p.payload[i] != 0 { panic("store escaped its width") }
  }
}
func ZEXT28(x uint16) uint64 { return uint64(x) }
func ZEXT48(x uint32) uint64 { return uint64(x) }
func main() {
  for _, a := range []uint16{0,1,0x1234,0x8000,0xffff} {
    for _, b := range []uint32{0,1,0x12345678,0x80000000,0xffffffff} {
      wantA, wantB = a,b
      if read_views(a,b) != uint64(a)^uint64(b) { panic("read words") }
    }
  }
}
