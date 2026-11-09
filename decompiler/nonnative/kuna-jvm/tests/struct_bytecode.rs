//! The byte cursor, bytecode parser and class-file model on hand-assembled
//! classes: the per-version quirks and malformed inputs `javac` never emits.

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::code_constants::*;
use kuna_jvm::code_type::CodeType;
use kuna_jvm::data_point::DataPoint;
use kuna_jvm::instruction::Instruction;
use kuna_jvm::instruction_impact::InstructionImpact;
use kuna_jvm::rt::{try_catch, JavaHash};

use crate::struct_classfile::end_class_scope;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_general_attribute::StructGeneralAttribute;
use kuna_jvm::struct_member::StructMember;
use kuna_jvm::struct_method::StructMethod;

#[derive(Default)]
struct Pool {
    bytes: Vec<u8>,
    count: u16,
}

impl Pool {
    fn push(&mut self, entry: &[u8], slots: u16) -> u16 {
        let index = self.count + 1;
        self.bytes.extend_from_slice(entry);
        self.count += slots;
        index
    }

    fn utf8(&mut self, s: &str) -> u16 {
        let mut e = vec![1];
        e.extend_from_slice(&(s.len() as u16).to_be_bytes());
        e.extend_from_slice(s.as_bytes());
        self.push(&e, 1)
    }

    fn class(&mut self, name: &str) -> u16 {
        let n = self.utf8(name);
        self.push(&[&[7u8][..], &n.to_be_bytes()].concat(), 1)
    }

    fn nat(&mut self, name: &str, desc: &str) -> u16 {
        let n = self.utf8(name);
        let d = self.utf8(desc);
        self.push(
            &[&[12u8][..], &n.to_be_bytes(), &d.to_be_bytes()].concat(),
            1,
        )
    }

    fn member_ref(&mut self, tag: u8, class: &str, name: &str, desc: &str) -> u16 {
        let c = self.class(class);
        let nt = self.nat(name, desc);
        self.push(
            &[&[tag][..], &c.to_be_bytes(), &nt.to_be_bytes()].concat(),
            1,
        )
    }

    fn long(&mut self, v: i64) -> u16 {
        self.push(&[&[5u8][..], &v.to_be_bytes()].concat(), 2)
    }
}

fn u2(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

fn attribute(pool: &mut Pool, name: &str, body: &[u8]) -> Vec<u8> {
    let n = pool.utf8(name);
    [&u2(n)[..], &(body.len() as u32).to_be_bytes(), body].concat()
}

fn attribute_raw_length(pool: &mut Pool, name: &str, length: u32, body: &[u8]) -> Vec<u8> {
    let n = pool.utf8(name);
    [&u2(n)[..], &length.to_be_bytes(), body].concat()
}

fn table(entries: &[Vec<u8>]) -> Vec<u8> {
    let mut out = u2(entries.len() as u16).to_vec();
    for e in entries {
        out.extend_from_slice(e);
    }
    out
}

struct Code<'a> {
    max_locals: u16,
    code: &'a [u8],
    handlers: &'a [[u16; 4]],
    attributes: Vec<Vec<u8>>,
    predates_java: bool,
}

fn code_attribute(pool: &mut Pool, c: Code) -> Vec<u8> {
    let mut body = Vec::new();
    if c.predates_java {
        body.push(2);
        body.push(c.max_locals as u8);
        body.extend_from_slice(&u2(c.code.len() as u16));
    } else {
        body.extend_from_slice(&u2(2));
        body.extend_from_slice(&u2(c.max_locals));
        body.extend_from_slice(&(c.code.len() as u32).to_be_bytes());
    }
    body.extend_from_slice(c.code);
    body.extend_from_slice(&u2(c.handlers.len() as u16));
    for h in c.handlers {
        for v in h {
            body.extend_from_slice(&u2(*v));
        }
    }
    body.extend_from_slice(&table(&c.attributes));
    attribute(pool, "Code", &body)
}

fn member(pool: &mut Pool, flags: u16, name: &str, desc: &str, attributes: &[Vec<u8>]) -> Vec<u8> {
    let n = pool.utf8(name);
    let d = pool.utf8(desc);
    [&u2(flags)[..], &u2(n), &u2(d), &table(attributes)].concat()
}

