package main

//go:noinline
func shift8(a int64, b uint8) int64 { return a + int64(int8(b>>1))*8 + 64 }

//go:noinline
func sign8(a int64, b uint8) int64 { return a + int64(int8(b))*8 + 64 }

//go:noinline
func sign16(a int64, b uint16) int64 { return a + int64(int16(b))*8 + 64 }

//go:noinline
func sign32(a int64, b uint32) int64 { return a + int64(int32(b))*8 + 64 }

//go:noinline
func zero8(a int64, b uint8) int64 { return a + int64(b)*8 + 64 }

//go:noinline
func signToUnsigned(b uint8) uint64 { return uint64(int64(int8(b)))*8 + 64 }

//go:noinline
func narrowSum(a int64, b, c uint16) int64 { return a + int64(int16(b+c))*8 + 64 }

//go:noinline
func narrowProduct(a int64, b, c uint32) int64 { return a + int64(int32(b*c)) }

//go:noinline
func lessDelta(a, b uint64) uint64 {
	var n uint64
	if a < b {
		n = 1
	}
	return a - n
}

//go:noinline
func lessDelta32(a, b uint32) uint32 {
	var n uint32
	if a < b {
		n = 1
	}
	return a - n
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
