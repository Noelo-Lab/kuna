package record

//go:noinline
func TwoSpills(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, x, y int64) int64 {
	return x*7 + y
}

//go:noinline
func LastRegister(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, x, y int64) int64 {
	return x*7 + y
}
