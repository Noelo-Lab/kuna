//! diffverify C5: concolic inputs. A machine run can record, next to each
//! value, an expression over the function's input bytes, and the condition of
//! every branch the function's own code takes. Negating one branch of a
//! recorded path, with the path before it held, gives z3 a question whose model
//! is an input that takes the other way. The model's bytes replace those inputs
//! in a new run, the rest of the state stays the base run's, and both sides
//! run and compare as on any run: z3 only picks inputs, it never decides a
//! verdict.
//!
//! An input byte is one whose value the run's random state chose: a register
//! or stack argument, or memory outside the image and the frame. A value that
//! goes through an operation the encoding does not cover, a call, or memory a
//! call may change, is concrete from there on; a model that misses its branch
//! for that is only a run that did not reach it.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};
use std::rc::Rc;

use kuna_num::opcodes::OpCode;

/// A bit-vector expression; booleans are one byte, 0 or 1, as in p-code.
#[derive(Debug)]
pub enum Expr {
    /// An input byte.
    Var(Rc<str>),
    Const(u128, u32),
    Op(&'static str, Vec<Rc<Expr>>, u32),
    Extract(Rc<Expr>, u32, u32),
    Concat(Vec<Rc<Expr>>),
    ZeroExt(Rc<Expr>, u32),
    SignExt(Rc<Expr>, u32),
    /// `(ite (op a b) 1 0)` as one byte.
    Test(&'static str, Rc<Expr>, Rc<Expr>),
    /// A signed overflow test of `+` (`true`) or `-`, as one byte.
    Overflow(bool, Rc<Expr>, Rc<Expr>),
}

impl Expr {
    pub fn width(&self) -> u32 {
        match self {
            Expr::Var(_) => 8,
            Expr::Const(_, w) | Expr::Op(_, _, w) => *w,
            Expr::Extract(_, hi, lo) => hi - lo + 1,
            Expr::Concat(parts) => parts.iter().map(|p| p.width()).sum(),
            Expr::ZeroExt(e, k) | Expr::SignExt(e, k) => e.width() + k,
            Expr::Test(..) | Expr::Overflow(..) => 8,
        }
    }
}

fn konst(v: u128, w: u32) -> Rc<Expr> {
    let m = if w >= 128 { u128::MAX } else { (1u128 << w) - 1 };
    Rc::new(Expr::Const(v & m, w))
}

fn fit(e: Rc<Expr>, w: u32) -> Rc<Expr> {
    let ew = e.width();
    match ew.cmp(&w) {
        std::cmp::Ordering::Equal => e,
        std::cmp::Ordering::Less => Rc::new(Expr::ZeroExt(e, w - ew)),
        std::cmp::Ordering::Greater => Rc::new(Expr::Extract(e, w - 1, 0)),
    }
}

/// The expression of p-code op `opc` with output `out` bytes over `ins`, each
/// an expression or a concrete value of its size; `None` for an op the
/// encoding does not cover (the output is then concrete).
pub fn op(opc: OpCode, out: u32, ins: &[(Option<Rc<Expr>>, u128, u32)]) -> Option<Rc<Expr>> {
    if ins.iter().all(|(e, _, _)| e.is_none()) {
        return None;
    }
    let ow = out * 8;
    let arg = |k: usize| -> Option<Rc<Expr>> {
        let (e, v, s) = ins.get(k)?;
        Some(e.clone().unwrap_or_else(|| konst(*v, s * 8)))
    };
    let same = |a: &Rc<Expr>, b: &Rc<Expr>| a.width() == b.width();
    use OpCode::*;
    let bin = |name: &'static str| -> Option<Rc<Expr>> {
        let (a, b) = (arg(0)?, arg(1)?);
        (same(&a, &b) && a.width() == ow).then(|| Rc::new(Expr::Op(name, vec![a, b], ow)))
    };
    let test = |name: &'static str, swap: bool| -> Option<Rc<Expr>> {
        let (a, b) = (arg(0)?, arg(1)?);
        if !same(&a, &b) || out != 1 {
            return None;
        }
        Some(Rc::new(if swap { Expr::Test(name, b, a) } else { Expr::Test(name, a, b) }))
    };
    let truth = |a: Rc<Expr>| Rc::new(Expr::Test("distinct", a.clone(), konst(0, a.width())));
    let logic = |name: &'static str| -> Option<Rc<Expr>> {
        let (a, b) = (arg(0)?, arg(1)?);
        (ow == 8).then(|| Rc::new(Expr::Op(name, vec![truth(a), truth(b)], 8)))
    };
    let shift = |name: &'static str, signed: bool| -> Option<Rc<Expr>> {
        let (a, b) = (arg(0)?, arg(1)?);
        if a.width() != ow {
            return None;
        }
        let wide = ow.max(b.width()) + 1;
        let a = if signed { Rc::new(Expr::SignExt(a, wide - ow)) } else { fit(a, wide) };
        let r = Rc::new(Expr::Op(name, vec![a, fit(b, wide)], wide));
        Some(Rc::new(Expr::Extract(r, ow - 1, 0)))
    };
    match opc {
        CPUI_COPY | CPUI_CAST => arg(0).filter(|a| a.width() == ow),
        CPUI_INT_ADD => bin("bvadd"),
        CPUI_INT_SUB => bin("bvsub"),
        CPUI_INT_MULT => bin("bvmul"),
        CPUI_INT_DIV => bin("bvudiv"),
        CPUI_INT_SDIV => bin("bvsdiv"),
        CPUI_INT_REM => bin("bvurem"),
        CPUI_INT_SREM => bin("bvsrem"),
        CPUI_INT_AND => bin("bvand"),
        CPUI_INT_OR => bin("bvor"),
        CPUI_INT_XOR => bin("bvxor"),
        CPUI_BOOL_AND => logic("bvand"),
        CPUI_BOOL_OR => logic("bvor"),
        CPUI_BOOL_XOR => logic("bvxor"),
        CPUI_INT_NEGATE => arg(0).filter(|a| a.width() == ow).map(|a| Rc::new(Expr::Op("bvnot", vec![a], ow))),
        CPUI_INT_2COMP => arg(0).filter(|a| a.width() == ow).map(|a| Rc::new(Expr::Op("bvneg", vec![a], ow))),
        CPUI_BOOL_NEGATE => arg(0).filter(|_| ow == 8).map(|a| Rc::new(Expr::Test("=", a.clone(), konst(0, a.width())))),
        CPUI_INT_LEFT => shift("bvshl", false),
        CPUI_INT_RIGHT => shift("bvlshr", false),
        CPUI_INT_SRIGHT => shift("bvashr", true),
        CPUI_INT_EQUAL => test("=", false),
        CPUI_INT_NOTEQUAL => test("distinct", false),
        CPUI_INT_LESS => test("bvult", false),
        CPUI_INT_SLESS => test("bvslt", false),
        CPUI_INT_LESSEQUAL => test("bvule", false),
        CPUI_INT_SLESSEQUAL => test("bvsle", false),
        CPUI_INT_CARRY => {
            let (a, b) = (arg(0)?, arg(1)?);
            if !same(&a, &b) || out != 1 {
                return None;
            }
            let sum = Rc::new(Expr::Op("bvadd", vec![a.clone(), b], a.width()));
            Some(Rc::new(Expr::Test("bvult", sum, a)))
        }
        CPUI_INT_SCARRY | CPUI_INT_SBORROW => {
            let (a, b) = (arg(0)?, arg(1)?);
            (same(&a, &b) && out == 1).then(|| Rc::new(Expr::Overflow(opc == CPUI_INT_SCARRY, a, b)))
        }
        CPUI_INT_ZEXT => {
            let a = arg(0)?;
            (ow > a.width()).then(|| Rc::new(Expr::ZeroExt(a.clone(), ow - a.width())))
        }
        CPUI_INT_SEXT => {
            let a = arg(0)?;
            (ow > a.width()).then(|| Rc::new(Expr::SignExt(a.clone(), ow - a.width())))
        }
        CPUI_PIECE => {
            let (hi, lo) = (arg(0)?, arg(1)?);
            (hi.width() + lo.width() == ow).then(|| Rc::new(Expr::Concat(vec![hi, lo])))
        }
        CPUI_SUBPIECE => {
            let a = arg(0)?;
            let (_, c, _) = ins.get(1)?;
            let lo = u32::try_from(*c).ok()?.checked_mul(8)?;
            let a = fit(a.clone(), a.width().max(lo + ow));
            Some(Rc::new(Expr::Extract(a, lo + ow - 1, lo)))
        }
        _ => None,
    }
}

