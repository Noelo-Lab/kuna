//! The constant pool, `CodeConstants`, `BytecodeVersion`, `DataPoint` and the
//! match engine, against Vineflower 1.12.0 golden vectors (`data/f2/GenF2Golden.java`).

use std::rc::Rc;

use crate::f2_types::{esc, lines, run, same, unesc, Checker};
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::bytecode_version::BytecodeVersion;
use kuna_jvm::code_constants::*;
use kuna_jvm::code_type::CodeType;
use kuna_jvm::constant_pool::ConstantPool;
use kuna_jvm::data_point::DataPoint;
use kuna_jvm::i_matchable::{IMatchable, MatchProperties};
use kuna_jvm::match_engine::MatchEngine;
use kuna_jvm::match_node::{MatchNode, MatchValue};
use kuna_jvm::pooled_constant::PooledConstant;
use kuna_jvm::primitive_constant::ConstValue;
use kuna_jvm::var_type::VarType;

const POOLS: &str = include_str!("data/f2/pools.tsv");
const MATCH: &str = include_str!("data/f2/match.tsv");
const CONSTANTS: &str = include_str!("data/f2/constants.tsv");

pub fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub fn dump_const(c: Option<&PooledConstant>) -> String {
    match c {
        None => "\\N".into(),
        Some(PooledConstant::Primitive(p)) => {
            let v = match &p.value {
                None => "\\N".to_string(),
                Some(ConstValue::Integer(i)) => format!("I{i}"),
                Some(ConstValue::Long(l)) => format!("J{l}"),
                Some(ConstValue::Float(f)) => format!("F{:x}", f.to_bits()),
                Some(ConstValue::Double(d)) => format!("D{:x}", d.to_bits()),
                Some(ConstValue::String(s)) => format!("S{}", esc(Some(s))),
            };
            format!("P{},{},{},{}", p.type_, p.index, p.is_array as i32, v)
        }
        Some(PooledConstant::Link(l)) => format!(
            "L{},{},{},{},{},{}",
            l.type_,
            l.index1,
            l.index2,
            esc(l.classname.as_deref()),
            esc(Some(&l.elementname)),
            esc(Some(&l.descriptor))
        ),
    }
}

pub fn read_pool(class_bytes: &[u8]) -> ConstantPool {
    let mut cursor = ByteCursor::new(class_bytes[8..].to_vec());
    ConstantPool::new_without_interceptor(&mut cursor)
}

#[test]
fn constant_pools_match_vineflower() {
    let mut c = Checker::new("ConstantPool");
    for l in lines(POOLS, "PO") {
        let bytes = unhex(l[2]);
        c.field(&format!("PO {}", l[1]), l[3], || {
            let pool = read_pool(&bytes);
            let all: Vec<String> = pool
                .get_pool()
                .iter()
                .map(|e| dump_const(e.as_ref()))
                .collect();
            all.join("|")
        });
    }
    let bad = unhex(
        lines(POOLS, "PO")
            .iter()
            .find(|l| l[1] == "bad-tag-200")
            .unwrap()[2],
    );
    assert_eq!(
        run(|| {
            read_pool(&bad);
            String::new()
        }),
        "EXC:RuntimeException:Invalid Constant Pool entry #2 Type: -56"
    );
    c.finish();
}

#[test]
fn constant_pool_accessors() {
    let row = lines(POOLS, "PO")
        .into_iter()
        .find(|l| l[1] == "kitchen-sink")
        .unwrap();
    let pool = read_pool(&unhex(row[2]));
    assert_eq!(
        &*pool.get_primitive_constant(1).unwrap().get_string(),
        "java/lang/String"
    );
    let mr = pool.get_link_constant(2).unwrap();
    assert_eq!(mr.classname.as_deref(), Some("java/lang/String"));
    assert_eq!(
        (mr.elementname.as_str(), mr.descriptor.as_str()),
        ("valueOf", "(I)Ljava/lang/String;")
    );
    assert!(pool.get_constant(23).is_none());
    assert!(pool.get_primitive_constant(28).unwrap().is_array);
    assert!(!pool.get_primitive_constant(29).unwrap().is_array);
    let [name, desc] = pool.get_class_element(ConstantPool::METHOD, "java/lang/String", 31, 32);
    assert_eq!(
        (name.as_str(), desc.as_str()),
        ("valueOf", "(I)Ljava/lang/String;")
    );
    assert!(pool
        .get_constant(2)
        .unwrap()
        .equals(&pool.get_constant(2).unwrap()));
    assert!(!pool
        .get_constant(2)
        .unwrap()
        .equals(&pool.get_constant(4).unwrap()));
    let caught = kuna_jvm::rt::try_catch(|| pool.get_link_constant(1)).unwrap_err();
    assert!(caught.is("ClassCastException"));
    assert!(pool
        .get_primitive_constant(18)
        .unwrap()
        .equals(&pool.get_primitive_constant(18).unwrap()));
    assert_eq!(
        pool.get_primitive_constant(18).unwrap().to_string(),
        "Primitive(0 -2147483648)"
    );
    assert_eq!(
        pool.get_primitive_constant(1).unwrap().to_string(),
        "Primitive(30 java/lang/String)"
    );
}

