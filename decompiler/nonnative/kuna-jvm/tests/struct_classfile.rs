//! The class-file model against `javap`.
//!
//! `tests/data/classfile/gen_golden.py` compiles the corpus under several
//! `javac` configurations and turns `javap -v -p -c` into `.golden` text;
//! [`render_class`] produces the same text from a parsed `StructClass`.
//! `KUNA_JVM_JAVAP_SWEEP=<dir>` compares every `.class`/`.golden` pair under
//! a directory (e.g. an extracted `java.base` run through the same script);
//! `KUNA_JVM_PARSE_SWEEP=<dir>` parses and expands every class under a
//! directory and fails on any panic.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::code_constants::*;
use kuna_jvm::constant_pool::ConstantPool;
use kuna_jvm::full_instruction_sequence::FullInstructionSequence;
use kuna_jvm::rt::{reset_class_scope, reset_session_scope, try_catch};
use kuna_jvm::struct_annotation_attribute::{AnnotationElement, AnnotationRecord};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_general_attribute::StructGeneralAttribute;
use kuna_jvm::struct_member::StructMember;
use kuna_jvm::struct_method::StructMethod;

const MNEMONICS: [&str; 202] = [
    "nop",
    "aconst_null",
    "iconst_m1",
    "iconst_0",
    "iconst_1",
    "iconst_2",
    "iconst_3",
    "iconst_4",
    "iconst_5",
    "lconst_0",
    "lconst_1",
    "fconst_0",
    "fconst_1",
    "fconst_2",
    "dconst_0",
    "dconst_1",
    "bipush",
    "sipush",
    "ldc",
    "ldc_w",
    "ldc2_w",
    "iload",
    "lload",
    "fload",
    "dload",
    "aload",
    "iload_0",
    "iload_1",
    "iload_2",
    "iload_3",
    "lload_0",
    "lload_1",
    "lload_2",
    "lload_3",
    "fload_0",
    "fload_1",
    "fload_2",
    "fload_3",
    "dload_0",
    "dload_1",
    "dload_2",
    "dload_3",
    "aload_0",
    "aload_1",
    "aload_2",
    "aload_3",
    "iaload",
    "laload",
    "faload",
    "daload",
    "aaload",
    "baload",
    "caload",
    "saload",
    "istore",
    "lstore",
    "fstore",
    "dstore",
    "astore",
    "istore_0",
    "istore_1",
    "istore_2",
    "istore_3",
    "lstore_0",
    "lstore_1",
    "lstore_2",
    "lstore_3",
    "fstore_0",
    "fstore_1",
    "fstore_2",
    "fstore_3",
    "dstore_0",
    "dstore_1",
    "dstore_2",
    "dstore_3",
    "astore_0",
    "astore_1",
    "astore_2",
    "astore_3",
    "iastore",
    "lastore",
    "fastore",
    "dastore",
    "aastore",
    "bastore",
    "castore",
    "sastore",
    "pop",
    "pop2",
    "dup",
    "dup_x1",
    "dup_x2",
    "dup2",
    "dup2_x1",
    "dup2_x2",
    "swap",
    "iadd",
    "ladd",
    "fadd",
    "dadd",
    "isub",
    "lsub",
    "fsub",
    "dsub",
    "imul",
    "lmul",
    "fmul",
    "dmul",
    "idiv",
    "ldiv",
    "fdiv",
    "ddiv",
    "irem",
    "lrem",
    "frem",
    "drem",
    "ineg",
    "lneg",
    "fneg",
    "dneg",
    "ishl",
    "lshl",
    "ishr",
    "lshr",
    "iushr",
    "lushr",
    "iand",
    "land",
    "ior",
    "lor",
    "ixor",
    "lxor",
    "iinc",
    "i2l",
    "i2f",
    "i2d",
    "l2i",
    "l2f",
    "l2d",
    "f2i",
    "f2l",
    "f2d",
    "d2i",
    "d2l",
    "d2f",
    "i2b",
    "i2c",
    "i2s",
    "lcmp",
    "fcmpl",
    "fcmpg",
    "dcmpl",
    "dcmpg",
    "ifeq",
    "ifne",
    "iflt",
    "ifge",
    "ifgt",
    "ifle",
    "if_icmpeq",
    "if_icmpne",
    "if_icmplt",
    "if_icmpge",
    "if_icmpgt",
    "if_icmple",
    "if_acmpeq",
    "if_acmpne",
    "goto",
    "jsr",
    "ret",
    "tableswitch",
    "lookupswitch",
    "ireturn",
    "lreturn",
    "freturn",
    "dreturn",
    "areturn",
    "return",
    "getstatic",
    "putstatic",
    "getfield",
    "putfield",
    "invokevirtual",
    "invokespecial",
    "invokestatic",
    "invokeinterface",
    "invokedynamic",
    "new",
    "newarray",
    "anewarray",
    "arraylength",
    "athrow",
    "checkcast",
    "instanceof",
    "monitorenter",
    "monitorexit",
    "wide",
    "multianewarray",
    "ifnull",
    "ifnonnull",
    "goto_w",
    "jsr_w",
];

