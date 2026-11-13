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
        for mode in [OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
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
        assert!(!reused(laad, OcteonEffectsMode::Atomic));
        assert!(!reused(laad, OcteonEffectsMode::Ordered));
        assert!(reused(syncw, OcteonEffectsMode::Atomic), "a barrier writes nothing");
        assert!(!reused(syncw, OcteonEffectsMode::Ordered), "ordered rereads after a barrier");
    }
}

#[test]
fn a_store_read_only_by_a_load_linked_is_kept() {
    // ori a0,zero,0x2000; sd a1,0(a0); lld t0,0(a0); sd zero,0(a0); jr ra; or v0,t0,zero
    for n32 in [false, true] {
        for mode in [OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
            let code = decompile_mode(
                &[0x34042000, 0xfc850000, 0xd08c0000, 0xfc800000, 0x03e00008, 0x01801025],
                n32,
                Some("unsigned long long probe(unsigned long long unused, unsigned long long value)"),
                false,
                mode,
            );
            let stored = code.find("= value;").unwrap_or_else(|| panic!("{mode:?}: {code}"));
            assert!(stored < code.find("octeon_load_linked64(").unwrap(), "{mode:?}: {code}");
        }
    }
}

#[test]
fn off_lifts_reservations_and_barriers_as_upstream_does() {
    // ori a0,zero,0x2000; daddiu a1,zero,7; sd a1,0(a0); sc a2,0(a0); ld v0,0(a0); jr ra; nop
    // then the same with syncw, and with ll t0,0(a0) returned through v0
    let store = [0x34042000, 0x64050007, 0xfc850000, 0xe0860000, 0xdc820000, 0x03e00008, 0];
    let barrier = [0x34042000, 0x64050007, 0xfc850000, 0x10f, 0xdc820000, 0x03e00008, 0];
    let load = [0x34042000, 0xc0820000, 0x03e00008, 0];
    for n32 in [false, true] {
        let off = |words: &[u32]| decompile_mode(words, n32, None, false, OcteonEffectsMode::Off);
        let code = off(&store);
        let compact: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(!code.contains("octeon_"), "{code}");
        assert!(compact.contains("CONCAT44(a2,7)"), "the store-conditional stores: {code}");
        let code = off(&barrier);
        assert!(code.contains("SYNC(4);") && !code.contains("octeon_"), "{code}");
        let code = off(&load);
        assert!(!code.contains("octeon_"), "{code}");
    }
}

#[test]
fn pointer_loads_stay_before_selected_octeon_memory_effects() {
    let operations = [
        (0x70850018, "octeon_fetch_add32"),
        (0x70850019, "octeon_fetch_add64"),
        (0x7085059f, "octeon_exchange32"),
        (0x708505df, "octeon_exchange64"),
        (0xc0880000, "octeon_load_linked32"),
        (0xd0880000, "octeon_load_linked64"),
        (0xe0850000, "octeon_store_conditional32"),
        (0xf0850000, "octeon_store_conditional64"),
        (0x7080071f, "octeon_zero_cache_block"),
        (0x7080075f, "octeon_zero_cache_block_transient"),
    ];
    for n32 in [false, true] {
        for mode in [OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
            for (operation, name) in operations {
                // ld v0,0(a0); OP; jr ra; nop
                let code = decompile_mode(
                    &[0xdc820000, operation, 0x03e00008, 0],
                    n32,
                    Some("unsigned long long probe(unsigned long long *pointer, unsigned long long value)"),
                    false,
                    mode,
                );
                let load = code.find("= *pointer;").unwrap_or_else(|| panic!("{mode:?}: {code}"));
                assert!(load < code.find(name).unwrap(), "{mode:?}: {code}");
                assert!(!code.contains("return *pointer;"), "{mode:?}: {code}");
            }
        }
        for selector in 0..=7 {
            let code = decompile_mode(
                &[0xdc820000, (selector << 6) | 0xf, 0x03e00008, 0],
                n32,
                Some("unsigned long long probe(unsigned long long *pointer)"),
                false,
                OcteonEffectsMode::Ordered,
            );
            assert!(code.find("= *pointer;").is_some_and(|p| p < code.find("octeon_sync").unwrap()), "{code}");
        }
        for mode in [OcteonEffectsMode::Off, OcteonEffectsMode::Atomic] {
            let code = decompile_mode(
                &[0xdc820000, 0x10f, 0x03e00008, 0],
                n32,
                Some("unsigned long long probe(unsigned long long *pointer)"),
                false,
                mode,
            );
            assert!(code.contains("return *pointer;"), "{mode:?}: {code}");
        }
    }
}

