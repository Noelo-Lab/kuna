//! The JVM engine behind the managed-engine contract (`kuna_managed`), which
//! the `kuna` CLI drives: inputs, options, the selection grammar, the JDK a
//! run reads, and the driver configuration of a run.
//!
//! Types are named by their internal names (`com/ex/Main$Inner`) and printed
//! with dots. A class is `com.ex.Main`, `com/ex/Main`, `Main.class`,
//! `com.ex.Main$Inner` or `com.ex.Main.Inner`, or a simple name (`Main`,
//! `Main.Inner`) that only one class has. A method is `<class>.<name>` or
//! `<class>#<name>`, optionally followed by its JVM descriptor; `<init>` and
//! `<clinit>` are written as such. Names are the class file's, also for
//! Kotlin: a property is its accessors (`getX`, `setX`), a companion member is
//! in `Outer$Companion`, a top-level function in its `…Kt` file class. A whole
//! target that names a class is that class; otherwise it splits at its last `.`.
//!
//! A run's engine logger is silent (per-method failures reach the output), it
//! marks method starts and ends on the CLI's watch and arms the CLI's memory
//! guard around each method.

use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use kuna_managed::{
    Catalog, Engine, Inventory, Kind, MemberInfo, MemoryGuard, Program, RunSettings, Runtime, Target, TypeInfo, Unit,
    Watch,
};

use crate::i_fernflower_logger::{IFernflowerLogger, Severity};
use crate::kuna_driver::{self, ClassResult, DriverConfig, Jobs, LoggerFactory};
use crate::kuna_input::{self, InputError, InputKind, InputSet, LoadOptions, SharedRuntime};
use crate::kuna_inventory::{self, class_access_names, method_access_names, ClassInfo};
use crate::kuna_options;
use crate::rt::Caught;

const DOCS_POINTER: &str = "kuna docs jvm";

/// The Kotlin plugin's switch; `--language java` turns it off.
const KT_ENABLE: &str = "kt-enable";

/// The JVM engine.
pub struct Jvm;

/// `com/ex/Main$Inner` as `com.ex.Main$Inner`.
pub fn java_name(internal: &str) -> String {
    internal.replace('/', ".")
}

/// The value of the last pair keyed `key`.
fn last<'a>(options: &'a [(String, String)], key: &str) -> Option<&'a str> {
    options.iter().rev().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