fn element(e: &AnnotationElement) -> String {
    match e {
        AnnotationElement::Enum {
            type_index,
            const_index,
            ..
        } => format!("e#{}.#{}", type_index, const_index),
        AnnotationElement::Class { index, .. } => format!("c#{}", index),
        AnnotationElement::Array { elements } => {
            let parts: Vec<String> = elements.iter().map(element).collect();
            format!("[{}]", parts.join(","))
        }
        AnnotationElement::Annotation(record) => format!("@{}", annotation(record)),
        AnnotationElement::Const { tag, index, .. } => format!("{}#{}", *tag as char, index),
    }
}

fn annotation(r: &AnnotationRecord) -> String {
    let parts: Vec<String> = r
        .name_indices
        .iter()
        .zip(&r.values)
        .map(|(n, v)| format!("#{}={}", n, element(v)))
        .collect();
    format!("#{}({})", r.type_index, parts.join(","))
}

/// The modifier bits `javap` prints for an `InnerClasses` entry.
fn inner_class_modifiers(flags: i32) -> i32 {
    let f = if flags & ACC_INTERFACE != 0 {
        flags & !ACC_ABSTRACT
    } else {
        flags
    };
    f & (ACC_PUBLIC | ACC_PRIVATE | ACC_PROTECTED | ACC_STATIC | ACC_ABSTRACT | ACC_FINAL)
}

