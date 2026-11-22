//! Compare local C against a single versioned Ghidra Server function.

use std::net::IpAddr;
use std::time::Duration;

use kuna_console::project::{decompile_targets_with, DecompileOptions};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use serde_json::json;
use similar::TextDiff;
use ureq::tls::{Certificate, RootCerts, TlsConfig};
use url::Url;

use super::{corpus_io, parse_address, postgres};

const MAX_CODE_LINES: usize = 1000;
const MAX_RESPONSE_BYTES: u64 = 8 * 1024 * 1024;

struct Args<'a> {
    binary: &'a str,
    address: Option<&'a str>,
    name: Option<&'a str>,
    id: i64,
    profile: &'a str,
    version: i32,
    json: bool,
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
            eprintln!("kuna bsim compare: {err}");
            1
        }
    }
}

fn parse(argv: &[String]) -> Result<Args<'_>, String> {
    let (mut binary, mut address, mut name, mut id, mut profile) = (None, None, None, None, None);
    let mut version = -1;
    let mut json = false;
    let mut iter = argv.iter().map(String::as_str);
    while let Some(flag) = iter.next() {
        let mut value = || iter.next().ok_or_else(|| format!("{flag} requires a value"));
        match flag {
            "--address" => address = Some(value()?),
            "--name" => name = Some(value()?),
            "--match-id" => id = value()?.parse::<i64>().ok().filter(|&x| x > 0),
            "--reference-profile" => profile = Some(value()?),
            "--version" => {
                version = value()?.parse::<i32>().ok().filter(|&v| v > 0)
                    .ok_or_else(|| "--version requires a positive stored version".to_owned())?;
            }
            "--json" => json = true,
            s if !s.starts_with('-') && binary.is_none() => binary = Some(s),
            _ => return Err(format!("unexpected argument {flag:?}")),
        }
    }
    if address.is_some() == name.is_some() {
        return Err("choose one source function with --address or --name".into());
    }
    Ok(Args {
        binary: binary.ok_or_else(|| "a source binary is required".to_owned())?,
        address,
        name,
        id: id.ok_or_else(|| "--match-id requires a positive PostgreSQL match ID".to_owned())?,
        profile: profile.filter(|p| !p.trim().is_empty())
            .ok_or_else(|| "--reference-profile is required".to_owned())?,
        version,
        json,
    })
}

#[derive(Deserialize, Serialize)]
struct Code {
    text: Option<String>,
    complete: bool,
    error: Option<String>,
}

impl Code {
    fn local(text: Option<String>, error: Option<String>) -> Self {
        if error.is_some() {
            return Self { text: None, complete: false, error };
        }
        let Some(mut text) = text.filter(|text| !text.trim().is_empty()) else {
            return Self {
                text: None,
                complete: false,
                error: Some("local decompilation did not return code".into()),
            };
        };
        let error = text.match_indices('\n').nth(MAX_CODE_LINES - 1)
            .map(|(i, _)| i + 1)
            .filter(|&end| end < text.len())
            .map(|end| {
                text.truncate(end);
                format!("local code exceeds {MAX_CODE_LINES} lines")
            });
        Self { text: Some(text), complete: error.is_none(), error }
    }

    fn complete_text(&self) -> Option<&str> {
        self.text.as_deref().filter(|text| self.complete && !text.trim().is_empty())
    }
}

fn code_diff(source: &Code, reference: &Code) -> Option<String> {
    Some(TextDiff::from_lines(source.complete_text()?, reference.complete_text()?)
        .unified_diff().header("kuna", "ghidra").to_string())
}

#[derive(Deserialize, Serialize)]
struct Program {
    name: String,
    md5: String,
    language: String,
    compiler: String,
}

#[derive(Deserialize, Serialize)]
struct Function {
    name: String,
    address: String,
    prototype: String,
}

#[derive(Deserialize, Serialize)]
struct Snapshot {
    program: Program,
    function: Function,
    code: Code,
}

#[derive(Deserialize, Serialize)]
struct Reference {
    #[serde(skip_serializing)]
    match_id: String,
    identity_verified: bool,
    profile: String,
    path: String,
    version: i32,
    snapshot: Snapshot,
}

