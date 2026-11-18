//! `ar` archive inputs: list a static library's members, select one, and stage
//! it for the path-based loader.
//!
//! ```text
//!   kuna <command> <lib.a> --member            # list the members
//!   kuna <command> <lib.a> --member NAME ...   # run on one member
//!   kuna <command> <lib.a>:NAME ...            # the same, as one path
//!   kuna decompile <lib.a> <symbol>            # the member defining <symbol>
//!   kuna functions <lib.a>                     # every member, names as member:name
//! ```
//!
//! A member is a relocatable object like any other `.o` input, so selection
//! ends by writing it to an owned temporary file and substituting that path in
//! the command's arguments. A thin archive's member is already a file and is
//! named directly.
//!
//! Members are not linked against each other: a call into another member
//! stays an external reference, as it is in the `.o` on its own.

use std::borrow::Cow;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

use object::read::archive::ArchiveFile;
use object::{Object, ObjectSymbol};

use crate::jsonfmt::{dumps_indent2, Json};

const MAGIC: &[u8] = b"!<arch>\n";
const THIN_MAGIC: &[u8] = b"!<thin>\n";

/// The subcommands whose input is one loadable image.
const IMAGE_COMMANDS: &[&str] = &[
    "decompile",
    "decompile-all",
    "decompile-project",
    "decompile-graph",
    "functions",
    "disassemble",
    "raw-pcode",
    "read",
    "xrefs",
    "strings",
    "crypto",
];

/// What `--member` asked for.
enum Request {
    List,
    Name(String),
}

/// How an archive input was settled.
pub(crate) enum Resolved {
    /// The arguments now name a loadable file; the guard owns a staged member.
    Run(Option<tempfile::TempDir>),
    /// The archive itself was the answer; exit with this status.
    Done(i32),
}

/// One member's name and bytes. A thin archive stores only the name, a path
/// relative to the archive, which `file` then holds.
struct Member<'a> {
    name: String,
    data: Cow<'a, [u8]>,
    file: Option<PathBuf>,
}

/// Remove a global `--member [NAME]` from `argv`. A `--member` that ends the
/// arguments or precedes another flag carries no name and asks for the listing.
fn take_request(argv: &mut Vec<String>) -> Option<Request> {
    let mut request = None;
    let mut i = 0;
    while i < argv.len() {
        if let Some(name) = argv[i].strip_prefix("--member=") {
            request = Some(Request::Name(name.to_string()));
            argv.remove(i);
        } else if argv[i] == "--member" {
            match argv.get(i + 1).filter(|next| !next.starts_with('-')).cloned() {
                Some(name) => {
                    request = Some(Request::Name(name));
                    argv.drain(i..i + 2);
                }
                None => {
                    request = Some(Request::List);
                    argv.remove(i);
                }
            }
        } else {
            i += 1;
        }
    }
    request
}

fn is_archive(path: &str) -> bool {
    use std::io::Read;
    let mut head = [0u8; 8];
    std::fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut head))
        .is_ok_and(|()| head == MAGIC || head == THIN_MAGIC)
}

/// The first argument naming an archive, as `(index, archive path, member)`;
/// the member is the `NAME` of a `lib.a:NAME` spelling.
fn find_archive(argv: &[String]) -> Option<(usize, String, Option<String>)> {
    argv.iter().enumerate().find_map(|(i, arg)| {
        if arg.starts_with('-') {
            return None;
        }
        if is_archive(arg) {
            return Some((i, arg.clone(), None));
        }
        let (path, member) = arg.rsplit_once(':')?;
        (!member.is_empty() && is_archive(path))
            .then(|| (i, path.to_string(), Some(member.to_string())))
    })
}

fn is_object(bytes: &[u8]) -> bool {
    object::File::parse(bytes).is_ok()
}

