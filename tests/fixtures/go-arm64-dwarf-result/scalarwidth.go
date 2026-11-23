package main

var i32 int32
var i64 int64

//go:noinline
func Rise(p *int32) int32 { return *p + 1 }

//go:noinline
func Fall(p *int32) int32 { return *p - 1 }

//go:noinline
func RiseWide(p *int32) int64 { return int64(*p) + 1 }

//go:noinline
func FallWide(p *int32) int64 { return int64(*p) - 1 }

func main() {
    i32 = Rise(&i32) + Fall(&i32)
    i64 = RiseWide(&i32) + FallWide(&i32)
}
