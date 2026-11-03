from common import *
import shutil
root=w/'ast-audit';(root/'src').mkdir(parents=True,exist_ok=False)
old=Path('/tmp/kuna-rust-integer-conversions/ast-audit')
for name in ['Cargo.toml','Cargo.lock']:shutil.copy2(old/name,root/name)
s=(old/'src/main.rs').read_text()
extra=r'''
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

'''
s=s.replace('fn main() {',extra+'fn main() {')
needle='''            record["original_parse_errors"] = json!({''';replacement='''            record["whole_assignment_rhs_conversions"] = match (&parsed_before, &parsed_after) {
                (Ok(a), Ok(b)) => json!(whole_assignment_conversions(a, b)),
                _ => Value::Null,
            };
            record["original_parse_errors"] = json!({''';assert needle in s;s=s.replace(needle,replacement)
s=s.replace('"scope": "Structural and modular-algebra classification only:', '"strict_assignment_scope": "A nonnull whole_assignment_rhs_conversions value proves that, after removing parentheses while retaining syntax-tree grouping, the complete candidate file equals the parent after removing newly added primitive casts or exact integer identity wrappers around complete assignment RHS expressions only. Existing RHS casts and every LHS are retained. Null requires the broader classification and review.", "scope": "Structural and modular-algebra classification only:')
(root/'src/main.rs').write_text(s)
print('Prepared STORE assignment audit',flush=True)