impl Engine for Jvm {
    fn name(&self) -> &'static str {
        "jvm"
    }

    fn label(&self) -> &'static str {
        "JVM"
    }

    fn inputs(&self) -> &'static str {
        "class files, jars, directories"
    }

    fn extension(&self) -> &'static str {
        "java"
    }

    fn sniff(&self, path: &Path) -> bool {
        kuna_input::sniff(path)
    }

    fn classify(&self, path: &Path) -> Result<Kind, String> {
        let kind = kuna_input::classify(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if !kind.is_supported() {
            return Err(format!("{} ({DOCS_POINTER})", InputError::Unsupported(path.into(), kind)));
        }
        Ok(Kind { name: kind.name(), single: kind == InputKind::ClassFile })
    }

    fn open(&self, path: &Path, options: &[(String, String)]) -> Result<Box<dyn Program>, String> {
        let siblings = last(options, "jvm_siblings").is_none_or(|v| v == "1");
        let input = kuna_input::load(path, &LoadOptions { siblings }).map_err(|e| e.to_string())?;
        kuna_driver::trace(|| "cli input read".into());
        let own = (input.kind() == InputKind::ClassFile)
            .then(|| input.classes().first().map(|c| c.entry.to_string()))
            .flatten();
        let types: Vec<TypeInfo> = kuna_inventory::list(&input).into_iter().map(type_info).collect();
        if types.is_empty() {
            return Err(format!("{}: no class file could be read", path.display()));
        }
        let engine_options = options
            .iter()
            .filter(|(k, _)| k != "jvm_siblings" && k != "include-runtime")
            .cloned()
            .collect();
        Ok(Box::new(JvmProgram {
            name: path.display().to_string(),
            input,
            inventory: Inventory::new(types, own, java_name),
            runtime_spec: last(options, "include-runtime").unwrap_or("1").to_string(),
            options: engine_options,
            runtime: None,
            libraries: Vec::new(),
        }))
    }

    fn resolve(&self, name: &str, value: &str) -> Result<(String, String), String> {
        kuna_options::resolve(name, value).map(|(key, value)| (key.to_string(), value))
    }

    fn language(&self, value: &str, input: &str, options: &mut Vec<(String, String)>) -> Result<(), String> {
        match value {
            "auto" => Ok(()),
            "java" if last(options, KT_ENABLE) == Some("1") => {
                Err("--language java writes every class as Java, and --option jvm_kt_enable on asks for Kotlin".into())
            }
            "java" => {
                options.push((KT_ENABLE.to_string(), "0".to_string()));
                Ok(())
            }
            "kotlin" => Err(format!(
                "--language kotlin: the JVM engine writes a class as Kotlin when the class was \
                 compiled from Kotlin (it carries @kotlin.Metadata), which --language auto, the \
                 default, already does; it cannot write a Java class as Kotlin \
                 (--language auto|java; {DOCS_POINTER})"
            )),
            _ => Err(format!(
                "--language {value}: {input}, which decompiles to Java, and \
                 to Kotlin for classes compiled from Kotlin (--language auto|java)"
            )),
        }
    }

    fn options(&self) -> Catalog {
        let owned: String = kuna_options::driver_owned()
            .iter()
            .map(|owned| format!("- `{}` -- {}\n", owned.vineflower, owned.kuna))
            .collect();
        Catalog {
            rows: kuna_options::rows().iter().map(|r| r.row).collect(),
            preamble: format!(
                "# kuna JVM option catalog\n\n**Generated** by `kuna catalog --markdown --engine jvm` from \
                 `decompiler/nonnative/kuna-jvm/options.toml` -- do not edit by hand; edit the \
                 table row and regenerate.\n\nThese options steer the JVM engine (`kuna-jvm`, a port of \
                 Vineflower 1.12.0), which decompiles class files, jars and directories of classes to Java, \
                 and classes compiled from Kotlin to Kotlin. They apply to those inputs only, as the options \
                 of `docs/options.md` apply to native inputs only. Set any of them per run on `decompile`, \
                 `decompile-all`, `decompile-project` and `functions`: `kuna decompile app.jar com.ex.Main \
                 --option <name> <value>` (repeatable). Every Vineflower option also answers \
                 to Vineflower's long and three-letter names (its `aliases`). Booleans take \
                 `on|off` (also `1|0|true|false`). Defaults are Vineflower 1.12.0's, its \
                 bundled plugins' included. kuna's own rows have no Vineflower name.\n\n\
                 Vineflower keys kuna sets itself, so they are not options:\n\n{owned}\nThree tiers:\n\n\
                 - **Toggleable transforms** -- restructure, remove, or insert code; the on/off control surface.\n\
                 - **Input & analysis** -- what the engine reads: the class universe, libraries, the JDK.\n\
                 - **Rendering defaults** -- how the text is written: Java or Kotlin, and how it is spelled.\n\n"
            ),
            sections: [
                (
                    "transform",
                    "## Toggleable transforms",
                    "Each of these restructures, removes or inserts code, so each stays flippable.",
                ),
                ("analysis", "## Input & analysis", "What the engine loads and infers before any class is written."),
                (
                    "core",
                    "## Rendering defaults",
                    "How the text is written: its language (Java, or Kotlin for Kotlin classes), \
                     formatting, literals, comments.",
                ),
            ],
            tail: "## Programmatic use\n\n```bash\n# discover (machine-readable; includes tier, symptoms, aliases):\n\
                   kuna catalog --engine jvm --json\n\n# transforms only:\nkuna catalog --engine jvm --tier transform\n\n\
                   # decompile with an option flipped (repeatable):\n\
                   kuna decompile app.jar com.ex.Main --option jvm_decompile_finally off\n```\n",
        }
    }

    fn check(&self) -> Vec<String> {
        kuna_options::driver_owned()
            .iter()
            .filter(|owned| kuna_options::find(owned.vineflower).is_some())
            .map(|owned| format!("driver-owned key {:?} is also an option row", owned.vineflower))
            .collect()
    }

    fn usage(&self, sub: &str) -> String {
        let body = match sub {
            "decompile" => {
                "kuna decompile <class|jar|dir> <class|method> [--json] [--omit line_mappings,classes] \\\n\
                 \x20             [--language auto|java] [--option NAME VALUE].. [--lib PATH].. [--jobs N|auto] \\\n\
                 \x20             [--mode M] [--max-fn-seconds N] [--max-fn-memory MiB]\n\
                 \n\
                 A class prints its whole compilation unit (a nested class: its own part);\n\
                 a method prints its slice. Targets: com.ex.Main, com/ex/Main, Main.class,\n\
                 com.ex.Main$Inner or com.ex.Main.Inner, bare Main or Main.Inner when unique;\n\
                 com.ex.Main.check or com.ex.Main#check, optionally with a JVM descriptor:\n\
                 com.ex.Main.check(Ljava/lang/String;)Z. A single .class input may omit it.\n\
                 A class compiled from Kotlin prints as Kotlin; --language java prints it as Java."
            }
            "decompile-all" => {
                "kuna decompile-all <class|jar|dir> [--json] [--functions a,b,..] [--filter REGEX] \\\n\
                 \x20                 [--min-size N] [--max-size N] [--sort addr|size|name] [--limit N] \\\n\
                 \x20                 [--omit line_mappings,classes] [--language auto|java] [--option NAME VALUE].. \\\n\
                 \x20                 [--lib PATH].. [--jobs N|auto] [--mode M] [--max-fn-seconds N] [--max-fn-memory MiB]\n\
                 \n\
                 Every compilation unit under `// Class:` headers; the triage flags narrow to\n\
                 methods (sizes are bytecode lengths) and print `// Method:` slices instead."
            }
            "decompile-project" => {
                "kuna decompile-project <class|jar|dir> [-o DIR] [--functions a,b,..] [--language auto|java] \\\n\
                 \x20                     [--option NAME VALUE].. [--lib PATH].. [--jobs N|auto] [--mode M] \\\n\
                 \x20                     [--max-fn-seconds N] [--max-fn-memory MiB]\n\
                 \n\
                 Writes DIR/<package path>/<Class>.java per top-level class, <Class>.kt for one\n\
                 compiled from Kotlin (default DIR: <input>.kuna). Exits 0 when at least one\n\
                 class was produced."
            }
            "functions" => {
                "kuna functions <class|jar|dir> [--json] [--filter REGEX] [--min-size N] [--max-size N] \\\n\
                 \x20             [--sort addr|size|name] [--limit N] [--option NAME VALUE]..\n\
                 \n\
                 Every method, read from the class files (nothing is decompiled), as\n\
                 `com.ex.Main.check(Ljava/lang/String;)Z`; sizes are bytecode lengths."
            }
            _ => "kuna decompile|decompile-all|decompile-project|functions <class|jar|dir> ..",
        };
        format!(
            "usage: {body}\n\nThis is the JVM engine's form (class files, jars, directories of classes;\n\
             Java and Kotlin output).\n\
             Options: `kuna catalog --engine jvm`; manual: `{DOCS_POINTER}`."
        )
    }

    fn hidden(&self, target: &Target, language: &str) -> String {
        let owner = java_name(target.owner());
        match target {
            Target::Members { name, .. } if language == "kotlin" => format!(
                "{owner}.{name} has no declaration of its own in the Kotlin source of {owner}: the Kotlin writer \
                 puts a primary constructor in the class header and folds lambda bodies, $default \
                 methods and @JvmStatic forwarders into the declarations they serve (--option \
                 jvm_kt_decompile_facades on writes a multifile facade's functions); --language java \
                 (jvm_kt_enable off) writes the class as Java, method by method, and --option \
                 jvm_remove_synthetic off adds its synthetic methods"
            ),
            Target::Members { name, .. } if name == "<init>" => format!(
                "{owner}.<init> is not in the decompiled source of {owner} (the engine leaves out default \
                 constructors, and always an anonymous class's; --option jvm_hide_default_constructor off \
                 prints the default ones)"
            ),
            Target::Members { name, .. } => format!(
                "{owner}.{name} is not in the decompiled source of {owner} (the engine folds synthetic and bridge \
                 methods into their users; --option jvm_remove_synthetic off and jvm_remove_bridge off \
                 print them)"
            ),
            Target::Type(_) => format!(
                "{owner} is not in the decompiled source: the engine writes an anonymous or local class inside \
                 the method that creates it, so not when that method failed, and leaves synthetic classes out \
                 (--option jvm_remove_synthetic off prints them)"
            ),
        }
    }
}

