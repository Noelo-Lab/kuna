//! Original instruction sequences test memory effects through final C recovery.
use std::path::PathBuf;

use kuna_base::xml::DocumentStorage;
use kuna_decomp::kuna_octeoneffects::OcteonEffectsMode;
use kuna_console::{
    assertions::{self, Body, Directive},
    engine::{bootstrap_program, EntrySelector},
    project::decompile_targets,
};

fn decompile(words: &[u32], n32: bool, prototype: &str) -> String {
    decompile_config(words, n32, Some(prototype), false)
}

fn decompile_config(words: &[u32], n32: bool, prototype: Option<&str>, readonly: bool) -> String {
    decompile_mode(words, n32, prototype, readonly, OcteonEffectsMode::default())
}

fn decompile_mode(
    words: &[u32],
    n32: bool,
    prototype: Option<&str>,
    readonly: bool,
    mode: OcteonEffectsMode,
) -> String {
    let target = format!(
        "MIPS:BE:64:octeon2{}:default",
        if n32 { "-32addr" } else { "" }
    );
    decompile_target(words, &target, prototype, readonly, mode)
}

fn decompile_target(
    words: &[u32],
    target: &str,
    prototype: Option<&str>,
    readonly: bool,
    mode: OcteonEffectsMode,
) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let bytes: String = words.iter().map(|w| format!("{w:08x}")).collect();
    let xml = format!("<binaryimage arch=\"{target}\"><bytechunk space=\"ram\" offset=\"0x1000\">{bytes}</bytechunk><bytechunk space=\"ram\" offset=\"0x2000\" readonly=\"{readonly}\">1122334455667788</bytechunk><symbol space=\"ram\" offset=\"0x1000\" name=\"probe\"/></binaryimage>");
    let mut documents = DocumentStorage::new();
    let image = documents
        .parse_document(xml.as_bytes())
        .unwrap()
        .get_root()
        .clone();
    let mut program = bootstrap_program(
        image,
        target,
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    program.arch_mut().readonlypropagate = true;
    program.arch_mut().octeon_effects = mode;
    if let Some(prototype) = prototype {
        program.set_assertions(vec![Directive {
            raw: format!("prototype probe {prototype}"),
            body: Body::Prototype {
                func: "probe".into(),
                decl: prototype.into(),
            },
        }]);
    }
    assertions::apply_program_scoped(&mut program);
    for outcome in program.assertion_outcomes() {
        assert_eq!(outcome.status, "applied", "{outcome:?}");
    }
    let entry = program
        .resolve_entry(&EntrySelector::Name("probe".into()))
        .unwrap();
    let result = decompile_targets(&mut program, vec![entry], false, false, false);
    assert!(result[0].error.is_none(), "{:?}", result[0].error);
    result[0].code.clone().unwrap()
}

#[test]
fn every_advertised_compiler_profile_preserves_memory_and_reservation_effects() {
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        for n32 in [false, true] {
            for compiler in if variant == "octeon3" {
                vec!["default", "hard"]
            } else {
                vec!["default"]
            } {
                let target = format!(
                    "MIPS:BE:64:{variant}{}:{compiler}",
                    if n32 { "-32addr" } else { "" }
                );
                let code = decompile_target(&[0xd0880000, 0xf0850000, 0x00a0102d, 0x03e00008, 0], &target,
                    Some("unsigned long long probe(unsigned long long *pointer, unsigned long long value)"), false, OcteonEffectsMode::default());
                assert!(code.contains("octeon_load_linked64"), "{target}: {code}");
                assert!(
                    code.contains("octeon_store_conditional64"),
                    "{target}: {code}"
                );
                if variant == "octeon" {
                    continue;
                }
                // sd a1,0(a0); saad a2,(a0); ld v0,0(a0); jr ra; nop
                let code = decompile_target(&[0xfc850000, 0x70860019, 0xdc820000, 0x03e00008, 0], &target,
                    Some("unsigned long long probe(unsigned long long *pointer, unsigned long long value, unsigned long long increment)"), false, OcteonEffectsMode::default());
                assert!(code.contains("octeon_fetch_add64"), "{target}: {code}");
                assert!(code.contains("*pointer = value"), "{target}: {code}");
                assert!(!code.contains("return value;"), "{target}: {code}");
            }
        }
    }
}

