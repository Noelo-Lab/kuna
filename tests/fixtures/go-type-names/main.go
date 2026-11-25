package main

import "os"

type Point struct{ X, Y int }

type Shape interface{ Area() int }

func (p *Point) Area() int { return p.X * p.Y }

var sink Shape

//go:noinline
func mk(x, y int) *Point { return &Point{x, y} }

//go:noinline
func use(s Shape) int { return s.Area() }

func main() {
	p := mk(len(os.Args), 3)
	sink = p
	os.Exit(use(p))
}
