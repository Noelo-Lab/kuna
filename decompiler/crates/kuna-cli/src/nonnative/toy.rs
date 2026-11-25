//! A toy managed engine, to hold the contract to an engine that is not the
//! JVM: it claims files that start with `TOY\n`, has two types, writes each as
//! fixed text and fails one member. The tests drive it through the same
//! generic code as the JVM engine.

use std::path::Path;

use kuna_managed::{
    Catalog, Engine, Inventory, Kind, MemberInfo, MemberSpan, Program, RunSettings, Runtime, Target, TypeInfo, Unit,
};

pub(crate) struct Toy;

pub(crate) static TOY: Toy = Toy;

pub(crate) fn kind() -> Kind {
    Kind { name: "toy file", single: false }
}

const BOX: &str = "type Box {\n  fn get() {\n    return 1\n  }\n  fn put(x) {\n    // could not decompile\n  }\n}\n";
const BAG: &str = "type Bag {\n  fn size() {\n    return 0\n  }\n}\n";

/// `(type, member, signature, error)`.
const MEMBERS: [(&str, &str, &str, Option<&str>); 3] =
    [("toy/Box", "get", "()I", None), ("toy/Box", "put", "(I)V", Some("bad stack")), ("toy/Bag", "size", "()I", None)];

impl Engine for Toy {
    fn name(&self) -> &'static str {
        "toy"
    }
    fn label(&self) -> &'static str {
        "Toy"
    }
    fn inputs(&self) -> &'static str {
        "toy files"
    }
    fn extension(&self) -> &'static str {
        "toy"
    }
    fn sniff(&self, path: &Path) -> bool {
        std::fs::read(path).is_ok_and(|bytes| bytes.starts_with(b"TOY\n"))
    }
    fn classify(&self, _path: &Path) -> Result<Kind, String> {
        Ok(kind())
    }
    fn open(&self, _path: &Path, _options: &[(String, String)]) -> Result<Box<dyn Program>, String> {
        let ty = |name: &str| TypeInfo {
            name: name.into(),
            size: 10,
            members: MEMBERS
                .iter()
                .filter(|m| m.0 == name)
                .map(|m| MemberInfo { name: m.1.into(), signature: m.2.into(), size: 4, ..MemberInfo::default() })
                .collect(),
            ..TypeInfo::default()
        };
        let types = vec![ty("toy/Box"), ty("toy/Bag")];
        Ok(Box::new(ToyProgram(Inventory::new(types, None, |n| n.replace('/', ".")))))
    }
    fn resolve(&self, name: &str, _value: &str) -> Result<(String, String), String> {
        Err(format!("unknown toy option {name}"))
    }
    fn language(&self, value: &str, input: &str, _options: &mut Vec<(String, String)>) -> Result<(), String> {
        match value {
            "auto" | "toy" => Ok(()),
            _ => Err(format!("--language {value}: {input}, which decompiles to toy")),
        }
    }
    fn options(&self) -> Catalog {
        Catalog { rows: Vec::new(), preamble: String::new(), sections: [("core", "", ""); 3], tail: "" }
    }
    fn check(&self) -> Vec<String> {
        Vec::new()
    }
    fn usage(&self, sub: &str) -> String {
        format!("usage: kuna {sub} <toy file>")
    }
    fn hidden(&self, target: &Target, _language: &str) -> String {
        format!("{} has no text", target.owner())
    }
}

struct ToyProgram(Inventory);

