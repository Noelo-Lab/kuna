//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/VarMapHolder.java`.
//!
//! The maps are `Shared`: `set` makes two holders alias the same maps, and the
//! state logic compares them by identity (`ptr_eq`). `ifTrue` is never null in
//! practice, but `setIfFalse` can null it, so it is an `Option`.
//!
//! kuna fix `field_read_all_paths` (spec 10.16): a holder made with
//! `all_paths` keeps, when it merges two states, only the field variables both
//! have.

use crate::p3_dataflow::kuna_fix_field_read_all_paths::FieldReadAllPaths;
use crate::rt::Shared;
use crate::s_forms_fast_map_direct::SFormsFastMapDirect;
use crate::validation_helper::ValidationHelper;

pub struct VarMapHolder {
    if_true: Option<Shared<SFormsFastMapDirect>>,
    if_false: Option<Shared<SFormsFastMapDirect>>,
    all_paths: bool,
}

fn same(a: Option<&Shared<SFormsFastMapDirect>>, b: Option<&Shared<SFormsFastMapDirect>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.ptr_eq(b),
        _ => false,
    }
}

impl VarMapHolder {
    fn new(
        if_true: Option<Shared<SFormsFastMapDirect>>,
        if_false: Option<Shared<SFormsFastMapDirect>>,
    ) -> VarMapHolder {
        VarMapHolder {
            if_true,
            if_false,
            all_paths: false,
        }
    }

    pub fn of_normal(holder: &Shared<SFormsFastMapDirect>) -> VarMapHolder {
        let copy = holder.borrow().get_copy();
        VarMapHolder::new(Some(Shared::new(copy)), None)
    }

    /// `of_normal`, merging field variables on all paths when `all_paths` is
    /// set (fix `field_read_all_paths`).
    pub fn of_normal_all_paths(
        holder: &Shared<SFormsFastMapDirect>,
        all_paths: bool,
    ) -> VarMapHolder {
        let mut res = Self::of_normal(holder);
        res.all_paths = all_paths;
        res
    }

    pub fn all_paths(&self) -> bool {
        self.all_paths
    }

    /// With `all_paths`, the field variables of `map` before a merge into it.
    fn fields_before(&self, map: &Shared<SFormsFastMapDirect>) -> Option<Vec<i32>> {
        self.all_paths.then(|| FieldReadAllPaths::fields(map))
    }

    /// With `all_paths`: drops from `merged` the field variables that it did not
    /// have before the merge (`before`) or that `other` lacks.
    fn keep_common_fields(
        merged: &Shared<SFormsFastMapDirect>,
        before: Option<Vec<i32>>,
        other: &Shared<SFormsFastMapDirect>,
    ) {
        let Some(before) = before else { return };
        if merged.ptr_eq(other) {
            return;
        }
        for key in FieldReadAllPaths::fields(merged) {
            if !before.contains(&key) || !other.borrow().contains_key(key) {
                merged.borrow_mut().remove(key);
            }
        }
    }

    fn assert_legal(&self) {
        ValidationHelper::assert_true(
            self.if_true.is_some() && !same(self.if_true.as_ref(), self.if_false.as_ref()),
            "VarMapHolder is in an illegal state",
        );
    }

    pub fn get_if_true(&self) -> Shared<SFormsFastMapDirect> {
        self.assert_legal();
        self.if_true.clone().expect("ifTrue")
    }

    pub fn get_if_false(&self) -> Shared<SFormsFastMapDirect> {
        self.assert_legal();
        match &self.if_false {
            None => self.if_true.clone().expect("ifTrue"),
            Some(if_false) => if_false.clone(),
        }
    }

    /// Returns the merged map. Java returns `null` only in the unreachable
    /// state where `ifTrue` is null; that panics here.
    pub fn to_normal(&mut self) -> Shared<SFormsFastMapDirect> {
        let before = self.fields_before(self.if_true.as_ref().expect("ifTrue"));
        let result = Self::merge_maps(
            self.if_true.as_ref().expect("ifTrue"),
            self.if_false.as_ref(),
        );
        if let Some(if_false) = &self.if_false {
            Self::keep_common_fields(&result, before, if_false);
        }
        self.if_false = None;
        result
    }

