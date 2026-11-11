// main.shift8
func shift8(a int64, b byte) int64 {
  return a + int64(int8(b >> 1)) * 8 + 64
}

// main.sign8
func sign8(a int64, b byte) int64 {
  return a + int64(int8(b)) * 8 + 64
}

// main.sign16
func sign16(a int64, b uint16) int64 {
  return a + int64(int16(b)) * 8 + 64
}

// main.sign32
func sign32(a int64, b uint32) int64 {
  return a + int64(int32(b)) * 8 + 64
}

// main.zero8
func zero8(a int64, b byte) int64 {
  return int64(uint64(a) + uint64(b) * 0x8 + 0x40)
}

// main.signToUnsigned
func signToUnsigned(b byte) uint64 {
  return uint64(int64(int8(b)) * 8) + 0x40
}

// main.narrowSum
func narrowSum(a int64, b uint16, c uint16) int64 {
  return a + int64(int16(c + b)) * 8 + 64
}

// main.narrowProduct
func narrowProduct(a int64, b uint32, c uint32) int64 {
  return a + int64(int32(b * c))
}

// main.lessDelta
func lessDelta(a uint64, b uint64) uint64 {
  return a - func(value bool) uint64 { if value { return 1 }; return 0 }(a < b)
}

// main.lessDelta32
func lessDelta32(a uint32, b uint32) uint32 {
  return a - func(value bool) uint32 { if value { return 1 }; return 0 }(a < b)
}