fn attribute_lines(out: &mut String, member: StructMember, pool: ConstantPool, prefix: &str) {
    let map = member.get_attributes();
    let mut names = map.borrow().key_vec();
    names.sort();
    writeln!(out, "{}attrs {}", prefix, names.join(",")).unwrap();
    for name in &names {
        let attr = map.borrow().get_cloned(name).unwrap();
        match name.as_str() {
            "Signature" => writeln!(
                out,
                "{}signature {}",
                prefix,
                attr.as_generic_signature().get_signature()
            )
            .unwrap(),
            "SourceFile" => writeln!(
                out,
                "{}sourcefile {}",
                prefix,
                attr.as_source_file().get_source_file(pool)
            )
            .unwrap(),
            "InnerClasses" => {
                for e in attr.as_inner_classes().get_entries() {
                    writeln!(
                        out,
                        "{}innerclass {} {} {} 0x{:04x}",
                        prefix,
                        e.inner_name,
                        e.enclosing_name.as_deref().unwrap_or("-"),
                        e.simple_name.as_deref().unwrap_or("-"),
                        inner_class_modifiers(e.access_flags)
                    )
                    .unwrap();
                }
            }
            "EnclosingMethod" => {
                let a = attr.as_enclosing_method();
                writeln!(
                    out,
                    "{}enclosing {} {}",
                    prefix,
                    a.get_class_name().unwrap(),
                    a.get_method_name().as_deref().unwrap_or("-")
                )
                .unwrap();
            }
            "NestHost" => writeln!(
                out,
                "{}nesthost {}",
                prefix,
                attr.as_nest_host().get_host_class(pool)
            )
            .unwrap(),
            "PermittedSubclasses" => writeln!(
                out,
                "{}permitted {}",
                prefix,
                attr.as_permitted_subclasses().get_classes().join(" ")
            )
            .unwrap(),
            "Exceptions" => {
                let a = attr.as_exceptions();
                let names: Vec<String> = (0..a.get_throws_exceptions().len() as i32)
                    .map(|i| a.get_exc_classname(i, pool))
                    .collect();
                writeln!(out, "{}exceptions {}", prefix, names.join(" ")).unwrap();
            }
            "MethodParameters" => {
                let items: Vec<String> = attr
                    .as_method_parameters()
                    .get_entries()
                    .iter()
                    .map(|e| {
                        format!(
                            "{}:0x{:04x}",
                            e.my_name.as_deref().unwrap_or("-"),
                            e.my_access_flags
                        )
                    })
                    .collect();
                writeln!(out, "{}params {}", prefix, items.join(" ")).unwrap();
            }
            "AnnotationDefault" => {
                let record = attr.as_ann_default().get_record().unwrap();
                writeln!(out, "{}default {}", prefix, element(&record)).unwrap();
            }
            "RuntimeVisibleAnnotations" | "RuntimeInvisibleAnnotations" => {
                for r in attr.as_annotation().get_records() {
                    writeln!(out, "{}anno {} {}", prefix, name, annotation(&r)).unwrap();
                }
            }
            "RuntimeVisibleParameterAnnotations" | "RuntimeInvisibleParameterAnnotations" => {
                for (p, records) in attr
                    .as_annotation_parameter()
                    .get_records()
                    .iter()
                    .enumerate()
                {
                    for r in records {
                        writeln!(out, "{}panno {} {} {}", prefix, name, p, annotation(r)).unwrap();
                    }
                }
            }
            "RuntimeVisibleTypeAnnotations" | "RuntimeInvisibleTypeAnnotations" => {
                for t in attr.as_type_annotation().get_records() {
                    let path: Vec<String> = t
                        .path
                        .iter()
                        .map(|p| format!("{}:{}", p.kind as i32, p.type_arg))
                        .collect();
                    writeln!(
                        out,
                        "{}tanno {} {:08x} [{}] {}",
                        prefix,
                        name,
                        t.target as u32,
                        path.join(" "),
                        annotation(&t.annotation)
                    )
                    .unwrap();
                }
            }
            "BootstrapMethods" => {
                let a = attr.as_bootstrap_methods();
                for i in 0..a.get_methods_number() {
                    let r = a.get_method_reference(i).unwrap();
                    writeln!(
                        out,
                        "{}bootstrap {} {}:{}.{}:{} args {}",
                        prefix,
                        i,
                        r.index1,
                        r.classname.as_deref().unwrap_or("null"),
                        r.elementname,
                        r.descriptor,
                        a.get_method_arguments(i).len()
                    )
                    .unwrap();
                }
            }
            "Record" => {
                for c in attr.as_record().get_components() {
                    let mut cn = c.get_attributes().borrow().key_vec();
                    cn.sort();
                    writeln!(
                        out,
                        "{}record {} {} attrs {}",
                        prefix,
                        c.get_name(),
                        c.get_descriptor(),
                        cn.join(",")
                    )
                    .unwrap();
                }
            }
            "Module" => {
                let m = attr.as_module();
                writeln!(
                    out,
                    "{}module {} 0x{:04x} {}",
                    prefix,
                    m.module_name(),
                    m.module_flags(),
                    m.module_version().as_deref().unwrap_or("-")
                )
                .unwrap();
                for r in m.requires() {
                    writeln!(
                        out,
                        "{} requires {} 0x{:04x} {}",
                        prefix,
                        r.module_name,
                        r.flags,
                        r.module_version.as_deref().unwrap_or("-")
                    )
                    .unwrap();
                }
                for e in m.exports() {
                    let to: String = e
                        .export_to_modules
                        .iter()
                        .map(|t| format!(" {}", t))
                        .collect();
                    writeln!(
                        out,
                        "{} exports {} 0x{:04x}{}",
                        prefix, e.package_name, e.flags, to
                    )
                    .unwrap();
                }
                for e in m.opens() {
                    let to: String = e
                        .opens_to_modules
                        .iter()
                        .map(|t| format!(" {}", t))
                        .collect();
                    writeln!(
                        out,
                        "{} opens {} 0x{:04x}{}",
                        prefix, e.package_name, e.flags, to
                    )
                    .unwrap();
                }
                for u in m.uses() {
                    writeln!(out, "{} uses {}", prefix, u).unwrap();
                }
                for p in m.provides() {
                    let with: String = p
                        .implementation_names
                        .iter()
                        .map(|t| format!(" {}", t))
                        .collect();
                    writeln!(out, "{} provides {} with{}", prefix, p.interface_name, with).unwrap();
                }
            }
            _ => {}
        }
    }
}

