use serde_json::{json, Value};
use syn::visit_mut::{self, VisitMut};
use syn::{BinOp, Expr, Type};

fn integer(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.qself.is_none() && p.path.leading_colon.is_none() && p.path.segments.len() == 1 && matches!(p.path.segments[0].ident.to_string().as_str(), "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64"))
}

#[derive(Default)]
struct Normalize { casts: usize, identities: usize, assignments: usize, reassociations: usize, negative_additions: usize }

fn local_place(expr: &Expr) -> bool {
    match expr {
        Expr::Path(p) => p.qself.is_none() && p.path.segments.len() == 1,
        Expr::Field(f) => local_place(&f.base),
        _ => false,
    }
}

fn associative(op: &BinOp) -> Option<u8> {
    match op { BinOp::Add(_) => Some(0), BinOp::Mul(_) => Some(1), BinOp::BitAnd(_) => Some(2), BinOp::BitOr(_) => Some(3), BinOp::BitXor(_) => Some(4), _ => None }
}

fn flatten(expr: Expr, operator: &BinOp, terms: &mut Vec<Expr>) {
    match expr {
        Expr::Binary(b) if b.attrs.is_empty() && associative(&b.op) == associative(operator) => {
            flatten(*b.left, operator, terms);
            flatten(*b.right, operator, terms);
        }
        other => terms.push(other),
    }
}

fn flatten_sum(expr: Expr, negative: bool, terms: &mut Vec<(bool, Expr)>) {
    match expr {
        Expr::Binary(b) if b.attrs.is_empty() && matches!(b.op, BinOp::Add(_) | BinOp::Sub(_)) => {
            let subtract = matches!(b.op, BinOp::Sub(_));
            flatten_sum(*b.left, negative, terms);
            flatten_sum(*b.right, negative ^ subtract, terms);
        }
        Expr::Unary(u) if matches!(u.op, syn::UnOp::Neg(_)) && matches!(&*u.expr, Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Int(_))) => terms.push((!negative, *u.expr)),
        other => terms.push((negative, other)),
    }
}

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
            TokenTree::Ident(id) if id == "self" => TokenTree::Ident(proc_macro2::Ident::new("__audit_self", id.span())),
            TokenTree::Ident(id) if id == "fn" && !matches!(tokens.get(index+1), Some(TokenTree::Ident(_))) => TokenTree::Ident(proc_macro2::Ident::new("__audit_fn", id.span())),
            TokenTree::Ident(id) if id == "_" && matches!(tokens.get(index+1), Some(TokenTree::Punct(p)) if p.as_char()==':') => TokenTree::Ident(proc_macro2::Ident::new("__audit_underscore", id.span())),
            other => other.clone(),
        };
        result.extend([token]); index += 1;
    }
    result
}

impl VisitMut for Normalize {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        loop {
            let replacement = match expr {
                Expr::Paren(p) => Some(*p.expr.clone()),
                Expr::Cast(c) if integer(&c.ty) => { self.casts += 1; Some(*c.expr.clone()) }
                Expr::Call(c) if c.args.len() == 1 => {
                    let identity = if let Expr::Path(p) = &*c.func {
                        let segments: Vec<_> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
                        p.qself.is_none() && p.path.leading_colon.is_some() && segments == ["core", "convert", "identity"] &&
                            matches!(&p.path.segments[2].arguments, syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 && matches!(&args.args[0], syn::GenericArgument::Type(t) if integer(t)))
                    } else { false };
                    if identity { self.identities += 1; Some(c.args[0].clone()) } else { None }
                }
                Expr::Binary(b) if local_place(&b.left) => {
                    let op = match b.op {
                        BinOp::AddAssign(_) => Some(BinOp::Add(Default::default())),
                        BinOp::SubAssign(_) => Some(BinOp::Sub(Default::default())),
                        BinOp::MulAssign(_) => Some(BinOp::Mul(Default::default())),
                        BinOp::BitAndAssign(_) => Some(BinOp::BitAnd(Default::default())),
                        BinOp::BitOrAssign(_) => Some(BinOp::BitOr(Default::default())),
                        BinOp::BitXorAssign(_) => Some(BinOp::BitXor(Default::default())),
                        _ => None,
                    };
                    op.map(|op| {
                        self.assignments += 1;
                        let mut binary = b.clone(); binary.op = op;
                        Expr::Assign(syn::ExprAssign { attrs: Vec::new(), left: b.left.clone(), eq_token: Default::default(), right: Box::new(Expr::Binary(binary)) })
                    })
                }
                _ => None,
            };
            if let Some(replacement) = replacement { *expr = replacement; } else { break; }
        }
        visit_mut::visit_expr_mut(self, expr);
        if let Expr::Binary(binary) = expr {
            if matches!(binary.op, BinOp::Add(_)) {
                if let Expr::Unary(negative) = &*binary.right {
                    if matches!(negative.op, syn::UnOp::Neg(_)) && matches!(&*negative.expr, Expr::Lit(lit) if matches!(lit.lit, syn::Lit::Int(_))) {
                        self.negative_additions += 1;
                        binary.op = BinOp::Sub(Default::default());
                        binary.right = negative.expr.clone();
                    }
                }
            }
            if matches!(binary.op, BinOp::Add(_) | BinOp::Sub(_)) {
                let mut terms = Vec::new(); flatten_sum(Expr::Binary(binary.clone()), false, &mut terms);
                if terms.len() > 2 { self.reassociations += 1; }
                let mut terms = terms.into_iter(); let (negative, mut value) = terms.next().unwrap();
                if negative { value = Expr::Unary(syn::ExprUnary { attrs: Vec::new(), op: syn::UnOp::Neg(Default::default()), expr: Box::new(value) }); }
                for (negative, term) in terms {
                    let op = if negative { BinOp::Sub(Default::default()) } else { BinOp::Add(Default::default()) };
                    value = Expr::Binary(syn::ExprBinary { attrs: Vec::new(), left: Box::new(value), op, right: Box::new(term) });
                }
                *expr = value;
                return;
            }
            if associative(&binary.op).is_some() {
                let op = binary.op; let mut terms = Vec::new();
                flatten(Expr::Binary(binary.clone()), &op, &mut terms);
                if terms.len() > 2 { self.reassociations += 1; }
                let mut terms = terms.into_iter(); let mut value = terms.next().unwrap();
                for term in terms { value = Expr::Binary(syn::ExprBinary { attrs: Vec::new(), left: Box::new(value), op, right: Box::new(term) }); }
                *expr = value;
            }
        }
    }
}