fn java_constant_name(n: &str) -> String {
    let mut out = String::new();
    for (i, part) in n.split('_').enumerate() {
        if i > 0 {
            out.push('_');
        }
        let chars: Vec<char> = part.chars().collect();
        for (j, &ch) in chars.iter().enumerate() {
            if j > 0
                && ch.is_ascii_uppercase()
                && (chars[j - 1].is_ascii_lowercase() || chars[j - 1].is_ascii_digit())
            {
                out.push('_');
            }
            out.push(ch.to_ascii_uppercase());
        }
    }
    out
}

fn rust_constants() -> Vec<(&'static str, String)> {
    vec![
        ("TYPE_BYTE", TYPE_BYTE.to_string()),
        ("TYPE_CHAR", TYPE_CHAR.to_string()),
        ("TYPE_DOUBLE", TYPE_DOUBLE.to_string()),
        ("TYPE_FLOAT", TYPE_FLOAT.to_string()),
        ("TYPE_INT", TYPE_INT.to_string()),
        ("TYPE_LONG", TYPE_LONG.to_string()),
        ("TYPE_SHORT", TYPE_SHORT.to_string()),
        ("TYPE_BOOLEAN", TYPE_BOOLEAN.to_string()),
        ("TYPE_OBJECT", TYPE_OBJECT.to_string()),
        ("TYPE_ADDRESS", TYPE_ADDRESS.to_string()),
        ("TYPE_VOID", TYPE_VOID.to_string()),
        ("TYPE_ANY", TYPE_ANY.to_string()),
        ("TYPE_GROUP2_EMPTY", TYPE_GROUP2_EMPTY.to_string()),
        ("TYPE_NULL", TYPE_NULL.to_string()),
        ("TYPE_NOTINITIALIZED", TYPE_NOTINITIALIZED.to_string()),
        ("TYPE_BYTECHAR", TYPE_BYTECHAR.to_string()),
        ("TYPE_SHORTCHAR", TYPE_SHORTCHAR.to_string()),
        ("TYPE_UNKNOWN", TYPE_UNKNOWN.to_string()),
        ("TYPE_GENVAR", TYPE_GENVAR.to_string()),
        ("ACC_PUBLIC", ACC_PUBLIC.to_string()),
        ("ACC_PRIVATE", ACC_PRIVATE.to_string()),
        ("ACC_PROTECTED", ACC_PROTECTED.to_string()),
        ("ACC_STATIC", ACC_STATIC.to_string()),
        ("ACC_FINAL", ACC_FINAL.to_string()),
        ("ACC_SYNCHRONIZED", ACC_SYNCHRONIZED.to_string()),
        ("ACC_OPEN", ACC_OPEN.to_string()),
        ("ACC_TRANSITIVE", ACC_TRANSITIVE.to_string()),
        ("ACC_STATIC_PHASE", ACC_STATIC_PHASE.to_string()),
        ("ACC_NATIVE", ACC_NATIVE.to_string()),
        ("ACC_ABSTRACT", ACC_ABSTRACT.to_string()),
        ("ACC_STRICT", ACC_STRICT.to_string()),
        ("ACC_VOLATILE", ACC_VOLATILE.to_string()),
        ("ACC_BRIDGE", ACC_BRIDGE.to_string()),
        ("ACC_TRANSIENT", ACC_TRANSIENT.to_string()),
        ("ACC_VARARGS", ACC_VARARGS.to_string()),
        ("ACC_SYNTHETIC", ACC_SYNTHETIC.to_string()),
        ("ACC_ANNOTATION", ACC_ANNOTATION.to_string()),
        ("ACC_ENUM", ACC_ENUM.to_string()),
        ("ACC_MANDATED", ACC_MANDATED.to_string()),
        ("ACC_MODULE", ACC_MODULE.to_string()),
        ("ACC_SUPER", ACC_SUPER.to_string()),
        ("ACC_INTERFACE", ACC_INTERFACE.to_string()),
        ("GROUP_GENERAL", GROUP_GENERAL.to_string()),
        ("GROUP_JUMP", GROUP_JUMP.to_string()),
        ("GROUP_SWITCH", GROUP_SWITCH.to_string()),
        ("GROUP_INVOCATION", GROUP_INVOCATION.to_string()),
        ("GROUP_FIELDACCESS", GROUP_FIELDACCESS.to_string()),
        ("GROUP_RETURN", GROUP_RETURN.to_string()),
        ("CONSTANT_UTF8", CONSTANT_UTF8.to_string()),
        ("CONSTANT_INTEGER", CONSTANT_INTEGER.to_string()),
        ("CONSTANT_FLOAT", CONSTANT_FLOAT.to_string()),
        ("CONSTANT_LONG", CONSTANT_LONG.to_string()),
        ("CONSTANT_DOUBLE", CONSTANT_DOUBLE.to_string()),
        ("CONSTANT_CLASS", CONSTANT_CLASS.to_string()),
        ("CONSTANT_STRING", CONSTANT_STRING.to_string()),
        ("CONSTANT_FIELDREF", CONSTANT_FIELDREF.to_string()),
        ("CONSTANT_METHODREF", CONSTANT_METHODREF.to_string()),
        (
            "CONSTANT_INTERFACE_METHODREF",
            CONSTANT_INTERFACE_METHODREF.to_string(),
        ),
        ("CONSTANT_NAME_AND_TYPE", CONSTANT_NAME_AND_TYPE.to_string()),
        ("CONSTANT_METHOD_HANDLE", CONSTANT_METHOD_HANDLE.to_string()),
        ("CONSTANT_METHOD_TYPE", CONSTANT_METHOD_TYPE.to_string()),
        ("CONSTANT_DYNAMIC", CONSTANT_DYNAMIC.to_string()),
        (
            "CONSTANT_INVOKE_DYNAMIC",
            CONSTANT_INVOKE_DYNAMIC.to_string(),
        ),
        ("CONSTANT_MODULE", CONSTANT_MODULE.to_string()),
        ("CONSTANT_PACKAGE", CONSTANT_PACKAGE.to_string()),
        (
            "CONSTANT_METHOD_HANDLE_REF_GET_FIELD",
            CONSTANT_METHOD_HANDLE_REF_GET_FIELD.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_GET_STATIC",
            CONSTANT_METHOD_HANDLE_REF_GET_STATIC.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_PUT_FIELD",
            CONSTANT_METHOD_HANDLE_REF_PUT_FIELD.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_PUT_STATIC",
            CONSTANT_METHOD_HANDLE_REF_PUT_STATIC.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_INVOKE_VIRTUAL",
            CONSTANT_METHOD_HANDLE_REF_INVOKE_VIRTUAL.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC",
            CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_INVOKE_SPECIAL",
            CONSTANT_METHOD_HANDLE_REF_INVOKE_SPECIAL.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_NEW_INVOKE_SPECIAL",
            CONSTANT_METHOD_HANDLE_REF_NEW_INVOKE_SPECIAL.to_string(),
        ),
        (
            "CONSTANT_METHOD_HANDLE_REF_INVOKE_INTERFACE",
            CONSTANT_METHOD_HANDLE_REF_INVOKE_INTERFACE.to_string(),
        ),
        ("OPC_NOP", OPC_NOP.to_string()),
        ("OPC_ACONST_NULL", OPC_ACONST_NULL.to_string()),
        ("OPC_ICONST_M1", OPC_ICONST_M1.to_string()),
        ("OPC_ICONST_0", OPC_ICONST_0.to_string()),
        ("OPC_ICONST_1", OPC_ICONST_1.to_string()),
        ("OPC_ICONST_2", OPC_ICONST_2.to_string()),
        ("OPC_ICONST_3", OPC_ICONST_3.to_string()),
        ("OPC_ICONST_4", OPC_ICONST_4.to_string()),
        ("OPC_ICONST_5", OPC_ICONST_5.to_string()),
        ("OPC_LCONST_0", OPC_LCONST_0.to_string()),
        ("OPC_LCONST_1", OPC_LCONST_1.to_string()),
        ("OPC_FCONST_0", OPC_FCONST_0.to_string()),
        ("OPC_FCONST_1", OPC_FCONST_1.to_string()),
        ("OPC_FCONST_2", OPC_FCONST_2.to_string()),
        ("OPC_DCONST_0", OPC_DCONST_0.to_string()),
        ("OPC_DCONST_1", OPC_DCONST_1.to_string()),
        ("OPC_BIPUSH", OPC_BIPUSH.to_string()),
        ("OPC_SIPUSH", OPC_SIPUSH.to_string()),
        ("OPC_LDC", OPC_LDC.to_string()),
        ("OPC_LDC_W", OPC_LDC_W.to_string()),
        ("OPC_LDC2_W", OPC_LDC2_W.to_string()),
        ("OPC_ILOAD", OPC_ILOAD.to_string()),
        ("OPC_LLOAD", OPC_LLOAD.to_string()),
        ("OPC_FLOAD", OPC_FLOAD.to_string()),
        ("OPC_DLOAD", OPC_DLOAD.to_string()),
        ("OPC_ALOAD", OPC_ALOAD.to_string()),
        ("OPC_ILOAD_0", OPC_ILOAD_0.to_string()),
        ("OPC_ILOAD_1", OPC_ILOAD_1.to_string()),
        ("OPC_ILOAD_2", OPC_ILOAD_2.to_string()),
        ("OPC_ILOAD_3", OPC_ILOAD_3.to_string()),
        ("OPC_LLOAD_0", OPC_LLOAD_0.to_string()),
        ("OPC_LLOAD_1", OPC_LLOAD_1.to_string()),
        ("OPC_LLOAD_2", OPC_LLOAD_2.to_string()),
        ("OPC_LLOAD_3", OPC_LLOAD_3.to_string()),
        ("OPC_FLOAD_0", OPC_FLOAD_0.to_string()),
        ("OPC_FLOAD_1", OPC_FLOAD_1.to_string()),
        ("OPC_FLOAD_2", OPC_FLOAD_2.to_string()),
        ("OPC_FLOAD_3", OPC_FLOAD_3.to_string()),
        ("OPC_DLOAD_0", OPC_DLOAD_0.to_string()),
        ("OPC_DLOAD_1", OPC_DLOAD_1.to_string()),
        ("OPC_DLOAD_2", OPC_DLOAD_2.to_string()),
        ("OPC_DLOAD_3", OPC_DLOAD_3.to_string()),
        ("OPC_ALOAD_0", OPC_ALOAD_0.to_string()),
        ("OPC_ALOAD_1", OPC_ALOAD_1.to_string()),
        ("OPC_ALOAD_2", OPC_ALOAD_2.to_string()),
        ("OPC_ALOAD_3", OPC_ALOAD_3.to_string()),
        ("OPC_IALOAD", OPC_IALOAD.to_string()),
        ("OPC_LALOAD", OPC_LALOAD.to_string()),
        ("OPC_FALOAD", OPC_FALOAD.to_string()),
        ("OPC_DALOAD", OPC_DALOAD.to_string()),
        ("OPC_AALOAD", OPC_AALOAD.to_string()),
        ("OPC_BALOAD", OPC_BALOAD.to_string()),
        ("OPC_CALOAD", OPC_CALOAD.to_string()),
        ("OPC_SALOAD", OPC_SALOAD.to_string()),
        ("OPC_ISTORE", OPC_ISTORE.to_string()),
        ("OPC_LSTORE", OPC_LSTORE.to_string()),
        ("OPC_FSTORE", OPC_FSTORE.to_string()),
        ("OPC_DSTORE", OPC_DSTORE.to_string()),
        ("OPC_ASTORE", OPC_ASTORE.to_string()),
        ("OPC_ISTORE_0", OPC_ISTORE_0.to_string()),
        ("OPC_ISTORE_1", OPC_ISTORE_1.to_string()),
        ("OPC_ISTORE_2", OPC_ISTORE_2.to_string()),
        ("OPC_ISTORE_3", OPC_ISTORE_3.to_string()),
        ("OPC_LSTORE_0", OPC_LSTORE_0.to_string()),
        ("OPC_LSTORE_1", OPC_LSTORE_1.to_string()),
        ("OPC_LSTORE_2", OPC_LSTORE_2.to_string()),
        ("OPC_LSTORE_3", OPC_LSTORE_3.to_string()),
        ("OPC_FSTORE_0", OPC_FSTORE_0.to_string()),
        ("OPC_FSTORE_1", OPC_FSTORE_1.to_string()),
        ("OPC_FSTORE_2", OPC_FSTORE_2.to_string()),
        ("OPC_FSTORE_3", OPC_FSTORE_3.to_string()),
        ("OPC_DSTORE_0", OPC_DSTORE_0.to_string()),
        ("OPC_DSTORE_1", OPC_DSTORE_1.to_string()),
        ("OPC_DSTORE_2", OPC_DSTORE_2.to_string()),
        ("OPC_DSTORE_3", OPC_DSTORE_3.to_string()),
        ("OPC_ASTORE_0", OPC_ASTORE_0.to_string()),
        ("OPC_ASTORE_1", OPC_ASTORE_1.to_string()),
        ("OPC_ASTORE_2", OPC_ASTORE_2.to_string()),
        ("OPC_ASTORE_3", OPC_ASTORE_3.to_string()),
        ("OPC_IASTORE", OPC_IASTORE.to_string()),
        ("OPC_LASTORE", OPC_LASTORE.to_string()),
        ("OPC_FASTORE", OPC_FASTORE.to_string()),
        ("OPC_DASTORE", OPC_DASTORE.to_string()),
        ("OPC_AASTORE", OPC_AASTORE.to_string()),
        ("OPC_BASTORE", OPC_BASTORE.to_string()),
        ("OPC_CASTORE", OPC_CASTORE.to_string()),
        ("OPC_SASTORE", OPC_SASTORE.to_string()),
        ("OPC_POP", OPC_POP.to_string()),
        ("OPC_POP2", OPC_POP2.to_string()),
        ("OPC_DUP", OPC_DUP.to_string()),
        ("OPC_DUP_X1", OPC_DUP_X1.to_string()),
        ("OPC_DUP_X2", OPC_DUP_X2.to_string()),
        ("OPC_DUP2", OPC_DUP2.to_string()),
        ("OPC_DUP2_X1", OPC_DUP2_X1.to_string()),
        ("OPC_DUP2_X2", OPC_DUP2_X2.to_string()),
        ("OPC_SWAP", OPC_SWAP.to_string()),
        ("OPC_IADD", OPC_IADD.to_string()),
        ("OPC_LADD", OPC_LADD.to_string()),
        ("OPC_FADD", OPC_FADD.to_string()),
        ("OPC_DADD", OPC_DADD.to_string()),
        ("OPC_ISUB", OPC_ISUB.to_string()),
        ("OPC_LSUB", OPC_LSUB.to_string()),
        ("OPC_FSUB", OPC_FSUB.to_string()),
        ("OPC_DSUB", OPC_DSUB.to_string()),
        ("OPC_IMUL", OPC_IMUL.to_string()),
        ("OPC_LMUL", OPC_LMUL.to_string()),
        ("OPC_FMUL", OPC_FMUL.to_string()),
        ("OPC_DMUL", OPC_DMUL.to_string()),
        ("OPC_IDIV", OPC_IDIV.to_string()),
        ("OPC_LDIV", OPC_LDIV.to_string()),
        ("OPC_FDIV", OPC_FDIV.to_string()),
        ("OPC_DDIV", OPC_DDIV.to_string()),
        ("OPC_IREM", OPC_IREM.to_string()),
        ("OPC_LREM", OPC_LREM.to_string()),
        ("OPC_FREM", OPC_FREM.to_string()),
        ("OPC_DREM", OPC_DREM.to_string()),
        ("OPC_INEG", OPC_INEG.to_string()),
        ("OPC_LNEG", OPC_LNEG.to_string()),
        ("OPC_FNEG", OPC_FNEG.to_string()),
        ("OPC_DNEG", OPC_DNEG.to_string()),
        ("OPC_ISHL", OPC_ISHL.to_string()),
        ("OPC_LSHL", OPC_LSHL.to_string()),
        ("OPC_ISHR", OPC_ISHR.to_string()),
        ("OPC_LSHR", OPC_LSHR.to_string()),
        ("OPC_IUSHR", OPC_IUSHR.to_string()),
        ("OPC_LUSHR", OPC_LUSHR.to_string()),
        ("OPC_IAND", OPC_IAND.to_string()),
        ("OPC_LAND", OPC_LAND.to_string()),
        ("OPC_IOR", OPC_IOR.to_string()),
        ("OPC_LOR", OPC_LOR.to_string()),
        ("OPC_IXOR", OPC_IXOR.to_string()),
        ("OPC_LXOR", OPC_LXOR.to_string()),
        ("OPC_IINC", OPC_IINC.to_string()),
        ("OPC_I2L", OPC_I2L.to_string()),
        ("OPC_I2F", OPC_I2F.to_string()),
        ("OPC_I2D", OPC_I2D.to_string()),
        ("OPC_L2I", OPC_L2I.to_string()),
        ("OPC_L2F", OPC_L2F.to_string()),
        ("OPC_L2D", OPC_L2D.to_string()),
        ("OPC_F2I", OPC_F2I.to_string()),
        ("OPC_F2L", OPC_F2L.to_string()),
        ("OPC_F2D", OPC_F2D.to_string()),
        ("OPC_D2I", OPC_D2I.to_string()),
        ("OPC_D2L", OPC_D2L.to_string()),
        ("OPC_D2F", OPC_D2F.to_string()),
        ("OPC_I2B", OPC_I2B.to_string()),
        ("OPC_I2C", OPC_I2C.to_string()),
        ("OPC_I2S", OPC_I2S.to_string()),
        ("OPC_LCMP", OPC_LCMP.to_string()),
        ("OPC_FCMPL", OPC_FCMPL.to_string()),
        ("OPC_FCMPG", OPC_FCMPG.to_string()),
        ("OPC_DCMPL", OPC_DCMPL.to_string()),
        ("OPC_DCMPG", OPC_DCMPG.to_string()),
        ("OPC_IFEQ", OPC_IFEQ.to_string()),
        ("OPC_IFNE", OPC_IFNE.to_string()),
        ("OPC_IFLT", OPC_IFLT.to_string()),
        ("OPC_IFGE", OPC_IFGE.to_string()),
        ("OPC_IFGT", OPC_IFGT.to_string()),
        ("OPC_IFLE", OPC_IFLE.to_string()),
        ("OPC_IF_ICMPEQ", OPC_IF_ICMPEQ.to_string()),
        ("OPC_IF_ICMPNE", OPC_IF_ICMPNE.to_string()),
        ("OPC_IF_ICMPLT", OPC_IF_ICMPLT.to_string()),
        ("OPC_IF_ICMPGE", OPC_IF_ICMPGE.to_string()),
        ("OPC_IF_ICMPGT", OPC_IF_ICMPGT.to_string()),
        ("OPC_IF_ICMPLE", OPC_IF_ICMPLE.to_string()),
        ("OPC_IF_ACMPEQ", OPC_IF_ACMPEQ.to_string()),
        ("OPC_IF_ACMPNE", OPC_IF_ACMPNE.to_string()),
        ("OPC_GOTO", OPC_GOTO.to_string()),
        ("OPC_JSR", OPC_JSR.to_string()),
        ("OPC_RET", OPC_RET.to_string()),
        ("OPC_TABLESWITCH", OPC_TABLESWITCH.to_string()),
        ("OPC_LOOKUPSWITCH", OPC_LOOKUPSWITCH.to_string()),
        ("OPC_IRETURN", OPC_IRETURN.to_string()),
        ("OPC_LRETURN", OPC_LRETURN.to_string()),
        ("OPC_FRETURN", OPC_FRETURN.to_string()),
        ("OPC_DRETURN", OPC_DRETURN.to_string()),
        ("OPC_ARETURN", OPC_ARETURN.to_string()),
        ("OPC_RETURN", OPC_RETURN.to_string()),
        ("OPC_GETSTATIC", OPC_GETSTATIC.to_string()),
        ("OPC_PUTSTATIC", OPC_PUTSTATIC.to_string()),
        ("OPC_GETFIELD", OPC_GETFIELD.to_string()),
        ("OPC_PUTFIELD", OPC_PUTFIELD.to_string()),
        ("OPC_INVOKEVIRTUAL", OPC_INVOKEVIRTUAL.to_string()),
        ("OPC_INVOKESPECIAL", OPC_INVOKESPECIAL.to_string()),
        ("OPC_INVOKESTATIC", OPC_INVOKESTATIC.to_string()),
        ("OPC_INVOKEINTERFACE", OPC_INVOKEINTERFACE.to_string()),
        ("OPC_INVOKEDYNAMIC", OPC_INVOKEDYNAMIC.to_string()),
        ("OPC_NEW", OPC_NEW.to_string()),
        ("OPC_NEWARRAY", OPC_NEWARRAY.to_string()),
        ("OPC_ANEWARRAY", OPC_ANEWARRAY.to_string()),
        ("OPC_ARRAYLENGTH", OPC_ARRAYLENGTH.to_string()),
        ("OPC_ATHROW", OPC_ATHROW.to_string()),
        ("OPC_CHECKCAST", OPC_CHECKCAST.to_string()),
        ("OPC_INSTANCEOF", OPC_INSTANCEOF.to_string()),
        ("OPC_MONITORENTER", OPC_MONITORENTER.to_string()),
        ("OPC_MONITOREXIT", OPC_MONITOREXIT.to_string()),
        ("OPC_WIDE", OPC_WIDE.to_string()),
        ("OPC_MULTIANEWARRAY", OPC_MULTIANEWARRAY.to_string()),
        ("OPC_IFNULL", OPC_IFNULL.to_string()),
        ("OPC_IFNONNULL", OPC_IFNONNULL.to_string()),
        ("OPC_GOTO_W", OPC_GOTO_W.to_string()),
        ("OPC_JSR_W", OPC_JSR_W.to_string()),
        ("CLINIT_NAME", CLINIT_NAME.to_string()),
        ("INIT_NAME", INIT_NAME.to_string()),
    ]
}

