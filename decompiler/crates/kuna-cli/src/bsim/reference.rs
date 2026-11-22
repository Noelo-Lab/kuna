//! Compare Kuna-decompiled C from local or verified remote executables.

use kuna_console::engine::{ConsoleProgram, EntrySelector, FunctionEntry};
use kuna_console::project::{decompile_targets_with, DecompileOptions};
use serde::Serialize;
use similar::TextDiff;

use super::{corpus_io, parse_address, postgres, server_client};

const MAX_CODE_LINES: usize = 1000;

struct Args<'a> {
    binary: &'a str,
    address: Option<&'a str>,
    name: Option<&'a str>,
    reference: Option<&'a str>,
    selector: ReferenceSelector<'a>,
    json: bool,
}

enum ReferenceSelector<'a> {
    Address(u64),
    MatchId(i64),
    Server { url: &'a str, id: server_client::MatchId<'a> },
}

pub(super) fn cmd_compare(argv: &[String]) -> i32 {
    let args = match parse(argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("kuna bsim compare: {err}");
            return 2;
        }
    };
    match compare(&args) {
        Ok(out) => crate::output::emit_with_status(&out, 0),
        Err(err) => {
            eprintln!("kuna bsim compare: {}", super::TerminalText(&err));
            1
        }
    }
}

fn parse(argv: &[String]) -> Result<Args<'_>, String> {
    let (mut binary, mut address, mut name, mut reference, mut entry, mut match_id, mut server) =
        (None, None, None, None, None, None, None);
    let mut json = false;
    let mut iter = argv.iter().map(String::as_str);
    while let Some(flag) = iter.next() {
        let mut value = || iter.next().ok_or_else(|| format!("{flag} requires a value"));
        match flag {
            "--address" => address = Some(value()?),
            "--name" => name = Some(value()?),
            "--reference" if reference.is_none() => reference = Some(value()?),
            "--reference-address" if entry.is_none() && match_id.is_none() => {
                entry = Some(parse_address(value()?)?);
            }
            "--match-id" if entry.is_none() && match_id.is_none() => match_id = Some(value()?),
            "--reference-address" | "--match-id" => return Err(REFERENCE_SELECTOR_ERROR.into()),
            "--server" if server.is_none() => server = Some(value()?),
            "--json" => json = true,
            s if !s.starts_with('-') && binary.is_none() => binary = Some(s),
            _ => return Err(format!("unexpected argument {flag:?}")),
        }
    }
    if address.is_some() == name.is_some() {
        return Err("choose one source function with --address or --name".into());
    }
    let selector = match (entry, match_id, server) {
        (Some(entry), None, None) if reference.is_some() => ReferenceSelector::Address(entry),
        (Some(_), None, None) => return Err("--reference-address requires a local --reference binary".into()),
        (None, Some(id), None) if reference.is_some() => {
            let id = id.parse::<i64>().ok().filter(|&id| id > 0)
                .ok_or("--match-id requires a positive PostgreSQL match ID when --server is absent")?;
            ReferenceSelector::MatchId(id)
        }
        (None, Some(_), None) => return Err("a PostgreSQL match ID requires --reference".into()),
        (None, Some(id), Some(url)) if reference.is_none() => {
            ReferenceSelector::Server { url, id: server_client::MatchId::parse(id)? }
        }
        (_, _, Some(_)) => return Err("--server requires --match-id and forbids --reference".into()),
        _ => return Err(REFERENCE_SELECTOR_ERROR.into()),
    };
    Ok(Args {
        binary: binary.ok_or_else(|| "a source binary is required".to_owned())?,
        address,
        name,
        reference,
        selector,
        json,
    })
}

const REFERENCE_SELECTOR_ERROR: &str =
    "choose exactly one reference selector: --reference-address or --match-id";

#[derive(Serialize)]
struct Code {
    text: Option<String>,
    complete: bool,
    error: Option<String>,
}

impl Code {
    fn from_decompilation(text: Option<String>, error: Option<String>) -> Self {
        if error.is_some() {
            return Self { text: None, complete: false, error };
        }
        let Some(mut text) = text.filter(|text| !text.trim().is_empty()) else {
            return Self {
                text: None,
                complete: false,
                error: Some("decompilation did not return code".into()),
            };
        };
        let error = text.match_indices('\n').nth(MAX_CODE_LINES - 1)
            .map(|(i, _)| i + 1)
            .filter(|&end| end < text.len())
            .map(|end| {
                text.truncate(end);
                format!("decompiled code exceeds {MAX_CODE_LINES} lines")
            });
        Self { text: Some(text), complete: error.is_none(), error }
    }

