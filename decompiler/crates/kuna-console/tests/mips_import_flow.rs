//! Loader-proven resolver transfers survive console flow and explicit overrides.
use std::path::PathBuf;
use kuna_console::{
    assertions::{self, Body, Directive},
    engine::{bootstrap_from_object, EntrySelector},
    project::decompile_targets,
};

#[test]
fn resolver_transfers_are_bounded_and_explicit_flow_takes_precedence() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for image in ["flow-mips32r2-be-local-ssp-stripped.so",
        "flow-mips32r2-le-local-ssp-stripped.so", "flow-octeon3-n32.so", "flow-octeon3-64.so"] {
        for explicit in [false, true] {
            let mut prog = bootstrap_from_object(
                root.join("tests/fixtures/mips-import-flow").join(image).to_str().unwrap(),
                "", &[root.join("specs").to_string_lossy().into_owned()],
            ).unwrap();
            let sites: Vec<_> = prog.arch().loader_branch_sites.iter().cloned().collect();
            assert!(sites.len() >= 4, "{image}");
            prog.commit_pending_analysis().unwrap();
            if explicit {
                prog.set_assertions(sites.iter().map(|site| Directive {
                    raw: format!("flow {} return", site.get_offset()),
                    body: Body::Flow { func: None, addr: site.get_offset(), kind: "return".into() },
                }).collect());
                assertions::apply_program_scoped(&mut prog);
            }
            for site in sites {
                let entry = prog.resolve_entry(&EntrySelector::Numeric(site.get_offset() - 8)).unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(result[0].error.is_none(), "{image}: {:?}", result[0].error);
                let code = result[0].code.as_ref().unwrap();
                assert!(!code.contains("halt_baddata"), "{image}: {code}");
                if explicit {
                    assert!(code.contains("return") && !code.contains("(*"), "{image}: {code}");
                } else {
                    assert!(code.contains("(*"), "resolver must retain its indirect transfer: {image}: {code}");
                }
            }
            if explicit {
                assert!(prog.assertion_outcomes().iter().all(|outcome| outcome.status == "applied"));
            }
        }
    }
}