fn type_info(class: ClassInfo) -> TypeInfo {
    TypeInfo {
        access: class_access_names(class.inner_access.unwrap_or(class.access)),
        members: class
            .methods
            .into_iter()
            .map(|m| MemberInfo {
                access: method_access_names(m.access),
                synthetic: m.access & 0x1040 != 0,
                size: u64::from(m.code_length.unwrap_or(0)),
                name: m.name,
                signature: m.descriptor,
            })
            .collect(),
        name: class.internal,
        outer: class.outer,
        source_file: class.source_file,
        size: class.size as u64,
    }
}

/// An opened JVM input.
struct JvmProgram {
    /// The input as the command line named it.
    name: String,
    input: InputSet,
    inventory: Inventory,
    /// `jvm_include_runtime`: `1`/`current`, `0`/empty (none), or a JDK home.
    runtime_spec: String,
    /// The Vineflower pairs, in command-line order.
    options: Vec<(String, String)>,
    runtime: Option<Arc<SharedRuntime>>,
    libraries: Vec<Arc<InputSet>>,
}

impl Program for JvmProgram {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn resolve(&self, text: &str) -> Result<Target, String> {
        resolve(&self.inventory, text, &self.name)
    }

    /// A JDK that cannot be found is a note, not an error, unless the command line named its home.
    fn prepare(&mut self, libraries: &[String]) -> Result<Option<Runtime>, String> {
        self.runtime = match self.runtime_spec.as_str() {
            "0" | "" => None,
            "1" | "current" => match SharedRuntime::open("1") {
                Ok(runtime) => Some(Arc::new(runtime)),
                Err(_) => {
                    eprintln!(
                        "note: no JDK found (JAVA_HOME, java on PATH, the usual install directories); \
                         decompiling without the Java class library. --option jvm_include_runtime <JDK home> \
                         names one; `off` silences this note"
                    );
                    None
                }
            },
            home => Some(Arc::new(
                SharedRuntime::open(home).map_err(|e| format!("--option jvm_include_runtime {home}: {e}"))?,
            )),
        };
        for lib in libraries {
            let set = kuna_input::load_library(Path::new(lib)).map_err(|e| format!("--lib {lib}: {e}"))?;
            self.libraries.push(Arc::new(set));
        }
        Ok(self.runtime.as_ref().map(|rt| Runtime {
            path: rt.home().display().to_string(),
            version: rt.version().map(str::to_string),
        }))
    }

