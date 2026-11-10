//! Port of `org/jetbrains/java/decompiler/util/TextUtil.java`.

use crate::bytecode_version::BytecodeVersion;
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::rt::{jchar, jnum, JavaStr};
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

pub struct TextUtil;

const KEYWORDS: [&str; 52] = [
    "abstract",
    "default",
    "if",
    "private",
    "this",
    "boolean",
    "do",
    "implements",
    "protected",
    "throw",
    "break",
    "double",
    "import",
    "public",
    "throws",
    "byte",
    "else",
    "instanceof",
    "return",
    "transient",
    "case",
    "extends",
    "int",
    "short",
    "try",
    "catch",
    "final",
    "interface",
    "static",
    "void",
    "char",
    "finally",
    "long",
    "strictfp",
    "volatile",
    "class",
    "float",
    "native",
    "super",
    "while",
    "const",
    "for",
    "new",
    "switch",
    "continue",
    "goto",
    "package",
    "synchronized",
    "true",
    "false",
    "null",
    "assert",
];

impl TextUtil {
    pub fn write_qualified_super(buf: &mut TextBuffer, qualifier: &str) {
        let class_node = DecompilerContext::get_current_class_node()
            .expect("TextUtil.writeQualifiedSuper: no current class node");
        let qualified_name = class_node.with(|d| d.class_struct).qualified_name();
        if qualifier != qualified_name {
            let short_name = DecompilerContext::get_import_collector()
                .get_short_name(&ExprProcessor::build_java_class_name(qualifier))
                .expect("NullPointerException");
            buf.append_all_classes(&short_name, Some(qualifier))
                .append_char('.');
        }
        buf.append("super");
    }

    pub fn get_indent_string(length: i32) -> String {
        if length == 0 {
            return String::new();
        }
        let mut buf = String::new();
        let indent = DecompilerContext::get_property(IFernflowerPreferences::INDENT_STRING)
            .unwrap_or_else(|| "null".to_string());
        TextUtil::append(&mut buf, &indent, length);
        buf
    }

    pub fn append(buf: &mut String, string: &str, times: i32) {
        for _ in 0..times.max(0) {
            buf.push_str(string);
        }
    }

    pub fn is_printable_unicode(c: u16) -> bool {
        let t = jchar::get_type(c);
        t != jchar::UNASSIGNED
            && t != jchar::LINE_SEPARATOR
            && t != jchar::PARAGRAPH_SEPARATOR
            && t != jchar::CONTROL
            && t != jchar::FORMAT
            && t != jchar::PRIVATE_USE
            && t != jchar::SURROGATE
    }

    /// `\uXXXX`: the last four hex digits of `value`, zero-padded.
    pub fn char_to_unicode_literal(value: i32) -> String {
        let s_temp = jnum::int_to_hex_string(value);
        let padded = format!("0000{s_temp}");
        format!("\\u{}", &padded[s_temp.len()..])
    }

    pub fn is_valid_identifier(id: &str, version: &BytecodeVersion, mt: StructMethod) -> bool {
        TextUtil::is_java_identifier(id) && !TextUtil::is_keyword(id, version, mt)
    }

    pub fn is_java_identifier(id: &str) -> bool {
        let units = id.utf16();
        if units.is_empty() || !jchar::is_java_identifier_start(units[0]) {
            return false;
        }
        units[1..]
            .iter()
            .all(|&c| jchar::is_java_identifier_part(c))
    }

    pub fn is_keyword(id: &str, version: &BytecodeVersion, mt: StructMethod) -> bool {
        KEYWORDS.contains(&id)
            || (version.has_enums() && id == "enum")
            || ((mt.get_access_flags() & ACC_STATIC) != 0 && id == "this")
    }

    pub fn get_instruction_name(opcode: i32) -> &'static str {
        OPCODE_NAMES[opcode as usize]
    }
}

const OPCODE_NAMES: [&str; 202] = [
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
