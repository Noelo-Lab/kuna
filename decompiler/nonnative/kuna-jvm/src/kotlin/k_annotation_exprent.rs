//! Port of `org/vineflower/kotlin/expr/KAnnotationExprent.java`.

use crate::annotation_exprent::AnnotationExprent;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

/// `KAnnotationExprent.UseSiteTarget`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum UseSiteTarget {
    File,
    Property,
    Field,
    Get,
    Set,
    Receiver,
    Param,
    Setparam,
    Delegate,
}

impl UseSiteTarget {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            UseSiteTarget::File => "FILE",
            UseSiteTarget::Property => "PROPERTY",
            UseSiteTarget::Field => "FIELD",
            UseSiteTarget::Get => "GET",
            UseSiteTarget::Set => "SET",
            UseSiteTarget::Receiver => "RECEIVER",
            UseSiteTarget::Param => "PARAM",
            UseSiteTarget::Setparam => "SETPARAM",
            UseSiteTarget::Delegate => "DELEGATE",
        }
    }
}

pub struct KAnnotationExprentData {
    pub use_site_target: Option<UseSiteTarget>,
}

kotlin_exprent!(pub KAnnotationExprent: AnnotationExprent, Annotation(KAnnotationExprentData),
                is_k_annotation, as_k_annotation, try_as_k_annotation);

impl KAnnotationExprent {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KAnnotationExprent.toJava")
    }
}