    fn run(&self, settings: &RunSettings, deliver: &mut dyn FnMut(Unit)) -> Result<Vec<String>, String> {
        let (watch, memory) = (settings.watch.clone(), settings.memory);
        let logger: LoggerFactory = Arc::new(move || {
            let slot = watch.slot();
            Rc::new(CliLogger {
                severity: Cell::new(Severity::Error),
                watch: watch.clone(),
                slot,
                memory,
            }) as Rc<dyn IFernflowerLogger>
        });
        let mut options = vec![("log-level".to_string(), "warn".to_string())];
        options.extend(self.options.iter().cloned());
        let mut config = DriverConfig::new(options, logger);
        config.jobs = settings.jobs.map_or(Jobs::Auto, Jobs::Exactly);
        config.available_memory = settings.available_memory;
        config.runtime = self.runtime.clone();
        config.libraries = self.libraries.clone();
        config.select = settings.select.clone();
        config.max_method_seconds = u32::try_from(settings.member_seconds).unwrap_or(u32::MAX);
        if let Some(memory) = memory {
            config.method_guard = Some(Arc::new(move || Rc::new(memory.over) as Rc<dyn Fn() -> Option<String>>));
        }
        kuna_driver::run(&self.input, &config, &mut |result| deliver(unit(result)))
            .map(|report| report.failed)
            .map_err(|e| e.to_string())
    }
}