/// The bytes of `e`, from the least significant.
pub fn byte(e: &Rc<Expr>, i: u32) -> Rc<Expr> {
    Rc::new(Expr::Extract(Rc::clone(e), 8 * i + 7, 8 * i))
}

/// One recorded branch of the function's own code: its instruction's address,
/// the index of its p-code op in that instruction, its condition, and whether
/// it was taken.
#[derive(Clone)]
pub struct Branch {
    pub at: u64,
    pub op: u32,
    pub cond: Rc<Expr>,
    pub taken: bool,
}

/// An input byte's place: space index and offset. A `bool` argument's byte
/// is 0 or 1.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Place {
    pub space: i32,
    pub off: u64,
    pub boolean: bool,
}

/// A recorded run: its branches and the input bytes they read.
#[derive(Clone, Default)]
pub struct Path {
    pub branches: Vec<Branch>,
    pub inputs: BTreeMap<Rc<str>, Place>,
}

/// Input bytes a model sets.
pub type Inputs = Rc<HashMap<(i32, u64), u8>>;

#[derive(Default)]
pub(crate) struct Emitter {
    ids: HashMap<*const Expr, usize>,
    defs: String,
}

impl Emitter {
    pub(crate) fn definitions(&self) -> &str {
        &self.defs
    }