#[derive(Deserialize)]
struct Response {
    success: bool,
    result: Reference,
}

fn compare(args: &Args<'_>) -> Result<String, String> {
    let base = std::env::var("KUNA_BSIM_REFERENCE_URL")
        .map_err(|_| "set KUNA_BSIM_REFERENCE_URL to the GhydraMCP worker base URL".to_owned())?;
    let endpoint = worker_endpoint(&base)?;
    let mut prog = corpus_io::load_corpus_program(args.binary)?;
    let entry = corpus_io::select_entry(&prog, args.address, args.name)?;
    let addr = format!("0x{:x}", prog.output_code_offset(entry.addr.get_offset()));
    let selected_name = entry.name.clone();
    let options = DecompileOptions { single_target: true, ..DecompileOptions::default() };
    let (name, local_code) = match decompile_targets_with(&mut prog, vec![entry], &options).pop() {
        Some(local) => (local.name, Code::local(local.code, local.error)),
        None => (selected_name, Code::local(None, Some("local decompilation did not return a function".into()))),
    };
    let matched = {
        let mut client = postgres::connect()?;
        let mut tx = postgres::read_only(&mut client)?;
        postgres::matched_function(&mut tx, args.id)?
    };
    let reference = fetch(endpoint, args, &matched)?;
    let reference_code = &reference.snapshot.code;
    let diff = code_diff(&local_code, reference_code);
    if args.json {
        return Ok(format!("{:#}\n", json!({
            "binary": args.binary,
            "source": { "name": name, "address": addr, "code": local_code },
            "match": matched,
            "reference": reference,
            "diff": diff,
        })));
    }
    let evidence = match diff {
        Some(diff) if diff.is_empty() => "Decompiled C is textually identical.\n".to_owned(),
        Some(diff) => diff,
        None => format!(
            "C diff unavailable: {}\n\n--- Kuna C ---\n{}\n--- Ghidra C ---\n{}\n",
            local_code.error.as_deref().or(reference_code.error.as_deref()).unwrap_or("incomplete code"),
            local_code.text.as_deref().unwrap_or("(no code returned)"),
            reference_code.text.as_deref().unwrap_or("(no code returned)"),
        ),
    };
    Ok(format!("{name} @ {addr} vs {} @ {} ({} at version {})\n{evidence}",
        reference.snapshot.function.name, reference.snapshot.function.address,
        reference.path, reference.version))
}

fn worker_endpoint(base: &str) -> Result<Url, String> {
    let mut url = Url::parse(base).map_err(|_| "invalid KUNA_BSIM_REFERENCE_URL".to_owned())?;
    let loopback = url.host_str().is_some_and(|host| host == "localhost" ||
        host.trim_matches(['[', ']']).parse::<IpAddr>().is_ok_and(|addr| addr.is_loopback()));
    if url.host().is_none() || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        || !url.username().is_empty() || url.password().is_some()
        || url.query().is_some() || url.fragment().is_some()
    {
        return Err("reference worker URL must use HTTPS or loopback HTTP, without credentials, query or fragment".into());
    }
    url.path_segments_mut().map_err(|()| "invalid reference worker URL")?
        .pop_if_empty().extend(["repositories", "inspect-match"]);
    Ok(url)
}

fn reference_path_matches(repository: &Url, resolved: &str) -> bool {
    // The worker returns the decoded path relative to the repository root,
    // not the full URL path beginning with the repository name.
    let Some((repository_name, indexed_path)) = repository.path().strip_prefix('/')
        .and_then(|path| path.split_once('/')) else {
        return false;
    };
    let Some(resolved) = resolved.strip_prefix('/') else {
        return false;
    };
    !repository_name.is_empty() && !resolved.is_empty()
        && percent_decode_str(indexed_path).decode_utf8().is_ok_and(|indexed| indexed == resolved)
}

fn reference_address_matches(resolved: &str, indexed: &str) -> bool {
    // Native snapshots report `space::0xOFFSET`, but the BSim index stores
    // only the offset. The worker verifies which address space to use.
    let Some((space, address)) = resolved.split_once("::") else {
        return false;
    };
    let Some(hex) = address.strip_prefix("0x") else {
        return false;
    };
    !space.trim().is_empty() && !space.contains(':') && !space.chars().any(char::is_control)
        && (1..=16).contains(&hex.len()) && hex.bytes().all(|b| b.is_ascii_hexdigit())
        && parse_address(indexed).is_ok_and(|indexed| parse_address(address) == Ok(indexed))
}

