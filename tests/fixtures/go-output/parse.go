package main
import("go/parser";"go/token";"os";"fmt")
func main(){bad:=false;for _,name:=range os.Args[1:]{if _,err:=parser.ParseFile(token.NewFileSet(),name,nil,parser.AllErrors);err!=nil{fmt.Println(err);bad=true}};if bad{os.Exit(1)}}