struct CliLogger {
    severity: Cell<Severity>,
    watch: Arc<Watch>,
    slot: usize,
    memory: Option<MemoryGuard>,
}

impl IFernflowerLogger for CliLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, _message: &str, _severity: Severity) {}

    fn write_message_ex(&self, _message: &str, _severity: Severity, _t: &Caught) {}

    fn start_method(&self, method_name: &str) {
        self.watch.enter(self.slot, method_name);
        if let Some(memory) = self.memory {
            (memory.arm)();
        }
    }

    fn end_method(&self) {
        if let Some(memory) = self.memory {
            (memory.disarm)();
        }
        self.watch.leave(self.slot);
    }
}

fn unit(result: ClassResult) -> Unit {
    let Some(class) = result.unit else {
        return Unit {
            name: result.qualified_name,
            error: result.error,
            ..Unit::default()
        };
    };
    Unit {
        name: result.qualified_name,
        language: class.language(),
        path: class.path,
        text: Some(class.content),
        error: result.error,
        types: class.classes,
        members: class.methods,
    }
}

/// The class `text` names in `inv`, or why it names none (`Ok(None)`: no such class).
fn class(inv: &Inventory, text: &str) -> Result<Option<String>, String> {
    let text = text.strip_suffix(".class").unwrap_or(text);
    if text.is_empty() {
        return Ok(None);
    }
    let mut candidate = text.replace('.', "/");
    if inv.get(&candidate).is_some() {
        return Ok(Some(candidate));
    }
    while let Some(slash) = candidate.rfind('/') {
        candidate.replace_range(slash..=slash, "$");
        if inv.get(&candidate).is_some() {
            return Ok(Some(candidate));
        }
    }
    if text.contains('/') {
        return Ok(None);
    }
    let nested = text.replace('.', "$");
    let simple: Vec<&str> = inv
        .types
        .iter()
        .map(|c| c.name.as_str())
        .filter(|internal| internal.rsplit('/').next() == Some(nested.as_str()))
        .collect();
    match simple.as_slice() {
        [] => Ok(None),
        [one] => Ok(Some(one.to_string())),
        many => Err(format!(
            "class name {text:?} is ambiguous: {}; give the qualified name",
            many.iter().map(|c| java_name(c)).collect::<Vec<_>>().join(", ")
        )),
    }
}

/// Resolves `text` against `inv` (`input` names the input); a single class
/// file's own class also takes bare method names.
fn resolve(inv: &Inventory, text: &str, input: &str) -> Result<Target, String> {
    let text = text.trim();
    let (head, descriptor) = match text.find('(') {
        Some(open) => (&text[..open], Some(text[open..].to_string())),
        None => (text, None),
    };
    if descriptor.is_none() {
        if let Some(class) = class(inv, head)? {
            return Ok(Target::Type(class));
        }
    }
    let split = head.rfind('#').or_else(|| head.rfind('.')).map(|at| (&head[..at], &head[at + 1..]));
    if let Some((class_text, name)) = split {
        if let Some(class) = class(inv, class_text)? {
            return method_target(inv, class, name, descriptor, text);
        }
    }
    if let (Some(class), false) = (inv.own.as_deref(), head.contains(['.', '#', '/'])) {
        if inv.get(class).is_some_and(|c| c.members.iter().any(|m| m.name == head)) {
            return method_target(inv, class.to_string(), head, descriptor, text);
        }
    }
    let mut message = format!("no class or method {text:?} in {input}");
    let mut names: Vec<String> = Vec::new();
    for class in &inv.types {
        let owner = java_name(&class.name);
        for method in &class.members {
            names.push(format!("{owner}.{}", method.name));
        }
        names.push(owner);
    }
    let wanted = head.replace('/', ".");
    match nearest(&wanted, names.iter().map(String::as_str)) {
        Some(near) => message.push_str(&format!("; did you mean {near:?}?")),
        None => message.push_str(&format!("; `kuna functions {input}` lists every method")),
    }
    Err(message)
}

