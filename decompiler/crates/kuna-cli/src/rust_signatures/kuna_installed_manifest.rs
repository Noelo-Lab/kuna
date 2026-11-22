//! Check a locally installed Rust signature manifest without loading an image.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::{load, Corpus};

const USAGE: &str = "usage: kuna rust-signatures check [--manifest PATH] [--json]\n";
const INSTALLED_MANIFEST: &str = "kuna/rust-signatures/x86_64/linux/manifest.json";

#[derive(Serialize)]
struct Inventory {
    corpus: Corpus,
    manifest_sha256: String,
    profile_count: usize,
    verified_bank_count: usize,
}

#[derive(Serialize)]
struct Check {
    status: &'static str,
    manifest_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inventory: Option<Inventory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

pub(crate) fn run(args: &[String]) -> i32 {
    crate::args::report(run_check(args))
}

fn run_check(args: &[String]) -> Result<i32, String> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        return Ok(crate::output::emit_with_status(
            &format!(
                "{USAGE}\nVerify a local Rust signature corpus without loading an image.\n\
                 --json emits {{status,manifest_path,inventory?,reason?}}.\n"
            ),
            0,
        ));
    }
    let Some((subcommand, flags)) = args.split_first() else {
        return Err(USAGE.trim_end().into());
    };
    if subcommand != "check" {
        return Err(format!(
            "unknown rust-signatures subcommand {subcommand:?}\n{USAGE}"
        ));
    }

    let mut manifest = None;
    let mut json = false;
    let mut i = 0;
    while let Some(arg) = flags.get(i) {
        match arg.as_str() {
            "--manifest" if manifest.is_none() => {
                manifest = Some(PathBuf::from(crate::args::take_value(flags, &mut i, arg)?));
            }
            "--manifest" => return Err("duplicate --manifest".into()),
            "--json" => json = true,
            _ => return Err(format!("unknown option {arg:?}\n{USAGE}")),
        }
        i += 1;
    }

    let path = manifest.or_else(default_manifest);
    let check = verify(path.as_deref());
    let status = if check.status == "ready" { 0 } else { 1 };
    let text = if json {
        let text = serde_json::to_string_pretty(&check).map_err(|error| error.to_string())?;
        format!("{text}\n")
    } else {
        let path = check
            .manifest_path
            .as_deref()
            .unwrap_or("the user data directory");
        let status = check.status;
        match (check.inventory, check.reason) {
            (Some(inventory), _) => format!(
                "Rust signature corpus {status} at {path} ({} profiles, {} verified banks)\n",
                inventory.profile_count, inventory.verified_bank_count,
            ),
            (_, Some(reason)) => format!("Rust signature corpus {status} at {path} ({reason})\n"),
            _ => format!("Rust signature corpus {status} at {path}\n"),
        }
    };
    Ok(crate::output::emit_with_status(&text, status))
}

fn default_manifest() -> Option<PathBuf> {
    let absolute_env = |name| {
        std::env::var_os(name)
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
    };
    let directory = if cfg!(windows) {
        absolute_env("LOCALAPPDATA")
    } else if cfg!(target_os = "macos") {
        absolute_env("HOME").map(|home| home.join("Library/Application Support"))
    } else {
        absolute_env("XDG_DATA_HOME")
            .or_else(|| absolute_env("HOME").map(|home| home.join(".local/share")))
    };
    Some(directory?.join(INSTALLED_MANIFEST))
}

fn verify(path: Option<&Path>) -> Check {
    let mut check = Check {
        status: "absent",
        manifest_path: path.map(|path| path.to_string_lossy().into_owned()),
        inventory: None,
        reason: None,
    };
    let Some(path) = path else { return check };
    let manifest = match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_file() => load(path).map(|(_, info)| info),
        Ok(_) => Err("manifest is not a regular file".into()),
        Err(error) if error.kind() == ErrorKind::NotFound => return check,
        Err(error) => Err(error.to_string()),
    };
    match manifest {
        Ok(info) => {
            let profile_count = info.candidates.len();
            let verified_bank_count = info.signatures.len();
            // The loader rejects duplicate and unrequested banks.
            check.status = if verified_bank_count == profile_count * 2 {
                "ready"
            } else {
                "incomplete"
            };
            check.manifest_path = Some(info.manifest_path);
            check.inventory = Some(Inventory {
                corpus: info.corpus,
                manifest_sha256: info.manifest_sha256,
                profile_count,
                verified_bank_count,
            });
        }
        Err(reason) => {
            check.status = "invalid";
            check.reason = Some(reason);
        }
    }
    check
}
