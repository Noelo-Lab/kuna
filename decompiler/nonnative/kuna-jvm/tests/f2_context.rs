//! The F2 golden rows that consulted the decompiler context in Java
//! (`data/f2/GenF2Golden.java`: an empty struct context, a collecting logger,
//! optionally a pool interceptor), replayed inside a real `DecompilerContext`.
//! A trailing `!` on a Java value only records that the context was consulted.

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::code_constants::CONSTANT_NAME_AND_TYPE;
use kuna_jvm::constant_pool::ConstantPool;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::generic_main::GenericMain;
use kuna_jvm::i_fernflower_logger::CollectingLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::pool_interceptor::PoolInterceptor;
use kuna_jvm::pooled_constant::PooledConstant;

use crate::common::{self, ContextGuard};
use crate::f2_pool_match::{dump_const, unhex};
use crate::f2_types::{
    check_generic_ops, check_lattice, dump, dump_class, dump_method, esc, lines, run, same, unesc,
    verify_class, verify_field, verify_method, Checker,
};

fn install() -> ContextGuard {
    common::install_context_with(&[(IFernflowerPreferences::LOG_LEVEL, "trace")])
}

fn strip(s: &str) -> &str {
    s.strip_suffix('!').unwrap_or(s)
}

/// The messages logged since the last call, as the generator prints them.
fn take_log(logger: &CollectingLogger) -> String {
    let records = std::mem::take(&mut *logger.records().borrow_mut());
    let msgs: Vec<String> = records
        .iter()
        .map(|r| format!("{}: {}", r.severity.name(), r.message))
        .collect();
    esc(Some(&msgs.join("|")))
}

#[test]
fn lattice_and_generic_ops_in_empty_context() {
    let _guard = install();
    let mut c = Checker::new("lattice in context");
    for l in lines(include_str!("data/f2/lattice.tsv"), "LA") {
        let stripped: Vec<&str> = l.iter().map(|f| strip(f)).collect();
        check_lattice(&mut c, &stripped);
    }
    c.finish();
    let mut c = Checker::new("generic ops in context");
    for l in lines(include_str!("data/f2/generic_ops.tsv"), "GO") {
        let stripped: Vec<&str> = l.iter().map(|f| strip(f)).collect();
        check_generic_ops(&mut c, &stripped);
    }
    c.finish();
}

#[test]
fn rejected_signatures_log_like_vineflower() {
    let guard = install();
    let logger = guard.logger();
    take_log(&logger);
    let mut n = 0;
    for line in include_str!("data/f2/signatures.tsv").lines() {
        let l: Vec<&str> = line.split('\t').collect();
        let (r, java_log) = match l[0] {
            "CS" if l[3] == "\\N" => (
                GenericMain::parse_class_signature(&unesc(l[1]).unwrap(), &unesc(l[2]).unwrap())
                    .map(|d| dump_class(&d)),
                l[4],
            ),
            "MS" if l[2] == "\\N" => (
                GenericMain::parse_method_signature(&unesc(l[1]).unwrap()).map(|d| dump_method(&d)),
                l[3],
            ),
            "FS" if l[2] == "\\N" => (
                GenericMain::parse_field_signature(&unesc(l[1]).unwrap())
                    .map(|d| dump(Some(&d.type_()))),
                l[3],
            ),
            _ => {
                take_log(&logger);
                continue;
            }
        };
        assert!(r.is_none(), "{line}");
        assert_eq!(take_log(&logger), java_log, "{line}");
        n += 1;
    }
    eprintln!("rejected signatures with matching log: {n}");
    assert!(n > 0);
}

