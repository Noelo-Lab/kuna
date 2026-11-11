package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"reflect"
	"sort"
)

type function struct {
	Address uint64 `json:"address"`
	Name    string `json:"name"`
	Code    string `json:"code"`
}
type corpus struct {
	Functions []function `json:"functions"`
}

var position = reflect.TypeOf(token.Pos(0))

func normalized(v reflect.Value) any {
	if !v.IsValid() {
		return nil
	}
	if v.Kind() == reflect.Interface || v.Kind() == reflect.Pointer {
		if v.IsNil() {
			return nil
		}
		if p, ok := v.Interface().(*ast.ParenExpr); ok {
			return normalized(reflect.ValueOf(p.X))
		}
		return normalized(v.Elem())
	}
	switch v.Kind() {
	case reflect.Struct:
		m := map[string]any{"kind": v.Type().String()}
		for i := 0; i < v.NumField(); i++ {
			f := v.Type().Field(i)
			if f.Type == position {
				continue
			}
			m[f.Name] = normalized(v.Field(i))
		}
		return m
	case reflect.Slice:
		a := make([]any, v.Len())
		for i := range a {
			a[i] = normalized(v.Index(i))
		}
		return a
	default:
		return v.Interface()
	}
}

var integers = []string{"byte", "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64"}
var booleanConversions = map[string]any{}

func conversion(b any) (string, any, bool) {
	m, ok := b.(map[string]any)
	if !ok || m["kind"] != "ast.CallExpr" {
		return "", nil, false
	}
	args, _ := m["Args"].([]any)
	if len(args) != 1 {
		return "", nil, false
	}
	f, ok := m["Fun"].(map[string]any)
	if !ok {
		return "", nil, false
	}
	if f["kind"] == "ast.Ident" {
		name, _ := f["Name"].(string)
		for _, ty := range integers {
			if name == ty {
				return name, args[0], true
			}
		}
	}
	if f["kind"] == "ast.FuncLit" {
		for _, ty := range integers {
			template, ok := booleanConversions[ty]
			if !ok {
				expr, err := parser.ParseExpr("func(value bool) " + ty + " { if value { return 1 }; return 0 }")
				if err != nil {
					panic(err)
				}
				template = normalized(reflect.ValueOf(expr))
				booleanConversions[ty] = template
			}
			if reflect.DeepEqual(f, template) {
				return "bool_to_" + ty, args[0], true
			}
		}
	}
	return "", nil, false
}

func same(a, b any) ([]string, bool) {
	if reflect.DeepEqual(a, b) {
		return nil, true
	}
	if am, ok := a.(map[string]any); ok {
		if bm, ok := b.(map[string]any); ok && len(am) == len(bm) && am["kind"] == bm["kind"] {
			keys := make([]string, 0, len(am))
			for k := range am {
				keys = append(keys, k)
			}
			sort.Strings(keys)
			added := []string{}
			good := true
			for _, k := range keys {
				xs, ok := same(am[k], bm[k])
				if !ok {
					good = false
					break
				}
				added = append(added, xs...)
			}
			if good {
				return added, true
			}
		}
	} else if aa, ok := a.([]any); ok {
		if ba, ok := b.([]any); ok && len(aa) == len(ba) {
			added := []string{}
			good := true
			for i := range aa {
				xs, ok := same(aa[i], ba[i])
				if !ok {
					good = false
					break
				}
				added = append(added, xs...)
			}
			if good {
				return added, true
			}
		}
	}
	if name, arg, ok := conversion(b); ok {
		if xs, ok := same(a, arg); ok {
			return append(xs, name), true
		}
	}
	return nil, false
}

func parse(code string) any {
	f, err := parser.ParseFile(token.NewFileSet(), "body.go", "package main\n"+code, parser.ParseComments|parser.SkipObjectResolution)
	if err != nil {
		panic(err)
	}
	return normalized(reflect.ValueOf(f))
}

func read(path string) corpus {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	var c corpus
	if err = json.Unmarshal(data, &c); err != nil {
		panic(err)
	}
	return c
}

func main() {
	before, after := read(os.Args[1]), read(os.Args[2])
	old := map[uint64]function{}
	for _, f := range before.Functions {
		old[f.Address] = f
	}
	output := map[string]any{}
	for _, f := range after.Functions {
		a, ok := old[f.Address]
		if !ok {
			panic(f.Name)
		}
		if a.Code == f.Code {
			continue
		}
		casts, ok := same(parse(a.Code), parse(f.Code))
		if !ok || len(casts) == 0 {
			panic("non-conversion AST change: " + f.Name)
		}
		output[fmt.Sprintf("0x%x", f.Address)] = map[string]any{"added_integer_conversions": casts, "other_ast_fields_and_comments_unchanged": true}
	}
	if err := json.NewEncoder(os.Stdout).Encode(output); err != nil {
		panic(err)
	}
}
