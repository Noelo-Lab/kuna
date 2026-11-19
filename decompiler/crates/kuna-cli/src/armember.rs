//! Archive inputs: list the members of an `ar` static library, a tar or zip
//! archive or a U-Boot FIT image, select one, and stage it for the path-based
//! loader.
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
//! named directly. A name stored more than once is spelled `NAME#K` for its
//! K-th occurrence.
//!
//! Members are not linked against each other: a call into another member
//! stays an external reference, as it is in the `.o` on its own.
//!
//! A tar archive (plain or gzip-compressed), a zip archive and a FIT image
//! (`kuna_analysis::loader::firmware::bundles`) take the same spellings; a
//! member is named by its stored path, or its `/images` node name. A FIT
//! with no member named loads the image its default configuration boots, and
//! an image that states a load address is handed to the firmware front end
//! as a flat image at that address. A tar or zip archive that holds no
//! object file passes through unless a member is named.

use std::borrow::Cow;
use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use kuna_analysis::loader::firmware::bundles::{self, Bundle, Entry, Kind};
use kuna_analysis::loader::firmware::containers::Flat;
use object::read::archive::ArchiveFile;
use object::{Object, ObjectSymbol};

use crate::jsonfmt::{dumps_indent2, Json};

const MAGIC: &[u8] = b"!<arch>\n";
const THIN_MAGIC: &[u8] = b"!<thin>\n";
const NOT_ARCHIVE: &str = "--member applies to an archive input (ar, tar, zip or FIT image)";

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
    "blobs",
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
    /// The first argument is a FIT whose selected image loads as this flat image.
    Flat(Box<Flat>),
}

/// One member: `name` is how it is selected (`stored`, or `stored#K` when
/// several members store that name). A thin archive stores only a path
/// relative to the archive, whose regular file `file` is read on demand.
struct Member<'a> {
    name: String,
    stored: String,
    data: &'a [u8],
    file: Option<PathBuf>,
    packed: Option<(&'a Bundle<'a>, &'a Entry)>,
    size: u64,
    object: OnceCell<bool>,
}

impl Member<'_> {
    fn bytes(&self) -> std::io::Result<Cow<'_, [u8]>> {
        match (&self.file, self.packed) {
            (Some(file), _) => std::fs::read(file).map(Cow::Owned),
            (None, Some((bundle, entry))) => bundle.unpack(entry).map_err(std::io::Error::other),
            (None, None) => Ok(Cow::Borrowed(self.data)),
        }
    }

    fn is_object(&self) -> bool {
        *self.object.get_or_init(|| self.bytes().is_ok_and(|b| is_object(&b)))
    }
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
    let mut head = Vec::with_capacity(512);
    std::fs::metadata(path).is_ok_and(|m| m.is_file())
        && std::fs::File::open(path).and_then(|f| f.take(512).read_to_end(&mut head)).is_ok()
        && (is_ar(&head) || bundles::sniff(&head))
}

fn is_ar(bytes: &[u8]) -> bool {
    bytes.starts_with(MAGIC) || bytes.starts_with(THIN_MAGIC)
}

/// The first argument naming an archive, as `(index, archive path, member)`;
/// the member is the `NAME` of a `lib.a:NAME` spelling. An argument that is an
/// existing path is never split.
fn find_archive(argv: &[String]) -> Option<(usize, String, Option<String>)> {
    argv.iter().enumerate().find_map(|(i, arg)| {
        if arg.starts_with('-') {
            return None;
        }
        if Path::new(arg).exists() {
            return is_archive(arg).then(|| (i, arg.clone(), None));
        }
        arg.match_indices(':').rev().find_map(|(at, _)| {
            let (path, member) = (&arg[..at], &arg[at + 1..]);
            (!member.is_empty() && is_archive(path))
                .then(|| (i, path.to_string(), Some(member.to_string())))
        })
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
        let stored = String::from_utf8_lossy(member.name()).into_owned();
        let (data, file, size) = if member.is_thin() {
            let file = Path::new(path).parent().unwrap_or(Path::new("")).join(&stored);
            let meta = std::fs::metadata(&file)
                .map_err(|e| format!("{path}({stored}): {}: {e}", file.display()))?;
            if !meta.is_file() {
                return Err(format!("{path}({stored}): {} is not a regular file", file.display()));
            }
            (&[][..], Some(file), meta.len())
        } else {
            let data = member
                .data(bytes)
                .map_err(|e| format!("{path}({stored}): archive member data error: {e}"))?;
            (data, None, data.len() as u64)
        };
        let name = stored.clone();
        out.push(Member { name, stored, data, file, packed: None, size, object: OnceCell::new() });
    }
    number_repeats(&mut out);
    Ok(out)
}

/// The members of a tar or zip archive or a FIT image.
fn bundle_members<'a>(bundle: &'a Bundle<'a>) -> Vec<Member<'a>> {
    let member = |entry: &'a Entry| Member {
        name: entry.name.clone(),
        stored: entry.name.clone(),
        data: &[],
        file: None,
        packed: Some((bundle, entry)),
        size: entry.size,
        object: OnceCell::new(),
    };
    let mut out: Vec<Member<'a>> = bundle.entries.iter().map(member).collect();
    number_repeats(&mut out);
    out
}

