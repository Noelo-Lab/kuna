//! Port of `org/jetbrains/java/decompiler/struct/gen/generics/GenericType.java`.
//!
//! `GenericType` is a view over a [`VarType`] whose data carries the generic
//! fields; see `var_type.rs`.
//!
//! kuna fix `generic_parent_without_signature` (`kuna_fixes`): in
//! `map_gen_vars_to`, an enclosing class without a `Signature` attribute (a
//! non-generic inner class of a generic class) maps as the generic type its
//! signature would declare, `Name<>`, where Vineflower dereferences the
//! missing signature.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::generic_main::GenericMain;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::jstring::from_utf16;
use crate::rt::{JHashMap, JHashSet, JavaStr};
use crate::var_type::{
    get_class, instance_of, str_equals, GenericData, JStringArg, VarType, VarTypeMap,
};

#[derive(Clone)]
pub struct GenericType(pub VarType);

impl Deref for GenericType {
    type Target = VarType;
    fn deref(&self) -> &VarType {
        &self.0
    }
}

impl From<GenericType> for VarType {
    fn from(g: GenericType) -> VarType {
        g.0
    }
}

thread_local! {
    static DUMMY_VAR: GenericType =
        GenericType::new(CodeType::Genvar, 0, "", None, Vec::new(), GenericType::WILDCARD_NO);
}

pub(crate) fn ch(c: u16) -> char {
    char::from_u32(c as u32).unwrap_or('\u{FFFD}')
}

/// `String.substring(begin, end)` over UTF-16 units.
#[track_caller]
pub(crate) fn jsub(s: &[u16], begin: usize, end: usize) -> &[u16] {
    if begin > end || end > s.len() {
        crate::jthrow!(StringIndexOutOfBoundsException; "Range [{}, {}) out of bounds for length {}", begin, end, s.len());
    }
    &s[begin..end]
}

/// `String.indexOf(char)` over UTF-16 units.
pub(crate) fn junits_index_of(s: &[u16], c: char) -> i32 {
    s.iter()
        .position(|&u| u as u32 == c as u32)
        .map_or(-1, |i| i as i32)
}

fn opt_ptr_eq(a: &Option<VarType>, b: &Option<VarType>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.ptr_eq(b),
        _ => false,
    }
}

fn wildcard_of(t: &Option<VarType>) -> i32 {
    match t {
        Some(t) if t.is_generic() => t.as_generic().get_wildcard(),
        _ => GenericType::WILDCARD_NO,
    }
}

impl GenericType {
    pub const WILDCARD_EXTENDS: i32 = 1;
    pub const WILDCARD_SUPER: i32 = 2;
    pub const WILDCARD_UNBOUND: i32 = 3;
    pub const WILDCARD_NO: i32 = 4;

    #[allow(non_snake_case)]
    pub fn DUMMY_VAR() -> GenericType {
        DUMMY_VAR.with(|d| d.clone())
    }

    /// Java: `GenericType(CodeType, int, String, VarType, List<VarType>, int)`; a
    /// null argument list is an empty one.
    pub fn new(
        type_: CodeType,
        array_dim: i32,
        value: impl JStringArg,
        parent: Option<VarType>,
        arguments: Vec<Option<VarType>>,
        wildcard: i32,
    ) -> GenericType {
        GenericType::new_shared(
            type_,
            array_dim,
            value.into_jstr(),
            parent,
            Rc::from(arguments),
            wildcard,
        )
    }

    pub(crate) fn new_shared(
        type_: CodeType,
        array_dim: i32,
        value: Option<Rc<str>>,
        parent: Option<VarType>,
        arguments: Rc<[Option<VarType>]>,
        wildcard: i32,
    ) -> GenericType {
        GenericType(VarType::new_with_generic(
            type_,
            array_dim,
            value,
            VarType::get_family(type_, array_dim),
            VarType::get_stack_size(type_, array_dim),
            Some(GenericData {
                parent,
                arguments,
                wildcard,
            }),
        ))
    }

