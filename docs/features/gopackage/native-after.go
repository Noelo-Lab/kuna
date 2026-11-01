// main.scale
func scale(p *point, k int64) {
  (*p).x = (*p).x * k
  (*p).y = (*p).y * k
  return
}
