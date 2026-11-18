//! (kuna `objcsdk`) C function prototypes from SDK headers, for the functions
//! an image imports.
//!
//! The headers `objcsdk` names declare far more than Objective-C methods:
//! CoreFoundation, libdispatch, Security, CoreGraphics, `os_log` and, in a
//! kernel SDK, the XNU functions a kext calls. An import of one of those had no
//! prototype, so a call whose arguments were set up before an earlier call lost
//! them (`CFStringGetLength()`) and a tail call printed bare
//! (`dispatch_async_f()`). This reads the C half of the same directory: an SDK
//! root contributes the `Headers/` of the frameworks the image links plus
//! `usr/include` (a kext: `Kernel.framework/Headers` alone), any other
//! directory every `.h` under it.
//!
//! Only declarations of names the image imports are parsed. A declaration is
//! `specifiers name(parameters)` with attribute macros (`CF_EXPORT`,
//! `API_AVAILABLE(...)`, `__attribute__((...))`) skipped, read from text cut
//! down to the target: an `#if` over the pointer width, the architecture or
//! the platform keeps only its live branch. Its types resolve
//! through `typedef`s, `struct` / `union` / `enum` definitions, the
//! `NS_ENUM` / `CF_ENUM` / `CF_OPTIONS` and `OS_ENUM` / `DISPATCH_ENUM` macro
//! forms, and the object-declaring macros (`DISPATCH_DECL(dispatch_queue)`
//! defines `dispatch_queue_t`). A `struct` whose members all resolve becomes a
//! real layout, so a pointer parameter to it has fields; an enum becomes its
//! named values.
//!
//! Nothing is guessed. A function gets no prototype, and keeps the call-site
//! inference, when a type does not resolve, when a by-value aggregate is not
//! placed in registers the way its members would be, or when two declarations
//! disagree (both branches of an `#if` the target does not decide are read).
//! A record defined only under such an `#if` gets no layout. A typedef or
//! record defined two ways is unresolved, except that a pointer to it is still
//! a pointer. A name the built-in libc tables type is left to them. Nothing
//! derived from the headers is written anywhere.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use object::macho;
use object::read::macho::{LoadCommandVariant, MachHeader};
use object::{FileKind, Object};

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use kuna_decomp::fspec::PrototypePieces;

use crate::analyzers::objc::kuna_objcsdk as objcsdk;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// Seeds the header-declared prototype of every imported C function.
pub struct SdkProtoPass;

impl AnalysisPass for SdkProtoPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "sdkproto"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if ctx.file.format() != object::BinaryFormat::MachO {
            return out;
        }
        let Some(root) = kuna_decomp::kuna_objcsdk::sdk_path() else { return out };
        let Some(image) = ctx.image else { return out };
        let (imports, reloc_imports) = super::loaded_import_addrs(ctx.file, ctx.bytes, image);
        let by_name = super::loaded_function_names(ctx.file, ctx.bytes, &reloc_imports, true);
        let wanted: BTreeSet<&str> = imports
            .iter()
            .map(|(name, _)| name.as_str())
            .chain(by_name.iter().map(String::as_str))
            .filter(|name| !name.starts_with("objc_msgSend") && !super::has_builtin_prototype(name))
            .collect();
        if wanted.is_empty() {
            return out;
        }
        let frameworks = objcsdk::linked_frameworks(ctx.file, ctx.bytes);
        let (ptr_size, word_size) = ctx.arch.data_org();
        let target = Target { ptr_size, machine: ctx.file.architecture(), platform: platform(ctx.file, ctx.bytes) };
        let texts = read_headers(&root, &frameworks, is_kext(ctx.bytes), target);
        let index = Index::new(&texts, &wanted, target);
        let mut builder = Builder::new(&index, ctx.arch.types(), ptr_size, word_size);
        for name in &wanted {
            let Some(pieces) = builder.prototype(name) else { continue };
            for (_, addr) in imports.iter().filter(|(n, _)| n == name) {
                out.prototypes_at.push((*addr, pieces.clone()));
            }
            if by_name.contains(*name) {
                out.prototypes.push(pieces);
            }
        }
        out
    }
}

/// What the image was built for, as far as it decides which header text applies.
#[derive(Clone, Copy)]
struct Target {
    ptr_size: i32,
    machine: object::Architecture,
    /// The Mach-O `PLATFORM_*` of the build version, zero when the image names none.
    platform: u32,
}

/// The platform an image's `LC_BUILD_VERSION` or `LC_VERSION_MIN_*` names.
fn platform(file: &object::File, bytes: &[u8]) -> u32 {
    fn read<Mach: MachHeader<Endian = object::Endianness>>(bytes: &[u8]) -> Option<u32> {
        let header = Mach::parse(bytes, 0).ok()?;
        let endian = header.endian().ok()?;
        let mut commands = header.load_commands(endian, bytes, 0).ok()?;
        while let Ok(Some(command)) = commands.next() {
            match command.variant() {
                Ok(LoadCommandVariant::BuildVersion(build)) => return Some(build.platform.get(endian)),
                Ok(LoadCommandVariant::VersionMin(_)) => {
                    return Some(match command.cmd() {
                        macho::LC_VERSION_MIN_MACOSX => macho::PLATFORM_MACOS,
                        macho::LC_VERSION_MIN_IPHONEOS => macho::PLATFORM_IOS,
                        macho::LC_VERSION_MIN_TVOS => macho::PLATFORM_TVOS,
                        _ => macho::PLATFORM_WATCHOS,
                    })
                }
                _ => {}
            }
        }
        None
    }
    let found = match (file.format(), FileKind::parse(bytes)) {
        (object::BinaryFormat::MachO, Ok(FileKind::MachO64)) => read::<macho::MachHeader64<object::Endianness>>(bytes),
        (object::BinaryFormat::MachO, Ok(FileKind::MachO32)) => read::<macho::MachHeader32<object::Endianness>>(bytes),
        _ => None,
    };
    found.unwrap_or(macho::PLATFORM_UNKNOWN)
}

/// Whether the Mach-O header says `MH_KEXT_BUNDLE`.
fn is_kext(bytes: &[u8]) -> bool {
    let Some(head) = bytes.get(..16) else { return false };
    let word = |at: usize, be: bool| {
        let w = [head[at], head[at + 1], head[at + 2], head[at + 3]];
        if be { u32::from_be_bytes(w) } else { u32::from_le_bytes(w) }
    };
    match word(0, false) {
        0xfeed_face | 0xfeed_facf => word(12, false) == object::macho::MH_KEXT_BUNDLE,
        0xcefa_edfe | 0xcffa_edfe => word(12, true) == object::macho::MH_KEXT_BUNDLE,
        _ => false,
    }
}

/// The header files the C reader takes from `root`, sorted.
fn header_paths(root: &Path, frameworks: &[String], kext: bool) -> Vec<PathBuf> {
    let fw_root = root.join("System/Library/Frameworks");
    let mut out = Vec::new();
    if !fw_root.is_dir() {
        objcsdk::walk(root, &mut out, 0);
    } else if kext && fw_root.join("Kernel.framework/Headers").is_dir() {
        objcsdk::walk(&fw_root.join("Kernel.framework/Headers"), &mut out, 0);
    } else {
        let dirs: Vec<PathBuf> = if frameworks.is_empty() {
            std::fs::read_dir(&fw_root)
                .map(|d| d.flatten().map(|e| e.path()).collect())
                .unwrap_or_default()
        } else {
            frameworks.iter().map(|f| fw_root.join(format!("{f}.framework"))).collect()
        };
        for dir in dirs {
            if !dir.ends_with("Kernel.framework") && !dir.ends_with("DriverKit.framework") {
                objcsdk::walk(&dir.join("Headers"), &mut out, 0);
            }
        }
        let include = root.join("usr/include");
        let cxx = include.join("c++");
        let mut c = Vec::new();
        objcsdk::walk(&include, &mut c, 0);
        out.extend(c.into_iter().filter(|p| !p.starts_with(&cxx)));
    }
    out.sort();
    out
}

/// One header with comments, preprocessor lines and Objective-C bodies removed.
struct Header {
    text: String,
    classes: Vec<String>,
    /// As [`Selected::undecided`] and [`Selected::packing`], over `text`.
    undecided: Vec<(usize, usize)>,
    packing: Vec<(usize, usize, u8)>,
}

fn read_headers(root: &Path, frameworks: &[String], kext: bool, target: Target) -> Vec<Header> {
    header_paths(root, frameworks, kext)
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .map(|raw| header(&raw, target))
        .collect()
}

/// Whether a predefined macro is set for the target: the pointer width, the
/// architecture, the platform (`TARGET_OS_*`), and what holds for any C
/// compile of an Apple target. `None` for a macro only the build decides.
fn predefined(name: &str, target: Target) -> Option<bool> {
    use object::Architecture as A;
    let arm64 = matches!(target.machine, A::Aarch64 | A::Aarch64_Ilp32);
    let on = |platforms: &[u32]| (target.platform != macho::PLATFORM_UNKNOWN).then(|| platforms.contains(&target.platform));
    Some(match name {
        "__LP64__" | "TARGET_RT_64_BIT" => target.ptr_size == 8,
        "__arm64__" | "__aarch64__" | "TARGET_CPU_ARM64" => arm64,
        "__x86_64__" | "TARGET_CPU_X86_64" => target.machine == A::X86_64,
        "__i386__" | "TARGET_CPU_X86" => target.machine == A::I386,
        "__arm__" | "TARGET_CPU_ARM" => target.machine == A::Arm,
        "__cplusplus" | "_WIN32" | "_WIN64" | "_MSC_VER" | "__linux__" | "__ANDROID__" | "__CYGWIN__" | "__BIG_ENDIAN__" | "TARGET_RT_BIG_ENDIAN"
        | "TARGET_OS_WIN32" | "TARGET_OS_WINDOWS" | "TARGET_OS_LINUX" | "TARGET_OS_UNIX" => false,
        "__APPLE__" | "__MACH__" | "__GNUC__" | "__clang__" | "__BLOCKS__" | "__LITTLE_ENDIAN__" | "TARGET_RT_LITTLE_ENDIAN" | "TARGET_OS_MAC" | "__MigPackStructs" => true,
        // The 32-bit inode and pre-UNIX03 variants exist on Intel alone.
        "__DARWIN_64_BIT_INO_T" | "__DARWIN_ONLY_64_BIT_INO_T" | "__DARWIN_UNIX03" | "__DARWIN_ONLY_UNIX_CONFORMANCE" if arm64 => true,
        "TARGET_OS_OSX" => on(&[macho::PLATFORM_MACOS])?,
        "TARGET_OS_IPHONE" => !on(&[macho::PLATFORM_MACOS, macho::PLATFORM_DRIVERKIT])?,
        "TARGET_OS_IOS" => on(&[macho::PLATFORM_IOS, macho::PLATFORM_MACCATALYST, macho::PLATFORM_IOSSIMULATOR])?,
        "TARGET_OS_TV" => on(&[macho::PLATFORM_TVOS, macho::PLATFORM_TVOSSIMULATOR])?,
        "TARGET_OS_WATCH" => on(&[macho::PLATFORM_WATCHOS, macho::PLATFORM_WATCHOSSIMULATOR])?,
        "TARGET_OS_MACCATALYST" => on(&[macho::PLATFORM_MACCATALYST])?,
        "TARGET_OS_SIMULATOR" => on(&[macho::PLATFORM_IOSSIMULATOR, macho::PLATFORM_TVOSSIMULATOR, macho::PLATFORM_WATCHOSSIMULATOR, macho::PLATFORM_VISIONOSSIMULATOR])?,
        _ => return None,
    })
}

