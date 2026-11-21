package main

//go:noinline
func add3(a, b, c int) int { return a*b + c }

var seed = 5

func main() { println(add3(seed, 2, 7)) }
