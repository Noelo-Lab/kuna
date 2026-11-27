//! Symbolic Java references have a native instruction address and a JVM lookup key.
use super::{EntryNames, XrefArgs};
use crate::function_info::function_json;
use crate::jsonfmt::{dumps_indent2, Json};
use kuna_decomp::{
    architecture::Architecture,
    kuna_androidcalls::{AndroidCalls, JavaCall},
};
use std::fmt::Write;

pub(super) fn is_method(spec: &str) -> bool {
    spec.split_once("->").is_some_and(|(class, method)| {
        ((class.starts_with('L') && class.ends_with(';')) || class.starts_with('['))
            && method.contains('(')
            && method.contains(')')
    })
}

pub(super) fn from(calls: &AndroidCalls, address: u64, function: bool) -> Vec<&JavaCall> {
    if function {
        return calls.java_in(address).iter().collect();
    }
    calls
        .java
        .iter()
        .filter(|call| call.site == address)
        .collect()
}

pub(super) fn to<'a>(calls: &'a [JavaCall], key: &str) -> Vec<&'a JavaCall> {
    calls
        .iter()
        .filter(|call| {
            call.targets
                .iter()
                .any(|target| target.key().as_deref() == Some(key))
        })
        .collect()
}

fn owner(names: &EntryNames, address: u64) -> String {
    names
        .at(address)
        .map(|(_, name)| name.to_owned())
        .unwrap_or_else(|| format!("0x{address:x}"))
}

fn json_row(call: &JavaCall, names: &EntryNames) -> Json {
    let Json::Object(mut fields) = crate::android_calls::java_call(call) else {
        unreachable!()
    };
    fields.push(("kind".into(), Json::Str("java-call".into())));
    fields.push((
        "from_function".into(),
        function_json(&owner(names, call.owner), call.owner),
    ));
    Json::Object(fields)
}

pub(super) fn fields(calls: &[&JavaCall], names: &EntryNames) -> Vec<(String, Json)> {
    vec![
        (
            "java_refs_count".into(),
            Json::Number(calls.len().to_string()),
        ),
        (
            "java_refs".into(),
            Json::Array(calls.iter().map(|call| json_row(call, names)).collect()),
        ),
    ]
}

pub(super) fn text_rows(calls: &[&JavaCall], names: &EntryNames) -> String {
    let mut text = String::new();
    for call in calls {
        let targets: Vec<_> = call
            .targets
            .iter()
            .map(|target| {
                let key = target.key().unwrap_or_else(|| target.display());
                format!("{key} @lookup 0x{:x}", target.lookup)
            })
            .collect();
        let partial = if call.incomplete { " (incomplete)" } else { "" };
        let _ = writeln!(
            text,
            "0x{:x}\tjava-call\t{}\t{} {}\t{}{partial}",
            call.site,
            owner(names, call.owner),
            call.dispatch,
            call.form,
            targets.join(" | ")
        );
    }
    text
}

pub(super) fn report(
    args: &XrefArgs,
    arch: &Architecture,
    names: &EntryNames,
    target: Json,
    calls: &[&JavaCall],
) -> String {
    let enabled = arch.android_jni && arch.analysis_jnicalls;
    let limited = arch.android_calls.java_limited;
    if args.json {
        let doc = Json::Object(vec![
            ("binary".into(), Json::Str(args.binary.clone())),
            ("target".into(), target),
            (
                "direction".into(),
                Json::Str(args.direction.as_str().into()),
            ),
            ("count".into(), Json::Number(calls.len().to_string())),
            (
                "coverage".into(),
                Json::Object(vec![
                    ("scope".into(), Json::Str("elf-jni-entry-points".into())),
                    ("enabled".into(), Json::Bool(enabled)),
                    ("limited".into(), Json::Bool(limited)),
                ]),
            ),
            (
                "xrefs".into(),
                Json::Array(calls.iter().map(|call| json_row(call, names)).collect()),
            ),
        ]);
        format!("{}\n", dumps_indent2(&doc))
    } else {
        let coverage = if !enabled {
            " (JNI call analysis inactive)"
        } else if limited {
            " (partial JNI analysis)"
        } else {
            ""
        };
        let noun = if calls.len() == 1 { "site" } else { "sites" };
        format!(
            "# {} Java call {noun} {} {}{coverage}\n{}",
            calls.len(),
            args.direction.as_str(),
            args.spec,
            text_rows(calls, names)
        )
    }
}
