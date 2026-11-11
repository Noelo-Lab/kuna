//! Port of `org/jetbrains/java/decompiler/struct/StructMethod.java`.
//!
//! The method lives in session scope but its expanded instruction sequence is
//! class scope. A sequence that went stale with its arena (the driver resets
//! the class scope between the batches of an engine it reuses) counts as
//! released: `get_instruction_sequence` does not return it and `expand_data`
//! rebuilds it, so a reused engine sees what a fresh one does.

use std::rc::Rc;

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::code_constants::*;
use crate::constant_pool::ConstantPool;
use crate::decompiler_context::DecompilerContext;
use crate::exception_handler::ExceptionHandler;
use crate::exception_table::ExceptionTable;
use crate::full_instruction_sequence::FullInstructionSequence;
use crate::generic_main::GenericMain;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_variable_name_provider::IVariableNameProvider;
use crate::instruction::Instruction;
use crate::java_subclass;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::{JHashMap, JavaEq, JavaHash};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_local_variable_table_attribute::StructLocalVariableTableAttribute;
use crate::struct_member::{AttributeMap, StructMember, StructMemberKind};

const OPR_ICONST: [i32; 7] = [-1, 0, 1, 2, 3, 4, 5];
const OPR_LOADSTORE: [i32; 20] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
const OPCS_LOAD: [i32; 5] = [OPC_ILOAD, OPC_LLOAD, OPC_FLOAD, OPC_DLOAD, OPC_ALOAD];
const OPCS_STORE: [i32; 5] = [OPC_ISTORE, OPC_LSTORE, OPC_FSTORE, OPC_DSTORE, OPC_ASTORE];

pub struct StructMethodData {
    pub name: String,
    pub descriptor: String,
    pub bytecode_version: BytecodeVersion,
    pub local_variables: i32,
    pub code_and_exceptions: Option<Rc<[u8]>>,
    pub seq: Option<FullInstructionSequence>,
    pub expanded: bool,
    pub class_qualified_name: String,
    pub signature: Option<GenericMethodDescriptor>,
    pub renamer: Option<Rc<dyn IVariableNameProvider>>,
}

java_subclass!(pub StructMethod: StructMember, StructMemberKind::Method(StructMethodData), is_method, as_method, try_as_method);

impl JavaEq for StructMethod {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for StructMethod {
    fn java_hash(&self) -> i32 {
        self.index() as i32
    }
}

impl StructMethod {
    pub fn create(
        in_: &mut ByteCursor,
        pool: ConstantPool,
        cl_qualified_name: &str,
        bytecode_version: BytecodeVersion,
        own: bool,
    ) -> StructMethod {
        let access_flags = in_.read_unsigned_short();
        let name_index = in_.read_unsigned_short();
        let descriptor_index = in_.read_unsigned_short();

        let values = pool.get_class_element(
            ConstantPool::METHOD,
            cl_qualified_name,
            name_index,
            descriptor_index,
        );

        let mut attributes = StructMember::read_attributes_code(in_, pool, own, bytecode_version);
        let code = attributes
            .remove(&StructGeneralAttribute::ATTRIBUTE_CODE.name.to_string())
            .map(|a| a.as_code());
        if let Some(code) = code {
            attributes.put_all(code.code_attributes());
        }

        let mut signature = None;
        if DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES) {
            let signature_attr = attributes
                .get_cloned(&StructGeneralAttribute::ATTRIBUTE_SIGNATURE.name.to_string())
                .map(|a| a.as_generic_signature());
            if let Some(signature_attr) = signature_attr {
                signature = GenericMain::parse_method_signature(&signature_attr.get_signature());
            }
        }

