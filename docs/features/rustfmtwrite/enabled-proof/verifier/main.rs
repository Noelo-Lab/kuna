use std::{collections::BTreeMap, fs};
use quote::ToTokens;
use syn::{visit_mut::{self, VisitMut}, parse::Parser, Expr, Stmt};
use syn::spanned::Spanned;
include!("helpers.rs");

fn text<T:ToTokens>(v:&T)->String {v.to_token_stream().to_string()}
fn unparen(mut e:Expr)->Expr {
    while let Expr::Paren(p)=e {assert!(p.attrs.is_empty());e=*p.expr;} e
}
include!("parameters.rs");

fn recovered(expr:&Expr)->Option<serde_json::Value> {
    let (inner,result_type)=match expr {
        Expr::Cast(c)=>{
            if !text(&c.expr).contains("use std :: fmt :: Write as _ ;"){return None;}
            assert!(c.attrs.is_empty());let ty=text(&c.ty);assert!(matches!(ty.as_str(),"u8"|"i8"));
            (unparen(*c.expr.clone()),ty)
        },
        Expr::MethodCall(c)if c.method=="is_err"=>(expr.clone(),"bool".into()),
        _=>return None,
    };
    let Expr::MethodCall(method)=inner else{panic!("not result bridge")};
    if !text(&method.receiver).contains("use std :: fmt :: Write as _ ;"){return None;}
    assert!(method.attrs.is_empty()&&method.args.is_empty()&&method.turbofish.is_none());assert_eq!(method.method,"is_err");
    let Expr::Block(block)=unparen(*method.receiver)else{panic!("no scoped trait import")};
    assert!(block.attrs.is_empty()&&block.label.is_none());assert_eq!(block.block.stmts.len(),2);
    assert_eq!(text(&block.block.stmts[0]),"use std :: fmt :: Write as _ ;");
    let mac=match &block.block.stmts[1]{
        Stmt::Expr(Expr::Macro(m),None)=>{assert!(m.attrs.is_empty());&m.mac},
        Stmt::Macro(m)=>{assert!(m.attrs.is_empty()&&m.semi_token.is_none());&m.mac},
        _=>panic!("not a single tail macro"),
    };
    let name=text(&mac.path);assert!(matches!(name.as_str(),"write"|"writeln"));
    let args:syn::punctuated::Punctuated<Expr,syn::Token![,]>=syn::punctuated::Punctuated::parse_terminated.parse2(mac.tokens.clone()).unwrap();assert!(args.len()>=2);
    let Expr::Reference(borrow)=unparen(args[0].clone())else{panic!("no writer borrow")};
    assert!(borrow.attrs.is_empty()&&borrow.mutability.is_some());
    let Expr::Unary(read)=unparen(*borrow.expr)else{panic!("no writer dereference")};
    assert!(read.attrs.is_empty()&&matches!(read.op,syn::UnOp::Deref(_)));
    let Expr::Cast(typed)=unparen(*read.expr)else{panic!("no String cast")};
    assert!(typed.attrs.is_empty());assert_eq!(text(&typed.ty),"* mut std :: string :: String");
    let writer=match unparen(*typed.expr){
        Expr::Cast(c)=>{assert!(c.attrs.is_empty());assert_eq!(text(&c.ty),"* mut _");plain_value(*c.expr)},
        e=>plain_value(e),
    };
    let Expr::Lit(literal)=&args[1]else{panic!("not literal")};
    let syn::Lit::Str(literal)=&literal.lit else{panic!("not string")};
    let parameters:Vec<_>=args.iter().skip(2).map(|arg|parameter(&text(arg))).collect();
    Some(serde_json::json!({"kind":"macro","sink":"write","name":name,"literal":literal.value(),"writer":writer,"result_type":result_type,"parameters":parameters,"arguments":args.iter().map(text).collect::<Vec<_>>()}))
}

