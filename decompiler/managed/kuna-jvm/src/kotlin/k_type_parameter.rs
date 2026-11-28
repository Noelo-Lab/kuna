//! Port of `org/vineflower/kotlin/struct/KTypeParameter.java`.

use crate::k_type::KType;
use crate::kt_proto::{self, Variance};
use crate::metadata_name_resolver::MetadataNameResolver;

#[derive(Clone)]
pub struct KTypeParameter {
    pub reified: bool,
    pub variance: Variance,
    pub upper_bounds: Vec<KType>,
    pub name: String,
}

impl KTypeParameter {
    pub fn new(
        reified: bool,
        variance: Variance,
        upper_bounds: Vec<KType>,
        name: String,
    ) -> KTypeParameter {
        KTypeParameter {
            reified,
            variance,
            upper_bounds,
            name,
        }
    }

    pub fn from(
        proto: &kt_proto::TypeParameter,
        resolver: &MetadataNameResolver,
    ) -> KTypeParameter {
        KTypeParameter::new(
            proto.get_reified(),
            proto.get_variance(),
            proto
                .get_upper_bound_list()
                .iter()
                .map(|type_| KType::from(type_, resolver))
                .collect(),
            resolver.resolve(proto.get_name()),
        )
    }
}
