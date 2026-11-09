//! Loader-proven resolver transfers survive console flow and explicit overrides.
use std::path::PathBuf;
use kuna_console::{
    assertions::{self, Body, Directive},
    engine::{bootstrap_from_object, EntrySelector},
    project::decompile_targets,
};

#[test]
fn only_concrete_got_import_declarations_supply_type_only_prototypes() {
    use kuna_decomp::{dtype::type_metatype, kuna_qualifiedcall};
    use std::rc::Rc;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for image in ["flow-mips32r2-be-local-ssp-stripped.so", "flow-mips32r2-le-local-ssp-stripped.so",
        "flow-octeon-be-local-ssp-stripped.so", "flow-octeon-le-local-ssp-stripped.so",
        "flow-octeon3-n32.so", "flow-octeon3-64.so"] {
        let mut prog = bootstrap_from_object(
            root.join("tests/fixtures/mips-import-flow").join(image).to_str().unwrap(),
            "", &[root.join("specs").to_string_lossy().into_owned()],
        ).unwrap();
        prog.commit_pending_analysis().unwrap();
        let ctx = prog.arch().build_arch_handle();
        let entry = ctx.loader_mips_imports.iter().find(|entry|
            ctx.callee_proto_pieces(entry).is_some_and(|p| p.name == "strlen")
        ).expect("loader-proven strlen import").clone();
        let pieces = ctx.callee_proto_pieces(&entry).unwrap();
        assert!(pieces.input_storage.is_empty() && pieces.output_storage.is_none());
        let proto = kuna_qualifiedcall::prototype(&ctx, &entry).unwrap();
        assert!(proto.is_output_locked() && proto.get_output().get_size() > 0);
        assert_eq!(proto.num_params(), 1);
        let void = prog.arch().types().get_type_void().unwrap();
        let unknown = prog.arch().types().get_base(4, type_metatype::TYPE_UNKNOWN).unwrap();
        for output in [None, Some(void), Some(unknown.clone())] {
            let mut incomplete = pieces.clone();
            incomplete.outtype = output;
            prog.arch_mut().set_function_prototype_pieces_at(&entry, incomplete);
            let ctx = prog.arch().build_arch_handle();
            assert!(kuna_qualifiedcall::prototype(&ctx, &entry).is_none(), "{image}");
        }
        let mut variadic = pieces.clone();
        variadic.first_var_arg_slot = 1;
        prog.arch_mut().set_function_prototype_pieces_at(&entry, variadic);
        assert!(kuna_qualifiedcall::prototype(&prog.arch().build_arch_handle(), &entry).is_none(),
            "a variadic declaration fixes no arity: {image}");
        let mut incomplete = pieces.clone();
        incomplete.intypes[0] = unknown;
        prog.arch_mut().set_function_prototype_pieces_at(&entry, incomplete);
        assert!(kuna_qualifiedcall::prototype(&prog.arch().build_arch_handle(), &entry).is_none());
        prog.arch_mut().set_function_prototype_pieces_at(&entry, pieces);
        prog.arch_mut().loader_mips_imports = Rc::default();
        assert!(kuna_qualifiedcall::prototype(&prog.arch().build_arch_handle(), &entry).is_none(),
            "an entry without GOT import provenance must keep the code-type lookup");
    }
}

#[test]
fn resolver_transfers_are_bounded_and_explicit_flow_takes_precedence() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for image in ["flow-mips32r2-be-local-ssp-stripped.so", "flow-mips32r2-le-local-ssp-stripped.so",
        "flow-octeon-be-local-ssp-stripped.so", "flow-octeon-le-local-ssp-stripped.so",
        "flow-octeon3-n32.so", "flow-octeon3-64.so"] {
        for explicit in [false, true] {
            let mut prog = bootstrap_from_object(
                root.join("tests/fixtures/mips-import-flow").join(image).to_str().unwrap(),
                "", &[root.join("specs").to_string_lossy().into_owned()],
            ).unwrap();
            let sites: Vec<_> = prog.arch().loader_transfer_sites.iter().cloned().collect();
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
                    assert!(code.contains("(*") && code.contains("return"),
                        "resolver is a tail call to the bound import: {image}: {code}");
                    assert!(!code.contains("jump-as-call") && !code.contains("jumptable"),
                        "a proven tail transfer is not an indirect branch: {image}: {code}");
                }
            }
            if explicit {
                assert!(prog.assertion_outcomes().iter().all(|outcome| outcome.status == "applied"));
            }
        }
    }
}

#[test]
fn the_listing_and_xrefs_walks_never_follow_a_resolver_fall_through() {
    use kuna_analysis::{listing::Listing, loadimage_object::ObjectLoadImage};
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let path = root.join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_chain_mips_O0");
    let path = path.to_str().unwrap();
    let bytes = std::fs::read(path).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let image = ObjectLoadImage::from_bytes(path, &bytes).unwrap();
    let &(site, end) = image.mips_linkage().unwrap().transfers.iter().max().unwrap();
    for enabled in [true, false] {
        let mut prog = bootstrap_from_object(path, "", &[root.join("specs").to_string_lossy().into_owned()]).unwrap();
        prog.arch_mut().set_kuna_option("stubtailcall", if enabled { "on" } else { "off" }).unwrap();
        let arch = prog.arch();
        let listing = Listing::build(&file, &image, arch, arch.translate(), &[site - 8]);
        assert!(listing.is_instruction_start(site));
        assert_eq!(listing.is_instruction_start(end), !enabled,
            "stubtailcall={enabled}: bytes after the last stub ({end:#x})");
        assert_eq!(listing.refs_from(site).is_empty(), enabled, "stubtailcall={enabled}");
        let index = kuna_analysis::listing::xrefs::build(&file, arch, arch.translate(), &[site - 8]);
        assert_eq!(index.is_decoded(end), !enabled, "xrefs, stubtailcall={enabled}");
    }
}