/// A class `T` (super `java/lang/Object`) with the given members and class attributes.
fn class_file(
    major: u16,
    minor: u16,
    pool: Pool,
    fields: &[Vec<u8>],
    methods: &[Vec<u8>],
    attributes: &[Vec<u8>],
) -> Vec<u8> {
    let mut pool = pool;
    let this = pool.class("T");
    let sup = pool.class("java/lang/Object");
    let mut out = vec![0xCA, 0xFE, 0xBA, 0xBE];
    out.extend_from_slice(&u2(minor));
    out.extend_from_slice(&u2(major));
    out.extend_from_slice(&u2(pool.count + 1));
    out.extend_from_slice(&pool.bytes);
    out.extend_from_slice(&u2(0x21));
    out.extend_from_slice(&u2(this));
    out.extend_from_slice(&u2(sup));
    out.extend_from_slice(&u2(0));
    out.extend_from_slice(&table(fields));
    out.extend_from_slice(&table(methods));
    out.extend_from_slice(&table(attributes));
    out
}

/// `StructClass.create` over the bytes.
fn create(bytes: Vec<u8>, own: bool) -> StructClass {
    crate::struct_classfile::create(bytes, own)
}

/// One static method `m()V` with this code; returns the parsed method, expanded.
fn single_method(
    major: u16,
    minor: u16,
    code: &[u8],
    handlers: &[[u16; 4]],
) -> (StructClass, StructMethod) {
    let mut pool = Pool::default();
    let code = code_attribute(
        &mut pool,
        Code {
            max_locals: 3,
            code,
            handlers,
            attributes: Vec::new(),
            predates_java: major <= 45 && minor <= 2,
        },
    );
    let m = member(&mut pool, 0x8, "m", "()V", &[code]);
    let cl = create(class_file(major, minor, pool, &[], &[m], &[]), true);
    let mt = cl.get_method("m", "()V").unwrap();
    mt.expand_data(cl);
    (cl, mt)
}

/// `(offset, opcode, wide, group, length, operands)` of every instruction.
fn shape(mt: StructMethod) -> Vec<(i32, i32, bool, i32, i32, Vec<i32>)> {
    mt.get_instruction_sequence()
        .unwrap()
        .iter()
        .map(|i| {
            (
                i.start_offset(),
                i.opcode(),
                i.wide(),
                i.group(),
                i.length(),
                i.operands().unwrap_or_default(),
            )
        })
        .collect()
}

#[test]
fn byte_cursor_reads_like_data_input_stream() {
    let mut c = ByteCursor::new(vec![
        0xFF, 0xFF, 0x80, 0x00, 0x80, 0x00, 0x00, 0x01, 0x3F, 0xC0, 0x00, 0x00, 0x00, 0x03, 0xC0,
        0x80, b'a', 0x12, 0x34,
    ]);
    assert_eq!(c.read_unsigned_byte(), 255);
    assert_eq!(c.read_byte(), -1);
    assert_eq!(c.read_short(), i16::MIN);
    assert_eq!(c.read_int(), i32::MIN + 1);
    assert_eq!(c.read_float(), 1.5);
    assert_eq!(c.read_utf(), "\0a");
    assert_eq!((c.position(), c.remaining()), (17, 2));
    assert_eq!(c.skip_bytes(5), 2);
    assert_eq!(c.skip_bytes(-1), 0);
    let caught = try_catch(|| c.read_unsigned_byte()).unwrap_err();
    assert!(caught.is("IOException"));

    let mut c = ByteCursor::new(vec![
        0, 0, 0, 0, 0, 0, 0, 7, 0x40, 0x09, 0x21, 0xFB, 0x54, 0x44, 0x2D, 0x18, 1, 2, 3,
    ]);
    assert_eq!(c.read_long(), 7);
    assert_eq!(c.read_double(), std::f64::consts::PI);
    assert_eq!(c.read(2), vec![1, 2]);
    assert!(try_catch(|| c.read(2)).unwrap_err().is("IOException"));
    assert!(try_catch(|| c.discard(2)).unwrap_err().is("IOException"));
    c.discard(1);
    assert_eq!(c.remaining(), 0);
    assert!(try_catch(|| ByteCursor::new(vec![0, 1, 2]).read_int())
        .unwrap_err()
        .is("IOException"));
}

