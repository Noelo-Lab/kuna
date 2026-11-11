//! The util.collections ports replayed against traces of Vineflower's own
//! classes (`data/GenCollectionsGolden.java` on vineflower-1.12.0.jar).

use kuna_jvm::fast_fixed_set::FastFixedSet;
use kuna_jvm::fast_fixed_set_factory::FastFixedSetFactory;
use kuna_jvm::fast_sparse_set_factory::{
    java_collection_to_string, FastSparseSet, FastSparseSetFactory,
};
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::rt::{JavaEq, JavaHash, Shared};
use kuna_jvm::s_forms_fast_map_direct::SFormsFastMapDirect;
use kuna_jvm::vb_style_collection::VBStyleCollection;

fn list<T: ToString>(items: impl Iterator<Item = T>) -> String {
    let parts: Vec<String> = items.map(|x| x.to_string()).collect();
    if parts.is_empty() {
        "-".to_string()
    } else {
        parts.join(",")
    }
}

fn ints(s: &str) -> Vec<i32> {
    if s == "-" {
        Vec::new()
    } else {
        s.split(',').map(|x| x.parse().unwrap()).collect()
    }
}

fn b(v: bool) -> &'static str {
    if v {
        "1"
    } else {
        "0"
    }
}

struct Line<'a> {
    op: &'a str,
    args: Vec<&'a str>,
    want: Option<&'a str>,
    at: usize,
}

fn parse(line: &str, at: usize) -> Line<'_> {
    let (head, want) = match line.split_once(" = ") {
        Some((h, w)) => (h, Some(w)),
        None => (line, None),
    };
    let mut parts = head.split(' ');
    let op = parts.next().unwrap();
    Line {
        op,
        args: parts.collect(),
        want,
        at,
    }
}

impl Line<'_> {
    fn n(&self, i: usize) -> i32 {
        self.args[i].parse().unwrap()
    }

    fn u(&self, i: usize) -> usize {
        self.args[i].parse().unwrap()
    }

    fn check(&self, got: impl AsRef<str>) {
        assert_eq!(
            got.as_ref(),
            self.want.expect("result"),
            "`{} {}` on line {}",
            self.op,
            self.args.join(" "),
            self.at
        );
    }
}

fn run_sparse(n: i32, lines: &[Line]) {
    let factory = FastSparseSetFactory::new(0..n);
    let mut sets: Vec<FastSparseSet<i32>> = (0..5).map(|_| factory.create_empty_set()).collect();
    for l in lines {
        match l.op {
            "n" => sets[l.u(0)] = factory.create_empty_set(),
            "a" => sets[l.u(0)].add(l.n(1)),
            "r" => sets[l.u(0)].remove(&l.n(1)),
            "c" => l.check(b(sets[l.u(0)].contains(&l.n(1)))),
            "cp" => sets[l.u(1)] = sets[l.u(0)].get_copy(),
            "u" | "i" | "x" => {
                let other = sets[l.u(1)].clone();
                let s = &mut sets[l.u(0)];
                match l.op {
                    "u" => s.union(&other),
                    "i" => s.intersection(&other),
                    _ => s.complement(&other),
                }
            }
            "k" => l.check(sets[l.u(0)].get_cardinality().to_string()),
            "e" => l.check(b(sets[l.u(0)].is_empty())),
            "it" => l.check(list(sets[l.u(0)].iter())),
            "p" => l.check(java_collection_to_string(
                sets[l.u(0)].to_plain_set().iter(),
            )),
            "q" => {
                let (s, t) = (&sets[l.u(0)], &sets[l.u(1)]);
                l.check(format!("{} {}", b(s.java_equals(t)), s.java_hash()));
            }
            other => panic!("sparse op {other} on line {}", l.at),
        }
    }
}