    fn complete_text(&self) -> Option<&str> {
        self.text.as_deref().filter(|text| self.complete && !text.trim().is_empty())
    }
}

fn code_diff(source: &Code, reference: &Code) -> Option<String> {
    Some(TextDiff::from_lines(source.complete_text()?, reference.complete_text()?)
        .unified_diff().header("source", "reference").to_string())
}

#[derive(Serialize)]
struct Function {
    name: String,
    address: String,
    code: Code,
}

#[derive(Serialize)]
struct Reference {
    binary: String,
    executable_md5: Option<String>,
    identity_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<String>,
    #[serde(flatten)]
    function: Function,
}

fn compare(args: &Args<'_>) -> Result<String, String> {
    let (reference, matched) = resolve_reference(args)?;
    let source = {
        let mut prog = corpus_io::load_corpus_program(args.binary)?;
        let entry = corpus_io::select_entry(&prog, args.address, args.name)?;
        decompile_entry(&mut prog, entry)
    };
    let reference_code = &reference.function.code;
    let diff = code_diff(&source.code, reference_code);
    if args.json {
        #[derive(Serialize)]
        struct Comparison<'a> {
            binary: &'a str,
            source: Function,
            #[serde(rename = "match")]
            matched: Option<serde_json::Value>,
            reference: Reference,
            diff: Option<String>,
        }
        let mut out = serde_json::to_string_pretty(&Comparison {
            binary: args.binary, source, matched, reference, diff,
        }).map_err(|err| err.to_string())?;
        out.push('\n');
        return Ok(out);
    }
    let evidence = match diff {
        Some(diff) if diff.is_empty() => "Decompiled C is textually identical.\n".to_owned(),
        Some(diff) => diff,
        None => format!(
            "C diff unavailable: {}\n\n--- Source C ---\n{}\n--- Reference C ---\n{}\n",
            source.code.error.as_deref().or(reference_code.error.as_deref()).unwrap_or("incomplete code"),
            source.code.text.as_deref().unwrap_or("(no code returned)"),
            reference_code.text.as_deref().unwrap_or("(no code returned)"),
        ),
    };
    let verified = if reference.sha256.is_some() {
        "; SHA-256 verified"
    } else if reference.identity_verified {
        "; MD5 verified"
    } else {
        ""
    };
    Ok(format!("{} @ {} vs {} @ {} [{}{verified}]\n{evidence}",
        super::TerminalText(&source.name), source.address,
        super::TerminalText(&reference.function.name), reference.function.address,
        super::TerminalText(&reference.binary)))
}

fn resolve_reference(args: &Args<'_>) -> Result<(Reference, Option<serde_json::Value>), String> {
    match &args.selector {
        ReferenceSelector::Address(address) => {
            let path = args.reference.expect("explicit address requires local reference");
            let mut prog = corpus_io::load_corpus_program(path)?;
            let entry = resolve_entry(&prog, *address, false)?;
            Ok((Reference {
                binary: path.into(),
                executable_md5: None,
                identity_verified: false,
                sha256: None,
                function: decompile_entry(&mut prog, entry),
            }, None))
        }
        ReferenceSelector::MatchId(id) => {
            let matched = {
                let mut client = postgres::connect()?;
                let mut tx = postgres::read_only(&mut client)?;
                postgres::matched_function(&mut tx, *id)?
            };
            let address = parse_address(&matched.address)?;
            let path = args.reference.expect("a PostgreSQL match ID requires local reference");
            let (mut prog, md5) = corpus_io::load_corpus_program_with_md5(path)?;
            if !md5.eq_ignore_ascii_case(&matched.executable_md5) {
                return Err(format!(
                    "reference executable MD5 mismatch for BSim match {}: expected {}, got {md5}",
                    matched.match_id, matched.executable_md5,
                ));
            }
            let entry = resolve_entry(&prog, address, true)?;
            let reference = Reference {
                binary: path.into(),
                executable_md5: Some(md5),
                identity_verified: true,
                sha256: None,
                function: decompile_entry(&mut prog, entry),
            };
            Ok((reference, Some(to_json(matched)?)))
        }
        ReferenceSelector::Server { url, id } => {
            let client = server_client::Client::new(url)?;
            let matched = client.function(id)?;
            use md5::{Digest, Md5};
            let bytes = client.download(id)?;
            let md5 = kuna_base::hex_lower(&Md5::digest(&bytes));
            if !md5.eq_ignore_ascii_case(&matched.md5) {
                return Err("Kuna BSim reference metadata MD5 does not match the downloaded binary".into());
            }
            let mut prog = crate::decompile_all::load_bsim_reference(bytes, &matched)?;
            let entry = resolve_entry(&prog, id.address, true)?;
            if !prog.any_executable_entry(std::slice::from_ref(&entry)) {
                return Err(format!("indexed entry {:#x} is not executable in the downloaded binary", id.address));
            }
            let reference = Reference {
                binary: matched.executable.clone(),
                executable_md5: Some(md5),
                identity_verified: true,
                sha256: Some(matched.sha256.clone()),
                function: decompile_entry(&mut prog, entry),
            };
            let mut matched = to_json(matched)?;
            matched["match_id"] = id.to_string().into();
            matched["address"] = format!("{:#x}", id.address).into();
            Ok((reference, Some(matched)))
        }
    }
}

