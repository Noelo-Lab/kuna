//! Port of `org/vineflower/kotlin/struct/KClass.java`.

use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;

#[derive(Clone)]
pub struct KClass {
    pub proto: kt_proto::Class,
    pub resolver: MetadataNameResolver,
}

impl KClass {
    pub fn new(proto: kt_proto::Class, resolver: MetadataNameResolver) -> KClass {
        KClass { proto, resolver }
    }

    pub fn flags(&self) -> i32 {
        self.proto.get_flags()
    }
}
