//! Port of `org/jetbrains/java/decompiler/code/CodeConstants.java`.
//!
//! Every constant is a module-level `pub const` (`code_constants::OPC_GOTO`) and,
//! for call sites written against the interface, an associated const
//! (`CodeConstants::OPC_GOTO`). Java names are upper-snake-cased
//! (`CONSTANT_MethodHandle_REF_getField` -> `CONSTANT_METHOD_HANDLE_REF_GET_FIELD`).

use std::rc::Rc;

use crate::rt::JHashSet;

pub struct CodeConstants;

macro_rules! code_constants {
    ($($name:ident: $ty:ty = $value:expr;)*) => {
        $(pub const $name: $ty = $value;)*
        impl CodeConstants {
            $(pub const $name: $ty = $value;)*
        }
    };
}

code_constants! {
    TYPE_BYTE: i32 = 0;
    TYPE_CHAR: i32 = 1;
    TYPE_DOUBLE: i32 = 2;
    TYPE_FLOAT: i32 = 3;
    TYPE_INT: i32 = 4;
    TYPE_LONG: i32 = 5;
    TYPE_SHORT: i32 = 6;
    TYPE_BOOLEAN: i32 = 7;
    TYPE_OBJECT: i32 = 8;
    TYPE_ADDRESS: i32 = 9;
    TYPE_VOID: i32 = 10;
    TYPE_ANY: i32 = 11;
    TYPE_GROUP2_EMPTY: i32 = 12;
    TYPE_NULL: i32 = 13;
    TYPE_NOTINITIALIZED: i32 = 14;
    TYPE_BYTECHAR: i32 = 15;
    TYPE_SHORTCHAR: i32 = 16;
    TYPE_UNKNOWN: i32 = 17;
    TYPE_GENVAR: i32 = 18;
    ACC_PUBLIC: i32 = 0x0001;
    ACC_PRIVATE: i32 = 0x0002;
    ACC_PROTECTED: i32 = 0x0004;
    ACC_STATIC: i32 = 0x0008;
    ACC_FINAL: i32 = 0x0010;
    ACC_SYNCHRONIZED: i32 = 0x0020;
    ACC_OPEN: i32 = 0x0020;
    ACC_TRANSITIVE: i32 = 0x0020;
    ACC_STATIC_PHASE: i32 = 0x0040;
    ACC_NATIVE: i32 = 0x0100;
    ACC_ABSTRACT: i32 = 0x0400;
    ACC_STRICT: i32 = 0x0800;
    ACC_VOLATILE: i32 = 0x0040;
    ACC_BRIDGE: i32 = 0x0040;
    ACC_TRANSIENT: i32 = 0x0080;
    ACC_VARARGS: i32 = 0x0080;
    ACC_SYNTHETIC: i32 = 0x1000;
    ACC_ANNOTATION: i32 = 0x2000;
    ACC_ENUM: i32 = 0x4000;
    ACC_MANDATED: i32 = 0x8000;
    ACC_MODULE: i32 = 0x8000;
    ACC_SUPER: i32 = 0x0020;
    ACC_INTERFACE: i32 = 0x0200;
    GROUP_GENERAL: i32 = 1;
    GROUP_JUMP: i32 = 2;
    GROUP_SWITCH: i32 = 3;
    GROUP_INVOCATION: i32 = 4;
    GROUP_FIELDACCESS: i32 = 5;
    GROUP_RETURN: i32 = 6;
    CONSTANT_UTF8: i32 = 1;
    CONSTANT_INTEGER: i32 = 3;
    CONSTANT_FLOAT: i32 = 4;
    CONSTANT_LONG: i32 = 5;
    CONSTANT_DOUBLE: i32 = 6;
    CONSTANT_CLASS: i32 = 7;
    CONSTANT_STRING: i32 = 8;
    CONSTANT_FIELDREF: i32 = 9;
    CONSTANT_METHODREF: i32 = 10;
    CONSTANT_INTERFACE_METHODREF: i32 = 11;
    CONSTANT_NAME_AND_TYPE: i32 = 12;
    CONSTANT_METHOD_HANDLE: i32 = 15;
    CONSTANT_METHOD_TYPE: i32 = 16;
    CONSTANT_DYNAMIC: i32 = 17;
    CONSTANT_INVOKE_DYNAMIC: i32 = 18;
    CONSTANT_MODULE: i32 = 19;
    CONSTANT_PACKAGE: i32 = 20;
    CONSTANT_METHOD_HANDLE_REF_GET_FIELD: i32 = 1;
    CONSTANT_METHOD_HANDLE_REF_GET_STATIC: i32 = 2;
    CONSTANT_METHOD_HANDLE_REF_PUT_FIELD: i32 = 3;
    CONSTANT_METHOD_HANDLE_REF_PUT_STATIC: i32 = 4;
    CONSTANT_METHOD_HANDLE_REF_INVOKE_VIRTUAL: i32 = 5;
    CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC: i32 = 6;
    CONSTANT_METHOD_HANDLE_REF_INVOKE_SPECIAL: i32 = 7;
    CONSTANT_METHOD_HANDLE_REF_NEW_INVOKE_SPECIAL: i32 = 8;
    CONSTANT_METHOD_HANDLE_REF_INVOKE_INTERFACE: i32 = 9;
    OPC_NOP: i32 = 0;
    OPC_ACONST_NULL: i32 = 1;
    OPC_ICONST_M1: i32 = 2;
    OPC_ICONST_0: i32 = 3;
    OPC_ICONST_1: i32 = 4;
    OPC_ICONST_2: i32 = 5;
    OPC_ICONST_3: i32 = 6;
    OPC_ICONST_4: i32 = 7;
    OPC_ICONST_5: i32 = 8;
    OPC_LCONST_0: i32 = 9;
    OPC_LCONST_1: i32 = 10;
    OPC_FCONST_0: i32 = 11;
    OPC_FCONST_1: i32 = 12;
    OPC_FCONST_2: i32 = 13;
    OPC_DCONST_0: i32 = 14;
    OPC_DCONST_1: i32 = 15;
    OPC_BIPUSH: i32 = 16;
    OPC_SIPUSH: i32 = 17;
    OPC_LDC: i32 = 18;
    OPC_LDC_W: i32 = 19;
    OPC_LDC2_W: i32 = 20;
    OPC_ILOAD: i32 = 21;
    OPC_LLOAD: i32 = 22;
    OPC_FLOAD: i32 = 23;
    OPC_DLOAD: i32 = 24;
    OPC_ALOAD: i32 = 25;
    OPC_ILOAD_0: i32 = 26;
    OPC_ILOAD_1: i32 = 27;
    OPC_ILOAD_2: i32 = 28;
    OPC_ILOAD_3: i32 = 29;
    OPC_LLOAD_0: i32 = 30;
    OPC_LLOAD_1: i32 = 31;
    OPC_LLOAD_2: i32 = 32;
    OPC_LLOAD_3: i32 = 33;
    OPC_FLOAD_0: i32 = 34;
    OPC_FLOAD_1: i32 = 35;
    OPC_FLOAD_2: i32 = 36;
    OPC_FLOAD_3: i32 = 37;
    OPC_DLOAD_0: i32 = 38;
    OPC_DLOAD_1: i32 = 39;
    OPC_DLOAD_2: i32 = 40;
    OPC_DLOAD_3: i32 = 41;
    OPC_ALOAD_0: i32 = 42;
    OPC_ALOAD_1: i32 = 43;
    OPC_ALOAD_2: i32 = 44;
    OPC_ALOAD_3: i32 = 45;
    OPC_IALOAD: i32 = 46;
    OPC_LALOAD: i32 = 47;
    OPC_FALOAD: i32 = 48;
    OPC_DALOAD: i32 = 49;
    OPC_AALOAD: i32 = 50;
    OPC_BALOAD: i32 = 51;
    OPC_CALOAD: i32 = 52;
    OPC_SALOAD: i32 = 53;
    OPC_ISTORE: i32 = 54;
    OPC_LSTORE: i32 = 55;
    OPC_FSTORE: i32 = 56;
    OPC_DSTORE: i32 = 57;
    OPC_ASTORE: i32 = 58;
    OPC_ISTORE_0: i32 = 59;
    OPC_ISTORE_1: i32 = 60;
    OPC_ISTORE_2: i32 = 61;
    OPC_ISTORE_3: i32 = 62;
    OPC_LSTORE_0: i32 = 63;
    OPC_LSTORE_1: i32 = 64;
    OPC_LSTORE_2: i32 = 65;
    OPC_LSTORE_3: i32 = 66;
    OPC_FSTORE_0: i32 = 67;
    OPC_FSTORE_1: i32 = 68;
    OPC_FSTORE_2: i32 = 69;
    OPC_FSTORE_3: i32 = 70;
    OPC_DSTORE_0: i32 = 71;
    OPC_DSTORE_1: i32 = 72;
    OPC_DSTORE_2: i32 = 73;
    OPC_DSTORE_3: i32 = 74;
    OPC_ASTORE_0: i32 = 75;
    OPC_ASTORE_1: i32 = 76;
    OPC_ASTORE_2: i32 = 77;
    OPC_ASTORE_3: i32 = 78;
    OPC_IASTORE: i32 = 79;
    OPC_LASTORE: i32 = 80;
    OPC_FASTORE: i32 = 81;
    OPC_DASTORE: i32 = 82;
    OPC_AASTORE: i32 = 83;
    OPC_BASTORE: i32 = 84;
    OPC_CASTORE: i32 = 85;
    OPC_SASTORE: i32 = 86;
    OPC_POP: i32 = 87;
    OPC_POP2: i32 = 88;
    OPC_DUP: i32 = 89;
    OPC_DUP_X1: i32 = 90;
    OPC_DUP_X2: i32 = 91;
    OPC_DUP2: i32 = 92;
    OPC_DUP2_X1: i32 = 93;
    OPC_DUP2_X2: i32 = 94;
    OPC_SWAP: i32 = 95;
    OPC_IADD: i32 = 96;
    OPC_LADD: i32 = 97;
    OPC_FADD: i32 = 98;
    OPC_DADD: i32 = 99;
    OPC_ISUB: i32 = 100;
    OPC_LSUB: i32 = 101;
    OPC_FSUB: i32 = 102;
    OPC_DSUB: i32 = 103;
    OPC_IMUL: i32 = 104;
    OPC_LMUL: i32 = 105;
    OPC_FMUL: i32 = 106;
    OPC_DMUL: i32 = 107;
    OPC_IDIV: i32 = 108;
    OPC_LDIV: i32 = 109;
    OPC_FDIV: i32 = 110;
    OPC_DDIV: i32 = 111;
    OPC_IREM: i32 = 112;
    OPC_LREM: i32 = 113;
    OPC_FREM: i32 = 114;
    OPC_DREM: i32 = 115;
    OPC_INEG: i32 = 116;
    OPC_LNEG: i32 = 117;
    OPC_FNEG: i32 = 118;
    OPC_DNEG: i32 = 119;
    OPC_ISHL: i32 = 120;
    OPC_LSHL: i32 = 121;
    OPC_ISHR: i32 = 122;
    OPC_LSHR: i32 = 123;
    OPC_IUSHR: i32 = 124;
    OPC_LUSHR: i32 = 125;
    OPC_IAND: i32 = 126;
    OPC_LAND: i32 = 127;
    OPC_IOR: i32 = 128;
    OPC_LOR: i32 = 129;
    OPC_IXOR: i32 = 130;
    OPC_LXOR: i32 = 131;
    OPC_IINC: i32 = 132;
    OPC_I2L: i32 = 133;
    OPC_I2F: i32 = 134;
    OPC_I2D: i32 = 135;
    OPC_L2I: i32 = 136;
    OPC_L2F: i32 = 137;
    OPC_L2D: i32 = 138;
    OPC_F2I: i32 = 139;
    OPC_F2L: i32 = 140;
    OPC_F2D: i32 = 141;
    OPC_D2I: i32 = 142;
    OPC_D2L: i32 = 143;
    OPC_D2F: i32 = 144;
    OPC_I2B: i32 = 145;
    OPC_I2C: i32 = 146;
    OPC_I2S: i32 = 147;
    OPC_LCMP: i32 = 148;
    OPC_FCMPL: i32 = 149;
    OPC_FCMPG: i32 = 150;
    OPC_DCMPL: i32 = 151;
    OPC_DCMPG: i32 = 152;
    OPC_IFEQ: i32 = 153;
    OPC_IFNE: i32 = 154;
    OPC_IFLT: i32 = 155;
    OPC_IFGE: i32 = 156;
    OPC_IFGT: i32 = 157;
    OPC_IFLE: i32 = 158;
    OPC_IF_ICMPEQ: i32 = 159;
    OPC_IF_ICMPNE: i32 = 160;
    OPC_IF_ICMPLT: i32 = 161;
    OPC_IF_ICMPGE: i32 = 162;
    OPC_IF_ICMPGT: i32 = 163;
    OPC_IF_ICMPLE: i32 = 164;
    OPC_IF_ACMPEQ: i32 = 165;
    OPC_IF_ACMPNE: i32 = 166;
    OPC_GOTO: i32 = 167;
    OPC_JSR: i32 = 168;
    OPC_RET: i32 = 169;
    OPC_TABLESWITCH: i32 = 170;
    OPC_LOOKUPSWITCH: i32 = 171;
    OPC_IRETURN: i32 = 172;
    OPC_LRETURN: i32 = 173;
    OPC_FRETURN: i32 = 174;
    OPC_DRETURN: i32 = 175;
    OPC_ARETURN: i32 = 176;
    OPC_RETURN: i32 = 177;
    OPC_GETSTATIC: i32 = 178;
    OPC_PUTSTATIC: i32 = 179;
    OPC_GETFIELD: i32 = 180;
    OPC_PUTFIELD: i32 = 181;
    OPC_INVOKEVIRTUAL: i32 = 182;
    OPC_INVOKESPECIAL: i32 = 183;
    OPC_INVOKESTATIC: i32 = 184;
    OPC_INVOKEINTERFACE: i32 = 185;
    OPC_INVOKEDYNAMIC: i32 = 186;
    OPC_NEW: i32 = 187;
    OPC_NEWARRAY: i32 = 188;
    OPC_ANEWARRAY: i32 = 189;
    OPC_ARRAYLENGTH: i32 = 190;
    OPC_ATHROW: i32 = 191;
    OPC_CHECKCAST: i32 = 192;
    OPC_INSTANCEOF: i32 = 193;
    OPC_MONITORENTER: i32 = 194;
    OPC_MONITOREXIT: i32 = 195;
    OPC_WIDE: i32 = 196;
    OPC_MULTIANEWARRAY: i32 = 197;
    OPC_IFNULL: i32 = 198;
    OPC_IFNONNULL: i32 = 199;
    OPC_GOTO_W: i32 = 200;
    OPC_JSR_W: i32 = 201;
    CLINIT_NAME: &'static str = "<clinit>";
    INIT_NAME: &'static str = "<init>";

}

