//! L# (Vaandrager, Garhewal, Rot, Wißmann, TACAS 2022): an observation tree
//! of the words the machine ran, a basis of pairwise apart nodes, and the
//! rules that grow it. Two nodes are apart only by a word both ran with
//! different results, so every basis state is a real state of the machine.

use std::collections::BTreeMap;

pub type Output = u64;

struct Node {
    parent: Option<(usize, usize)>,
    /// Symbol index to (result, child).
    next: BTreeMap<usize, (Output, usize)>,
}

/// What the learner asks: the results of a word run from the initial state.
pub trait Oracle {
    fn query(&mut self, word: &[u64]) -> Result<Vec<Output>, String>;
}

pub struct Tree {
    nodes: Vec<Node>,
    pub alphabet: Vec<u64>,
    pub basis: Vec<usize>,
    pub queries: u64,
    pub symbols_run: u64,
}

/// A complete hypothesis: per state and symbol, the result and next state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hypothesis {
    pub states: usize,
    pub delta: Vec<Vec<(Output, usize)>>,
}

impl Hypothesis {
    pub fn run(&self, word: &[usize]) -> (Vec<Output>, usize) {
        let mut q = 0;
        let mut out = Vec::with_capacity(word.len());
        for a in word {
            let (o, t) = self.delta[q][*a];
            out.push(o);
            q = t;
        }
        (out, q)
    }
}

impl Tree {
    pub fn new(alphabet: Vec<u64>) -> Tree {
        Tree {
            nodes: vec![Node {
                parent: None,
                next: BTreeMap::new(),
            }],
            alphabet,
            basis: vec![0],
            queries: 0,
            symbols_run: 0,
        }
    }

    pub fn access(&self, mut n: usize) -> Vec<usize> {
        let mut w = Vec::new();
        while let Some((p, a)) = self.nodes[n].parent {
            w.push(a);
            n = p;
        }
        w.reverse();
        w
    }

    pub fn symbols(&self, word: &[usize]) -> Vec<u64> {
        word.iter().map(|a| self.alphabet[*a]).collect()
    }

    pub fn symbol(&mut self, value: u64) -> usize {
        if let Some(i) = self.alphabet.iter().position(|v| *v == value) {
            return i;
        }
        self.alphabet.push(value);
        self.alphabet.len() - 1
    }

    fn walk(&self, word: &[usize]) -> Option<usize> {
        let mut n = 0;
        for a in word {
            n = self.nodes[n].next.get(a)?.1;
        }
        Some(n)
    }

    /// Run `word` unless the tree has it; the results, read from the tree.
    pub fn query(
        &mut self,
        oracle: &mut dyn Oracle,
        word: &[usize],
    ) -> Result<Vec<Output>, String> {
        if self.walk(word).is_none() {
            let outs = oracle.query(&self.symbols(word))?;
            self.queries += 1;
            self.symbols_run += word.len() as u64;
            let mut n = 0;
            for (a, o) in word.iter().zip(outs) {
                n = match self.nodes[n].next.get(a) {
                    Some(&(seen, child)) => {
                        if seen != o {
                            return Err("the machine gave two results for one word: it is not deterministic".into());
                        }
                        child
                    }
                    None => {
                        let child = self.nodes.len();
                        self.nodes.push(Node {
                            parent: Some((n, *a)),
                            next: BTreeMap::new(),
                        });
                        self.nodes[n].next.insert(*a, (o, child));
                        child
                    }
                };
            }
        }
        let mut n = 0;
        Ok(word
            .iter()
            .map(|a| {
                let (o, c) = self.nodes[n].next[a];
                n = c;
                o
            })
            .collect())
    }