#[test]
fn invokedynamic_operands_depend_on_the_class_version() {
    let code = [0xBA, 0x00, 0x05, 0x00, 0x00, 0xB1];
    let (_, java6) = single_method(50, 0, &code, &[]);
    assert_eq!(
        shape(java6),
        vec![
            (0, OPC_INVOKEDYNAMIC, false, GROUP_GENERAL, 1, vec![]),
            (1, OPC_NOP, false, GROUP_GENERAL, 1, vec![]),
            (2, OPC_BIPUSH, false, GROUP_GENERAL, 1, vec![2]),
            (3, OPC_NOP, false, GROUP_GENERAL, 1, vec![]),
            (4, OPC_NOP, false, GROUP_GENERAL, 1, vec![]),
            (5, OPC_RETURN, false, GROUP_RETURN, 1, vec![]),
        ]
    );
    let (_, java7) = single_method(51, 0, &code, &[]);
    assert_eq!(
        shape(java7),
        vec![
            (0, OPC_INVOKEDYNAMIC, false, GROUP_INVOCATION, 5, vec![5]),
            (5, OPC_RETURN, false, GROUP_RETURN, 1, vec![]),
        ]
    );
}

#[test]
fn subroutines_wide_forms_and_long_jumps() {
    let code = [
        0xC9, 0x00, 0x00, 0x00, 0x10, // 0: jsr_w +16
        0xC8, 0x00, 0x00, 0x00, 0x14, // 5: goto_w +20
        0xC4, 0x84, 0x01, 0x2C, 0xFF, 0xFE, // 10: wide iinc 300, -2
        0x4C, // 16: astore_1
        0xC4, 0xA9, 0x01, 0x2C, // 17: wide ret 300
        0xC4, 0x19, 0x01, 0x00, // 21: wide aload 256
        0x02, // 25: iconst_m1
        0x11, 0xFF, 0x38, // 26: sipush -200
        0x10, 0x80, // 29: bipush -128
        0xA7, 0xFF, 0xFB, // 31: goto -5
        0xB1, // 34: return
    ];
    let (_, mt) = single_method(49, 0, &code, &[]);
    assert_eq!(
        shape(mt),
        vec![
            (0, OPC_JSR, false, GROUP_JUMP, 5, vec![16]),
            (5, OPC_GOTO, false, GROUP_JUMP, 5, vec![20]),
            (10, OPC_IINC, true, GROUP_GENERAL, 6, vec![300, -2]),
            (16, OPC_ASTORE, false, GROUP_GENERAL, 1, vec![1]),
            (17, OPC_RET, true, GROUP_RETURN, 4, vec![300]),
            (21, OPC_ALOAD, true, GROUP_GENERAL, 4, vec![256]),
            (25, OPC_BIPUSH, false, GROUP_GENERAL, 1, vec![-1]),
            (26, OPC_SIPUSH, false, GROUP_GENERAL, 3, vec![-200]),
            (29, OPC_BIPUSH, false, GROUP_GENERAL, 2, vec![-128]),
            (31, OPC_GOTO, false, GROUP_JUMP, 3, vec![-5]),
            (34, OPC_RETURN, false, GROUP_RETURN, 1, vec![]),
        ]
    );
    let seq = mt.get_instruction_sequence().unwrap();
    let dest = |i: i32| seq.get_instr(i).as_jump().destination();
    assert_eq!((dest(0), dest(1), dest(9)), (3, 6, 7));
    assert!(!seq.get_instr(0).can_fall_through());
    assert!(!seq.get_instr(4).can_fall_through());
    assert!(seq.get_instr(2).can_fall_through());
    assert_eq!(seq.get_instr(2).to_string(), "@wide @iinc 12c -2");
    assert_eq!(seq.get_instr(6).to_string(), "@bipush -1");
    assert_eq!(seq.get_instr(7).to_string(), "@sipush -c8");
    assert_eq!(seq.to_string_indent(0).lines().next(), Some("0: @jsr 10"));
}

