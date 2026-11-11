package main
type point struct{x,y int64}
// main.scale
func scale(p *point, k int64) {
  (*p).x = (*p).x * k
  (*p).y = (*p).y * k
  return
}

func main(){v:=point{3,-7};scale(&v,4);if v.x!=12||v.y != -28{panic("fields")}}