#[test]
fn code_constants_match_vineflower() {
    let ours = rust_constants();
    let mut seen = 0;
    for l in lines(CONSTANTS, "CC") {
        let name = java_constant_name(l[1]);
        let value = unesc(l[2]).unwrap();
        let ours = ours
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("missing {name}"));
        assert_eq!(ours.1, value, "{name}");
        seen += 1;
    }
    assert_eq!(seen, ours.len());
    assert_eq!(CodeConstants::OPC_GOTO, OPC_GOTO);
    assert_eq!(CodeConstants::INIT_NAME, "<init>");
    for l in lines(CONSTANTS, "SP") {
        assert_eq!(
            CodeConstants::is_return_polymorphic(l[1], l[2]).to_string(),
            l[3],
            "{} {}",
            l[1],
            l[2]
        );
        assert_eq!(
            CodeConstants::are_parameters_polymorphic(l[1], l[2]).to_string(),
            l[4],
            "{} {}",
            l[1],
            l[2]
        );
    }
}

#[test]
fn bytecode_version_predicates() {
    let v8 = BytecodeVersion::new(52, 0);
    assert!(v8.has_lambdas() && v8.has_enums() && !v8.has_jsr() && !v8.has_indy_string_concat());
    let v6 = BytecodeVersion::new(50, 0);
    assert!(v6.has_jsr() && !v6.has_invoke_dynamic());
    let p17 = BytecodeVersion::new(61, 0xffff);
    assert!(p17.has_switch_pattern_match() && p17.has_sealed_classes());
    assert!(!BytecodeVersion::new(61, 0).has_switch_pattern_match());
    assert!(BytecodeVersion::new(59, 65535).has_sealed_classes());
    assert!(
        BytecodeVersion::new(45, 2).predates_java() && !BytecodeVersion::new(45, 3).predates_java()
    );
    assert!(BytecodeVersion::new(48, 0).has14_class_references());
    assert_eq!(
        BytecodeVersion::new(0x1_0034, 0x2_0001),
        BytecodeVersion::new(52, 1)
    );
    assert_eq!(v8.compare_to(&v6), 1);
    assert_eq!(v6.compare_to(&v8), -1);
    assert_eq!(
        BytecodeVersion::new(52, 1).compare_to(&BytecodeVersion::new(52, 0)),
        1
    );
    assert!(v6 < v8);
    assert_eq!(
        BytecodeVersion::new(65, 65535).hash_code(),
        (65 << 16) | 65535
    );
    assert_eq!(
        BytecodeVersion::new(0xffff, 0).hash_code(),
        0xffff_0000u32 as i32
    );
}