fn method_target(
    inv: &Inventory,
    class: String,
    name: &str,
    descriptor: Option<String>,
    text: &str,
) -> Result<Target, String> {
    let info = inv.get(&class).expect("resolved class");
    let found = info
        .members
        .iter()
        .any(|m| m.name == name && descriptor.as_deref().is_none_or(|d| d == m.signature));
    if found {
        return Ok(Target::Members { owner: class, name: name.to_string(), signature: descriptor });
    }
    let owner = java_name(&class);
    let overloads: Vec<String> = info
        .members
        .iter()
        .filter(|m| m.name == name)
        .map(|m| format!("{owner}.{}{}", m.name, m.signature))
        .collect();
    let mut message = format!("no method {text:?} in {owner}");
    let accessors = property_accessors(info, name);
    if !overloads.is_empty() {
        message.push_str(&format!("; its overloads are {}", overloads.join(", ")));
    } else if !accessors.is_empty() {
        let named: Vec<String> = accessors.iter().map(|a| format!("{:?}", format!("{owner}.{a}"))).collect();
        message.push_str(&format!(
            "; a Kotlin property is selected by its JVM accessors: {}",
            named.join(", ")
        ));
    } else if let Some(near) = nearest(name, info.members.iter().map(|m| m.name.as_str())) {
        message.push_str(&format!("; did you mean {:?}?", format!("{owner}.{near}")));
    }
    Err(message)
}

/// The getter and setter names Kotlin gives a property `name` that `class` has.
fn property_accessors<'a>(class: &'a TypeInfo, name: &str) -> Vec<&'a str> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Vec::new();
    };
    let capitalized: String = first.to_uppercase().chain(chars).collect();
    let wanted = [format!("get{capitalized}"), format!("is{capitalized}"), format!("set{capitalized}")];
    let mut found: Vec<&str> = Vec::new();
    for want in &wanted {
        if let Some(m) = class.members.iter().find(|m| &m.name == want) {
            found.push(m.name.as_str());
        }
    }
    found
}

