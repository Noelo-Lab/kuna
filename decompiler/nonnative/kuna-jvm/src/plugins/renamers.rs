//! Port of `org/vineflower/variablerenaming/Renamers.java`. The static map is
//! per thread, like every engine instance.

use std::cell::RefCell;
use std::rc::Rc;

use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::rt::JHashMap;

thread_local! {
    static PROVIDERS: RefCell<JHashMap<String, Rc<dyn IVariableNamingFactory>>> =
        RefCell::new(JHashMap::new());
}

pub struct Renamers;

impl Renamers {
    pub fn register_provider(name: &str, factory: Rc<dyn IVariableNamingFactory>) {
        PROVIDERS.with(|p| {
            p.borrow_mut().put(name.to_string(), factory);
        });
    }

    pub fn get(name: &str) -> Option<Rc<dyn IVariableNamingFactory>> {
        PROVIDERS.with(|p| p.borrow().get_cloned(&name.to_string()))
    }
}
