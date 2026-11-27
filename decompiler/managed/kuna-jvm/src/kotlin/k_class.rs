//! Port of `org/vineflower/kotlin/struct/KClass.java`.

use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;

#[derive(Clone)]
pub struct KClass {
    pub proto: kt_proto::Class,
    /// `null` when the metadata has no `d2` (see `KotlinChooser.parseMetadataFor`).
    pub resolver: Option<MetadataNameResolver>,
}

impl KClass {
    pub fn new(proto: kt_proto::Class, resolver: Option<MetadataNameResolver>) -> KClass {
        KClass { proto, resolver }
    }

    pub fn flags(&self) -> i32 {
        self.proto.get_flags()
    }
}