#[test]
fn a_pointer_load_cannot_sink_into_an_expression_beyond_a_conditional_atomic() {
    // ld v0,0(a0); beq a2,zero,done; nop; saad a1,(a0);
    // done: jr ra; daddu v0,v0,a2
    for n32 in [false, true] {
        let code = decompile(
            &[0xdc820000, 0x10c00002, 0, 0x70850019, 0x03e00008, 0x0046102d],
            n32,
            "unsigned long long probe(unsigned long long *pointer, unsigned long long value, unsigned long long extra)",
        );
        assert!(code.find("= *pointer;").is_some_and(|p| p < code.find("if (").unwrap()), "{code}");
        assert!(!code.contains("return *pointer"), "{code}");
        assert!(code.contains("extra"), "{code}");
    }
}

#[test]
fn cache_block_zeroing_keeps_neighboring_unescaped_stack_aliases() {
    for n32 in [false, true] {
        for mode in [OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
            for operation in [0x7080071f, 0x7080075f] {
                for offset in [8, 24] {
                    // daddiu sp,sp,-32; sd a1,offset(sp); daddiu a0,sp,16;
                    // zcb[t] (a0); ld v0,offset(sp); jr ra; daddiu sp,sp,32
                    let code = decompile_mode(
                        &[0x67bdffe0, 0xffa50000 | offset, 0x67a40010, operation,
                          0xdfa20000 | offset, 0x03e00008, 0x67bd0020],
                        n32,
                        Some("unsigned long long probe(unsigned long long unused, unsigned long long value)"),
                        false,
                        mode,
                    );
                    assert!(!code.contains("return value;"), "{mode:?}: {code}");
                    let store = code.find("= value;").unwrap_or_else(|| panic!("{mode:?}: {code}"));
                    assert!(store < code.find("octeon_zero_cache_block").unwrap(), "{mode:?}: {code}");
                }
            }
        }
    }
}

/// A block zeroed through an incoming pointer starts less than 128 bytes below
/// the entry stack pointer: a 256-byte frame keeps its top block's spills
/// (slots at -8 and -128 from entry) and folds the deeper ones (-136, -248).
#[test]
fn cache_block_zeroing_through_an_incoming_pointer_reaches_only_the_top_frame_block() {
    for n32 in [false, true] {
        for mode in [OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
            for operation in [0x7080071f, 0x7080075f] {
                for (slot, kept) in [(248, true), (128, true), (120, false), (8, false)] {
                    // daddiu sp,sp,-256; sd a1,slot(sp); zcb[t] (a0); ld v0,slot(sp);
                    // jr ra; daddiu sp,sp,256
                    let code = decompile_mode(
                        &[0x67bdff00, 0xffa50000 | slot, operation,
                          0xdfa20000 | slot, 0x03e00008, 0x67bd0100],
                        n32,
                        Some("unsigned long long probe(unsigned long long *block, unsigned long long value)"),
                        false,
                        mode,
                    );
                    assert_eq!(!code.contains("return value;"), kept, "{mode:?} slot {slot}: {code}");
                    assert!(code.contains("octeon_zero_cache_block"), "{mode:?}: {code}");
                }
            }
        }
    }
}

#[test]
fn cache_block_zeroing_through_transformed_inputs_keeps_deep_stack_guards() {
    let transforms = [
        (0x6484ff00, "daddiu a0,a0,-256"),
        (0x64840080, "daddiu a0,a0,128"),
        (0x0086202d, "daddu a0,a0,a2"),
        (0x0086202f, "dsubu a0,a0,a2"),
        (0x00862024, "and a0,a0,a2"),
        (0x00862025, "or a0,a0,a2"),
        (0x00862026, "xor a0,a0,a2"),
        (0x00042078, "dsll a0,a0,1"),
        (0x0004203e, "dsrl32 a0,a0,0"),
        (0x0004202f, "dsubu a0,zero,a0"),
    ];
    for n32 in [false, true] {
        for mode in [OcteonEffectsMode::Atomic, OcteonEffectsMode::Ordered] {
            for operation in [0x7080071f, 0x7080075f] {
                for (transform, name) in transforms {
                    // daddiu sp,sp,-256; sd a1,8(sp); TRANSFORM; zcb[t] (a0);
                    // ld v0,8(sp); jr ra; daddiu sp,sp,256
                    let code = decompile_mode(
                        &[0x67bdff00, 0xffa50008, transform, operation,
                          0xdfa20008, 0x03e00008, 0x67bd0100],
                        n32,
                        Some("unsigned long long probe(unsigned long long *block, unsigned long long value, unsigned long long mask)"),
                        false,
                        mode,
                    );
                    assert!(!code.contains("return value;"), "{name} {mode:?}: {code}");
                    assert!(!code.contains("return 0;"), "block alignment is unknown: {code}");
                    let store = code.find("= value;").unwrap_or_else(|| panic!("{name}: {code}"));
                    assert!(store < code.find("octeon_zero_cache_block").unwrap(), "{name}: {code}");
                }
            }
        }
    }
}