    pub fn get_normal(&self) -> Shared<SFormsFastMapDirect> {
        self.assert_is_normal();

        self.if_true.clone().expect("ifTrue")
    }

    pub fn assert_is_normal(&self) {
        ValidationHelper::assert_true(self.is_normal(), "VarMapHolder is not in normal state");
    }

    pub fn set_if_true(&mut self, if_true: Shared<SFormsFastMapDirect>) {
        if !same(self.if_true.as_ref(), Some(&if_true)) && self.if_false.is_none() {
            self.if_false = self.if_true.clone();
        } else if same(self.if_false.as_ref(), Some(&if_true)) {
            self.if_false = None;
        }

        self.if_true = Some(if_true);
    }

    pub fn set_if_false(&mut self, if_false: Shared<SFormsFastMapDirect>) {
        if same(self.if_true.as_ref(), Some(&if_false)) {
            self.if_true = None;
        } else {
            self.if_false = Some(if_false);
        }
    }

    pub fn set_normal(&mut self, normal: Shared<SFormsFastMapDirect>) {
        self.if_false = None;
        self.if_true = Some(normal);
    }

    pub fn set(&mut self, b_var_maps: &VarMapHolder) {
        self.if_true = b_var_maps.if_true.clone();
        self.if_false = b_var_maps.if_false.clone();
    }

    pub fn merge_if_true(&mut self, map2: Option<&Shared<SFormsFastMapDirect>>) {
        let Some(map2) = map2 else { return };
        if same(self.if_true.as_ref(), Some(map2)) || map2.borrow().is_empty() {
            return;
        }

        self.make_fully_mutable();
        let if_true = self.if_true.clone().expect("ifTrue");
        let before = self.fields_before(&if_true);
        if_true.borrow_mut().union(&map2.borrow());
        Self::keep_common_fields(&if_true, before, map2);
    }

    pub fn merge_if_false(&mut self, map2: Option<&Shared<SFormsFastMapDirect>>) {
        let Some(map2) = map2 else { return };
        if same(self.if_false.as_ref(), Some(map2)) || map2.borrow().is_empty() {
            return;
        }

        self.make_fully_mutable();
        let if_false = self.if_false.clone().expect("ifFalse");
        let before = self.fields_before(&if_false);
        if_false.borrow_mut().union(&map2.borrow());
        Self::keep_common_fields(&if_false, before, map2);
    }

    pub fn merge_normal(&mut self, map2: Option<&Shared<SFormsFastMapDirect>>) {
        self.assert_is_normal();

        let Some(map2) = map2 else { return };
        if same(self.if_true.as_ref(), Some(map2)) || map2.borrow().is_empty() {
            return;
        }

        let if_true = self.if_true.clone().expect("ifTrue");
        let before = self.fields_before(&if_true);
        if_true.borrow_mut().union(&map2.borrow());
        Self::keep_common_fields(&if_true, before, map2);
    }

    pub fn is_normal(&self) -> bool {
        self.if_false.is_none()
    }

    pub fn swap(&mut self) {
        if self.if_false.is_none() {
            return;
        }

        std::mem::swap(&mut self.if_true, &mut self.if_false);
    }

    pub fn make_fully_mutable(&mut self) {
        if self.if_false.is_some() {
            ValidationHelper::assert_true(
                !same(self.if_true.as_ref(), self.if_false.as_ref()),
                "VarMapHolder is in an illegal state",
            );
            return;
        }

        let copy = self.if_true.as_ref().expect("ifTrue").borrow().get_copy();
        self.if_false = Some(Shared::new(copy));
    }

    pub fn merge_maps(
        map_to: &Shared<SFormsFastMapDirect>,
        map2: Option<&Shared<SFormsFastMapDirect>>,
    ) -> Shared<SFormsFastMapDirect> {
        if let Some(map2) = map2 {
            if !map_to.ptr_eq(map2) && !map2.borrow().is_empty() {
                map_to.borrow_mut().union(&map2.borrow());
            }
        }

        map_to.clone()
    }

    pub fn remove_all_fields(&mut self) {
        if let Some(if_false) = &self.if_false {
            if_false.borrow_mut().remove_all_fields();
        }
        self.if_true
            .as_ref()
            .expect("ifTrue")
            .borrow_mut()
            .remove_all_fields();
    }
}
