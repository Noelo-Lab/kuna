//! Call-site prototypes: the declaration one call instruction is decompiled
//! against, shared by the console's `override prototype` and the `--assert`
//! directive `callproto [<func>::]<callsite> <C declaration>`.
//!
//! The declaration is the grammar `prototype` takes, naming no calling
//! convention but the default one (under any name the compiler spec gives it),
//! which the call is laid out under. Each parameter may be followed by
//! `@<storage>` in the spelling `param` takes (`RDI`, `%RDI`,
//! `[stack,-0x18,8]`); a `@<storage>` after the parameter list places the
//! return value. A parameter without one is placed by the default convention
//! around those that have one.

use kuna_base::address::Address;
use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces, PrototypePieces};
use kuna_decomp::funcdata::Funcdata;

use crate::engine::ConsoleProgram;
use crate::grammar::DataOrg;

/// The console's answer for a call-site prototype at an address no call is at.
pub(crate) const NO_CALL: &str = "No call is made at this address";

/// Where one `@<storage>` annotation was written.
#[derive(Debug, PartialEq)]
enum Place {
    Param(usize),
    Return,
}

/// Parse a call-site declaration into the pieces a prototype override carries.
pub(crate) fn parse(prog: &ConsoleProgram, decl: &str) -> Result<PrototypePieces, String> {
    let (text, storage) = split_storage(decl)?;
    let (addr_size, word_size) = prog.arch().data_org();
    let org = DataOrg { addr_size, word_size };
    let text = text.trim();
    let text = if text.ends_with(';') { text.to_string() } else { format!("{text};") };
    let models = crate::assertions::model_names(prog);
    let (mut pieces, model) =
        crate::grammar::parse_protopieces_with_model(&text, prog.arch().types(), org, &models)
            .map_err(|e| e.explain().to_string())?;
    let arch = prog.arch();
    let is_default = |name: &str| {
        arch.default_fp().is_some_and(|d| {
            d.get_name() == name
                || arch
                    .get_model(name)
                    .is_some_and(|m| m.is_compatible(d) && m.has_this_pointer() == d.has_this_pointer())
        })
    };
    if !model.is_empty() && !is_default(&model) {
        return Err(format!(
            "a call-site prototype is laid out under the default calling convention and \
             cannot name {model}; place its parameters with @<storage>"
        ));
    }
    for (place, token) in storage {
        let token = crate::assertsyntax::normalize_storage(&token);
        let mut stream = crate::interface::CommandStream::new(&token);
        let (addr, _) = crate::ifacedecomp::parse_machaddr(prog, &mut stream, false)?;
        match place {
            Place::Param(slot) => {
                let type_ = pieces
                    .intypes
                    .get(slot)
                    .cloned()
                    .ok_or_else(|| format!("@{token} follows no parameter"))?;
                pieces.input_storage.push((
                    slot as i32,
                    ParameterPieces {
                        addr,
                        type_: Some(type_),
                        flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
                    },
                ));
            }
            Place::Return => {
                let type_ = pieces
                    .outtype
                    .clone()
                    .filter(|t| t.get_size() > 0)
                    .ok_or_else(|| format!("@{token} places a void return"))?;
                pieces.output_storage = Some(ParameterPieces {
                    addr,
                    type_: Some(type_),
                    flags: parameter_pieces_flags::TYPELOCK,
                });
            }
        }
    }
    Ok(pieces)
}

/// Is a call made by the instruction at `callpoint`?
pub(crate) fn has_call_at(fd: &Funcdata, callpoint: &Address) -> bool {
    (0..fd.num_calls()).any(|i| {
        fd.obank().get(fd.get_call_specs(i).get_op()).is_some_and(|op| op.get_addr() == callpoint)
    })
}

/// Separate the `@<storage>` annotations from the C declaration they sit in.
fn split_storage(decl: &str) -> Result<(String, Vec<(Place, String)>), String> {
    let bytes = decl.as_bytes();
    let mut open = None;
    let mut depth = 0usize;
    for (i, b) in bytes.iter().enumerate() {
        match b {
            b'(' => {
                if depth == 0 {
                    open = Some(i);
                }
                depth += 1;
            }
            b')' => depth = depth.checked_sub(1).ok_or("unbalanced ')' in declaration")?,
            _ => {}
        }
    }
    let mut text = String::with_capacity(decl.len());
    let mut storage: Vec<(Place, String)> = Vec::new();
    let mut slot = 0usize;
    let mut depth = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b != b'@' {
            match b {
                b'(' => depth += 1,
                b')' => depth = depth.saturating_sub(1),
                b',' if depth == 1 && open.is_some_and(|o| i > o) => slot += 1,
                _ => {}
            }
            text.push(b as char);
            i += 1;
            continue;
        }
        let place = match open {
            Some(o) if i > o && depth == 1 => Place::Param(slot),
            Some(o) if i > o && depth == 0 => Place::Return,
            _ => return Err("@<storage> must follow a parameter or the parameter list".into()),
        };
        if storage.iter().any(|(p, _)| *p == place) {
            return Err("one @<storage> per parameter".into());
        }
        i += 1;
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        let start = i;
        if bytes.get(i) == Some(&b'[') {
            while i < bytes.len() && bytes[i] != b']' {
                i += 1;
            }
            i = (i + 1).min(bytes.len());
        } else {
            while i < bytes.len() && !matches!(bytes[i], b' ' | b',' | b')' | b';') {
                i += 1;
            }
        }
        if start == i {
            return Err("@ needs a storage location".into());
        }
        storage.push((place, decl[start..i].to_string()));
    }
    Ok((text, storage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_annotations_are_split_by_parameter() {
        let (text, storage) =
            split_storage("int f(char *a @RDI, void (*cb)(int, int) @ [register,0x80,8], int c) @EAX")
                .unwrap();
        assert_eq!(text, "int f(char *a , void (*cb)(int, int) , int c) ");
        assert_eq!(
            storage,
            vec![
                (Place::Param(0), "RDI".to_string()),
                (Place::Param(1), "[register,0x80,8]".to_string()),
                (Place::Return, "EAX".to_string()),
            ]
        );
    }

    #[test]
    fn misplaced_storage_is_an_error() {
        assert!(split_storage("int @EAX f(void)").is_err());
        assert!(split_storage("int f(int a @RDI @RSI)").is_err());
        assert!(split_storage("int f(int a @)").is_err());
    }
}