/// The closest of `candidates` to `wanted`: equal ignoring case first, else
/// the nearest within an edit budget that grows with the name.
fn nearest<'a>(wanted: &str, candidates: impl Iterator<Item = &'a str> + Clone) -> Option<&'a str> {
    let lowered = wanted.to_ascii_lowercase();
    if let Some(hit) = candidates.clone().find(|c| c.to_ascii_lowercase() == lowered) {
        return Some(hit);
    }
    let budget = (wanted.chars().count() / 4).clamp(1, 3);
    candidates
        .map(|c| (kuna_options::distance(&lowered, &c.to_ascii_lowercase()), c))
        .filter(|(d, _)| *d <= budget)
        .min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(b.1)))
        .map(|(_, c)| c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class(internal: &str, outer: Option<&str>, methods: &[(&str, &str)]) -> TypeInfo {
        TypeInfo {
            name: internal.into(),
            outer: outer.map(Into::into),
            members: methods
                .iter()
                .map(|(n, d)| MemberInfo { name: (*n).into(), signature: (*d).into(), ..MemberInfo::default() })
                .collect(),
            ..TypeInfo::default()
        }
    }

    fn universe() -> Inventory {
        let types = vec![
            class(
                "com/ex/Main",
                None,
                &[("<init>", "(I)V"), ("check", "(Ljava/lang/String;)Z"), ("add", "(II)I"), ("add", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")],
            ),
            class("com/ex/Main$Checker", Some("com/ex/Main"), &[("verify", "(Ljava/lang/String;)Z")]),
            class("com/ex/Main$1", Some("com/ex/Main"), &[("run", "()V")]),
            class("com/ex/util/Helper", None, &[("twice", "(I)I")]),
        ];
        Inventory::new(types, None, java_name)
    }

    #[test]
    fn classes_resolve_in_every_spelling() {
        let u = universe();
        let main = Target::Type("com/ex/Main".into());
        for text in ["com.ex.Main", "com/ex/Main", "Main", "Main.class", "com/ex/Main.class"] {
            assert_eq!(resolve(&u, text, "app.jar").unwrap(), main, "{text}");
        }
        let nested = Target::Type("com/ex/Main$Checker".into());
        for text in ["com.ex.Main$Checker", "com.ex.Main.Checker", "Main$Checker", "Main.Checker"] {
            assert_eq!(resolve(&u, text, "app.jar").unwrap(), nested, "{text}");
        }
    }

    #[test]
    fn methods_resolve_by_name_and_descriptor() {
        let mut u = universe();
        let check = Target::Members { owner: "com/ex/Main".into(), name: "check".into(), signature: None };
        for text in ["com.ex.Main.check", "com.ex.Main#check", "Main.check", "com/ex/Main.check"] {
            assert_eq!(resolve(&u, text, "app.jar").unwrap(), check, "{text}");
        }
        let add = resolve(&u, "com.ex.Main.add(II)I", "app.jar").unwrap();
        assert!(add.matches("com/ex/Main", "add", "(II)I"));
        assert!(!add.matches("com/ex/Main", "add", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"));
        assert!(resolve(&u, "com.ex.Main.<init>", "app.jar").is_ok());
        assert!(resolve(&u, "com.ex.Main.Checker.verify", "app.jar").is_ok());
        assert_eq!(
            resolve(&u, "Main.Checker.verify", "app.jar").unwrap(),
            Target::Members { owner: "com/ex/Main$Checker".into(), name: "verify".into(), signature: None }
        );
        u.own = Some("com/ex/Main".into());
        assert_eq!(resolve(&u, "check", "Main.class").unwrap(), check);
    }

    #[test]
    fn a_kotlin_property_name_points_at_its_accessors() {
        let types = vec![class(
            "pkg/TestKt",
            None,
            &[("getCount", "()I"), ("setCount", "(I)V"), ("isReady", "()Z"), ("run", "()V")],
        )];
        let u = Inventory::new(types, None, java_name);
        let e = resolve(&u, "TestKt.count", "kt.jar").unwrap_err();
        assert!(
            e.ends_with("a Kotlin property is selected by its JVM accessors: \"pkg.TestKt.getCount\", \"pkg.TestKt.setCount\""),
            "{e}"
        );
        let e = resolve(&u, "TestKt.ready", "kt.jar").unwrap_err();
        assert!(e.contains("\"pkg.TestKt.isReady\""), "{e}");
    }

    #[test]
    fn misses_suggest_the_nearest_name() {
        let u = universe();
        let e = resolve(&u, "com.ex.Mian", "app.jar").unwrap_err();
        assert!(e.contains("did you mean \"com.ex.Main\"?"), "{e}");
        let e = resolve(&u, "com.ex.Main.chek", "app.jar").unwrap_err();
        assert!(e.contains("did you mean \"com.ex.Main.check\"?"), "{e}");
        let e = resolve(&u, "com.ex.Main.add(J)J", "app.jar").unwrap_err();
        assert!(e.contains("its overloads are com.ex.Main.add(II)I"), "{e}");
        let e = resolve(&u, "zzzz", "app.jar").unwrap_err();
        assert!(e.contains("kuna functions app.jar"), "{e}");
    }
}
