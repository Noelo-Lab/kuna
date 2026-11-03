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
            while let Expr::Binary(right) = &*binary.right {
                if associative(&binary.op).is_none() || associative(&binary.op) != associative(&right.op) { break; }
                self.reassociations += 1;
                let left = Expr::Binary(syn::ExprBinary { attrs: Vec::new(), left: binary.left.clone(), op: binary.op, right: right.left.clone() });
                binary.right = right.right.clone();
                binary.left = Box::new(left);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data: Value = serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let mut good = Vec::new(); let mut pending = Vec::new();
    for row in data["records"].as_array().unwrap() {
        for change in row["changes"].as_array().unwrap() {
            let mut record = json!({"fixture": row["fixture"], "address": change["address"], "name": change["name"], "gopairreturns": change["gopairreturns"]});
            let fields = change["fields"].as_array().unwrap().iter().all(|v| matches!(v.as_str(), Some("code" | "line_mappings")));
            match (syn::parse_file(change["parent"]["code"].as_str().unwrap()), syn::parse_file(change["candidate"]["code"].as_str().unwrap())) {
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
                    record["reason"] = json!({"parent_parse_error": a.err().map(|e| e.to_string()), "candidate_parse_error": b.err().map(|e| e.to_string())});
                    pending.push(record);
                }
            }
        }
    }
    let result = json!({"scope": "Structural and modular-algebra classification only: primitive integer casts and exact core integer identity calls are stripped from both trees; compound assignments to local names or their fields are expanded; associative integer operators are reassociated without operand reordering; addition of a negative integer constant becomes subtraction. Parentheses are removed in the AST while other operator nesting remains. The same-width primitive guards and wrapping arithmetic are required for the algebraic classification. Native equivalence, type correctness, and preservation of existing casts are not established by this audit alone.", "verified": good.len(), "pending_count": pending.len(), "records": good, "pending": pending});
    std::fs::write(&args[2], serde_json::to_string_pretty(&result).unwrap()).unwrap();
    println!("AST structure verified={} pending={}", result["verified"], result["pending_count"]);
}
