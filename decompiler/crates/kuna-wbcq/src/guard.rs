//! Path conditions as transition guards: evaluate a branch condition on a
//! concrete input, and print it.

use std::rc::Rc;

use kuna_verify::concolic::{Branch, Expr};

fn mask(w: u32) -> u128 {
    if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    }
}

fn signed(v: u128, w: u32) -> i128 {
    if w >= 128 {
        return v as i128;
    }
    let s = 1u128 << (w - 1);
    if v & s != 0 {
        (v | !mask(w)) as i128
    } else {
        v as i128
    }
}

/// The value of `e` with input byte `xi` set from `input`; `None` for an
/// expression the prototype cannot evaluate.
pub fn eval(e: &Expr, input: u64) -> Option<u128> {
    Some(match e {
        Expr::Var(name) => {
            let i: u32 = name.strip_prefix('x')?.parse().ok()?;
            u128::from((input >> (8 * i)) as u8)
        }
        Expr::Const(v, _) => *v,
        Expr::Extract(x, hi, lo) => (eval(x, input)? >> lo) & mask(hi - lo + 1),
        Expr::Concat(parts) => {
            let mut v = 0u128;
            for p in parts {
                v = (v << p.width()) | eval(p, input)?;
            }
            v
        }
        Expr::ZeroExt(x, _) => eval(x, input)?,
        Expr::SignExt(x, k) => (signed(eval(x, input)?, x.width()) as u128) & mask(x.width() + k),
        Expr::Test(op, a, b) => {
            let (w, x, y) = (a.width(), eval(a, input)?, eval(b, input)?);
            u128::from(match *op {
                "=" => x == y,
                "distinct" => x != y,
                "bvult" => x < y,
                "bvule" => x <= y,
                "bvslt" => signed(x, w) < signed(y, w),
                "bvsle" => signed(x, w) <= signed(y, w),
                _ => return None,
            })
        }
        Expr::Overflow(add, a, b) => {
            let w = a.width();
            let (x, y) = (signed(eval(a, input)?, w), signed(eval(b, input)?, w));
            let r = if *add {
                x.checked_add(y)
            } else {
                x.checked_sub(y)
            };
            let lo = -(1i128 << (w - 1));
            let hi = (1i128 << (w - 1)) - 1;
            u128::from(r.is_none_or(|r| r < lo || r > hi))
        }
        Expr::Op(op, args, w) => {
            let w = *w;
            let x = eval(&args[0], input)?;
            let y = match args.get(1) {
                Some(b) => eval(b, input)?,
                None => 0,
            };
            let aw = args[0].width();
            let v = match *op {
                "bvadd" => x.wrapping_add(y),
                "bvsub" => x.wrapping_sub(y),
                "bvmul" => x.wrapping_mul(y),
                "bvudiv" => x.checked_div(y).unwrap_or(mask(w)),
                "bvurem" => x.checked_rem(y).unwrap_or(x),
                "bvsdiv" => signed(x, aw).checked_div(signed(y, aw))? as u128,
                "bvsrem" => signed(x, aw).checked_rem(signed(y, aw))? as u128,
                "bvand" => x & y,
                "bvor" => x | y,
                "bvxor" => x ^ y,
                "bvnot" => !x,
                "bvneg" => x.wrapping_neg(),
                "bvshl" => {
                    if y >= u128::from(w) {
                        0
                    } else {
                        x << y
                    }
                }
                "bvlshr" => {
                    if y >= u128::from(w) {
                        0
                    } else {
                        x >> y
                    }
                }
                "bvashr" => (signed(x, w) >> y.min(u128::from(w) - 1)) as u128,
                _ => return None,
            };
            v & mask(w)
        }
    })
}

/// Whether `input` takes every branch of `path` the way it went.
pub fn holds(path: &[Branch], input: u64) -> Option<bool> {
    for b in path {
        if (eval(&b.cond, input)? != 0) != b.taken {
            return Some(false);
        }
    }
    Some(true)
}

