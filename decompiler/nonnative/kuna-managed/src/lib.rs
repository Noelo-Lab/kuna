//! The contract between the `kuna` CLI and a managed-bytecode engine.
//!
//! An [`Engine`] recognizes its inputs, opens one as a [`Program`] and owns
//! its `--option` rows. A program lists its types and members ([`Inventory`]),
//! resolves the engine's selection grammar to a [`Target`], and decompiles a
//! selection, handing over each root type's source file ([`Unit`]) when it is
//! done. The rest is the CLI's and the same for every engine: command lines,
//! triage, the text and JSON documents, the source-tree export, failure
//! reports, the stall watchdog ([`Watch`]) and the memory guard
//! ([`MemoryGuard`]). A type is named in the engine's own spelling throughout
//! (`com/ex/Main$Inner`); [`Inventory::display`] prints one.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

pub trait Engine: Sync {
    /// `jvm`: the `--engine` value and catalog tag, the prefix of its options, `kuna docs jvm`.
    fn name(&self) -> &'static str;
    /// `JVM`: how messages name the engine and its inputs.
    fn label(&self) -> &'static str;
    /// What its inputs are: `class files, jars, directories`.
    fn inputs(&self) -> &'static str;
    /// The file extension of a type that has no text (`java`).
    fn extension(&self) -> &'static str;
    /// Whether `path` is this engine's input; runs on every invocation, so a `stat` and a short read.
    fn sniff(&self, path: &Path) -> bool;
    /// What `path` holds, or why it cannot be decompiled.
    fn classify(&self, path: &Path) -> Result<Kind, String>;
    /// Reads `path` under the resolved `--option` pairs.
    fn open(&self, path: &Path, options: &[(String, String)]) -> Result<Box<dyn Program>, String>;
    /// The engine key and value `--option NAME VALUE` sets.
    fn resolve(&self, name: &str, value: &str) -> Result<(String, String), String>;
    /// Applies `--language VALUE` to the command line's `options`; `input` reads `x.jar is a JVM input (jar)`.
    fn language(&self, value: &str, input: &str, options: &mut Vec<(String, String)>) -> Result<(), String>;
    /// The option rows and the text of the generated catalog around them.
    fn options(&self) -> Catalog;
    /// What is wrong with the option rows beyond the rules every engine's follow.
    fn check(&self) -> Vec<String>;
    /// `kuna <sub> <input> --help`.
    fn usage(&self, sub: &str) -> String;
    /// Why `target` has no text of its own in a unit written in `language`.
    fn hidden(&self, target: &Target, language: &str) -> String;
}

pub trait Program {
    fn inventory(&self) -> &Inventory;
    /// The type or members `text` names, or why it names none.
    fn resolve(&self, text: &str) -> Result<Target, String>;
    /// Opens what a run reads besides the input: `libraries`, and the runtime library it returns.
    fn prepare(&mut self, libraries: &[String]) -> Result<Option<Runtime>, String>;
    /// Decompiles the roots among `settings.select`, handing each unit to `deliver` in root order; the selected
    /// types without a unit, or why the run stopped.
    fn run(&self, settings: &RunSettings, deliver: &mut dyn FnMut(Unit)) -> Result<Vec<String>, String>;
}

pub struct Kind {
    /// `jar`, as in `JVM input (jar)`.
    pub name: &'static str,
    /// One type, which `decompile` takes without a target.
    pub single: bool,
}

/// An engine's option rows and the text of `docs/options-<name>.md` around them.
pub struct Catalog {
    pub rows: Vec<OptionRow>,
    pub preamble: String,
    /// The tier sections in order: tier, heading, blurb.
    pub sections: [(&'static str, &'static str, &'static str); 3],
    pub tail: &'static str,
}

/// The fields of a `[[settable]]` row of `phases.toml`, and the other names the option answers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptionRow {
    pub option: &'static str,
    pub aliases: &'static [&'static str],
    pub values: &'static str,
    pub default: &'static str,
    pub destructive: bool,
    pub phase: &'static str,
    pub subphase: &'static str,
    pub strength: &'static str,
    pub rewind: &'static str,
    pub issue: &'static str,
    pub summary: &'static str,
    pub use_when: &'static str,
    pub example: &'static str,
    pub source_decompiler: &'static str,
    pub inspiration: &'static str,
    pub change_kind: &'static str,
    pub tier: &'static str,
    /// `|`-separated.
    pub symptoms: &'static str,
}

