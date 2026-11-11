package main

import (
	"fmt"
	"os"
	"syscall"
)

type Point struct {
	X   int
	Y   int
	Tag string
}

type Envelope struct {
	Lead byte
	*Point
	Tail uint16
}

type Z struct { X int }
type A struct { Z; Tail int }

//go:noinline
func mk(i int) Point {
	return Point{X: i, Y: i * 3, Tag: "t"}
}

func main() {
	p := mk(len(os.Args))
	fmt.Println(p)
	fmt.Println(Envelope{Lead: 1, Point: &p, Tail: 2})
	fmt.Println(A{Z{3}, 4})
	fmt.Println(syscall.Iovec{})
}
