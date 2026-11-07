use serde_json::{json, Value};
use syn::Expr;
use syn::visit_mut::{self, VisitMut};
fn normalize_tokens(stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    use proc_macro2::{Group, TokenTree};
    let tokens: Vec<_> = stream.into_iter().collect();
    let mut result = proc_macro2::TokenStream::new(); let mut index = 0;
    while index < tokens.len() {
        let token = match &tokens[index] {
            TokenTree::Group(group) => TokenTree::Group(Group::new(group.delimiter(), normalize_tokens(group.stream()))),
            TokenTree::Ident(id) if id == "unsafe" && !matches!(tokens.get(index+1), Some(TokenTree::Ident(next)) if next == "fn" || next == "extern" || next == "trait" || next == "impl") && !matches!(tokens.get(index+1), Some(TokenTree::Group(g)) if g.delimiter() == proc_macro2::Delimiter::Brace) => TokenTree::Ident(proc_macro2::Ident::new("__audit_unsafe", id.span())),
            TokenTree::Ident(id) if id == "self" => TokenTree::Ident(proc_macro2::Ident::new("__audit_self", id.span())),
            TokenTree::Ident(id) if id == "fn" && (matches!(index.checked_sub(1).and_then(|n| tokens.get(n)), Some(TokenTree::Punct(p)) if p.as_char()=='.') || !matches!(tokens.get(index+1), Some(TokenTree::Ident(next)) if next != "as")) => TokenTree::Ident(proc_macro2::Ident::new("__audit_fn", id.span())),
            TokenTree::Ident(id) if id == "_" && matches!(tokens.get(index+1), Some(TokenTree::Punct(p)) if p.as_char()==':') => TokenTree::Ident(proc_macro2::Ident::new("__audit_underscore", id.span())),
            other => other.clone(),
        };
        result.extend([token]); index += 1;
    }
    result
}