#[test]
fn data_point_tracks_locals() {
    let mut p = DataPoint::new();
    p.set_variable(3, VarType::VARTYPE_INT());
    assert_eq!(p.get_local_variables().len(), 4);
    assert_eq!(p.get_variable(0).type_, CodeType::Notinitialized);
    assert!(p.get_variable(3).ptr_eq(&VarType::VARTYPE_INT()));
    assert_eq!(p.get_variable(9).type_, CodeType::Notinitialized);
    let q = p.copy();
    p.set_variable(0, VarType::VARTYPE_LONG());
    assert_eq!(q.get_variable(0).type_, CodeType::Notinitialized);
    assert_eq!(q.get_local_variables().len(), 4);
}

pub fn dump_node(n: &MatchNode) -> String {
    let props = [
        MatchProperties::StatementType,
        MatchProperties::StatementRet,
        MatchProperties::StatementStatsize,
        MatchProperties::StatementExprsize,
        MatchProperties::StatementPosition,
        MatchProperties::StatementIftype,
        MatchProperties::ExprentType,
        MatchProperties::ExprentRet,
        MatchProperties::ExprentPosition,
        MatchProperties::ExprentFunctype,
        MatchProperties::ExprentExittype,
        MatchProperties::ExprentConsttype,
        MatchProperties::ExprentConstvalue,
        MatchProperties::ExprentInvocationClass,
        MatchProperties::ExprentInvocationSignature,
        MatchProperties::ExprentParameter,
        MatchProperties::ExprentVarIndex,
        MatchProperties::ExprentName,
    ];
    let mut rules = Vec::new();
    for p in props {
        let mut vals = Vec::new();
        n.iterate_rules(|k, v| {
            if k == p {
                vals.push(match &v.value {
                    None => format!("{}:\\N:-", v.parameter),
                    Some(_) => format!(
                        "{}:{}:{}",
                        v.parameter,
                        esc(Some(&v.to_string())),
                        v.is_variable()
                    ),
                });
            }
            true
        });
        if !vals.is_empty() {
            rules.push(format!("{}={}", p.name(), vals.join(",")));
        }
    }
    let children: String = n.get_children().iter().map(dump_node).collect();
    format!("N({};{};{})", n.get_type(), rules.join("|"), children)
}

