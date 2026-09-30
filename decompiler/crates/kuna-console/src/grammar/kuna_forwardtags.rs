//! (kuna) A `struct`/`union` tag named before it is defined.
//!
//! Upstream's `oldStruct`/`oldUnion` accept only a tag the type factory already
//! holds, so a record could not mention itself (`struct Node { struct Node
//! *next; }`), two records could not mention each other, and `struct Node;` /
//! `typedef struct Node Node;` could not declare a tag ahead of its body. C
//! declares such a tag as an incomplete type, and so does this parser:
//!
//! * inside a member list, any unknown tag is declared (the enclosing record's
//!   own tag, a sibling defined later, an opaque handle);
//! * outside every member list, only [`super::parse_c`] may declare one, and
//!   only when the declaration is the tag alone (`struct Node;`, `struct Node
//!   Node;`); any other mention of an unknown tag keeps upstream's error.
//!
//! The incomplete stub is the one `newStruct`/`newUnion` completes when the body
//! arrives, so the tag is a single type throughout, and the factory records it as
//! declared ahead so a pointer built to the stub reads the completed record's
//! members (`kuna_decomp::kuna_completedrecord`). A member that holds a
//! declared-but-incomplete record by value (itself included) is rejected, as C
//! rejects it, and every stub this parse declared is withdrawn if the parse fails.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_decomp::dtype::{type_metatype, Datatype};

use super::{flags, CParse, TypeDeclarator};

/// One tag a parse declared incomplete.
pub(super) struct ForwardTag {
    name: String,
    /// Named outside every member list, so [`CParse::settle_loose_tags`] decides
    /// whether the declaration may keep it.
    loose: bool,
    /// Upstream's unknown-tag error, located where the tag was read.
    error: String,
}

impl CParse<'_> {
    /// May an unknown tag read at the current token be declared?
    pub(super) fn may_declare_tag(&self) -> bool {
        self.member_depth > 0 || self.loose_tags_ok
    }

    /// Declare `ident` as an incomplete struct (or union) and remember it.
    pub(super) fn declare_tag(&mut self, ident: &str, union: bool) -> KunaResult<Rc<Datatype>> {
        let what = if union { "union" } else { "struct" };
        let error = self.located(&format!("Identifier does not represent a {what} as required"));
        let res = if union {
            self.factory.get_type_union(ident)?
        } else {
            self.factory.get_type_struct(ident)?
        };
        self.factory.kuna_note_declared_ahead(&res);
        self.forward_tags.push(ForwardTag {
            name: ident.to_string(),
            loose: self.member_depth == 0,
            error,
        });
        Ok(res)
    }

    /// Does a member of `record` of type `field_type` hold an incomplete record
    /// by value: `record` itself, or a tag this parse declared and has not yet
    /// completed?  Arrays are looked through.
    pub(super) fn is_incomplete_member(&self, record: &Rc<Datatype>, field_type: &Rc<Datatype>) -> bool {
        let mut ct = Rc::clone(field_type);
        while let Some(elem) = ct.get_array_base() {
            ct = elem;
        }
        let meta = ct.get_metatype();
        if meta != type_metatype::TYPE_STRUCT && meta != type_metatype::TYPE_UNION {
            return false;
        }
        ct.is_incomplete()
            && (Rc::ptr_eq(&ct, record)
                || self.forward_tags.iter().any(|t| t.name == ct.get_name()))
    }

    /// Record C's "incomplete type" error for member `decl`.
    pub(super) fn reject_incomplete_member(&mut self, decl: &TypeDeclarator, field_type: &Datatype) {
        let mut ct = field_type.get_array_base();
        let mut name = field_type.get_name().to_string();
        while let Some(elem) = ct {
            name = elem.get_name().to_string();
            ct = elem.get_array_base();
        }
        self.set_error(&format!(
            "Member {} has incomplete type {}",
            decl.get_identifier(),
            name
        ));
    }

    /// Keep a tag declared outside every member list only when the declaration
    /// is that tag alone, `struct Node;` or `struct Node Node;`; otherwise the
    /// parse fails with upstream's unknown-tag error.
    pub(super) fn settle_loose_tags(&mut self, decls: &[TypeDeclarator]) -> KunaResult<()> {
        let Some(tag) = self.forward_tags.iter().find(|t| t.loose) else {
            return Ok(());
        };
        let alone = decls.len() == 1
            && self.forward_tags.iter().filter(|t| t.loose).count() == 1
            && declares_only(&decls[0], &tag.name);
        if alone {
            return Ok(());
        }
        let error = tag.error.clone();
        self.discard_forward_tags();
        Err(KunaError::parse(error))
    }

    /// Withdraw every tag this parse declared that is still incomplete.
    pub(super) fn discard_forward_tags(&mut self) {
        for tag in std::mem::take(&mut self.forward_tags) {
            if let Ok(Some(ct)) = self.factory.find_by_name(&tag.name) {
                let meta = ct.get_metatype();
                if ct.is_incomplete()
                    && (meta == type_metatype::TYPE_STRUCT || meta == type_metatype::TYPE_UNION)
                {
                    let _ = self.factory.destroy_type(&ct);
                }
            }
        }
    }

    /// Discard the declared tags when a post-parse step fails.
    pub(super) fn settle<T>(&mut self, res: KunaResult<T>) -> KunaResult<T> {
        if res.is_err() {
            self.discard_forward_tags();
        }
        res
    }
}

/// Is `decl` the tag `tag` and nothing else: no pointer, array or function
/// modifier, not `extern`, and declaring either no name or the tag's own?
fn declares_only(decl: &TypeDeclarator, tag: &str) -> bool {
    decl.num_modifiers() == 0
        && !decl.has_property(flags::F_EXTERN)
        && decl.get_base_type().is_some_and(|b| b.get_name() == tag)
        && (decl.get_identifier().is_empty() || decl.get_identifier() == tag)
}

#[cfg(test)]
mod tests;