fn switch_code(pad: usize) -> (Vec<u8>, i32) {
    let mut code = vec![0x00; pad];
    let start = code.len() as i32;
    code.push(0xAA);
    while !code.len().is_multiple_of(4) {
        code.push(0);
    }
    let table_len = (code.len() as i32 - start) + 12 + 3 * 4;
    let lookup_start = start + table_len;
    let mut lookup = vec![0xABu8];
    while !(lookup_start as usize + lookup.len()).is_multiple_of(4) {
        lookup.push(0);
    }
    let lookup_len = lookup.len() as i32 + 8 + 2 * 8;
    let ret = lookup_start + lookup_len;
    for v in [
        ret - start,
        -1,
        1,
        ret - start,
        lookup_start - start,
        ret - start,
    ] {
        code.extend_from_slice(&v.to_be_bytes());
    }
    code.extend_from_slice(&lookup);
    for v in [
        ret - lookup_start,
        2,
        -5,
        start - lookup_start,
        1000,
        ret - lookup_start,
    ] {
        code.extend_from_slice(&v.to_be_bytes());
    }
    code.push(0xB1);
    (code, ret)
}

#[test]
fn switch_padding_at_every_alignment() {
    for pad in 0..4 {
        let (code, ret) = switch_code(pad);
        let (_, mt) = single_method(52, 0, &code, &[]);
        let seq = mt.get_instruction_sequence().unwrap();
        let table = seq.get_instr(pad as i32).as_switch();
        let lookup = seq.get_instr(pad as i32 + 1).as_switch();
        let last = seq.get_instr(pad as i32 + 2);
        assert_eq!(last.start_offset(), ret, "pad {}", pad);
        assert_eq!(last.opcode(), OPC_RETURN);
        assert_eq!(table.group(), GROUP_SWITCH);
        assert_eq!(table.get_values(), vec![-1, 0, 1]);
        let (t, l, r) = (pad as i32, pad as i32 + 1, pad as i32 + 2);
        assert_eq!(table.get_destinations(), vec![r, l, r]);
        assert_eq!(table.get_default_destination(), r);
        assert_eq!(lookup.get_values(), vec![-5, 1000]);
        assert_eq!(lookup.get_destinations(), vec![t, r]);
        assert_eq!(lookup.operands().unwrap()[1], 2);
        let copy = table.java_clone();
        assert_ne!(copy.0, table.0);
        assert_eq!(copy.get_destinations(), table.get_destinations());
        assert!(Instruction::equals(Some(copy.0), Some(table.0)));
        assert!(!Instruction::equals(Some(copy.0), Some(lookup.0)));
        assert!(!Instruction::equals(None, Some(lookup.0)));
    }
}

#[test]
fn tableswitch_range_wraps_like_java() {
    let mut code = vec![0xAA, 0, 0, 0];
    for v in [16, i32::MIN, i32::MAX] {
        code.extend_from_slice(&v.to_be_bytes());
    }
    code.push(0xB1);
    let (_, mt) = single_method(52, 0, &code, &[]);
    let seq = mt.get_instruction_sequence().unwrap();
    assert_eq!(seq.length(), 2);
    let sw = seq.get_instr(0).as_switch();
    assert_eq!(sw.operands().unwrap(), vec![16, i32::MIN, i32::MAX]);
    assert!(sw.get_values().is_empty());
    assert_eq!(sw.get_default_destination(), 1);
}

#[test]
fn java_1_0_code_attribute_layout() {
    let (_, mt) = single_method(45, 2, &[0x04, 0x3C, 0xB1], &[]);
    assert_eq!(mt.get_local_variables(), 3);
    assert_eq!(
        shape(mt),
        vec![
            (0, OPC_BIPUSH, false, GROUP_GENERAL, 1, vec![1]),
            (1, OPC_ISTORE, false, GROUP_GENERAL, 1, vec![1]),
            (2, OPC_RETURN, false, GROUP_RETURN, 1, vec![]),
        ]
    );
    let (_, mt) = single_method(45, 3, &[0x04, 0x3C, 0xB1], &[]);
    assert_eq!(shape(mt).len(), 3);
}

