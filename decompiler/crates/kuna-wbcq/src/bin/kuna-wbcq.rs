//! kuna-wbcq <binary> <0xentry> [--initial HEX]... [--context-bytes N] [--mutate]
//!   [--words N] [--length L] [--max-states K] [--sleighpath DIR]
//!
//! Learn the machine of `result step(ctx *, input)` at the entry, from each
//! initial context given (zeros by default), and print JSON.

use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

use kuna_verify::state::Env;
use kuna_wbcq::engine::{learn, report, Config};
use kuna_wbcq::machine::Abi;
use serde_json::json;

const USAGE: &str =
    "usage: kuna-wbcq <binary> <0xentry> [--initial HEX]... [--context-bytes N] [--mutate] \
                     [--words N] [--length L] [--max-states K] [--sleighpath DIR]";

fn spec_roots(sleighpath: Option<&str>) -> Vec<String> {
    if let Some(p) = sleighpath {
        return vec![p.to_string()];
    }
    let mut roots = Vec::new();
    for var in ["KUNA_SPECS", "SLEIGHHOME"] {
        if let Ok(v) = std::env::var(var) {
            if !v.is_empty() && !roots.contains(&v) {
                roots.push(v);
            }
        }
    }
    if let Ok(p) = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs")
        .canonicalize()
    {
        roots.push(p.to_string_lossy().into_owned());
    }
    roots
}

fn hex_bytes(s: &str) -> Result<Vec<u8>, String> {
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(s.get(i..i + 2).ok_or("odd hex")?, 16).map_err(|e| e.to_string())
        })
        .collect()
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kuna-wbcq: {e}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mut positional, mut initials, mut mutate, mut sleighpath) =
        (Vec::new(), Vec::new(), false, None);
    let mut context_bytes = 8usize;
    let mut cfg = Config::default();
    let mut i = 0;
    while i < args.len() {
        let value = |i: usize| {
            args.get(i + 1)
                .cloned()
                .ok_or_else(|| format!("{} needs a value", args[i]))
        };
        let num = |i: usize| value(i)?.parse::<usize>().map_err(|e| e.to_string());
        match args[i].as_str() {
            "--initial" => initials.push(hex_bytes(&value(i)?)?),
            "--context-bytes" => context_bytes = num(i)?,
            "--words" => cfg.words = num(i)?,
            "--length" => cfg.length = num(i)?,
            "--max-states" => cfg.max_states = num(i)?,
            "--sleighpath" => sleighpath = Some(value(i)?),
            "--mutate" => {
                mutate = true;
                i += 1;
                continue;
            }
            a if a.starts_with("--") => return Err(format!("unknown flag {a}\n{USAGE}")),
            a => {
                positional.push(a.to_string());
                i += 1;
                continue;
            }
        }
        i += 2;
    }
    let [binary, entry] = positional.as_slice() else {
        return Err(USAGE.into());
    };
    cfg.entry =
        u64::from_str_radix(entry.trim_start_matches("0x"), 16).map_err(|e| e.to_string())?;
    if initials.is_empty() {
        initials.push(vec![0; context_bytes]);
    }
    let roots = spec_roots(sleighpath.as_deref());
    let started = Instant::now();
    let out = kuna_verify::session::with_program(
        binary,
        &[],
        &roots,
        |prog| -> Result<serde_json::Value, String> {
            let loaded = started.elapsed().as_secs_f64();
            let env = Env::new(prog, 1).map_err(|h| h.render())?;
            let abi = Abi::for_program(prog)?;
            let mut results = Vec::new();
            for initial in &initials {
                let mut cfg = Config {
                    initial: initial.clone(),
                    ..Config::default()
                };
                cfg.entry = u64::from_str_radix(entry.trim_start_matches("0x"), 16)
                    .map_err(|e| e.to_string())?;
                let t = Instant::now();
                let (l, mut m) = match learn(&env, &abi, &cfg) {
                    Ok(done) => done,
                    Err(why) => {
                        eprintln!("initial {}: unsupported: {why}", hex(initial));
                        results.push(json!({ "initial": hex(initial), "learned": { "status": format!("unsupported: {why}") } }));
                        continue;
                    }
                };
                let learn_ms = t.elapsed().as_secs_f64() * 1e3;
                let mut r = report(&l, &m);
                eprintln!("initial {}: {} in {learn_ms:.0} ms", hex(initial), l.status);
                if mutate && l.hypothesis.is_some() {
                    let t = Instant::now();
                    r["mutation"] = kuna_wbcq::mutate::run(&l, &mut m, &cfg)?;
                    eprintln!("mutation: {:.0} ms", t.elapsed().as_secs_f64() * 1e3);
                }
                results.push(json!({ "initial": hex(initial), "learned": r }));
            }
            eprintln!("load: {loaded:.3} s");
            Ok(json!({ "binary": binary, "entry": entry, "runs": results }))
        },
    )??;
    println!(
        "{}",
        serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