#[derive(Default)]
struct Normalize {sites:Vec<serde_json::Value>}
impl VisitMut for Normalize {
    fn visit_expr_mut(&mut self,expr:&mut Expr) {
        let raw=text(expr);
        let site=match expr {
            Expr::Call(c)if matches!(text(&c.func).as_str(),"core :: fmt :: write"|"write")=>{
                assert!(c.attrs.is_empty());Some(serde_json::json!({"kind":"runtime","sink":"write","arguments":c.args.iter().map(text).collect::<Vec<_>>()}))
            },
            _=>recovered(expr),
        };
        if let Some(mut site)=site {
            site["raw"]=raw.into();site["source_line_start"]=expr.span().start().line.into();site["source_line_end"]=expr.span().end().line.into();
            let index=self.sites.len();self.sites.push(site);*expr=syn::parse_str(&format!("__kuna_write_site({index})")).unwrap();
        }else{visit_mut::visit_expr_mut(self,expr);}
    }
}

fn check(before:&str,after:&str)->serde_json::Value {
    assert_eq!(comments(before),comments(after),"comments changed");
    let(mut a,ra)=parse(before);let(mut b,rb)=parse(after);assert_eq!(ra,rb);
    let mut na=Normalize::default();let mut nb=Normalize::default();na.visit_file_mut(&mut a);nb.visit_file_mut(&mut b);
    assert_eq!(a,b,"AST outside recovered calls changed");assert_eq!(na.sites.len(),nb.sites.len());
    let mut changes=Vec::new();
    for(old,new)in na.sites.iter().zip(nb.sites.iter()) {
        if old["raw"]==new["raw"]{continue;}
        assert_eq!(old["kind"],"runtime");assert_eq!(new["kind"],"macro");
        let args=old["arguments"].as_array().unwrap();assert_eq!(args.len(),4);
        assert_eq!(text(&unparen(syn::parse_str(args[0].as_str().unwrap()).unwrap())),new["writer"]);
        changes.push(serde_json::json!({"before":old,"after":new}));
    }
    assert!(!changes.is_empty());serde_json::json!({"changes":changes,"identical_preexisting_syntax_repairs":ra})
}

fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.len()==2&&args[1]=="--selfcheck" {
        let a="fn f(a0:*mut u8,p:*mut u32)->u8 {let keep=1; /* keep */ core::fmt::write(a0,4096,8192,&mut p)}";
        let b="fn f(a0:*mut u8,p:*mut u32)->u8 {let keep=1; /* keep */ ({use std::fmt::Write as _; write!(&mut *((a0 as *mut _) as *mut std::string::String),\"[{}]\",&*((p as *const _) as *const u32))}).is_err() as u8}";
        assert_eq!(check(a,b)["changes"].as_array().unwrap().len(),1);
        std::panic::set_hook(Box::new(|_|{}));
        let mutations=[b.replace("is_err","is_ok"),b.replace("as u8","as u64"),b.replace("((a0 as","((other as"),b.replace("use std::fmt::Write as _;",""),b.replace("&mut *","&*"),b.replace("let keep=1;",""),b.replace("/* keep */","/* changed */"),b.replace("write!(&mut", "side_effect(); write!(&mut"),b.replace("((a0 as","((side_effect() as")];
        for bad in &mutations{assert!(std::panic::catch_unwind(||check(a,bad)).is_err());}
        println!("{} result, writer, scope, side-effect, statement and comment mutations rejected",mutations.len());return;
    }
    let a:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let b:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let before:BTreeMap<_,_>=a["functions"].as_array().unwrap().iter().map(|v|(v["address"].as_u64().unwrap(),v)).collect();let mut result=BTreeMap::new();
    for v in b["functions"].as_array().unwrap(){let address=v["address"].as_u64().unwrap();let old=before[&address];if old["code"]!=v["code"]{eprintln!("checking {address:#x} {}",v["name"]);result.insert(format!("{address:#x}"),check(old["code"].as_str().unwrap(),v["code"].as_str().unwrap()));}}
    println!("{}",serde_json::to_string(&result).unwrap());
}