#[test]
fn exception_table_ranges_and_catch_all() {
    let mut pool = Pool::default();
    let ioe = pool.class("java/io/IOException");
    let code = code_attribute(
        &mut pool,
        Code {
            max_locals: 1,
            code: &[0x00, 0x00, 0xB1, 0x4B, 0xB1],
            handlers: &[[0, 5, 3, ioe], [1, 3, 3, 0]],
            attributes: Vec::new(),
            predates_java: false,
        },
    );
    let m = member(&mut pool, 0x8, "m", "()V", &[code]);
    let cl = create(class_file(52, 0, pool, &[], &[m], &[]), true);
    let mt = cl.get_method("m", "()V").unwrap();
    mt.expand_data(cl);
    let handlers = mt
        .get_instruction_sequence()
        .unwrap()
        .exception_table()
        .get_handlers();
    assert_eq!(handlers.len(), 2);
    assert_eq!(
        (handlers[0].from(), handlers[0].to(), handlers[0].handler()),
        (0, 5, 3)
    );
    assert_eq!(
        handlers[0].exception_class().as_deref(),
        Some("java/io/IOException")
    );
    assert_eq!(
        (
            handlers[1].from,
            handlers[1].to,
            handlers[1].exception_class.clone()
        ),
        (1, 3, None)
    );
    assert_ne!(handlers[0].java_hash(), handlers[1].java_hash());

    let (cl, mt) = single_method(52, 0, &[0x00, 0xB1], &[]);
    let _ = cl;
    assert_eq!(mt.get_instruction_sequence().unwrap().length(), 2);
    let caught =
        try_catch(|| single_method(52, 0, &[0x10, 0x05, 0xB1], &[[1, 3, 2, 0]])).unwrap_err();
    assert!(
        caught.message.contains("handler start"),
        "{}",
        caught.message
    );
}

fn lvt(pool: &mut Pool, name: &str, rows: &[(u16, u16, &str, &str, u16)]) -> Vec<u8> {
    let mut body = u2(rows.len() as u16).to_vec();
    for (start, len, n, d, slot) in rows {
        let ni = pool.utf8(n);
        let di = pool.utf8(d);
        for v in [*start, *len, ni, di, *slot] {
            body.extend_from_slice(&u2(v));
        }
    }
    attribute(pool, name, &body)
}

fn method_with_code_attributes(
    build: impl FnOnce(&mut Pool) -> Vec<Vec<u8>>,
) -> (StructClass, StructMethod) {
    let mut pool = Pool::default();
    let attributes = build(&mut pool);
    let code = code_attribute(
        &mut pool,
        Code {
            max_locals: 2,
            code: &[0x00, 0x00, 0x00, 0x00, 0xB1],
            handlers: &[],
            attributes,
            predates_java: false,
        },
    );
    let m = member(&mut pool, 0x8, "m", "(II)V", &[code]);
    let cl = create(class_file(52, 0, pool, &[], &[m], &[]), true);
    let mt = cl.get_method("m", "(II)V").unwrap();
    (cl, mt)
}

