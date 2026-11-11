package main

//go:noinline
func pair(x int64) (int64, int64) { return x, x+1 }
var first, second int64
func main() { first,second=pair(41) }