/// The truth of an `#if` expression over [`predefined`] macros, `defined`,
/// `!`, `&&` and `||`; `None` when it turns on anything else.
fn condition(t: &[Tok], target: Target, depth: usize) -> Option<bool> {
    if depth > MAX_DEPTH {
        return None;
    }
    let operand = |t: &[Tok]| -> Option<bool> {
        let negated = t.iter().take_while(|t| **t == Tok::Punct(b'!')).count();
        let value = match t[negated..] {
            [Tok::Ident("defined"), Tok::Ident(name)] | [Tok::Ident(name)] => predefined(name, target),
            [Tok::Ident("defined"), Tok::Group(b'(', inner)] => predefined(inner.trim(), target),
            [Tok::Group(b'(', inner)] => condition(&lex(inner), target, depth + 1),
            [Tok::Ident("__has_feature" | "__has_extension"), Tok::Group(b'(', feature)] => {
                matches!(feature.trim(), "assume_nonnull" | "nullability" | "attribute_availability_with_message" | "blocks" | "enumerator_attributes").then_some(true)
            }
            [Tok::Num(n)] => Some(n != "0"),
            _ => None,
        };
        value.map(|v| v ^ (negated % 2 == 1))
    };
    let join = |parts: Vec<Option<bool>>, decisive: bool| {
        if parts.contains(&Some(decisive)) {
            Some(decisive)
        } else {
            parts.iter().all(Option::is_some).then_some(!decisive)
        }
    };
    let all = |t: &[Tok]| join(t.split(|x| *x == Tok::Punct(b'&')).filter(|p| !p.is_empty()).map(operand).collect(), false);
    let mut alternatives = Vec::new();
    let mut from = 0;
    for k in 0..t.len() {
        if t[k] == Tok::Punct(b'|') && t.get(k + 1) == Some(&Tok::Punct(b'|')) {
            alternatives.push(all(&t[from..k]));
            from = k + 2;
        }
    }
    alternatives.push(all(&t[from.min(t.len())..]));
    join(alternatives, true)
}

/// A header cut down to the target.
struct Selected {
    text: String,
    /// The ranges of `text` under a condition that cannot be decided.
    undecided: Vec<(usize, usize)>,
    /// The ranges of `text` under a packing pragma, with the alignment cap in
    /// force; a cap of zero is one the reader could not follow.
    packing: Vec<(usize, usize, u8)>,
}

/// One open `#if` … `#endif`.
struct Conditional {
    /// The text around it is read.
    outer: bool,
    /// A branch is known taken; `None` while that is undecided.
    taken: Option<bool>,
    /// The current branch is read.
    read: bool,
    /// The current branch is read only because its condition is undecided.
    unsure: bool,
}

/// The alignment cap a `#pragma pack` or `#pragma options align` leaves in
/// force (`None`: natural alignment), or `Err` for a form not followed.
fn packing_pragma(rest: &str, cap: Option<u8>, stack: &mut Vec<Option<u8>>) -> Result<Option<u8>, ()> {
    let width = |n: &str| match n.parse::<u8>() {
        Ok(n @ (1 | 2 | 4)) => Ok(Some(n)),
        Ok(8 | 16) => Ok(None),
        _ => Err(()),
    };
    let rest = rest.trim();
    if let Some(args) = rest.strip_prefix("pack") {
        let args = args.trim().strip_prefix('(').and_then(|a| a.split(')').next()).ok_or(())?;
        let args: Vec<&str> = args.split(',').map(str::trim).filter(|a| !a.is_empty()).collect();
        return match args[..] {
            [] => Ok(None),
            ["push"] => {
                stack.push(cap);
                Ok(cap)
            }
            ["push", n] => {
                stack.push(cap);
                width(n)
            }
            ["pop"] => Ok(stack.pop().flatten()),
            [n] => width(n),
            _ => Err(()),
        };
    }
    let Some(mode) = rest.strip_prefix("options").and_then(|o| o.split_once("align=")).map(|(_, m)| m.trim()) else { return Ok(cap) };
    match mode {
        "reset" => Ok(stack.pop().flatten()),
        "mac68k" | "packed" | "power" | "natural" => {
            stack.push(cap);
            Ok(match mode {
                "mac68k" => Some(2),
                "packed" => Some(1),
                _ => None,
            })
        }
        _ => Err(()),
    }
}

/// `text` without the conditional branches the target rules out. A condition
/// that cannot be decided keeps every branch and marks their text; an include
/// guard is no condition. Directive lines are dropped, the packing pragmas
/// after their effect is recorded.
fn select_branches(text: &str, target: Target) -> Selected {
    let mut out = Selected { text: String::with_capacity(text.len()), undecided: Vec::new(), packing: Vec::new() };
    let mut open: Vec<Conditional> = Vec::new();
    let (mut cap, mut stack, mut lost) = (None, Vec::new(), false);
    let lines: Vec<&str> = text.split('\n').collect();
    for (at, line) in lines.iter().enumerate() {
        let active = open.last().is_none_or(|c| c.read);
        let unsure = active && open.iter().any(|c| c.unsure);
        let from = out.text.len();
        let directive = line.trim_start().strip_prefix('#').map(str::trim_start);
        if directive.is_none() && active && !line.trim_start().starts_with("_Pragma(") {
            out.text.push_str(line);
        }
        out.text.push('\n');
        if let Some(directive) = directive {
            let (word, rest) = directive.split_at(directive.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(directive.len()));
            let guard = |name: &str| {
                let next = lines[at + 1..].iter().map(|l| l.trim()).find(|l| !l.is_empty()).unwrap_or("");
                let defined = next.strip_prefix('#').map(str::trim_start).and_then(|d| d.strip_prefix("define")).map(str::trim_start);
                defined.is_some_and(|d| d.strip_prefix(name).is_some_and(|after| !after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')))
            };
            let value = |rest: &str| match word {
                "ifdef" => predefined(rest.trim(), target),
                "ifndef" if guard(rest.trim()) => Some(true),
                "ifndef" => predefined(rest.trim(), target).map(|v| !v),
                _ => match lex(rest)[..] {
                    [Tok::Punct(b'!'), Tok::Ident("defined"), Tok::Group(b'(', name)] | [Tok::Punct(b'!'), Tok::Ident("defined"), Tok::Ident(name)] if guard(name.trim()) => Some(true),
                    ref toks => condition(toks, target, 0),
                },
            };
            match word {
                "if" | "ifdef" | "ifndef" => {
                    let v = value(rest);
                    open.push(Conditional { outer: active, taken: v, read: active && v != Some(false), unsure: v.is_none() });
                }
                "elif" => {
                    if let Some(c) = open.last_mut() {
                        let v = if c.taken == Some(true) { Some(false) } else { value(rest) };
                        c.read = c.outer && v != Some(false);
                        c.unsure = v != Some(false) && !(c.taken == Some(false) && v == Some(true));
                        c.taken = match (c.taken, v) {
                            (Some(true), _) | (_, Some(true)) => Some(true),
                            (Some(false), v) => v,
                            (None, _) => None,
                        };
                    }
                }
                "else" => {
                    if let Some(c) = open.last_mut() {
                        c.read = c.outer && c.taken != Some(true);
                        c.unsure = c.taken.is_none();
                    }
                }
                "endif" => {
                    open.pop();
                }
                "pragma" if unsure && (rest.trim_start().starts_with("pack") || rest.contains("align=")) => lost = true,
                "pragma" if active => match packing_pragma(rest, cap, &mut stack) {
                    Ok(now) => cap = now,
                    Err(()) => lost = true,
                },
                _ => {}
            }
        }
        let to = out.text.len();
        let still = open.last().is_none_or(|c| c.read);
        if unsure && still && open.iter().any(|c| c.unsure) {
            match out.undecided.last_mut() {
                Some(last) if last.1 == from => last.1 = to,
                _ => out.undecided.push((from, to)),
            }
        }
        if let Some(n) = if lost { Some(0) } else { cap }.filter(|_| active && still) {
            match out.packing.last_mut() {
                Some(last) if last.1 == from && last.2 == n => last.1 = to,
                _ => out.packing.push((from, to, n)),
            }
        }
    }
    out
}

fn header(raw: &str, target: Target) -> Header {
    let Selected { text: stripped, undecided, packing } = select_branches(&objcsdk::strip(raw, true), target);
    let mut text = String::with_capacity(stripped.len());
    let mut classes = Vec::new();
    let mut in_objc = false;
    let first_name = |rest: &str| -> Option<String> {
        let name: String = rest.trim_start().chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
        (!name.is_empty()).then_some(name)
    };
    for line in stripped.split('\n') {
        let t = line.trim_start();
        let mut keep = false;
        if let Some(k) = t.find("@interface").or_else(|| t.find("@implementation")) {
            classes.extend(first_name(t[k..].split_once(char::is_whitespace).map_or("", |(_, r)| r)));
            in_objc = true;
        } else if t.starts_with("@protocol") && !t.contains(';') {
            in_objc = true;
        } else if t.starts_with("@end") {
            in_objc = false;
        } else if let Some(rest) = t.strip_prefix("@class") {
            classes.extend(rest.split(',').filter_map(first_name));
        } else {
            keep = !in_objc && !t.starts_with('@');
        }
        if keep {
            text.push_str(line);
        } else {
            text.extend(std::iter::repeat_n(' ', line.len()));
        }
        text.push('\n');
    }
    Header { text, classes, undecided, packing }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tok<'a> {
    Ident(&'a str),
    Num(&'a str),
    Chars(&'a str),
    /// A balanced `( )`, `[ ]` or `{ }` with its inner text.
    Group(u8, &'a str),
    Punct(u8),
}

/// The index of the bracket closing the one at `b[open]`, or `b.len()`.
fn close(b: &[u8], open: usize) -> usize {
    let mut depth = 0usize;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return i;
                }
            }
            q @ (b'"' | b'\'') => i = quote_end(b, i, q),
            _ => {}
        }
        i += 1;
    }
    b.len()
}

/// The index of the quote closing the literal opened at `b[open]`, or `b.len()`.
fn quote_end(b: &[u8], open: usize, quote: u8) -> usize {
    let mut i = open + 1;
    while i < b.len() && b[i] != quote && b[i] != b'\n' {
        if b[i] == b'\\' {
            i += 1;
        }
        i += 1;
    }
    i.min(b.len())
}

/// The top-level tokens of `text`; a bracketed group is one token.
fn lex(text: &str) -> Vec<Tok<'_>> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || !c.is_ascii();
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            let s = i;
            while i < b.len() && (word(b[i]) || b[i] == b'.') {
                i += 1;
            }
            out.push(Tok::Num(&text[s..i]));
        } else if word(c) {
            let s = i;
            while i < b.len() && word(b[i]) {
                i += 1;
            }
            out.push(Tok::Ident(&text[s..i]));
        } else if c == b'"' || c == b'\'' {
            let e = quote_end(b, i, c);
            out.push(if c == b'\'' { Tok::Chars(text.get(i + 1..e).unwrap_or("")) } else { Tok::Punct(b'"') });
            i = (e + 1).min(b.len());
        } else if matches!(c, b'(' | b'[' | b'{') {
            let e = close(b, i);
            out.push(Tok::Group(c, text.get(i + 1..e).unwrap_or("")));
            i = (e + 1).min(b.len());
        } else {
            out.push(Tok::Punct(c));
            i += 1;
        }
    }
    out
}

/// The declarations of `text`, split at `;` outside brackets. A linkage block
/// (`extern "C" { }`) is transparent and an inline function body is dropped
/// with its head.
fn statements(text: &str) -> Vec<&str> {
    statements_at(text).into_iter().map(|(_, stmt)| stmt).collect()
}

