package main
import "fmt"
type Pointer *Pointer
type Holder struct { Link Pointer; Tail int }
func main() { fmt.Printf("%#v\n", Holder{Tail: 7}) }
