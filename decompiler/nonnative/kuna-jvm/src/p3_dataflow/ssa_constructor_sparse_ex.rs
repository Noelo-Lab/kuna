//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/SSAConstructorSparseEx.java`.
//!
//! A newtype over `SFormsConstructor` (see there); the overriding bodies take
//! the base as `this` so the base's dispatchers can reach them.
#![allow(unused_variables)]

use std::ops::{Deref, DerefMut};

use crate::field_exprent::FieldExprent;
use crate::rt::{JHashMap, Shared};
use crate::s_forms_constructor::{SFormsConstructor, SFormsConstructorKind};
use crate::s_forms_fast_map_direct::{SFormsFastMapDirect, VersionSet};
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

pub struct SSAConstructorSparseExData {
    phi: JHashMap<VarVersionPair, VersionSet>,
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
        versions: VersionSet,
    ) {
        let var_index = var_exprent.get_index();
        let current_version = var_exprent.get_version();
        let var_version = var_exprent.get_var_version_pair();
        let existing = if current_version != 0 {
            Self::d(this).phi.get_cloned(&var_version)
        } else {
            None
        };
        if let Some(phi_set) = existing {
            union_sets(&phi_set, &versions);
        } else {
            let next_ver = this.get_next_free_version(var_index, Some(stat));
            var_exprent.set_version(next_ver);

            Self::d_mut(this)
                .phi
                .put(var_exprent.get_var_version_pair(), versions);
        }

        var_map.borrow_mut().set_current_var_exprent(var_exprent);
    }

    pub fn get_phi(&self) -> &JHashMap<VarVersionPair, VersionSet> {
        &Self::d(&self.0).phi
    }

    pub fn get_simple_reverse_phi_lookup(&self) -> JHashMap<VarVersionPair, i32> {
        let phi = &Self::d(&self.0).phi;
        let mut ret: JHashMap<VarVersionPair, i32> = JHashMap::new();
        for (key, value) in phi.iter() {
            let index = key.var;
            let mut left = *key;

            while let Some(&version) = ret.get(&left) {
                if version == left.version {
                    break;
                }
                left = VarVersionPair::new(index, version);
            }

            let vers: Vec<i32> = value.borrow().iter().collect();
            for ver in vers {
                let mut right = VarVersionPair::new(index, ver);

                while let Some(&version) = ret.get(&right) {
                    if version == right.version {
                        break;
                    }
                    right = VarVersionPair::new(index, version);
                }

                if left.version != right.version {
                    if right.version < left.version {
                        ret.put(left, right.version);
                        left = right;
                    } else {
                        ret.put(right, left.version);
                    }
                }
            }
        }

        for key in phi.keys() {
            let mut pair = *key;

            while let Some(&version) = ret.get(&pair) {
                if version == pair.version {
                    break;
                }
                pair = VarVersionPair::new(pair.var, version);
            }

            ret.put(*key, pair.version);
        }

        ret
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

/// `a.union(b)` on two shared version sets, which may be the same set.
fn union_sets(a: &VersionSet, b: &VersionSet) {
    if a.ptr_eq(b) {
        let snapshot = b.borrow().get_copy();
        a.borrow_mut().union(&snapshot);
    } else {
        a.borrow_mut().union(&b.borrow());
    }
}