/// [`statements`], each with its byte offset in `text`.
fn statements_at(text: &str) -> Vec<(usize, &str)> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let (mut start, mut paren, mut brace, mut body) = (0usize, 0usize, 0usize, false);
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            q @ (b'"' | b'\'') => i = quote_end(b, i, q),
            b'(' => paren += 1,
            b')' => paren = paren.saturating_sub(1),
            b'{' if brace == 0 && paren == 0 => {
                let head = text[start..i].trim_end();
                if head.ends_with("\"C\"") || head.ends_with("\"C++\"") {
                    start = i + 1;
                } else {
                    body = is_function_head(head);
                    brace = 1;
                }
            }
            b'{' => brace += 1,
            b'}' if brace == 0 => start = i + 1,
            b'}' => {
                brace -= 1;
                if brace == 0 && paren == 0 && body {
                    start = i + 1;
                    body = false;
                }
            }
            b';' if brace == 0 && paren == 0 => {
                out.push((start, &text[start..i]));
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// Whether the text before a top-level `{` is a function's declarator rather
/// than a record, an enum or an enum macro.
fn is_function_head(head: &str) -> bool {
    if !head.ends_with(')') {
        return false;
    }
    let toks = lex(head);
    if toks.iter().any(|t| matches!(t, Tok::Ident("typedef" | "enum"))) {
        return false;
    }
    !matches!(toks[..], [.., Tok::Ident(m), Tok::Group(b'(', _)] if is_enum_macro(m))
}

fn is_enum_macro(name: &str) -> bool {
    name.ends_with("ENUM") || name.ends_with("OPTIONS")
}

/// The macros that declare `name_t` as an opaque object pointer:
/// `DISPATCH_DECL`, `OS_OBJECT_DECL_SUBCLASS`, `NW_OBJECT_DECL`.
fn is_object_macro(name: &str) -> bool {
    macro_shape(name) && !name.contains("TYPE_DECL") && !name.contains("PROTO") && (name.ends_with("_DECL") || name.contains("_DECL_"))
}

/// An attribute-macro spelling: all capitals with an underscore.
fn macro_shape(t: &str) -> bool {
    t.len() > 2 && t.contains('_') && t.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
}

fn is_attribute(t: &str) -> bool {
    matches!(t, "__attribute__" | "__attribute" | "__asm" | "__asm__" | "asm" | "__declspec" | "_Alignas" | "__alignas")
}

fn is_qualifier(t: &str) -> bool {
    matches!(
        t,
        "const" | "volatile" | "restrict" | "__restrict" | "__restrict__" | "__const" | "__volatile" | "extern" | "static" | "inline" | "__inline"
            | "__inline__" | "register" | "typedef" | "_Nullable" | "_Nonnull" | "_Null_unspecified" | "_Nullable_result" | "__nullable" | "__nonnull"
            | "__null_unspecified" | "nullable" | "nonnull" | "null_unspecified" | "__unused" | "__kindof" | "__strong" | "__weak"
            | "__unsafe_unretained" | "__autoreleasing" | "__block" | "_Atomic" | "__signed" | "__extension__" | "_Noreturn"
    )
}

fn is_keyword(t: &str) -> bool {
    matches!(t, "void" | "char" | "short" | "int" | "long" | "signed" | "unsigned" | "float" | "double" | "_Bool" | "bool")
}

/// A resolved C type. Records and enums are named; their bodies stay in the
/// [`Index`] until a prototype needs them.
#[derive(Clone, PartialEq, Debug)]
enum Ty {
    Void,
    Int(u8, bool),
    Char,
    Bool,
    Float(u8),
    Ptr(Box<Ty>),
    /// Element and count; a count of zero is an unsized array.
    Array(Box<Ty>, u32),
    Record(String),
    /// Name (empty when anonymous), width and signedness.
    Enum(String, u8, bool),
    /// A function type, meaningful only behind a pointer.
    Func,
    /// An Objective-C class or runtime object, meaningful only behind a pointer.
    Object(String),
    /// A name no header resolves, meaningful only behind a pointer.
    Unknown,
}

/// Where a `typedef` name comes from.
enum Def<'a> {
    /// Declarator `part` (comma-separated) of the statement.
    Stmt(&'a str, usize),
    /// An enum macro's underlying type text.
    Enum(&'a str),
    /// An object-declaring macro: a pointer to the named opaque record.
    Object(String),
    /// A definition with an undecided conditional inside it.
    Unreadable,
}

struct RecordDef<'a> {
    union: bool,
    body: &'a str,
    /// The alignment cap a packing pragma puts on the members; zero when the
    /// body is not a layout the reader models (a packing attribute, a pragma
    /// it does not follow, or an undecided conditional inside the body).
    cap: u8,
    /// The definition sits under an undecided conditional: it may agree with
    /// a plain one but gives no layout alone.
    conditional: bool,
}

/// The agreed shape of a record.
#[derive(Clone, PartialEq)]
struct Shape {
    union: bool,
    cap: u8,
    members: Vec<(String, Ty)>,
}

/// How a statement stands to the conditionals that could not be decided.
#[derive(Clone, Copy, PartialEq)]
enum Standing {
    /// Under none: it is part of every configuration.
    Plain,
    /// Wholly under one: it may not apply.
    Conditional,
    /// One ends inside it: its text is not one declaration.
    Divided,
}

struct EnumDef<'a> {
    base: Option<&'a str>,
    /// The enumerator list, when the definition is at hand.
    body: Option<&'a str>,
}

/// A parsed function declaration.
#[derive(Clone, PartialEq, Debug)]
struct Sig {
    ret: Ty,
    params: Vec<Ty>,
    vararg: bool,
}

const MAX_DEPTH: usize = 24;

/// A base-type text that resolves to nothing.
const UNREADABLE: &str = "$";

/// Every definition the headers hold, by name, resolved on demand.
struct Index<'a> {
    typedefs: HashMap<String, Vec<Def<'a>>>,
    records: HashMap<&'a str, Vec<RecordDef<'a>>>,
    enums: HashMap<String, Vec<EnumDef<'a>>>,
    classes: HashSet<&'a str>,
    /// The statements declaring each wanted function, each with the offset
    /// its last undecided conditional ends at (zero when none ends inside it).
    functions: HashMap<&'a str, Vec<(&'a str, usize)>>,
    ptr: u8,
    machine: object::Architecture,
    resolved: RefCell<HashMap<String, Option<Ty>>>,
}

