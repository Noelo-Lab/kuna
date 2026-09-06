//! The `--assert` override plane end-to-end — `docs/re-needs/no-cli-rename-or-prototype-override.md`.
//!
//! `rename`, `retype`, `map param`, `map return`, `map address`, `comment
//! instruction` and `parse line extern` all work in the console, and none of
//! them was reachable from the `kuna` binary. This drives the in-process path
//! `kuna decompile --json` / `decompile-all` take, and asserts for every
//! directive that **the emitted C changed** — not that the command returned Ok.
//!
//! That distinction is the whole point. `override prototype` has printed
//! "Successfully added override" and changed nothing since it was ported, and it
//! got there by being reviewed on its return value. A directive that is accepted
//! and inert is worse than one that errors, because an agent cannot tell.
//!
//! Fixture: `kuna-analysis/tests/fixtures/fauxware` — a small unstripped x86-64
//! ELF whose `authenticate` has an 8-byte stack buffer (`v2`), two pointer
//! parameters and a call to a named global (`sneaky`), so every directive has
//! something observable to move.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::assertions::{self, Body, Directive, Outcome};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::project::decompile_targets;

const TARGET: &str = "authenticate";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture and run the analysis commit.  `None` ⇒ specs-less skip.
fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_assertplane: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit");
    Some(prog)
}

fn directive(spec: &str, body: Body) -> Directive {
    Directive { raw: spec.to_string(), body }
}

