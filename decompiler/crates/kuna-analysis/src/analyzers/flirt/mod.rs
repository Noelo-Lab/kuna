//! (kuna `flirt`) Name library functions with IDA FLIRT `.sig` signatures.
//!
//! The `.sig` files ship with IDA (`<ida>/sig/<proc>/*.sig`); kuna reads them
//! ([`sig`]) and matches every discovered function entry against them, the way
//! IDA's `AU_LIBF` pass does. Reading the files and the byte-level match
//! (trie, CRC, tail bytes, instruction bits) is the `flirt-sig` crate; this
//! module adds the reference names and the rounds:
//!
//! 1. walk the byte trie over the first `pattern_len` bytes at the entry;
//! 2. at a leaf, a module matches when the CRC-16 of the `crc_len` bytes after
//!    the prefix, its tail bytes (offsets after the CRC region) and its
//!    expected instruction bits agree;
//! 3. a module that names a referenced function is accepted only when the
//!    instruction at the reference offset points to a function of that name
//!    (a branch or call target, or on x86 the address in its 32-bit field).
//!    Names found in one round can satisfy references in the next, so
//!    matching runs up to [`MAX_ROUNDS`] rounds (IDA's three `AU_LIBF`
//!    queues).
//!
//! Modules that disagree on the name are arbitrated by their reference sites,
//! and a module wholly inside the trie prefix must not contradict its sites
//! (see [`match_all`]). A name goes to one address only (IDA's collision
//! rule without `AF_SIGMLT`). The names go out as [`FidMatch`] renames, so they
//! only replace `sub_` placeholders.
//!
//! The signature files are named by `KUNA_FLIRT_SIGS` (`:`-separated files or
//! directories).

use std::collections::{BTreeMap, HashMap, HashSet};

use object::read::{Object, ObjectSection};

use crate::listing::Listing;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Phase};
use flirt_sig::{Endian, Matcher, Module, Signature, FLAG_STARTUP};

/// The environment variable that lists the `.sig` files.
pub const FLIRT_SIGS_ENV: &str = "KUNA_FLIRT_SIGS";

/// Matching rounds: a module whose reference names a function another module
/// names is accepted in the round after that name exists.
pub const MAX_ROUNDS: usize = 3;

/// The FLIRT matcher pass (a Listing consumer).
pub struct FlirtPass;

impl AnalysisPass for FlirtPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "flirt"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let Some(listing) = ctx.listing else { return out };
        let sigs = load_sigs(ctx.file.format());
        if sigs.is_empty() {
            return out;
        }
        let endian = if ctx.file.is_little_endian() { Endian::Little } else { Endian::Big };
        let matchers: Vec<Matcher<'_>> = sigs.iter().map(|s| Matcher::new(s, endian)).collect();
        let image = Image::new(ctx.file);
        let entries: Vec<u64> = listing.functions().map(|(&a, _)| a).collect();
        let known: HashMap<u64, String> = listing
            .functions()
            .filter_map(|(&a, f)| f.name.clone().map(|n| (a, n)))
            .collect();
        for (addr, name) in match_all(&matchers, &image, listing, &entries, known, true) {
            out.fid_names.push(FidMatch { addr, name });
        }
        out
    }
}

/// Parse the signature files `KUNA_FLIRT_SIGS` names (files, or directories
/// whose `*.sig` files are all taken) that apply to a binary of `format`:
/// startup signatures and those whose header excludes the file type are left
/// out. Unreadable or malformed files are skipped.
pub fn load_sigs(format: object::BinaryFormat) -> Vec<Signature> {
    let Some(list) = std::env::var_os(FLIRT_SIGS_ENV) else { return Vec::new() };
    let ft = match format {
        object::BinaryFormat::Elf => Some(18),
        object::BinaryFormat::Pe => Some(11),
        object::BinaryFormat::Coff => Some(10),
        object::BinaryFormat::MachO => Some(25),
        _ => None,
    };
    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    for p in std::env::split_paths(&list) {
        match std::fs::read_dir(&p) {
            Ok(dir) => {
                let mut sigs: Vec<_> = dir
                    .flatten()
                    .map(|e| e.path())
                    .filter(|f| f.extension().is_some_and(|x| x == "sig"))
                    .collect();
                sigs.sort();
                paths.extend(sigs);
            }
            Err(_) => paths.push(p),
        }
    }
    paths
        .iter()
        .filter_map(|p| std::fs::read(p).ok())
        .filter(|d| applies_to(d, ft))
        .filter_map(|d| Signature::parse(&d).ok())
        .collect()
}