impl<'a> Index<'a> {
    fn new(headers: &'a [Header], wanted: &BTreeSet<&str>, target: Target) -> Index<'a> {
        let mut index = Index {
            typedefs: HashMap::new(),
            records: HashMap::new(),
            enums: HashMap::new(),
            classes: HashSet::new(),
            functions: HashMap::new(),
            ptr: target.ptr_size.clamp(1, 8) as u8,
            machine: target.machine,
            resolved: RefCell::new(HashMap::new()),
        };
        for h in headers {
            index.classes.extend(h.classes.iter().map(String::as_str));
            for (at, stmt) in statements_at(&h.text) {
                let from = at + stmt.len() - stmt.trim_start().len();
                let to = at + stmt.trim_end().len();
                let standing = if h.undecided.iter().any(|r| from < r.1 && r.1 < to) {
                    Standing::Divided
                } else if h.undecided.iter().any(|r| r.0 < to && from < r.1) {
                    Standing::Conditional
                } else {
                    Standing::Plain
                };
                let mut cap = 8;
                for &(start, end, n) in h.packing.iter().filter(|r| r.0 < to && from < r.1) {
                    cap = if start <= from && to <= end { n } else { 0 };
                }
                let settled = h.undecided.iter().filter(|r| from < r.1 && r.1 < to).map(|r| r.1 - at).max().unwrap_or(0);
                index.scan(stmt, wanted, cap, standing, settled);
            }
        }
        index
    }

    /// Record what one statement defines, without resolving anything.
    fn scan(&mut self, stmt: &'a str, wanted: &BTreeSet<&str>, cap: u8, standing: Standing, settled: usize) {
        let toks = lex(stmt);
        if self.scan_macro(&toks, standing) {
            return;
        }
        let typedef = toks.iter().position(|t| *t == Tok::Ident("typedef"));
        let mut anon = None;
        for (k, part) in typedef.iter().flat_map(|at| toks[at + 1..].split(|t| *t == Tok::Punct(b','))).enumerate() {
            if let Some(name) = declarator_name(part) {
                anon = anon.or(Some(name));
                let def = if standing == Standing::Divided { Def::Unreadable } else { Def::Stmt(stmt, k) };
                self.typedefs.entry(name.to_string()).or_default().push(def);
            }
        }
        let cap = if standing == Standing::Divided || stmt.contains("packed") || stmt.contains("aligned") { 0 } else { cap };
        self.scan_aggregates(stmt, &toks, anon, cap, standing);
        if typedef.is_some() {
            return;
        }
        for pair in toks.windows(2) {
            if let [Tok::Ident(name), Tok::Group(b'(', _)] = pair {
                if wanted.contains(name) {
                    self.functions.entry(name).or_default().push((stmt, settled));
                }
            }
        }
    }

    /// The statements that are one macro: `[typedef] NS_ENUM(Base, Name) { … }`
    /// defines `Name`, `OS_ENUM(name, Base, …)` defines `name_t`, and
    /// `DISPATCH_DECL(name)` defines `name_t` as a pointer to `name_s`.
    fn scan_macro(&mut self, toks: &[Tok<'a>], standing: Standing) -> bool {
        let simple = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
        let defined = toks.windows(3).find_map(|w| match *w {
            [Tok::Ident(m), Tok::Group(b'(', args), Tok::Group(b'{', body)] if is_enum_macro(m) => Some((m, args, body)),
            _ => None,
        });
        let (name, def, body) = if let Some((m, args, body)) = defined {
            let args: Vec<&str> = args.split(',').map(str::trim).collect();
            let [base, name] = args[..] else { return true };
            let base = if m.ends_with("ERROR_ENUM") { "NSInteger" } else { base };
            (name.to_string(), Def::Enum(base), Some(body))
        } else {
            match *toks {
                [.., Tok::Ident(m), Tok::Group(b'(', args)] if is_enum_macro(m) && !toks.contains(&Tok::Ident("typedef")) => {
                    let mut head = args.splitn(3, ',').map(str::trim);
                    let (Some(name), Some(base)) = (head.next().filter(|n| simple(n)), head.next()) else { return true };
                    (format!("{name}_t"), Def::Enum(base), head.next())
                }
                [.., Tok::Ident(m), Tok::Group(b'(', args)] if is_object_macro(m) => {
                    let Some(name) = args.split(',').next().map(str::trim).filter(|n| simple(n)) else { return true };
                    (format!("{name}_t"), Def::Object(format!("{name}_s")), None)
                }
                _ => return false,
            }
        };
        if standing == Standing::Divided {
            self.typedefs.entry(name).or_default().push(Def::Unreadable);
            return true;
        }
        if let Def::Enum(base) = def {
            let body = body.filter(|_| standing == Standing::Plain);
            self.enums.entry(name.clone()).or_default().push(EnumDef { base: Some(base), body });
        }
        self.typedefs.entry(name).or_default().push(def);
        true
    }

    /// Every `struct` / `union` / `enum` body of a statement, under its tag or,
    /// untagged in a `typedef`, under the first name that declares. Only a
    /// plain statement gives enumerator names.
    fn scan_aggregates(&mut self, stmt: &'a str, toks: &[Tok<'a>], anon: Option<&'a str>, cap: u8, standing: Standing) {
        let mut i = 0;
        while i < toks.len() {
            let Tok::Ident(kw @ ("struct" | "union" | "enum")) = toks[i] else {
                i += 1;
                continue;
            };
            i += 1;
            skip_noise(toks, &mut i);
            let mut tag = None;
            if let Some(Tok::Ident(t)) = toks.get(i) {
                tag = Some(*t);
                i += 1;
            }
            let base_from = i;
            while kw == "enum" && toks.get(i).is_some_and(|t| !matches!(t, Tok::Group(b'{', _) | Tok::Punct(b','))) {
                i += 1;
            }
            let base = (i > base_from + 1 && toks[base_from] == Tok::Punct(b':')).then(|| span(stmt, &toks[base_from + 1..i])).flatten();
            let body = if let Some(Tok::Group(b'{', body)) = toks.get(i) { Some(*body) } else { None };
            let Some(name) = tag.or(anon) else { continue };
            if kw != "enum" {
                if let Some(body) = body {
                    self.records.entry(name).or_default().push(RecordDef { union: kw == "union", body, cap, conditional: standing != Standing::Plain });
                }
            } else if standing == Standing::Divided {
                self.enums.entry(name.to_string()).or_default().push(EnumDef { base: Some(UNREADABLE), body: None });
            } else if body.is_some() || base.is_some() {
                self.enums.entry(name.to_string()).or_default().push(EnumDef { base, body: body.filter(|_| standing == Standing::Plain) });
            }
        }
    }

    fn known_type(&self, name: &str) -> bool {
        is_keyword(name) || builtin(name, self.ptr, self.machine).is_some() || self.typedefs.contains_key(name) || self.classes.contains(name)
    }

    fn is_noise(&self, name: &str) -> bool {
        !self.known_type(name) && (macro_shape(name) || name.starts_with("__") || is_attribute(name))
    }

    /// Step over attribute macros, each with its argument list.
    fn skip_noise(&self, t: &[Tok], i: &mut usize) {
        loop {
            match t.get(*i) {
                Some(Tok::Ident(n)) if is_attribute(n) || self.is_noise(n) => {
                    *i += 1;
                    if matches!(t.get(*i), Some(Tok::Group(b'(', _))) {
                        *i += 1;
                    }
                }
                Some(Tok::Ident(n)) if is_qualifier(n) => *i += 1,
                Some(Tok::Punct(b'"')) => *i += 1,
                _ => return,
            }
        }
    }

    /// The type a `typedef` name stands for, `Ty::Unknown` when the headers
    /// define it two ways or not readably.
    fn typedef(&self, name: &str, depth: usize) -> Ty {
        if let Some(done) = self.resolved.borrow().get(name) {
            return done.clone().unwrap_or(Ty::Unknown);
        }
        if depth > MAX_DEPTH {
            return Ty::Unknown;
        }
        self.resolved.borrow_mut().insert(name.to_string(), None);
        let mut found: Option<Ty> = None;
        let mut agreed = true;
        for def in self.typedefs.get(name).map(Vec::as_slice).unwrap_or(&[]) {
            let ty = match def {
                Def::Object(record) => Some(Ty::Ptr(Box::new(Ty::Record(record.clone())))),
                Def::Enum(base) => match self.type_of(base, depth + 1) {
                    Some(Ty::Int(size, signed)) => Some(Ty::Enum(name.to_string(), size, signed)),
                    Some(Ty::Enum(_, size, signed)) => Some(Ty::Enum(name.to_string(), size, signed)),
                    Some(Ty::Char) => Some(Ty::Enum(name.to_string(), 1, true)),
                    _ => None,
                },
                Def::Stmt(stmt, part) => self.typedef_part(stmt, *part, name, depth + 1),
                Def::Unreadable => None,
            };
            match (&found, ty) {
                (_, None) => agreed = false,
                (None, Some(ty)) => found = Some(ty),
                (Some(Ty::Ptr(have)), Some(Ty::Ptr(ty))) if **have != *ty => found = Some(Ty::Ptr(Box::new(Ty::Unknown))),
                (Some(Ty::Int(size, signed)), Some(Ty::Enum(name, s, g))) if (*size, *signed) == (s, g) => found = Some(Ty::Enum(name, s, g)),
                (Some(Ty::Enum(_, size, signed)), Some(Ty::Int(s, g))) if (*size, *signed) == (s, g) => {}
                (Some(have), Some(ty)) => agreed &= *have == ty,
            }
        }
        let result = found.filter(|ty| agreed && *ty != Ty::Unknown);
        self.resolved.borrow_mut().insert(name.to_string(), result.clone());
        result.unwrap_or(Ty::Unknown)
    }

    /// Declarator `part` of a `typedef` statement, which must declare `name`.
    fn typedef_part(&self, stmt: &str, part: usize, name: &str, depth: usize) -> Option<Ty> {
        let toks = lex(stmt);
        let at = toks.iter().position(|t| *t == Tok::Ident("typedef"))?;
        let rest = &toks[at + 1..];
        let mut parts = rest.split(|t| *t == Tok::Punct(b','));
        let first = parts.next()?;
        let anon = declarator_name(first);
        if part == 0 && (stmt.contains("transparent_union") || stmt.contains("TRANSPARENT_UNION")) {
            let body = first.iter().find_map(|t| if let Tok::Group(b'{', body) = t { Some(*body) } else { None })?;
            return self.members(body, depth + 1)?.into_iter().next().map(|(_, ty)| ty);
        }
        let stop = if part == 0 { Some(name) } else { anon };
        let mut i = 0;
        let base = self.specifiers(first, &mut i, stop, anon, depth)?;
        let decl = if part == 0 { &first[i..] } else { parts.nth(part - 1)? };
        let mut j = 0;
        let (ty, declared) = self.declarator(base, decl, &mut j, depth)?;
        (j == decl.len() && declared == Some(name)).then_some(ty)
    }

    /// The type a text spells, with no declarator name (`const char *`).
    fn type_of(&self, text: &str, depth: usize) -> Option<Ty> {
        let toks = lex(text);
        self.abstract_type(&toks, depth)
    }

    fn abstract_type(&self, toks: &[Tok], depth: usize) -> Option<Ty> {
        let mut i = 0;
        let base = self.specifiers(toks, &mut i, None, None, depth)?;
        let (ty, name) = self.declarator(base, toks, &mut i, depth)?;
        (i == toks.len() && name.is_none_or(|n| !self.known_type(n))).then_some(ty)
    }

    /// The declaration specifiers at `t[*i..]`: qualifiers, attribute macros
    /// and one base type. Stops at the declarator (`stop` when its name is
    /// known ahead, else the first name after a complete base).
    fn specifiers(&self, t: &[Tok], i: &mut usize, stop: Option<&str>, anon: Option<&str>, depth: usize) -> Option<Ty> {
        if depth > MAX_DEPTH {
            return None;
        }
        let mut words: Vec<&str> = Vec::new();
        let mut base: Option<Ty> = None;
        while let Some(tok) = t.get(*i) {
            match *tok {
                Tok::Punct(b'"') => *i += 1,
                Tok::Punct(b'<') if base.is_some() => {
                    let mut open = 0usize;
                    while let Some(tok) = t.get(*i) {
                        *i += 1;
                        match tok {
                            Tok::Punct(b'<') => open += 1,
                            Tok::Punct(b'>') => open -= 1,
                            _ => {}
                        }
                        if open == 0 {
                            break;
                        }
                    }
                }
                Tok::Ident(w) if Some(w) == stop => break,
                Tok::Ident(w) if is_qualifier(w) => *i += 1,
                Tok::Ident(w) if is_attribute(w) => self.skip_noise(t, i),
                Tok::Ident(w) if is_keyword(w) => {
                    if base.is_some() {
                        return None;
                    }
                    words.push(w);
                    *i += 1;
                }
                Tok::Ident("struct" | "union" | "enum") => {
                    if base.is_some() || !words.is_empty() {
                        return None;
                    }
                    base = Some(self.tagged(t, i, stop, anon, depth)?);
                }
                Tok::Ident(w) if base.is_some() || !words.is_empty() => {
                    if !self.is_noise(w) {
                        break;
                    }
                    self.skip_noise(t, i);
                }
                Tok::Ident(w) => {
                    if let Some(ty) = self.named(w, depth) {
                        base = Some(ty);
                        *i += 1;
                    } else if self.is_noise(w) {
                        self.skip_noise(t, i);
                    } else {
                        base = Some(Ty::Unknown);
                        *i += 1;
                    }
                }
                _ => break,
            }
        }
        base.or_else(|| keyword_type(&words, self.ptr))
    }

    /// `struct Tag`, `union Tag`, `enum Tag : Base`, each with an optional body.
    fn tagged(&self, t: &[Tok], i: &mut usize, stop: Option<&str>, anon: Option<&str>, depth: usize) -> Option<Ty> {
        let Some(Tok::Ident(kw)) = t.get(*i).copied() else { return None };
        *i += 1;
        skip_noise(t, i);
        let mut tag = None;
        if let Some(Tok::Ident(name)) = t.get(*i).copied() {
            if Some(name) != stop || t[*i + 1..].contains(&Tok::Ident(name)) {
                tag = Some(name);
                *i += 1;
            }
        }
        if kw != "enum" {
            let body = matches!(t.get(*i), Some(Tok::Group(b'{', _)));
            *i += usize::from(body);
            return Some(Ty::Record(tag.or(anon.filter(|_| body))?.to_string()));
        }
        let mut base = None;
        if t.get(*i) == Some(&Tok::Punct(b':')) {
            let from = *i + 1;
            while t.get(*i).is_some_and(|t| !matches!(t, Tok::Group(b'{', _))) && t.get(*i).copied() != stop.map(Tok::Ident) {
                *i += 1;
            }
            base = Some(self.abstract_type(&t[from..*i], depth + 1)?);
        }
        let body = matches!(t.get(*i), Some(Tok::Group(b'{', _)));
        *i += usize::from(body);
        let name = tag.or(anon.filter(|_| body)).unwrap_or("");
        if base.is_none() {
            let mut bases = self.enums.get(name).into_iter().flatten().filter_map(|d| d.base);
            if let Some(first) = bases.next() {
                let ty = self.type_of(first, depth + 1)?;
                if bases.any(|b| self.type_of(b, depth + 1).as_ref() != Some(&ty)) {
                    return None;
                }
                base = Some(ty);
            }
        }
        match base {
            None => Some(Ty::Enum(name.to_string(), 4, true)),
            Some(Ty::Int(size, signed)) | Some(Ty::Enum(_, size, signed)) => Some(Ty::Enum(name.to_string(), size, signed)),
            Some(Ty::Char) => Some(Ty::Enum(name.to_string(), 1, true)),
            Some(Ty::Bool) => Some(Ty::Enum(name.to_string(), 1, false)),
            Some(_) => None,
        }
    }

    /// The type a single identifier names, `None` when it names none.
    fn named(&self, w: &str, depth: usize) -> Option<Ty> {
        if let Some(ty) = builtin(w, self.ptr, self.machine) {
            return Some(ty);
        }
        if self.typedefs.contains_key(w) {
            return Some(self.typedef(w, depth + 1));
        }
        self.classes.contains(w).then(|| Ty::Object(w.to_string()))
    }

    /// A declarator after its specifiers: pointers, an optional name, array
    /// and function suffixes. A bitfield is refused.
    fn declarator<'t>(&self, base: Ty, t: &[Tok<'t>], i: &mut usize, depth: usize) -> Option<(Ty, Option<&'t str>)> {
        let mut ty = base;
        loop {
            self.skip_noise(t, i);
            if t.get(*i) != Some(&Tok::Punct(b'*')) {
                break;
            }
            ty = Ty::Ptr(Box::new(ty));
            *i += 1;
        }
        let mut name = None;
        while let (Some(Tok::Ident(_)), Some(Tok::Ident(next))) = (t.get(*i), t.get(*i + 1)) {
            if self.is_noise(next) {
                break;
            }
            *i += 1;
        }
        match t.get(*i).copied() {
            Some(Tok::Ident(n)) => {
                name = Some(n);
                *i += 1;
            }
            Some(Tok::Group(b'(', inner)) => {
                let inner = lex(inner);
                let mut k = 0;
                self.skip_noise(&inner, &mut k);
                if !matches!(inner.get(k), Some(Tok::Punct(b'*' | b'^'))) || inner.iter().any(|t| matches!(t, Tok::Group(b'[', _))) {
                    return None;
                }
                k += 1;
                self.skip_noise(&inner, &mut k);
                if let Some(Tok::Ident(n)) = inner.get(k).copied() {
                    name = Some(n);
                    k += 1;
                }
                self.skip_noise(&inner, &mut k);
                if k != inner.len() {
                    return None;
                }
                *i += 1;
                if matches!(t.get(*i), Some(Tok::Group(b'(', _))) {
                    *i += 1;
                    ty = Ty::Ptr(Box::new(Ty::Func));
                } else {
                    ty = Ty::Ptr(Box::new(ty));
                }
            }
            _ => {}
        }
        let mut dims = Vec::new();
        loop {
            self.skip_noise(t, i);
            match t.get(*i).copied() {
                Some(Tok::Group(b'[', inner)) => {
                    let size: Vec<Tok> = lex(inner).into_iter().filter(|t| !matches!(t, Tok::Ident(q) if is_qualifier(q))).collect();
                    let count = if size.is_empty() { Some(0) } else { eval(&size, &BTreeMap::new(), depth + 1) };
                    dims.push(u32::try_from(count?).ok()?);
                    *i += 1;
                }
                Some(Tok::Group(b'(', _)) if name.is_some() && dims.is_empty() => {
                    ty = Ty::Func;
                    *i += 1;
                }
                Some(Tok::Punct(b':')) => return None,
                _ => break,
            }
        }
        for count in dims.into_iter().rev() {
            ty = Ty::Array(Box::new(ty), count);
        }
        Some((ty, name))
    }

    /// The members of a record body, in order. `None` when one is not a plain
    /// named member of a resolved type.
    fn members(&self, body: &str, depth: usize) -> Option<Vec<(String, Ty)>> {
        let mut out = Vec::new();
        for stmt in statements(body) {
            let toks = lex(stmt);
            let mut i = 0;
            let base = self.specifiers(&toks, &mut i, None, None, depth + 1)?;
            for part in toks[i..].split(|t| *t == Tok::Punct(b',')) {
                let mut j = 0;
                let (ty, name) = self.declarator(base.clone(), part, &mut j, depth + 1)?;
                if j != part.len() || matches!(ty, Ty::Func | Ty::Void | Ty::Unknown | Ty::Object(_) | Ty::Array(_, 0)) {
                    return None;
                }
                out.push((name?.to_string(), ty));
            }
        }
        (!out.is_empty()).then_some(out)
    }

    /// The one shape the headers give record `name`.
    fn record(&self, name: &str, depth: usize) -> Option<Shape> {
        if depth > MAX_DEPTH || self.ptr != 8 {
            return None;
        }
        let mut found: Option<Shape> = None;
        let defs = self.records.get(name)?;
        if defs.iter().all(|def| def.conditional) {
            return None;
        }
        for def in defs {
            if def.cap == 0 {
                return None;
            }
            let this = Shape { union: def.union, cap: def.cap, members: self.members(def.body, depth + 1)? };
            if found.as_ref().is_some_and(|have| *have != this) {
                return None;
            }
            found = Some(this);
        }
        found
    }

    /// The named values of enum `name` at `size` bytes, when every definition
    /// of it agrees.
    fn enumerators(&self, name: &str, size: u8) -> Option<BTreeMap<u64, String>> {
        let mask = if size >= 8 { u64::MAX } else { (1u64 << (size * 8)) - 1 };
        let mut found: Option<BTreeMap<u64, String>> = None;
        for def in self.enums.get(name)? {
            let Some(body) = def.body else { continue };
            let mut known: BTreeMap<&str, i128> = BTreeMap::new();
            let mut values = BTreeMap::new();
            let mut next: Option<i128> = Some(0);
            for item in lex(body).split(|t| *t == Tok::Punct(b',')) {
                let Some(Tok::Ident(label)) = item.first().copied() else { continue };
                let value = match item.iter().position(|t| *t == Tok::Punct(b'=')) {
                    Some(eq) => eval(&item[eq + 1..], &known, 0),
                    None => next,
                };
                next = value.map(|v| v + 1);
                if let Some(v) = value {
                    known.insert(label, v);
                    values.entry((v as u64) & mask).or_insert_with(|| label.to_string());
                }
            }
            if found.as_ref().is_some_and(|have| *have != values) {
                return None;
            }
            found = Some(values);
        }
        found.filter(|v| !v.is_empty())
    }

    /// The agreed declaration of imported function `name`.
    fn signature(&self, name: &str) -> Option<Sig> {
        let mut found: Option<Sig> = None;
        for &(stmt, settled) in self.functions.get(name)? {
            let toks = lex(stmt);
            let at = toks.windows(2).position(|w| matches!(w, [Tok::Ident(n), Tok::Group(b'(', _)] if *n == name))?;
            let Tok::Group(_, params) = toks[at + 1] else { return None };
            let from = toks[..at].iter().rposition(|t| matches!(t, Tok::Group(b'{', _))).map_or(0, |k| k + 1);
            let typed = |t: &Tok| matches!(t, Tok::Ident(w) if self.known_type(w) || matches!(*w, "struct" | "union" | "enum" | "const"));
            let from = toks[from..at].iter().position(typed).map_or(from, |k| from + k);
            if settled > 0 && !matches!(toks[from], Tok::Ident(first) if offset_in(stmt, first).is_some_and(|k| settled <= k)) {
                return None;
            }
            let ret = self.abstract_type(&toks[from..at], 0)?;
            let ret = match ret {
                Ty::Func | Ty::Unknown | Ty::Object(_) | Ty::Array(..) => return None,
                ty => ty,
            };
            let sig = self.parameters(params, ret)?;
            if found.as_ref().is_some_and(|have| *have != sig) {
                return None;
            }
            found = Some(sig);
        }
        found
    }

    fn parameters(&self, text: &str, ret: Ty) -> Option<Sig> {
        let toks = lex(text);
        let mut sig = Sig { ret, params: Vec::new(), vararg: false };
        if toks.is_empty() {
            return None;
        }
        if toks == [Tok::Ident("void")] {
            return Some(sig);
        }
        let mut angles = 0usize;
        let top_comma = |t: &Tok| {
            match t {
                Tok::Punct(b'<') => angles += 1,
                Tok::Punct(b'>') => angles = angles.saturating_sub(1),
                _ => {}
            }
            angles == 0 && *t == Tok::Punct(b',')
        };
        for part in toks.split(top_comma) {
            if sig.vararg {
                return None;
            }
            if part == [Tok::Punct(b'.'); 3] {
                sig.vararg = true;
                continue;
            }
            let mut i = 0;
            let base = self.specifiers(part, &mut i, None, None, 0)?;
            let (ty, _) = self.declarator(base, part, &mut i, 0)?;
            if i != part.len() {
                return None;
            }
            sig.params.push(match ty {
                Ty::Array(elem, _) => Ty::Ptr(elem),
                Ty::Func => Ty::Ptr(Box::new(Ty::Func)),
                Ty::Void | Ty::Unknown | Ty::Object(_) => return None,
                ty => ty,
            });
        }
        Some(sig)
    }
}

