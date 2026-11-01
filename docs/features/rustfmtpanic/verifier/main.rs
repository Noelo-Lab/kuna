use std::{collections::BTreeMap, fs};
use quote::ToTokens;
use syn::{visit_mut::{self, VisitMut}, parse::Parser, Expr, Stmt};
use syn::spanned::Spanned;
include!("helpers.rs");

#[derive(Default)]
struct Normalize { sites: Vec<serde_json::Value> }
impl VisitMut for Normalize {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if let Stmt::Macro(m) = stmt {
            if m.mac.path.is_ident("panic") {
                *stmt = Stmt::Expr(Expr::Macro(syn::ExprMacro { attrs: m.attrs.clone(), mac: m.mac.clone() }), m.semi_token);
            }
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let site = match expr {
            Expr::Call(c) if c.func.to_token_stream().to_string() == "core :: panicking :: panic_fmt" => {
                assert!(c.attrs.is_empty());
                Some(serde_json::json!({"kind":"runtime", "source_line_start":c.span().start().line,"source_line_end":c.span().end().line,"arguments":c.args.iter().map(|e|e.to_token_stream().to_string()).collect::<Vec<_>>()}))
            }
            Expr::Macro(m) if m.mac.path.is_ident("panic") => {
                assert!(m.attrs.is_empty());
                let args=syn::punctuated::Punctuated::<Expr,syn::Token![,]>::parse_terminated.parse2(m.mac.tokens.clone()).unwrap();
                let Some(Expr::Lit(first))=args.first() else {panic!("macro has no string")};
                let syn::Lit::Str(literal)=&first.lit else {panic!("not literal")};
                let mut parameters=Vec::new();
                for arg in args.iter().skip(1) {
                    let Expr::Reference(reference)=arg else {panic!("not borrowed")};
                    assert!(reference.mutability.is_none() && reference.attrs.is_empty());
                    let Expr::Unary(read)=reference.expr.as_ref() else {panic!("not dereference")};
                    assert!(matches!(read.op,syn::UnOp::Deref(_)) && read.attrs.is_empty());
                    let unparen=|mut e:Expr| {while let Expr::Paren(p)=e {assert!(p.attrs.is_empty());e=*p.expr;} e};
                    let Expr::Cast(typed)=unparen(*read.expr.clone()) else {panic!("no typed read")};
                    let syn::Type::Ptr(ptr)=typed.ty.as_ref() else {panic!("not pointer")};
                    let primitive=ptr.elem.to_token_stream().to_string();
                    assert!(ptr.mutability.is_none() && matches!(primitive.as_str(),"u8"|"i8"|"u16"|"i16"|"u32"|"i32"|"u64"|"i64"));
                    let Expr::Cast(bridge)=unparen(*typed.expr) else {panic!("no pointer bridge")};
                    assert_eq!(bridge.ty.to_token_stream().to_string(),"* const _");
                    let pointer=bridge.expr.to_token_stream().to_string();
                    parameters.push(serde_json::json!({"primitive":primitive,"pointer":pointer}));
                }
                Some(serde_json::json!({"kind":"macro","literal":literal.value(),"arguments":parameters}))
            }
            _ => None,
        };
        if let Some(site)=site {
            let index=self.sites.len();self.sites.push(site);
            *expr=syn::parse_str(&format!("__kuna_panic_site({index})")).unwrap();
        } else { visit_mut::visit_expr_mut(self,expr); }
    }
}
fn check(before:&str,after:&str)->serde_json::Value {
    assert_eq!(comments(before),comments(after),"comments changed");
    let (mut a,ra)=parse(before);let(mut b,rb)=parse(after);assert_eq!(ra,rb);
    let mut na=Normalize::default();let mut nb=Normalize::default();na.visit_file_mut(&mut a);nb.visit_file_mut(&mut b);
    assert_eq!(a,b,"AST outside panic calls changed");assert_eq!(na.sites.len(),nb.sites.len());
    let mut changes=Vec::new();
    for (old,new) in na.sites.iter().zip(nb.sites.iter()) {
        if old["kind"]==new["kind"] && old["arguments"]==new["arguments"] && old["literal"]==new["literal"] {continue;}
        assert_eq!(old["kind"],"runtime");assert_eq!(new["kind"],"macro");
        assert_eq!(old["arguments"].as_array().unwrap().len(),3);
        changes.push(serde_json::json!({"before":old,"after":new}));
    }
    assert!(!changes.is_empty());serde_json::json!({"changes":changes,"identical_preexisting_syntax_repairs":ra})
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.len()==2 && args[1]=="--selfcheck" {
        let a="fn f(x:u64){let v=x; /* keep */ core::panicking::panic_fmt(0x2000,9,0x3000);} ";
        let b="fn f(x:u64){let v=x; /* keep */ panic!(\"{{é}}\\n\");} ";
        assert_eq!(check(a,b)["changes"].as_array().unwrap().len(),1);
        std::panic::set_hook(Box::new(|_|{}));
        for bad in [b.replace("let v=x;",""),b.replace("keep","drop"),b.replace("x:u64","x:i64"),b.replace("let v=x;","let v=x+1;"),b.replace("panic!","print!"),b.replace("panic!(\"{{é}}\\n\");","panic!(\"{{é}}\\n\"); panic!(\"extra\");")] {
            assert!(std::panic::catch_unwind(||check(a,&bad)).is_err());
        }
        println!("6 statement, type, comment, sink and extra-call mutations rejected");return;
    }
    let a:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let b:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let before:BTreeMap<_,_>=a["functions"].as_array().unwrap().iter().map(|v|(v["address"].as_u64().unwrap(),v)).collect();
    let mut result=BTreeMap::new();
    for v in b["functions"].as_array().unwrap() {
        let address=v["address"].as_u64().unwrap();let old=before[&address];
        if old["code"]!=v["code"] {
            eprintln!("checking {address:#x} {}",v["name"]);
            result.insert(format!("{address:#x}"),check(old["code"].as_str().unwrap(),v["code"].as_str().unwrap()));
        }
    }
    println!("{}",serde_json::to_string(&result).unwrap());
}
