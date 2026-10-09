//! Every NOP fall-through remains queryable on x86-64 and AArch64.

use kuna_analysis::listing::{Listing, RefKind};
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::{engine::bootstrap_from_object, project::decompile_targets};
use std::path::PathBuf;

#[test]
fn fallthrough_references_and_selected_and_whole_program_output() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = vec![root.join("specs").to_str().unwrap().to_string()];
    for (architecture, stride) in [("x86_64", 1u64), ("aarch64", 4)] {
        let path = root.join(format!(
            "decompiler/crates/kuna-analysis/tests/fixtures/listing_fallthrough_{architecture}.elf"
        ));
        let path = path.to_str().unwrap();
        let mut prog = bootstrap_from_object(path, "", &specs).expect("matching processor spec");
        let bytes = std::fs::read(path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let image = ObjectLoadImage::from_bytes(path, &bytes).unwrap();
        let arch = prog.arch();
        let listing = Listing::build(&file, &image, arch, arch.translate(), &[0x401000, 0x401010]);
        assert_eq!(listing.num_instructions(), 2052);
        assert_eq!(listing.function_count(), 2);
        assert_eq!(listing.ref_source_iter().count(), 2050);
        for i in 0..2048 {
            let from = 0x401010 + i * stride;
            let outgoing = listing.refs_from(from);
            assert_eq!(outgoing.len(), 1);
            assert_eq!(outgoing[0].from, from);
            assert_eq!(outgoing[0].to, from + stride);
            assert_eq!(outgoing[0].kind, RefKind::Code);
            assert_eq!(listing.refs_to(from + stride), outgoing);
            assert!(listing.has_refs_to(from + stride));
            assert_eq!(listing.ref_count_to(from + stride), 1);
        }
        assert!(listing.refs_to(0x401010).is_empty());
        let terminal = 0x401010 + 2048 * stride + if stride == 1 { 5 } else { 4 };
        assert!(listing.refs_from(terminal).is_empty());
        drop(listing);

        prog.arch_mut().set_kuna_option("listing", "on").unwrap();
        prog.commit_pending_analysis().unwrap();
        let targets = prog.function_entries_executable();
        assert_eq!(targets.len(), 2);
        let selected = targets
            .iter()
            .filter(|f| f.name == "_start")
            .cloned()
            .collect();
        let selected = decompile_targets(&mut prog, selected, false, false, true);
        assert_eq!(selected.len(), 1);
        assert!(selected[0].error.is_none());
        assert!(selected[0].code.as_ref().unwrap().contains("return 7;"));
        let whole = decompile_targets(&mut prog, targets, false, false, true);
        assert_eq!(whole.len(), 2);
        for function in &whole {
            assert!(function.error.is_none(), "{:?}", function.error);
            let code = function.code.as_ref().expect("complete body");
            if function.name == "_start" {
                assert_eq!(function.code, selected[0].code);
            } else {
                assert!(code.contains("return 3;"), "{code}");
            }
        }
    }
}
