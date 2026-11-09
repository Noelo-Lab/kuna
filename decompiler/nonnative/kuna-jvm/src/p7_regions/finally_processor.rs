//! Port of `org/jetbrains/java/decompiler/modules/decompiler/FinallyProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::basic_block::BasicBlock;
use crate::bytecode_version::BytecodeVersion;
use crate::catch_all_statement::CatchAllStatement;
use crate::code_constants::{
    OPC_ALOAD, OPC_ARETURN, OPC_ASTORE, OPC_DLOAD, OPC_DRETURN, OPC_DSTORE, OPC_FLOAD, OPC_FRETURN,
    OPC_FSTORE, OPC_IINC, OPC_ILOAD, OPC_IRETURN, OPC_ISTORE, OPC_LLOAD, OPC_LRETURN, OPC_LSTORE,
    OPC_SASTORE,
};
use crate::control_flow_graph::ControlFlowGraph;
use crate::instruction::Instruction;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet};
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::unported;
use crate::var_processor::VarProcessor;
use crate::var_versions_graph::VarVersionsGraph;

/// Inlines `finally` handlers, or guards them with a semaphore variable when the
/// copies cannot be verified. Plain owned value: `MethodProcessor` creates one per
/// method and calls `iterate_graph` until it returns false.
pub struct FinallyProcessor {
    finally_blocks: JHashMap<BasicBlock, Option<i32>>,
    catchall_blocks: JHashSet<BasicBlock>,
    mt: StructMethod,
    method_descriptor: MethodDescriptor,
    var_processor: VarProcessor,
    ssuversions: Option<VarVersionsGraph>,
    instr_rewrites: Option<JHashMap<Instruction, i32>>,
}

struct Record {
    first_code: i32,
    exception_offset: i32,
    map_last: JLinkedHashMap<BasicBlock, bool>,
}

impl Record {
    fn new(
        first_code: i32,
        exception_offset: i32,
        map_last: JLinkedHashMap<BasicBlock, bool>,
    ) -> Record {
        Record {
            first_code,
            exception_offset,
            map_last,
        }
    }
}

struct Area {
    start: BasicBlock,
    sample: JHashSet<BasicBlock>,
    next: Option<BasicBlock>,
    side_exits: JHashSet<BasicBlock>,
}

impl Area {
    fn new(
        start: BasicBlock,
        sample: JHashSet<BasicBlock>,
        next: Option<BasicBlock>,
        side_exits: JHashSet<BasicBlock>,
    ) -> Area {
        Area {
            start,
            sample,
            next,
            side_exits,
        }
    }
}

/// The local class `BlockStackEntry` of `compareSubgraphsEx`.
struct BlockStackEntry {
    pub block_catch: BasicBlock,
    pub block_sample: BasicBlock,
    pub lst_store_vars: Vec<[i32; 2]>,
}

impl BlockStackEntry {
    fn new(
        block_catch: BasicBlock,
        block_sample: BasicBlock,
        lst_store_vars: &[[i32; 2]],
    ) -> BlockStackEntry {
        BlockStackEntry {
            block_catch,
            block_sample,
            lst_store_vars: lst_store_vars.to_vec(),
        }
    }
}

/// One `BasicBlock[]{from, to, trueExit}` value of `compareSubgraphsEx`'s `mapNext`.
type NextEdge = (BasicBlock, BasicBlock, Option<BasicBlock>);

const STORE_CODES: [i32; 5] = [OPC_ISTORE, OPC_LSTORE, OPC_FSTORE, OPC_DSTORE, OPC_ASTORE];

const NEXT_CODES: [[i32; 2]; 5] = [
    [OPC_ILOAD, OPC_IRETURN],
    [OPC_LLOAD, OPC_LRETURN],
    [OPC_FLOAD, OPC_FRETURN],
    [OPC_DLOAD, OPC_DRETURN],
    [OPC_ALOAD, OPC_ARETURN],
];

impl FinallyProcessor {
    pub fn new(
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
    ) -> FinallyProcessor {
        FinallyProcessor {
            finally_blocks: JHashMap::new(),
            catchall_blocks: JHashSet::new(),
            mt,
            method_descriptor: md.clone(),
            var_processor: var_proc,
            ssuversions: None,
            instr_rewrites: None,
        }
    }

