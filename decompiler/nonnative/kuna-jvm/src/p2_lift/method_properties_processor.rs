//! Port of `org/jetbrains/java/decompiler/modules/code/MethodPropertiesProcessor.java`.

use crate::code_constants::{
    ACC_STATIC, ACC_SYNTHETIC, OPC_ALOAD, OPC_BIPUSH, OPC_GOTO, OPC_IFNE, OPC_ILOAD,
    OPC_INVOKEVIRTUAL, OPC_IRETURN, OPC_LDC, OPC_LDC_W, OPC_POP, OPC_POP2, OPC_PUTSTATIC,
    OPC_RETURN,
};
use crate::instruction::Instruction;
use crate::method_properties::MethodProperties;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

pub struct MethodPropertiesProcessor;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum LambdaInstrProgress {
    Loads,
    Invoke,
    Popreturn,
}

impl MethodPropertiesProcessor {
    pub fn parse(mt: StructMethod, cl: StructClass, props: MethodProperties) {
        if !mt.contains_code() {
            return;
        }

        Self::find_synthetic_reference_lambda(mt, cl, props);
        Self::find_assert_field(mt, cl, props);
    }

    fn find_assert_field(mt: StructMethod, cl: StructClass, props: MethodProperties) {
        if mt.get_name() == "<clinit>" {
            if let Err(e) = try_catch(|| mt.expand_data(cl)) {
                eprintln!("{}", e.message);
                return;
            }

            let pool = cl.get_pool();

            let name = cl.with(|d| d.qualified_name.clone());

            let seq = mt.get_instruction_sequence().expect("NullPointerException");
            let size = seq.length();
            for i in 0..size {
                let instr = seq.get_instr(i);
                let opcode = |k: i32| seq.get_instr(k).with(|d| d.opcode);
                if instr.with(|d| d.opcode) == OPC_INVOKEVIRTUAL && i > 0 && i + 5 < size {
                    let link = pool.get_link_constant(instr.operand(0));
                    if link.classname == "java/lang/Class"
                        && link.elementname == "desiredAssertionStatus"
                    {
                        let last = seq.get_instr(i - 1);
                        let last_opcode = last.with(|d| d.opcode);
                        if (last_opcode == OPC_LDC || last_opcode == OPC_LDC_W)
                            && opcode(i + 1) == OPC_IFNE
                            && opcode(i + 2) == OPC_BIPUSH
                            && seq.get_instr(i + 2).operand(0) == 1
                            && opcode(i + 3) == OPC_GOTO
                            && opcode(i + 4) == OPC_BIPUSH
                            && seq.get_instr(i + 4).operand(0) == 0
                        {
                            let field = seq.get_instr(i + 5);
                            if field.with(|d| d.opcode) == OPC_PUTSTATIC {
                                let putstatic = pool.get_link_constant(field.operand(0));
                                if name == putstatic.classname && putstatic.descriptor == "Z" {
                                    let assert_field =
                                        cl.get_field(&putstatic.elementname, &putstatic.descriptor);
                                    props.with_mut(|d| d.assert_field = assert_field);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_synthetic_reference_lambda(mt: StructMethod, cl: StructClass, props: MethodProperties) {
        if mt.has_modifier(ACC_STATIC) && mt.has_modifier(ACC_SYNTHETIC) {
            if let Err(e) = try_catch(|| mt.expand_data(cl)) {
                eprintln!("{}", e.message);
                return;
            }

            let seq = mt.get_instruction_sequence().expect("NullPointerException");
            let _md = mt.method_descriptor();

            let mut progress = LambdaInstrProgress::Loads;

            for i in 0..seq.length() {
                let instr = seq.get_instr(i);

                if let Some(next) = Self::choose_next_progress(progress, instr) {
                    progress = next;
                }

                if !Self::is_instr_acceptable(progress, instr) {
                    return;
                }
            }

            props.with_mut(|d| d.is_synthetic_reference_lambda = true);
        }
    }

    fn is_instr_acceptable(progress: LambdaInstrProgress, instr: Instruction) -> bool {
        let opcode = instr.with(|d| d.opcode);
        match progress {
            LambdaInstrProgress::Loads => (OPC_ILOAD..=OPC_ALOAD).contains(&opcode),
            LambdaInstrProgress::Invoke => opcode == OPC_INVOKEVIRTUAL,
            LambdaInstrProgress::Popreturn => {
                opcode == OPC_POP
                    || opcode == OPC_POP2
                    || (OPC_IRETURN..=OPC_RETURN).contains(&opcode)
            }
        }
    }

    fn choose_next_progress(
        progress: LambdaInstrProgress,
        instr: Instruction,
    ) -> Option<LambdaInstrProgress> {
        if progress == LambdaInstrProgress::Loads && instr.with(|d| d.opcode) == OPC_INVOKEVIRTUAL {
            return Some(LambdaInstrProgress::Invoke);
        }

        if progress == LambdaInstrProgress::Invoke {
            return Some(LambdaInstrProgress::Popreturn);
        }

        None
    }
}