/// Whether a `.sig` file's header admits a binary of IDA file type `ft`
/// (`FT_*` bit index): not a startup signature, and the file-type mask is unset
/// or names `ft`. Read from the raw header so excluded files are never parsed.
pub fn applies_to(data: &[u8], ft: Option<u32>) -> bool {
    if data.len() < 18 || &data[..6] != b"IDASGN" {
        return false;
    }
    let formats = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let flags = u16::from_le_bytes([data[16], data[17]]);
    flags & FLAG_STARTUP == 0 && (formats == 0 || ft.is_none_or(|b| formats >> b & 1 == 1))
}

/// The image bytes by address: every section with file content.
pub struct Image<'a> {
    spans: Vec<(u64, &'a [u8])>,
    /// x86 code addresses through 32-bit instruction fields (`rel32` from the
    /// instruction end on x86-64, absolute everywhere).
    x86_fields: bool,
}

impl<'a> Image<'a> {
    pub fn new(file: &'a object::File<'a>) -> Self {
        let mut spans: Vec<(u64, &[u8])> = file
            .sections()
            .filter(|s| s.address() != 0)
            .filter_map(|s| Some((s.address(), s.data().ok()?)))
            .filter(|(_, d)| !d.is_empty())
            .collect();
        spans.sort_by_key(|&(a, _)| a);
        let x86_fields = matches!(file.architecture(), object::Architecture::X86_64 | object::Architecture::I386);
        Image { spans, x86_fields }
    }

    /// The bytes from `addr` to the end of its section.
    pub fn from(&self, addr: u64) -> &'a [u8] {
        self.spans
            .iter()
            .find(|&&(a, d)| addr >= a && addr - a < d.len() as u64)
            .map_or(&[], |&(a, d)| &d[(addr - a) as usize..])
    }

    /// On x86, the addresses the 32-bit field at `at` of the instruction ending
    /// at `end` can encode: RIP-relative, then absolute.
    fn field_targets(&self, at: u64, end: u64) -> Vec<u64> {
        let b = self.from(at);
        if !self.x86_fields || at + 4 > end || b.len() < 4 {
            return Vec::new();
        }
        let v = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        vec![end.wrapping_add_signed(i64::from(v as i32)), u64::from(v)]
    }
}

