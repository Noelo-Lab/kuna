package main

import "unsafe"

var sink [7]uint64

//go:noinline
func Length(s string) int64 {
    return int64(len(s))
}

//go:noinline
func ByteOr(s string, i int64, fallback byte) byte {
    if uint64(i) >= uint64(len(s)) {
        return fallback
    }
    return s[i]
}

//go:noinline
func Forward(s string, i int64, fallback byte) byte {
    return ByteOr(s, i, fallback)
}

//go:noinline
func FromPrefix(s string, prefix int64, i int64, fallback byte) byte {
    if uint64(prefix) > uint64(len(s)) {
        return fallback
    }
    return ByteOr(s[:prefix], i, fallback)
}

//go:noinline
func FromOther(s string, i int64, fallback byte) byte {
    if len(s) == 0 {
        return fallback
    }
    return ByteOr("other", i, fallback)
}

//go:noinline
func HeaderData(s string) uintptr {
    return uintptr(unsafe.Pointer(unsafe.StringData(s))) ^ uintptr(len(s))
}

//go:noinline
func UnsafeFirst(s string) byte {
    return *unsafe.StringData(s)
}

//go:noinline
func ExplicitWords(data *byte, size int64, i int64, fallback byte) byte {
    if uint64(i) >= uint64(size) {
        return fallback
    }
    return *(*byte)(unsafe.Add(unsafe.Pointer(data), i))
}

func main() {
    const text = "x\xc3\xa9\x00\xff"
    sink[0] = uint64(Length(text))
    sink[1] = uint64(Forward(text, 2, 91))
    sink[2] = uint64(FromPrefix(text, 3, 2, 91))
    sink[3] = uint64(HeaderData(text))
    sink[4] = uint64(UnsafeFirst(text))
    sink[5] = uint64(ExplicitWords(unsafe.StringData(text), int64(len(text)), 2, 91))
    sink[6] = uint64(FromOther(text, 2, 91))
}
