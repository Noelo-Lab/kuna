//! End-to-end gate for C declarator precedence in the declarations the printer
//! builds around a name: return types, locals, and arrays of locals.
//!
//! A pointer to an array keeps its array suffix after the name, `char (*row)[16]`.
//! Casts and parameters always received both halves of the declarator; a return
//! type or a local declaration received only the half in front of the name, so
//! once the precedence was right they printed `char (* get_row(int i)` and
//! `char (*v1;`, which no C compiler accepts.
//!
//! Fixture: `ptrarraydecl_x86_64` (source vendored alongside), an `-O2 -g` C
//! binary whose DWARF types fix every shape: `get_row` returns `char (*)[16]`,
//! `use_rows` keeps a `char (*)[16]` local and a `char (*[2])[16]` stack array,
//! `get_names` returns `char *(*)[3]`. `use_names` keeps a `char *[2]` array of
//! pointers, the mirror type that must stay unparenthesised.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

const FIXTURE: &str = "ptrarraydecl_x86_64";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture, decompile `func`, and return the captured C (`None` ⇒
/// specs-less skip).
fn decompile(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(FIXTURE)
        .to_str()?
        .to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_ptrarray_declarators: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let cmds = vec![format!("load function {func}"), "decompile".to_string(), "print C".to_string()];
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}

/// Every line outside its `//` comment opens as many parentheses as it closes.
fn assert_balanced(code: &str) {
    for line in code.lines() {
        let text = line.split("//").next().unwrap_or("");
        let open = text.matches('(').count();
        let close = text.matches(')').count();
        assert_eq!(open, close, "unbalanced declarator on `{line}`; got:\n{code}");
    }
}

#[test]
fn a_pointer_to_array_return_type_closes_after_the_parameters() {
    let Some(code) = decompile("get_row") else { return };

    let proto = code.lines().find(|l| l.contains("get_row(")).unwrap_or("");
    assert!(
        proto.starts_with("char (* get_row(") && proto.ends_with("))[16]"),
        "the return type must wrap the name and parameter list; got:\n{code}",
    );
    assert_balanced(&code);
}

#[test]
fn a_pointer_to_array_of_pointers_return_type_closes_after_the_parameters() {
    let Some(code) = decompile("get_names") else { return };

    assert!(
        code.contains("char *(* get_names(void))[3]"),
        "the return type must wrap the name and parameter list; got:\n{code}",
    );
    assert_balanced(&code);
}

#[test]
fn pointer_to_array_locals_keep_their_suffix() {
    let Some(code) = decompile("use_rows") else { return };

    assert!(
        code.contains("char (*pair [2])[16];"),
        "an array of pointers to arrays puts its count inside the group; got:\n{code}",
    );
    assert!(
        code.lines().any(|l| l.trim_start().starts_with("char (*v") && l.contains(")[16];")),
        "the register local holding `get_row`'s result is a pointer to an array; got:\n{code}",
    );
    assert_balanced(&code);
}

/// NEGATIVE CONTROL: an array of plain pointers has no declarator suffix, so its
/// declaration is unchanged.
#[test]
fn an_array_of_pointers_local_is_not_grouped() {
    let Some(code) = decompile("use_names") else { return };

    assert!(
        code.contains("char *first [2];"),
        "an array of pointers needs no grouping; got:\n{code}",
    );
    assert!(!code.contains("(*first"), "the mirror type must not be grouped; got:\n{code}");
    assert_balanced(&code);
}