    fn gd(&self) -> &GenericData {
        self.0
            .generic
            .as_ref()
            .expect("GenericType view over a plain VarType")
    }

    fn shared_arguments(&self) -> Rc<[Option<VarType>]> {
        self.gd().arguments.clone()
    }

    /// Java: `parse(String)`
    pub fn parse(signature: &str) -> VarType {
        GenericType::parse_wildcard(signature, GenericType::WILDCARD_NO)
    }

    /// Java: `parse(String, int)`
    pub fn parse_wildcard(signature: &str, wildcard: i32) -> VarType {
        GenericType::parse_units(&signature.utf16(), wildcard)
    }

    pub(crate) fn parse_units(signature: &[u16], wildcard: i32) -> VarType {
        let mut signature = signature;
        let mut type_ = CodeType::Byte;
        let mut array_dim: i32 = 0;
        let mut value: Option<String> = None;
        let mut params: Option<Vec<Option<VarType>>> = None;
        let mut parent: Option<VarType> = None;

        let mut index = 0;
        while index < signature.len() {
            match ch(signature[index]) {
                '[' => array_dim = array_dim.wrapping_add(1),
                'T' => {
                    type_ = CodeType::Genvar;
                    value = Some(from_utf16(jsub(signature, index + 1, signature.len() - 1)));
                    break;
                }
                'L' => {
                    type_ = CodeType::Object;
                    signature = jsub(signature, index + 1, signature.len() - 1);
                    let mut cl = GenericType::next_class_signature(signature);

                    if cl.len() == signature.len() {
                        let arg_start = junits_index_of(cl, '<');
                        if arg_start >= 0 {
                            let arg_start = arg_start as usize;
                            value = Some(from_utf16(&cl[..arg_start]));
                            params = Some(GenericType::parse_arguments_list(jsub(
                                cl,
                                arg_start + 1,
                                cl.len() - 1,
                            )));
                        } else {
                            value = Some(from_utf16(cl));
                        }
                    } else {
                        let mut name_buff = String::new();
                        while !signature.is_empty() {
                            let mut name = cl;
                            let mut args: Option<&[u16]> = None;

                            let arg_start = junits_index_of(cl, '<');
                            if arg_start >= 0 {
                                let arg_start = arg_start as usize;
                                name = &cl[..arg_start];
                                args = Some(jsub(cl, arg_start + 1, cl.len() - 1));
                            }

                            if !name_buff.is_empty() {
                                name_buff.push('$');
                            }
                            name_buff.push_str(&from_utf16(name));

                            value = Some(name_buff.clone());
                            params = args.map(GenericType::parse_arguments_list);

                            if cl.len() == signature.len() {
                                break;
                            } else {
                                if parent.is_none() && params.is_none() {
                                    let v = value.as_deref().unwrap_or("null");
                                    parent = Some(GenericType::parse(&format!("L{};", v)));
                                } else {
                                    parent = Some(
                                        GenericType::new(
                                            CodeType::Object,
                                            0,
                                            value.clone(),
                                            parent.take(),
                                            params.clone().unwrap_or_default(),
                                            wildcard,
                                        )
                                        .0,
                                    );
                                }

                                signature = jsub(signature, cl.len() + 1, signature.len());
                            }
                            cl = GenericType::next_class_signature(signature);
                        }
                    }
                    break;
                }
                _ => {
                    value = Some(from_utf16(jsub(signature, index, index + 1)));
                    type_ = VarType::get_type_unit(signature[index]);
                }
            }
            index += 1;
        }

        let value: Option<Rc<str>> = value.map(Rc::from);
        if type_ == CodeType::Genvar {
            GenericType::new_shared(
                type_,
                array_dim,
                value,
                None,
                Rc::from(Vec::new()),
                wildcard,
            )
            .0
        } else if type_ == CodeType::Object {
            if parent.is_none() && params.is_none() && wildcard == GenericType::WILDCARD_NO {
                VarType::new_value(type_, array_dim, value)
            } else {
                GenericType::new(
                    type_,
                    array_dim,
                    value,
                    parent,
                    params.unwrap_or_default(),
                    wildcard,
                )
                .0
            }
        } else {
            VarType::new_value(type_, array_dim, value)
        }
    }

