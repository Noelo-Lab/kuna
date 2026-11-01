use std::{collections::BTreeMap, fs};
use quote::ToTokens;
use syn::{visit_mut::{self, VisitMut}, Expr};

#[derive(Default)]
struct Normalize { casts: Vec<Vec<String>> }
impl VisitMut for Normalize {
    fn visit_expr_mut(&mut self, expression: &mut Expr) {
        let mut casts = Vec::new();
        loop {
            match expression {
                Expr::Paren(p) => {
                    assert!(p.attrs.is_empty());
                    *expression = *p.expr.clone();
                }
                Expr::Cast(c) => {
                    assert!(c.attrs.is_empty());
                    casts.push(c.ty.to_token_stream().to_string());
                    *expression = *c.expr.clone();
                }
                _ => break,
            }
        }
        self.casts.push(casts);
        visit_mut::visit_expr_mut(self, expression);
    }
}
fn comments(source: &str) -> Vec<String> {
    let mut starts = vec![0];
    for (i, b) in source.bytes().enumerate() { if b == b'\n' { starts.push(i + 1); } }
    let index = |p: proc_macro2::LineColumn| {
        let start = starts[p.line - 1];
        let tail = &source[start..];
        start + tail.char_indices().nth(p.column).map_or(tail.len(), |(i,_)| i)
    };
    fn ranges(ts: proc_macro2::TokenStream, out: &mut Vec<proc_macro2::Span>) {
        for token in ts {
            match token {
                proc_macro2::TokenTree::Group(g) => {
                    out.push(g.span_open()); ranges(g.stream(), out); out.push(g.span_close());
                }
                t => out.push(t.span()),
            }
        }
    }
    let mut spans = Vec::new();
    ranges(source.parse().unwrap(), &mut spans);
    let mut intervals: Vec<_> = spans.iter().map(|s| (index(s.start()), index(s.end()))).collect();
    intervals.sort();
    let mut end = 0;
    let mut comments = Vec::new();
    for (a, b) in intervals {
        if a > end && !source[end..a].trim().is_empty() { comments.push(source[end..a].trim().to_string()); }
        end = end.max(b);
    }
    if !source[end..].trim().is_empty() { comments.push(source[end..].trim().to_string()); }
    comments
}
fn parse(source: &str) -> (syn::File, BTreeMap<&'static str,usize>) {
    if let Ok(file) = syn::parse_file(source) { return (file,BTreeMap::new()); }
    fn repair(tokens: proc_macro2::TokenStream, count: &mut BTreeMap<&'static str,usize>) -> proc_macro2::TokenStream {
        use proc_macro2::{TokenTree,Delimiter,Group,Ident};
        let items: Vec<_> = tokens.into_iter().collect();
        items.iter().enumerate().map(|(i,t)| match t {
            TokenTree::Group(g) => {
                let mut group=Group::new(g.delimiter(),repair(g.stream(),count));
                group.set_span(g.span());TokenTree::Group(group)
            }
            TokenTree::Ident(id) if id=="in"
                && matches!(items.get(i+1),Some(TokenTree::Group(g)) if g.delimiter()==Delimiter::Parenthesis)
                && (i==0 || !matches!(items[i-1],TokenTree::Ident(_))) => {
                *count.entry("reserved_in_call").or_insert(0)+=1;TokenTree::Ident(Ident::new_raw("in",id.span()))
            }
            TokenTree::Ident(id) if id=="_"
                && matches!(items.get(i+1),Some(TokenTree::Punct(p)) if p.as_char()==':')
                && matches!(items.get(i+2),Some(TokenTree::Punct(p)) if p.as_char()==':') => {
                *count.entry("placeholder_namespace").or_insert(0)+=1;
                TokenTree::Ident(Ident::new("__kuna_unresolved_namespace",id.span()))
            }
            _ => t.clone(),
        }).collect()
    }
    let mut count=BTreeMap::new();
    let tokens=repair(source.parse().unwrap(), &mut count);
    if count.is_empty() {
        let error=syn::parse_file(source).unwrap_err(); let at=error.span().start();
        panic!("pre-existing parse: {error} at {at:?}: {}",source.lines().nth(at.line-1).unwrap_or(""));
    }
    let file=syn::parse2(tokens).unwrap_or_else(|error| {
        let at=error.span().start();
        panic!("parse after keyword repair: {error} at {at:?}: {}",source.lines().nth(at.line-1).unwrap_or(""));
    });
    (file,count)
}
fn check(before: &str, after: &str) -> BTreeMap<String, usize> {
    assert_eq!(comments(before), comments(after), "comments changed");
    let (mut a, repairs_a) = parse(before);
    let (mut b, repairs_b) = parse(after);
    assert_eq!(repairs_a,repairs_b,"keyword-call repair differs");
    let mut na = Normalize::default(); let mut nb = Normalize::default();
    na.visit_file_mut(&mut a); nb.visit_file_mut(&mut b);
    assert_eq!(a,b,"AST changed beyond integer conversions");
    assert_eq!(na.casts.len(), nb.casts.len());
    let mut added = BTreeMap::new();
    for (old,new) in na.casts.iter().zip(&nb.casts) {
        let mut i = 0;
        for ty in new {
            if old.get(i) == Some(ty) { i += 1; }
            else {
                assert!(matches!(ty.as_str(), "i8"|"i16"|"i32"|"i64"|"i128"|"u8"|"u16"|"u32"|"u64"|"u128"), "not integer cast: {ty}");
                *added.entry(ty.clone()).or_insert(0) += 1;
            }
        }
        assert_eq!(i,old.len(),"existing cast removed or changed: {old:?} -> {new:?}");
    }
    added
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len()==2 && args[1]=="--selfcheck" {
        let a="fn f(a:u64)->i64 { /* keep */ (a as u8) as i64 * 8 + 64 }";
        let b="fn f(a:u64)->i64 { /* keep */ ((a as u8) as i8) as i64 * 8 + 64 }";
        assert_eq!(check(a,b).get("i8"),Some(&1));
        let unicode_a = "fn f(x:u8)->i64 { let s=\"é\"; /* сохранено */ x as i64 }";
        let unicode_b = unicode_a.replace("x as i64", "x as i8 as i64");
        assert_eq!(check(unicode_a, &unicode_b).get("i8"), Some(&1));
        let keyword_a = "fn f(x:u8)->i64 { let y=in(0x31); x as i64 }";
        let keyword_b = keyword_a.replace("x as i64", "x as i8 as i64");
        assert_eq!(check(keyword_a,&keyword_b).get("i8"),Some(&1));
        assert_eq!(parse(keyword_a).1.get("reserved_in_call"),Some(&1));
        let path_a="fn f(x:u8)->i64 { let y=_::f(); x as i64 }";
        assert_eq!(check(path_a,&path_a.replace("x as i64","x as i8 as i64")).get("i8"),Some(&1));
        std::panic::set_hook(Box::new(|_| {}));
        for bad in [b.replace(" * 8", " * 9"),b.replace("keep","changed"),b.replace("a:u64","a:i64"),b.replace("as u8", "as u16"),b.replace("as u8", ""),b.replace("* 8 + 64", "* (8 + 64)"),b.replace("as i8", "as f32")] {
            assert!(std::panic::catch_unwind(||check(a,&bad)).is_err(),"accepted invalid mutation: {bad}");
        }
        println!("accepted signed reinterpretation; rejected 7 semantic/comment/type mutations");
        return;
    }
    let a: serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let b: serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let before: BTreeMap<_,_>=a["functions"].as_array().unwrap().iter().map(|v|(v["address"].as_u64().unwrap(),v)).collect();
    let mut result=BTreeMap::new();
    for v in b["functions"].as_array().unwrap() {
        let address=v["address"].as_u64().unwrap();let old=before[&address];
        if old["code"]!=v["code"] {
            eprintln!("checking {address:#x} {}",v["name"]);
            let x=old["code"].as_str().unwrap();let y=v["code"].as_str().unwrap();
            result.insert(format!("{address:#x}"),serde_json::json!({"added_integer_casts":check(x,y),"identical_preexisting_syntax_repairs":parse(x).1}));
        }
    }
    println!("{}",serde_json::to_string(&result).unwrap());
}
