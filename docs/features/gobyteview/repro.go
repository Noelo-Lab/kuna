package main
//go:noinline
func pair(x int64)(int64,int64){return x,x+1}
var a,b int64
func main(){a,b=pair(41)}
