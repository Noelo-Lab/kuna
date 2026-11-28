use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use kuna_verify::symbolic;

pub(super) fn run(args: &[String]) -> Result<ExitCode, String> {
    let [binary, function, flags @ ..] = args else {
        return Err("prove needs an ELF and a function".into());
    };
    let mut stop = None;
    let mut whole = false;
    let mut induct = false;
    let mut loop_bound = None;
    let mut output = None;
    let mut sleighpath = None;
    let mut solver = "z3";
    let mut budget = 4096;
    let mut timeout = 5000;
    let mut inputs = Vec::new();
    let mut bindings = Vec::new();
    let mut ranges = Vec::new();
    let mut memory = Vec::new();
    let mut indirect = BTreeMap::<u64, Vec<u64>>::new();
    let mut decompile_options = Vec::new();
    let mut opcode_mutant = None;
    let mut flags = flags.iter().map(String::as_str);
    while let Some(flag) = flags.next() {
        if flag == "--whole-function" {
            whole = true;
            continue;
        }
        if flag == "--induct" {
            induct = true;
            continue;
        }
        let value = flags
            .next()
            .ok_or_else(|| format!("{flag} needs a value"))?;
        match flag {
            "--stop-at" => stop = Some(number(value)?),
            "--loop-bound" => {
                loop_bound = Some(
                    u8::try_from(number(value)?)
                        .ok()
                        .filter(|bound| (1..=32).contains(bound))
                        .ok_or("--loop-bound requires 1..32 visits per loop location")?,
                );
            }
            "--indirect-target" => {
                let (site, target) = value
                    .split_once(':')
                    .ok_or("--indirect-target expects SITE:DESTINATION")?;
                let targets = indirect.entry(number(site)?).or_default();
                let target = number(target)?;
                if targets.len() == 16 || targets.contains(&target) {
                    return Err("an indirect site needs one to sixteen distinct targets".into());
                }
                targets.push(target);
            }
            "--return" => {
                let (reg, bytes) = value
                    .split_once(':')
                    .ok_or("--return expects REGISTER:BYTES")?;
                output = Some((
                    reg,
                    bytes.parse::<u32>().map_err(|_| "invalid return width")?,
                ));
            }
            "--input" => inputs.push(value.to_owned()),
            "--bind" => {
                let bits = flags.next().ok_or("--bind expects REGISTER VALUE")?;
                bindings.push((value.to_owned(), number(bits)?));
            }
            "--range" => {
                let low = flags.next().ok_or("--range expects REGISTER LOW HIGH")?;
                let high = flags.next().ok_or("--range expects REGISTER LOW HIGH")?;
                ranges.push((value.to_owned(), number(low)?, number(high)?));
            }
            "--memory" | "--private-memory" | "--image-memory" => {
                let (address, size) = value
                    .split_once(':')
                    .ok_or("memory windows expect ADDRESS:BYTES")?;
                let size =
                    u32::try_from(number(size)?).map_err(|_| "memory window size is too large")?;
                let mode = match flag {
                    "--private-memory" => symbolic::MemoryMode::PrivateStack,
                    "--image-memory" => symbolic::MemoryMode::Image,
                    _ => symbolic::MemoryMode::Shared,
                };
                memory.push(symbolic::MemoryRange {
                    address: number(address)?,
                    size,
                    mode,
                });
            }
            "--option" => {
                let setting = flags.next().ok_or("--option expects NAME VALUE")?;
                decompile_options.push((value.to_owned(), setting.to_owned()));
            }
            "--budget" => budget = number(value)?,
            "--timeout-ms" => timeout = number(value)?,
            "--solver" => solver = value,
            "--opcode-mutant" => {
                opcode_mutant = Some(
                    usize::try_from(number(value)?).map_err(|_| "--opcode-mutant is too large")?,
                )
            }
            "--sleighpath" => sleighpath = Some(value),
            _ => return Err(format!("unknown prove option {flag}")),
        }
    }
    if !(1..=60_000).contains(&timeout) || !(1..=200_000).contains(&budget) {
        return Err("prove requires timeout 1..60000 ms and budget 1..200000 ops".into());
    }
    let (reg, width) = output.ok_or("prove requires --return REGISTER:BYTES")?;
    if whole == stop.is_some() {
        return Err("prove requires exactly one of --stop-at ADDR or --whole-function".into());
    }
    if !whole && !indirect.is_empty() {
        return Err("--indirect-target requires --whole-function".into());
    }
    if !whole && loop_bound.is_some() {
        return Err("--loop-bound requires --whole-function".into());
    }
    if induct && (!whole || loop_bound.is_some() || !memory.is_empty()) {
        return Err(
            "--induct requires --whole-function without a loop bound or memory windows".into(),
        );
    }
    let opts = symbolic::Options {
        inputs: &inputs,
        bindings: &bindings,
        ranges: &ranges,
        memory: &memory,
        stop,
        loop_bound,
        induct,
        indirect: &indirect,
        output: reg,
        output_bytes: width,
        step_limit: budget,
        timeout: Duration::from_millis(timeout),
        solver: Path::new(solver),
        opcode_mutant,
    };
    let roots = super::spec_roots(sleighpath);
    let evidence = symbolic::prove(binary, function, &decompile_options, &roots, &opts)?;
    println!("{evidence}");
    Ok(match evidence["classification"].as_str() {
        Some("MODEL_EQUIVALENCE" | "BOUNDED_EQUIVALENCE") => ExitCode::SUCCESS,
        Some("COUNTEREXAMPLE") => ExitCode::from(1),
        _ => ExitCode::from(3),
    })
}

fn number(text: &str) -> Result<u64, String> {
    let parsed = match text.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16),
        None => text.parse(),
    };
    parsed.map_err(|_| format!("invalid unsigned number {text}"))
}

pub(super) fn rv64_terms(args: &[String]) -> Result<ExitCode, String> {
    let [binary, function, flags @ ..] = args else {
        return Err("rv64-terms needs an ELF and a function".into());
    };
    let mut sleighpath = None;
    let mut mutation = None;
    for pair in flags.chunks(2) {
        match pair {
            [flag, path] if flag == "--sleighpath" => sleighpath = Some(path.as_str()),
            [flag, op] if flag == "--opcode-mutant" => {
                mutation =
                    Some(usize::try_from(number(op)?).map_err(|_| "--opcode-mutant is too large")?)
            }
            _ => {
                return Err("rv64-terms accepts --sleighpath DIR and --opcode-mutant INDEX".into())
            }
        }
    }
    println!(
        "{}",
        symbolic::rv64_terms(binary, function, &super::spec_roots(sleighpath), mutation)?
    );
    Ok(ExitCode::SUCCESS)
}
