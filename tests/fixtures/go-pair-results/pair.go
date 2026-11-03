package main

import (
    "fmt"
    "os"
    "strconv"
)

var calls uint64

//go:noinline
func pair(a, b int64) (int64, int64) { return a + b, a - b }

//go:noinline
func divmod(a, b int64) (int64, int64) { return a / b, a % b }

//go:noinline
func unsignedPair(a, b uint64) (uint64, uint64) { return a + b, a ^ b }

//go:noinline
func branchPair(a, b int64) (int64, int64) {
    if a < b { return b, a }
    return a, b
}

//go:noinline
func consume(a, b int64) int64 { x, y := pair(a, b); return x ^ y }

//go:noinline
func first(a, b int64) int64 { x, _ := pair(a, b); return x }

//go:noinline
func second(a, b int64) int64 { _, y := pair(a, b); return y }

//go:noinline
func forward(a, b int64) (int64, int64) { return pair(a, b) }

//go:noinline
func effect(a int64) (int64, int64) { calls++; return a, a + 1 }

//go:noinline
func consumeEffect(a int64) int64 { x, y := effect(a); return x ^ y }

//go:noinline
func narrow(a uint32) (uint32, uint32) { return a, a + 1 }

//go:noinline
func floats(a float64) (float64, float64) { return a, -a }

//go:noinline
func mixed(a int64) (int64, float64) { return a, float64(a) }

//go:noinline
func triple(a int64) (int64, int64, int64) { return a, a + 1, a + 2 }

type ordinaryPair struct { First, Second int64 }

//go:noinline
func aggregate(a int64) ordinaryPair { return ordinaryPair{a, a + 1} }

//go:noinline
func zeroPair()(uint64,uint64){return 0,0}
//go:noinline
func lowPair(a uint64)(uint64,uint64){return a,0}
//go:noinline
func signedLowPair(a int64)(int64,int64){return a,0}
//go:noinline
func highPair(a uint64)(uint64,uint64){return 0,a}
//go:noinline
func onesPair()(int64,int64){return -1,-1}
//go:noinline
func choosePair(a,b uint64)(uint64,uint64){if a==0{return 0,0};return a,b}

func main() {
    a, _ := strconv.ParseInt(os.Args[1], 10, 64)
    b, _ := strconv.ParseInt(os.Args[2], 10, 64)
    x, y := pair(a, b); fmt.Println(x, y)
    x, y = branchPair(a, b); fmt.Println(x, y)
    x, y = forward(a, b); fmt.Println(x, y)
    u, v := unsignedPair(uint64(a), uint64(b)); fmt.Println(u, v)
    fmt.Println(consume(a, b), first(a, b), second(a, b))
    calls = 0; e := consumeEffect(a); fmt.Println(e, calls)
    if b != 0 { x, y = divmod(a, b); fmt.Println(x, y) }
    n, m := narrow(uint32(a)); fmt.Println(n, m)
    f, g := floats(float64(a)); fmt.Println(f, g)
    x, f = mixed(a); fmt.Println(x, f)
    x, y, z := triple(a); fmt.Println(x, y, z)
    fmt.Println(aggregate(a))
    fmt.Println(zeroPair()); fmt.Println(lowPair(uint64(a))); fmt.Println(signedLowPair(a))
    fmt.Println(highPair(uint64(a))); fmt.Println(onesPair()); fmt.Println(choosePair(uint64(a),uint64(b)))
}