/// Run every round of matching and return the accepted `(entry, name)` pairs.
///
/// At each entry every module that passes the byte checks and whose required
/// references resolve is a candidate. With `sites` on (what the pass runs):
/// an entry that is a bare return is never named; a module that lies wholly
/// inside the trie prefix and names reference sites is dropped when a site's
/// target carries another name, and waits for the last round unless one site
/// agrees; and candidates that disagree on the name are scored by their sites
/// the way IDA 9.x arbitrates ambiguous matches (`libida.so` 0x70c280): +4 for
/// a site whose target already has the expected name, -4 for one whose target
/// has another name, 0 when the target is unnamed. The strictly best name wins;
/// a tie is left for a later round, when more targets are named. A name found
/// at several entries in one round goes to the strictly best scored one, or to
/// none. A round that names nothing new makes the next one the last. With
/// `sites` off, a rename needs every candidate to agree.
pub fn match_all(
    sigs: &[Matcher<'_>],
    image: &Image<'_>,
    listing: &Listing,
    entries: &[u64],
    mut known: HashMap<u64, String>,
    sites: bool,
) -> BTreeMap<u64, String> {
    let mut named: BTreeMap<u64, String> = BTreeMap::new();
    let mut refused: HashSet<String> = HashSet::new();
    let mut last = false;
    for round in 0..MAX_ROUNDS {
        last |= round + 1 == MAX_ROUNDS;
        let mut found: Vec<(u64, String, i32)> = Vec::new();
        for &ea in entries {
            if named.contains_key(&ea) {
                continue;
            }
            let code = image.from(ea);
            if sites && listing.instruction_at(ea).is_some_and(|i| i.flow.is_terminal && !i.flow.is_jump) {
                continue;
            }
            let mut best: BTreeMap<String, i32> = BTreeMap::new();
            for s in sigs {
                let pattern_len = usize::from(s.signature().header.pattern_len);
                for m in s.match_at(code) {
                    let unnamed_ok = sites && last && m.length as usize > pattern_len;
                    if !reference_ok(m, ea, image, listing, &known, unnamed_ok) {
                        continue;
                    }
                    let Some(name) = function_name(m, pattern_len, sites) else { continue };
                    let tally = if sites { site_tally(m, ea, image, listing, &known) } else { Tally::default() };
                    let unconfirmed = tally.differ > 0 || (tally.agree == 0 && !last);
                    if sites && m.length as usize <= pattern_len && tally.named > 0 && unconfirmed {
                        continue;
                    }
                    let score = 4 * (tally.agree as i32 - tally.differ as i32);
                    let e = best.entry(name).or_insert(score);
                    *e = (*e).max(score);
                }
            }
            let mut ranked: Vec<(&String, i32)> = best.iter().map(|(n, &s)| (n, s)).collect();
            ranked.sort_by_key(|&(_, s)| std::cmp::Reverse(s));
            let decided = match ranked.as_slice() {
                [(name, score)] => Some(((*name).clone(), *score)),
                [(name, top), (_, next), ..] if sites && top > next => Some(((*name).clone(), *top)),
                _ => None,
            };
            if let Some((name, score)) = decided {
                found.push((ea, name, score));
            }
        }
        let mut scores: HashMap<&str, Vec<i32>> = HashMap::new();
        for (_, n, s) in &found {
            scores.entry(n.as_str()).or_default().push(*s);
        }
        let mut accepted: Vec<(u64, String)> = Vec::new();
        for (ea, name, score) in &found {
            let rivals = &scores[name.as_str()];
            let top = rivals.iter().copied().max().unwrap_or(*score);
            if rivals.iter().filter(|&&o| o == top).count() > 1 || (!sites && rivals.len() > 1) {
                refused.insert(name.clone());
            } else if *score == top {
                accepted.push((*ea, name.clone()));
            }
        }
        let before = named.len();
        for (ea, name) in accepted {
            if !refused.contains(&name) && !known.values().any(|k| k == &name) {
                known.insert(ea, name.clone());
                named.insert(ea, name);
            }
        }
        if !sites {
            named.retain(|_, n| !refused.contains(n));
        }
        if named.len() == before {
            if last {
                break;
            }
            last = true;
        }
    }
    named
}

/// The module's own name: the public at offset 0, unless it is unnamed (`?`) or
/// doubtful (flag 4). Without `sites`, a module too short to tell apart (no
/// longer than the trie prefix, with nothing referenced to check) names
/// nothing either.
fn function_name(m: &Module, pattern_len: usize, sites: bool) -> Option<String> {
    let p = m.publics.iter().find(|p| p.offset == 0)?;
    if p.name.is_empty() || p.name.starts_with(b"?") || p.flags & 0x04 != 0 {
        return None;
    }
    if !sites && m.length as usize <= pattern_len && m.reference.is_none() && m.sites.iter().all(|s| s.name.len() < 2) {
        return None;
    }
    Some(String::from_utf8_lossy(&p.name).into_owned())
}

/// How the instruction covering a module offset relates to the name the module
/// expects there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SiteMatch {
    Agrees,
    Differs,
    Unnamed,
}