/// Step over attribute spellings without consulting any typedef.
fn skip_noise(t: &[Tok], i: &mut usize) {
    while let Some(Tok::Ident(n)) = t.get(*i) {
        let group = matches!(t.get(*i + 1), Some(Tok::Group(b'(', _)));
        if is_attribute(n) || (macro_shape(n) && (group || matches!(t.get(*i + 1), Some(Tok::Ident(_))))) {
            *i += 1 + usize::from(group);
        } else {
            return;
        }
    }
}

/// The byte offset of `inner` in `outer`, which it is a slice of.
fn offset_in(outer: &str, inner: &str) -> Option<usize> {
    let at = (inner.as_ptr() as usize).checked_sub(outer.as_ptr() as usize)?;
    (at + inner.len() <= outer.len()).then_some(at)
}

/// The source text `toks` cover in `stmt`, which they were lexed from.
fn span<'a>(stmt: &'a str, toks: &[Tok<'a>]) -> Option<&'a str> {
    let text = |t: &Tok<'a>| match *t {
        Tok::Ident(s) | Tok::Num(s) | Tok::Chars(s) | Tok::Group(_, s) => Some(s),
        Tok::Punct(_) => None,
    };
    let first = toks.iter().find_map(text)?;
    let last = toks.iter().rev().find_map(text)?;
    stmt.get(offset_in(stmt, first)?..offset_in(stmt, last)? + last.len())
}

/// The name one `typedef` declarator declares, read from its end.
fn declarator_name<'a>(part: &[Tok<'a>]) -> Option<&'a str> {
    let mut j = part.len();
    loop {
        match part[..j] {
            [.., Tok::Group(b'[', _)] => j -= 1,
            [.., Tok::Ident(m), Tok::Group(b'(', _)] if macro_shape(m) || is_attribute(m) => j -= 2,
            [.., Tok::Group(b'(' | b'[', _), Tok::Ident(m)] if macro_shape(m) || m.starts_with("__") => j -= 1,
            [.., Tok::Ident("struct" | "union" | "enum"), Tok::Ident(_), Tok::Ident(_)] => break,
            [.., Tok::Ident(prev), Tok::Ident(m)] if (macro_shape(m) || m.starts_with("__")) && !is_keyword(prev) && !is_qualifier(prev) => {
                if part[..j - 1].iter().filter(|t| !matches!(t, Tok::Ident(q) if is_qualifier(q))).count() < 2 {
                    break;
                }
                j -= 1;
            }
            _ => break,
        }
    }
    let inner_name = |inner: &'a str| {
        let toks = lex(inner);
        matches!(toks.first(), Some(Tok::Punct(b'*' | b'^')))
            .then(|| toks.iter().rev().find_map(|t| if let Tok::Ident(n) = t { Some(*n) } else { None }).filter(|n| !is_qualifier(n)))
            .flatten()
    };
    match part[..j] {
        [.., Tok::Group(b'(', inner), Tok::Group(b'(', _)] | [.., Tok::Group(b'(', inner)] if inner.trim_start().starts_with(['*', '^']) => inner_name(inner),
        [.., Tok::Ident(n), Tok::Group(b'(', _)] | [.., Tok::Ident(n)] if !is_keyword(n) && !is_qualifier(n) => Some(n),
        _ => None,
    }
}

/// The type the C keywords of a declaration spell (`unsigned long`).
fn keyword_type(words: &[&str], ptr: u8) -> Option<Ty> {
    let count = |w: &str| words.iter().filter(|x| **x == w).count();
    let unsigned = count("unsigned") > 0;
    let longs = count("long");
    if words.is_empty() || (unsigned && count("signed") > 0) {
        return None;
    }
    let sized = unsigned || count("signed") > 0 || longs > 0 || count("short") > 0;
    Some(if count("void") > 0 {
        (words.len() == 1).then_some(Ty::Void)?
    } else if count("_Bool") + count("bool") > 0 {
        (words.len() == 1).then_some(Ty::Bool)?
    } else if count("float") > 0 {
        (words.len() == 1).then_some(Ty::Float(4))?
    } else if count("double") > 0 {
        (words.len() == 1).then_some(Ty::Float(8))?
    } else if count("char") > 0 {
        match (longs + count("short"), sized) {
            (0, false) => Ty::Char,
            (0, true) => Ty::Int(1, !unsigned),
            _ => return None,
        }
    } else if count("short") > 0 {
        (longs == 0).then_some(Ty::Int(2, !unsigned))?
    } else {
        match longs {
            0 => Ty::Int(4, !unsigned),
            1 => Ty::Int(ptr, !unsigned),
            2 => Ty::Int(8, !unsigned),
            _ => return None,
        }
    })
}

/// The names every Apple header relies on without a readable definition: the
/// fixed-width and pointer-width integers, the Objective-C runtime handles and
/// the variable-argument list.
fn builtin(name: &str, ptr: u8, machine: object::Architecture) -> Option<Ty> {
    let object = |record: &str| Ty::Ptr(Box::new(Ty::Object(record.to_string())));
    Some(match name {
        "id" | "instancetype" => object("objc_object"),
        "Class" => object("objc_class"),
        "SEL" => object("objc_selector"),
        "IMP" => Ty::Ptr(Box::new(Ty::Func)),
        "va_list" | "__builtin_va_list" | "__darwin_va_list" | "__gnuc_va_list" => match machine {
            object::Architecture::X86_64 => Ty::Array(Box::new(Ty::Int(8, false)), 3),
            _ => Ty::Ptr(Box::new(Ty::Char)),
        },
        "int64_t" | "SInt64" | "__int64_t" => Ty::Int(8, true),
        "uint64_t" | "UInt64" | "__uint64_t" => Ty::Int(8, false),
        "CGFloat" => Ty::Float(ptr),
        _ => match objcsdk::scalar(name)? {
            "v" => Ty::Void,
            "B" => Ty::Bool,
            "c" => Ty::Int(1, true),
            "C" => Ty::Int(1, false),
            "s" => Ty::Int(2, true),
            "S" => Ty::Int(2, false),
            "i" => Ty::Int(4, true),
            "I" => Ty::Int(4, false),
            "q" => Ty::Int(ptr, true),
            "Q" => Ty::Int(ptr, false),
            "f" => Ty::Float(4),
            "d" => Ty::Float(8),
            _ => return None,
        },
    })
}

/// The value of a constant expression over integer literals, character
/// constants and the enumerators already `known`.
fn eval(t: &[Tok], known: &BTreeMap<&str, i128>, depth: usize) -> Option<i128> {
    if depth > MAX_DEPTH {
        return None;
    }
    const LEVELS: &[&[u8]] = &[b"|", b"^", b"&", b"<>", b"+-", b"*/"];
    fn level(t: &[Tok], i: &mut usize, known: &BTreeMap<&str, i128>, depth: usize, at: usize) -> Option<i128> {
        if at == LEVELS.len() {
            return unary(t, i, known, depth);
        }
        let mut left = level(t, i, known, depth, at + 1)?;
        while let Some(Tok::Punct(op)) = t.get(*i).copied() {
            if !LEVELS[at].contains(&op) {
                break;
            }
            *i += 1;
            if at == 3 {
                if t.get(*i) != Some(&Tok::Punct(op)) {
                    return None;
                }
                *i += 1;
            }
            let right = level(t, i, known, depth, at + 1)?;
            left = match op {
                b'|' => left | right,
                b'^' => left ^ right,
                b'&' => left & right,
                b'<' => left.checked_shl(u32::try_from(right).ok().filter(|r| *r < 64)?)?,
                b'>' => left.checked_shr(u32::try_from(right).ok().filter(|r| *r < 64)?)?,
                b'+' => left.checked_add(right)?,
                b'-' => left.checked_sub(right)?,
                b'*' => left.checked_mul(right)?,
                _ => left.checked_div(right)?,
            };
        }
        Some(left)
    }
    fn unary(t: &[Tok], i: &mut usize, known: &BTreeMap<&str, i128>, depth: usize) -> Option<i128> {
        let tok = t.get(*i).copied()?;
        *i += 1;
        match tok {
            Tok::Punct(b'-') => unary(t, i, known, depth)?.checked_neg(),
            Tok::Punct(b'+') => unary(t, i, known, depth),
            Tok::Punct(b'~') => Some(!unary(t, i, known, depth)?),
            Tok::Num(n) => number(n),
            Tok::Chars(c) => (c.len() <= 8 && !c.contains('\\')).then(|| c.bytes().fold(0i128, |v, b| (v << 8) | i128::from(b))),
            Tok::Ident(n) => known.get(n).copied(),
            Tok::Group(b'(', inner) => eval(&lex(inner), known, depth + 1),
            _ => None,
        }
    }
    let mut i = 0;
    let value = level(t, &mut i, known, depth, 0)?;
    (i == t.len()).then_some(value)
}

