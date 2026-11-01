package main
type point struct {x,y int64}
// main.scale
func scale(p *point, k int64) {
  (*p).x = (*p).x * k
  (*p).y = (*p).y * k
  return
}

func main(){p:=point{3,-7};scale(&p,4);if p.x!=12 || p.y!=-28 {panic("scale")}}
