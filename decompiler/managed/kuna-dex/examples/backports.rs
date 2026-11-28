//! Writes `src/backports.tsv` from probe DEX files (`fixtures/dex/backports/regen.sh`):
//! `backports INDEX DEX...`, where INDEX lines are `N, owner#name(desc), kind` and
//! method `bp/Probe.pN` of each DEX calls that JDK method. Where d8 called a
//! synthetic method instead, its body's shape maps to the JDK method; a shape
//! two methods share is kept for `Math` over `StrictMath` (same code) and
//! dropped otherwise.

use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let index: BTreeMap<String, (String, String)> = std::fs::read_to_string(&args[1])
        .expect("index")
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            (format!("p{}", f[0]), (f[2].to_string(), f[1].to_string()))
        })
        .collect();
    let mut shapes: BTreeMap<u64, BTreeSet<(String, String)>> = BTreeMap::new();
    for path in &args[2..] {
        let data = std::fs::read(path).expect("dex");
        let dex = kuna_dex::Dex::parse(&data).expect("parse");
        for i in 0..dex.class_count() {
            if dex.class_name(i).ok().as_deref() != Some("bp/Probe") {
                continue;
            }
            let data = dex.class_data(dex.class_def(i).expect("def").class_data_off).expect("data");
            for em in &data.direct_methods {
                let mi = dex.method(em.method).expect("method");
                let name = kuna_dex::mutf8_to_string(dex.string(mi.name).expect("name"));
                let (Some(target), true) = (index.get(&name), em.code_off != 0) else { continue };
                let code = dex.code_item(em.code_off).expect("code");
                for x in kuna_dex::decode(code.insns).expect("decode") {
                    if !matches!(x.op, 0x71 | 0x77) {
                        continue;
                    }
                    let m = dex.method(x.idx).expect("callee");
                    let Some(def) = dex.class_of_type(m.class).and_then(|c| dex.class_def(c).ok()) else { continue };
                    let sd = dex.class_data(def.class_data_off).expect("synthetic");
                    let Some(sm) = sd.direct_methods.iter().find(|e| e.method == x.idx) else { continue };
                    let scode = dex.code_item(sm.code_off).expect("synthetic code");
                    for h in dex.body_shapes(m.proto, &scode).expect("shape") {
                        shapes.entry(h).or_default().insert(target.clone());
                    }
                }
            }
        }
    }
    for (h, set) in shapes {
        let keep: Vec<_> = set.iter().filter(|t| !t.1.starts_with("java/lang/StrictMath#")).collect();
        if let [(kind, method)] = keep[..] {
            println!("{h:016x}\t{kind}\t{method}");
        }
    }
}