/// Fold the byte plumbing a register read leaves: an extract of a concat
/// picks its part, and consecutive extracts of one value join.
fn simplify(e: &Rc<Expr>) -> Rc<Expr> {
    match &**e {
        Expr::Extract(x, hi, lo) => {
            let x = simplify(x);
            if *lo == 0 && *hi + 1 == x.width() {
                return x;
            }
            match &*x {
                Expr::ZeroExt(y, _) if *hi < y.width() => {
                    simplify(&Rc::new(Expr::Extract(Rc::clone(y), *hi, *lo)))
                }
                Expr::ZeroExt(y, _) if *lo >= y.width() => Rc::new(Expr::Const(0, hi - lo + 1)),
                Expr::Extract(y, _, l2) => {
                    simplify(&Rc::new(Expr::Extract(Rc::clone(y), l2 + hi, l2 + lo)))
                }
                Expr::Concat(parts) => {
                    let mut off = 0;
                    for p in parts.iter().rev() {
                        let w = p.width();
                        if *lo >= off && *hi < off + w {
                            return simplify(&Rc::new(Expr::Extract(
                                Rc::clone(p),
                                hi - off,
                                lo - off,
                            )));
                        }
                        off += w;
                    }
                    Rc::new(Expr::Extract(x, *hi, *lo))
                }
                _ => Rc::new(Expr::Extract(x, *hi, *lo)),
            }
        }
        Expr::Concat(parts) => {
            let mut flat: Vec<Rc<Expr>> = Vec::new();
            for p in parts {
                let p = simplify(p);
                match &*p {
                    Expr::Concat(inner) => flat.extend(inner.iter().cloned()),
                    _ => flat.push(p),
                }
            }
            let mut merged: Vec<Rc<Expr>> = Vec::new();
            for p in flat {
                if let (Some(last), Expr::Extract(y, h2, l2)) = (merged.last(), &*p) {
                    if let Expr::Extract(x, h1, l1) = &**last {
                        if format!("{x:?}") == format!("{y:?}") && *l1 == h2 + 1 {
                            let joined = simplify(&Rc::new(Expr::Extract(Rc::clone(x), *h1, *l2)));
                            *merged.last_mut().expect("a last part") = joined;
                            continue;
                        }
                    }
                }
                merged.push(p);
            }
            if merged.len() == 1 {
                merged.pop().expect("one part")
            } else {
                Rc::new(Expr::Concat(merged))
            }
        }
        Expr::Op(op, args, w) => Rc::new(Expr::Op(op, args.iter().map(simplify).collect(), *w)),
        Expr::Test(op, a, b) => Rc::new(Expr::Test(op, simplify(a), simplify(b))),
        Expr::ZeroExt(x, k) => Rc::new(Expr::ZeroExt(simplify(x), *k)),
        Expr::SignExt(x, k) => Rc::new(Expr::SignExt(simplify(x), *k)),
        Expr::Overflow(add, a, b) => Rc::new(Expr::Overflow(*add, simplify(a), simplify(b))),
        _ => Rc::clone(e),
    }
}

fn is_input(e: &Expr) -> bool {
    matches!(e, Expr::Concat(parts) if parts.len() == 4
        && parts.iter().rev().enumerate().all(|(i, p)| matches!(&**p, Expr::Var(n) if **n == *format!("x{i}"))))
}

fn render_expr(e: &Rc<Expr>) -> String {
    if is_input(e) {
        return "in".into();
    }
    match &**e {
        Expr::Var(n) => match n.strip_prefix('x').and_then(|i| i.parse::<u32>().ok()) {
            Some(i) => format!("in[{}:{}]", 8 * i + 7, 8 * i),
            None => n.to_string(),
        },
        Expr::Test("=", a, b)
            if matches!(**b, Expr::Const(0, _)) && matches!(**a, Expr::Test(..)) =>
        {
            format!("!{}", render_expr(a))
        }
        Expr::Test(op, a, b)
            if matches!(**b, Expr::Const(0, _)) && matches!(&**a, Expr::Op("bvsub", _, _)) =>
        {
            let Expr::Op(_, args, _) = &**a else {
                unreachable!()
            };
            render_expr(&Rc::new(Expr::Test(
                op,
                Rc::clone(&args[0]),
                Rc::clone(&args[1]),
            )))
        }
        Expr::Const(v, _) => format!("{v:#x}"),
        Expr::Extract(x, hi, lo) => format!("{}[{hi}:{lo}]", render_expr(x)),
        Expr::Concat(parts)
            if parts.len() > 1
                && parts[..parts.len() - 1]
                    .iter()
                    .all(|p| matches!(**p, Expr::Const(0, _))) =>
        {
            render_expr(&parts[parts.len() - 1])
        }
        Expr::Concat(parts) => format!(
            "concat({})",
            parts.iter().map(render_expr).collect::<Vec<_>>().join(", ")
        ),
        Expr::ZeroExt(x, _) => render_expr(x),
        Expr::SignExt(x, k) => format!("sext{k}({})", render_expr(x)),
        Expr::Test(op, a, b) => {
            let sym = match *op {
                "=" => "==",
                "distinct" => "!=",
                "bvult" => "<u",
                "bvule" => "<=u",
                "bvslt" => "<s",
                "bvsle" => "<=s",
                other => other,
            };
            format!("({} {sym} {})", render_expr(a), render_expr(b))
        }
        Expr::Overflow(add, a, b) => format!(
            "{}({}, {})",
            if *add { "scarry" } else { "sborrow" },
            render_expr(a),
            render_expr(b)
        ),
        Expr::Op(op, args, _) => format!(
            "{op}({})",
            args.iter().map(render_expr).collect::<Vec<_>>().join(", ")
        ),
    }
}

/// One path's condition, as text: its branches, each negated where not taken.
pub fn render(path: &[Branch]) -> String {
    if path.is_empty() {
        return "true".into();
    }
    path.iter()
        .map(|b| {
            let c = render_expr(&simplify(&b.cond));
            match (b.taken, c.strip_prefix('!')) {
                (true, _) => c,
                (false, Some(inner)) => inner.to_string(),
                (false, None) => format!("!{c}"),
            }
        })
        .collect::<Vec<_>>()
        .join(" && ")
}
