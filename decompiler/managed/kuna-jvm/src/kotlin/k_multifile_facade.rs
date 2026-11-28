//! Port of `org/vineflower/kotlin/struct/KMultifileFacade.java`.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KMultifileFacade {
    pub files: Vec<String>,
}

impl KMultifileFacade {
    pub fn new(files: Vec<String>) -> KMultifileFacade {
        KMultifileFacade { files }
    }
}
