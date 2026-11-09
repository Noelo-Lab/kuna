//! Port of `org/jetbrains/java/decompiler/util/collections/SFormsFastMapDirect.java`.
//!
//! Live variable versions per variable, in three segments: locals (index 0),
//! stack variables at `VarExprent::STACK_BASE` and up (1), and fields, keyed
//! by negative indices (2). The version sets are Java references that callers
//! mutate through `get` and `entry_list`, so they are `Shared`.

use crate::fast_sparse_set_factory::{
    java_collection_to_string, FastSparseSet, FastSparseSetFactory,
};
use crate::rt::Shared;
use crate::var_exprent::VarExprent;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;

pub type VersionSet = Shared<FastSparseSet<i32>>;

pub struct SFormsFastMapDirect {
    size: i32,
    factory: FastSparseSetFactory<i32>,
    elements: [Vec<Option<VersionSet>>; 3],
    next: [Vec<i32>; 3],
}

fn with_pair(
    first: &VersionSet,
    second: &VersionSet,
    op: impl FnOnce(&mut FastSparseSet<i32>, &FastSparseSet<i32>),
) {
    if first.ptr_eq(second) {
        let snapshot = second.borrow().clone();
        op(&mut first.borrow_mut(), &snapshot);
    } else {
        op(&mut first.borrow_mut(), &second.borrow());
    }
}

fn segment(key: i32) -> (usize, i32) {
    if key < 0 {
        (2, key.wrapping_neg())
    } else if key >= VarExprent::STACK_BASE {
        (1, key - VarExprent::STACK_BASE)
    } else {
        (0, key)
    }
}

