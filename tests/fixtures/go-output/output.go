package main
import "unsafe"
//go:noinline
func add(a,b int64) int64 {return a+b}
//go:noinline
func choose(a,b int64) int64 {if a>10{return b+1};if a<0{return b-1};return b*2}
//go:noinline
func sumFor(n int64) int64 {var sum int64;for i:=int64(0);i<n;i++{if i&1==0{continue};sum+=i};return sum}
//go:noinline
func shortCalls(a,b int64) int64 {if a>0 && add(a,b)>4{return 7};return 2}
//go:noinline
func loopCalls(a,b int64) int64 {var sum int64;for a>0 && add(a,b)>4{sum+=a;a--};return sum}
//go:noinline
func dispatch(n,a int64) int64 {switch n {case 0:a+=3;fallthrough;case 1:a*=2;case 2,3:a-=4;case 4:a^=7;case 5:a+=8;default:a=-a};return a}
//go:noinline
func readAt(p *int32,i int64) int32 {return *(*int32)(unsafe.Add(unsafe.Pointer(p),i*4))}
//go:noinline
func writeAt(p *int32,i int64,v int32) {*(*int32)(unsafe.Add(unsafe.Pointer(p),i*4))=v}
//go:noinline
func fromWord(p uintptr) *int32 {return (*int32)(unsafe.Pointer(p))}
//go:noinline
func toWord(p *int32) uintptr {return uintptr(unsafe.Pointer(p))}
//go:noinline
func retype(p *float64) *int32 {return (*int32)(unsafe.Pointer(p))}
//go:noinline
func narrow(a uint64) uint32 {return uint32(a)}
//go:noinline
func extend(a int8) int64 {return int64(a)}
//go:noinline
func number(a int64) float64 {return float64(a)}
func main(){xs:=[4]int32{1,2,3,4};writeAt(&xs[0],2,17);println(add(2,3),choose(11,2),sumFor(9),shortCalls(2,3),loopCalls(9,-1),dispatch(0,2),readAt(&xs[0],2),fromWord(toWord(&xs[0])),retype(nil),narrow(0x123456789),extend(-3),number(7))}