fn members<'a>(path: &str, bytes: &'a [u8]) -> Result<Vec<Member<'a>>, String> {
    let archive =
        ArchiveFile::parse(bytes).map_err(|e| format!("{path}: not a valid archive: {e}"))?;
    let mut out = Vec::new();
    for member in archive.members() {
        let member = member.map_err(|e| format!("{path}: archive member error: {e}"))?;
        let name = String::from_utf8_lossy(member.name()).into_owned();
        let (data, file) = if member.is_thin() {
            let file = Path::new(path).parent().unwrap_or(Path::new("")).join(&name);
            let data = std::fs::read(&file).map_err(|e| format!("{path}({name}): {e}"))?;
            (Cow::Owned(data), Some(file))
        } else {
            let data = member
                .data(bytes)
                .map_err(|e| format!("{path}({name}): archive member data error: {e}"))?;
            (Cow::Borrowed(data), None)
        };
        out.push(Member { name, data, file });
    }
    Ok(out)
}

fn list(path: &str, members: &[Member<'_>], json: bool) -> i32 {
    let text = if json {
        let rows = members
            .iter()
            .map(|m| {
                Json::Object(vec![
                    ("name".into(), Json::Str(m.name.clone())),
                    ("size".into(), Json::Number(m.data.len().to_string())),
                    ("object".into(), Json::Bool(is_object(&m.data))),
                ])
            })
            .collect();
        let root = Json::Object(vec![
            ("archive".into(), Json::Str(path.to_string())),
            ("members".into(), Json::Array(rows)),
        ]);
        format!("{}\n", dumps_indent2(&root))
    } else {
        members.iter().map(|m| format!("{}\t{}\n", m.name, m.data.len())).collect()
    };
    crate::output::emit_with_status(&text, 0)
}

/// Whether the object in `data` defines a symbol one of `wanted` spells; a
/// Mach-O symbol matches without its leading underscore too.
fn defines(data: &[u8], wanted: &[&str]) -> bool {
    let Ok(file) = object::File::parse(data) else {
        return false;
    };
    file.symbols().filter(|s| s.is_definition()).any(|s| {
        s.name().is_ok_and(|name| {
            wanted.iter().any(|w| name == *w || name.strip_prefix('_') == Some(*w))
        })
    })
}

fn names(members: &[&Member<'_>]) -> String {
    members.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", ")
}

/// The one member the arguments select: the named one, the only object, or
/// the only object defining a symbol the arguments spell.
fn select<'m, 'a>(
    path: &str,
    members: &'m [Member<'a>],
    name: Option<&str>,
    argv: &[String],
    at: usize,
) -> Result<&'m Member<'a>, String> {
    if let Some(name) = name {
        let named: Vec<&Member<'_>> = members.iter().filter(|m| m.name == name).collect();
        return match named.as_slice() {
            [one] => Ok(one),
            [] => Err(format!(
                "{path}: no member named {name}; `--member` with no name lists them"
            )),
            many => Err(format!("{path}: {} members are named {name}", many.len())),
        };
    }
    let objects: Vec<&Member<'_>> =
        members.iter().filter(|m| is_object(&m.data)).collect();
    if let [one] = objects.as_slice() {
        return Ok(one);
    }
    if objects.is_empty() {
        return Err(format!("{path}: the archive holds no object member"));
    }
    let wanted: Vec<&str> = argv
        .iter()
        .enumerate()
        .filter(|(i, arg)| *i != at && !arg.starts_with('-'))
        .map(|(_, arg)| arg.as_str())
        .collect();
    let defining: Vec<&Member<'_>> = objects
        .iter()
        .copied()
        .filter(|m| defines(&m.data, &wanted))
        .collect();
    match defining.as_slice() {
        [one] => {
            eprintln!("note: {path}: using member {}", one.name);
            Ok(one)
        }
        [] => Err(format!(
            "{path} is an archive of {} object members and the arguments name no symbol \
             one of them defines; select one with --member NAME ({})",
            objects.len(),
            names(&objects)
        )),
        many => Err(format!(
            "{path}: more than one member defines that symbol; select one with \
             --member NAME ({})",
            names(many)
        )),
    }
}

/// Write `member` to an owned temporary directory as `<archive>/<member>`, so
/// a path echoed in output still reads as the member of its archive.
fn stage(archive: &str, member: &Member<'_>) -> std::io::Result<(tempfile::TempDir, PathBuf)> {
    let base = |path: &str, fallback: &str| {
        Path::new(path).file_name().map_or_else(|| fallback.into(), |n| n.to_os_string())
    };
    let dir = tempfile::Builder::new().prefix("kuna_ar_").tempdir()?;
    let inner = dir.path().join(base(archive, "archive"));
    std::fs::create_dir(&inner)?;
    let path = inner.join(base(&member.name, "member.o"));
    std::fs::File::create(&path)?.write_all(&member.data)?;
    Ok((dir, path))
}

