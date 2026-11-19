//! The `__PRELINK_INFO,__info` XML plist of a kernelcache: one dictionary
//! per kext in `_PrelinkInfoDictionary`, read for its bundle id, version, load
//! address and the `IOClass` names of its `IOKitPersonalities`.

use std::collections::HashMap;
use std::rc::Rc;

use super::{load_commands, rd_u32, rd_u64, LC_SEGMENT_64};

#[derive(Debug, PartialEq)]
pub(super) struct KextInfo {
    pub id: String,
    pub version: String,
    pub load_addr: Option<u64>,
    /// `ModuleIndex`: the kext's slot in `__kmod_info`/`__kmod_start` of a
    /// merged (iOS 12-14) prelinked kernelcache.
    pub module_index: Option<u64>,
    pub classes: Vec<String>,
}

#[derive(Debug, Clone)]
enum Value {
    Dict(Rc<Vec<(String, Value)>>),
    Array(Rc<Vec<Value>>),
    Str(String),
    Int(u64),
    Other,
}

impl Value {
    fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Dict(items) => items.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }
}

struct Parser<'a> {
    text: &'a str,
    pos: usize,
    ids: HashMap<String, Value>,
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let at = tag.find(&format!("{name}=\""))? + name.len() + 2;
    tag[at..].split('"').next().map(str::to_string)
}

fn unescape(s: &str) -> String {
    s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&")
}

impl<'a> Parser<'a> {
    /// The next tag, `<...>`, skipping text and declarations.
    fn tag(&mut self) -> Option<&'a str> {
        loop {
            let start = self.pos + self.text.get(self.pos..)?.find('<')?;
            let end = start + self.text[start..].find('>')?;
            self.pos = end + 1;
            let tag = &self.text[start + 1..end];
            if !tag.starts_with('?') && !tag.starts_with('!') {
                return Some(tag);
            }
        }
    }

    fn text_until(&mut self, close: &str) -> Option<String> {
        let end = self.pos + self.text[self.pos..].find(close)?;
        let s = unescape(&self.text[self.pos..end]);
        self.pos = end + close.len();
        Some(s)
    }

    fn value(&mut self, depth: usize) -> Option<Value> {
        let tag = self.tag()?;
        self.value_from(tag, depth)
    }

    fn value_from(&mut self, tag: &str, depth: usize) -> Option<Value> {
        if depth > 64 {
            return None;
        }
        let name = tag.split(|c: char| c.is_whitespace() || c == '/').next()?;
        if let Some(r) = attr(tag, "IDREF") {
            return Some(self.ids.get(&r).cloned().unwrap_or(Value::Other));
        }
        let empty = tag.ends_with('/');
        let v = match name {
            "dict" if !empty => {
                let mut items = Vec::new();
                loop {
                    let t = self.tag()?;
                    if t == "/dict" {
                        break;
                    }
                    let key = if t == "key" { self.text_until("</key>")? } else { return None };
                    items.push((key, self.value(depth + 1)?));
                }
                Value::Dict(Rc::new(items))
            }
            "array" if !empty => {
                let mut items = Vec::new();
                loop {
                    let t = self.tag()?;
                    if t == "/array" {
                        break;
                    }
                    items.push(self.value_from(t, depth + 1)?);
                }
                Value::Array(Rc::new(items))
            }
            "string" if !empty => Value::Str(self.text_until("</string>")?),
            "integer" if !empty => {
                let s = self.text_until("</integer>")?;
                let s = s.trim();
                let n = match s.strip_prefix("0x") {
                    Some(h) => u64::from_str_radix(h, 16).ok(),
                    None => s.parse::<i64>().ok().map(|v| v as u64),
                };
                n.map_or(Value::Other, Value::Int)
            }
            "data" if !empty => {
                self.text_until("</data>")?;
                Value::Other
            }
            "dict" => Value::Dict(Rc::default()),
            "array" => Value::Array(Rc::default()),
            "string" => Value::Str(String::new()),
            _ => Value::Other,
        };
        if let Some(id) = attr(tag, "ID") {
            self.ids.insert(id, v.clone());
        }
        Some(v)
    }
}

/// The `__PRELINK_INFO,__info` bytes of the image whose header is at offset 0.
fn info_section(bytes: &[u8]) -> Option<&[u8]> {
    for (cmd, p) in load_commands(bytes, 0)? {
        if cmd != LC_SEGMENT_64 || bytes.get(p + 8..p + 22) != Some(b"__PRELINK_INFO") {
            continue;
        }
        for k in 0..rd_u32(bytes, p + 64)? as usize {
            let s = p + 72 + k * 80;
            if bytes.get(s..s + 7) == Some(b"__info\0") {
                let (size, off) = (rd_u64(bytes, s + 40)? as usize, rd_u32(bytes, s + 48)? as usize);
                return bytes.get(off..off.checked_add(size)?);
            }
        }
    }
    None
}