/// Decompile `authenticate` under `directives`, returning `(C, report)`.
fn decompile_with(directives: Vec<Directive>) -> Option<(String, Vec<Outcome>)> {
    let mut prog = load()?;
    if !directives.is_empty() {
        prog.set_assertions(directives);
        assertions::apply_program_scoped(&mut prog);
    }
    let entry = prog
        .resolve_entry(&EntrySelector::Name(TARGET.to_string()))
        .expect("fauxware has an `authenticate`");
    let funcs = decompile_targets(&mut prog, vec![entry], false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    Some((code, prog.assertion_outcomes()))
}

/// Every outcome is `applied`; panics naming the offender otherwise.
fn all_applied(report: &[Outcome]) {
    for outcome in report {
        assert_eq!(
            outcome.status, "applied",
            "{:?} was rejected: {:?}",
            outcome.directive, outcome.detail
        );
    }
}

/// The un-asserted baseline every case below is measured against.
#[test]
fn the_baseline_names_nothing_the_directives_name() {
    let Some((code, report)) = decompile_with(Vec::new()) else { return };
    assert!(report.is_empty(), "no directives ⇒ no report rows");
    assert!(code.contains("char v2 [8]"), "baseline lost its 8-byte buffer:\n{code}");
    assert!(!code.contains("credbuf"), "baseline already names credbuf:\n{code}");
    assert!(code.contains("sneaky"), "baseline lost the named global:\n{code}");
}

/// `prototype` + `type` + `name` — the acceptance probe's three directives, and
/// the need's own headline: an agent states the signature, a local's type and a
/// local's name in one invocation and all three land in the C.
#[test]
fn prototype_type_and_name_all_reach_the_emitted_c() {
    let Some((code, report)) = decompile_with(vec![
        directive(
            "prototype authenticate int4 authenticate(char *user,char *pass)",
            Body::Prototype {
                func: TARGET.into(),
                decl: "int4 authenticate(char *user,char *pass)".into(),
            },
        ),
        directive(
            "type v2 char[16]",
            Body::Type { func: None, symbol: "v2".into(), decl: "char[16]".into() },
        ),
        directive(
            "name v2 credbuf",
            Body::Name { func: None, symbol: "v2".into(), newname: "credbuf".into() },
        ),
    ]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.contains("authenticate(char *user,char *pass)"),
        "the declared signature did not reach the C:\n{code}"
    );
    assert!(code.contains("char credbuf [16];"), "the retype+rename did not land:\n{code}");
    assert!(!code.contains("char v2 [8]"), "the original buffer survived:\n{code}");
}

/// A directive is applied in the order it was given: `type` then `name` retypes
/// and then renames, where the reverse order leaves the second naming a symbol
/// the first already renamed away.  The rejection is reported, not swallowed.
#[test]
fn directive_order_is_the_callers_order_and_a_miss_is_reported() {
    let Some((code, report)) = decompile_with(vec![
        directive(
            "name v2 credbuf",
            Body::Name { func: None, symbol: "v2".into(), newname: "credbuf".into() },
        ),
        directive(
            "type v2 char[16]",
            Body::Type { func: None, symbol: "v2".into(), decl: "char[16]".into() },
        ),
    ]) else {
        return;
    };
    assert_eq!(report[0].status, "applied");
    assert_eq!(report[1].status, "rejected");
    assert_eq!(report[1].detail.as_deref(), Some("No symbol named: v2"));
    // The rename still took, so the run is not all-or-nothing: an agent batching
    // forty renames against a re-decompiled binary does not lose the other 39.
    assert!(code.contains("credbuf"), "the applied half was rolled back:\n{code}");
}

/// `param` — a locked input storage and name (`map param`).
#[test]
fn param_locks_the_input_storage_and_name() {
    let Some((code, report)) = decompile_with(vec![directive(
        "param 0 %RDI char *username",
        Body::Param {
            func: None,
            index: 0,
            storage: "%RDI".into(),
            decl: "char *username".into(),
        },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.contains("authenticate(char *username)"),
        "the locked parameter did not reach the signature:\n{code}"
    );
}

/// A `param` QUALIFIED with another function declares that function's
/// prototype, and the effect shows up at the CALL SITE
/// (`docs/re-needs/qualified-parameter-assertions-modify.md`).
///
/// Before this the qualifier was dropped on the way to the console, so
/// `param callee::0 ...` renamed and retyped the CALLER's inputs while the
/// callee kept its empty argument list.  Both halves are asserted here: the
/// declared name must not land on the caller, and the storage the directive
/// names must be the storage the argument is read from — `%RDI` gives
/// `open(a0)` and `%RSI`, which holds the mode operand, does not.  Same slot,
/// same type: only the declared storage differs, so a lowering that dropped it
/// could not tell the two runs apart.
#[test]
fn a_qualified_param_declares_the_callee_and_not_the_caller() {
    let call_line = |storage: &str| -> Option<String> {
        let (code, report) = decompile_with(vec![directive(
            &format!("param open::0 {storage} char *pathname"),
            Body::Param {
                func: Some("open".into()),
                index: 0,
                storage: storage.into(),
                decl: "char *pathname".into(),
            },
        )])?;
        all_applied(&report);
        let signature = code.lines().next().unwrap_or_default().to_string();
        assert!(
            !signature.contains("pathname"),
            "the callee's parameter name landed on the CALLER: {signature}"
        );
        Some(
            code.lines()
                .find(|l| l.contains("open("))
                .unwrap_or_else(|| panic!("no call to open:\n{code}"))
                .trim()
                .to_string(),
        )
    };
    let Some(rdi) = call_line("%RDI") else { return };
    let Some(rsi) = call_line("%RSI") else { return };
    assert!(rdi.contains("open(a0)"), "the declared RDI argument is missing: {rdi}");
    assert_ne!(rdi, rsi, "the declared storage did not pick the argument");
}

/// The `return` half of the same plumbing: a qualified `return` parks the
/// callee's output storage, so what the call site reads back moves with it —
/// and, as above, the caller's own return is left alone.
#[test]
fn a_qualified_return_declares_the_callee_output() {
    let Some((baseline, _)) = decompile_with(Vec::new()) else { return };
    let Some((code, report)) = decompile_with(vec![directive(
        "return open::%RBX int4",
        Body::Return { func: Some("open".into()), storage: "%RBX".into(), decl: "int4".into() },
    )]) else {
        return;
    };
    all_applied(&report);
    assert_ne!(
        baseline, code,
        "a qualified `return` on a callee changed nothing in the caller"
    );
    assert!(
        code.lines().next().unwrap_or_default().starts_with("unsigned long authenticate("),
        "the qualified directive rewrote the CALLER's return:\n{code}"
    );
}

/// `return` — a locked return storage and type (`map return`).
///
/// This is also the regression for the abort below: the directive parks pieces
/// that carry output storage, and before the fix those aborted the process.
#[test]
fn return_locks_the_output_storage_and_type() {
    let Some((code, report)) = decompile_with(vec![directive(
        "return %RAX int4",
        Body::Return { func: None, storage: "%RAX".into(), decl: "int4".into() },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.starts_with("int4 authenticate") || code.starts_with("int authenticate"),
        "the locked return type did not reach the signature:\n{code}"
    );
}

/// `typedef` interns a type, and `type` can then name it — the pair is what lets
/// an agent describe a structure kuna never saw.
#[test]
fn a_typedef_is_nameable_by_a_later_type_directive() {
    let Some((code, report)) = decompile_with(vec![
        directive(
            "typedef struct creds { char raw[16]; };",
            Body::Typedef { decl: "struct creds { char raw[16]; };".into() },
        ),
        directive(
            "type v2 creds",
            Body::Type { func: None, symbol: "v2".into(), decl: "creds".into() },
        ),
    ]) else {
        return;
    };
    all_applied(&report);
    assert!(code.contains("creds v2;"), "the interned struct did not type the local:\n{code}");
    assert!(code.contains("v2.raw"), "the struct fields did not render:\n{code}");
}

/// `data` — a named, typed global (`map address`), observable at the call that
/// passes it.
#[test]
fn data_renames_the_global_at_its_use() {
    let Some((code, report)) = decompile_with(vec![directive(
        "data 0x601048 char *shadowpw",
        Body::Data { addr: 0x601048, decl: "char *shadowpw".into() },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(code.contains("shadowpw"), "the declared global did not reach the C:\n{code}");
    assert!(!code.contains("sneaky"), "the loader name survived the declaration:\n{code}");
}

/// `comment` — an agent's own note, rendered into the C at the instruction.
#[test]
fn comment_reaches_the_emitted_c() {
    let Some((code, report)) = decompile_with(vec![directive(
        "comment 0x400699 open the credentials file",
        Body::Comment {
            func: None,
            addr: 0x400699,
            text: "open the credentials file".into(),
        },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.contains("/* open the credentials file */"),
        "the comment did not reach the C:\n{code}"
    );
}

/// `function` — the `--define-function` spelling, carried by the same plane.
#[test]
fn function_declares_a_bounded_entry() {
    let Some(mut prog) = load() else { return };
    prog.set_assertions(vec![directive(
        "function 0x400664-0x400680=authstub",
        Body::Function { start: 0x400664, end: Some(0x400680), name: Some("authstub".into()) },
    )]);
    assertions::apply_program_scoped(&mut prog);
    all_applied(&prog.assertion_outcomes());
    let entry = prog
        .resolve_entry(&EntrySelector::Name("authstub".to_string()))
        .expect("the declared name resolves");
    assert_eq!(entry.addr.get_offset(), 0x400664);
    let funcs = decompile_targets(&mut prog, vec![entry], true, false, false);
    assert_eq!(funcs[0].name, "authstub");
    assert_eq!(funcs[0].size, 0x1c, "the declared extent is what the record reports");
}

/// An unqualified symbol-scoped directive cannot bind on a multi-function run —
/// it would silently mean "every function that happens to have a `v2`" — so it is
/// rejected with a detail that says how to write it instead.  A qualified one
/// binds to exactly the function it names.
#[test]
fn a_multi_function_run_needs_the_directive_to_name_its_function() {
    let Some(mut prog) = load() else { return };
    prog.set_assertions(vec![
        directive(
            "name v2 credbuf",
            Body::Name { func: None, symbol: "v2".into(), newname: "credbuf".into() },
        ),
        directive(
            "name authenticate::v2 credbuf",
            Body::Name {
                func: Some(TARGET.into()),
                symbol: "v2".into(),
                newname: "credbuf".into(),
            },
        ),
    ]);
    assertions::apply_program_scoped(&mut prog);
    let targets: Vec<_> = ["authenticate", "main"]
        .iter()
        .map(|n| prog.resolve_entry(&EntrySelector::Name(n.to_string())).expect("resolves"))
        .collect();
    let funcs = decompile_targets(&mut prog, targets, true, false, false);
    let report = prog.assertion_outcomes();
    assert_eq!(report[0].status, "rejected", "an unqualified directive bound anyway");
    assert!(
        report[0].detail.as_deref().unwrap_or_default().contains("<func>::<operand>"),
        "the rejection does not say how to qualify it: {:?}",
        report[0].detail
    );
    assert_eq!(report[1].status, "applied");
    let authenticate = funcs.iter().find(|f| f.name == "authenticate").expect("decompiled");
    assert!(
        authenticate.code.as_deref().unwrap_or_default().contains("credbuf"),
        "the qualified directive did not reach its function"
    );
}

/// A `map return`-shaped prototype — explicit output storage and NO declared
/// return type — used to abort the process (`outtype null`,
/// `ParamListStandardOut::assignMap`) the moment its function was decompiled, so
/// the one console command an agent would reach for to fix a return value killed
/// the session.  The declared type is the return type.
#[test]
fn output_only_prototype_pieces_do_not_abort_the_drive() {
    use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces, PrototypePieces};
    let Some(mut prog) = load() else { return };
    let entry = prog
        .resolve_entry(&EntrySelector::Name(TARGET.to_string()))
        .expect("fauxware has an `authenticate`");
    let int4 = prog
        .arch()
        .types()
        .get_base(4, kuna_decomp::dtype::type_metatype::TYPE_INT)
        .expect("int4");
    let rax = prog
        .arch()
        .manage()
        .get_space_by_name("register")
        .map(|s| kuna_base::address::Address::new(std::rc::Rc::clone(s), 0))
        .expect("x86-64 has a register space");
    let pieces = PrototypePieces {
        name: TARGET.to_string(),
        first_var_arg_slot: -1,
        output_storage: Some(ParameterPieces {
            addr: rax,
            type_: Some(int4),
            flags: parameter_pieces_flags::TYPELOCK,
        }),
        ..Default::default()
    };
    let addr = entry.addr.clone();
    let step = kuna_console::decompile_step::decompile_one(
        prog.arch_mut(),
        TARGET,
        addr,
        0,
        &kuna_console::decompile_step::DecompileSeed {
            mapped_symbols: &[],
            usepoint_symbols: &[],
            dynamic_symbols: &[],
            pending_proto: Some(&pieces),
            flow_overrides: &[],
            mapped_params: &[],
        },
        &[],
    );
    assert!(step.result.is_ok(), "an output-only prototype aborted the drive");
}

/// The declarations kuna PRINTS are declarations kuna ACCEPTS
/// (`docs/re-needs/prototype-assertions-reject-ordinary.md`).  Until the
/// C-declaration grammar learned the standard scalar keywords, a base type was
/// whatever `findByName` answered, so `int` / `unsigned int` / `long long` --
/// exactly what the printer emits -- were rejected as syntax errors while
/// `int4` / `uint4` / `int8` worked.  Five testers filed it in one round, and
/// `docs/cli.md`'s own worked example was among the rejected forms.
#[test]
fn standard_c_scalar_types_reach_the_emitted_c() {
    let Some((code, report)) = decompile_with(vec![
        directive(
            "prototype authenticate unsigned int authenticate(char *user,char *pass)",
            Body::Prototype {
                func: TARGET.into(),
                decl: "unsigned int authenticate(char *user,char *pass)".into(),
            },
        ),
        directive(
            "prototype read long long read(int fd,void *buf,unsigned long n)",
            Body::Prototype {
                func: "read".into(),
                decl: "long long read(int fd,void *buf,unsigned long n)".into(),
            },
        ),
        directive(
            "type v2 unsigned char[8]",
            Body::Type { func: None, symbol: "v2".into(), decl: "unsigned char[8]".into() },
        ),
    ]) else {
        return;
    };
    all_applied(&report);
    // This surface renders core types with their interned names (the C speller
    // is a `--mode` preset the CLI applies, not this bare drive), so the
    // declared `unsigned int` reads back as `uint4` and `unsigned char` as
    // `uint1` -- either spelling names the type that was asserted.  The
    // baseline return type is an 8-byte integer, so a 4-byte unsigned one is
    // the discriminator.
    assert!(
        code.contains("uint4 authenticate(char *user,char *pass)")
            || code.contains("unsigned int authenticate(char *user,char *pass)"),
        "the C return type did not reach the C:\n{code}"
    );
    assert!(
        code.contains("uint1 v2 [8];") || code.contains("unsigned char v2 [8];"),
        "a multi-word scalar did not survive as a `type` base:\n{code}"
    );
}

/// `<func>` is what the signature binds to, not the name inside the declaration
/// (`docs/re-needs/text-output-silently-ignores.md`).  An agent that has worked
/// out what a stripped function does writes the declaration under the name it
/// deserves -- `void *hashit(void *out,void *input)` for `authenticate` -- and
/// the types must still land on the function that was named as the target.
///
/// This surface always did that (`assertions::apply_prototype` overwrites
/// `pieces.name`); the console script did not, and the two are asserted together
/// because either alone is self-consistent.
#[test]
fn a_declaration_written_under_another_name_still_binds_to_its_target() {
    let Some((code, report)) = decompile_with(vec![directive(
        "prototype authenticate void *hashit(void *out,void *input)",
        Body::Prototype {
            func: TARGET.into(),
            decl: "void *hashit(void *out,void *input)".into(),
        },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.contains("authenticate(void *out,void *input)"),
        "the declared signature did not reach its target:\n{code}"
    );
    assert!(
        !code.contains("hashit"),
        "the declaration's name became a function:\n{code}"
    );
}

/// A combination that is not a C type is named, not answered with a bare
/// "Syntax error" pointing at the second keyword.
#[test]
fn an_impossible_scalar_combination_is_rejected_by_name() {
    let Some((_code, report)) = decompile_with(vec![directive(
        "prototype authenticate short long authenticate(void)",
        Body::Prototype {
            func: TARGET.into(),
            decl: "short long authenticate(void)".into(),
        },
    )]) else {
        return;
    };
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, "rejected", "{report:?}");
    let detail = report[0].detail.clone().unwrap_or_default();
    assert!(
        detail.contains("Invalid combination of C type specifiers: short long"),
        "the rejection did not name the combination: {detail}"
    );
}

/// `<func>` may be an ENTRY ADDRESS, not just a name — an agent has the address
/// long before it has a name it trusts, and the address form used to be
/// accepted and then dropped on the floor
/// (`docs/re-needs/accepted-sqrt-prototype-still.md`): nothing is called
/// `0x400664`, so the by-name park landed on no symbol at all while the report
/// still said `applied`.
#[test]
fn a_prototype_at_an_entry_address_binds_to_the_function_there() {
    let Some((code, report)) = decompile_with(vec![directive(
        "prototype 0x400664 void *hashit(void *out,void *input)",
        Body::Prototype {
            func: "0x400664".into(),
            decl: "void *hashit(void *out,void *input)".into(),
        },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.contains("authenticate(void *out,void *input)"),
        "the address-form signature did not reach the function at 0x400664:\n{code}"
    );
    assert!(!code.contains("hashit"), "the declaration's name became a function:\n{code}");
}

/// The address form is what reaches a CALLEE the name form can miss: the park
/// is keyed by entry address, which is the key
/// `ArchContext::callee_proto_pieces` already reads a call site back through.
/// `strcmp` here is a PLT stub, the same shape as the PE import thunk the need
/// was filed on.
#[test]
fn an_address_form_prototype_reaches_a_callee_at_that_address() {
    let Some((baseline, _)) = decompile_with(Vec::new()) else { return };
    assert!(baseline.contains("strcmp(a1,sneaky)"), "baseline moved:\n{baseline}");
    let Some((code, report)) = decompile_with(vec![directive(
        "prototype 0x400550 int4 strcmp(char *a,char *b,unsigned long n)",
        Body::Prototype {
            func: "0x400550".into(),
            decl: "int4 strcmp(char *a,char *b,unsigned long n)".into(),
        },
    )]) else {
        return;
    };
    all_applied(&report);
    assert!(
        code.contains("strcmp(a1,sneaky,"),
        "the declared third argument never reached the call site:\n{code}"
    );
}

/// An explicitly `0x`-prefixed operand that starts no function is REJECTED with
/// the address in the detail.  `0x...` is not a C identifier, so such a
/// directive can never bind, and reporting it `applied` — which is what the
/// whole family did — leaves an agent with no way to tell.
#[test]
fn an_address_that_starts_no_function_is_rejected_by_address() {
    let Some((_code, report)) = decompile_with(vec![directive(
        "prototype 0x999999 int4 nope(void)",
        Body::Prototype { func: "0x999999".into(), decl: "int4 nope(void)".into() },
    )]) else {
        return;
    };
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, "rejected", "{report:?}");
    let detail = report[0].detail.clone().unwrap_or_default();
    assert!(detail.contains("no function starts at 0x999999"), "unhelpful detail: {detail}");
}

/// A qualified `param` takes the same operand, so a callee can be typed by
/// address as well as by name.
#[test]
fn a_qualified_param_accepts_an_entry_address_as_its_function() {
    let Some((code, report)) = decompile_with(vec![directive(
        "param 0x400560::0 %RDI char *pathname",
        Body::Param {
            func: Some("0x400560".into()),
            index: 0,
            storage: "%RDI".into(),
            decl: "char *pathname".into(),
        },
    )]) else {
        return;
    };
    all_applied(&report);
    let call = code
        .lines()
        .find(|l| l.contains("open("))
        .unwrap_or_else(|| panic!("no call to open:\n{code}"));
    assert!(call.contains("open(a0)"), "the declared RDI argument is missing: {call}");
}
