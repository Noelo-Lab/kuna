package main

import (
 "encoding/json"
 "fmt"
 "go/ast"
 "go/parser"
 "go/token"
 "go/types"
 "os"
 "strings"
)

type function struct { Address uint64 `json:"address"`; Name string `json:"name"`; Code string `json:"code"` }
type corpus struct { Functions []function `json:"functions"` }
type finding struct { Address uint64 `json:"address"`; Name string `json:"name"`; Errors []string `json:"errors"` }
type imports struct{}
func(imports) Import(path string)(*types.Package,error) {
 if path=="unsafe" {return types.Unsafe,nil}
 return nil,fmt.Errorf("unavailable import %s",path)
}
func main() {
 data,err:=os.ReadFile(os.Args[1]);if err!=nil{panic(err)}
 var input corpus;if err=json.Unmarshal(data,&input);err!=nil{panic(err)}
 results:=[]finding{}
 for _,f:=range input.Functions {
  fs:=token.NewFileSet()
  file,err:=parser.ParseFile(fs,"body.go","package main\nimport \"unsafe\"\n"+f.Code,0)
  if err!=nil{panic(err)}
  messages:=[]string{}
  config:=types.Config{Importer:imports{},Error:func(err error){
   message:=err.Error()
   if strings.Contains(message,"mismatched types") || strings.Contains(message,"cannot use") || strings.Contains(message,"cannot convert") {
    if e,ok:=err.(types.Error);ok{message=e.Msg}
    messages=append(messages,message)
   }
  }}
  _,_=config.Check("main",fs,[]*ast.File{file},nil)
  if len(messages)>0{results=append(results,finding{f.Address,f.Name,messages})}
 }
 if err=json.NewEncoder(os.Stdout).Encode(results);err!=nil{panic(err)}
}
