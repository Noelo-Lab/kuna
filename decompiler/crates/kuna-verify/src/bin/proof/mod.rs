use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use kuna_verify::symbolic;

pub(super) fn run(args: &[String]) -> Result<ExitCode, String> {
    let [binary, function, flags @ ..] = args else {
        return Err("prove needs an ELF and a function".into());
    };
    let mut stop = None;
    let mut output = None;
    let mut sleighpath = None;
    let mut solver = "z3";
    let mut budget = 4096;
    let mut timeout = 5000;
    let mut inputs = Vec::new();
    let mut bindings = Vec::new();
    let mut decompile_options = Vec::new();
    let mut opcode_mutant = None;
    let mut flags = flags.iter().map(String::as_str);
    while let Some(flag) = flags.next() {
        let value = flags
            .next()
            .ok_or_else(|| format!("{flag} needs a value"))?;
        match flag {
            "--stop-at" => stop = Some(number(value)?),
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
    let opts = symbolic::Options {
        inputs: &inputs,
        bindings: &bindings,
        stop: stop.ok_or("prove requires --stop-at ADDR")?,
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
        Some("MODEL_EQUIVALENCE") => ExitCode::SUCCESS,
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