#[test]
fn match_engine_parses_like_vineflower() {
    let mut c = Checker::new("MatchEngine(String)");
    for l in lines(MATCH, "ME") {
        let d = unesc(l[1]).unwrap();
        c.field(&format!("ME {}", l[1]), l[2], || {
            dump_node(&MatchEngine::new_description(&d).get_root_node())
        });
    }
    c.finish();
    let joined = MatchEngine::new(&["statement statsize:1", " exprent ret:$x$"]);
    assert_eq!(
        dump_node(&joined.get_root_node()),
        dump_node(
            &MatchEngine::new_description("statement statsize:1\n exprent ret:$x$").get_root_node()
        )
    );
    assert!(same("EXC:X:a", "EXC:Panic:b"));
}

/// A statement (`stats`) or expression (`exprs` empty) tree standing in for the IR.
struct Mock {
    id: i32,
    statement: bool,
    stats: Vec<M>,
    exprs: Vec<M>,
}

#[derive(Clone)]
struct M(Rc<Mock>);

fn position(
    match_node: &MatchNode,
    prop: MatchProperties,
    list: &[M],
    index: i32,
) -> Option<Box<dyn IMatchable>> {
    if list.is_empty() {
        return None;
    }
    match match_node.get_rule_value(prop) {
        Some(p) => {
            let p = p.as_string();
            let n: i32 = p.parse().ok()?;
            let len = list.len() as i32;
            Some(Box::new(list[((len + n) % len) as usize].clone()))
        }
        None if (index as usize) < list.len() => Some(Box::new(list[index as usize].clone())),
        None => None,
    }
}

