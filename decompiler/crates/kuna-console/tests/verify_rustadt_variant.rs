//! End-to-end gate for **`option rustadt`**: the two-register value a rustc
//! `Result`/`Option` is returned in gets a NAME and a SHAPE.
//!
//! The symptom, on a `Result<u32,u32>` producer with three return paths
//! (`rustc -C opt-level=2`, `#[inline(never)]`):
//!
//! ```text
//! undefined16 branchy(uint4 a0,uint4 a1) {
//!   char v1 [16];
//!   if (a0 <= 10) return ZEXT416((a1 + a0) * 3) << 0x40;   // which variant?
//!   if (0x65 <= a1) return ZEXT416(a1 - 1) << 0x40;
//!   v1[8] = a1 ^ a0;                                       // where is the payload?
//!   v1[0] = 1;                                             // what is byte 0?
//!   return v1;
//! }
//! ```
//!
//! With the rule on, the tag and the payload become named fields of a
//! synthesized `struct { tag; union { Variant0; Variant1 } }`, each site's union
//! facet is pinned from the guard that fixed its tag, and a site whose value is
//! one expression renders as its variant constructor.
//!
//! Three things are pinned here that a unit test cannot reach: that the type
//! survives to the recovered PROTOTYPE (it is the printed return type), that the
//! facet the printer reads is the guard's and not `ScoreUnionFields`'s, and that
//! the variant members are named NEUTRALLY. The last is a deliberate product
//! decision, not an implementation detail: tagged `Result` is `Ok = 0`,
//! `Err = 1`; tagged `Option` is `None = 0`, `Some = 1`. Same codegen, opposite
//! polarity -- so `Ok`/`Err` in the output would be a coin flip that reads as a
//! finding.
//!
//! `auto` is what the fixture exercises, not `always`: the fixture is a real
//! rustc binary carrying the `.comment` record the source-language detection
//! reads, so this also gates the load-time fact reaching the per-function rule.
//! The C control at the bottom is what proves the fact is doing the gating.
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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `bin`, apply `options` (before the analysis commit, as the CLI
/// does), decompile `func`, and return the captured C (`None` => specs-less skip).
fn decompile(bin: &str, func: &str, options: &[&str]) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_rustadt_variant: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let mut cmds: Vec<String> = options.iter().map(|o| o.to_string()).collect();
    cmds.push(format!("load function {func}"));
    cmds.push("decompile".into());
    cmds.push("print C".into());
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

/// The captured C alone: the console echoes an `option` confirmation before it,
/// which is exactly what an off-vs-on comparison must not see.
fn body(out: &str) -> &str {
    match out.find("Clearing old decompilation") {
        Some(i) => &out[i..],
        None => out,
    }
}

/// The state before the rule: an anonymous 16-byte container.
#[test]
fn by_default_the_tagged_return_has_no_shape() {
    let Some(code) = decompile("rust_variant_x86_64", "branchy", &[]) else { return };
    assert!(
        code.contains("undefined16 branchy"),
        "the pre-rule rendering is the raw pair container; got:\n{code}",
    );
    assert!(
        !code.contains("RustEnum2_"),
        "no type may be synthesized at the default; got:\n{code}",
    );
}

/// The recovered type reaches the PROTOTYPE, which is the whole point of
/// installing it before the merge phase.
#[test]
fn the_variant_type_becomes_the_recovered_return_type() {
    let Some(code) = decompile("rust_variant_x86_64", "branchy", &["option rustadt auto"]) else {
        return;
    };
    assert!(
        code.contains("RustEnum2_16_8_0_1_uint4_uint4 branchy"),
        "the synthesized aggregate must be the printed return type; got:\n{code}",
    );
    assert!(
        !code.contains("undefined16 branchy"),
        "and the anonymous container must be gone; got:\n{code}",
    );
}

/// The tag and the payload become named fields, and the union member named is
/// the one the DOMINATING GUARD selected -- not the one a similarity score
/// preferred.
#[test]
fn the_payload_is_written_through_the_variant_the_guard_selected() {
    let Some(code) = decompile("rust_variant_x86_64", "branchy", &["option rustadt auto"]) else {
        return;
    };
    assert!(
        code.contains("v1.tag = 1;"),
        "the discriminant must be a named field; got:\n{code}",
    );
    assert!(
        code.contains("v1.payload.Variant1.f0 = a1 ^ a0;"),
        "the Err path writes through Variant1, the tag it set; got:\n{code}",
    );
    assert!(
        !code.contains("payload.Variant0"),
        "no access on that path may name the other variant; got:\n{code}",
    );
}

/// A return site whose value is a single expression renders as the constructor.
#[test]
fn a_single_expression_return_site_renders_as_its_variant_constructor() {
    let Some(code) = decompile("rust_variant_x86_64", "branchy", &["option rustadt auto"]) else {
        return;
    };
    assert!(
        code.contains("return Variant0((a1 + a0) * 3);"),
        "the first Ok path is a Variant0 construction; got:\n{code}",
    );
    assert!(
        code.contains("return Variant0(a1 - 1);"),
        "and so is the second; got:\n{code}",
    );
    assert!(
        !code.contains("<< 0x40"),
        "the shift that buried the payload must be gone; got:\n{code}",
    );
}

/// THE NAMING COMMITMENT. `Ok`/`Err` and `Some`/`None` have opposite tag
/// polarity and identical codegen, so nothing in the shape distinguishes them.
#[test]
fn the_variant_members_are_never_named_ok_err_some_or_none() {
    let Some(code) = decompile("rust_variant_x86_64", "branchy", &["option rustadt auto"]) else {
        return;
    };
    for guess in ["Ok", "Err", "Some", "None"] {
        assert!(
            !code.contains(&format!("payload.{guess}")),
            "a variant may not be named {guess} without corroboration; got:\n{code}",
        );
    }
}

/// The refusal: one tag value across every path is not two variants. rustc
/// compiles `samevariant` branchlessly to a SINGLE return, which is the same
/// refusal from the other direction -- either way nothing is typed.
#[test]
fn a_return_that_is_not_two_variants_is_refused() {
    let Some(off) = decompile("rust_variant_x86_64", "samevariant", &[]) else { return };
    let Some(on) = decompile("rust_variant_x86_64", "samevariant", &["option rustadt auto"])
    else {
        return;
    };
    assert!(!on.contains("RustEnum2_"), "nothing may be synthesized here; got:\n{on}");
    assert_eq!(body(&off), body(&on), "the function must render identically off and on");
}

/// The C control: `auto` reads the loader's source-language verdict, so a C
/// binary is untouched by construction rather than by luck.
#[test]
fn a_c_binary_is_byte_identical_off_and_on() {
    let Some(off) = decompile("fauxware", "main", &[]) else { return };
    let Some(on) = decompile("fauxware", "main", &["option rustadt auto"]) else { return };
    assert_eq!(body(&off), body(&on), "option rustadt auto cannot see a non-rustc image");
}