#[derive(Default)]
struct Parentheses;
impl VisitMut for Parentheses {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        while let Expr::Paren(p) = expr { *expr = *p.expr.clone(); }
        visit_mut::visit_expr_mut(self, expr);
    }
}

#[derive(Default)]
struct Assignments { values: Vec<(Expr, Expr)> }
impl VisitMut for Assignments {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        if let Expr::Assign(a) = expr { self.values.push((*a.left.clone(), *a.right.clone())); }
    }
}

fn identity_argument(expr: &Expr) -> Option<&Expr> {
    let Expr::Call(c) = expr else { return None; };
    if c.args.len() != 1 { return None; }
    let Expr::Path(p) = &*c.func else { return None; };
    let segments: Vec<_> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
    if p.qself.is_some() || p.path.leading_colon.is_none() || segments != ["core", "convert", "identity"] { return None; }
    if !matches!(&p.path.segments[2].arguments, syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 && matches!(&args.args[0], syn::GenericArgument::Type(t) if integer(t))) { return None; }
    Some(&c.args[0])
}

struct AssignmentConversions {
    baseline: Vec<(Expr, Expr)>, index: usize, conversions: usize, invalid: bool,
}
impl VisitMut for AssignmentConversions {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        if let Expr::Assign(a) = expr {
            let Some((left,right)) = self.baseline.get(self.index) else { self.invalid = true; return; };
            self.index += 1;
            if format!("{left:?}") != format!("{:?}", a.left) { self.invalid = true; return; }
            if format!("{right:?}") == format!("{:?}", a.right) { return; }
            let mut value = &*a.right; let mut casts = 0; let mut matched = false;
            for _ in 0..4 {
                if let Expr::Cast(c) = value {
                    if !integer(&c.ty) { break; }
                    casts += 1; value = &c.expr;
                } else if let Some(inner) = identity_argument(value) { value = inner; }
                else { break; }
                if casts > 0 && format!("{right:?}") == format!("{value:?}") { matched = true; break; }
            }
            if matched { self.conversions += 1; a.right = Box::new(right.clone()); }
            else { self.invalid = true; }
        }
    }
}

