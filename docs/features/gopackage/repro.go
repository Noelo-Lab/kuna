package main
type point struct { x,y int64 }
//go:noinline
func scale(p *point, k int64) { p.x*=k; p.y*=k }
var value=point{3,-7}
func main() { scale(&value,4) }
