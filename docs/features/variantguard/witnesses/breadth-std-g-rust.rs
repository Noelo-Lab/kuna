// A std-linked `-g` Rust witness: the enum shapes a real program actually has.
use std::collections::{BTreeMap, HashMap};
use std::error::Error;

pub enum Tree { Leaf(i64), Node(Box<Tree>, Box<Tree>) }
pub enum Expr { Lit(i64), Bin(Box<Expr>, Box<Expr>), Neg(Box<Expr>) }
#[derive(Clone, Copy)]
pub struct Pair { pub a: u64, pub b: u64 }

#[inline(never)]
pub fn depth(t: &Tree) -> i64 {
    match t { Tree::Leaf(v) => *v, Tree::Node(l, r) => 1 + depth(l).max(depth(r)) }
}

#[inline(never)]
pub fn eval(e: &Expr) -> i64 {
    match e { Expr::Lit(v) => *v, Expr::Bin(a, b) => eval(a) + eval(b), Expr::Neg(a) => -eval(a) }
}

#[inline(never)]
pub fn divide(a: u64, b: u64) -> Result<u64, u64> {
    if b == 0 { return Err(a); }
    Ok(a / b)
}

#[inline(never)]
pub fn use_divide(a: u64, b: u64) -> u64 {
    match divide(a, b) { Ok(v) => v, Err(e) => e + 100 }
}

#[inline(never)]
pub fn checked(a: u64, b: u64) -> Result<u64, u32> {
    if b == 0 { return Err(7); }
    Ok(a * b)
}

#[inline(never)]
pub fn use_checked(a: u64, b: u64) -> u64 {
    match checked(a, b) { Ok(v) => v, Err(e) => e as u64 }
}

#[inline(never)]
pub fn find(m: &HashMap<u64, Pair>, k: u64) -> Option<Pair> { m.get(&k).copied() }

#[inline(never)]
pub fn tally(m: &BTreeMap<u64, u64>) -> u64 { m.values().copied().sum() }

#[inline(never)]
pub fn parse(s: &str) -> Result<u64, Box<dyn Error>> { Ok(s.trim().parse::<u64>()?) }

#[inline(never)]
pub fn classify(x: u64) -> Result<Pair, String> {
    if x > 100 { return Err(format!("too big: {x}")); }
    Ok(Pair { a: x, b: x + 1 })
}

#[inline(never)]
pub fn use_classify(x: u64) -> u64 {
    match classify(x) { Ok(p) => p.a + p.b, Err(s) => s.len() as u64 }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args.len() as u64;
    let mut hm: HashMap<u64, Pair> = HashMap::new();
    hm.insert(n, Pair { a: n, b: n * 2 });
    let mut bm: BTreeMap<u64, u64> = BTreeMap::new();
    bm.insert(n, n * 3);
    let t = Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)));
    let e = Expr::Bin(Box::new(Expr::Lit(3)), Box::new(Expr::Neg(Box::new(Expr::Lit(4)))));
    let acc = use_divide(n, n) + use_checked(n, n) + tally(&bm)
        + find(&hm, n).map(|p| p.a + p.b).unwrap_or(0)
        + parse(&args[0]).unwrap_or(0)
        + use_classify(n)
        + depth(&t) as u64 + eval(&e) as u64;
    println!("{acc}");
}
