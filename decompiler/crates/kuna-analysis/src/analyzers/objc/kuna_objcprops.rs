//! Objective-C properties: the `property_list_t` of a class, metaclass,
//! category or protocol, read for the one thing a method's own encoding lacks.
//!
//! A method encoding spells an object as a bare `@`; the property it
//! implements spells it with its class (`T@"Base",&,N,Vinner`), so the
//! synthesized getter and setter can return and take a `Base *` that is the
//! class's own struct (§1.4 `objcivars`), and a field read through the result
//! renders. Each property yields its getter selector (`name`, or `G<sel>`) with
//! `<T>@:` and, unless `R` (readonly), its setter (`setName:`, or `S<sel>`)
//! with `v@:<T>`; the type is the attribute's own encoding text.

use super::sections::MachoImage;

/// `property_list_t` header offsets: `entsizeAndFlags` (`u32`), `count` (`u32`),
/// then `property_t { name, attributes }` records.
mod property_list_t {
    pub const ENTSIZE: u64 = 0x00;
    pub const COUNT: u64 = 0x04;
    pub const FIRST: u64 = 0x08;
}

/// A property's selectors with their encodings: `(selector, encoding)`.
pub fn declarations(img: &MachoImage, list_va: u64) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let Some(ent) = img.read_u32(list_va + property_list_t::ENTSIZE) else { return out };
    let Some(count) = img.read_u32(list_va + property_list_t::COUNT) else { return out };
    let entsize = (ent & 0xffff) as u64;
    if entsize < 2 * img.ptr_size() {
        return out;
    }
    for i in 0..count.min(4096) as u64 {
        let rec = list_va.wrapping_add(property_list_t::FIRST).wrapping_add(i.wrapping_mul(entsize));
        let Some(name) = img.rebase_ptr(rec).and_then(|p| img.read_cstr(p)) else { continue };
        let Some(attrs) = img.rebase_ptr(rec + img.ptr_size()).and_then(|p| img.read_cstr(p)) else { continue };
        out.extend(from_attributes(&name, &attrs));
    }
    out
}

/// The getter and setter a property declares, from its attribute string
/// (`T@"Base",&,N,Vinner`; `R` readonly, `G`/`S` custom getter/setter).
pub fn from_attributes(name: &str, attrs: &str) -> Vec<(String, String)> {
    let mut ty = None;
    let mut readonly = false;
    let mut getter = None;
    let mut setter = None;
    for attr in split_attributes(attrs) {
        match attr.as_bytes().first() {
            Some(b'T') => ty = Some(attr[1..].to_string()),
            Some(b'R') => readonly = true,
            Some(b'G') => getter = Some(attr[1..].to_string()),
            Some(b'S') => setter = Some(attr[1..].to_string()),
            _ => {}
        }
    }
    let Some(ty) = ty.filter(|t| !t.is_empty()) else { return Vec::new() };
    let mut out = vec![(getter.unwrap_or_else(|| name.to_string()), format!("{ty}@:"))];
    if !readonly {
        let setter = setter.unwrap_or_else(|| {
            let mut c = name.chars();
            format!("set{}{}:", c.next().map(|f| f.to_ascii_uppercase()).unwrap_or_default(), c.as_str())
        });
        out.push((setter, format!("v@:{ty}")));
    }
    out
}

/// The comma-separated attributes, a comma inside a `T` type's quotes or braces
/// (`T{CGRect={CGPoint=dd}{CGSize=dd}}`, `T@"NSArray<X>"`) not splitting.
fn split_attributes(attrs: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let (mut depth, mut quoted) = (0i32, false);
    for c in attrs.chars() {
        match c {
            '"' => quoted = !quoted,
            '{' | '(' | '[' if !quoted => depth += 1,
            '}' | ')' | ']' if !quoted => depth -= 1,
            ',' if !quoted && depth <= 0 => {
                out.push(std::mem::take(&mut cur));
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    out.push(cur);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_yield_typed_getter_and_setter() {
        assert_eq!(
            from_attributes("inner", "T@\"Base\",&,N,Vinner"),
            [("inner".to_string(), "@\"Base\"@:".to_string()), ("setInner:".to_string(), "v@:@\"Base\"".to_string())]
        );
        assert_eq!(from_attributes("count", "Ti,R,N,Vcount"), [("count".to_string(), "i@:".to_string())]);
        assert_eq!(
            from_attributes("hidden", "TB,N,GisHidden,SsetIsHidden:,Vhidden"),
            [("isHidden".to_string(), "B@:".to_string()), ("setIsHidden:".to_string(), "v@:B".to_string())]
        );
        assert_eq!(
            from_attributes("frame", "T{CGRect={CGPoint=dd}{CGSize=dd}},N,Vframe")[0].1,
            "{CGRect={CGPoint=dd}{CGSize=dd}}@:"
        );
        assert!(from_attributes("x", "N,Vx").is_empty(), "no type, no declaration");
    }
}