    pub(crate) fn name(&mut self, e: &Rc<Expr>) -> String {
        if let Expr::Var(v) = &**e {
            return v.to_string();
        }
        let key = Rc::as_ptr(e);
        if let Some(&id) = self.ids.get(&key) {
            return format!("n{id}");
        }
        let body = match &**e {
            Expr::Var(_) => unreachable!(),
            Expr::Const(v, w) => format!("(_ bv{v} {w})"),
            Expr::Op(name, args, _) => {
                let a: Vec<String> = args.iter().map(|x| self.name(x)).collect();
                format!("({name} {})", a.join(" "))
            }
            Expr::Extract(x, hi, lo) => format!("((_ extract {hi} {lo}) {})", self.name(x)),
            Expr::Concat(parts) => {
                let a: Vec<String> = parts.iter().map(|x| self.name(x)).collect();
                if a.len() == 1 { a[0].clone() } else { format!("(concat {})", a.join(" ")) }
            }
            Expr::ZeroExt(x, k) => format!("((_ zero_extend {k}) {})", self.name(x)),
            Expr::SignExt(x, k) => format!("((_ sign_extend {k}) {})", self.name(x)),
            Expr::Test(name, a, b) => format!("(ite ({name} {} {}) #x01 #x00)", self.name(a), self.name(b)),
            Expr::Overflow(add, a, b) => {
                let (x, y, w) = (self.name(a), self.name(b), a.width());
                let r = if *add { format!("(bvadd {x} {y})") } else { format!("(bvsub {x} {y})") };
                let sign = |t: &str| format!("((_ extract {} {}) {t})", w - 1, w - 1);
                let cond = if *add {
                    format!("(and (= {} {}) (distinct {} {}))", sign(&x), sign(&y), sign(&x), sign(&r))
                } else {
                    format!("(and (distinct {} {}) (distinct {} {}))", sign(&x), sign(&y), sign(&x), sign(&r))
                };
                format!("(ite {cond} #x01 #x00)")
            }
        };
        let id = self.ids.len();
        self.ids.insert(key, id);
        let _ = writeln!(self.defs, "(define-fun n{id} () (_ BitVec {}) {body})", e.width());
        format!("n{id}")
    }
}

/// One z3 process kept for a function's queries: each query is pushed,
/// checked and popped, so z3 starts once.
pub struct Solver {
    child: std::process::Child,
    out: std::io::BufReader<std::process::ChildStdout>,
}

impl Solver {
    fn start(millis: u32) -> Option<Solver> {
        let mut child = Command::new("z3")
            .args(["-in", "-smt2", &format!("-t:{millis}")])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let out = std::io::BufReader::new(child.stdout.take()?);
        let mut s = Solver { child, out };
        s.send("(set-option :print-success false)\n(set-logic QF_BV)\n")?;
        Some(s)
    }

    fn send(&mut self, text: &str) -> Option<()> {
        let stdin = self.child.stdin.as_mut()?;
        stdin.write_all(text.as_bytes()).ok()?;
        stdin.flush().ok()
    }

    fn line(&mut self) -> Option<String> {
        let mut l = String::new();
        (std::io::BufRead::read_line(&mut self.out, &mut l).ok()? > 0).then_some(l)
    }

