use crate::common;
use std::process::Command;
#[path = "fixtures/ppc32_import_fixture.rs"]
mod fixture;

fn decompile(bytes: &[u8], entry: &str, enabled: bool) -> String {
    let path = common::scratch_file("ppc32-call-contract", "elf");
    std::fs::write(&path, bytes).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", path.to_str().unwrap(), entry, "--mode", "fast", "--option", "ppc32picimports", if enabled { "on" } else { "off" }])
        .output().unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn terminal_calls_do_not_name_shared_stubs() {
    use object::Object;
    for (plt, little) in [fixture::Plt::Short, fixture::Plt::Long].into_iter().flat_map(|p| [false, true].map(|l| (p,l))) {
        let mut f = fixture::Fixture::new(plt);
        f.little = little;
        f.pic_collision = true;
        f.caller = Some(vec![if matches!(plt, fixture::Plt::Short) { 0x3bc02800 } else { 0x3fc00002 }, 0x480007fd, 0x38600007, 0x4e800020]);
        let bytes = f.bytes();
        let off = decompile(&bytes, "check", false);
        let on = decompile(&bytes, "check", true);
        assert!(off.contains("return 7;"), "{off}");
        assert!(on.contains("__stack_chk_fail("), "{on}");
        assert!(!on.contains("return 7;"), "{on}");
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert!(file.dynamic_relocations().is_some());
        let stub = decompile(&bytes, "0x1800", true);
        assert!(!stub.contains("__stack_chk_fail"), "{stub}");
    }
}

#[test]
fn returning_callers_are_left_unchanged() {
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = little;
        f.pic_collision = true;
        let bytes = f.bytes();
        assert_eq!(decompile(&bytes, "check", false), decompile(&bytes, "check", true));
    }
}

#[test]
fn callback_entries_keep_reachable_returns() {
    let mut cases = Vec::new();
    for name in ["base-spill", "local-fallthrough", "internal-link"] {
        cases.push((name, "check", "return 7;"));
    }
    for name in ["indirect-chain", "literal", "literal-halfword", "store-word-halfword", "shift-spill", "stack-bytes", "unproved-stub", "callee-output"] {
        cases.push((name, "check2", "return 7;"));
    }
    for name in ["tail-callback", "callee-callback", "callback-shift", "callback-call-clobber", "unsupported-rfi", "unsupported-tar", "string-store", "split-store", "unaligned-data", "composed-words", "vector-copy", "mapped-data", "note-valid", "resolver", "store-halfword", "stub-interior", "stub-interior-direct", "stub-interior-literal", "byte-composed", "lr-alias-returning"] {
        cases.push((name, "0x1004", "return;"));
    }
    cases.extend([("return-export", "0x1014", "return 7;"), ("float-copy", "0x1008", "return;")]);
    for (name, entry, expected) in cases {
        for little in [false, true] {
            let bytes = fixture::proof_gap(name, little);
            let on = decompile(&bytes, entry, true);
            assert!(on.contains(expected) && !on.contains("__stack_chk_fail(") && !on.contains("no-return"), "{name} little={little}: {on}");
            assert_eq!(on, decompile(&bytes, entry, false), "{name} little={little}");
        }
    }
}

#[test]
fn memory_spills_and_computed_returns_are_not_slot_proofs() {
    let spill = vec![0x7c0802a6,0x9421fff0,0x90010014,0x93c10008,
        0x3bc02800,0x93c10004,0x3bc02804,0x480007e5,
        0x83c10004,0x480007dd,0x38600007,0x83c10008,0x80010014,0x38210010,0x7c0803a6,0x4e800020];
    let dispatch = vec![0x7c0802a6,0x9421ffe0,0x90010024,0x93c10018,
        0x3bc02804,0x2c030000,0x41820010,0x3800102c,0x7c0803a6,0x4e800020,
        0x3bc02800,0x480007d5,0x38600007,0x83c10018,0x80010024,0x38210020,0x7c0803a6,0x4e800020];
    for code in [spill, dispatch] {
        for little in [false, true] {
            let mut f = fixture::Fixture::new(fixture::Plt::Short);
            f.pic_collision = true;
            f.little = little;
            f.caller = Some(code.clone());
            let bytes = f.bytes();
            let on = decompile(&bytes, "check", true);
            assert!(!on.contains("__stack_chk_fail("), "{on}");
            assert_eq!(on, decompile(&bytes, "check", false));
        }
    }
}

