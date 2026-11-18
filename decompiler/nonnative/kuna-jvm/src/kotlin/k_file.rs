//! Port of `org/vineflower/kotlin/struct/KFile.java`.

#[derive(Clone)]
pub struct KFile {
    pub multifile_name: Option<String>,
}

impl KFile {
    pub fn new(multifile_name: Option<String>) -> KFile {
        KFile { multifile_name }
    }
}
