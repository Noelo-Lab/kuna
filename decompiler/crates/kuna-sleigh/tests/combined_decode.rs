//! Paired decode must preserve text, p-code and context while avoiding a second
//! image read for instructions that have no context commits or delay slots.
use kuna_base::{
    address::Address,
    error::{KunaError, KunaResult},
};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use kuna_sleigh::{
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    sleigh::Sleigh,
    translate::{AssemblyEmit, PcodeEmit, Translate},
};
use std::{cell::Cell, path::PathBuf, rc::Rc};

const BASE: u64 = 0x1000;
struct Bytes {
    data: Vec<u8>,
    reads: Rc<Cell<usize>>,
    fail: bool,
}
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-combined-decode"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        self.reads.set(self.reads.get() + 1);
        if self.fail {
            return Err(KunaError::data_unavail("unmapped instruction"));
        }
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = addr
                .get_offset()
                .checked_sub(BASE)
                .and_then(|off| self.data.get(off as usize + i))
                .copied()
                .unwrap_or(0);
        }
        Ok(())
    }
}

fn engine(spec: &str, bytes: &[u8], context: &[(&str, u32)]) -> (Sleigh, Rc<Cell<usize>>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sla = std::fs::read(root.join("specs/Ghidra/Processors").join(spec)).unwrap();
    let reads = Rc::new(Cell::new(0));
    let image = Bytes {
        data: bytes.to_vec(),
        reads: Rc::clone(&reads),
        fail: false,
    };
    let mut engine = Sleigh::new(Box::new(image), Box::new(ContextInternal::new()));
    engine.initialize_from_sla(&sla).unwrap();
    for &(name, value) in context {
        engine.set_context_default(name, value);
    }
    (engine, reads)
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Text(Option<(String, String)>);
impl AssemblyEmit for Text {
    fn dump(&mut self, _: &Address, mnemonic: &str, body: &str) {
        self.0 = Some((mnemonic.into(), body.into()));
    }
}
#[derive(Default, Debug, PartialEq, Eq)]
struct Ops(Vec<String>);
impl PcodeEmit for Ops {
    fn dump(&mut self, _: &Address, op: OpCode, out: Option<&VarnodeData>, vars: &[VarnodeData]) {
        let var = |v: &VarnodeData| {
            (
                v.space.as_ref().map(|s| s.get_name().to_string()),
                v.offset,
                v.size,
            )
        };
        self.0.push(format!(
            "{op:?} {:?} {:?}",
            out.map(var),
            vars.iter().map(var).collect::<Vec<_>>()
        ));
    }
}
fn addr(engine: &Sleigh, offset: u64) -> Address {
    Address::new(
        Rc::clone(engine.base().manager().get_space_by_name("ram").unwrap()),
        BASE + offset,
    )
}

fn compare(spec: &str, bytes: &[u8], context: &[(&str, u32)], offsets: &[u64]) -> (usize, usize) {
    let (old, old_reads) = engine(spec, bytes, context);
    let (new, new_reads) = engine(spec, bytes, context);
    for &offset in offsets {
        let a = addr(&old, offset);
        let b = addr(&new, offset);
        let mut old_ops = Ops::default();
        let mut new_ops = Ops::default();
        let mut old_text = Text::default();
        let mut new_text = Text::default();
        let old_len = old
            .one_instruction(&mut old_ops, &a)
            .map_err(|e| format!("{e:?}"));
        if old_len.is_ok() {
            let _ = old.print_assembly(&mut old_text, &a);
        }
        let translate: &dyn Translate = &new;
        let new_len = translate
            .one_instruction_with_assembly(&mut new_ops, &mut new_text, &b)
            .map_err(|e| format!("{e:?}"));
        assert_eq!(new_len, old_len, "{spec} at {offset:#x}");
        assert!(
            new_len.is_ok(),
            "case must actually decode: {spec} at {offset:#x}"
        );
        assert_eq!(new_ops, old_ops, "p-code: {spec} at {offset:#x}");
        assert_eq!(new_text, old_text, "assembly: {spec} at {offset:#x}");
        for target in 0..bytes.len() as u64 {
            let old_context =
                old.with_context_db_mut(|db| db.get_context(&addr(&old, target)).to_vec());
            let new_context =
                new.with_context_db_mut(|db| db.get_context(&addr(&new, target)).to_vec());
            assert_eq!(new_context, old_context, "context at {target:#x}");
        }
    }
    (old_reads.get(), new_reads.get())
}

#[test]
fn plain_arm_instructions_need_one_parse_each() {
    let (old, new) = compare(
        "ARM/data/languages/ARM7_le.sla",
        &[0, 0xf0, 0x20, 0xe3, 7, 0, 0xa0, 0xe3, 1, 0, 0x80, 0xe2],
        &[("TMode", 0), ("LRset", 0)],
        &[0, 4, 8],
    );
    assert_eq!((old, new), (6, 3));
}

#[test]
fn arm_context_commits_keep_the_post_lift_assembly_parse() {
    let (old, new) = compare(
        "ARM/data/languages/ARM7_le.sla",
        &[0, 0, 0, 0xfa, 0, 0xf0, 0x20, 0xe3, 1, 0x20, 0x70, 0x47],
        &[("TMode", 0), ("LRset", 0)],
        &[0, 8],
    );
    assert!(new < old);
    assert!(
        new > 2,
        "the BLX context-writing instruction must use the fallback"
    );
}

#[test]
fn thumb_it_context_and_text_match() {
    compare(
        "ARM/data/languages/ARM7_le.sla",
        &[8, 0xbf, 1, 0x20, 0x70, 0x47],
        &[("TMode", 1), ("LRset", 0)],
        &[0, 2, 4],
    );
}

#[test]
fn variable_length_x86_text_and_flow_match() {
    let (old, new) = compare(
        "x86/data/languages/x86-64.sla",
        &[0x48, 0x01, 0xfe, 0xb9, 0x44, 0x33, 0x22, 0x11, 0xc3],
        &[("addrsize", 2), ("opsize", 1), ("longMode", 1)],
        &[0, 3, 8],
    );
    assert_eq!((old, new), (6, 3));
}

#[test]
fn delay_slots_keep_the_original_length_and_assembly() {
    let (old, new) = compare(
        "Sparc/data/languages/SparcV9_32.sla",
        &[0x81, 0xc7, 0xe0, 8, 0x81, 0xe8, 0, 0],
        &[],
        &[0],
    );
    assert_eq!(old, new, "delay-slot assembly must use the fallback");
    assert!(
        old >= 3,
        "decode the instruction, its slot, and its assembly"
    );
}

#[test]
fn failed_lift_does_not_emit_assembly_or_redecode() {
    let (mut engine, reads) = engine("ARM/data/languages/ARM7_le.sla", &[], &[("TMode", 0)]);
    engine.set_loader(Box::new(Bytes {
        data: Vec::new(),
        reads: Rc::clone(&reads),
        fail: true,
    }));
    let mut text = Text::default();
    assert!(engine
        .one_instruction_with_assembly(&mut Ops::default(), &mut text, &addr(&engine, 0))
        .is_err());
    assert_eq!(text, Text::default());
    assert_eq!(reads.get(), 1);
}

#[test]
fn mips_mode_switch_and_delay_slot_keep_the_sequential_result() {
    let (old, new) = compare(
        "MIPS/data/languages/mips32be.sla",
        &[0x74, 0, 4, 2, 0, 0, 0, 0, 0xec, 0x98, 0xea, 0x12],
        &[("ISA_MODE", 0), ("RELP", 1), ("REL6", 0)],
        &[0, 8],
    );
    assert!(new < old);
    assert!(
        new >= 4,
        "JALX must retain its delay-slot and assembly parses"
    );
}

#[test]
fn repeated_relative_branches_are_resolved_at_each_address() {
    let bytes = [0, 0, 0, 0xea].repeat(16);
    compare(
        "ARM/data/languages/ARM7_le.sla",
        &bytes,
        &[("TMode", 0), ("LRset", 0)],
        &[0, 4, 8, 12],
    );
    let (engine, _) = engine(
        "ARM/data/languages/ARM7_le.sla",
        &bytes,
        &[("TMode", 0), ("LRset", 0)],
    );
    let mut first = Ops::default();
    let mut second = Ops::default();
    engine
        .one_instruction_with_assembly(&mut first, &mut Text::default(), &addr(&engine, 0))
        .unwrap();
    engine
        .one_instruction_with_assembly(&mut second, &mut Text::default(), &addr(&engine, 4))
        .unwrap();
    assert_ne!(
        first, second,
        "equal encodings have different PC-relative targets"
    );
}

#[test]
fn repeated_bytes_respect_changed_context_and_replaced_images() {
    let bytes = [0, 0xf0, 0x20, 0xe3].repeat(8);
    let (mut cached, _) = engine("ARM/data/languages/ARM7_le.sla", &bytes, &[("TMode", 0)]);
    cached
        .one_instruction_with_assembly(&mut Ops::default(), &mut Text::default(), &addr(&cached, 0))
        .unwrap();
    for mode in [1, 0] {
        cached.set_context_default("TMode", mode);
        let (fresh, _) = engine("ARM/data/languages/ARM7_le.sla", &bytes, &[("TMode", mode)]);
        let mut got = Text::default();
        let mut want = Text::default();
        let a =
            cached.one_instruction_with_assembly(&mut Ops::default(), &mut got, &addr(&cached, 0));
        let b = fresh.one_instruction(&mut Ops::default(), &addr(&fresh, 0));
        if b.is_ok() {
            let _ = fresh.print_assembly(&mut want, &addr(&fresh, 0));
        }
        assert_eq!(
            a.map_err(|e| format!("{e:?}")),
            b.map_err(|e| format!("{e:?}"))
        );
        assert_eq!(got, want);
    }
    cached.set_loader(Box::new(Bytes {
        data: [7, 0, 0xa0, 0xe3].repeat(8),
        reads: Rc::new(Cell::new(0)),
        fail: false,
    }));
    let mut got = Text::default();
    cached
        .one_instruction_with_assembly(&mut Ops::default(), &mut got, &addr(&cached, 0))
        .unwrap();
    assert!(got.0.unwrap().0.starts_with("mov"));
}

#[test]
fn reusable_lifts_require_local_reads_and_no_context_commits() {
    for (spec, bytes, context, reusable) in [
        (
            "ARM/data/languages/ARM7_le.sla",
            vec![1, 0, 0xa0, 0xe3],
            vec![("TMode", 0), ("LRset", 0)],
            true,
        ),
        (
            "ARM/data/languages/ARM7_le.sla",
            vec![0, 0, 0, 0xfa],
            vec![("TMode", 0), ("LRset", 0)],
            false,
        ),
        (
            "ARM/data/languages/ARM7_le.sla",
            vec![8, 0xbf, 1, 0x20],
            vec![("TMode", 1), ("LRset", 0)],
            false,
        ),
        (
            "Sparc/data/languages/SparcV9_32.sla",
            vec![0x81, 0xc7, 0xe0, 8, 0x81, 0xe8, 0, 0],
            vec![],
            false,
        ),
        (
            "Toy/data/languages/toy_le.sla",
            vec![0, 0x80, 1, 0],
            vec![],
            false,
        ),
    ] {
        let (old, _) = engine(spec, &bytes, &context);
        let (new, _) = engine(spec, &bytes, &context);
        let mut expected_ops = Ops::default();
        let mut expected_text = Text::default();
        let len = old
            .one_instruction_with_assembly(&mut expected_ops, &mut expected_text, &addr(&old, 0))
            .unwrap();
        let mut ops = Ops::default();
        let mut text = Text::default();
        let mut key = Some(vec![u32::MAX]);
        let actual = (&new as &dyn Translate)
            .one_instruction_reusable(&mut ops, &mut text, &addr(&new, 0), &mut key)
            .unwrap();
        assert_eq!(actual, len);
        assert_eq!(ops, expected_ops);
        assert_eq!(text, expected_text);
        assert_eq!(key.is_some(), reusable, "{spec}: {bytes:x?}");
        if let Some(key) = key {
            assert_eq!(
                key,
                new.with_context_db_mut(|db| db.get_context(&addr(&new, 0)).to_vec())
            );
        }
    }
}

#[test]
fn reuse_keys_track_effective_context_read_overrides() {
    let (engine, _) = engine(
        "ARM/data/languages/ARM7_le.sla",
        &[1, 0, 0xa0, 0xe3],
        &[("TMode", 0), ("LRset", 0)],
    );
    let at = addr(&engine, 0);
    let (word, mask) = engine.with_context_db_mut(|db| {
        let var = db.get_variable(b"TMode").unwrap();
        (var.get_word() as usize, var.get_mask() << var.get_shift())
    });
    let decode = || {
        let mut text = Text::default();
        let mut key = None;
        let len = engine.one_instruction_reusable(
            &mut Ops::default(), &mut text, &at, &mut key,
        ).unwrap();
        (len, text.0.unwrap().0, key.unwrap())
    };
    let (len, mnemonic, arm_key) = decode();
    assert_eq!((len, mnemonic.as_str()), (4, "mov"));
    assert!(engine.matches_decode_context(&at, &arm_key));

    engine.set_context_read_override(word, mask, mask);
    let (len, mnemonic, thumb_key) = decode();
    assert_eq!((len, mnemonic.as_str()), (2, "movs"));
    assert!(!engine.matches_decode_context(&at, &arm_key));
    assert_ne!(thumb_key, arm_key);
    assert!(engine.matches_decode_context(&at, &thumb_key));

    engine.with_context_db_mut(|db| db.set_variable_default(b"TMode", 1).unwrap());
    engine.set_context_read_override(word, 0, 0);
    assert!(engine.matches_decode_context(&at, &thumb_key));
    engine.set_context_read_override(word, mask, 0);
    assert!(engine.matches_decode_context(&at, &arm_key));
    assert!(!engine.matches_decode_context(&at, &thumb_key));
    assert_eq!(decode().2, arm_key);
}

#[test]
fn query_reuse_retains_assembly_only_probes_and_promotes_them_to_pcode() {
    let (engine, reads) = engine("ARM/data/languages/ARM7_le.sla",
        &[1, 0, 0xa0, 0xe3], &[("TMode", 0)]);
    let at = addr(&engine, 0);
    let mut expected_ops = Ops::default();
    let mut expected_text = Text::default();
    engine.one_instruction_with_assembly(&mut expected_ops, &mut expected_text, &at).unwrap();
    let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    engine.print_assembly(&mut Text::default(), &at).unwrap();
    let warm = reads.get();
    let mut text = Text::default();
    engine.print_assembly(&mut text, &at).unwrap();
    assert_eq!(text, expected_text);
    assert_eq!(reads.get(), warm);
    let mut ops = Ops::default();
    engine.one_instruction(&mut ops, &at).unwrap();
    assert_eq!(ops, expected_ops);
    assert!(reads.get() > warm);
    let promoted = reads.get();
    let mut ops = Ops::default();
    engine.one_instruction_with_assembly(&mut ops, &mut Text::default(), &at).unwrap();
    assert_eq!(ops, expected_ops);
    assert_eq!(reads.get(), promoted);
}

#[test]
fn query_reuse_renders_a_pcode_only_hit_without_a_nested_cache_borrow() {
    let (engine, _) = engine("ARM/data/languages/ARM7_le.sla",
        &[1, 0, 0xa0, 0xe3], &[("TMode", 0)]);
    let at = addr(&engine, 0);
    let mut expected = Text::default();
    engine.print_assembly(&mut expected, &at).unwrap();
    let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    engine.one_instruction(&mut Ops::default(), &at).unwrap();
    let mut text = Text::default();
    engine.one_instruction_with_assembly(&mut Ops::default(), &mut text, &at).unwrap();
    assert_eq!(text, expected);
}

#[test]
fn query_reuse_preserves_pcode_text_context_and_lifetime() {
    let (engine, reads) = engine("ARM/data/languages/ARM7_le.sla",
        &[1, 0, 0xa0, 0xe3, 0, 0, 0, 0xfa], &[("TMode", 0), ("LRset", 0)]);
    let at = addr(&engine, 0);
    let decode = || {
        let mut ops = Ops::default();
        let len = engine.one_instruction(&mut ops, &at).unwrap();
        let mut text = Text::default();
        engine.print_assembly(&mut text, &at).unwrap();
        (len, ops, text)
    };
    let expected = decode();
    let scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    assert_eq!(decode(), expected);
    let warm_reads = reads.get();
    assert_eq!(decode(), expected);
    assert_eq!(reads.get(), warm_reads);
    let (word, mask) = engine.with_context_db_mut(|db| {
        let var = db.get_variable(b"TMode").unwrap();
        (var.get_word() as usize, var.get_mask() << var.get_shift())
    });
    engine.set_context_read_override(word, mask, mask);
    assert_eq!(decode().0, 2);
    assert!(reads.get() > warm_reads);
    engine.set_context_read_override(word, 0, 0);
    let thumb_reads = reads.get();
    assert_eq!(decode(), expected);
    assert_eq!(reads.get(), thumb_reads);
    engine.one_instruction(&mut Ops::default(), &addr(&engine, 4)).unwrap();
    assert!(!engine.last_context_commits().is_empty());
    assert_eq!(decode(), expected);
    assert!(engine.last_context_commits().is_empty());
    drop(scope);
    let before = reads.get();
    assert_eq!(decode(), expected);
    assert!(reads.get() > before);
}

#[test]
fn query_reuse_does_not_cache_context_writes_or_delayed_decodes() {
    for (spec, bytes, context) in [
        ("ARM/data/languages/ARM7_le.sla", vec![0,0,0,0xfa], vec![("TMode",0),("LRset",0)]),
        ("ARM/data/languages/ARM7_le.sla", vec![8,0xbf,1,0x20], vec![("TMode",1),("LRset",0)]),
        ("Sparc/data/languages/SparcV9_32.sla",vec![0x81,0xc7,0xe0,8,0x81,0xe8,0,0],vec![]),
    ] {
        let (engine, reads) = engine(spec,&bytes,&context);
        let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
        engine.allow_context_set(false);
        for _ in 0..2 {
            let before = reads.get();
            engine.one_instruction(&mut Ops::default(), &addr(&engine,0)).unwrap();
            assert!(reads.get() > before, "{spec}: {bytes:x?}");
            let before = reads.get();
            engine.print_assembly(&mut Text::default(), &addr(&engine,0)).unwrap();
            engine.print_assembly(&mut Text::default(), &addr(&engine,0)).unwrap();
            assert!(reads.get() >= before + 2, "assembly {spec}: {bytes:x?}");
        }
    }
}

#[test]
fn query_reuse_keys_include_address_and_every_context_word() {
    let (engine, reads) = engine("ARM/data/languages/ARM7_le.sla",
        &[1,0,0xa0,0xe3,1,0,0xa0,0xe3], &[("TMode",0),("LRset",0)]);
    let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    engine.one_instruction(&mut Ops::default(), &addr(&engine,0)).unwrap();
    let before = reads.get();
    engine.one_instruction(&mut Ops::default(), &addr(&engine,4)).unwrap();
    assert!(reads.get() > before);
    let before = reads.get();
    engine.with_context_db_mut(|db| db.set_variable_default(b"LRset",1).unwrap());
    engine.one_instruction(&mut Ops::default(), &addr(&engine,0)).unwrap();
    assert!(reads.get() > before);
}

#[test]
fn too_small_query_cache_does_not_retain_a_decode() {
    let (engine, reads) = engine("ARM/data/languages/ARM7_le.sla",
        &[1,0,0xa0,0xe3], &[("TMode",0),("LRset",0)]);
    let _scope = engine.decode_reuse_scope(1).unwrap();
    for _ in 0..2 {
        let before = reads.get();
        engine.one_instruction(&mut Ops::default(), &addr(&engine,0)).unwrap();
        assert!(reads.get() > before);
    }
}

#[test]
fn query_reuse_does_not_retain_failed_decodes() {
    let (mut engine, reads) = engine("ARM/data/languages/ARM7_le.sla", &[], &[("TMode",0)]);
    engine.set_loader(Box::new(Bytes { data: Vec::new(), reads: reads.clone(), fail: true }));
    let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    for expected in 1..=2 {
        let mut ops = Ops::default();
        let mut text = Text::default();
        assert!(engine.one_instruction_with_assembly(&mut ops, &mut text, &addr(&engine,0)).is_err());
        assert_eq!(ops, Ops::default());
        assert_eq!(text, Text::default());
        assert_eq!(reads.get(), expected);
    }
}

#[test]
fn cached_decode_still_checks_the_entire_mapped_instruction() {
    #[derive(Debug)]
    struct Mapped(u64);
    impl kuna_sleigh::loadimage::ImageBytes for Mapped {
        fn fill_span(&self, _: &mut [u8], _: u64) -> usize { unreachable!() }
        fn mapped_covers(&self, lo: u64, hi: u64) -> bool { lo >= BASE && hi <= BASE + self.0 }
    }
    let (engine, reads) = engine("ARM/data/languages/ARM7_le.sla",
        &[1,0,0xa0,0xe3], &[("TMode",0),("LRset",0)]);
    let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    engine.one_instruction(&mut Ops::default(), &addr(&engine,0)).unwrap();
    let before = reads.get();
    let mut ops = Ops::default();
    assert!(engine.one_instruction_checked(&mut ops, &addr(&engine,0), &Mapped(2)).is_err());
    assert_eq!(ops, Ops::default());
    assert!(engine.one_instruction_checked(&mut ops, &addr(&engine,0), &Mapped(4)).is_ok());
    assert_eq!(reads.get(), before);
}

#[test]
fn cached_decode_retains_stateful_loaders_failure_behavior() {
    struct Windowed { bytes: Bytes, window: Rc<Cell<u64>> }
    impl LoadImage for Windowed {
        fn get_file_name(&self) -> &str { "synthetic-windowed-decode" }
        fn get_arch_type(&self) -> Vec<u8> { Vec::new() }
        fn adjust_vma(&mut self, _: i64) {}
        fn read_window(&self) -> Option<u64> { Some(self.window.get()) }
        fn load_fill(&mut self, out: &mut [u8], at: &Address) -> KunaResult<()> {
            if self.window.get() != BASE { return Err(KunaError::data_unavail("unmapped window")); }
            self.bytes.load_fill(out, at)
        }
    }
    let (mut engine, reads) = engine("ARM/data/languages/ARM7_le.sla", &[], &[("TMode",0),("LRset",0)]);
    let window = Rc::new(Cell::new(BASE));
    engine.set_loader(Box::new(Windowed {
        bytes: Bytes { data: vec![1,0,0xa0,0xe3], reads, fail: false }, window: window.clone(),
    }));
    let _scope = engine.decode_reuse_scope(32 * 1024 * 1024).unwrap();
    engine.one_instruction(&mut Ops::default(), &addr(&engine,0)).unwrap();
    window.set(0x2000);
    let mut ops = Ops::default();
    assert!(engine.one_instruction(&mut ops, &addr(&engine,0)).is_err());
    assert_eq!(ops, Ops::default());
    let mut text = Text::default();
    assert!(engine.print_assembly(&mut text, &addr(&engine,0)).is_err());
    assert_eq!(text, Text::default());
}