    /// One balanced s-expression of the output.
    fn sexpr(&mut self) -> Option<String> {
        let mut text = String::new();
        let mut depth = 0i32;
        loop {
            let l = self.line()?;
            depth += l.matches('(').count() as i32 - l.matches(')').count() as i32;
            text.push_str(&l);
            if depth <= 0 && text.contains('(') {
                return Some(text);
            }
        }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        let _ = self.send("(exit)\n");
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// What one query gave.
enum Answer {
    Model(HashMap<(i32, u64), u8>),
    /// Unsatisfiable, unknown in the time given, or nothing to ask.
    None,
    /// z3 did not answer as expected: the process is not used again.
    Broken,
}

/// Ask z3 for inputs that hold `path.branches[..k]` and take `branches[k]`
/// the other way. The query's scope is popped whatever the answer.
fn solve(solver: &mut Solver, path: &Path, k: usize) -> Answer {
    let mut em = Emitter { ids: HashMap::new(), defs: String::new() };
    let mut asserts = String::new();
    for (i, b) in path.branches[..=k].iter().enumerate() {
        let c = em.name(&b.cond);
        let taken = if i == k { !b.taken } else { b.taken };
        let _ = writeln!(asserts, "(assert ({} {c} #x00))", if taken { "distinct" } else { "=" });
    }
    let mut script = String::from("(push 1)\n");
    let mut used = BTreeSet::new();
    for (name, place) in &path.inputs {
        if em_mentions(&em, &asserts, name) {
            used.insert(Rc::clone(name));
            let _ = writeln!(script, "(declare-const {name} (_ BitVec 8))");
            if place.boolean {
                let _ = writeln!(script, "(assert (bvule {name} #x01))");
            }
        }
    }
    if used.is_empty() {
        return Answer::None;
    }
    script.push_str(&em.defs);
    script.push_str(&asserts);
    script.push_str("(check-sat)\n");
    if solver.send(&script).is_none() {
        return Answer::Broken;
    }
    let names: Vec<&str> = used.iter().map(|n| &**n).collect();
    let answer = match solver.line().as_deref().map(str::trim) {
        Some("sat") => match model(solver, path, &names) {
            Some(m) => Answer::Model(m),
            None => Answer::Broken,
        },
        Some("unsat" | "unknown") => Answer::None,
        _ => Answer::Broken,
    };
    if solver.send("(pop 1)\n").is_none() {
        return Answer::Broken;
    }
    answer
}

/// The input bytes of a satisfiable query.
fn model(solver: &mut Solver, path: &Path, names: &[&str]) -> Option<HashMap<(i32, u64), u8>> {
    solver.send(&format!("(get-value ({}))\n", names.join(" ")))?;
    let text = solver.sexpr()?;
    let mut model = HashMap::new();
    for name in names {
        let at = text.find(&format!("({name} "))? + name.len() + 2;
        let v = text[at..].split(|c: char| c == ')' || c.is_whitespace()).next()?;
        let byte = if let Some(h) = v.strip_prefix("#x") {
            u8::from_str_radix(h, 16).ok()?
        } else {
            u8::from_str_radix(v.strip_prefix("#b")?, 2).ok()?
        };
        let place = path.inputs.get(*name)?;
        model.insert((place.space, place.off), byte);
    }
    Some(model)
}

fn em_mentions(em: &Emitter, asserts: &str, name: &str) -> bool {
    let token = |s: &str| s.split(|c: char| c == '(' || c == ')' || c.is_whitespace()).any(|t| t == name);
    token(&em.defs) || token(asserts)
}

/// Which branch to flip next: across every recorded path, the first branch
/// whose other direction no run has taken. A direction is asked about once per
/// path prefix (the branches before it and how they went): unsatisfiable after
/// one prefix, it may not be after another.
#[derive(Default)]
pub struct Worklist {
    solver: Option<Solver>,
    /// z3 could not be started.
    pub unavailable: bool,
    /// z3 stopped answering as expected and was not used again.
    pub broken: bool,
    paths: Vec<(u32, Option<Inputs>, Path)>,
    covered: HashSet<(u64, u32, bool)>,
    tried: HashSet<(u64, u32, bool, u64)>,
    pub queries: u32,
    pub found: u32,
}

/// What concolic exploration did for one function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Summary {
    pub queries: u32,
    pub models: u32,
    pub solver: SolverState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolverState {
    /// No branch needed a query.
    Unused,
    Ran,
    Unavailable,
    Broken,
}

impl SolverState {
    pub fn name(self) -> &'static str {
        match self {
            SolverState::Unused => "unused",
            SolverState::Ran => "ran",
            SolverState::Unavailable => "unavailable",
            SolverState::Broken => "broken",
        }
    }
}

fn prefix(branches: &[Branch]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for b in branches {
        (b.at, b.op, b.taken).hash(&mut h);
    }
    h.finish()
}

impl Worklist {
    pub fn add(&mut self, run: u32, inputs: Option<Inputs>, path: Path) {
        for b in &path.branches {
            self.covered.insert((b.at, b.op, b.taken));
        }
        self.paths.push((run, inputs, path));
    }

