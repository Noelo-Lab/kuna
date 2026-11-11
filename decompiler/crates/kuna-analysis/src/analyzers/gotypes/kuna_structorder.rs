//! Complete named Go value fields before their enclosing struct layouts.

use std::collections::HashMap;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Visiting,
    Complete,
    Refused,
}

enum Frame {
    Enter(GoType),
    Leave(GoType, Vec<u64>),
}

fn dependencies(img: &GoImage, meta: &TypeMeta, t: &GoType) -> Option<Vec<GoType>> {
    let fields = plausible_slice(img, t.vma + img.type_base() + img.word_size, 4096);
    let Some((ptr, len)) = fields else {
        return (word(img, t.vma + img.type_base() + 2 * img.word_size) == Some(0)
            && word(img, t.vma + img.type_base() + 3 * img.word_size) == Some(0))
            .then(Vec::new);
    };
    let mut result = Vec::new();
    for i in 0..len {
        let vma = word(img, ptr + i * 3 * img.word_size + img.word_size)?;
        if vma < meta.types || vma.checked_add(img.type_base())? > meta.etypes {
            return None;
        }
        let child = read_type(img, meta.types, meta.etypes, (vma - meta.types) as u32)?;
        if child.kind == KIND_STRUCT
            && child.size != 0
            && child.tflag & TFLAG_NAMED != 0
            && child.name.contains('.')
        {
            if !name_ok(&child.name) {
                return None;
            }
            result.push(child);
        }
    }
    Some(result)
}

pub(super) fn complete_order(
    tf: &dyn TypeFactory,
    img: &GoImage,
    meta: &TypeMeta,
    roots: Vec<GoType>,
) -> Vec<GoType> {
    let mut work: Vec<_> = roots.into_iter().rev().map(Frame::Enter).collect();
    let mut state = HashMap::new();
    let mut result = Vec::new();
    while let Some(frame) = work.pop() {
        if state.len() > GOTYPES_MAX_TYPELINKS || work.len() > 2 * GOTYPES_MAX_TYPELINKS {
            break;
        }
        match frame {
            Frame::Enter(t) => {
                match state.get(&t.vma) {
                    Some(State::Visiting) => {
                        state.insert(t.vma, State::Refused);
                        continue;
                    }
                    Some(_) => continue,
                    None => {}
                }
                if tf.find_by_name(&t.name).ok().flatten().is_some_and(|ct| {
                    ct.get_metatype() == type_metatype::TYPE_STRUCT && ct.get_size() != 0
                }) {
                    state.insert(t.vma, State::Complete);
                    continue;
                }
                let Some(children) = dependencies(img, meta, &t) else {
                    state.insert(t.vma, State::Refused);
                    continue;
                };
                state.insert(t.vma, State::Visiting);
                let ids = children.iter().map(|child| child.vma).collect();
                work.push(Frame::Leave(t, ids));
                work.extend(children.into_iter().rev().map(Frame::Enter));
            }
            Frame::Leave(t, children) => {
                if state.get(&t.vma) == Some(&State::Visiting)
                    && children.iter().all(|child| state.get(child) == Some(&State::Complete))
                {
                    state.insert(t.vma, State::Complete);
                    result.push(t);
                } else {
                    state.insert(t.vma, State::Refused);
                }
            }
        }
    }
    result
}