/// Every type of an input, read without decompiling.
pub struct Inventory {
    pub types: Vec<TypeInfo>,
    /// A single-type input's type: what `decompile` without a target means.
    pub own: Option<String>,
    /// A type name as messages print it (`com.ex.Main$Inner`).
    pub display: fn(&str) -> String,
    index: BTreeMap<String, usize>,
}

#[derive(Default)]
pub struct TypeInfo {
    pub name: String,
    /// The type it is declared in.
    pub outer: Option<String>,
    pub access: Vec<&'static str>,
    pub source_file: Option<String>,
    /// Bytes of its definition.
    pub size: u64,
    pub members: Vec<MemberInfo>,
}

#[derive(Default)]
pub struct MemberInfo {
    pub name: String,
    pub signature: String,
    pub access: Vec<&'static str>,
    /// Compiler-generated: `--limit` leaves it out.
    pub synthetic: bool,
    /// Bytes of code.
    pub size: u64,
}

impl Inventory {
    pub fn new(types: Vec<TypeInfo>, own: Option<String>, display: fn(&str) -> String) -> Inventory {
        let index = types.iter().enumerate().map(|(i, t)| (t.name.clone(), i)).collect();
        Inventory { types, own, display, index }
    }

    pub fn get(&self, name: &str) -> Option<&TypeInfo> {
        self.index.get(name).map(|&i| &self.types[i])
    }

    pub fn member(&self, owner: &str, name: &str, signature: &str) -> Option<&MemberInfo> {
        self.get(owner)?.members.iter().find(|m| m.name == name && m.signature == signature)
    }

    /// `name` and the types it is declared in, innermost first, as far as the input has them. The engine writes
    /// a type into the unit of the nearest of them it takes for a root, which need not be the last.
    pub fn chain(&self, name: &str) -> Vec<String> {
        let mut chain = vec![name.to_string()];
        for _ in 0..256 {
            let current = chain.last().map(String::as_str).unwrap_or_default();
            match self.get(current).and_then(|t| t.outer.clone()) {
                Some(outer) if outer != current && self.index.contains_key(&outer) => chain.push(outer),
                _ => break,
            }
        }
        chain
    }

    /// The outermost type of `name`'s chain.
    pub fn root_of(&self, name: &str) -> String {
        self.chain(name).pop().unwrap_or_default()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Type(String),
    /// The members `name` of `owner`, one overload when `signature` is given.
    Members { owner: String, name: String, signature: Option<String> },
}

impl Target {
    pub fn owner(&self) -> &str {
        match self {
            Target::Type(owner) | Target::Members { owner, .. } => owner,
        }
    }

    pub fn matches(&self, owner: &str, name: &str, signature: &str) -> bool {
        match self {
            Target::Type(_) => false,
            Target::Members { owner: o, name: n, signature: s } => {
                o == owner && n == name && s.as_deref().is_none_or(|s| s == signature)
            }
        }
    }
}

/// The library an engine reads for context, such as the JVM's JDK.
pub struct Runtime {
    pub path: String,
    pub version: Option<String>,
}

pub struct RunSettings {
    /// Engine threads; `None`: as many as fit.
    pub jobs: Option<usize>,
    /// The types whose units to deliver; `None`: every root.
    pub select: Option<BTreeSet<String>>,
    /// Seconds a member may take; 0: no limit.
    pub member_seconds: u64,
    /// Memory to plan with, in bytes.
    pub available_memory: Option<u64>,
    pub watch: Arc<Watch>,
    pub memory: Option<MemoryGuard>,
}

/// The CLI's per-member memory limit, run on the engine thread of the member.
#[derive(Clone, Copy)]
pub struct MemoryGuard {
    /// Arms the limit for the member that starts, until `disarm`.
    pub arm: fn(),
    pub disarm: fn(),
    /// Why the member must stop, if it must: polled with its deadline.
    pub over: fn() -> Option<String>,
}

/// The members in progress on the engine threads, one slot per engine instance, for the CLI's stall watchdog.
#[derive(Default)]
pub struct Watch {
    busy: Mutex<BTreeMap<usize, (Instant, String)>>,
    slots: AtomicUsize,
}

impl Watch {
    pub fn slot(&self) -> usize {
        self.slots.fetch_add(1, Ordering::Relaxed)
    }

