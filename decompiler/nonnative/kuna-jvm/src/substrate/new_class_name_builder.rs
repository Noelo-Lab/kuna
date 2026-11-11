//! Port of `org/jetbrains/java/decompiler/struct/gen/NewClassNameBuilder.java`.

pub trait NewClassNameBuilder {
    fn build_new_classname(&self, class_name: &str) -> Option<String>;
}

impl<F: Fn(&str) -> Option<String>> NewClassNameBuilder for F {
    fn build_new_classname(&self, class_name: &str) -> Option<String> {
        self(class_name)
    }
}
