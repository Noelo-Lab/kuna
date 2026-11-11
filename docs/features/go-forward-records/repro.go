package main
import "fmt"
type Node struct { Next *Node; Value int }
func main() { fmt.Println(Node{Value: 7}) }
