use std::{collections::BTreeMap, fs};
use quote::ToTokens;
use syn::{visit_mut::{self, VisitMut}, parse::Parser, Expr, Stmt};
use syn::spanned::Spanned;
include!("helpers.rs");

fn text<T:ToTokens>(v:&T)->String {v.to_token_stream().to_string()}
fn unparen(mut e:Expr)->Expr {
    while let Expr::Paren(p)=e {assert!(p.attrs.is_empty());e=*p.expr;} e
}
fn sink(name:&str)->Option<&'static str> {
    match name {
        "_print"|"std :: io :: stdio :: _print"=>Some("stdout"),
        "_eprint"|"std :: io :: stdio :: _eprint"=>Some("stderr"),
        "panic_fmt"|"core :: panicking :: panic_fmt"=>Some("panic"),
        "format_inner"|"alloc :: fmt :: format :: format_inner"=>Some("format"),
        _=>None,
    }
}
fn macro_sink(name:&str)->Option<&'static str> {
    match name {"print"|"println"=>Some("stdout"),"eprint"|"eprintln"=>Some("stderr"),"panic"=>Some("panic"),"format"=>Some("format"),_=>None}
}
fn arguments(m:&syn::Macro)->syn::punctuated::Punctuated<Expr,syn::Token![,]> {
    syn::punctuated::Punctuated::parse_terminated.parse2(m.tokens.clone()).unwrap()
}
fn macro_site(m:&syn::Macro, destination:Option<&Expr>)->serde_json::Value {
    let args=arguments(m);
    serde_json::json!({"kind":"macro","sink":macro_sink(&text(&m.path)).unwrap(),"name":text(&m.path),
        "destination":destination.map(text),"arguments":args.iter().map(text).collect::<Vec<_>>()})
}
#[derive(Default)]
struct Normalize {sites:Vec<serde_json::Value>}
impl VisitMut for Normalize {
    fn visit_stmt_mut(&mut self,stmt:&mut Stmt) {
        if let Stmt::Macro(m)=stmt {
            if macro_sink(&text(&m.mac.path)).is_some() {
                *stmt=Stmt::Expr(Expr::Macro(syn::ExprMacro{attrs:m.attrs.clone(),mac:m.mac.clone()}),m.semi_token);
            }
        }
        visit_mut::visit_stmt_mut(self,stmt);
    }
    fn visit_expr_mut(&mut self,expr:&mut Expr) {
        let raw=text(expr);
        let site=match expr {
            Expr::Call(c) if sink(&text(&c.func)).is_some()=>{
                assert!(c.attrs.is_empty());
                Some(serde_json::json!({"kind":"runtime","sink":sink(&text(&c.func)),"arguments":c.args.iter().map(text).collect::<Vec<_>>()}))
            }
            Expr::Call(c) if text(&c.func)=="std :: ptr :: write" && c.args.len()==2 => {
                if let Expr::Macro(m)=&c.args[1] {
                    if text(&m.mac.path)=="format" {
                        assert!(c.attrs.is_empty() && m.attrs.is_empty());Some(macro_site(&m.mac,Some(&c.args[0])))
                    } else {None}
                } else {None}
            }
            Expr::Macro(m) if macro_sink(&text(&m.mac.path)).is_some()=>{
                assert!(m.attrs.is_empty());Some(macro_site(&m.mac,None))
            }
            _=>None,
        };
        if let Some(mut site)=site {
            site["raw"]=raw.into();site["source_line_start"]=expr.span().start().line.into();site["source_line_end"]=expr.span().end().line.into();
            let index=self.sites.len();self.sites.push(site);
            *expr=syn::parse_str(&format!("__kuna_str_site({index})")).unwrap();
        } else {visit_mut::visit_expr_mut(self,expr);}
    }
}
fn plain_value(e:Expr)->String {
    let e=unparen(e);
    assert!(matches!(e,Expr::Path(_)|Expr::Lit(_)),"new string input expression needs review: {}",text(&e));
    text(&e)
}
fn parameter(source:&str)->serde_json::Value {
    let expr=syn::parse_str::<Expr>(source).unwrap();
    if let Expr::Call(outer)=&expr {
        assert!(outer.attrs.is_empty());assert_eq!(text(&outer.func),"std :: str :: from_utf8_unchecked");assert_eq!(outer.args.len(),1);
        let Expr::Call(inner)=&outer.args[0] else {panic!("missing slice constructor")};
        assert!(inner.attrs.is_empty());assert_eq!(text(&inner.func),"std :: slice :: from_raw_parts");assert_eq!(inner.args.len(),2);
        let Expr::Cast(data)=unparen(inner.args[0].clone()) else {panic!("no byte pointer cast")};
        assert!(data.attrs.is_empty());assert_eq!(text(&data.ty),"* const u8");
        let data=match unparen(*data.expr) {
            Expr::Cast(bridge)=>{assert!(bridge.attrs.is_empty());assert_eq!(text(&bridge.ty),"* const _");plain_value(*bridge.expr)}
            e=>plain_value(e),
        };
        let Expr::Cast(length)=unparen(inner.args[1].clone()) else {panic!("no usize cast")};
        assert!(length.attrs.is_empty());assert_eq!(text(&length.ty),"usize");
        return serde_json::json!({"kind":"string","data":data,"length":plain_value(*length.expr)});
    }
    let Expr::Reference(reference)=expr else {panic!("not borrowed primitive")};
    assert!(reference.mutability.is_none() && reference.attrs.is_empty());
    let Expr::Unary(read)=*reference.expr else {panic!("not dereference")};
    assert!(matches!(read.op,syn::UnOp::Deref(_)) && read.attrs.is_empty());
    let Expr::Cast(typed)=unparen(*read.expr) else {panic!("no typed read")};
    let syn::Type::Ptr(ptr)=typed.ty.as_ref() else {panic!("not pointer")};
    let primitive=text(&ptr.elem);assert!(ptr.mutability.is_none() && matches!(primitive.as_str(),"u8"|"i8"|"u16"|"i16"|"u32"|"i32"|"u64"|"i64"));
    let Expr::Cast(bridge)=unparen(*typed.expr) else {panic!("no pointer bridge")};
    assert_eq!(text(&bridge.ty),"* const _");
    serde_json::json!({"kind":"primitive","primitive":primitive,"pointer":plain_value(*bridge.expr)})
}
fn check(before:&str,after:&str)->serde_json::Value {
    assert_eq!(comments(before),comments(after),"comments changed");
    let(mut a,ra)=parse(before);let(mut b,rb)=parse(after);assert_eq!(ra,rb);
    let mut na=Normalize::default();let mut nb=Normalize::default();na.visit_file_mut(&mut a);nb.visit_file_mut(&mut b);
    assert_eq!(a,b,"AST outside recovered calls changed");assert_eq!(na.sites.len(),nb.sites.len());
    let mut changes=Vec::new();
    for(old,new)in na.sites.iter().zip(nb.sites.iter_mut()) {
        if old["raw"]==new["raw"] {continue;}
        assert_eq!(old["kind"],"runtime");assert_eq!(new["kind"],"macro");assert_eq!(old["sink"],new["sink"]);
        let oldargs=old["arguments"].as_array().unwrap();
        assert_eq!(oldargs.len(),if old["sink"]=="stdout"||old["sink"]=="stderr"{2}else{3});
        if old["sink"]=="format" {
            let Expr::Cast(dest)=unparen(syn::parse_str(new["destination"].as_str().unwrap()).unwrap())else{panic!("no destination cast")};
            assert_eq!(text(&dest.ty),"* mut std :: string :: String");
            let dest=match unparen(*dest.expr){Expr::Cast(bridge)=>{assert_eq!(text(&bridge.ty),"* mut _");*bridge.expr},e=>e};
            assert_eq!(text(&unparen(dest)),text(&unparen(syn::parse_str(oldargs[0].as_str().unwrap()).unwrap())));
        }else{assert!(new["destination"].is_null());}
        let args=new["arguments"].as_array().unwrap();
        let Expr::Lit(literal)=syn::parse_str(args[0].as_str().unwrap()).unwrap()else{panic!("not literal")};
        let syn::Lit::Str(literal)=literal.lit else{panic!("not string")};
        let params:Vec<_>=args.iter().skip(1).map(|a|parameter(a.as_str().unwrap())).collect();
        assert!(params.iter().any(|a|a["kind"]=="string"));
        new["literal"]=literal.value().into();new["parameters"]=params.into();
        changes.push(serde_json::json!({"before":old,"after":new}));
    }
    assert!(!changes.is_empty());serde_json::json!({"changes":changes,"identical_preexisting_syntax_repairs":ra})
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.len()==2 && args[1]=="--selfcheck" {
        let a="fn f(x:u64,n:u64){let v=x; /* keep */ std::io::stdio::_print(4096,&mut v);}";
        let b="fn f(x:u64,n:u64){let v=x; /* keep */ print!(\"{}\",std::str::from_utf8_unchecked(std::slice::from_raw_parts(v as *const u8,n as usize)));}";
        assert_eq!(check(a,b)["changes"].as_array().unwrap().len(),1);
        std::panic::set_hook(Box::new(|_|{}));
        let mutations=[b.replace("let v=x;",""),b.replace("keep","drop"),b.replace("x:u64","x:i64"),b.replace("let v=x;","let v=x+1;"),b.replace("print!","eprint!"),b.replace("*const u8","*const u16"),b.replace("v as *const","side_effect() as *const"),b.replace("n as usize","other() as usize")];
        for bad in &mutations {assert!(std::panic::catch_unwind(||check(a,bad)).is_err());}
        println!("{} independent statement, comment, type, stream, width and side-effect mutations rejected",mutations.len());return;
    }
    let a:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let b:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let before:BTreeMap<_,_>=a["functions"].as_array().unwrap().iter().map(|v|(v["address"].as_u64().unwrap(),v)).collect();
    let mut result=BTreeMap::new();
    for v in b["functions"].as_array().unwrap(){let address=v["address"].as_u64().unwrap();let old=before[&address];if old["code"]!=v["code"]{eprintln!("checking {address:#x} {}",v["name"]);result.insert(format!("{address:#x}"),check(old["code"].as_str().unwrap(),v["code"].as_str().unwrap()));}}
    println!("{}",serde_json::to_string(&result).unwrap());
}