fn link(pool: ConstantPool, index: i32) -> String {
    let l = pool.get_link_constant(index).unwrap();
    format!(
        "{}.{}:{}",
        l.classname.as_deref().unwrap_or("null"),
        l.elementname,
        l.descriptor
    )
}

fn offset_of(seq: FullInstructionSequence, index: i32) -> String {
    if index < 0 || index >= seq.length() {
        return "?".to_string();
    }
    seq.get_instr(index).start_offset().to_string()
}

fn code_lines(out: &mut String, mt: StructMethod, cl: StructClass, prefix: &str) {
    let pool = cl.get_pool();
    writeln!(out, "{}code locals={}", prefix, mt.get_local_variables()).unwrap();
    mt.expand_data(cl);
    let seq = mt.get_instruction_sequence().unwrap();
    let p2 = format!("{} ", prefix);
    for instr in seq.iter() {
        let opcode = instr.opcode();
        let mut line = format!(
            "{}{}: {}",
            p2,
            instr.start_offset(),
            MNEMONICS[opcode as usize]
        );
        if instr.wide() {
            line.push_str(" wide");
        }
        for op in instr.operands().unwrap_or_default() {
            write!(line, " {}", op).unwrap();
        }
        let mut extra: Vec<String> = Vec::new();
        if let Some(j) = instr.try_as_jump() {
            extra.push(format!("-> {}", offset_of(seq, j.destination())));
        } else if let Some(s) = instr.try_as_switch() {
            let cases: Vec<String> = s
                .get_values()
                .iter()
                .zip(s.get_destinations())
                .map(|(v, d)| format!("{}:{}", v, offset_of(seq, d)))
                .collect();
            extra.push(format!(
                "-> {} [{}]",
                offset_of(seq, s.get_default_destination()),
                cases.join(" ")
            ));
        } else {
            match opcode {
                OPC_GETSTATIC | OPC_PUTSTATIC | OPC_GETFIELD | OPC_PUTFIELD | OPC_INVOKEVIRTUAL
                | OPC_INVOKESPECIAL | OPC_INVOKESTATIC | OPC_INVOKEINTERFACE => {
                    extra.push(link(pool, instr.operand(0)))
                }
                OPC_INVOKEDYNAMIC => {
                    let l = pool.get_link_constant(instr.operand(0)).unwrap();
                    extra.push(format!("#{}:{}:{}", l.index1, l.elementname, l.descriptor));
                }
                OPC_NEW | OPC_ANEWARRAY | OPC_CHECKCAST | OPC_INSTANCEOF | OPC_MULTIANEWARRAY => {
                    extra.push(
                        pool.get_primitive_constant(instr.operand(0))
                            .unwrap()
                            .get_string(),
                    )
                }
                _ => {}
            }
        }
        if !extra.is_empty() {
            write!(line, " | {}", extra.join(" | ")).unwrap();
        }
        writeln!(out, "{}", line).unwrap();
    }
    let last = seq.get_last();
    let code_length = last.start_offset() + last.length();
    for h in seq.exception_table().get_handlers() {
        let to = if h.to < seq.length() {
            seq.get_instr(h.to).start_offset()
        } else {
            code_length
        };
        writeln!(
            out,
            "{}handler {} {} {} {}",
            p2,
            seq.get_instr(h.from).start_offset(),
            to,
            seq.get_instr(h.handler).start_offset(),
            h.exception_class.as_deref().unwrap_or("any")
        )
        .unwrap();
    }
    if let Some(lnt) = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE) {
        for pair in lnt.get_raw_data().chunks(2) {
            writeln!(out, "{}line {} {}", p2, pair[0], pair[1]).unwrap();
        }
    }
    if let Some(lvt) = mt.get_local_variable_attr() {
        for v in lvt.get_variables() {
            let sig = v
                .get_signature()
                .map(|s| format!(" {}", s))
                .unwrap_or_default();
            writeln!(
                out,
                "{}lvt {} {} {} {} {}{}",
                p2,
                v.index,
                v.get_start(),
                v.length,
                v.get_name(),
                v.get_descriptor(),
                sig
            )
            .unwrap();
        }
    }
}