fn number(text: &str) -> Option<i128> {
    let digits = text.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(hex) = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")) {
        i128::from_str_radix(hex, 16).ok()
    } else if digits.len() > 1 && digits.starts_with('0') {
        i128::from_str_radix(&digits[1..], 8).ok()
    } else {
        digits.parse().ok()
    }
}

/// How a by-value record travels as an argument.
enum Passing {
    /// One register per member, in member order.
    Spread(Vec<Ty>),
    /// A pointer to a caller-held copy.
    ByReference,
}

/// Turns resolved types into the architecture's datatypes.
struct Builder<'i, 'a> {
    index: &'i Index<'a>,
    types: &'i dyn TypeFactory,
    ptr_size: i32,
    word_size: u32,
    /// Each record asked for: its completed type, size and alignment.
    records: HashMap<String, Option<(Rc<Datatype>, i32, i32)>>,
    opaque: HashMap<String, Option<Rc<Datatype>>>,
    enums: HashMap<String, Option<Rc<Datatype>>>,
}

impl<'i, 'a> Builder<'i, 'a> {
    fn new(index: &'i Index<'a>, types: &'i dyn TypeFactory, ptr_size: i32, word_size: u32) -> Builder<'i, 'a> {
        Builder { index, types, ptr_size, word_size, records: HashMap::new(), opaque: HashMap::new(), enums: HashMap::new() }
    }

    /// The prototype of imported function `name`, when its declaration is
    /// readable and its arguments are placed as the default model places them.
    fn prototype(&mut self, name: &str) -> Option<PrototypePieces> {
        let sig = self.index.signature(name)?;
        let (int_regs, float_regs) = match self.index.machine {
            object::Architecture::Aarch64 => (8usize, 8usize),
            object::Architecture::X86_64 => (6, 8),
            _ => (0, 0),
        };
        let outtype = match &sig.ret {
            Ty::Record(_) => {
                let Passing::Spread(_) = self.passing(&sig.ret)? else { return None };
                self.build(&sig.ret, 0)?
            }
            ty => self.build(ty, 0)?,
        };
        let (mut ints, mut floats) = (0usize, 0usize);
        let mut intypes = Vec::new();
        for param in &sig.params {
            let members = match param {
                Ty::Record(_) => match self.passing(param)? {
                    Passing::Spread(members) => members,
                    Passing::ByReference => vec![Ty::Ptr(Box::new(param.clone()))],
                },
                ty => vec![ty.clone()],
            };
            let is_float = |ty: &Ty| matches!(ty, Ty::Float(_));
            let (f, n) = (members.iter().filter(|m| is_float(m)).count(), members.iter().filter(|m| !is_float(m)).count());
            if members.len() > 1 && (floats + f > float_regs || ints + n > int_regs) {
                return None;
            }
            floats += f;
            ints += n;
            for member in &members {
                intypes.push(self.build(member, 0)?);
            }
        }
        Some(PrototypePieces {
            name: name.to_string(),
            outtype: Some(outtype),
            innames: vec![String::new(); intypes.len()],
            first_var_arg_slot: if sig.vararg { intypes.len() as i32 } else { -1 },
            intypes,
            ..Default::default()
        })
    }

    /// AAPCS64 spreads a homogeneous floating-point aggregate of up to four
    /// members over `v` registers and two words over `x` registers, and passes
    /// anything over sixteen bytes by reference. System V x86-64 spreads two
    /// doubles or two words. Every other shape is unplaced.
    fn passing(&mut self, ty: &Ty) -> Option<Passing> {
        let Ty::Record(name) = ty else { return None };
        let (_, size, _) = self.record(name, 0)?;
        let mut leaves = Vec::new();
        self.leaves(ty, &mut leaves, 0)?;
        let word = |ty: &Ty| matches!(ty, Ty::Int(8, _) | Ty::Ptr(_) | Ty::Enum(_, 8, _));
        let aarch64 = self.index.machine == object::Architecture::Aarch64;
        let homogeneous = match leaves.first() {
            Some(Ty::Float(8)) => leaves.iter().all(|l| *l == Ty::Float(8)),
            Some(Ty::Float(4)) => aarch64 && leaves.iter().all(|l| *l == Ty::Float(4)),
            _ => false,
        };
        let max_floats = if aarch64 { 4 } else { 2 };
        if (homogeneous && leaves.len() <= max_floats) || (leaves.len() <= 2 && leaves.iter().all(word)) {
            Some(Passing::Spread(leaves))
        } else if aarch64 && size > 16 {
            Some(Passing::ByReference)
        } else {
            None
        }
    }

    /// The scalar members of a by-value type, nested structs and arrays
    /// flattened; `None` through a union or past sixteen members.
    fn leaves(&mut self, ty: &Ty, out: &mut Vec<Ty>, depth: usize) -> Option<()> {
        match ty {
            Ty::Record(name) => {
                let shape = self.index.record(name, depth)?;
                if shape.union {
                    return None;
                }
                for (_, member) in &shape.members {
                    self.leaves(member, out, depth + 1)?;
                }
            }
            Ty::Array(elem, count) => {
                for _ in 0..(*count).min(17) {
                    self.leaves(elem, out, depth + 1)?;
                }
            }
            scalar => out.push(scalar.clone()),
        }
        (out.len() <= 16).then_some(())
    }

    fn build(&mut self, ty: &Ty, depth: usize) -> Option<Rc<Datatype>> {
        if depth > MAX_DEPTH {
            return None;
        }
        let types = self.types;
        match ty {
            Ty::Void => types.get_type_void().ok(),
            Ty::Int(size, signed) => {
                let meta = if *signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
                types.get_base(i32::from(*size), meta).ok()
            }
            Ty::Char => types.get_type_char(1).ok(),
            Ty::Bool => types.get_base(1, type_metatype::TYPE_BOOL).ok(),
            Ty::Float(size) => types.get_base(i32::from(*size), type_metatype::TYPE_FLOAT).ok(),
            Ty::Ptr(inner) => {
                let pointee = match &**inner {
                    Ty::Func | Ty::Unknown | Ty::Void => types.get_type_void().ok()?,
                    Ty::Object(name) => self.opaque(name, false)?,
                    Ty::Record(name) => match self.record(name, depth + 1) {
                        Some((done, _, _)) => done,
                        None => {
                            let union = self.index.records.get(name.as_str()).is_some_and(|d| d.iter().all(|d| d.union));
                            self.opaque(name, union)?
                        }
                    },
                    other => self.build(other, depth + 1)?,
                };
                types.get_type_pointer(self.ptr_size, pointee, self.word_size).ok()
            }
            Ty::Array(elem, count) if *count > 0 => {
                let elem = self.build(elem, depth + 1)?;
                types.get_type_array(i32::try_from(*count).ok()?, elem).ok()
            }
            Ty::Record(name) => self.record(name, depth + 1).map(|(done, _, _)| done),
            Ty::Enum(name, size, signed) => self.enumeration(name, *size, *signed),
            Ty::Array(..) | Ty::Func | Ty::Object(_) | Ty::Unknown => None,
        }
    }

    /// The record a pointer names without a layout: the one already under
    /// that name, else a declared one, else `void`.
    fn opaque(&mut self, name: &str, union: bool) -> Option<Rc<Datatype>> {
        if let Some(done) = self.opaque.get(name) {
            return done.clone();
        }
        let done = match self.types.find_by_name(name).ok()? {
            Some(t) if matches!(t.get_metatype(), type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION) => Some(t),
            Some(_) => self.types.get_type_void().ok(),
            None => self.types.kuna_declare_record(name, union).ok().or_else(|| self.types.get_type_void().ok()),
        };
        self.opaque.insert(name.to_string(), done.clone());
        done
    }

    /// The completed type of record `name` with its size and alignment.
    fn record(&mut self, name: &str, depth: usize) -> Option<(Rc<Datatype>, i32, i32)> {
        if let Some(done) = self.records.get(name) {
            return done.clone();
        }
        self.records.insert(name.to_string(), None);
        let done = self.complete(name, depth);
        self.records.insert(name.to_string(), done.clone());
        done
    }

    fn complete(&mut self, name: &str, depth: usize) -> Option<(Rc<Datatype>, i32, i32)> {
        let Shape { union, cap, members } = self.index.record(name, depth)?;
        let want = if union { type_metatype::TYPE_UNION } else { type_metatype::TYPE_STRUCT };
        let shell = match self.types.find_by_name(name).ok()? {
            Some(t) if t.get_metatype() != want => return None,
            Some(t) if !t.is_incomplete() => return Some((Rc::clone(&t), t.get_size(), t.get_alignment())),
            Some(t) => t,
            None => self.types.kuna_declare_record(name, union).ok()?,
        };
        let mut fields = Vec::new();
        let (mut offset, mut size, mut align) = (0i32, 0i32, 1i32);
        for (k, (field, ty)) in members.iter().enumerate() {
            let (field_size, field_align) = self.layout(ty, depth + 1)?;
            let field_align = field_align.min(i32::from(cap));
            let built = self.build(ty, depth + 1)?;
            align = align.max(field_align);
            if union {
                fields.push(TypeField::new(k as i32, 0, field.clone(), built));
                size = size.max(field_size);
            } else {
                offset = offset.checked_add(field_align - 1)? / field_align * field_align;
                fields.push(TypeField::new(offset, offset, field.clone(), built));
                offset = offset.checked_add(field_size)?;
                size = offset;
            }
        }
        let size = size.checked_add(align - 1)? / align * align;
        let done = if union {
            self.types.set_fields_union_raw(&shell, fields, size, align)
        } else {
            self.types.set_fields_struct_raw(&shell, fields, Vec::new(), size, align, 0)
        };
        Some((done.ok()?, size, align))
    }

    /// Size and alignment of a member type under the 64-bit Apple and System V
    /// layouts, where every scalar is aligned to its size.
    fn layout(&mut self, ty: &Ty, depth: usize) -> Option<(i32, i32)> {
        match ty {
            Ty::Int(size, _) | Ty::Float(size) | Ty::Enum(_, size, _) => Some((i32::from(*size), i32::from(*size))),
            Ty::Char | Ty::Bool => Some((1, 1)),
            Ty::Ptr(_) => Some((self.ptr_size, self.ptr_size)),
            Ty::Array(elem, count) if *count > 0 => {
                let (size, align) = self.layout(elem, depth + 1)?;
                Some((size.checked_mul(i32::try_from(*count).ok()?)?, align))
            }
            Ty::Record(name) => self.record(name, depth + 1).map(|(_, size, align)| (size, align)),
            _ => None,
        }
    }

    /// The enum type of `name` with its named values, or the plain integer of
    /// its width when the values are not at hand.
    fn enumeration(&mut self, name: &str, size: u8, signed: bool) -> Option<Rc<Datatype>> {
        let plain = |types: &dyn TypeFactory| {
            let meta = if signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
            types.get_base(i32::from(size), meta).ok()
        };
        if name.is_empty() {
            return plain(self.types);
        }
        if let Some(done) = self.enums.get(name) {
            return done.clone().or_else(|| plain(self.types));
        }
        let built = self.index.enumerators(name, size).and_then(|values| match self.types.find_by_name(name).ok()? {
            Some(t) => (t.is_enum_type() && t.get_size() == i32::from(size)).then_some(t),
            None => {
                let meta = if signed { type_metatype::TYPE_ENUM_INT } else { type_metatype::TYPE_ENUM_UINT };
                let shell = self.types.get_type_enum_sized(name, i32::from(size), meta).ok()?;
                self.types.set_enum_values(&shell, values).ok()
            }
        });
        self.enums.insert(name.to_string(), built.clone());
        built.or_else(|| plain(self.types))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARM64: Target = Target { ptr_size: 8, machine: object::Architecture::Aarch64, platform: macho::PLATFORM_IOS };
    const X86_64: Target = Target { ptr_size: 8, machine: object::Architecture::X86_64, platform: macho::PLATFORM_MACOS };
    const ARM32: Target = Target { ptr_size: 4, machine: object::Architecture::Arm, platform: macho::PLATFORM_IOS };

    fn fixture(target: Target) -> Vec<Header> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sdk_c");
        read_headers(&dir, &[], false, target)
    }

