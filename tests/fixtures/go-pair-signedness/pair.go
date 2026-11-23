package main

var keep [2]uint64

//go:noinline
func Mixed(a int64, b uint64, pick bool) (int64, uint64) {
    if pick {
        return a - 3, b ^ (1 << 63)
    }
    return int64(b), uint64(a + 5)
}

//go:noinline
func Named(a int64, b uint64, pick bool) (difference int64, mask uint64) {
    difference, mask = Mixed(a, b, pick)
    if pick {
        mask += uint64(difference)
    } else {
        difference -= int64(mask)
    }
    return
}

//go:noinline
func Choose(a int64, b uint64, pick bool) (int64, uint64) {
    var signed int64
    var bits uint64
    if pick {
        signed, bits = Mixed(a, b, false)
    } else {
        signed, bits = Mixed(a, b, true)
    }
    return signed, bits
}

//go:noinline
func First(a int64, b uint64, pick bool) int64 {
    result, _ := Mixed(a, b, pick)
    return result - 17
}

//go:noinline
func Second(a int64, b uint64, pick bool) uint64 {
    _, result := Mixed(a, b, pick)
    return result ^ 31
}

//go:noinline
func Reuse(a int64, b uint64, pick bool) (int64, uint64) {
    signed, bits := Mixed(a, b, pick)
    if pick {
        x, y := Mixed(signed, bits, !pick)
        return x, y ^ bits
    }
    return signed, bits
}

//go:noinline
func Retype(a int64, b uint64, pick bool) (uint64, int64) {
    x, y := Mixed(a, b, pick)
    return uint64(x), int64(y)
}

//go:noinline
func Reorder(a int64, b uint64, pick bool) (uint64, int64) {
    x, y := Mixed(a, b, pick)
    return y, x
}

//go:noinline
func MergeRetype(a int64, b uint64, pick bool) (uint64, int64) {
    var x uint64
    var y int64
    if pick {
        signed, bits := Mixed(a, b, pick)
        x, y = uint64(signed), int64(bits)
    } else {
        x, y = Reorder(a, b, pick)
    }
    return x, y
}

func main() {
    a, b := Named(-5, 1<<63, true)
    c, d := Choose(-7, 1<<60, false)
    e, f := Reuse(-11, 3, true)
    keep[0] = uint64(a + c + e + First(-9, 1<<63, false))
    keep[1] = b + d + f + Second(-17, 1<<63, true)
    g, h := Retype(-23, 1<<61, false)
    i, j := Reorder(-29, 1<<60, true)
    k, l := MergeRetype(-31, 1<<62, false)
    keep[0] ^= g + i + k
    keep[1] ^= uint64(h + j + l)
}
