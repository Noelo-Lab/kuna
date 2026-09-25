//! `kuna_wasm` — the in-browser decompiler entry point.
//!
//! Usage (argv is supplied by the WASI host / shell):
//!   kuna_wasm <binary> <spec-root> list [OPTIONS]
//!   kuna_wasm <binary> <spec-root> decompile [<name>|0x<addr>] [OPTIONS]
//!   kuna_wasm <binary> <spec-root> inspect <name>|0x<addr> [OPTIONS]
//!   kuna_wasm <binary> <spec-root> read 0x<addr> <len> [OPTIONS]
//!   kuna_wasm <binary> <spec-root> xrefs <name>|0x<addr> [OPTIONS]
//!   kuna_wasm <binary> <spec-root> project [<display-name>] [OPTIONS]
//! OPTIONS: --mode MODE, --language LANG, and repeatable --assert DIRECTIVE.
//!
//! `<binary>` and `<spec-root>` are paths in the (virtual) filesystem. Writes a
//! JSON document to stdout (`list`/`decompile`: the shape of
//! `kuna decompile-all --json` plus a per-function `"kind"`; `inspect`/`read`/`xrefs`:
//! the study view's documents; `project`: the whole-binary
//! `.c`/`.h`/`.asm`/`README.md` artifacts — named after `<display-name>`,
//! default the binary's basename — as one document); errors go to stderr with a
//! nonzero exit code. See `kuna_wasm::run_request` and
//! `docs/web-integration.md`.

use std::process::ExitCode;

/// The arguments after `<binary> <spec-root> <command>`.
#[derive(Debug, Default, PartialEq, Eq)]
struct Tail {
    positionals: Vec<String>,
    mode: Option<String>,
    language: Option<String>,
    asserts: Vec<String>,
}

fn parse_tail(argv: &[String]) -> Result<Tail, String> {
    let mut tail = Tail::default();
    let mut i = 4;
    while i < argv.len() {
        let flag = argv[i].as_str();
        match flag {
            "--mode" | "--language" | "--assert" => {
                i += 1;
                let value = argv.get(i).cloned().ok_or_else(|| format!("{flag} requires a value"))?;
                match flag {
                    "--mode" => tail.mode = Some(value),
                    "--language" => tail.language = Some(value),
                    _ => tail.asserts.push(value),
                }
            }
            flag if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            value if tail.positionals.len() < 2 => tail.positionals.push(value.to_string()),
            value => return Err(format!("unexpected argument {value:?}")),
        }
        i += 1;
    }
    Ok(tail)
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 4 {
        eprintln!(
            "usage: {} <binary> <spec-root> <list|decompile|inspect|read|xrefs|project> \
             [name|0xaddr|0xaddr len|display-name] [--mode auto|reliable|aggressive|fast] \
             [--language auto|c|rust] [--assert DIRECTIVE]...",
            argv[0]
        );
        return ExitCode::from(64);
    }
    let tail = match parse_tail(&argv) {
        Ok(parsed) => parsed,
        Err(msg) => {
            eprintln!("error: {msg}");
            return ExitCode::from(64);
        }
    };

    match kuna_wasm::run_request(&kuna_wasm::Request {
        binary: &argv[1],
        spec_root: &argv[2],
        cmd: &argv[3],
        args: &tail.positionals,
        mode: tail.mode.as_deref(),
        language: tail.language.as_deref(),
        asserts: &tail.asserts,
    }) {
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
    use super::{parse_tail, Tail};

    fn argv(tail: &[&str]) -> Vec<String> {
        ["kuna_wasm", "binary", "specs", "decompile"]
            .into_iter()
            .chain(tail.iter().copied())
            .map(str::to_string)
            .collect()
    }

    fn tail(positionals: &[&str], mode: Option<&str>, language: Option<&str>) -> Tail {
        Tail {
            positionals: positionals.iter().map(|s| s.to_string()).collect(),
            mode: mode.map(str::to_string),
            language: language.map(str::to_string),
            asserts: Vec::new(),
        }
    }

    #[test]
    fn mode_and_positional_can_appear_in_either_order() {
        assert_eq!(
            parse_tail(&argv(&["main", "--mode", "fast"])).unwrap(),
            tail(&["main"], Some("fast"), None)
        );
        assert_eq!(
            parse_tail(&argv(&["--mode", "auto", "main"])).unwrap(),
            tail(&["main"], Some("auto"), None)
        );
    }

    #[test]
    fn mode_requires_a_value_and_at_most_two_positionals_are_allowed() {
        assert!(parse_tail(&argv(&["--mode"])).is_err());
        assert_eq!(parse_tail(&argv(&["0x10", "16"])).unwrap(), tail(&["0x10", "16"], None, None));
        assert!(parse_tail(&argv(&["0x10", "16", "third"])).is_err());
    }

    /// (kuna outlang) The language rides beside the mode, in any order, and an
    /// omitted one stays `None` so the engine's auto policy applies.
    #[test]
    fn language_parses_beside_the_mode_in_either_order() {
        assert_eq!(
            parse_tail(&argv(&["main", "--language", "rust"])).unwrap(),
            tail(&["main"], None, Some("rust"))
        );
        assert_eq!(
            parse_tail(&argv(&["--language", "rust", "--mode", "fast", "main"])).unwrap(),
            tail(&["main"], Some("fast"), Some("rust"))
        );
        assert!(parse_tail(&argv(&["--language"])).is_err());
    }

    /// `--assert` repeats, keeps its order, may sit anywhere, and carries a
    /// whole directive (spaces included) as one value.
    #[test]
    fn asserts_repeat_in_order_and_keep_their_spaces() {
        let parsed = parse_tail(&argv(&[
            "--assert",
            "name v1 total",
            "main",
            "--mode",
            "fast",
            "--assert",
            "prototype 0x1161 long sum_to(int count)",
        ]))
        .unwrap();
        assert_eq!(parsed.positionals, vec!["main".to_string()]);
        assert_eq!(parsed.mode.as_deref(), Some("fast"));
        assert_eq!(
            parsed.asserts,
            vec!["name v1 total".to_string(), "prototype 0x1161 long sum_to(int count)".to_string()]
        );
        assert!(parse_tail(&argv(&["main", "--assert"])).unwrap_err().contains("--assert"));
    }
}