/// The golden text of a parsed class, method bodies expanded.
pub fn render_class(cl: StructClass) -> String {
    let pool = cl.get_pool();
    let mut out = String::new();
    let version = cl.get_version();
    writeln!(out, "class {}", cl.qualified_name()).unwrap();
    writeln!(out, "version {}.{}", version.major, version.minor).unwrap();
    writeln!(out, "flags 0x{:04x}", cl.get_access_flags()).unwrap();
    let sup = cl
        .super_class()
        .map(|s| s.get_string())
        .unwrap_or_else(|| "-".to_string());
    writeln!(out, "super {}", sup).unwrap();
    let ifaces = cl.get_interface_names();
    let names: String = ifaces.iter().map(|i| format!(" {}", i)).collect();
    writeln!(out, "interfaces {}{}", ifaces.len(), names).unwrap();
    attribute_lines(&mut out, cl.0, pool, "");
    for fd in cl.get_fields().iter() {
        writeln!(out, "field {} {}", fd.get_name(), fd.get_descriptor()).unwrap();
        writeln!(out, " flags 0x{:04x}", fd.get_access_flags()).unwrap();
        attribute_lines(&mut out, fd.0, pool, " ");
    }
    for mt in cl.get_methods().iter() {
        writeln!(out, "method {} {}", mt.get_name(), mt.get_descriptor()).unwrap();
        writeln!(out, " flags 0x{:04x}", mt.get_access_flags()).unwrap();
        attribute_lines(&mut out, mt.0, pool, " ");
        if mt.contains_code() {
            code_lines(&mut out, mt, cl, " ");
        }
    }
    out
}

/// `StructClass.create` as the tests call it; the one place a decompilation
/// context would be installed for class reading.
pub fn create(bytes: Vec<u8>, own: bool) -> StructClass {
    StructClass::create(&mut ByteCursor::new(bytes), own)
}

pub fn parse(bytes: Vec<u8>) -> StructClass {
    create(bytes, true)
}

/// Compare golden and rendered text; a `?` golden token matches any token.
fn first_difference(golden: &str, actual: &str) -> Option<String> {
    let g: Vec<&str> = golden.lines().collect();
    let a: Vec<&str> = actual.lines().collect();
    for i in 0..g.len().max(a.len()) {
        let gl = g.get(i).copied().unwrap_or("<eof>");
        let al = a.get(i).copied().unwrap_or("<eof>");
        let same = gl == al || {
            let gt: Vec<&str> = gl.split(' ').collect();
            let at: Vec<&str> = al.split(' ').collect();
            gt.len() == at.len() && gt.iter().zip(&at).all(|(x, y)| *x == "?" || x == y)
        };
        if !same {
            return Some(format!("line {}:\n  javap: {}\n  kuna:  {}", i + 1, gl, al));
        }
    }
    None
}

