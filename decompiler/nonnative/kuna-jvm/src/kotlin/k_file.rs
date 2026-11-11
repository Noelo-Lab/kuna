//! Port of `org/vineflower/kotlin/struct/KFile.java`.

use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;

#[derive(Clone)]
pub struct KFile {
    pub proto: kt_proto::Package,
    pub resolver: Option<MetadataNameResolver>,
    pub multifile_name: Option<String>,
}

impl KFile {
    pub fn new(
        proto: kt_proto::Package,
        resolver: Option<MetadataNameResolver>,
        multifile_name: Option<String>,
    ) -> KFile {
        KFile {
            proto,
            resolver,
            multifile_name,
        }
    }
}