/// Every constructor sign-extends a word result, so it is `int` and needs no
/// `(int4)` cast where the register is read whole; its address stays unsigned,
/// so a declared `unsigned int *` needs no pointer cast. A doubleword stays
/// unsigned.
#[test]
fn word_results_are_signed_like_the_registers_they_extend_into() {
    for n32 in [false, true] {
        // lai v0,(a0); jr ra; nop
        let lai = decompile_config(&[0x7080109f, 0x03e00008, 0], n32, None, false);
        assert!(lai.contains("return octeon_fetch_add32(") && !lai.contains("(int4)"), "{lai}");
        // ll t0,0(a0); dsra v0,t0,4; jr ra; nop
        let ll = decompile_config(&[0xc0880000, 0x0008113b, 0x03e00008, 0], n32, None, false);
        assert!(ll.contains("octeon_load_linked32(") && !ll.contains("(int4)"), "{ll}");
        // laad v0,(a0),a1; jr ra; nop
        let laad = decompile_config(&[0x708514df, 0x03e00008, 0], n32, None, false);
        assert!(laad.contains("uint8 probe(uint8 *"), "{laad}");
        // ll t0,0(a0); addu t0,t0,a1; sc t0,0(a0); jr ra; or v0,t0,zero
        let loop_body = decompile(
            &[0xc0880000, 0x01054021, 0xe0880000, 0x03e00008, 0x01001025],
            n32,
            "unsigned long long probe(unsigned int *p, unsigned int v)",
        );
        assert!(!loop_body.contains("(int4 *)"), "{loop_body}");
    }
}

#[test]
fn a_scalar_atomic_still_leaves_an_unescaped_neighbor_unchanged() {
    // As zero_neighbor, with saad a2,(a0) accessing only offset 16.
    for n32 in [false, true] {
        let code = decompile(
            &[0x67bdffe0, 0xffa50008, 0x67a40010, 0x70860019,
              0xdfa20008, 0x03e00008, 0x67bd0020],
            n32,
            "unsigned long long probe(unsigned long long unused, unsigned long long value, unsigned long long increment)",
        );
        assert!(code.contains("return value;"), "{code}");
    }
}

#[test]
fn recovered_pointer_snapshots_execute_with_the_original_old_value() {
    use std::process::Command;

    let compilers: Vec<_> = ["cc", "clang"].into_iter().filter(|compiler| {
        match Command::new(compiler).arg("--version").output() {
            Ok(result) => { assert!(result.status.success()); true }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
            Err(error) => panic!("{compiler}: {error}"),
        }
    }).collect();
    if compilers.is_empty() {
        eprintln!("native pointer-snapshot execution requires a C compiler");
        return;
    }
    for n32 in [false, true] {
        for conditional in [false, true] {
            let words: &[u32] = if conditional {
                &[0xdc820000, 0x10c00002, 0, 0x70850019, 0x03e00008, 0x0046102d]
            } else {
                &[0xdc820000, 0x70850019, 0x03e00008, 0]
            };
            let prototype = if conditional {
                "unsigned long long probe(unsigned long long *pointer, unsigned long long value, unsigned long long extra)"
            } else {
                "unsigned long long probe(unsigned long long *pointer, unsigned long long value)"
            };
            let code = decompile(words, n32, prototype);
            let call = if conditional { "probe(&word,increment,extra)" } else { "probe(&word,increment)" };
            let source = format!(r#"
#include <stdint.h>
typedef uint64_t uint8;
static uint8 octeon_fetch_add64(uint8 *pointer,uint8 value) {{
    uint8 old = *pointer;
    *pointer = old + value;
    return old;
}}
{code}
int main(void) {{
    const uint8 values[] = {{0,7,UINT64_MAX,UINT64_C(0x8123456789abcdef)}};
    for (unsigned i = 0; i < 4; ++i) {{
        for (unsigned j = 0; j < 4; ++j) {{
            for (uint8 extra = 0; extra <= 3; extra += 3) {{
                uint8 old = values[i], increment = values[j], word = old;
                uint8 result = {call};
                uint8 expected = old + ({conditional} ? extra : 0);
                uint8 memory = old + ((!{conditional} || extra) ? increment : 0);
                if (result != expected || word != memory) return 1;
            }}
        }}
    }}
    return 0;
}}
"#, conditional = u8::from(conditional));
            let file = crate::common::scratch_file("octeon-pointer-snapshot", "c");
            let executable = file.with_extension("exe");
            std::fs::write(&file, source).unwrap();
            for compiler in &compilers {
                for optimization in ["-O0", "-O2"] {
                    let result = Command::new(compiler)
                        .args(["-std=c11", optimization, "-Werror=implicit-function-declaration", "-Werror=incompatible-pointer-types"])
                        .arg(&file).arg("-o").arg(&executable).output().unwrap();
                    assert!(result.status.success(), "{compiler}: {}\n{code}", String::from_utf8_lossy(&result.stderr));
                    let result = Command::new(&executable).output().unwrap();
                    assert!(result.status.success(), "{compiler} {optimization}: old value or updated memory differs: {code}");
                }
            }
        }
    }
}
