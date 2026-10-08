//! C qualifiers belong to a particular type layer, independently of its shape.

use std::rc::Rc;

use crate::dtype::Datatype;

pub const CONST: u8 = 1;
pub const RESTRICT: u8 = 2;
pub const VOLATILE: u8 = 4;
pub const ELEM_QUALIFIED: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("qualified", 4094);

impl Datatype {
    pub fn c_qualifiers(&self) -> u8 {
        self.qualifiers
    }

    pub fn qualified_base(&self) -> Option<&Rc<Datatype>> {
        self.qualified_base.as_ref()
    }
}

/// Keep the shape visible to inference, but retain the qualified layer for spelling.
pub(crate) fn qualified_type(base: Rc<Datatype>, qualifiers: u8) -> Rc<Datatype> {
    if qualifiers == 0 {
        return base;
    }
    let qualifiers = qualifiers | base.c_qualifiers();
    let base = base.qualified_base().cloned().unwrap_or(base);
    let mut ty = (*base).clone();
    ty.id = 0;
    ty.name.clear();
    ty.display_name.clear();
    ty.flags &= !crate::dtype::flags::coretype;
    ty.typedef_imm = None;
    ty.qualifiers = qualifiers;
    ty.qualified_base = Some(base);
    Rc::new(ty)
}

pub fn spelling(qualifiers: u8) -> String {
    if qualifiers == 0 {
        return String::new();
    }
    [
        (CONST, "const"),
        (VOLATILE, "volatile"),
        (RESTRICT, "restrict"),
    ]
    .into_iter()
    .filter_map(|(bit, text)| (qualifiers & bit != 0).then_some(text))
    .collect::<Vec<_>>()
    .join(" ")
}

/// Qualifiers on a typedef are inherited without expanding its printed name.
pub(crate) fn effective_qualifiers(mut ty: &Datatype) -> u8 {
    let mut qualifiers = 0;
    for _ in 0..64 {
        qualifiers |= ty.c_qualifiers();
        match ty.qualified_base().or_else(|| ty.get_typedef()) {
            Some(base) => ty = base,
            None => break,
        }
    }
    qualifiers
}

pub(crate) fn unqualified_shape(mut ty: &Datatype) -> &Datatype {
    for _ in 0..64 {
        if effective_qualifiers(ty) == 0 {
            break;
        }
        match ty.qualified_base().or_else(|| ty.get_typedef()) {
            Some(base) => ty = base,
            None => break,
        }
    }
    ty
}

pub fn is_volatile(ty: &Datatype) -> bool {
    effective_qualifiers(ty) & VOLATILE != 0
}

/// Loading a value drops its outer qualifiers, including those hidden by aliases.
pub fn unqualified_value(mut ty: Rc<Datatype>) -> Rc<Datatype> {
    for _ in 0..64 {
        if effective_qualifiers(&ty) == 0 {
            break;
        }
        match ty.qualified_base().or_else(|| ty.get_typedef()).cloned() {
            Some(base) => ty = base,
            None => break,
        }
    }
    ty
}

/// Recover a declared pointer through address expressions before type inference runs.
pub(crate) fn address_type(
    data: &crate::funcdata::Funcdata,
    id: crate::context::VarnodeId,
    budget: &mut u32,
) -> Option<Rc<Datatype>> {
    use crate::dtype::type_metatype::TYPE_PTR;
    use kuna_num::opcodes::OpCode;

    let node = data.vbank().get(id)?;
    let ty = node.get_type();
    let known = (ty.get_metatype() == TYPE_PTR).then(|| Rc::clone(ty));
    if node.is_type_lock() || node.get_def().is_none() {
        return known;
    }
    if *budget == 0 {
        return known;
    }
    *budget -= 1;
    let op = data.obank().get(node.get_def()?)?;
    let ty = match op.code() {
        OpCode::CPUI_COPY
        | OpCode::CPUI_CAST
        | OpCode::CPUI_PTRADD
        | OpCode::CPUI_PTRSUB
        | OpCode::CPUI_INT_SUB => address_type(data, op.get_in(0)?, budget),
        OpCode::CPUI_INT_ADD => address_type(data, op.get_in(0)?, budget)
            .or_else(|| address_type(data, op.get_in(1)?, budget)),
        OpCode::CPUI_LOAD => address_type(data, op.get_in(1)?, budget)
            .and_then(|ty| ty.get_ptr_to())
            .map(unqualified_value)
            .filter(|ty| ty.get_metatype() == TYPE_PTR),
        _ => None,
    };
    ty.or(known)
}

#[cfg(test)]
mod tests;
