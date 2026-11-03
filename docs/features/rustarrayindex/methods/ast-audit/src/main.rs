use serde_json::{json, Value};
use syn::{Expr, Type};
use syn::visit_mut::{self, VisitMut};
fn normalize_tokens(stream: proc_macro2::TokenStream, erase_integer_casts: bool) -> proc_macro2::TokenStream {
    use proc_macro2::{Group, TokenTree};
    let tokens: Vec<_> = stream.into_iter().collect();
    let mut result = proc_macro2::TokenStream::new(); let mut index = 0;
    while index < tokens.len() {
        if erase_integer_casts && matches!(&tokens[index], TokenTree::Ident(id) if id == "as") && index + 1 < tokens.len()
            && matches!(&tokens[index+1], TokenTree::Ident(id) if matches!(id.to_string().as_str(), "i8"|"u8"|"i16"|"u16"|"i32"|"u32"|"i64"|"u64"))
        { index += 2; continue; }
        let token = match &tokens[index] {
            TokenTree::Group(group) => TokenTree::Group(Group::new(group.delimiter(), normalize_tokens(group.stream(), erase_integer_casts))),
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
fn same(a: &Expr, b: &Expr) -> bool { format!("{a:?}") == format!("{b:?}") }
fn primitive(ty: &Type) -> bool {
    matches!(ty, Type::Path(path) if path.qself.is_none() && path.path.leading_colon.is_none()
        && path.path.segments.len() == 1 && matches!(path.path.segments[0].ident.to_string().as_str(),
            "i8"|"u8"|"i16"|"u16"|"i32"|"u32"|"i64"|"u64"))
}
fn context(expr: &Expr) -> Option<Expr> {
    let Expr::Cast(cast) = expr else { return None; };
    if !primitive(&cast.ty) { return None; }
    let Expr::Call(call) = &*cast.expr else { return None; };
    if call.args.len() != 1 { return None; }
    let Expr::Path(path) = &*call.func else { return None; };
    let parts: Vec<_> = path.path.segments.iter().map(|s| s.ident.to_string()).collect();
    if path.qself.is_some() || path.path.leading_colon.is_none() || parts != ["core", "convert", "identity"] { return None; }
    let syn::PathArguments::AngleBracketed(args) = &path.path.segments[2].arguments else { return None; };
    if args.args.len() != 1 { return None; }
    let syn::GenericArgument::Type(ty) = &args.args[0] else { return None; };
    (format!("{ty:?}") == format!("{:?}", cast.ty)).then(|| call.args[0].clone())
}
#[derive(Default)] struct OldIndices { indices: Vec<Expr> }
impl VisitMut for OldIndices {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        if let Expr::Index(index) = expr { self.indices.push(*index.index.clone()); }
    }
}
struct Restore { old: Vec<Expr>, visited: usize, casts: usize, contexts: usize, rejected: usize }
impl VisitMut for Restore {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        let Expr::Index(index) = expr else { return; };
        let before = self.old.get(self.visited); self.visited += 1;
        let Some(before) = before else { self.rejected += 1; return; };
        if same(&index.index, before) { return; }
        let Expr::Cast(cast) = &*index.index else { self.rejected += 1; return; };
        if !matches!(&*cast.ty, Type::Path(path) if path.qself.is_none() && path.path.leading_colon.is_none() && path.path.is_ident("usize")) {
            self.rejected += 1; return;
        }
        if same(&cast.expr, before) { index.index = Box::new(before.clone()); self.casts += 1; return; }
        if context(&cast.expr).is_some_and(|inner| same(&inner, before)) {
            index.index = Box::new(before.clone()); self.casts += 1; self.contexts += 1; return;
        }
        self.rejected += 1;
    }
}
fn compare(before: &syn::File, after: &syn::File) -> Option<(usize, usize)> {
    let mut before = before.clone(); let mut after = after.clone();
    Parentheses.visit_file_mut(&mut before); Parentheses.visit_file_mut(&mut after);
    let mut old = OldIndices::default(); old.visit_file_mut(&mut before);
    let mut restore = Restore { old: old.indices, visited: 0, casts: 0, contexts: 0, rejected: 0 };
    restore.visit_file_mut(&mut after);
    (restore.casts > 0 && restore.rejected == 0 && restore.visited == restore.old.len()
        && format!("{before:?}") == format!("{after:?}")).then_some((restore.casts, restore.contexts))
}
fn parse(source: &str) -> Result<syn::File, String> {
    source.parse().map_err(|e: proc_macro2::LexError| e.to_string())
        .and_then(|tokens| syn::parse2(normalize_tokens(tokens, false)).map_err(|e| e.to_string()))
}
fn main() {
    let cases = [
        ("a[i]", "a[i as usize]", true),
        ("a[i as u64]", "a[(i as u64) as usize]", true),
        ("a[i]", "a[j as usize]", false),
        ("a[i]", "b[i as usize]", false),
        ("a[i]", "a[(i as i32) as usize]", false),
        ("a[i]", "a[(i as isize) as usize]", false),
        ("a[i]", "a[i as usize]+callback()", false),
        ("a[7 >> n]", "a[(::core::convert::identity::<u64>(7 >> n) as u64) as usize]", true),
        ("a[7 >> n]", "a[(::core::convert::identity::<i32>(7 >> n) as u64) as usize]", false),
        ("a[i as usize]", "a[i as usize]", false),
        ("a[i as usize]+b[j]", "a[i as usize]+b[j as usize]", true),
        ("a[b[i]]", "a[b[i as usize] as usize]", true),
        ("&mut a[i]", "&mut a[i as usize]", true),
        ("a[i]=value", "a[i as usize]=value", true),
        ("a[i]=value", "a[i as usize]=value+1", false),
        ("a[i] as u8", "a[i as usize]", false),
        ("a[index()]+callback()", "a[index() as usize]+callback()+callback()", false),
        ("a[(::core::convert::identity::<u64>(7 >> n) as u64)]", "a[(::core::convert::identity::<u64>(7 >> n) as u64) as usize]", true),
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
        rows.push(json!({"fixture":row["fixture"],"language":change["language"],"gopairreturns":change["gopairreturns"],"address":change["address"],"name":change["name"],"raw_parent_parse_error":raw_old.err().map(|e|e.to_string()),"raw_candidate_parse_error":raw_new.err().map(|e|e.to_string()),"identifier_normalization_only":normalized,"parent_parse_error":before.err(),"candidate_parse_error":after.err(),"classified":result.is_some(),"new_index_casts":result.map(|r|r.0),"literal_width_contexts":result.map(|r|r.1)}));
    }}
    let pending = rows.iter().filter(|r|r["classified"]!=true).count();
    let result = json!({"scope":"Exact AST comparison after removing only new usize index casts and matching primitive integer identity context. Existing indices/casts, operands, calls, dereferences, addresses, assignments and surrounding AST remain identical. This is not whole-runtime native equivalence or global/address correctness proof.","self_checks":cases.len(),"changed_records":rows.len(),"pending":pending,"records":rows});
    std::fs::write(&args[2],serde_json::to_string_pretty(&result).unwrap()+"\n").unwrap();
    println!("{} records, {} pending",result["changed_records"],pending);
}