    pub fn iterate_graph(
        &mut self,
        cl: StructClass,
        mt: StructMethod,
        root: RootStatement,
        graph: ControlFlowGraph,
    ) -> bool {
        unported!("FinallyProcessor.iterateGraph")
    }

    fn get_finally_information(
        &mut self,
        cl: StructClass,
        mt: StructMethod,
        root: RootStatement,
        fstat: CatchAllStatement,
    ) -> Option<Record> {
        unported!("FinallyProcessor.getFinallyInformation")
    }

    fn insert_semaphore(
        graph: ControlFlowGraph,
        set_try: &JLinkedHashSet<BasicBlock>,
        head: BasicBlock,
        handler: BasicBlock,
        var: i32,
        information: &Record,
        bytecode_version: &BytecodeVersion,
    ) {
        unported!("FinallyProcessor.insertSemaphore")
    }

    fn insert_block_before(graph: ControlFlowGraph, oldblock: BasicBlock, newblock: BasicBlock) {
        unported!("FinallyProcessor.insertBlockBefore")
    }

    fn get_all_basic_blocks(stat: Statement) -> JLinkedHashSet<BasicBlock> {
        unported!("FinallyProcessor.getAllBasicBlocks")
    }

    fn verify_finally_ex(
        &self,
        graph: ControlFlowGraph,
        fstat: CatchAllStatement,
        information: &Record,
    ) -> bool {
        unported!("FinallyProcessor.verifyFinallyEx")
    }

    #[allow(clippy::too_many_arguments)]
    fn compare_subgraphs_ex(
        &self,
        graph: ControlFlowGraph,
        start_sample: BasicBlock,
        catch_blocks: &JLinkedHashSet<BasicBlock>,
        start_catch: BasicBlock,
        finallytype: i32,
        map_last: &JLinkedHashMap<BasicBlock, bool>,
        skipped_first: bool,
    ) -> Option<Area> {
        unported!("FinallyProcessor.compareSubgraphsEx")
    }

    fn get_side_exits(set_next: &[NextEdge]) -> JHashSet<BasicBlock> {
        let mut set = JHashSet::new();

        for next in set_next {
            if next.2.is_none() {
                set.add(next.1);
            }
        }

        set
    }

    fn get_unique_next(graph: ControlFlowGraph, set_next: &[NextEdge]) -> Option<BasicBlock> {
        unported!("FinallyProcessor.getUniqueNext")
    }

    fn compare_basic_blocks_ex(
        &self,
        graph: ControlFlowGraph,
        pattern: BasicBlock,
        sample: BasicBlock,
        type_: i32,
        finallytype: i32,
        lst_store_vars: &mut Vec<[i32; 2]>,
    ) -> bool {
        unported!("FinallyProcessor.compareBasicBlocksEx")
    }

    pub fn equal_instructions(
        &self,
        first: Instruction,
        second: Instruction,
        lst_store_vars: &mut Vec<[i32; 2]>,
    ) -> bool {
        unported!("FinallyProcessor.equalInstructions")
    }

    fn is_opc_var(opc: i32) -> bool {
        (opc >= OPC_ILOAD && opc <= OPC_SASTORE) || opc == OPC_IINC
    }

    fn delete_area(graph: ControlFlowGraph, area: &Area) {
        unported!("FinallyProcessor.deleteArea")
    }

    fn remove_exception_instructions_ex(block: BasicBlock, blocktype: i32, finallytype: i32) {
        unported!("FinallyProcessor.removeExceptionInstructionsEx")
    }

    fn inline_return_var(graph: ControlFlowGraph, handler: BasicBlock, inf: &Record) {
        unported!("FinallyProcessor.inlineReturnVar")
    }

    fn index_of(instr: Instruction) -> i32 {
        let opcode = instr.with(|d| d.opcode);
        for (i, &code) in STORE_CODES.iter().enumerate() {
            if opcode == code {
                return i as i32;
            }
        }

        -1
    }
}
