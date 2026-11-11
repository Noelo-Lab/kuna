package main
// main.shift8
func shift8(a int64, b byte) int64 {
  return a + int8(b >> 1) * 8 + 64
}

// main.sign8
func sign8(a int64, b byte) int64 {
  return a + int8(b) * 8 + 64
}

// main.sign16
func sign16(a int64, b uint16) int64 {
  return a + int16(b) * 8 + 64
}

// main.sign32
func sign32(a int64, b uint32) int64 {
  return a + int32(b) * 8 + 64
}

// main.zero8
func zero8(a int64, b byte) int64 {
  return int64(uint64(a) + uint64(b) * 0x8 + 0x40)
}

// main.signToUnsigned
func signToUnsigned(b byte) uint64 {
  return uint64(int8(b) * 8) + 0x40
}

// main.narrowSum
func narrowSum(a int64, b uint16, c uint16) int64 {
  return a + int16(c + b) * 8 + 64
}

// main.narrowProduct
func narrowProduct(a int64, b uint32, c uint32) int64 {
  return a + int32(b * c)
}

// main.lessDelta
func lessDelta(a uint64, b uint64) uint64 {
  return a - (a < b)
}

// main.lessDelta32
func lessDelta32(a uint32, b uint32) uint32 {
  return a - (a < b)
}

func main() {
	inputs := make([]uint64, 256)
	for i := range inputs {
		inputs[i] = uint64(i)
	}
	inputs = append(inputs, 0x7fff, 0x8000, 0xffff, 0x7fffffff, 0x80000000, 0xffffffff, 0xffffffffffffffff)
	for _, a := range []int64{0, 1, -1, -17, 0x7fffffffffffffff, -0x8000000000000000} {
		for _, b := range inputs {
			println(shift8(a, uint8(b)), sign8(a, uint8(b)), sign16(a, uint16(b)), sign32(a, uint32(b)), zero8(a, uint8(b)), signToUnsigned(uint8(b)), narrowSum(a, uint16(b), 65521), narrowProduct(a, uint32(b), 0x10007), lessDelta(uint64(a), b), lessDelta32(uint32(a), uint32(b)))
		}
	}
}
