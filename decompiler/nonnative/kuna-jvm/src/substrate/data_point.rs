//! Port of `org/jetbrains/java/decompiler/struct/gen/DataPoint.java`.

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::list_stack::ListStack;
use crate::method_descriptor::MethodDescriptor;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

pub struct DataPoint {
    local_variables: Vec<VarType>,
    stack: ListStack<VarType>,
}

impl Default for DataPoint {
    fn default() -> Self {
        DataPoint::new()
    }
}

impl DataPoint {
    pub fn new() -> DataPoint {
        DataPoint {
            local_variables: Vec::new(),
            stack: ListStack::new(),
        }
    }

    pub fn set_variable(&mut self, index: i32, value: VarType) {
        if index >= self.local_variables.len() as i32 {
            for _ in self.local_variables.len() as i32..=index {
                self.local_variables
                    .push(VarType::new(CodeType::Notinitialized));
            }
        }
        self.local_variables[index as usize] = value;
    }

    pub fn get_variable(&self, index: i32) -> VarType {
        if index < self.local_variables.len() as i32 {
            self.local_variables[index as usize].clone()
        } else {
            VarType::new(CodeType::Notinitialized)
        }
    }

    pub fn copy(&self) -> DataPoint {
        let mut point = DataPoint::new();
        point.set_local_variables(self.local_variables.clone());
        point.set_stack(self.stack.clone());
        point
    }

    pub fn get_initial_data_point(cl: StructClass, mt: StructMethod) -> DataPoint {
        let mut point = DataPoint::new();

        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());

        let mut k: i32 = 0;
        if !mt.has_modifier(ACC_STATIC) {
            let name = cl.with(|d| d.qualified_name.clone());
            point.set_variable(k, VarType::new_value(CodeType::Object, 0, name));
            k += 1;
        }

        for var in &md.params {
            point.set_variable(k, var.clone());
            k += 1;
            if var.stack_size == 2 {
                point.set_variable(k, VarType::new(CodeType::Group2empty));
                k += 1;
            }
        }

        point
    }

    pub fn get_local_variables(&self) -> &Vec<VarType> {
        &self.local_variables
    }

    pub fn get_local_variables_mut(&mut self) -> &mut Vec<VarType> {
        &mut self.local_variables
    }

    pub fn set_local_variables(&mut self, local_variables: Vec<VarType>) {
        self.local_variables = local_variables;
    }

    pub fn get_stack(&mut self) -> &mut ListStack<VarType> {
        &mut self.stack
    }

    pub fn get_stack_ref(&self) -> &ListStack<VarType> {
        &self.stack
    }

    pub fn set_stack(&mut self, stack: ListStack<VarType>) {
        self.stack = stack;
    }
}
