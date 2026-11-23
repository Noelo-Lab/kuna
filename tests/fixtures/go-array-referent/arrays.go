package main

import "unsafe"

var r byte
var w uint16
var q *byte

//go:noinline
func Read(p *[16]byte, i uint64) byte { return p[i&15] }

//go:noinline
func Write(p *[16]byte, i uint64, value byte) { p[i&15] = value }

//go:noinline
func ReadWord(p *[8]uint16, i uint64) uint16 { return p[i&7] }

//go:noinline
func Indirect(slot **[16]byte, i uint64) byte { return (*slot)[i&15] }

//go:noinline
func PointerValue(p *[16]*byte, i uint64) *byte { return p[i&15] }

//go:noinline
func TruePointerLoad(slot **byte, i uint64) byte {
    return *(*byte)(unsafe.Add(unsafe.Pointer(*slot), i&15))
}

func main() {
    a := [16]byte{1}
    p := &a
    b := [8]uint16{321}
    m := [16]*byte{&a[1]}
    n := &a[0]
    Write(&a, 3, 127)
    r = Read(&a,3) + Indirect(&p,3) + TruePointerLoad(&n,3)
    w = ReadWord(&b,0)
    q = PointerValue(&m,0)
}