#[test]
fn local_variable_tables_merge_like_vineflower() {
    let (_, mt) = method_with_code_attributes(|pool| {
        vec![
            lvt(
                pool,
                "LocalVariableTable",
                &[(0, 5, "b", "I", 1), (2, 3, "z", "Ljava/util/List;", 0)],
            ),
            lvt(
                pool,
                "LocalVariableTable",
                &[(0, 5, "a", "Ljava/util/List;", 0)],
            ),
            lvt(
                pool,
                "LocalVariableTypeTable",
                &[(0, 5, "a", "Ljava/util/List<TT;>;", 0)],
            ),
        ]
    });
    let table = mt.get_local_variable_attr().unwrap();
    let names: Vec<String> = table.get_variables().iter().map(|v| v.get_name()).collect();
    assert_eq!(names, vec!["z", "b", "a"]);
    let a = &table.get_variables()[2];
    assert_eq!(a.get_signature().as_deref(), Some("Ljava/util/List<TT;>;"));
    assert!(table.get_variables()[0].get_signature().is_none());
    assert!(table
        .get_variables()
        .iter()
        .all(|v| v.get_version().version == 1));
    assert_eq!(
        a.to_string(),
        "'(0,0-5)Ljava/util/List;<Ljava/util/List<TT;>;> a'"
    );
    assert_eq!(table.get_name(1, 3).as_deref(), Some("b"));
    assert_eq!(table.get_name(1, 5), None);
    assert_eq!(
        table.get_descriptor(0, 1).as_deref(),
        Some("Ljava/util/List;")
    );
    assert_eq!(table.matching_vars_index(0).len(), 2);
    assert_eq!(table.get_range(0, 5).len(), 3);
    assert!(table.contains_name("z") && !table.contains_name("q"));
    let names = table.get_map_names();
    assert_eq!(names.len(), 2);
    assert_eq!(
        names
            .get_cloned(&kuna_jvm::var_version_pair::VarVersionPair::new(0, 1))
            .as_deref(),
        Some("a")
    );
    let lvtt = mt
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE)
        .unwrap();
    assert_eq!(
        lvtt.get_signature(0, 0).as_deref(),
        Some("Ljava/util/List<TT;>;")
    );
    assert_eq!(a.rename("q".to_string()).get_signature(), a.get_signature());

    let caught = try_catch(|| {
        method_with_code_attributes(|pool| {
            vec![
                lvt(pool, "LocalVariableTable", &[]),
                lvt(pool, "LocalVariableTable", &[(0, 5, "a", "I", 0)]),
            ]
        })
    })
    .unwrap_err();
    assert!(
        caught.is("UnsupportedOperationException"),
        "{}",
        caught.message
    );
    let (_, mt) = method_with_code_attributes(|pool| {
        vec![
            lvt(pool, "LocalVariableTable", &[(0, 5, "a", "I", 0)]),
            lvt(pool, "LocalVariableTable", &[]),
        ]
    });
    assert_eq!(
        mt.get_local_variable_attr().unwrap().get_variables().len(),
        1
    );
}

#[test]
fn unknown_attributes_are_skipped_and_bad_lengths_fail() {
    let mut pool = Pool::default();
    let foo = attribute(&mut pool, "Foo", &[1, 2, 3]);
    let syn = attribute(&mut pool, "Synthetic", &[]);
    let f = member(&mut pool, 0x2, "f", "I", &[foo.clone(), syn]);
    let src = pool.utf8("T.java");
    let source = attribute(&mut pool, "SourceFile", &u2(src));
    let cl = create(class_file(52, 0, pool, &[f], &[], &[foo, source]), true);
    let fd = cl.get_field("f", "I").unwrap();
    assert!(fd.is_synthetic());
    assert!(!fd.has_modifier(ACC_SYNTHETIC));
    assert_eq!(
        fd.get_attributes().borrow().key_vec(),
        vec!["Synthetic".to_string()]
    );
    let sf = cl
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_SOURCE_FILE)
        .unwrap();
    assert_eq!(sf.get_source_file(cl.get_pool()), "T.java");
    assert!(!cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_SIGNATURE));
    assert!(cl
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_RECORD)
        .is_none());
    assert!(cl.get_record_components().is_none());

    let mut pool = Pool::default();
    let bad = attribute_raw_length(&mut pool, "Foo", 0xFFFF_FFFF, &[]);
    let f = member(&mut pool, 0x2, "f", "I", &[bad]);
    let bytes = class_file(52, 0, pool, &[f], &[], &[]);
    let caught = try_catch(|| create(bytes, true)).unwrap_err();
    assert!(caught.is("IOException"), "{}", caught.message);
}

fn corpus_class(config: &str, name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/classfile/classes")
        .join(config)
        .join(format!("{}.class", name));
    std::fs::read(path).unwrap()
}

#[test]
fn every_truncation_fails_with_io_exception() {
    let bytes = corpus_class("r21g", "Iface");
    for cut in (0..bytes.len()).step_by(3) {
        let prefix = bytes[..cut].to_vec();
        let caught = try_catch(|| create(prefix, true)).unwrap_err();
        assert!(caught.is("IOException"), "cut {}: {}", cut, caught.message);
    }
    assert!(try_catch(|| create(bytes, true)).is_ok());
}

