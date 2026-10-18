//! Whole mapped objects behind named global references actually printed in C.
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

#[derive(Debug, Default)]
pub struct References {
    objects: BTreeMap<(u64, String), Rc<Datatype>>,
}

impl References {
    /// Retain the whole mapped declaration, never the type of a partial read.
    /// Split mappings and uncertain geometry do not prove a standalone object.
    pub fn record(&mut self, fd: &Funcdata, arch: &Architecture, address: &Address, name: &str) {
        if !plain_identifier(name) {
            return;
        }
        let Some(data) = arch.manage().get_default_data_space() else {
            return;
        };
        if address.get_space().map(|s| s.get_index()) != Some(data.get_index()) {
            return;
        }
        let context = fd.get_arch();
        let Some(container) = context.query_container_global(address, 1, &Address::new_invalid())
        else {
            return;
        };
        if container.symbol_offset != 0 {
            return;
        }
        let Some(ty) = container.symbol_type else {
            return;
        };
        if ty.get_size() <= 0 || ty.get_metatype() == type_metatype::TYPE_CODE {
            return;
        }
        let Some((base, extent)) = context.global_symbol_extent(address) else {
            return;
        };
        if base != container.entry_addr.get_offset() || extent != ty.get_size() {
            return;
        }
        let Some((mapped_name, _, _)) =
            context.name_for_global_varnode(address, 1, &Address::new_invalid())
        else {
            return;
        };
        if name != mapped_name {
            return;
        }
        self.objects.entry((base, name.to_string())).or_insert(ty);
    }

    /// A pointer into a complete named object stays relative to that object.
    pub fn void_address(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        offset: u64,
    ) -> Option<String> {
        let data = arch.manage().get_default_data_space()?;
        if data.get_word_size() != 1 || offset == 0 {
            return None;
        }
        let address = Address::new(data.clone(), offset);
        let context = fd.get_arch();
        let container = context.query_container_global(&address, 1, &Address::new_invalid())?;
        let (base, extent) = context.global_symbol_extent(&address)?;
        if container.symbol_offset != 0
            || base != container.entry_addr.get_offset()
            || extent <= 0
            || base.checked_add(extent as u64)? <= offset
        {
            return None;
        }
        let base_address = Address::new(data.clone(), base);
        let (name, _, _, scopes) =
            context.name_for_global_varnode_scoped(&base_address, 1, &Address::new_invalid())?;
        if !scopes.is_empty()
            || !plain_identifier(&name)
            || fd
                .get_scope_local()
                .is_some_and(|scope| scope.local_name_used(&name))
        {
            return None;
        }
        self.record(fd, arch, &base_address, &name);
        if !self.objects.contains_key(&(base, name.clone())) {
            return None;
        }
        let delta = offset.checked_sub(base)?;
        Some(if delta == 0 {
            format!("(&{name})")
        } else {
            format!("((unsigned char *)&{name} + 0x{delta:x})")
        })
    }

    pub fn objects(&self) -> impl Iterator<Item = (u64, &str, &Rc<Datatype>)> {
        self.objects
            .iter()
            .map(|((address, name), ty)| (*address, name.as_str(), ty))
    }
}

fn plain_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        && !"auto break case char const continue default do double else enum extern float for goto if inline int long register restrict return short signed sizeof static struct switch typedef union unsigned void volatile while _Alignas _Alignof _Atomic _Bool _Complex _Generic _Imaginary _Noreturn _Static_assert _Thread_local bool true false asm __asm__ typeof __typeof__"
            .split_ascii_whitespace().any(|keyword| keyword == name)
}