    fn next_class_signature(value: &[u16]) -> &[u16] {
        let mut counter: i32 = 0;
        let mut index = 0;
        while index < value.len() {
            match ch(value[index]) {
                '<' => counter += 1,
                '>' => counter -= 1,
                '.' => {
                    if counter == 0 {
                        break;
                    }
                }
                _ => {}
            }
            index += 1;
        }
        &value[..index]
    }

    fn parse_arguments_list(value: &[u16]) -> Vec<Option<VarType>> {
        let mut value = value;
        let mut args = Vec::new();

        while !value.is_empty() {
            let mut type_str = GenericType::next_type_units(value);
            let len = type_str.len();
            let mut wildcard = GenericType::WILDCARD_NO;

            match ch(type_str[0]) {
                '*' => wildcard = GenericType::WILDCARD_UNBOUND,
                '+' => wildcard = GenericType::WILDCARD_EXTENDS,
                '-' => wildcard = GenericType::WILDCARD_SUPER,
                _ => {}
            }

            if wildcard != GenericType::WILDCARD_NO {
                type_str = &type_str[1..];
            }

            args.push(if type_str.is_empty() {
                None
            } else {
                Some(GenericType::parse_units(type_str, wildcard))
            });

            value = &value[len..];
        }

        args
    }

    pub fn get_next_type(value: &str) -> String {
        from_utf16(GenericType::next_type_units(&value.utf16()))
    }

    pub(crate) fn next_type_units(value: &[u16]) -> &[u16] {
        let mut counter: i32 = 0;
        let mut index = 0;
        let mut cont_mode = false;

        while index < value.len() {
            match ch(value[index]) {
                '*' => {
                    if !cont_mode {
                        break;
                    }
                }
                'L' | 'T' => {
                    if !cont_mode {
                        cont_mode = true;
                    }
                }
                '[' | '+' | '-' => {}
                '<' => counter += 1,
                '>' => counter -= 1,
                ';' => {
                    if counter == 0 {
                        break;
                    }
                }
                _ => {
                    if !cont_mode {
                        break;
                    }
                }
            }
            index += 1;
        }

        jsub(value, 0, index + 1)
    }

    pub fn with_wildcard(var: &VarType, wildcard: i32) -> VarType {
        if var.is_generic() {
            let gen_var = var.as_generic();
            return GenericType::new_shared(
                gen_var.type_,
                gen_var.array_dim,
                gen_var.value.clone(),
                gen_var.get_parent(),
                gen_var.shared_arguments(),
                wildcard,
            )
            .0;
        }
        GenericType::new_shared(
            var.type_,
            var.array_dim,
            var.value.clone(),
            None,
            Rc::from(Vec::new()),
            wildcard,
        )
        .0
    }

    pub fn decrease_array_dim(&self) -> GenericType {
        GenericType::new_shared(
            self.type_,
            self.array_dim - 1,
            self.value.clone(),
            self.get_parent(),
            self.shared_arguments(),
            self.get_wildcard(),
        )
    }

    pub fn resize_array_dim(&self, new_array_dim: i32) -> VarType {
        GenericType::new_shared(
            self.type_,
            new_array_dim,
            self.value.clone(),
            self.get_parent(),
            self.shared_arguments(),
            self.get_wildcard(),
        )
        .0
    }

    pub fn get_parent(&self) -> Option<VarType> {
        self.gd().parent.clone()
    }

    pub fn get_arguments(&self) -> &[Option<VarType>] {
        &self.gd().arguments
    }

    pub fn is_generic(&self) -> bool {
        let gd = self.gd();
        self.type_ == CodeType::Genvar
            || !gd.arguments.is_empty()
            || gd.parent.is_some()
            || gd.wildcard != GenericType::WILDCARD_NO
    }