/// Spell the K-th member storing a repeated name `NAME#K`.
fn number_repeats(out: &mut [Member<'_>]) {
    let mut total: BTreeMap<String, usize> = BTreeMap::new();
    for m in out.iter() {
        *total.entry(m.stored.clone()).or_default() += 1;
    }
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for m in out.iter_mut().filter(|m| total[&m.stored] > 1) {
        let k = seen.entry(m.stored.clone()).or_default();
        *k += 1;
        m.name = format!("{}#{k}", m.stored);
    }
}

/// What a FIT image node states, as listing fields.
fn fit_fields(member: &Member<'_>) -> Vec<(String, Json)> {
    let Some(fit) = member.packed.and_then(|(_, entry)| entry.fit.as_ref()) else {
        return Vec::new();
    };
    let address = |v: Option<u64>| v.map_or(Json::Null, |v| Json::Str(format!("{v:#x}")));
    let or_none = |s: &str| if s.is_empty() { "none" } else { s }.to_string();
    vec![
        ("type".into(), Json::Str(fit.kind.clone())),
        ("arch".into(), Json::Str(fit.arch.clone())),
        ("os".into(), Json::Str(fit.os.clone())),
        ("compression".into(), Json::Str(or_none(&fit.compression))),
        ("load".into(), address(fit.load)),
        ("entry".into(), address(fit.entry)),
    ]
}

fn list(path: &str, members: &[Member<'_>], json: bool) -> i32 {
    let text = if json {
        let rows = members
            .iter()
            .map(|m| {
                let mut row = vec![
                    ("name".into(), Json::Str(m.name.clone())),
                    ("size".into(), Json::Number(m.size.to_string())),
                    ("object".into(), Json::Bool(m.is_object())),
                ];
                row.extend(fit_fields(m));
                Json::Object(row)
            })
            .collect();
        let root = Json::Object(vec![
            ("archive".into(), Json::Str(path.to_string())),
            ("members".into(), Json::Array(rows)),
        ]);
        format!("{}\n", dumps_indent2(&root))
    } else {
        members
            .iter()
            .map(|m| {
                let facts: String = fit_fields(m)
                    .iter()
                    .filter_map(|(key, value)| match value {
                        Json::Str(s) if !s.is_empty() => Some(format!("\t{key}={s}")),
                        _ => None,
                    })
                    .collect();
                format!("{}\t{}{facts}\n", m.name, m.size)
            })
            .collect()
    };
    crate::output::emit_with_status(&text, 0)
}

/// Whether `member` defines a symbol one of `wanted` spells; a Mach-O symbol
/// matches without its leading underscore too.
fn defines(member: &Member<'_>, wanted: &[&str]) -> bool {
    let Ok(data) = member.bytes() else {
        return false;
    };
    let Ok(file) = object::File::parse(&*data) else {
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
    default: Option<&str>,
    argv: &[String],
    at: usize,
) -> Result<&'m Member<'a>, String> {
    if let Some(name) = name {
        if let Some(one) = members.iter().find(|m| m.name == name) {
            return Ok(one);
        }
        let stored: Vec<&Member<'_>> = members.iter().filter(|m| m.stored == name).collect();
        return Err(if stored.is_empty() {
            format!("{path}: no member named {name}; `--member` with no name lists them")
        } else {
            format!(
                "{path}: {} members are named {name}; select one as {}",
                stored.len(),
                names(&stored)
            )
        });
    }
    if let Some(one) = default.and_then(|d| members.iter().find(|m| m.stored == d)) {
        eprintln!("note: {path}: using member {} of the default configuration", one.name);
        return Ok(one);
    }
    let loads = |m: &&Member<'_>| m.packed.is_some_and(|(_, e)| e.fit.as_ref().is_some_and(|f| f.load.is_some()));
    let objects: Vec<&Member<'_>> = members.iter().filter(|m| m.is_object()).collect();
    if let ([one], _) | ([], [one]) = (objects.as_slice(), members.iter().filter(loads).collect::<Vec<_>>().as_slice()) {
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
        .filter(|m| defines(m, &wanted))
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
    let path = inner.join(base(&member.stored, "member.o"));
    std::fs::File::create(&path)?.write_all(&member.bytes()?)?;
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
fn functions_of_all(path: &str, members: &[Member<'_>], argv: &[String]) -> i32 {
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
    for member in members.iter().filter(|m| m.is_object()) {
        let member_arg = format!("--member={}", member.name);
        let run = Command::new(&exe).arg("functions").args(argv).arg(member_arg).output();
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
    if !IMAGE_COMMANDS.contains(&sub)
        || argv.iter().any(|a| a == "-h" || a == "--help" || a == "--raw-image")
    {
        return Ok(Resolved::Run(None));
    }
    let request = take_request(argv);
    let Some((at, path, inline)) = find_archive(argv) else {
        return match request {
            Some(_) => Err(NOT_ARCHIVE.into()),
            None => Ok(Resolved::Run(None)),
        };
    };
    if argv.iter().any(|arg| arg == "--apply-rust-signatures") {
        return Err("--apply-rust-signatures requires a standalone linked ELF image; archive inputs and member selection are unsupported".into());
    }
    let bytes = std::fs::read(&path).map_err(|e| format!("{path}: {e}"))?;
    let bundle = if is_ar(&bytes) { None } else { Bundle::parse(&bytes).map_err(|e| format!("{path}: {e}"))? };
    let members = match &bundle {
        Some(bundle) => bundle_members(bundle),
        None if is_ar(&bytes) => members(&path, &bytes)?,
        None if request.is_some() || inline.is_some() => return Err(NOT_ARCHIVE.into()),
        None => return Ok(Resolved::Run(None)),
    };
    let fit = bundle.as_ref().filter(|b| b.kind == Kind::Fit);
    let name = match request {
        Some(Request::List) => {
            return Ok(Resolved::Done(list(&path, &members, argv.iter().any(|a| a == "--json"))))
        }
        Some(Request::Name(name)) => Some(name),
        None => inline,
    };
    let listing = sub == "functions" && !argv.iter().any(|a| a == "--summary");
    if name.is_none() && fit.is_none() && listing && members.iter().any(Member::is_object) {
        return Ok(Resolved::Done(functions_of_all(&path, &members, argv)));
    }
    if name.is_none() && bundle.is_some() && fit.is_none() && !members.iter().any(Member::is_object) {
        return Ok(Resolved::Run(None));
    }
    let default = fit.and_then(|b| b.default.as_deref());
    let member = select(&path, &members, name.as_deref(), default, argv, at)?;
    if let (Some((bundle, entry)), true) = (member.packed, at == 0 && crate::rawfront::RAW_COMMANDS.contains(&sub)) {
        if let Some(flat) = bundle.flat(entry).map_err(|e| format!("{path}: {e}"))?.filter(|f| !is_object(&f.bytes)) {
            return Ok(Resolved::Flat(Box::new(flat)));
        }
    }
    if let Some(file) = &member.file {
        let stored = Path::new(&member.stored);
        if stored.is_absolute() || stored.components().any(|c| c == Component::ParentDir) {
            eprintln!("note: {path}: thin member {} is read from {}", member.name, file.display());
        }
        argv[at] = file.to_string_lossy().into_owned();
        return Ok(Resolved::Run(None));
    }
    let (dir, staged) = stage(&path, member)
        .map_err(|e| format!("{path}: cannot stage member {}: {e}", member.name))?;
    argv[at] = staged.to_string_lossy().into_owned();
    Ok(Resolved::Run(Some(dir)))
}