fn run_sforms(lines: &[Line]) {
    let factory = FastSparseSetFactory::new(0..64);
    let mut maps: Vec<SFormsFastMapDirect> = (0..4)
        .map(|_| SFormsFastMapDirect::new(factory.clone()))
        .collect();
    for l in lines {
        let m = l.u(0);
        match l.op {
            "set" => maps[m].set_current_var(l.n(1), l.n(2)),
            "addv" => match maps[m].get(l.n(1)) {
                Some(set) => set.borrow_mut().add(l.n(2)),
                None => {
                    let mut set = factory.create_empty_set();
                    set.add(l.n(2));
                    maps[m].put(l.n(1), Shared::new(set));
                }
            },
            "rm" => maps[m].remove(l.n(1)),
            "rmf" => maps[m].remove_all_fields(),
            "rms" => maps[m].remove_all_stacks(),
            "cp" => maps[l.u(1)] = maps[m].get_copy(),
            "u" | "i" | "x" => {
                let other = maps[l.u(1)].get_copy();
                match l.op {
                    "u" => maps[m].union(&other),
                    "i" => maps[m].intersection(&other),
                    _ => maps[m].complement(&other),
                }
            }
            "s" => {
                let key: i32 = l.want.unwrap().rsplit(' ').next().unwrap().parse().unwrap();
                let map = &maps[m];
                l.check(format!(
                    "{} {} {} {}",
                    map.size(),
                    b(map.is_empty()),
                    b(map.contains_key(key)),
                    key
                ));
            }
            "str" => l.check(maps[m].to_string()),
            "ent" => {
                let entries: Vec<String> = maps[m]
                    .entry_list()
                    .into_iter()
                    .map(|(k, v)| format!("{}:{}", k, list(v.borrow().iter())))
                    .collect();
                l.check(entries.join(";"));
            }
            other => panic!("sforms op {other} on line {}", l.at),
        }
    }
}

fn show(v: Option<String>) -> String {
    v.unwrap_or_else(|| "null".to_string())
}

fn run_vb(lines: &[Line]) {
    let mut c: VBStyleCollection<String, i32> = VBStyleCollection::new();
    for l in lines {
        match l.op {
            "add" => drop(c.add(l.args[0].to_string())),
            "addk" => c.add_with_key(l.args[0].to_string(), l.n(1)),
            "put" => l.check(show(c.put_with_key(l.args[0].to_string(), l.n(1)))),
            "addat" => c.add_at(l.u(0), l.args[1].to_string()),
            "addki" => c.add_with_key_and_index(l.u(0), l.args[1].to_string(), l.n(2)),
            "rmk" => c.remove_with_key(&l.n(0)),
            "rmat" => l.check(c.remove_at(l.u(0))),
            "getk" => {
                let k = l.n(0);
                let index = if c.contains_key(&k) {
                    c.get_index_by_key(&k) as i32
                } else {
                    -1
                };
                l.check(format!(
                    "{} {} {}",
                    show(c.get_with_key(&k)),
                    b(c.contains_key(&k)),
                    index
                ));
            }
            "getat" => {
                let i = l.u(0);
                l.check(format!(
                    "{} {}",
                    c.get(i),
                    show(c.get_key(i).map(|k| k.to_string()))
                ));
            }
            "clear" => c.clear(),
            "clone" => c = c.clone(),
            "setat" => l.check(c.set(l.u(0), l.args[1].to_string())),
            "addall" => {
                let items: Vec<String> = match l.args[0] {
                    "-" => Vec::new(),
                    s => s.split(',').map(str::to_string).collect(),
                };
                c.add_all(items);
            }
            "addallk" => {
                let items: Vec<String> = match l.args[0] {
                    "-" => Vec::new(),
                    s => s.split(',').map(str::to_string).collect(),
                };
                c.add_all_with_key(items, ints(l.args[1]));
            }
            "vb" => {
                let keys = list(
                    c.get_lst_keys()
                        .iter()
                        .map(|k| k.map_or("null".to_string(), |k| k.to_string())),
                );
                let probe = l.args[0].to_string();
                l.check(format!(
                    "{} {} {}",
                    c.to_string_vb(),
                    keys,
                    c.index_of(&probe)
                ));
            }
            "idx" => {
                let mut idx = String::new();
                for k in 0..30 {
                    let i = if c.contains_key(&k) {
                        c.get_index_by_key(&k) as i32
                    } else {
                        -1
                    };
                    idx.push_str(&format!("{i},"));
                }
                l.check(idx);
            }
            other => panic!("vb op {other} on line {}", l.at),
        }
    }
}

