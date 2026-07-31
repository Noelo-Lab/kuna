//! `kuna_wasm` — the in-browser decompiler entry point.
//!
//! Usage (argv is supplied by the WASI host / shell):
//!   kuna_wasm <binary> <spec-root> list [--mode MODE]
//!   kuna_wasm <binary> <spec-root> decompile [<name>|0x<addr>] [--mode MODE]
//!   kuna_wasm <binary> <spec-root> project [<display-name>] [--mode MODE]
//!
//! `<binary>` and `<spec-root>` are paths in the (virtual) filesystem. Writes a
//! JSON document to stdout (`list`/`decompile`: the shape of
//! `kuna decompile-all --json` plus a per-function `"kind"`; `project`: the
//! whole-binary `.c`/`.h`/`.asm`/`README.md` artifacts — named after
//! `<display-name>`, default the binary's basename — as one document); errors
//! go to stderr with a nonzero exit code. See `kuna_wasm::run` and
//! `docs/web-integration.md`.

use std::process::ExitCode;

fn parse_tail(argv: &[String]) -> Result<(Option<String>, Option<String>), String> {
    let mut arg = None;
    let mut mode = None;
    let mut i = 4;
    while i < argv.len() {
        match argv[i].as_str() {
            "--mode" => {
                i += 1;
                let value = argv.get(i).ok_or("--mode requires a value")?;
                mode = Some(value.clone());
            }
            flag if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            value if arg.is_none() => arg = Some(value.to_string()),
            value => return Err(format!("unexpected argument {value:?}")),
        }
        i += 1;
    }
    Ok((arg, mode))
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 4 {
        eprintln!(
            "usage: {} <binary> <spec-root> <list|decompile|project> \
             [name|0xaddr|display-name] [--mode auto|reliable|aggressive|fast]",
            argv[0]
        );
        return ExitCode::from(64);
    }
    let binary = &argv[1];
    let spec_root = &argv[2];
    let cmd = &argv[3];
    let (arg, mode) = match parse_tail(&argv) {
        Ok(parsed) => parsed,
        Err(msg) => {
            eprintln!("error: {msg}");
            return ExitCode::from(64);
        }
    };

    match kuna_wasm::run_with_mode(binary, spec_root, cmd, arg.as_deref(), mode.as_deref()) {
        Ok(payload) => {
            println!("{payload}");
            ExitCode::SUCCESS
        }
        Err(msg) => {
            eprintln!("error: {msg}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_tail;

    fn argv(tail: &[&str]) -> Vec<String> {
        ["kuna_wasm", "binary", "specs", "decompile"]
            .into_iter()
            .chain(tail.iter().copied())
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn mode_and_positional_can_appear_in_either_order() {
        assert_eq!(
            parse_tail(&argv(&["main", "--mode", "fast"])).unwrap(),
            (Some("main".into()), Some("fast".into()))
        );
        assert_eq!(
            parse_tail(&argv(&["--mode", "auto", "main"])).unwrap(),
            (Some("main".into()), Some("auto".into()))
        );
    }

    #[test]
    fn mode_requires_a_value_and_only_one_positional_is_allowed() {
        assert!(parse_tail(&argv(&["--mode"])).is_err());
        assert!(parse_tail(&argv(&["main", "other"])).is_err());
    }
}