    pub fn get_wildcard(&self) -> i32 {
        self.gd().wildcard
    }

    pub fn get_cast_name(&self) -> String {
        let value = self.value_str();
        let cls_name = match self.get_parent() {
            None => DecompilerContext::get_import_collector()
                .get_short_name(&value.replace('/', "."))
                .unwrap_or_else(|| "null".to_string()),
            Some(parent) if parent.is_generic() => {
                let parent_len = parent.value_str().jlen();
                format!(
                    "{}.{}",
                    parent.as_generic().get_cast_name(),
                    value.jsubstring_from(parent_len + 1)
                )
            }
            Some(parent) => {
                let tail = match value.rfind('.') {
                    Some(p) => &value[p + 1..],
                    None => value,
                };
                format!(
                    "{}.{}",
                    DecompilerContext::get_import_collector()
                        .get_short_name(&parent.value_str().replace('/', "."))
                        .as_deref()
                        .unwrap_or("null"),
                    tail
                )
            }
        };
        cls_name + &self.get_type_arguments()
    }

    fn get_type_arguments(&self) -> String {
        let mut buffer = String::new();
        let arguments = self.get_arguments();
        if !arguments.is_empty() {
            buffer.push('<');

            for (i, par) in arguments.iter().enumerate() {
                if i > 0 {
                    buffer.push_str(", ");
                }

                match par {
                    None => buffer.push('?'),
                    Some(par) if par.is_generic() => {
                        let gen = par.as_generic();
                        match gen.get_wildcard() {
                            GenericType::WILDCARD_EXTENDS => buffer.push_str("? extends "),
                            GenericType::WILDCARD_SUPER => buffer.push_str("? super "),
                            _ => {}
                        }
                        buffer.push_str(&GenericMain::get_generic_cast_type_name(&gen));
                    }
                    Some(par) => buffer.push_str(&ExprProcessor::get_cast_type_name(par)),
                }
            }

            buffer.push('>');
        }
        buffer
    }

    /// `GenericType.toString()`.
    pub fn to_java_string(&self) -> String {
        let mut buf = String::new();
        match self.get_wildcard() {
            GenericType::WILDCARD_EXTENDS => buf.push_str("? extends "),
            GenericType::WILDCARD_SUPER => buf.push_str("? super "),
            _ => {}
        }
        buf.push_str(&self.0.to_string_base());
        buf.push_str(&self.get_type_arguments());
        buf
    }

    pub fn remap<M: VarTypeMap + ?Sized>(&self, map: &M) -> Option<VarType> {
        let main = self.0.remap_base(map);
        if !main.as_ref().is_some_and(|m| m.ptr_eq(&self.0)) {
            let wild = wildcard_of(&main);
            if let Some(m) = &main {
                if self.get_wildcard() != GenericType::WILDCARD_NO && wild != self.get_wildcard() {
                    return Some(GenericType::with_wildcard(m, self.get_wildcard()));
                }
            }
            return main;
        }

        let mut changed = false;
        let mut parent = self.get_parent();

        if parent.as_ref().is_some_and(|p| map.vt_contains_key(p)) {
            parent = map.vt_get(parent.as_ref().unwrap());
            changed = true;
        }

        let mut new_args: Vec<Option<VarType>> = Vec::new();
        for arg in self.get_arguments() {
            let mut new_arg = None;
            if let Some(a) = arg {
                new_arg = a.remap(map);
            }

            if !opt_ptr_eq(&new_arg, arg) {
                new_args.push(new_arg);
                changed = true;
            } else {
                new_args.push(arg.clone());
            }
        }

        if changed {
            return Some(
                GenericType::new(
                    self.type_,
                    self.array_dim,
                    self.value.clone(),
                    parent,
                    new_args,
                    self.get_wildcard(),
                )
                .0,
            );
        }
        Some(self.0.clone())
    }