    fn with<R>(headers: &[Header], names: &[&str], target: Target, run: impl FnOnce(&Index) -> R) -> R {
        let wanted: BTreeSet<&str> = names.iter().copied().collect();
        run(&Index::new(headers, &wanted, target))
    }

    fn sig(text: &str, name: &str) -> Option<Sig> {
        with(&[header(text, ARM64)], &[name], ARM64, |index| index.signature(name))
    }

    fn ptr(to: Ty) -> Ty {
        Ty::Ptr(Box::new(to))
    }

    fn record(name: &str) -> Ty {
        Ty::Record(name.to_string())
    }

    const LONG: Ty = Ty::Int(8, true);
    const INT: Ty = Ty::Int(4, true);

    /// The stand-in headers: macro-wrapped prototypes, typedef chains, the enum
    /// and object macros, a transparent union, a variadic, and the two
    /// declarations that must stay unread.
    #[test]
    fn reads_the_fixture_headers() {
        let headers = fixture(ARM64);
        let names = [
            "CFStringGetLength",
            "CFStringCreateWithCString",
            "CFStringCompareWithOptions",
            "CFShowFormat",
            "dispatch_async_f",
            "dispatch_set_tier",
            "GadgetGetInfo",
            "GadgetAmbiguous",
            "GadgetOpaque",
        ];
        with(&headers, &names, ARM64, |index| {
            let string = ptr(record("__CFString"));
            assert_eq!(index.signature("CFStringGetLength"), Some(Sig { ret: LONG, params: vec![string.clone()], vararg: false }));
            assert_eq!(
                index.signature("CFStringCreateWithCString"),
                Some(Sig { ret: string.clone(), params: vec![ptr(record("__CFAllocator")), ptr(Ty::Char), Ty::Int(4, false)], vararg: false })
            );
            assert_eq!(
                index.signature("CFStringCompareWithOptions"),
                Some(Sig {
                    ret: Ty::Enum("CFComparisonResult".into(), 8, true),
                    params: vec![string.clone(), string.clone(), record("CFRange"), Ty::Enum("CFStringCompareFlags".into(), 8, false)],
                    vararg: false,
                })
            );
            assert_eq!(index.signature("CFShowFormat"), Some(Sig { ret: Ty::Void, params: vec![string], vararg: true }));
            let queue = ptr(record("dispatch_queue_s"));
            assert_eq!(
                index.signature("dispatch_async_f"),
                Some(Sig { ret: Ty::Void, params: vec![queue.clone(), ptr(Ty::Void), ptr(Ty::Func)], vararg: false })
            );
            assert_eq!(
                index.signature("dispatch_set_tier"),
                Some(Sig { ret: Ty::Void, params: vec![queue, Ty::Enum("dispatch_tier_t".into(), 4, false)], vararg: false }),
                "a transparent union is its first member"
            );
            assert_eq!(index.signature("GadgetGetInfo"), Some(Sig { ret: INT, params: vec![ptr(Ty::Void), ptr(record("GadgetInfo"))], vararg: false }));
            assert_eq!(index.signature("GadgetAmbiguous"), None, "two declarations that disagree");
            assert_eq!(index.signature("GadgetOpaque"), None, "an unresolved type by value");
            let values = |name: &str, size| index.enumerators(name, size).unwrap_or_default().into_iter().collect::<Vec<_>>();
            assert_eq!(
                values("CFStringCompareFlags", 8),
                [(1, "kCFCompareCaseInsensitive".to_string()), (4, "kCFCompareBackwards".to_string()), (64, "kCFCompareNumerically".to_string())]
            );
            assert_eq!(values("dispatch_tier_t", 4), [(9, "DISPATCH_TIER_LOW".to_string()), (0x21, "DISPATCH_TIER_HIGH".to_string())]);
            assert_eq!(values("CFComparisonResult", 8)[2], (u64::MAX, "kCFCompareLessThan".to_string()));
        });
    }

    /// The branch of an `#if` the target rules out is not read, so `CFIndex`
    /// has one width per target instead of two that disagree.
    #[test]
    fn conditional_branches_follow_the_target() {
        let text = "#if __LP64__\ntypedef long W;\n#else\ntypedef int W;\n#endif\n#ifdef __cplusplus\nextern \"C\" {\n#endif\nW f(W);\n#ifdef __cplusplus\n}\n#endif\n";
        assert_eq!(sig(text, "f"), Some(Sig { ret: LONG, params: vec![LONG], vararg: false }));
        let narrow = with(&[header(text, ARM32)], &["f"], ARM32, |index| index.signature("f"));
        assert_eq!(narrow, Some(Sig { ret: INT, params: vec![INT], vararg: false }));

        let lex_if = |e: &str| condition(&lex(e), ARM64, 0);
        assert_eq!(lex_if("defined(__arm64__) && !defined(__cplusplus)"), Some(true));
        assert_eq!(lex_if("defined(__x86_64__) || defined __i386__"), Some(false));
        assert_eq!(lex_if("defined(__x86_64__) || HAVE_FEATURE"), None);
        assert_eq!(lex_if("__LP64__ && FOO"), None);
        assert_eq!(lex_if("!__LP64__ && FOO"), Some(false));
        assert_eq!(lex_if("__IPHONE_OS_VERSION_MIN_REQUIRED < 50000"), None);
        assert_eq!(lex_if("TARGET_OS_IPHONE && !TARGET_OS_OSX"), Some(true));
        assert_eq!(lex_if("__has_feature(assume_nonnull)"), Some(true));
        assert_eq!(lex_if("__has_feature(cxx_rvalue_references)"), None);
        assert_eq!(condition(&lex("TARGET_OS_IPHONE"), X86_64, 0), Some(false));
        assert_eq!(condition(&lex("TARGET_OS_IPHONE"), Target { platform: macho::PLATFORM_UNKNOWN, ..ARM64 }, 0), None);

        let undecided = "#if FEATURE\ntypedef long V;\n#elif __LP64__\ntypedef long V;\n#else\ntypedef char V;\n#endif\nV g(void);\n";
        assert_eq!(sig(undecided, "g"), Some(Sig { ret: LONG, params: vec![], vararg: false }), "the else after a taken branch is not read");
        let both = "#if FEATURE\ntypedef long V;\n#else\ntypedef char V;\n#endif\nV g(void);\n";
        assert_eq!(sig(both, "g"), None, "an undecided condition keeps both, and they disagree");
    }

    #[test]
    fn statements_skip_linkage_blocks_and_inline_bodies() {
        let text = "extern \"C\" {\nint a(int);\nstatic inline int b(int x) { return x; }\nstruct S { int f; };\ntypedef CF_ENUM(int, E) { A, B };\n}\nint c(void);";
        let stmts: Vec<&str> = statements(text).into_iter().map(str::trim).collect();
        assert_eq!(stmts, ["int a(int)", "struct S { int f; }", "typedef CF_ENUM(int, E) { A, B }", "int c(void)"]);
    }

    /// Attribute macros in every position, qualifier macros the headers define
    /// themselves, and the declarator shapes of a parameter.
    #[test]
    fn declarations_are_read_through_their_noise() {
        let text = concat!(
            "typedef struct __Ref *Ref;\n",
            "typedef unsigned int CC_LONG;\n",
            "typedef Ref Name CF_TYPED_ENUM;\n",
            "typedef unsigned long offset_t __kernel_ptr_semantics;\n",
            "typedef void (*callback_t)(void *) __TVOS_PROHIBITED;\n",
            "API_AVAILABLE(macos(10.6), ios(4.0)) CF_EXPORT Ref cg_nullable make(Ref cg_nullable src, const double * __nullable v) CG_AVAILABLE_STARTING(10.0, 2.0);\n",
            "extern unsigned char *CC_HASH(const void *data, CC_LONG len, unsigned char *md) API_AVAILABLE(macos(10.4));\n",
            "mig_external\nextern\n__TVOS_PROHIBITED\nint mig_call(offset_t address, Name name);\n",
            "int spawn(char *const argv[__restrict], char buf[16], void (*done)(int), void (^block)(void), callback_t cb);\n",
            "int printf_like(const char * restrict, ...) __printflike(1, 2);\n",
            "int unspecified();\n",
            "long double wide(void);\n",
            "int bits(__int128 v);\n",
        );
        let get = |name: &str| sig(text, name);
        let r = ptr(record("__Ref"));
        assert_eq!(get("make"), Some(Sig { ret: r.clone(), params: vec![r.clone(), ptr(Ty::Float(8))], vararg: false }));
        assert_eq!(get("CC_HASH"), Some(Sig { ret: ptr(Ty::Int(1, false)), params: vec![ptr(Ty::Void), Ty::Int(4, false), ptr(Ty::Int(1, false))], vararg: false }));
        assert_eq!(get("mig_call"), Some(Sig { ret: INT, params: vec![Ty::Int(8, false), r], vararg: false }));
        assert_eq!(
            get("spawn"),
            Some(Sig { ret: INT, params: vec![ptr(ptr(Ty::Char)), ptr(Ty::Char), ptr(Ty::Func), ptr(Ty::Func), ptr(Ty::Func)], vararg: false })
        );
        assert_eq!(get("printf_like"), Some(Sig { ret: INT, params: vec![ptr(Ty::Char)], vararg: true }));
        assert_eq!(get("unspecified"), None, "an empty parameter list declares nothing");
        assert_eq!(get("wide"), None);
        assert_eq!(get("bits"), None);
    }

    /// A typedef defined two ways is unresolved, except that two pointers are
    /// still a pointer and an enum agrees with its underlying integer.
    #[test]
    fn disagreeing_definitions_are_not_chosen_between() {
        let sized = "typedef int T;\ntypedef long T;\nvoid f(T);\nvoid g(T *);\n";
        assert_eq!(sig(sized, "f"), None);
        assert_eq!(sig(sized, "g"), Some(Sig { ret: Ty::Void, params: vec![ptr(Ty::Unknown)], vararg: false }));
        let pointers = "typedef struct a_s *T;\nOS_OBJECT_DECL(b);\ntypedef struct b_s *T;\ntypedef T U;\nvoid f(T, b_t, U);\n";
        assert_eq!(sig(pointers, "f"), Some(Sig { ret: Ty::Void, params: vec![ptr(Ty::Unknown), ptr(record("b_s")), ptr(Ty::Unknown)], vararg: false }));
        let enums = "OS_ENUM(mode, unsigned int, MODE_A = 1, MODE_B);\ntypedef mode_t alias_t;\ntypedef unsigned int alias_t;\nvoid f(alias_t);\n";
        assert_eq!(sig(enums, "f"), Some(Sig { ret: Ty::Void, params: vec![Ty::Enum("mode_t".into(), 4, false)], vararg: false }));
        let bare = "NS_ENUM(long, Kind) { KindA, KindB = 4 } API_AVAILABLE(ios(13.0));\nvoid f(Kind);\n";
        assert_eq!(sig(bare, "f"), Some(Sig { ret: Ty::Void, params: vec![Ty::Enum("Kind".into(), 8, true)], vararg: false }));
        let repeated = "int f(int);\nint f(int x);\nextern int f(const int);\n";
        assert_eq!(sig(repeated, "f"), Some(Sig { ret: INT, params: vec![INT], vararg: false }));
    }

    /// An SDK root gives the linked frameworks plus `usr/include` without its
    /// C++ tree; a kext gets `Kernel.framework` alone.
    #[test]
    fn header_set_follows_the_image() {
        let root = std::env::temp_dir().join(format!("kuna-sdkprotos-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for file in [
            "System/Library/Frameworks/Alpha.framework/Headers/Alpha.h",
            "System/Library/Frameworks/Beta.framework/Headers/Beta.h",
            "System/Library/Frameworks/Kernel.framework/Headers/IOKit/IOLib.h",
            "usr/include/sys/types.h",
            "usr/include/c++/v1/vector.h",
        ] {
            let path = root.join(file);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, "int f(void);\n").unwrap();
        }
        let names = |frameworks: &[String], kext| -> Vec<String> {
            header_paths(&root, frameworks, kext).iter().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()).collect()
        };
        assert_eq!(names(&["Alpha".to_string()], false), ["Alpha.h", "types.h"]);
        assert_eq!(names(&[], false), ["Alpha.h", "Beta.h", "types.h"]);
        assert_eq!(names(&[], true), ["IOLib.h"]);
        let plain = root.join("usr/include");
        assert_eq!(header_paths(&plain, &[], false).len(), 2, "any other directory is read whole");
        std::fs::remove_dir_all(&root).unwrap();

