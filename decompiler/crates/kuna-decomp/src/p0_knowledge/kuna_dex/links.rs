//! (kuna `dexmodel`) Method references resolved across every DEX file of an APK.
//!
//! An `invoke` names its method by an index into its own file's `method_ids`:
//! the declaring class, the name and the prototype, all as strings. The method
//! it runs is defined in whichever DEX file of the APK holds that class (or the
//! nearest superclass that declares it), at that method's first instruction.
//! [`MethodIndex`] resolves a reference to that address, and [`calls`] lists
//! every `invoke` of the loaded files with the method it reaches, which is the
//! call graph `kuna xrefs` answers from for DEX and APK input.

use std::collections::HashMap;
use std::rc::Rc;

use super::format::java_type_name;
use super::DexModel;

/// `(class descriptor, method name, prototype descriptor)`.
type MethodKey = (String, String, String);

/// Every defined method of the loaded DEX files, by its key, and every class's
/// superclass.
pub struct MethodIndex {
    methods: HashMap<MethodKey, u64>,
    superclass: HashMap<String, String>,
}

impl MethodIndex {
    pub fn new(models: &[Rc<DexModel>]) -> MethodIndex {
        let mut methods = HashMap::new();
        let mut superclass = HashMap::new();
        for model in models {
            for class in &model.file.classes {
                let desc = model.descriptor(class.class_idx);
                if class.superclass_idx != super::format::NO_INDEX {
                    superclass.entry(desc.clone()).or_insert_with(|| model.descriptor(class.superclass_idx));
                }
                for method in &class.methods {
                    let (Some(code), Some(key)) = (method.code, model.method_key(method.method_idx)) else { continue };
                    methods.entry(key).or_insert(model.base + u64::from(code.insns_off));
                }
            }
        }
        MethodIndex { methods, superclass }
    }

    /// The code address the method `key` runs: its own class's definition, or
    /// the nearest superclass's.
    pub fn resolve(&self, key: &MethodKey) -> Option<u64> {
        let (mut class, name, proto) = key.clone();
        for _ in 0..64 {
            if let Some(&addr) = self.methods.get(&(class.clone(), name.clone(), proto.clone())) {
                return Some(addr);
            }
            class = self.superclass.get(&class)?.clone();
        }
        None
    }
}

/// One `invoke`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DexCall {
    /// Address of the `invoke` instruction.
    pub site: u64,
    /// Entry of the method the instruction is in.
    pub caller: u64,
    /// Entry of the method it reaches, when a loaded file defines it.
    pub callee: Option<u64>,
    /// The method it names, `pkg::Class::name`.
    pub callee_name: String,
    /// `invoke-virtual pkg.Class.name(desc)`.
    pub instruction: String,
}

fn invoke_mnemonic(op: u8) -> Option<&'static str> {
    Some(match op {
        0x6e => "invoke-virtual",
        0x6f => "invoke-super",
        0x70 => "invoke-direct",
        0x71 => "invoke-static",
        0x72 => "invoke-interface",
        0x74 => "invoke-virtual/range",
        0x75 => "invoke-super/range",
        0x76 => "invoke-direct/range",
        0x77 => "invoke-static/range",
        0x78 => "invoke-interface/range",
        0xfa => "invoke-polymorphic",
        0xfb => "invoke-polymorphic/range",
        _ => return None,
    })
}

/// Every `invoke` in the code of the loaded DEX files, in address order.
pub fn calls(models: &[Rc<DexModel>]) -> Vec<DexCall> {
    let index = MethodIndex::new(models);
    let mut out = Vec::new();
    for model in models {
        for class in &model.file.classes {
            for method in &class.methods {
                let Some(code) = method.code else { continue };
                let entry = model.base + u64::from(code.insns_off);
                let end = entry + 2 * u64::from(code.insns_units);
                let mut at = entry;
                while at < end {
                    let Some(size) = model.instruction_size(at).filter(|&s| s > 0) else { break };
                    let mnemonic = model.bytes_at(at, 1).and_then(|b| invoke_mnemonic(b[0]));
                    if let (Some(mnemonic), Some(b)) = (mnemonic, model.bytes_at(at + 2, 2)) {
                        let method_idx = u32::from(u16::from_le_bytes([b[0], b[1]]));
                        if let Some(key) = model.method_key(method_idx) {
                            let class = java_type_name(&key.0);
                            out.push(DexCall {
                                site: at,
                                caller: entry,
                                callee: index.resolve(&key),
                                callee_name: format!("{}::{}", class.replace('.', "::"), key.1),
                                instruction: format!("{mnemonic} {class}.{}{}", key.1, key.2),
                            });
                        }
                    }
                    at += size;
                }
            }
        }
    }
    out.sort_by_key(|c| c.site);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_calls_into_the_same_file() {
        let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../tests/stages/fixtures/dex/hello.dex"));
        let model = Rc::new(DexModel::parse(bytes.to_vec(), 0).unwrap());
        let functions = model.functions();
        let many = functions.iter().find(|f| f.name == "demo::Hello::many").unwrap().addr;
        let all = calls(&[Rc::clone(&model)]);
        assert!(all.iter().any(|c| c.callee == Some(many) && c.instruction.starts_with("invoke-static/range")));
        let println = all.iter().find(|c| c.callee_name == "java::io::PrintStream::println").unwrap();
        assert_eq!(println.callee, None);
    }
}