impl IMatchable for M {
    fn find_object(&self, match_node: &MatchNode, index: i32) -> Option<Box<dyn IMatchable>> {
        if match_node.get_type() == MatchNode::MATCHNODE_STATEMENT {
            position(
                match_node,
                MatchProperties::StatementPosition,
                &self.0.stats,
                index,
            )
        } else {
            position(
                match_node,
                MatchProperties::ExprentPosition,
                &self.0.exprs,
                index,
            )
        }
    }

    fn match_(&self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        let want = if self.0.statement {
            MatchNode::MATCHNODE_STATEMENT
        } else {
            MatchNode::MATCHNODE_EXPRENT
        };
        if match_node.get_type() != want {
            return false;
        }
        match_node.iterate_rules(|key, value| match key {
            MatchProperties::StatementStatsize => {
                self.0.stats.len() as i32 == value.value.as_ref().unwrap().as_integer()
            }
            MatchProperties::StatementRet | MatchProperties::ExprentRet => engine
                .check_and_set_variable_value(
                    &value.value.as_ref().unwrap().as_string(),
                    Some(MatchValue::Integer(self.0.id)),
                ),
            _ => true,
        })
    }
}

fn mock(id: i32, statement: bool, stats: Vec<M>, exprs: Vec<M>) -> M {
    M(Rc::new(Mock {
        id,
        statement,
        stats,
        exprs,
    }))
}

