//! Apply NDK callback prototypes to the native addresses proven by a Binder class definition.
use crate::grammar::{parse_c, parse_protopieces, DataOrg};
use kuna_analysis::pass::{AnalysisOutput, FidMatch};
use kuna_base::error::KunaResult;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_androidcalls::BinderService;
use std::collections::{BTreeMap, BTreeSet};

const TYPES: &[(&str, &str)] = &[
    ("AIBinder", "typedef struct AIBinder AIBinder;"),
    ("AParcel", "typedef struct AParcel AParcel;"),
    ("AIBinder_Class", "typedef struct AIBinder_Class AIBinder_Class;"),
    ("binder_status_t", "typedef int binder_status_t;"),
    ("transaction_code_t", "typedef unsigned int transaction_code_t;"),
    ("AIBinder_Class_onCreate", "typedef void *(*AIBinder_Class_onCreate)(void *);"),
    ("AIBinder_Class_onDestroy", "typedef void (*AIBinder_Class_onDestroy)(void *);"),
    ("AIBinder_Class_onTransact", "typedef binder_status_t (*AIBinder_Class_onTransact)(AIBinder *, transaction_code_t, const AParcel *, AParcel *);"),
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Callback {
    Create,
    Destroy,
    Transact,
}

impl Callback {
    fn name(self) -> &'static str {
        match self {
            Self::Create => "onCreate",
            Self::Destroy => "onDestroy",
            Self::Transact => "onTransact",
        }
    }

    fn declaration(self, name: &str) -> String {
        match self {
            Self::Create => format!("void *{name}(void *args);"),
            Self::Destroy => format!("void {name}(void *userData);"),
            Self::Transact => format!("binder_status_t {name}(AIBinder *binder, transaction_code_t code, const AParcel *in, AParcel *out);"),
        }
    }
}

pub(crate) fn bind(
    arch: &Architecture,
    file: &object::File,
    bytes: &[u8],
    services: &[BinderService],
    out: &mut AnalysisOutput,
) -> KunaResult<()> {
    let (addr_size, word_size) = arch.data_org();
    let org = DataOrg {
        addr_size,
        word_size,
    };
    let types = arch.types();
    for &(name, declaration) in TYPES {
        if types.find_by_name(name)?.is_none() {
            parse_c(declaration, types, org, &[], |_, _| Ok(()))?;
        }
    }
    let mut roles = BTreeMap::<u64, BTreeSet<(Callback, &str)>>::new();
    for service in services {
        for (addr, role) in [
            (service.create, Callback::Create),
            (service.destroy, Callback::Destroy),
            (service.transact, Callback::Transact),
        ] {
            roles
                .entry(addr)
                .or_default()
                .insert((role, &service.descriptor));
        }
    }
    let mut prototypes = Vec::new();
    for (addr, bindings) in roles {
        let &(role, descriptor) = bindings.first().unwrap();
        if !bindings.iter().all(|&(other, _)| other == role) {
            continue;
        }
        out.entries.push(addr);
        let name = if bindings.len() == 1 {
            let identifier: String = descriptor
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            let name = format!("{identifier}_{}", role.name());
            out.fallback_names.push(FidMatch {
                addr,
                name: name.clone(),
                by_signature: false,
            });
            name
        } else {
            role.name().to_owned()
        };
        prototypes.push((
            addr,
            parse_protopieces(&role.declaration(&name), types, org)?,
        ));
    }
    for import in kuna_analysis::loader::format::resolve_imports(file, bytes) {
        let declaration = match import.name.as_slice() {
            b"AIBinder_Class_define" => "AIBinder_Class *AIBinder_Class_define(const char *interfaceDescriptor, AIBinder_Class_onCreate onCreate, AIBinder_Class_onDestroy onDestroy, AIBinder_Class_onTransact onTransact);",
            b"AIBinder_Class_setTransactionCodeToFunctionNameMap" if addr_size == 8 => "void AIBinder_Class_setTransactionCodeToFunctionNameMap(AIBinder_Class *clazz, const char *const *names, unsigned long length);",
            b"AIBinder_Class_setTransactionCodeToFunctionNameMap" => "void AIBinder_Class_setTransactionCodeToFunctionNameMap(AIBinder_Class *clazz, const char *const *names, unsigned int length);",
            b"AIBinder_getUserData" => "void *AIBinder_getUserData(AIBinder *binder);",
            b"AParcel_readInt32" => "binder_status_t AParcel_readInt32(const AParcel *parcel, int *value);",
            b"AParcel_writeInt32" => "binder_status_t AParcel_writeInt32(AParcel *parcel, int value);",
            _ => continue,
        };
        prototypes.push((import.addr, parse_protopieces(declaration, types, org)?));
    }
    out.prototypes_at.splice(..0, prototypes);
    Ok(())
}