    pub fn equals_exact(&self, o: &VarType) -> bool {
        if o.ptr_eq(&self.0) {
            return true;
        }

        let Some(gt) = o.try_as_generic() else {
            return self.get_parent().is_none()
                && self.get_arguments().is_empty()
                && self.get_wildcard() == GenericType::WILDCARD_NO
                && o.equals(&self.0);
        };

        if self.type_ != gt.type_
            || self.array_dim != gt.array_dim
            || self.get_wildcard() != gt.get_wildcard()
            || self.value != gt.value
        {
            return false;
        }

        self.arguments_equal(&gt)
    }

    pub fn is_type_unfinished(&self) -> bool {
        for gv in self.get_all_generic_vars() {
            if gv.value_str().is_empty() {
                return true;
            }
        }
        false
    }

    pub fn arguments_equal(&self, gt: &GenericType) -> bool {
        let args = self.get_arguments();
        let other = gt.get_arguments();
        if args.len() != other.len() {
            return false;
        }

        for i in 0..args.len() {
            let (t, o) = match (&args[i], &other[i]) {
                (None, None) => continue,
                (Some(t), Some(o)) => (t, o),
                _ => return false,
            };

            if t.is_generic() != o.is_generic() || !t.equals(o) {
                return false;
            }

            if t.is_generic() && !t.as_generic().equals_exact(o) {
                return false;
            }
        }
        true
    }

    pub fn is_assignable(
        from: &VarType,
        to: &VarType,
        named: &JHashMap<VarType, Vec<VarType>>,
    ) -> bool {
        let mut from = from.clone();
        if from.array_dim != to.array_dim {
            return false;
        }

        if from.type_ == CodeType::Object && from.type_ == to.type_ {
            if !instance_of(&from.value, &to.value) {
                return false;
            }
        } else if !from.equals(to) {
            if from.type_ == CodeType::Genvar && from.type_ != to.type_ && named.contains_key(&from)
            {
                return named.get(&from).unwrap().iter().any(|bound| {
                    let mut bound = bound.clone();
                    if to.is_generic() && !str_equals(&bound.value, &to.value) {
                        let new = GenericType::get_generic_super_type(&bound, to);
                        let new = new.expect("NullPointerException");
                        if !bound.ptr_eq(&new) && new.is_generic() {
                            bound = new;
                        }
                    }
                    GenericType::are_arguments_assignable(&bound, to, named)
                });
            } else {
                return false;
            }
        }

        if to.is_generic() && !str_equals(&from.value, &to.value) {
            let new = GenericType::get_generic_super_type(&from, to);
            let new = new.expect("NullPointerException");
            if !from.ptr_eq(&new) && new.is_generic() {
                from = new;
            }
        }

        GenericType::are_arguments_assignable(&from, to, named)
    }