#[test]
fn direct_indirect_and_tail_calls_use_the_same_slot_proof() {
    for little in [false, true] {
        for code in [
            vec![0x3bc02800, 0x480007fd, 0x38600007, 0x4e800020],
            vec![0x3bc02800, 0x817e0800, 0x7d6903a6, 0x4e800421, 0x38600007, 0x4e800020],
            vec![0x3bc02800, 0x817e0800, 0x7d6903a6, 0x4e800420],
            vec![0x3bc02800, 0x480007fc],
        ] {
            let mut f = fixture::Fixture::new(fixture::Plt::Short);
            f.little = little;
            f.pic_collision = true;
            f.caller = Some(code.clone());
            let on = decompile(&f.bytes(), "check", true);
            assert!(on.contains("__stack_chk_fail("), "little={little}, code={code:x?}: {on}");
            assert!(!on.contains("return 7;"), "{on}");
        }
    }
}

#[test]
fn shared_call_instruction_is_scoped_to_its_entry() {
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = little;
        f.pic_collision = true;
        f.caller = Some(vec![0x3bc02800,0x480007fd,0x38600007,0x4e800020]);
        let bytes = f.bytes();
        assert!(decompile(&bytes, "check", true).contains("__stack_chk_fail("));
        let interior = decompile(&bytes, "0x1004", true);
        assert!(interior.contains("return 7;") && !interior.contains("__stack_chk_fail("), "{interior}");
    }
}

#[test]
fn slots_are_metadata_and_unwind_bounds_use_the_elf_byte_order() {
    use object::{Object, ObjectSection};
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = little;
        f.pic_collision = true;
        f.unwind_only = true;
        f.unwind_padding = 4;
        let bytes = f.bytes();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let slots = kuna_analysis::loader::kuna_ppc32_imports::pic_slots(&file);
        assert_eq!(slots.len(), 2);
        assert_eq!((slots[0].address, slots[0].name.as_str(), slots[0].no_return), (0x3000, "__stack_chk_fail", true));
        assert_eq!((slots[1].address, slots[1].name.as_str(), slots[1].no_return), (0x3004, "extra", false));
        assert!(kuna_analysis::loader::format::resolve_imports(&file, &bytes).is_empty());
        assert_eq!(kuna_analysis::analyzers::entry::kuna_fdeinterior::scan_fde_bodies(&file), vec![(0x1000, 0x103c)]);
        let offset = file.section_by_name(".eh_frame").unwrap().file_range().unwrap().0 as usize;
        let mut reversed = bytes.clone();
        reversed[offset + 32..offset + 36].reverse();
        reversed[offset + 36..offset + 40].reverse();
        let wrong = object::File::parse(reversed.as_slice()).unwrap();
        assert_ne!(kuna_analysis::analyzers::entry::kuna_fdeinterior::scan_fde_bodies(&wrong), vec![(0x1000, 0x103c)]);
        for (relocation, addend) in [(object::elf::R_PPC_ADDR32, 0), (object::elf::R_PPC_JMP_SLOT, 4)] {
            f.relocation = relocation;
            f.addend = addend;
            let bytes = f.bytes();
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert!(!kuna_analysis::loader::kuna_ppc32_imports::pic_slots(&file).iter().any(|s| s.address == 0x3000));
        }
    }
}

