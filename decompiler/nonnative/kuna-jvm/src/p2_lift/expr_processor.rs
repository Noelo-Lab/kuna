//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ExprProcessor.java`.

use crate::array_exprent::ArrayExprent;
use crate::assert_processor::AssertProcessor;
use crate::assignment_exprent::AssignmentExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::cancelation_manager::CancelationManager;
use crate::code_constants::*;
use crate::code_type::CodeType;
use crate::condy_helper::CondyHelper;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::decompiler_context::DecompilerContext;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::generic_type::GenericType;
use crate::if_exprent::{IfExprent, Type as IfType};
use crate::invocation_exprent::InvocationExprent;
use crate::jthrow;
use crate::list_stack::ListStack;
use crate::method_descriptor::MethodDescriptor;
use crate::monitor_exprent::{MonitorExprent, Type as MonitorType};
use crate::new_exprent::NewExprent;
use crate::plugin_context::PluginContext;
use crate::pooled_constant::PooledConstant;
use crate::primitive_exprs_list::PrimitiveExprsList;
use crate::root_statement::RootStatement;
use crate::rt::{BitSet, JHashMap, JList, JavaEq, JavaHash, JavaListExt};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;

thread_local! {
    static MAP_CONSTS: JHashMap<i32, FunctionType> = {
        let mut map = JHashMap::new();
        map.put(OPC_ARRAYLENGTH, FunctionType::ArrayLength);
        map.put(OPC_CHECKCAST, FunctionType::Cast);
        map.put(OPC_INSTANCEOF, FunctionType::Instanceof);
        map
    };

    static CONSTS: [VarType; 14] = [
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_FLOAT(),
        VarType::VARTYPE_LONG(),
        VarType::VARTYPE_DOUBLE(),
        VarType::VARTYPE_CLASS(),
        VarType::VARTYPE_STRING(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_METHODTYPE(),
    ];

    static VAR_TYPES: [VarType; 5] = [
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_LONG(),
        VarType::VARTYPE_FLOAT(),
        VarType::VARTYPE_DOUBLE(),
        VarType::VARTYPE_OBJECT(),
    ];

    static ARR_TYPES: [VarType; 8] = [
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_LONG(),
        VarType::VARTYPE_FLOAT(),
        VarType::VARTYPE_DOUBLE(),
        VarType::VARTYPE_OBJECT(),
        VarType::VARTYPE_BOOLEAN(),
        VarType::VARTYPE_CHAR(),
        VarType::VARTYPE_SHORT(),
    ];
}

const FUNC1: [FunctionType; 5] = [
    FunctionType::Add,
    FunctionType::Sub,
    FunctionType::Mul,
    FunctionType::Div,
    FunctionType::Rem,
];

const FUNC2: [FunctionType; 6] = [
    FunctionType::Shl,
    FunctionType::Shr,
    FunctionType::Ushr,
    FunctionType::And,
    FunctionType::Or,
    FunctionType::Xor,
];

const FUNC3: [FunctionType; 15] = [
    FunctionType::I2l,
    FunctionType::I2f,
    FunctionType::I2d,
    FunctionType::L2i,
    FunctionType::L2f,
    FunctionType::L2d,
    FunctionType::F2i,
    FunctionType::F2l,
    FunctionType::F2d,
    FunctionType::D2i,
    FunctionType::D2l,
    FunctionType::D2f,
    FunctionType::I2b,
    FunctionType::I2c,
    FunctionType::I2s,
];

const FUNC4: [FunctionType; 5] = [
    FunctionType::Lcmp,
    FunctionType::Fcmpl,
    FunctionType::Fcmpg,
    FunctionType::Dcmpl,
    FunctionType::Dcmpg,
];

const FUNC5: [IfType; 6] = [
    IfType::Eq,
    IfType::Ne,
    IfType::Lt,
    IfType::Ge,
    IfType::Gt,
    IfType::Le,
];

const FUNC6: [IfType; 8] = [
    IfType::Icmpeq,
    IfType::Icmpne,
    IfType::Icmplt,
    IfType::Icmpge,
    IfType::Icmpgt,
    IfType::Icmple,
    IfType::Acmpeq,
    IfType::Acmpne,
];

const FUNC7: [IfType; 2] = [IfType::Null, IfType::Nonnull];

const FUNC8: [MonitorType; 2] = [MonitorType::Enter, MonitorType::Exit];

const ARR_TYPE_IDS: [CodeType; 8] = [
    CodeType::Boolean,
    CodeType::Char,
    CodeType::Float,
    CodeType::Double,
    CodeType::Byte,
    CodeType::Short,
    CodeType::Int,
    CodeType::Long,
];

const TYPE_NAMES: [&str; 8] = [
    "byte", "char", "double", "float", "int", "long", "short", "boolean",
];

/// The stack-to-expression lift of one method body.
///
/// A plain owned value: built per method by `MethodProcessor` and never stored.
pub struct ExprProcessor {
    method_descriptor: MethodDescriptor,
    var_processor: VarProcessor,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum NullCastType {
    Cast,
    DontCast,
    DontCastAtAll,
}

impl NullCastType {
    pub fn cast(self) -> bool {
        match self {
            NullCastType::Cast => true,
            NullCastType::DontCast => false,
            NullCastType::DontCastAtAll => false,
        }
    }
}

impl JavaHash for NullCastType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for NullCastType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl ExprProcessor {
    pub const UNDEFINED_TYPE_STRING: &'static str = "<undefinedtype>";
    pub const UNREPRESENTABLE_TYPE_STRING: &'static str = "<unrepresentable>";
    pub const UNKNOWN_TYPE_STRING: &'static str = "<unknown>";
    pub const NULL_TYPE_STRING: &'static str = "<null>";

    pub fn new(md: &MethodDescriptor, var_proc: VarProcessor) -> ExprProcessor {
        ExprProcessor {
            method_descriptor: md.clone(),
            var_processor: var_proc,
        }
    }

    pub fn process_statement(&self, root: RootStatement, cl: StructClass) {
        let mut flat_helper = FlattenStatementsHelper::new();
        let dgraph = flat_helper.build_direct_graph(root);

        ValidationHelper::validate_d_graph(dgraph, root);

        let mut map_catch: JHashMap<String, VarExprent> = JHashMap::new();
        Self::collect_catch_vars(root.0, &flat_helper, &mut map_catch);

        let mut map_data: JHashMap<DirectNode, PrimitiveExprsList> = JHashMap::new();

        let mut stack: ListStack<DirectNode> = ListStack::new();

        let first = dgraph.first().expect("NullPointerException: dgraph.first");
        stack.push(first);
        map_data.put(first, PrimitiveExprsList::new());

        while !stack.is_empty() {
            CancelationManager::check_canceled();
            let node = stack.pop();

            let mut catch_data = map_catch
                .get_cloned(&node.id())
                .map(Self::get_expression_data);

            if let Some(block) = node.block() {
                let data = match catch_data.as_mut() {
                    Some(data) => data,
                    None => map_data
                        .get_mut(&node)
                        .expect("NullPointerException: mapData.get(node)"),
                };
                self.process_block(block, data, cl);
                block.set_exprents(data.get_lst_exprents());
            }

            let successors = node.get_successors(DirectEdgeType::Regular).to_vec();
            for cd in successors {
                let nd = cd.get_destination();
                if !map_data.contains_key(&nd) {
                    let copy = match &catch_data {
                        Some(data) => data.copy_stack(),
                        None => map_data
                            .get(&node)
                            .expect("NullPointerException: mapData.get(node)")
                            .copy_stack(),
                    };
                    map_data.put(nd, Self::copy_var_exprents(copy));
                    stack.push(nd);
                }
            }
        }

        Self::init_statement_exprents(root.0);
    }

    fn copy_var_exprents(mut data: PrimitiveExprsList) -> PrimitiveExprsList {
        let stack = data.get_stack();
        Self::copy_entries(stack);
        data
    }

    pub fn copy_entries(stack: &mut [Exprent]) {
        for entry in stack.iter_mut() {
            *entry = entry.copy();
        }
    }

    fn collect_catch_vars(
        stat: Statement,
        flatthelper: &FlattenStatementsHelper,
        map: &mut JHashMap<String, VarExprent>,
    ) {
        let mut lst: Option<JList<VarExprent>> = None;

        if let Some(catchall) = stat.try_as_catch_all() {
            if !catchall.is_finally() {
                lst = Some(catchall.get_vars());
            }
        } else if let Some(catch_stat) = stat.try_as_catch() {
            lst = Some(catch_stat.get_vars());
        }

        if let Some(lst) = lst {
            let mut i = 1;
            while i < stat.get_stats().borrow().size() {
                let st = stat.get_stats().borrow().get(i);
                let node = flatthelper
                    .get_direct_node(st)
                    .expect("NullPointerException: flatthelper.getDirectNode");
                map.put(node.id(), lst.get(i - 1));
                i += 1;
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::collect_catch_vars(st, flatthelper, map);
        }
    }

    fn init_statement_exprents(stat: Statement) {
        stat.init_exprents();

        let sub_stats = stat.get_stats().borrow().to_vec();

        for st in sub_stats {
            Self::init_statement_exprents(st);
        }
    }

    pub fn process_block(
        &self,
        stat: BasicBlockStatement,
        data: &mut PrimitiveExprsList,
        cl: StructClass,
    ) {
        let pool = cl.get_pool();
        let bootstrap = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_BOOTSTRAP_METHODS);

        let block = stat.get_block();

        let exprlist = data.get_lst_exprents();
        let stack = data.get_stack();

        let seq = block.get_seq();

        let mut i = 0;
        while i < seq.length() {
            let instr = seq.get_instr(i);
            let opcode = instr.opcode();
            let bytecode_offset = block.get_old_offset(i);
            let mut bytecode_offsets: Option<BitSet> = None;
            if bytecode_offset >= 0 {
                let mut offsets = BitSet::new();
                offsets.set_range(
                    bytecode_offset as usize,
                    (bytecode_offset + instr.length()) as usize,
                );
                bytecode_offsets = Some(offsets);
            }

            match opcode {
                OPC_ACONST_NULL => {
                    let e =
                        ConstExprent::new(VarType::VARTYPE_NULL(), None, bytecode_offsets.as_ref());
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_BIPUSH | OPC_SIPUSH => {
                    let e =
                        ConstExprent::new_int(instr.operand(0), true, bytecode_offsets.as_ref());
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_LCONST_0 | OPC_LCONST_1 => {
                    let e = ConstExprent::new(
                        VarType::VARTYPE_LONG(),
                        Some(ConstValue::Long((opcode - OPC_LCONST_0) as i64)),
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_FCONST_0 | OPC_FCONST_1 | OPC_FCONST_2 => {
                    let e = ConstExprent::new(
                        VarType::VARTYPE_FLOAT(),
                        Some(ConstValue::Float((opcode - OPC_FCONST_0) as f32)),
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_DCONST_0 | OPC_DCONST_1 => {
                    let e = ConstExprent::new(
                        VarType::VARTYPE_DOUBLE(),
                        Some(ConstValue::Double((opcode - OPC_DCONST_0) as f64)),
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_LDC | OPC_LDC_W | OPC_LDC2_W => {
                    let cn = pool.get_constant(instr.operand(0));
                    match cn {
                        Some(PooledConstant::Primitive(cn)) => {
                            let const_type =
                                CONSTS.with(|c| c[(cn.type_ - CONSTANT_INTEGER) as usize].clone());
                            let e = ConstExprent::new(
                                const_type,
                                cn.value.as_ref().map(ConstValue::from),
                                bytecode_offsets.as_ref(),
                            );
                            self.push_ex(stack, &exprlist, e.0);
                        }
                        Some(PooledConstant::Link(invoke_constant))
                            if invoke_constant.type_ == CONSTANT_DYNAMIC =>
                        {
                            let mut bootstrap_method = None;
                            let mut bootstrap_arguments = None;
                            if let Some(bootstrap) = bootstrap {
                                bootstrap_method =
                                    bootstrap.get_method_reference(invoke_constant.index1);
                                bootstrap_arguments =
                                    Some(bootstrap.get_method_arguments(invoke_constant.index1));
                            }

                            let exprinv = InvocationExprent::new_invoke(
                                opcode,
                                &invoke_constant,
                                bootstrap_method,
                                bootstrap_arguments,
                                stack,
                                bytecode_offsets.as_ref(),
                            );
                            if exprinv.get_descriptor().ret.type_ == CodeType::Void {
                                exprlist.add(exprinv.0);
                            } else {
                                let e = CondyHelper::simplify_condy(exprinv);
                                self.push_ex(stack, &exprlist, e);
                            }
                        }
                        Some(PooledConstant::Link(link)) => {
                            let e = ConstExprent::new(
                                VarType::VARTYPE_STRING(),
                                Some(ConstValue::String(link.elementname.clone())),
                                bytecode_offsets.as_ref(),
                            );
                            self.push_ex(stack, &exprlist, e.0);
                        }
                        None => {}
                    }
                }
                OPC_ILOAD | OPC_LLOAD | OPC_FLOAD | OPC_DLOAD | OPC_ALOAD => {
                    let var_exprent = VarExprent::new_bytecode(
                        instr.operand(0),
                        Some(VAR_TYPES.with(|t| t[(opcode - OPC_ILOAD) as usize].clone())),
                        Some(self.var_processor),
                        bytecode_offsets.as_ref(),
                    );
                    var_exprent.set_backing_instr(Some(instr));
                    self.var_processor
                        .find_lvt(var_exprent, bytecode_offset + instr.length());
                    self.push_ex(stack, &exprlist, var_exprent.0);
                }
                OPC_IALOAD | OPC_LALOAD | OPC_FALOAD | OPC_DALOAD | OPC_AALOAD | OPC_BALOAD
                | OPC_CALOAD | OPC_SALOAD => {
                    let index = stack.pop();
                    let arr = stack.pop();

                    let mut vartype: Option<VarType> = None;
                    match opcode {
                        OPC_LALOAD => vartype = Some(VarType::VARTYPE_LONG()),
                        OPC_DALOAD => vartype = Some(VarType::VARTYPE_DOUBLE()),
                        _ => {}
                    }
                    let hard_type = ARR_TYPES.with(|t| t[(opcode - OPC_IALOAD) as usize].clone());
                    let e = ArrayExprent::new(arr, index, hard_type, bytecode_offsets.as_ref());
                    self.push_ex_type(stack, &exprlist, e.0, vartype.as_ref());
                }
                OPC_ISTORE | OPC_LSTORE | OPC_FSTORE | OPC_DSTORE | OPC_ASTORE => {
                    let expr = stack.pop();
                    let varindex = instr.operand(0);
                    if let Some(offsets) = bytecode_offsets.as_mut() {
                        offsets.set_range(
                            bytecode_offset as usize,
                            (bytecode_offset + instr.length()) as usize,
                        );
                    }
                    let var_exprent = VarExprent::new_bytecode(
                        varindex,
                        Some(VAR_TYPES.with(|t| t[(opcode - OPC_ISTORE) as usize].clone())),
                        Some(self.var_processor),
                        bytecode_offsets.as_ref(),
                    );
                    var_exprent.set_backing_instr(Some(instr));
                    self.var_processor
                        .find_lvt(var_exprent, bytecode_offset + instr.length());
                    let assign =
                        AssignmentExprent::new(var_exprent.0, expr, bytecode_offsets.as_ref());
                    exprlist.add(assign.0);
                }
                OPC_IASTORE | OPC_LASTORE | OPC_FASTORE | OPC_DASTORE | OPC_AASTORE
                | OPC_BASTORE | OPC_CASTORE | OPC_SASTORE => {
                    let value = stack.pop();
                    let index_store = stack.pop();
                    let arr_store = stack.pop();
                    let hard_type = ARR_TYPES.with(|t| t[(opcode - OPC_IASTORE) as usize].clone());
                    let arr = ArrayExprent::new(
                        arr_store,
                        index_store,
                        hard_type,
                        bytecode_offsets.as_ref(),
                    );
                    let arrassign = AssignmentExprent::new(arr.0, value, bytecode_offsets.as_ref());
                    exprlist.add(arrassign.0);
                }
                OPC_IADD | OPC_LADD | OPC_FADD | OPC_DADD | OPC_ISUB | OPC_LSUB | OPC_FSUB
                | OPC_DSUB | OPC_IMUL | OPC_LMUL | OPC_FMUL | OPC_DMUL | OPC_IDIV | OPC_LDIV
                | OPC_FDIV | OPC_DDIV | OPC_IREM | OPC_LREM | OPC_FREM | OPC_DREM => {
                    let e = FunctionExprent::new_stack(
                        FUNC1[((opcode - OPC_IADD) / 4) as usize],
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_ISHL | OPC_LSHL | OPC_ISHR | OPC_LSHR | OPC_IUSHR | OPC_LUSHR | OPC_IAND
                | OPC_LAND | OPC_IOR | OPC_LOR | OPC_IXOR | OPC_LXOR => {
                    let e = FunctionExprent::new_stack(
                        FUNC2[((opcode - OPC_ISHL) / 2) as usize],
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_INEG | OPC_LNEG | OPC_FNEG | OPC_DNEG => {
                    let e = FunctionExprent::new_stack(
                        FunctionType::Neg,
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_IINC => {
                    let vevar = VarExprent::new_bytecode(
                        instr.operand(0),
                        Some(VarType::VARTYPE_INT()),
                        Some(self.var_processor),
                        bytecode_offsets.as_ref(),
                    );
                    vevar.set_backing_instr(Some(instr));
                    self.var_processor
                        .find_lvt(vevar, bytecode_offset + instr.length());
                    let func_type = if instr.operand(1) < 0 {
                        FunctionType::Sub
                    } else {
                        FunctionType::Add
                    };
                    let operands = vec![
                        vevar.copy(),
                        ConstExprent::new(
                            VarType::VARTYPE_INT(),
                            Some(ConstValue::Int(instr.operand(1).wrapping_abs())),
                            None,
                        )
                        .0,
                    ];
                    let func = FunctionExprent::new(func_type, operands, bytecode_offsets.as_ref());
                    exprlist
                        .add(AssignmentExprent::new(vevar.0, func.0, bytecode_offsets.as_ref()).0);
                }
                OPC_I2L | OPC_I2F | OPC_I2D | OPC_L2I | OPC_L2F | OPC_L2D | OPC_F2I | OPC_F2L
                | OPC_F2D | OPC_D2I | OPC_D2L | OPC_D2F | OPC_I2B | OPC_I2C | OPC_I2S => {
                    let e = FunctionExprent::new_stack(
                        FUNC3[(opcode - OPC_I2L) as usize],
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_LCMP | OPC_FCMPL | OPC_FCMPG | OPC_DCMPL | OPC_DCMPG => {
                    let e = FunctionExprent::new_stack(
                        FUNC4[(opcode - OPC_LCMP) as usize],
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_IFEQ | OPC_IFNE | OPC_IFLT | OPC_IFGE | OPC_IFGT | OPC_IFLE => {
                    let e = IfExprent::new(
                        FUNC5[(opcode - OPC_IFEQ) as usize].get_negative(),
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    exprlist.add(e.0);
                }
                OPC_IF_ICMPEQ | OPC_IF_ICMPNE | OPC_IF_ICMPLT | OPC_IF_ICMPGE | OPC_IF_ICMPGT
                | OPC_IF_ICMPLE | OPC_IF_ACMPEQ | OPC_IF_ACMPNE => {
                    let e = IfExprent::new(
                        FUNC6[(opcode - OPC_IF_ICMPEQ) as usize].get_negative(),
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    exprlist.add(e.0);
                }
                OPC_IFNULL | OPC_IFNONNULL => {
                    let e = IfExprent::new(
                        FUNC7[(opcode - OPC_IFNULL) as usize].get_negative(),
                        stack,
                        bytecode_offsets.as_ref(),
                    );
                    exprlist.add(e.0);
                }
                OPC_TABLESWITCH | OPC_LOOKUPSWITCH => {
                    let value = stack.pop();
                    exprlist.add(SwitchHeadExprent::new(value, bytecode_offsets.as_ref()).0);
                }
                OPC_IRETURN | OPC_LRETURN | OPC_FRETURN | OPC_DRETURN | OPC_ARETURN
                | OPC_RETURN | OPC_ATHROW => {
                    let exit_type = if opcode == OPC_ATHROW {
                        ExitType::Throw
                    } else {
                        ExitType::Return
                    };
                    let value = if opcode == OPC_RETURN {
                        None
                    } else {
                        Some(stack.pop())
                    };
                    let ret_type = if opcode == OPC_ATHROW {
                        None
                    } else {
                        Some(self.method_descriptor.ret.clone())
                    };
                    let e = ExitExprent::new(
                        exit_type,
                        value,
                        ret_type,
                        bytecode_offsets.as_ref(),
                        Some(self.method_descriptor.clone()),
                    );
                    exprlist.add(e.0);
                }
                OPC_MONITORENTER | OPC_MONITOREXIT => {
                    let value = stack.pop();
                    let monitor = MonitorExprent::new(
                        FUNC8[(opcode - OPC_MONITORENTER) as usize],
                        value,
                        bytecode_offsets.as_ref(),
                    );

                    if opcode == OPC_MONITOREXIT && stat.is_removable_monitorexit() {
                        monitor.set_remove(true);
                    }

                    exprlist.add(monitor.0);
                }
                OPC_CHECKCAST | OPC_INSTANCEOF | OPC_ARRAYLENGTH => {
                    if opcode != OPC_ARRAYLENGTH {
                        let class_name = pool
                            .get_primitive_constant(instr.operand(0))
                            .expect("NullPointerException: pool.getPrimitiveConstant")
                            .get_string();
                        let type_ = VarType::new_signature_cl(&class_name, true);
                        stack.push(ConstExprent::new(type_, None, None).0);
                    }
                    let func_type = MAP_CONSTS.with(|m| {
                        *m.get(&opcode)
                            .expect("NullPointerException: mapConsts.get(opcode)")
                    });
                    let e = FunctionExprent::new_stack(func_type, stack, bytecode_offsets.as_ref());
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_GETSTATIC | OPC_GETFIELD => {
                    let cn = pool
                        .get_link_constant(instr.operand(0))
                        .expect("NullPointerException: pool.getLinkConstant");
                    let instance = if opcode == OPC_GETSTATIC {
                        None
                    } else {
                        Some(stack.pop())
                    };
                    let e = FieldExprent::new_link(&cn, instance, bytecode_offsets.as_ref());
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_PUTSTATIC | OPC_PUTFIELD => {
                    let valfield = stack.pop();
                    let cn = pool
                        .get_link_constant(instr.operand(0))
                        .expect("NullPointerException: pool.getLinkConstant");
                    let instance = if opcode == OPC_PUTSTATIC {
                        None
                    } else {
                        Some(stack.pop())
                    };
                    let exprfield =
                        FieldExprent::new_link(&cn, instance, bytecode_offsets.as_ref());
                    exprlist.add(
                        AssignmentExprent::new(exprfield.0, valfield, bytecode_offsets.as_ref()).0,
                    );
                }
                OPC_INVOKEVIRTUAL | OPC_INVOKESPECIAL | OPC_INVOKESTATIC | OPC_INVOKEINTERFACE
                | OPC_INVOKEDYNAMIC => {
                    if opcode != OPC_INVOKEDYNAMIC || instr.bytecode_version().has_invoke_dynamic()
                    {
                        let invoke_constant = pool
                            .get_link_constant(instr.operand(0))
                            .expect("NullPointerException: pool.getLinkConstant");

                        let mut bootstrap_method = None;
                        let mut bootstrap_arguments = None;
                        if opcode == OPC_INVOKEDYNAMIC {
                            if let Some(bootstrap) = bootstrap {
                                bootstrap_method =
                                    bootstrap.get_method_reference(invoke_constant.index1);
                                bootstrap_arguments =
                                    Some(bootstrap.get_method_arguments(invoke_constant.index1));
                            }
                        }

                        let exprinv = InvocationExprent::new_invoke(
                            opcode,
                            &invoke_constant,
                            bootstrap_method,
                            bootstrap_arguments,
                            stack,
                            bytecode_offsets.as_ref(),
                        );
                        if exprinv.get_descriptor().ret.type_ == CodeType::Void {
                            exprlist.add(exprinv.0);
                        } else {
                            self.push_ex(stack, &exprlist, exprinv.0);
                        }
                    }
                }
                OPC_NEW | OPC_ANEWARRAY | OPC_MULTIANEWARRAY => {
                    let dimensions = if opcode == OPC_NEW {
                        0
                    } else if opcode == OPC_ANEWARRAY {
                        1
                    } else {
                        instr.operand(1)
                    };
                    let class_name = pool
                        .get_primitive_constant(instr.operand(0))
                        .expect("NullPointerException: pool.getPrimitiveConstant")
                        .get_string();
                    let mut arr_type = VarType::new_signature_cl(&class_name, true);
                    if opcode != OPC_MULTIANEWARRAY {
                        arr_type = arr_type.resize_array_dim(arr_type.array_dim + dimensions);
                    }
                    let e = NewExprent::new_stack(
                        arr_type,
                        stack,
                        dimensions,
                        bytecode_offsets.as_ref(),
                    );
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_NEWARRAY => {
                    let new_type =
                        VarType::new_dim(ARR_TYPE_IDS[(instr.operand(0) - 4) as usize], 1);
                    let e = NewExprent::new_stack(new_type, stack, 1, bytecode_offsets.as_ref());
                    self.push_ex(stack, &exprlist, e.0);
                }
                OPC_DUP => {
                    let e = stack.get_by_offset(-1).copy();
                    self.push_ex(stack, &exprlist, e);
                }
                OPC_DUP_X1 => {
                    self.insert_by_offset_ex(-2, stack, &exprlist, -1);
                }
                OPC_DUP_X2 => {
                    if stack.get_by_offset(-2).get_expr_type().stack_size == 2 {
                        self.insert_by_offset_ex(-2, stack, &exprlist, -1);
                    } else {
                        self.insert_by_offset_ex(-3, stack, &exprlist, -1);
                    }
                }
                OPC_DUP2 => {
                    if stack.get_by_offset(-1).get_expr_type().stack_size == 2 {
                        let e = stack.get_by_offset(-1).copy();
                        self.push_ex(stack, &exprlist, e);
                    } else {
                        let e = stack.get_by_offset(-2).copy();
                        self.push_ex(stack, &exprlist, e);
                        let e = stack.get_by_offset(-2).copy();
                        self.push_ex(stack, &exprlist, e);
                    }
                }
                OPC_DUP2_X1 => {
                    if stack.get_by_offset(-1).get_expr_type().stack_size == 2 {
                        self.insert_by_offset_ex(-2, stack, &exprlist, -1);
                    } else {
                        self.insert_by_offset_ex(-3, stack, &exprlist, -2);
                        self.insert_by_offset_ex(-3, stack, &exprlist, -1);
                    }
                }
                OPC_DUP2_X2 => {
                    if stack.get_by_offset(-1).get_expr_type().stack_size == 2 {
                        if stack.get_by_offset(-2).get_expr_type().stack_size == 2 {
                            self.insert_by_offset_ex(-2, stack, &exprlist, -1);
                        } else {
                            self.insert_by_offset_ex(-3, stack, &exprlist, -1);
                        }
                    } else if stack.get_by_offset(-3).get_expr_type().stack_size == 2 {
                        self.insert_by_offset_ex(-3, stack, &exprlist, -2);
                        self.insert_by_offset_ex(-3, stack, &exprlist, -1);
                    } else {
                        self.insert_by_offset_ex(-4, stack, &exprlist, -2);
                        self.insert_by_offset_ex(-4, stack, &exprlist, -1);
                    }
                }
                OPC_SWAP => {
                    self.insert_by_offset_ex(-2, stack, &exprlist, -1);
                    stack.pop();
                }
                OPC_POP => {
                    stack.pop();
                    if !exprlist.is_empty() {
                        let last = exprlist.get(exprlist.len() - 1);
                        if let Some(assign) = last.try_as_assignment() {
                            if let Some(invocation) = assign.get_right().try_as_invocation() {
                                if i + 1 < seq.length()
                                    && ((!invocation.is_static()
                                        && invocation.get_name() == "getClass"
                                        && invocation.get_string_descriptor()
                                            == "()Ljava/lang/Class;")
                                        || (invocation.is_static()
                                            && invocation.get_classname() == "java/util/Objects"
                                            && invocation.get_name() == "requireNonNull"
                                            && invocation.get_string_descriptor()
                                                == "(Ljava/lang/Object;)Ljava/lang/Object;"))
                                {
                                    let next_opc = seq.get_instr(i + 1).opcode();
                                    if (OPC_ACONST_NULL..=OPC_LDC2_W).contains(&next_opc)
                                        || (next_opc == OPC_ASTORE
                                            || (OPC_ASTORE_0..=OPC_ASTORE_3).contains(&next_opc))
                                    {
                                        invocation.set_synthetic_null_check();
                                    }
                                }
                            }
                        }
                    }
                }
                OPC_POP2 => {
                    if stack.get_by_offset(-1).get_expr_type().stack_size == 1 {
                        stack.pop();
                    }
                    stack.pop();
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn push_ex(&self, stack: &mut ListStack<Exprent>, exprlist: &JList<Exprent>, exprent: Exprent) {
        self.push_ex_type(stack, exprlist, exprent, None);
    }

    /// Java: pushEx(ListStack, List, Exprent, VarType)
    fn push_ex_type(
        &self,
        stack: &mut ListStack<Exprent>,
        exprlist: &JList<Exprent>,
        exprent: Exprent,
        vartype: Option<&VarType>,
    ) {
        ValidationHelper::not_null(Some(&exprent));

        let varindex = VarExprent::STACK_BASE + stack.size() as i32;
        let var_type = match vartype {
            None => exprent.get_expr_type(),
            Some(vartype) => vartype.clone(),
        };
        let var = VarExprent::new(varindex, Some(var_type), Some(self.var_processor));
        var.set_stack(true);

        exprlist.add(AssignmentExprent::new(var.0, exprent, None).0);
        stack.push(var.copy());
    }

    fn insert_by_offset_ex(
        &self,
        offset: i32,
        stack: &mut ListStack<Exprent>,
        exprlist: &JList<Exprent>,
        copyoffset: i32,
    ) {
        let base = VarExprent::STACK_BASE + stack.size() as i32;

        let mut lst: Vec<VarExprent> = Vec::new();

        let mut i = -1;
        while i >= offset {
            let varex = stack.pop();
            let varnew = VarExprent::new(
                base + i + 1,
                Some(varex.get_expr_type()),
                Some(self.var_processor),
            );
            varnew.set_stack(true);
            exprlist.add(AssignmentExprent::new(varnew.0, varex, None).0);
            lst.insert(0, varnew.copy().as_var());
            i -= 1;
        }

        let exprent = lst[(lst.len() as i32 + copyoffset) as usize].copy();
        let var = VarExprent::new(
            base + offset,
            Some(exprent.get_expr_type()),
            Some(self.var_processor),
        );
        var.set_stack(true);
        exprlist.add(AssignmentExprent::new(var.0, exprent, None).0);
        lst.insert(0, var.copy().as_var());

        for expr in lst {
            stack.push(expr.0);
        }
    }

    pub fn canonicalize_casts(stat: RootStatement) -> bool {
        let mut res = false;
        while Self::canonicalize_casts_stat(stat.0) {
            res = true;
        }

        res
    }

    /// Java: canonicalizeCasts(Statement)
    fn canonicalize_casts_stat(stat: Statement) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::canonicalize_casts_stat(st);
        }

        if stat.is_basic_block() {
            let exprents = stat
                .get_exprents()
                .expect("NullPointerException: stat.getExprents()")
                .to_vec();
            for exprent in exprents {
                for ex in exprent.get_all_exprents_rec_self(true, true) {
                    let Some(func) = ex.try_as_function() else {
                        continue;
                    };
                    if func.get_func_type() != FunctionType::Cast {
                        continue;
                    }
                    let inner = func.get_lst_operands().get(0);
                    let cast = func.get_lst_operands().get(1);

                    let Some(func2) = inner.try_as_function() else {
                        continue;
                    };
                    if func2.get_func_type() != FunctionType::Cast {
                        continue;
                    }
                    let inner2 = func2.get_lst_operands().get(0);
                    let cast2 = func2.get_lst_operands().get(1);

                    if cast.get_expr_type().equals(&cast2.get_expr_type()) {
                        ex.replace_exprent(inner, inner2);
                        ex.add_bytecode_offsets(inner2.get_bytecode().as_ref());
                        ex.add_bytecode_offsets(inner.get_bytecode().as_ref());
                        res = true;
                    }
                }
            }
        }

        res
    }

    pub fn mark_expr_oddities(root: RootStatement) {
        let _lock = DecompilerContext::get_import_collector().lock();
        Self::mark_expr_oddities_stat(root, root.0);
    }

    /// Java: markExprOddities(RootStatement, Statement)
    fn mark_expr_oddities_stat(root: RootStatement, stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::mark_expr_oddities_stat(root, st);
        }

        for ex in stat.get_var_definitions().to_vec() {
            Self::mark_expr_oddity(root, ex, true);
        }

        if stat.is_basic_block() {
            if stat.is_labeled() {
                root.add_comment_error("$VF: Made invalid labels!", true);
            }

            let exprents = stat
                .get_exprents()
                .expect("NullPointerException: stat.getExprents()")
                .to_vec();
            for ex in exprents {
                Self::mark_expr_oddity(root, ex, true);
            }
        }

        if let Some(if_st) = stat.try_as_if() {
            Self::mark_expr_oddity(root, if_st.get_headexprent().0, false);
        }
    }

    fn mark_expr_oddity(root: RootStatement, ex: Exprent, block: bool) {
        if ex.is_monitor() {
            root.add_comment_error(
                "$VF: Could not create synchronized statement, marking monitor enters and exits",
                true,
            );
        }
        if block && ex.is_if() {
            root.add_comment_error(
                "$VF: Accidentally destroyed if statement, the decompiled code is not correct!",
                true,
            );
        }

        let wrapper = AssertProcessor::get_holding_class();
        let prop = wrapper.get_method_properties("<clinit>", "()V");

        for e in ex.get_all_exprents_rec_self(true, true) {
            if let Some(var) = e.try_as_var() {
                if var.is_definition() && Self::is_invalid_type_name(&var.get_definition_type())
                    || var.get_expr_type().ptr_eq(&VarType::VARTYPE_UNKNOWN())
                {
                    root.add_comment_error(
                        "$VF: Could not properly define all variable types!",
                        true,
                    );
                }
            } else if let Some(func) = e.try_as_function() {
                if func.get_func_type() == FunctionType::Cast && func.does_cast() {
                    let operands = func.get_lst_operands();
                    if Self::is_invalid_type_name(&operands.get(1).to_string()) {
                        root.add_comment_error(
                            "$VF: Could not properly define all variable types!",
                            true,
                        );
                    }
                }
            } else if let Some(field) = e.try_as_field() {
                if let Some(prop) = &prop {
                    if let Some(assert_field) = prop.assert_field {
                        if root.mt() != prop.mt
                            && field.get_name() == assert_field.get_name()
                            && field.get_classname() == wrapper.get_class_struct().qualified_name()
                        {
                            root.add_comment_error(
                                "$VF: Could not resugar all assert statements!",
                                true,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn get_type_name(type_: &VarType) -> String {
        Self::get_type_name_short(type_, true)
    }

    /// Java: getTypeName(VarType, boolean)
    pub fn get_type_name_short(type_: &VarType, get_short: bool) -> String {
        let tp = type_.type_;
        if tp.ordinal() <= CodeType::Boolean.ordinal() {
            return TYPE_NAMES[tp.ordinal() as usize].to_string();
        } else if tp == CodeType::Unknown {
            return Self::UNKNOWN_TYPE_STRING.to_string();
        } else if tp == CodeType::Null {
            return Self::NULL_TYPE_STRING.to_string();
        } else if tp == CodeType::Void {
            return "void".to_string();
        } else if tp == CodeType::Genvar && type_.is_generic() {
            return type_.value_or_null().to_string();
        } else if tp == CodeType::Object {
            if type_.is_generic() {
                return type_.as_generic().get_cast_name();
            }
            let name = Self::build_java_class_name(type_.value_str());
            let ret = if get_short {
                DecompilerContext::get_import_collector().get_short_name(&name)
            } else {
                Some(name)
            };

            return ret.unwrap_or_else(|| Self::UNDEFINED_TYPE_STRING.to_string());
        }

        if tp == CodeType::Bytechar {
            return "<bytechar>".to_string();
        }

        if tp == CodeType::Shortchar {
            return "<shortchar>".to_string();
        }

        jthrow!("invalid type: {}", tp.name())
    }

    pub fn is_invalid_type_name(name: &str) -> bool {
        Self::UNDEFINED_TYPE_STRING == name
            || Self::NULL_TYPE_STRING == name
            || Self::UNKNOWN_TYPE_STRING == name
    }

    pub fn get_cast_type_name(type_: &VarType) -> String {
        Self::get_cast_type_name_short(type_, true)
    }

    /// Java: getCastTypeName(VarType, boolean)
    pub fn get_cast_type_name_short(type_: &VarType, get_short: bool) -> String {
        let mut s = Self::get_type_name_short(type_, get_short);
        TextUtil::append(&mut s, "[]", type_.array_dim);
        s
    }

    pub fn get_expression_data(var: VarExprent) -> PrimitiveExprsList {
        let mut prlst = PrimitiveExprsList::new();
        let vartmp = VarExprent::new(
            VarExprent::STACK_BASE,
            Some(var.get_expr_type()),
            var.get_processor(),
        );
        vartmp.set_stack(true);

        let catch_var = var.copy().as_var();
        catch_var.set_catch_temp_var(true);
        prlst
            .get_lst_exprents()
            .add(AssignmentExprent::new(vartmp.0, catch_var.0, None).0);
        prlst.get_stack().push(vartmp.copy());
        prlst
    }

    pub fn ends_with_semicolon(expr: Exprent) -> bool {
        !(expr.is_switch_head()
            || expr.is_monitor()
            || expr.is_if()
            || expr.try_as_var().is_some_and(|v| v.is_class_def()))
    }

    pub fn add_deleted_goto_instruction_mapping(stat: Statement, buffer: &mut TextBuffer) {
        if let Some(bb) = stat.try_as_basic_block() {
            let block = bb.get_block();
            let offsets = block.get_instr_old_offsets();
            if !offsets.is_empty() && offsets.len() as i32 > block.get_seq().length() {
                buffer.add_bytecode_mapping_offset(offsets.get(offsets.len() - 1));
            }
        }
    }

    pub fn jmp_wrapper(stat: Statement, indent: i32, semicolon: bool) -> TextBuffer {
        let mut buf = stat.to_java(indent);

        let lst_succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        if lst_succs.len() == 1 {
            let edge = lst_succs[0];
            if edge.get_type() != StatEdge::TYPE_REGULAR
                && edge.explicit()
                && !edge.get_destination().is_dummy_exit()
            {
                buf.append_indent(indent);

                match edge.get_type() {
                    StatEdge::TYPE_BREAK => {
                        Self::add_deleted_goto_instruction_mapping(stat, &mut buf);
                        buf.append("break");
                    }
                    StatEdge::TYPE_CONTINUE => {
                        Self::add_deleted_goto_instruction_mapping(stat, &mut buf);
                        buf.append("continue");
                    }
                    _ => {}
                }

                if edge.labeled() {
                    let closure = edge.closure().expect("NullPointerException: edge.closure");
                    buf.append(" label").append_int(closure.id());
                }
                buf.append(";").append_line_separator();
            }
        }

        if buf.length() == 0 && semicolon {
            buf.append_indent(indent)
                .append(";")
                .append_line_separator();
        }

        buf
    }

    pub fn build_java_class_name(name: &str) -> String {
        let mut res = name.replace('/', ".");

        if res.contains('$') {
            let cl = DecompilerContext::get_struct_context().get_class(name);
            if cl.is_none_or(|cl| !cl.is_own()) {
                res = res.replace('$', ".");
            }
        }

        res
    }

    pub fn list_to_java(lst: Option<&JList<Exprent>>, indent: i32) -> TextBuffer {
        let lst = match lst {
            Some(lst) if !lst.is_empty() => lst,
            _ => return TextBuffer::new(),
        };

        let cl = DecompilerContext::get_current_class();
        let plugin_context = PluginContext::get_current_context();
        let spec = cl.and_then(|cl| plugin_context.get_language_spec(cl));

        let mut buf = TextBuffer::new();
        let lst = Exprent::sort_indexed(&lst.to_vec());

        for expr in lst {
            let is_class_def = expr.try_as_var().is_some_and(|v| v.is_class_def());
            if buf.length() > 0 && is_class_def {
                buf.append_line_separator();
            }

            expr.get_inferred_expr_type(None);

            let content = expr.to_java(indent);

            if content.length() > 0 {
                if !expr.try_as_var().is_some_and(|v| v.is_class_def()) {
                    buf.append_indent(indent);
                }
                buf.append_buffer(&content);
                if expr
                    .try_as_monitor()
                    .is_some_and(|m| m.get_mon_type() == MonitorType::Enter)
                {
                    buf.append("{} // $VF: monitorenter ");
                }
                if let Some(spec) = spec {
                    match spec {}
                } else if Self::ends_with_semicolon(expr) {
                    buf.append_char(';');
                }
                buf.append_line_separator();
            }
        }

        buf
    }

    pub fn get_default_array_value(arr_type: &VarType) -> ConstExprent {
        if arr_type.type_ == CodeType::Object || arr_type.array_dim > 0 {
            ConstExprent::new(VarType::VARTYPE_NULL(), None, None)
        } else if arr_type.type_ == CodeType::Float {
            ConstExprent::new(VarType::VARTYPE_FLOAT(), Some(ConstValue::Float(0.0)), None)
        } else if arr_type.type_ == CodeType::Long {
            ConstExprent::new(VarType::VARTYPE_LONG(), Some(ConstValue::Long(0)), None)
        } else if arr_type.type_ == CodeType::Double {
            ConstExprent::new(
                VarType::VARTYPE_DOUBLE(),
                Some(ConstValue::Double(0.0)),
                None,
            )
        } else {
            ConstExprent::new_int(0, true, None)
        }
    }

    pub fn get_casted_exprent(
        exprent: Exprent,
        left_type: &VarType,
        buffer: &mut TextBuffer,
        indent: i32,
        cast_null: bool,
    ) -> bool {
        Self::get_casted_exprent_full(
            exprent,
            left_type,
            buffer,
            indent,
            if cast_null {
                NullCastType::Cast
            } else {
                NullCastType::DontCast
            },
            false,
            false,
            false,
        )
    }

    /// Java: getCastedExprent(Exprent, VarType, TextBuffer, int, NullCastType, boolean, boolean, boolean)
    #[allow(clippy::too_many_arguments)]
    pub fn get_casted_exprent_full(
        exprent: Exprent,
        left_type: &VarType,
        buffer: &mut TextBuffer,
        indent: i32,
        cast_null: NullCastType,
        cast_always: bool,
        cast_narrowing: bool,
        unbox: bool,
    ) -> bool {
        let mut exprent = exprent;
        let mut left_type = left_type.clone();

        if unbox {
            if let Some(invocation_exprent) = exprent.try_as_invocation() {
                if invocation_exprent.is_boxing_call() && !invocation_exprent.should_force_boxing()
                {
                    exprent = invocation_exprent.get_lst_parameters().get(0);
                    let param_type = invocation_exprent.get_descriptor().params[0].type_;
                    if exprent
                        .try_as_const()
                        .is_some_and(|c| c.get_const_type().type_ != param_type)
                    {
                        left_type = VarType::new(param_type);
                    }
                }
            }
        }

        let right_type = exprent.get_inferred_expr_type(Some(&left_type));
        exprent = Self::narrow_generic_cast_type(exprent, &left_type);

        let do_cast = !left_type.higher_equal_in_lattice_than(&right_type)
            && (right_type.equals(&VarType::VARTYPE_OBJECT())
                || left_type.type_ != CodeType::Object);
        let do_cast_null = cast_null.cast()
            && right_type.type_ == CodeType::Null
            && Self::UNDEFINED_TYPE_STRING != Self::get_type_name(&left_type);
        let do_cast_narrowing = cast_narrowing
            && Self::is_int_constant(exprent)
            && Self::is_narrowed_int_type(&left_type);
        let do_cast_generics =
            Self::do_generic_types_cast(exprent, Some(&left_type), Some(&right_type));

        let mut cast =
            cast_always || do_cast || do_cast_null || do_cast_narrowing || do_cast_generics;

        if cast_null == NullCastType::DontCastAtAll && right_type.type_ == CodeType::Null {
            cast = cast_always;
        }

        let cast_lambda = !cast
            && exprent.is_new()
            && !left_type.equals(&right_type)
            && Self::lambda_needs_cast(&left_type, exprent.as_new());

        let quote = cast && exprent.get_precedence() >= FunctionType::Cast.precedence();

        if cast_narrowing && exprent.try_as_const().is_some_and(|c| !c.is_null()) {
            if left_type.equals(&VarType::VARTYPE_BYTE_OBJ()) {
                left_type = VarType::VARTYPE_BYTE();
            } else if left_type.equals(&VarType::VARTYPE_SHORT_OBJ()) {
                left_type = VarType::VARTYPE_SHORT();
            }
        }

        if cast {
            buffer
                .append_char('(')
                .append_cast_type_name(&left_type)
                .append_char(')');
        }

        if cast_lambda {
            buffer
                .append_char('(')
                .append_cast_type_name(&right_type)
                .append_char(')');
        }

        if quote {
            buffer.append_char('(');
        }

        if let Some(c) = exprent.try_as_const() {
            c.adjust_const_type(&left_type);
        }

        if cast {
            if let Some(new_exprent) = exprent.try_as_new() {
                if new_exprent.is_var_arg_param() {
                    new_exprent.set_var_arg_param(false);
                }
            }
        }

        buffer.append_buffer(&exprent.to_java(indent));

        if quote {
            buffer.append_char(')');
        }

        cast
    }

    pub fn narrow_generic_cast_type(expr: Exprent, type_: &VarType) -> Exprent {
        if type_.is_generic() {
            if let Some(func) = expr.try_as_function() {
                if func.get_func_type() == FunctionType::Cast {
                    let func_type = func.get_expr_type();

                    let gen_type = type_.as_generic();
                    if type_.value.as_deref() == Some(func_type.value_str())
                        && !gen_type.get_arguments().is_empty()
                    {
                        if !func_type.is_generic() {
                            let cast = func.get_lst_operands().get(1).as_const();
                            cast.set_const_type(type_.clone());
                        } else if gen_type.equals_exact(&func_type) && !func.does_cast() {
                            func.set_needs_cast(true);
                        }
                    }
                }
            }
        }

        expr
    }

    fn do_generic_types_cast(ex: Exprent, left: Option<&VarType>, right: Option<&VarType>) -> bool {
        let named = ex.get_named_generics();
        let (left, right) = match (left, right) {
            (Some(left), Some(right)) => (left, right),
            _ => return false,
        };

        if left.is_generic() && right.is_generic() {
            if Self::should_generic_types_cast(&named, left, right) {
                return true;
            }

            let left_generic = left.as_generic();
            let right_generic = right.as_generic();

            if left_generic.get_arguments().len() != right_generic.get_arguments().len() {
                return false;
            }

            for i in 0..left_generic.get_arguments().len() {
                let left_type = left_generic.get_arguments()[i].clone();
                let right_type = right_generic.get_arguments()[i].clone();
                if right_type.is_none() && left_type.is_some() {
                    return true;
                }

                if let Some(left_type) = &left_type {
                    let right_type = right_type
                        .as_ref()
                        .expect("NullPointerException: rightType");
                    if Self::should_generic_types_cast(&named, left_type, right_type) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn should_generic_types_cast(
        named: &JHashMap<VarType, Vec<VarType>>,
        left_type: &VarType,
        right_type: &VarType,
    ) -> bool {
        if left_type.is_generic() {
            let gen_left = left_type.as_generic();
            if right_type.is_generic() {
                let gen_right = right_type.as_generic();

                if left_type.higher_equal_in_lattice_than(right_type) {
                    if (gen_left.get_wildcard() == GenericType::WILDCARD_NO
                        || gen_left.get_wildcard() == GenericType::WILDCARD_EXTENDS)
                        && gen_right.get_wildcard() == GenericType::WILDCARD_SUPER
                    {
                        let mut ok = true;
                        if gen_left.get_wildcard() == GenericType::WILDCARD_EXTENDS
                            && !gen_left.get_arguments().is_empty()
                            && !gen_right.get_arguments().is_empty()
                            && left_type.equals(right_type)
                        {
                            ok = false;
                        }

                        if ok {
                            return true;
                        }
                    }
                } else {
                    if gen_left.get_wildcard() == GenericType::WILDCARD_EXTENDS
                        && gen_right.get_wildcard() == GenericType::WILDCARD_SUPER
                    {
                        return true;
                    }
                    if left_type.type_ == CodeType::Genvar
                        && right_type.type_ == CodeType::Genvar
                        && gen_left.get_wildcard() == GenericType::WILDCARD_NO
                        && gen_left.get_wildcard() == GenericType::WILDCARD_NO
                    {
                        let object = VarType::VARTYPE_OBJECT();
                        if named.contains_key(left_type)
                            && named.contains_key(right_type)
                            && named
                                .get(left_type)
                                .expect("NullPointerException: named.get(leftType)")
                                .j_contains(&object)
                            && named
                                .get(right_type)
                                .expect("NullPointerException: named.get(rightType)")
                                .j_contains(&object)
                        {
                            return true;
                        }
                    }
                }

                if (gen_left.get_wildcard() == GenericType::WILDCARD_NO
                    || gen_left.get_wildcard() == GenericType::WILDCARD_SUPER)
                    && gen_right.get_wildcard() == GenericType::WILDCARD_EXTENDS
                {
                    return true;
                }

                if gen_left.get_arguments().len() != gen_right.get_arguments().len() {
                    return false;
                }

                if gen_left.get_wildcard() == gen_right.get_wildcard() {
                    for i in 0..gen_left.get_arguments().len() {
                        let lt = gen_left.get_arguments()[i].clone();
                        let rt = gen_right.get_arguments()[i].clone();
                        if rt.is_none()
                            && lt.as_ref().is_some_and(|lt| lt.type_ == CodeType::Genvar)
                        {
                            return true;
                        }

                        if let (Some(lt), Some(rt)) = (&lt, &rt) {
                            if Self::should_generic_types_cast(named, lt, rt) {
                                return true;
                            }
                        }
                    }
                }
            } else if gen_left.get_wildcard() == GenericType::WILDCARD_NO
                && gen_left.get_arguments().is_empty()
            {
                return true;
            }
        }

        false
    }

    fn is_int_constant(exprent: Exprent) -> bool {
        if let Some(c) = exprent.try_as_const() {
            return matches!(
                c.get_const_type().type_,
                CodeType::Byte
                    | CodeType::Bytechar
                    | CodeType::Short
                    | CodeType::Shortchar
                    | CodeType::Int
            );
        }

        false
    }

    fn is_narrowed_int_type(type_: &VarType) -> bool {
        VarType::VARTYPE_INT().higher_in_lattice_than(type_)
            || type_.equals(&VarType::VARTYPE_BYTE_OBJ())
            || type_.equals(&VarType::VARTYPE_SHORT_OBJ())
    }

    fn lambda_needs_cast(left: &VarType, exprent: NewExprent) -> bool {
        if exprent.is_lambda() && !exprent.is_method_reference() {
            let cls = match left.value.as_deref() {
                Some(name) => DecompilerContext::get_struct_context().get_class(name),
                None => None,
            };
            return match cls {
                None => true,
                Some(cls) => cls
                    .get_method_key(&exprent.get_lambda_method_key())
                    .is_none(),
            };
        }
        false
    }
}