#[derive(Default)] struct Parentheses;
impl VisitMut for Parentheses {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        while let Expr::Paren(paren) = expr { *expr = *paren.expr.clone(); }
        visit_mut::visit_expr_mut(self, expr);
    }
}
#[derive(Default, Clone)] struct Review { counts: Vec<u64> }
#[derive(Default)] struct Children { expressions: Vec<Expr> }
impl VisitMut for Children {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        self.expressions.push(expr.clone());
        *expr = syn::parse_str("__audit_child").unwrap();
    }
}
fn shape(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut skeleton = expr.clone(); let mut children = Children::default();
    visit_mut::visit_expr_mut(&mut children, &mut skeleton);
    (skeleton, children.expressions)
}
fn integer(expr: &Expr) -> Option<u64> {
    let Expr::Lit(lit) = expr else { return None; };
    if !lit.attrs.is_empty() { return None; }
    let syn::Lit::Int(value) = &lit.lit else { return None; };
    if !value.suffix().is_empty() { return None; }
    value.base10_parse().ok()
}
fn expression(before: &Expr, after: &Expr, review: &mut Review) -> bool {
    if before == after { return true; }
    if let (Expr::Assign(old), Expr::Assign(new)) = (before, after) {
        if let Expr::Repeat(repeat) = &*new.right {
            if integer(&old.right) == Some(0) && integer(&repeat.expr) == Some(0)
                && repeat.attrs.is_empty()
            {
                if let Some(count) = integer(&repeat.len).filter(|n| *n > 0 && *n <= i32::MAX as u64) {
                    let mut restored = new.clone(); restored.right = repeat.expr.clone();
                    let mut branch = review.clone();
                    if expression(before, &Expr::Assign(restored), &mut branch) {
                        branch.counts.push(count); *review = branch; return true;
                    }
                }
            }
        }
    }
    let (a, ac) = shape(before); let (b, bc) = shape(after);
    if a != b || ac.len() != bc.len() { return false; }
    ac.iter().zip(bc.iter()).all(|(a,b)| expression(a,b,review))
}
fn compare(before: &syn::File, after: &syn::File) -> Option<Review> {
    let mut a = before.clone(); let mut b = after.clone();
    Parentheses.visit_file_mut(&mut a); Parentheses.visit_file_mut(&mut b);
    let mut ac = Children::default(); let mut bc = Children::default();
    ac.visit_file_mut(&mut a); bc.visit_file_mut(&mut b);
    if a != b || ac.expressions.len() != bc.expressions.len() { return None; }
    let mut review = Review::default();
    (ac.expressions.iter().zip(bc.expressions.iter()).all(|(a,b)| expression(a,b,&mut review))
        && !review.counts.is_empty()).then_some(review)
}
fn parse(source: &str) -> Result<syn::File, String> {
    source.parse().map_err(|e: proc_macro2::LexError| e.to_string())
        .and_then(|tokens| syn::parse2(normalize_tokens(tokens)).map_err(|e| e.to_string()))
}
fn main() {
    let cases = [
        ("*p = 0", "*p = [0; 16]", true),
        ("(*p).bytes = 0", "(*p).bytes = [0; 16]", true),
        ("*call() = 0", "*call() = [0; 16]", true),
        ("*p = 0; *q = 0", "*p = [0; 16]; *q = [0; 32]", true),
        ("*p = 0", "*p = [0; 1]", true),
        ("*p = 0", "*q = [0; 16]", false),
        ("*p = 1", "*p = [0; 16]", false),
        ("*p = zero()", "*p = [0; 16]", false),
        ("*p = 0", "*p = [1; 16]", false),
        ("*p = 0", "*p = [0; 0]", false),
        ("*p = 0", "*p = [0; count()]", false),
        ("*p = 0", "*p = [0.0; 16]", false),
        ("*p = 0", "*p = [false; 16]", false),
        ("*p = 0", "*p = [0u8; 16]", false),
        ("*p = 0", "*p = [0; 16usize]", false),
        ("return 0", "return [0; 16]", false),
        ("*call() = 0", "*other() = [0; 16]", false),
        ("*p = 0; side()", "*p = [0; 16]; side(); side()", false),
        ("*p = 0; *q = x", "*p = [0; 16]; *q = y", false),
        ("*(p as *mut u8) = 0", "*(p as *mut u64) = [0; 16]", false),
    ];
    for (old, new, expected) in cases {
        let a = syn::parse_file(&format!("fn test(){{{old};}}")).unwrap();
        let b = syn::parse_file(&format!("fn test(){{{new};}}")).unwrap();
        assert_eq!(compare(&a, &b).is_some(), expected, "{old} => {new}");
    }
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 { println!("{} self-checks passed", cases.len()); return; }
    let input: Value = serde_json::from_slice(&std::fs::read(&args[1]).unwrap()).unwrap(); let mut rows = Vec::new();
    for row in input["records"].as_array().unwrap() { for change in row["changes"].as_array().unwrap() {
        let old = change["parent"]["code"].as_str().unwrap(); let new = change["candidate"]["code"].as_str().unwrap();
        let raw_old = syn::parse_file(old); let raw_new = syn::parse_file(new);
        let normalized = raw_old.is_err() || raw_new.is_err();
        let before = if normalized { parse(old) } else { raw_old.clone().map_err(|e| e.to_string()) };
        let after = if normalized { parse(new) } else { raw_new.clone().map_err(|e| e.to_string()) };
        let result = match (&before, &after) { (Ok(a), Ok(b)) => compare(a, b), _ => None };
        rows.push(json!({"fixture":row["fixture"],"language":change["language"],"gopairreturns":change["gopairreturns"],"address":change["address"],"name":change["name"],"raw_parent_parse_error":raw_old.err().map(|e|e.to_string()),"raw_candidate_parse_error":raw_new.err().map(|e|e.to_string()),"identifier_normalization_only":normalized,"parent_parse_error":before.err(),"candidate_parse_error":after.err(),"classified":result.is_some(),"array_zero_stores":result.as_ref().map(|r|r.counts.len()),"byte_counts":result.as_ref().map(|r|&r.counts)}));
    }}
    let pending = rows.iter().filter(|r|r["classified"]!=true).count();
    let result = json!({"scope":"Recursive paired AST comparison permits replacing only an assignment RHS literal zero with a positive-length repeat of literal zero. Assignment destinations, casts, calls, ordering and all surrounding AST remain identical. Byte counts are recorded; old scalar syntax alone does not prove storage width. This does not prove whole-program native equivalence or arbitrary pointer validity.","self_checks":cases.len(),"changed_records":rows.len(),"pending":pending,"records":rows});
    std::fs::write(&args[2],serde_json::to_string_pretty(&result).unwrap()+"\n").unwrap();
    println!("{} records, {} pending",result["changed_records"],pending);
}
