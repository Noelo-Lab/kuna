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
    if !matches!(expr,Expr::Paren(_)|Expr::Block(_)) {return None;}
    if !text(expr).contains("use std :: io :: Write as _ ;") {return None;}
    let Expr::Block(block)=unparen(expr.clone())else{panic!("no scoped trait import")};
    assert!(block.attrs.is_empty()&&block.label.is_none());assert_eq!(block.block.stmts.len(),2);
    assert_eq!(text(&block.block.stmts[0]),"use std :: io :: Write as _ ;");
    let Stmt::Expr(Expr::Match(result),None)=&block.block.stmts[1]else{panic!("no owned result conversion")};
    let Expr::Macro(mac)=unparen(*result.expr.clone())else{panic!("no single formatting call")};
    assert!(mac.attrs.is_empty());
    let mut normalized=result.clone();normalized.expr=Box::new(syn::parse_str("__format_call()").unwrap());
    let result_type=["u64","i64"].into_iter().find(|ty| {
        let expected:Expr=syn::parse_str(&format!("match __format_call() {{std::result::Result::Ok(())=>0{ty},std::result::Result::Err(_kuna_io_error)=>std::mem::transmute::<std::io::Error,*mut ()>(_kuna_io_error) as usize as {ty}}}")).unwrap();
        Expr::Match(normalized.clone())==expected
    }).expect("owned result conversion changed");
    let name=text(&mac.mac.path);assert!(matches!(name.as_str(),"write"|"writeln"));
    let args:syn::punctuated::Punctuated<Expr,syn::Token![,]>=syn::punctuated::Punctuated::parse_terminated.parse2(mac.mac.tokens.clone()).unwrap();assert!(args.len()>=2);
    let Expr::Reference(borrow)=unparen(args[0].clone())else{panic!("no original writer borrow")};
    assert!(borrow.attrs.is_empty()&&borrow.mutability.is_some());
    let(receiver,typed)=match unparen(*borrow.expr) {
        Expr::Unary(read)=>{
            assert!(read.attrs.is_empty()&&matches!(read.op,syn::UnOp::Deref(_)));
            let Expr::Cast(typed)=unparen(*read.expr)else{panic!("no reference-cell cast")};
            assert_eq!(text(&typed.ty),"* mut & std :: io :: Stdout");
            ("ReferenceCell",typed)
        },
        Expr::Call(call)=>{
            assert!(call.attrs.is_empty());assert_eq!(call.args.len(),1);
            assert_eq!(text(&call.func),"std :: mem :: transmute :: < * mut () , std :: io :: Stdout >");
            let Expr::Cast(typed)=unparen(call.args[0].clone())else{panic!("no promoted-inner cast")};
            assert_eq!(text(&typed.ty),"* mut ()");
            ("PromotedInner",typed)
        },
        _=>panic!("unknown receiver expression"),
    };
    assert!(typed.attrs.is_empty());
    let writer=match unparen(*typed.expr){
        Expr::Cast(c)=>{assert!(c.attrs.is_empty());assert_eq!(text(&c.ty),"* mut _");plain_value(*c.expr)},
        e=>plain_value(e),
    };
    let Expr::Lit(literal)=&args[1]else{panic!("not literal")};
    let syn::Lit::Str(literal)=&literal.lit else{panic!("not string")};
    let parameters:Vec<_>=args.iter().skip(2).map(|arg|parameter(&text(arg))).collect();
    Some(serde_json::json!({"kind":"macro","sink":"write","name":name,"literal":literal.value(),"writer":writer,"receiver":receiver,"result_type":result_type,"parameters":parameters,"arguments":args.iter().map(text).collect::<Vec<_>>()}))
}