/// What the instruction covering `ea + off` points to, against `want`: its
/// branch and call targets, plus on x86 the address its 32-bit field at that
/// offset encodes (a `lea` or `mov` of a function's address, which the Listing
/// does not record as a flow).
fn site_match(
    ea: u64,
    off: i64,
    want: &[u8],
    image: &Image<'_>,
    listing: &Listing,
    known: &HashMap<u64, String>,
) -> Option<SiteMatch> {
    let at = ea.checked_add_signed(off)?;
    let insn = listing.instruction_before(at.saturating_add(1))?;
    let end = insn.addr + u64::from(insn.len);
    if at >= end {
        return None;
    }
    let mut targets: Vec<u64> = insn.flows.clone();
    targets.extend(image.field_targets(at, end));
    let names: Vec<&String> = targets.iter().filter_map(|t| known.get(t)).collect();
    let want = String::from_utf8_lossy(want);
    Some(if names.iter().any(|k| same_name(k, &want)) {
        SiteMatch::Agrees
    } else if !names.is_empty() {
        SiteMatch::Differs
    } else {
        SiteMatch::Unnamed
    })
}

/// A module's named reference sites (its classic reference and its v11 sites
/// with a real name) at `ea`, by verdict.
#[derive(Debug, Default)]
struct Tally {
    named: usize,
    agree: usize,
    differ: usize,
}

fn site_tally(m: &Module, ea: u64, image: &Image<'_>, listing: &Listing, known: &HashMap<u64, String>) -> Tally {
    let reference = m.reference.as_ref().filter(|r| r.name != b".").map(|r| (r.offset, r.name.as_slice()));
    let sites = m
        .sites
        .iter()
        .filter(|s| s.name.len() >= 2 && s.flags & 0x08 == 0)
        .map(|s| (i64::from(s.offset), s.name.as_slice()));
    let mut seen: HashSet<i64> = HashSet::new();
    let mut t = Tally::default();
    for (off, name) in reference.into_iter().chain(sites).filter(|&(o, _)| seen.insert(o)) {
        t.named += 1;
        match site_match(ea, off, name, image, listing, known) {
            Some(SiteMatch::Agrees) => t.agree += 1,
            Some(SiteMatch::Differs) => t.differ += 1,
            _ => {}
        }
    }
    t
}

/// A module's reference (and, for a module of at most 16 bytes, its named v11
/// sites) must name a function the instruction at that offset points to. A
/// target with no name yet leaves the module for a later round; with
/// `unnamed_ok` (the last round, for a module whose CRC covers bytes past the
/// trie prefix) the classic reference may stay unnamed.
fn reference_ok(
    m: &Module,
    ea: u64,
    image: &Image<'_>,
    listing: &Listing,
    known: &HashMap<u64, String>,
    unnamed_ok: bool,
) -> bool {
    let reference_ok = m.reference.as_ref().filter(|r| r.name != b".").is_none_or(|r| {
        match site_match(ea, r.offset, &r.name, image, listing, known) {
            Some(SiteMatch::Agrees) => true,
            Some(SiteMatch::Unnamed) => unnamed_ok,
            _ => false,
        }
    });
    reference_ok
        && m
            .sites
            .iter()
            .filter(|s| m.length <= 0x10 && s.name.len() >= 2 && s.flags & 0x08 == 0)
            .all(|s| site_match(ea, i64::from(s.offset), &s.name, image, listing, known) == Some(SiteMatch::Agrees))
}

/// FLIRT names carry the object file's spelling (`_strlen` on a Mach-O or COFF
/// library); compare with jump-thunk and import prefixes, `@N` decorations and
/// leading underscores stripped.
fn same_name(have: &str, want: &str) -> bool {
    have == want || bare_name(have) == bare_name(want)
}

fn bare_name(name: &str) -> &str {
    let mut s = name;
    while let Some(rest) = s.strip_prefix("j_") {
        s = rest;
    }
    s = ["__imp__", "__imp_", "_imp__", "_imp_", "imp__", "imp_"]
        .iter()
        .find_map(|p| s.strip_prefix(p))
        .unwrap_or(s);
    let body = s.strip_prefix('@').unwrap_or(s);
    if let Some((base, n)) = body.rsplit_once('@') {
        if !base.is_empty() && !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) {
            s = base;
        }
    }
    s.trim_start_matches('_')
}

#[cfg(test)]
mod tests;
