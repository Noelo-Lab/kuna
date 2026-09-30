//! Several `name`/`type` directives in one run, each read against the output the
//! run printed before any of them applied (issue #784).
//!
//! Naming one register local used to empty the pass the next directive resolves
//! against, so of two independent renames the second always answered `No symbol
//! named:`, in either order.  Every case runs on both surfaces: the text one
//! drives `decomp_dbg` through a console script, `--json` runs in-process.
//!
//! Fixture: `kuna-analysis/tests/fixtures/localnamebatch_x86_64` (source beside
//! it): two and three independent register locals, and two stack locals.

mod common;

/// The decompiled C and the rejected directives of one run, from the text
/// surface and from `--json`.  Both must agree before either is returned.
fn decompile(function: &str, directives: &[&str]) -> (String, Vec<String>) {
    let binary = common::fixture("localnamebatch_x86_64");
    let mut args = vec!["decompile", &binary, function, "--mode", "reliable"];
    for directive in directives {
        args.extend(["--assert", directive]);
    }
    let (text, stderr, _) = common::run_kuna(&args);
    let text_rejected: Vec<String> = stderr
        .lines()
        .filter(|line| line.contains("rejected"))
        .map(str::to_string)
        .collect();
    args.push("--json");
    let (doc, stderr, _) = common::run_kuna(&args);
    let parsed: serde_json::Value =
        serde_json::from_str(&doc).unwrap_or_else(|e| panic!("{e}: {stderr}\n{doc}"));
    let code = parsed
        .pointer("/functions/0/code")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("no code: {stderr}\n{doc}"))
        .to_string();
    let rejected: Vec<String> = parsed["assertions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["status"] != "applied")
        .map(|a| format!("{}: {}", a["directive"], a["detail"]))
        .collect();
    assert_eq!(
        text.trim_end(),
        code.trim_end(),
        "the two surfaces disagree"
    );
    assert_eq!(
        text_rejected.len(),
        rejected.len(),
        "{text_rejected:?} vs {rejected:?}"
    );
    (code, rejected)
}

fn applies(function: &str, directives: &[&str], declarations: &[&str]) -> String {
    let (code, rejected) = decompile(function, directives);
    assert!(
        rejected.is_empty(),
        "{directives:?} rejected {rejected:?}:\n{code}"
    );
    for decl in declarations {
        assert!(code.contains(decl), "{directives:?}: no `{decl}`:\n{code}");
    }
    code
}

/// The report's own case: two register locals named after `observe`'s
/// parameters, each renamed independently, in both orders.
#[test]
fn two_register_local_renames_apply_in_either_order() {
    let prototypes = [
        "prototype two_locals int two_locals(void)",
        "prototype observe void observe(int index, int sum)",
    ];
    let index = "name two_locals::index iteration";
    let sum = "name two_locals::sum accumulator";
    for order in [[index, sum], [sum, index]] {
        let directives: Vec<&str> = prototypes.iter().copied().chain(order).collect();
        let code = applies(
            "two_locals",
            &directives,
            &[
                "int4 iteration; // ebx",
                "int4 accumulator; // r12d",
                "observe(iteration,accumulator);",
            ],
        );
        assert!(!code.contains("index") && !code.contains("sum"), "{code}");
    }
}

#[test]
fn three_register_local_renames_apply_in_any_order() {
    let expected = [
        "int4 a; // ebx",
        "int4 b; // r12d",
        "int4 c; // r13d",
        "observe3(a,b,c);",
    ];
    applies(
        "three_locals",
        &["name v1 a", "name v2 b", "name v3 c"],
        &expected,
    );
    applies(
        "three_locals",
        &["name v3 c", "name v1 a", "name v2 b"],
        &expected,
    );
}

#[test]
fn stack_local_renames_apply_in_either_order() {
    let expected = ["int4 first; // stack - 0x10", "fill(&first,second);"];
    applies(
        "stack_pair",
        &["name v1 first", "name v2 second"],
        &expected,
    );
    applies(
        "stack_pair",
        &["name v2 second", "name v1 first"],
        &expected,
    );
}

/// `name a b` with `name b a` names the two variables the caller saw.  The
/// second directive used to find the Symbol the first had just created and
/// rename it back, reporting both `applied` over unchanged output.
#[test]
fn a_swap_exchanges_the_two_names() {
    applies(
        "two_locals",
        &["name v1 v2", "name v2 v1"],
        &["int4 v2; // ebx", "int4 v1; // r12d"],
    );
    applies(
        "stack_pair",
        &["name v1 v2", "name v2 v1"],
        &["int4 v2; // stack - 0x10", "fill(&v2,v1);"],
    );
}

/// A later directive may name a local by what the pass printed or by what an
/// earlier directive renamed it to, and a `type` stated after a `name` keeps it.
#[test]
fn a_renamed_local_answers_to_either_name() {
    for second in ["type v1 unsigned int", "type i unsigned int"] {
        applies(
            "two_locals",
            &["name v1 i", second],
            &["uint4 i; // ebx", "i = 0;"],
        );
        applies(
            "stack_pair",
            &["name v1 i", second],
            &["uint4 i; // stack - 0x10", "fill(&i,"],
        );
    }
    applies(
        "two_locals",
        &["type v1 unsigned int", "name v1 i"],
        &["uint4 i; // ebx"],
    );
}

/// A printed name wins over a name an earlier directive gave: after `name v1 v2`
/// the identifier `v2` still means the variable printed as `v2`.
#[test]
fn a_printed_name_outranks_a_name_given_in_the_batch() {
    applies(
        "two_locals",
        &["name v1 v2", "type v2 unsigned int s"],
        &["int4 v2; // ebx", "uint4 s; // r12d"],
    );
}

/// A register-backed Symbol a directive created keeps its storage width for a
/// later `type`, and a name two directives gave is ambiguous rather than a pick.
#[test]
fn later_directives_keep_the_rejections() {
    let (_, rejected) = decompile("two_locals", &["name v1 i", "type i char *"]);
    assert_eq!(
        rejected,
        ["\"type i char *\": \"Storage is 4 bytes, the stated type is 8\""]
    );
    let (_, rejected) = decompile(
        "two_locals",
        &["name v1 x", "name v2 x", "type x unsigned int"],
    );
    assert_eq!(
        rejected,
        ["\"type x unsigned int\": \"More than one symbol named: x (2)\""]
    );
}

/// `pick_flag` holds a pointer in rax and then an int in eax.  Given a Symbol
/// each, the second pass folded them into one variable (`text._0_4_ = 0`), so
/// the later directive is rejected and names the one it lost to.
#[test]
fn overlapping_register_locals_reject_the_later_directive() {
    let prototypes = [
        "prototype lookup char *lookup(void)",
        "prototype probe int probe(char *s)",
        "prototype pick_flag int pick_flag(void)",
    ];
    let text = "name s text";
    let flag = "name v1 flag";
    for (order, applied, rejected_detail) in [
        ([text, flag], "char *text; // rax", "\"name v1 flag\": \"Storage of v1 overlaps s, which an earlier directive already changed\""),
        ([flag, text], "uint4 flag; // eax", "\"name s text\": \"Storage of s overlaps v1, which an earlier directive already changed\""),
    ] {
        let directives: Vec<&str> = prototypes.iter().copied().chain(order).collect();
        let (code, rejected) = decompile("pick_flag", &directives);
        assert_eq!(rejected, [rejected_detail], "{code}");
        assert!(code.contains(applied), "{code}");
        assert!(!code.contains("_0_4_"), "two locals were folded into one:\n{code}");
    }
}