#[test]
fn foreign_classes_drop_method_bodies() {
    let cl = create(corpus_class("r17p", "Basics"), false);
    assert!(!cl.is_own());
    for mt in cl.get_methods().iter() {
        assert!(!mt.contains_code());
        assert_eq!(mt.get_local_variables(), -1);
        assert!(!mt.has_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE));
        mt.expand_data(cl);
        assert!(mt.get_instruction_sequence().is_none());
    }
    let mt = cl
        .get_method("tryCatch", "(Ljava/io/InputStream;)I")
        .unwrap();
    assert!(mt.has_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS));
}

#[test]
fn expand_data_rebuilds_a_stale_sequence() {
    let cl = create(corpus_class("r21g", "Basics"), true);
    let mt = cl.get_method("table", "(I)I").unwrap();
    mt.expand_data(cl);
    let first = mt.get_instruction_sequence().unwrap();
    mt.expand_data(cl);
    assert_eq!(mt.get_instruction_sequence(), Some(first));
    let length = first.length();
    end_class_scope();
    assert!(!first.is_live());
    mt.expand_data(cl);
    let second = mt.get_instruction_sequence().unwrap();
    assert!(second.is_live());
    assert_eq!(second.length(), length);
    mt.release_resources();
    assert!(mt.get_instruction_sequence().is_none());
    mt.expand_data(cl);
    assert!(mt.get_instruction_sequence().unwrap().is_live());
}

#[test]
fn members_share_the_base_api() {
    let cl = create(corpus_class("r21g", "Records"), true);
    let components = cl.get_record_components().unwrap();
    assert_eq!(
        components.iter().map(|c| c.get_name()).collect::<Vec<_>>(),
        vec!["name", "age", "value", "tags"]
    );
    let rc = components[0];
    assert!(rc.has_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS));
    assert_eq!(rc.get_access_flags(), 0);
    let field = cl.get_field("name", "Ljava/lang/String;").unwrap();
    let ctor = cl
        .get_methods()
        .iter()
        .copied()
        .find(|m| m.get_name() == "<init>")
        .unwrap();
    let members: Vec<StructMember> = vec![cl.0, field.0, ctor.0, rc.into()];
    assert_eq!(
        members.iter().map(|m| m.to_string()).collect::<Vec<_>>(),
        vec![
            "Records".to_string(),
            "name".to_string(),
            "<init> (Ljava/lang/String;ILjava/lang/Comparable;[Ljava/util/List;)V".to_string(),
            "name".to_string()
        ]
    );
    assert!(members[3].is_record_component() && members[3].is_field());
    assert!(!members[1].is_record_component());
    assert_eq!(members[3].as_record_component(), rc);
    assert!(try_catch(|| members[1].as_record_component())
        .unwrap_err()
        .is("ClassCastException"));
    assert!(members.iter().all(|m| m.get_version().major == 65));
    assert!(ctor.get_signature().is_some());
    assert_eq!(ctor.get_class_qualified_name(), "Records");
    assert!(cl.has_field("age", "I"));
    assert_eq!(
        cl.get_method_key("compareTo (LRecords;)I"),
        cl.get_method("compareTo", "(LRecords;)I")
    );
    let lambda = cl
        .get_methods()
        .iter()
        .copied()
        .find(|m| m.get_name() == "describe")
        .unwrap();
    assert!(!lambda.is_synthetic());
    let synthetic = create(corpus_class("r21g", "Lambdas"), true)
        .get_methods()
        .iter()
        .filter(|m| m.is_synthetic())
        .count();
    assert!(synthetic >= 5);
    assert_eq!(
        cl.get_method_recursive("compareTo", "(LRecords;)I"),
        cl.get_method("compareTo", "(LRecords;)I")
    );
}

