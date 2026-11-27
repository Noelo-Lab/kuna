//! Android call facts shared by decompilation records, project indexes and xrefs.
use crate::jsonfmt::Json;
use kuna_decomp::kuna_androidcalls::{AndroidCalls, BinderService, JavaCall, JavaTarget};

fn hex(addr: u64) -> Json {
    Json::Str(format!("0x{addr:x}"))
}

pub(crate) fn java_target(target: &JavaTarget) -> Json {
    Json::Object(vec![
        ("key".into(), target.key().map_or(Json::Null, Json::Str)),
        (
            "class".into(),
            target.class.clone().map_or(Json::Null, Json::Str),
        ),
        ("name".into(), Json::Str(target.name.clone())),
        ("descriptor".into(), Json::Str(target.descriptor.clone())),
        ("lookup".into(), hex(target.lookup)),
    ])
}

pub(crate) fn java_call(call: &JavaCall) -> Json {
    Json::Object(vec![
        ("owner".into(), hex(call.owner)),
        ("site".into(), hex(call.site)),
        ("dispatch".into(), Json::Str(call.dispatch.clone())),
        ("form".into(), Json::Str(call.form.clone())),
        (
            "targets".into(),
            Json::Array(call.targets.iter().map(java_target).collect()),
        ),
        ("incomplete".into(), Json::Bool(call.incomplete)),
    ])
}

pub(crate) fn binder_service(service: &BinderService) -> Json {
    let transactions = service
        .transactions
        .iter()
        .map(|t| {
            Json::Object(vec![
                ("code".into(), Json::Number(t.code.to_string())),
                ("name".into(), t.name.clone().map_or(Json::Null, Json::Str)),
                ("site".into(), t.site.map_or(Json::Null, hex)),
            ])
        })
        .collect();
    Json::Object(vec![
        ("descriptor".into(), Json::Str(service.descriptor.clone())),
        ("define_site".into(), hex(service.define_site)),
        ("create".into(), hex(service.create)),
        ("destroy".into(), hex(service.destroy)),
        ("transact".into(), hex(service.transact)),
        ("transactions".into(), Json::Array(transactions)),
    ])
}

pub(crate) fn extend(fields: &mut Vec<(String, Json)>, facts: Option<&AndroidCalls>) {
    let Some(facts) = facts else { return };
    if !facts.java.is_empty() {
        fields.push((
            "java_calls".into(),
            Json::Array(facts.java.iter().map(java_call).collect()),
        ));
    }
    if !facts.binder.is_empty() {
        fields.push((
            "binder_services".into(),
            Json::Array(facts.binder.iter().map(binder_service).collect()),
        ));
    }
}

#[cfg(test)]
pub(crate) fn sample() -> Box<AndroidCalls> {
    Box::new(AndroidCalls::new(
        vec![JavaCall {
            owner: 0x401000,
            site: 0x401008,
            dispatch: "static".into(),
            form: "A".into(),
            targets: vec![JavaTarget {
                class: Some("test/Bridge".into()),
                name: "foo".into(),
                descriptor: "(I)V".into(),
                lookup: 0x401004,
            }],
            incomplete: true,
        }],
        vec![BinderService {
            descriptor: "test.IExample".into(),
            define_site: 0x401018,
            create: 0x402000,
            destroy: 0x403000,
            transact: 0x404000,
            transactions: vec![kuna_decomp::kuna_androidcalls::BinderTransaction {
                code: 7,
                name: None,
                site: Some(0x404014),
            }],
        }],
    ))
}
