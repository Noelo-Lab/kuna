//! Port of `org/vineflower/kotlin/struct/KTypeParameter.java`.

#![allow(dead_code, unused_variables)]

use crate::k_type::KType;
use crate::kt_proto::{self, Variance};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::unported;

#[derive(Clone)]
pub struct KTypeParameter {
    pub reified: bool,
    pub variance: Variance,
    pub upper_bounds: Vec<KType>,
    pub name: String,
    pub id: i32,
}

impl KTypeParameter {
    pub fn new(
        reified: bool,
        variance: Variance,
        upper_bounds: Vec<KType>,
        name: String,
        id: i32,
    ) -> KTypeParameter {
        KTypeParameter {
            reified,
            variance,
            upper_bounds,
            name,
            id,
        }
    }

    pub fn from(
        proto: &kt_proto::TypeParameter,
        resolver: &MetadataNameResolver,
    ) -> KTypeParameter {
        unported!("KTypeParameter.from")
    }
}