#[derive(Default)]
struct Normalize {sites:Vec<serde_json::Value>}
impl VisitMut for Normalize {
    fn visit_expr_mut(&mut self,expr:&mut Expr) {
        let raw=text(expr);
        let site=match expr {
            Expr::Call(c)if matches!(text(&c.func).as_str(),"_std :: io :: stdio :: Stdout :: write_fmt"|"std :: io :: stdio :: Stdout :: write_fmt")=>{
                assert!(c.attrs.is_empty());Some(serde_json::json!({"kind":"runtime","sink":"write","arguments":c.args.iter().map(text).collect::<Vec<_>>()}))
            },
            _=>recovered(expr),
        };
        if let Some(mut site)=site {
            site["raw"]=raw.into();site["source_line_start"]=expr.span().start().line.into();site["source_line_end"]=expr.span().end().line.into();
            let index=self.sites.len();self.sites.push(site);*expr=syn::parse_str(&format!("__kuna_stdout_site({index})")).unwrap();
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
        let args=old["arguments"].as_array().unwrap();assert_eq!(args.len(),3);
        assert_eq!(text(&unparen(syn::parse_str(args[0].as_str().unwrap()).unwrap())),new["writer"]);
        changes.push(serde_json::json!({"before":old,"after":new}));
    }
    assert!(!changes.is_empty());serde_json::json!({"changes":changes,"identical_preexisting_syntax_repairs":ra})
}

fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.len()==2&&args[1]=="--selfcheck" {
        let mut mutations_rejected=0;
        for ty in ["u64","i64"] {
            for writer in ["&mut *(a0 as *mut &std::io::Stdout)","&mut *((a0 as *mut _) as *mut &std::io::Stdout)","&mut std::mem::transmute::<*mut (),std::io::Stdout>(a0 as *mut ())","&mut std::mem::transmute::<*mut (),std::io::Stdout>((a0 as *mut _) as *mut ())"] {
                let a=format!("fn f(a0:u64,p:*mut u32)->{ty} {{let keep=1; /* keep */ _std::io::stdio::Stdout::write_fmt(a0,4096,&mut p)}}");
                let b=format!("fn f(a0:u64,p:*mut u32)->{ty} {{let keep=1; /* keep */ ({{use std::io::Write as _; match write!({writer},\"[{{}}]\",&*((p as *const _) as *const u32)) {{std::result::Result::Ok(())=>0{ty},std::result::Result::Err(_kuna_io_error)=>std::mem::transmute::<std::io::Error,*mut ()>(_kuna_io_error) as usize as {ty}}}}})}}");
                assert_eq!(check(&a,&b)["changes"].as_array().unwrap().len(),1);
                std::panic::set_hook(Box::new(|_|{}));
                let mutations=[b.replace("Ok(())=>0","Ok(())=>1"),b.replace(" as usize as "," as u32 as "),b.replace("a0 as","other as"),b.replace("use std::io::Write as _;",""),b.replace("&mut ","&"),b.replace("let keep=1;",""),b.replace("/* keep */","/* changed */"),b.replace("match write!", "side_effect(); match write!"),b.replace("a0 as","side_effect() as"),b.replace("Err(_kuna_io_error)=>std", "Err(_kuna_io_error) if condition()=>std"),b.replace("std::io::Error,*mut ()>", "std::io::Error,u64>"),b.replace("Err(_kuna_io_error)=>std", "Err(_kuna_io_error)=>drop(std")];
                for bad in &mutations {assert!(std::panic::catch_unwind(||check(&a,bad)).is_err());mutations_rejected+=1;}
            }
        }
        println!("{mutations_rejected} result, receiver, scope, side-effect, statement and comment mutations rejected");return;
    }
    let a:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let b:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let before:BTreeMap<_,_>=a["functions"].as_array().unwrap().iter().map(|v|(v["address"].as_u64().unwrap(),v)).collect();let mut result=BTreeMap::new();
    for v in b["functions"].as_array().unwrap(){let address=v["address"].as_u64().unwrap();let old=before[&address];if old["code"]!=v["code"]{eprintln!("checking {address:#x} {}",v["name"]);result.insert(format!("{address:#x}"),check(old["code"].as_str().unwrap(),v["code"].as_str().unwrap()));}}
    println!("{}",serde_json::to_string(&result).unwrap());
}
