// main.scale
func scale(p *main.point, k int64) {
  (*p).x = (*p).x * k
  (*p).y = (*p).y * k
  return
}
