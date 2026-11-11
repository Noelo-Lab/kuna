//! Iteration order of the Java hash collections against vectors a JDK 21 produced
//! (`data/GenHashGolden.java`, run with `--add-opens java.base/java.util=ALL-UNNAMED`).
//! Lines marked `t` treeified a bin on the JVM, which the port does not model.

use kuna_jvm::rt::jstring::{from_utf16, java_string_hash};
use kuna_jvm::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, JavaEq, JavaHash};

#[derive(Clone, Debug, PartialEq)]
enum Key {
    Int(i32),
    Str(String),
}

impl JavaHash for Key {
    fn java_hash(&self) -> i32 {
        match self {
            Key::Int(i) => *i,
            Key::Str(s) => java_string_hash(s),
        }
    }
}

impl JavaEq for Key {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

fn decode(tok: &str) -> Key {
    match tok.strip_prefix('x') {
        Some(hex) => {
            let units: Vec<u16> = (0..hex.len() / 4)
                .map(|i| u16::from_str_radix(&hex[i * 4..i * 4 + 4], 16).unwrap())
                .collect();
            Key::Str(from_utf16(&units))
        }
        None => Key::Int(tok.parse().unwrap()),
    }
}

fn encode(key: &Key) -> String {
    use kuna_jvm::rt::JavaStr;
    match key {
        Key::Int(i) => i.to_string(),
        Key::Str(s) => format!(
            "x{}",
            s.utf16()
                .iter()
                .map(|u| format!("{u:04x}"))
                .collect::<String>()
        ),
    }
}

enum Coll {
    Set(JHashSet<Key>),
    LSet(JLinkedHashSet<Key>),
    Map(JHashMap<Key, usize>),
    LMap(JLinkedHashMap<Key, usize>),
}

fn run(kind: &str, linked: bool, cap: i64, ops: &str) -> Vec<Key> {
    let mut c = match (kind, linked, cap) {
        ("S", false, -1) => Coll::Set(JHashSet::new()),
        ("S", false, n) => Coll::Set(JHashSet::with_capacity(n as usize)),
        ("S", true, -1) => Coll::LSet(JLinkedHashSet::new()),
        ("S", true, n) => Coll::LSet(JLinkedHashSet::with_capacity(n as usize)),
        ("M", false, -1) => Coll::Map(JHashMap::new()),
        ("M", false, n) => Coll::Map(JHashMap::with_capacity(n as usize)),
        ("M", true, -1) => Coll::LMap(JLinkedHashMap::new()),
        ("M", true, n) => Coll::LMap(JLinkedHashMap::with_capacity(n as usize)),
        _ => unreachable!(),
    };
    for (i, op) in ops.split(',').enumerate() {
        let (code, rest) = op.split_at(1);
        match code {
            "a" => {
                let k = decode(rest);
                match &mut c {
                    Coll::Set(s) => drop(s.add(k)),
                    Coll::LSet(s) => drop(s.add(k)),
                    Coll::Map(m) => drop(m.put(k, i)),
                    Coll::LMap(m) => drop(m.put(k, i)),
                }
            }
            "r" => {
                let k = decode(rest);
                match &mut c {
                    Coll::Set(s) => drop(s.remove(&k)),
                    Coll::LSet(s) => drop(s.remove(&k)),
                    Coll::Map(m) => drop(m.remove(&k)),
                    Coll::LMap(m) => drop(m.remove(&k)),
                }
            }
            "c" => match &mut c {
                Coll::Set(s) => s.clear(),
                Coll::LSet(s) => s.clear(),
                Coll::Map(m) => m.clear(),
                Coll::LMap(m) => m.clear(),
            },
            "C" => {
                c = match &c {
                    Coll::Set(s) => Coll::Set(s.clone()),
                    Coll::LSet(s) => Coll::LSet(s.clone()),
                    Coll::Map(m) => Coll::Map(m.clone()),
                    Coll::LMap(m) => Coll::LMap(m.clone()),
                }
            }
            "K" => {
                c = match &c {
                    Coll::Set(s) => Coll::Set(s.java_clone()),
                    _ => unreachable!(),
                }
            }
            "A" => {
                let keys: Vec<Key> = if rest.is_empty() {
                    Vec::new()
                } else {
                    rest.split('/').map(decode).collect()
                };
                let mut src: JLinkedHashMap<Key, usize> = JLinkedHashMap::new();
                for k in &keys {
                    src.put(k.clone(), i);
                }
                match &mut c {
                    Coll::Set(s) => drop(s.add_all(keys)),
                    Coll::LSet(s) => drop(s.add_all(keys)),
                    Coll::Map(m) => m.put_all(src.entry_vec()),
                    Coll::LMap(m) => m.put_all(src.entry_vec()),
                }
            }
            other => panic!("unknown op {other}"),
        }
    }
    match &c {
        Coll::Set(s) => s.to_vec(),
        Coll::LSet(s) => s.to_vec(),
        Coll::Map(m) => m.key_vec(),
        Coll::LMap(m) => m.key_vec(),
    }
}

#[test]
fn iteration_order_matches_jdk() {
    let data = include_str!("data/jhash_order.txt");
    let mut scenarios = 0;
    let mut treeified = 0;
    for (lineno, line) in data.lines().enumerate() {
        let f: Vec<&str> = line.split(';').collect();
        if f[0] == "H" {
            let Key::Str(s) = decode(f[1]) else {
                unreachable!()
            };
            assert_eq!(
                java_string_hash(&s),
                f[2].parse::<i32>().unwrap(),
                "String.hashCode line {}",
                lineno + 1
            );
            continue;
        }
        if f[0].starts_with('t') {
            treeified += 1;
            continue;
        }
        let got = run(f[0], f[2] == "L", f[3].parse().unwrap(), f[4]);
        let got: Vec<String> = got.iter().map(encode).collect();
        assert_eq!(got.join(","), f[5], "scenario on line {}", lineno + 1);
        scenarios += 1;
    }
    assert_eq!((scenarios, treeified), (1135, 85));
}