#[test]
fn pool_interceptor_renames_like_vineflower() {
    let guard = install();
    let interceptor = PoolInterceptor::new();
    interceptor.add_name("java/lang/String", "a/S");
    interceptor.add_name("java/util/List", "b/L");
    interceptor.add_name(
        "java/lang/String valueOf (I)Ljava/lang/String;",
        "a/S vo (I)La/S;",
    );
    interceptor.add_name(
        "java/lang/String CASE_INSENSITIVE_ORDER Ljava/util/Comparator;",
        "a/S cio Ljava/util/Comparator;",
    );
    interceptor.add_name("java/lang/Object", "o/O");
    guard
        .context()
        .with_mut(|d| d.pool_interceptor = Some(interceptor));
    assert!(DecompilerContext::get_pool_interceptor().is_some());

    let pools = include_str!("data/f2/pools.tsv");
    let po = lines(pools, "PO");
    let mut c = Checker::new("pool interceptor");
    for name in ["kitchen-sink", "PoolSample.class"] {
        let bytes = unhex(po.iter().find(|l| l[1] == name).unwrap()[2]);
        let pool = ConstantPool::new(&mut ByteCursor::new(bytes[8..].to_vec()));
        for l in lines(pools, "PI").into_iter().filter(|l| l[1] == name) {
            let idx: i32 = l[2].parse().unwrap();
            let entry = pool.get_constant(idx).unwrap();
            let what = format!("PI {name} {idx}");
            match &entry {
                PooledConstant::Primitive(_) => c.field(&what, l[3], || {
                    dump_const(pool.get_primitive_constant(idx).map(Into::into).as_ref())
                }),
                PooledConstant::Link(nt) => {
                    c.field(&what, l[3], || {
                        dump_const(pool.get_link_constant(idx).map(Into::into).as_ref())
                    });
                    if nt.type_ == CONSTANT_NAME_AND_TYPE {
                        let mut f = 4;
                        for cn in ["a/S", "java/lang/String", "x/Y"] {
                            for et in [ConstantPool::METHOD, ConstantPool::FIELD] {
                                c.field(&format!("CE {name} {idx} {cn} {et}"), l[f], || {
                                    let element =
                                        pool.get_class_element(et, cn, nt.index1, nt.index2);
                                    esc(Some(&element.join(" ")))
                                });
                                f += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    c.finish();
}

/// `result[!]` or `result[!] log`: the Java field without its context flag.
fn unflag(java: &str) -> String {
    match java.find(" WARN: ") {
        Some(p) => format!("{}{}", strip(&java[..p]), &java[p..]),
        None => strip(java).to_string(),
    }
}

#[test]
fn verify_rows_log_like_vineflower() {
    let guard = install();
    let logger = guard.logger();
    take_log(&logger);
    let (mut n, mut logged) = (0, 0);
    let mut failures = Vec::new();
    for line in include_str!("data/f2/verify.tsv").lines() {
        let l: Vec<&str> = line.split('\t').collect();
        let mut check = |java: String, rust: String| {
            n += 1;
            if !same(&java, &rust) {
                failures.push(format!("{line}\n   java: {java}\n   rust: {rust}"));
            }
        };
        match l[0] {
            "VM" | "VC" => {
                let r = run(|| {
                    if l[0] == "VM" {
                        verify_method(&l)
                    } else {
                        verify_class(&l)
                    }
                });
                let log = take_log(&logger);
                let java_log = l.get(6).copied().unwrap_or("");
                if !java_log.is_empty() {
                    logged += 1;
                }
                check(
                    format!("{}\t{}", strip(l[5]), java_log),
                    format!("{r}\t{log}"),
                );
            }
            "VF" => {
                for (i, strict) in [(5, false), (6, true)] {
                    let r = run(|| verify_field(&l, strict));
                    let log = take_log(&logger);
                    let java = unflag(l[i]);
                    if java.contains(" WARN: ") {
                        logged += 1;
                    }
                    check(
                        java,
                        if log.is_empty() {
                            r
                        } else {
                            format!("{r} {log}")
                        },
                    );
                }
            }
            _ => continue,
        }
    }
    eprintln!("verify rows in context: {n} checked, {logged} with a logged mismatch");
    assert!(logged > 0);
    assert!(
        failures.is_empty(),
        "{} mismatches:\n{}",
        failures.len(),
        failures
            .iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}