fn whole_assignment_conversions(before: &syn::File, after: &syn::File) -> Option<usize> {
    let mut before = before.clone(); let mut after = after.clone();
    Parentheses.visit_file_mut(&mut before); Parentheses.visit_file_mut(&mut after);
    let mut assignments = Assignments::default(); assignments.visit_file_mut(&mut before);
    let mut normalize = AssignmentConversions { baseline: assignments.values, index: 0, conversions: 0, invalid: false };
    normalize.visit_file_mut(&mut after);
    if !normalize.invalid && normalize.index == normalize.baseline.len() && normalize.conversions > 0 && format!("{before:?}") == format!("{after:?}") { Some(normalize.conversions) } else { None }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data: Value = serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let mut good = Vec::new(); let mut pending = Vec::new();
    for row in data["records"].as_array().unwrap() {
        for change in row["changes"].as_array().unwrap() {
            let mut record = json!({"fixture": row["fixture"], "address": change["address"], "name": change["name"], "gopairreturns": change["gopairreturns"]});
            let fields = change["fields"].as_array().unwrap().iter().all(|v| matches!(v.as_str(), Some("code" | "line_mappings")));
            let before = change["parent"]["code"].as_str().unwrap(); let after = change["candidate"]["code"].as_str().unwrap();
            let mut parsed_before = syn::parse_file(before); let mut parsed_after = syn::parse_file(after);
            record["whole_assignment_rhs_conversions"] = match (&parsed_before, &parsed_after) {
                (Ok(a), Ok(b)) => json!(whole_assignment_conversions(a, b)),
                _ => Value::Null,
            };
            record["original_parse_errors"] = json!({"parent":parsed_before.as_ref().err().map(|e|format!("{} at {:?}",e,e.span())),"candidate":parsed_after.as_ref().err().map(|e|format!("{} at {:?}",e,e.span()))});
            if parsed_before.is_err() || parsed_after.is_err() {
                if let (Ok(a),Ok(b)) = (before.parse::<proc_macro2::TokenStream>(),after.parse::<proc_macro2::TokenStream>()) {
                    parsed_before = syn::parse_file(&normalize_tokens(a.clone(), false).to_string());
                    parsed_after = syn::parse_file(&normalize_tokens(b.clone(), false).to_string());
                    if let (Ok(x), Ok(y)) = (&parsed_before, &parsed_after) {
                        record["whole_assignment_rhs_conversions"] = json!(whole_assignment_conversions(x, y));
                        record["strict_assignment_identifier_normalization"] = json!(true);
                    } else {
                        parsed_before = syn::parse_file(&normalize_tokens(a, true).to_string());
                        parsed_after = syn::parse_file(&normalize_tokens(b, true).to_string());
                        record["primitive_cast_erasure_before_parse"] = json!(true);
                    }
                    record["review_only_lexical_normalization_before_parse"] = json!("Erase primitive integer casts; rename keyword identifiers fn/self and underscore path roots to reserved audit identifiers. Raw parse status is retained separately. No emitted or native-tested body is rewritten.");
                }
            }
            match (parsed_before, parsed_after) {
                (Ok(mut a), Ok(mut b)) => {
                    let mut x = Normalize::default(); let mut y = Normalize::default();
                    x.visit_file_mut(&mut a); y.visit_file_mut(&mut b);
                    record["primitive_cast_counts"] = json!([x.casts, y.casts]);
                    record["identity_counts"] = json!([x.identities, y.identities]);
                    record["compound_assignment_counts"] = json!([x.assignments, y.assignments]);
                    record["modular_reassociation_counts"] = json!([x.reassociations, y.reassociations]);
                    record["negative_constant_addition_counts"] = json!([x.negative_additions, y.negative_additions]);
                    if fields && format!("{a:?}") == format!("{b:?}") { good.push(record); }
                    else {
                        let before = format!("{a:?}"); let after = format!("{b:?}");
                        let index = before.bytes().zip(after.bytes()).position(|(a,b)| a!=b).unwrap_or(before.len().min(after.len()));
                        record["first_difference"] = json!({"offset":index,"parent":before.chars().skip(index.saturating_sub(100)).take(600).collect::<String>(),"candidate":after.chars().skip(index.saturating_sub(100)).take(600).collect::<String>()});
                        record["reason"] = json!("Normalized syntax trees or metadata differ"); pending.push(record);
                    }
                }
                (a, b) => {
                    record["reason"] = json!({"parent_parse_error": a.err().map(|e| format!("{} at {:?}",e,e.span())), "candidate_parse_error": b.err().map(|e| format!("{} at {:?}",e,e.span()))});
                    pending.push(record);
                }
            }
        }
    }
    let result = json!({"strict_assignment_scope": "A nonnull whole_assignment_rhs_conversions value proves that, after any separately recorded identifier-only parser normalization and removing parentheses while retaining syntax-tree grouping, the complete candidate file equals the parent after removing newly added primitive casts or exact integer identity wrappers around complete assignment RHS expressions only. Existing RHS casts and every LHS are retained. Null requires the broader classification and review.", "scope": "Structural and modular-algebra classification only: primitive integer casts and exact core integer identity calls are stripped from both trees; compound assignments to local names or their fields are expanded; associative integer operators and mixed addition/subtraction are reassociated without operand reordering; addition of a negative integer constant becomes subtraction. For originally unparseable functions only, primitive casts are lexically erased and invalid fn/self/underscore-path identifiers are renamed for structural review; raw parse errors remain recorded. Parentheses are removed in the AST while other operator nesting remains. The same-width primitive guards and wrapping arithmetic are required for the algebraic classification. Native equivalence, type correctness, and preservation of existing casts are not established by this audit alone.", "verified": good.len(), "pending_count": pending.len(), "records": good, "pending": pending});
    std::fs::write(&args[2], serde_json::to_string_pretty(&result).unwrap()).unwrap();
    println!("AST structure verified={} pending={}", result["verified"], result["pending_count"]);
}
