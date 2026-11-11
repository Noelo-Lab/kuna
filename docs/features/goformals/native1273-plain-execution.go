package main
import "unsafe"
// main.add
func add(a int64, b int64) int64 {
  return a + b
}
// main.choose
func choose(a int64, b int64) int64 {
  if 10 < a {
    return b + 1
  }
  if a <= -1 {
    return b + -1
  }
  return b * 2
}
// main.sumFor
func sumFor(n int64) int64 {
  var v1 int64 // rbx | rdx
  var v2 int64 // rcx
  var v3 int64 // rbx | rdx
  
  v3 = 0
  for v2 = 0; v2 < n; v2 = int64(uint64(v2) + 0x1) {
    v1 = int64(uint64(v3) + uint64(v2))
    if uint64(v2) & 0x1 == 0x0 {
      v1 = v3
    }
    v3 = v1
  }
  return v3
}
// main.shortCalls
func shortCalls(a int64, b int64) int64 {
  if 1 <= a {
    if 4 < add(a,b) {
      return 7
    }
  }
  return 2
}
// main.loopCalls
func loopCalls(a int64, b int64) int64 {
  var v1 int64 // rcx | rdx | stack - 0x18
  
  v1 = 0
  for {
    if !((1 <= a && (5 <= add(a,b)))) {
      break
    }
    v1 = v1 + a
    a = a + -1
  }
  return v1
}
// main.dispatch
func dispatch(n int64, a int64) int64 {
  if 2 <= n { // branch-flip
    if n <= 3 {
      return a + -4
    }
    if uint64(n) == 0x4 {
      return int64(uint64(a) ^ 0x7)
    }
    if uint64(n) == 0x5 {
      return a + 8
    }
    a = -a
  } else {
    if uint64(n) != 0x0 { // branch-flip
      if uint64(n) != 0x1 {
        a = -a
        return a
      }
    } else {
      a = a + 3
    }
    a = a * 2
  }
  return a
}
// main.readAt
func readAt(p *int32, i int64) int32 {
  return *(*int32)(unsafe.Add(unsafe.Pointer(p),i * 4))
}
// main.writeAt
func writeAt(p *int32, i int64, v int32) {
  *(*int32)(unsafe.Add(unsafe.Pointer(p),i * 4)) = v
  return
}
// main.fromWord
func fromWord(p uint64) *int32 {
  return (*int32)(unsafe.Pointer(uintptr(p)))
}
// main.toWord
func toWord(p *int32) uint64 {
  return uint64(uintptr(unsafe.Pointer(p)))
}
// main.retype
func retype(p *float64) *int32 {
  return (*int32)(unsafe.Pointer(p))
}
// main.narrow
func narrow(a uint64) uint32 {
  return uint32(a)
}
// main.extend
func extend(a int8) int64 {
  return int64(a)
}
// main.number
func number(a int64) float64 {
  return float64(a)
}
func main(){
 xs:=[4]int32{1,2,3,4}
 writeAt(&xs[0],2,17)
 if add(2,3)!=5 || choose(11,2)!=3 || choose(-1,2)!=1 || choose(2,3)!=6 {panic("arithmetic")}
 if sumFor(9)!=16 || shortCalls(2,3)!=7 || shortCalls(0,7)!=2 || loopCalls(9,-1)!=30 {panic("loops")}
 if dispatch(0,2)!=10 || dispatch(1,2)!=4 || dispatch(2,9)!=5 || dispatch(8,2)!=-2 {panic("switch")}
 if readAt(&xs[0],2)!=17 || fromWord(toWord(&xs[0]))!=&xs[0] || retype(nil)!=nil {panic("pointers")}
 if narrow(0x123456789)!=0x23456789 || extend(-3)!=-3 || number(7)!=7 {panic("conversions")}
}