/// One `functions` listing line with its name prefixed `member:`.
fn prefixed(member: &str, line: &str) -> String {
    match line.split_once('\t') {
        Some((address, rest)) => format!("{address}\t{member}:{rest}\n"),
        None => format!("{line}\n"),
    }
}

/// `kuna functions <lib.a>`: list every object member through its own run and
/// join the listings, each name prefixed `member:`. With `--json` the result is
/// `{archive, members: [{member, functions: <that member's document>}]}`.
fn functions_of_all(path: &str, members: &[Member<'_>], argv: &[String], at: usize) -> i32 {
    let json = argv.iter().any(|a| a == "--json");
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(e) => {
            eprintln!("error: cannot re-run kuna for each member: {e}");
            return 1;
        }
    };
    let mut text = String::new();
    let mut listed = 0usize;
    let mut status = 0;
    for member in members.iter().filter(|m| is_object(&m.data)) {
        let mut args = argv.to_vec();
        args[at] = format!("{path}:{}", member.name);
        let run = Command::new(&exe).arg("functions").args(&args).output();
        let out = match run {
            Ok(out) => out,
            Err(e) => {
                eprintln!("error: {path}({}): {e}", member.name);
                return 1;
            }
        };
        eprint!("{}", String::from_utf8_lossy(&out.stderr));
        if !out.status.success() {
            eprintln!("note: {path}({}): skipped", member.name);
            status = out.status.code().unwrap_or(1);
            continue;
        }
        let body = String::from_utf8_lossy(&out.stdout);
        if json {
            text.push_str(if listed == 0 { "\n" } else { ",\n" });
            let name = dumps_indent2(&Json::Str(member.name.clone()));
            text.push_str(&format!("{{\"member\": {name}, \"functions\": {}}}", body.trim_end()));
        } else {
            text.extend(body.lines().map(|line| prefixed(&member.name, line)));
        }
        listed += 1;
    }
    if json {
        let archive = dumps_indent2(&Json::Str(path.to_string()));
        text = format!("{{\"archive\": {archive}, \"members\": [{text}\n]}}\n");
    }
    crate::output::emit_with_status(&text, if listed == 0 { status.max(1) } else { 0 })
}

/// Settle an archive named in `argv` before `sub` runs: answer a listing,
/// fan `functions` out over the members, or replace the archive argument with
/// the selected member's path. Arguments naming no archive pass through.
pub(crate) fn resolve(sub: &str, argv: &mut Vec<String>) -> Result<Resolved, String> {
    if !IMAGE_COMMANDS.contains(&sub) || argv.iter().any(|a| a == "-h" || a == "--help") {
        return Ok(Resolved::Run(None));
    }
    let request = take_request(argv);
    let Some((at, path, inline)) = find_archive(argv) else {
        return match request {
            Some(_) => Err("--member applies to an ar archive input".into()),
            None => Ok(Resolved::Run(None)),
        };
    };
    let bytes = std::fs::read(&path).map_err(|e| format!("{path}: {e}"))?;
    let members = members(&path, &bytes)?;
    let name = match request {
        Some(Request::List) => {
            return Ok(Resolved::Done(list(&path, &members, argv.iter().any(|a| a == "--json"))))
        }
        Some(Request::Name(name)) => Some(name),
        None => inline,
    };
    let objects = members.iter().filter(|m| is_object(&m.data)).count();
    let listing = sub == "functions" && !argv.iter().any(|a| a == "--summary");
    if name.is_none() && listing && objects > 1 {
        return Ok(Resolved::Done(functions_of_all(&path, &members, argv, at)));
    }
    let member = select(&path, &members, name.as_deref(), argv, at)?;
    if let Some(file) = &member.file {
        argv[at] = file.to_string_lossy().into_owned();
        return Ok(Resolved::Run(None));
    }
    let (dir, staged) = stage(&path, member)
        .map_err(|e| format!("{path}: cannot stage member {}: {e}", member.name))?;
    argv[at] = staged.to_string_lossy().into_owned();
    Ok(Resolved::Run(Some(dir)))
}