impl Program for ToyProgram {
    fn inventory(&self) -> &Inventory {
        &self.0
    }
    fn resolve(&self, text: &str) -> Result<Target, String> {
        let name = text.replace('.', "/");
        if self.0.get(&name).is_some() {
            return Ok(Target::Type(name));
        }
        match name.rsplit_once('/') {
            Some((owner, member)) if self.0.get(owner).is_some_and(|t| t.members.iter().any(|m| m.name == member)) => {
                Ok(Target::Members { owner: owner.into(), name: member.into(), signature: None })
            }
            _ => Err(format!("no type or member {text:?}")),
        }
    }
    fn prepare(&mut self, _libraries: &[String]) -> Result<Option<Runtime>, String> {
        Ok(None)
    }
    fn run(&self, settings: &RunSettings, deliver: &mut dyn FnMut(Unit)) -> Result<Vec<String>, String> {
        for name in ["toy/Box", "toy/Bag"].into_iter().filter(|n| settings.select.as_ref().is_none_or(|s| s.contains(*n))) {
            let text = if name == "toy/Box" { BOX } else { BAG };
            let line = |at: usize| text[..at].matches('\n').count() as u32 + 1;
            let members = MEMBERS
                .iter()
                .filter(|m| m.0 == name)
                .map(|&(owner, member, signature, error)| {
                    let start = text.find(&format!("  fn {member}")).unwrap_or(0);
                    let end = start + text[start..].find("  }\n").unwrap_or(0) + 4;
                    MemberSpan {
                        owner: owner.into(),
                        name: member.into(),
                        signature: signature.into(),
                        access: vec!["public"],
                        start,
                        end,
                        start_line: line(start),
                        end_line: line(end - 1),
                        error: error.map(Into::into),
                        lines: vec![(line(start) + 1, vec![0, 1])],
                        nested: Vec::new(),
                        warnings: Vec::new(),
                    }
                })
                .collect();
            let path = format!("{name}.toy");
            deliver(Unit { name: name.into(), path, language: "toy", text: Some(text.into()), members, ..Unit::default() });
        }
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::TOY;
    use crate::output::CAPTURED;

    /// `kuna <argv..>` through the toy engine alone: exit status and stdout.
    fn kuna(argv: &[&str]) -> (i32, String) {
        let rest: Vec<String> = argv[1..].iter().map(|s| s.to_string()).collect();
        CAPTURED.set(Some(Vec::new()));
        let status = super::super::dispatch(&[&TOY], argv[0], &rest).expect("the toy engine claims the input");
        (status, String::from_utf8(CAPTURED.take().unwrap_or_default()).unwrap())
    }

    fn json(text: &str) -> serde_json::Value {
        serde_json::from_str(text).expect("a JSON document")
    }

    #[test]
    fn the_generic_cli_drives_a_second_engine() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("x.toy");
        std::fs::write(&input, "TOY\n").unwrap();
        let input = input.to_str().unwrap();
        let no_limit = ["--max-fn-memory", "0"];

        let (status, listed) = kuna(&["functions", input]);
        assert_eq!((status, listed.as_str()), (0, "toy.Box.get()I\ntoy.Box.put(I)V\ntoy.Bag.size()I\n"));
        assert_eq!(kuna(&["functions", input, "--isa", "x86"]).0, 2, "a native flag is refused");

        let (status, get) = kuna(&["decompile", input, "toy.Box.get", no_limit[0], no_limit[1]]);
        assert_eq!((status, get.as_str()), (0, "fn get() {\n  return 1\n}\n"));
        let (status, put) = kuna(&["decompile", input, "toy.Box.put", no_limit[0], no_limit[1]]);
        assert_eq!(status, 1, "its only selected member failed: {put}");

        let (status, doc) = kuna(&["decompile", input, "toy.Box", "--json", no_limit[0], no_limit[1]]);
        let doc = json(&doc);
        assert_eq!(status, 0);
        assert_eq!((doc["engine"].as_str(), doc["language"].as_str(), doc["count"].as_u64()), (Some("toy"), Some("toy"), Some(2)));
        assert_eq!(doc["classes"][0]["status"], "decompiled");
        assert_eq!(doc["functions"][1]["error"], "bad stack");

        let (status, doc) = kuna(&["decompile-all", input, "--json", no_limit[0], no_limit[1]]);
        let doc = json(&doc);
        assert_eq!((status, doc["count"].as_u64()), (0, Some(3)));
        let names: Vec<&str> = doc["classes"].as_array().unwrap().iter().map(|c| c["name"].as_str().unwrap()).collect();
        assert_eq!(names, ["toy.Box", "toy.Bag"]);

        let out = dir.path().join("out");
        let (status, summary) = kuna(&["decompile-project", input, "-o", out.to_str().unwrap(), no_limit[0], no_limit[1]]);
        assert_eq!(status, 0);
        let bytes = super::BOX.len() + super::BAG.len();
        let tail = format!(": 2 .toy files ({bytes} bytes); classes: 2 ok, 0 failed; methods: 2 ok, 1 failed\n");
        assert!(summary.ends_with(&tail), "{summary}");
        assert_eq!(std::fs::read_to_string(out.join("toy/Bag.toy")).unwrap(), super::BAG);
    }
}