#[test]
fn match_engine_walks_and_binds() {
    let tree = mock(
        1,
        true,
        vec![
            mock(
                2,
                true,
                vec![],
                vec![
                    mock(20, false, vec![], vec![]),
                    mock(21, false, vec![], vec![]),
                ],
            ),
            mock(3, true, vec![], vec![]),
        ],
        vec![],
    );
    let engine = MatchEngine::new(&[
        "statement statsize:2 ret:$root$",
        " statement ret:$first$",
        "  exprent position:-1 ret:$last$",
        "  exprent ret:$e1$",
        " statement ret:$second$",
    ]);
    assert!(engine.match_(&tree));
    let var = |n: &str| engine.get_variable_value(n).map(|v| v.as_integer());
    assert_eq!(
        (var("$root$"), var("$first$"), var("$second$")),
        (Some(1), Some(2), Some(3))
    );
    assert_eq!((var("$last$"), var("$e1$")), (Some(21), Some(21)));

    let same_var = MatchEngine::new(&["statement", " statement ret:$x$", " statement ret:$x$"]);
    assert!(!same_var.match_(&tree));
    assert!(same_var.check_and_set_variable_value("$y$", Some(MatchValue::String("a".into()))));
    assert!(same_var.check_and_set_variable_value("$y$", Some(MatchValue::String("a".into()))));
    assert!(!same_var.check_and_set_variable_value("$y$", Some(MatchValue::Integer(1))));
    assert!(!same_var.check_and_set_variable_value("$y$", None));

    let too_many = MatchEngine::new(&["statement", " statement", " statement", " statement"]);
    assert!(!too_many.match_(&tree));
    let wrong_size = MatchEngine::new(&["statement statsize:3"]);
    assert!(!wrong_size.match_(&tree));
    assert!(wrong_size.get_variable_value("$root$").is_none());
}