impl SFormsFastMapDirect {
    pub fn new(factory: FastSparseSetFactory<i32>) -> SFormsFastMapDirect {
        SFormsFastMapDirect {
            size: 0,
            factory,
            elements: [Vec::new(), Vec::new(), Vec::new()],
            next: [Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn get_copy(&self) -> SFormsFastMapDirect {
        let mut map = SFormsFastMapDirect::new(self.factory.clone());
        map.size = self.size;
        for i in (0..3).rev() {
            let arr = &self.elements[i];
            let length = arr.len();
            if length > 0 {
                let arrnext = &self.next[i];
                let mut arrnew: Vec<Option<VersionSet>> = vec![None; length];
                let mut arrnextnew = arrnext.clone();
                arrnextnew.resize(length, 0);
                let mut pointer = 0usize;
                loop {
                    if let Some(set) = &arr[pointer] {
                        arrnew[pointer] = Some(Shared::new(set.borrow().get_copy()));
                    }
                    pointer = arrnext[pointer] as usize;
                    if pointer == 0 {
                        break;
                    }
                }
                map.elements[i] = arrnew;
                map.next[i] = arrnextnew;
            }
        }
        map
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn put(&mut self, key: i32, value: VersionSet) {
        self.put_internal(key, Some(value), false);
    }

    pub fn remove(&mut self, key: i32) {
        self.put_internal(key, None, true);
    }

    pub fn remove_all_fields(&mut self) {
        self.remove_all_in(2);
    }

    pub fn remove_all_stacks(&mut self) {
        self.remove_all_in(1);
    }

    fn remove_all_in(&mut self, index: usize) {
        for i in (0..self.elements[index].len()).rev() {
            if self.elements[index][i].is_some() {
                self.elements[index][i] = None;
                self.size -= 1;
            }
            self.next[index][i] = 0;
        }
    }

    fn put_internal(&mut self, key: i32, value: Option<VersionSet>, remove: bool) {
        let (index, ikey) = segment(key);
        if ikey as i64 >= self.elements[index].len() as i64 {
            if remove {
                return;
            }
            self.ensure_capacity(index, ikey + 1, false);
        }
        let slot = ikey as usize;
        let added = value.is_some();
        let oldval = std::mem::replace(&mut self.elements[index][slot], value);
        let arrnext = &mut self.next[index];
        if oldval.is_none() && added {
            self.size += 1;
            let oldnext = arrnext[slot];
            SFormsFastMapDirect::change_next(arrnext, ikey, oldnext, ikey);
        } else if oldval.is_some() && !added {
            self.size -= 1;
            let newnext = arrnext[slot];
            SFormsFastMapDirect::change_next(arrnext, ikey, ikey, newnext);
        }
    }

    fn change_next(arrnext: &mut [i32], key: i32, oldnext: i32, newnext: i32) {
        let mut i = key - 1;
        while i >= 0 {
            if arrnext[i as usize] == oldnext {
                arrnext[i as usize] = newnext;
            } else {
                break;
            }
            i -= 1;
        }
    }

    pub fn contains_key(&self, key: i32) -> bool {
        self.get(key).is_some()
    }

    /// Java: `get(int)`; the returned set aliases the one in the map.
    pub fn get(&self, key: i32) -> Option<VersionSet> {
        let (index, key) = segment(key);
        let arr = &self.elements[index];
        if (key as i64) < arr.len() as i64 {
            return arr[key as usize].clone();
        }
        None
    }

    /// Java: `get(VarExprent)`.
    pub fn get_var_exprent(&self, var_exprent: VarExprent) -> Option<VersionSet> {
        self.get(var_exprent.get_index())
    }

    pub fn complement(&mut self, map: &SFormsFastMapDirect) {
        for i in (0..3).rev() {
            if self.elements[i].is_empty() {
                continue;
            }
            let lst_extern = &map.elements[i];
            let mut pointer = 0usize;
            loop {
                if let Some(first) = self.elements[i][pointer].clone() {
                    if pointer >= lst_extern.len() {
                        break;
                    }
                    if let Some(second) = &lst_extern[pointer] {
                        with_pair(&first, second, |a, b| a.complement(b));
                        if first.borrow().is_empty() {
                            self.elements[i][pointer] = None;
                            self.size -= 1;
                            let newnext = self.next[i][pointer];
                            SFormsFastMapDirect::change_next(
                                &mut self.next[i],
                                pointer as i32,
                                pointer as i32,
                                newnext,
                            );
                        }
                    }
                }
                pointer = self.next[i][pointer] as usize;
                if pointer == 0 {
                    break;
                }
            }
        }
    }

    pub fn intersection(&mut self, map: &SFormsFastMapDirect) {
        for i in (0..3).rev() {
            if self.elements[i].is_empty() {
                continue;
            }
            let lst_extern = &map.elements[i];
            let mut pointer = 0usize;
            loop {
                if let Some(first) = self.elements[i][pointer].clone() {
                    let second = if pointer < lst_extern.len() {
                        lst_extern[pointer].clone()
                    } else {
                        None
                    };
                    if let Some(second) = &second {
                        with_pair(&first, second, |a, b| a.intersection(b));
                    }
                    if second.is_none() || first.borrow().is_empty() {
                        self.elements[i][pointer] = None;
                        self.size -= 1;
                        let newnext = self.next[i][pointer];
                        SFormsFastMapDirect::change_next(
                            &mut self.next[i],
                            pointer as i32,
                            pointer as i32,
                            newnext,
                        );
                    }
                }
                pointer = self.next[i][pointer] as usize;
                if pointer == 0 {
                    break;
                }
            }
        }
    }

    pub fn union(&mut self, map: &SFormsFastMapDirect) {
        for i in (0..3).rev() {
            let lst_extern = &map.elements[i];
            if lst_extern.is_empty() {
                continue;
            }
            let arrnext_extern = &map.next[i];
            let mut pointer = 0usize;
            loop {
                if pointer >= self.elements[i].len() {
                    self.ensure_capacity(i, lst_extern.len() as i32, true);
                }
                if let Some(second) = &lst_extern[pointer] {
                    match self.elements[i][pointer].clone() {
                        None => {
                            self.elements[i][pointer] =
                                Some(Shared::new(second.borrow().get_copy()));
                            self.size += 1;
                            let oldnext = self.next[i][pointer];
                            SFormsFastMapDirect::change_next(
                                &mut self.next[i],
                                pointer as i32,
                                oldnext,
                                pointer as i32,
                            );
                        }
                        Some(first) => with_pair(&first, second, |a, b| a.union(b)),
                    }
                }
                pointer = arrnext_extern[pointer] as usize;
                if pointer == 0 {
                    break;
                }
            }
        }
    }

    /// `(key, set)` for every live entry: fields, then stack variables, then locals.
    pub fn entry_list(&self) -> Vec<(i32, VersionSet)> {
        let mut list = Vec::new();
        for i in (0..3).rev() {
            for (ikey, ent) in self.elements[i].iter().enumerate() {
                if let Some(ent) = ent {
                    let ikey = ikey as i32;
                    let key = match i {
                        0 => ikey,
                        1 => ikey + VarExprent::STACK_BASE,
                        _ => -ikey,
                    };
                    list.push((key, ent.clone()));
                }
            }
        }
        list
    }

    fn ensure_capacity(&mut self, index: usize, size: i32, exact: bool) {
        let arr_len = self.elements[index].len() as i32;
        let mut minsize = size;
        if !exact {
            minsize = 2 * arr_len / 3 + 1;
            if size > minsize {
                minsize = size;
            }
        }
        if minsize < arr_len {
            crate::jthrow!(ArrayIndexOutOfBoundsException; "arraycopy: last destination index {} out of bounds for length {}", arr_len, minsize);
        }
        self.elements[index].resize(minsize as usize, None);
        self.next[index].resize(minsize as usize, 0);
    }

    pub fn set_current_var(&mut self, var: i32, version: i32) {
        let mut set = self.factory.create_empty_set();
        set.add(version);
        self.put(var, Shared::new(set));
    }

    /// Java: `setCurrentVar(VarExprent)`.
    pub fn set_current_var_exprent(&mut self, var_exprent: VarExprent) {
        self.set_current_var(var_exprent.get_index(), var_exprent.get_version());
    }

    /// Java: `setCurrentVar(VarVersionNode)`.
    pub fn set_current_var_node(&mut self, node: VarVersionNode) {
        let (var, version) = node.with(|d| (d.var, d.version));
        self.set_current_var(var, version);
    }

    /// Java: `setCurrentVar(VarVersionPair)`.
    pub fn set_current_var_pair(&mut self, pair: VarVersionPair) {
        self.set_current_var(pair.var, pair.version);
    }
}

impl std::fmt::Display for SFormsFastMapDirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::from("{");
        for (n, (key, set)) in self.entry_list().into_iter().enumerate() {
            if n > 0 {
                buffer.push_str(", ");
            }
            let plain = set.borrow().to_plain_set();
            buffer.push_str(&format!(
                "{key}={{{}}}",
                java_collection_to_string(plain.iter())
            ));
        }
        buffer.push('}');
        f.write_str(&buffer)
    }
}