#[test]
fn annotations_become_exprents_per_class_scope() {
    let cl = create(corpus_class("r21g", "Annos"), true);
    let attr = cl
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS)
        .unwrap();
    let first = attr.get_annotations();
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].get_class_name(), "Everything");
    assert_eq!(first[0].get_par_names(), vec!["i", "str", "ints", "en"]);
    assert_eq!(attr.get_annotations(), first);
    end_class_scope();
    assert!(!first[0].is_live());
    let second = attr.get_annotations();
    assert!(second[0].is_live());
    assert_eq!(second[0].get_class_name(), "Everything");

    let method = cl
        .get_methods()
        .iter()
        .copied()
        .find(|m| m.get_name() == "method")
        .unwrap();
    let params = method
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS)
        .unwrap()
        .get_param_annotations();
    assert_eq!(
        params.iter().map(Vec::len).collect::<Vec<_>>(),
        vec![1, 0, 1]
    );
    let types = method
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_TYPE_ANNOTATIONS)
        .unwrap()
        .get_annotations();
    assert!(types
        .iter()
        .all(|t| t.get_annotation().get_class_name() == "TU"));
    assert!(types.iter().any(|t| t.get_target_type() == 0x40));

    let everything = create(corpus_class("r21g", "Everything"), true);
    let ints = everything.get_method("ints", "()[I").unwrap();
    let default = ints
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_ANNOTATION_DEFAULT)
        .unwrap()
        .get_default_value()
        .unwrap();
    let array = default.as_new();
    assert_eq!(array.get_lst_array_elements().len(), 3);
    assert_eq!(array.get_new_type().array_dim, 1);
    assert_eq!(array.get_new_type().type_, CodeType::Int);
}

#[test]
fn instruction_impact_tracks_stack_types() {
    let mut pool = Pool::default();
    let field = pool.member_ref(9, "T", "f", "J");
    let call = pool.member_ref(10, "T", "g", "(JI)Ljava/lang/String;");
    let arr = pool.class("[[I");
    let string = pool.class("java/lang/String");
    let wide = pool.long(1 << 40);
    let mut code = vec![
        0x0A, // lconst_1                 J G
        0x05, // iconst_2                 J G I
        0x5B, // dup_x2                   I J G I
        0x57, // pop                      I J G
        0x5C, // dup2                     I J G J G
        0x58, // pop2                     I J G
        0x88, // l2i                      I I
        0x5F, // swap                     I I
        0x60, // iadd                     I
        0x87, // i2d                      D G
        0x01, // aconst_null              D G null
        0xC0, // checkcast String         D G String
    ];
    code.extend_from_slice(&u2(string));
    code.push(0xB2); // getstatic f:J      D G String J G
    code.extend_from_slice(&u2(field));
    code.push(0x04); // iconst_1           ... J G I
    code.push(0xB8); // invokestatic g     D G String String
    code.extend_from_slice(&u2(call));
    code.extend_from_slice(&[0x10, 0x03, 0x10, 0x04, 0xC5]); // bipush 3, bipush 4, multianewarray [[I 2
    code.extend_from_slice(&u2(arr));
    code.push(2);
    code.extend_from_slice(&[0x10, 0x05, 0xBC, 10]); // bipush 5, newarray int
    code.push(0x14); // ldc2_w long
    code.extend_from_slice(&u2(wide));
    code.push(0xB1);
    let code = code_attribute(
        &mut pool,
        Code {
            max_locals: 0,
            code: &code,
            handlers: &[],
            attributes: Vec::new(),
            predates_java: false,
        },
    );
    let m = member(&mut pool, 0x8, "m", "()V", &[code]);
    let cl = create(class_file(52, 0, pool, &[], &[m], &[]), true);
    let mt = cl.get_method("m", "()V").unwrap();
    mt.expand_data(cl);
    let mut data = DataPoint::new();
    for instr in mt.get_instruction_sequence().unwrap().iter() {
        InstructionImpact::step_types(&mut data, instr, cl.get_pool());
    }
    let stack = data.get_stack();
    let mut types = Vec::new();
    while stack.len() > 0 {
        let t = stack.pop();
        types.push((t.type_, t.array_dim, t.value.as_deref().map(str::to_string)));
    }
    types.reverse();
    let s = |v: &str| Some(v.to_string());
    assert_eq!(
        types,
        vec![
            (CodeType::Double, 0, s("D")),
            (CodeType::Group2empty, 0, s("G")),
            (CodeType::Object, 0, s("java/lang/String")),
            (CodeType::Object, 0, s("java/lang/String")),
            (CodeType::Object, 2, s("[[I")),
            (CodeType::Int, 1, s("I")),
            (CodeType::Long, 0, s("J")),
            (CodeType::Group2empty, 0, s("G")),
        ]
    );
}