fn resolve_entry(prog: &ConsoleProgram, address: u64, exact: bool) -> Result<FunctionEntry, String> {
    let entry = prog.resolve_entry(&EntrySelector::Numeric(address))
        .map_err(|err| format!("reference: {err}"))?;
    if exact && prog.output_code_offset(entry.addr.get_offset()) != address {
        return Err(format!("reference does not resolve to indexed entry {address:#x}"));
    }
    Ok(entry)
}

fn to_json(value: impl Serialize) -> Result<serde_json::Value, String> {
    serde_json::to_value(value).map_err(|err| err.to_string())
}

fn decompile_entry(prog: &mut ConsoleProgram, entry: FunctionEntry) -> Function {
    let options = DecompileOptions { single_target: true, ..DecompileOptions::default() };
    let result = decompile_targets_with(prog, vec![entry], &options)
        .pop().expect("one decompile target returns one result");
    Function {
        name: result.name,
        address: format!("{:#x}", result.address),
        code: Code::from_decompilation(result.code, result.error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_requires_one_reference_selector_and_a_binary() {
        fn args(tail: &[&str]) -> Vec<String> {
            ["source", "--name", "main"].into_iter().chain(tail.iter().copied())
                .map(str::to_owned).collect()
        }

        let argv = args(&["--reference", "ref", "--reference-address", "0x16"]);
        let offline = parse(&argv).unwrap();
        assert_eq!(offline.reference, Some("ref"));
        assert!(matches!(offline.selector, ReferenceSelector::Address(0x16)));
        let argv = args(&["--reference", "ref", "--match-id", "42"]);
        assert!(matches!(parse(&argv).unwrap().selector, ReferenceSelector::MatchId(42)));
        for bad in [
            &["--reference", "ref"][..],
            &["--reference-address", "16"],
            &["--reference", "ref", "--reference-address", "16", "--match-id", "42"],
            &["--reference", "ref", "--match-id", "42", "--match-id", "43"],
            &["--reference", "ref", "--match-id", "0"],
            &["--reference", "ref", "--reference-address", "badhex"],
        ] {
            assert!(parse(&args(bad)).is_err(), "accepted {bad:?}");
        }
    }

    #[test]
    fn unavailable_code_is_incomplete_and_has_no_diff() {
        let reference = Code { text: Some("int ref(void) {}\n".into()), complete: true, error: None };
        let source = Code::from_decompilation(None, Some("decompilation failed".into()));
        assert_eq!(code_diff(&source, &reference), None);
        assert_eq!(json!(source), json!({
            "text": null, "complete": false, "error": "decompilation failed"
        }));

        let empty = Code::from_decompilation(Some(" \n".into()), None);
        assert_eq!(code_diff(&empty, &reference), None);
        assert_eq!(json!(empty), json!({
            "text": null, "complete": false, "error": "decompilation did not return code"
        }));
    }

    #[test]
    fn complete_code_at_line_limit_can_be_compared_but_truncated_code_cannot() {
        let text = "line\n".repeat(MAX_CODE_LINES);
        let over_limit = format!("{text}tail");
        let source = Code::from_decompilation(Some(text), None);
        let truncated = Code::from_decompilation(Some(over_limit), None);
        assert_eq!(code_diff(&source, &source).as_deref(), Some(""));
        assert_eq!(code_diff(&truncated, &source), None);
        assert_eq!(code_diff(&source, &truncated), None);
        assert_eq!(truncated.text, source.text);
    }
}