    #[allow(clippy::if_same_then_else)]
    pub fn are_arguments_assignable(
        from: &VarType,
        to: &VarType,
        named: &JHashMap<VarType, Vec<VarType>>,
    ) -> bool {
        if from.is_generic() && to.is_generic() {
            let gen_from = from.as_generic();
            let gen_to = to.as_generic();
            let from_args = gen_from.get_arguments();
            let to_args = gen_to.get_arguments();

            if from_args.len() != to_args.len() {
                return from_args.is_empty() || to_args.is_empty();
            }

            for i in 0..from_args.len() {
                let Some(t) = &to_args[i] else {
                    continue;
                };

                let t_wild = if t.is_generic() {
                    t.as_generic().get_wildcard()
                } else {
                    GenericType::WILDCARD_NO
                };

                let f = match &from_args[i] {
                    None => {
                        let cls = get_class(&gen_from.value).expect("NullPointerException");
                        let sig = cls.get_signature().expect("NullPointerException");
                        let bounds = sig.fbounds()[i][0].clone();
                        if VarType::VARTYPE_OBJECT().equals(&bounds) {
                            return false;
                        }
                        bounds
                    }
                    Some(f)
                        if f.type_ == CodeType::Genvar
                            && f.type_ != t.type_
                            && named.contains_key(f) =>
                    {
                        named.get(f).unwrap()[0].clone()
                    }
                    Some(f) => f.clone(),
                };

                let f_wild = if f.is_generic() {
                    f.as_generic().get_wildcard()
                } else {
                    GenericType::WILDCARD_NO
                };

                if t_wild == GenericType::WILDCARD_EXTENDS {
                    if f_wild == GenericType::WILDCARD_SUPER || !instance_of(&f.value, &t.value) {
                        return false;
                    }
                } else if t_wild == GenericType::WILDCARD_SUPER {
                    if f_wild == GenericType::WILDCARD_EXTENDS || !instance_of(&t.value, &f.value) {
                        return false;
                    }
                } else if t_wild == GenericType::WILDCARD_NO
                    && f_wild != t_wild
                    && gen_from.get_wildcard() == gen_to.get_wildcard()
                {
                    return false;
                } else if !str_equals(&f.value, &t.value) {
                    return false;
                }

                if !GenericType::are_arguments_assignable(&f, t, named) {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_all_generic_vars(&self) -> Vec<GenericType> {
        let mut ret = Vec::new();

        if self.type_ == CodeType::Genvar {
            ret.push(self.resize_array_dim(0).as_generic());
            return ret;
        }

        for arg in self.get_arguments().iter().flatten() {
            if arg.is_generic() {
                ret.extend(arg.as_generic().get_all_generic_vars());
            }
        }
        ret
    }

    pub fn clean_lowered_generic_types(
        temp_map: &mut JHashMap<VarType, Option<VarType>>,
        type1: &GenericType,
        type2: &GenericType,
        can_clean: Option<&JHashSet<VarType>>,
    ) {
        let args1 = type1.get_arguments();
        let args2 = type2.get_arguments();
        if args1.len() == args2.len() {
            for k in 0..args1.len() {
                let arg1 = &args1[k];
                let arg2 = &args2[k];

                if let Some(a1) = arg1 {
                    if can_clean.is_none_or(|c| c.contains(a1)) && arg2.is_none() {
                        temp_map.remove(a1);
                    }
                }
            }
        }
    }

    pub fn find_base_type(&self) -> Option<GenericType> {
        let cl = get_class(&self.value)?;
        let sig = cl.get_signature()?;
        Some(sig.generic_type())
    }

    pub fn map_gen_vars_to(
        &self,
        other: &GenericType,
        map: &mut JHashMap<VarType, Option<VarType>>,
    ) {
        let args = self.get_arguments();
        let other_args = other.get_arguments();
        if args.len() != other_args.len() {
            return;
        }

        for i in 0..args.len() {
            let Some(mut this_arg) = args[i].clone() else {
                continue;
            };
            let mut other_arg = other_args[i].clone();

            if GenericType::DUMMY_VAR().equals_opt(other_arg.as_ref()) {
                continue;
            }

            if this_arg.type_ == CodeType::Genvar {
                let t_wild = this_arg.as_generic().get_wildcard();
                let o_wild = wildcard_of(&other_arg);

                if t_wild == o_wild && t_wild != GenericType::WILDCARD_NO {
                    this_arg = GenericType::with_wildcard(&this_arg, GenericType::WILDCARD_NO);
                    other_arg = Some(GenericType::with_wildcard(
                        other_arg.as_ref().expect("NullPointerException"),
                        GenericType::WILDCARD_NO,
                    ));
                }

                match other_arg {
                    None if this_arg.array_dim == 0 => {
                        if !map.contains_key(&this_arg) {
                            map.put(this_arg, None);
                        }
                    }
                    Some(mut oa) if this_arg.array_dim <= oa.array_dim => {
                        if this_arg.array_dim > 0 {
                            oa = oa.resize_array_dim(oa.array_dim - this_arg.array_dim);
                            this_arg = this_arg.resize_array_dim(0);
                        }
                        if !map.contains_key(&this_arg) {
                            map.put(this_arg, Some(oa));
                        } else {
                            let curr = map.get(&this_arg).cloned().flatten();
                            let c_wild = wildcard_of(&curr);
                            if o_wild != c_wild {
                                map.put(
                                    this_arg,
                                    Some(GenericType::with_wildcard(&oa, GenericType::WILDCARD_NO)),
                                );
                            }
                        }
                    }
                    _ => {}
                }
            } else if this_arg.is_generic() && other_arg.as_ref().is_some_and(|o| o.is_generic()) {
                this_arg
                    .as_generic()
                    .map_gen_vars_to(&other_arg.unwrap().as_generic(), map);
            }
        }

        if let Some(other_parent) = other.get_parent().filter(|p| p.is_generic()) {
            let mut parent = self
                .get_parent()
                .filter(|p| p.is_generic())
                .map(|p| p.as_generic());
            if parent.is_none() {
                if let Some(cls) = get_class(&other_parent.value) {
                    if kuna_fixes::enabled(KunaFix::GenericParentWithoutSignature) {
                        parent = Some(match cls.get_signature() {
                            Some(sig) => sig.generic_type(),
                            None => GenericType::parse(&format!("L{}<>;", cls.qualified_name()))
                                .as_generic(),
                        });
                    } else {
                        parent = Some(
                            cls.get_signature()
                                .expect("NullPointerException")
                                .generic_type(),
                        );
                    }
                }
            }

            if let Some(parent) = parent {
                parent.map_gen_vars_to(&other_parent.as_generic(), map);
            }
        }
    }

    pub fn has_unknown_generic_type(&self, named_generics: &JHashSet<VarType>) -> bool {
        if self.type_ == CodeType::Genvar {
            return !named_generics.contains(&self.resize_array_dim(0));
        }

        for arg in self.get_arguments().iter().flatten() {
            if arg.is_generic() && arg.as_generic().has_unknown_generic_type(named_generics) {
                return true;
            }
        }
        false
    }

    pub fn get_generic_super_type(derived_type: &VarType, super_type: &VarType) -> Option<VarType> {
        let dcls = get_class(&derived_type.value);
        let scls = get_class(&super_type.value);

        if let (Some(dcls), Some(scls)) = (dcls, scls) {
            let hierarchy = dcls.get_all_generics();
            let scls_name = scls.qualified_name();

            if hierarchy.contains_key(&scls_name) {
                if let Some(scls_sig) = scls.get_signature() {
                    let mut temp_map: JHashMap<VarType, Option<VarType>> = JHashMap::new();

                    if derived_type.is_generic() {
                        if let Some(dsig) = dcls.get_signature() {
                            let dgen = dsig.generic_type();
                            let derived = derived_type.as_generic();
                            dgen.map_gen_vars_to(&derived, &mut temp_map);
                            GenericType::clean_lowered_generic_types(
                                &mut temp_map,
                                &dgen,
                                &derived,
                                None,
                            );
                            let fparameters = dsig.fparameters();
                            let fbounds = dsig.fbounds();
                            for (i, fparameter) in fparameters.iter().enumerate() {
                                let param = GenericType::parse(&format!("T{};", fparameter));
                                if temp_map.get(&param).cloned().flatten().is_none() {
                                    let bounds = &fbounds[i];
                                    if !bounds.is_empty() {
                                        let replacement = bounds[0].remap(&temp_map);
                                        if !VarType::VARTYPE_OBJECT()
                                            .equals_opt(replacement.as_ref())
                                        {
                                            temp_map.put(param, replacement);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return scls_sig
                        .generic_type()
                        .remap(&**hierarchy.get(&scls_name).unwrap())
                        .expect("NullPointerException")
                        .remap(&temp_map);
                }
            }
        }
        Some(derived_type.clone())
    }
}

impl std::fmt::Display for GenericType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_java_string())
    }
}

impl std::fmt::Debug for GenericType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