#[test]
fn atomics_treat_image_memory_as_stores_do_and_keep_partial_overlaps() {
    // ori a0,zero,0x2000; daddiu a1,zero,7; laad zero,(a0),a1; ld v0,0(a0)
    for n32 in [false, true] {
        let words = [0x34042000, 0x64050007, 0x708504df, 0xdc820000, 0x03e00008, 0];
        let code = decompile_config(&words, n32, Some("unsigned long long probe(void)"), false);
        assert!(code.contains("octeon_fetch_add64"), "{code}");
        assert!(!code.contains("return 0x1122334455667788"), "writable memory folded: {code}");
        // Read-only image bytes fold after an atomic exactly as after `sd`.
        let atomic = decompile_config(&words, n32, Some("unsigned long long probe(void)"), true);
        let store = decompile_config(
            &[0x34042000, 0x64050007, 0xfc850000, 0xdc820000, 0x03e00008, 0],
            n32,
            Some("unsigned long long probe(void)"),
            true,
        );
        assert!(atomic.contains("return 0x1122334455667788"), "{atomic}");
        assert!(store.contains("return 0x1122334455667788"), "{store}");
        // Zero an eight-byte object, then update only its low word.
        let code = decompile(
            &[
                0x34042004, 0xfc80fffc, 0x64050001, 0x7085049f, 0xdc82fffc, 0x03e00008, 0,
            ],
            n32,
            "unsigned long long probe(void)",
        );
        assert!(code.contains("octeon_fetch_add32"), "{code}");
        assert!(!code.contains("return 0;"), "{code}");
    }
}

#[test]
fn unasserted_n32_atomic_address_cannot_narrow_a_returned_integer() {
    // laad t0,(a0),a1; daddu v0,a0,zero; jr ra; nop
    let code = decompile_config(&[0x708544df, 0x0080102d, 0x03e00008, 0], true, None, false);
    assert!(code.contains("octeon_fetch_add64"), "{code}");
    assert!(
        code.contains("unsigned long long")
            || code.contains("ulonglong")
            || code.contains("uint64_t"),
        "{code}"
    );
    assert!(!code.contains("return (uint)"), "{code}");
}

#[test]
fn repeated_ordering_operations_keep_both_neighboring_memory_observations() {
    // sd a1,0(a0); syncw; syncw; ld v0,0(a0); jr ra; nop
    for n32 in [false, true] {
        let code = decompile(
            &[0xfc850000, 0x10f, 0x10f, 0xdc820000, 0x03e00008, 0],
            n32,
            "unsigned long long probe(unsigned long long *pointer, unsigned long long value)",
        );
        assert_eq!(code.matches("octeon_syncw(").count(), 2, "{code}");
        assert!(code.contains("*pointer = value"), "{code}");
        assert!(!code.contains("return value;"), "{code}");
    }
}

#[test]
fn atomic_write_prevents_global_value_folding_and_keeps_its_input_store() {
    // ori a0,zero,0x2000; daddiu a1,zero,7; sd a1,0(a0);
    // laad zero,(a0),a1; ld v0,0(a0); jr ra; nop
    for n32 in [false, true] {
        let code = decompile(
            &[
                0x34042000, 0x64050007, 0xfc850000, 0x708504df, 0xdc820000, 0x03e00008, 0,
            ],
            n32,
            "unsigned long long probe(void)",
        );
        let compact: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(code.contains("octeon_fetch_add64"), "{code}");
        assert!(
            !compact.contains("return7;"),
            "atomic write was lost: {code}"
        );
        assert!(
            compact.contains("=7;"),
            "store feeding atomic was lost: {code}"
        );
    }
}

#[test]
fn atomic_stack_read_keeps_store_even_without_a_subsequent_load() {
    // daddiu sp,sp,-32; sd a1,16(sp); daddiu a0,sp,16;
    // laad v0,(a0),a2; daddiu sp,sp,32; jr ra; nop
    for n32 in [false, true] {
        let code = decompile(&[0x67bdffe0, 0xffa50010, 0x67a40010, 0x708614df,
            0x67bd0020, 0x03e00008, 0], n32,
            "unsigned long long probe(unsigned long long unused, unsigned long long value, unsigned long long increment)");
        let compact: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(code.contains("octeon_fetch_add64"), "{code}");
        assert!(
            compact.contains("=value;"),
            "atomic input store was lost: {code}"
        );
        assert!(code.contains("increment"), "{code}");
    }
}

