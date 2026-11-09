//! Port of `org/jetbrains/java/decompiler/api/java/JavaPassRegistrar.java`.

use std::rc::Rc;

use crate::class_pass::ClassPass;
use crate::class_pass_location::ClassPassLocation;
use crate::java_pass_location::JavaPassLocation;
use crate::named_pass::NamedPass;
use crate::rt::JHashMap;

#[derive(Default)]
pub struct JavaPassRegistrar {
    passes: JHashMap<JavaPassLocation, Vec<NamedPass>>,
    class_passes: JHashMap<ClassPassLocation, Vec<Rc<dyn ClassPass>>>,
}

impl JavaPassRegistrar {
    pub fn new() -> JavaPassRegistrar {
        JavaPassRegistrar::default()
    }

    pub fn register(&mut self, location: JavaPassLocation, pass: NamedPass) {
        self.passes.compute_if_absent(location, Vec::new).push(pass);
    }

    pub fn register_class_pass(&mut self, location: ClassPassLocation, pass: Rc<dyn ClassPass>) {
        self.class_passes
            .compute_if_absent(location, Vec::new)
            .push(pass);
    }

    pub fn get_passes(&self) -> JHashMap<JavaPassLocation, Vec<NamedPass>> {
        self.passes.clone()
    }

    pub fn get_class_passes(&self) -> JHashMap<ClassPassLocation, Vec<Rc<dyn ClassPass>>> {
        self.class_passes.clone()
    }
}