fn class_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|e| e == "class") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// Parse every class under `classes` and compare with `<golden_root>/<rel>.golden`.
fn compare_tree(classes: &Path, golden_root: &Path) -> (usize, Vec<String>) {
    let mut checked = 0;
    let mut failures = Vec::new();
    for (n, class) in class_files(classes).iter().enumerate() {
        let rel = class
            .strip_prefix(classes)
            .unwrap()
            .with_extension("golden");
        let golden_path = golden_root.join(rel);
        let Ok(golden) = std::fs::read_to_string(&golden_path) else {
            continue;
        };
        let bytes = std::fs::read(class).unwrap();
        let rendered = try_catch(|| render_class(parse(bytes)));
        checked += 1;
        match rendered {
            Ok(text) => {
                if let Some(diff) = first_difference(&golden, &text) {
                    failures.push(format!("{}: {}", class.display(), diff));
                }
            }
            Err(caught) => failures.push(format!(
                "{}: threw {} at {}",
                class.display(),
                caught.message,
                caught.location
            )),
        }
        reset_class_scope();
        if n % 500 == 499 {
            reset_session_scope();
        }
    }
    (checked, failures)
}

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/classfile/classes")
}

#[test]
fn corpus_matches_javap() {
    let dir = corpus_dir();
    let (checked, failures) = compare_tree(&dir, &dir);
    assert!(
        checked >= 100,
        "only {} golden classes found under {}",
        checked,
        dir.display()
    );
    assert!(
        failures.is_empty(),
        "{} of {} classes differ from javap:\n{}",
        failures.len(),
        checked,
        failures.join("\n")
    );
    eprintln!("class-file corpus: {} classes match javap", checked);
}

#[test]
fn javap_sweep() {
    let Ok(dir) = std::env::var("KUNA_JVM_JAVAP_SWEEP") else {
        return;
    };
    let golden = std::env::var("KUNA_JVM_JAVAP_SWEEP_GOLDEN").unwrap_or_else(|_| dir.clone());
    let (checked, failures) = compare_tree(Path::new(&dir), Path::new(&golden));
    for f in failures.iter().take(20) {
        eprintln!("{}", f);
    }
    eprintln!(
        "javap sweep: {} classes compared, {} differ",
        checked,
        failures.len()
    );
    assert!(failures.is_empty());
}

#[test]
fn parse_sweep() {
    let Ok(dir) = std::env::var("KUNA_JVM_PARSE_SWEEP") else {
        return;
    };
    let mut parsed = 0;
    let mut instructions = 0;
    let mut failures = Vec::new();
    for (n, class) in class_files(Path::new(&dir)).iter().enumerate() {
        let bytes = std::fs::read(class).unwrap();
        match try_catch(|| {
            let cl = parse(bytes);
            let mut count = 0;
            for mt in cl.get_methods().iter() {
                mt.expand_data(cl);
                if mt.contains_code() {
                    count += mt.get_instruction_sequence().unwrap().length() as usize;
                }
                mt.release_resources();
            }
            count
        }) {
            Ok(count) => {
                parsed += 1;
                instructions += count;
            }
            Err(caught) => failures.push(format!(
                "{}: {} at {}",
                class.display(),
                caught.message,
                caught.location
            )),
        }
        reset_class_scope();
        if n % 500 == 499 {
            reset_session_scope();
        }
    }
    for f in failures.iter().take(20) {
        eprintln!("{}", f);
    }
    eprintln!(
        "parse sweep: {} classes parsed ({} instructions), {} failed",
        parsed,
        instructions,
        failures.len()
    );
    assert!(failures.is_empty());
}
