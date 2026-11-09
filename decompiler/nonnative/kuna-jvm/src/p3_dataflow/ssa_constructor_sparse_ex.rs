//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/SSAConstructorSparseEx.java`.
//!
//! A newtype over `SFormsConstructor` (see there); the overriding bodies take
//! the base as `this` so the base's dispatchers can reach them.
#![allow(dead_code, unused_variables)]

use std::ops::{Deref, DerefMut};

use crate::fast_sparse_set_factory::FastSparseSet;
use crate::field_exprent::FieldExprent;
use crate::rt::{JHashMap, Shared};
use crate::s_forms_constructor::{SFormsConstructor, SFormsConstructorKind};
use crate::s_forms_fast_map_direct::SFormsFastMapDirect;
use crate::statement::Statement;
use crate::unported;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

pub struct SSAConstructorSparseExData {
    phi: JHashMap<VarVersionPair, FastSparseSet<i32>>,
}

pub struct SSAConstructorSparseEx(pub SFormsConstructor);

impl Deref for SSAConstructorSparseEx {
    type Target = SFormsConstructor;
    fn deref(&self) -> &SFormsConstructor {
        &self.0
    }
}

impl DerefMut for SSAConstructorSparseEx {
    fn deref_mut(&mut self) -> &mut SFormsConstructor {
        &mut self.0
    }
}

impl Default for SSAConstructorSparseEx {
    fn default() -> Self {
        Self::new()
    }
}

impl SSAConstructorSparseEx {
    pub fn new() -> SSAConstructorSparseEx {
        SSAConstructorSparseEx(SFormsConstructor::new(
            false,
            false,
            SFormsConstructorKind::Ssa(SSAConstructorSparseExData {
                phi: JHashMap::new(),
            }),
        ))
    }

    fn d(this: &SFormsConstructor) -> &SSAConstructorSparseExData {
        match &this.kind {
            SFormsConstructorKind::Ssa(d) => d,
            _ => unreachable!("not an SSAConstructorSparseEx"),
        }
    }

    fn d_mut(this: &mut SFormsConstructor) -> &mut SSAConstructorSparseExData {
        match &mut this.kind {
            SFormsConstructorKind::Ssa(d) => d,
            _ => unreachable!("not an SSAConstructorSparseEx"),
        }
    }

    pub fn mark_direct_assignment(
        this: &mut SFormsConstructor,
        var_version_pair: VarVersionPair,
        right_pair: VarVersionPair,
    ) {
    }

    pub fn get_or_create_phantom(
        this: &mut SFormsConstructor,
        pair: VarVersionPair,
    ) -> VarVersionPair {
        pair
    }

    pub fn get_field_index(this: &mut SFormsConstructor, field: FieldExprent) -> i32 {
        -1
    }

    pub fn var_read_single_version(
        this: &mut SFormsConstructor,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
        varmap: &Shared<SFormsFastMapDirect>,
        last_version: i32,
    ) {
        var_exprent.set_version(last_version);
    }

    pub fn var_read_multiple_versions(
        this: &mut SFormsConstructor,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
        var_map: &Shared<SFormsFastMapDirect>,
        versions: FastSparseSet<i32>,
    ) {
        unported!("SSAConstructorSparseEx.varReadMultipleVersions")
    }

    pub fn get_phi(&self) -> &JHashMap<VarVersionPair, FastSparseSet<i32>> {
        &Self::d(&self.0).phi
    }

    pub fn get_simple_reverse_phi_lookup(&self) -> JHashMap<VarVersionPair, i32> {
        unported!("SSAConstructorSparseEx.getSimpleReversePhiLookup")
    }

    pub fn init_version(this: &mut SFormsConstructor, var_exprent: VarExprent, stat: Statement) {
        if var_exprent.get_version() == 0 {
            let next_version = this.get_next_free_version(var_exprent.get_index(), Some(stat));

            var_exprent.set_version(next_version);
        }
    }

    pub fn init_parameter(
        this: &mut SFormsConstructor,
        var_index: i32,
        var_map: &Shared<SFormsFastMapDirect>,
        is_catch_var: bool,
    ) {
        let root: Option<Statement> = this.root.map(Into::into);
        let version = this.get_next_free_version(var_index, root);

        var_map.borrow_mut().set_current_var(var_index, version);
    }
}
