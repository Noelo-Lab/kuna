use std::{collections::BTreeMap, fs};
use proc_macro2::{TokenTree, TokenStream, Delimiter};
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

fn compare(a: TokenStream, b: TokenStream, changes: &mut BTreeMap<String, usize>) {
    let a: Vec<_> = a.into_iter().collect();
    let b: Vec<_> = b.into_iter().collect();
    assert_eq!(a.len(), b.len(), "token count changed");
    for (i, (x, y)) in a.iter().zip(&b).enumerate() {
        match (x, y) {
            (TokenTree::Group(x), TokenTree::Group(y)) => {
                assert_eq!(x.delimiter(), y.delimiter());
                compare(x.stream(), y.stream(), changes);
            }
            (TokenTree::Ident(x), TokenTree::Ident(y)) if x.to_string() != y.to_string() => {
                let old = x.to_string();
                assert!(matches!(old.as_str(), "in" | "break" | "yield"), "unexpected keyword: {old}");
                assert_eq!(y.to_string(), format!("r#{old}"));
                assert!(matches!(a.get(i+1), Some(TokenTree::Group(g)) if g.delimiter()==Delimiter::Parenthesis));
                assert!(i == 0 || !matches!(a[i-1], TokenTree::Ident(_)), "not a call");
                *changes.entry(old).or_insert(0) += 1;
            }
            (TokenTree::Punct(x), TokenTree::Punct(y)) => {
                assert_eq!(x.as_char(), y.as_char());
                assert_eq!(x.spacing(), y.spacing());
            }
            _ => assert_eq!(x.to_string(), y.to_string(), "non-callee token changed"),
        }
    }
}
fn check(a: &str, b: &str) -> BTreeMap<String, usize> {
    assert_eq!(comments(a), comments(b), "comments changed");
    let mut result=BTreeMap::new();
    compare(a.parse().unwrap(), b.parse().unwrap(), &mut result);
    result
}
fn selftest() {
    let before="unsafe fn f() { let a = in(3); break(); yield(a); } // café";
    let after="unsafe fn f() { let a = r#in(3); r#break(); r#yield(a); } // café";
    assert_eq!(check(before, after).values().sum::<usize>(), 3);
    let invalid = [
        after.replace("(3)", "(4)"), after.replace("yield(a)", "yield(a+1)"),
        after.replace("café", "cafe"), after.replace("let a", "let b"),
        after.replace("in(3)", "in(3,4)"),
    ];
    for candidate in invalid {
        assert!(std::panic::catch_unwind(||check(before,&candidate)).is_err());
    }
    for (a,b) in [
        ("fn in() {}", "fn r#in() {}"), ("let in = 1;", "let r#in = 1;"),
        ("out(1)", "r#out(1)"), ("self()", "r#self()"),
        ("r#in(1)", "in(1)"), (r##"f("in")"##, r##"f("r#in")"##),
    ] { assert!(std::panic::catch_unwind(||check(a,b)).is_err()); }
    assert!(check("fn f(){r#in(1); _::f();}","fn f(){r#in(1); _::f();}").is_empty());
    println!("token and comment verifier mutation checks passed");
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str)==Some("--selftest") {selftest();return;}
    let a:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let b:serde_json::Value=serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let a=a["functions"].as_array().unwrap(); let b=b["functions"].as_array().unwrap();
    assert_eq!(a.len(), b.len());
    let old:BTreeMap<_,_>=a.iter().map(|f|(f["address"].as_u64().unwrap(),f)).collect();
    let mut output=BTreeMap::new();
    for f in b {
        let addr=f["address"].as_u64().unwrap();let prior=old[&addr];
        if prior["code"]!=f["code"] {
            let changes=check(prior["code"].as_str().unwrap(), f["code"].as_str().unwrap());
            assert!(!changes.is_empty());output.insert(format!("{addr:#x}"),changes);
        }
    }
    println!("{}",serde_json::to_string_pretty(&output).unwrap());
}