        StructMethod::new(
            access_flags,
            attributes,
            values[0].clone(),
            values[1].clone(),
            bytecode_version,
            if own { code } else { None },
            cl_qualified_name.to_string(),
            signature,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        access_flags: i32,
        attributes: AttributeMap,
        name: String,
        descriptor: String,
        bytecode_version: BytecodeVersion,
        code: Option<crate::struct_code_attribute::StructCodeAttribute>,
        class_qualified_name: String,
        signature: Option<GenericMethodDescriptor>,
    ) -> StructMethod {
        let (local_variables, code_and_exceptions) = match code {
            Some(code) => (code.local_variables(), code.code_and_exception_data()),
            None => (-1, None),
        };
        StructMethod(StructMember::new_member(
            access_flags,
            attributes,
            StructMemberKind::Method(StructMethodData {
                name,
                descriptor,
                bytecode_version,
                local_variables,
                code_and_exceptions,
                seq: None,
                expanded: false,
                class_qualified_name,
                signature,
                renamer: None,
            }),
        ))
    }

    pub fn expand_data(self, class_struct: StructClass) {
        let (code, needed) = self.d(|d| {
            (
                d.code_and_exceptions.clone(),
                !d.expanded || d.seq.is_some_and(|s| !s.is_live()),
            )
        });
        if let Some(code) = code {
            if needed {
                let seq = self
                    .parse_bytecode(&mut ByteCursor::from_shared(code), class_struct.get_pool());
                self.d_mut(|d| {
                    d.seq = Some(seq);
                    d.expanded = true;
                });
            }
        }
    }

    pub fn release_resources(self) {
        self.d_mut(|d| {
            if d.code_and_exceptions.is_some() && d.expanded {
                d.seq = None;
                d.expanded = false;
            }
        })
    }