    /// A word both nodes ran with different results, shortest first.
    pub fn apart(&self, a: usize, b: usize) -> Option<Vec<usize>> {
        let mut level = vec![(a, b, Vec::new())];
        while !level.is_empty() {
            let mut deeper = Vec::new();
            for (x, y, w) in level {
                for (sym, &(ox, cx)) in &self.nodes[x].next {
                    if let Some(&(oy, cy)) = self.nodes[y].next.get(sym) {
                        let mut w2 = w.clone();
                        w2.push(*sym);
                        if ox != oy {
                            return Some(w2);
                        }
                        deeper.push((cx, cy, w2));
                    }
                }
            }
            level = deeper;
        }
        None
    }

    fn frontier(&self) -> Vec<usize> {
        let mut f = Vec::new();
        for &b in &self.basis {
            for &(_, c) in self.nodes[b].next.values() {
                if !self.basis.contains(&c) {
                    f.push(c);
                }
            }
        }
        f
    }

    fn candidates(&self, f: usize) -> Vec<usize> {
        self.basis
            .iter()
            .copied()
            .filter(|b| self.apart(f, *b).is_none())
            .collect()
    }

    /// Apply the rules until the tree defines a hypothesis.
    pub fn stabilize(
        &mut self,
        oracle: &mut dyn Oracle,
        max_states: usize,
    ) -> Result<Option<Hypothesis>, String> {
        loop {
            if self.basis.len() > max_states {
                return Ok(None);
            }
            if let Some(f) = self
                .frontier()
                .into_iter()
                .find(|f| self.candidates(*f).is_empty())
            {
                self.basis.push(f);
                continue;
            }
            let mut extended = false;
            for b in self.basis.clone() {
                for a in 0..self.alphabet.len() {
                    if !self.nodes[b].next.contains_key(&a) {
                        let mut w = self.access(b);
                        w.push(a);
                        self.query(oracle, &w)?;
                        extended = true;
                    }
                }
            }
            if extended {
                continue;
            }
            let mut separated = false;
            for f in self.frontier() {
                let c = self.candidates(f);
                if c.len() >= 2 {
                    let w = self.apart(c[0], c[1]).expect("basis nodes are apart");
                    let mut word = self.access(f);
                    word.extend(w);
                    self.query(oracle, &word)?;
                    separated = true;
                }
            }
            if separated {
                continue;
            }
            let index = |n: usize| -> usize {
                let target = if self.basis.contains(&n) {
                    n
                } else {
                    self.candidates(n)[0]
                };
                self.basis
                    .iter()
                    .position(|b| *b == target)
                    .expect("a basis node")
            };
            let delta = self
                .basis
                .iter()
                .map(|b| {
                    (0..self.alphabet.len())
                        .map(|a| (self.nodes[*b].next[&a].0, index(self.nodes[*b].next[&a].1)))
                        .collect()
                })
                .collect();
            return Ok(Some(Hypothesis {
                states: self.basis.len(),
                delta,
            }));
        }
    }

    /// Rivest–Schapire on a word the hypothesis gets wrong: run each
    /// `access(q_i) · word[i..]`, so the node where the hypothesis goes wrong
    /// becomes apart from the state it was taken for.
    pub fn counterexample(
        &mut self,
        oracle: &mut dyn Oracle,
        h: &Hypothesis,
        word: &[usize],
    ) -> Result<(), String> {
        self.query(oracle, word)?;
        let mut q = 0;
        for i in 0..word.len() {
            let mut w = self.access(self.basis[q]);
            w.extend_from_slice(&word[i..]);
            self.query(oracle, &w)?;
            q = h.delta[q][word[i]].1;
        }
        Ok(())
    }

    /// Every pair of basis states with the word that tells them apart.
    pub fn witnesses(&self) -> Vec<(usize, usize, Vec<usize>)> {
        let mut out = Vec::new();
        for i in 0..self.basis.len() {
            for j in i + 1..self.basis.len() {
                if let Some(w) = self.apart(self.basis[i], self.basis[j]) {
                    out.push((i, j, w));
                }
            }
        }
        out
    }
}
