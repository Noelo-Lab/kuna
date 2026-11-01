package main
//go:noinline
func pair(x int64)(int64,int64){return x,x+1}
//go:noinline
func combine(x int64)int64{a,b:=pair(x);return a*3+b}
var result int64
func main(){result=combine(41)}
