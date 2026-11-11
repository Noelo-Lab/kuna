//! kuna fix `kt_suspend_function_type` (spec 10.16): in Kotlin output, a
//! suspend function type is written `suspend (A) -> R`.
//!
//! The metadata records a suspend function type as the function type kotlinc
//! compiles it to, `Function2<A, Continuation<R>, Any?>`, with the type's
//! `suspend` flag set. Vineflower ignores the flag and writes `(A,
//! Continuation<R>) -> Any?` for a parameter declared `suspend (A) -> R`
//! (kotlinx-coroutines' `block`, `transform` and `action` parameters), which
//! names another type and makes every `block(a)` in the source an error. With
//! the fix a function type whose flag is set and whose last parameter is a
//! `Continuation` is written without that parameter, with the continuation's
//! type argument as the result and `suspend` in front, the type the
//! metadata describes.

use crate::k_type::{KType, KTypeData};
use crate::kt_flags::Flags;
use crate::kt_proto;
use crate::text_buffer::TextBuffer;

const CONTINUATION: &str = "kotlin/coroutines/Continuation";

/// Whether the metadata type `type_` is a suspend function type.
pub fn is_suspend(type_: &kt_proto::Type) -> bool {
    let flags = type_.message().scalar(1).map_or(0, |v| v as i32);
    Flags::SUSPEND_TYPE.get(flags)
}

/// `k`, a function type marked suspend, written as one: `None` when its last
/// parameter is not a continuation.
pub fn write(k: &KTypeData, indent: i32) -> Option<TextBuffer> {
    let arguments = k.type_arguments.as_ref()?;
    if arguments.len() < 2 {
        return None;
    }
    let continuation = &arguments[arguments.len() - 2].type_;
    let continuation = continuation.0.kotlin.as_deref()?;
    if continuation.kotlin_type != CONTINUATION {
        return None;
    }
    let result: KType = continuation.type_arguments.as_ref()?.first()?.type_.clone();
    let mut buf = TextBuffer::new();
    buf.append(if k.is_nullable { "(suspend (" } else { "suspend (" });
    buf.push_newline_group(indent, 1);
    buf.append_possible_newline();
    for (i, argument) in arguments[..arguments.len() - 2].iter().enumerate() {
        if i != 0 {
            buf.append(",").append_possible_newline_alt(" ");
        }
        buf.append_buffer(&argument.stringify(indent + 1));
    }
    buf.append_possible_newline_dedent("", true);
    buf.pop_newline_group();
    buf.append(") -> ");
    buf.append_buffer(&result.stringify(indent + 1));
    if k.is_nullable {
        buf.append(")?");
    }
    Some(buf)
}
