//! Publish a named function and its original binary to Kuna's BSim server.

use std::path::Path;

use kuna_analysis::bsim::lshweights::LshProfile;
use kuna_bsim_server::PublishRequest;
use serde_json::json;

use super::{parse_address, server_client::{Client, MatchId}};

struct Args<'a> {
    binary: &'a str,
    server: &'a str,
    entry: PublishRequest,
    json: bool,
}

const USAGE: &str = "usage: kuna bsim push <binary> --server URL (--address A | --name N)\n\
                    \x20                     [--rename NAME] [--profile PROFILE] [--json]";

pub(super) fn run(argv: &[String]) -> i32 {
    if matches!(argv, [arg] if arg == "--help" || arg == "-h") {
        println!("{USAGE}\nUses KUNA_BSIM_SERVER_WRITE_TOKEN. JSON: {{match_id, profile, sha256, address, name, outcome}}.");
        return 0;
    }
    let args = match parse(argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("kuna bsim push: {err}\n{USAGE}");
            return 2;
        }
    };
    match push(args) {
        Ok(out) => crate::output::emit_with_status(&out, 0),
        Err(err) => {
            eprintln!("kuna bsim push: {}", super::TerminalText(&err));
            1
        }
    }
}

fn parse(argv: &[String]) -> Result<Args<'_>, String> {
    let (mut binary, mut server, mut address, mut name, mut rename, mut profile) =
        (None, None, None, None, None, None);
    let mut json = false;
    let mut iter = argv.iter().map(String::as_str);
    while let Some(arg) = iter.next() {
        let mut take = || iter.next().filter(|s| !s.is_empty()).ok_or_else(|| format!("{arg} requires a value"));
        match arg {
            "--server" if server.is_none() => server = Some(take()?),
            "--address" if address.is_none() => address = Some(parse_address(take()?)?),
            "--name" if name.is_none() => name = Some(take()?.to_owned()),
            "--rename" if rename.is_none() => rename = Some(take()?.to_owned()),
            "--profile" if profile.is_none() => {
                let value = take()?;
                profile = Some(LshProfile::from_name(value).ok_or_else(|| format!("unknown BSim profile {value:?}"))?.name().to_owned());
            }
            "--json" => json = true,
            s if binary.is_none() && !s.starts_with('-') => binary = Some(s),
            _ => return Err(format!("unexpected argument {arg:?}")),
        }
    }
    if address.is_some() == name.is_some() {
        return Err("choose exactly one function with --address or --name".into());
    }
    let binary = binary.ok_or("a binary path is required")?;
    let executable = Path::new(binary).file_name().and_then(|s| s.to_str())
        .ok_or("binary path must have a UTF-8 filename")?.to_owned();
    Ok(Args {
        binary,
        server: server.ok_or("--server is required")?,
        entry: PublishRequest { executable, address, symbol: name, rename, profile },
        json,
    })
}

fn push(args: Args<'_>) -> Result<String, String> {
    let published = Client::writer(args.server)?.publish(Path::new(args.binary), &args.entry)?;
    let address = format!("{:#x}", published.address);
    let profile = LshProfile::from_name(&published.profile).ok_or("server returned an invalid profile")?;
    let id = MatchId { profile, sha256: &published.sha256, address: published.address }.to_string();
    if args.json {
        let json = json!({
            "match_id": id,
            "profile": published.profile,
            "sha256": published.sha256,
            "address": address,
            "name": published.name,
            "outcome": published.outcome,
        });
        Ok(format!("{}\n", serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?))
    } else {
        Ok(format!("{} @ {address} ({:?})\nmatch ID: {id}\n",
            super::TerminalText(&published.name), published.outcome))
    }
}