#[test]
fn call_names_and_xrefs_agree_without_renaming_functions() {
    let (returning, terminal) = fixture::cache_pair();
    let path = common::scratch_file("ppc32-scoped-xrefs", "elf");
    let run = |args: &[&str]| {
        let out = Command::new(env!("CARGO_BIN_EXE_kuna")).args(args).output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    };
    std::fs::write(&path, terminal).unwrap();
    let p = path.to_str().unwrap();
    assert_eq!(run(&["functions",p,"--json","--option","ppc32picimports","off"]), run(&["functions",p,"--json","--option","ppc32picimports","on"]));
    let refs = run(&["xrefs",p,"--to","__stack_chk_fail","--json","--option","ppc32picimports","on"]);
    let refs: serde_json::Value = serde_json::from_str(&refs).unwrap();
    assert!(!refs["xrefs"].as_array().unwrap().is_empty(), "{refs}");
    let off = run(&["decompile",p,"check","--mode","fast","--option","ppc32picimports","on","--option","noreturn_known","off"]);
    assert!(off.contains("return 7;") && !off.contains("__stack_chk_fail("), "{off}");
    for jobs in ["1","2"] {
        let all = run(&["decompile-all",p,"--mode","fast","--json","--jobs",jobs,"--option","ppc32picimports","on"]);
        let all: serde_json::Value = serde_json::from_str(&all).unwrap();
        let check = all["functions"].as_array().unwrap().iter().find(|f| f["name"] == "check").unwrap();
        assert!(check["code"].as_str().unwrap().contains("__stack_chk_fail("), "{check}");
    }
    std::fs::write(&path, returning).unwrap();
    let refs = run(&["xrefs",p,"--to","__stack_chk_fail","--json","--option","ppc32picimports","on"]);
    let refs: serde_json::Value = serde_json::from_str(&refs).unwrap();
    assert!(refs["xrefs"].as_array().unwrap().is_empty(), "{refs}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn shared_tail_calls_have_the_same_import_in_decompilation_and_xrefs() {
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = little;
        f.pic_collision = true;
        f.caller = Some(vec![0x3bc02800, 0x48000008]);
        f.second_caller = Some(vec![0x3bc02804, 0x480007f5, 0x38600007, 0x4e800020]);
        let bytes = f.bytes();
        assert!(decompile(&bytes, "check", true).contains("__stack_chk_fail("));
        assert_eq!(decompile(&bytes, "check2", false), decompile(&bytes, "check2", true));
        let path = common::scratch_file("ppc32-shared-tail-xrefs", "elf");
        std::fs::write(&path, bytes).unwrap();
        let query = |direction: &str, target: &str, enabled: bool| {
            let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args(["xrefs", path.to_str().unwrap(), direction, target, "--json", "--kind", "call",
                    "--mode", "fast", "--option", "ppc32picimports", if enabled { "on" } else { "off" }])
                .output().unwrap();
            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
            serde_json::from_slice::<serde_json::Value>(&out.stdout).unwrap()["xrefs"].as_array().unwrap().clone()
        };
        assert!(query("--to", "0x3000", false).is_empty());
        let incoming = query("--to", "__stack_chk_fail", true);
        assert_eq!(incoming.len(), 1, "little={little}: {incoming:?}");
        assert_eq!(incoming[0]["from_address"], 0x100c);
        assert_eq!(incoming[0]["to_address"], 0x3000);
        assert_eq!(incoming[0]["from_function"]["address"], 0x1000);
        assert_eq!(query("--to", "0x3000", true), incoming);
        assert!(query("--from", "check", true).iter().any(|r| r["from_address"] == 0x100c && r["to_address"] == 0x3000));
        assert!(!query("--from", "check2", true).iter().any(|r| r["to_address"] == 0x3000));
        let text = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["xrefs", path.to_str().unwrap(), "--to", "__stack_chk_fail", "--option", "ppc32picimports", "on"])
            .output().unwrap();
        assert!(text.status.success());
        assert!(String::from_utf8_lossy(&text.stdout).contains("check+0xc"));
        f.second_caller.as_mut().unwrap()[0] = 0x3bc02800;
        std::fs::write(&path, f.bytes()).unwrap();
        let shared = query("--to", "__stack_chk_fail", true);
        assert_eq!(shared.len(), 1);
        assert!(shared[0]["from_function"].is_null(), "{shared:?}");
        for (name, entry) in [("check", 0x1000), ("check2", 0x1008)] {
            let rows = query("--from", name, true);
            let call = rows.iter().find(|r| r["to_address"] == 0x3000).unwrap();
            assert_eq!(call["from_function"]["address"], entry);
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn api_call_bindings_are_entry_local_and_cleared() {
    let mut f = fixture::Fixture::new(fixture::Plt::Short);
    f.pic_collision = true;
    f.caller = Some(vec![0x3bc02800,0x480007fd,0x38600007,0x4e800020]);
    let path = common::scratch_file("ppc32-api", "elf");
    std::fs::write(&path, f.bytes()).unwrap();
    let specs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs").canonicalize().unwrap();
    let mut prog = kuna_console::engine::bootstrap_from_image(path.to_str().unwrap(), "", &[specs.to_string_lossy().into_owned()]).unwrap();
    prog.commit_pending_analysis().unwrap();
    prog.arch_mut().set_kuna_option("ppc32picimports", "on").unwrap();
    let entry = prog.lookup_symbol("check").unwrap();
    let mut fd = kuna_decomp::decompile_drive::build_and_follow_flow(prog.arch_mut(), "check", entry, 16).unwrap();
    assert_eq!(fd.ppc32_import_calls().count(), 1);
    let call = fd.ppc32_import_calls().next().unwrap();
    assert_eq!((call.site, call.target, call.import.address), (0x1004, Some(0x1800), 0x3000));
    assert!(fd.get_call_specs(0).proto().is_no_return());
    assert!(prog.function_named_at(0x1800).is_none_or(|name| name != "__stack_chk_fail"));
    fd.clear();
    assert_eq!(fd.ppc32_import_calls().count(), 0);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn every_path_to_a_call_must_agree_on_its_slot() {
    for little in [false, true] {
        for conflict in [false, true] {
            let mut f = fixture::Fixture::new(fixture::Plt::Short);
            f.pic_collision = true;
            f.little = little;
            f.caller = Some(vec![0x2c030000,0x4182000c,0x3bc02800,0x48000008,
                if conflict { 0x3bc02804 } else { 0x3bc02800 },
                0x480007ed,0x38600007,0x4e800020]);
            let bytes = f.bytes();
            let on = decompile(&bytes, "check", true);
            if conflict {
                assert!(!on.contains("__stack_chk_fail("), "{on}");
                assert_eq!(on, decompile(&bytes, "check", false));
            } else {
                assert!(on.contains("__stack_chk_fail(") && !on.contains("return 7;"), "{on}");
            }
        }
    }
}

#[test]
fn a_stack_derived_address_outside_the_frame_is_not_private_memory() {
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.pic_collision = true;
        f.little = little;
        // A stack-relative offset can wrap into mapped import data. It is not
        // a private spill merely because the address expression contains r1.
        f.caller = Some(vec![0x3bc02800,0x3d808000,0x618c3000,0x7d616214,
            0x906b0000,0x480007ed,0x38600007,0x4e800020]);
        let bytes = f.bytes();
        let on = decompile(&bytes, "check", true);
        assert!(!on.contains("__stack_chk_fail("), "{on}");
        assert_eq!(on, decompile(&bytes, "check", false));
    }
}

#[test]
fn imported_writes_invalidate_later_slot_reads() {
    for little in [false, true] {
        let bytes = fixture::slot_write(little);
        let off = decompile(&bytes, "check", false);
        let on = decompile(&bytes, "check", true);
        assert!(off.contains("return 7;"), "{off}");
        assert_eq!(on, off, "little={little}");
    }
}

#[test]
fn indirect_xrefs_include_decoded_instructions() {
    for little in [false, true] {
        for (transfer, text) in [(0x4e800421, "bctrl"), (0x4e800420, "bctr")] {
            let mut f = fixture::Fixture::new(fixture::Plt::Short);
            f.little = little;
            f.pic_collision = true;
            f.caller = Some(vec![0x3bc02800, 0x817e0800, 0x7d6903a6, transfer, 0x38600007, 0x4e800020]);
            let path = common::scratch_file("ppc32-indirect-xref-text", "elf");
            std::fs::write(&path, f.bytes()).unwrap();
            for (direction, target) in [("--from", "check"), ("--to", "__stack_chk_fail")] {
                for json in [false, true] {
                    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
                    cmd.args(["xrefs", path.to_str().unwrap(), direction, target, "--kind", "call",
                        "--mode", "fast", "--option", "ppc32picimports", "on"]);
                    if json { cmd.arg("--json"); }
                    let out = cmd.output().unwrap();
                    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
                    if json {
                        let result: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
                        let rows = result["xrefs"].as_array().unwrap();
                        let edge = rows.iter().find(|r| r["from_address"] == 0x100c && r["to_address"] == 0x3000).unwrap();
                        assert_eq!(edge["instruction"], text, "{edge}");
                    } else {
                        assert!(String::from_utf8_lossy(&out.stdout).contains(&format!("\t{text}\n")), "{}", String::from_utf8_lossy(&out.stdout));
                    }
                }
            }
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
fn call_clobbers_reach_slot_loads_at_joins() {
    for little in [false, true] {
        for code in [
            vec![0x3bc02804, 0x480007fd, 0x3bc02800, 0x817e0800, 0x7d6903a6, 0x4e800421, 0x38600007, 0x4e800020],
            vec![0x3bc02800, 0x2c030000, 0x41820010, 0x3bc02804, 0x480007f1, 0x3bc02800, 0x480007e9, 0x38600007, 0x4e800020],
        ] {
            let mut f = fixture::Fixture::new(fixture::Plt::Short);
            f.little = little;
            f.pic_collision = true;
            f.caller = Some(code);
            let bytes = f.bytes();
            let off = decompile(&bytes, "check", false);
            assert!(off.contains("return 7;"), "{off}");
            assert_eq!(decompile(&bytes, "check", true), off, "little={little}");
        }
    }
}

#[test]
fn targets_loaded_before_a_call_survive_only_in_preserved_registers() {
    for little in [false, true] {
        for (load, ctr, preserved) in [(0x81de0800, 0x7dc903a6, true), (0x819e0800, 0x7d8903a6, false)] {
            let mut f = fixture::Fixture::new(fixture::Plt::Short);
            f.little = little;
            f.pic_collision = true;
            f.caller = Some(vec![0x3bc02800, load, 0x3bc02804, 0x480007f5, ctr, 0x4e800421, 0x38600007, 0x4e800020]);
            let bytes = f.bytes();
            let on = decompile(&bytes, "check", true);
            if preserved {
                assert!(on.contains("extra(") && on.contains("__stack_chk_fail("), "{on}");
                assert!(!on.contains("return 7;"), "{on}");
            } else {
                assert_eq!(on, decompile(&bytes, "check", false));
                assert!(on.contains("return 7;"), "{on}");
            }
        }
    }
}

#[test]
fn distinct_stubs_do_not_hide_imported_slot_writes() {
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = little;
        f.pic_collision = true;
        f.import = "memcpy";
        f.second_stub = true;
        let mut code = vec![0x7c0802a6,0x9421ffe0,0x90010024,0x93c10018,
            0x3bc02800,0x39601100,0x91610008,0x38603004,0x38810008,0x38a00004,
            0x480007d9,0x3bde0008,0x480007e1,0x38600007,0x83c10018,
            0x80010024,0x38210020,0x7c0803a6,0x4e800020];
        code.resize(0x100/4,0x60000000);
        code.extend([0x38600009,0x4e800020]);
        f.caller = Some(code);
        let mut bytes = f.bytes();
        let off = bytes.windows(6).position(|b| b == b"extra\0").unwrap();
        bytes[off..off+6].copy_from_slice(b"abort\0");
        let on = decompile(&bytes, "check", true);
        assert!(on.contains("return 7;") && !on.contains("abort("), "{on}");
        assert_eq!(on, decompile(&bytes, "check", false));
    }
}