fn fixed_line(set: &FastFixedSet<i32>) -> String {
    format!(
        "{} {} {}",
        list(set.iter()),
        set,
        java_collection_to_string(set.to_plain_set().iter())
    )
}

fn run_fixed(n: i32, lines: &[Line]) {
    let factory = FastFixedSetFactory::create((0..n).map(|i| i * 3));
    let mut sets: Vec<FastFixedSet<i32>> = (0..4).map(|_| factory.create_empty_set()).collect();
    for l in lines {
        let s = l.u(0);
        match l.op {
            "n" => sets[s] = factory.create_empty_set(),
            "a" => l.check(b(sets[s].add(l.n(1)))),
            "r" => l.check(b(sets[s].remove(&l.n(1)))),
            "c" => l.check(b(sets[s].contains(&l.n(1)))),
            "aa" | "ra" | "xa" => {
                let other = sets[l.u(1)].clone();
                let r = match l.op {
                    "aa" => sets[s].add_all(&other),
                    "ra" => sets[s].retain_all(&other),
                    _ => sets[s].remove_all(&other),
                };
                l.check(b(r));
            }
            "ca" => {
                let (a, t) = (&sets[s], &sets[l.u(1)]);
                l.check(format!("{} {}", b(a.contains_all(t)), b(a.java_equals(t))));
            }
            "sz" => l.check(format!(
                "{} {}",
                sets[s].get_real_size(),
                b(sets[s].is_empty())
            )),
            "it" => l.check(fixed_line(&sets[s])),
            "cl" => sets[l.u(1)] = sets[s].clone(),
            "ret" => {
                let k = l.n(1);
                sets[s].retain(|v| v % k != 0);
            }
            "aac" => l.check(b(sets[s].add_all_collection(ints(l.args[1])))),
            "rac" => l.check(b(sets[s].retain_all_collection(&ints(l.args[1])))),
            "xac" => l.check(b(sets[s].remove_all_collection(ints(l.args[1]).iter()))),
            "all" => sets[s] = factory.create_copied_set(),
            "clr" => sets[s].clear(),
            other => panic!("fixed op {other} on line {}", l.at),
        }
    }
}

#[test]
fn collections_match_vineflower() {
    let data = include_str!("data/collections_golden.txt");
    let mut scenarios: Vec<(String, Vec<Line>)> = Vec::new();
    for (i, raw) in data.lines().enumerate() {
        if raw.starts_with("SPARSE")
            || raw.starts_with("SFORMS")
            || raw == "VB"
            || raw.starts_with("FIXED")
        {
            scenarios.push((raw.to_string(), Vec::new()));
        } else {
            scenarios.last_mut().unwrap().1.push(parse(raw, i + 1));
        }
    }
    let mut counts = [0; 4];
    for (header, lines) in &scenarios {
        let mut h = header.split(' ');
        match h.next().unwrap() {
            "SPARSE" => {
                run_sparse(h.next().unwrap().parse().unwrap(), lines);
                counts[0] += 1;
            }
            "SFORMS" => {
                run_sforms(lines);
                counts[1] += 1;
            }
            "VB" => {
                run_vb(lines);
                counts[2] += 1;
            }
            _ => {
                run_fixed(h.next().unwrap().parse().unwrap(), lines);
                counts[3] += 1;
            }
        }
    }
    assert_eq!(counts, [30, 30, 30, 24]);
}

#[test]
fn list_stack_offsets() {
    let mut s: ListStack<i32> = ListStack::new();
    for i in 1..=5 {
        s.push(i);
    }
    assert_eq!(s.peek(), 5);
    assert_eq!(s.get_by_offset(-2), 4);
    s.insert_by_offset(-1, 9);
    assert_eq!(s.to_vec(), vec![1, 2, 3, 4, 9, 5]);
    assert_eq!(s.pop_n(2), Some(9));
    assert_eq!(s.pop_n(0), None);
    s.remove_multiple(1);
    assert_eq!(s.clone().pop(), 3);
    assert_eq!(s.size(), 3);
}