    pub fn summary(&self) -> Summary {
        let solver = if self.unavailable {
            SolverState::Unavailable
        } else if self.broken {
            SolverState::Broken
        } else if self.queries > 0 {
            SolverState::Ran
        } else {
            SolverState::Unused
        };
        Summary { queries: self.queries, models: self.found, solver }
    }

    /// The next run to make: its base run, the input bytes to set (the base
    /// run's own merged with the model's), and the branch it aims at.
    pub fn next(&mut self, max_queries: u32, millis: u32) -> Option<(u32, Inputs, u64)> {
        let mut p = 0;
        while p < self.paths.len() {
            let n = self.paths[p].2.branches.len();
            for k in 0..n {
                if self.queries >= max_queries || self.unavailable || self.broken {
                    return None;
                }
                let branches = &self.paths[p].2.branches;
                let b = &branches[k];
                let want = (b.at, b.op, !b.taken);
                if self.covered.contains(&want) || !self.tried.insert((want.0, want.1, want.2, prefix(&branches[..k]))) {
                    continue;
                }
                if self.solver.is_none() {
                    self.solver = Solver::start(millis);
                    if self.solver.is_none() {
                        self.unavailable = true;
                        return None;
                    }
                }
                self.queries += 1;
                let solver = self.solver.as_mut()?;
                match solve(solver, &self.paths[p].2, k) {
                    Answer::Model(model) => {
                        self.found += 1;
                        let (run, base, _) = &self.paths[p];
                        let mut inputs: HashMap<(i32, u64), u8> = base.as_deref().cloned().unwrap_or_default();
                        inputs.extend(model);
                        return Some((*run, Rc::new(inputs), want.0));
                    }
                    Answer::None => {}
                    Answer::Broken => {
                        self.solver = None;
                        self.broken = true;
                        return None;
                    }
                }
            }
            p += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(e: &Expr) -> u128 {
        match e {
            Expr::Const(v, _) => *v,
            Expr::Op(name, a, w) => {
                let (x, y) = (eval(&a[0]), a.get(1).map_or(0, |b| eval(b)));
                let v = match *name {
                    "bvand" => x & y,
                    "bvor" => x | y,
                    "bvxor" => x ^ y,
                    other => panic!("{other}"),
                };
                v & ((1u128 << w) - 1)
            }
            Expr::Test(name, a, b) => {
                let (x, y) = (eval(a), eval(b));
                u128::from(match *name {
                    "=" => x == y,
                    "distinct" => x != y,
                    other => panic!("{other}"),
                })
            }
            other => panic!("{other:?}"),
        }
    }

    /// p-code's boolean ops read any nonzero byte as true, as the concrete
    /// evaluator does, and give 0 or 1.
    #[test]
    fn boolean_ops_normalize_their_inputs() {
        let two = Rc::new(Expr::Const(2, 8));
        let ins = |a: &Rc<Expr>, b: u128| vec![(Some(Rc::clone(a)), 0, 1), (None, b, 1)];
        let at = |opc, ins: &[(Option<Rc<Expr>>, u128, u32)]| eval(&op(opc, 1, ins).unwrap());
        assert_eq!(at(OpCode::CPUI_BOOL_AND, &ins(&two, 1)), 1);
        assert_eq!(at(OpCode::CPUI_BOOL_OR, &ins(&two, 1)), 1);
        assert_eq!(at(OpCode::CPUI_BOOL_XOR, &ins(&two, 1)), 0);
        assert_eq!(at(OpCode::CPUI_BOOL_NEGATE, &[(Some(two), 0, 1)]), 0);
        assert_eq!(at(OpCode::CPUI_INT_AND, &ins(&Rc::new(Expr::Const(2, 8)), 1)), 0);
    }
}
