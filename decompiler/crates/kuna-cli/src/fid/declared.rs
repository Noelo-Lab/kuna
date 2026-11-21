//! `kuna fid build --assert`: prototypes for objects that do not describe
//! their own functions.
//!
//! The value is an `--assert` directive or `@FILE` of them, in the syntax
//! `kuna decompile --assert` reads. Only `prototype <name> <declaration>` and
//! `typedef <declaration>` mean something to a database: a stated prototype
//! goes to every record of that name, over one read from the object's debug
//! information, and carries every stated typedef.

use std::collections::BTreeMap;

use kuna_analysis::fid::db::FidRecord;
use kuna_console::assertions::Body;

#[derive(Default)]
pub(super) struct Declared {
    prototypes: BTreeMap<String, String>,
    typedefs: Vec<String>,
}

impl Declared {
    pub(super) fn add(&mut self, value: &str) -> Result<(), String> {
        for directive in crate::assertdecl::parse_flag(value)? {
            match directive.body {
                Body::Prototype { func, decl } => {
                    self.prototypes.insert(func, one_line(decl.trim_end_matches(';')));
                }
                Body::Typedef { decl } => {
                    self.typedefs.push(format!("{};", one_line(decl.trim_end_matches(';'))));
                }
                _ => {
                    return Err(format!(
                        "--assert {value:?}: a database takes only prototype and typedef directives"
                    ))
                }
            }
        }
        Ok(())
    }

    /// Give each record a stated prototype; the names stated that no record has.
    pub(super) fn fill(&self, records: &mut [FidRecord]) -> Vec<&str> {
        for record in records.iter_mut() {
            if let Some(prototype) = self.prototypes.get(&record.name) {
                record.prototype = Some(prototype.clone());
                record.typedefs = self.typedefs.as_slice().into();
            }
        }
        self.prototypes
            .keys()
            .filter(|name| !records.iter().any(|r| &r.name == *name))
            .map(String::as_str)
            .collect()
    }
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(name: &str, prototype: Option<&str>) -> FidRecord {
        FidRecord {
            full_hash: 1,
            specific_hash: 2,
            code_unit_size: 4,
            specific_addl: 0,
            flags: 0,
            name: name.into(),
            prototype: prototype.map(str::to_owned),
            typedefs: Default::default(),
        }
    }

    #[test]
    fn a_stated_prototype_reaches_its_records_and_replaces_a_read_one() {
        let mut declared = Declared::default();
        declared.add("typedef struct hdr { unsigned int  magic; }").unwrap();
        declared.add("prototype parse int parse(hdr *out);").unwrap();
        declared.add("prototype absent void absent(void)").unwrap();
        assert!(declared.add("name v1 total").is_err());
        let mut records = [record("parse", Some("int parse(void *a)")), record("other", None)];
        assert_eq!(declared.fill(&mut records), ["absent"]);
        assert_eq!(records[0].prototype.as_deref(), Some("int parse(hdr *out)"));
        assert_eq!(*records[0].typedefs, ["struct hdr { unsigned int magic; };"]);
        assert_eq!(records[1], record("other", None));
    }
}
