package main

type Node struct {
	Value uint32
	Next  *Node
}

type Reader interface {
	Read() uint32
}

type Record struct {
	Flag   uint8
	Header string
	Child  *Node
	Tail   uint16
	Map    map[uint16]*Node
	Queue  chan<- *Node
	Call   func([]byte) (*Node, bool)
	Reader Reader
}

var Root *Record
var Word uint32
var Sink any

func (x *Node) Read() uint32 { return x.Value }

//go:noinline
func createWord(value uint32) *Node {
	result := new(Node)
	result.Value = value
	return result
}

//go:noinline
func create(value uint32) *Record {
	result := new(Record)
	result.Flag = 17
	result.Header = "go"
	result.Child = createWord(value)
	result.Child.Next = result.Child
	result.Tail = 199
	return result
}

//go:noinline
func inspect(r *Record) uint32 { return r.Child.Next.Value + uint32(r.Tail) }

func main() {
	Root = create(31)
	Word = inspect(Root)
	Sink = []any{Root, Root.Map, Root.Queue, Root.Call}
}