    fn parse_bytecode(self, in_: &mut ByteCursor, pool: ConstantPool) -> FullInstructionSequence {
        let bytecode_version = self.get_bytecode_version();
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut offset_to_index: JHashMap<i32, i32> = JHashMap::new();

        let length = in_.read_int();
        let mut i = 0;
        while i < length {
            let offset = i;

            let mut opcode = in_.read_unsigned_byte();
            let mut group = GROUP_GENERAL;

            let wide = opcode == OPC_WIDE;

            if wide {
                i += 1;
                opcode = in_.read_unsigned_byte();
            }

            let mut operands: Vec<i32> = Vec::new();

            if (OPC_ICONST_M1..=OPC_ICONST_5).contains(&opcode) {
                operands.push(OPR_ICONST[(opcode - OPC_ICONST_M1) as usize]);
                opcode = OPC_BIPUSH;
            } else if (OPC_ILOAD_0..=OPC_ALOAD_3).contains(&opcode) {
                operands.push(OPR_LOADSTORE[(opcode - OPC_ILOAD_0) as usize]);
                opcode = OPCS_LOAD[((opcode - OPC_ILOAD_0) / 4) as usize];
            } else if (OPC_ISTORE_0..=OPC_ASTORE_3).contains(&opcode) {
                operands.push(OPR_LOADSTORE[(opcode - OPC_ISTORE_0) as usize]);
                opcode = OPCS_STORE[((opcode - OPC_ISTORE_0) / 4) as usize];
            } else {
                match opcode {
                    OPC_BIPUSH => {
                        operands.push(in_.read_byte() as i32);
                        i += 1;
                    }
                    OPC_LDC | OPC_NEWARRAY => {
                        operands.push(in_.read_unsigned_byte());
                        i += 1;
                    }
                    OPC_SIPUSH | OPC_IFEQ | OPC_IFNE | OPC_IFLT | OPC_IFGE | OPC_IFGT
                    | OPC_IFLE | OPC_IF_ICMPEQ | OPC_IF_ICMPNE | OPC_IF_ICMPLT | OPC_IF_ICMPGE
                    | OPC_IF_ICMPGT | OPC_IF_ICMPLE | OPC_IF_ACMPEQ | OPC_IF_ACMPNE | OPC_GOTO
                    | OPC_JSR | OPC_IFNULL | OPC_IFNONNULL => {
                        if opcode != OPC_SIPUSH {
                            group = GROUP_JUMP;
                        }
                        operands.push(in_.read_short() as i32);
                        i += 2;
                    }
                    OPC_LDC_W | OPC_LDC2_W | OPC_GETSTATIC | OPC_PUTSTATIC | OPC_GETFIELD
                    | OPC_PUTFIELD | OPC_INVOKEVIRTUAL | OPC_INVOKESPECIAL | OPC_INVOKESTATIC
                    | OPC_NEW | OPC_ANEWARRAY | OPC_CHECKCAST | OPC_INSTANCEOF => {
                        operands.push(in_.read_unsigned_short());
                        i += 2;
                        if (OPC_GETSTATIC..=OPC_PUTFIELD).contains(&opcode) {
                            group = GROUP_FIELDACCESS;
                        } else if (OPC_INVOKEVIRTUAL..=OPC_INVOKESTATIC).contains(&opcode) {
                            group = GROUP_INVOCATION;
                        }
                    }
                    OPC_INVOKEDYNAMIC => {
                        if bytecode_version.has_invoke_dynamic() {
                            operands.push(in_.read_unsigned_short());
                            in_.discard(2);
                            group = GROUP_INVOCATION;
                            i += 4;
                        }
                    }
                    OPC_ILOAD | OPC_LLOAD | OPC_FLOAD | OPC_DLOAD | OPC_ALOAD | OPC_ISTORE
                    | OPC_LSTORE | OPC_FSTORE | OPC_DSTORE | OPC_ASTORE | OPC_RET => {
                        if wide {
                            operands.push(in_.read_unsigned_short());
                            i += 2;
                        } else {
                            operands.push(in_.read_unsigned_byte());
                            i += 1;
                        }
                        if opcode == OPC_RET {
                            group = GROUP_RETURN;
                        }
                    }
                    OPC_IINC => {
                        if wide {
                            operands.push(in_.read_unsigned_short());
                            operands.push(in_.read_short() as i32);
                            i += 4;
                        } else {
                            operands.push(in_.read_unsigned_byte());
                            operands.push(in_.read_byte() as i32);
                            i += 2;
                        }
                    }
                    OPC_GOTO_W | OPC_JSR_W => {
                        opcode = if opcode == OPC_JSR_W {
                            OPC_JSR
                        } else {
                            OPC_GOTO
                        };
                        operands.push(in_.read_int());
                        group = GROUP_JUMP;
                        i += 4;
                    }
                    OPC_INVOKEINTERFACE => {
                        operands.push(in_.read_unsigned_short());
                        operands.push(in_.read_unsigned_byte());
                        in_.discard(1);
                        group = GROUP_INVOCATION;
                        i += 4;
                    }
                    OPC_MULTIANEWARRAY => {
                        operands.push(in_.read_unsigned_short());
                        operands.push(in_.read_unsigned_byte());
                        i += 3;
                    }
                    OPC_TABLESWITCH => {
                        let padding = (4 - (i + 1) % 4) % 4;
                        in_.discard(padding as usize);
                        i += padding;
                        operands.push(in_.read_int());
                        i += 4;
                        let low = in_.read_int();
                        operands.push(low);
                        i += 4;
                        let high = in_.read_int();
                        operands.push(high);
                        i += 4;

                        let count = high.wrapping_sub(low).wrapping_add(1);
                        let mut j = 0;
                        while j < count {
                            operands.push(in_.read_int());
                            i += 4;
                            j += 1;
                        }
                        group = GROUP_SWITCH;
                    }
                    OPC_LOOKUPSWITCH => {
                        let padding = (4 - (i + 1) % 4) % 4;
                        in_.discard(padding as usize);
                        i += padding;
                        operands.push(in_.read_int());
                        i += 4;
                        let npairs = in_.read_int();
                        operands.push(npairs);
                        i += 4;

                        let mut j = 0;
                        while j < npairs {
                            operands.push(in_.read_int());
                            i += 4;
                            operands.push(in_.read_int());
                            i += 4;
                            j += 1;
                        }
                        group = GROUP_SWITCH;
                    }
                    OPC_IRETURN | OPC_LRETURN | OPC_FRETURN | OPC_DRETURN | OPC_ARETURN
                    | OPC_RETURN | OPC_ATHROW => {
                        group = GROUP_RETURN;
                    }
                    _ => {}
                }
            }

            let ops = if operands.is_empty() {
                None
            } else {
                Some(operands)
            };

            i += 1;

            let instr = Instruction::create(
                opcode,
                wide,
                group,
                bytecode_version,
                ops,
                offset,
                i - offset,
            );

            offset_to_index.put(offset, instructions.len() as i32);
            instructions.push(instr);
        }

        let mut lst_handlers: Vec<ExceptionHandler> = Vec::new();

        let exception_count = in_.read_unsigned_short();
        for _ in 0..exception_count {
            let from = *offset_to_index
                .get(&in_.read_unsigned_short())
                .expect("handler start");
            let to = offset_to_index
                .get_or_default(&in_.read_unsigned_short(), instructions.len() as i32);
            let handler = *offset_to_index
                .get(&in_.read_unsigned_short())
                .expect("handler target");

            let exc_class = in_.read_unsigned_short();
            let exception_class = if exc_class == 0 {
                None
            } else {
                Some(
                    pool.get_primitive_constant(exc_class)
                        .expect("catch type")
                        .get_string()
                        .to_string(),
                )
            };

            lst_handlers.push(ExceptionHandler::new(from, to, handler, exception_class));
        }

        let seq = FullInstructionSequence::new(
            instructions,
            offset_to_index,
            ExceptionTable::new(lst_handlers),
        );

        for instr in seq.iter() {
            instr.init_instruction(seq);
        }

        seq
    }