fn fetch(mut endpoint: Url, args: &Args<'_>, matched: &postgres::RemoteMatch) -> Result<Reference, String> {
    let repository = matched.repository_url.as_ref()
        .ok_or_else(|| "the selected BSim match has no usable Ghidra Server reference".to_owned())?;
    // The worker uses this UUID as a request correlation ID, not an access grant.
    let request_id = uuid::Uuid::new_v4().to_string();
    endpoint.query_pairs_mut().extend_pairs([
        ("profile", args.profile),
        ("match_id", &request_id),
        ("repository_url", repository.as_str()),
        ("executable_md5", &matched.executable_md5),
        ("address", &matched.address),
        ("function", &matched.name),
        ("version", &args.version.to_string()),
        ("max_code_lines", &MAX_CODE_LINES.to_string()),
    ]);
    let mut config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(90)))
        .max_redirects(0).max_redirects_will_error(true).proxy(None);
    if endpoint.scheme() == "https" {
        let native = rustls_native_certs::load_native_certs();
        if native.certs.is_empty() {
            return Err("could not load native CA certificates for the reference worker (check TLS trust)".into());
        }
        let roots = RootCerts::from(native.certs.into_iter()
            .map(|cert| Certificate::from_der(cert.as_ref()).to_owned()));
        config = config.tls_config(TlsConfig::builder().root_certs(roots).build());
    }
    let agent: ureq::Agent = config.build().into();
    let mut request = agent.get(endpoint.as_str());
    if let Ok(token) = std::env::var("KUNA_BSIM_REFERENCE_TOKEN") {
        let value = ureq::http::HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|_| "invalid KUNA_BSIM_REFERENCE_TOKEN".to_owned())?;
        request = request.header("Authorization", value);
    }
    let mut response = request.call().map_err(|err| match err {
        ureq::Error::StatusCode(status) => format!("reference worker rejected the request (HTTP {status})"),
        _ => "reference worker connection failed (check endpoint and TLS trust)".to_owned(),
    })?;
    let data = response.body_mut().with_config().limit(MAX_RESPONSE_BYTES).read_to_vec()
        .map_err(|_| "could not read bounded reference worker response".to_owned())?;
    let Response { success, result } = serde_json::from_slice(&data)
        .map_err(|_| "invalid GhydraMCP reference response".to_owned())?;
    if !success || !result.identity_verified || result.match_id != request_id || result.profile != args.profile
        || !result.snapshot.program.md5.eq_ignore_ascii_case(&matched.executable_md5)
        || !reference_path_matches(repository, &result.path)
        || !reference_address_matches(&result.snapshot.function.address, &matched.address)
        || result.version < 1 || (args.version != -1 && result.version != args.version)
    {
        return Err("reference worker did not verify the selected match identity and version".into());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_local_code_is_incomplete_and_has_no_diff() {
        let reference = Code { text: Some("int ref(void) {}\n".into()), complete: true, error: None };
        let source = Code::local(None, Some("local decompilation failed".into()));
        assert_eq!(code_diff(&source, &reference), None);
        assert_eq!(json!(source), json!({
            "text": null, "complete": false, "error": "local decompilation failed"
        }));

        let empty = Code::local(Some(" \n".into()), None);
        assert_eq!(code_diff(&empty, &reference), None);
        assert_eq!(json!(empty), json!({
            "text": null, "complete": false, "error": "local decompilation did not return code"
        }));
    }

    #[test]
    fn complete_code_at_line_limit_can_be_compared_but_truncated_code_cannot() {
        let text = "line\n".repeat(MAX_CODE_LINES);
        let over_limit = format!("{text}tail");
        let source = Code::local(Some(text), None);
        let truncated = Code::local(Some(over_limit), None);
        assert_eq!(code_diff(&source, &source).as_deref(), Some(""));
        assert_eq!(code_diff(&truncated, &source), None);
        assert_eq!(truncated.text, source.text);
    }
}