    pub fn enter(&self, slot: usize, member: &str) {
        self.lock().insert(slot, (Instant::now(), member.to_string()));
    }

    pub fn leave(&self, slot: usize) {
        self.lock().remove(&slot);
    }

    /// A member that has run longer than `limit`.
    pub fn stuck(&self, limit: Duration) -> Option<String> {
        self.lock().values().find(|(since, _)| since.elapsed() >= limit).map(|(_, member)| member.clone())
    }

    fn lock(&self) -> MutexGuard<'_, BTreeMap<usize, (Instant, String)>> {
        self.busy.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// The source file of a root type.
#[derive(Default)]
pub struct Unit {
    pub name: String,
    /// `com/ex/Main.java`.
    pub path: String,
    /// `java`, `kotlin`, ...
    pub language: &'static str,
    /// `None`: the engine wrote no text.
    pub text: Option<String>,
    /// Why there is no text, or why it is an error comment.
    pub error: Option<String>,
    /// The types and members written into the text, in writing order.
    pub types: Vec<TypeSpan>,
    pub members: Vec<MemberSpan>,
}

/// A type written into a unit's text: bytes `start..end`, from 1-based line `start_line`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeSpan {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub start_line: u32,
    /// Why its text is an error comment.
    pub error: Option<String>,
    /// Bodies in its text outside its members that could not be decompiled.
    pub nested: Vec<Failure>,
}

/// A member written into a unit's text, from the start of its first line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberSpan {
    pub owner: String,
    pub name: String,
    pub signature: String,
    pub access: Vec<&'static str>,
    pub start: usize,
    pub end: usize,
    pub start_line: u32,
    pub end_line: u32,
    pub error: Option<String>,
    /// `(line, bytecode offsets)`, in line order.
    pub lines: Vec<(u32, Vec<u32>)>,
    /// Bodies written inside it that could not be decompiled.
    pub nested: Vec<Failure>,
    /// Engine comments that flag its text as possibly wrong.
    pub warnings: Vec<String>,
}

/// A body that could not be decompiled: a member (`com.ex.Main.lambda$main$0(I)V`) or a type (`com.ex.Main$1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub body: String,
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inventory() -> Inventory {
        let ty = |name: &str, outer: Option<&str>| TypeInfo {
            name: name.into(),
            outer: outer.map(Into::into),
            ..TypeInfo::default()
        };
        let types = vec![ty("a/Main", None), ty("a/Main$In", Some("a/Main")), ty("a/Main$In$1", Some("a/Main$In"))];
        Inventory::new(types, None, |n| n.replace('/', "."))
    }

    #[test]
    fn a_chain_runs_out_through_the_declaring_types() {
        let inv = inventory();
        assert_eq!(inv.chain("a/Main$In$1"), ["a/Main$In$1", "a/Main$In", "a/Main"]);
        assert_eq!(inv.chain("a/Main"), ["a/Main"]);
        assert_eq!(inv.root_of("a/Main$In"), "a/Main");
        assert_eq!(inv.root_of("b/Gone"), "b/Gone");
    }

    #[test]
    fn a_member_past_the_limit_is_reported_stuck() {
        let watch = Watch::default();
        watch.enter(watch.slot(), "loop ()V");
        assert_eq!(watch.stuck(Duration::from_secs(3600)), None);
        assert_eq!(watch.stuck(Duration::ZERO).as_deref(), Some("loop ()V"));
        watch.leave(0);
        assert_eq!(watch.stuck(Duration::ZERO), None);
    }
}