pub(super) fn kexts(bytes: &[u8]) -> Option<Vec<KextInfo>> {
    let raw = info_section(bytes)?;
    let text = std::str::from_utf8(&raw[..raw.iter().position(|&b| b == 0).unwrap_or(raw.len())]).ok()?;
    let mut parser = Parser { text, pos: 0, ids: HashMap::new() };
    let root = loop {
        let tag = parser.tag()?;
        if tag == "plist" || tag.starts_with("plist ") {
            break parser.value(0)?;
        }
        if tag == "dict" {
            break parser.value_from(tag, 0)?;
        }
    };
    let Value::Array(items) = root.get("_PrelinkInfoDictionary")? else { return None };
    Some(
        items
            .iter()
            .filter_map(|d| {
                let id = d.get("CFBundleIdentifier")?.str()?.to_string();
                let version = d
                    .get("CFBundleShortVersionString")
                    .or_else(|| d.get("CFBundleVersion"))
                    .and_then(Value::str)
                    .unwrap_or("")
                    .to_string();
                let int = |key: &str| match d.get(key) {
                    Some(Value::Int(a)) => Some(*a),
                    _ => None,
                };
                let (load_addr, module_index) = (int("_PrelinkExecutableLoadAddr"), int("ModuleIndex"));
                let mut classes: Vec<String> = match d.get("IOKitPersonalities") {
                    Some(Value::Dict(ps)) => {
                        ps.iter().filter_map(|(_, p)| p.get("IOClass")?.str().map(str::to_string)).collect()
                    }
                    _ => Vec::new(),
                };
                classes.sort();
                classes.dedup();
                Some(KextInfo { id, version, load_addr, module_index, classes })
            })
            .collect(),
    )
}

/// One line per kext for `--option kernelcache list`.
pub(super) fn describe(info: &KextInfo) -> String {
    let mut line = info.id.clone();
    if !info.version.is_empty() {
        line += &format!(" {}", info.version);
    }
    if let Some(a) = info.load_addr {
        line += &format!(" @{a:#x}");
    }
    if !info.classes.is_empty() {
        line += &format!(" IOClass: {}", info.classes.join(", "));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ids_versions_classes_and_idrefs() {
        let xml = r#"<?xml version="1.0"?><plist version="1.0"><dict>
            <key>_PrelinkInfoDictionary</key><array>
              <dict><key>CFBundleIdentifier</key><string>com.example.A</string>
                <key>CFBundleVersion</key><string ID="1">1.0</string>
                <key>_PrelinkExecutableLoadAddr</key><integer size="64">0xfffffff007004000</integer>
                <key>IOKitPersonalities</key><dict>
                  <key>p1</key><dict><key>IOClass</key><string ID="2">AService</string></dict>
                  <key>p2</key><dict><key>IOClass</key><string IDREF="2"/><key>IOProbeScore</key><integer>5</integer></dict>
                </dict>
                <key>OSBundleLibraries</key><dict/>
                <key>Blob</key><data>AAAA</data>
              </dict>
              <dict><key>CFBundleIdentifier</key><string>com.example.B</string>
                <key>CFBundleVersion</key><string IDREF="1"/></dict>
            </array></dict></plist>"#;
        let mut p = Parser { text: xml, pos: 0, ids: HashMap::new() };
        while p.tag() != Some(r#"plist version="1.0""#) {}
        let root = p.value(0).unwrap();
        let Some(Value::Array(items)) = root.get("_PrelinkInfoDictionary") else { panic!() };
        assert_eq!(items.len(), 2);
        let bare = &xml[xml.find("<dict>").unwrap()..xml.rfind("</plist>").unwrap()];
        let mut p = Parser { text: bare, pos: 0, ids: HashMap::new() };
        let tag = p.tag().unwrap();
        assert!(p.value_from(tag, 0).unwrap().get("_PrelinkInfoDictionary").is_some());
        assert_eq!(items[1].get("CFBundleVersion").and_then(Value::str), Some("1.0"));
        let ps = items[0].get("IOKitPersonalities").unwrap();
        assert_eq!(ps.get("p2").and_then(|d| d.get("IOClass")).and_then(Value::str), Some("AService"));
    }

    /// Each level holds ten references to the level below: copying the shared
    /// values would grow to 10^39 nodes, sharing them keeps the parse linear.
    #[test]
    fn idref_expansion_shares_values_instead_of_copying_them() {
        let mut xml = String::from(r#"<dict ID="v0"><key>k</key><string>x</string></dict>"#);
        for level in 1..40 {
            let refs: String = (0..10).map(|_| format!(r#"<array IDREF="v{}"/>"#, level - 1)).collect();
            xml += &format!(r#"<array ID="v{level}">{refs}</array>"#);
        }
        xml += r#"<array IDREF="v39"/>"#;
        let text = format!("<array>{xml}</array>");
        let mut p = Parser { text: &text, pos: 0, ids: HashMap::new() };
        let tag = p.tag().unwrap();
        let Some(Value::Array(items)) = p.value_from(tag, 0) else { panic!() };
        assert_eq!(items.len(), 41);
    }
}