#[test]
fn n32_atomic_address_use_preserves_full_width_integer_comparison() {
    // laad t0,(a0),a2; xor v0,a0,a1; sltiu v0,v0,1; jr ra; nop
    let code = decompile(&[0x708644df, 0x00851026, 0x2c420001, 0x03e00008, 0], true,
        "unsigned long long probe(unsigned long long raw, unsigned long long expected, unsigned long long increment)");
    let compact: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(
        compact.contains("raw==expected") || compact.contains("expected==raw"),
        "{code}"
    );
    assert!(code.contains("octeon_fetch_add64"), "{code}");
}

#[test]
fn store_conditional_retains_runtime_success_and_memory_effect() {
    // ll t0,0(a0); sc a1,0(a0); daddu v0,a1,zero; jr ra; nop
    for n32 in [false, true] {
        let code = decompile(
            &[0xc0880000, 0xe0850000, 0x00a0102d, 0x03e00008, 0],
            n32,
            "unsigned long long probe(unsigned int *pointer, unsigned int value)",
        );
        let compact: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(code.contains("octeon_load_linked32"), "{code}");
        assert!(code.contains("octeon_store_conditional32"), "{code}");
        assert!(!compact.contains("return1;"), "{code}");
        assert!(!compact.contains("return0;"), "{code}");
    }
}

#[test]
fn multiple_atomic_results_keep_their_sequenced_memory_observations() {
    // laad v0,(a0),a1; laad v1,(a0),a2; dsubu v0,v1,v0; jr ra; nop
    for n32 in [false, true] {
        let code = decompile(
            &[0x708514df, 0x70861cdf, 0x0062102f, 0x03e00008, 0],
            n32,
            "unsigned long long probe(unsigned long long *pointer, unsigned long long first, unsigned long long second)",
        );
        assert_eq!(code.matches("octeon_fetch_add64(").count(), 2, "{code}");
        for statement in code.split(';') {
            assert!(
                statement.matches("octeon_fetch_add64(").count() <= 1,
                "unsequenced atomic calls in one C expression: {code}"
            );
        }
        assert!(
            code.find("pointer,first").unwrap() < code.find("pointer,second").unwrap(),
            "{code}"
        );
    }
}

#[test]
fn a_barrier_neither_forces_a_stack_spill_nor_blocks_readonly_folding() {
    for n32 in [false, true] {
        for mode in [OcteonEffectsMode::Writes, OcteonEffectsMode::Ordered] {
            // daddiu sp,sp,-16; sd a0,8(sp); syncw; ld v0,8(sp); jr ra; daddiu sp,sp,16
            let code = decompile_mode(
                &[0x67bdfff0, 0xffa40008, 0x10f, 0xdfa20008, 0x03e00008, 0x67bd0010],
                n32,
                Some("unsigned long long probe(unsigned long long value)"),
                false,
                mode,
            );
            assert!(code.contains("octeon_syncw("), "{mode:?}: {code}");
            assert!(code.contains("return value;"), "{mode:?}: {code}");
            // ori a0,zero,0x2000; syncw; ld v0,0(a0); jr ra; nop
            let code = decompile_mode(
                &[0x34042000, 0x10f, 0xdc820000, 0x03e00008, 0],
                n32,
                Some("unsigned long long probe(void)"),
                true,
                mode,
            );
            assert!(code.contains("return 0x1122334455667788"), "{mode:?}: {code}");
        }
    }
}

#[test]
fn the_mode_decides_which_operations_guard_writable_memory() {
    // ori a0,zero,0x2000; daddiu a1,zero,7; sd a1,0(a0); OP; ld v0,0(a0); jr ra; nop
    let program = |op: u32| [0x34042000, 0x64050007, 0xfc850000, op, 0xdc820000, 0x03e00008, 0];
    let laad = 0x708504df;
    let syncw = 0x10f;
    for n32 in [false, true] {
        let reused = |op: u32, mode: OcteonEffectsMode| {
            let code = decompile_mode(&program(op), n32, Some("unsigned long long probe(void)"), false, mode);
            code.chars().filter(|c| !c.is_whitespace()).collect::<String>().contains("return7;")
        };
        assert!(reused(laad, OcteonEffectsMode::Off), "off leaves the atomic opaque");
        assert!(!reused(laad, OcteonEffectsMode::Writes));
        assert!(!reused(laad, OcteonEffectsMode::Ordered));
        assert!(reused(syncw, OcteonEffectsMode::Writes), "a barrier writes nothing");
        assert!(!reused(syncw, OcteonEffectsMode::Ordered), "ordered rereads after a barrier");
    }
}
