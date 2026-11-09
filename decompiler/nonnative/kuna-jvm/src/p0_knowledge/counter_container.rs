//! Port of `org/jetbrains/java/decompiler/main/collectors/CounterContainer.java`.

use crate::{arena_identity, arena_object};

pub struct CounterContainerData {
    pub values: [i32; 4],
}

arena_object!(pub CounterContainer => CounterContainerData, Class);
arena_identity!(CounterContainer);

impl CounterContainer {
    pub const STATEMENT_COUNTER: i32 = 0;
    pub const EXPRESSION_COUNTER: i32 = 1;
    pub const VAR_COUNTER: i32 = 2;
    pub const DOT_FILE_COUNTER: i32 = 3;

    #[allow(clippy::new_without_default)]
    pub fn new() -> CounterContainer {
        CounterContainer::alloc(CounterContainerData {
            values: [1, 1, 1, 1],
        })
    }

    pub fn set_counter(self, counter: i32, value: i32) {
        self.with_mut(|d| d.values[counter as usize] = value)
    }

    pub fn get_counter(self, counter: i32) -> i32 {
        self.with(|d| d.values[counter as usize])
    }

    pub fn get_counter_and_increment(self, counter: i32) -> i32 {
        self.with_mut(|d| {
            let value = d.values[counter as usize];
            d.values[counter as usize] = value.wrapping_add(1);
            value
        })
    }
}