/// JVMS 2.9.3 signature polymorphic methods.
pub struct SignaturePolymorphic;

struct PolymorphicSets {
    var_handle_return_polymorphic: Rc<JHashSet<String>>,
    var_handle_parameter_polymorphic: Rc<JHashSet<String>>,
}

fn string_set(items: &[&str]) -> JHashSet<String> {
    JHashSet::from_collection(items.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

thread_local! {
    static POLYMORPHIC_SETS: PolymorphicSets = {
        let mut ret = string_set(&["get", "getVolatile", "getOpaque", "getAcquire"]);
        let mut param = string_set(&[
            "set",
            "setVolatile",
            "setOpaque",
            "setRelease",
            "compareAndSet",
            "weakCompareAndSetPlain",
            "weakCompareAndSet",
            "weakCompareAndSetAcquire",
            "weakCompareAndSetRelease",
        ]);
        let base = [
            "compareAndExchange",
            "getAndSet",
            "getAndAdd",
            "getAndBitwiseOr",
            "getAndBitwiseAnd",
            "getAndBitwiseXor",
        ];
        for b in base {
            ret.add(b.to_string());
            ret.add(format!("{}Acquire", b));
            ret.add(format!("{}Release", b));
        }
        param.add_all(ret.to_vec());
        PolymorphicSets {
            var_handle_return_polymorphic: Rc::new(ret),
            var_handle_parameter_polymorphic: Rc::new(param),
        }
    };
}

impl SignaturePolymorphic {
    #[allow(non_snake_case)]
    pub fn VAR_HANDLE_RETURN_POLYMORPHIC() -> Rc<JHashSet<String>> {
        POLYMORPHIC_SETS.with(|s| s.var_handle_return_polymorphic.clone())
    }

    #[allow(non_snake_case)]
    pub fn VAR_HANDLE_PARAMETER_POLYMORPHIC() -> Rc<JHashSet<String>> {
        POLYMORPHIC_SETS.with(|s| s.var_handle_parameter_polymorphic.clone())
    }
}

impl CodeConstants {
    pub fn is_signature_polymorphic(classname: &str, name: &str, check_return: bool) -> bool {
        if classname == "java/lang/invoke/MethodHandle" {
            return name == "invokeExact" || name == "invoke";
        }
        if classname == "java/lang/invoke/VarHandle" {
            let name = name.to_string();
            return if check_return {
                SignaturePolymorphic::VAR_HANDLE_RETURN_POLYMORPHIC().contains(&name)
            } else {
                SignaturePolymorphic::VAR_HANDLE_PARAMETER_POLYMORPHIC().contains(&name)
            };
        }
        false
    }

    pub fn is_return_polymorphic(classname: &str, name: &str) -> bool {
        CodeConstants::is_signature_polymorphic(classname, name, true)
    }

    pub fn are_parameters_polymorphic(classname: &str, name: &str) -> bool {
        CodeConstants::is_signature_polymorphic(classname, name, false)
    }
}
