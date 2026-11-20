package main

func store(a, b uint64)

var sink uint64

//go:noinline
func callStore(a, b uint64) {
	store(a, b)
	store(b, a)
}

//go:noinline
func abi0(n uint64) uint64 {
	return n ^ 13
}

func main() {
	callStore(3, 5)
	sink = abi0(sink)
}
