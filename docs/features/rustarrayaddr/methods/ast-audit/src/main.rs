use serde_json::{json, Value};
use syn::{Expr, Type};
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
fn primitive(ty: &Type) -> bool {
    matches!(ty, Type::Path(path) if path.qself.is_none() && path.path.leading_colon.is_none()
        && path.path.segments.len() == 1 && matches!(path.path.segments[0].ident.to_string().as_str(),
            "i8"|"u8"|"i16"|"u16"|"i32"|"u32"|"i64"|"u64"))
}
fn literal(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Int(_)))
        || matches!(expr, Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) && literal(&unary.expr))
}
fn named(ty: &Type, name: &str) -> bool {
    matches!(ty, Type::Path(path) if path.qself.is_none() && path.path.leading_colon.is_none() && path.path.is_ident(name))
}
fn raw_place(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::RawAddr(raw) if matches!(raw.mutability, syn::PointerMutability::Mut(_)) => Some(*raw.expr.clone()),
        _ => None,
    }
}
#[derive(Default)] struct Restore { addresses: usize, integer_casts: usize, scales: Vec<String>, targets: Vec<String> }
impl VisitMut for Restore {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        let Expr::Cast(cast) = expr else { return; };
        if primitive(&cast.ty) {
            if let Some(place) = raw_place(&cast.expr) {
                cast.expr = Box::new(place); self.integer_casts += 1;
            }
            return;
        }
        if !matches!(&*cast.ty, Type::Ptr(ptr) if ptr.mutability.is_some()) { return; }
        let Expr::MethodCall(offset) = &*cast.expr else { return; };
        if offset.method != "wrapping_byte_offset" || offset.turbofish.is_some() || offset.args.len() != 1 { return; }
        let Some(place) = raw_place(&offset.receiver) else { return; };
        let Expr::MethodCall(mul) = &offset.args[0] else { return; };
        if mul.method != "wrapping_mul" || mul.turbofish.is_some() || mul.args.len() != 1 || !literal(&mul.args[0]) { return; }
        let Expr::Cast(index) = &*mul.receiver else { return; };
        if !named(&index.ty, "isize") { return; }
        let inner = if let Expr::Cast(source) = &*index.expr {
            if primitive(&source.ty) && literal(&source.expr) { *source.expr.clone() }
            else { *index.expr.clone() }
        } else { *index.expr.clone() };
        let subscript = if literal(&inner) { inner }
            else { Expr::Cast(syn::ExprCast { attrs: Vec::new(), expr: Box::new(inner), as_token: Default::default(), ty: Box::new(syn::parse_str("usize").unwrap()) }) };
        self.addresses += 1; self.scales.push(format!("{:?}", mul.args[0])); self.targets.push(format!("{:?}", cast.ty));
        *expr = Expr::Reference(syn::ExprReference { attrs: Vec::new(), and_token: Default::default(), mutability: Some(Default::default()),
            expr: Box::new(Expr::Index(syn::ExprIndex { attrs: Vec::new(), expr: Box::new(place), bracket_token: Default::default(), index: Box::new(subscript) })) });
    }
}
fn compare(before: &syn::File, after: &syn::File) -> Option<Restore> {
    let mut before = before.clone(); let mut after = after.clone();
    Parentheses.visit_file_mut(&mut before); Parentheses.visit_file_mut(&mut after);
    let mut restore = Restore::default(); restore.visit_file_mut(&mut after);
    (restore.addresses + restore.integer_casts > 0 && format!("{before:?}") == format!("{after:?}")).then_some(restore)
}
fn parse(source: &str) -> Result<syn::File, String> {
    source.parse().map_err(|e: proc_macro2::LexError| e.to_string())
        .and_then(|tokens| syn::parse2(normalize_tokens(tokens)).map_err(|e| e.to_string()))
}
fn main() {
    let cases = [
        ("&mut a[i as usize]", "(&raw mut a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32", true),
        ("&mut a[3]", "(&raw mut a).wrapping_byte_offset(((3 as i64) as isize).wrapping_mul(4)) as *mut i32", true),
        ("&mut a[i as usize] as *mut u8", "((&raw mut a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32) as *mut u8", true),
        ("a as i64", "(&raw mut a) as i64", true),
        ("a as i64 + call()", "(&raw mut a) as i64 + call()", true),
        ("&mut a[(::core::convert::identity::<u64>(7>>n) as u64) as usize]", "(&raw mut a).wrapping_byte_offset(((::core::convert::identity::<u64>(7>>n) as u64) as isize).wrapping_mul(4)) as *mut i32", true),
        ("&mut a[i as usize]", "(&raw mut b).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32", false),
        ("&mut a[i as usize]", "(&raw mut a).wrapping_byte_offset((j as isize).wrapping_mul(4)) as *mut i32", false),
        ("a[i as usize]", "(&raw mut a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32", false),
        ("&mut a[i as usize]", "(&mut a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32", false),
        ("&mut a[i as usize]", "(&raw const a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32", false),
        ("&mut a[i as usize]", "(&raw mut a).byte_offset((i as isize).wrapping_mul(4)) as *mut i32", false),
        ("&mut a[i as usize]", "(&raw mut a).wrapping_byte_offset((i as isize)*4) as *mut i32", false),
        ("&mut a[i as usize]", "(&raw mut a).wrapping_byte_offset(((i as i32) as isize).wrapping_mul(4)) as *mut i32", false),
        ("&mut a[i as usize]", "(&raw mut a).wrapping_byte_offset((i as isize).wrapping_mul(call())) as *mut i32", false),
        ("&mut a[i as usize] as *mut u8", "(&raw mut a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32", false),
        ("a as i64", "(&raw mut b) as i64", false),
        ("a as i64", "(&raw mut a) as u64", false),
        ("a as i64", "(&mut a) as i64", false),
        ("a as i64 + call()", "(&raw mut a) as i64 + call() + call()", false),
        ("&mut base()[index() as usize]", "(&raw mut base()).wrapping_byte_offset((index() as isize).wrapping_mul(4)) as *mut i32", true),
        ("a[i as usize] = x", "*((&raw mut a).wrapping_byte_offset((i as isize).wrapping_mul(4)) as *mut i32) = x", false),
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
        rows.push(json!({"fixture":row["fixture"],"language":change["language"],"gopairreturns":change["gopairreturns"],"address":change["address"],"name":change["name"],"raw_parent_parse_error":raw_old.err().map(|e|e.to_string()),"raw_candidate_parse_error":raw_new.err().map(|e|e.to_string()),"identifier_normalization_only":normalized,"parent_parse_error":before.err(),"candidate_parse_error":after.err(),"classified":result.is_some(),"raw_address_transformations":result.as_ref().map(|r|r.addresses),"array_integer_casts":result.as_ref().map(|r|r.integer_casts),"byte_scales":result.as_ref().map(|r|&r.scales),"pointer_targets":result.as_ref().map(|r|&r.targets)}));
    }}
    let pending = rows.iter().filter(|r|r["classified"]!=true).count();
    let result = json!({"scope":"Exact AST reconstruction of new raw-array address arithmetic and array-to-integer decay. Places, indices, integer contexts, existing casts, calls and surrounding AST remain identical. New byte scales and pointer targets are recorded, not inferred from old AST. This does not prove whole-program native equivalence or arbitrary field-base validity.","self_checks":cases.len(),"changed_records":rows.len(),"pending":pending,"records":rows});
    std::fs::write(&args[2],serde_json::to_string_pretty(&result).unwrap()+"\n").unwrap();
    println!("{} records, {} pending",result["changed_records"],pending);
}