    pub fn get_name(self) -> String {
        self.d(|d| d.name.clone())
    }

    /// `name.equals(getName())` without copying the name.
    pub fn name_eq(self, name: &str) -> bool {
        self.d(|d| d.name == name)
    }

    pub fn get_descriptor(self) -> String {
        self.d(|d| d.descriptor.clone())
    }

    pub fn method_descriptor(self) -> MethodDescriptor {
        MethodDescriptor::parse_descriptor_struct(self, None)
    }

    pub fn get_bytecode_version(self) -> BytecodeVersion {
        self.d(|d| d.bytecode_version)
    }

    pub fn contains_code(self) -> bool {
        self.d(|d| d.code_and_exceptions.is_some())
    }

    pub fn get_local_variables(self) -> i32 {
        self.d(|d| d.local_variables)
    }

    pub fn get_instruction_sequence(self) -> Option<FullInstructionSequence> {
        self.d(|d| d.seq.filter(|s| s.is_live()))
    }

    pub fn get_version(self) -> BytecodeVersion {
        self.get_bytecode_version()
    }

    pub fn get_variable_namer(self) -> Rc<dyn IVariableNameProvider> {
        if let Some(renamer) = self.d(|d| d.renamer.clone()) {
            return renamer;
        }
        let renamer = DecompilerContext::get_naming_factory().create_factory(self);
        self.d_mut(|d| d.renamer = Some(renamer.clone()));
        renamer
    }

    pub fn clear_variable_namer(self) {
        self.d_mut(|d| d.renamer = None)
    }

    pub fn get_local_variable_attr(self) -> Option<StructLocalVariableTableAttribute> {
        self.get_attribute(StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TABLE)
    }

    pub fn get_class_qualified_name(self) -> String {
        self.d(|d| d.class_qualified_name.clone())
    }

    pub fn get_signature(self) -> Option<GenericMethodDescriptor> {
        self.d(|d| d.signature.clone())
    }
}

impl std::fmt::Display for StructMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, descriptor) = self.d(|d| (d.name.clone(), d.descriptor.clone()));
        write!(f, "{} {}", name, descriptor)
    }
}