        let mut macho = vec![0u8; 32];
        macho[..4].copy_from_slice(&0xfeed_facfu32.to_le_bytes());
        macho[12..16].copy_from_slice(&object::macho::MH_KEXT_BUNDLE.to_le_bytes());
        assert!(is_kext(&macho));
        macho[12..16].copy_from_slice(&object::macho::MH_EXECUTE.to_le_bytes());
        assert!(!is_kext(&macho) && !is_kext(&macho[..8]));
    }

    /// Prototypes as the model will place them: a two-word struct spread over
    /// two registers, four doubles over four, a wider struct by reference; the
    /// layouts behind the pointers; no prototype where the convention differs.
    #[test]
    fn prototypes_place_by_value_aggregates() {
        let headers = fixture(ARM64);
        let names = ["CFStringFindWithOptions", "GadgetRectGetWidth", "GadgetSum", "GadgetGetInfo", "CFShowFormat", "dispatch_set_tier"];
        let types = super::super::tests::fmt_factory();
        with(&headers, &names, ARM64, |index| {
            let mut builder = Builder::new(index, &types, 8, 1);
            let find = builder.prototype("CFStringFindWithOptions").expect("prototype");
            assert_eq!(find.intypes.len(), 6, "theString, stringToFind, location, length, options, result");
            assert_eq!(find.outtype.as_ref().unwrap().get_metatype(), type_metatype::TYPE_BOOL);
            assert!(find.intypes[4].is_enum_type());
            let range = find.intypes[5].get_ptr_to().expect("CFRange *");
            assert_eq!((range.get_name(), range.get_size()), ("CFRange", 16));

            let width = builder.prototype("GadgetRectGetWidth").expect("prototype");
            assert_eq!(width.intypes.len(), 4);
            assert!(width.intypes.iter().all(|t| t.get_metatype() == type_metatype::TYPE_FLOAT && t.get_size() == 8));

            let sum = builder.prototype("GadgetSum").expect("prototype");
            assert_eq!(sum.intypes.len(), 2);
            assert_eq!(sum.intypes[0].get_ptr_to().map(|t| t.get_size()), Some(32), "over sixteen bytes: a pointer to the copy");

            let info = builder.prototype("GadgetGetInfo").expect("prototype");
            let layout = info.intypes[1].get_ptr_to().expect("GadgetInfo *");
            assert_eq!(layout.get_size(), 32);
            let offsets: Vec<(i32, String)> = (0..4).filter_map(|k| layout.get_field(k)).map(|f| (f.offset, f.name.clone())).collect();
            assert_eq!(offsets, [(0, "version".to_string()), (8, "count".to_string()), (16, "label".to_string()), (24, "flags".to_string())]);

            assert_eq!(builder.prototype("CFShowFormat").expect("prototype").first_var_arg_slot, 1);
            assert_eq!(builder.prototype("dispatch_set_tier").expect("prototype").intypes[1].get_size(), 4);
        });
        with(&fixture(X86_64), &names, X86_64, |index| {
            let mut builder = Builder::new(index, &types, 8, 1);
            assert!(builder.prototype("GadgetRectGetWidth").is_none(), "System V passes four doubles in memory");
            assert!(builder.prototype("GadgetSum").is_none());
            assert_eq!(builder.prototype("CFStringFindWithOptions").map(|p| p.intypes.len()), Some(6));
        });
    }

    /// A packed record, a bitfield and a member of unknown size have no
    /// layout: a pointer to one is opaque and a by-value use is refused.
    #[test]
    fn unreadable_layouts_stay_opaque() {
        let text = concat!(
            "struct Packed { char a; long b; } __attribute__((packed));\n",
            "struct Bits { unsigned a : 3; unsigned b : 5; };\n",
            "struct Sized { long sig; char opaque[SIZE_MACRO]; };\n",
            "struct Node { struct Node *next; long value; };\n",
            "long p(struct Packed *); long b(struct Bits *); long s(struct Sized *); long n(struct Node *);\n",
            "long byvalue(struct Packed);\n",
        );
        let headers = [header(text, ARM64)];
        let types = super::super::tests::fmt_factory();
        with(&headers, &["p", "b", "s", "n", "byvalue"], ARM64, |index| {
            let mut builder = Builder::new(index, &types, 8, 1);
            for name in ["p", "b", "s"] {
                let pointee = builder.prototype(name).expect("prototype").intypes[0].get_ptr_to().expect("pointer");
                assert!(pointee.is_incomplete(), "{name}");
            }
            let node = builder.prototype("n").expect("prototype").intypes[0].get_ptr_to().expect("pointer");
            assert_eq!((node.get_size(), node.get_field(1).map(|f| f.offset)), (16, Some(8)), "a self-referential record completes");
            assert!(builder.prototype("byvalue").is_none());
        });
    }

    /// A packing pragma caps the alignment of the members it covers; a form
    /// the reader does not follow, or one under an undecided condition, leaves
    /// every later record of the file without a layout.
    #[test]
    fn packing_pragmas_cap_member_alignment() {
        let text = concat!(
            "struct Before { char a; long b; };\n",
            "#pragma pack(push, 4)\n",
            "struct Four { char a; long b; int c; };\n",
            "#pragma pack(push, 1)\n",
            "struct One { char a; long b; };\n",
            "#pragma pack(pop)\n",
            "struct FourAgain { short a; long b; };\n",
            "#pragma pack(pop)\n",
            "struct After { char a; long b; };\n",
            "#pragma options align=mac68k\n",
            "struct Two { char a; long b; };\n",
            "#pragma options align=reset\n",
            "#pragma pack(push, n)\n",
            "struct Lost { char a; long b; };\n",
            "#pragma pack(pop)\n",
            "struct LostToo { char a; long b; };\n",
            "long f(struct Before *, struct Four *, struct One *, struct FourAgain *, struct After *, struct Two *, struct Lost *, struct LostToo *);\n",
        );
        let headers = [header(text, ARM64)];
        let types = super::super::tests::fmt_factory();
        with(&headers, &["f"], ARM64, |index| {
            let proto = Builder::new(index, &types, 8, 1).prototype("f").expect("prototype");
            let shape = |k: usize| {
                let record = proto.intypes[k].get_ptr_to().expect("pointer");
                (!record.is_incomplete()).then(|| (record.get_size(), record.get_field(1).map(|f| f.offset)))
            };
            assert_eq!(shape(0), Some((16, Some(8))));
            assert_eq!(shape(1), Some((16, Some(4))));
            assert_eq!(shape(2), Some((9, Some(1))));
            assert_eq!(shape(3), Some((12, Some(4))));
            assert_eq!(shape(4), Some((16, Some(8))));
            assert_eq!(shape(5), Some((10, Some(2))));
            assert_eq!(shape(6), None);
            assert_eq!(shape(7), None);
        });
        let guarded = "#ifdef FEATURE\n#pragma pack(push, 2)\n#endif\nstruct S { char a; long b; };\nlong f(struct S *);\n";
        with(&[header(guarded, ARM64)], &["f"], ARM64, |index| {
            let proto = Builder::new(index, &types, 8, 1).prototype("f").expect("prototype");
            assert!(proto.intypes[0].get_ptr_to().expect("pointer").is_incomplete());
        });
    }

    /// A record or enumerator list under a condition that cannot be decided,
    /// or with one inside it, is not taken for the layout: the 32-bit-inode
    /// `struct stat` is the shape of this. An include guard is no condition.
    #[test]
    fn undecided_conditionals_give_no_layout() {
        let text = concat!(
            "#ifndef GUARD_H\n#define GUARD_H\n",
            "#if !defined(OTHER_GUARD)\n#define OTHER_GUARD 1\n",
            "struct Plain { long a; long b; };\n",
            "#if FEATURE\n",
            "struct Either MACRO_BODY;\n",
            "#else\n",
            "struct Either { int old; long layout; };\n",
            "#endif\n",
            "struct Split { long a;\n#ifdef FEATURE\n long extra;\n#endif\n long b; };\n",
            "#if TARGET_OS_IPHONE\n",
            "struct Phone { long a; int b; };\n",
            "typedef enum { PhoneA = 1, PhoneB } PhoneKind;\n",
            "#else\n",
            "struct Phone { int a; };\n",
            "#endif\n",
            "#ifndef OTHER_DEFINES_PLAIN\n",
            "struct Plain { long a; long b; };\n",
            "#endif\n",
            "#if FEATURE\n",
            "typedef enum { MaybeA, MaybeB } MaybeKind;\n",
            "#endif\n",
            "long f(struct Plain *, struct Either *, struct Split *, struct Phone *, PhoneKind, MaybeKind);\n",
            "long g(int a\n#if FEATURE\n, int b\n#endif\n);\n",
            "API_AVAILABLE(ios(2.0))\n#if FEATURE\nint h(long *_Nonnull);\n#else\nint h(long *);\n#endif\n",
            "#ifdef linkage\nlinkage\n#else\nextern\n#endif\nint mig(long port);\n",
            "#if FEATURE\nlong\n#else\nint\n#endif\nwidth(void);\n",
            "#endif\n#endif\n",
        );
        let headers = [header(text, ARM64)];
        let types = super::super::tests::fmt_factory();
        with(&headers, &["f", "g", "h", "mig", "width"], ARM64, |index| {
            let proto = Builder::new(index, &types, 8, 1).prototype("f").expect("prototype");
            let complete = |k: usize| !proto.intypes[k].get_ptr_to().expect("pointer").is_incomplete();
            assert!(complete(0), "inside two include guards, and repeated the same under a condition");
            assert!(!complete(1), "the other branch may define it");
            assert!(!complete(2), "a member under an undecided condition");
            assert!(complete(3), "a platform condition is decided");
            assert_eq!(proto.intypes[3].get_ptr_to().map(|t| t.get_size()), Some(16));
            assert!(proto.intypes[4].is_enum_type());
            assert!(!proto.intypes[5].is_enum_type(), "enumerators under an undecided condition");
            assert_eq!(proto.intypes[5].get_size(), 4);
            assert_eq!(index.signature("g"), None, "a declaration divided by an undecided condition");
            assert_eq!(index.signature("h"), Some(Sig { ret: INT, params: vec![ptr(LONG)], vararg: false }), "an attribute ahead of both branches");
            assert_eq!(index.signature("mig"), Some(Sig { ret: INT, params: vec![LONG], vararg: false }), "a conditional linkage word ahead of the type");
            assert_eq!(index.signature("width"), None, "a conditional return type");
        });
    }

    #[test]
    fn constant_expressions_evaluate() {
        let value = |text: &str| eval(&lex(text), &BTreeMap::from([("kBase", 16i128)]), 0);
        assert_eq!(value("(1UL << 6) | kBase"), Some(80));
        assert_eq!(value("0x08000100"), Some(0x0800_0100));
        assert_eq!(value("-1L"), Some(-1));
        assert_eq!(value("~0u & 0xff"), Some(0xff));
        assert_eq!(value("'abcd'"), Some(0x6162_6364));
        assert_eq!(value("010 + 2 * 3"), Some(14));
        assert_eq!(value("1 << 200"), None);
        assert_eq!(value("sizeof(int)"), None);
        assert_eq!(value("kOther"), None);
    }

    /// Unbalanced brackets, hostile nesting and non-ASCII text are read to the
    /// end without a prototype, never a panic.
    #[test]
    fn malformed_headers_yield_nothing() {
        let deep = format!("typedef int {}x{};\nint f(int);\n", "(".repeat(5000), ")".repeat(5000));
        assert_eq!(sig(&deep, "f"), Some(Sig { ret: INT, params: vec![INT], vararg: false }));
        let chain: String = (0..200).map(|k| format!("typedef T{} T{};\n", k + 1, k)).collect();
        assert_eq!(sig(&format!("{chain}int f(T0);\n"), "f"), None);
        let cyclic = "typedef A B;\ntypedef B A;\nstruct S { struct S inner; };\nint f(A);\nint g(struct S);\n";
        assert_eq!(sig(cyclic, "f"), None);
        for text in ["int f(int", "int f(int));", "struct { int f(", "int f(caf\u{e9} x);", "typedef int \u{e9};\nint f(\u{e9});", "int f(int x[1 <<]);", "#if (((\nint f(int);"] {
            let _ = sig(text, "f");
        }
        let headers = [header(cyclic, ARM64)];
        let types = super::super::tests::fmt_factory();
        with(&headers, &["g"], ARM64, |index| assert!(Builder::new(index, &types, 8, 1).prototype("g").is_none()));
    }
}
